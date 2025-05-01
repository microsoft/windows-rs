use super::*;

pub struct WorkPool(TP_CALLBACK_ENVIRON_V3);

impl WorkPool {
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

    pub fn submit<F: FnMut() + Send + 'static>(&self, f: F) -> Work {
        
    }

    pub fn cancel(&self) {
        unsafe {
            CloseThreadpoolCleanupGroupMembers(self.0.CleanupGroup, 1, core::mem::zeroed());
        }
    }

    pub fn join(&self) {
        unsafe {
            CloseThreadpoolCleanupGroupMembers(self.0.CleanupGroup, 0, core::mem::zeroed());
        }
    }
}

impl Default for WorkPool {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: ok to drop before joining?
impl Drop for WorkPool {
    fn drop(&mut self) {
        unsafe {
            CloseThreadpoolCleanupGroup(self.0.CleanupGroup);
            CloseThreadpool(self.0.Pool);
        }
    }
}
