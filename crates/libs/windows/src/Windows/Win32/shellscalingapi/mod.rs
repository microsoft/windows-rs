#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GetDpiForMonitor(hmonitor: super::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut super::UINT, dpiy: *mut super::UINT) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetDpiForMonitor(hmonitor : super::HMONITOR, dpitype : MONITOR_DPI_TYPE, dpix : *mut super::UINT, dpiy : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { GetDpiForMonitor(hmonitor, dpitype, dpix as _, dpiy as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDpiForShellUIComponent(param0: SHELL_UI_COMPONENT) -> super::UINT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-2.dll" "system" fn GetDpiForShellUIComponent(param0 : SHELL_UI_COMPONENT) -> super::UINT);
    unsafe { GetDpiForShellUIComponent(param0) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessDpiAwareness(hprocess: Option<super::HANDLE>, value: *mut PROCESS_DPI_AWARENESS) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetProcessDpiAwareness(hprocess : super::HANDLE, value : *mut PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe { GetProcessDpiAwareness(hprocess.unwrap_or(core::mem::zeroed()) as _, value as _) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn GetScaleFactorForDevice(devicetype: DISPLAY_DEVICE_TYPE) -> super::DEVICE_SCALE_FACTOR {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn GetScaleFactorForDevice(devicetype : DISPLAY_DEVICE_TYPE) -> super::DEVICE_SCALE_FACTOR);
    unsafe { GetScaleFactorForDevice(devicetype) }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn GetScaleFactorForMonitor(hmon: super::HMONITOR, pscale: *mut super::DEVICE_SCALE_FACTOR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetScaleFactorForMonitor(hmon : super::HMONITOR, pscale : *mut super::DEVICE_SCALE_FACTOR) -> windows_core::HRESULT);
    unsafe { GetScaleFactorForMonitor(hmon, pscale as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RegisterScaleChangeEvent(hevent: super::HANDLE, pdwcookie: *mut super::DWORD_PTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn RegisterScaleChangeEvent(hevent : super::HANDLE, pdwcookie : *mut super::DWORD_PTR) -> windows_core::HRESULT);
    unsafe { RegisterScaleChangeEvent(hevent, pdwcookie as _) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn RegisterScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, hwndnotify: super::HWND, umsgnotify: super::UINT, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn RegisterScaleChangeNotifications(displaydevice : DISPLAY_DEVICE_TYPE, hwndnotify : super::HWND, umsgnotify : super::UINT, pdwcookie : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { RegisterScaleChangeNotifications(displaydevice, hwndnotify, umsgnotify, pdwcookie as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RevokeScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, dwcookie: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-0.dll" "system" fn RevokeScaleChangeNotifications(displaydevice : DISPLAY_DEVICE_TYPE, dwcookie : super::DWORD) -> windows_core::HRESULT);
    unsafe { RevokeScaleChangeNotifications(displaydevice, dwcookie) }
}
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn SetProcessDpiAwareness(value : PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe { SetProcessDpiAwareness(value) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn UnregisterScaleChangeEvent(dwcookie: super::DWORD_PTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn UnregisterScaleChangeEvent(dwcookie : super::DWORD_PTR) -> windows_core::HRESULT);
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
pub type SCALE_CHANGE_FLAGS = i32;
pub const SCF_PHYSICAL: SCALE_CHANGE_FLAGS = 2;
pub const SCF_SCALE: SCALE_CHANGE_FLAGS = 1;
pub const SCF_VALUE_NONE: SCALE_CHANGE_FLAGS = 0;
pub type SHELL_UI_COMPONENT = i32;
pub const SHELL_UI_COMPONENT_DESKBAND: SHELL_UI_COMPONENT = 2;
pub const SHELL_UI_COMPONENT_NOTIFICATIONAREA: SHELL_UI_COMPONENT = 1;
pub const SHELL_UI_COMPONENT_TASKBARS: SHELL_UI_COMPONENT = 0;
