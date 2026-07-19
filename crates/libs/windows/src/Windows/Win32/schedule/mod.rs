pub type PSCHEDULE = *mut SCHEDULE;
pub type PSCHEDULE_HEADER = *mut SCHEDULE_HEADER;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHEDULE {
    pub Size: u32,
    pub Bandwidth: u32,
    pub NumberOfSchedules: u32,
    pub Schedules: [SCHEDULE_HEADER; 1],
}
impl Default for SCHEDULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCHEDULE_BANDWIDTH: u32 = 1;
pub const SCHEDULE_DATA_ENTRIES: u32 = 168;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHEDULE_HEADER {
    pub Type: u32,
    pub Offset: u32,
}
pub const SCHEDULE_INTERVAL: u32 = 0;
pub const SCHEDULE_PRIORITY: u32 = 2;
