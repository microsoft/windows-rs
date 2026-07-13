#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for ACPI_REAL_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACPI_TIME_ADJUST_DAYLIGHT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACPI_TIME_AND_ALARM_CAPABILITIES {
    pub AcWakeSupported: bool,
    pub DcWakeSupported: bool,
    pub S4AcWakeSupported: bool,
    pub S4DcWakeSupported: bool,
    pub S5AcWakeSupported: bool,
    pub S5DcWakeSupported: bool,
    pub S4S5WakeStatusSupported: bool,
    pub DeepestWakeSystemState: u32,
    pub RealTimeFeaturesSupported: bool,
    pub RealTimeResolution: ACPI_TIME_RESOLUTION,
}
pub const ACPI_TIME_IN_DAYLIGHT: u32 = 2;
pub type ACPI_TIME_RESOLUTION = i32;
pub const ACPI_TIME_ZONE_UNKNOWN: u32 = 2047;
pub const ACTIVE_COOLING: u32 = 0;
pub const AcpiTimeResolutionMax: ACPI_TIME_RESOLUTION = 2;
pub const AcpiTimeResolutionMilliseconds: ACPI_TIME_RESOLUTION = 0;
pub const AcpiTimeResolutionSeconds: ACPI_TIME_RESOLUTION = 1;
pub const BATTERY_CAPACITY_RELATIVE: u32 = 1073741824;
pub type BATTERY_CHARGER_ID = windows_core::GUID;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BATTERY_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub VaData: [u32; 1],
}
impl Default for BATTERY_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BATTERY_CHARGING: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_CHARGING_SOURCE {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub MaxCurrent: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_CHARGING_SOURCE_INFORMATION {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub SourceOnline: bool,
}
pub type BATTERY_CHARGING_SOURCE_TYPE = i32;
pub const BATTERY_CRITICAL: u32 = 8;
pub const BATTERY_DISCHARGING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BATTERY_IS_SHORT_TERM: u32 = 536870912;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_MANUFACTURE_DATE {
    pub Day: u8,
    pub Month: u8,
    pub Year: u16,
}
pub const BATTERY_POWER_ON_LINE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_QUERY_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_QUERY_INFORMATION_LEVEL,
    pub AtRate: u32,
}
pub type BATTERY_QUERY_INFORMATION_LEVEL = i32;
pub const BATTERY_SEALED: u32 = 268435456;
pub const BATTERY_SET_CHARGER_ID_SUPPORTED: u32 = 8;
pub const BATTERY_SET_CHARGE_SUPPORTED: u32 = 1;
pub const BATTERY_SET_CHARGINGSOURCE_SUPPORTED: u32 = 4;
pub const BATTERY_SET_DISCHARGE_SUPPORTED: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BATTERY_SET_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_SET_INFORMATION_LEVEL,
    pub Buffer: [u8; 1],
}
impl Default for BATTERY_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BATTERY_SET_INFORMATION_LEVEL = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_STATUS {
    pub PowerState: u32,
    pub Capacity: u32,
    pub Voltage: u32,
    pub Rate: i32,
}
pub const BATTERY_SYSTEM_BATTERY: u32 = 2147483648;
pub const BATTERY_TAG_INVALID: u32 = 0;
pub const BATTERY_TEST_EXEMPT: u32 = 16;
pub const BATTERY_UNKNOWN_CAPACITY: u32 = 4294967295;
pub const BATTERY_UNKNOWN_CURRENT: u32 = 4294967295;
pub const BATTERY_UNKNOWN_RATE: u32 = 2147483648;
pub const BATTERY_UNKNOWN_TIME: u32 = 4294967295;
pub const BATTERY_UNKNOWN_VOLTAGE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BATTERY_USB_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub Reserved: u32,
    pub Flags: u32,
    pub MaxCurrent: u32,
    pub Voltage: u32,
    pub PortType: USB_CHARGER_PORT,
    pub PortId: u64,
    pub PowerSourceInformation: *mut core::ffi::c_void,
    pub OemCharger: BATTERY_CHARGER_ID,
}
impl Default for BATTERY_USB_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BATTERY_USB_CHARGER_STATUS_FN_DEFAULT_USB: u32 = 1;
pub const BATTERY_USB_CHARGER_STATUS_UCM_PD: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_WAIT_STATUS {
    pub BatteryTag: u32,
    pub Timeout: u32,
    pub PowerState: u32,
    pub LowCapacity: u32,
    pub HighCapacity: u32,
}
pub const BatteryCharge: BATTERY_SET_INFORMATION_LEVEL = 1;
pub const BatteryChargerId: BATTERY_SET_INFORMATION_LEVEL = 4;
pub const BatteryChargerStatus: BATTERY_SET_INFORMATION_LEVEL = 5;
pub const BatteryChargingSource: BATTERY_SET_INFORMATION_LEVEL = 3;
pub const BatteryChargingSourceType_AC: BATTERY_CHARGING_SOURCE_TYPE = 1;
pub const BatteryChargingSourceType_Max: BATTERY_CHARGING_SOURCE_TYPE = 4;
pub const BatteryChargingSourceType_USB: BATTERY_CHARGING_SOURCE_TYPE = 2;
pub const BatteryChargingSourceType_Wireless: BATTERY_CHARGING_SOURCE_TYPE = 3;
pub const BatteryCriticalBias: BATTERY_SET_INFORMATION_LEVEL = 0;
pub const BatteryDeviceName: BATTERY_QUERY_INFORMATION_LEVEL = 4;
pub const BatteryDischarge: BATTERY_SET_INFORMATION_LEVEL = 2;
pub const BatteryEstimatedTime: BATTERY_QUERY_INFORMATION_LEVEL = 3;
pub const BatteryGranularityInformation: BATTERY_QUERY_INFORMATION_LEVEL = 1;
pub const BatteryInformation: BATTERY_QUERY_INFORMATION_LEVEL = 0;
pub const BatteryManufactureDate: BATTERY_QUERY_INFORMATION_LEVEL = 5;
pub const BatteryManufactureName: BATTERY_QUERY_INFORMATION_LEVEL = 6;
pub const BatterySerialNumber: BATTERY_QUERY_INFORMATION_LEVEL = 8;
pub const BatteryTemperature: BATTERY_QUERY_INFORMATION_LEVEL = 2;
pub const BatteryUniqueID: BATTERY_QUERY_INFORMATION_LEVEL = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOMIZED_IO_CAPABILITIES {
    pub SupportedInputs: u32,
    pub SupportedOutputs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOMIZED_IO_QUERY_INPUT_RETURN {
    pub FunctionId: u32,
    pub ErrorCode: u32,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOMIZED_IO_SEND_OUTPUT_BUFFER {
    pub FunctionId: u32,
    pub Value: u32,
}
pub const GUID_CLASS_INPUT: windows_core::GUID = windows_core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVICE_ACPI_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x97f99bf6_4497_4f18_bb22_4b9fb2fbef9c);
pub const GUID_DEVICE_APPLICATIONLAUNCH_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x629758ee_986e_4d9e_8e47_de27f8ab054d);
pub const GUID_DEVICE_BATTERY: windows_core::GUID = windows_core::GUID::from_u128(0x72631e54_78a4_11d0_bcf7_00aa00b7b32a);
pub const GUID_DEVICE_FAN: windows_core::GUID = windows_core::GUID::from_u128(0x05ecd13d_81da_4a2a_8a4c_524f23dd4dc9);
pub const GUID_DEVICE_LID: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d52_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_MEMORY: windows_core::GUID = windows_core::GUID::from_u128(0x3fd0f03d_92e0_45fb_b75c_5ed8ffb01021);
pub const GUID_DEVICE_MESSAGE_INDICATOR: windows_core::GUID = windows_core::GUID::from_u128(0xcd48a365_fa94_4ce2_a232_a1b764e5d8b4);
pub const GUID_DEVICE_POWER_ADAPTER: windows_core::GUID = windows_core::GUID::from_u128(0xf76c6c62_7dea_43cd_8689_d9a4af3d8557);
pub const GUID_DEVICE_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x97fadb10_4e33_40ae_359c_8bef029dbdd0);
pub const GUID_DEVICE_SYS_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d53_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_THERMAL_ZONE: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d51_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVINTERFACE_CUSTOMIZED_IO: windows_core::GUID = windows_core::GUID::from_u128(0x2ed8544a_8eef_4033_b2a0_04aaa507cecb);
pub const GUID_DEVINTERFACE_POWER_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x8f366301_091e_4056_b92f_958b27625fce);
pub const GUID_DEVINTERFACE_TEMPERATURE_SENSOR: windows_core::GUID = windows_core::GUID::from_u128(0x2a6c8538_7895_4d56_8567_795d3844858a);
pub const GUID_DEVINTERFACE_THERMAL_COOLING: windows_core::GUID = windows_core::GUID::from_u128(0xdbe4373d_3c81_40cb_ace4_e0e5d05f0c9f);
pub const GUID_DEVINTERFACE_THERMAL_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x927ec093_69a4_4bc0_bd02_711664714463);
pub const IOCTL_ACPI_GET_REAL_TIME: u32 = 2703888;
pub const IOCTL_ACPI_SET_REAL_TIME: u32 = 2720276;
pub const IOCTL_BATTERY_CHARGING_SOURCE_CHANGE: u32 = 2703440;
pub const IOCTL_BATTERY_QUERY_INFORMATION: u32 = 2703428;
pub const IOCTL_BATTERY_QUERY_STATUS: u32 = 2703436;
pub const IOCTL_BATTERY_QUERY_TAG: u32 = 2703424;
pub const IOCTL_BATTERY_SET_INFORMATION: u32 = 2719816;
pub const IOCTL_GET_ACPI_TIME_AND_ALARM_CAPABILITIES: u32 = 2703900;
pub const IOCTL_GET_PROCESSOR_OBJ_INFO: u32 = 2703744;
pub const IOCTL_GET_SYS_BUTTON_CAPS: u32 = 2703680;
pub const IOCTL_GET_SYS_BUTTON_EVENT: u32 = 2703684;
pub const IOCTL_GET_WAKE_ALARM_POLICY: u32 = 2736652;
pub const IOCTL_GET_WAKE_ALARM_SYSTEM_POWERSTATE: u32 = 2703896;
pub const IOCTL_GET_WAKE_ALARM_VALUE: u32 = 2736648;
pub const IOCTL_NOTIFY_SWITCH_EVENT: u32 = 2703616;
pub const IOCTL_QUERY_CUSTOMIZED_INPUT_FROM_PLATFORM: u32 = 2704004;
pub const IOCTL_QUERY_CUSTOMIZED_IO_CAPABILITIES: u32 = 2704000;
pub const IOCTL_QUERY_LID: u32 = 2703552;
pub const IOCTL_RUN_ACTIVE_COOLING_METHOD: u32 = 2719880;
pub const IOCTL_SEND_CUSTOMIZED_OUTPUT_TO_PLATFORM: u32 = 2720392;
pub const IOCTL_SET_SYS_MESSAGE_INDICATOR: u32 = 2720192;
pub const IOCTL_SET_WAKE_ALARM_POLICY: u32 = 2720260;
pub const IOCTL_SET_WAKE_ALARM_VALUE: u32 = 2720256;
pub const IOCTL_THERMAL_QUERY_INFORMATION: u32 = 2703488;
pub const IOCTL_THERMAL_READ_POLICY: u32 = 2703508;
pub const IOCTL_THERMAL_READ_TEMPERATURE: u32 = 2703504;
pub const IOCTL_THERMAL_SET_COOLING_POLICY: u32 = 2719876;
pub const IOCTL_THERMAL_SET_PASSIVE_LIMIT: u32 = 2719884;
pub const MAX_ACTIVE_COOLING_LEVELS: u32 = 10;
pub const MAX_BATTERY_STRING_SIZE: u32 = 128;
pub type PACPI_REAL_TIME = *mut ACPI_REAL_TIME;
pub type PACPI_TIME_AND_ALARM_CAPABILITIES = *mut ACPI_TIME_AND_ALARM_CAPABILITIES;
pub type PACPI_TIME_RESOLUTION = *mut ACPI_TIME_RESOLUTION;
pub const PASSIVE_COOLING: u32 = 1;
pub type PBATTERY_CHARGER_ID = *mut windows_core::GUID;
pub type PBATTERY_CHARGER_STATUS = *mut BATTERY_CHARGER_STATUS;
pub type PBATTERY_CHARGING_SOURCE = *mut BATTERY_CHARGING_SOURCE;
pub type PBATTERY_CHARGING_SOURCE_INFORMATION = *mut BATTERY_CHARGING_SOURCE_INFORMATION;
pub type PBATTERY_CHARGING_SOURCE_TYPE = *mut BATTERY_CHARGING_SOURCE_TYPE;
pub type PBATTERY_INFORMATION = *mut BATTERY_INFORMATION;
pub type PBATTERY_MANUFACTURE_DATE = *mut BATTERY_MANUFACTURE_DATE;
pub type PBATTERY_QUERY_INFORMATION = *mut BATTERY_QUERY_INFORMATION;
pub type PBATTERY_SET_INFORMATION = *mut BATTERY_SET_INFORMATION;
pub type PBATTERY_STATUS = *mut BATTERY_STATUS;
pub type PBATTERY_USB_CHARGER_STATUS = *mut BATTERY_USB_CHARGER_STATUS;
pub type PBATTERY_WAIT_STATUS = *mut BATTERY_WAIT_STATUS;
pub type PCUSTOMIZED_IO_CAPABILITIES = *mut CUSTOMIZED_IO_CAPABILITIES;
pub type PCUSTOMIZED_IO_QUERY_INPUT_RETURN = *mut CUSTOMIZED_IO_QUERY_INPUT_RETURN;
pub type PCUSTOMIZED_IO_SEND_OUTPUT_BUFFER = *mut CUSTOMIZED_IO_SEND_OUTPUT_BUFFER;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_ADAPTER_CHARGE_REQUIREMENT {
    pub AcAdapterType: u32,
    pub MinimumPower: u32,
    pub NominalPower: u32,
    pub MaximumPower: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union POWER_ADAPTER_POWER_STATES {
    pub States: POWER_ADAPTER_POWER_STATES_0,
    pub AsUlong: u32,
}
impl Default for POWER_ADAPTER_POWER_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_ADAPTER_POWER_STATES_0 {
    pub _bitfield: u32,
}
pub const POWER_ADAPTER_REC_TIME_NOT_AVAILABLE: u64 = 18446744073709551615;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POWER_ADAPTER_SET_STATUS_BUFFER {
    pub Version: u8,
    pub RecOverride: bool,
    pub Reserved: [u8; 2],
}
impl Default for POWER_ADAPTER_SET_STATUS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_ADAPTER_STATUS {
    pub Version: u8,
    pub Reserved: [u8; 3],
    pub PowerState: POWER_ADAPTER_POWER_STATES,
    pub PeakPower: u32,
    pub MaxOutputPower: u32,
    pub MaxInputPower: u32,
    pub RecStartTime: u64,
    pub RecEndTime: u64,
}
impl Default for POWER_ADAPTER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PPOWER_ADAPTER_CHARGE_REQUIREMENT = *mut POWER_ADAPTER_CHARGE_REQUIREMENT;
pub type PPOWER_ADAPTER_POWER_STATES = *mut POWER_ADAPTER_POWER_STATES;
pub type PPOWER_ADAPTER_SET_STATUS_BUFFER = *mut POWER_ADAPTER_SET_STATUS_BUFFER;
pub type PPOWER_ADAPTER_STATUS = *mut POWER_ADAPTER_STATUS;
pub type PPROCESSOR_OBJECT_INFO = *mut PROCESSOR_OBJECT_INFO;
pub type PPROCESSOR_OBJECT_INFO_EX = *mut PROCESSOR_OBJECT_INFO_EX;
#[cfg(feature = "devpropdef")]
pub const PROCESSOR_NUMBER_PKEY: super::devpropdef::DEVPROPKEY = super::devpropdef::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x5724c81d_d5af_4c1f_a103_a06e28f204c6), pid: super::devpropdef::DEVPROPID(1) };
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_OBJECT_INFO {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_OBJECT_INFO_EX {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
    pub InitialApicId: u32,
}
#[cfg(feature = "basetsd")]
pub type PTHERMAL_INFORMATION = *mut THERMAL_INFORMATION;
pub type PTHERMAL_POLICY = *mut THERMAL_POLICY;
pub type PTHERMAL_WAIT_READ = *mut THERMAL_WAIT_READ;
pub type PUSB_CHARGER_PORT = *mut USB_CHARGER_PORT;
pub type PWAKE_ALARM_INFORMATION = *mut WAKE_ALARM_INFORMATION;
pub const SYS_BUTTON_LID: u32 = 4;
pub const SYS_BUTTON_LID_CHANGED: u32 = 524288;
pub const SYS_BUTTON_LID_CLOSED: u32 = 131072;
pub const SYS_BUTTON_LID_INITIAL: u32 = 262144;
pub const SYS_BUTTON_LID_OPEN: u32 = 65536;
pub const SYS_BUTTON_LID_STATE_MASK: u32 = 196608;
pub const SYS_BUTTON_POWER: u32 = 1;
pub const SYS_BUTTON_SLEEP: u32 = 2;
pub const SYS_BUTTON_WAKE: u32 = 2147483648;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct THERMAL_INFORMATION {
    pub ThermalStamp: u32,
    pub ThermalConstant1: u32,
    pub ThermalConstant2: u32,
    pub Processors: super::basetsd::KAFFINITY,
    pub SamplingPeriod: u32,
    pub CurrentTemperature: u32,
    pub PassiveTripPoint: u32,
    pub CriticalTripPoint: u32,
    pub ActiveTripPointCount: u8,
    pub ActiveTripPoint: [u32; 10],
}
#[cfg(feature = "basetsd")]
impl Default for THERMAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct THERMAL_POLICY {
    pub Version: u32,
    pub WaitForUpdate: bool,
    pub Hibernate: bool,
    pub Critical: bool,
    pub ThermalStandby: bool,
    pub ActivationReasons: u32,
    pub PassiveLimit: u32,
    pub ActiveLevel: u32,
    pub OverThrottled: bool,
}
pub const THERMAL_POLICY_VERSION_1: u32 = 1;
pub const THERMAL_POLICY_VERSION_2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct THERMAL_WAIT_READ {
    pub Timeout: u32,
    pub LowTemperature: u32,
    pub HighTemperature: u32,
}
pub const THERMAL_WAIT_READ_TIMEOUT_IMMEDIATE: u32 = 0;
pub const THERMAL_WAIT_READ_TIMEOUT_NONE: u32 = 4294967295;
pub const TZ_ACTIVATION_REASON_CURRENT: u32 = 2;
pub const TZ_ACTIVATION_REASON_THERMAL: u32 = 1;
pub const UNKNOWN_CAPACITY: i32 = -1;
pub const UNKNOWN_CURRENT: i32 = -1;
pub const UNKNOWN_RATE: i32 = -2147483648;
pub const UNKNOWN_VOLTAGE: i32 = -1;
pub type USB_CHARGER_PORT = i32;
pub const UsbChargerPort_Legacy: USB_CHARGER_PORT = 0;
pub const UsbChargerPort_Max: USB_CHARGER_PORT = 2;
pub const UsbChargerPort_TypeC: USB_CHARGER_PORT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WAKE_ALARM_INFORMATION {
    pub TimerIdentifier: u32,
    pub Timeout: u32,
}
