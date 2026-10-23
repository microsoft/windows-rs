#[cfg(feature = "shared")]
pub type PSCHEDULE = *mut SCHEDULE;
#[cfg(feature = "shared")]
pub type PSCHEDULE_HEADER = *mut SCHEDULE_HEADER;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHEDULE {
    pub Size: super::ULONG,
    pub Bandwidth: super::ULONG,
    pub NumberOfSchedules: super::ULONG,
    pub Schedules: [SCHEDULE_HEADER; 1],
}
#[cfg(feature = "shared")]
impl Default for SCHEDULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCHEDULE_BANDWIDTH: i32 = 1;
pub const SCHEDULE_DATA_ENTRIES: i32 = 168;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHEDULE_HEADER {
    pub Type: super::ULONG,
    pub Offset: super::ULONG,
}
pub const SCHEDULE_INTERVAL: i32 = 0;
pub const SCHEDULE_PRIORITY: i32 = 2;
