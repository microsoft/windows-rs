#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDpiForMonitor(hmonitor: super::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetDpiForMonitor(hmonitor : super::HMONITOR, dpitype : MONITOR_DPI_TYPE, dpix : *mut u32, dpiy : *mut u32) -> windows_core::HRESULT);
    unsafe { GetDpiForMonitor(hmonitor, dpitype, dpix as _, dpiy as _) }
}
#[inline]
pub unsafe fn GetDpiForShellUIComponent(param0: SHELL_UI_COMPONENT) -> u32 {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-2.dll" "system" fn GetDpiForShellUIComponent(param0 : SHELL_UI_COMPONENT) -> u32);
    unsafe { GetDpiForShellUIComponent(param0) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessDpiAwareness(hprocess: Option<super::HANDLE>) -> windows_core::Result<PROCESS_DPI_AWARENESS> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetProcessDpiAwareness(hprocess : super::HANDLE, value : *mut PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetProcessDpiAwareness(hprocess.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn GetScaleFactorForDevice(devicetype: DISPLAY_DEVICE_TYPE) -> super::DEVICE_SCALE_FACTOR {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn GetScaleFactorForDevice(devicetype : DISPLAY_DEVICE_TYPE) -> super::DEVICE_SCALE_FACTOR);
    unsafe { GetScaleFactorForDevice(devicetype) }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn GetScaleFactorForMonitor(hmon: super::HMONITOR) -> windows_core::Result<super::DEVICE_SCALE_FACTOR> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetScaleFactorForMonitor(hmon : super::HMONITOR, pscale : *mut super::DEVICE_SCALE_FACTOR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetScaleFactorForMonitor(hmon, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterScaleChangeEvent(hevent: super::HANDLE) -> windows_core::Result<usize> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn RegisterScaleChangeEvent(hevent : super::HANDLE, pdwcookie : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RegisterScaleChangeEvent(hevent, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, hwndnotify: super::HWND, umsgnotify: u32) -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn RegisterScaleChangeNotifications(displaydevice : DISPLAY_DEVICE_TYPE, hwndnotify : super::HWND, umsgnotify : u32, pdwcookie : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RegisterScaleChangeNotifications(displaydevice, hwndnotify, umsgnotify, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RevokeScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, dwcookie: u32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn RevokeScaleChangeNotifications(displaydevice : DISPLAY_DEVICE_TYPE, dwcookie : u32) -> windows_core::HRESULT);
    unsafe { RevokeScaleChangeNotifications(displaydevice, dwcookie) }
}
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn SetProcessDpiAwareness(value : PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe { SetProcessDpiAwareness(value) }
}
#[inline]
pub unsafe fn UnregisterScaleChangeEvent(dwcookie: usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn UnregisterScaleChangeEvent(dwcookie : usize) -> windows_core::HRESULT);
    unsafe { UnregisterScaleChangeEvent(dwcookie) }
}
pub const DEVICE_IMMERSIVE: DISPLAY_DEVICE_TYPE = 1;
pub const DEVICE_PRIMARY: DISPLAY_DEVICE_TYPE = 0;
pub type DISPLAY_DEVICE_TYPE = i32;
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = 1;
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = 0;
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = 0;
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = 2;
pub type MONITOR_DPI_TYPE = i32;
pub type PROCESS_DPI_AWARENESS = i32;
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = 0;
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2;
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1;
pub type SCALE_CHANGE_FLAGS = u32;
pub const SCF_PHYSICAL: SCALE_CHANGE_FLAGS = 2;
pub const SCF_SCALE: SCALE_CHANGE_FLAGS = 1;
pub const SCF_VALUE_NONE: SCALE_CHANGE_FLAGS = 0;
pub type SHELL_UI_COMPONENT = i32;
pub const SHELL_UI_COMPONENT_DESKBAND: SHELL_UI_COMPONENT = 2;
pub const SHELL_UI_COMPONENT_NOTIFICATIONAREA: SHELL_UI_COMPONENT = 1;
pub const SHELL_UI_COMPONENT_TASKBARS: SHELL_UI_COMPONENT = 0;
