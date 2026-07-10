#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("setupapi.dll" "system" fn InstallHinfSectionA(window : super::windef::HWND, modulehandle : super::minwindef::HINSTANCE, commandline : windows_sys::core::PCSTR, showcommand : i32));
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("setupapi.dll" "system" fn InstallHinfSectionW(window : super::windef::HWND, modulehandle : super::minwindef::HINSTANCE, commandline : windows_sys::core::PCWSTR, showcommand : i32));
windows_link::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_sys::core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_sys::core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_sys::core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_sys::core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListA(diskspace : HDSKSPC, targetfilespec : windows_sys::core::PCSTR, filesize : i64, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListW(diskspace : HDSKSPC, targetfilespec : windows_sys::core::PCWSTR, filesize : i64, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddToSourceListA(flags : u32, source : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAddToSourceListW(flags : u32, source : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListA(diskspace : HDSKSPC, driveroot : windows_sys::core::PCSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListW(diskspace : HDSKSPC, driveroot : windows_sys::core::PCWSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupBackupErrorA(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCSTR, sourcefile : windows_sys::core::PCSTR, targetfile : windows_sys::core::PCSTR, win32errorcode : u32, style : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupBackupErrorW(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCWSTR, sourcefile : windows_sys::core::PCWSTR, targetfile : windows_sys::core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupCancelTemporarySourceList() -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupCloseFileQueue(queuehandle : HSPFILEQ) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupCloseInfFile(infhandle : HINF));
windows_link::link!("setupapi.dll" "system" fn SetupCloseLog());
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupCommitFileQueueA(owner : super::windef::HWND, queuehandle : HSPFILEQ, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupCommitFileQueueW(owner : super::windef::HWND, queuehandle : HSPFILEQ, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionA(infhandle : HINF, sectionname : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionW(infhandle : HINF, sectionname : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupCopyErrorA(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCSTR, diskname : windows_sys::core::PCSTR, pathtosource : windows_sys::core::PCSTR, sourcefile : windows_sys::core::PCSTR, targetpathfile : windows_sys::core::PCSTR, win32errorcode : u32, style : u32, pathbuffer : windows_sys::core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupCopyErrorW(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCWSTR, diskname : windows_sys::core::PCWSTR, pathtosource : windows_sys::core::PCWSTR, sourcefile : windows_sys::core::PCWSTR, targetpathfile : windows_sys::core::PCWSTR, win32errorcode : u32, style : u32, pathbuffer : windows_sys::core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupCopyOEMInfA(sourceinffilename : windows_sys::core::PCSTR, oemsourcemedialocation : windows_sys::core::PCSTR, oemsourcemediatype : u32, copystyle : u32, destinationinffilename : windows_sys::core::PSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupCopyOEMInfW(sourceinffilename : windows_sys::core::PCWSTR, oemsourcemedialocation : windows_sys::core::PCWSTR, oemsourcemediatype : u32, copystyle : u32, destinationinffilename : windows_sys::core::PWSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListA(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
windows_link::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListW(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
windows_link::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileA(sourcefilename : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR, compressiontype : *const u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileW(sourcefilename : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR, compressiontype : *const u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackA(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackW(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDeleteErrorA(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, win32errorcode : u32, style : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDeleteErrorW(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupDestroyDiskSpaceList(diskspace : HDSKSPC) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiAskForOEMDisk(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoList(flags : u32, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExA(flags : u32, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExW(flags : u32, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiBuildDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, drivertype : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiCallClassInstaller(installfunction : DI_FUNCTION, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiCancelDriverInfoSearch(deviceinfoset : HDEVINFO) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiChangeState(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameA(classname : windows_sys::core::PCSTR, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExA(classname : windows_sys::core::PCSTR, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExW(classname : windows_sys::core::PCWSTR, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameW(classname : windows_sys::core::PCWSTR, classguidlist : *mut windows_sys::core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidA(classguid : *const windows_sys::core::GUID, classname : windows_sys::core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExA(classguid : *const windows_sys::core::GUID, classname : windows_sys::core::PSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExW(classguid : *const windows_sys::core::GUID, classname : windows_sys::core::PWSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidW(classguid : *const windows_sys::core::GUID, classname : windows_sys::core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : HINF, infsectionname : windows_sys::core::PCSTR) -> super::minwindef::HKEY);
#[cfg(feature = "minwindef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : HINF, infsectionname : windows_sys::core::PCWSTR) -> super::minwindef::HKEY);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoA(deviceinfoset : HDEVINFO, devicename : windows_sys::core::PCSTR, classguid : *const windows_sys::core::GUID, devicedescription : windows_sys::core::PCSTR, hwndparent : super::windef::HWND, creationflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoList(classguid : *const windows_sys::core::GUID, hwndparent : super::windef::HWND) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExA(classguid : *const windows_sys::core::GUID, hwndparent : super::windef::HWND, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExW(classguid : *const windows_sys::core::GUID, hwndparent : super::windef::HWND, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoW(deviceinfoset : HDEVINFO, devicename : windows_sys::core::PCWSTR, classguid : *const windows_sys::core::GUID, devicedescription : windows_sys::core::PCWSTR, hwndparent : super::windef::HWND, creationflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_sys::core::GUID, referencestring : windows_sys::core::PCSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::winreg::REGSAM, infhandle : HINF, infsectionname : windows_sys::core::PCSTR) -> super::minwindef::HKEY);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::winreg::REGSAM, infhandle : HINF, infsectionname : windows_sys::core::PCWSTR) -> super::minwindef::HKEY);
windows_link::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_sys::core::GUID, referencestring : windows_sys::core::PCWSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDeleteDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceData(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "commctrl")]
windows_link::link!("setupapi.dll" "system" fn SetupDiDestroyClassImageList(classimagelistdata : *const SP_CLASSIMAGELIST_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDestroyDeviceInfoList(deviceinfoset : HDEVINFO) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiDestroyDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiDrawMiniIcon(hdc : super::windef::HDC, rc : super::windef::RECT, miniiconindex : i32, flags : u32) -> i32);
windows_link::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInfo(deviceinfoset : HDEVINFO, memberindex : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_sys::core::GUID, memberindex : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32, memberindex : u32, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32, memberindex : u32, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionA(context : *const INFCONTEXT, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_sys::core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionW(context : *const INFCONTEXT, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_sys::core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallA(infhandle : HINF, infsectionname : windows_sys::core::PCSTR, infsectionwithext : windows_sys::core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExA(infhandle : HINF, infsectionname : windows_sys::core::PCSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_sys::core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_sys::core::PSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExW(infhandle : HINF, infsectionname : windows_sys::core::PCWSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_sys::core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_sys::core::PWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallW(infhandle : HINF, infsectionname : windows_sys::core::PCWSTR, infsectionwithext : windows_sys::core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassBitmapIndex(classguid : *const windows_sys::core::GUID, miniiconindex : *mut i32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionA(classguid : *const windows_sys::core::GUID, classdescription : windows_sys::core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExA(classguid : *const windows_sys::core::GUID, classdescription : windows_sys::core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExW(classguid : *const windows_sys::core::GUID, classdescription : windows_sys::core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionW(classguid : *const windows_sys::core::GUID, classdescription : windows_sys::core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : super::prsht::LPPROPSHEETHEADERA, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : super::prsht::LPPROPSHEETHEADERW, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevsA(classguid : *const windows_sys::core::GUID, enumerator : windows_sys::core::PCSTR, hwndparent : super::windef::HWND, flags : u32) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExA(classguid : *const windows_sys::core::GUID, enumerator : windows_sys::core::PCSTR, hwndparent : super::windef::HWND, flags : u32, deviceinfoset : HDEVINFO, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExW(classguid : *const windows_sys::core::GUID, enumerator : windows_sys::core::PCWSTR, hwndparent : super::windef::HWND, flags : u32, deviceinfoset : HDEVINFO, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassDevsW(classguid : *const windows_sys::core::GUID, enumerator : windows_sys::core::PCWSTR, hwndparent : super::windef::HWND, flags : u32) -> HDEVINFO);
#[cfg(feature = "commctrl")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassImageIndex(classimagelistdata : *const SP_CLASSIMAGELIST_DATA, classguid : *const windows_sys::core::GUID, imageindex : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "commctrl")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassImageList(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "commctrl")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExA(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "commctrl")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExW(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyExW(classguid : *const windows_sys::core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeys(classguid : *const windows_sys::core::GUID, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeysExW(classguid : *const windows_sys::core::GUID, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyW(classguid : *const windows_sys::core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyA(classguid : *const windows_sys::core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyW(classguid : *const windows_sys::core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_sys::core::PCSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_sys::core::PCWSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListClass(deviceinfoset : HDEVINFO, classguid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailA(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailW(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_A) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_sys::core::PSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_sys::core::PWSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceAlias(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid : *const windows_sys::core::GUID, aliasdeviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyKeys(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameA(hwprofile : u32, friendlyname : windows_sys::core::PSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExA(hwprofile : u32, friendlyname : windows_sys::core::PSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExW(hwprofile : u32, friendlyname : windows_sys::core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameW(hwprofile : u32, friendlyname : windows_sys::core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileList(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExA(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExW(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetINFClassA(infname : windows_sys::core::PCSTR, classguid : *mut windows_sys::core::GUID, classname : windows_sys::core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetINFClassW(infname : windows_sys::core::PCWSTR, classguid : *mut windows_sys::core::GUID, classname : windows_sys::core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiGetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiGetWizardPage(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, installwizarddata : *const SP_INSTALLWIZARD_DATA, pagetype : u32, flags : u32) -> super::prsht::HPROPSHEETPAGE);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallClassA(hwndparent : super::windef::HWND, inffilename : windows_sys::core::PCSTR, flags : u32, filequeue : HSPFILEQ) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallClassExA(hwndparent : super::windef::HWND, inffilename : windows_sys::core::PCSTR, flags : u32, filequeue : HSPFILEQ, interfaceclassguid : *const windows_sys::core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallClassExW(hwndparent : super::windef::HWND, inffilename : windows_sys::core::PCWSTR, flags : u32, filequeue : HSPFILEQ, interfaceclassguid : *const windows_sys::core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallClassW(hwndparent : super::windef::HWND, inffilename : windows_sys::core::PCWSTR, flags : u32, filequeue : HSPFILEQ) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiInstallDriverFiles(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiLoadClassIcon(classguid : *const windows_sys::core::GUID, largeicon : *mut super::windef::HICON, miniiconindex : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiLoadDeviceIcon(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, cxicon : u32, cyicon : u32, flags : u32, hicon : *mut super::windef::HICON) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKey(classguid : *const windows_sys::core::GUID, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExA(classguid : *const windows_sys::core::GUID, samdesired : super::winreg::REGSAM, flags : u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> super::minwindef::HKEY);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExW(classguid : *const windows_sys::core::GUID, samdesired : super::winreg::REGSAM, flags : u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> super::minwindef::HKEY);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoA(deviceinfoset : HDEVINFO, deviceinstanceid : windows_sys::core::PCSTR, hwndparent : super::windef::HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoW(deviceinfoset : HDEVINFO, deviceinstanceid : windows_sys::core::PCWSTR, hwndparent : super::windef::HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceA(deviceinfoset : HDEVINFO, devicepath : windows_sys::core::PCSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
windows_link::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceW(deviceinfoset : HDEVINFO, devicepath : windows_sys::core::PCWSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiRegisterCoDeviceInstallers(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiRegisterDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, flags : u32, compareproc : PSP_DETSIG_CMPPROC, comparecontext : *const core::ffi::c_void, dupdeviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiRemoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiRemoveDeviceInterface(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiRestartDevices(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSelectBestCompatDrv(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSelectDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSelectOEMDrv(hwndparent : super::windef::HWND, deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyExW(classguid : *const windows_sys::core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyW(classguid : *const windows_sys::core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyA(classguid : *const windows_sys::core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_sys::core::PCSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyW(classguid : *const windows_sys::core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_sys::core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_A) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfaceDefault(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA, flags : u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "devpropdef")]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : u32, propertybuffer : *const u8, propertybuffersize : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : u32, propertybuffer : *const u8, propertybuffersize : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiSetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDiUnremoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListA(diskspace : HDSKSPC, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
windows_link::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListW(diskspace : HDSKSPC, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
windows_link::link!("setupapi.dll" "system" fn SetupEnumInfSectionsA(infhandle : HINF, index : u32, buffer : windows_sys::core::PSTR, size : u32, sizeneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupEnumInfSectionsW(infhandle : HINF, index : u32, buffer : windows_sys::core::PWSTR, size : u32, sizeneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFindFirstLineA(infhandle : HINF, section : windows_sys::core::PCSTR, key : windows_sys::core::PCSTR, context : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFindFirstLineW(infhandle : HINF, section : windows_sys::core::PCWSTR, key : windows_sys::core::PCWSTR, context : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFindNextLine(contextin : *const INFCONTEXT, contextout : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFindNextMatchLineA(contextin : *const INFCONTEXT, key : windows_sys::core::PCSTR, contextout : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFindNextMatchLineW(contextin : *const INFCONTEXT, key : windows_sys::core::PCWSTR, contextout : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFreeSourceListA(list : *mut *mut windows_sys::core::PCSTR, count : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupFreeSourceListW(list : *mut *mut windows_sys::core::PCWSTR, count : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetBackupInformationA(queuehandle : HSPFILEQ, backupparams : PSP_BACKUP_QUEUE_PARAMS_A) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetBackupInformationW(queuehandle : HSPFILEQ, backupparams : PSP_BACKUP_QUEUE_PARAMS_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetBinaryField(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : *mut u8, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetFieldCount(context : *const INFCONTEXT) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoA(sourcefilename : windows_sys::core::PCSTR, actualsourcefilename : *mut windows_sys::core::PSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExA(sourcefilename : windows_sys::core::PCSTR, actualsourcefilenamebuffer : windows_sys::core::PCSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExW(sourcefilename : windows_sys::core::PCWSTR, actualsourcefilenamebuffer : windows_sys::core::PCWSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoW(sourcefilename : windows_sys::core::PCWSTR, actualsourcefilename : *mut windows_sys::core::PWSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileQueueCount(filequeue : HSPFILEQ, subqueuefileop : u32, numoperations : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetFileQueueFlags(filequeue : HSPFILEQ, flags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationA(filename : windows_sys::core::PCSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, localename : windows_sys::core::PCSTR, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationW(filename : windows_sys::core::PCWSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, localename : windows_sys::core::PCWSTR, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfFileListA(directorypath : windows_sys::core::PCSTR, infstyle : u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfFileListW(directorypath : windows_sys::core::PCWSTR, infstyle : u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfInformationA(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfInformationW(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameA(driverstorelocation : windows_sys::core::PCSTR, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameW(driverstorelocation : windows_sys::core::PCWSTR, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetIntField(context : *const INFCONTEXT, fieldindex : u32, integervalue : *mut i32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineByIndexA(infhandle : HINF, section : windows_sys::core::PCSTR, index : u32, context : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineByIndexW(infhandle : HINF, section : windows_sys::core::PCWSTR, index : u32, context : *mut INFCONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineCountA(infhandle : HINF, section : windows_sys::core::PCSTR) -> i32);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineCountW(infhandle : HINF, section : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineTextA(context : *const INFCONTEXT, infhandle : HINF, section : windows_sys::core::PCSTR, key : windows_sys::core::PCSTR, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetLineTextW(context : *const INFCONTEXT, infhandle : HINF, section : windows_sys::core::PCWSTR, key : windows_sys::core::PCWSTR, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetNonInteractiveMode() -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationA(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_sys::core::PCSTR, sourceid : *mut u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationW(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_sys::core::PCWSTR, sourceid : *mut u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeA(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_sys::core::PCSTR, section : windows_sys::core::PCSTR, filesize : *mut u32, roundingfactor : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeW(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_sys::core::PCWSTR, section : windows_sys::core::PCWSTR, filesize : *mut u32, roundingfactor : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceInfoA(infhandle : HINF, sourceid : u32, infodesired : u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetSourceInfoW(infhandle : HINF, sourceid : u32, infodesired : u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetStringFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetStringFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetTargetPathA(infhandle : HINF, infcontext : *const INFCONTEXT, section : windows_sys::core::PCSTR, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupGetTargetPathW(infhandle : HINF, infcontext : *const INFCONTEXT, section : windows_sys::core::PCWSTR, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "spapidef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupGetThreadLogToken() -> super::spapidef::SP_LOG_TOKEN);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallback(ownerwindow : super::windef::HWND) -> *mut core::ffi::c_void);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallbackEx(ownerwindow : super::windef::HWND, alternateprogresswindow : super::windef::HWND, progressmessage : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("setupapi.dll" "system" fn SetupInitializeFileLogA(logfilename : windows_sys::core::PCSTR, flags : u32) -> HSPFILELOG);
windows_link::link!("setupapi.dll" "system" fn SetupInitializeFileLogW(logfilename : windows_sys::core::PCWSTR, flags : u32) -> HSPFILELOG);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFileA(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_sys::core::PCSTR, sourcepathroot : windows_sys::core::PCSTR, destinationname : windows_sys::core::PCSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFileExA(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_sys::core::PCSTR, sourcepathroot : windows_sys::core::PCSTR, destinationname : windows_sys::core::PCSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, filewasinuse : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFileExW(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_sys::core::PCWSTR, sourcepathroot : windows_sys::core::PCWSTR, destinationname : windows_sys::core::PCWSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, filewasinuse : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFileW(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_sys::core::PCWSTR, sourcepathroot : windows_sys::core::PCWSTR, destinationname : windows_sys::core::PCWSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionA(infhandle : HINF, layoutinfhandle : HINF, filequeue : HSPFILEQ, sectionname : windows_sys::core::PCSTR, sourcerootpath : windows_sys::core::PCSTR, copyflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionW(infhandle : HINF, layoutinfhandle : HINF, filequeue : HSPFILEQ, sectionname : windows_sys::core::PCWSTR, sourcerootpath : windows_sys::core::PCWSTR, copyflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionA(owner : super::windef::HWND, infhandle : HINF, sectionname : windows_sys::core::PCSTR, flags : u32, relativekeyroot : super::minwindef::HKEY, sourcerootpath : windows_sys::core::PCSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionW(owner : super::windef::HWND, infhandle : HINF, sectionname : windows_sys::core::PCWSTR, flags : u32, relativekeyroot : super::minwindef::HKEY, sourcerootpath : windows_sys::core::PCWSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionA(infhandle : HINF, sectionname : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExA(infhandle : HINF, sectionname : windows_sys::core::PCSTR, flags : u32, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExW(infhandle : HINF, sectionname : windows_sys::core::PCWSTR, flags : u32, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionW(infhandle : HINF, sectionname : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupIterateCabinetA(cabinetfile : windows_sys::core::PCSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupIterateCabinetW(cabinetfile : windows_sys::core::PCWSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupLogErrorA(messagestring : windows_sys::core::PCSTR, severity : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupLogErrorW(messagestring : windows_sys::core::PCWSTR, severity : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupLogFileA(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCSTR, sourcefilename : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR, checksum : u32, disktagfile : windows_sys::core::PCSTR, diskdescription : windows_sys::core::PCSTR, otherinfo : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupLogFileW(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCWSTR, sourcefilename : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR, checksum : u32, disktagfile : windows_sys::core::PCWSTR, diskdescription : windows_sys::core::PCWSTR, otherinfo : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileA(filename : windows_sys::core::PCSTR, infhandle : HINF, errorline : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileW(filename : windows_sys::core::PCWSTR, infhandle : HINF, errorline : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupOpenFileQueue() -> HSPFILEQ);
windows_link::link!("setupapi.dll" "system" fn SetupOpenInfFileA(filename : windows_sys::core::PCSTR, infclass : windows_sys::core::PCSTR, infstyle : u32, errorline : *mut u32) -> HINF);
windows_link::link!("setupapi.dll" "system" fn SetupOpenInfFileW(filename : windows_sys::core::PCWSTR, infclass : windows_sys::core::PCWSTR, infstyle : u32, errorline : *mut u32) -> HINF);
windows_link::link!("setupapi.dll" "system" fn SetupOpenLog(erase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupOpenMasterInf() -> HINF);
windows_link::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreA(queuehandle : HSPFILEQ, backuppath : windows_sys::core::PCSTR, restoreflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreW(queuehandle : HSPFILEQ, backuppath : windows_sys::core::PCWSTR, restoreflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupPromptForDiskA(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCSTR, diskname : windows_sys::core::PCSTR, pathtosource : windows_sys::core::PCSTR, filesought : windows_sys::core::PCSTR, tagfile : windows_sys::core::PCSTR, diskpromptstyle : u32, pathbuffer : windows_sys::core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupPromptForDiskW(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCWSTR, diskname : windows_sys::core::PCWSTR, pathtosource : windows_sys::core::PCWSTR, filesought : windows_sys::core::PCWSTR, tagfile : windows_sys::core::PCWSTR, diskpromptstyle : u32, pathbuffer : windows_sys::core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupPromptReboot(filequeue : HSPFILEQ, owner : super::windef::HWND, scanonly : windows_sys::core::BOOL) -> i32);
windows_link::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListA(diskspace : HDSKSPC, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListW(diskspace : HDSKSPC, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryFileLogA(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR, desiredinfo : SetupFileLogInfo, dataout : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryFileLogW(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR, desiredinfo : SetupFileLogInfo, dataout : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : PSP_ALTPLATFORM_INFO, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_A) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : PSP_ALTPLATFORM_INFO, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_sys::core::PCSTR, returnbuffer : windows_sys::core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_sys::core::PCWSTR, returnbuffer : windows_sys::core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQuerySourceListA(flags : u32, list : *mut *mut windows_sys::core::PCSTR, count : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQuerySourceListW(flags : u32, list : *mut *mut windows_sys::core::PCWSTR, count : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveA(diskspace : HDSKSPC, drivespec : windows_sys::core::PCSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveW(diskspace : HDSKSPC, drivespec : windows_sys::core::PCWSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopyA(queuehandle : HSPFILEQ, sourcerootpath : windows_sys::core::PCSTR, sourcepath : windows_sys::core::PCSTR, sourcefilename : windows_sys::core::PCSTR, sourcedescription : windows_sys::core::PCSTR, sourcetagfile : windows_sys::core::PCSTR, targetdirectory : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectA(copyparams : *const SP_FILE_COPY_PARAMS_A) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectW(copyparams : *const SP_FILE_COPY_PARAMS_W) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopySectionA(queuehandle : HSPFILEQ, sourcerootpath : windows_sys::core::PCSTR, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopySectionW(queuehandle : HSPFILEQ, sourcerootpath : windows_sys::core::PCWSTR, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCWSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueCopyW(queuehandle : HSPFILEQ, sourcerootpath : windows_sys::core::PCWSTR, sourcepath : windows_sys::core::PCWSTR, sourcefilename : windows_sys::core::PCWSTR, sourcedescription : windows_sys::core::PCWSTR, sourcetagfile : windows_sys::core::PCWSTR, targetdirectory : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyA(queuehandle : HSPFILEQ, infhandle : HINF, sourcerootpath : windows_sys::core::PCSTR, sourcefilename : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyW(queuehandle : HSPFILEQ, infhandle : HINF, sourcerootpath : windows_sys::core::PCWSTR, sourcefilename : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR, copystyle : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDeleteA(queuehandle : HSPFILEQ, pathpart1 : windows_sys::core::PCSTR, pathpart2 : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionA(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionW(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueDeleteW(queuehandle : HSPFILEQ, pathpart1 : windows_sys::core::PCWSTR, pathpart2 : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueRenameA(queuehandle : HSPFILEQ, sourcepath : windows_sys::core::PCSTR, sourcefilename : windows_sys::core::PCSTR, targetpath : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueRenameSectionA(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueRenameSectionW(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupQueueRenameW(queuehandle : HSPFILEQ, sourcepath : windows_sys::core::PCWSTR, sourcefilename : windows_sys::core::PCWSTR, targetpath : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryA(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCSTR, targetfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryW(fileloghandle : HSPFILELOG, logsectionname : windows_sys::core::PCWSTR, targetfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListA(diskspace : HDSKSPC, targetfilespec : windows_sys::core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListW(diskspace : HDSKSPC, targetfilespec : windows_sys::core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListA(flags : u32, source : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListW(flags : u32, source : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_sys::core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_sys::core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_sys::core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_sys::core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupRenameErrorA(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCSTR, sourcefile : windows_sys::core::PCSTR, targetfile : windows_sys::core::PCSTR, win32errorcode : u32, style : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupRenameErrorW(hwndparent : super::windef::HWND, dialogtitle : windows_sys::core::PCWSTR, sourcefile : windows_sys::core::PCWSTR, targetfile : windows_sys::core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupScanFileQueueA(filequeue : HSPFILEQ, flags : u32, window : super::windef::HWND, callbackroutine : PSP_FILE_CALLBACK_A, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("setupapi.dll" "system" fn SetupScanFileQueueW(filequeue : HSPFILEQ, flags : u32, window : super::windef::HWND, callbackroutine : PSP_FILE_CALLBACK_W, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetDirectoryIdA(infhandle : HINF, id : u32, directory : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExA(infhandle : HINF, id : u32, directory : windows_sys::core::PCSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExW(infhandle : HINF, id : u32, directory : windows_sys::core::PCWSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetDirectoryIdW(infhandle : HINF, id : u32, directory : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformA(queuehandle : HSPFILEQ, alternateplatforminfo : PSP_ALTPLATFORM_INFO, alternatedefaultcatalogfile : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformW(queuehandle : HSPFILEQ, alternateplatforminfo : PSP_ALTPLATFORM_INFO, alternatedefaultcatalogfile : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetFileQueueFlags(filequeue : HSPFILEQ, flagmask : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetNonInteractiveMode(noninteractiveflag : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideA(r#override : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideW(r#override : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetSourceListA(flags : u32, sourcelist : *const windows_sys::core::PCSTR, sourcecount : u32) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupSetSourceListW(flags : u32, sourcelist : *const windows_sys::core::PCWSTR, sourcecount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "spapidef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupSetThreadLogToken(logtoken : super::spapidef::SP_LOG_TOKEN));
windows_link::link!("setupapi.dll" "system" fn SetupTermDefaultQueueCallback(context : *const core::ffi::c_void));
windows_link::link!("setupapi.dll" "system" fn SetupTerminateFileLog(fileloghandle : HSPFILELOG) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupUninstallNewlyCopiedInfs(filequeue : HSPFILEQ, flags : u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupUninstallOEMInfA(inffilename : windows_sys::core::PCSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupUninstallOEMInfW(inffilename : windows_sys::core::PCWSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupVerifyInfFileA(infname : windows_sys::core::PCSTR, altplatforminfo : PSP_ALTPLATFORM_INFO, infsignerinfo : PSP_INF_SIGNER_INFO_A) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn SetupVerifyInfFileW(infname : windows_sys::core::PCWSTR, altplatforminfo : PSP_ALTPLATFORM_INFO, infsignerinfo : PSP_INF_SIGNER_INFO_W) -> windows_sys::core::BOOL);
#[cfg(all(feature = "spapidef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "C" fn SetupWriteTextLog(logtoken : super::spapidef::SP_LOG_TOKEN, category : u32, flags : u32, messagestr : windows_sys::core::PCSTR, ...));
#[cfg(all(feature = "spapidef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "C" fn SetupWriteTextLogError(logtoken : super::spapidef::SP_LOG_TOKEN, category : u32, logflags : u32, error : u32, messagestr : windows_sys::core::PCSTR, ...));
#[cfg(all(feature = "spapidef", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn SetupWriteTextLogInfLine(logtoken : super::spapidef::SP_LOG_TOKEN, flags : u32, infhandle : HINF, context : *const INFCONTEXT));
pub type CABINET_INFO = CABINET_INFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_sys::core::PCSTR,
    pub CabinetFile: windows_sys::core::PCSTR,
    pub DiskName: windows_sys::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl Default for CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_sys::core::PCSTR,
    pub CabinetFile: windows_sys::core::PCSTR,
    pub DiskName: windows_sys::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_sys::core::PCWSTR,
    pub CabinetFile: windows_sys::core::PCWSTR,
    pub DiskName: windows_sys::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl Default for CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_sys::core::PCWSTR,
    pub CabinetFile: windows_sys::core::PCWSTR,
    pub DiskName: windows_sys::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_sys::core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_sys::core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COPYFLG_FORCE_FILE_IN_USE: u32 = 8;
pub const COPYFLG_IN_USE_TRY_RENAME: u32 = 16384;
pub const COPYFLG_NODECOMP: u32 = 2048;
pub const COPYFLG_NOPRUNE: u32 = 8192;
pub const COPYFLG_NOSKIP: u32 = 2;
pub const COPYFLG_NOVERSIONCHECK: u32 = 4;
pub const COPYFLG_NO_OVERWRITE: u32 = 16;
pub const COPYFLG_NO_VERSION_DIALOG: u32 = 32;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: u32 = 64;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: u32 = 256;
pub const COPYFLG_REPLACEONLY: u32 = 1024;
pub const COPYFLG_REPLACE_BOOT_FILE: u32 = 4096;
pub const COPYFLG_WARN_IF_SKIP: u32 = 1;
pub const DELFLG_IN_USE: u32 = 1;
pub const DELFLG_IN_USE1: u32 = 65536;
pub const DIBCI_NODISPLAYCLASS: u32 = 2;
pub const DIBCI_NOINSTALLCLASS: u32 = 1;
pub const DICD_GENERATE_ID: u32 = 1;
pub const DICD_INHERIT_CLASSDRVS: u32 = 2;
pub const DICLASSPROP_INSTALLER: u32 = 1;
pub const DICLASSPROP_INTERFACE: u32 = 2;
pub const DICS_DISABLE: u32 = 2;
pub const DICS_ENABLE: u32 = 1;
pub const DICS_FLAG_CONFIGGENERAL: u32 = 4;
pub const DICS_FLAG_CONFIGSPECIFIC: u32 = 2;
pub const DICS_FLAG_GLOBAL: u32 = 1;
pub const DICS_PROPCHANGE: u32 = 3;
pub const DICS_START: u32 = 4;
pub const DICS_STOP: u32 = 5;
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: u32 = 35;
pub const DIF_ADDPROPERTYPAGE_BASIC: u32 = 36;
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: u32 = 40;
pub const DIF_ALLOW_INSTALL: u32 = 24;
pub const DIF_ASSIGNRESOURCES: u32 = 3;
pub const DIF_CALCDISKSPACE: u32 = 11;
pub const DIF_DESTROYPRIVATEDATA: u32 = 12;
pub const DIF_DESTROYWIZARDDATA: u32 = 17;
pub const DIF_DETECT: u32 = 15;
pub const DIF_DETECTCANCEL: u32 = 33;
pub const DIF_DETECTVERIFY: u32 = 20;
pub const DIF_ENABLECLASS: u32 = 19;
pub const DIF_FINISHINSTALL_ACTION: u32 = 42;
pub const DIF_FIRSTTIMESETUP: u32 = 6;
pub const DIF_FOUNDDEVICE: u32 = 7;
pub const DIF_INSTALLCLASSDRIVERS: u32 = 10;
pub const DIF_INSTALLDEVICE: u32 = 2;
pub const DIF_INSTALLDEVICEFILES: u32 = 21;
pub const DIF_INSTALLINTERFACES: u32 = 32;
pub const DIF_INSTALLWIZARD: u32 = 16;
pub const DIF_MOVEDEVICE: u32 = 14;
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: u32 = 30;
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: u32 = 29;
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: u32 = 28;
pub const DIF_NEWDEVICEWIZARD_PRESELECT: u32 = 26;
pub const DIF_NEWDEVICEWIZARD_SELECT: u32 = 27;
pub const DIF_POWERMESSAGEWAKE: u32 = 39;
pub const DIF_PROPERTIES: u32 = 4;
pub const DIF_PROPERTYCHANGE: u32 = 18;
pub const DIF_REGISTERDEVICE: u32 = 25;
pub const DIF_REGISTER_COINSTALLERS: u32 = 34;
pub const DIF_REMOVE: u32 = 5;
pub const DIF_RESERVED1: u32 = 37;
pub const DIF_RESERVED2: u32 = 48;
pub const DIF_SELECTBESTCOMPATDRV: u32 = 23;
pub const DIF_SELECTCLASSDRIVERS: u32 = 8;
pub const DIF_SELECTDEVICE: u32 = 1;
pub const DIF_TROUBLESHOOTER: u32 = 38;
pub const DIF_UNREMOVE: u32 = 22;
pub const DIF_UNUSED1: u32 = 31;
pub const DIF_UPDATEDRIVER_UI: u32 = 41;
pub const DIF_VALIDATECLASSDRIVERS: u32 = 9;
pub const DIF_VALIDATEDRIVER: u32 = 13;
pub const DIGCDP_FLAG_ADVANCED: u32 = 2;
pub const DIGCDP_FLAG_BASIC: u32 = 1;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: u32 = 4;
pub const DIGCDP_FLAG_REMOTE_BASIC: u32 = 3;
pub const DIGCF_ALLCLASSES: u32 = 4;
pub const DIGCF_DEFAULT: u32 = 1;
pub const DIGCF_DEVICEINTERFACE: u32 = 16;
pub const DIGCF_INTERFACEDEVICE: u32 = 16;
pub const DIGCF_PRESENT: u32 = 2;
pub const DIGCF_PROFILE: u32 = 8;
pub const DIOCR_INSTALLER: u32 = 1;
pub const DIOCR_INTERFACE: u32 = 2;
pub const DIODI_NO_ADD: u32 = 1;
pub const DIOD_CANCEL_REMOVE: u32 = 4;
pub const DIOD_INHERIT_CLASSDRVS: u32 = 2;
pub const DIREG_BOTH: u32 = 4;
pub const DIREG_DEV: u32 = 1;
pub const DIREG_DRV: u32 = 2;
pub const DIRID_ABSOLUTE: i32 = -1;
pub const DIRID_ABSOLUTE_16BIT: u32 = 65535;
pub const DIRID_APPS: u32 = 24;
pub const DIRID_BOOT: u32 = 30;
pub const DIRID_COLOR: u32 = 23;
pub const DIRID_COMMON_APPDATA: u32 = 16419;
pub const DIRID_COMMON_DESKTOPDIRECTORY: u32 = 16409;
pub const DIRID_COMMON_DOCUMENTS: u32 = 16430;
pub const DIRID_COMMON_FAVORITES: u32 = 16415;
pub const DIRID_COMMON_PROGRAMS: u32 = 16407;
pub const DIRID_COMMON_STARTMENU: u32 = 16406;
pub const DIRID_COMMON_STARTUP: u32 = 16408;
pub const DIRID_COMMON_TEMPLATES: u32 = 16429;
pub const DIRID_DEFAULT: u32 = 11;
pub const DIRID_DRIVERS: u32 = 12;
pub const DIRID_DRIVER_STORE: u32 = 13;
pub const DIRID_FONTS: u32 = 20;
pub const DIRID_HELP: u32 = 18;
pub const DIRID_INF: u32 = 17;
pub const DIRID_IOSUBSYS: u32 = 12;
pub const DIRID_LOADER: u32 = 54;
pub const DIRID_NULL: u32 = 0;
pub const DIRID_PRINTPROCESSOR: u32 = 55;
pub const DIRID_PROGRAM_FILES: u32 = 16422;
pub const DIRID_PROGRAM_FILES_COMMON: u32 = 16427;
pub const DIRID_PROGRAM_FILES_COMMONX86: u32 = 16428;
pub const DIRID_PROGRAM_FILES_X86: u32 = 16426;
pub const DIRID_SHARED: u32 = 25;
pub const DIRID_SPOOL: u32 = 51;
pub const DIRID_SPOOLDRIVERS: u32 = 52;
pub const DIRID_SRCPATH: u32 = 1;
pub const DIRID_SYSTEM: u32 = 11;
pub const DIRID_SYSTEM16: u32 = 50;
pub const DIRID_SYSTEM_X86: u32 = 16425;
pub const DIRID_USER: u32 = 32768;
pub const DIRID_USERPROFILE: u32 = 53;
pub const DIRID_VIEWERS: u32 = 21;
pub const DIRID_WINDOWS: u32 = 10;
pub const DI_AUTOASSIGNRES: u32 = 64;
pub const DI_CLASSINSTALLPARAMS: u32 = 1048576;
pub const DI_COMPAT_FROM_CLASS: u32 = 524288;
pub const DI_DIDCLASS: u32 = 32;
pub const DI_DIDCOMPAT: u32 = 16;
pub const DI_DISABLED: u32 = 2048;
pub const DI_DONOTCALLCONFIGMG: u32 = 131072;
pub const DI_DRIVERPAGE_ADDED: u32 = 67108864;
pub const DI_ENUMSINGLEINF: u32 = 65536;
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: u32 = 2048;
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: u32 = 268435456;
pub const DI_FLAGSEX_ALWAYSWRITEIDS: u32 = 512;
pub const DI_FLAGSEX_APPENDDRIVERLIST: u32 = 262144;
pub const DI_FLAGSEX_BACKUPONREPLACE: u32 = 1048576;
pub const DI_FLAGSEX_CI_FAILED: u32 = 4;
pub const DI_FLAGSEX_DEVICECHANGE: u32 = 256;
pub const DI_FLAGSEX_DIDCOMPATINFO: u32 = 32;
pub const DI_FLAGSEX_DIDINFOLIST: u32 = 16;
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: u32 = 2097152;
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: u32 = 8388608;
pub const DI_FLAGSEX_FILTERCLASSES: u32 = 64;
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: u32 = 33554432;
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: u32 = 8;
pub const DI_FLAGSEX_INET_DRIVER: u32 = 131072;
pub const DI_FLAGSEX_INSTALLEDDRIVER: u32 = 67108864;
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: u32 = 65536;
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: u32 = 4096;
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: u32 = 134217728;
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: u32 = 32768;
pub const DI_FLAGSEX_POWERPAGE_ADDED: u32 = 16777216;
pub const DI_FLAGSEX_PREINSTALLBACKUP: u32 = 524288;
pub const DI_FLAGSEX_PROPCHANGE_PENDING: u32 = 1024;
pub const DI_FLAGSEX_RECURSIVESEARCH: u32 = 1073741824;
pub const DI_FLAGSEX_RESERVED1: u32 = 4194304;
pub const DI_FLAGSEX_RESERVED2: u32 = 1;
pub const DI_FLAGSEX_RESERVED3: u32 = 2;
pub const DI_FLAGSEX_RESERVED4: u32 = 16384;
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: u32 = 536870912;
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: u32 = 2147483648;
pub const DI_FLAGSEX_SETFAILEDINSTALL: u32 = 128;
pub const DI_FLAGSEX_USECLASSFORCOMPAT: u32 = 8192;
pub const DI_FORCECOPY: u32 = 33554432;
pub type DI_FUNCTION = u32;
pub const DI_GENERALPAGE_ADDED: u32 = 4096;
pub const DI_INF_IS_SORTED: u32 = 32768;
pub const DI_INSTALLDISABLED: u32 = 262144;
pub const DI_MULTMFGS: u32 = 1024;
pub const DI_NEEDREBOOT: u32 = 256;
pub const DI_NEEDRESTART: u32 = 128;
pub const DI_NOBROWSE: u32 = 512;
pub const DI_NODI_DEFAULTACTION: u32 = 2097152;
pub const DI_NOFILECOPY: u32 = 16777216;
pub const DI_NOSELECTICONS: u32 = 1073741824;
pub const DI_NOVCP: u32 = 8;
pub const DI_NOWRITE_IDS: u32 = 2147483648;
pub const DI_OVERRIDE_INFFLAGS: u32 = 268435456;
pub const DI_PROPERTIES_CHANGE: u32 = 16384;
pub const DI_PROPS_NOCHANGEUSAGE: u32 = 536870912;
pub const DI_QUIETINSTALL: u32 = 8388608;
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: u32 = 2;
pub const DI_REMOVEDEVICE_GLOBAL: u32 = 1;
pub const DI_RESOURCEPAGE_ADDED: u32 = 8192;
pub const DI_SHOWALL: u32 = 7;
pub const DI_SHOWCLASS: u32 = 4;
pub const DI_SHOWCOMPAT: u32 = 2;
pub const DI_SHOWOEM: u32 = 1;
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: u32 = 2;
pub const DI_USECI_SELECTSTRINGS: u32 = 134217728;
pub const DMI_BKCOLOR: u32 = 2;
pub const DMI_MASK: u32 = 1;
pub const DMI_USERECT: u32 = 4;
pub const DNF_ALWAYSEXCLUDEFROMLIST: u32 = 524288;
pub const DNF_AUTHENTICODE_SIGNED: u32 = 131072;
pub const DNF_BAD_DRIVER: u32 = 2048;
pub const DNF_BASIC_DRIVER: u32 = 65536;
pub const DNF_CLASS_DRIVER: u32 = 32;
pub const DNF_COMPATIBLE_DRIVER: u32 = 64;
pub const DNF_DUPDESC: u32 = 1;
pub const DNF_DUPDRIVERVER: u32 = 32768;
pub const DNF_DUPPROVIDER: u32 = 4096;
pub const DNF_EXCLUDEFROMLIST: u32 = 4;
pub const DNF_INBOX_DRIVER: u32 = 1048576;
pub const DNF_INET_DRIVER: u32 = 128;
pub const DNF_INF_IS_SIGNED: u32 = 8192;
pub const DNF_INSTALLEDDRIVER: u32 = 262144;
pub const DNF_LEGACYINF: u32 = 16;
pub const DNF_NODRIVER: u32 = 8;
pub const DNF_OEM_F6_INF: u32 = 16384;
pub const DNF_OLDDRIVER: u32 = 2;
pub const DNF_OLD_INET_DRIVER: u32 = 1024;
pub const DNF_REQUESTADDITIONALSOFTWARE: u32 = 2097152;
pub const DNF_UNUSED1: u32 = 256;
pub const DNF_UNUSED2: u32 = 512;
pub const DNF_UNUSED_22: u32 = 4194304;
pub const DNF_UNUSED_23: u32 = 8388608;
pub const DNF_UNUSED_24: u32 = 16777216;
pub const DNF_UNUSED_25: u32 = 33554432;
pub const DNF_UNUSED_26: u32 = 67108864;
pub const DNF_UNUSED_27: u32 = 134217728;
pub const DNF_UNUSED_28: u32 = 268435456;
pub const DNF_UNUSED_29: u32 = 536870912;
pub const DNF_UNUSED_30: u32 = 1073741824;
pub const DNF_UNUSED_31: u32 = 2147483648;
pub const DPROMPT_BUFFERTOOSMALL: u32 = 3;
pub const DPROMPT_CANCEL: u32 = 1;
pub const DPROMPT_OUTOFMEMORY: u32 = 4;
pub const DPROMPT_SKIPFILE: u32 = 2;
pub const DPROMPT_SUCCESS: u32 = 0;
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743;
pub const DRIVER_HARDWAREID_RANK: u32 = 4095;
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648;
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: u32 = 8;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: u32 = 2;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: u32 = 4;
pub const DYNAWIZ_FLAG_PAGESADDED: u32 = 1;
pub const ENABLECLASS_FAILURE: u32 = 2;
pub const ENABLECLASS_QUERY: u32 = 0;
pub const ENABLECLASS_SUCCESS: u32 = 1;
pub const ERROR_AUTHENTICODE_DISALLOWED: i32 = -536870336;
pub const ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: i32 = -536870333;
pub const ERROR_AUTHENTICODE_TRUSTED_PUBLISHER: i32 = -536870335;
pub const ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED: i32 = -536870334;
pub const ERROR_BAD_INTERFACE_INSTALLSECT: i32 = -536870371;
pub const ERROR_BAD_SECTION_NAME_LINE: i32 = -536870911;
pub const ERROR_BAD_SERVICE_INSTALLSECT: i32 = -536870377;
pub const ERROR_CANT_LOAD_CLASS_ICON: i32 = -536870388;
pub const ERROR_CANT_REMOVE_DEVINST: i32 = -536870350;
pub const ERROR_CLASS_MISMATCH: i32 = -536870399;
pub const ERROR_DEVICE_INSTALLER_NOT_READY: i32 = -536870330;
pub const ERROR_DEVICE_INSTALL_BLOCKED: i32 = -536870328;
pub const ERROR_DEVICE_INTERFACE_ACTIVE: i32 = -536870373;
pub const ERROR_DEVICE_INTERFACE_REMOVED: i32 = -536870372;
pub const ERROR_DEVINFO_DATA_LOCKED: i32 = -536870381;
pub const ERROR_DEVINFO_LIST_LOCKED: i32 = -536870382;
pub const ERROR_DEVINFO_NOT_REGISTERED: i32 = -536870392;
pub const ERROR_DEVINSTALL_QUEUE_NONNATIVE: i32 = -536870352;
pub const ERROR_DEVINST_ALREADY_EXISTS: i32 = -536870393;
pub const ERROR_DI_BAD_PATH: i32 = -536870380;
pub const ERROR_DI_DONT_INSTALL: i32 = -536870357;
pub const ERROR_DI_DO_DEFAULT: i32 = -536870386;
pub const ERROR_DI_FUNCTION_OBSOLETE: i32 = -536870338;
pub const ERROR_DI_NOFILECOPY: i32 = -536870385;
pub const ERROR_DI_POSTPROCESSING_REQUIRED: i32 = -536870362;
pub const ERROR_DRIVER_INSTALL_BLOCKED: i32 = -536870327;
pub const ERROR_DRIVER_NONNATIVE: i32 = -536870348;
pub const ERROR_DRIVER_STORE_ADD_FAILED: i32 = -536870329;
pub const ERROR_DRIVER_STORE_DELETE_FAILED: i32 = -536870324;
pub const ERROR_DUPLICATE_FOUND: i32 = -536870398;
pub const ERROR_EXPECTED_SECTION_NAME: i32 = -536870912;
pub const ERROR_FILEQUEUE_LOCKED: i32 = -536870378;
pub const ERROR_FILE_HASH_NOT_IN_CATALOG: i32 = -536870325;
pub const ERROR_GENERAL_SYNTAX: i32 = -536870909;
pub const ERROR_INF_IN_USE_BY_DEVICES: i32 = -536870339;
pub const ERROR_INTERFACE_DEVICE_ACTIVE: i32 = -536870373;
pub const ERROR_INTERFACE_DEVICE_REMOVED: i32 = -536870372;
pub const ERROR_INVALID_CLASS: i32 = -536870394;
pub const ERROR_INVALID_CLASS_INSTALLER: i32 = -536870387;
pub const ERROR_INVALID_COINSTALLER: i32 = -536870361;
pub const ERROR_INVALID_DEVINST_NAME: i32 = -536870395;
pub const ERROR_INVALID_FILTER_DRIVER: i32 = -536870356;
pub const ERROR_INVALID_HWPROFILE: i32 = -536870384;
pub const ERROR_INVALID_INF_LOGCONFIG: i32 = -536870358;
pub const ERROR_INVALID_MACHINENAME: i32 = -536870368;
pub const ERROR_INVALID_PROPPAGE_PROVIDER: i32 = -536870364;
pub const ERROR_INVALID_REFERENCE_STRING: i32 = -536870369;
pub const ERROR_INVALID_REG_PROPERTY: i32 = -536870391;
pub const ERROR_INVALID_TARGET: i32 = -536870349;
pub const ERROR_IN_WOW64: i32 = -536870347;
pub const ERROR_KEY_DOES_NOT_EXIST: i32 = -536870396;
pub const ERROR_LINE_NOT_FOUND: i32 = -536870654;
pub const ERROR_MACHINE_UNAVAILABLE: i32 = -536870366;
pub const ERROR_NON_WINDOWS_DRIVER: i32 = -536870354;
pub const ERROR_NON_WINDOWS_NT_DRIVER: i32 = -536870355;
pub const ERROR_NOT_AN_INSTALLED_OEM_INF: i32 = -536870340;
pub const ERROR_NOT_DISABLEABLE: i32 = -536870351;
pub const ERROR_NOT_INSTALLED: i32 = -536866816;
pub const ERROR_NO_ASSOCIATED_CLASS: i32 = -536870400;
pub const ERROR_NO_ASSOCIATED_SERVICE: i32 = -536870375;
pub const ERROR_NO_AUTHENTICODE_CATALOG: i32 = -536870337;
pub const ERROR_NO_BACKUP: i32 = -536870653;
pub const ERROR_NO_CATALOG_FOR_OEM_INF: i32 = -536870353;
pub const ERROR_NO_CLASSINSTALL_PARAMS: i32 = -536870379;
pub const ERROR_NO_CLASS_DRIVER_LIST: i32 = -536870376;
pub const ERROR_NO_COMPAT_DRIVERS: i32 = -536870360;
pub const ERROR_NO_CONFIGMGR_SERVICES: i32 = -536870365;
pub const ERROR_NO_DEFAULT_DEVICE_INTERFACE: i32 = -536870374;
pub const ERROR_NO_DEFAULT_INTERFACE_DEVICE: i32 = -536870374;
pub const ERROR_NO_DEVICE_ICON: i32 = -536870359;
pub const ERROR_NO_DEVICE_SELECTED: i32 = -536870383;
pub const ERROR_NO_DRIVER_SELECTED: i32 = -536870397;
pub const ERROR_NO_INF: i32 = -536870390;
pub const ERROR_NO_SUCH_DEVICE_INTERFACE: i32 = -536870363;
pub const ERROR_NO_SUCH_DEVINST: i32 = -536870389;
pub const ERROR_NO_SUCH_INTERFACE_CLASS: i32 = -536870370;
pub const ERROR_NO_SUCH_INTERFACE_DEVICE: i32 = -536870363;
pub const ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE: i32 = -536870331;
pub const ERROR_PNP_REGISTRY_ERROR: i32 = -536870342;
pub const ERROR_REMOTE_COMM_FAILURE: i32 = -536870367;
pub const ERROR_REMOTE_REQUEST_UNSUPPORTED: i32 = -536870341;
pub const ERROR_SCE_DISABLED: i32 = -536870344;
pub const ERROR_SECTION_NAME_TOO_LONG: i32 = -536870910;
pub const ERROR_SECTION_NOT_FOUND: i32 = -536870655;
pub const ERROR_SET_SYSTEM_RESTORE_POINT: i32 = -536870346;
pub const ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH: i32 = -536870332;
pub const ERROR_UNKNOWN_EXCEPTION: i32 = -536870343;
pub const ERROR_UNRECOVERABLE_STACK_OVERFLOW: i32 = -536870144;
pub const ERROR_WRONG_INF_STYLE: i32 = -536870656;
pub const ERROR_WRONG_INF_TYPE: i32 = -536870326;
pub const EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW: i32 = -536870144;
pub const FILEOP_ABORT: u32 = 0;
pub const FILEOP_BACKUP: u32 = 3;
pub const FILEOP_COPY: u32 = 0;
pub const FILEOP_DELETE: u32 = 2;
pub const FILEOP_DOIT: u32 = 1;
pub const FILEOP_NEWPATH: u32 = 4;
pub const FILEOP_RENAME: u32 = 1;
pub const FILEOP_RETRY: u32 = 1;
pub const FILEOP_SKIP: u32 = 2;
pub type FILEPATHS = FILEPATHS_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILEPATHS_A {
    pub Target: windows_sys::core::PCSTR,
    pub Source: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl Default for FILEPATHS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILEPATHS_A {
    pub Target: windows_sys::core::PCSTR,
    pub Source: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILEPATHS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILEPATHS_SIGNERINFO = FILEPATHS_SIGNERINFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_sys::core::PCSTR,
    pub Source: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_sys::core::PCSTR,
    pub Version: windows_sys::core::PCSTR,
    pub CatalogFile: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FILEPATHS_SIGNERINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_sys::core::PCSTR,
    pub Source: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_sys::core::PCSTR,
    pub Version: windows_sys::core::PCSTR,
    pub CatalogFile: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILEPATHS_SIGNERINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_sys::core::PCWSTR,
    pub Source: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_sys::core::PCWSTR,
    pub Version: windows_sys::core::PCWSTR,
    pub CatalogFile: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FILEPATHS_SIGNERINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_sys::core::PCWSTR,
    pub Source: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_sys::core::PCWSTR,
    pub Version: windows_sys::core::PCWSTR,
    pub CatalogFile: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILEPATHS_SIGNERINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILEPATHS_W {
    pub Target: windows_sys::core::PCWSTR,
    pub Source: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl Default for FILEPATHS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILEPATHS_W {
    pub Target: windows_sys::core::PCWSTR,
    pub Source: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILEPATHS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_COMPRESSION_MSZIP: u32 = 2;
pub const FILE_COMPRESSION_NONE: u32 = 0;
pub const FILE_COMPRESSION_NTCAB: u32 = 3;
pub const FILE_COMPRESSION_WINLZA: u32 = 1;
pub type FILE_IN_CABINET_INFO = FILE_IN_CABINET_INFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_sys::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_sys::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: windows_sys::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: windows_sys::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLG_ADDPROPERTY_AND: u32 = 16;
pub const FLG_ADDPROPERTY_APPEND: u32 = 4;
pub const FLG_ADDPROPERTY_NOCLOBBER: u32 = 1;
pub const FLG_ADDPROPERTY_OR: u32 = 8;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: u32 = 2;
pub const FLG_ADDREG_32BITKEY: u32 = 16384;
pub const FLG_ADDREG_64BITKEY: u32 = 4096;
pub const FLG_ADDREG_APPEND: u32 = 8;
pub const FLG_ADDREG_BINVALUETYPE: u32 = 1;
pub const FLG_ADDREG_DELREG_BIT: u32 = 32768;
pub const FLG_ADDREG_DELVAL: u32 = 4;
pub const FLG_ADDREG_KEYONLY: u32 = 16;
pub const FLG_ADDREG_KEYONLY_COMMON: u32 = 8192;
pub const FLG_ADDREG_NOCLOBBER: u32 = 2;
pub const FLG_ADDREG_OVERWRITEONLY: u32 = 32;
pub const FLG_ADDREG_TYPE_BINARY: u32 = 1;
pub const FLG_ADDREG_TYPE_DWORD: u32 = 65537;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: u32 = 131072;
pub const FLG_ADDREG_TYPE_MASK: i32 = -65535;
pub const FLG_ADDREG_TYPE_MULTI_SZ: u32 = 65536;
pub const FLG_ADDREG_TYPE_NONE: u32 = 131073;
pub const FLG_ADDREG_TYPE_QWORD: u32 = 720897;
pub const FLG_ADDREG_TYPE_SZ: u32 = 0;
pub const FLG_BITREG_32BITKEY: u32 = 16384;
pub const FLG_BITREG_64BITKEY: u32 = 4096;
pub const FLG_BITREG_CLEARBITS: u32 = 0;
pub const FLG_BITREG_SETBITS: u32 = 1;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: u32 = 1;
pub const FLG_DELREG_32BITKEY: u32 = 16384;
pub const FLG_DELREG_64BITKEY: u32 = 4096;
pub const FLG_DELREG_KEYONLY_COMMON: u32 = 8192;
pub const FLG_DELREG_MULTI_SZ_DELSTRING: u32 = 98306;
pub const FLG_DELREG_OPERATION_MASK: u32 = 254;
pub const FLG_DELREG_TYPE_BINARY: u32 = 1;
pub const FLG_DELREG_TYPE_DWORD: u32 = 65537;
pub const FLG_DELREG_TYPE_EXPAND_SZ: u32 = 131072;
pub const FLG_DELREG_TYPE_MASK: i32 = -65535;
pub const FLG_DELREG_TYPE_MULTI_SZ: u32 = 65536;
pub const FLG_DELREG_TYPE_NONE: u32 = 131073;
pub const FLG_DELREG_TYPE_QWORD: u32 = 720897;
pub const FLG_DELREG_TYPE_SZ: u32 = 0;
pub const FLG_DELREG_VALUE: u32 = 0;
pub const FLG_INI2REG_32BITKEY: u32 = 16384;
pub const FLG_INI2REG_64BITKEY: u32 = 4096;
pub const FLG_PROFITEM_CSIDL: u32 = 8;
pub const FLG_PROFITEM_CURRENTUSER: u32 = 1;
pub const FLG_PROFITEM_DELETE: u32 = 2;
pub const FLG_PROFITEM_GROUP: u32 = 4;
pub const FLG_REGSVR_DLLINSTALL: u32 = 2;
pub const FLG_REGSVR_DLLREGISTER: u32 = 1;
pub type HDEVINFO = *mut core::ffi::c_void;
pub type HDSKSPC = *mut core::ffi::c_void;
pub type HINF = *mut core::ffi::c_void;
pub type HSPFILELOG = *mut core::ffi::c_void;
pub type HSPFILEQ = *mut core::ffi::c_void;
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: u32 = 10010;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: u32 = 10004;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: u32 = 10003;
pub const IDD_DYNAWIZ_FIRSTPAGE: u32 = 10000;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: u32 = 10011;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: u32 = 10007;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: u32 = 10008;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: u32 = 10006;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: u32 = 10012;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: u32 = 10009;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: u32 = 10002;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: u32 = 10001;
pub const IDF_CHECKFIRST: u32 = 256;
pub const IDF_NOBEEP: u32 = 512;
pub const IDF_NOBROWSE: u32 = 1;
pub const IDF_NOCOMPRESSED: u32 = 8;
pub const IDF_NODETAILS: u32 = 4;
pub const IDF_NOFOREGROUND: u32 = 1024;
pub const IDF_NOREMOVABLEMEDIAPROMPT: u32 = 4096;
pub const IDF_NOSKIP: u32 = 2;
pub const IDF_OEMDISK: u32 = 2147483648;
pub const IDF_USEDISKNAMEASPROMPT: u32 = 8192;
pub const IDF_WARNIFSKIP: u32 = 2048;
pub const IDI_CLASSICON_OVERLAYFIRST: u32 = 500;
pub const IDI_CLASSICON_OVERLAYLAST: u32 = 502;
pub const IDI_CONFLICT: u32 = 161;
pub const IDI_DISABLED_OVL: u32 = 501;
pub const IDI_FORCED_OVL: u32 = 502;
pub const IDI_PROBLEM_OVL: u32 = 500;
pub const IDI_RESOURCE: u32 = 159;
pub const IDI_RESOURCEFIRST: u32 = 159;
pub const IDI_RESOURCELAST: u32 = 161;
pub const IDI_RESOURCEOVERLAYFIRST: u32 = 161;
pub const IDI_RESOURCEOVERLAYLAST: u32 = 161;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(target_arch = "x86")]
impl Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INFINFO_DEFAULT_SEARCH: u32 = 3;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: u32 = 2;
pub const INFINFO_INF_PATH_LIST_SEARCH: u32 = 5;
pub const INFINFO_INF_SPEC_IS_HINF: u32 = 1;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: u32 = 4;
pub const INF_STYLE_CACHE_DISABLE: u32 = 32;
pub const INF_STYLE_CACHE_ENABLE: u32 = 16;
pub const INF_STYLE_CACHE_IGNORE: u32 = 64;
pub const INF_STYLE_NONE: u32 = 0;
pub const INF_STYLE_OLDNT: u32 = 1;
pub const INF_STYLE_WIN4: u32 = 2;
pub const LINE_LEN: u32 = 256;
pub const LogSevError: u32 = 2;
pub const LogSevFatalError: u32 = 3;
pub const LogSevInformation: u32 = 0;
pub const LogSevMaximum: u32 = 4;
pub const LogSevWarning: u32 = 1;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: u32 = 11000;
pub const MAX_INF_SECTION_NAME_LENGTH: u32 = 255;
pub const MAX_INF_STRING_LENGTH: u32 = 4096;
pub const MAX_INSTALLWIZARD_DYNAPAGES: u32 = 20;
pub const MAX_INSTRUCTION_LEN: u32 = 256;
pub const MAX_LABEL_LEN: u32 = 30;
pub const MAX_SERVICE_NAME_LEN: u32 = 256;
pub const MAX_SUBTITLE_LEN: u32 = 256;
pub const MAX_TITLE_LEN: u32 = 60;
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: u32 = 10000;
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: u32 = 32768;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: u32 = 1;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: u32 = 1024;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: u32 = 2;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: u32 = 8192;
pub const NDW_INSTALLFLAG_KNOWNCLASS: u32 = 524288;
pub const NDW_INSTALLFLAG_NEEDREBOOT: u32 = 256;
pub const NDW_INSTALLFLAG_NEEDRESTART: u32 = 128;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: u32 = 512;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: u32 = 4096;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: u32 = 131072;
pub const NDW_INSTALLFLAG_PCMCIAMODE: u32 = 65536;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: u32 = 16384;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: u32 = 2048;
pub const NDW_INSTALLFLAG_USERCANCEL: u32 = 262144;
pub type PCABINET_INFO = PCABINET_INFO_A;
pub type PCABINET_INFO_A = *mut CABINET_INFO_A;
pub type PCABINET_INFO_W = *mut CABINET_INFO_W;
pub type PCOINSTALLER_CONTEXT_DATA = *mut COINSTALLER_CONTEXT_DATA;
pub type PDETECT_PROGRESS_NOTIFY = Option<unsafe extern "system" fn(progressnotifyparam: *const core::ffi::c_void, detectcomplete: u32) -> windows_sys::core::BOOL>;
pub type PFILEPATHS = PFILEPATHS_A;
pub type PFILEPATHS_A = *mut FILEPATHS_A;
pub type PFILEPATHS_SIGNERINFO = PFILEPATHS_SIGNERINFO_A;
pub type PFILEPATHS_SIGNERINFO_A = *mut FILEPATHS_SIGNERINFO_A;
pub type PFILEPATHS_SIGNERINFO_W = *mut FILEPATHS_SIGNERINFO_W;
pub type PFILEPATHS_W = *mut FILEPATHS_W;
pub type PFILE_IN_CABINET_INFO = PFILE_IN_CABINET_INFO_A;
pub type PFILE_IN_CABINET_INFO_A = *mut FILE_IN_CABINET_INFO_A;
pub type PFILE_IN_CABINET_INFO_W = *mut FILE_IN_CABINET_INFO_W;
pub type PINFCONTEXT = *mut INFCONTEXT;
pub type PSOURCE_MEDIA = PSOURCE_MEDIA_A;
pub type PSOURCE_MEDIA_A = *mut SOURCE_MEDIA_A;
pub type PSOURCE_MEDIA_W = *mut SOURCE_MEDIA_W;
#[cfg(all(feature = "prsht", feature = "windef"))]
pub type PSP_ADDPROPERTYPAGE_DATA = PSP_NEWDEVICEWIZARD_DATA;
pub type PSP_ALTPLATFORM_INFO = PSP_ALTPLATFORM_INFO_V2;
pub type PSP_ALTPLATFORM_INFO_V1 = *mut SP_ALTPLATFORM_INFO_V1;
pub type PSP_ALTPLATFORM_INFO_V2 = *mut SP_ALTPLATFORM_INFO_V2;
pub type PSP_ALTPLATFORM_INFO_V3 = *mut SP_ALTPLATFORM_INFO_V3;
pub type PSP_BACKUP_QUEUE_PARAMS = PSP_BACKUP_QUEUE_PARAMS_V2;
pub type PSP_BACKUP_QUEUE_PARAMS_A = PSP_BACKUP_QUEUE_PARAMS_V2_A;
pub type PSP_BACKUP_QUEUE_PARAMS_V1 = PSP_BACKUP_QUEUE_PARAMS_V1_A;
pub type PSP_BACKUP_QUEUE_PARAMS_V1_A = *mut SP_BACKUP_QUEUE_PARAMS_V1_A;
pub type PSP_BACKUP_QUEUE_PARAMS_V1_W = *mut SP_BACKUP_QUEUE_PARAMS_V1_W;
pub type PSP_BACKUP_QUEUE_PARAMS_V2 = PSP_BACKUP_QUEUE_PARAMS_V2_A;
pub type PSP_BACKUP_QUEUE_PARAMS_V2_A = *mut SP_BACKUP_QUEUE_PARAMS_V2_A;
pub type PSP_BACKUP_QUEUE_PARAMS_V2_W = *mut SP_BACKUP_QUEUE_PARAMS_V2_W;
pub type PSP_BACKUP_QUEUE_PARAMS_W = PSP_BACKUP_QUEUE_PARAMS_V2_W;
#[cfg(feature = "commctrl")]
pub type PSP_CLASSIMAGELIST_DATA = *mut SP_CLASSIMAGELIST_DATA;
pub type PSP_CLASSINSTALL_HEADER = *mut SP_CLASSINSTALL_HEADER;
pub type PSP_DETECTDEVICE_PARAMS = *mut SP_DETECTDEVICE_PARAMS;
pub type PSP_DETSIG_CMPPROC = Option<unsafe extern "system" fn(deviceinfoset: HDEVINFO, newdevicedata: *const SP_DEVINFO_DATA, existingdevicedata: *const SP_DEVINFO_DATA, comparecontext: *const core::ffi::c_void) -> u32>;
pub type PSP_DEVICE_INTERFACE_DATA = *mut SP_DEVICE_INTERFACE_DATA;
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA = PSP_DEVICE_INTERFACE_DETAIL_DATA_A;
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_A = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_W = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;
pub type PSP_DEVINFO_DATA = *mut SP_DEVINFO_DATA;
#[cfg(feature = "winnt")]
pub type PSP_DEVINFO_LIST_DETAIL_DATA = PSP_DEVINFO_LIST_DETAIL_DATA_A;
#[cfg(feature = "winnt")]
pub type PSP_DEVINFO_LIST_DETAIL_DATA_A = *mut SP_DEVINFO_LIST_DETAIL_DATA_A;
#[cfg(feature = "winnt")]
pub type PSP_DEVINFO_LIST_DETAIL_DATA_W = *mut SP_DEVINFO_LIST_DETAIL_DATA_W;
#[cfg(feature = "windef")]
pub type PSP_DEVINSTALL_PARAMS = PSP_DEVINSTALL_PARAMS_A;
#[cfg(feature = "windef")]
pub type PSP_DEVINSTALL_PARAMS_A = *mut SP_DEVINSTALL_PARAMS_A;
#[cfg(feature = "windef")]
pub type PSP_DEVINSTALL_PARAMS_W = *mut SP_DEVINSTALL_PARAMS_W;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA = PSP_DRVINFO_DATA_V2;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA_A = PSP_DRVINFO_DATA_V2_A;
pub type PSP_DRVINFO_DATA_V1 = PSP_DRVINFO_DATA_V1_A;
pub type PSP_DRVINFO_DATA_V1_A = *mut SP_DRVINFO_DATA_V1_A;
pub type PSP_DRVINFO_DATA_V1_W = *mut SP_DRVINFO_DATA_V1_W;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA_V2 = PSP_DRVINFO_DATA_V2_A;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA_V2_A = *mut SP_DRVINFO_DATA_V2_A;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA_V2_W = *mut SP_DRVINFO_DATA_V2_W;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSP_DRVINFO_DATA_W = PSP_DRVINFO_DATA_V2_W;
#[cfg(feature = "minwindef")]
pub type PSP_DRVINFO_DETAIL_DATA = PSP_DRVINFO_DETAIL_DATA_A;
#[cfg(feature = "minwindef")]
pub type PSP_DRVINFO_DETAIL_DATA_A = *mut SP_DRVINFO_DETAIL_DATA_A;
#[cfg(feature = "minwindef")]
pub type PSP_DRVINFO_DETAIL_DATA_W = *mut SP_DRVINFO_DETAIL_DATA_W;
pub type PSP_DRVINSTALL_PARAMS = *mut SP_DRVINSTALL_PARAMS;
pub type PSP_ENABLECLASS_PARAMS = *mut SP_ENABLECLASS_PARAMS;
pub type PSP_FILE_CALLBACK_A = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub type PSP_FILE_CALLBACK_W = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub type PSP_FILE_COPY_PARAMS = PSP_FILE_COPY_PARAMS_A;
pub type PSP_FILE_COPY_PARAMS_A = *mut SP_FILE_COPY_PARAMS_A;
pub type PSP_FILE_COPY_PARAMS_W = *mut SP_FILE_COPY_PARAMS_W;
pub type PSP_INF_INFORMATION = *mut SP_INF_INFORMATION;
pub type PSP_INF_SIGNER_INFO = PSP_INF_SIGNER_INFO_V2;
pub type PSP_INF_SIGNER_INFO_A = PSP_INF_SIGNER_INFO_V2_A;
pub type PSP_INF_SIGNER_INFO_V1 = PSP_INF_SIGNER_INFO_V1_A;
pub type PSP_INF_SIGNER_INFO_V1_A = *mut SP_INF_SIGNER_INFO_V1_A;
pub type PSP_INF_SIGNER_INFO_V1_W = *mut SP_INF_SIGNER_INFO_V1_W;
pub type PSP_INF_SIGNER_INFO_V2 = PSP_INF_SIGNER_INFO_V2_A;
pub type PSP_INF_SIGNER_INFO_V2_A = *mut SP_INF_SIGNER_INFO_V2_A;
pub type PSP_INF_SIGNER_INFO_V2_W = *mut SP_INF_SIGNER_INFO_V2_W;
pub type PSP_INF_SIGNER_INFO_W = PSP_INF_SIGNER_INFO_V2_W;
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
pub type PSP_INSTALLWIZARD_DATA = *mut SP_INSTALLWIZARD_DATA;
pub type PSP_INTERFACE_DEVICE_DATA = PSP_DEVICE_INTERFACE_DATA;
pub type PSP_INTERFACE_DEVICE_DETAIL_DATA = PSP_INTERFACE_DEVICE_DETAIL_DATA_A;
pub type PSP_INTERFACE_DEVICE_DETAIL_DATA_A = PSP_DEVICE_INTERFACE_DETAIL_DATA_A;
pub type PSP_INTERFACE_DEVICE_DETAIL_DATA_W = PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
#[cfg(all(feature = "prsht", feature = "windef"))]
pub type PSP_NEWDEVICEWIZARD_DATA = *mut SP_NEWDEVICEWIZARD_DATA;
pub type PSP_ORIGINAL_FILE_INFO = PSP_ORIGINAL_FILE_INFO_A;
pub type PSP_ORIGINAL_FILE_INFO_A = *mut SP_ORIGINAL_FILE_INFO_A;
pub type PSP_ORIGINAL_FILE_INFO_W = *mut SP_ORIGINAL_FILE_INFO_W;
pub type PSP_POWERMESSAGEWAKE_PARAMS = PSP_POWERMESSAGEWAKE_PARAMS_A;
pub type PSP_POWERMESSAGEWAKE_PARAMS_A = *mut SP_POWERMESSAGEWAKE_PARAMS_A;
pub type PSP_POWERMESSAGEWAKE_PARAMS_W = *mut SP_POWERMESSAGEWAKE_PARAMS_W;
pub type PSP_PROPCHANGE_PARAMS = *mut SP_PROPCHANGE_PARAMS;
pub type PSP_PROPSHEETPAGE_REQUEST = *mut SP_PROPSHEETPAGE_REQUEST;
pub type PSP_REGISTER_CONTROL_STATUS = PSP_REGISTER_CONTROL_STATUSA;
pub type PSP_REGISTER_CONTROL_STATUSA = *mut SP_REGISTER_CONTROL_STATUSA;
pub type PSP_REGISTER_CONTROL_STATUSW = *mut SP_REGISTER_CONTROL_STATUSW;
pub type PSP_REMOVEDEVICE_PARAMS = *mut SP_REMOVEDEVICE_PARAMS;
pub type PSP_SELECTDEVICE_PARAMS = PSP_SELECTDEVICE_PARAMS_A;
pub type PSP_SELECTDEVICE_PARAMS_A = *mut SP_SELECTDEVICE_PARAMS_A;
pub type PSP_SELECTDEVICE_PARAMS_W = *mut SP_SELECTDEVICE_PARAMS_W;
pub type PSP_TROUBLESHOOTER_PARAMS = PSP_TROUBLESHOOTER_PARAMS_A;
pub type PSP_TROUBLESHOOTER_PARAMS_A = *mut SP_TROUBLESHOOTER_PARAMS_A;
pub type PSP_TROUBLESHOOTER_PARAMS_W = *mut SP_TROUBLESHOOTER_PARAMS_W;
pub type PSP_UNREMOVEDEVICE_PARAMS = *mut SP_UNREMOVEDEVICE_PARAMS;
pub const SCWMI_CLOBBER_SECURITY: u32 = 1;
pub const SETDIRID_NOT_FULL_PATH: u32 = 1;
pub const SIGNERSCORE_AUTHENTICODE: u32 = 251658240;
pub const SIGNERSCORE_INBOX: u32 = 218103811;
pub const SIGNERSCORE_LOGO_PREMIUM: u32 = 218103809;
pub const SIGNERSCORE_LOGO_STANDARD: u32 = 218103810;
pub const SIGNERSCORE_MASK: u32 = 4278190080;
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840;
pub const SIGNERSCORE_UNCLASSIFIED: u32 = 218103812;
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080;
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648;
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472;
pub const SIGNERSCORE_WHQL: u32 = 218103813;
pub type SOURCE_MEDIA = SOURCE_MEDIA_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_sys::core::PCSTR,
    pub Tagfile: windows_sys::core::PCSTR,
    pub Description: windows_sys::core::PCSTR,
    pub SourcePath: windows_sys::core::PCSTR,
    pub SourceFile: windows_sys::core::PCSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SOURCE_MEDIA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_sys::core::PCSTR,
    pub Tagfile: windows_sys::core::PCSTR,
    pub Description: windows_sys::core::PCSTR,
    pub SourcePath: windows_sys::core::PCSTR,
    pub SourceFile: windows_sys::core::PCSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SOURCE_MEDIA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_sys::core::PCWSTR,
    pub Tagfile: windows_sys::core::PCWSTR,
    pub Description: windows_sys::core::PCWSTR,
    pub SourcePath: windows_sys::core::PCWSTR,
    pub SourceFile: windows_sys::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SOURCE_MEDIA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_sys::core::PCWSTR,
    pub Tagfile: windows_sys::core::PCWSTR,
    pub Description: windows_sys::core::PCWSTR,
    pub SourcePath: windows_sys::core::PCWSTR,
    pub SourceFile: windows_sys::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SOURCE_MEDIA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPCRP_CHARACTERISTICS: u32 = 27;
pub const SPCRP_DEVTYPE: u32 = 25;
pub const SPCRP_EXCLUSIVE: u32 = 26;
pub const SPCRP_LOWERFILTERS: u32 = 18;
pub const SPCRP_MAXIMUM_PROPERTY: u32 = 28;
pub const SPCRP_SECURITY: u32 = 23;
pub const SPCRP_SECURITY_SDS: u32 = 24;
pub const SPCRP_UPPERFILTERS: u32 = 17;
pub const SPDIT_CLASSDRIVER: u32 = 1;
pub const SPDIT_COMPATDRIVER: u32 = 2;
pub const SPDIT_NODRIVER: u32 = 0;
pub const SPDRP_ADDRESS: u32 = 28;
pub const SPDRP_BASE_CONTAINERID: u32 = 36;
pub const SPDRP_BUSNUMBER: u32 = 21;
pub const SPDRP_BUSTYPEGUID: u32 = 19;
pub const SPDRP_CAPABILITIES: u32 = 15;
pub const SPDRP_CHARACTERISTICS: u32 = 27;
pub const SPDRP_CLASS: u32 = 7;
pub const SPDRP_CLASSGUID: u32 = 8;
pub const SPDRP_COMPATIBLEIDS: u32 = 2;
pub const SPDRP_CONFIGFLAGS: u32 = 10;
pub const SPDRP_DEVICEDESC: u32 = 0;
pub const SPDRP_DEVICE_POWER_DATA: u32 = 30;
pub const SPDRP_DEVTYPE: u32 = 25;
pub const SPDRP_DRIVER: u32 = 9;
pub const SPDRP_ENUMERATOR_NAME: u32 = 22;
pub const SPDRP_EXCLUSIVE: u32 = 26;
pub const SPDRP_FRIENDLYNAME: u32 = 12;
pub const SPDRP_HARDWAREID: u32 = 1;
pub const SPDRP_INSTALL_STATE: u32 = 34;
pub const SPDRP_LEGACYBUSTYPE: u32 = 20;
pub const SPDRP_LOCATION_INFORMATION: u32 = 13;
pub const SPDRP_LOCATION_PATHS: u32 = 35;
pub const SPDRP_LOWERFILTERS: u32 = 18;
pub const SPDRP_MAXIMUM_PROPERTY: u32 = 37;
pub const SPDRP_MFG: u32 = 11;
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 14;
pub const SPDRP_REMOVAL_POLICY: u32 = 31;
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 32;
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: u32 = 33;
pub const SPDRP_SECURITY: u32 = 23;
pub const SPDRP_SECURITY_SDS: u32 = 24;
pub const SPDRP_SERVICE: u32 = 4;
pub const SPDRP_UI_NUMBER: u32 = 16;
pub const SPDRP_UI_NUMBER_DESC_FORMAT: u32 = 29;
pub const SPDRP_UNUSED0: u32 = 3;
pub const SPDRP_UNUSED1: u32 = 5;
pub const SPDRP_UNUSED2: u32 = 6;
pub const SPDRP_UPPERFILTERS: u32 = 17;
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: u32 = 2;
pub const SPDSL_IGNORE_DISK: u32 = 1;
pub const SPFILELOG_FORCENEW: u32 = 2;
pub const SPFILELOG_OEMFILE: u32 = 1;
pub const SPFILELOG_QUERYONLY: u32 = 4;
pub const SPFILELOG_SYSTEMLOG: u32 = 1;
pub const SPFILENOTIFY_BACKUPERROR: u32 = 22;
pub const SPFILENOTIFY_CABINETINFO: u32 = 16;
pub const SPFILENOTIFY_COPYERROR: u32 = 13;
pub const SPFILENOTIFY_DELETEERROR: u32 = 7;
pub const SPFILENOTIFY_ENDBACKUP: u32 = 23;
pub const SPFILENOTIFY_ENDCOPY: u32 = 12;
pub const SPFILENOTIFY_ENDDELETE: u32 = 6;
pub const SPFILENOTIFY_ENDQUEUE: u32 = 2;
pub const SPFILENOTIFY_ENDREGISTRATION: u32 = 32;
pub const SPFILENOTIFY_ENDRENAME: u32 = 9;
pub const SPFILENOTIFY_ENDSUBQUEUE: u32 = 4;
pub const SPFILENOTIFY_FILEEXTRACTED: u32 = 19;
pub const SPFILENOTIFY_FILEINCABINET: u32 = 17;
pub const SPFILENOTIFY_FILEOPDELAYED: u32 = 20;
pub const SPFILENOTIFY_LANGMISMATCH: u32 = 65536;
pub const SPFILENOTIFY_NEEDMEDIA: u32 = 14;
pub const SPFILENOTIFY_NEEDNEWCABINET: u32 = 18;
pub const SPFILENOTIFY_QUEUESCAN: u32 = 15;
pub const SPFILENOTIFY_QUEUESCAN_EX: u32 = 24;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: u32 = 64;
pub const SPFILENOTIFY_RENAMEERROR: u32 = 10;
pub const SPFILENOTIFY_STARTBACKUP: u32 = 21;
pub const SPFILENOTIFY_STARTCOPY: u32 = 11;
pub const SPFILENOTIFY_STARTDELETE: u32 = 5;
pub const SPFILENOTIFY_STARTQUEUE: u32 = 1;
pub const SPFILENOTIFY_STARTREGISTRATION: u32 = 25;
pub const SPFILENOTIFY_STARTRENAME: u32 = 8;
pub const SPFILENOTIFY_STARTSUBQUEUE: u32 = 3;
pub const SPFILENOTIFY_TARGETEXISTS: u32 = 131072;
pub const SPFILENOTIFY_TARGETNEWER: u32 = 262144;
pub const SPFILEQ_FILE_IN_USE: u32 = 1;
pub const SPFILEQ_REBOOT_IN_PROGRESS: u32 = 4;
pub const SPFILEQ_REBOOT_RECOMMENDED: u32 = 2;
pub const SPID_ACTIVE: u32 = 1;
pub const SPID_DEFAULT: u32 = 2;
pub const SPID_REMOVED: u32 = 4;
pub const SPINST_ALL: u32 = 2047;
pub const SPINST_BITREG: u32 = 32;
pub const SPINST_COPYINF: u32 = 512;
pub const SPINST_DEVICEINSTALL: u32 = 1048576;
pub const SPINST_FILES: u32 = 16;
pub const SPINST_INI2REG: u32 = 8;
pub const SPINST_INIFILES: u32 = 2;
pub const SPINST_LOGCONFIG: u32 = 1;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: u32 = 262144;
pub const SPINST_LOGCONFIG_IS_FORCED: u32 = 131072;
pub const SPINST_PROFILEITEMS: u32 = 256;
pub const SPINST_PROPERTIES: u32 = 1024;
pub const SPINST_REGISTERCALLBACKAWARE: u32 = 524288;
pub const SPINST_REGISTRY: u32 = 4;
pub const SPINST_REGSVR: u32 = 64;
pub const SPINST_SINGLESECTION: u32 = 65536;
pub const SPINST_UNREGSVR: u32 = 128;
pub const SPINT_ACTIVE: u32 = 1;
pub const SPINT_DEFAULT: u32 = 2;
pub const SPINT_REMOVED: u32 = 4;
pub const SPOST_MAX: u32 = 3;
pub const SPOST_NONE: u32 = 0;
pub const SPOST_PATH: u32 = 1;
pub const SPOST_URL: u32 = 2;
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: u32 = 3;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: u32 = 2;
pub const SPPSR_SELECT_DEVICE_RESOURCES: u32 = 1;
pub const SPQ_DELAYED_COPY: u32 = 1;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: u32 = 2;
pub const SPQ_FLAG_BACKUP_AWARE: u32 = 1;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: u32 = 8;
pub const SPQ_FLAG_FILES_MODIFIED: u32 = 4;
pub const SPQ_FLAG_VALID: u32 = 15;
pub const SPQ_SCAN_ACTIVATE_DRP: u32 = 1024;
pub const SPQ_SCAN_FILE_COMPARISON: u32 = 512;
pub const SPQ_SCAN_FILE_PRESENCE: u32 = 1;
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: u32 = 256;
pub const SPQ_SCAN_FILE_VALIDITY: u32 = 2;
pub const SPQ_SCAN_INFORM_USER: u32 = 16;
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: u32 = 32;
pub const SPQ_SCAN_PRUNE_DELREN: u32 = 128;
pub const SPQ_SCAN_USE_CALLBACK: u32 = 4;
pub const SPQ_SCAN_USE_CALLBACKEX: u32 = 8;
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: u32 = 64;
pub const SPQ_SCAN_USE_OEM_CATALOGS: u32 = 2048;
pub const SPRDI_FIND_DUPS: u32 = 1;
pub const SPREG_DLLINSTALL: u32 = 4;
pub const SPREG_GETPROCADDR: u32 = 2;
pub const SPREG_LOADLIBRARY: u32 = 1;
pub const SPREG_REGSVR: u32 = 3;
pub const SPREG_SUCCESS: u32 = 0;
pub const SPREG_TIMEOUT: u32 = 5;
pub const SPREG_UNKNOWN: u32 = 4294967295;
pub const SPSVCINST_ASSOCSERVICE: u32 = 2;
pub const SPSVCINST_CLOBBER_SECURITY: u32 = 1024;
pub const SPSVCINST_DELETEEVENTLOGENTRY: u32 = 4;
pub const SPSVCINST_NOCLOBBER_BOOTFLAGS: u32 = 262144;
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: u32 = 32768;
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: u32 = 128;
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: u32 = 256;
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: u32 = 8;
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: u32 = 32;
pub const SPSVCINST_NOCLOBBER_FAILUREACTIONS: u32 = 131072;
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: u32 = 64;
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: u32 = 4096;
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: u32 = 16384;
pub const SPSVCINST_NOCLOBBER_STARTTYPE: u32 = 16;
pub const SPSVCINST_NOCLOBBER_TRIGGERS: u32 = 8192;
pub const SPSVCINST_STARTSERVICE: u32 = 2048;
pub const SPSVCINST_STOPSERVICE: u32 = 512;
pub const SPSVCINST_TAGTOFRONT: u32 = 1;
pub const SPSVCINST_UNIQUE_NAME: u32 = 65536;
pub const SPWPT_SELECTDEVICE: u32 = 1;
pub const SPWP_USE_DEVINFO_DATA: u32 = 1;
#[cfg(all(feature = "prsht", feature = "windef"))]
pub type SP_ADDPROPERTYPAGE_DATA = SP_NEWDEVICEWIZARD_DATA;
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: u32 = 2;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: u32 = 1;
pub type SP_ALTPLATFORM_INFO = SP_ALTPLATFORM_INFO_V2;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_BACKUP_BACKUPPASS: u32 = 1;
pub const SP_BACKUP_BOOTFILE: u32 = 8;
pub const SP_BACKUP_DEMANDPASS: u32 = 2;
pub type SP_BACKUP_QUEUE_PARAMS = SP_BACKUP_QUEUE_PARAMS_V2;
pub type SP_BACKUP_QUEUE_PARAMS_A = SP_BACKUP_QUEUE_PARAMS_V2_A;
pub type SP_BACKUP_QUEUE_PARAMS_V1 = SP_BACKUP_QUEUE_PARAMS_V1_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_BACKUP_QUEUE_PARAMS_V2 = SP_BACKUP_QUEUE_PARAMS_V2_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_BACKUP_QUEUE_PARAMS_W = SP_BACKUP_QUEUE_PARAMS_V2_W;
pub const SP_BACKUP_SPECIAL: u32 = 4;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "commctrl")]
#[derive(Clone, Copy)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::commctrl::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "commctrl")]
impl Default for SP_CLASSIMAGELIST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "commctrl")]
#[derive(Clone, Copy)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::commctrl::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "commctrl")]
impl Default for SP_CLASSIMAGELIST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
pub const SP_COPY_ALREADYDECOMP: u32 = 4194304;
pub const SP_COPY_DELETESOURCE: u32 = 1;
pub const SP_COPY_FORCE_IN_USE: u32 = 512;
pub const SP_COPY_FORCE_NEWER: u32 = 8192;
pub const SP_COPY_FORCE_NOOVERWRITE: u32 = 4096;
pub const SP_COPY_HARDLINK: u32 = 268435456;
pub const SP_COPY_INBOX_INF: u32 = 134217728;
pub const SP_COPY_IN_USE_NEEDS_REBOOT: u32 = 256;
pub const SP_COPY_IN_USE_TRY_RENAME: u32 = 67108864;
pub const SP_COPY_LANGUAGEAWARE: u32 = 32;
pub const SP_COPY_NEWER: u32 = 4;
pub const SP_COPY_NEWER_ONLY: u32 = 65536;
pub const SP_COPY_NEWER_OR_SAME: u32 = 4;
pub const SP_COPY_NOBROWSE: u32 = 32768;
pub const SP_COPY_NODECOMP: u32 = 16;
pub const SP_COPY_NOOVERWRITE: u32 = 8;
pub const SP_COPY_NOPRUNE: u32 = 1048576;
pub const SP_COPY_NOSKIP: u32 = 1024;
pub const SP_COPY_OEMINF_CATALOG_ONLY: u32 = 262144;
pub const SP_COPY_OEM_F6_INF: u32 = 2097152;
pub const SP_COPY_PNPLOCKED: u32 = 33554432;
pub const SP_COPY_REPLACEONLY: u32 = 2;
pub const SP_COPY_REPLACE_BOOT_FILE: u32 = 524288;
pub const SP_COPY_RESERVED: u32 = 131072;
pub const SP_COPY_SOURCEPATH_ABSOLUTE: u32 = 128;
pub const SP_COPY_SOURCE_ABSOLUTE: u32 = 64;
pub const SP_COPY_WARNIFSKIP: u32 = 16384;
pub const SP_COPY_WINDOWS_SIGNED: u32 = 16777216;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
pub type SP_DEVICE_INTERFACE_DETAIL_DATA = SP_DEVICE_INTERFACE_DETAIL_DATA_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [i8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [i8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(feature = "winnt")]
pub type SP_DEVINFO_LIST_DETAIL_DATA = SP_DEVINFO_LIST_DETAIL_DATA_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub RemoteMachineHandle: super::winnt::HANDLE,
    pub RemoteMachineName: [i8; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub RemoteMachineHandle: super::winnt::HANDLE,
    pub RemoteMachineName: [i8; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub RemoteMachineHandle: super::winnt::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: windows_sys::core::GUID,
    pub RemoteMachineHandle: super::winnt::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type SP_DEVINSTALL_PARAMS = SP_DEVINSTALL_PARAMS_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::windef::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [i8; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::windef::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::windef::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::windef::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type SP_DRVINFO_DATA = SP_DRVINFO_DATA_V2;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type SP_DRVINFO_DATA_A = SP_DRVINFO_DATA_V2_A;
pub type SP_DRVINFO_DATA_V1 = SP_DRVINFO_DATA_V1_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type SP_DRVINFO_DATA_V2 = SP_DRVINFO_DATA_V2_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
    pub DriverDate: super::minwindef::FILETIME,
    pub DriverVersion: super::winnt::DWORDLONG,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
    pub DriverDate: super::minwindef::FILETIME,
    pub DriverVersion: super::winnt::DWORDLONG,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::minwindef::FILETIME,
    pub DriverVersion: super::winnt::DWORDLONG,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::minwindef::FILETIME,
    pub DriverVersion: super::winnt::DWORDLONG,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type SP_DRVINFO_DATA_W = SP_DRVINFO_DATA_V2_W;
#[cfg(feature = "minwindef")]
pub type SP_DRVINFO_DETAIL_DATA = SP_DRVINFO_DETAIL_DATA_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::minwindef::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [i8; 256],
    pub InfFileName: [i8; 260],
    pub DrvDescription: [i8; 256],
    pub HardwareID: [i8; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
impl Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::minwindef::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [i8; 256],
    pub InfFileName: [i8; 260],
    pub DrvDescription: [i8; 256],
    pub HardwareID: [i8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
impl Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::minwindef::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
impl Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::minwindef::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
impl Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: windows_sys::core::GUID,
    pub EnableMessage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: windows_sys::core::GUID,
    pub EnableMessage: u32,
}
pub type SP_FILE_COPY_PARAMS = SP_FILE_COPY_PARAMS_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_sys::core::PCSTR,
    pub SourcePath: windows_sys::core::PCSTR,
    pub SourceFilename: windows_sys::core::PCSTR,
    pub SourceDescription: windows_sys::core::PCSTR,
    pub SourceTagfile: windows_sys::core::PCSTR,
    pub TargetDirectory: windows_sys::core::PCSTR,
    pub TargetFilename: windows_sys::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_sys::core::PCSTR,
    pub SourcePath: windows_sys::core::PCSTR,
    pub SourceFilename: windows_sys::core::PCSTR,
    pub SourceDescription: windows_sys::core::PCSTR,
    pub SourceTagfile: windows_sys::core::PCSTR,
    pub TargetDirectory: windows_sys::core::PCSTR,
    pub TargetFilename: windows_sys::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_sys::core::PCWSTR,
    pub SourcePath: windows_sys::core::PCWSTR,
    pub SourceFilename: windows_sys::core::PCWSTR,
    pub SourceDescription: windows_sys::core::PCWSTR,
    pub SourceTagfile: windows_sys::core::PCWSTR,
    pub TargetDirectory: windows_sys::core::PCWSTR,
    pub TargetFilename: windows_sys::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_sys::core::PCWSTR,
    pub SourcePath: windows_sys::core::PCWSTR,
    pub SourceFilename: windows_sys::core::PCWSTR,
    pub SourceDescription: windows_sys::core::PCWSTR,
    pub SourceTagfile: windows_sys::core::PCWSTR,
    pub TargetDirectory: windows_sys::core::PCWSTR,
    pub TargetFilename: windows_sys::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_FLAG_CABINETCONTINUATION: u32 = 2048;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_INFORMATION {
    pub InfStyle: u32,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_INF_INFORMATION {
    pub InfStyle: u32,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_INF_SIGNER_INFO = SP_INF_SIGNER_INFO_V2;
pub type SP_INF_SIGNER_INFO_A = SP_INF_SIGNER_INFO_V2_A;
pub type SP_INF_SIGNER_INFO_V1 = SP_INF_SIGNER_INFO_V1_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_INF_SIGNER_INFO_V2 = SP_INF_SIGNER_INFO_V2_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_INF_SIGNER_INFO_W = SP_INF_SIGNER_INFO_V2_W;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::prsht::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::minwindef::LPARAM,
    pub hwndWizardDlg: super::windef::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
impl Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::prsht::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::minwindef::LPARAM,
    pub hwndWizardDlg: super::windef::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
impl Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_INTERFACE_DEVICE_DATA = SP_DEVICE_INTERFACE_DATA;
pub type SP_INTERFACE_DEVICE_DETAIL_DATA = SP_INTERFACE_DEVICE_DETAIL_DATA_A;
pub type SP_INTERFACE_DEVICE_DETAIL_DATA_A = SP_DEVICE_INTERFACE_DETAIL_DATA_A;
pub type SP_INTERFACE_DEVICE_DETAIL_DATA_W = SP_DEVICE_INTERFACE_DETAIL_DATA_W;
pub const SP_MAX_MACHINENAME_LENGTH: u32 = 263;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "prsht", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::prsht::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::windef::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "prsht", feature = "windef"))]
impl Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "prsht", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::prsht::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::windef::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "prsht", feature = "windef"))]
impl Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_ORIGINAL_FILE_INFO = SP_ORIGINAL_FILE_INFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [i8; 260],
    pub OriginalCatalogName: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [i8; 260],
    pub OriginalCatalogName: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_POWERMESSAGEWAKE_PARAMS = SP_POWERMESSAGEWAKE_PARAMS_A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [i8; 512],
}
impl Default for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(target_arch = "x86")]
impl Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: PSP_DEVINFO_DATA,
}
#[cfg(target_arch = "x86")]
impl Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: PSP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_REGISTER_CONTROL_STATUS = SP_REGISTER_CONTROL_STATUSA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_REGISTER_CONTROL_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_REGISTER_CONTROL_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_REGISTER_CONTROL_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_REGISTER_CONTROL_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
pub type SP_SELECTDEVICE_PARAMS = SP_SELECTDEVICE_PARAMS_A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [i8; 60],
    pub Instructions: [i8; 256],
    pub ListLabel: [i8; 30],
    pub SubTitle: [i8; 256],
    pub Reserved: [u8; 2],
}
impl Default for SP_SELECTDEVICE_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SP_TROUBLESHOOTER_PARAMS = SP_TROUBLESHOOTER_PARAMS_A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [i8; 260],
    pub HtmlTroubleShooter: [i8; 260],
}
impl Default for SP_TROUBLESHOOTER_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
pub const SRCINFO_DESCRIPTION: u32 = 3;
pub const SRCINFO_FLAGS: u32 = 4;
pub const SRCINFO_PATH: u32 = 1;
pub const SRCINFO_TAGFILE: u32 = 2;
pub const SRCINFO_TAGFILE2: u32 = 5;
pub const SRCLIST_APPEND: u32 = 512;
pub const SRCLIST_NOBROWSE: u32 = 2;
pub const SRCLIST_NOSTRIPPLATFORM: u32 = 1024;
pub const SRCLIST_SUBDIRS: u32 = 256;
pub const SRCLIST_SYSIFADMIN: u32 = 64;
pub const SRCLIST_SYSTEM: u32 = 16;
pub const SRCLIST_TEMPORARY: u32 = 1;
pub const SRCLIST_USER: u32 = 32;
pub const SRC_FLAGS_CABFILE: u32 = 16;
pub const SUOI_FORCEDELETE: u32 = 1;
pub const SUOI_INTERNAL1: u32 = 2;
pub const SetupFileLogChecksum: SetupFileLogInfo = 1;
pub const SetupFileLogDiskDescription: SetupFileLogInfo = 3;
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = 2;
pub type SetupFileLogInfo = i32;
pub const SetupFileLogMax: SetupFileLogInfo = 5;
pub const SetupFileLogOtherInfo: SetupFileLogInfo = 4;
pub const SetupFileLogSourceFilename: SetupFileLogInfo = 0;
