impl ::core::default::Default for RM_APP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_APP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RM_APP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_APP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RM_FILTER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_ACTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RM_FILTER_TRIGGER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_FILTER_TRIGGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_TRIGGER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Process == other.Process && self.strAppName == other.strAppName && self.strServiceShortName == other.strServiceShortName && self.ApplicationType == other.ApplicationType && self.AppStatus == other.AppStatus && self.TSSessionId == other.TSSessionId && self.bRestartable == other.bRestartable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RM_PROCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_PROCESS_INFO").field("Process", &self.Process).field("strAppName", &self.strAppName).field("strServiceShortName", &self.strServiceShortName).field("ApplicationType", &self.ApplicationType).field("AppStatus", &self.AppStatus).field("TSSessionId", &self.TSSessionId).field("bRestartable", &self.bRestartable).finish()
    }
}
impl ::core::default::Default for RM_REBOOT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_REBOOT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_REBOOT_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for RM_SHUTDOWN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RM_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_UNIQUE_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_UNIQUE_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwProcessId == other.dwProcessId && self.ProcessStartTime == other.ProcessStartTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RM_UNIQUE_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_UNIQUE_PROCESS").field("dwProcessId", &self.dwProcessId).field("ProcessStartTime", &self.ProcessStartTime).finish()
    }
}
