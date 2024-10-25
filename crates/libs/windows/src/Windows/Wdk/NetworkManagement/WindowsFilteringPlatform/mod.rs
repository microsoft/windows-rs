#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
pub type FWPM_SERVICE_STATE_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, newstate: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE)>;
