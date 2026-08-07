#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn InstallHinfSectionA<P2>(window: super::HWND, modulehandle: super::HINSTANCE, commandline: P2, showcommand: i32)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn InstallHinfSectionA(window : super::HWND, modulehandle : super::HINSTANCE, commandline : windows_core::PCSTR, showcommand : i32));
    unsafe { InstallHinfSectionA(window, modulehandle, commandline.param().abi(), showcommand) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn InstallHinfSectionW<P2>(window: super::HWND, modulehandle: super::HINSTANCE, commandline: P2, showcommand: i32)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn InstallHinfSectionW(window : super::HWND, modulehandle : super::HINSTANCE, commandline : windows_core::PCWSTR, showcommand : i32));
    unsafe { InstallHinfSectionW(window, modulehandle, commandline.param().abi(), showcommand) }
}
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListA<P3>(diskspace: HDSKSPC, infhandle: HINF, layoutinfhandle: Option<HINF>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddInstallSectionToDiskSpaceListA(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListW<P3>(diskspace: HDSKSPC, infhandle: HINF, layoutinfhandle: Option<HINF>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddInstallSectionToDiskSpaceListW(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListA<P3>(diskspace: HDSKSPC, infhandle: HINF, listinfhandle: Option<HINF>, sectionname: P3, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddSectionToDiskSpaceListA(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListW<P3>(diskspace: HDSKSPC, infhandle: HINF, listinfhandle: Option<HINF>, sectionname: P3, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddSectionToDiskSpaceListW(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddToDiskSpaceListA<P1>(diskspace: HDSKSPC, targetfilespec: P1, filesize: i64, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListA(diskspace : HDSKSPC, targetfilespec : windows_core::PCSTR, filesize : i64, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddToDiskSpaceListA(diskspace, targetfilespec.param().abi(), filesize, operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddToDiskSpaceListW<P1>(diskspace: HDSKSPC, targetfilespec: P1, filesize: i64, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListW(diskspace : HDSKSPC, targetfilespec : windows_core::PCWSTR, filesize : i64, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddToDiskSpaceListW(diskspace, targetfilespec.param().abi(), filesize, operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAddToSourceListA<P1>(flags: u32, source: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToSourceListA(flags : u32, source : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupAddToSourceListA(flags, source.param().abi()) }
}
#[inline]
pub unsafe fn SetupAddToSourceListW<P1>(flags: u32, source: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToSourceListW(flags : u32, source : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupAddToSourceListW(flags, source.param().abi()) }
}
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListA<P1>(diskspace: HDSKSPC, driveroot: P1, amount: i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListA(diskspace : HDSKSPC, driveroot : windows_core::PCSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAdjustDiskSpaceListA(diskspace, driveroot.param().abi(), amount, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListW<P1>(diskspace: HDSKSPC, driveroot: P1, amount: i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListW(diskspace : HDSKSPC, driveroot : windows_core::PCWSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAdjustDiskSpaceListW(diskspace, driveroot.param().abi(), amount, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupBackupErrorA<P1, P2, P3>(hwndparent: super::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupBackupErrorA(hwndparent : super::HWND, dialogtitle : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetfile : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupBackupErrorA(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupBackupErrorW<P1, P2, P3>(hwndparent: super::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupBackupErrorW(hwndparent : super::HWND, dialogtitle : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetfile : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupBackupErrorW(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupCancelTemporarySourceList() -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupCancelTemporarySourceList() -> windows_core::BOOL);
    unsafe { SetupCancelTemporarySourceList() }
}
#[inline]
pub unsafe fn SetupCloseFileQueue(queuehandle: HSPFILEQ) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseFileQueue(queuehandle : HSPFILEQ) -> windows_core::BOOL);
    unsafe { SetupCloseFileQueue(queuehandle) }
}
#[inline]
pub unsafe fn SetupCloseInfFile(infhandle: HINF) {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseInfFile(infhandle : HINF));
    unsafe { SetupCloseInfFile(infhandle) }
}
#[inline]
pub unsafe fn SetupCloseLog() {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseLog());
    unsafe { SetupCloseLog() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupCommitFileQueueA(owner: Option<super::HWND>, queuehandle: HSPFILEQ, msghandler: PSP_FILE_CALLBACK_A, context: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupCommitFileQueueA(owner : super::HWND, queuehandle : HSPFILEQ, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupCommitFileQueueA(owner.unwrap_or(core::mem::zeroed()) as _, queuehandle, msghandler, context) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupCommitFileQueueW(owner: Option<super::HWND>, queuehandle: HSPFILEQ, msghandler: PSP_FILE_CALLBACK_W, context: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupCommitFileQueueW(owner : super::HWND, queuehandle : HSPFILEQ, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupCommitFileQueueW(owner.unwrap_or(core::mem::zeroed()) as _, queuehandle, msghandler, context) }
}
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionA<P1>(infhandle: HINF, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionA(infhandle : HINF, sectionname : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupConfigureWmiFromInfSectionA(infhandle, sectionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionW<P1>(infhandle: HINF, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionW(infhandle : HINF, sectionname : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupConfigureWmiFromInfSectionW(infhandle, sectionname.param().abi(), flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupCopyErrorA<P1, P2, P3, P4, P5>(hwndparent: super::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, sourcefile: P4, targetpathfile: P5, win32errorcode: u32, style: u32, pathbuffer: Option<windows_core::PSTR>, pathbuffersize: u32, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyErrorA(hwndparent : super::HWND, dialogtitle : windows_core::PCSTR, diskname : windows_core::PCSTR, pathtosource : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetpathfile : windows_core::PCSTR, win32errorcode : u32, style : u32, pathbuffer : windows_core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupCopyErrorA(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), sourcefile.param().abi(), targetpathfile.param().abi(), win32errorcode, style, pathbuffer.unwrap_or(core::mem::zeroed()) as _, pathbuffersize, pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupCopyErrorW<P1, P2, P3, P4, P5>(hwndparent: super::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, sourcefile: P4, targetpathfile: P5, win32errorcode: u32, style: u32, pathbuffer: Option<windows_core::PWSTR>, pathbuffersize: u32, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyErrorW(hwndparent : super::HWND, dialogtitle : windows_core::PCWSTR, diskname : windows_core::PCWSTR, pathtosource : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetpathfile : windows_core::PCWSTR, win32errorcode : u32, style : u32, pathbuffer : windows_core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupCopyErrorW(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), sourcefile.param().abi(), targetpathfile.param().abi(), win32errorcode, style, pathbuffer.unwrap_or(core::mem::zeroed()) as _, pathbuffersize, pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupCopyOEMInfA<P0, P1>(sourceinffilename: P0, oemsourcemedialocation: P1, oemsourcemediatype: u32, copystyle: u32, destinationinffilename: Option<windows_core::PSTR>, destinationinffilenamesize: u32, requiredsize: Option<*mut u32>, destinationinffilenamecomponent: Option<*mut windows_core::PSTR>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyOEMInfA(sourceinffilename : windows_core::PCSTR, oemsourcemedialocation : windows_core::PCSTR, oemsourcemediatype : u32, copystyle : u32, destinationinffilename : windows_core::PSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SetupCopyOEMInfA(sourceinffilename.param().abi(), oemsourcemedialocation.param().abi(), oemsourcemediatype, copystyle, destinationinffilename.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamecomponent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupCopyOEMInfW<P0, P1>(sourceinffilename: P0, oemsourcemedialocation: P1, oemsourcemediatype: u32, copystyle: u32, destinationinffilename: Option<windows_core::PWSTR>, destinationinffilenamesize: u32, requiredsize: Option<*mut u32>, destinationinffilenamecomponent: Option<*mut windows_core::PWSTR>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyOEMInfW(sourceinffilename : windows_core::PCWSTR, oemsourcemedialocation : windows_core::PCWSTR, oemsourcemediatype : u32, copystyle : u32, destinationinffilename : windows_core::PWSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SetupCopyOEMInfW(sourceinffilename.param().abi(), oemsourcemedialocation.param().abi(), oemsourcemediatype, copystyle, destinationinffilename.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamecomponent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupCreateDiskSpaceListA(reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> HDSKSPC {
    windows_core::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListA(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
    unsafe { SetupCreateDiskSpaceListA(reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupCreateDiskSpaceListW(reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> HDSKSPC {
    windows_core::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListW(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
    unsafe { SetupCreateDiskSpaceListW(reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDecompressOrCopyFileA<P0, P1>(sourcefilename: P0, targetfilename: P1, compressiontype: Option<*const u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileA(sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, compressiontype : *const u32) -> u32);
    unsafe { SetupDecompressOrCopyFileA(sourcefilename.param().abi(), targetfilename.param().abi(), compressiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDecompressOrCopyFileW<P0, P1>(sourcefilename: P0, targetfilename: P1, compressiontype: Option<*const u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileW(sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, compressiontype : *const u32) -> u32);
    unsafe { SetupDecompressOrCopyFileW(sourcefilename.param().abi(), targetfilename.param().abi(), compressiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDefaultQueueCallbackA(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackA(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
    unsafe { SetupDefaultQueueCallbackA(context, notification, param1, param2) }
}
#[inline]
pub unsafe fn SetupDefaultQueueCallbackW(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackW(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
    unsafe { SetupDefaultQueueCallbackW(context, notification, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDeleteErrorA<P1, P2>(hwndparent: super::HWND, dialogtitle: P1, file: P2, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDeleteErrorA(hwndparent : super::HWND, dialogtitle : windows_core::PCSTR, file : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupDeleteErrorA(hwndparent, dialogtitle.param().abi(), file.param().abi(), win32errorcode, style) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDeleteErrorW<P1, P2>(hwndparent: super::HWND, dialogtitle: P1, file: P2, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDeleteErrorW(hwndparent : super::HWND, dialogtitle : windows_core::PCWSTR, file : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupDeleteErrorW(hwndparent, dialogtitle.param().abi(), file.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupDestroyDiskSpaceList(diskspace: HDSKSPC) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDestroyDiskSpaceList(diskspace : HDSKSPC) -> windows_core::BOOL);
    unsafe { SetupDestroyDiskSpaceList(diskspace as _) }
}
#[inline]
pub unsafe fn SetupDiAskForOEMDisk(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiAskForOEMDisk(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiAskForOEMDisk(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoList(flags: u32, classguidlist: Option<*mut windows_core::GUID>, classguidlistsize: u32, requiredsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoList(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoList(flags, classguidlist.unwrap_or(core::mem::zeroed()) as _, classguidlistsize, requiredsize as _) }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExA<P4>(flags: u32, classguidlist: Option<*mut windows_core::GUID>, classguidlistsize: u32, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExA(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoListExA(flags, classguidlist.unwrap_or(core::mem::zeroed()) as _, classguidlistsize, requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExW<P4>(flags: u32, classguidlist: Option<*mut windows_core::GUID>, classguidlistsize: u32, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExW(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoListExW(flags, classguidlist.unwrap_or(core::mem::zeroed()) as _, classguidlistsize, requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiBuildDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, drivertype: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, drivertype : u32) -> windows_core::BOOL);
    unsafe { SetupDiBuildDriverInfoList(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype) }
}
#[inline]
pub unsafe fn SetupDiCallClassInstaller(installfunction: DI_FUNCTION, deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCallClassInstaller(installfunction : DI_FUNCTION, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCallClassInstaller(installfunction, deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiCancelDriverInfoSearch(deviceinfoset: HDEVINFO) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCancelDriverInfoSearch(deviceinfoset : HDEVINFO) -> windows_core::BOOL);
    unsafe { SetupDiCancelDriverInfoSearch(deviceinfoset) }
}
#[inline]
pub unsafe fn SetupDiChangeState(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiChangeState(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiChangeState(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameA<P0>(classname: P0, classguidlist: *mut windows_core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameA(classname : windows_core::PCSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameA(classname.param().abi(), classguidlist as _, classguidlistsize, requiredsize as _) }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExA<P0, P4>(classname: P0, classguidlist: *mut windows_core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExA(classname : windows_core::PCSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameExA(classname.param().abi(), classguidlist as _, classguidlistsize, requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExW<P0, P4>(classname: P0, classguidlist: *mut windows_core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExW(classname : windows_core::PCWSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameExW(classname.param().abi(), classguidlist as _, classguidlistsize, requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameW<P0>(classname: P0, classguidlist: *mut windows_core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameW(classname : windows_core::PCWSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameW(classname.param().abi(), classguidlist as _, classguidlistsize, requiredsize as _) }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidA(classguid: *const windows_core::GUID, classname: windows_core::PSTR, classnamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidA(classguid : *const windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidA(classguid, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExA<P4>(classguid: *const windows_core::GUID, classname: windows_core::PSTR, classnamesize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExA(classguid : *const windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidExA(classguid, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExW<P4>(classguid: *const windows_core::GUID, classname: windows_core::PWSTR, classnamesize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExW(classguid : *const windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidExW(classguid, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidW(classguid: *const windows_core::GUID, classname: windows_core::PWSTR, classnamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidW(classguid : *const windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidW(classguid, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyA<P6>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: Option<HINF>, infsectionname: P6) -> super::HKEY
where
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : HINF, infsectionname : windows_core::PCSTR) -> super::HKEY);
    unsafe { SetupDiCreateDevRegKeyA(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyW<P6>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: Option<HINF>, infsectionname: P6) -> super::HKEY
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : HINF, infsectionname : windows_core::PCWSTR) -> super::HKEY);
    unsafe { SetupDiCreateDevRegKeyW(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoA<P1, P3>(deviceinfoset: HDEVINFO, devicename: P1, classguid: *const windows_core::GUID, devicedescription: P3, hwndparent: Option<super::HWND>, creationflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoA(deviceinfoset : HDEVINFO, devicename : windows_core::PCSTR, classguid : *const windows_core::GUID, devicedescription : windows_core::PCSTR, hwndparent : super::HWND, creationflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInfoA(deviceinfoset, devicename.param().abi(), classguid, devicedescription.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, creationflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoList(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::HWND>) -> HDEVINFO {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoList(classguid : *const windows_core::GUID, hwndparent : super::HWND) -> HDEVINFO);
    unsafe { SetupDiCreateDeviceInfoList(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExA<P2>(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::HWND>, machinename: P2, reserved: Option<*const core::ffi::c_void>) -> HDEVINFO
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExA(classguid : *const windows_core::GUID, hwndparent : super::HWND, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    unsafe { SetupDiCreateDeviceInfoListExA(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExW<P2>(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::HWND>, machinename: P2, reserved: Option<*const core::ffi::c_void>) -> HDEVINFO
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExW(classguid : *const windows_core::GUID, hwndparent : super::HWND, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    unsafe { SetupDiCreateDeviceInfoListExW(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoW<P1, P3>(deviceinfoset: HDEVINFO, devicename: P1, classguid: *const windows_core::GUID, devicedescription: P3, hwndparent: Option<super::HWND>, creationflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoW(deviceinfoset : HDEVINFO, devicename : windows_core::PCWSTR, classguid : *const windows_core::GUID, devicedescription : windows_core::PCWSTR, hwndparent : super::HWND, creationflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInfoW(deviceinfoset, devicename.param().abi(), classguid, devicedescription.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, creationflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceA<P3>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const windows_core::GUID, referencestring: P3, creationflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, referencestring : windows_core::PCSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInterfaceA(deviceinfoset, deviceinfodata, interfaceclassguid, referencestring.param().abi(), creationflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyA<P5>(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: super::REGSAM, infhandle: Option<HINF>, infsectionname: P5) -> super::HKEY
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::REGSAM, infhandle : HINF, infsectionname : windows_core::PCSTR) -> super::HKEY);
    unsafe { SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyW<P5>(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: super::REGSAM, infhandle: Option<HINF>, infsectionname: P5) -> super::HKEY
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::REGSAM, infhandle : HINF, infsectionname : windows_core::PCWSTR) -> super::HKEY);
    unsafe { SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) }
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceW<P3>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const windows_core::GUID, referencestring: P3, creationflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, referencestring : windows_core::PCWSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInterfaceW(deviceinfoset, deviceinfodata, interfaceclassguid, referencestring.param().abi(), creationflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiDeleteDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDevRegKey(deviceinfoset, deviceinfodata, scope, hwprofile, keytype) }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInfo(deviceinfoset, deviceinfodata) }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceData(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInterfaceData(deviceinfoset, deviceinterfacedata) }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyClassImageList(classimagelistdata : *const SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDestroyClassImageList(classimagelistdata) }
}
#[inline]
pub unsafe fn SetupDiDestroyDeviceInfoList(deviceinfoset: HDEVINFO) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyDeviceInfoList(deviceinfoset : HDEVINFO) -> windows_core::BOOL);
    unsafe { SetupDiDestroyDeviceInfoList(deviceinfoset) }
}
#[inline]
pub unsafe fn SetupDiDestroyDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32) -> windows_core::BOOL);
    unsafe { SetupDiDestroyDriverInfoList(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiDrawMiniIcon(hdc: super::HDC, rc: super::RECT, miniiconindex: i32, flags: u32) -> i32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDrawMiniIcon(hdc : super::HDC, rc : super::RECT, miniiconindex : i32, flags : u32) -> i32);
    unsafe { SetupDiDrawMiniIcon(hdc, rc, miniiconindex, flags) }
}
#[inline]
pub unsafe fn SetupDiEnumDeviceInfo(deviceinfoset: HDEVINFO, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInfo(deviceinfoset : HDEVINFO, memberindex : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiEnumDeviceInfo(deviceinfoset, memberindex, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiEnumDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, interfaceclassguid: *const windows_core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, memberindex : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiEnumDeviceInterfaces(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid, memberindex, deviceinterfacedata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiEnumDriverInfoA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: u32, memberindex: u32, driverinfodata: PSP_DRVINFO_DATA_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32, memberindex : u32, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_core::BOOL);
    unsafe { SetupDiEnumDriverInfoA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype, memberindex, driverinfodata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiEnumDriverInfoW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: u32, memberindex: u32, driverinfodata: PSP_DRVINFO_DATA_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : u32, memberindex : u32, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_core::BOOL);
    unsafe { SetupDiEnumDriverInfoW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype, memberindex, driverinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsectionwithext: Option<windows_core::PSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionA(context : *const INFCONTEXT, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualModelsSectionA(context, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsectionwithext: Option<windows_core::PWSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionW(context : *const INFCONTEXT, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualModelsSectionW(context, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallA<P1>(infhandle: HINF, infsectionname: P1, infsectionwithext: Option<windows_core::PSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PSTR>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallA(infhandle : HINF, infsectionname : windows_core::PCSTR, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallA(infhandle, infsectionname.param().abi(), infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExA<P1>(infhandle: HINF, infsectionname: P1, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsectionwithext: Option<windows_core::PSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PSTR>, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExA(infhandle : HINF, infsectionname : windows_core::PCSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallExA(infhandle, infsectionname.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExW<P1>(infhandle: HINF, infsectionname: P1, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsectionwithext: Option<windows_core::PWSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PWSTR>, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExW(infhandle : HINF, infsectionname : windows_core::PCWSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallExW(infhandle, infsectionname.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallW<P1>(infhandle: HINF, infsectionname: P1, infsectionwithext: Option<windows_core::PWSTR>, infsectionwithextsize: u32, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PWSTR>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallW(infhandle : HINF, infsectionname : windows_core::PCWSTR, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallW(infhandle, infsectionname.param().abi(), infsectionwithext.unwrap_or(core::mem::zeroed()) as _, infsectionwithextsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassBitmapIndex(classguid: Option<*const windows_core::GUID>, miniiconindex: *mut i32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassBitmapIndex(classguid : *const windows_core::GUID, miniiconindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassBitmapIndex(classguid.unwrap_or(core::mem::zeroed()) as _, miniiconindex as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionA(classguid: *const windows_core::GUID, classdescription: windows_core::PSTR, classdescriptionsize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionA(classguid : *const windows_core::GUID, classdescription : windows_core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionA(classguid, classdescription, classdescriptionsize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExA<P4>(classguid: *const windows_core::GUID, classdescription: windows_core::PSTR, classdescriptionsize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExA(classguid : *const windows_core::GUID, classdescription : windows_core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionExA(classguid, classdescription, classdescriptionsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExW<P4>(classguid: *const windows_core::GUID, classdescription: windows_core::PWSTR, classdescriptionsize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExW(classguid : *const windows_core::GUID, classdescription : windows_core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionExW(classguid, classdescription, classdescriptionsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionW(classguid: *const windows_core::GUID, classdescription: windows_core::PWSTR, classdescriptionsize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionW(classguid : *const windows_core::GUID, classdescription : windows_core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionW(classguid, classdescription, classdescriptionsize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt", feature = "winuser"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, propertysheetheader: super::LPPROPSHEETHEADERA, propertysheetheaderpagelistsize: u32, requiredsize: Option<*mut u32>, propertysheettype: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : super::LPPROPSHEETHEADERA, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDevPropertySheetsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, propertysheetheader, propertysheetheaderpagelistsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, propertysheettype) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt", feature = "winuser"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, propertysheetheader: super::LPPROPSHEETHEADERW, propertysheetheaderpagelistsize: u32, requiredsize: Option<*mut u32>, propertysheettype: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : super::LPPROPSHEETHEADERW, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDevPropertySheetsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, propertysheetheader, propertysheetheaderpagelistsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, propertysheettype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetClassDevsA<P1>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::HWND>, flags: u32) -> HDEVINFO
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsA(classguid : *const windows_core::GUID, enumerator : windows_core::PCSTR, hwndparent : super::HWND, flags : u32) -> HDEVINFO);
    unsafe { SetupDiGetClassDevsA(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetClassDevsExA<P1, P5>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::HWND>, flags: u32, deviceinfoset: Option<HDEVINFO>, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> HDEVINFO
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExA(classguid : *const windows_core::GUID, enumerator : windows_core::PCSTR, hwndparent : super::HWND, flags : u32, deviceinfoset : HDEVINFO, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    unsafe { SetupDiGetClassDevsExA(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetClassDevsExW<P1, P5>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::HWND>, flags: u32, deviceinfoset: Option<HDEVINFO>, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> HDEVINFO
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExW(classguid : *const windows_core::GUID, enumerator : windows_core::PCWSTR, hwndparent : super::HWND, flags : u32, deviceinfoset : HDEVINFO, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    unsafe { SetupDiGetClassDevsExW(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetClassDevsW<P1>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::HWND>, flags: u32) -> HDEVINFO
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsW(classguid : *const windows_core::GUID, enumerator : windows_core::PCWSTR, hwndparent : super::HWND, flags : u32) -> HDEVINFO);
    unsafe { SetupDiGetClassDevsW(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const windows_core::GUID, imageindex: *mut i32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageIndex(classimagelistdata : *const SP_CLASSIMAGELIST_DATA, classguid : *const windows_core::GUID, imageindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageIndex(classimagelistdata, classguid, imageindex as _) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageList(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageList(classimagelistdata as _) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn SetupDiGetClassImageListExA<P1>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: P1, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExA(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageListExA(classimagelistdata as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn SetupDiGetClassImageListExW<P1>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: P1, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExW(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageListExW(classimagelistdata as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*mut SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*mut SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyExW<P7>(classguid: *const windows_core::GUID, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>, flags: u32, machinename: P7, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyExW(classguid : *const windows_core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyExW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeys(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: u32, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeys(classguid : *const windows_core::GUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyKeys(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount, requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeysExW<P5>(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: u32, requiredpropertykeycount: Option<*mut u32>, flags: u32, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeysExW(classguid : *const windows_core::GUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyKeysExW(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount, requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyW(classguid: *const windows_core::GUID, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyW(classguid : *const windows_core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyA<P6>(classguid: *const windows_core::GUID, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: Option<*mut u32>, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyA(classguid : *const windows_core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassRegistryPropertyA(classguid, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyW<P6>(classguid: *const windows_core::GUID, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: Option<*mut u32>, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyW(classguid : *const windows_core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassRegistryPropertyW(classguid, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyA<P2>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: P2, flags: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_core::PCSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetCustomDevicePropertyA(deviceinfoset, deviceinfodata, custompropertyname.param().abi(), flags, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyW<P2>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: P2, flags: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_core::PCWSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetCustomDevicePropertyW(deviceinfoset, deviceinfodata, custompropertyname.param().abi(), flags, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListClass(deviceinfoset: HDEVINFO, classguid: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListClass(deviceinfoset : HDEVINFO, classguid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListClass(deviceinfoset, classguid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailA(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListDetailA(deviceinfoset, deviceinfosetdetaildata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailW(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListDetailW(deviceinfoset, deviceinfosetdetaildata as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: Option<windows_core::PSTR>, deviceinstanceidsize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_core::PSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstanceIdA(deviceinfoset, deviceinfodata, deviceinstanceid.unwrap_or(core::mem::zeroed()) as _, deviceinstanceidsize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: Option<windows_core::PWSTR>, deviceinstanceidsize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_core::PWSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstanceIdW(deviceinfoset, deviceinfodata, deviceinstanceid.unwrap_or(core::mem::zeroed()) as _, deviceinstanceidsize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const windows_core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceAlias(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid : *const windows_core::GUID, aliasdeviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceAlias(deviceinfoset, deviceinterfacedata, aliasinterfaceclassguid, aliasdeviceinterfacedata as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: Option<*mut SP_DEVICE_INTERFACE_DETAIL_DATA_A>, deviceinterfacedetaildatasize: u32, requiredsize: Option<*mut u32>, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceDetailA(deviceinfoset, deviceinterfacedata, deviceinterfacedetaildata.unwrap_or(core::mem::zeroed()) as _, deviceinterfacedetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: Option<*mut SP_DEVICE_INTERFACE_DETAIL_DATA_W>, deviceinterfacedetaildatasize: u32, requiredsize: Option<*mut u32>, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceDetailW(deviceinfoset, deviceinterfacedata, deviceinterfacedetaildata.unwrap_or(core::mem::zeroed()) as _, deviceinterfacedetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: u32, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset, deviceinterfacedata, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount, requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfacePropertyW(deviceinfoset, deviceinterfacedata, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetDevicePropertyKeys(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: u32, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyKeys(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDevicePropertyKeys(deviceinfoset, deviceinfodata, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount, requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiGetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDevicePropertyW(deviceinfoset, deviceinfodata, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceRegistryPropertyA(deviceinfoset, deviceinfodata, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: Option<*mut u8>, propertybuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceRegistryPropertyW(deviceinfoset, deviceinfodata, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_A, driverinfodetaildata: Option<*mut SP_DRVINFO_DETAIL_DATA_A>, driverinfodetaildatasize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInfoDetailA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinfodetaildata.unwrap_or(core::mem::zeroed()) as _, driverinfodetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_W, driverinfodetaildata: Option<*mut SP_DRVINFO_DETAIL_DATA_W>, driverinfodetaildatasize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInfoDetailW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinfodetaildata.unwrap_or(core::mem::zeroed()) as _, driverinfodetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: windows_core::PSTR, friendlynamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameA(hwprofile : u32, friendlyname : windows_core::PSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameA(hwprofile, friendlyname, friendlynamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExA<P4>(hwprofile: u32, friendlyname: windows_core::PSTR, friendlynamesize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExA(hwprofile : u32, friendlyname : windows_core::PSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameExA(hwprofile, friendlyname, friendlynamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExW<P4>(hwprofile: u32, friendlyname: windows_core::PWSTR, friendlynamesize: u32, requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExW(hwprofile : u32, friendlyname : windows_core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameExW(hwprofile, friendlyname, friendlynamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: windows_core::PWSTR, friendlynamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameW(hwprofile : u32, friendlyname : windows_core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameW(hwprofile, friendlyname, friendlynamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileList(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileList(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileList(hwprofilelist as _, hwprofilelistsize, requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileListExA<P4>(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExA(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileListExA(hwprofilelist as _, hwprofilelistsize, requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileListExW<P4>(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExW(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileListExW(hwprofilelist as _, hwprofilelistsize, requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetINFClassA<P0>(infname: P0, classguid: *mut windows_core::GUID, classname: windows_core::PSTR, classnamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetINFClassA(infname : windows_core::PCSTR, classguid : *mut windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetINFClassA(infname.param().abi(), classguid as _, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetINFClassW<P0>(infname: P0, classguid: *mut windows_core::GUID, classname: windows_core::PWSTR, classnamesize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetINFClassW(infname : windows_core::PCWSTR, classguid : *mut windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetINFClassW(infname.param().abi(), classguid as _, classname, classnamesize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiGetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDevice(deviceinfoset, deviceinfodata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDriverA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiGetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDriverW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata as _) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef"))]
#[inline]
pub unsafe fn SetupDiGetWizardPage(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::HPROPSHEETPAGE {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetWizardPage(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, installwizarddata : *const SP_INSTALLWIZARD_DATA, pagetype : u32, flags : u32) -> super::HPROPSHEETPAGE);
    unsafe { SetupDiGetWizardPage(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, installwizarddata, pagetype, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiInstallClassA<P1>(hwndparent: Option<super::HWND>, inffilename: P1, flags: u32, filequeue: Option<HSPFILEQ>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassA(hwndparent : super::HWND, inffilename : windows_core::PCSTR, flags : u32, filequeue : HSPFILEQ) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassA(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiInstallClassExA<P1>(hwndparent: Option<super::HWND>, inffilename: P1, flags: u32, filequeue: Option<HSPFILEQ>, interfaceclassguid: Option<*const windows_core::GUID>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassExA(hwndparent : super::HWND, inffilename : windows_core::PCSTR, flags : u32, filequeue : HSPFILEQ, interfaceclassguid : *const windows_core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassExA(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiInstallClassExW<P1>(hwndparent: Option<super::HWND>, inffilename: P1, flags: u32, filequeue: Option<HSPFILEQ>, interfaceclassguid: Option<*const windows_core::GUID>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassExW(hwndparent : super::HWND, inffilename : windows_core::PCWSTR, flags : u32, filequeue : HSPFILEQ, interfaceclassguid : *const windows_core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassExW(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiInstallClassW<P1>(hwndparent: Option<super::HWND>, inffilename: P1, flags: u32, filequeue: Option<HSPFILEQ>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassW(hwndparent : super::HWND, inffilename : windows_core::PCWSTR, flags : u32, filequeue : HSPFILEQ) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassW(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiInstallDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDevice(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiInstallDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDeviceInterfaces(deviceinfoset, deviceinfodata) }
}
#[inline]
pub unsafe fn SetupDiInstallDriverFiles(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDriverFiles(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDriverFiles(deviceinfoset, deviceinfodata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiLoadClassIcon(classguid: *const windows_core::GUID, largeicon: Option<*mut super::HICON>, miniiconindex: Option<*mut i32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiLoadClassIcon(classguid : *const windows_core::GUID, largeicon : *mut super::HICON, miniiconindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiLoadClassIcon(classguid, largeicon.unwrap_or(core::mem::zeroed()) as _, miniiconindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiLoadDeviceIcon(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::HICON) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiLoadDeviceIcon(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, cxicon : u32, cyicon : u32, flags : u32, hicon : *mut super::HICON) -> windows_core::BOOL);
    unsafe { SetupDiLoadDeviceIcon(deviceinfoset, deviceinfodata, cxicon, cyicon, flags, hicon as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiOpenClassRegKey(classguid: Option<*const windows_core::GUID>, samdesired: super::REGSAM) -> super::HKEY {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKey(classguid : *const windows_core::GUID, samdesired : super::REGSAM) -> super::HKEY);
    unsafe { SetupDiOpenClassRegKey(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExA<P3>(classguid: Option<*const windows_core::GUID>, samdesired: super::REGSAM, flags: u32, machinename: P3, reserved: Option<*const core::ffi::c_void>) -> super::HKEY
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExA(classguid : *const windows_core::GUID, samdesired : super::REGSAM, flags : u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> super::HKEY);
    unsafe { SetupDiOpenClassRegKeyExA(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExW<P3>(classguid: Option<*const windows_core::GUID>, samdesired: super::REGSAM, flags: u32, machinename: P3, reserved: Option<*const core::ffi::c_void>) -> super::HKEY
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExW(classguid : *const windows_core::GUID, samdesired : super::REGSAM, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> super::HKEY);
    unsafe { SetupDiOpenClassRegKeyExW(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiOpenDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: super::REGSAM) -> super::HKEY {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, samdesired : super::REGSAM) -> super::HKEY);
    unsafe { SetupDiOpenDevRegKey(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, samdesired) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoA<P1>(deviceinfoset: HDEVINFO, deviceinstanceid: P1, hwndparent: Option<super::HWND>, openflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoA(deviceinfoset : HDEVINFO, deviceinstanceid : windows_core::PCSTR, hwndparent : super::HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInfoA(deviceinfoset, deviceinstanceid.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, openflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoW<P1>(deviceinfoset: HDEVINFO, deviceinstanceid: P1, hwndparent: Option<super::HWND>, openflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoW(deviceinfoset : HDEVINFO, deviceinstanceid : windows_core::PCWSTR, hwndparent : super::HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInfoW(deviceinfoset, deviceinstanceid.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, openflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceA<P1>(deviceinfoset: HDEVINFO, devicepath: P1, openflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceA(deviceinfoset : HDEVINFO, devicepath : windows_core::PCSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInterfaceA(deviceinfoset, devicepath.param().abi(), openflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: super::REGSAM) -> super::HKEY {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : super::REGSAM) -> super::HKEY);
    unsafe { SetupDiOpenDeviceInterfaceRegKey(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired) }
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceW<P1>(deviceinfoset: HDEVINFO, devicepath: P1, openflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceW(deviceinfoset : HDEVINFO, devicepath : windows_core::PCWSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInterfaceW(deviceinfoset, devicepath.param().abi(), openflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRegisterCoDeviceInstallers(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRegisterCoDeviceInstallers(deviceinfoset, deviceinfodata) }
}
#[inline]
pub unsafe fn SetupDiRegisterDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: PSP_DETSIG_CMPPROC, comparecontext: Option<*const core::ffi::c_void>, dupdeviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRegisterDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, flags : u32, compareproc : PSP_DETSIG_CMPPROC, comparecontext : *const core::ffi::c_void, dupdeviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRegisterDeviceInfo(deviceinfoset, deviceinfodata as _, flags, compareproc, comparecontext.unwrap_or(core::mem::zeroed()) as _, dupdeviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiRemoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRemoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRemoveDevice(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiRemoveDeviceInterface(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRemoveDeviceInterface(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRemoveDeviceInterface(deviceinfoset, deviceinterfacedata as _) }
}
#[inline]
pub unsafe fn SetupDiRestartDevices(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRestartDevices(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRestartDevices(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiSelectBestCompatDrv(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectBestCompatDrv(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectBestCompatDrv(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiSelectDevice(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectDevice(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiSelectOEMDrv(hwndparent: Option<super::HWND>, deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectOEMDrv(hwndparent : super::HWND, deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectOEMDrv(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*const SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize) }
}
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*const SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiSetClassPropertyExW<P6>(classguid: *const windows_core::GUID, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyExW(classguid : *const windows_core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassPropertyExW(classguid, propertykey, propertytype, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiSetClassPropertyW(classguid: *const windows_core::GUID, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyW(classguid : *const windows_core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassPropertyW(classguid, propertykey, propertytype, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyA<P4>(classguid: *const windows_core::GUID, property: u32, propertybuffer: Option<&[u8]>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyA(classguid : *const windows_core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassRegistryPropertyA(classguid, property, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyW<P4>(classguid: *const windows_core::GUID, property: u32, propertybuffer: Option<&[u8]>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyW(classguid : *const windows_core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassRegistryPropertyW(classguid, property, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams) }
}
#[inline]
pub unsafe fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfaceDefault(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInterfaceDefault(deviceinfoset, deviceinterfacedata as _, flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInterfacePropertyW(deviceinfoset, deviceinterfacedata, propertykey, propertytype, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn SetupDiSetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDevicePropertyW(deviceinfoset, deviceinfodata, propertykey, propertytype, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: Option<&[u8]>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : u32, propertybuffer : *const u8, propertybuffersize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceRegistryPropertyA(deviceinfoset, deviceinfodata as _, property, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: Option<&[u8]>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : u32, propertybuffer : *const u8, propertybuffersize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceRegistryPropertyW(deviceinfoset, deviceinfodata as _, property, propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr()), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiSetDriverInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: PSP_DRVINFO_DATA_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiSetDriverInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams) }
}
#[inline]
pub unsafe fn SetupDiSetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDevice(deviceinfoset, deviceinfodata) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiSetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, driverinfodata: Option<PSP_DRVINFO_DATA_A>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_A) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDriverA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupDiSetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, driverinfodata: Option<PSP_DRVINFO_DATA_W>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : PSP_DRVINFO_DATA_W) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDriverW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDiUnremoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiUnremoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiUnremoveDevice(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListA(diskspace: HDSKSPC, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> HDSKSPC {
    windows_core::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListA(diskspace : HDSKSPC, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
    unsafe { SetupDuplicateDiskSpaceListA(diskspace, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListW(diskspace: HDSKSPC, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> HDSKSPC {
    windows_core::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListW(diskspace : HDSKSPC, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> HDSKSPC);
    unsafe { SetupDuplicateDiskSpaceListW(diskspace, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupEnumInfSectionsA(infhandle: HINF, index: u32, buffer: Option<windows_core::PSTR>, size: u32, sizeneeded: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupEnumInfSectionsA(infhandle : HINF, index : u32, buffer : windows_core::PSTR, size : u32, sizeneeded : *mut u32) -> windows_core::BOOL);
    unsafe { SetupEnumInfSectionsA(infhandle, index, buffer.unwrap_or(core::mem::zeroed()) as _, size, sizeneeded.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupEnumInfSectionsW(infhandle: HINF, index: u32, buffer: Option<windows_core::PWSTR>, size: u32, sizeneeded: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupEnumInfSectionsW(infhandle : HINF, index : u32, buffer : windows_core::PWSTR, size : u32, sizeneeded : *mut u32) -> windows_core::BOOL);
    unsafe { SetupEnumInfSectionsW(infhandle, index, buffer.unwrap_or(core::mem::zeroed()) as _, size, sizeneeded.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupFindFirstLineA<P1, P2>(infhandle: HINF, section: P1, key: P2, context: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindFirstLineA(infhandle : HINF, section : windows_core::PCSTR, key : windows_core::PCSTR, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindFirstLineA(infhandle, section.param().abi(), key.param().abi(), context as _) }
}
#[inline]
pub unsafe fn SetupFindFirstLineW<P1, P2>(infhandle: HINF, section: P1, key: P2, context: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindFirstLineW(infhandle : HINF, section : windows_core::PCWSTR, key : windows_core::PCWSTR, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindFirstLineW(infhandle, section.param().abi(), key.param().abi(), context as _) }
}
#[inline]
pub unsafe fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextLine(contextin : *const INFCONTEXT, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextLine(contextin, contextout as _) }
}
#[inline]
pub unsafe fn SetupFindNextMatchLineA<P1>(contextin: *const INFCONTEXT, key: P1, contextout: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextMatchLineA(contextin : *const INFCONTEXT, key : windows_core::PCSTR, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextMatchLineA(contextin, key.param().abi(), contextout as _) }
}
#[inline]
pub unsafe fn SetupFindNextMatchLineW<P1>(contextin: *const INFCONTEXT, key: P1, contextout: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextMatchLineW(contextin : *const INFCONTEXT, key : windows_core::PCWSTR, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextMatchLineW(contextin, key.param().abi(), contextout as _) }
}
#[inline]
pub unsafe fn SetupFreeSourceListA(list: *mut *mut windows_core::PCSTR, count: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupFreeSourceListA(list : *mut *mut windows_core::PCSTR, count : u32) -> windows_core::BOOL);
    unsafe { SetupFreeSourceListA(list as _, count) }
}
#[inline]
pub unsafe fn SetupFreeSourceListW(list: *mut *mut windows_core::PCWSTR, count: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupFreeSourceListW(list : *mut *mut windows_core::PCWSTR, count : u32) -> windows_core::BOOL);
    unsafe { SetupFreeSourceListW(list as _, count) }
}
#[inline]
pub unsafe fn SetupGetBackupInformationA(queuehandle: HSPFILEQ, backupparams: PSP_BACKUP_QUEUE_PARAMS_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBackupInformationA(queuehandle : HSPFILEQ, backupparams : PSP_BACKUP_QUEUE_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupGetBackupInformationA(queuehandle, backupparams as _) }
}
#[inline]
pub unsafe fn SetupGetBackupInformationW(queuehandle: HSPFILEQ, backupparams: PSP_BACKUP_QUEUE_PARAMS_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBackupInformationW(queuehandle : HSPFILEQ, backupparams : PSP_BACKUP_QUEUE_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupGetBackupInformationW(queuehandle, backupparams as _) }
}
#[inline]
pub unsafe fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<*mut u8>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBinaryField(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : *mut u8, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetBinaryField(context, fieldindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFieldCount(context : *const INFCONTEXT) -> u32);
    unsafe { SetupGetFieldCount(context) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoA<P0>(sourcefilename: P0, actualsourcefilename: *mut windows_core::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoA(sourcefilename : windows_core::PCSTR, actualsourcefilename : *mut windows_core::PSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> u32);
    unsafe { SetupGetFileCompressionInfoA(sourcefilename.param().abi(), actualsourcefilename as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExA<P0>(sourcefilename: P0, actualsourcefilenamebuffer: Option<&[u8]>, requiredbufferlen: Option<*mut u32>, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExA(sourcefilename : windows_core::PCSTR, actualsourcefilenamebuffer : windows_core::PCSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileCompressionInfoExA(sourcefilename.param().abi(), core::mem::transmute(actualsourcefilenamebuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), actualsourcefilenamebuffer.map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlen.unwrap_or(core::mem::zeroed()) as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExW<P0>(sourcefilename: P0, actualsourcefilenamebuffer: Option<&[u16]>, requiredbufferlen: Option<*mut u32>, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExW(sourcefilename : windows_core::PCWSTR, actualsourcefilenamebuffer : windows_core::PCWSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileCompressionInfoExW(sourcefilename.param().abi(), core::mem::transmute(actualsourcefilenamebuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), actualsourcefilenamebuffer.map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlen.unwrap_or(core::mem::zeroed()) as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoW<P0>(sourcefilename: P0, actualsourcefilename: *mut windows_core::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoW(sourcefilename : windows_core::PCWSTR, actualsourcefilename : *mut windows_core::PWSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut u32) -> u32);
    unsafe { SetupGetFileCompressionInfoW(sourcefilename.param().abi(), actualsourcefilename as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileQueueCount(filequeue: HSPFILEQ, subqueuefileop: u32, numoperations: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileQueueCount(filequeue : HSPFILEQ, subqueuefileop : u32, numoperations : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileQueueCount(filequeue, subqueuefileop, numoperations as _) }
}
#[inline]
pub unsafe fn SetupGetFileQueueFlags(filequeue: HSPFILEQ, flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileQueueFlags(filequeue : HSPFILEQ, flags : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileQueueFlags(filequeue, flags as _) }
}
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationA<P0, P2>(filename: P0, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, localename: P2, returnbuffer: windows_core::PSTR, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationA(filename : windows_core::PCSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, localename : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfDriverStoreLocationA(filename.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, localename.param().abi(), returnbuffer, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationW<P0, P2>(filename: P0, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, localename: P2, returnbuffer: windows_core::PWSTR, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationW(filename : windows_core::PCWSTR, alternateplatforminfo : PSP_ALTPLATFORM_INFO, localename : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfDriverStoreLocationW(filename.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, localename.param().abi(), returnbuffer, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfFileListA<P0>(directorypath: P0, infstyle: u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfFileListA(directorypath : windows_core::PCSTR, infstyle : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfFileListA(directorypath.param().abi(), infstyle, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfFileListW<P0>(directorypath: P0, infstyle: u32, returnbuffer: windows_core::PWSTR, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfFileListW(directorypath : windows_core::PCWSTR, infstyle : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfFileListW(directorypath.param().abi(), infstyle, returnbuffer, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfInformationA(infspec: *const core::ffi::c_void, searchcontrol: u32, returnbuffer: Option<*mut SP_INF_INFORMATION>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfInformationA(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfInformationA(infspec, searchcontrol, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfInformationW(infspec: *const core::ffi::c_void, searchcontrol: u32, returnbuffer: Option<*mut SP_INF_INFORMATION>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfInformationW(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfInformationW(infspec, searchcontrol, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfPublishedNameA<P0>(driverstorelocation: P0, returnbuffer: windows_core::PSTR, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameA(driverstorelocation : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfPublishedNameA(driverstorelocation.param().abi(), returnbuffer, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetInfPublishedNameW<P0>(driverstorelocation: P0, returnbuffer: windows_core::PWSTR, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameW(driverstorelocation : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfPublishedNameW(driverstorelocation.param().abi(), returnbuffer, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetIntField(context : *const INFCONTEXT, fieldindex : u32, integervalue : *mut i32) -> windows_core::BOOL);
    unsafe { SetupGetIntField(context, fieldindex, integervalue as _) }
}
#[inline]
pub unsafe fn SetupGetLineByIndexA<P1>(infhandle: HINF, section: P1, index: u32, context: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineByIndexA(infhandle : HINF, section : windows_core::PCSTR, index : u32, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupGetLineByIndexA(infhandle, section.param().abi(), index, context as _) }
}
#[inline]
pub unsafe fn SetupGetLineByIndexW<P1>(infhandle: HINF, section: P1, index: u32, context: *mut INFCONTEXT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineByIndexW(infhandle : HINF, section : windows_core::PCWSTR, index : u32, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupGetLineByIndexW(infhandle, section.param().abi(), index, context as _) }
}
#[inline]
pub unsafe fn SetupGetLineCountA<P1>(infhandle: HINF, section: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineCountA(infhandle : HINF, section : windows_core::PCSTR) -> i32);
    unsafe { SetupGetLineCountA(infhandle, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupGetLineCountW<P1>(infhandle: HINF, section: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineCountW(infhandle : HINF, section : windows_core::PCWSTR) -> i32);
    unsafe { SetupGetLineCountW(infhandle, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupGetLineTextA<P2, P3>(context: Option<*const INFCONTEXT>, infhandle: Option<HINF>, section: P2, key: P3, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineTextA(context : *const INFCONTEXT, infhandle : HINF, section : windows_core::PCSTR, key : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetLineTextA(context.unwrap_or(core::mem::zeroed()) as _, infhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), key.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetLineTextW<P2, P3>(context: Option<*const INFCONTEXT>, infhandle: Option<HINF>, section: P2, key: P3, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineTextW(context : *const INFCONTEXT, infhandle : HINF, section : windows_core::PCWSTR, key : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetLineTextW(context.unwrap_or(core::mem::zeroed()) as _, infhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), key.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetMultiSzFieldA(context, fieldindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetMultiSzFieldW(context, fieldindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetNonInteractiveMode() -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetNonInteractiveMode() -> windows_core::BOOL);
    unsafe { SetupGetNonInteractiveMode() }
}
#[inline]
pub unsafe fn SetupGetSourceFileLocationA<P2>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, filename: P2, sourceid: *mut u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationA(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_core::PCSTR, sourceid : *mut u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileLocationA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), sourceid as _, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetSourceFileLocationW<P2>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, filename: P2, sourceid: *mut u32, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationW(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_core::PCWSTR, sourceid : *mut u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileLocationW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), sourceid as _, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetSourceFileSizeA<P2, P3>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, filename: P2, section: P3, filesize: *mut u32, roundingfactor: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeA(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_core::PCSTR, section : windows_core::PCSTR, filesize : *mut u32, roundingfactor : u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileSizeA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), section.param().abi(), filesize as _, roundingfactor) }
}
#[inline]
pub unsafe fn SetupGetSourceFileSizeW<P2, P3>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, filename: P2, section: P3, filesize: *mut u32, roundingfactor: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeW(infhandle : HINF, infcontext : *const INFCONTEXT, filename : windows_core::PCWSTR, section : windows_core::PCWSTR, filesize : *mut u32, roundingfactor : u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileSizeW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), section.param().abi(), filesize as _, roundingfactor) }
}
#[inline]
pub unsafe fn SetupGetSourceInfoA(infhandle: HINF, sourceid: u32, infodesired: u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceInfoA(infhandle : HINF, sourceid : u32, infodesired : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceInfoA(infhandle, sourceid, infodesired, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetSourceInfoW(infhandle: HINF, sourceid: u32, infodesired: u32, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceInfoW(infhandle : HINF, sourceid : u32, infodesired : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceInfoW(infhandle, sourceid, infodesired, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetStringFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetStringFieldA(context, fieldindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetStringFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetStringFieldW(context, fieldindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetTargetPathA<P2>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, section: P2, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetTargetPathA(infhandle : HINF, infcontext : *const INFCONTEXT, section : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetTargetPathA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupGetTargetPathW<P2>(infhandle: HINF, infcontext: Option<*const INFCONTEXT>, section: P2, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetTargetPathW(infhandle : HINF, infcontext : *const INFCONTEXT, section : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetTargetPathW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "spapidef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupGetThreadLogToken() -> super::SP_LOG_TOKEN {
    windows_core::link!("setupapi.dll" "system" fn SetupGetThreadLogToken() -> super::SP_LOG_TOKEN);
    unsafe { SetupGetThreadLogToken() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupInitDefaultQueueCallback(ownerwindow: Option<super::HWND>) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallback(ownerwindow : super::HWND) -> *mut core::ffi::c_void);
    unsafe { SetupInitDefaultQueueCallback(ownerwindow.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupInitDefaultQueueCallbackEx(ownerwindow: Option<super::HWND>, alternateprogresswindow: Option<super::HWND>, progressmessage: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallbackEx(ownerwindow : super::HWND, alternateprogresswindow : super::HWND, progressmessage : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { SetupInitDefaultQueueCallbackEx(ownerwindow.unwrap_or(core::mem::zeroed()) as _, alternateprogresswindow.unwrap_or(core::mem::zeroed()) as _, progressmessage, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInitializeFileLogA<P0>(logfilename: P0, flags: u32) -> HSPFILELOG
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInitializeFileLogA(logfilename : windows_core::PCSTR, flags : u32) -> HSPFILELOG);
    unsafe { SetupInitializeFileLogA(logfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupInitializeFileLogW<P0>(logfilename: P0, flags: u32) -> HSPFILELOG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInitializeFileLogW(logfilename : windows_core::PCWSTR, flags : u32) -> HSPFILELOG);
    unsafe { SetupInitializeFileLogW(logfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupInstallFileA<P2, P3, P4>(infhandle: Option<HINF>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: u32, copymsghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileA(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCSTR, sourcepathroot : windows_core::PCSTR, destinationname : windows_core::PCSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallFileA(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInstallFileExA<P2, P3, P4>(infhandle: Option<HINF>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: u32, copymsghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>, filewasinuse: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileExA(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCSTR, sourcepathroot : windows_core::PCSTR, destinationname : windows_core::PCSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, filewasinuse : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupInstallFileExA(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _, filewasinuse as _) }
}
#[inline]
pub unsafe fn SetupInstallFileExW<P2, P3, P4>(infhandle: Option<HINF>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: u32, copymsghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>, filewasinuse: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileExW(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCWSTR, sourcepathroot : windows_core::PCWSTR, destinationname : windows_core::PCWSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, filewasinuse : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupInstallFileExW(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _, filewasinuse as _) }
}
#[inline]
pub unsafe fn SetupInstallFileW<P2, P3, P4>(infhandle: Option<HINF>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: u32, copymsghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileW(infhandle : HINF, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCWSTR, sourcepathroot : windows_core::PCWSTR, destinationname : windows_core::PCWSTR, copystyle : u32, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallFileW(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionA<P3, P4>(infhandle: HINF, layoutinfhandle: Option<HINF>, filequeue: HSPFILEQ, sectionname: P3, sourcerootpath: P4, copyflags: u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionA(infhandle : HINF, layoutinfhandle : HINF, filequeue : HSPFILEQ, sectionname : windows_core::PCSTR, sourcerootpath : windows_core::PCSTR, copyflags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallFilesFromInfSectionA(infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, filequeue, sectionname.param().abi(), sourcerootpath.param().abi(), copyflags) }
}
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionW<P3, P4>(infhandle: HINF, layoutinfhandle: Option<HINF>, filequeue: HSPFILEQ, sectionname: P3, sourcerootpath: P4, copyflags: u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionW(infhandle : HINF, layoutinfhandle : HINF, filequeue : HSPFILEQ, sectionname : windows_core::PCWSTR, sourcerootpath : windows_core::PCWSTR, copyflags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallFilesFromInfSectionW(infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, filequeue, sectionname.param().abi(), sourcerootpath.param().abi(), copyflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetupInstallFromInfSectionA<P2, P5>(owner: Option<super::HWND>, infhandle: HINF, sectionname: P2, flags: u32, relativekeyroot: Option<super::HKEY>, sourcerootpath: P5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionA(owner : super::HWND, infhandle : HINF, sectionname : windows_core::PCSTR, flags : u32, relativekeyroot : super::HKEY, sourcerootpath : windows_core::PCSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupInstallFromInfSectionA(owner.unwrap_or(core::mem::zeroed()) as _, infhandle, sectionname.param().abi(), flags, relativekeyroot.unwrap_or(core::mem::zeroed()) as _, sourcerootpath.param().abi(), copyflags, msghandler, context.unwrap_or(core::mem::zeroed()) as _, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetupInstallFromInfSectionW<P2, P5>(owner: Option<super::HWND>, infhandle: HINF, sectionname: P2, flags: u32, relativekeyroot: Option<super::HKEY>, sourcerootpath: P5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionW(owner : super::HWND, infhandle : HINF, sectionname : windows_core::PCWSTR, flags : u32, relativekeyroot : super::HKEY, sourcerootpath : windows_core::PCWSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupInstallFromInfSectionW(owner.unwrap_or(core::mem::zeroed()) as _, infhandle, sectionname.param().abi(), flags, relativekeyroot.unwrap_or(core::mem::zeroed()) as _, sourcerootpath.param().abi(), copyflags, msghandler, context.unwrap_or(core::mem::zeroed()) as _, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionA<P1>(infhandle: HINF, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionA(infhandle : HINF, sectionname : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionA(infhandle, sectionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExA<P1>(infhandle: HINF, sectionname: P1, flags: u32, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExA(infhandle : HINF, sectionname : windows_core::PCSTR, flags : u32, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionExA(infhandle, sectionname.param().abi(), flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExW<P1>(infhandle: HINF, sectionname: P1, flags: u32, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExW(infhandle : HINF, sectionname : windows_core::PCWSTR, flags : u32, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionExW(infhandle, sectionname.param().abi(), flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionW<P1>(infhandle: HINF, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionW(infhandle : HINF, sectionname : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionW(infhandle, sectionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupIterateCabinetA<P0>(cabinetfile: P0, reserved: Option<u32>, msghandler: PSP_FILE_CALLBACK_A, context: *const core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupIterateCabinetA(cabinetfile : windows_core::PCSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupIterateCabinetA(cabinetfile.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, msghandler, context) }
}
#[inline]
pub unsafe fn SetupIterateCabinetW<P0>(cabinetfile: P0, reserved: Option<u32>, msghandler: PSP_FILE_CALLBACK_W, context: *const core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupIterateCabinetW(cabinetfile : windows_core::PCWSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupIterateCabinetW(cabinetfile.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, msghandler, context) }
}
#[inline]
pub unsafe fn SetupLogErrorA<P0>(messagestring: P0, severity: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogErrorA(messagestring : windows_core::PCSTR, severity : u32) -> windows_core::BOOL);
    unsafe { SetupLogErrorA(messagestring.param().abi(), severity) }
}
#[inline]
pub unsafe fn SetupLogErrorW<P0>(messagestring: P0, severity: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogErrorW(messagestring : windows_core::PCWSTR, severity : u32) -> windows_core::BOOL);
    unsafe { SetupLogErrorW(messagestring.param().abi(), severity) }
}
#[inline]
pub unsafe fn SetupLogFileA<P1, P2, P3, P5, P6, P7>(fileloghandle: HSPFILELOG, logsectionname: P1, sourcefilename: P2, targetfilename: P3, checksum: u32, disktagfile: P5, diskdescription: P6, otherinfo: P7, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogFileA(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, checksum : u32, disktagfile : windows_core::PCSTR, diskdescription : windows_core::PCSTR, otherinfo : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupLogFileA(fileloghandle, logsectionname.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), checksum, disktagfile.param().abi(), diskdescription.param().abi(), otherinfo.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupLogFileW<P1, P2, P3, P5, P6, P7>(fileloghandle: HSPFILELOG, logsectionname: P1, sourcefilename: P2, targetfilename: P3, checksum: u32, disktagfile: P5, diskdescription: P6, otherinfo: P7, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogFileW(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, checksum : u32, disktagfile : windows_core::PCWSTR, diskdescription : windows_core::PCWSTR, otherinfo : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupLogFileW(fileloghandle, logsectionname.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), checksum, disktagfile.param().abi(), diskdescription.param().abi(), otherinfo.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupOpenAppendInfFileA<P0>(filename: P0, infhandle: HINF, errorline: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileA(filename : windows_core::PCSTR, infhandle : HINF, errorline : *mut u32) -> windows_core::BOOL);
    unsafe { SetupOpenAppendInfFileA(filename.param().abi(), infhandle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenAppendInfFileW<P0>(filename: P0, infhandle: HINF, errorline: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileW(filename : windows_core::PCWSTR, infhandle : HINF, errorline : *mut u32) -> windows_core::BOOL);
    unsafe { SetupOpenAppendInfFileW(filename.param().abi(), infhandle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenFileQueue() -> HSPFILEQ {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenFileQueue() -> HSPFILEQ);
    unsafe { SetupOpenFileQueue() }
}
#[inline]
pub unsafe fn SetupOpenInfFileA<P0, P1>(filename: P0, infclass: P1, infstyle: u32, errorline: Option<*mut u32>) -> HINF
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenInfFileA(filename : windows_core::PCSTR, infclass : windows_core::PCSTR, infstyle : u32, errorline : *mut u32) -> HINF);
    unsafe { SetupOpenInfFileA(filename.param().abi(), infclass.param().abi(), infstyle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenInfFileW<P0, P1>(filename: P0, infclass: P1, infstyle: u32, errorline: Option<*mut u32>) -> HINF
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenInfFileW(filename : windows_core::PCWSTR, infclass : windows_core::PCWSTR, infstyle : u32, errorline : *mut u32) -> HINF);
    unsafe { SetupOpenInfFileW(filename.param().abi(), infclass.param().abi(), infstyle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenLog(erase: bool) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenLog(erase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupOpenLog(erase.into()) }
}
#[inline]
pub unsafe fn SetupOpenMasterInf() -> HINF {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenMasterInf() -> HINF);
    unsafe { SetupOpenMasterInf() }
}
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreA<P1>(queuehandle: HSPFILEQ, backuppath: P1, restoreflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreA(queuehandle : HSPFILEQ, backuppath : windows_core::PCSTR, restoreflags : u32) -> windows_core::BOOL);
    unsafe { SetupPrepareQueueForRestoreA(queuehandle, backuppath.param().abi(), restoreflags) }
}
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreW<P1>(queuehandle: HSPFILEQ, backuppath: P1, restoreflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreW(queuehandle : HSPFILEQ, backuppath : windows_core::PCWSTR, restoreflags : u32) -> windows_core::BOOL);
    unsafe { SetupPrepareQueueForRestoreW(queuehandle, backuppath.param().abi(), restoreflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupPromptForDiskA<P1, P2, P3, P4, P5>(hwndparent: super::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, filesought: P4, tagfile: P5, diskpromptstyle: u32, pathbuffer: Option<windows_core::PSTR>, pathbuffersize: u32, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPromptForDiskA(hwndparent : super::HWND, dialogtitle : windows_core::PCSTR, diskname : windows_core::PCSTR, pathtosource : windows_core::PCSTR, filesought : windows_core::PCSTR, tagfile : windows_core::PCSTR, diskpromptstyle : u32, pathbuffer : windows_core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupPromptForDiskA(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), filesought.param().abi(), tagfile.param().abi(), diskpromptstyle, pathbuffer.unwrap_or(core::mem::zeroed()) as _, pathbuffersize, pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupPromptForDiskW<P1, P2, P3, P4, P5>(hwndparent: super::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, filesought: P4, tagfile: P5, diskpromptstyle: u32, pathbuffer: Option<windows_core::PWSTR>, pathbuffersize: u32, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPromptForDiskW(hwndparent : super::HWND, dialogtitle : windows_core::PCWSTR, diskname : windows_core::PCWSTR, pathtosource : windows_core::PCWSTR, filesought : windows_core::PCWSTR, tagfile : windows_core::PCWSTR, diskpromptstyle : u32, pathbuffer : windows_core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupPromptForDiskW(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), filesought.param().abi(), tagfile.param().abi(), diskpromptstyle, pathbuffer.unwrap_or(core::mem::zeroed()) as _, pathbuffersize, pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupPromptReboot(filequeue: Option<HSPFILEQ>, owner: Option<super::HWND>, scanonly: bool) -> i32 {
    windows_core::link!("setupapi.dll" "system" fn SetupPromptReboot(filequeue : HSPFILEQ, owner : super::HWND, scanonly : windows_core::BOOL) -> i32);
    unsafe { SetupPromptReboot(filequeue.unwrap_or(core::mem::zeroed()) as _, owner.unwrap_or(core::mem::zeroed()) as _, scanonly.into()) }
}
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListA(diskspace: HDSKSPC, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListA(diskspace : HDSKSPC, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryDrivesInDiskSpaceListA(diskspace, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListW(diskspace: HDSKSPC, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListW(diskspace : HDSKSPC, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryDrivesInDiskSpaceListW(diskspace, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryFileLogA<P1, P2>(fileloghandle: HSPFILELOG, logsectionname: P1, targetfilename: P2, desiredinfo: SetupFileLogInfo, dataout: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryFileLogA(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCSTR, targetfilename : windows_core::PCSTR, desiredinfo : SetupFileLogInfo, dataout : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryFileLogA(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi(), desiredinfo, dataout.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryFileLogW<P1, P2>(fileloghandle: HSPFILELOG, logsectionname: P1, targetfilename: P2, desiredinfo: SetupFileLogInfo, dataout: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryFileLogW(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, desiredinfo : SetupFileLogInfo, dataout : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryFileLogW(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi(), desiredinfo, dataout.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfFileInformationA(infinformation, infindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfFileInformationW(infinformation, infindex, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : PSP_ALTPLATFORM_INFO, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_A) -> windows_core::BOOL);
    unsafe { SetupQueryInfOriginalFileInformationA(infinformation, infindex, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, originalfileinfo as _) }
}
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : PSP_ALTPLATFORM_INFO, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_W) -> windows_core::BOOL);
    unsafe { SetupQueryInfOriginalFileInformationW(infinformation, infindex, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, originalfileinfo as _) }
}
#[inline]
pub unsafe fn SetupQueryInfVersionInformationA<P2>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: P2, returnbuffer: Option<windows_core::PSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfVersionInformationA(infinformation, infindex, key.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueryInfVersionInformationW<P2>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: P2, returnbuffer: Option<windows_core::PWSTR>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfVersionInformationW(infinformation, infindex, key.param().abi(), returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQuerySourceListA(flags: u32, list: *mut *mut windows_core::PCSTR, count: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySourceListA(flags : u32, list : *mut *mut windows_core::PCSTR, count : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQuerySourceListA(flags, list as _, count as _) }
}
#[inline]
pub unsafe fn SetupQuerySourceListW(flags: u32, list: *mut *mut windows_core::PCWSTR, count: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySourceListW(flags : u32, list : *mut *mut windows_core::PCWSTR, count : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQuerySourceListW(flags, list as _, count as _) }
}
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveA<P1>(diskspace: HDSKSPC, drivespec: P1, spacerequired: *mut i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveA(diskspace : HDSKSPC, drivespec : windows_core::PCSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupQuerySpaceRequiredOnDriveA(diskspace, drivespec.param().abi(), spacerequired as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveW<P1>(diskspace: HDSKSPC, drivespec: P1, spacerequired: *mut i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveW(diskspace : HDSKSPC, drivespec : windows_core::PCWSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupQuerySpaceRequiredOnDriveW(diskspace, drivespec.param().abi(), spacerequired as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupQueueCopyA<P1, P2, P3, P4, P5, P6, P7>(queuehandle: HSPFILEQ, sourcerootpath: P1, sourcepath: P2, sourcefilename: P3, sourcedescription: P4, sourcetagfile: P5, targetdirectory: P6, targetfilename: P7, copystyle: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyA(queuehandle : HSPFILEQ, sourcerootpath : windows_core::PCSTR, sourcepath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, sourcedescription : windows_core::PCSTR, sourcetagfile : windows_core::PCSTR, targetdirectory : windows_core::PCSTR, targetfilename : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopyA(queuehandle, sourcerootpath.param().abi(), sourcepath.param().abi(), sourcefilename.param().abi(), sourcedescription.param().abi(), sourcetagfile.param().abi(), targetdirectory.param().abi(), targetfilename.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectA(copyparams : *const SP_FILE_COPY_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupQueueCopyIndirectA(copyparams) }
}
#[inline]
pub unsafe fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectW(copyparams : *const SP_FILE_COPY_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupQueueCopyIndirectW(copyparams) }
}
#[inline]
pub unsafe fn SetupQueueCopySectionA<P1, P4>(queuehandle: HSPFILEQ, sourcerootpath: P1, infhandle: HINF, listinfhandle: Option<HINF>, section: P4, copystyle: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopySectionA(queuehandle : HSPFILEQ, sourcerootpath : windows_core::PCSTR, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopySectionA(queuehandle, sourcerootpath.param().abi(), infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueCopySectionW<P1, P4>(queuehandle: HSPFILEQ, sourcerootpath: P1, infhandle: HINF, listinfhandle: Option<HINF>, section: P4, copystyle: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopySectionW(queuehandle : HSPFILEQ, sourcerootpath : windows_core::PCWSTR, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopySectionW(queuehandle, sourcerootpath.param().abi(), infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueCopyW<P1, P2, P3, P4, P5, P6, P7>(queuehandle: HSPFILEQ, sourcerootpath: P1, sourcepath: P2, sourcefilename: P3, sourcedescription: P4, sourcetagfile: P5, targetdirectory: P6, targetfilename: P7, copystyle: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyW(queuehandle : HSPFILEQ, sourcerootpath : windows_core::PCWSTR, sourcepath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, sourcedescription : windows_core::PCWSTR, sourcetagfile : windows_core::PCWSTR, targetdirectory : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopyW(queuehandle, sourcerootpath.param().abi(), sourcepath.param().abi(), sourcefilename.param().abi(), sourcedescription.param().abi(), sourcetagfile.param().abi(), targetdirectory.param().abi(), targetfilename.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueDefaultCopyA<P2, P3, P4>(queuehandle: HSPFILEQ, infhandle: HINF, sourcerootpath: P2, sourcefilename: P3, targetfilename: P4, copystyle: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyA(queuehandle : HSPFILEQ, infhandle : HINF, sourcerootpath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueDefaultCopyA(queuehandle, infhandle, sourcerootpath.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueDefaultCopyW<P2, P3, P4>(queuehandle: HSPFILEQ, infhandle: HINF, sourcerootpath: P2, sourcefilename: P3, targetfilename: P4, copystyle: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyW(queuehandle : HSPFILEQ, infhandle : HINF, sourcerootpath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueDefaultCopyW(queuehandle, infhandle, sourcerootpath.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), copystyle) }
}
#[inline]
pub unsafe fn SetupQueueDeleteA<P1, P2>(queuehandle: HSPFILEQ, pathpart1: P1, pathpart2: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteA(queuehandle : HSPFILEQ, pathpart1 : windows_core::PCSTR, pathpart2 : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteA(queuehandle, pathpart1.param().abi(), pathpart2.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueDeleteSectionA<P3>(queuehandle: HSPFILEQ, infhandle: HINF, listinfhandle: Option<HINF>, section: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionA(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteSectionA(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueDeleteSectionW<P3>(queuehandle: HSPFILEQ, infhandle: HINF, listinfhandle: Option<HINF>, section: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionW(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteSectionW(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueDeleteW<P1, P2>(queuehandle: HSPFILEQ, pathpart1: P1, pathpart2: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteW(queuehandle : HSPFILEQ, pathpart1 : windows_core::PCWSTR, pathpart2 : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteW(queuehandle, pathpart1.param().abi(), pathpart2.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueRenameA<P1, P2, P3, P4>(queuehandle: HSPFILEQ, sourcepath: P1, sourcefilename: P2, targetpath: P3, targetfilename: P4) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameA(queuehandle : HSPFILEQ, sourcepath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetpath : windows_core::PCSTR, targetfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameA(queuehandle, sourcepath.param().abi(), sourcefilename.param().abi(), targetpath.param().abi(), targetfilename.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueRenameSectionA<P3>(queuehandle: HSPFILEQ, infhandle: HINF, listinfhandle: Option<HINF>, section: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameSectionA(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameSectionA(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueRenameSectionW<P3>(queuehandle: HSPFILEQ, infhandle: HINF, listinfhandle: Option<HINF>, section: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameSectionW(queuehandle : HSPFILEQ, infhandle : HINF, listinfhandle : HINF, section : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameSectionW(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupQueueRenameW<P1, P2, P3, P4>(queuehandle: HSPFILEQ, sourcepath: P1, sourcefilename: P2, targetpath: P3, targetfilename: P4) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameW(queuehandle : HSPFILEQ, sourcepath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetpath : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameW(queuehandle, sourcepath.param().abi(), sourcefilename.param().abi(), targetpath.param().abi(), targetfilename.param().abi()) }
}
#[inline]
pub unsafe fn SetupRemoveFileLogEntryA<P1, P2>(fileloghandle: HSPFILELOG, logsectionname: P1, targetfilename: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryA(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCSTR, targetfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFileLogEntryA(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi()) }
}
#[inline]
pub unsafe fn SetupRemoveFileLogEntryW<P1, P2>(fileloghandle: HSPFILELOG, logsectionname: P1, targetfilename: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryW(fileloghandle : HSPFILELOG, logsectionname : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFileLogEntryW(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi()) }
}
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListA<P1>(diskspace: HDSKSPC, targetfilespec: P1, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListA(diskspace : HDSKSPC, targetfilespec : windows_core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveFromDiskSpaceListA(diskspace, targetfilespec.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListW<P1>(diskspace: HDSKSPC, targetfilespec: P1, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListW(diskspace : HDSKSPC, targetfilespec : windows_core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveFromDiskSpaceListW(diskspace, targetfilespec.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupRemoveFromSourceListA<P1>(flags: u32, source: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListA(flags : u32, source : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFromSourceListA(flags, source.param().abi()) }
}
#[inline]
pub unsafe fn SetupRemoveFromSourceListW<P1>(flags: u32, source: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListW(flags : u32, source : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFromSourceListW(flags, source.param().abi()) }
}
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListA<P3>(diskspace: HDSKSPC, infhandle: HINF, layoutinfhandle: Option<HINF>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveInstallSectionFromDiskSpaceListA(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListW<P3>(diskspace: HDSKSPC, infhandle: HINF, layoutinfhandle: Option<HINF>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, layoutinfhandle : HINF, sectionname : windows_core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveInstallSectionFromDiskSpaceListW(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListA<P3>(diskspace: HDSKSPC, infhandle: HINF, listinfhandle: Option<HINF>, sectionname: P3, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListA(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_core::PCSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveSectionFromDiskSpaceListA(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListW<P3>(diskspace: HDSKSPC, infhandle: HINF, listinfhandle: Option<HINF>, sectionname: P3, operation: u32, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListW(diskspace : HDSKSPC, infhandle : HINF, listinfhandle : HINF, sectionname : windows_core::PCWSTR, operation : u32, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveSectionFromDiskSpaceListW(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupRenameErrorA<P1, P2, P3>(hwndparent: super::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRenameErrorA(hwndparent : super::HWND, dialogtitle : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetfile : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupRenameErrorA(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupRenameErrorW<P1, P2, P3>(hwndparent: super::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRenameErrorW(hwndparent : super::HWND, dialogtitle : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetfile : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupRenameErrorW(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupScanFileQueueA(filequeue: HSPFILEQ, flags: u32, window: Option<super::HWND>, callbackroutine: PSP_FILE_CALLBACK_A, callbackcontext: Option<*const core::ffi::c_void>, result: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupScanFileQueueA(filequeue : HSPFILEQ, flags : u32, window : super::HWND, callbackroutine : PSP_FILE_CALLBACK_A, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_core::BOOL);
    unsafe { SetupScanFileQueueA(filequeue, flags, window.unwrap_or(core::mem::zeroed()) as _, callbackroutine, callbackcontext.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetupScanFileQueueW(filequeue: HSPFILEQ, flags: u32, window: Option<super::HWND>, callbackroutine: PSP_FILE_CALLBACK_W, callbackcontext: Option<*const core::ffi::c_void>, result: *mut u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupScanFileQueueW(filequeue : HSPFILEQ, flags : u32, window : super::HWND, callbackroutine : PSP_FILE_CALLBACK_W, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_core::BOOL);
    unsafe { SetupScanFileQueueW(filequeue, flags, window.unwrap_or(core::mem::zeroed()) as _, callbackroutine, callbackcontext.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdA<P2>(infhandle: HINF, id: u32, directory: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdA(infhandle : HINF, id : u32, directory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdA(infhandle, id, directory.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdExA<P2>(infhandle: HINF, id: u32, directory: P2, flags: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExA(infhandle : HINF, id : u32, directory : windows_core::PCSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdExA(infhandle, id, directory.param().abi(), flags, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdExW<P2>(infhandle: HINF, id: u32, directory: P2, flags: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExW(infhandle : HINF, id : u32, directory : windows_core::PCWSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdExW(infhandle, id, directory.param().abi(), flags, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdW<P2>(infhandle: HINF, id: u32, directory: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdW(infhandle : HINF, id : u32, directory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdW(infhandle, id, directory.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformA<P2>(queuehandle: HSPFILEQ, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, alternatedefaultcatalogfile: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformA(queuehandle : HSPFILEQ, alternateplatforminfo : PSP_ALTPLATFORM_INFO, alternatedefaultcatalogfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueAlternatePlatformA(queuehandle, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, alternatedefaultcatalogfile.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformW<P2>(queuehandle: HSPFILEQ, alternateplatforminfo: Option<PSP_ALTPLATFORM_INFO>, alternatedefaultcatalogfile: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformW(queuehandle : HSPFILEQ, alternateplatforminfo : PSP_ALTPLATFORM_INFO, alternatedefaultcatalogfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueAlternatePlatformW(queuehandle, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, alternatedefaultcatalogfile.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetFileQueueFlags(filequeue: HSPFILEQ, flagmask: u32, flags: u32) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueFlags(filequeue : HSPFILEQ, flagmask : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueFlags(filequeue, flagmask, flags) }
}
#[inline]
pub unsafe fn SetupSetNonInteractiveMode(noninteractiveflag: bool) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupSetNonInteractiveMode(noninteractiveflag : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupSetNonInteractiveMode(noninteractiveflag.into()) }
}
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideA<P0>(r#override: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideA(r#override : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetPlatformPathOverrideA(r#override.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideW<P0>(r#override: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideW(r#override : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetPlatformPathOverrideW(r#override.param().abi()) }
}
#[inline]
pub unsafe fn SetupSetSourceListA(flags: u32, sourcelist: &[windows_core::PCSTR]) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupSetSourceListA(flags : u32, sourcelist : *const windows_core::PCSTR, sourcecount : u32) -> windows_core::BOOL);
    unsafe { SetupSetSourceListA(flags, sourcelist.as_ptr(), sourcelist.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetupSetSourceListW(flags: u32, sourcelist: &[windows_core::PCWSTR]) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupSetSourceListW(flags : u32, sourcelist : *const windows_core::PCWSTR, sourcecount : u32) -> windows_core::BOOL);
    unsafe { SetupSetSourceListW(flags, sourcelist.as_ptr(), sourcelist.len().try_into().unwrap()) }
}
#[cfg(all(feature = "spapidef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupSetThreadLogToken(logtoken: super::SP_LOG_TOKEN) {
    windows_core::link!("setupapi.dll" "system" fn SetupSetThreadLogToken(logtoken : super::SP_LOG_TOKEN));
    unsafe { SetupSetThreadLogToken(logtoken) }
}
#[inline]
pub unsafe fn SetupTermDefaultQueueCallback(context: *const core::ffi::c_void) {
    windows_core::link!("setupapi.dll" "system" fn SetupTermDefaultQueueCallback(context : *const core::ffi::c_void));
    unsafe { SetupTermDefaultQueueCallback(context) }
}
#[inline]
pub unsafe fn SetupTerminateFileLog(fileloghandle: HSPFILELOG) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupTerminateFileLog(fileloghandle : HSPFILELOG) -> windows_core::BOOL);
    unsafe { SetupTerminateFileLog(fileloghandle) }
}
#[inline]
pub unsafe fn SetupUninstallNewlyCopiedInfs(filequeue: HSPFILEQ, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallNewlyCopiedInfs(filequeue : HSPFILEQ, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallNewlyCopiedInfs(filequeue, flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupUninstallOEMInfA<P0>(inffilename: P0, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallOEMInfA(inffilename : windows_core::PCSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallOEMInfA(inffilename.param().abi(), flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupUninstallOEMInfW<P0>(inffilename: P0, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallOEMInfW(inffilename : windows_core::PCWSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallOEMInfW(inffilename.param().abi(), flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupVerifyInfFileA<P0>(infname: P0, altplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsignerinfo: PSP_INF_SIGNER_INFO_A) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupVerifyInfFileA(infname : windows_core::PCSTR, altplatforminfo : PSP_ALTPLATFORM_INFO, infsignerinfo : PSP_INF_SIGNER_INFO_A) -> windows_core::BOOL);
    unsafe { SetupVerifyInfFileA(infname.param().abi(), altplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsignerinfo as _) }
}
#[inline]
pub unsafe fn SetupVerifyInfFileW<P0>(infname: P0, altplatforminfo: Option<PSP_ALTPLATFORM_INFO>, infsignerinfo: PSP_INF_SIGNER_INFO_W) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupVerifyInfFileW(infname : windows_core::PCWSTR, altplatforminfo : PSP_ALTPLATFORM_INFO, infsignerinfo : PSP_INF_SIGNER_INFO_W) -> windows_core::BOOL);
    unsafe { SetupVerifyInfFileW(infname.param().abi(), altplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsignerinfo as _) }
}
#[cfg(all(feature = "spapidef", feature = "winnt"))]
#[inline]
pub unsafe fn SetupWriteTextLogInfLine(logtoken: super::SP_LOG_TOKEN, flags: u32, infhandle: HINF, context: *const INFCONTEXT) {
    windows_core::link!("setupapi.dll" "system" fn SetupWriteTextLogInfLine(logtoken : super::SP_LOG_TOKEN, flags : u32, infhandle : HINF, context : *const INFCONTEXT));
    unsafe { SetupWriteTextLogInfLine(logtoken, flags, infhandle, context) }
}
pub type CABINET_INFO = CABINET_INFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_core::PCSTR,
    pub CabinetFile: windows_core::PCSTR,
    pub DiskName: windows_core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_core::PCSTR,
    pub CabinetFile: windows_core::PCSTR,
    pub DiskName: windows_core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_core::PCWSTR,
    pub CabinetFile: windows_core::PCWSTR,
    pub DiskName: windows_core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_core::PCWSTR,
    pub CabinetFile: windows_core::PCWSTR,
    pub DiskName: windows_core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
pub const COPYFLG_FORCE_FILE_IN_USE: i32 = 8;
pub const COPYFLG_IN_USE_TRY_RENAME: i32 = 16384;
pub const COPYFLG_NODECOMP: i32 = 2048;
pub const COPYFLG_NOPRUNE: i32 = 8192;
pub const COPYFLG_NOSKIP: i32 = 2;
pub const COPYFLG_NOVERSIONCHECK: i32 = 4;
pub const COPYFLG_NO_OVERWRITE: i32 = 16;
pub const COPYFLG_NO_VERSION_DIALOG: i32 = 32;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: i32 = 64;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: i32 = 256;
pub const COPYFLG_REPLACEONLY: i32 = 1024;
pub const COPYFLG_REPLACE_BOOT_FILE: i32 = 4096;
pub const COPYFLG_WARN_IF_SKIP: i32 = 1;
pub const DELFLG_IN_USE: i32 = 1;
pub const DELFLG_IN_USE1: i32 = 65536;
pub const DIBCI_NODISPLAYCLASS: i32 = 2;
pub const DIBCI_NOINSTALLCLASS: i32 = 1;
pub const DICD_GENERATE_ID: i32 = 1;
pub const DICD_INHERIT_CLASSDRVS: i32 = 2;
pub const DICLASSPROP_INSTALLER: i32 = 1;
pub const DICLASSPROP_INTERFACE: i32 = 2;
pub const DICS_DISABLE: i32 = 2;
pub const DICS_ENABLE: i32 = 1;
pub const DICS_FLAG_CONFIGGENERAL: i32 = 4;
pub const DICS_FLAG_CONFIGSPECIFIC: i32 = 2;
pub const DICS_FLAG_GLOBAL: i32 = 1;
pub const DICS_PROPCHANGE: i32 = 3;
pub const DICS_START: i32 = 4;
pub const DICS_STOP: i32 = 5;
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: i32 = 1;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: i32 = 35;
pub const DIF_ADDPROPERTYPAGE_BASIC: i32 = 36;
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: i32 = 40;
pub const DIF_ALLOW_INSTALL: i32 = 24;
pub const DIF_ASSIGNRESOURCES: i32 = 3;
pub const DIF_CALCDISKSPACE: i32 = 11;
pub const DIF_DESTROYPRIVATEDATA: i32 = 12;
pub const DIF_DESTROYWIZARDDATA: i32 = 17;
pub const DIF_DETECT: i32 = 15;
pub const DIF_DETECTCANCEL: i32 = 33;
pub const DIF_DETECTVERIFY: i32 = 20;
pub const DIF_ENABLECLASS: i32 = 19;
pub const DIF_FINISHINSTALL_ACTION: i32 = 42;
pub const DIF_FIRSTTIMESETUP: i32 = 6;
pub const DIF_FOUNDDEVICE: i32 = 7;
pub const DIF_INSTALLCLASSDRIVERS: i32 = 10;
pub const DIF_INSTALLDEVICE: i32 = 2;
pub const DIF_INSTALLDEVICEFILES: i32 = 21;
pub const DIF_INSTALLINTERFACES: i32 = 32;
pub const DIF_INSTALLWIZARD: i32 = 16;
pub const DIF_MOVEDEVICE: i32 = 14;
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: i32 = 30;
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: i32 = 29;
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: i32 = 28;
pub const DIF_NEWDEVICEWIZARD_PRESELECT: i32 = 26;
pub const DIF_NEWDEVICEWIZARD_SELECT: i32 = 27;
pub const DIF_POWERMESSAGEWAKE: i32 = 39;
pub const DIF_PROPERTIES: i32 = 4;
pub const DIF_PROPERTYCHANGE: i32 = 18;
pub const DIF_REGISTERDEVICE: i32 = 25;
pub const DIF_REGISTER_COINSTALLERS: i32 = 34;
pub const DIF_REMOVE: i32 = 5;
pub const DIF_RESERVED1: i32 = 37;
pub const DIF_RESERVED2: i32 = 48;
pub const DIF_SELECTBESTCOMPATDRV: i32 = 23;
pub const DIF_SELECTCLASSDRIVERS: i32 = 8;
pub const DIF_SELECTDEVICE: i32 = 1;
pub const DIF_TROUBLESHOOTER: i32 = 38;
pub const DIF_UNREMOVE: i32 = 22;
pub const DIF_UNUSED1: i32 = 31;
pub const DIF_UPDATEDRIVER_UI: i32 = 41;
pub const DIF_VALIDATECLASSDRIVERS: i32 = 9;
pub const DIF_VALIDATEDRIVER: i32 = 13;
pub const DIGCDP_FLAG_ADVANCED: i32 = 2;
pub const DIGCDP_FLAG_BASIC: i32 = 1;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: i32 = 4;
pub const DIGCDP_FLAG_REMOTE_BASIC: i32 = 3;
pub const DIGCF_ALLCLASSES: i32 = 4;
pub const DIGCF_DEFAULT: i32 = 1;
pub const DIGCF_DEVICEINTERFACE: i32 = 16;
pub const DIGCF_INTERFACEDEVICE: i32 = 16;
pub const DIGCF_PRESENT: i32 = 2;
pub const DIGCF_PROFILE: i32 = 8;
pub const DIOCR_INSTALLER: i32 = 1;
pub const DIOCR_INTERFACE: i32 = 2;
pub const DIODI_NO_ADD: i32 = 1;
pub const DIOD_CANCEL_REMOVE: i32 = 4;
pub const DIOD_INHERIT_CLASSDRVS: i32 = 2;
pub const DIREG_BOTH: i32 = 4;
pub const DIREG_DEV: i32 = 1;
pub const DIREG_DRV: i32 = 2;
pub const DIRID_ABSOLUTE: i32 = -1;
pub const DIRID_ABSOLUTE_16BIT: i32 = 65535;
pub const DIRID_APPS: i32 = 24;
pub const DIRID_BOOT: i32 = 30;
pub const DIRID_COLOR: i32 = 23;
pub const DIRID_COMMON_APPDATA: i32 = 16419;
pub const DIRID_COMMON_DESKTOPDIRECTORY: i32 = 16409;
pub const DIRID_COMMON_DOCUMENTS: i32 = 16430;
pub const DIRID_COMMON_FAVORITES: i32 = 16415;
pub const DIRID_COMMON_PROGRAMS: i32 = 16407;
pub const DIRID_COMMON_STARTMENU: i32 = 16406;
pub const DIRID_COMMON_STARTUP: i32 = 16408;
pub const DIRID_COMMON_TEMPLATES: i32 = 16429;
pub const DIRID_DEFAULT: i32 = 11;
pub const DIRID_DRIVERS: i32 = 12;
pub const DIRID_DRIVER_STORE: i32 = 13;
pub const DIRID_FONTS: i32 = 20;
pub const DIRID_HELP: i32 = 18;
pub const DIRID_INF: i32 = 17;
pub const DIRID_IOSUBSYS: i32 = 12;
pub const DIRID_LOADER: i32 = 54;
pub const DIRID_NULL: i32 = 0;
pub const DIRID_PRINTPROCESSOR: i32 = 55;
pub const DIRID_PROGRAM_FILES: i32 = 16422;
pub const DIRID_PROGRAM_FILES_COMMON: i32 = 16427;
pub const DIRID_PROGRAM_FILES_COMMONX86: i32 = 16428;
pub const DIRID_PROGRAM_FILES_X86: i32 = 16426;
pub const DIRID_SHARED: i32 = 25;
pub const DIRID_SPOOL: i32 = 51;
pub const DIRID_SPOOLDRIVERS: i32 = 52;
pub const DIRID_SRCPATH: i32 = 1;
pub const DIRID_SYSTEM: i32 = 11;
pub const DIRID_SYSTEM16: i32 = 50;
pub const DIRID_SYSTEM_X86: i32 = 16425;
pub const DIRID_USER: i32 = 32768;
pub const DIRID_USERPROFILE: i32 = 53;
pub const DIRID_VIEWERS: i32 = 21;
pub const DIRID_WINDOWS: i32 = 10;
pub const DI_AUTOASSIGNRES: i32 = 64;
pub const DI_CLASSINSTALLPARAMS: i32 = 1048576;
pub const DI_COMPAT_FROM_CLASS: i32 = 524288;
pub const DI_DIDCLASS: i32 = 32;
pub const DI_DIDCOMPAT: i32 = 16;
pub const DI_DISABLED: i32 = 2048;
pub const DI_DONOTCALLCONFIGMG: i32 = 131072;
pub const DI_DRIVERPAGE_ADDED: i32 = 67108864;
pub const DI_ENUMSINGLEINF: i32 = 65536;
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: i32 = 2048;
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: i32 = 268435456;
pub const DI_FLAGSEX_ALWAYSWRITEIDS: i32 = 512;
pub const DI_FLAGSEX_APPENDDRIVERLIST: i32 = 262144;
pub const DI_FLAGSEX_BACKUPONREPLACE: i32 = 1048576;
pub const DI_FLAGSEX_CI_FAILED: i32 = 4;
pub const DI_FLAGSEX_DEVICECHANGE: i32 = 256;
pub const DI_FLAGSEX_DIDCOMPATINFO: i32 = 32;
pub const DI_FLAGSEX_DIDINFOLIST: i32 = 16;
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: i32 = 2097152;
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: i32 = 8388608;
pub const DI_FLAGSEX_FILTERCLASSES: i32 = 64;
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: i32 = 33554432;
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: i32 = 8;
pub const DI_FLAGSEX_INET_DRIVER: i32 = 131072;
pub const DI_FLAGSEX_INSTALLEDDRIVER: i32 = 67108864;
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: i32 = 65536;
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: i32 = 4096;
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: i32 = 134217728;
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: i32 = 32768;
pub const DI_FLAGSEX_POWERPAGE_ADDED: i32 = 16777216;
pub const DI_FLAGSEX_PREINSTALLBACKUP: i32 = 524288;
pub const DI_FLAGSEX_PROPCHANGE_PENDING: i32 = 1024;
pub const DI_FLAGSEX_RECURSIVESEARCH: i32 = 1073741824;
pub const DI_FLAGSEX_RESERVED1: i32 = 4194304;
pub const DI_FLAGSEX_RESERVED2: i32 = 1;
pub const DI_FLAGSEX_RESERVED3: i32 = 2;
pub const DI_FLAGSEX_RESERVED4: i32 = 16384;
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: i32 = 536870912;
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: u32 = 2147483648;
pub const DI_FLAGSEX_SETFAILEDINSTALL: i32 = 128;
pub const DI_FLAGSEX_USECLASSFORCOMPAT: i32 = 8192;
pub const DI_FORCECOPY: i32 = 33554432;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DI_FUNCTION(pub u32);
pub const DI_GENERALPAGE_ADDED: i32 = 4096;
pub const DI_INF_IS_SORTED: i32 = 32768;
pub const DI_INSTALLDISABLED: i32 = 262144;
pub const DI_MULTMFGS: i32 = 1024;
pub const DI_NEEDREBOOT: i32 = 256;
pub const DI_NEEDRESTART: i32 = 128;
pub const DI_NOBROWSE: i32 = 512;
pub const DI_NODI_DEFAULTACTION: i32 = 2097152;
pub const DI_NOFILECOPY: i32 = 16777216;
pub const DI_NOSELECTICONS: i32 = 1073741824;
pub const DI_NOVCP: i32 = 8;
pub const DI_NOWRITE_IDS: u32 = 2147483648;
pub const DI_OVERRIDE_INFFLAGS: i32 = 268435456;
pub const DI_PROPERTIES_CHANGE: i32 = 16384;
pub const DI_PROPS_NOCHANGEUSAGE: i32 = 536870912;
pub const DI_QUIETINSTALL: i32 = 8388608;
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: i32 = 2;
pub const DI_REMOVEDEVICE_GLOBAL: i32 = 1;
pub const DI_RESOURCEPAGE_ADDED: i32 = 8192;
pub const DI_SHOWALL: i32 = 7;
pub const DI_SHOWCLASS: i32 = 4;
pub const DI_SHOWCOMPAT: i32 = 2;
pub const DI_SHOWOEM: i32 = 1;
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: i32 = 2;
pub const DI_USECI_SELECTSTRINGS: i32 = 134217728;
pub const DMI_BKCOLOR: i32 = 2;
pub const DMI_MASK: i32 = 1;
pub const DMI_USERECT: i32 = 4;
pub const DNF_ALWAYSEXCLUDEFROMLIST: i32 = 524288;
pub const DNF_AUTHENTICODE_SIGNED: i32 = 131072;
pub const DNF_BAD_DRIVER: i32 = 2048;
pub const DNF_BASIC_DRIVER: i32 = 65536;
pub const DNF_CLASS_DRIVER: i32 = 32;
pub const DNF_COMPATIBLE_DRIVER: i32 = 64;
pub const DNF_DUPDESC: i32 = 1;
pub const DNF_DUPDRIVERVER: i32 = 32768;
pub const DNF_DUPPROVIDER: i32 = 4096;
pub const DNF_EXCLUDEFROMLIST: i32 = 4;
pub const DNF_INBOX_DRIVER: i32 = 1048576;
pub const DNF_INET_DRIVER: i32 = 128;
pub const DNF_INF_IS_SIGNED: i32 = 8192;
pub const DNF_INSTALLEDDRIVER: i32 = 262144;
pub const DNF_LEGACYINF: i32 = 16;
pub const DNF_NODRIVER: i32 = 8;
pub const DNF_OEM_F6_INF: i32 = 16384;
pub const DNF_OLDDRIVER: i32 = 2;
pub const DNF_OLD_INET_DRIVER: i32 = 1024;
pub const DNF_REQUESTADDITIONALSOFTWARE: i32 = 2097152;
pub const DNF_UNUSED1: i32 = 256;
pub const DNF_UNUSED2: i32 = 512;
pub const DNF_UNUSED_22: i32 = 4194304;
pub const DNF_UNUSED_23: i32 = 8388608;
pub const DNF_UNUSED_24: i32 = 16777216;
pub const DNF_UNUSED_25: i32 = 33554432;
pub const DNF_UNUSED_26: i32 = 67108864;
pub const DNF_UNUSED_27: i32 = 134217728;
pub const DNF_UNUSED_28: i32 = 268435456;
pub const DNF_UNUSED_29: i32 = 536870912;
pub const DNF_UNUSED_30: i32 = 1073741824;
pub const DNF_UNUSED_31: u32 = 2147483648;
pub const DPROMPT_BUFFERTOOSMALL: i32 = 3;
pub const DPROMPT_CANCEL: i32 = 1;
pub const DPROMPT_OUTOFMEMORY: i32 = 4;
pub const DPROMPT_SKIPFILE: i32 = 2;
pub const DPROMPT_SUCCESS: i32 = 0;
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743;
pub const DRIVER_HARDWAREID_RANK: i32 = 4095;
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648;
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: i32 = 8;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: i32 = 2;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: i32 = 4;
pub const DYNAWIZ_FLAG_PAGESADDED: i32 = 1;
pub const ENABLECLASS_FAILURE: i32 = 2;
pub const ENABLECLASS_QUERY: i32 = 0;
pub const ENABLECLASS_SUCCESS: i32 = 1;
pub const ERROR_AUTHENTICODE_DISALLOWED: u32 = 3758096960;
pub const ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: u32 = 3758096963;
pub const ERROR_AUTHENTICODE_TRUSTED_PUBLISHER: u32 = 3758096961;
pub const ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED: u32 = 3758096962;
pub const ERROR_BAD_INTERFACE_INSTALLSECT: u32 = 3758096925;
pub const ERROR_BAD_SECTION_NAME_LINE: u32 = 3758096385;
pub const ERROR_BAD_SERVICE_INSTALLSECT: u32 = 3758096919;
pub const ERROR_CANT_LOAD_CLASS_ICON: u32 = 3758096908;
pub const ERROR_CANT_REMOVE_DEVINST: u32 = 3758096946;
pub const ERROR_CLASS_MISMATCH: u32 = 3758096897;
pub const ERROR_DEVICE_INSTALLER_NOT_READY: u32 = 3758096966;
pub const ERROR_DEVICE_INSTALL_BLOCKED: u32 = 3758096968;
pub const ERROR_DEVICE_INTERFACE_ACTIVE: u32 = 3758096923;
pub const ERROR_DEVICE_INTERFACE_REMOVED: u32 = 3758096924;
pub const ERROR_DEVINFO_DATA_LOCKED: u32 = 3758096915;
pub const ERROR_DEVINFO_LIST_LOCKED: u32 = 3758096914;
pub const ERROR_DEVINFO_NOT_REGISTERED: u32 = 3758096904;
pub const ERROR_DEVINSTALL_QUEUE_NONNATIVE: u32 = 3758096944;
pub const ERROR_DEVINST_ALREADY_EXISTS: u32 = 3758096903;
pub const ERROR_DI_BAD_PATH: u32 = 3758096916;
pub const ERROR_DI_DONT_INSTALL: u32 = 3758096939;
pub const ERROR_DI_DO_DEFAULT: u32 = 3758096910;
pub const ERROR_DI_FUNCTION_OBSOLETE: u32 = 3758096958;
pub const ERROR_DI_NOFILECOPY: u32 = 3758096911;
pub const ERROR_DI_POSTPROCESSING_REQUIRED: u32 = 3758096934;
pub const ERROR_DRIVER_INSTALL_BLOCKED: u32 = 3758096969;
pub const ERROR_DRIVER_NONNATIVE: u32 = 3758096948;
pub const ERROR_DRIVER_STORE_ADD_FAILED: u32 = 3758096967;
pub const ERROR_DRIVER_STORE_DELETE_FAILED: u32 = 3758096972;
pub const ERROR_DUPLICATE_FOUND: u32 = 3758096898;
pub const ERROR_EXPECTED_SECTION_NAME: u32 = 3758096384;
pub const ERROR_FILEQUEUE_LOCKED: u32 = 3758096918;
pub const ERROR_FILE_HASH_NOT_IN_CATALOG: u32 = 3758096971;
pub const ERROR_GENERAL_SYNTAX: u32 = 3758096387;
pub const ERROR_INF_IN_USE_BY_DEVICES: u32 = 3758096957;
pub const ERROR_INTERFACE_DEVICE_ACTIVE: u32 = 3758096923;
pub const ERROR_INTERFACE_DEVICE_REMOVED: u32 = 3758096924;
pub const ERROR_INVALID_CLASS: u32 = 3758096902;
pub const ERROR_INVALID_CLASS_INSTALLER: u32 = 3758096909;
pub const ERROR_INVALID_COINSTALLER: u32 = 3758096935;
pub const ERROR_INVALID_DEVINST_NAME: u32 = 3758096901;
pub const ERROR_INVALID_FILTER_DRIVER: u32 = 3758096940;
pub const ERROR_INVALID_HWPROFILE: u32 = 3758096912;
pub const ERROR_INVALID_INF_LOGCONFIG: u32 = 3758096938;
pub const ERROR_INVALID_MACHINENAME: u32 = 3758096928;
pub const ERROR_INVALID_PROPPAGE_PROVIDER: u32 = 3758096932;
pub const ERROR_INVALID_REFERENCE_STRING: u32 = 3758096927;
pub const ERROR_INVALID_REG_PROPERTY: u32 = 3758096905;
pub const ERROR_INVALID_TARGET: u32 = 3758096947;
pub const ERROR_IN_WOW64: u32 = 3758096949;
pub const ERROR_KEY_DOES_NOT_EXIST: u32 = 3758096900;
pub const ERROR_LINE_NOT_FOUND: u32 = 3758096642;
pub const ERROR_MACHINE_UNAVAILABLE: u32 = 3758096930;
pub const ERROR_NON_WINDOWS_DRIVER: u32 = 3758096942;
pub const ERROR_NON_WINDOWS_NT_DRIVER: u32 = 3758096941;
pub const ERROR_NOT_AN_INSTALLED_OEM_INF: u32 = 3758096956;
pub const ERROR_NOT_DISABLEABLE: u32 = 3758096945;
pub const ERROR_NOT_INSTALLED: u32 = 3758100480;
pub const ERROR_NO_ASSOCIATED_CLASS: u32 = 3758096896;
pub const ERROR_NO_ASSOCIATED_SERVICE: u32 = 3758096921;
pub const ERROR_NO_AUTHENTICODE_CATALOG: u32 = 3758096959;
pub const ERROR_NO_BACKUP: u32 = 3758096643;
pub const ERROR_NO_CATALOG_FOR_OEM_INF: u32 = 3758096943;
pub const ERROR_NO_CLASSINSTALL_PARAMS: u32 = 3758096917;
pub const ERROR_NO_CLASS_DRIVER_LIST: u32 = 3758096920;
pub const ERROR_NO_COMPAT_DRIVERS: u32 = 3758096936;
pub const ERROR_NO_CONFIGMGR_SERVICES: u32 = 3758096931;
pub const ERROR_NO_DEFAULT_DEVICE_INTERFACE: u32 = 3758096922;
pub const ERROR_NO_DEFAULT_INTERFACE_DEVICE: u32 = 3758096922;
pub const ERROR_NO_DEVICE_ICON: u32 = 3758096937;
pub const ERROR_NO_DEVICE_SELECTED: u32 = 3758096913;
pub const ERROR_NO_DRIVER_SELECTED: u32 = 3758096899;
pub const ERROR_NO_INF: u32 = 3758096906;
pub const ERROR_NO_SUCH_DEVICE_INTERFACE: u32 = 3758096933;
pub const ERROR_NO_SUCH_DEVINST: u32 = 3758096907;
pub const ERROR_NO_SUCH_INTERFACE_CLASS: u32 = 3758096926;
pub const ERROR_NO_SUCH_INTERFACE_DEVICE: u32 = 3758096933;
pub const ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE: u32 = 3758096965;
pub const ERROR_PNP_REGISTRY_ERROR: u32 = 3758096954;
pub const ERROR_REMOTE_COMM_FAILURE: u32 = 3758096929;
pub const ERROR_REMOTE_REQUEST_UNSUPPORTED: u32 = 3758096955;
pub const ERROR_SCE_DISABLED: u32 = 3758096952;
pub const ERROR_SECTION_NAME_TOO_LONG: u32 = 3758096386;
pub const ERROR_SECTION_NOT_FOUND: u32 = 3758096641;
pub const ERROR_SET_SYSTEM_RESTORE_POINT: u32 = 3758096950;
pub const ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH: u32 = 3758096964;
pub const ERROR_UNKNOWN_EXCEPTION: u32 = 3758096953;
pub const ERROR_UNRECOVERABLE_STACK_OVERFLOW: u32 = 3758097152;
pub const ERROR_WRONG_INF_STYLE: u32 = 3758096640;
pub const ERROR_WRONG_INF_TYPE: u32 = 3758096970;
pub const EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW: u32 = 3758097152;
pub const FILEOP_ABORT: i32 = 0;
pub const FILEOP_BACKUP: i32 = 3;
pub const FILEOP_COPY: i32 = 0;
pub const FILEOP_DELETE: i32 = 2;
pub const FILEOP_DOIT: i32 = 1;
pub const FILEOP_NEWPATH: i32 = 4;
pub const FILEOP_RENAME: i32 = 1;
pub const FILEOP_RETRY: i32 = 1;
pub const FILEOP_SKIP: i32 = 2;
pub type FILEPATHS = FILEPATHS_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILEPATHS_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
pub type FILEPATHS_SIGNERINFO = FILEPATHS_SIGNERINFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCSTR,
    pub Version: windows_core::PCSTR,
    pub CatalogFile: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCSTR,
    pub Version: windows_core::PCSTR,
    pub CatalogFile: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCWSTR,
    pub Version: windows_core::PCWSTR,
    pub CatalogFile: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCWSTR,
    pub Version: windows_core::PCWSTR,
    pub CatalogFile: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILEPATHS_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
pub const FILE_COMPRESSION_MSZIP: i32 = 2;
pub const FILE_COMPRESSION_NONE: i32 = 0;
pub const FILE_COMPRESSION_NTCAB: i32 = 3;
pub const FILE_COMPRESSION_WINLZA: i32 = 1;
pub type FILE_IN_CABINET_INFO = FILE_IN_CABINET_INFO_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_core::PCSTR,
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
    pub NameInCabinet: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: windows_core::PCWSTR,
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
pub const FLG_ADDPROPERTY_AND: i32 = 16;
pub const FLG_ADDPROPERTY_APPEND: i32 = 4;
pub const FLG_ADDPROPERTY_NOCLOBBER: i32 = 1;
pub const FLG_ADDPROPERTY_OR: i32 = 8;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: i32 = 2;
pub const FLG_ADDREG_32BITKEY: i32 = 16384;
pub const FLG_ADDREG_64BITKEY: i32 = 4096;
pub const FLG_ADDREG_APPEND: i32 = 8;
pub const FLG_ADDREG_BINVALUETYPE: i32 = 1;
pub const FLG_ADDREG_DELREG_BIT: i32 = 32768;
pub const FLG_ADDREG_DELVAL: i32 = 4;
pub const FLG_ADDREG_KEYONLY: i32 = 16;
pub const FLG_ADDREG_KEYONLY_COMMON: i32 = 8192;
pub const FLG_ADDREG_NOCLOBBER: i32 = 2;
pub const FLG_ADDREG_OVERWRITEONLY: i32 = 32;
pub const FLG_ADDREG_TYPE_BINARY: i32 = 1;
pub const FLG_ADDREG_TYPE_DWORD: i32 = 65537;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: i32 = 131072;
pub const FLG_ADDREG_TYPE_MASK: u32 = 4294901761;
pub const FLG_ADDREG_TYPE_MULTI_SZ: i32 = 65536;
pub const FLG_ADDREG_TYPE_NONE: i32 = 131073;
pub const FLG_ADDREG_TYPE_QWORD: i32 = 720897;
pub const FLG_ADDREG_TYPE_SZ: i32 = 0;
pub const FLG_BITREG_32BITKEY: i32 = 16384;
pub const FLG_BITREG_64BITKEY: i32 = 4096;
pub const FLG_BITREG_CLEARBITS: i32 = 0;
pub const FLG_BITREG_SETBITS: i32 = 1;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: i32 = 1;
pub const FLG_DELREG_32BITKEY: i32 = 16384;
pub const FLG_DELREG_64BITKEY: i32 = 4096;
pub const FLG_DELREG_KEYONLY_COMMON: i32 = 8192;
pub const FLG_DELREG_MULTI_SZ_DELSTRING: i32 = 98306;
pub const FLG_DELREG_OPERATION_MASK: i32 = 254;
pub const FLG_DELREG_TYPE_BINARY: i32 = 1;
pub const FLG_DELREG_TYPE_DWORD: i32 = 65537;
pub const FLG_DELREG_TYPE_EXPAND_SZ: i32 = 131072;
pub const FLG_DELREG_TYPE_MASK: u32 = 4294901761;
pub const FLG_DELREG_TYPE_MULTI_SZ: i32 = 65536;
pub const FLG_DELREG_TYPE_NONE: i32 = 131073;
pub const FLG_DELREG_TYPE_QWORD: i32 = 720897;
pub const FLG_DELREG_TYPE_SZ: i32 = 0;
pub const FLG_DELREG_VALUE: i32 = 0;
pub const FLG_INI2REG_32BITKEY: i32 = 16384;
pub const FLG_INI2REG_64BITKEY: i32 = 4096;
pub const FLG_PROFITEM_CSIDL: i32 = 8;
pub const FLG_PROFITEM_CURRENTUSER: i32 = 1;
pub const FLG_PROFITEM_DELETE: i32 = 2;
pub const FLG_PROFITEM_GROUP: i32 = 4;
pub const FLG_REGSVR_DLLINSTALL: i32 = 2;
pub const FLG_REGSVR_DLLREGISTER: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDEVINFO(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDSKSPC(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HINF(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HSPFILELOG(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HSPFILEQ(pub *mut core::ffi::c_void);
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: i32 = 10010;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: i32 = 10004;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: i32 = 10003;
pub const IDD_DYNAWIZ_FIRSTPAGE: i32 = 10000;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: i32 = 10011;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: i32 = 10007;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: i32 = 10008;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: i32 = 10006;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: i32 = 10012;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: i32 = 10009;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: i32 = 10002;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: i32 = 10001;
pub const IDF_CHECKFIRST: i32 = 256;
pub const IDF_NOBEEP: i32 = 512;
pub const IDF_NOBROWSE: i32 = 1;
pub const IDF_NOCOMPRESSED: i32 = 8;
pub const IDF_NODETAILS: i32 = 4;
pub const IDF_NOFOREGROUND: i32 = 1024;
pub const IDF_NOREMOVABLEMEDIAPROMPT: i32 = 4096;
pub const IDF_NOSKIP: i32 = 2;
pub const IDF_OEMDISK: u32 = 2147483648;
pub const IDF_USEDISKNAMEASPROMPT: i32 = 8192;
pub const IDF_WARNIFSKIP: i32 = 2048;
pub const IDI_CLASSICON_OVERLAYFIRST: i32 = 500;
pub const IDI_CLASSICON_OVERLAYLAST: i32 = 502;
pub const IDI_CONFLICT: i32 = 161;
pub const IDI_DISABLED_OVL: i32 = 501;
pub const IDI_FORCED_OVL: i32 = 502;
pub const IDI_PROBLEM_OVL: i32 = 500;
pub const IDI_RESOURCE: i32 = 159;
pub const IDI_RESOURCEFIRST: i32 = 159;
pub const IDI_RESOURCELAST: i32 = 161;
pub const IDI_RESOURCEOVERLAYFIRST: i32 = 161;
pub const IDI_RESOURCEOVERLAYLAST: i32 = 161;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
pub const INFINFO_DEFAULT_SEARCH: i32 = 3;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: i32 = 2;
pub const INFINFO_INF_PATH_LIST_SEARCH: i32 = 5;
pub const INFINFO_INF_SPEC_IS_HINF: i32 = 1;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: i32 = 4;
pub const INF_STYLE_CACHE_DISABLE: i32 = 32;
pub const INF_STYLE_CACHE_ENABLE: i32 = 16;
pub const INF_STYLE_CACHE_IGNORE: i32 = 64;
pub const INF_STYLE_NONE: i32 = 0;
pub const INF_STYLE_OLDNT: i32 = 1;
pub const INF_STYLE_WIN4: i32 = 2;
pub const LINE_LEN: i32 = 256;
pub const LogSevError: i32 = 2;
pub const LogSevFatalError: i32 = 3;
pub const LogSevInformation: i32 = 0;
pub const LogSevMaximum: i32 = 4;
pub const LogSevWarning: i32 = 1;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: i32 = 11000;
pub const MAX_INF_SECTION_NAME_LENGTH: i32 = 255;
pub const MAX_INF_STRING_LENGTH: i32 = 4096;
pub const MAX_INSTALLWIZARD_DYNAPAGES: i32 = 20;
pub const MAX_INSTRUCTION_LEN: i32 = 256;
pub const MAX_LABEL_LEN: i32 = 30;
pub const MAX_SERVICE_NAME_LEN: i32 = 256;
pub const MAX_SUBTITLE_LEN: i32 = 256;
pub const MAX_TITLE_LEN: i32 = 60;
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: i32 = 10000;
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: i32 = 32768;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: i32 = 1;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: i32 = 1024;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: i32 = 2;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: i32 = 8192;
pub const NDW_INSTALLFLAG_KNOWNCLASS: i32 = 524288;
pub const NDW_INSTALLFLAG_NEEDREBOOT: i32 = 256;
pub const NDW_INSTALLFLAG_NEEDRESTART: i32 = 128;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: i32 = 512;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: i32 = 4096;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: i32 = 131072;
pub const NDW_INSTALLFLAG_PCMCIAMODE: i32 = 65536;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: i32 = 16384;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: i32 = 2048;
pub const NDW_INSTALLFLAG_USERCANCEL: i32 = 262144;
pub type PCABINET_INFO = PCABINET_INFO_A;
pub type PCABINET_INFO_A = *mut CABINET_INFO_A;
pub type PCABINET_INFO_W = *mut CABINET_INFO_W;
pub type PCOINSTALLER_CONTEXT_DATA = *mut COINSTALLER_CONTEXT_DATA;
pub type PDETECT_PROGRESS_NOTIFY = Option<unsafe extern "system" fn(progressnotifyparam: *const core::ffi::c_void, detectcomplete: u32) -> windows_core::BOOL>;
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
pub const SCWMI_CLOBBER_SECURITY: i32 = 1;
pub const SETDIRID_NOT_FULL_PATH: i32 = 1;
pub const SIGNERSCORE_AUTHENTICODE: i32 = 251658240;
pub const SIGNERSCORE_INBOX: i32 = 218103811;
pub const SIGNERSCORE_LOGO_PREMIUM: i32 = 218103809;
pub const SIGNERSCORE_LOGO_STANDARD: i32 = 218103810;
pub const SIGNERSCORE_MASK: u32 = 4278190080;
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840;
pub const SIGNERSCORE_UNCLASSIFIED: i32 = 218103812;
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080;
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648;
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472;
pub const SIGNERSCORE_WHQL: i32 = 218103813;
pub type SOURCE_MEDIA = SOURCE_MEDIA_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_core::PCSTR,
    pub Tagfile: windows_core::PCSTR,
    pub Description: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFile: windows_core::PCSTR,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_core::PCSTR,
    pub Tagfile: windows_core::PCSTR,
    pub Description: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFile: windows_core::PCSTR,
    pub Flags: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_core::PCWSTR,
    pub Tagfile: windows_core::PCWSTR,
    pub Description: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFile: windows_core::PCWSTR,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_core::PCWSTR,
    pub Tagfile: windows_core::PCWSTR,
    pub Description: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFile: windows_core::PCWSTR,
    pub Flags: u32,
}
pub const SPCRP_CHARACTERISTICS: i32 = 27;
pub const SPCRP_DEVTYPE: i32 = 25;
pub const SPCRP_EXCLUSIVE: i32 = 26;
pub const SPCRP_LOWERFILTERS: i32 = 18;
pub const SPCRP_MAXIMUM_PROPERTY: i32 = 28;
pub const SPCRP_SECURITY: i32 = 23;
pub const SPCRP_SECURITY_SDS: i32 = 24;
pub const SPCRP_UPPERFILTERS: i32 = 17;
pub const SPDIT_CLASSDRIVER: i32 = 1;
pub const SPDIT_COMPATDRIVER: i32 = 2;
pub const SPDIT_NODRIVER: i32 = 0;
pub const SPDRP_ADDRESS: i32 = 28;
pub const SPDRP_BASE_CONTAINERID: i32 = 36;
pub const SPDRP_BUSNUMBER: i32 = 21;
pub const SPDRP_BUSTYPEGUID: i32 = 19;
pub const SPDRP_CAPABILITIES: i32 = 15;
pub const SPDRP_CHARACTERISTICS: i32 = 27;
pub const SPDRP_CLASS: i32 = 7;
pub const SPDRP_CLASSGUID: i32 = 8;
pub const SPDRP_COMPATIBLEIDS: i32 = 2;
pub const SPDRP_CONFIGFLAGS: i32 = 10;
pub const SPDRP_DEVICEDESC: i32 = 0;
pub const SPDRP_DEVICE_POWER_DATA: i32 = 30;
pub const SPDRP_DEVTYPE: i32 = 25;
pub const SPDRP_DRIVER: i32 = 9;
pub const SPDRP_ENUMERATOR_NAME: i32 = 22;
pub const SPDRP_EXCLUSIVE: i32 = 26;
pub const SPDRP_FRIENDLYNAME: i32 = 12;
pub const SPDRP_HARDWAREID: i32 = 1;
pub const SPDRP_INSTALL_STATE: i32 = 34;
pub const SPDRP_LEGACYBUSTYPE: i32 = 20;
pub const SPDRP_LOCATION_INFORMATION: i32 = 13;
pub const SPDRP_LOCATION_PATHS: i32 = 35;
pub const SPDRP_LOWERFILTERS: i32 = 18;
pub const SPDRP_MAXIMUM_PROPERTY: i32 = 37;
pub const SPDRP_MFG: i32 = 11;
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: i32 = 14;
pub const SPDRP_REMOVAL_POLICY: i32 = 31;
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: i32 = 32;
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: i32 = 33;
pub const SPDRP_SECURITY: i32 = 23;
pub const SPDRP_SECURITY_SDS: i32 = 24;
pub const SPDRP_SERVICE: i32 = 4;
pub const SPDRP_UI_NUMBER: i32 = 16;
pub const SPDRP_UI_NUMBER_DESC_FORMAT: i32 = 29;
pub const SPDRP_UNUSED0: i32 = 3;
pub const SPDRP_UNUSED1: i32 = 5;
pub const SPDRP_UNUSED2: i32 = 6;
pub const SPDRP_UPPERFILTERS: i32 = 17;
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: i32 = 2;
pub const SPDSL_IGNORE_DISK: i32 = 1;
pub const SPFILELOG_FORCENEW: i32 = 2;
pub const SPFILELOG_OEMFILE: i32 = 1;
pub const SPFILELOG_QUERYONLY: i32 = 4;
pub const SPFILELOG_SYSTEMLOG: i32 = 1;
pub const SPFILENOTIFY_BACKUPERROR: i32 = 22;
pub const SPFILENOTIFY_CABINETINFO: i32 = 16;
pub const SPFILENOTIFY_COPYERROR: i32 = 13;
pub const SPFILENOTIFY_DELETEERROR: i32 = 7;
pub const SPFILENOTIFY_ENDBACKUP: i32 = 23;
pub const SPFILENOTIFY_ENDCOPY: i32 = 12;
pub const SPFILENOTIFY_ENDDELETE: i32 = 6;
pub const SPFILENOTIFY_ENDQUEUE: i32 = 2;
pub const SPFILENOTIFY_ENDREGISTRATION: i32 = 32;
pub const SPFILENOTIFY_ENDRENAME: i32 = 9;
pub const SPFILENOTIFY_ENDSUBQUEUE: i32 = 4;
pub const SPFILENOTIFY_FILEEXTRACTED: i32 = 19;
pub const SPFILENOTIFY_FILEINCABINET: i32 = 17;
pub const SPFILENOTIFY_FILEOPDELAYED: i32 = 20;
pub const SPFILENOTIFY_LANGMISMATCH: i32 = 65536;
pub const SPFILENOTIFY_NEEDMEDIA: i32 = 14;
pub const SPFILENOTIFY_NEEDNEWCABINET: i32 = 18;
pub const SPFILENOTIFY_QUEUESCAN: i32 = 15;
pub const SPFILENOTIFY_QUEUESCAN_EX: i32 = 24;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: i32 = 64;
pub const SPFILENOTIFY_RENAMEERROR: i32 = 10;
pub const SPFILENOTIFY_STARTBACKUP: i32 = 21;
pub const SPFILENOTIFY_STARTCOPY: i32 = 11;
pub const SPFILENOTIFY_STARTDELETE: i32 = 5;
pub const SPFILENOTIFY_STARTQUEUE: i32 = 1;
pub const SPFILENOTIFY_STARTREGISTRATION: i32 = 25;
pub const SPFILENOTIFY_STARTRENAME: i32 = 8;
pub const SPFILENOTIFY_STARTSUBQUEUE: i32 = 3;
pub const SPFILENOTIFY_TARGETEXISTS: i32 = 131072;
pub const SPFILENOTIFY_TARGETNEWER: i32 = 262144;
pub const SPFILEQ_FILE_IN_USE: i32 = 1;
pub const SPFILEQ_REBOOT_IN_PROGRESS: i32 = 4;
pub const SPFILEQ_REBOOT_RECOMMENDED: i32 = 2;
pub const SPID_ACTIVE: i32 = 1;
pub const SPID_DEFAULT: i32 = 2;
pub const SPID_REMOVED: i32 = 4;
pub const SPINST_ALL: i32 = 2047;
pub const SPINST_BITREG: i32 = 32;
pub const SPINST_COPYINF: i32 = 512;
pub const SPINST_DEVICEINSTALL: i32 = 1048576;
pub const SPINST_FILES: i32 = 16;
pub const SPINST_INI2REG: i32 = 8;
pub const SPINST_INIFILES: i32 = 2;
pub const SPINST_LOGCONFIG: i32 = 1;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: i32 = 262144;
pub const SPINST_LOGCONFIG_IS_FORCED: i32 = 131072;
pub const SPINST_PROFILEITEMS: i32 = 256;
pub const SPINST_PROPERTIES: i32 = 1024;
pub const SPINST_REGISTERCALLBACKAWARE: i32 = 524288;
pub const SPINST_REGISTRY: i32 = 4;
pub const SPINST_REGSVR: i32 = 64;
pub const SPINST_SINGLESECTION: i32 = 65536;
pub const SPINST_UNREGSVR: i32 = 128;
pub const SPINT_ACTIVE: i32 = 1;
pub const SPINT_DEFAULT: i32 = 2;
pub const SPINT_REMOVED: i32 = 4;
pub const SPOST_MAX: i32 = 3;
pub const SPOST_NONE: i32 = 0;
pub const SPOST_PATH: i32 = 1;
pub const SPOST_URL: i32 = 2;
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: i32 = 3;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: i32 = 2;
pub const SPPSR_SELECT_DEVICE_RESOURCES: i32 = 1;
pub const SPQ_DELAYED_COPY: i32 = 1;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: i32 = 2;
pub const SPQ_FLAG_BACKUP_AWARE: i32 = 1;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: i32 = 8;
pub const SPQ_FLAG_FILES_MODIFIED: i32 = 4;
pub const SPQ_FLAG_VALID: i32 = 15;
pub const SPQ_SCAN_ACTIVATE_DRP: i32 = 1024;
pub const SPQ_SCAN_FILE_COMPARISON: i32 = 512;
pub const SPQ_SCAN_FILE_PRESENCE: i32 = 1;
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: i32 = 256;
pub const SPQ_SCAN_FILE_VALIDITY: i32 = 2;
pub const SPQ_SCAN_INFORM_USER: i32 = 16;
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: i32 = 32;
pub const SPQ_SCAN_PRUNE_DELREN: i32 = 128;
pub const SPQ_SCAN_USE_CALLBACK: i32 = 4;
pub const SPQ_SCAN_USE_CALLBACKEX: i32 = 8;
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: i32 = 64;
pub const SPQ_SCAN_USE_OEM_CATALOGS: i32 = 2048;
pub const SPRDI_FIND_DUPS: i32 = 1;
pub const SPREG_DLLINSTALL: i32 = 4;
pub const SPREG_GETPROCADDR: i32 = 2;
pub const SPREG_LOADLIBRARY: i32 = 1;
pub const SPREG_REGSVR: i32 = 3;
pub const SPREG_SUCCESS: i32 = 0;
pub const SPREG_TIMEOUT: i32 = 5;
pub const SPREG_UNKNOWN: u32 = 4294967295;
pub const SPSVCINST_ASSOCSERVICE: i32 = 2;
pub const SPSVCINST_CLOBBER_SECURITY: i32 = 1024;
pub const SPSVCINST_DELETEEVENTLOGENTRY: i32 = 4;
pub const SPSVCINST_NOCLOBBER_BOOTFLAGS: i32 = 262144;
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: i32 = 32768;
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: i32 = 128;
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: i32 = 256;
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: i32 = 8;
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: i32 = 32;
pub const SPSVCINST_NOCLOBBER_FAILUREACTIONS: i32 = 131072;
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: i32 = 64;
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: i32 = 4096;
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: i32 = 16384;
pub const SPSVCINST_NOCLOBBER_STARTTYPE: i32 = 16;
pub const SPSVCINST_NOCLOBBER_TRIGGERS: i32 = 8192;
pub const SPSVCINST_STARTSERVICE: i32 = 2048;
pub const SPSVCINST_STOPSERVICE: i32 = 512;
pub const SPSVCINST_TAGTOFRONT: i32 = 1;
pub const SPSVCINST_UNIQUE_NAME: i32 = 65536;
pub const SPWPT_SELECTDEVICE: i32 = 1;
pub const SPWP_USE_DEVINFO_DATA: i32 = 1;
#[cfg(all(feature = "prsht", feature = "windef"))]
pub type SP_ADDPROPERTYPAGE_DATA = SP_NEWDEVICEWIZARD_DATA;
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: i32 = 2;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: i32 = 1;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const SP_BACKUP_BACKUPPASS: i32 = 1;
pub const SP_BACKUP_BOOTFILE: i32 = 8;
pub const SP_BACKUP_DEMANDPASS: i32 = 2;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const SP_BACKUP_SPECIAL: i32 = 4;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "commctrl")]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::HIMAGELIST,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "commctrl")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::HIMAGELIST,
    pub Reserved: usize,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
pub const SP_COPY_ALREADYDECOMP: i32 = 4194304;
pub const SP_COPY_DELETESOURCE: i32 = 1;
pub const SP_COPY_FORCE_IN_USE: i32 = 512;
pub const SP_COPY_FORCE_NEWER: i32 = 8192;
pub const SP_COPY_FORCE_NOOVERWRITE: i32 = 4096;
pub const SP_COPY_HARDLINK: i32 = 268435456;
pub const SP_COPY_INBOX_INF: i32 = 134217728;
pub const SP_COPY_IN_USE_NEEDS_REBOOT: i32 = 256;
pub const SP_COPY_IN_USE_TRY_RENAME: i32 = 67108864;
pub const SP_COPY_LANGUAGEAWARE: i32 = 32;
pub const SP_COPY_NEWER: i32 = 4;
pub const SP_COPY_NEWER_ONLY: i32 = 65536;
pub const SP_COPY_NEWER_OR_SAME: i32 = 4;
pub const SP_COPY_NOBROWSE: i32 = 32768;
pub const SP_COPY_NODECOMP: i32 = 16;
pub const SP_COPY_NOOVERWRITE: i32 = 8;
pub const SP_COPY_NOPRUNE: i32 = 1048576;
pub const SP_COPY_NOSKIP: i32 = 1024;
pub const SP_COPY_OEMINF_CATALOG_ONLY: i32 = 262144;
pub const SP_COPY_OEM_F6_INF: i32 = 2097152;
pub const SP_COPY_PNPLOCKED: i32 = 33554432;
pub const SP_COPY_REPLACEONLY: i32 = 2;
pub const SP_COPY_REPLACE_BOOT_FILE: i32 = 524288;
pub const SP_COPY_RESERVED: i32 = 131072;
pub const SP_COPY_SOURCEPATH_ABSOLUTE: i32 = 128;
pub const SP_COPY_SOURCE_ABSOLUTE: i32 = 64;
pub const SP_COPY_WARNIFSKIP: i32 = 16384;
pub const SP_COPY_WINDOWS_SIGNED: i32 = 16777216;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub ClassGuid: windows_core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
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
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::HANDLE,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::HANDLE,
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
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::HANDLE,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::HANDLE,
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
    pub hwndParent: super::HWND,
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
#[derive(Clone, Copy, Debug)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::HWND,
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
    pub hwndParent: super::HWND,
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
#[derive(Clone, Copy, Debug)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::HWND,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub DriverDate: super::FILETIME,
    pub DriverVersion: super::DWORDLONG,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
    pub DriverDate: super::FILETIME,
    pub DriverVersion: super::DWORDLONG,
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
    pub DriverDate: super::FILETIME,
    pub DriverVersion: super::DWORDLONG,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::FILETIME,
    pub DriverVersion: super::DWORDLONG,
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
    pub InfDate: super::FILETIME,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::FILETIME,
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
    pub InfDate: super::FILETIME,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::FILETIME,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub ClassGuid: windows_core::GUID,
    pub EnableMessage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: windows_core::GUID,
    pub EnableMessage: u32,
}
pub type SP_FILE_COPY_PARAMS = SP_FILE_COPY_PARAMS_A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFilename: windows_core::PCSTR,
    pub SourceDescription: windows_core::PCSTR,
    pub SourceTagfile: windows_core::PCSTR,
    pub TargetDirectory: windows_core::PCSTR,
    pub TargetFilename: windows_core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFilename: windows_core::PCSTR,
    pub SourceDescription: windows_core::PCSTR,
    pub SourceTagfile: windows_core::PCSTR,
    pub TargetDirectory: windows_core::PCSTR,
    pub TargetFilename: windows_core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFilename: windows_core::PCWSTR,
    pub SourceDescription: windows_core::PCWSTR,
    pub SourceTagfile: windows_core::PCWSTR,
    pub TargetDirectory: windows_core::PCWSTR,
    pub TargetFilename: windows_core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFilename: windows_core::PCWSTR,
    pub SourceDescription: windows_core::PCWSTR,
    pub SourceTagfile: windows_core::PCWSTR,
    pub TargetDirectory: windows_core::PCWSTR,
    pub TargetFilename: windows_core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: windows_core::PCWSTR,
}
pub const SP_FLAG_CABINETCONTINUATION: i32 = 2048;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub DynamicPages: [super::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::LPARAM,
    pub hwndWizardDlg: super::HWND,
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
    pub DynamicPages: [super::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::LPARAM,
    pub hwndWizardDlg: super::HWND,
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
pub const SP_MAX_MACHINENAME_LENGTH: i32 = 263;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "prsht", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::HWND,
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
    pub DynamicPages: [super::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::HWND,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Default)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: PSP_DEVINFO_DATA,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: PSP_DEVINFO_DATA,
}
pub type SP_REGISTER_CONTROL_STATUS = SP_REGISTER_CONTROL_STATUSA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
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
pub const SRCINFO_DESCRIPTION: i32 = 3;
pub const SRCINFO_FLAGS: i32 = 4;
pub const SRCINFO_PATH: i32 = 1;
pub const SRCINFO_TAGFILE: i32 = 2;
pub const SRCINFO_TAGFILE2: i32 = 5;
pub const SRCLIST_APPEND: i32 = 512;
pub const SRCLIST_NOBROWSE: i32 = 2;
pub const SRCLIST_NOSTRIPPLATFORM: i32 = 1024;
pub const SRCLIST_SUBDIRS: i32 = 256;
pub const SRCLIST_SYSIFADMIN: i32 = 64;
pub const SRCLIST_SYSTEM: i32 = 16;
pub const SRCLIST_TEMPORARY: i32 = 1;
pub const SRCLIST_USER: i32 = 32;
pub const SRC_FLAGS_CABFILE: i32 = 16;
pub const SUOI_FORCEDELETE: i32 = 1;
pub const SUOI_INTERNAL1: i32 = 2;
pub const SetupFileLogChecksum: SetupFileLogInfo = 1;
pub const SetupFileLogDiskDescription: SetupFileLogInfo = 3;
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = 2;
pub type SetupFileLogInfo = i32;
pub const SetupFileLogMax: SetupFileLogInfo = 5;
pub const SetupFileLogOtherInfo: SetupFileLogInfo = 4;
pub const SetupFileLogSourceFilename: SetupFileLogInfo = 0;
