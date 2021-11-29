#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACPI_REAL_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ACPI_REAL_TIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for ACPI_REAL_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Year == other.Year && self.Month == other.Month && self.Day == other.Day && self.Hour == other.Hour && self.Minute == other.Minute && self.Second == other.Second && self.Valid == other.Valid && self.Milliseconds == other.Milliseconds && self.TimeZone == other.TimeZone && self.DayLight == other.DayLight && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for ACPI_REAL_TIME {}
unsafe impl ::windows::core::Abi for ACPI_REAL_TIME {
    type Abi = Self;
}
pub const ACPI_TIME_ADJUST_DAYLIGHT: u32 = 1u32;
pub const ACPI_TIME_IN_DAYLIGHT: u32 = 2u32;
pub const ACPI_TIME_ZONE_UNKNOWN: u32 = 2047u32;
pub const ACTIVE_COOLING: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ADMINISTRATOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ADMINISTRATOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for ADMINISTRATOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.MinSleep == other.MinSleep && self.MaxSleep == other.MaxSleep && self.MinVideoTimeout == other.MinVideoTimeout && self.MaxVideoTimeout == other.MaxVideoTimeout && self.MinSpindownTimeout == other.MinSpindownTimeout && self.MaxSpindownTimeout == other.MaxSpindownTimeout
    }
}
impl ::core::cmp::Eq for ADMINISTRATOR_POWER_POLICY {}
unsafe impl ::windows::core::Abi for ADMINISTRATOR_POWER_POLICY {
    type Abi = Self;
}
pub const BATTERY_CAPACITY_RELATIVE: u32 = 1073741824u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub VaData: [u32; 1],
}
impl BATTERY_CHARGER_STATUS {}
impl ::core::default::Default for BATTERY_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_CHARGER_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGER_STATUS").field("Type", &self.Type).field("VaData", &self.VaData).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.VaData == other.VaData
    }
}
impl ::core::cmp::Eq for BATTERY_CHARGER_STATUS {}
unsafe impl ::windows::core::Abi for BATTERY_CHARGER_STATUS {
    type Abi = Self;
}
pub const BATTERY_CHARGING: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_CHARGING_SOURCE {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub MaxCurrent: u32,
}
impl BATTERY_CHARGING_SOURCE {}
impl ::core::default::Default for BATTERY_CHARGING_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_CHARGING_SOURCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGING_SOURCE").field("Type", &self.Type).field("MaxCurrent", &self.MaxCurrent).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_CHARGING_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.MaxCurrent == other.MaxCurrent
    }
}
impl ::core::cmp::Eq for BATTERY_CHARGING_SOURCE {}
unsafe impl ::windows::core::Abi for BATTERY_CHARGING_SOURCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BATTERY_CHARGING_SOURCE_INFORMATION {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub SourceOnline: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl BATTERY_CHARGING_SOURCE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_CHARGING_SOURCE_INFORMATION").field("Type", &self.Type).field("SourceOnline", &self.SourceOnline).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.SourceOnline == other.SourceOnline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BATTERY_CHARGING_SOURCE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BATTERY_CHARGING_SOURCE_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BATTERY_CHARGING_SOURCE_TYPE(pub i32);
pub const BatteryChargingSourceType_AC: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(1i32);
pub const BatteryChargingSourceType_USB: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(2i32);
pub const BatteryChargingSourceType_Wireless: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(3i32);
pub const BatteryChargingSourceType_Max: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(4i32);
impl ::core::convert::From<i32> for BATTERY_CHARGING_SOURCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_CHARGING_SOURCE_TYPE {
    type Abi = Self;
}
pub const BATTERY_CLASS_MAJOR_VERSION: u32 = 1u32;
pub const BATTERY_CLASS_MINOR_VERSION: u32 = 0u32;
pub const BATTERY_CLASS_MINOR_VERSION_1: u32 = 1u32;
pub const BATTERY_CRITICAL: u32 = 8u32;
pub const BATTERY_CYCLE_COUNT_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef98db24_0014_4c25_a50b_c724ae5cd371);
pub const BATTERY_DISCHARGING: u32 = 2u32;
pub const BATTERY_FULL_CHARGED_CAPACITY_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40b40565_96f7_4435_8694_97e0e4395905);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for BATTERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.Technology == other.Technology && self.Reserved == other.Reserved && self.Chemistry == other.Chemistry && self.DesignedCapacity == other.DesignedCapacity && self.FullChargedCapacity == other.FullChargedCapacity && self.DefaultAlert1 == other.DefaultAlert1 && self.DefaultAlert2 == other.DefaultAlert2 && self.CriticalBias == other.CriticalBias && self.CycleCount == other.CycleCount
    }
}
impl ::core::cmp::Eq for BATTERY_INFORMATION {}
unsafe impl ::windows::core::Abi for BATTERY_INFORMATION {
    type Abi = Self;
}
pub const BATTERY_IS_SHORT_TERM: u32 = 536870912u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_MANUFACTURE_DATE {
    pub Day: u8,
    pub Month: u8,
    pub Year: u16,
}
impl BATTERY_MANUFACTURE_DATE {}
impl ::core::default::Default for BATTERY_MANUFACTURE_DATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_MANUFACTURE_DATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_MANUFACTURE_DATE").field("Day", &self.Day).field("Month", &self.Month).field("Year", &self.Year).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_MANUFACTURE_DATE {
    fn eq(&self, other: &Self) -> bool {
        self.Day == other.Day && self.Month == other.Month && self.Year == other.Year
    }
}
impl ::core::cmp::Eq for BATTERY_MANUFACTURE_DATE {}
unsafe impl ::windows::core::Abi for BATTERY_MANUFACTURE_DATE {
    type Abi = Self;
}
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_1: u32 = 1u32;
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_2: u32 = 2u32;
pub const BATTERY_POWER_ON_LINE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_QUERY_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_QUERY_INFORMATION_LEVEL,
    pub AtRate: u32,
}
impl BATTERY_QUERY_INFORMATION {}
impl ::core::default::Default for BATTERY_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_QUERY_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_QUERY_INFORMATION").field("BatteryTag", &self.BatteryTag).field("InformationLevel", &self.InformationLevel).field("AtRate", &self.AtRate).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_QUERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.InformationLevel == other.InformationLevel && self.AtRate == other.AtRate
    }
}
impl ::core::cmp::Eq for BATTERY_QUERY_INFORMATION {}
unsafe impl ::windows::core::Abi for BATTERY_QUERY_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BATTERY_QUERY_INFORMATION_LEVEL(pub i32);
pub const BatteryInformation: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(0i32);
pub const BatteryGranularityInformation: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(1i32);
pub const BatteryTemperature: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(2i32);
pub const BatteryEstimatedTime: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(3i32);
pub const BatteryDeviceName: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(4i32);
pub const BatteryManufactureDate: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(5i32);
pub const BatteryManufactureName: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(6i32);
pub const BatteryUniqueID: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(7i32);
pub const BatterySerialNumber: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(8i32);
impl ::core::convert::From<i32> for BATTERY_QUERY_INFORMATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_QUERY_INFORMATION_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_REPORTING_SCALE {
    pub Granularity: u32,
    pub Capacity: u32,
}
impl BATTERY_REPORTING_SCALE {}
impl ::core::default::Default for BATTERY_REPORTING_SCALE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_REPORTING_SCALE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_REPORTING_SCALE").field("Granularity", &self.Granularity).field("Capacity", &self.Capacity).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_REPORTING_SCALE {
    fn eq(&self, other: &Self) -> bool {
        self.Granularity == other.Granularity && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for BATTERY_REPORTING_SCALE {}
unsafe impl ::windows::core::Abi for BATTERY_REPORTING_SCALE {
    type Abi = Self;
}
pub const BATTERY_RUNTIME_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x535a3767_1ac2_49bc_a077_3f7a02e40aec);
pub const BATTERY_SEALED: u32 = 268435456u32;
pub const BATTERY_SET_CHARGER_ID_SUPPORTED: u32 = 8u32;
pub const BATTERY_SET_CHARGE_SUPPORTED: u32 = 1u32;
pub const BATTERY_SET_CHARGINGSOURCE_SUPPORTED: u32 = 4u32;
pub const BATTERY_SET_DISCHARGE_SUPPORTED: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_SET_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_SET_INFORMATION_LEVEL,
    pub Buffer: [u8; 1],
}
impl BATTERY_SET_INFORMATION {}
impl ::core::default::Default for BATTERY_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_SET_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_SET_INFORMATION").field("BatteryTag", &self.BatteryTag).field("InformationLevel", &self.InformationLevel).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_SET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.InformationLevel == other.InformationLevel && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for BATTERY_SET_INFORMATION {}
unsafe impl ::windows::core::Abi for BATTERY_SET_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BATTERY_SET_INFORMATION_LEVEL(pub i32);
pub const BatteryCriticalBias: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(0i32);
pub const BatteryCharge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(1i32);
pub const BatteryDischarge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(2i32);
pub const BatteryChargingSource: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(3i32);
pub const BatteryChargerId: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(4i32);
pub const BatteryChargerStatus: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(5i32);
impl ::core::convert::From<i32> for BATTERY_SET_INFORMATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BATTERY_SET_INFORMATION_LEVEL {
    type Abi = Self;
}
pub const BATTERY_STATIC_DATA_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e1e463_e4e2_4ea9_80cb_9bd4b3ca0655);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_STATUS {
    pub PowerState: u32,
    pub Capacity: u32,
    pub Voltage: u32,
    pub Rate: i32,
}
impl BATTERY_STATUS {}
impl ::core::default::Default for BATTERY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_STATUS").field("PowerState", &self.PowerState).field("Capacity", &self.Capacity).field("Voltage", &self.Voltage).field("Rate", &self.Rate).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.PowerState == other.PowerState && self.Capacity == other.Capacity && self.Voltage == other.Voltage && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for BATTERY_STATUS {}
unsafe impl ::windows::core::Abi for BATTERY_STATUS {
    type Abi = Self;
}
pub const BATTERY_STATUS_CHANGE_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcddfa0c3_7c5b_4e43_a034_059fa5b84364);
pub const BATTERY_STATUS_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4670d1_ebbf_416e_87ce_374a4ebc111a);
pub const BATTERY_SYSTEM_BATTERY: u32 = 2147483648u32;
pub const BATTERY_TAG_CHANGE_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e1f6e19_8786_4d23_94fc_9e746bd5d888);
pub const BATTERY_TAG_INVALID: u32 = 0u32;
pub const BATTERY_TEMPERATURE_WMI_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a52a14d_adce_4a44_9a3e_c8d8f15ff2c2);
pub const BATTERY_UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_RATE: u32 = 2147483648u32;
pub const BATTERY_UNKNOWN_TIME: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_VOLTAGE: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_USB_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub Reserved: u32,
    pub Flags: u32,
    pub MaxCurrent: u32,
    pub Voltage: u32,
    pub PortType: USB_CHARGER_PORT,
    pub PortId: u64,
    pub PowerSourceInformation: *mut ::core::ffi::c_void,
    pub OemCharger: ::windows::core::GUID,
}
impl BATTERY_USB_CHARGER_STATUS {}
impl ::core::default::Default for BATTERY_USB_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_USB_CHARGER_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for BATTERY_USB_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Flags == other.Flags && self.MaxCurrent == other.MaxCurrent && self.Voltage == other.Voltage && self.PortType == other.PortType && self.PortId == other.PortId && self.PowerSourceInformation == other.PowerSourceInformation && self.OemCharger == other.OemCharger
    }
}
impl ::core::cmp::Eq for BATTERY_USB_CHARGER_STATUS {}
unsafe impl ::windows::core::Abi for BATTERY_USB_CHARGER_STATUS {
    type Abi = Self;
}
pub const BATTERY_USB_CHARGER_STATUS_FN_DEFAULT_USB: u32 = 1u32;
pub const BATTERY_USB_CHARGER_STATUS_UCM_PD: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BATTERY_WAIT_STATUS {
    pub BatteryTag: u32,
    pub Timeout: u32,
    pub PowerState: u32,
    pub LowCapacity: u32,
    pub HighCapacity: u32,
}
impl BATTERY_WAIT_STATUS {}
impl ::core::default::Default for BATTERY_WAIT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BATTERY_WAIT_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BATTERY_WAIT_STATUS").field("BatteryTag", &self.BatteryTag).field("Timeout", &self.Timeout).field("PowerState", &self.PowerState).field("LowCapacity", &self.LowCapacity).field("HighCapacity", &self.HighCapacity).finish()
    }
}
impl ::core::cmp::PartialEq for BATTERY_WAIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.Timeout == other.Timeout && self.PowerState == other.PowerState && self.LowCapacity == other.LowCapacity && self.HighCapacity == other.HighCapacity
    }
}
impl ::core::cmp::Eq for BATTERY_WAIT_STATUS {}
unsafe impl ::windows::core::Abi for BATTERY_WAIT_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CM_POWER_DATA {
    pub PD_Size: u32,
    pub PD_MostRecentPowerState: DEVICE_POWER_STATE,
    pub PD_Capabilities: u32,
    pub PD_D1Latency: u32,
    pub PD_D2Latency: u32,
    pub PD_D3Latency: u32,
    pub PD_PowerStateMapping: [DEVICE_POWER_STATE; 7],
    pub PD_DeepestSystemWake: SYSTEM_POWER_STATE,
}
impl CM_POWER_DATA {}
impl ::core::default::Default for CM_POWER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CM_POWER_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CM_POWER_DATA")
            .field("PD_Size", &self.PD_Size)
            .field("PD_MostRecentPowerState", &self.PD_MostRecentPowerState)
            .field("PD_Capabilities", &self.PD_Capabilities)
            .field("PD_D1Latency", &self.PD_D1Latency)
            .field("PD_D2Latency", &self.PD_D2Latency)
            .field("PD_D3Latency", &self.PD_D3Latency)
            .field("PD_PowerStateMapping", &self.PD_PowerStateMapping)
            .field("PD_DeepestSystemWake", &self.PD_DeepestSystemWake)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CM_POWER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PD_Size == other.PD_Size && self.PD_MostRecentPowerState == other.PD_MostRecentPowerState && self.PD_Capabilities == other.PD_Capabilities && self.PD_D1Latency == other.PD_D1Latency && self.PD_D2Latency == other.PD_D2Latency && self.PD_D3Latency == other.PD_D3Latency && self.PD_PowerStateMapping == other.PD_PowerStateMapping && self.PD_DeepestSystemWake == other.PD_DeepestSystemWake
    }
}
impl ::core::cmp::Eq for CM_POWER_DATA {}
unsafe impl ::windows::core::Abi for CM_POWER_DATA {
    type Abi = Self;
}
#[inline]
pub unsafe fn CallNtPowerInformation(informationlevel: POWER_INFORMATION_LEVEL, inputbuffer: *const ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNtPowerInformation(informationlevel: POWER_INFORMATION_LEVEL, inputbuffer: *const ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> i32;
        }
        ::core::mem::transmute(CallNtPowerInformation(::core::mem::transmute(informationlevel), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(inputbufferlength), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(outputbufferlength)))
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
        ::core::mem::transmute(CanUserWritePwrScheme())
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    pub Callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
    pub Context: *mut ::core::ffi::c_void,
}
impl DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {}
impl ::core::default::Default for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS").field("Context", &self.Context).finish()
    }
}
impl ::core::cmp::PartialEq for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {}
unsafe impl ::windows::core::Abi for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICE_POWER_STATE(pub i32);
pub const PowerDeviceUnspecified: DEVICE_POWER_STATE = DEVICE_POWER_STATE(0i32);
pub const PowerDeviceD0: DEVICE_POWER_STATE = DEVICE_POWER_STATE(1i32);
pub const PowerDeviceD1: DEVICE_POWER_STATE = DEVICE_POWER_STATE(2i32);
pub const PowerDeviceD2: DEVICE_POWER_STATE = DEVICE_POWER_STATE(3i32);
pub const PowerDeviceD3: DEVICE_POWER_STATE = DEVICE_POWER_STATE(4i32);
pub const PowerDeviceMaximum: DEVICE_POWER_STATE = DEVICE_POWER_STATE(5i32);
impl ::core::convert::From<i32> for DEVICE_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DEVICE_POWER_STATE {
    type Abi = Self;
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
        ::core::mem::transmute(DeletePwrScheme(::core::mem::transmute(uiid)))
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
        ::core::mem::transmute(DevicePowerClose())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerEnumDevices(queryindex: u32, queryinterpretationflags: u32, queryflags: u32, preturnbuffer: *mut u8, pbuffersize: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerEnumDevices(queryindex: u32, queryinterpretationflags: u32, queryflags: u32, preturnbuffer: *mut u8, pbuffersize: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(DevicePowerEnumDevices(::core::mem::transmute(queryindex), ::core::mem::transmute(queryinterpretationflags), ::core::mem::transmute(queryflags), ::core::mem::transmute(preturnbuffer), ::core::mem::transmute(pbuffersize)))
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
        ::core::mem::transmute(DevicePowerOpen(::core::mem::transmute(debugmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DevicePowerSetDeviceState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(devicedescription: Param0, setflags: u32, setdata: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevicePowerSetDeviceState(devicedescription: super::super::Foundation::PWSTR, setflags: u32, setdata: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(DevicePowerSetDeviceState(devicedescription.into_param().abi(), ::core::mem::transmute(setflags), ::core::mem::transmute(setdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EFFECTIVE_POWER_MODE(pub i32);
pub const EffectivePowerModeBatterySaver: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(0i32);
pub const EffectivePowerModeBetterBattery: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(1i32);
pub const EffectivePowerModeBalanced: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(2i32);
pub const EffectivePowerModeHighPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(3i32);
pub const EffectivePowerModeMaxPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(4i32);
pub const EffectivePowerModeGameMode: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(5i32);
pub const EffectivePowerModeMixedReality: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(6i32);
impl ::core::convert::From<i32> for EFFECTIVE_POWER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EFFECTIVE_POWER_MODE {
    type Abi = Self;
}
pub type EFFECTIVE_POWER_MODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(mode: EFFECTIVE_POWER_MODE, context: *const ::core::ffi::c_void)>;
pub const EFFECTIVE_POWER_MODE_V1: u32 = 1u32;
pub const EFFECTIVE_POWER_MODE_V2: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_CHANNEL_MEASUREMENT_DATA {
    pub AbsoluteEnergy: u64,
    pub AbsoluteTime: u64,
}
impl EMI_CHANNEL_MEASUREMENT_DATA {}
impl ::core::default::Default for EMI_CHANNEL_MEASUREMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_CHANNEL_MEASUREMENT_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_CHANNEL_MEASUREMENT_DATA").field("AbsoluteEnergy", &self.AbsoluteEnergy).field("AbsoluteTime", &self.AbsoluteTime).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_CHANNEL_MEASUREMENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteEnergy == other.AbsoluteEnergy && self.AbsoluteTime == other.AbsoluteTime
    }
}
impl ::core::cmp::Eq for EMI_CHANNEL_MEASUREMENT_DATA {}
unsafe impl ::windows::core::Abi for EMI_CHANNEL_MEASUREMENT_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_CHANNEL_V2 {
    pub MeasurementUnit: EMI_MEASUREMENT_UNIT,
    pub ChannelNameSize: u16,
    pub ChannelName: [u16; 1],
}
impl EMI_CHANNEL_V2 {}
impl ::core::default::Default for EMI_CHANNEL_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_CHANNEL_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_CHANNEL_V2").field("MeasurementUnit", &self.MeasurementUnit).field("ChannelNameSize", &self.ChannelNameSize).field("ChannelName", &self.ChannelName).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_CHANNEL_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit && self.ChannelNameSize == other.ChannelNameSize && self.ChannelName == other.ChannelName
    }
}
impl ::core::cmp::Eq for EMI_CHANNEL_V2 {}
unsafe impl ::windows::core::Abi for EMI_CHANNEL_V2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_MEASUREMENT_DATA_V2 {
    pub ChannelData: [EMI_CHANNEL_MEASUREMENT_DATA; 1],
}
impl EMI_MEASUREMENT_DATA_V2 {}
impl ::core::default::Default for EMI_MEASUREMENT_DATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_MEASUREMENT_DATA_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_MEASUREMENT_DATA_V2").field("ChannelData", &self.ChannelData).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_MEASUREMENT_DATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelData == other.ChannelData
    }
}
impl ::core::cmp::Eq for EMI_MEASUREMENT_DATA_V2 {}
unsafe impl ::windows::core::Abi for EMI_MEASUREMENT_DATA_V2 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EMI_MEASUREMENT_UNIT(pub i32);
pub const EmiMeasurementUnitPicowattHours: EMI_MEASUREMENT_UNIT = EMI_MEASUREMENT_UNIT(0i32);
impl ::core::convert::From<i32> for EMI_MEASUREMENT_UNIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EMI_MEASUREMENT_UNIT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_METADATA_SIZE {
    pub MetadataSize: u32,
}
impl EMI_METADATA_SIZE {}
impl ::core::default::Default for EMI_METADATA_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_METADATA_SIZE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_METADATA_SIZE").field("MetadataSize", &self.MetadataSize).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_METADATA_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataSize == other.MetadataSize
    }
}
impl ::core::cmp::Eq for EMI_METADATA_SIZE {}
unsafe impl ::windows::core::Abi for EMI_METADATA_SIZE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for EMI_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_METADATA_V1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for EMI_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit && self.HardwareOEM == other.HardwareOEM && self.HardwareModel == other.HardwareModel && self.HardwareRevision == other.HardwareRevision && self.MeteredHardwareNameSize == other.MeteredHardwareNameSize && self.MeteredHardwareName == other.MeteredHardwareName
    }
}
impl ::core::cmp::Eq for EMI_METADATA_V1 {}
unsafe impl ::windows::core::Abi for EMI_METADATA_V1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_METADATA_V2 {
    pub HardwareOEM: [u16; 16],
    pub HardwareModel: [u16; 16],
    pub HardwareRevision: u16,
    pub ChannelCount: u16,
    pub Channels: [EMI_CHANNEL_V2; 1],
}
impl EMI_METADATA_V2 {}
impl ::core::default::Default for EMI_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_METADATA_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_METADATA_V2").field("HardwareOEM", &self.HardwareOEM).field("HardwareModel", &self.HardwareModel).field("HardwareRevision", &self.HardwareRevision).field("ChannelCount", &self.ChannelCount).field("Channels", &self.Channels).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareOEM == other.HardwareOEM && self.HardwareModel == other.HardwareModel && self.HardwareRevision == other.HardwareRevision && self.ChannelCount == other.ChannelCount && self.Channels == other.Channels
    }
}
impl ::core::cmp::Eq for EMI_METADATA_V2 {}
unsafe impl ::windows::core::Abi for EMI_METADATA_V2 {
    type Abi = Self;
}
pub const EMI_NAME_MAX: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EMI_VERSION {
    pub EmiVersion: u16,
}
impl EMI_VERSION {}
impl ::core::default::Default for EMI_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EMI_VERSION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMI_VERSION").field("EmiVersion", &self.EmiVersion).finish()
    }
}
impl ::core::cmp::PartialEq for EMI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.EmiVersion == other.EmiVersion
    }
}
impl ::core::cmp::Eq for EMI_VERSION {}
unsafe impl ::windows::core::Abi for EMI_VERSION {
    type Abi = Self;
}
pub const EMI_VERSION_V1: u32 = 1u32;
pub const EMI_VERSION_V2: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EXECUTION_STATE(pub u32);
pub const ES_AWAYMODE_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(64u32);
pub const ES_CONTINUOUS: EXECUTION_STATE = EXECUTION_STATE(2147483648u32);
pub const ES_DISPLAY_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(2u32);
pub const ES_SYSTEM_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(1u32);
pub const ES_USER_PRESENT: EXECUTION_STATE = EXECUTION_STATE(4u32);
impl ::core::convert::From<u32> for EXECUTION_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EXECUTION_STATE {
    type Abi = Self;
}
impl ::core::ops::BitOr for EXECUTION_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for EXECUTION_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for EXECUTION_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for EXECUTION_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for EXECUTION_STATE {
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
pub unsafe fn EnumPwrSchemes<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(lpfn: PWRSCHEMESENUMPROC, lparam: Param1) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumPwrSchemes(lpfn: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(EnumPwrSchemes(::core::mem::transmute(lpfn), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub LidOpenWakeAc: SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: u32,
}
impl GLOBAL_MACHINE_POWER_POLICY {}
impl ::core::default::Default for GLOBAL_MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GLOBAL_MACHINE_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GLOBAL_MACHINE_POWER_POLICY").field("Revision", &self.Revision).field("LidOpenWakeAc", &self.LidOpenWakeAc).field("LidOpenWakeDc", &self.LidOpenWakeDc).field("BroadcastCapacityResolution", &self.BroadcastCapacityResolution).finish()
    }
}
impl ::core::cmp::PartialEq for GLOBAL_MACHINE_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.LidOpenWakeAc == other.LidOpenWakeAc && self.LidOpenWakeDc == other.LidOpenWakeDc && self.BroadcastCapacityResolution == other.BroadcastCapacityResolution
    }
}
impl ::core::cmp::Eq for GLOBAL_MACHINE_POWER_POLICY {}
unsafe impl ::windows::core::Abi for GLOBAL_MACHINE_POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl GLOBAL_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLOBAL_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLOBAL_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GLOBAL_POWER_POLICY").field("user", &self.user).field("mach", &self.mach).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLOBAL_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user && self.mach == other.mach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLOBAL_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GLOBAL_POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLOBAL_USER_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for GLOBAL_USER_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.PowerButtonAc == other.PowerButtonAc && self.PowerButtonDc == other.PowerButtonDc && self.SleepButtonAc == other.SleepButtonAc && self.SleepButtonDc == other.SleepButtonDc && self.LidCloseAc == other.LidCloseAc && self.LidCloseDc == other.LidCloseDc && self.DischargePolicy == other.DischargePolicy && self.GlobalFlags == other.GlobalFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLOBAL_USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GLOBAL_USER_POWER_POLICY {
    type Abi = Self;
}
pub const GUID_CLASS_INPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVICE_ACPI_TIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97f99bf6_4497_4f18_bb22_4b9fb2fbef9c);
pub const GUID_DEVICE_APPLICATIONLAUNCH_BUTTON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x629758ee_986e_4d9e_8e47_de27f8ab054d);
pub const GUID_DEVICE_BATTERY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72631e54_78a4_11d0_bcf7_00aa00b7b32a);
pub const GUID_DEVICE_ENERGY_METER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45bd8344_7ed6_49cf_a440_c276c933b053);
pub const GUID_DEVICE_FAN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05ecd13d_81da_4a2a_8a4c_524f23dd4dc9);
pub const GUID_DEVICE_LID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4afa3d52_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_MEMORY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd0f03d_92e0_45fb_b75c_5ed8ffb01021);
pub const GUID_DEVICE_MESSAGE_INDICATOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd48a365_fa94_4ce2_a232_a1b764e5d8b4);
pub const GUID_DEVICE_PROCESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97fadb10_4e33_40ae_359c_8bef029dbdd0);
pub const GUID_DEVICE_SYS_BUTTON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4afa3d53_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_THERMAL_ZONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4afa3d51_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVINTERFACE_THERMAL_COOLING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbe4373d_3c81_40cb_ace4_e0e5d05f0c9f);
pub const GUID_DEVINTERFACE_THERMAL_MANAGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x927ec093_69a4_4bc0_bd02_711664714463);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(GetActivePwrScheme(::core::mem::transmute(puiid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(GetCurrentPowerPolicies(::core::mem::transmute(pglobalpowerpolicy), ::core::mem::transmute(ppowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDevicePowerState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, pfon: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDevicePowerState(hdevice: super::super::Foundation::HANDLE, pfon: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetDevicePowerState(hdevice.into_param().abi(), ::core::mem::transmute(pfon)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPwrCapabilities(lpspc: *mut SYSTEM_POWER_CAPABILITIES) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPwrCapabilities(lpspc: *mut SYSTEM_POWER_CAPABILITIES) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(GetPwrCapabilities(::core::mem::transmute(lpspc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPwrDiskSpindownRange(puimax: *mut u32, puimin: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPwrDiskSpindownRange(puimax: *mut u32, puimin: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(GetPwrDiskSpindownRange(::core::mem::transmute(puimax), ::core::mem::transmute(puimin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemPowerStatus(lpsystempowerstatus: *mut SYSTEM_POWER_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemPowerStatus(lpsystempowerstatus: *mut SYSTEM_POWER_STATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemPowerStatus(::core::mem::transmute(lpsystempowerstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HPOWERNOTIFY(pub isize);
impl ::core::default::Default for HPOWERNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HPOWERNOTIFY {}
unsafe impl ::windows::core::Abi for HPOWERNOTIFY {
    type Abi = Self;
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
pub unsafe fn IsAdminOverrideActive(papp: *const ADMINISTRATOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAdminOverrideActive(papp: *const ADMINISTRATOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(IsAdminOverrideActive(::core::mem::transmute(papp)))
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
        ::core::mem::transmute(IsPwrHibernateAllowed())
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
        ::core::mem::transmute(IsPwrShutdownAllowed())
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
        ::core::mem::transmute(IsPwrSuspendAllowed())
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
        ::core::mem::transmute(IsSystemResumeAutomatic())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LATENCY_TIME(pub i32);
pub const LT_DONT_CARE: LATENCY_TIME = LATENCY_TIME(0i32);
pub const LT_LOWEST_LATENCY: LATENCY_TIME = LATENCY_TIME(1i32);
impl ::core::convert::From<i32> for LATENCY_TIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LATENCY_TIME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MACHINE_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for MACHINE_POWER_POLICY {
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
impl ::core::cmp::Eq for MACHINE_POWER_POLICY {}
unsafe impl ::windows::core::Abi for MACHINE_POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub ProcessorPolicyAc: PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: PROCESSOR_POWER_POLICY,
}
impl MACHINE_PROCESSOR_POWER_POLICY {}
impl ::core::default::Default for MACHINE_PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MACHINE_PROCESSOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MACHINE_PROCESSOR_POWER_POLICY").field("Revision", &self.Revision).field("ProcessorPolicyAc", &self.ProcessorPolicyAc).field("ProcessorPolicyDc", &self.ProcessorPolicyDc).finish()
    }
}
impl ::core::cmp::PartialEq for MACHINE_PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.ProcessorPolicyAc == other.ProcessorPolicyAc && self.ProcessorPolicyDc == other.ProcessorPolicyDc
    }
}
impl ::core::cmp::Eq for MACHINE_PROCESSOR_POWER_POLICY {}
unsafe impl ::windows::core::Abi for MACHINE_PROCESSOR_POWER_POLICY {
    type Abi = Self;
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
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, r#type: u32, setting: *const ::core::ffi::c_void) -> u32>;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct POWERBROADCAST_SETTING {
    pub PowerSetting: ::windows::core::GUID,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl POWERBROADCAST_SETTING {}
impl ::core::default::Default for POWERBROADCAST_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POWERBROADCAST_SETTING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POWERBROADCAST_SETTING").field("PowerSetting", &self.PowerSetting).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::core::cmp::PartialEq for POWERBROADCAST_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.PowerSetting == other.PowerSetting && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for POWERBROADCAST_SETTING {}
unsafe impl ::windows::core::Abi for POWERBROADCAST_SETTING {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for POWER_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_ACTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct POWER_ACTION_POLICY {
    pub Action: POWER_ACTION,
    pub Flags: u32,
    pub EventCode: POWER_ACTION_POLICY_EVENT_CODE,
}
impl POWER_ACTION_POLICY {}
impl ::core::default::Default for POWER_ACTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POWER_ACTION_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POWER_ACTION_POLICY").field("Action", &self.Action).field("Flags", &self.Flags).field("EventCode", &self.EventCode).finish()
    }
}
impl ::core::cmp::PartialEq for POWER_ACTION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.Flags == other.Flags && self.EventCode == other.EventCode
    }
}
impl ::core::cmp::Eq for POWER_ACTION_POLICY {}
unsafe impl ::windows::core::Abi for POWER_ACTION_POLICY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_ACTION_POLICY_EVENT_CODE(pub u32);
pub const POWER_FORCE_TRIGGER_RESET: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(2147483648u32);
pub const POWER_LEVEL_USER_NOTIFY_EXEC: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(4u32);
pub const POWER_LEVEL_USER_NOTIFY_SOUND: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(2u32);
pub const POWER_LEVEL_USER_NOTIFY_TEXT: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(1u32);
pub const POWER_USER_NOTIFY_BUTTON: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(8u32);
pub const POWER_USER_NOTIFY_SHUTDOWN: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(16u32);
impl ::core::convert::From<u32> for POWER_ACTION_POLICY_EVENT_CODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_ACTION_POLICY_EVENT_CODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const POWER_ATTRIBUTE_HIDE: u32 = 1u32;
pub const POWER_ATTRIBUTE_SHOW_AOAC: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_DATA_ACCESSOR(pub i32);
pub const ACCESS_AC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(0i32);
pub const ACCESS_DC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(1i32);
pub const ACCESS_FRIENDLY_NAME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(2i32);
pub const ACCESS_DESCRIPTION: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(3i32);
pub const ACCESS_POSSIBLE_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(4i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(5i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(6i32);
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
impl ::core::convert::From<i32> for POWER_DATA_ACCESSOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_DATA_ACCESSOR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_INFORMATION_LEVEL(pub i32);
pub const SystemPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(0i32);
pub const SystemPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(1i32);
pub const VerifySystemPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(2i32);
pub const VerifySystemPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(3i32);
pub const SystemPowerCapabilities: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(4i32);
pub const SystemBatteryState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(5i32);
pub const SystemPowerStateHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(6i32);
pub const ProcessorStateHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(7i32);
pub const SystemPowerPolicyCurrent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(8i32);
pub const AdministratorPowerPolicy: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(9i32);
pub const SystemReserveHiberFile: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(10i32);
pub const ProcessorInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(11i32);
pub const SystemPowerInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(12i32);
pub const ProcessorStateHandler2: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(13i32);
pub const LastWakeTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(14i32);
pub const LastSleepTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(15i32);
pub const SystemExecutionState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(16i32);
pub const SystemPowerStateNotifyHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(17i32);
pub const ProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(18i32);
pub const ProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(19i32);
pub const VerifyProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(20i32);
pub const VerifyProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(21i32);
pub const ProcessorPowerPolicyCurrent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(22i32);
pub const SystemPowerStateLogging: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(23i32);
pub const SystemPowerLoggingEntry: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(24i32);
pub const SetPowerSettingValue: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(25i32);
pub const NotifyUserPowerSetting: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(26i32);
pub const PowerInformationLevelUnused0: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(27i32);
pub const SystemMonitorHiberBootPowerOff: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(28i32);
pub const SystemVideoState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(29i32);
pub const TraceApplicationPowerMessage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(30i32);
pub const TraceApplicationPowerMessageEnd: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(31i32);
pub const ProcessorPerfStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(32i32);
pub const ProcessorIdleStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(33i32);
pub const ProcessorCap: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(34i32);
pub const SystemWakeSource: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(35i32);
pub const SystemHiberFileInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(36i32);
pub const TraceServicePowerMessage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(37i32);
pub const ProcessorLoad: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(38i32);
pub const PowerShutdownNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(39i32);
pub const MonitorCapabilities: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(40i32);
pub const SessionPowerInit: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(41i32);
pub const SessionDisplayState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(42i32);
pub const PowerRequestCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(43i32);
pub const PowerRequestAction: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(44i32);
pub const GetPowerRequestList: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(45i32);
pub const ProcessorInformationEx: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(46i32);
pub const NotifyUserModeLegacyPowerEvent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(47i32);
pub const GroupPark: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(48i32);
pub const ProcessorIdleDomains: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(49i32);
pub const WakeTimerList: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(50i32);
pub const SystemHiberFileSize: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(51i32);
pub const ProcessorIdleStatesHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(52i32);
pub const ProcessorPerfStatesHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(53i32);
pub const ProcessorPerfCapHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(54i32);
pub const ProcessorSetIdle: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(55i32);
pub const LogicalProcessorIdling: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(56i32);
pub const UserPresence: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(57i32);
pub const PowerSettingNotificationName: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(58i32);
pub const GetPowerSettingValue: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(59i32);
pub const IdleResiliency: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(60i32);
pub const SessionRITState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(61i32);
pub const SessionConnectNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(62i32);
pub const SessionPowerCleanup: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(63i32);
pub const SessionLockState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(64i32);
pub const SystemHiberbootState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(65i32);
pub const PlatformInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(66i32);
pub const PdcInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(67i32);
pub const MonitorInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(68i32);
pub const FirmwareTableInformationRegistered: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(69i32);
pub const SetShutdownSelectedTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(70i32);
pub const SuspendResumeInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(71i32);
pub const PlmPowerRequestCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(72i32);
pub const ScreenOff: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(73i32);
pub const CsDeviceNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(74i32);
pub const PlatformRole: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(75i32);
pub const LastResumePerformance: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(76i32);
pub const DisplayBurst: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(77i32);
pub const ExitLatencySamplingPercentage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(78i32);
pub const RegisterSpmPowerSettings: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(79i32);
pub const PlatformIdleStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(80i32);
pub const ProcessorIdleVeto: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(81i32);
pub const PlatformIdleVeto: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(82i32);
pub const SystemBatteryStatePrecise: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(83i32);
pub const ThermalEvent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(84i32);
pub const PowerRequestActionInternal: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(85i32);
pub const BatteryDeviceState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(86i32);
pub const PowerInformationInternal: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(87i32);
pub const ThermalStandby: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(88i32);
pub const SystemHiberFileType: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(89i32);
pub const PhysicalPowerButtonPress: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(90i32);
pub const QueryPotentialDripsConstraint: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(91i32);
pub const EnergyTrackerCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(92i32);
pub const EnergyTrackerQuery: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(93i32);
pub const UpdateBlackBoxRecorder: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(94i32);
pub const SessionAllowExternalDmaDevices: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(95i32);
pub const SendSuspendResumeNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(96i32);
pub const PowerInformationLevelMaximum: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(97i32);
impl ::core::convert::From<i32> for POWER_INFORMATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_INFORMATION_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for POWER_PLATFORM_ROLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_PLATFORM_ROLE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_PLATFORM_ROLE_VERSION(pub u32);
pub const POWER_PLATFORM_ROLE_V1: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(1u32);
pub const POWER_PLATFORM_ROLE_V2: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(2u32);
impl ::core::convert::From<u32> for POWER_PLATFORM_ROLE_VERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_PLATFORM_ROLE_VERSION {
    type Abi = Self;
}
impl ::core::ops::BitOr for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for POWER_PLATFORM_ROLE_VERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for POWER_PLATFORM_ROLE_VERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for POWER_PLATFORM_ROLE_VERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_POLICY {
    pub user: USER_POWER_POLICY,
    pub mach: MACHINE_POWER_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POWER_POLICY").field("user", &self.user).field("mach", &self.mach).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user && self.mach == other.mach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_REQUEST_TYPE(pub i32);
pub const PowerRequestDisplayRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(0i32);
pub const PowerRequestSystemRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(1i32);
pub const PowerRequestAwayModeRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(2i32);
pub const PowerRequestExecutionRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(3i32);
impl ::core::convert::From<i32> for POWER_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_REQUEST_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(pub u32);
pub const DEVICE_NOTIFY_SERVICE_HANDLE: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS = POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(1u32);
pub const DEVICE_NOTIFY_CALLBACK: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS = POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(2u32);
pub const DEVICE_NOTIFY_WINDOW_HANDLE: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS = POWER_SETTING_REGISTER_NOTIFICATION_FLAGS(0u32);
impl ::core::convert::From<u32> for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PROCESSOR_NUMBER_PKEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5724c81d_d5af_4c1f_a103_a06e28f204c6), pid: 1u32 };
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_OBJECT_INFO {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
}
impl PROCESSOR_OBJECT_INFO {}
impl ::core::default::Default for PROCESSOR_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_OBJECT_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROCESSOR_OBJECT_INFO").field("PhysicalID", &self.PhysicalID).field("PBlkAddress", &self.PBlkAddress).field("PBlkLength", &self.PBlkLength).finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID && self.PBlkAddress == other.PBlkAddress && self.PBlkLength == other.PBlkLength
    }
}
impl ::core::cmp::Eq for PROCESSOR_OBJECT_INFO {}
unsafe impl ::windows::core::Abi for PROCESSOR_OBJECT_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_OBJECT_INFO_EX {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
    pub InitialApicId: u32,
}
impl PROCESSOR_OBJECT_INFO_EX {}
impl ::core::default::Default for PROCESSOR_OBJECT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_OBJECT_INFO_EX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROCESSOR_OBJECT_INFO_EX").field("PhysicalID", &self.PhysicalID).field("PBlkAddress", &self.PBlkAddress).field("PBlkLength", &self.PBlkLength).field("InitialApicId", &self.InitialApicId).finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_OBJECT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID && self.PBlkAddress == other.PBlkAddress && self.PBlkLength == other.PBlkLength && self.InitialApicId == other.InitialApicId
    }
}
impl ::core::cmp::Eq for PROCESSOR_OBJECT_INFO_EX {}
unsafe impl ::windows::core::Abi for PROCESSOR_OBJECT_INFO_EX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROCESSOR_POWER_POLICY").field("Revision", &self.Revision).field("DynamicThrottle", &self.DynamicThrottle).field("Spare", &self.Spare).field("_bitfield", &self._bitfield).field("PolicyCount", &self.PolicyCount).field("Policy", &self.Policy).finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.DynamicThrottle == other.DynamicThrottle && self.Spare == other.Spare && self._bitfield == other._bitfield && self.PolicyCount == other.PolicyCount && self.Policy == other.Policy
    }
}
impl ::core::cmp::Eq for PROCESSOR_POWER_POLICY {}
unsafe impl ::windows::core::Abi for PROCESSOR_POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for PROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_POWER_POLICY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for PROCESSOR_POWER_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TimeCheck == other.TimeCheck && self.DemoteLimit == other.DemoteLimit && self.PromoteLimit == other.PromoteLimit && self.DemotePercent == other.DemotePercent && self.PromotePercent == other.PromotePercent && self.Spare == other.Spare && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for PROCESSOR_POWER_POLICY_INFO {}
unsafe impl ::windows::core::Abi for PROCESSOR_POWER_POLICY_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PWRSCHEMESENUMPROC = ::core::option::Option<unsafe extern "system" fn(index: u32, namesize: u32, name: super::super::Foundation::PWSTR, descriptionsize: u32, description: super::super::Foundation::PWSTR, policy: *const POWER_POLICY, context: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN>;
#[cfg(feature = "Win32_Foundation")]
pub type PWRSCHEMESENUMPROC_V1 = ::core::option::Option<unsafe extern "system" fn(index: u32, namesize: u32, name: *const i8, descriptionsize: u32, description: *const i8, policy: *const POWER_POLICY, context: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN>;
#[inline]
pub unsafe fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerCanRestoreIndividualDefaultPowerScheme(::core::mem::transmute(schemeguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerClearRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(powerrequest: Param0, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerClearRequest(powerrequest: super::super::Foundation::HANDLE, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PowerClearRequest(powerrequest.into_param().abi(), ::core::mem::transmute(requesttype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreatePossibleSetting<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootsystempowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreatePossibleSetting(rootsystempowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32) -> u32;
        }
        ::core::mem::transmute(PowerCreatePossibleSetting(rootsystempowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(possiblesettingindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn PowerCreateRequest(context: *const super::Threading::REASON_CONTEXT) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreateRequest(context: *const super::Threading::REASON_CONTEXT) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(PowerCreateRequest(::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreateSetting<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootsystempowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerCreateSetting(rootsystempowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerCreateSetting(rootsystempowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDeleteScheme<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDeleteScheme(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerDeleteScheme(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid)))
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
        ::core::mem::transmute(PowerDeterminePlatformRole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerDeterminePlatformRoleEx(version: POWER_PLATFORM_ROLE_VERSION) -> POWER_PLATFORM_ROLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDeterminePlatformRoleEx(version: POWER_PLATFORM_ROLE_VERSION) -> POWER_PLATFORM_ROLE;
        }
        ::core::mem::transmute(PowerDeterminePlatformRoleEx(::core::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDuplicateScheme<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, sourceschemeguid: *const ::windows::core::GUID, destinationschemeguid: *mut *mut ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerDuplicateScheme(rootpowerkey: super::Registry::HKEY, sourceschemeguid: *const ::windows::core::GUID, destinationschemeguid: *mut *mut ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerDuplicateScheme(rootpowerkey.into_param().abi(), ::core::mem::transmute(sourceschemeguid), ::core::mem::transmute(destinationschemeguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerEnumerate<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, accessflags: POWER_DATA_ACCESSOR, index: u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerEnumerate(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, accessflags: POWER_DATA_ACCESSOR, index: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerEnumerate(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(accessflags), ::core::mem::transmute(index), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerGetActiveScheme<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(userrootpowerkey: Param0, activepolicyguid: *mut *mut ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerGetActiveScheme(userrootpowerkey: super::Registry::HKEY, activepolicyguid: *mut *mut ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerGetActiveScheme(userrootpowerkey.into_param().abi(), ::core::mem::transmute(activepolicyguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerImportPowerScheme<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(rootpowerkey: Param0, importfilenamepath: Param1, destinationschemeguid: *mut *mut ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerImportPowerScheme(rootpowerkey: super::Registry::HKEY, importfilenamepath: super::super::Foundation::PWSTR, destinationschemeguid: *mut *mut ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerImportPowerScheme(rootpowerkey.into_param().abi(), importfilenamepath.into_param().abi(), ::core::mem::transmute(destinationschemeguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerIsSettingRangeDefined(subkeyguid: *const ::windows::core::GUID, settingguid: *const ::windows::core::GUID) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerIsSettingRangeDefined(subkeyguid: *const ::windows::core::GUID, settingguid: *const ::windows::core::GUID) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(PowerIsSettingRangeDefined(::core::mem::transmute(subkeyguid), ::core::mem::transmute(settingguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerOpenSystemPowerKey<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(phsystempowerkey: *mut super::Registry::HKEY, access: u32, openexisting: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerOpenSystemPowerKey(phsystempowerkey: *mut super::Registry::HKEY, access: u32, openexisting: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(PowerOpenSystemPowerKey(::core::mem::transmute(phsystempowerkey), ::core::mem::transmute(access), openexisting.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PowerOpenUserPowerKey<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(phuserpowerkey: *mut super::Registry::HKEY, access: u32, openexisting: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerOpenUserPowerKey(phuserpowerkey: *mut super::Registry::HKEY, access: u32, openexisting: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(PowerOpenUserPowerKey(::core::mem::transmute(phuserpowerkey), ::core::mem::transmute(access), openexisting.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACDefaultIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acdefaultindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACDefaultIndex(rootpowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acdefaultindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadACDefaultIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemepersonalityguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(acdefaultindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValue<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACValue(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadACValue(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(r#type), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValueIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acvalueindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadACValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acvalueindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadACValueIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(acvalueindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCDefaultIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcdefaultindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCDefaultIndex(rootpowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcdefaultindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadDCDefaultIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemepersonalityguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(dcdefaultindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValue<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCValue(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadDCValue(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(r#type), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValueIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcvalueindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDCValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcvalueindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadDCValueIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(dcvalueindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDescription<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadDescription(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadDescription(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadFriendlyName(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadFriendlyName(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadIconResourceSpecifier<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadIconResourceSpecifier(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadIconResourceSpecifier(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleDescription<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleDescription(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadPossibleDescription(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleFriendlyName(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadPossibleFriendlyName(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleValue<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadPossibleValue(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: *mut u32, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadPossibleValue(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(r#type), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerReadSettingAttributes(subgroupguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadSettingAttributes(subgroupguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerReadSettingAttributes(::core::mem::transmute(subgroupguid), ::core::mem::transmute(powersettingguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueIncrement<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueincrement: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueIncrement(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueincrement: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadValueIncrement(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valueincrement)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMax<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valuemaximum: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueMax(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valuemaximum: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadValueMax(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valuemaximum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMin<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueminimum: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueMin(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueminimum: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadValueMin(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valueminimum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueUnitsSpecifier<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerReadValueUnitsSpecifier(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(PowerReadValueUnitsSpecifier(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRegisterForEffectivePowerModeNotifications(version: u32, callback: EFFECTIVE_POWER_MODE_CALLBACK, context: *const ::core::ffi::c_void, registrationhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRegisterForEffectivePowerModeNotifications(version: u32, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registrationhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PowerRegisterForEffectivePowerModeNotifications(::core::mem::transmute(version), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(registrationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerRegisterSuspendResumeNotification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(flags: u32, recipient: Param1, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRegisterSuspendResumeNotification(flags: u32, recipient: super::super::Foundation::HANDLE, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PowerRegisterSuspendResumeNotification(::core::mem::transmute(flags), recipient.into_param().abi(), ::core::mem::transmute(registrationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRemovePowerSetting(powersettingsubkeyguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRemovePowerSetting(powersettingsubkeyguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerRemovePowerSetting(::core::mem::transmute(powersettingsubkeyguid), ::core::mem::transmute(powersettingguid)))
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
        ::core::mem::transmute(PowerReplaceDefaultPowerSchemes())
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
        ::core::mem::transmute(PowerReportThermalEvent(::core::mem::transmute(event)))
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
        ::core::mem::transmute(PowerRestoreDefaultPowerSchemes())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerRestoreIndividualDefaultPowerScheme(::core::mem::transmute(schemeguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSetActiveScheme<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(userrootpowerkey: Param0, schemeguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSetActiveScheme(userrootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerSetActiveScheme(userrootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerSetRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(powerrequest: Param0, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSetRequest(powerrequest: super::super::Foundation::HANDLE, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PowerSetRequest(powerrequest.into_param().abi(), ::core::mem::transmute(requesttype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerSettingAccessCheck(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingAccessCheck(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(PowerSettingAccessCheck(::core::mem::transmute(accessflags), ::core::mem::transmute(powerguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSettingAccessCheckEx(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::core::GUID, accesstype: super::Registry::REG_SAM_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingAccessCheckEx(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::core::GUID, accesstype: super::Registry::REG_SAM_FLAGS) -> u32;
        }
        ::core::mem::transmute(PowerSettingAccessCheckEx(::core::mem::transmute(accessflags), ::core::mem::transmute(powerguid), ::core::mem::transmute(accesstype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PowerSettingRegisterNotification<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(settingguid: *const ::windows::core::GUID, flags: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS, recipient: Param2, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingRegisterNotification(settingguid: *const ::windows::core::GUID, flags: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS, recipient: super::super::Foundation::HANDLE, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PowerSettingRegisterNotification(::core::mem::transmute(settingguid), ::core::mem::transmute(flags), recipient.into_param().abi(), ::core::mem::transmute(registrationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerSettingUnregisterNotification<'a, Param0: ::windows::core::IntoParam<'a, HPOWERNOTIFY>>(registrationhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerSettingUnregisterNotification(registrationhandle: HPOWERNOTIFY) -> u32;
        }
        ::core::mem::transmute(PowerSettingUnregisterNotification(registrationhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PowerUnregisterFromEffectivePowerModeNotifications(::core::mem::transmute(registrationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerUnregisterSuspendResumeNotification<'a, Param0: ::windows::core::IntoParam<'a, HPOWERNOTIFY>>(registrationhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerUnregisterSuspendResumeNotification(registrationhandle: HPOWERNOTIFY) -> u32;
        }
        ::core::mem::transmute(PowerUnregisterSuspendResumeNotification(registrationhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACDefaultIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootsystempowerkey: Param0, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, defaultacindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteACDefaultIndex(rootsystempowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, defaultacindex: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteACDefaultIndex(rootsystempowerkey.into_param().abi(), ::core::mem::transmute(schemepersonalityguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(defaultacindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACValueIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acvalueindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteACValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, acvalueindex: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteACValueIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(acvalueindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCDefaultIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootsystempowerkey: Param0, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, defaultdcindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDCDefaultIndex(rootsystempowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, defaultdcindex: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteDCDefaultIndex(rootsystempowerkey.into_param().abi(), ::core::mem::transmute(schemepersonalityguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(defaultdcindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCValueIndex<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcvalueindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDCValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, dcvalueindex: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteDCValueIndex(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(dcvalueindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDescription<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteDescription(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteDescription(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteFriendlyName(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteFriendlyName(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteIconResourceSpecifier<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteIconResourceSpecifier(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::core::GUID, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteIconResourceSpecifier(rootpowerkey.into_param().abi(), ::core::mem::transmute(schemeguid), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleDescription<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleDescription(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWritePossibleDescription(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleFriendlyName(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWritePossibleFriendlyName(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleValue<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: u32, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWritePossibleValue(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, r#type: u32, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWritePossibleValue(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(r#type), ::core::mem::transmute(possiblesettingindex), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PowerWriteSettingAttributes(subgroupguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, attributes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteSettingAttributes(subgroupguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, attributes: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteSettingAttributes(::core::mem::transmute(subgroupguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(attributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueIncrement<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueincrement: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueIncrement(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueincrement: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteValueIncrement(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valueincrement)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMax<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valuemaximum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueMax(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valuemaximum: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteValueMax(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valuemaximum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMin<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueminimum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueMin(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, valueminimum: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteValueMin(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(valueminimum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueUnitsSpecifier<'a, Param0: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(rootpowerkey: Param0, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PowerWriteValueUnitsSpecifier(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::core::GUID, powersettingguid: *const ::windows::core::GUID, buffer: *const u8, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(PowerWriteValueUnitsSpecifier(rootpowerkey.into_param().abi(), ::core::mem::transmute(subgroupofpowersettingsguid), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ReadGlobalPwrPolicy(::core::mem::transmute(pglobalpowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ReadProcessorPwrScheme(::core::mem::transmute(uiid), ::core::mem::transmute(pmachineprocessorpowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadPwrScheme(uiid: u32, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadPwrScheme(uiid: u32, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ReadPwrScheme(::core::mem::transmute(uiid), ::core::mem::transmute(ppowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterPowerSettingNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hrecipient: Param0, powersettingguid: *const ::windows::core::GUID, flags: u32) -> HPOWERNOTIFY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterPowerSettingNotification(hrecipient: super::super::Foundation::HANDLE, powersettingguid: *const ::windows::core::GUID, flags: u32) -> HPOWERNOTIFY;
        }
        ::core::mem::transmute(RegisterPowerSettingNotification(hrecipient.into_param().abi(), ::core::mem::transmute(powersettingguid), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterSuspendResumeNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hrecipient: Param0, flags: u32) -> HPOWERNOTIFY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterSuspendResumeNotification(hrecipient: super::super::Foundation::HANDLE, flags: u32) -> HPOWERNOTIFY;
        }
        ::core::mem::transmute(RegisterSuspendResumeNotification(hrecipient.into_param().abi(), ::core::mem::transmute(flags)))
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
        ::core::mem::transmute(RequestWakeupLatency(::core::mem::transmute(latency)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SET_POWER_SETTING_VALUE {
    pub Version: u32,
    pub Guid: ::windows::core::GUID,
    pub PowerCondition: SYSTEM_POWER_CONDITION,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl SET_POWER_SETTING_VALUE {}
impl ::core::default::Default for SET_POWER_SETTING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SET_POWER_SETTING_VALUE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SET_POWER_SETTING_VALUE").field("Version", &self.Version).field("Guid", &self.Guid).field("PowerCondition", &self.PowerCondition).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::core::cmp::PartialEq for SET_POWER_SETTING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Guid == other.Guid && self.PowerCondition == other.PowerCondition && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for SET_POWER_SETTING_VALUE {}
unsafe impl ::windows::core::Abi for SET_POWER_SETTING_VALUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for SYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_BATTERY_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SYSTEM_BATTERY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.AcOnLine == other.AcOnLine && self.BatteryPresent == other.BatteryPresent && self.Charging == other.Charging && self.Discharging == other.Discharging && self.Spare1 == other.Spare1 && self.Tag == other.Tag && self.MaxCapacity == other.MaxCapacity && self.RemainingCapacity == other.RemainingCapacity && self.Rate == other.Rate && self.EstimatedTime == other.EstimatedTime && self.DefaultAlert1 == other.DefaultAlert1 && self.DefaultAlert2 == other.DefaultAlert2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_BATTERY_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_BATTERY_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for SYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POWER_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SYSTEM_POWER_CAPABILITIES {
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
impl ::core::cmp::Eq for SYSTEM_POWER_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_POWER_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYSTEM_POWER_CONDITION(pub i32);
pub const PoAc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(0i32);
pub const PoDc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(1i32);
pub const PoHot: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(2i32);
pub const PoConditionMaximum: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(3i32);
impl ::core::convert::From<i32> for SYSTEM_POWER_CONDITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_POWER_CONDITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for SYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POWER_LEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SYSTEM_POWER_LEVEL").field("Enable", &self.Enable).field("Spare", &self.Spare).field("BatteryLevel", &self.BatteryLevel).field("PowerPolicy", &self.PowerPolicy).field("MinSystemState", &self.MinSystemState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_POWER_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Spare == other.Spare && self.BatteryLevel == other.BatteryLevel && self.PowerPolicy == other.PowerPolicy && self.MinSystemState == other.MinSystemState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_POWER_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_POWER_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for SYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
            .field("BroadcastCapacityResolution", &self.BroadcastCapacityResolution)
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
impl ::core::cmp::PartialEq for SYSTEM_POWER_POLICY {
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
impl ::core::cmp::Eq for SYSTEM_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_POWER_POLICY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for SYSTEM_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_POWER_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for SYSTEM_POWER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_POWER_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SYSTEM_POWER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ACLineStatus == other.ACLineStatus && self.BatteryFlag == other.BatteryFlag && self.BatteryLifePercent == other.BatteryLifePercent && self.SystemStatusFlag == other.SystemStatusFlag && self.BatteryLifeTime == other.BatteryLifeTime && self.BatteryFullLifeTime == other.BatteryFullLifeTime
    }
}
impl ::core::cmp::Eq for SYSTEM_POWER_STATUS {}
unsafe impl ::windows::core::Abi for SYSTEM_POWER_STATUS {
    type Abi = Self;
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
pub unsafe fn SetActivePwrScheme(uiid: u32, pglobalpowerpolicy: *const GLOBAL_POWER_POLICY, ppowerpolicy: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetActivePwrScheme(uiid: u32, pglobalpowerpolicy: *const GLOBAL_POWER_POLICY, ppowerpolicy: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(SetActivePwrScheme(::core::mem::transmute(uiid), ::core::mem::transmute(pglobalpowerpolicy), ::core::mem::transmute(ppowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSuspendState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(bhibernate: Param0, bforce: Param1, bwakeupeventsdisabled: Param2) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSuspendState(bhibernate: super::super::Foundation::BOOLEAN, bforce: super::super::Foundation::BOOLEAN, bwakeupeventsdisabled: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(SetSuspendState(bhibernate.into_param().abi(), bforce.into_param().abi(), bwakeupeventsdisabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemPowerState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fsuspend: Param0, fforce: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemPowerState(fsuspend: super::super::Foundation::BOOL, fforce: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetSystemPowerState(fsuspend.into_param().abi(), fforce.into_param().abi()))
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
        ::core::mem::transmute(SetThreadExecutionState(::core::mem::transmute(esflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const THERMAL_COOLING_INTERFACE_VERSION: u32 = 1u32;
pub const THERMAL_DEVICE_INTERFACE_VERSION: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for THERMAL_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for THERMAL_EVENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("THERMAL_EVENT").field("Version", &self.Version).field("Size", &self.Size).field("Type", &self.Type).field("Temperature", &self.Temperature).field("TripPointTemperature", &self.TripPointTemperature).field("Initiator", &self.Initiator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for THERMAL_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Type == other.Type && self.Temperature == other.Temperature && self.TripPointTemperature == other.TripPointTemperature && self.Initiator == other.Initiator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for THERMAL_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for THERMAL_EVENT {
    type Abi = Self;
}
pub const THERMAL_EVENT_VERSION: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for THERMAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for THERMAL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for THERMAL_INFORMATION {
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
impl ::core::cmp::Eq for THERMAL_INFORMATION {}
unsafe impl ::windows::core::Abi for THERMAL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for THERMAL_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for THERMAL_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for THERMAL_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.WaitForUpdate == other.WaitForUpdate && self.Hibernate == other.Hibernate && self.Critical == other.Critical && self.ThermalStandby == other.ThermalStandby && self.ActivationReasons == other.ActivationReasons && self.PassiveLimit == other.PassiveLimit && self.ActiveLevel == other.ActiveLevel && self.OverThrottled == other.OverThrottled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for THERMAL_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for THERMAL_POLICY {
    type Abi = Self;
}
pub const THERMAL_POLICY_VERSION_1: u32 = 1u32;
pub const THERMAL_POLICY_VERSION_2: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct THERMAL_WAIT_READ {
    pub Timeout: u32,
    pub LowTemperature: u32,
    pub HighTemperature: u32,
}
impl THERMAL_WAIT_READ {}
impl ::core::default::Default for THERMAL_WAIT_READ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for THERMAL_WAIT_READ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("THERMAL_WAIT_READ").field("Timeout", &self.Timeout).field("LowTemperature", &self.LowTemperature).field("HighTemperature", &self.HighTemperature).finish()
    }
}
impl ::core::cmp::PartialEq for THERMAL_WAIT_READ {
    fn eq(&self, other: &Self) -> bool {
        self.Timeout == other.Timeout && self.LowTemperature == other.LowTemperature && self.HighTemperature == other.HighTemperature
    }
}
impl ::core::cmp::Eq for THERMAL_WAIT_READ {}
unsafe impl ::windows::core::Abi for THERMAL_WAIT_READ {
    type Abi = Self;
}
pub const TZ_ACTIVATION_REASON_CURRENT: u32 = 2u32;
pub const TZ_ACTIVATION_REASON_THERMAL: u32 = 1u32;
pub const UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const UNKNOWN_RATE: u32 = 2147483648u32;
pub const UNKNOWN_VOLTAGE: u32 = 4294967295u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct USB_CHARGER_PORT(pub i32);
pub const UsbChargerPort_Legacy: USB_CHARGER_PORT = USB_CHARGER_PORT(0i32);
pub const UsbChargerPort_TypeC: USB_CHARGER_PORT = USB_CHARGER_PORT(1i32);
pub const UsbChargerPort_Max: USB_CHARGER_PORT = USB_CHARGER_PORT(2i32);
impl ::core::convert::From<i32> for USB_CHARGER_PORT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for USB_CHARGER_PORT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_POWER_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for USER_POWER_POLICY {
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
impl ::core::cmp::Eq for USER_POWER_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_POWER_POLICY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterPowerSettingNotification<'a, Param0: ::windows::core::IntoParam<'a, HPOWERNOTIFY>>(handle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterPowerSettingNotification(handle: HPOWERNOTIFY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterPowerSettingNotification(handle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterSuspendResumeNotification<'a, Param0: ::windows::core::IntoParam<'a, HPOWERNOTIFY>>(handle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterSuspendResumeNotification(handle: HPOWERNOTIFY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterSuspendResumeNotification(handle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidatePowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidatePowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ValidatePowerPolicies(::core::mem::transmute(pglobalpowerpolicy), ::core::mem::transmute(ppowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WAKE_ALARM_INFORMATION {
    pub TimerIdentifier: u32,
    pub Timeout: u32,
}
impl WAKE_ALARM_INFORMATION {}
impl ::core::default::Default for WAKE_ALARM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WAKE_ALARM_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WAKE_ALARM_INFORMATION").field("TimerIdentifier", &self.TimerIdentifier).field("Timeout", &self.Timeout).finish()
    }
}
impl ::core::cmp::PartialEq for WAKE_ALARM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TimerIdentifier == other.TimerIdentifier && self.Timeout == other.Timeout
    }
}
impl ::core::cmp::Eq for WAKE_ALARM_INFORMATION {}
unsafe impl ::windows::core::Abi for WAKE_ALARM_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(WriteGlobalPwrPolicy(::core::mem::transmute(pglobalpowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(WriteProcessorPwrScheme(::core::mem::transmute(uiid), ::core::mem::transmute(pmachineprocessorpowerpolicy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePwrScheme<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(puiid: *const u32, lpszschemename: Param1, lpszdescription: Param2, lpscheme: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePwrScheme(puiid: *const u32, lpszschemename: super::super::Foundation::PWSTR, lpszdescription: super::super::Foundation::PWSTR, lpscheme: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(WritePwrScheme(::core::mem::transmute(puiid), lpszschemename.into_param().abi(), lpszdescription.into_param().abi(), ::core::mem::transmute(lpscheme)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
