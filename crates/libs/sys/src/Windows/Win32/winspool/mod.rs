#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AbortPrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddFormA(hprinter : super::winnt::HANDLE, level : u32, pform : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddFormW(hprinter : super::winnt::HANDLE, level : u32, pform : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddJobA(hprinter : super::winnt::HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddJobW(hprinter : super::winnt::HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddMonitorA(pname : windows_sys::core::PCSTR, level : u32, pmonitors : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddMonitorW(pname : windows_sys::core::PCWSTR, level : u32, pmonitors : *const u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn AddPortA(pname : windows_sys::core::PCSTR, hwnd : super::windef::HWND, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn AddPortW(pname : windows_sys::core::PCWSTR, hwnd : super::windef::HWND, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, ppathname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, ppathname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorA(pname : windows_sys::core::PCSTR, level : u32, pprovidorinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorW(pname : windows_sys::core::PCWSTR, level : u32, pprovidorinfo : *const u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddPrinterA(pname : windows_sys::core::PCSTR, level : u32, pprinter : *mut u8) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2A(hwnd : super::windef::HWND, pszname : windows_sys::core::PCSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2W(hwnd : super::windef::HWND, pszname : windows_sys::core::PCWSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverA(pname : windows_sys::core::PCSTR, level : u32, pdriverinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExA(pname : windows_sys::core::PCSTR, level : u32, lpbdriverinfo : *mut u8, dwfilecopyflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExW(pname : windows_sys::core::PCWSTR, level : u32, lpbdriverinfo : *mut u8, dwfilecopyflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverW(pname : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *const u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn AddPrinterW(pname : windows_sys::core::PCWSTR, level : u32, pprinter : *mut u8) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesA(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : *mut super::wingdi::DEVMODEA, pdevmodeinput : *const super::wingdi::DEVMODEA) -> i32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesW(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : *mut super::wingdi::DEVMODEW, pdevmodeinput : *const super::wingdi::DEVMODEW) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn ClosePrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn CloseSpoolFileHandle(hprinter : super::winnt::HANDLE, hspoolfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn CommitSpoolData(hprinter : super::winnt::HANDLE, hspoolfile : super::winnt::HANDLE, cbcommit : u32) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn ConfigurePortA(pname : windows_sys::core::PCSTR, hwnd : super::windef::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn ConfigurePortW(pname : windows_sys::core::PCWSTR, hwnd : super::windef::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn ConnectToPrinterDlg(hwnd : super::windef::HWND, flags : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::minwindef::FILETIME, dwldriverversion : super::winnt::DWORDLONG, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::minwindef::FILETIME, dwldriverversion : super::winnt::DWORDLONG, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeleteFormA(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeleteFormW(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeleteJobNamedProperty(hprinter : super::winnt::HANDLE, jobid : u32, pszname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn DeletePortA(pname : windows_sys::core::PCSTR, hwnd : super::windef::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn DeletePortW(pname : windows_sys::core::PCWSTR, hwnd : super::windef::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprovidorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprovidorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataA(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataW(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR) -> u32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesA(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : *mut super::wingdi::DEVMODEA, pdevmodeinput : *const super::wingdi::DEVMODEA, fmode : u32) -> i32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesW(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : *mut super::wingdi::DEVMODEW, pdevmodeinput : *const super::wingdi::DEVMODEW, fmode : u32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EndDocPrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EndPagePrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumFormsA(hprinter : super::winnt::HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumFormsW(hprinter : super::winnt::HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumJobNamedProperties(hprinter : super::winnt::HANDLE, jobid : u32, pcproperties : *mut u32, ppproperties : *mut *mut PrintNamedProperty) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumJobsA(hprinter : super::winnt::HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumJobsW(hprinter : super::winnt::HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumMonitorsA(pname : windows_sys::core::PCSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumMonitorsW(pname : windows_sys::core::PCWSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPortsA(pname : windows_sys::core::PCSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPortsW(pname : windows_sys::core::PCWSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesA(pname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesW(pname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataA(hprinter : super::winnt::HANDLE, dwindex : u32, pvaluename : windows_sys::core::PSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataW(hprinter : super::winnt::HANDLE, dwindex : u32, pvaluename : windows_sys::core::PWSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR, psubkey : windows_sys::core::PSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR, psubkey : windows_sys::core::PWSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrintersA(flags : u32, name : windows_sys::core::PCSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintersW(flags : u32, name : windows_sys::core::PCWSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "C" fn ExtDeviceMode(hwnd : super::windef::HWND, hinst : super::winnt::HANDLE, pdevmodeoutput : *mut super::wingdi::DEVMODEA, pdevicename : windows_sys::core::PCSTR, pport : windows_sys::core::PCSTR, pdevmodeinput : *const super::wingdi::DEVMODEA, pprofile : windows_sys::core::PCSTR, fmode : u32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn FindClosePrinterChangeNotification(hchange : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn FindFirstPrinterChangeNotification(hprinter : super::winnt::HANDLE, fdwfilter : u32, fdwoptions : u32, pprinternotifyoptions : *const core::ffi::c_void) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn FindNextPrinterChangeNotification(hchange : super::winnt::HANDLE, pdwchange : *mut u32, pvreserved : *const core::ffi::c_void, ppprinternotifyinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn FlushPrinter(hprinter : super::winnt::HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32, csleep : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn FreePrintNamedPropertyArray(cproperties : u32, ppproperties : *mut *mut PrintNamedProperty));
windows_link::link!("winspool.drv" "system" fn FreePrintPropertyValue(pvalue : *mut PrintPropertyValue));
windows_link::link!("winspool.drv" "system" fn FreePrinterNotifyInfo(pprinternotifyinfo : *const PRINTER_NOTIFY_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszzcoredriverdependencies : windows_sys::core::PCSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERA) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszzcoredriverdependencies : windows_sys::core::PCWSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERW) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterA(pszbuffer : windows_sys::core::PSTR, pcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterW(pszbuffer : windows_sys::core::PWSTR, pcchbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetFormA(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetFormW(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCWSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetJobA(hprinter : super::winnt::HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetJobNamedPropertyValue(hprinter : super::winnt::HANDLE, jobid : u32, pszname : windows_sys::core::PCWSTR, pvalue : *mut PrintPropertyValue) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetJobW(hprinter : super::winnt::HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrintExecutionData(pdata : *mut PRINT_EXECUTION_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrintOutputInfo(hwnd : super::windef::HWND, pszprinter : windows_sys::core::PCWSTR, phfile : *mut super::winnt::HANDLE, ppszoutputfile : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterA(hprinter : super::winnt::HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataA(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataW(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2A(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2W(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverA(hprinter : super::winnt::HANDLE, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszlanguage : windows_sys::core::PCSTR, pszpackageid : windows_sys::core::PCSTR, pszdriverpackagecab : windows_sys::core::PSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszlanguage : windows_sys::core::PCWSTR, pszpackageid : windows_sys::core::PCWSTR, pszdriverpackagecab : windows_sys::core::PWSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverW(hprinter : super::winnt::HANDLE, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetPrinterW(hprinter : super::winnt::HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn GetSpoolFileHandle(hprinter : super::winnt::HANDLE) -> super::winnt::HANDLE);
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszdrivername : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszdrivername : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeA(pdevmode : *const super::wingdi::DEVMODEA, devmodesize : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeW(pdevmode : *const super::wingdi::DEVMODEW, devmodesize : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2A(pprintername : windows_sys::core::PCSTR, phprinter : *mut super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSA, poptions : *const PRINTER_OPTIONSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2W(pprintername : windows_sys::core::PCWSTR, phprinter : *mut super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSW, poptions : *const PRINTER_OPTIONSW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinterA(pprintername : windows_sys::core::PCSTR, phprinter : *mut super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinterW(pprintername : windows_sys::core::PCWSTR, phprinter : *mut super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxA(hprinter : super::winnt::HANDLE, error : u32, hwnd : super::windef::HWND, ptext : windows_sys::core::PCSTR, pcaption : windows_sys::core::PCSTR, dwtype : u32) -> u32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxW(hprinter : super::winnt::HANDLE, error : u32, hwnd : super::windef::HWND, ptext : windows_sys::core::PCWSTR, pcaption : windows_sys::core::PCWSTR, dwtype : u32) -> u32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterProperties(hwnd : super::windef::HWND, hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn ReadPrinter(hprinter : super::winnt::HANDLE, pbuf : *mut core::ffi::c_void, cbbuf : u32, pnobytesread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn ReportJobProcessingProgress(printerhandle : super::winnt::HANDLE, jobid : u32, joboperation : EPrintXPSJobOperation, jobprogress : EPrintXPSJobProgress) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn ResetPrinterA(hprinter : super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
windows_link::link!("winspool.drv" "system" fn ResetPrinterW(hprinter : super::winnt::HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn ScheduleJob(hprinter : super::winnt::HANDLE, jobid : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterA(pszprinter : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterW(pszprinter : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetFormA(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCSTR, level : u32, pform : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetFormW(hprinter : super::winnt::HANDLE, pformname : windows_sys::core::PCWSTR, level : u32, pform : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetJobA(hprinter : super::winnt::HANDLE, jobid : u32, level : u32, pjob : *mut u8, command : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetJobNamedProperty(hprinter : super::winnt::HANDLE, jobid : u32, pproperty : *const PrintNamedProperty) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetJobW(hprinter : super::winnt::HANDLE, jobid : u32, level : u32, pjob : *mut u8, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPortA(pname : windows_sys::core::PCSTR, pportname : windows_sys::core::PCSTR, dwlevel : u32, pportinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPortW(pname : windows_sys::core::PCWSTR, pportname : windows_sys::core::PCWSTR, dwlevel : u32, pportinfo : *const u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterA(hprinter : super::winnt::HANDLE, level : u32, pprinter : *mut u8, command : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataA(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExA(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExW(hprinter : super::winnt::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataW(hprinter : super::winnt::HANDLE, pvaluename : windows_sys::core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn SetPrinterW(hprinter : super::winnt::HANDLE, level : u32, pprinter : *mut u8, command : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn StartDocPrinterA(hprinter : super::winnt::HANDLE, level : u32, pdocinfo : *mut u8) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn StartDocPrinterW(hprinter : super::winnt::HANDLE, level : u32, pdocinfo : *mut u8) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn StartPagePrinter(hprinter : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : u32, hwnd : super::windef::HWND, pszdestinfpath : windows_sys::core::PSTR, pcchdestinfpath : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : u32, hwnd : super::windef::HWND, pszdestinfpath : windows_sys::core::PWSTR, pcchdestinfpath : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn WaitForPrinterChange(hprinter : super::winnt::HANDLE, flags : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn WritePrinter(hprinter : super::winnt::HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winspool.drv" "system" fn XcvDataW(hxcv : super::winnt::HANDLE, pszdataname : windows_sys::core::PCWSTR, pinputdata : *const u8, cbinputdata : u32, poutputdata : *mut u8, cboutputdata : u32, pcboutputneeded : *mut u32, pdwstatus : *mut u32) -> windows_sys::core::BOOL);
pub type ADDJOB_INFO_1 = ADDJOB_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADDJOB_INFO_1A {
    pub Path: windows_sys::core::PSTR,
    pub JobId: u32,
}
impl Default for ADDJOB_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADDJOB_INFO_1W {
    pub Path: windows_sys::core::PWSTR,
    pub JobId: u32,
}
impl Default for ADDJOB_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APD_COPY_ALL_FILES: u32 = 4;
pub const APD_COPY_FROM_DIRECTORY: u32 = 16;
pub const APD_COPY_NEW_FILES: u32 = 8;
pub const APD_STRICT_DOWNGRADE: u32 = 2;
pub const APD_STRICT_UPGRADE: u32 = 1;
pub const BIDI_ACCESS_ADMINISTRATOR: u32 = 1;
pub const BIDI_ACCESS_USER: u32 = 2;
pub const BIDI_ACTION_ENUM_SCHEMA: windows_sys::core::PCWSTR = windows_sys::core::w!("EnumSchema");
pub const BIDI_ACTION_GET: windows_sys::core::PCWSTR = windows_sys::core::w!("Get");
pub const BIDI_ACTION_GET_ALL: windows_sys::core::PCWSTR = windows_sys::core::w!("GetAll");
pub const BIDI_ACTION_GET_WITH_ARGUMENT: windows_sys::core::PCWSTR = windows_sys::core::w!("GetWithArgument");
pub const BIDI_ACTION_SET: windows_sys::core::PCWSTR = windows_sys::core::w!("Set");
pub const BIDI_BLOB: BIDI_TYPE = 7;
pub const BIDI_BOOL: BIDI_TYPE = 3;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BIDI_DATA {
    pub dwBidiType: u32,
    pub u: BIDI_DATA_0,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union BIDI_DATA_0 {
    pub bData: windows_sys::core::BOOL,
    pub iData: i32,
    pub sData: windows_sys::core::PWSTR,
    pub fData: f32,
    pub biData: BINARY_CONTAINER,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_ENUM: BIDI_TYPE = 6;
pub const BIDI_FLOAT: BIDI_TYPE = 2;
pub const BIDI_INT: BIDI_TYPE = 1;
pub const BIDI_NULL: BIDI_TYPE = 0;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_REQUEST_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: u32,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_REQUEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_RESPONSE_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: u32,
    pub dwReqNumber: u32,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BIDI_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_STRING: BIDI_TYPE = 4;
pub const BIDI_TEXT: BIDI_TYPE = 5;
pub type BIDI_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct BINARY_CONTAINER {
    pub cbBuf: u32,
    pub pData: super::minwindef::LPBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for BINARY_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type CORE_PRINTER_DRIVER = CORE_PRINTER_DRIVERA;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub szPackageID: [i8; 260],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for CORE_PRINTER_DRIVERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub szPackageID: [u16; 260],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for CORE_PRINTER_DRIVERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DATATYPES_INFO_1 = DATATYPES_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DATATYPES_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for DATATYPES_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DATATYPES_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DATATYPES_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEF_PRIORITY: u32 = 1;
pub const DI_CHANNEL: u32 = 1;
pub const DI_MEMORYMAP_WRITE: u32 = 1;
pub const DI_READ_SPOOL_JOB: u32 = 3;
pub type DOC_INFO_1 = DOC_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_1A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
}
impl Default for DOC_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_1W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
}
impl Default for DOC_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOC_INFO_2 = DOC_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_2A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl Default for DOC_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_2W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl Default for DOC_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOC_INFO_3 = DOC_INFO_3A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_3A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub dwFlags: u32,
}
impl Default for DOC_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOC_INFO_3W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub dwFlags: u32,
}
impl Default for DOC_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DPD_DELETE_ALL_FILES: u32 = 4;
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2;
pub const DPD_DELETE_UNUSED_FILES: u32 = 1;
pub type DRIVER_INFO_1 = DRIVER_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRIVER_INFO_2 = DRIVER_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_2A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_2W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRIVER_INFO_3 = DRIVER_INFO_3A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_3A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub pHelpFile: windows_sys::core::PSTR,
    pub pDependentFiles: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDefaultDataType: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_3W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub pHelpFile: windows_sys::core::PWSTR,
    pub pDependentFiles: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDefaultDataType: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRIVER_INFO_4 = DRIVER_INFO_4A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_4A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub pHelpFile: windows_sys::core::PSTR,
    pub pDependentFiles: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDefaultDataType: windows_sys::core::PSTR,
    pub pszzPreviousNames: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_4W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub pHelpFile: windows_sys::core::PWSTR,
    pub pDependentFiles: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDefaultDataType: windows_sys::core::PWSTR,
    pub pszzPreviousNames: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRIVER_INFO_5 = DRIVER_INFO_5A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_5A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl Default for DRIVER_INFO_5A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_5W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl Default for DRIVER_INFO_5W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type DRIVER_INFO_6 = DRIVER_INFO_6A;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_6A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub pHelpFile: windows_sys::core::PSTR,
    pub pDependentFiles: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDefaultDataType: windows_sys::core::PSTR,
    pub pszzPreviousNames: windows_sys::core::PSTR,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub pszMfgName: windows_sys::core::PSTR,
    pub pszOEMUrl: windows_sys::core::PSTR,
    pub pszHardwareID: windows_sys::core::PSTR,
    pub pszProvider: windows_sys::core::PSTR,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DRIVER_INFO_6A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_6W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub pHelpFile: windows_sys::core::PWSTR,
    pub pDependentFiles: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDefaultDataType: windows_sys::core::PWSTR,
    pub pszzPreviousNames: windows_sys::core::PWSTR,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub pszMfgName: windows_sys::core::PWSTR,
    pub pszOEMUrl: windows_sys::core::PWSTR,
    pub pszHardwareID: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DRIVER_INFO_6W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type DRIVER_INFO_8 = DRIVER_INFO_8A;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_8A {
    pub cVersion: u32,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub pHelpFile: windows_sys::core::PSTR,
    pub pDependentFiles: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDefaultDataType: windows_sys::core::PSTR,
    pub pszzPreviousNames: windows_sys::core::PSTR,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub pszMfgName: windows_sys::core::PSTR,
    pub pszOEMUrl: windows_sys::core::PSTR,
    pub pszHardwareID: windows_sys::core::PSTR,
    pub pszProvider: windows_sys::core::PSTR,
    pub pszPrintProcessor: windows_sys::core::PSTR,
    pub pszVendorSetup: windows_sys::core::PSTR,
    pub pszzColorProfiles: windows_sys::core::PSTR,
    pub pszInfPath: windows_sys::core::PSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: windows_sys::core::PSTR,
    pub ftMinInboxDriverVerDate: super::minwindef::FILETIME,
    pub dwlMinInboxDriverVerVersion: super::winnt::DWORDLONG,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DRIVER_INFO_8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_8W {
    pub cVersion: u32,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub pHelpFile: windows_sys::core::PWSTR,
    pub pDependentFiles: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDefaultDataType: windows_sys::core::PWSTR,
    pub pszzPreviousNames: windows_sys::core::PWSTR,
    pub ftDriverDate: super::minwindef::FILETIME,
    pub dwlDriverVersion: super::winnt::DWORDLONG,
    pub pszMfgName: windows_sys::core::PWSTR,
    pub pszOEMUrl: windows_sys::core::PWSTR,
    pub pszHardwareID: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
    pub pszPrintProcessor: windows_sys::core::PWSTR,
    pub pszVendorSetup: windows_sys::core::PWSTR,
    pub pszzColorProfiles: windows_sys::core::PWSTR,
    pub pszInfPath: windows_sys::core::PWSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: windows_sys::core::PWSTR,
    pub ftMinInboxDriverVerDate: super::minwindef::FILETIME,
    pub dwlMinInboxDriverVerVersion: super::winnt::DWORDLONG,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DRIVER_INFO_8W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRIVER_KERNELMODE: u32 = 1;
pub const DRIVER_USERMODE: u32 = 2;
pub const DSPRINT_PENDING: u32 = 2147483648;
pub const DSPRINT_PUBLISH: u32 = 1;
pub const DSPRINT_REPUBLISH: u32 = 8;
pub const DSPRINT_UNPUBLISH: u32 = 4;
pub const DSPRINT_UPDATE: u32 = 2;
pub type EPrintPropertyType = i32;
pub type EPrintXPSJobOperation = i32;
pub type EPrintXPSJobProgress = i32;
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014;
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004;
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000;
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012;
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013;
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011;
pub const ERROR_BIDI_NOT_SUPPORTED: u32 = 50;
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: u32 = 13016;
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: u32 = 13015;
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: u32 = 13005;
pub const ERROR_BIDI_SCHEMA_READ_ONLY: u32 = 13002;
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: u32 = 13010;
pub const ERROR_BIDI_SERVER_OFFLINE: u32 = 13003;
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: u32 = 13006;
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: u32 = 13008;
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: u32 = 13007;
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: u32 = 13009;
pub const ERROR_BIDI_STATUS_OK: u32 = 0;
pub const ERROR_BIDI_STATUS_WARNING: u32 = 13001;
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: u32 = 13017;
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: u32 = 13018;
pub const FORM_BUILTIN: u32 = 1;
#[cfg(feature = "Win32_windef")]
pub type FORM_INFO_1 = FORM_INFO_1A;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct FORM_INFO_1A {
    pub Flags: u32,
    pub pName: windows_sys::core::PSTR,
    pub Size: super::windef::SIZEL,
    pub ImageableArea: super::windef::RECTL,
}
#[cfg(feature = "Win32_windef")]
impl Default for FORM_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct FORM_INFO_1W {
    pub Flags: u32,
    pub pName: windows_sys::core::PWSTR,
    pub Size: super::windef::SIZEL,
    pub ImageableArea: super::windef::RECTL,
}
#[cfg(feature = "Win32_windef")]
impl Default for FORM_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type FORM_INFO_2 = FORM_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct FORM_INFO_2A {
    pub Flags: u32,
    pub pName: windows_sys::core::PCSTR,
    pub Size: super::windef::SIZEL,
    pub ImageableArea: super::windef::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_sys::core::PCSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_sys::core::PCSTR,
    pub wLangId: super::winnt::LANGID,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for FORM_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct FORM_INFO_2W {
    pub Flags: u32,
    pub pName: windows_sys::core::PCWSTR,
    pub Size: super::windef::SIZEL,
    pub ImageableArea: super::windef::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_sys::core::PCWSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_sys::core::PCWSTR,
    pub wLangId: super::winnt::LANGID,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for FORM_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FORM_PRINTER: u32 = 2;
pub const FORM_USER: u32 = 0;
pub const IPDFP_COPY_ALL_FILES: u32 = 1;
pub const JOB_ACCESS_ADMINISTER: u32 = 16;
pub const JOB_ACCESS_READ: u32 = 32;
pub const JOB_ALL_ACCESS: u32 = 983088;
pub const JOB_CONTROL_CANCEL: u32 = 3;
pub const JOB_CONTROL_DELETE: u32 = 5;
pub const JOB_CONTROL_LAST_PAGE_EJECTED: u32 = 7;
pub const JOB_CONTROL_PAUSE: u32 = 1;
pub const JOB_CONTROL_PENDING_ON_DEVICE: u32 = 11;
pub const JOB_CONTROL_RELEASE: u32 = 9;
pub const JOB_CONTROL_RESTART: u32 = 4;
pub const JOB_CONTROL_RESUME: u32 = 2;
pub const JOB_CONTROL_RETAIN: u32 = 8;
pub const JOB_CONTROL_SEND_TOAST: u32 = 10;
pub const JOB_CONTROL_SENT_TO_PRINTER: u32 = 6;
pub const JOB_EXECUTE: u32 = 131088;
#[cfg(feature = "Win32_minwinbase")]
pub type JOB_INFO_1 = JOB_INFO_1A;
#[repr(C)]
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy)]
pub struct JOB_INFO_1A {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pStatus: windows_sys::core::PSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for JOB_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy)]
pub struct JOB_INFO_1W {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pStatus: windows_sys::core::PWSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for JOB_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type JOB_INFO_2 = JOB_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct JOB_INFO_2A {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pNotifyName: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pDevMode: super::wingdi::LPDEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for JOB_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct JOB_INFO_2W {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pNotifyName: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pDevMode: super::wingdi::LPDEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for JOB_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_3 {
    pub JobId: u32,
    pub NextJobId: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type JOB_INFO_4 = JOB_INFO_4A;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct JOB_INFO_4A {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pNotifyName: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pDevMode: super::wingdi::LPDEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for JOB_INFO_4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct JOB_INFO_4W {
    pub JobId: u32,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pNotifyName: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pDevMode: super::wingdi::LPDEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::minwinbase::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for JOB_INFO_4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: u32 = 23;
pub const JOB_NOTIFY_FIELD_DATATYPE: u32 = 5;
pub const JOB_NOTIFY_FIELD_DEVMODE: u32 = 9;
pub const JOB_NOTIFY_FIELD_DOCUMENT: u32 = 13;
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: u32 = 8;
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: u32 = 1;
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: u32 = 4;
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: u32 = 21;
pub const JOB_NOTIFY_FIELD_PARAMETERS: u32 = 7;
pub const JOB_NOTIFY_FIELD_PORT_NAME: u32 = 2;
pub const JOB_NOTIFY_FIELD_POSITION: u32 = 15;
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: u32 = 0;
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 6;
pub const JOB_NOTIFY_FIELD_PRIORITY: u32 = 14;
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: u32 = 24;
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12;
pub const JOB_NOTIFY_FIELD_START_TIME: u32 = 17;
pub const JOB_NOTIFY_FIELD_STATUS: u32 = 10;
pub const JOB_NOTIFY_FIELD_STATUS_STRING: u32 = 11;
pub const JOB_NOTIFY_FIELD_SUBMITTED: u32 = 16;
pub const JOB_NOTIFY_FIELD_TIME: u32 = 19;
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: u32 = 22;
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: u32 = 20;
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: u32 = 18;
pub const JOB_NOTIFY_FIELD_USER_NAME: u32 = 3;
pub const JOB_NOTIFY_TYPE: u32 = 1;
pub const JOB_POSITION_UNSPECIFIED: u32 = 0;
pub const JOB_READ: u32 = 131104;
pub const JOB_STATUS_BLOCKED_DEVQ: u32 = 512;
pub const JOB_STATUS_COMPLETE: u32 = 4096;
pub const JOB_STATUS_DELETED: u32 = 256;
pub const JOB_STATUS_DELETING: u32 = 4;
pub const JOB_STATUS_ERROR: u32 = 2;
pub const JOB_STATUS_OFFLINE: u32 = 32;
pub const JOB_STATUS_PAPEROUT: u32 = 64;
pub const JOB_STATUS_PAUSED: u32 = 1;
pub const JOB_STATUS_PRINTED: u32 = 128;
pub const JOB_STATUS_PRINTING: u32 = 16;
pub const JOB_STATUS_RENDERING_LOCALLY: u32 = 16384;
pub const JOB_STATUS_RESTART: u32 = 2048;
pub const JOB_STATUS_RETAINED: u32 = 8192;
pub const JOB_STATUS_SPOOLING: u32 = 8;
pub const JOB_STATUS_USER_INTERVENTION: u32 = 1024;
pub const JOB_WRITE: u32 = 131088;
pub type LPADDJOB_INFO_1 = LPADDJOB_INFO_1A;
pub type LPADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
pub type LPADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
#[cfg(feature = "Win32_minwindef")]
pub type LPBIDI_DATA = *mut BIDI_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type LPBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
#[cfg(feature = "Win32_minwindef")]
pub type LPBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type LPBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
#[cfg(feature = "Win32_minwindef")]
pub type LPBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
pub type LPDATATYPES_INFO_1 = LPDATATYPES_INFO_1A;
pub type LPDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
pub type LPDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
pub type LPDOC_INFO_1 = LPDOC_INFO_1A;
pub type LPDOC_INFO_1A = *mut DOC_INFO_1A;
pub type LPDOC_INFO_1W = *mut DOC_INFO_1W;
pub type LPDOC_INFO_2 = LPDOC_INFO_2A;
pub type LPDOC_INFO_2A = *mut DOC_INFO_2A;
pub type LPDOC_INFO_2W = *mut DOC_INFO_2W;
pub type LPDOC_INFO_3 = LPDOC_INFO_3A;
pub type LPDOC_INFO_3A = *mut DOC_INFO_3A;
pub type LPDOC_INFO_3W = *mut DOC_INFO_3W;
pub type LPDRIVER_INFO_1 = LPDRIVER_INFO_1A;
pub type LPDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
pub type LPDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
pub type LPDRIVER_INFO_2 = LPDRIVER_INFO_2A;
pub type LPDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
pub type LPDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
pub type LPDRIVER_INFO_3 = LPDRIVER_INFO_3A;
pub type LPDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
pub type LPDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
pub type LPDRIVER_INFO_4 = LPDRIVER_INFO_4A;
pub type LPDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
pub type LPDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
pub type LPDRIVER_INFO_5 = LPDRIVER_INFO_5A;
pub type LPDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
pub type LPDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_6 = LPDRIVER_INFO_6A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_8 = LPDRIVER_INFO_8A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type LPDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
#[cfg(feature = "Win32_windef")]
pub type LPFORM_INFO_1 = LPFORM_INFO_1A;
#[cfg(feature = "Win32_windef")]
pub type LPFORM_INFO_1A = *mut FORM_INFO_1A;
#[cfg(feature = "Win32_windef")]
pub type LPFORM_INFO_1W = *mut FORM_INFO_1W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPFORM_INFO_2 = LPFORM_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPFORM_INFO_2A = *mut FORM_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPFORM_INFO_2W = *mut FORM_INFO_2W;
#[cfg(feature = "Win32_minwinbase")]
pub type LPJOB_INFO_1 = LPJOB_INFO_1A;
#[cfg(feature = "Win32_minwinbase")]
pub type LPJOB_INFO_1A = *mut JOB_INFO_1A;
#[cfg(feature = "Win32_minwinbase")]
pub type LPJOB_INFO_1W = *mut JOB_INFO_1W;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_2 = LPJOB_INFO_2A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_2A = *mut JOB_INFO_2A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_2W = *mut JOB_INFO_2W;
pub type LPJOB_INFO_3 = *mut JOB_INFO_3;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_4 = LPJOB_INFO_4A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_4A = *mut JOB_INFO_4A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPJOB_INFO_4W = *mut JOB_INFO_4W;
pub type LPMONITOR_INFO_1 = LPMONITOR_INFO_1A;
pub type LPMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
pub type LPMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
pub type LPMONITOR_INFO_2 = LPMONITOR_INFO_2A;
pub type LPMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
pub type LPMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
pub type LPPORT_INFO_1 = LPPORT_INFO_1A;
pub type LPPORT_INFO_1A = *mut PORT_INFO_1A;
pub type LPPORT_INFO_1W = *mut PORT_INFO_1W;
pub type LPPORT_INFO_2 = LPPORT_INFO_2A;
pub type LPPORT_INFO_2A = *mut PORT_INFO_2A;
pub type LPPORT_INFO_2W = *mut PORT_INFO_2W;
pub type LPPORT_INFO_3 = LPPORT_INFO_3A;
pub type LPPORT_INFO_3A = *mut PORT_INFO_3A;
pub type LPPORT_INFO_3W = *mut PORT_INFO_3W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_DEFAULTS = LPPRINTER_DEFAULTSA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
#[cfg(feature = "Win32_minwindef")]
pub type LPPRINTER_ENUM_VALUES = LPPRINTER_ENUM_VALUESA;
#[cfg(feature = "Win32_minwindef")]
pub type LPPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "Win32_minwindef")]
pub type LPPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
pub type LPPRINTER_INFO_1 = LPPRINTER_INFO_1A;
pub type LPPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
pub type LPPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_INFO_2 = LPPRINTER_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type LPPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
#[cfg(feature = "Win32_winnt")]
pub type LPPRINTER_INFO_3 = *mut PRINTER_INFO_3;
pub type LPPRINTER_INFO_4 = LPPRINTER_INFO_4A;
pub type LPPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
pub type LPPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
pub type LPPRINTER_INFO_5 = LPPRINTER_INFO_5A;
pub type LPPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
pub type LPPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
pub type LPPRINTER_INFO_6 = *mut PRINTER_INFO_6;
pub type LPPRINTER_INFO_7 = LPPRINTER_INFO_7A;
pub type LPPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
pub type LPPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_8 = LPPRINTER_INFO_8A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_9 = LPPRINTER_INFO_9A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
pub type LPPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
pub type LPPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type LPPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
#[cfg(feature = "Win32_minwindef")]
pub type LPPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
pub type LPPRINTER_OPTIONS = LPPRINTER_OPTIONSA;
pub type LPPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
pub type LPPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type LPPRINTPROCESSOR_INFO_1 = LPPRINTPROCESSOR_INFO_1A;
pub type LPPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
pub type LPPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
pub type LPPROVIDOR_INFO_1 = LPPROVIDOR_INFO_1A;
pub type LPPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
pub type LPPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
pub type LPPROVIDOR_INFO_2 = LPPROVIDOR_INFO_2A;
pub type LPPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
pub type LPPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64;
pub const MAX_PRIORITY: u32 = 99;
pub const MIN_PRIORITY: u32 = 1;
pub type MONITOR_INFO_1 = MONITOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for MONITOR_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for MONITOR_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MONITOR_INFO_2 = MONITOR_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO_2A {
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDLLName: windows_sys::core::PSTR,
}
impl Default for MONITOR_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO_2W {
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDLLName: windows_sys::core::PWSTR,
}
impl Default for MONITOR_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MS_PRINT_JOB_OUTPUT_FILE: windows_sys::core::PCWSTR = windows_sys::core::w!("MsPrintJobOutputFile");
pub const NORMAL_PRINT: u32 = 0;
pub const NO_PRIORITY: u32 = 0;
pub type PADDJOB_INFO_1 = PADDJOB_INFO_1A;
pub type PADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
pub type PADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
#[cfg(feature = "Win32_minwindef")]
pub type PBIDI_DATA = *mut BIDI_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type PBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
#[cfg(feature = "Win32_minwindef")]
pub type PBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type PBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
#[cfg(feature = "Win32_minwindef")]
pub type PBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type PBINARY_CONTAINER = *mut BINARY_CONTAINER;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCORE_PRINTER_DRIVER = PCORE_PRINTER_DRIVERA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCORE_PRINTER_DRIVERA = *mut CORE_PRINTER_DRIVERA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCORE_PRINTER_DRIVERW = *mut CORE_PRINTER_DRIVERW;
pub type PDATATYPES_INFO_1 = PDATATYPES_INFO_1A;
pub type PDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
pub type PDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
pub type PDOC_INFO_1 = PDOC_INFO_1A;
pub type PDOC_INFO_1A = *mut DOC_INFO_1A;
pub type PDOC_INFO_1W = *mut DOC_INFO_1W;
pub type PDOC_INFO_2 = PDOC_INFO_2A;
pub type PDOC_INFO_2A = *mut DOC_INFO_2A;
pub type PDOC_INFO_2W = *mut DOC_INFO_2W;
pub type PDOC_INFO_3 = PDOC_INFO_3A;
pub type PDOC_INFO_3A = *mut DOC_INFO_3A;
pub type PDOC_INFO_3W = *mut DOC_INFO_3W;
pub type PDRIVER_INFO_1 = PDRIVER_INFO_1A;
pub type PDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
pub type PDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
pub type PDRIVER_INFO_2 = PDRIVER_INFO_2A;
pub type PDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
pub type PDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
pub type PDRIVER_INFO_3 = PDRIVER_INFO_3A;
pub type PDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
pub type PDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
pub type PDRIVER_INFO_4 = PDRIVER_INFO_4A;
pub type PDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
pub type PDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
pub type PDRIVER_INFO_5 = PDRIVER_INFO_5A;
pub type PDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
pub type PDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_6 = PDRIVER_INFO_6A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_8 = PDRIVER_INFO_8A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
#[cfg(feature = "Win32_windef")]
pub type PFORM_INFO_1 = PFORM_INFO_1A;
#[cfg(feature = "Win32_windef")]
pub type PFORM_INFO_1A = *mut FORM_INFO_1A;
#[cfg(feature = "Win32_windef")]
pub type PFORM_INFO_1W = *mut FORM_INFO_1W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PFORM_INFO_2 = PFORM_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PFORM_INFO_2A = *mut FORM_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PFORM_INFO_2W = *mut FORM_INFO_2W;
#[cfg(feature = "Win32_minwinbase")]
pub type PJOB_INFO_1 = PJOB_INFO_1A;
#[cfg(feature = "Win32_minwinbase")]
pub type PJOB_INFO_1A = *mut JOB_INFO_1A;
#[cfg(feature = "Win32_minwinbase")]
pub type PJOB_INFO_1W = *mut JOB_INFO_1W;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_2 = PJOB_INFO_2A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_2A = *mut JOB_INFO_2A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_2W = *mut JOB_INFO_2W;
pub type PJOB_INFO_3 = *mut JOB_INFO_3;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_4 = PJOB_INFO_4A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_4A = *mut JOB_INFO_4A;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PJOB_INFO_4W = *mut JOB_INFO_4W;
pub type PMONITOR_INFO_1 = PMONITOR_INFO_1A;
pub type PMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
pub type PMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
pub type PMONITOR_INFO_2 = PMONITOR_INFO_2A;
pub type PMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
pub type PMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
pub type PORT_INFO_1 = PORT_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for PORT_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for PORT_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PORT_INFO_2 = PORT_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_2A {
    pub pPortName: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDescription: windows_sys::core::PSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl Default for PORT_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_2W {
    pub pPortName: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDescription: windows_sys::core::PWSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl Default for PORT_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PORT_INFO_3 = PORT_INFO_3A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_3A {
    pub dwStatus: u32,
    pub pszStatus: windows_sys::core::PSTR,
    pub dwSeverity: u32,
}
impl Default for PORT_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_INFO_3W {
    pub dwStatus: u32,
    pub pszStatus: windows_sys::core::PWSTR,
    pub dwSeverity: u32,
}
impl Default for PORT_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PORT_STATUS_DOOR_OPEN: u32 = 7;
pub const PORT_STATUS_NO_TONER: u32 = 6;
pub const PORT_STATUS_OFFLINE: u32 = 1;
pub const PORT_STATUS_OUTPUT_BIN_FULL: u32 = 4;
pub const PORT_STATUS_OUT_OF_MEMORY: u32 = 9;
pub const PORT_STATUS_PAPER_JAM: u32 = 2;
pub const PORT_STATUS_PAPER_OUT: u32 = 3;
pub const PORT_STATUS_PAPER_PROBLEM: u32 = 5;
pub const PORT_STATUS_POWER_SAVE: u32 = 12;
pub const PORT_STATUS_TONER_LOW: u32 = 10;
pub const PORT_STATUS_TYPE_ERROR: u32 = 1;
pub const PORT_STATUS_TYPE_INFO: u32 = 3;
pub const PORT_STATUS_TYPE_WARNING: u32 = 2;
pub const PORT_STATUS_USER_INTERVENTION: u32 = 8;
pub const PORT_STATUS_WARMING_UP: u32 = 11;
pub const PORT_TYPE_NET_ATTACHED: u32 = 8;
pub const PORT_TYPE_READ: u32 = 2;
pub const PORT_TYPE_REDIRECTED: u32 = 4;
pub const PORT_TYPE_WRITE: u32 = 1;
pub const PPCAPS_BOOKLET_EDGE: u32 = 1;
pub const PPCAPS_BORDER_PRINT: u32 = 1;
pub const PPCAPS_DONT_SEND_EXTRA_PAGES_FOR_DUPLEX: u32 = 2;
pub const PPCAPS_DOWN_THEN_LEFT: u32 = 8;
pub const PPCAPS_DOWN_THEN_RIGHT: u32 = 2;
pub const PPCAPS_LEFT_THEN_DOWN: u32 = 4;
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1;
pub const PPCAPS_RIGHT_THEN_DOWN: u32 = 1;
pub const PPCAPS_SQUARE_SCALING: u32 = 1;
pub type PPORT_INFO_1 = PPORT_INFO_1A;
pub type PPORT_INFO_1A = *mut PORT_INFO_1A;
pub type PPORT_INFO_1W = *mut PORT_INFO_1W;
pub type PPORT_INFO_2 = PPORT_INFO_2A;
pub type PPORT_INFO_2A = *mut PORT_INFO_2A;
pub type PPORT_INFO_2W = *mut PORT_INFO_2W;
pub type PPORT_INFO_3 = PPORT_INFO_3A;
pub type PPORT_INFO_3A = *mut PORT_INFO_3A;
pub type PPORT_INFO_3W = *mut PORT_INFO_3W;
pub type PPRINTER_CONNECTION_INFO_1 = PPRINTER_CONNECTION_INFO_1A;
pub type PPRINTER_CONNECTION_INFO_1A = *mut PRINTER_CONNECTION_INFO_1A;
pub type PPRINTER_CONNECTION_INFO_1W = *mut PRINTER_CONNECTION_INFO_1W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_DEFAULTS = PPRINTER_DEFAULTSA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
#[cfg(feature = "Win32_minwindef")]
pub type PPRINTER_ENUM_VALUES = PPRINTER_ENUM_VALUESA;
#[cfg(feature = "Win32_minwindef")]
pub type PPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "Win32_minwindef")]
pub type PPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
pub type PPRINTER_INFO_1 = PPRINTER_INFO_1A;
pub type PPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
pub type PPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_INFO_2 = PPRINTER_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
#[cfg(feature = "Win32_winnt")]
pub type PPRINTER_INFO_3 = *mut PRINTER_INFO_3;
pub type PPRINTER_INFO_4 = PPRINTER_INFO_4A;
pub type PPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
pub type PPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
pub type PPRINTER_INFO_5 = PPRINTER_INFO_5A;
pub type PPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
pub type PPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
pub type PPRINTER_INFO_6 = *mut PRINTER_INFO_6;
pub type PPRINTER_INFO_7 = PPRINTER_INFO_7A;
pub type PPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
pub type PPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_8 = PPRINTER_INFO_8A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_9 = PPRINTER_INFO_9A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
pub type PPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
pub type PPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
#[cfg(feature = "Win32_minwindef")]
pub type PPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
#[cfg(feature = "Win32_minwindef")]
pub type PPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
pub type PPRINTER_OPTIONS = PPRINTER_OPTIONSA;
pub type PPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
pub type PPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type PPRINTPROCESSOR_CAPS_1 = *mut PRINTPROCESSOR_CAPS_1;
pub type PPRINTPROCESSOR_CAPS_2 = *mut PRINTPROCESSOR_CAPS_2;
pub type PPRINTPROCESSOR_INFO_1 = PPRINTPROCESSOR_INFO_1A;
pub type PPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
pub type PPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
pub type PPROVIDOR_INFO_1 = PPROVIDOR_INFO_1A;
pub type PPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
pub type PPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
pub type PPROVIDOR_INFO_2 = PPROVIDOR_INFO_2A;
pub type PPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
pub type PPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
pub const PRINTER_ACCESS_ADMINISTER: u32 = 4;
pub const PRINTER_ACCESS_MANAGE_LIMITED: u32 = 64;
pub const PRINTER_ACCESS_USE: u32 = 8;
pub const PRINTER_ALL_ACCESS: u32 = 983052;
pub const PRINTER_ATTRIBUTE_DEFAULT: u32 = 4;
pub const PRINTER_ATTRIBUTE_DIRECT: u32 = 2;
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: u32 = 512;
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: u32 = 2048;
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: u32 = 128;
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: u32 = 8388608;
pub const PRINTER_ATTRIBUTE_FAX: u32 = 16384;
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: u32 = 1048576;
pub const PRINTER_ATTRIBUTE_HIDDEN: u32 = 32;
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: u32 = 256;
pub const PRINTER_ATTRIBUTE_LOCAL: u32 = 64;
pub const PRINTER_ATTRIBUTE_MACHINE: u32 = 524288;
pub const PRINTER_ATTRIBUTE_NETWORK: u32 = 16;
pub const PRINTER_ATTRIBUTE_PER_USER: u32 = 4194304;
pub const PRINTER_ATTRIBUTE_PUBLISHED: u32 = 8192;
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: u32 = 262144;
pub const PRINTER_ATTRIBUTE_PUSHED_USER: u32 = 131072;
pub const PRINTER_ATTRIBUTE_QUEUED: u32 = 1;
pub const PRINTER_ATTRIBUTE_RAW_ONLY: u32 = 4096;
pub const PRINTER_ATTRIBUTE_SHARED: u32 = 8;
pub const PRINTER_ATTRIBUTE_TS: u32 = 32768;
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: u32 = 2097152;
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: u32 = 1024;
pub const PRINTER_CHANGE_ADD_FORM: u32 = 65536;
pub const PRINTER_CHANGE_ADD_JOB: u32 = 256;
pub const PRINTER_CHANGE_ADD_PORT: u32 = 1048576;
pub const PRINTER_CHANGE_ADD_PRINTER: u32 = 1;
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: u32 = 268435456;
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: u32 = 16777216;
pub const PRINTER_CHANGE_ALL: u32 = 2138570751;
pub const PRINTER_CHANGE_CONFIGURE_PORT: u32 = 2097152;
pub const PRINTER_CHANGE_DELETE_FORM: u32 = 262144;
pub const PRINTER_CHANGE_DELETE_JOB: u32 = 1024;
pub const PRINTER_CHANGE_DELETE_PORT: u32 = 4194304;
pub const PRINTER_CHANGE_DELETE_PRINTER: u32 = 4;
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: u32 = 1073741824;
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: u32 = 67108864;
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: u32 = 8;
pub const PRINTER_CHANGE_FORM: u32 = 458752;
pub const PRINTER_CHANGE_JOB: u32 = 65280;
pub const PRINTER_CHANGE_PORT: u32 = 7340032;
pub const PRINTER_CHANGE_PRINTER: u32 = 255;
pub const PRINTER_CHANGE_PRINTER_DRIVER: u32 = 1879048192;
pub const PRINTER_CHANGE_PRINT_PROCESSOR: u32 = 117440512;
pub const PRINTER_CHANGE_SERVER: u32 = 134217728;
pub const PRINTER_CHANGE_SET_FORM: u32 = 131072;
pub const PRINTER_CHANGE_SET_JOB: u32 = 512;
pub const PRINTER_CHANGE_SET_PRINTER: u32 = 2;
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: u32 = 536870912;
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648;
pub const PRINTER_CHANGE_WRITE_JOB: u32 = 2048;
pub type PRINTER_CONNECTION_INFO_1 = PRINTER_CONNECTION_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_CONNECTION_INFO_1A {
    pub dwFlags: u32,
    pub pszDriverName: windows_sys::core::PSTR,
}
impl Default for PRINTER_CONNECTION_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_CONNECTION_INFO_1W {
    pub dwFlags: u32,
    pub pszDriverName: windows_sys::core::PWSTR,
}
impl Default for PRINTER_CONNECTION_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_CONNECTION_MISMATCH: u32 = 32;
pub const PRINTER_CONNECTION_NO_UI: u32 = 64;
pub const PRINTER_CONTROL_PAUSE: u32 = 1;
pub const PRINTER_CONTROL_PURGE: u32 = 3;
pub const PRINTER_CONTROL_RESUME: u32 = 2;
pub const PRINTER_CONTROL_SET_STATUS: u32 = 4;
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PRINTER_DEFAULTS = PRINTER_DEFAULTSA;
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: windows_sys::core::PSTR,
    pub pDevMode: super::wingdi::LPDEVMODEA,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for PRINTER_DEFAULTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: windows_sys::core::PWSTR,
    pub pDevMode: super::wingdi::LPDEVMODEW,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for PRINTER_DEFAULTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_DRIVER_CATEGORY_3D: u32 = 4096;
pub const PRINTER_DRIVER_CATEGORY_CLOUD: u32 = 8192;
pub const PRINTER_DRIVER_CATEGORY_FAX: u32 = 64;
pub const PRINTER_DRIVER_CATEGORY_FILE: u32 = 128;
pub const PRINTER_DRIVER_CATEGORY_SERVICE: u32 = 512;
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: u32 = 256;
pub const PRINTER_DRIVER_CLASS: u32 = 8;
pub const PRINTER_DRIVER_DERIVED: u32 = 16;
pub const PRINTER_DRIVER_NOT_SHAREABLE: u32 = 32;
pub const PRINTER_DRIVER_PACKAGE_AWARE: u32 = 1;
pub const PRINTER_DRIVER_SANDBOX_DISABLED: u32 = 2048;
pub const PRINTER_DRIVER_SANDBOX_ENABLED: u32 = 4;
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: u32 = 1024;
pub const PRINTER_DRIVER_XPS: u32 = 2;
pub const PRINTER_ENUM_CATEGORY_3D: u32 = 67108864;
pub const PRINTER_ENUM_CATEGORY_ALL: u32 = 33554432;
pub const PRINTER_ENUM_CONNECTIONS: u32 = 4;
pub const PRINTER_ENUM_CONTAINER: u32 = 32768;
pub const PRINTER_ENUM_DEFAULT: u32 = 1;
pub const PRINTER_ENUM_EXPAND: u32 = 16384;
pub const PRINTER_ENUM_FAVORITE: u32 = 4;
pub const PRINTER_ENUM_HIDE: u32 = 16777216;
pub const PRINTER_ENUM_ICON1: u32 = 65536;
pub const PRINTER_ENUM_ICON2: u32 = 131072;
pub const PRINTER_ENUM_ICON3: u32 = 262144;
pub const PRINTER_ENUM_ICON4: u32 = 524288;
pub const PRINTER_ENUM_ICON5: u32 = 1048576;
pub const PRINTER_ENUM_ICON6: u32 = 2097152;
pub const PRINTER_ENUM_ICON7: u32 = 4194304;
pub const PRINTER_ENUM_ICON8: u32 = 8388608;
pub const PRINTER_ENUM_ICONMASK: u32 = 16711680;
pub const PRINTER_ENUM_LOCAL: u32 = 2;
pub const PRINTER_ENUM_NAME: u32 = 8;
pub const PRINTER_ENUM_NETWORK: u32 = 64;
pub const PRINTER_ENUM_REMOTE: u32 = 16;
pub const PRINTER_ENUM_SHARED: u32 = 32;
#[cfg(feature = "Win32_minwindef")]
pub type PRINTER_ENUM_VALUES = PRINTER_ENUM_VALUESA;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: windows_sys::core::PSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: super::minwindef::LPBYTE,
    pub cbData: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRINTER_ENUM_VALUESA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: windows_sys::core::PWSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: super::minwindef::LPBYTE,
    pub cbData: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRINTER_ENUM_VALUESW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648;
pub const PRINTER_ERROR_JAM: u32 = 2;
pub const PRINTER_ERROR_OUTOFPAPER: u32 = 1;
pub const PRINTER_ERROR_OUTOFTONER: u32 = 4;
pub const PRINTER_ERROR_SEVERE: u32 = 536870912;
pub const PRINTER_ERROR_WARNING: u32 = 1073741824;
pub const PRINTER_EXECUTE: u32 = 131080;
pub type PRINTER_INFO_1 = PRINTER_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_1A {
    pub Flags: u32,
    pub pDescription: windows_sys::core::PSTR,
    pub pName: windows_sys::core::PSTR,
    pub pComment: windows_sys::core::PSTR,
}
impl Default for PRINTER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_1W {
    pub Flags: u32,
    pub pDescription: windows_sys::core::PWSTR,
    pub pName: windows_sys::core::PWSTR,
    pub pComment: windows_sys::core::PWSTR,
}
impl Default for PRINTER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
pub type PRINTER_INFO_2 = PRINTER_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_2A {
    pub pServerName: windows_sys::core::PSTR,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pShareName: windows_sys::core::PSTR,
    pub pPortName: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pComment: windows_sys::core::PSTR,
    pub pLocation: windows_sys::core::PSTR,
    pub pDevMode: super::wingdi::LPDEVMODEA,
    pub pSepFile: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for PRINTER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_2W {
    pub pServerName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pShareName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pComment: windows_sys::core::PWSTR,
    pub pLocation: windows_sys::core::PWSTR,
    pub pDevMode: super::wingdi::LPDEVMODEW,
    pub pSepFile: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi", feature = "Win32_winnt"))]
impl Default for PRINTER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PRINTER_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRINTER_INFO_4 = PRINTER_INFO_4A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_4A {
    pub pPrinterName: windows_sys::core::PSTR,
    pub pServerName: windows_sys::core::PSTR,
    pub Attributes: u32,
}
impl Default for PRINTER_INFO_4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_4W {
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pServerName: windows_sys::core::PWSTR,
    pub Attributes: u32,
}
impl Default for PRINTER_INFO_4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRINTER_INFO_5 = PRINTER_INFO_5A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_5A {
    pub pPrinterName: windows_sys::core::PSTR,
    pub pPortName: windows_sys::core::PSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl Default for PRINTER_INFO_5A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_5W {
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl Default for PRINTER_INFO_5W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_6 {
    pub dwStatus: u32,
}
pub type PRINTER_INFO_7 = PRINTER_INFO_7A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_7A {
    pub pszObjectGUID: windows_sys::core::PSTR,
    pub dwAction: u32,
}
impl Default for PRINTER_INFO_7A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: windows_sys::core::PWSTR,
    pub dwAction: u32,
}
impl Default for PRINTER_INFO_7W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PRINTER_INFO_8 = PRINTER_INFO_8A;
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_8A {
    pub pDevMode: super::wingdi::LPDEVMODEA,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for PRINTER_INFO_8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_8W {
    pub pDevMode: super::wingdi::LPDEVMODEW,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for PRINTER_INFO_8W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PRINTER_INFO_9 = PRINTER_INFO_9A;
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_9A {
    pub pDevMode: super::wingdi::LPDEVMODEA,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for PRINTER_INFO_9A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_9W {
    pub pDevMode: super::wingdi::LPDEVMODEW,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for PRINTER_INFO_9W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_CATEGORY_3D: u32 = 8192;
pub const PRINTER_NOTIFY_CATEGORY_ALL: u32 = 4096;
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: u32 = 13;
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: u32 = 21;
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: u32 = 28;
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: u32 = 25;
pub const PRINTER_NOTIFY_FIELD_CJOBS: u32 = 20;
pub const PRINTER_NOTIFY_FIELD_COMMENT: u32 = 5;
pub const PRINTER_NOTIFY_FIELD_DATATYPE: u32 = 11;
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: u32 = 15;
pub const PRINTER_NOTIFY_FIELD_DEVMODE: u32 = 7;
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: u32 = 4;
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: u32 = 27;
pub const PRINTER_NOTIFY_FIELD_LOCATION: u32 = 6;
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: u32 = 26;
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: u32 = 23;
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: u32 = 10;
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: u32 = 3;
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: u32 = 1;
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 9;
pub const PRINTER_NOTIFY_FIELD_PRIORITY: u32 = 14;
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12;
pub const PRINTER_NOTIFY_FIELD_SEPFILE: u32 = 8;
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: u32 = 0;
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: u32 = 2;
pub const PRINTER_NOTIFY_FIELD_START_TIME: u32 = 16;
pub const PRINTER_NOTIFY_FIELD_STATUS: u32 = 18;
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: u32 = 19;
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: u32 = 24;
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: u32 = 22;
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}
impl Default for PRINTER_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO_DATA {
    pub Type: u16,
    pub Field: u16,
    pub Reserved: u32,
    pub Id: u32,
    pub NotifyData: PRINTER_NOTIFY_INFO_DATA_0,
}
impl Default for PRINTER_NOTIFY_INFO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRINTER_NOTIFY_INFO_DATA_0 {
    pub adwData: [u32; 2],
    pub Data: PRINTER_NOTIFY_INFO_DATA_0_0,
}
impl Default for PRINTER_NOTIFY_INFO_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PRINTER_NOTIFY_INFO_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub pTypes: PPRINTER_NOTIFY_OPTIONS_TYPE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRINTER_NOTIFY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: u16,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Count: u32,
    pub pFields: super::minwindef::PWORD,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_TYPE: u32 = 0;
pub type PRINTER_OPTIONS = PRINTER_OPTIONSA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_OPTIONSA {
    pub cbSize: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_OPTIONSW {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = 2;
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = 4;
pub type PRINTER_OPTION_FLAGS = i32;
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = 1;
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = 8;
pub const PRINTER_READ: u32 = 131080;
pub const PRINTER_STATUS_BUSY: u32 = 512;
pub const PRINTER_STATUS_DOOR_OPEN: u32 = 4194304;
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: u32 = 67108864;
pub const PRINTER_STATUS_ERROR: u32 = 2;
pub const PRINTER_STATUS_INITIALIZING: u32 = 32768;
pub const PRINTER_STATUS_IO_ACTIVE: u32 = 256;
pub const PRINTER_STATUS_MANUAL_FEED: u32 = 32;
pub const PRINTER_STATUS_NOT_AVAILABLE: u32 = 4096;
pub const PRINTER_STATUS_NO_TONER: u32 = 262144;
pub const PRINTER_STATUS_OFFLINE: u32 = 128;
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: u32 = 2048;
pub const PRINTER_STATUS_OUT_OF_MEMORY: u32 = 2097152;
pub const PRINTER_STATUS_PAGE_PUNT: u32 = 524288;
pub const PRINTER_STATUS_PAPER_JAM: u32 = 8;
pub const PRINTER_STATUS_PAPER_OUT: u32 = 16;
pub const PRINTER_STATUS_PAPER_PROBLEM: u32 = 64;
pub const PRINTER_STATUS_PAUSED: u32 = 1;
pub const PRINTER_STATUS_PENDING_DELETION: u32 = 4;
pub const PRINTER_STATUS_POWER_SAVE: u32 = 16777216;
pub const PRINTER_STATUS_PRINTING: u32 = 1024;
pub const PRINTER_STATUS_PROCESSING: u32 = 16384;
pub const PRINTER_STATUS_SERVER_OFFLINE: u32 = 33554432;
pub const PRINTER_STATUS_SERVER_UNKNOWN: u32 = 8388608;
pub const PRINTER_STATUS_TONER_LOW: u32 = 131072;
pub const PRINTER_STATUS_USER_INTERVENTION: u32 = 1048576;
pub const PRINTER_STATUS_WAITING: u32 = 8192;
pub const PRINTER_STATUS_WARMING_UP: u32 = 65536;
pub const PRINTER_WRITE: u32 = 131080;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_CAPS_2 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwDuplexHandlingCaps: u32,
    pub dwNupDirectionCaps: u32,
    pub dwNupBorderCaps: u32,
    pub dwBookletHandlingCaps: u32,
    pub dwScalingCaps: u32,
}
pub type PRINTPROCESSOR_INFO_1 = PRINTPROCESSOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for PRINTPROCESSOR_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for PRINTPROCESSOR_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRINT_EXECUTION_CONTEXT = i32;
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = 0;
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = 3;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = 2;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = 1;
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: u32,
}
pub type PROVIDOR_INFO_1 = PROVIDOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDLLName: windows_sys::core::PSTR,
}
impl Default for PROVIDOR_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDLLName: windows_sys::core::PWSTR,
}
impl Default for PROVIDOR_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROVIDOR_INFO_2 = PROVIDOR_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: windows_sys::core::PSTR,
}
impl Default for PROVIDOR_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: windows_sys::core::PWSTR,
}
impl Default for PROVIDOR_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintNamedProperty {
    pub propertyName: *mut u16,
    pub propertyValue: PrintPropertyValue,
}
impl Default for PrintNamedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintPropertiesCollection {
    pub numberOfProperties: u32,
    pub propertiesCollection: *mut PrintNamedProperty,
}
impl Default for PrintPropertiesCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintPropertyValue {
    pub ePropertyType: EPrintPropertyType,
    pub value: PrintPropertyValue_0,
}
impl Default for PrintPropertyValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PrintPropertyValue_0 {
    pub propertyByte: u8,
    pub propertyString: windows_sys::core::PWSTR,
    pub propertyInt32: i32,
    pub propertyInt64: i64,
    pub propertyBlob: PrintPropertyValue_0_0,
}
impl Default for PrintPropertyValue_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PrintPropertyValue_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REVERSE_PRINT: u32 = 1;
pub const SERVER_ACCESS_ADMINISTER: u32 = 1;
pub const SERVER_ACCESS_ENUMERATE: u32 = 2;
pub const SERVER_ALL_ACCESS: u32 = 983043;
pub const SERVER_EXECUTE: u32 = 131074;
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0;
pub const SERVER_NOTIFY_TYPE: u32 = 2;
pub const SERVER_READ: u32 = 131074;
pub const SERVER_WRITE: u32 = 131075;
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS_SEPARATOR: u32 = 92;
pub const SPOOL_FILE_PERSISTENT: u32 = 1;
pub const SPOOL_FILE_TEMPORARY: u32 = 2;
pub const STRING_LANGPAIR: u32 = 4;
pub const STRING_MUIDLL: u32 = 2;
pub const STRING_NONE: u32 = 1;
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4;
pub const UPDP_SILENT_UPLOAD: u32 = 1;
pub const UPDP_UPLOAD_ALWAYS: u32 = 2;
pub const kAddingDocumentSequence: EPrintXPSJobProgress = 0;
pub const kAddingFixedDocument: EPrintXPSJobProgress = 2;
pub const kAddingFixedPage: EPrintXPSJobProgress = 4;
pub const kDocumentSequenceAdded: EPrintXPSJobProgress = 1;
pub const kFixedDocumentAdded: EPrintXPSJobProgress = 3;
pub const kFixedPageAdded: EPrintXPSJobProgress = 5;
pub const kFontAdded: EPrintXPSJobProgress = 7;
pub const kImageAdded: EPrintXPSJobProgress = 8;
pub const kJobConsumption: EPrintXPSJobOperation = 2;
pub const kJobProduction: EPrintXPSJobOperation = 1;
pub const kPropertyTypeBuffer: EPrintPropertyType = 10;
pub const kPropertyTypeByte: EPrintPropertyType = 4;
pub const kPropertyTypeDevMode: EPrintPropertyType = 6;
pub const kPropertyTypeInt32: EPrintPropertyType = 2;
pub const kPropertyTypeInt64: EPrintPropertyType = 3;
pub const kPropertyTypeNotificationOptions: EPrintPropertyType = 9;
pub const kPropertyTypeNotificationReply: EPrintPropertyType = 8;
pub const kPropertyTypeSD: EPrintPropertyType = 7;
pub const kPropertyTypeString: EPrintPropertyType = 1;
pub const kPropertyTypeTime: EPrintPropertyType = 5;
pub const kResourceAdded: EPrintXPSJobProgress = 6;
pub const kXpsDocumentCommitted: EPrintXPSJobProgress = 9;
