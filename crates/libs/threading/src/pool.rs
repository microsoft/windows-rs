use super::*;

pub struct Pool(TP_CALLBACK_ENVIRON_V3);

impl Pool {
    pub fn new() -> Self {
        Self::with_priority(Priority::Normal)
    }
    
    pub fn with_priority(priority: Priority) -> Self {
        let mut e = TP_CALLBACK_ENVIRON_V3 {
            Version: 3,
            CallbackPriority: priority as i32,
            Size: core::mem::size_of::<TP_CALLBACK_ENVIRON_V3>() as u32,
            ..Default::default()
        };

        unsafe {
            e.Pool = check(CreateThreadpool(core::mem::zeroed()));
            e.CleanupGroup = check(CreateThreadpoolCleanupGroup());
        }

        Self(e)
    }

    pub fn set_thread_limits(&self, min: u32, max: u32) {
        unsafe {
            check(SetThreadpoolThreadMinimum(self.0.Pool, min));
            SetThreadpoolThreadMaximum(self.0.Pool, max);
        }
    }

    pub fn submit<F: FnOnce() + Send + 'static>(&self, f: F) {
        try_submit(&self.0, f)
    }

    pub fn join(&self) {
        unsafe {
            CloseThreadpoolCleanupGroupMembers(self.0.CleanupGroup, 0, core::mem::zeroed());
        }
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: ok to drop before joining?
impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            CloseThreadpoolCleanupGroup(self.0.CleanupGroup);
            CloseThreadpool(self.0.Pool);
        }
    }
}
