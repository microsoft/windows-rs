#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCHEDULE(pub *mut SCHEDULE);
impl PSCHEDULE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCHEDULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCHEDULE_HEADER(pub *mut SCHEDULE_HEADER);
impl PSCHEDULE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCHEDULE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHEDULE_HEADER {
    pub Type: u32,
    pub Offset: u32,
}
pub const SCHEDULE_INTERVAL: u32 = 0;
pub const SCHEDULE_PRIORITY: u32 = 2;
