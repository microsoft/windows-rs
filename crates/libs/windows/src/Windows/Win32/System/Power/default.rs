impl ::core::default::Default for ACPI_REAL_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACPI_REAL_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Year == other.Year && self.Month == other.Month && self.Day == other.Day && self.Hour == other.Hour && self.Minute == other.Minute && self.Second == other.Second && self.Valid == other.Valid && self.Milliseconds == other.Milliseconds && self.TimeZone == other.TimeZone && self.DayLight == other.DayLight && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for ACPI_REAL_TIME {}
impl ::core::fmt::Debug for ACPI_REAL_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACPI_REAL_TIME").field("Year", &self.Year).field("Month", &self.Month).field("Day", &self.Day).field("Hour", &self.Hour).field("Minute", &self.Minute).field("Second", &self.Second).field("Valid", &self.Valid).field("Milliseconds", &self.Milliseconds).field("TimeZone", &self.TimeZone).field("DayLight", &self.DayLight).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for ADMINISTRATOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADMINISTRATOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.MinSleep == other.MinSleep && self.MaxSleep == other.MaxSleep && self.MinVideoTimeout == other.MinVideoTimeout && self.MaxVideoTimeout == other.MaxVideoTimeout && self.MinSpindownTimeout == other.MinSpindownTimeout && self.MaxSpindownTimeout == other.MaxSpindownTimeout
    }
}
impl ::core::cmp::Eq for ADMINISTRATOR_POWER_POLICY {}
impl ::core::fmt::Debug for ADMINISTRATOR_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADMINISTRATOR_POWER_POLICY").field("MinSleep", &self.MinSleep).field("MaxSleep", &self.MaxSleep).field("MinVideoTimeout", &self.MinVideoTimeout).field("MaxVideoTimeout", &self.MaxVideoTimeout).field("MinSpindownTimeout", &self.MinSpindownTimeout).field("MaxSpindownTimeout", &self.MaxSpindownTimeout).finish()
    }
}
impl ::core::default::Default for BATTERY_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.VaData == other.VaData
    }
}
impl ::core::cmp::Eq for BATTERY_CHARGER_STATUS {}
impl ::core::fmt::Debug for BATTERY_CHARGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_CHARGER_STATUS").field("Type", &self.Type).field("VaData", &self.VaData).finish()
    }
}
impl ::core::default::Default for BATTERY_CHARGING_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_CHARGING_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.MaxCurrent == other.MaxCurrent
    }
}
impl ::core::cmp::Eq for BATTERY_CHARGING_SOURCE {}
impl ::core::fmt::Debug for BATTERY_CHARGING_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_CHARGING_SOURCE").field("Type", &self.Type).field("MaxCurrent", &self.MaxCurrent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_CHARGING_SOURCE_INFORMATION").field("Type", &self.Type).field("SourceOnline", &self.SourceOnline).finish()
    }
}
impl ::core::default::Default for BATTERY_CHARGING_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_CHARGING_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_CHARGING_SOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.Technology == other.Technology && self.Reserved == other.Reserved && self.Chemistry == other.Chemistry && self.DesignedCapacity == other.DesignedCapacity && self.FullChargedCapacity == other.FullChargedCapacity && self.DefaultAlert1 == other.DefaultAlert1 && self.DefaultAlert2 == other.DefaultAlert2 && self.CriticalBias == other.CriticalBias && self.CycleCount == other.CycleCount
    }
}
impl ::core::cmp::Eq for BATTERY_INFORMATION {}
impl ::core::fmt::Debug for BATTERY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_INFORMATION").field("Capabilities", &self.Capabilities).field("Technology", &self.Technology).field("Reserved", &self.Reserved).field("Chemistry", &self.Chemistry).field("DesignedCapacity", &self.DesignedCapacity).field("FullChargedCapacity", &self.FullChargedCapacity).field("DefaultAlert1", &self.DefaultAlert1).field("DefaultAlert2", &self.DefaultAlert2).field("CriticalBias", &self.CriticalBias).field("CycleCount", &self.CycleCount).finish()
    }
}
impl ::core::default::Default for BATTERY_MANUFACTURE_DATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_MANUFACTURE_DATE {
    fn eq(&self, other: &Self) -> bool {
        self.Day == other.Day && self.Month == other.Month && self.Year == other.Year
    }
}
impl ::core::cmp::Eq for BATTERY_MANUFACTURE_DATE {}
impl ::core::fmt::Debug for BATTERY_MANUFACTURE_DATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_MANUFACTURE_DATE").field("Day", &self.Day).field("Month", &self.Month).field("Year", &self.Year).finish()
    }
}
impl ::core::default::Default for BATTERY_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_QUERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.InformationLevel == other.InformationLevel && self.AtRate == other.AtRate
    }
}
impl ::core::cmp::Eq for BATTERY_QUERY_INFORMATION {}
impl ::core::fmt::Debug for BATTERY_QUERY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_QUERY_INFORMATION").field("BatteryTag", &self.BatteryTag).field("InformationLevel", &self.InformationLevel).field("AtRate", &self.AtRate).finish()
    }
}
impl ::core::default::Default for BATTERY_QUERY_INFORMATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_QUERY_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_QUERY_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for BATTERY_REPORTING_SCALE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_REPORTING_SCALE {
    fn eq(&self, other: &Self) -> bool {
        self.Granularity == other.Granularity && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for BATTERY_REPORTING_SCALE {}
impl ::core::fmt::Debug for BATTERY_REPORTING_SCALE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_REPORTING_SCALE").field("Granularity", &self.Granularity).field("Capacity", &self.Capacity).finish()
    }
}
impl ::core::default::Default for BATTERY_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_SET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.InformationLevel == other.InformationLevel && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for BATTERY_SET_INFORMATION {}
impl ::core::fmt::Debug for BATTERY_SET_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_SET_INFORMATION").field("BatteryTag", &self.BatteryTag).field("InformationLevel", &self.InformationLevel).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for BATTERY_SET_INFORMATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_SET_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_SET_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for BATTERY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.PowerState == other.PowerState && self.Capacity == other.Capacity && self.Voltage == other.Voltage && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for BATTERY_STATUS {}
impl ::core::fmt::Debug for BATTERY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_STATUS").field("PowerState", &self.PowerState).field("Capacity", &self.Capacity).field("Voltage", &self.Voltage).field("Rate", &self.Rate).finish()
    }
}
impl ::core::default::Default for BATTERY_USB_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_USB_CHARGER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Flags == other.Flags && self.MaxCurrent == other.MaxCurrent && self.Voltage == other.Voltage && self.PortType == other.PortType && self.PortId == other.PortId && self.PowerSourceInformation == other.PowerSourceInformation && self.OemCharger == other.OemCharger
    }
}
impl ::core::cmp::Eq for BATTERY_USB_CHARGER_STATUS {}
impl ::core::fmt::Debug for BATTERY_USB_CHARGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_USB_CHARGER_STATUS").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Flags", &self.Flags).field("MaxCurrent", &self.MaxCurrent).field("Voltage", &self.Voltage).field("PortType", &self.PortType).field("PortId", &self.PortId).field("PowerSourceInformation", &self.PowerSourceInformation).field("OemCharger", &self.OemCharger).finish()
    }
}
impl ::core::default::Default for BATTERY_WAIT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BATTERY_WAIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryTag == other.BatteryTag && self.Timeout == other.Timeout && self.PowerState == other.PowerState && self.LowCapacity == other.LowCapacity && self.HighCapacity == other.HighCapacity
    }
}
impl ::core::cmp::Eq for BATTERY_WAIT_STATUS {}
impl ::core::fmt::Debug for BATTERY_WAIT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BATTERY_WAIT_STATUS").field("BatteryTag", &self.BatteryTag).field("Timeout", &self.Timeout).field("PowerState", &self.PowerState).field("LowCapacity", &self.LowCapacity).field("HighCapacity", &self.HighCapacity).finish()
    }
}
impl ::core::default::Default for CM_POWER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CM_POWER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PD_Size == other.PD_Size && self.PD_MostRecentPowerState == other.PD_MostRecentPowerState && self.PD_Capabilities == other.PD_Capabilities && self.PD_D1Latency == other.PD_D1Latency && self.PD_D2Latency == other.PD_D2Latency && self.PD_D3Latency == other.PD_D3Latency && self.PD_PowerStateMapping == other.PD_PowerStateMapping && self.PD_DeepestSystemWake == other.PD_DeepestSystemWake
    }
}
impl ::core::cmp::Eq for CM_POWER_DATA {}
impl ::core::fmt::Debug for CM_POWER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_POWER_DATA").field("PD_Size", &self.PD_Size).field("PD_MostRecentPowerState", &self.PD_MostRecentPowerState).field("PD_Capabilities", &self.PD_Capabilities).field("PD_D1Latency", &self.PD_D1Latency).field("PD_D2Latency", &self.PD_D2Latency).field("PD_D3Latency", &self.PD_D3Latency).field("PD_PowerStateMapping", &self.PD_PowerStateMapping).field("PD_DeepestSystemWake", &self.PD_DeepestSystemWake).finish()
    }
}
impl ::core::default::Default for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICE_POWER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_POWER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EFFECTIVE_POWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EFFECTIVE_POWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EFFECTIVE_POWER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EMI_CHANNEL_MEASUREMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_CHANNEL_MEASUREMENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteEnergy == other.AbsoluteEnergy && self.AbsoluteTime == other.AbsoluteTime
    }
}
impl ::core::cmp::Eq for EMI_CHANNEL_MEASUREMENT_DATA {}
impl ::core::fmt::Debug for EMI_CHANNEL_MEASUREMENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_CHANNEL_MEASUREMENT_DATA").field("AbsoluteEnergy", &self.AbsoluteEnergy).field("AbsoluteTime", &self.AbsoluteTime).finish()
    }
}
impl ::core::default::Default for EMI_CHANNEL_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_CHANNEL_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit && self.ChannelNameSize == other.ChannelNameSize && self.ChannelName == other.ChannelName
    }
}
impl ::core::cmp::Eq for EMI_CHANNEL_V2 {}
impl ::core::fmt::Debug for EMI_CHANNEL_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_CHANNEL_V2").field("MeasurementUnit", &self.MeasurementUnit).field("ChannelNameSize", &self.ChannelNameSize).field("ChannelName", &self.ChannelName).finish()
    }
}
impl ::core::default::Default for EMI_MEASUREMENT_DATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_MEASUREMENT_DATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelData == other.ChannelData
    }
}
impl ::core::cmp::Eq for EMI_MEASUREMENT_DATA_V2 {}
impl ::core::fmt::Debug for EMI_MEASUREMENT_DATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_MEASUREMENT_DATA_V2").field("ChannelData", &self.ChannelData).finish()
    }
}
impl ::core::default::Default for EMI_MEASUREMENT_UNIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMI_MEASUREMENT_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMI_MEASUREMENT_UNIT").field(&self.0).finish()
    }
}
impl ::core::default::Default for EMI_METADATA_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_METADATA_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataSize == other.MetadataSize
    }
}
impl ::core::cmp::Eq for EMI_METADATA_SIZE {}
impl ::core::fmt::Debug for EMI_METADATA_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_METADATA_SIZE").field("MetadataSize", &self.MetadataSize).finish()
    }
}
impl ::core::default::Default for EMI_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.MeasurementUnit == other.MeasurementUnit && self.HardwareOEM == other.HardwareOEM && self.HardwareModel == other.HardwareModel && self.HardwareRevision == other.HardwareRevision && self.MeteredHardwareNameSize == other.MeteredHardwareNameSize && self.MeteredHardwareName == other.MeteredHardwareName
    }
}
impl ::core::cmp::Eq for EMI_METADATA_V1 {}
impl ::core::fmt::Debug for EMI_METADATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_METADATA_V1").field("MeasurementUnit", &self.MeasurementUnit).field("HardwareOEM", &self.HardwareOEM).field("HardwareModel", &self.HardwareModel).field("HardwareRevision", &self.HardwareRevision).field("MeteredHardwareNameSize", &self.MeteredHardwareNameSize).field("MeteredHardwareName", &self.MeteredHardwareName).finish()
    }
}
impl ::core::default::Default for EMI_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareOEM == other.HardwareOEM && self.HardwareModel == other.HardwareModel && self.HardwareRevision == other.HardwareRevision && self.ChannelCount == other.ChannelCount && self.Channels == other.Channels
    }
}
impl ::core::cmp::Eq for EMI_METADATA_V2 {}
impl ::core::fmt::Debug for EMI_METADATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_METADATA_V2").field("HardwareOEM", &self.HardwareOEM).field("HardwareModel", &self.HardwareModel).field("HardwareRevision", &self.HardwareRevision).field("ChannelCount", &self.ChannelCount).field("Channels", &self.Channels).finish()
    }
}
impl ::core::default::Default for EMI_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.EmiVersion == other.EmiVersion
    }
}
impl ::core::cmp::Eq for EMI_VERSION {}
impl ::core::fmt::Debug for EMI_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMI_VERSION").field("EmiVersion", &self.EmiVersion).finish()
    }
}
impl ::core::default::Default for EXECUTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXECUTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXECUTION_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EXECUTION_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EXECUTION_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EXECUTION_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EXECUTION_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EXECUTION_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GLOBAL_MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GLOBAL_MACHINE_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.LidOpenWakeAc == other.LidOpenWakeAc && self.LidOpenWakeDc == other.LidOpenWakeDc && self.BroadcastCapacityResolution == other.BroadcastCapacityResolution
    }
}
impl ::core::cmp::Eq for GLOBAL_MACHINE_POWER_POLICY {}
impl ::core::fmt::Debug for GLOBAL_MACHINE_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLOBAL_MACHINE_POWER_POLICY").field("Revision", &self.Revision).field("LidOpenWakeAc", &self.LidOpenWakeAc).field("LidOpenWakeDc", &self.LidOpenWakeDc).field("BroadcastCapacityResolution", &self.BroadcastCapacityResolution).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLOBAL_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GLOBAL_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLOBAL_POWER_POLICY").field("user", &self.user).field("mach", &self.mach).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GLOBAL_USER_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLOBAL_USER_POWER_POLICY").field("Revision", &self.Revision).field("PowerButtonAc", &self.PowerButtonAc).field("PowerButtonDc", &self.PowerButtonDc).field("SleepButtonAc", &self.SleepButtonAc).field("SleepButtonDc", &self.SleepButtonDc).field("LidCloseAc", &self.LidCloseAc).field("LidCloseDc", &self.LidCloseDc).field("DischargePolicy", &self.DischargePolicy).field("GlobalFlags", &self.GlobalFlags).finish()
    }
}
impl ::core::default::Default for LATENCY_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LATENCY_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LATENCY_TIME").field(&self.0).finish()
    }
}
impl ::core::default::Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MACHINE_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.MinSleepAc == other.MinSleepAc && self.MinSleepDc == other.MinSleepDc && self.ReducedLatencySleepAc == other.ReducedLatencySleepAc && self.ReducedLatencySleepDc == other.ReducedLatencySleepDc && self.DozeTimeoutAc == other.DozeTimeoutAc && self.DozeTimeoutDc == other.DozeTimeoutDc && self.DozeS4TimeoutAc == other.DozeS4TimeoutAc && self.DozeS4TimeoutDc == other.DozeS4TimeoutDc && self.MinThrottleAc == other.MinThrottleAc && self.MinThrottleDc == other.MinThrottleDc && self.pad1 == other.pad1 && self.OverThrottledAc == other.OverThrottledAc && self.OverThrottledDc == other.OverThrottledDc
    }
}
impl ::core::cmp::Eq for MACHINE_POWER_POLICY {}
impl ::core::fmt::Debug for MACHINE_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MACHINE_POWER_POLICY")
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
impl ::core::default::Default for MACHINE_PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MACHINE_PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.ProcessorPolicyAc == other.ProcessorPolicyAc && self.ProcessorPolicyDc == other.ProcessorPolicyDc
    }
}
impl ::core::cmp::Eq for MACHINE_PROCESSOR_POWER_POLICY {}
impl ::core::fmt::Debug for MACHINE_PROCESSOR_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MACHINE_PROCESSOR_POWER_POLICY").field("Revision", &self.Revision).field("ProcessorPolicyAc", &self.ProcessorPolicyAc).field("ProcessorPolicyDc", &self.ProcessorPolicyDc).finish()
    }
}
impl ::core::default::Default for POWERBROADCAST_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POWERBROADCAST_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.PowerSetting == other.PowerSetting && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for POWERBROADCAST_SETTING {}
impl ::core::fmt::Debug for POWERBROADCAST_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWERBROADCAST_SETTING").field("PowerSetting", &self.PowerSetting).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for POWER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_ACTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POWER_ACTION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.Flags == other.Flags && self.EventCode == other.EventCode
    }
}
impl ::core::cmp::Eq for POWER_ACTION_POLICY {}
impl ::core::fmt::Debug for POWER_ACTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_ACTION_POLICY").field("Action", &self.Action).field("Flags", &self.Flags).field("EventCode", &self.EventCode).finish()
    }
}
impl ::core::default::Default for POWER_ACTION_POLICY_EVENT_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_ACTION_POLICY_EVENT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_ACTION_POLICY_EVENT_CODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for POWER_COOLING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_COOLING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_COOLING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_DATA_ACCESSOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_DATA_ACCESSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_DATA_ACCESSOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_INFORMATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_PLATFORM_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_PLATFORM_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_PLATFORM_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_PLATFORM_ROLE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_PLATFORM_ROLE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_PLATFORM_ROLE_VERSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_POLICY").field("user", &self.user).field("mach", &self.mach).finish()
    }
}
impl ::core::default::Default for POWER_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_SETTING_REGISTER_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_SETTING_REGISTER_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESSOR_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID && self.PBlkAddress == other.PBlkAddress && self.PBlkLength == other.PBlkLength
    }
}
impl ::core::cmp::Eq for PROCESSOR_OBJECT_INFO {}
impl ::core::fmt::Debug for PROCESSOR_OBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_OBJECT_INFO").field("PhysicalID", &self.PhysicalID).field("PBlkAddress", &self.PBlkAddress).field("PBlkLength", &self.PBlkLength).finish()
    }
}
impl ::core::default::Default for PROCESSOR_OBJECT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_OBJECT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalID == other.PhysicalID && self.PBlkAddress == other.PBlkAddress && self.PBlkLength == other.PBlkLength && self.InitialApicId == other.InitialApicId
    }
}
impl ::core::cmp::Eq for PROCESSOR_OBJECT_INFO_EX {}
impl ::core::fmt::Debug for PROCESSOR_OBJECT_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_OBJECT_INFO_EX").field("PhysicalID", &self.PhysicalID).field("PBlkAddress", &self.PBlkAddress).field("PBlkLength", &self.PBlkLength).field("InitialApicId", &self.InitialApicId).finish()
    }
}
impl ::core::default::Default for PROCESSOR_POWER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_POWER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.MaxMhz == other.MaxMhz && self.CurrentMhz == other.CurrentMhz && self.MhzLimit == other.MhzLimit && self.MaxIdleState == other.MaxIdleState && self.CurrentIdleState == other.CurrentIdleState
    }
}
impl ::core::cmp::Eq for PROCESSOR_POWER_INFORMATION {}
impl ::core::fmt::Debug for PROCESSOR_POWER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_POWER_INFORMATION").field("Number", &self.Number).field("MaxMhz", &self.MaxMhz).field("CurrentMhz", &self.CurrentMhz).field("MhzLimit", &self.MhzLimit).field("MaxIdleState", &self.MaxIdleState).field("CurrentIdleState", &self.CurrentIdleState).finish()
    }
}
impl ::core::default::Default for PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_POWER_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.DynamicThrottle == other.DynamicThrottle && self.Spare == other.Spare && self._bitfield == other._bitfield && self.PolicyCount == other.PolicyCount && self.Policy == other.Policy
    }
}
impl ::core::cmp::Eq for PROCESSOR_POWER_POLICY {}
impl ::core::fmt::Debug for PROCESSOR_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_POWER_POLICY").field("Revision", &self.Revision).field("DynamicThrottle", &self.DynamicThrottle).field("Spare", &self.Spare).field("_bitfield", &self._bitfield).field("PolicyCount", &self.PolicyCount).field("Policy", &self.Policy).finish()
    }
}
impl ::core::default::Default for PROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_POWER_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TimeCheck == other.TimeCheck && self.DemoteLimit == other.DemoteLimit && self.PromoteLimit == other.PromoteLimit && self.DemotePercent == other.DemotePercent && self.PromotePercent == other.PromotePercent && self.Spare == other.Spare && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for PROCESSOR_POWER_POLICY_INFO {}
impl ::core::fmt::Debug for PROCESSOR_POWER_POLICY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_POWER_POLICY_INFO").field("TimeCheck", &self.TimeCheck).field("DemoteLimit", &self.DemoteLimit).field("PromoteLimit", &self.PromoteLimit).field("DemotePercent", &self.DemotePercent).field("PromotePercent", &self.PromotePercent).field("Spare", &self.Spare).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SET_POWER_SETTING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SET_POWER_SETTING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Guid == other.Guid && self.PowerCondition == other.PowerCondition && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for SET_POWER_SETTING_VALUE {}
impl ::core::fmt::Debug for SET_POWER_SETTING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_POWER_SETTING_VALUE").field("Version", &self.Version).field("Guid", &self.Guid).field("PowerCondition", &self.PowerCondition).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_BATTERY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_BATTERY_STATE")
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
impl ::core::default::Default for SYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_POWER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POWER_CAPABILITIES")
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
impl ::core::default::Default for SYSTEM_POWER_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_POWER_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_POWER_CONDITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_POWER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_POWER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIdlenessAllowed == other.MaxIdlenessAllowed && self.Idleness == other.Idleness && self.TimeRemaining == other.TimeRemaining && self.CoolingMode == other.CoolingMode
    }
}
impl ::core::cmp::Eq for SYSTEM_POWER_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_POWER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POWER_INFORMATION").field("MaxIdlenessAllowed", &self.MaxIdlenessAllowed).field("Idleness", &self.Idleness).field("TimeRemaining", &self.TimeRemaining).field("CoolingMode", &self.CoolingMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_POWER_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POWER_LEVEL").field("Enable", &self.Enable).field("Spare", &self.Spare).field("BatteryLevel", &self.BatteryLevel).field("PowerPolicy", &self.PowerPolicy).field("MinSystemState", &self.MinSystemState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POWER_POLICY")
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
impl ::core::default::Default for SYSTEM_POWER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_POWER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_POWER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_POWER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ACLineStatus == other.ACLineStatus && self.BatteryFlag == other.BatteryFlag && self.BatteryLifePercent == other.BatteryLifePercent && self.SystemStatusFlag == other.SystemStatusFlag && self.BatteryLifeTime == other.BatteryLifeTime && self.BatteryFullLifeTime == other.BatteryFullLifeTime
    }
}
impl ::core::cmp::Eq for SYSTEM_POWER_STATUS {}
impl ::core::fmt::Debug for SYSTEM_POWER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POWER_STATUS").field("ACLineStatus", &self.ACLineStatus).field("BatteryFlag", &self.BatteryFlag).field("BatteryLifePercent", &self.BatteryLifePercent).field("SystemStatusFlag", &self.SystemStatusFlag).field("BatteryLifeTime", &self.BatteryLifeTime).field("BatteryFullLifeTime", &self.BatteryFullLifeTime).finish()
    }
}
impl ::core::default::Default for THERMAL_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for THERMAL_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Type == other.Type && self.Temperature == other.Temperature && self.TripPointTemperature == other.TripPointTemperature && self.Initiator == other.Initiator
    }
}
impl ::core::cmp::Eq for THERMAL_EVENT {}
impl ::core::fmt::Debug for THERMAL_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THERMAL_EVENT").field("Version", &self.Version).field("Size", &self.Size).field("Type", &self.Type).field("Temperature", &self.Temperature).field("TripPointTemperature", &self.TripPointTemperature).field("Initiator", &self.Initiator).finish()
    }
}
impl ::core::default::Default for THERMAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for THERMAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThermalStamp == other.ThermalStamp && self.ThermalConstant1 == other.ThermalConstant1 && self.ThermalConstant2 == other.ThermalConstant2 && self.Processors == other.Processors && self.SamplingPeriod == other.SamplingPeriod && self.CurrentTemperature == other.CurrentTemperature && self.PassiveTripPoint == other.PassiveTripPoint && self.CriticalTripPoint == other.CriticalTripPoint && self.ActiveTripPointCount == other.ActiveTripPointCount && self.ActiveTripPoint == other.ActiveTripPoint
    }
}
impl ::core::cmp::Eq for THERMAL_INFORMATION {}
impl ::core::fmt::Debug for THERMAL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THERMAL_INFORMATION")
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for THERMAL_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for THERMAL_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THERMAL_POLICY").field("Version", &self.Version).field("WaitForUpdate", &self.WaitForUpdate).field("Hibernate", &self.Hibernate).field("Critical", &self.Critical).field("ThermalStandby", &self.ThermalStandby).field("ActivationReasons", &self.ActivationReasons).field("PassiveLimit", &self.PassiveLimit).field("ActiveLevel", &self.ActiveLevel).field("OverThrottled", &self.OverThrottled).finish()
    }
}
impl ::core::default::Default for THERMAL_WAIT_READ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for THERMAL_WAIT_READ {
    fn eq(&self, other: &Self) -> bool {
        self.Timeout == other.Timeout && self.LowTemperature == other.LowTemperature && self.HighTemperature == other.HighTemperature
    }
}
impl ::core::cmp::Eq for THERMAL_WAIT_READ {}
impl ::core::fmt::Debug for THERMAL_WAIT_READ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THERMAL_WAIT_READ").field("Timeout", &self.Timeout).field("LowTemperature", &self.LowTemperature).field("HighTemperature", &self.HighTemperature).finish()
    }
}
impl ::core::default::Default for USB_CHARGER_PORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USB_CHARGER_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USB_CHARGER_PORT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USER_POWER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_POWER_POLICY")
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
impl ::core::default::Default for WAKE_ALARM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAKE_ALARM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TimerIdentifier == other.TimerIdentifier && self.Timeout == other.Timeout
    }
}
impl ::core::cmp::Eq for WAKE_ALARM_INFORMATION {}
impl ::core::fmt::Debug for WAKE_ALARM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAKE_ALARM_INFORMATION").field("TimerIdentifier", &self.TimerIdentifier).field("Timeout", &self.Timeout).finish()
    }
}
