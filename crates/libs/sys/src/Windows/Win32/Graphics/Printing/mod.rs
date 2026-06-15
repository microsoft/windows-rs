#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
windows_link::link!("winspool.drv" "system" fn AbortPrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddFormA(hprinter : PRINTER_HANDLE, level : u32, pform : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddFormW(hprinter : PRINTER_HANDLE, level : u32, pform : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddJobA(hprinter : PRINTER_HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddJobW(hprinter : PRINTER_HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddMonitorA(pname : windows_sys::core::PCSTR, level : u32, pmonitors : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddMonitorW(pname : windows_sys::core::PCWSTR, level : u32, pmonitors : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPortA(pname : windows_sys::core::PCSTR, hwnd : super::super::Foundation::HWND, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPortW(pname : windows_sys::core::PCWSTR, hwnd : super::super::Foundation::HWND, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn AddPrintDeviceObject(hprinter : PRINTER_HANDLE, phdeviceobject : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, ppathname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, ppathname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorA(pname : windows_sys::core::PCSTR, level : u32, pprovidorinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorW(pname : windows_sys::core::PCWSTR, level : u32, pprovidorinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterA(pname : windows_sys::core::PCSTR, level : u32, pprinter : *const u8) -> PRINTER_HANDLE);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2A(hwnd : super::super::Foundation::HWND, pszname : windows_sys::core::PCSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2W(hwnd : super::super::Foundation::HWND, pszname : windows_sys::core::PCWSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverA(pname : windows_sys::core::PCSTR, level : u32, pdriverinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExA(pname : windows_sys::core::PCSTR, level : u32, lpbdriverinfo : *const u8, dwfilecopyflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExW(pname : windows_sys::core::PCWSTR, level : u32, lpbdriverinfo : *const u8, dwfilecopyflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverW(pname : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterW(pname : windows_sys::core::PCWSTR, level : u32, pprinter : *const u8) -> PRINTER_HANDLE);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesA(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : *mut super::Gdi::DEVMODEA, pdevmodeinput : *const super::Gdi::DEVMODEA) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesW(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : *mut super::Gdi::DEVMODEW, pdevmodeinput : *const super::Gdi::DEVMODEW) -> i32);
windows_link::link!("spoolss.dll" "system" fn AppendPrinterNotifyInfoData(pinfodest : *const PRINTER_NOTIFY_INFO, pdatasrc : *const PRINTER_NOTIFY_INFO_DATA, fdwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn CallRouterFindFirstPrinterChangeNotification(hprinterrpc : super::super::Foundation::HANDLE, fdwfilterflags : u32, fdwoptions : u32, hnotify : super::super::Foundation::HANDLE, pprinternotifyoptions : *const PRINTER_NOTIFY_OPTIONS) -> u32);
windows_link::link!("winspool.drv" "system" fn ClosePrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn CloseSpoolFileHandle(hprinter : PRINTER_HANDLE, hspoolfile : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn CommitSpoolData(hprinter : PRINTER_HANDLE, hspoolfile : super::super::Foundation::HANDLE, cbcommit : u32) -> super::super::Foundation::HANDLE);
windows_link::link!("compstui.dll" "system" fn CommonPropertySheetUIA(hwndowner : super::super::Foundation::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::super::Foundation::LPARAM, presult : *mut u32) -> i32);
windows_link::link!("compstui.dll" "system" fn CommonPropertySheetUIW(hwndowner : super::super::Foundation::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::super::Foundation::LPARAM, presult : *mut u32) -> i32);
windows_link::link!("winspool.drv" "system" fn ConfigurePortA(pname : windows_sys::core::PCSTR, hwnd : super::super::Foundation::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn ConfigurePortW(pname : windows_sys::core::PCWSTR, hwnd : super::super::Foundation::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn ConnectToPrinterDlg(hwnd : super::super::Foundation::HWND, flags : u32) -> super::super::Foundation::HANDLE);
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::super::Foundation::FILETIME, dwldriverversion : u64, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::super::Foundation::FILETIME, dwldriverversion : u64, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn CreatePrintAsyncNotifyChannel(pszname : windows_sys::core::PCWSTR, pnotificationtype : *const windows_sys::core::GUID, euserfilter : PrintAsyncNotifyUserFilter, econversationstyle : PrintAsyncNotifyConversationStyle, pcallback : *mut core::ffi::c_void, ppiasynchnotification : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn CreatePrinterIC(hprinter : PRINTER_HANDLE, pdevmode : *const super::Gdi::DEVMODEW) -> super::super::Foundation::HANDLE);
windows_link::link!("winspool.drv" "system" fn DeleteFormA(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeleteFormW(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeleteJobNamedProperty(hprinter : PRINTER_HANDLE, jobid : u32, pszname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePortA(pname : windows_sys::core::PCSTR, hwnd : super::super::Foundation::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePortW(pname : windows_sys::core::PCWSTR, hwnd : super::super::Foundation::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprovidorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprovidorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterIC(hprinteric : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR) -> u32);
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn DevQueryPrint(hprinter : PRINTER_HANDLE, pdevmode : *const super::Gdi::DEVMODEA, presid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn DevQueryPrintEx(pdqpinfo : *mut DEVQUERYPRINT_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesA(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : *mut super::Gdi::DEVMODEA, pdevmodeinput : *const super::Gdi::DEVMODEA, fmode : u32) -> i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesW(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : *mut super::Gdi::DEVMODEW, pdevmodeinput : *const super::Gdi::DEVMODEW, fmode : u32) -> i32);
windows_link::link!("winspool.drv" "system" fn EndDocPrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EndPagePrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumFormsA(hprinter : PRINTER_HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumFormsW(hprinter : PRINTER_HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumJobNamedProperties(hprinter : PRINTER_HANDLE, jobid : u32, pcproperties : *mut u32, ppproperties : *mut *mut PrintNamedProperty) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumJobsA(hprinter : PRINTER_HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumJobsW(hprinter : PRINTER_HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumMonitorsA(pname : windows_sys::core::PCSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumMonitorsW(pname : windows_sys::core::PCWSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPortsA(pname : windows_sys::core::PCSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPortsW(pname : windows_sys::core::PCWSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesA(pname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesW(pname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataA(hprinter : PRINTER_HANDLE, dwindex : u32, pvaluename : windows_sys::core::PSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataW(hprinter : PRINTER_HANDLE, dwindex : u32, pvaluename : windows_sys::core::PWSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR, psubkey : windows_sys::core::PSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR, psubkey : windows_sys::core::PWSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn EnumPrintersA(flags : u32, name : windows_sys::core::PCSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn EnumPrintersW(flags : u32, name : windows_sys::core::PCWSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn ExtDeviceMode(hwnd : super::super::Foundation::HWND, hinst : super::super::Foundation::HANDLE, pdevmodeoutput : *mut super::Gdi::DEVMODEA, pdevicename : windows_sys::core::PCSTR, pport : windows_sys::core::PCSTR, pdevmodeinput : *const super::Gdi::DEVMODEA, pprofile : windows_sys::core::PCSTR, fmode : u32) -> i32);
windows_link::link!("winspool.drv" "system" fn FindClosePrinterChangeNotification(hchange : FINDPRINTERCHANGENOTIFICATION_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn FindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwfilter : u32, fdwoptions : u32, pprinternotifyoptions : *const core::ffi::c_void) -> FINDPRINTERCHANGENOTIFICATION_HANDLE);
windows_link::link!("winspool.drv" "system" fn FindNextPrinterChangeNotification(hchange : FINDPRINTERCHANGENOTIFICATION_HANDLE, pdwchange : *mut u32, pvreserved : *const core::ffi::c_void, ppprinternotifyinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn FlushPrinter(hprinter : PRINTER_HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32, csleep : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn FreePrintNamedPropertyArray(cproperties : u32, ppproperties : *mut *mut PrintNamedProperty));
windows_link::link!("winspool.drv" "system" fn FreePrintPropertyValue(pvalue : *mut PrintPropertyValue));
windows_link::link!("winspool.drv" "system" fn FreePrinterNotifyInfo(pprinternotifyinfo : *const PRINTER_NOTIFY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiDeleteSpoolFileHandle(spoolfilehandle : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiEndDocEMF(spoolfilehandle : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiEndPageEMF(spoolfilehandle : super::super::Foundation::HANDLE, dwoptimization : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("gdi32.dll" "system" fn GdiGetDC(spoolfilehandle : super::super::Foundation::HANDLE) -> super::Gdi::HDC);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("gdi32.dll" "system" fn GdiGetDevmodeForPage(spoolfilehandle : super::super::Foundation::HANDLE, dwpagenumber : u32, pcurrdm : *mut *mut super::Gdi::DEVMODEW, plastdm : *mut *mut super::Gdi::DEVMODEW) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiGetPageCount(spoolfilehandle : super::super::Foundation::HANDLE) -> u32);
windows_link::link!("gdi32.dll" "system" fn GdiGetPageHandle(spoolfilehandle : super::super::Foundation::HANDLE, page : u32, pdwpagetype : *mut u32) -> super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("gdi32.dll" "system" fn GdiGetSpoolFileHandle(pwszprintername : windows_sys::core::PCWSTR, pdevmode : *mut super::Gdi::DEVMODEW, pwszdocname : windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE);
windows_link::link!("gdi32.dll" "system" fn GdiPlayPageEMF(spoolfilehandle : super::super::Foundation::HANDLE, hemf : super::super::Foundation::HANDLE, prectdocument : *mut super::super::Foundation::RECT, prectborder : *mut super::super::Foundation::RECT, prectclip : *mut super::super::Foundation::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("gdi32.dll" "system" fn GdiResetDCEMF(spoolfilehandle : super::super::Foundation::HANDLE, pcurrdm : *mut super::Gdi::DEVMODEW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Storage_Xps")]
windows_link::link!("gdi32.dll" "system" fn GdiStartDocEMF(spoolfilehandle : super::super::Foundation::HANDLE, pdocinfo : *mut super::super::Storage::Xps::DOCINFOW) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiStartPageEMF(spoolfilehandle : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn GenerateCopyFilePaths(pszprintername : windows_sys::core::PCWSTR, pszdirectory : windows_sys::core::PCWSTR, psplclientinfo : *const u8, dwlevel : u32, pszsourcedir : windows_sys::core::PWSTR, pcchsourcedirsize : *mut u32, psztargetdir : windows_sys::core::PWSTR, pcchtargetdirsize : *mut u32, dwflags : u32) -> u32);
windows_link::link!("compstui.dll" "system" fn GetCPSUIUserData(hdlg : super::super::Foundation::HWND) -> usize);
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszzcoredriverdependencies : windows_sys::core::PCSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERA) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszzcoredriverdependencies : windows_sys::core::PCWSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERW) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterA(pszbuffer : windows_sys::core::PSTR, pcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterW(pszbuffer : windows_sys::core::PWSTR, pcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetFormA(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetFormW(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCWSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetJobA(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("spoolss.dll" "system" fn GetJobAttributes(pprintername : windows_sys::core::PCWSTR, pdevmode : *const super::Gdi::DEVMODEW, pattributeinfo : *mut ATTRIBUTE_INFO_3) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("spoolss.dll" "system" fn GetJobAttributesEx(pprintername : windows_sys::core::PCWSTR, pdevmode : *const super::Gdi::DEVMODEW, dwlevel : u32, pattributeinfo : *mut u8, nsize : u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetJobNamedPropertyValue(hprinter : PRINTER_HANDLE, jobid : u32, pszname : windows_sys::core::PCWSTR, pvalue : *mut PrintPropertyValue) -> u32);
windows_link::link!("winspool.drv" "system" fn GetJobW(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrintExecutionData(pdata : *mut PRINT_EXECUTION_DATA) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrintOutputInfo(hwnd : super::super::Foundation::HWND, pszprinter : windows_sys::core::PCWSTR, phfile : *mut super::super::Foundation::HANDLE, ppszoutputfile : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterA(hprinter : PRINTER_HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn GetPrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2A(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2W(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverA(hprinter : PRINTER_HANDLE, penvironment : windows_sys::core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszlanguage : windows_sys::core::PCSTR, pszpackageid : windows_sys::core::PCSTR, pszdriverpackagecab : windows_sys::core::PSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszlanguage : windows_sys::core::PCWSTR, pszpackageid : windows_sys::core::PCWSTR, pszdriverpackagecab : windows_sys::core::PWSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverW(hprinter : PRINTER_HANDLE, penvironment : windows_sys::core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetPrinterW(hprinter : PRINTER_HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn GetSpoolFileHandle(hprinter : PRINTER_HANDLE) -> super::super::Foundation::HANDLE);
windows_link::link!("spoolss.dll" "system" fn ImpersonatePrinterClient(htoken : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszdrivername : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszdrivername : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeA(pdevmode : *const super::Gdi::DEVMODEA, devmodesize : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeW(pdevmode : *const super::Gdi::DEVMODEW, devmodesize : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2A(pprintername : windows_sys::core::PCSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA, poptions : *const PRINTER_OPTIONSA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2W(pprintername : windows_sys::core::PCWSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW, poptions : *const PRINTER_OPTIONSW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn OpenPrinterA(pprintername : windows_sys::core::PCSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn OpenPrinterW(pprintername : windows_sys::core::PCWSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn PartialReplyPrinterChangeNotification(hprinter : PRINTER_HANDLE, pdatasrc : *const PRINTER_NOTIFY_INFO_DATA) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn PlayGdiScriptOnPrinterIC(hprinteric : super::super::Foundation::HANDLE, pin : *const u8, cin : u32, pout : *mut u8, cout : u32, ul : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxA(hprinter : PRINTER_HANDLE, error : u32, hwnd : super::super::Foundation::HWND, ptext : windows_sys::core::PCSTR, pcaption : windows_sys::core::PCSTR, dwtype : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxW(hprinter : PRINTER_HANDLE, error : u32, hwnd : super::super::Foundation::HWND, ptext : windows_sys::core::PCWSTR, pcaption : windows_sys::core::PCWSTR, dwtype : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn PrinterProperties(hwnd : super::super::Foundation::HWND, hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn ProvidorFindClosePrinterChangeNotification(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn ProvidorFindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwflags : u32, fdwoptions : u32, hnotify : super::super::Foundation::HANDLE, pprinternotifyoptions : *const core::ffi::c_void, pvreserved1 : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn ReadPrinter(hprinter : PRINTER_HANDLE, pbuf : *mut core::ffi::c_void, cbbuf : u32, pnobytesread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn RegisterForPrintAsyncNotifications(pszname : windows_sys::core::PCWSTR, pnotificationtype : *const windows_sys::core::GUID, euserfilter : PrintAsyncNotifyUserFilter, econversationstyle : PrintAsyncNotifyConversationStyle, pcallback : *mut core::ffi::c_void, phnotify : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("spoolss.dll" "system" fn RemovePrintDeviceObject(hdeviceobject : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("spoolss.dll" "system" fn ReplyPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwchangeflags : u32, pdwresult : *mut u32, pprinternotifyinfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn ReplyPrinterChangeNotificationEx(hnotify : super::super::Foundation::HANDLE, dwcolor : u32, fdwflags : u32, pdwresult : *mut u32, pprinternotifyinfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn ReportJobProcessingProgress(printerhandle : super::super::Foundation::HANDLE, jobid : u32, joboperation : EPrintXPSJobOperation, jobprogress : EPrintXPSJobProgress) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn ResetPrinterA(hprinter : PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winspool.drv" "system" fn ResetPrinterW(hprinter : PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn RevertToPrinterSelf() -> super::super::Foundation::HANDLE);
windows_link::link!("spoolss.dll" "system" fn RouterAllocBidiMem(numbytes : usize) -> *mut core::ffi::c_void);
windows_link::link!("spoolss.dll" "system" fn RouterAllocBidiResponseContainer(count : u32) -> *mut BIDI_RESPONSE_CONTAINER);
windows_link::link!("spoolss.dll" "system" fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata : u32) -> *mut PRINTER_NOTIFY_INFO);
windows_link::link!("spoolss.dll" "system" fn RouterCreatePrintAsyncNotificationChannel(pname : windows_sys::core::PCWSTR, pnotificationtype : *const windows_sys::core::GUID, enotifyfilter : PrintAsyncNotifyUserFilter, econversationstyle : PrintAsyncNotifyConversationStyle, pcallback : *mut core::ffi::c_void, ppiasynchnotification : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("spoolss.dll" "system" fn RouterFreeBidiMem(pmempointer : *const core::ffi::c_void));
windows_link::link!("winspool.drv" "system" fn RouterFreeBidiResponseContainer(pdata : *const BIDI_RESPONSE_CONTAINER) -> u32);
windows_link::link!("spoolss.dll" "system" fn RouterFreePrinterNotifyInfo(pinfo : *const PRINTER_NOTIFY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn RouterGetPrintClassObject(pprinter : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("spoolss.dll" "system" fn RouterUnregisterForPrintAsyncNotifications(hnotify : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn ScheduleJob(hprinter : PRINTER_HANDLE, jobid : u32) -> windows_sys::core::BOOL);
windows_link::link!("compstui.dll" "system" fn SetCPSUIUserData(hdlg : super::super::Foundation::HWND, cpsuiuserdata : usize) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterA(pszprinter : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterW(pszprinter : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetFormA(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCSTR, level : u32, pform : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetFormW(hprinter : PRINTER_HANDLE, pformname : windows_sys::core::PCWSTR, level : u32, pform : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetJobA(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *const u8, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetJobNamedProperty(hprinter : PRINTER_HANDLE, jobid : u32, pproperty : *const PrintNamedProperty) -> u32);
windows_link::link!("winspool.drv" "system" fn SetJobW(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *const u8, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPortA(pname : windows_sys::core::PCSTR, pportname : windows_sys::core::PCSTR, dwlevel : u32, pportinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPortW(pname : windows_sys::core::PCWSTR, pportname : windows_sys::core::PCWSTR, dwlevel : u32, pportinfo : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPrinterA(hprinter : PRINTER_HANDLE, level : u32, pprinter : *const u8, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetPrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn SetPrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_sys::core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn SetPrinterW(hprinter : PRINTER_HANDLE, level : u32, pprinter : *const u8, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn SplIsSessionZero(hprinter : PRINTER_HANDLE, jobid : u32, pissessionzero : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("spoolss.dll" "system" fn SplPromptUIInUsersSession(hprinter : PRINTER_HANDLE, jobid : u32, puiparams : *const SHOWUIPARAMS, presponse : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn SpoolerCopyFileEvent(pszprintername : windows_sys::core::PCWSTR, pszkey : windows_sys::core::PCWSTR, dwcopyfileevent : u32) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn SpoolerFindClosePrinterChangeNotification(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn SpoolerFindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwfilterflags : u32, fdwoptions : u32, pprinternotifyoptions : *const core::ffi::c_void, pvreserved : *const core::ffi::c_void, pnotificationconfig : *const core::ffi::c_void, phnotify : *mut super::super::Foundation::HANDLE, phevent : *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn SpoolerFindNextPrinterChangeNotification(hprinter : PRINTER_HANDLE, pfdwchange : *mut u32, pprinternotifyoptions : *const core::ffi::c_void, ppprinternotifyinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("spoolss.dll" "system" fn SpoolerFreePrinterNotifyInfo(pinfo : *const PRINTER_NOTIFY_INFO));
windows_link::link!("spoolss.dll" "system" fn SpoolerRefreshPrinterChangeNotification(hprinter : PRINTER_HANDLE, dwcolor : u32, poptions : *const PRINTER_NOTIFY_OPTIONS, ppinfo : *mut *mut PRINTER_NOTIFY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn StartDocPrinterA(hprinter : PRINTER_HANDLE, level : u32, pdocinfo : *const DOC_INFO_1A) -> u32);
windows_link::link!("winspool.drv" "system" fn StartDocPrinterW(hprinter : PRINTER_HANDLE, level : u32, pdocinfo : *const DOC_INFO_1W) -> u32);
windows_link::link!("winspool.drv" "system" fn StartPagePrinter(hprinter : PRINTER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn UnRegisterForPrintAsyncNotifications(param0 : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("spoolss.dll" "system" fn UpdatePrintDeviceObject(hprinter : PRINTER_HANDLE, hdeviceobject : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : u32, hwnd : super::super::Foundation::HWND, pszdestinfpath : windows_sys::core::PSTR, pcchdestinfpath : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : u32, hwnd : super::super::Foundation::HWND, pszdestinfpath : windows_sys::core::PWSTR, pcchdestinfpath : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn WaitForPrinterChange(hprinter : PRINTER_HANDLE, flags : u32) -> u32);
windows_link::link!("winspool.drv" "system" fn WritePrinter(hprinter : PRINTER_HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn XcvDataW(hxcv : super::super::Foundation::HANDLE, pszdataname : windows_sys::core::PCWSTR, pinputdata : *const u8, cbinputdata : u32, poutputdata : *mut u8, cboutputdata : u32, pcboutputneeded : *mut u32, pdwstatus : *mut u32) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADDJOB_INFO_1W {
    pub Path: windows_sys::core::PWSTR,
    pub JobId: u32,
}
impl Default for ADDJOB_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ALREADY_REGISTERED: PrintAsyncNotifyError = 15;
pub const ALREADY_UNREGISTERED: PrintAsyncNotifyError = 14;
pub const APD_COPY_ALL_FILES: u32 = 4;
pub const APD_COPY_FROM_DIRECTORY: u32 = 16;
pub const APD_COPY_NEW_FILES: u32 = 8;
pub const APD_STRICT_DOWNGRADE: u32 = 2;
pub const APD_STRICT_UPGRADE: u32 = 1;
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1;
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2;
pub const ASYNC_CALL_ALREADY_PARKED: PrintAsyncNotifyError = 12;
pub const ASYNC_CALL_IN_PROGRESS: PrintAsyncNotifyError = 17;
pub const ASYNC_NOTIFICATION_FAILURE: PrintAsyncNotifyError = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_1 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_2 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_3 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_4 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
    pub dwDuplexFlags: u32,
    pub dwNupDirection: u32,
    pub dwBookletFlags: u32,
    pub dwScalingPercentX: u32,
    pub dwScalingPercentY: u32,
}
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
#[derive(Clone, Copy)]
pub struct BIDI_DATA {
    pub dwBidiType: u32,
    pub u: BIDI_DATA_0,
}
impl Default for BIDI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BIDI_DATA_0 {
    pub bData: windows_sys::core::BOOL,
    pub iData: i32,
    pub sData: windows_sys::core::PWSTR,
    pub fData: f32,
    pub biData: BINARY_CONTAINER,
}
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
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
impl Default for BIDI_REQUEST_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: u32,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
impl Default for BIDI_REQUEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
impl Default for BIDI_RESPONSE_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: u32,
    pub dwReqNumber: u32,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
impl Default for BIDI_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_STRING: BIDI_TYPE = 4;
pub const BIDI_TEXT: BIDI_TYPE = 5;
pub type BIDI_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BINARY_CONTAINER {
    pub cbBuf: u32,
    pub pData: *mut u8,
}
impl Default for BINARY_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BOOKLET_EDGE_LEFT: u32 = 0;
pub const BOOKLET_EDGE_RIGHT: u32 = 1;
pub const BOOKLET_PRINT: u32 = 2;
pub const BORDER_PRINT: u32 = 0;
pub const BidiRequest: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb9162a23_45f9_47cc_80f5_fe0fe9b9e1a2);
pub const BidiRequestContainer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc5b8a24_db05_4a01_8388_22edf6c2bbba);
pub const BidiSpl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a614240_a4c5_4c33_bd87_1bc709331639);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BranchOfficeJobData {
    pub eEventType: EBranchOfficeJobEventType,
    pub JobId: u32,
    pub JobInfo: BranchOfficeJobData_0,
}
impl Default for BranchOfficeJobData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BranchOfficeJobData_0 {
    pub LogJobPrinted: BranchOfficeJobDataPrinted,
    pub LogJobRendered: BranchOfficeJobDataRendered,
    pub LogJobError: BranchOfficeJobDataError,
    pub LogPipelineFailed: BranchOfficeJobDataPipelineFailed,
    pub LogOfflineFileFull: BranchOfficeLogOfflineFileFull,
}
impl Default for BranchOfficeJobData_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BranchOfficeJobDataContainer {
    pub cJobDataEntries: u32,
    pub JobData: [BranchOfficeJobData; 1],
}
impl Default for BranchOfficeJobDataContainer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchOfficeJobDataError {
    pub LastError: u32,
    pub pDocumentName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pDataType: windows_sys::core::PWSTR,
    pub TotalSize: i64,
    pub PrintedSize: i64,
    pub TotalPages: u32,
    pub PrintedPages: u32,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pJobError: windows_sys::core::PWSTR,
    pub pErrorDescription: windows_sys::core::PWSTR,
}
impl Default for BranchOfficeJobDataError {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchOfficeJobDataPipelineFailed {
    pub pDocumentName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pExtraErrorInfo: windows_sys::core::PWSTR,
}
impl Default for BranchOfficeJobDataPipelineFailed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchOfficeJobDataPrinted {
    pub Status: u32,
    pub pDocumentName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub Size: i64,
    pub TotalPages: u32,
}
impl Default for BranchOfficeJobDataPrinted {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeJobDataRendered {
    pub Size: i64,
    pub ICMMethod: u32,
    pub Color: i16,
    pub PrintQuality: i16,
    pub YResolution: i16,
    pub Copies: i16,
    pub TTOption: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchOfficeLogOfflineFileFull {
    pub pMachineName: windows_sys::core::PWSTR,
}
impl Default for BranchOfficeLogOfflineFileFull {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CC_BIG5: i32 = -10;
pub const CC_CP437: i32 = -1;
pub const CC_CP850: i32 = -2;
pub const CC_CP863: i32 = -3;
pub const CC_DEFAULT: u32 = 0;
pub const CC_GB2312: i32 = -16;
pub const CC_ISC: i32 = -11;
pub const CC_JIS: i32 = -12;
pub const CC_JIS_ANK: i32 = -13;
pub const CC_NOPRECNV: u32 = 65535;
pub const CC_NS86: i32 = -14;
pub const CC_SJIS: i32 = -17;
pub const CC_TCA: i32 = -15;
pub const CC_WANSUNG: i32 = -18;
pub const CDM_CONVERT: u32 = 1;
pub const CDM_CONVERT351: u32 = 2;
pub const CDM_DRIVER_DEFAULT: u32 = 4;
pub const CHANNEL_ACQUIRED: PrintAsyncNotifyError = 16;
pub const CHANNEL_ALREADY_CLOSED: PrintAsyncNotifyError = 8;
pub const CHANNEL_ALREADY_OPENED: PrintAsyncNotifyError = 9;
pub const CHANNEL_CLOSED_BY_ANOTHER_LISTENER: PrintAsyncNotifyError = 2;
pub const CHANNEL_CLOSED_BY_SAME_LISTENER: PrintAsyncNotifyError = 3;
pub const CHANNEL_CLOSED_BY_SERVER: PrintAsyncNotifyError = 1;
pub const CHANNEL_NOT_OPENED: PrintAsyncNotifyError = 11;
pub const CHANNEL_RELEASED_BY_LISTENER: PrintAsyncNotifyError = 4;
pub const CHANNEL_WAITING_FOR_CLIENT_NOTIFICATION: PrintAsyncNotifyError = 10;
pub const CHKBOXS_FALSE_PDATA: u32 = 3;
pub const CHKBOXS_FALSE_TRUE: u32 = 0;
pub const CHKBOXS_NONE_PDATA: u32 = 6;
pub const CHKBOXS_NO_PDATA: u32 = 4;
pub const CHKBOXS_NO_YES: u32 = 1;
pub const CHKBOXS_OFF_ON: u32 = 2;
pub const CHKBOXS_OFF_PDATA: u32 = 5;
pub const CLSID_OEMPTPROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x91723892_45d2_48e2_9ec9_562379daf992);
pub const CLSID_OEMRENDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d6abf26_9f38_11d1_882a_00c04fb961ec);
pub const CLSID_OEMUI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xabce80d7_9f46_11d1_882a_00c04fb961ec);
pub const CLSID_OEMUIMXDC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e144300_5b43_4288_932a_5e4dd6d82bed);
pub const CLSID_PTPROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46ac151b_8490_4531_96cc_55bf2bf19e11);
pub const CLSID_XPSRASTERIZER_FACTORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x503e79bf_1d09_4764_9d72_1eb0c65967c6);
pub const COLOR_OPTIMIZATION: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug)]
pub struct COMPROPSHEETUI {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pCallerName: *mut i8,
    pub UserData: usize,
    pub pHelpFile: *mut i8,
    pub pfnCallBack: _CPSUICALLBACK,
    pub pOptItem: *mut OPTITEM,
    pub pDlgPage: *mut DLGPAGE,
    pub cOptItem: u16,
    pub cDlgPage: u16,
    pub IconID: usize,
    pub pOptItemName: *mut i8,
    pub CallerVersion: u16,
    pub OptItemVersion: u16,
    pub dwReserved: [usize; 4],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for COMPROPSHEETUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFIG_INFO_DATA_1 {
    pub Reserved: [u8; 128],
    pub dwVersion: u32,
}
impl Default for CONFIG_INFO_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COPYFILE_EVENT_ADD_PRINTER_CONNECTION: u32 = 3;
pub const COPYFILE_EVENT_DELETE_PRINTER: u32 = 2;
pub const COPYFILE_EVENT_DELETE_PRINTER_CONNECTION: u32 = 4;
pub const COPYFILE_EVENT_FILES_CHANGED: u32 = 5;
pub const COPYFILE_EVENT_SET_PRINTER_DATAEX: u32 = 1;
pub const COPYFILE_FLAG_CLIENT_SPOOLER: u32 = 1;
pub const COPYFILE_FLAG_SERVER_SPOOLER: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [i8; 260],
}
impl Default for CORE_PRINTER_DRIVERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [u16; 260],
}
impl Default for CORE_PRINTER_DRIVERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0;
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 3;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3;
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 5;
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4;
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5;
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 1;
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15;
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1;
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6;
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25;
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10;
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8;
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14;
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24;
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 17;
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16;
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17;
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13;
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 12;
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11;
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12;
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22;
pub const CPSFUNC_SET_DATABLOCK: u32 = 21;
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23;
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26;
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7;
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20;
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 19;
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18;
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19;
pub const CPSFUNC_SET_RESULT: u32 = 9;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct CPSUICBPARAM {
    pub cbSize: u16,
    pub Reason: u16,
    pub hDlg: super::super::Foundation::HWND,
    pub pOptItem: *mut OPTITEM,
    pub cOptItem: u16,
    pub Flags: u16,
    pub pCurItem: *mut OPTITEM,
    pub Anonymous: CPSUICBPARAM_0,
    pub UserData: usize,
    pub Result: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CPSUICBPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union CPSUICBPARAM_0 {
    pub OldSel: i32,
    pub pOldSel: *mut i8,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CPSUICBPARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4;
pub const CPSUICB_ACTION_NONE: u32 = 0;
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3;
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1;
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2;
pub const CPSUICB_REASON_ABOUT: u32 = 9;
pub const CPSUICB_REASON_APPLYNOW: u32 = 6;
pub const CPSUICB_REASON_DLGPROC: u32 = 3;
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2;
pub const CPSUICB_REASON_EXTPUSH: u32 = 5;
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8;
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11;
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7;
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1;
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0;
pub const CPSUICB_REASON_SETACTIVE: u32 = 10;
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CPSUIDATABLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4;
pub const CPSUIF_ICONID_AS_HICON: u32 = 2;
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1;
pub const CPSUI_CANCEL: u32 = 0;
pub const CPSUI_OK: u32 = 1;
pub const CPSUI_REBOOTSYSTEM: u32 = 3;
pub const CPSUI_RESTARTWINDOWS: u32 = 2;
pub const CUSTOMPARAM_HEIGHT: u32 = 1;
pub const CUSTOMPARAM_HEIGHTOFFSET: u32 = 3;
pub const CUSTOMPARAM_MAX: u32 = 5;
pub const CUSTOMPARAM_ORIENTATION: u32 = 4;
pub const CUSTOMPARAM_WIDTH: u32 = 0;
pub const CUSTOMPARAM_WIDTHOFFSET: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOMSIZEPARAM {
    pub dwOrder: i32,
    pub lMinVal: i32,
    pub lMaxVal: i32,
}
pub const Compression_Fast: EXpsCompressionOptions = 3;
pub const Compression_Normal: EXpsCompressionOptions = 1;
pub const Compression_NotCompressed: EXpsCompressionOptions = 0;
pub const Compression_Small: EXpsCompressionOptions = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DATATYPES_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for DATATYPES_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DATATYPES_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DATATYPES_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATA_HEADER {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wDataID: u16,
    pub dwDataSize: u32,
    pub dwReserved: u32,
}
pub const DEF_PRIORITY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DELETE_PORT_DATA_1 {
    pub psztPortName: [u16; 64],
    pub Reserved: [u8; 98],
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl Default for DELETE_PORT_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICEPROPERTYHEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
}
impl Default for DEVICEPROPERTYHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVQUERYPRINT_INFO {
    pub cbSize: u16,
    pub Level: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pszErrorStr: windows_sys::core::PWSTR,
    pub cchErrorStr: u32,
    pub cchNeeded: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DEVQUERYPRINT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DF_BKSP_OK: u32 = 64;
pub const DF_NOITALIC: u32 = 1;
pub const DF_NOUNDER: u32 = 2;
pub const DF_NO_BOLD: u32 = 8;
pub const DF_NO_DOUBLE_UNDERLINE: u32 = 16;
pub const DF_NO_STRIKETHRU: u32 = 32;
pub const DF_TYPE_CAPSL: u32 = 3;
pub const DF_TYPE_HPINTELLIFONT: u32 = 0;
pub const DF_TYPE_OEM1: u32 = 4;
pub const DF_TYPE_OEM2: u32 = 5;
pub const DF_TYPE_PST1: u32 = 2;
pub const DF_TYPE_TRUETYPE: u32 = 1;
pub const DF_XM_CR: u32 = 4;
pub const DISPID_PRINTEREXTENSION_CONTEXT: u32 = 11800;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION: u32 = 12100;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_COUNT: u32 = 12101;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_GETAT: u32 = 12102;
pub const DISPID_PRINTEREXTENSION_CONTEXT_DRIVERPROPERTIES: u32 = 11803;
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTERQUEUE: u32 = 11801;
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTSCHEMATICKET: u32 = 11802;
pub const DISPID_PRINTEREXTENSION_CONTEXT_USERPROPERTIES: u32 = 11804;
pub const DISPID_PRINTEREXTENSION_EVENT: u32 = 12200;
pub const DISPID_PRINTEREXTENSION_EVENTARGS: u32 = 12000;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_BIDINOTIFICATION: u32 = 12001;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_DETAILEDREASONID: u32 = 12005;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REASONID: u32 = 12002;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REQUEST: u32 = 12003;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_SOURCEAPPLICATION: u32 = 12004;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWMODAL: u32 = 12006;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWPARENT: u32 = 12007;
pub const DISPID_PRINTEREXTENSION_EVENT_ONDRIVEREVENT: u32 = 12201;
pub const DISPID_PRINTEREXTENSION_EVENT_ONPRINTERQUEUESENUMERATED: u32 = 12202;
pub const DISPID_PRINTEREXTENSION_REQUEST: u32 = 11900;
pub const DISPID_PRINTEREXTENSION_REQUEST_CANCEL: u32 = 11901;
pub const DISPID_PRINTEREXTENSION_REQUEST_COMPLETE: u32 = 11902;
pub const DISPID_PRINTERPROPERTYBAG: u32 = 11400;
pub const DISPID_PRINTERPROPERTYBAG_GETBOOL: u32 = 11401;
pub const DISPID_PRINTERPROPERTYBAG_GETBYTES: u32 = 11407;
pub const DISPID_PRINTERPROPERTYBAG_GETINT32: u32 = 11403;
pub const DISPID_PRINTERPROPERTYBAG_GETREADSTREAM: u32 = 11409;
pub const DISPID_PRINTERPROPERTYBAG_GETSTRING: u32 = 11405;
pub const DISPID_PRINTERPROPERTYBAG_GETWRITESTREAM: u32 = 11410;
pub const DISPID_PRINTERPROPERTYBAG_SETBOOL: u32 = 11402;
pub const DISPID_PRINTERPROPERTYBAG_SETBYTES: u32 = 11408;
pub const DISPID_PRINTERPROPERTYBAG_SETINT32: u32 = 11404;
pub const DISPID_PRINTERPROPERTYBAG_SETSTRING: u32 = 11406;
pub const DISPID_PRINTERQUEUE: u32 = 11600;
pub const DISPID_PRINTERQUEUEEVENT: u32 = 11700;
pub const DISPID_PRINTERQUEUEEVENT_ONBIDIRESPONSERECEIVED: u32 = 11701;
pub const DISPID_PRINTERQUEUEVIEW: u32 = 12700;
pub const DISPID_PRINTERQUEUEVIEW_EVENT: u32 = 12800;
pub const DISPID_PRINTERQUEUEVIEW_EVENT_ONCHANGED: u32 = 12801;
pub const DISPID_PRINTERQUEUEVIEW_SETVIEWRANGE: u32 = 12701;
pub const DISPID_PRINTERQUEUE_GETPRINTERQUEUEVIEW: u32 = 11606;
pub const DISPID_PRINTERQUEUE_GETPROPERTIES: u32 = 11604;
pub const DISPID_PRINTERQUEUE_HANDLE: u32 = 11601;
pub const DISPID_PRINTERQUEUE_NAME: u32 = 11602;
pub const DISPID_PRINTERQUEUE_SENDBIDIQUERY: u32 = 11603;
pub const DISPID_PRINTERQUEUE_SENDBIDISETREQUESTASYNC: u32 = 11605;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG: u32 = 11500;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBOOL: u32 = 11501;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBYTES: u32 = 11507;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETINT32: u32 = 11503;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETREADSTREAM: u32 = 11509;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTREAMASXML: u32 = 11411;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTRING: u32 = 11505;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETWRITESTREAM: u32 = 11510;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBOOL: u32 = 11502;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBYTES: u32 = 11508;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETINT32: u32 = 11504;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETSTRING: u32 = 11506;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM: u32 = 11200;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_READ: u32 = 11201;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_WRITE: u32 = 11202;
pub const DISPID_PRINTERSCRIPTABLESTREAM: u32 = 11300;
pub const DISPID_PRINTERSCRIPTABLESTREAM_COMMIT: u32 = 11301;
pub const DISPID_PRINTERSCRIPTABLESTREAM_SEEK: u32 = 11302;
pub const DISPID_PRINTERSCRIPTABLESTREAM_SETSIZE: u32 = 11303;
pub const DISPID_PRINTERSCRIPTCONTEXT: u32 = 12300;
pub const DISPID_PRINTERSCRIPTCONTEXT_DRIVERPROPERTIES: u32 = 12301;
pub const DISPID_PRINTERSCRIPTCONTEXT_QUEUEPROPERTIES: u32 = 12302;
pub const DISPID_PRINTERSCRIPTCONTEXT_USERPROPERTIES: u32 = 12303;
pub const DISPID_PRINTJOBCOLLECTION: u32 = 12600;
pub const DISPID_PRINTJOBCOLLECTION_COUNT: u32 = 12601;
pub const DISPID_PRINTJOBCOLLECTION_GETAT: u32 = 12602;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION: u32 = 10900;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT: u32 = 11100;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT_COMPLETED: u32 = 11101;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_CANCEL: u32 = 10902;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_START: u32 = 10901;
pub const DISPID_PRINTSCHEMA_CAPABILITIES: u32 = 10800;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE: u32 = 10802;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE_KEYNAME: u32 = 10801;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETOPTIONS: u32 = 10807;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETPARAMETERDEFINITION: u32 = 10808;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETSELECTEDOPTION: u32 = 10806;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMAXVALUE: u32 = 10805;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMINVALUE: u32 = 10804;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_PAGEIMAGEABLESIZE: u32 = 10803;
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT: u32 = 10100;
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT_DISPLAYNAME: u32 = 10101;
pub const DISPID_PRINTSCHEMA_ELEMENT: u32 = 10000;
pub const DISPID_PRINTSCHEMA_ELEMENT_NAME: u32 = 10002;
pub const DISPID_PRINTSCHEMA_ELEMENT_NAMESPACEURI: u32 = 10003;
pub const DISPID_PRINTSCHEMA_ELEMENT_XMLNODE: u32 = 10001;
pub const DISPID_PRINTSCHEMA_FEATURE: u32 = 10600;
pub const DISPID_PRINTSCHEMA_FEATURE_DISPLAYUI: u32 = 10604;
pub const DISPID_PRINTSCHEMA_FEATURE_GETOPTION: u32 = 10603;
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTEDOPTION: u32 = 10601;
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTIONTYPE: u32 = 10602;
pub const DISPID_PRINTSCHEMA_NUPOPTION: u32 = 10400;
pub const DISPID_PRINTSCHEMA_NUPOPTION_PAGESPERSHEET: u32 = 10401;
pub const DISPID_PRINTSCHEMA_OPTION: u32 = 10200;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION: u32 = 10500;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_COUNT: u32 = 10501;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_GETAT: u32 = 10502;
pub const DISPID_PRINTSCHEMA_OPTION_CONSTRAINED: u32 = 10202;
pub const DISPID_PRINTSCHEMA_OPTION_GETPROPERTYVALUE: u32 = 10203;
pub const DISPID_PRINTSCHEMA_OPTION_SELECTED: u32 = 10201;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE: u32 = 10700;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_HEIGHT: u32 = 10706;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_WIDTH: u32 = 10705;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_HEIGHT: u32 = 10702;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_WIDTH: u32 = 10701;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_HEIGHT: u32 = 10704;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_WIDTH: u32 = 10703;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION: u32 = 10300;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_HEIGHT: u32 = 10302;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_WIDTH: u32 = 10301;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION: u32 = 12500;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_DATATYPE: u32 = 12503;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMAX: u32 = 12505;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMIN: u32 = 12504;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_UNITTYPE: u32 = 12502;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_USERINPUTREQUIRED: u32 = 12501;
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER: u32 = 12400;
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER_VALUE: u32 = 12401;
pub const DISPID_PRINTSCHEMA_TICKET: u32 = 11000;
pub const DISPID_PRINTSCHEMA_TICKET_COMMITASYNC: u32 = 11004;
pub const DISPID_PRINTSCHEMA_TICKET_GETCAPABILITIES: u32 = 11006;
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE: u32 = 11002;
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE_KEYNAME: u32 = 11001;
pub const DISPID_PRINTSCHEMA_TICKET_GETPARAMETERINITIALIZER: u32 = 11008;
pub const DISPID_PRINTSCHEMA_TICKET_JOBCOPIESALLDOCUMENTS: u32 = 11007;
pub const DISPID_PRINTSCHEMA_TICKET_NOTIFYXMLCHANGED: u32 = 11005;
pub const DISPID_PRINTSCHEMA_TICKET_VALIDATEASYNC: u32 = 11003;
pub const DI_CHANNEL: u32 = 1;
pub const DI_MEMORYMAP_WRITE: u32 = 1;
pub const DI_READ_SPOOL_JOB: u32 = 3;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pTabName: *mut i8,
    pub IconID: usize,
    pub Anonymous: DLGPAGE_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for DLGPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union DLGPAGE_0 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for DLGPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMPUB_BOOKLET_EDGE: u32 = 21;
pub const DMPUB_COLOR: u32 = 6;
pub const DMPUB_COPIES_COLLATE: u32 = 3;
pub const DMPUB_DEFSOURCE: u32 = 4;
pub const DMPUB_DITHERTYPE: u32 = 13;
pub const DMPUB_DUPLEX: u32 = 7;
pub const DMPUB_FIRST: u32 = 1;
pub const DMPUB_FORMNAME: u32 = 9;
pub const DMPUB_ICMINTENT: u32 = 11;
pub const DMPUB_ICMMETHOD: u32 = 10;
pub const DMPUB_LAST: u32 = 21;
pub const DMPUB_MANUAL_DUPLEX: u32 = 19;
pub const DMPUB_MEDIATYPE: u32 = 12;
pub const DMPUB_NONE: u32 = 0;
pub const DMPUB_NUP: u32 = 16;
pub const DMPUB_NUP_DIRECTION: u32 = 18;
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98;
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97;
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99;
pub const DMPUB_ORIENTATION: u32 = 1;
pub const DMPUB_OUTPUTBIN: u32 = 14;
pub const DMPUB_PAGEORDER: u32 = 17;
pub const DMPUB_PRINTQUALITY: u32 = 5;
pub const DMPUB_QUALITY: u32 = 15;
pub const DMPUB_SCALE: u32 = 2;
pub const DMPUB_STAPLE: u32 = 20;
pub const DMPUB_TTOPTION: u32 = 8;
pub const DMPUB_USER: u32 = 100;
pub const DM_ADVANCED: u32 = 16;
pub const DM_INVALIDATE_DRIVER_CACHE: u32 = 536870912;
pub const DM_NOPERMISSION: u32 = 32;
pub const DM_PROMPT_NON_MODAL: u32 = 1073741824;
pub const DM_RESERVED: u32 = 2147483648;
pub const DM_USER_DEFAULT: u32 = 64;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_CREATEDCPRE {
    pub pszDriver: windows_sys::core::PWSTR,
    pub pszDevice: windows_sys::core::PWSTR,
    pub pdm: *mut super::Gdi::DEVMODEW,
    pub bIC: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DOCEVENT_CREATEDCPRE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_ESCAPE {
    pub iEscape: i32,
    pub cjInput: i32,
    pub pvInData: *mut core::ffi::c_void,
}
impl Default for DOCEVENT_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_FILTER {
    pub cbSize: u32,
    pub cElementsAllocated: u32,
    pub cElementsNeeded: u32,
    pub cElementsReturned: u32,
    pub aDocEventCall: [u32; 1],
}
impl Default for DOCEVENT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOCUMENTEVENT_ABORTDOC: u32 = 9;
pub const DOCUMENTEVENT_CREATEDCPOST: u32 = 2;
pub const DOCUMENTEVENT_CREATEDCPRE: u32 = 1;
pub const DOCUMENTEVENT_DELETEDC: u32 = 10;
pub const DOCUMENTEVENT_ENDDOC: u32 = 8;
pub const DOCUMENTEVENT_ENDDOCPOST: u32 = 12;
pub const DOCUMENTEVENT_ENDDOCPRE: u32 = 8;
pub const DOCUMENTEVENT_ENDPAGE: u32 = 7;
pub const DOCUMENTEVENT_ESCAPE: u32 = 11;
pub const DOCUMENTEVENT_FAILURE: i32 = -1;
pub const DOCUMENTEVENT_FIRST: u32 = 1;
pub const DOCUMENTEVENT_LAST: u32 = 15;
pub const DOCUMENTEVENT_QUERYFILTER: u32 = 14;
pub const DOCUMENTEVENT_RESETDCPOST: u32 = 4;
pub const DOCUMENTEVENT_RESETDCPRE: u32 = 3;
pub const DOCUMENTEVENT_SPOOLED: u32 = 65536;
pub const DOCUMENTEVENT_STARTDOC: u32 = 5;
pub const DOCUMENTEVENT_STARTDOCPOST: u32 = 13;
pub const DOCUMENTEVENT_STARTDOCPRE: u32 = 5;
pub const DOCUMENTEVENT_STARTPAGE: u32 = 6;
pub const DOCUMENTEVENT_SUCCESS: u32 = 1;
pub const DOCUMENTEVENT_UNSUPPORTED: u32 = 0;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST: u32 = 5;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE: u32 = 2;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST: u32 = 11;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE: u32 = 8;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST: u32 = 13;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE: u32 = 1;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST: u32 = 12;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE: u32 = 7;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE: u32 = 3;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST: u32 = 4;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST: u32 = 10;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE: u32 = 9;
pub const DOCUMENTEVENT_XPS_CANCELJOB: u32 = 6;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCUMENTPROPERTYHEADER {
    pub cbSize: u16,
    pub Reserved: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
    pub pdmIn: *mut super::Gdi::DEVMODEA,
    pub pdmOut: *mut super::Gdi::DEVMODEA,
    pub cbOut: u32,
    pub fMode: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DOCUMENTPROPERTYHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOC_INFO_INTERNAL {
    pub pDocName: *mut i8,
    pub pOutputFile: *mut i8,
    pub pDatatype: *mut i8,
    pub bLowILJob: windows_sys::core::BOOL,
    pub hTokenLowIL: super::super::Foundation::HANDLE,
}
impl Default for DOC_INFO_INTERNAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOC_INFO_INTERNAL_LEVEL: u32 = 100;
pub const DONT_SEND_EXTRA_PAGES_FOR_DUPLEX: u32 = 2;
pub const DOWN_THEN_LEFT: u32 = 8;
pub const DOWN_THEN_RIGHT: u32 = 2;
pub const DPD_DELETE_ALL_FILES: u32 = 4;
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2;
pub const DPD_DELETE_UNUSED_FILES: u32 = 1;
pub const DPF_ICONID_AS_HICON: u32 = 1;
pub const DPF_USE_HDLGTEMPLATE: u32 = 2;
pub const DPS_NOPERMISSION: u32 = 1;
pub const DP_STD_DOCPROPPAGE1: u32 = 65533;
pub const DP_STD_DOCPROPPAGE2: u32 = 65534;
pub const DP_STD_RESERVED_START: u32 = 65520;
pub const DP_STD_TREEVIEWPAGE: u32 = 65535;
pub const DRIVER_EVENT_DELETE: u32 = 2;
pub const DRIVER_EVENT_INITIALIZE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_sys::core::PSTR,
    pub pszOEMUrl: windows_sys::core::PSTR,
    pub pszHardwareID: windows_sys::core::PSTR,
    pub pszProvider: windows_sys::core::PSTR,
}
impl Default for DRIVER_INFO_6A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_sys::core::PWSTR,
    pub pszOEMUrl: windows_sys::core::PWSTR,
    pub pszHardwareID: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
}
impl Default for DRIVER_INFO_6W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
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
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
impl Default for DRIVER_INFO_8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
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
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
impl Default for DRIVER_INFO_8W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRIVER_KERNELMODE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_UPGRADE_INFO_1 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
}
impl Default for DRIVER_UPGRADE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_UPGRADE_INFO_2 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
    pub cVersion: u32,
    pub pName: *mut i8,
    pub pEnvironment: *mut i8,
    pub pDriverPath: *mut i8,
    pub pDataFile: *mut i8,
    pub pConfigFile: *mut i8,
    pub pHelpFile: *mut i8,
    pub pDependentFiles: *mut i8,
    pub pMonitorName: *mut i8,
    pub pDefaultDataType: *mut i8,
    pub pszzPreviousNames: *mut i8,
}
impl Default for DRIVER_UPGRADE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRIVER_USERMODE: u32 = 2;
pub const DSPRINT_PENDING: u32 = 2147483648;
pub const DSPRINT_PUBLISH: u32 = 1;
pub const DSPRINT_REPUBLISH: u32 = 8;
pub const DSPRINT_UNPUBLISH: u32 = 4;
pub const DSPRINT_UPDATE: u32 = 2;
pub type EATTRIBUTE_DATATYPE = i32;
pub type EBranchOfficeJobEventType = i32;
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1;
pub const ECBF_CHECKNAME_ONLY: u32 = 128;
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2;
pub const ECBF_ICONID_AS_HICON: u32 = 4;
pub const ECBF_MASK: u32 = 255;
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16;
pub const ECBF_OVERLAY_NO_ICON: u32 = 64;
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32;
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type EMFPLAYPROC = Option<unsafe extern "system" fn(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::HANDLE) -> i32>;
pub const EMF_PP_COLOR_OPTIMIZATION: u32 = 1;
pub const EPF_ICONID_AS_HICON: u32 = 8;
pub const EPF_INCL_SETUP_TITLE: u32 = 2;
pub const EPF_MASK: u32 = 255;
pub const EPF_NO_DOT_DOT_DOT: u32 = 4;
pub const EPF_OVERLAY_NO_ICON: u32 = 64;
pub const EPF_OVERLAY_STOP_ICON: u32 = 32;
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16;
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1;
pub const EPF_USE_HDLGTEMPLATE: u32 = 128;
pub type EPrintPropertyType = i32;
pub type EPrintXPSJobOperation = i32;
pub type EPrintXPSJobProgress = i32;
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014;
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004;
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000;
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012;
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013;
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011;
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
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2;
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10;
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33;
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31;
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32;
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43;
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999;
pub const ERR_CPSUI_GETLASTERROR: i32 = -1;
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000;
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16;
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14;
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29;
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30;
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26;
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25;
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24;
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39;
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35;
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4;
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19;
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23;
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20;
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21;
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3;
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13;
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38;
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34;
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12;
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11;
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41;
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8;
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6;
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28;
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27;
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42;
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40;
pub const ERR_CPSUI_NULL_HINST: i32 = -5;
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7;
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18;
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22;
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17;
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36;
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15;
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9;
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTCHKBOX {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub pSeparator: *mut i8,
    pub pCheckedName: *mut i8,
    pub IconID: usize,
    pub wReserved: [u16; 4],
    pub dwReserved: [usize; 2],
}
impl Default for EXTCHKBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct EXTPUSH {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub Anonymous1: EXTPUSH_0,
    pub IconID: usize,
    pub Anonymous2: EXTPUSH_1,
    pub dwReserved: [usize; 3],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union EXTPUSH_0 {
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pfnCallBack: super::super::Foundation::FARPROC,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union EXTPUSH_1 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXTTEXTMETRIC {
    pub emSize: i16,
    pub emPointSize: i16,
    pub emOrientation: i16,
    pub emMasterHeight: i16,
    pub emMinScale: i16,
    pub emMaxScale: i16,
    pub emMasterUnits: i16,
    pub emCapHeight: i16,
    pub emXHeight: i16,
    pub emLowerCaseAscent: i16,
    pub emLowerCaseDescent: i16,
    pub emSlant: i16,
    pub emSuperScript: i16,
    pub emSubScript: i16,
    pub emSuperScriptSize: i16,
    pub emSubScriptSize: i16,
    pub emUnderlineOffset: i16,
    pub emUnderlineWidth: i16,
    pub emDoubleUpperUnderlineOffset: i16,
    pub emDoubleLowerUnderlineOffset: i16,
    pub emDoubleUpperUnderlineWidth: i16,
    pub emDoubleLowerUnderlineWidth: i16,
    pub emStrikeOutOffset: i16,
    pub emStrikeOutWidth: i16,
    pub emKernPairs: u16,
    pub emKernTracks: u16,
}
pub type EXpsCompressionOptions = i32;
pub type EXpsFontOptions = i32;
pub type EXpsFontRestriction = i32;
pub type EXpsJobConsumption = i32;
pub const E_VERSION_NOT_SUPPORTED: u32 = 2147745793;
pub const FG_CANCHANGE: u32 = 128;
pub const FILL_WITH_DEFAULTS: u32 = 1;
pub type FINDPRINTERCHANGENOTIFICATION_HANDLE = *mut core::ffi::c_void;
pub const FMTID_PrinterPropertyBag: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x75f9adca_097d_45c3_a6e4_bab29e276f3e);
pub const FNT_INFO_CURRENTFONTID: u32 = 10;
pub const FNT_INFO_FONTBOLD: u32 = 6;
pub const FNT_INFO_FONTHEIGHT: u32 = 4;
pub const FNT_INFO_FONTITALIC: u32 = 7;
pub const FNT_INFO_FONTMAXWIDTH: u32 = 13;
pub const FNT_INFO_FONTSTRIKETHRU: u32 = 9;
pub const FNT_INFO_FONTUNDERLINE: u32 = 8;
pub const FNT_INFO_FONTWIDTH: u32 = 5;
pub const FNT_INFO_GRAYPERCENTAGE: u32 = 1;
pub const FNT_INFO_MAX: u32 = 14;
pub const FNT_INFO_NEXTFONTID: u32 = 2;
pub const FNT_INFO_NEXTGLYPH: u32 = 3;
pub const FNT_INFO_PRINTDIRINCCDEGREES: u32 = 0;
pub const FNT_INFO_TEXTXRES: u32 = 12;
pub const FNT_INFO_TEXTYRES: u32 = 11;
pub const FONT_DIR_SORTED: u32 = 1;
pub const FONT_FL_DEVICEFONT: u32 = 16;
pub const FONT_FL_GLYPHSET_GTT: u32 = 32;
pub const FONT_FL_GLYPHSET_RLE: u32 = 64;
pub const FONT_FL_IFI: u32 = 2;
pub const FONT_FL_PERMANENT_SF: u32 = 8;
pub const FONT_FL_RESERVED: u32 = 32768;
pub const FONT_FL_SOFTFONT: u32 = 4;
pub const FONT_FL_UFM: u32 = 1;
pub const FORM_BUILTIN: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORM_INFO_1A {
    pub Flags: u32,
    pub pName: windows_sys::core::PSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
impl Default for FORM_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORM_INFO_1W {
    pub Flags: u32,
    pub pName: windows_sys::core::PWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
impl Default for FORM_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORM_INFO_2A {
    pub Flags: u32,
    pub pName: windows_sys::core::PCSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_sys::core::PCSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_sys::core::PCSTR,
    pub wLangId: u16,
}
impl Default for FORM_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORM_INFO_2W {
    pub Flags: u32,
    pub pName: windows_sys::core::PCWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_sys::core::PCWSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_sys::core::PCWSTR,
    pub wLangId: u16,
}
impl Default for FORM_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FORM_PRINTER: u32 = 2;
pub const FORM_USER: u32 = 0;
pub const FinalPageCount: PageCountType = 0;
pub const Font_Normal: EXpsFontOptions = 0;
pub const Font_Obfusticate: EXpsFontOptions = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLYPHRUN {
    pub wcLow: u16,
    pub wGlyphCount: u16,
}
pub const GPD_OEMCUSTOMDATA: u32 = 1;
pub const GUID_DEVINTERFACE_IPPUSB_PRINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf2f40381_f46d_4e51_bce7_62de6cf2d098);
pub const GUID_DEVINTERFACE_USBPRINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x28d78fad_5a12_11d1_ae5b_0000f803a8c2);
pub const IDI_CPSUI_ADVANCE: u32 = 64058;
pub const IDI_CPSUI_AUTOSEL: u32 = 64025;
pub const IDI_CPSUI_COLLATE: u32 = 64030;
pub const IDI_CPSUI_COLOR: u32 = 64040;
pub const IDI_CPSUI_COPY: u32 = 64046;
pub const IDI_CPSUI_DEVICE: u32 = 64060;
pub const IDI_CPSUI_DEVICE2: u32 = 64061;
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080;
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042;
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043;
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044;
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041;
pub const IDI_CPSUI_DOCUMENT: u32 = 64059;
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032;
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085;
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031;
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084;
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033;
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086;
pub const IDI_CPSUI_EMPTY: u32 = 64000;
pub const IDI_CPSUI_ENVELOPE: u32 = 64010;
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097;
pub const IDI_CPSUI_ERROR: u32 = 64050;
pub const IDI_CPSUI_FALSE: u32 = 64005;
pub const IDI_CPSUI_FAX: u32 = 64095;
pub const IDI_CPSUI_FONTCART: u32 = 64013;
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012;
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098;
pub const IDI_CPSUI_FONTSUB: u32 = 64081;
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076;
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073;
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072;
pub const IDI_CPSUI_GRAPHIC: u32 = 64057;
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048;
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047;
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017;
pub const IDI_CPSUI_HT_HOST: u32 = 64016;
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053;
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052;
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051;
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000;
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111;
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078;
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107;
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099;
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105;
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087;
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089;
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094;
pub const IDI_CPSUI_MEM: u32 = 64011;
pub const IDI_CPSUI_MONO: u32 = 64039;
pub const IDI_CPSUI_NO: u32 = 64003;
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069;
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111;
pub const IDI_CPSUI_OFF: u32 = 64007;
pub const IDI_CPSUI_ON: u32 = 64008;
pub const IDI_CPSUI_OPTION: u32 = 64066;
pub const IDI_CPSUI_OPTION2: u32 = 64067;
pub const IDI_CPSUI_OUTBIN: u32 = 64055;
pub const IDI_CPSUI_OUTPUT: u32 = 64056;
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096;
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009;
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026;
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027;
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028;
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092;
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093;
pub const IDI_CPSUI_PORTRAIT: u32 = 64022;
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082;
pub const IDI_CPSUI_PRINTER: u32 = 64062;
pub const IDI_CPSUI_PRINTER2: u32 = 64063;
pub const IDI_CPSUI_PRINTER3: u32 = 64064;
pub const IDI_CPSUI_PRINTER4: u32 = 64065;
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079;
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077;
pub const IDI_CPSUI_QUESTION: u32 = 64075;
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034;
pub const IDI_CPSUI_RES_HIGH: u32 = 64037;
pub const IDI_CPSUI_RES_LOW: u32 = 64035;
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036;
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038;
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091;
pub const IDI_CPSUI_ROT_LAND: u32 = 64024;
pub const IDI_CPSUI_ROT_PORT: u32 = 64110;
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074;
pub const IDI_CPSUI_SCALING: u32 = 64045;
pub const IDI_CPSUI_SEL_NONE: u32 = 64001;
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088;
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090;
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015;
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014;
pub const IDI_CPSUI_STD_FORM: u32 = 64054;
pub const IDI_CPSUI_STOP: u32 = 64068;
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071;
pub const IDI_CPSUI_TELEPHONE: u32 = 64083;
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029;
pub const IDI_CPSUI_TRUE: u32 = 64006;
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019;
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020;
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018;
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021;
pub const IDI_CPSUI_WARNING: u32 = 64002;
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070;
pub const IDI_CPSUI_WATERMARK: u32 = 64049;
pub const IDI_CPSUI_YES: u32 = 64004;
pub const IDS_CPSUI_ABOUT: u32 = 64848;
pub const IDS_CPSUI_ADVANCED: u32 = 64722;
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716;
pub const IDS_CPSUI_ALL: u32 = 64841;
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718;
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857;
pub const IDS_CPSUI_BOND: u32 = 64786;
pub const IDS_CPSUI_BOOKLET: u32 = 64873;
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888;
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889;
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890;
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810;
pub const IDS_CPSUI_CHANGE: u32 = 64702;
pub const IDS_CPSUI_CHANGED: u32 = 64846;
pub const IDS_CPSUI_CHANGES: u32 = 64845;
pub const IDS_CPSUI_COARSE: u32 = 64787;
pub const IDS_CPSUI_COLLATE: u32 = 64756;
pub const IDS_CPSUI_COLLATED: u32 = 64757;
pub const IDS_CPSUI_COLON_SEP: u32 = 64707;
pub const IDS_CPSUI_COLOR: u32 = 64764;
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744;
pub const IDS_CPSUI_COPIES: u32 = 64831;
pub const IDS_CPSUI_COPY: u32 = 64830;
pub const IDS_CPSUI_DEFAULT: u32 = 64732;
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714;
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811;
pub const IDS_CPSUI_DEVICE: u32 = 64842;
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725;
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852;
pub const IDS_CPSUI_DITHERING: u32 = 64752;
pub const IDS_CPSUI_DOCUMENT: u32 = 64715;
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882;
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880;
pub const IDS_CPSUI_DRAFT: u32 = 64759;
pub const IDS_CPSUI_DUPLEX: u32 = 64745;
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804;
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805;
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790;
pub const IDS_CPSUI_ERROR: u32 = 64733;
pub const IDS_CPSUI_EXIST: u32 = 64736;
pub const IDS_CPSUI_FALSE: u32 = 64726;
pub const IDS_CPSUI_FAST: u32 = 64838;
pub const IDS_CPSUI_FAX: u32 = 64835;
pub const IDS_CPSUI_FINE: u32 = 64788;
pub const IDS_CPSUI_FORMNAME: u32 = 64747;
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812;
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798;
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856;
pub const IDS_CPSUI_GLOSSY: u32 = 64783;
pub const IDS_CPSUI_GRAPHIC: u32 = 64720;
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765;
pub const IDS_CPSUI_HALFTONE: u32 = 64791;
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817;
pub const IDS_CPSUI_HIGH: u32 = 64762;
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768;
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792;
pub const IDS_CPSUI_ICM: u32 = 64748;
pub const IDS_CPSUI_ICMINTENT: u32 = 64750;
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749;
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776;
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781;
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780;
pub const IDS_CPSUI_ICM_NO: u32 = 64777;
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779;
pub const IDS_CPSUI_ICM_YES: u32 = 64778;
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818;
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754;
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809;
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808;
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712;
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708;
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823;
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881;
pub const IDS_CPSUI_LINEART: u32 = 64789;
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770;
pub const IDS_CPSUI_LOW: u32 = 64760;
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801;
pub const IDS_CPSUI_MAILBOX: u32 = 64829;
pub const IDS_CPSUI_MAKE: u32 = 64833;
pub const IDS_CPSUI_MANUALFEED: u32 = 64813;
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883;
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885;
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884;
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803;
pub const IDS_CPSUI_MEDIA: u32 = 64751;
pub const IDS_CPSUI_MEDIUM: u32 = 64761;
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802;
pub const IDS_CPSUI_MONOCHROME: u32 = 64766;
pub const IDS_CPSUI_MORE: u32 = 64701;
pub const IDS_CPSUI_NO: u32 = 64728;
pub const IDS_CPSUI_NONE: u32 = 64734;
pub const IDS_CPSUI_NOT: u32 = 64735;
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737;
pub const IDS_CPSUI_NO_NAME: u32 = 64850;
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740;
pub const IDS_CPSUI_NUP: u32 = 64864;
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891;
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892;
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878;
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867;
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869;
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865;
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870;
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868;
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866;
pub const IDS_CPSUI_OF: u32 = 64704;
pub const IDS_CPSUI_OFF: u32 = 64730;
pub const IDS_CPSUI_ON: u32 = 64731;
pub const IDS_CPSUI_ONLYONE: u32 = 64800;
pub const IDS_CPSUI_OPTION: u32 = 64703;
pub const IDS_CPSUI_OPTIONS: u32 = 64721;
pub const IDS_CPSUI_ORIENTATION: u32 = 64738;
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796;
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863;
pub const IDS_CPSUI_PAGEORDER: u32 = 64855;
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816;
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719;
pub const IDS_CPSUI_PERCENT: u32 = 64711;
pub const IDS_CPSUI_PLOT: u32 = 64836;
pub const IDS_CPSUI_PORTRAIT: u32 = 64753;
pub const IDS_CPSUI_POSTER: u32 = 64874;
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875;
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876;
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877;
pub const IDS_CPSUI_PRESENTATION: u32 = 64763;
pub const IDS_CPSUI_PRINT: u32 = 64834;
pub const IDS_CPSUI_PRINTER: u32 = 64717;
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814;
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815;
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758;
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742;
pub const IDS_CPSUI_PROPERTIES: u32 = 64713;
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861;
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860;
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862;
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859;
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858;
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705;
pub const IDS_CPSUI_REGULAR: u32 = 64785;
pub const IDS_CPSUI_RESET: u32 = 64840;
pub const IDS_CPSUI_RESOLUTION: u32 = 64743;
pub const IDS_CPSUI_REVERT: u32 = 64844;
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709;
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824;
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879;
pub const IDS_CPSUI_ROTATED: u32 = 64839;
pub const IDS_CPSUI_ROT_LAND: u32 = 64755;
pub const IDS_CPSUI_ROT_PORT: u32 = 64886;
pub const IDS_CPSUI_SCALING: u32 = 64739;
pub const IDS_CPSUI_SETTING: u32 = 64851;
pub const IDS_CPSUI_SETTINGS: u32 = 64843;
pub const IDS_CPSUI_SETUP: u32 = 64700;
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771;
pub const IDS_CPSUI_SIDE1: u32 = 64871;
pub const IDS_CPSUI_SIDE2: u32 = 64872;
pub const IDS_CPSUI_SIMPLEX: u32 = 64767;
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710;
pub const IDS_CPSUI_SLOT1: u32 = 64819;
pub const IDS_CPSUI_SLOT2: u32 = 64820;
pub const IDS_CPSUI_SLOT3: u32 = 64821;
pub const IDS_CPSUI_SLOT4: u32 = 64822;
pub const IDS_CPSUI_SLOW: u32 = 64837;
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807;
pub const IDS_CPSUI_SOURCE: u32 = 64741;
pub const IDS_CPSUI_STACKER: u32 = 64828;
pub const IDS_CPSUI_STANDARD: u32 = 64782;
pub const IDS_CPSUI_STAPLE: u32 = 64887;
pub const IDS_CPSUI_STAPLER: u32 = 64825;
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827;
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826;
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723;
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853;
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854;
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724;
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700;
pub const IDS_CPSUI_STRID_LAST: u32 = 64892;
pub const IDS_CPSUI_TO: u32 = 64706;
pub const IDS_CPSUI_TOTAL: u32 = 64832;
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806;
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784;
pub const IDS_CPSUI_TRUE: u32 = 64727;
pub const IDS_CPSUI_TTOPTION: u32 = 64746;
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773;
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774;
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772;
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775;
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799;
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794;
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793;
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795;
pub const IDS_CPSUI_VERSION: u32 = 64849;
pub const IDS_CPSUI_VERTICAL: u32 = 64769;
pub const IDS_CPSUI_WARNING: u32 = 64847;
pub const IDS_CPSUI_WATERMARK: u32 = 64797;
pub const IDS_CPSUI_YES: u32 = 64729;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1;
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0;
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2;
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4;
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3;
pub const INTERNAL_NOTIFICATION_QUEUE_IS_FULL: PrintAsyncNotifyError = 19;
pub const INVALID_NOTIFICATION_TYPE: PrintAsyncNotifyError = 20;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INVOC {
    pub dwCount: u32,
    pub loOffset: u32,
}
pub const IOCTL_USBPRINT_ADD_CHILD_DEVICE: u32 = 2228316;
pub const IOCTL_USBPRINT_ADD_MSIPP_COMPAT_ID: u32 = 2228308;
pub const IOCTL_USBPRINT_CYCLE_PORT: u32 = 2228320;
pub const IOCTL_USBPRINT_GET_1284_ID: u32 = 2228276;
pub const IOCTL_USBPRINT_GET_INTERFACE_TYPE: u32 = 2228300;
pub const IOCTL_USBPRINT_GET_LPT_STATUS: u32 = 2228272;
pub const IOCTL_USBPRINT_GET_MFG_MDL_ID: u32 = 2228324;
pub const IOCTL_USBPRINT_GET_PROTOCOL: u32 = 2228292;
pub const IOCTL_USBPRINT_SET_DEVICE_ID: u32 = 2228312;
pub const IOCTL_USBPRINT_SET_PORT_NUMBER: u32 = 2228304;
pub const IOCTL_USBPRINT_SET_PROTOCOL: u32 = 2228296;
pub const IOCTL_USBPRINT_SOFT_RESET: u32 = 2228288;
pub const IOCTL_USBPRINT_VENDOR_GET_COMMAND: u32 = 2228284;
pub const IOCTL_USBPRINT_VENDOR_SET_COMMAND: u32 = 2228280;
pub const IPDFP_COPY_ALL_FILES: u32 = 1;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgErrorInfo {
    pub description: core::mem::ManuallyDrop<windows_sys::core::BSTR>,
    pub guid: windows_sys::core::GUID,
    pub helpContext: u32,
    pub helpFile: core::mem::ManuallyDrop<windows_sys::core::BSTR>,
    pub source: core::mem::ManuallyDrop<windows_sys::core::BSTR>,
    pub devDescription: core::mem::ManuallyDrop<windows_sys::core::BSTR>,
    pub errorID: windows_sys::core::GUID,
    pub cUserParameters: u32,
    pub aUserParameters: *mut windows_sys::core::BSTR,
    pub userFallback: core::mem::ManuallyDrop<windows_sys::core::BSTR>,
    pub exceptionID: u32,
}
impl Default for ImgErrorInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IntermediatePageCount: PageCountType = 1;
pub const JOB_ACCESS_ADMINISTER: u32 = 16;
pub const JOB_ACCESS_READ: u32 = 32;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
impl Default for JOB_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
impl Default for JOB_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOB_INFO_3 {
    pub JobId: u32,
    pub NextJobId: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
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
#[repr(C)]
#[cfg(feature = "Win32_Devices_Display")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERNDATA {
    pub dwSize: u32,
    pub dwKernPairNum: u32,
    pub KernPair: [super::super::Devices::Display::FD_KERNINGPAIR; 1],
}
#[cfg(feature = "Win32_Devices_Display")]
impl Default for KERNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LEFT_THEN_DOWN: u32 = 4;
pub const LOCAL_ONLY_REGISTRATION: PrintAsyncNotifyError = 23;
pub const LPR: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPTABLE {
    pub dwSize: u32,
    pub dwGlyphNum: u32,
    pub Trans: [TRANSDATA; 1],
}
impl Default for MAPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_ADDRESS_STR_LEN: u32 = 13;
pub const MAX_CHANNEL_COUNT_EXCEEDED: PrintAsyncNotifyError = 22;
pub const MAX_CPSFUNC_INDEX: u32 = 26;
pub const MAX_DEVICEDESCRIPTION_STR_LEN: u32 = 257;
pub const MAX_DLGPAGE_COUNT: u32 = 64;
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64;
pub const MAX_IPADDR_STR_LEN: u32 = 16;
pub const MAX_NETWORKNAME2_LEN: u32 = 128;
pub const MAX_NETWORKNAME_LEN: u32 = 49;
pub const MAX_NOTIFICATION_SIZE_EXCEEDED: PrintAsyncNotifyError = 18;
pub const MAX_PORTNAME_LEN: u32 = 64;
pub const MAX_PRIORITY: u32 = 99;
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5;
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5;
pub const MAX_QUEUENAME_LEN: u32 = 33;
pub const MAX_REGISTRATION_COUNT_EXCEEDED: PrintAsyncNotifyError = 21;
pub const MAX_RES_STR_CHARS: u32 = 160;
pub const MAX_SNMP_COMMUNITY_STR_LEN: u32 = 33;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MESSAGEBOX_PARAMS {
    pub cbSize: u32,
    pub pTitle: windows_sys::core::PWSTR,
    pub pMessage: windows_sys::core::PWSTR,
    pub Style: u32,
    pub dwTimeout: u32,
    pub bWait: windows_sys::core::BOOL,
}
impl Default for MESSAGEBOX_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIN_PRIORITY: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOR {
    pub pfnEnumPorts: PFN_PRINTING_ENUMPORTS,
    pub pfnOpenPort: PFN_PRINTING_OPENPORT,
    pub pfnOpenPortEx: PFN_PRINTING_OPENPORTEX,
    pub pfnStartDocPort: PFN_PRINTING_STARTDOCPORT,
    pub pfnWritePort: PFN_PRINTING_WRITEPORT,
    pub pfnReadPort: PFN_PRINTING_READPORT,
    pub pfnEndDocPort: PFN_PRINTING_ENDDOCPORT,
    pub pfnClosePort: PFN_PRINTING_CLOSEPORT,
    pub pfnAddPort: PFN_PRINTING_ADDPORT,
    pub pfnAddPortEx: PFN_PRINTING_ADDPORTEX,
    pub pfnConfigurePort: PFN_PRINTING_CONFIGUREPORT,
    pub pfnDeletePort: PFN_PRINTING_DELETEPORT,
    pub pfnGetPrinterDataFromPort: PFN_PRINTING_GETPRINTERDATAFROMPORT,
    pub pfnSetPortTimeOuts: PFN_PRINTING_SETPORTTIMEOUTS,
    pub pfnXcvOpenPort: PFN_PRINTING_XCVOPENPORT,
    pub pfnXcvDataPort: PFN_PRINTING_XCVDATAPORT,
    pub pfnXcvClosePort: PFN_PRINTING_XCVCLOSEPORT,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOR2 {
    pub cbSize: u32,
    pub pfnEnumPorts: PFN_PRINTING_ENUMPORTS2,
    pub pfnOpenPort: PFN_PRINTING_OPENPORT2,
    pub pfnOpenPortEx: PFN_PRINTING_OPENPORTEX2,
    pub pfnStartDocPort: PFN_PRINTING_STARTDOCPORT2,
    pub pfnWritePort: PFN_PRINTING_WRITEPORT2,
    pub pfnReadPort: PFN_PRINTING_READPORT2,
    pub pfnEndDocPort: PFN_PRINTING_ENDDOCPORT2,
    pub pfnClosePort: PFN_PRINTING_CLOSEPORT2,
    pub pfnAddPort: PFN_PRINTING_ADDPORT2,
    pub pfnAddPortEx: PFN_PRINTING_ADDPORTEX2,
    pub pfnConfigurePort: PFN_PRINTING_CONFIGUREPORT2,
    pub pfnDeletePort: PFN_PRINTING_DELETEPORT2,
    pub pfnGetPrinterDataFromPort: PFN_PRINTING_GETPRINTERDATAFROMPORT2,
    pub pfnSetPortTimeOuts: PFN_PRINTING_SETPORTTIMEOUTS2,
    pub pfnXcvOpenPort: PFN_PRINTING_XCVOPENPORT2,
    pub pfnXcvDataPort: PFN_PRINTING_XCVDATAPORT2,
    pub pfnXcvClosePort: PFN_PRINTING_XCVCLOSEPORT2,
    pub pfnShutdown: PFN_PRINTING_SHUTDOWN2,
    pub pfnSendRecvBidiDataFromPort: PFN_PRINTING_SENDRECVBIDIDATAFROMPORT2,
    pub pfnNotifyUsedPorts: PFN_PRINTING_NOTIFYUSEDPORTS2,
    pub pfnNotifyUnusedPorts: PFN_PRINTING_NOTIFYUNUSEDPORTS2,
    pub pfnPowerEvent: PFN_PRINTING_POWEREVENT2,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOREX {
    pub dwMonitorSize: u32,
    pub Monitor: MONITOR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITORINIT {
    pub cbSize: u32,
    pub hSpooler: super::super::Foundation::HANDLE,
    pub hckRegistryRoot: super::super::System::Registry::HKEY,
    pub pMonitorReg: *mut MONITORREG,
    pub bLocal: windows_sys::core::BOOL,
    pub pszServerName: windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for MONITORINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORREG {
    pub cbSize: u32,
    pub fpCreateKey: isize,
    pub fpOpenKey: isize,
    pub fpCloseKey: isize,
    pub fpDeleteKey: isize,
    pub fpEnumKey: isize,
    pub fpQueryInfoKey: isize,
    pub fpSetValue: isize,
    pub fpDeleteValue: isize,
    pub fpEnumValue: isize,
    pub fpQueryValue: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORUI {
    pub dwMonitorUISize: u32,
    pub pfnAddPortUI: isize,
    pub pfnConfigurePortUI: isize,
    pub pfnDeletePortUI: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for MONITOR_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for MONITOR_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const MTYPE_ADD: u32 = 64;
pub const MTYPE_COMPOSE: u32 = 1;
pub const MTYPE_DIRECT: u32 = 2;
pub const MTYPE_DISABLE: u32 = 128;
pub const MTYPE_DOUBLE: u32 = 16;
pub const MTYPE_DOUBLEBYTECHAR_MASK: u32 = 24;
pub const MTYPE_FORMAT_MASK: u32 = 7;
pub const MTYPE_PAIRED: u32 = 4;
pub const MTYPE_PREDEFIN_MASK: u32 = 224;
pub const MTYPE_REPLACE: u32 = 32;
pub const MTYPE_SINGLE: u32 = 8;
pub const MV_GRAPHICS: u32 = 4;
pub const MV_PHYSICAL: u32 = 8;
pub const MV_RELATIVE: u32 = 2;
pub const MV_SENDXMOVECMD: u32 = 16;
pub const MV_SENDYMOVECMD: u32 = 32;
pub const MV_UPDATE: u32 = 1;
pub const MXDCOP_GET_FILENAME: u32 = 14;
pub const MXDCOP_PRINTTICKET_FIXED_DOC: u32 = 24;
pub const MXDCOP_PRINTTICKET_FIXED_DOC_SEQ: u32 = 22;
pub const MXDCOP_PRINTTICKET_FIXED_PAGE: u32 = 26;
pub const MXDCOP_SET_S0PAGE: u32 = 28;
pub const MXDCOP_SET_S0PAGE_RESOURCE: u32 = 30;
pub const MXDCOP_SET_XPSPASSTHRU_MODE: u32 = 32;
pub const MXDC_ESCAPE: u32 = 4122;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_ESCAPE_HEADER_T {
    pub cbInput: u32,
    pub cbOutput: u32,
    pub opCode: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_GET_FILENAME_DATA_T {
    pub cbOutput: u32,
    pub wszData: [u16; 1],
}
impl Default for MXDC_GET_FILENAME_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MXDC_IMAGETYPE_JPEGHIGH_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = 1;
pub const MXDC_IMAGETYPE_JPEGLOW_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = 3;
pub const MXDC_IMAGETYPE_JPEGMEDIUM_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = 2;
pub const MXDC_IMAGETYPE_PNG: MXDC_IMAGE_TYPE_ENUMS = 4;
pub type MXDC_IMAGE_TYPE_ENUMS = i32;
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_270_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = -90;
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_90_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = 90;
pub const MXDC_LANDSCAPE_ROTATE_NONE: MXDC_LANDSCAPE_ROTATION_ENUMS = 0;
pub type MXDC_LANDSCAPE_ROTATION_ENUMS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_PRINTTICKET_DATA_T {
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_PRINTTICKET_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_PRINTTICKET_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub printTicketData: MXDC_PRINTTICKET_DATA_T,
}
pub const MXDC_RESOURCE_DICTIONARY: MXDC_S0_PAGE_ENUMS = 5;
pub const MXDC_RESOURCE_ICC_PROFILE: MXDC_S0_PAGE_ENUMS = 6;
pub const MXDC_RESOURCE_JPEG: MXDC_S0_PAGE_ENUMS = 1;
pub const MXDC_RESOURCE_JPEG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = 7;
pub const MXDC_RESOURCE_MAX: MXDC_S0_PAGE_ENUMS = 9;
pub const MXDC_RESOURCE_PNG: MXDC_S0_PAGE_ENUMS = 2;
pub const MXDC_RESOURCE_PNG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = 8;
pub const MXDC_RESOURCE_TIFF: MXDC_S0_PAGE_ENUMS = 3;
pub const MXDC_RESOURCE_TTF: MXDC_S0_PAGE_ENUMS = 0;
pub const MXDC_RESOURCE_WDP: MXDC_S0_PAGE_ENUMS = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_S0PAGE_DATA_T {
    pub dwSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_S0PAGE_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageData: MXDC_S0PAGE_DATA_T,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageResourcePassthrough: MXDC_XPS_S0PAGE_RESOURCE_T,
}
pub type MXDC_S0_PAGE_ENUMS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_XPS_S0PAGE_RESOURCE_T {
    pub dwSize: u32,
    pub dwResourceType: u32,
    pub szUri: [u8; 260],
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_XPS_S0PAGE_RESOURCE_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NORMAL_PRINT: u32 = 0;
pub type NOTIFICATION_CALLBACK_COMMANDS = i32;
pub const NOTIFICATION_COMMAND_CONTEXT_ACQUIRE: NOTIFICATION_CALLBACK_COMMANDS = 1;
pub const NOTIFICATION_COMMAND_CONTEXT_RELEASE: NOTIFICATION_CALLBACK_COMMANDS = 2;
pub const NOTIFICATION_COMMAND_NOTIFY: NOTIFICATION_CALLBACK_COMMANDS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NOTIFICATION_CONFIG_1 {
    pub cbSize: u32,
    pub fdwFlags: u32,
    pub pfnNotifyCallback: ROUTER_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
}
impl Default for NOTIFICATION_CONFIG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NOTIFICATION_CONFIG_ASYNC_CHANNEL: NOTIFICATION_CONFIG_FLAGS = 8;
pub const NOTIFICATION_CONFIG_CREATE_EVENT: NOTIFICATION_CONFIG_FLAGS = 1;
pub const NOTIFICATION_CONFIG_EVENT_TRIGGER: NOTIFICATION_CONFIG_FLAGS = 4;
pub type NOTIFICATION_CONFIG_FLAGS = i32;
pub const NOTIFICATION_CONFIG_REGISTER_CALLBACK: NOTIFICATION_CONFIG_FLAGS = 2;
pub const NOTIFICATION_RELEASE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba9a5027_a70e_4ae7_9b7d_eb3e06ad4157);
pub const NOT_REGISTERED: PrintAsyncNotifyError = 13;
pub const NO_BORDER_PRINT: u32 = 1;
pub const NO_COLOR_OPTIMIZATION: u32 = 0;
pub const NO_LISTENERS: PrintAsyncNotifyError = 7;
pub const NO_PRIORITY: u32 = 0;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type OEMCUIPCALLBACK = Option<unsafe extern "system" fn(param0: *mut CPSUICBPARAM, param1: *mut OEMCUIPPARAM) -> i32>;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OEMCUIPPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub pDrvOptItems: *mut OPTITEM,
    pub cDrvOptItems: u32,
    pub pOEMOptItems: *mut OPTITEM,
    pub cOEMOptItems: u32,
    pub pOEMUserData: *mut core::ffi::c_void,
    pub OEMCUIPCallback: OEMCUIPCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OEMCUIPPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OEMCUIP_DOCPROP: u32 = 1;
pub const OEMCUIP_PRNPROP: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMDMPARAM {
    pub cbSize: u32,
    pub pdriverobj: *mut core::ffi::c_void,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub pPublicDMIn: *mut super::Gdi::DEVMODEA,
    pub pPublicDMOut: *mut super::Gdi::DEVMODEA,
    pub pOEMDMIn: *mut core::ffi::c_void,
    pub pOEMDMOut: *mut core::ffi::c_void,
    pub cbBufSize: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for OEMDMPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OEMDM_CONVERT: u32 = 3;
pub const OEMDM_DEFAULT: u32 = 2;
pub const OEMDM_MERGE: u32 = 4;
pub const OEMDM_SIZE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMFONTINSTPARAM {
    pub cbSize: u32,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub hHeap: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
    pub pFontInstallerName: windows_sys::core::PWSTR,
}
impl Default for OEMFONTINSTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OEMGDS_FREEMEM: u32 = 32769;
pub const OEMGDS_JOBTIMEOUT: u32 = 32770;
pub const OEMGDS_MAX: u32 = 65536;
pub const OEMGDS_MAXBITMAP: u32 = 32774;
pub const OEMGDS_MINOUTLINE: u32 = 32773;
pub const OEMGDS_MIN_DOCSTICKY: u32 = 1;
pub const OEMGDS_MIN_PRINTERSTICKY: u32 = 32768;
pub const OEMGDS_PRINTFLAGS: u32 = 32768;
pub const OEMGDS_PROTOCOL: u32 = 32772;
pub const OEMGDS_PSDM_CUSTOMSIZE: u32 = 6;
pub const OEMGDS_PSDM_DIALECT: u32 = 2;
pub const OEMGDS_PSDM_FLAGS: u32 = 1;
pub const OEMGDS_PSDM_NUP: u32 = 4;
pub const OEMGDS_PSDM_PSLEVEL: u32 = 5;
pub const OEMGDS_PSDM_TTDLFMT: u32 = 3;
pub const OEMGDS_UNIDM_FLAGS: u32 = 16385;
pub const OEMGDS_UNIDM_GPDVER: u32 = 16384;
pub const OEMGDS_WAITTIMEOUT: u32 = 32771;
pub const OEMGI_GETINTERFACEVERSION: u32 = 2;
pub const OEMGI_GETPUBLISHERINFO: u32 = 4;
pub const OEMGI_GETREQUESTEDHELPERINTERFACES: u32 = 5;
pub const OEMGI_GETSIGNATURE: u32 = 1;
pub const OEMGI_GETVERSION: u32 = 3;
pub const OEMPUBLISH_DEFAULT: u32 = 0;
pub const OEMPUBLISH_IPRINTCOREHELPER: u32 = 1;
pub const OEMTTY_INFO_CODEPAGE: u32 = 2;
pub const OEMTTY_INFO_MARGINS: u32 = 1;
pub const OEMTTY_INFO_NUM_UFMS: u32 = 3;
pub const OEMTTY_INFO_UFM_IDS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMUIOBJ {
    pub cbSize: u32,
    pub pOemUIProcs: *mut OEMUIPROCS,
}
impl Default for OEMUIOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OEMUIPROCS {
    pub DrvGetDriverSetting: PFN_DrvGetDriverSetting,
    pub DrvUpdateUISetting: PFN_DrvUpdateUISetting,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMUIPSPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut core::ffi::c_void,
    pub pOEMUserData: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub pOemEntry: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for OEMUIPSPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OEM_DMEXTRAHEADER {
    pub dwSize: u32,
    pub dwSignature: u32,
    pub dwVersion: u32,
}
pub const OEM_MODE_PUBLISHER: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pHelpFile: *mut i8,
    pub dwReserved: [usize; 4],
}
impl Default for OIEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OIEXTF_ANSI_STRING: u32 = 1;
pub const OPTCF_HIDE: u32 = 1;
pub const OPTCF_MASK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTCOMBO {
    pub cbSize: u16,
    pub Flags: u8,
    pub cListItem: u16,
    pub pListItem: *mut OPTPARAM,
    pub Sel: i32,
    pub dwReserved: [u32; 3],
}
impl Default for OPTCOMBO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTIF_CALLBACK: i32 = 4;
pub const OPTIF_CHANGED: i32 = 8;
pub const OPTIF_CHANGEONCE: i32 = 16;
pub const OPTIF_COLLAPSE: i32 = 1;
pub const OPTIF_DISABLED: i32 = 32;
pub const OPTIF_ECB_CHECKED: i32 = 64;
pub const OPTIF_EXT_DISABLED: i32 = 256;
pub const OPTIF_EXT_HIDE: i32 = 128;
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024;
pub const OPTIF_HAS_POIEXT: i32 = 65536;
pub const OPTIF_HIDE: i32 = 2;
pub const OPTIF_INITIAL_TVITEM: i32 = 32768;
pub const OPTIF_MASK: i32 = 131071;
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048;
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384;
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192;
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096;
pub const OPTIF_SEL_AS_HICON: i32 = 512;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct OPTITEM {
    pub cbSize: u16,
    pub Level: u8,
    pub DlgPageIdx: u8,
    pub Flags: u32,
    pub UserData: usize,
    pub pName: *mut i8,
    pub Anonymous1: OPTITEM_0,
    pub Anonymous2: OPTITEM_1,
    pub pOptType: *mut OPTTYPE,
    pub HelpIndex: u32,
    pub DMPubID: u8,
    pub UserItemID: u8,
    pub wReserved: u16,
    pub pOIExt: *mut OIEXT,
    pub dwReserved: [usize; 3],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union OPTITEM_0 {
    pub Sel: i32,
    pub pSel: *mut i8,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union OPTITEM_1 {
    pub pExtChkBox: *mut EXTCHKBOX,
    pub pExtPush: *mut EXTPUSH,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: *mut i8,
    pub IconID: usize,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReserved: [usize; 2],
}
impl Default for OPTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTPF_DISABLED: u32 = 2;
pub const OPTPF_HIDE: u32 = 1;
pub const OPTPF_ICONID_AS_HICON: u32 = 4;
pub const OPTPF_MASK: u32 = 127;
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32;
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16;
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8;
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64;
pub const OPTTF_MASK: u32 = 3;
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2;
pub const OPTTF_TYPE_DISABLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTTYPE {
    pub cbSize: u16,
    pub Type: u8,
    pub Flags: u8,
    pub Count: u16,
    pub BegCtrlID: u16,
    pub pOptParam: *mut OPTPARAM,
    pub Style: u16,
    pub wReserved: [u16; 3],
    pub dwReserved: [usize; 3],
}
impl Default for OPTTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8;
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16;
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4;
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2;
pub const OTS_LBCB_SORT: u32 = 1;
pub const OTS_MASK: u32 = 255;
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128;
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32;
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64;
pub const PDEV_ADJUST_GRAPHICS_RESOLUTION_TYPE: u32 = 4;
pub const PDEV_ADJUST_IMAGEABLE_ORIGIN_AREA_TYPE: u32 = 8;
pub const PDEV_ADJUST_PAPER_MARGIN_TYPE: u32 = 1;
pub const PDEV_ADJUST_PHYSICAL_PAPER_SIZE_TYPE: u32 = 16;
pub const PDEV_HOSTFONT_ENABLED_TYPE: u32 = 2;
pub const PDEV_USE_TRUE_COLOR_TYPE: u32 = 3;
pub type PFNCOMPROPSHEET = Option<unsafe extern "system" fn(hcompropsheet: super::super::Foundation::HANDLE, function: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> isize>;
pub type PFNPROPSHEETUI = Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> i32>;
pub type PFN_DrvGetDriverSetting = Option<unsafe extern "system" fn(pdriverobj: *mut core::ffi::c_void, feature: windows_sys::core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_DrvUpdateUISetting = Option<unsafe extern "system" fn(pdriverobj: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_sys::core::BOOL>;
pub type PFN_DrvUpgradeRegistrySetting = Option<unsafe extern "system" fn(hprinter: super::super::Foundation::HANDLE, pfeature: windows_sys::core::PCSTR, poption: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ADDPORT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ADDPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ADDPORTEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *const u8, param3: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ADDPORTEX2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: u32, param3: *const u8, param4: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_CLOSEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_CLOSEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_CONFIGUREPORT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_CONFIGUREPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_DELETEPORT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_DELETEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ENDDOCPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ENDDOCPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ENUMPORTS = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *mut u8, param3: u32, param4: *mut u32, param5: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_ENUMPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: u32, param3: *mut u8, param4: u32, param5: *mut u32, param6: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_GETPRINTERDATAFROMPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR, param4: u32, param5: windows_sys::core::PCWSTR, param6: u32, param7: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_GETPRINTERDATAFROMPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR, param4: u32, param5: windows_sys::core::PCWSTR, param6: u32, param7: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_NOTIFYUNUSEDPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_sys::core::PCWSTR) -> u32>;
pub type PFN_PRINTING_NOTIFYUSEDPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_sys::core::PCWSTR) -> u32>;
pub type PFN_PRINTING_OPENPORT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_OPENPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
pub type PFN_PRINTING_OPENPORTEX = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut super::super::Foundation::HANDLE, param4: *const MONITOR2) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
pub type PFN_PRINTING_OPENPORTEX2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR, param4: *mut super::super::Foundation::HANDLE, param5: *const MONITOR2) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_System_Power")]
pub type PFN_PRINTING_POWEREVENT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *const super::super::System::Power::POWERBROADCAST_SETTING) -> u32>;
pub type PFN_PRINTING_READPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut u8, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_READPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut u8, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_SENDRECVBIDIDATAFROMPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_sys::core::PCWSTR, param3: *const BIDI_REQUEST_CONTAINER, param4: *mut *mut BIDI_RESPONSE_CONTAINER) -> u32>;
#[cfg(feature = "Win32_Devices_Communication")]
pub type PFN_PRINTING_SETPORTTIMEOUTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const super::super::Devices::Communication::COMMTIMEOUTS, param2: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Devices_Communication")]
pub type PFN_PRINTING_SETPORTTIMEOUTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const super::super::Devices::Communication::COMMTIMEOUTS, param2: u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_SHUTDOWN2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE)>;
pub type PFN_PRINTING_STARTDOCPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: u32, param3: u32, param4: *const u8) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_STARTDOCPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: u32, param3: u32, param4: *const u8) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_WRITEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const u8, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_WRITEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const u8, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_XCVCLOSEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_XCVCLOSEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_XCVDATAPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: *const u8, param3: u32, param4: *mut u8, param5: u32, param6: *mut u32) -> u32>;
pub type PFN_PRINTING_XCVDATAPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: *const u8, param3: u32, param4: *mut u8, param5: u32, param6: *mut u32) -> u32>;
pub type PFN_PRINTING_XCVOPENPORT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFN_PRINTING_XCVOPENPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_sys::core::PCWSTR, param2: u32, param3: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_1 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 49],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub sztIPAddress: [u16; 16],
    pub Reserved: [u8; 540],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
}
impl Default for PORT_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_2 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 128],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub Reserved: [u8; 514],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
    pub dwPortMonitorMibIndex: u32,
}
impl Default for PORT_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_LIST_1 {
    pub dwVersion: u32,
    pub cPortData: u32,
    pub pPortData: [PORT_DATA_2; 1],
}
impl Default for PORT_DATA_LIST_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for PORT_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for PORT_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const PRINTER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = 4;
pub const PRINTER_ACCESS_MANAGE_LIMITED: PRINTER_ACCESS_RIGHTS = 64;
pub type PRINTER_ACCESS_RIGHTS = u32;
pub const PRINTER_ACCESS_USE: PRINTER_ACCESS_RIGHTS = 8;
pub const PRINTER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = 983052;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_DEFAULTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_DEFAULTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_DELETE: PRINTER_ACCESS_RIGHTS = 65536;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: windows_sys::core::PSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl Default for PRINTER_ENUM_VALUESA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: windows_sys::core::PWSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
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
pub const PRINTER_EVENT_ADD_CONNECTION: u32 = 1;
pub const PRINTER_EVENT_ADD_CONNECTION_NO_UI: u32 = 9;
pub const PRINTER_EVENT_ATTRIBUTES_CHANGED: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_EVENT_ATTRIBUTES_INFO {
    pub cbSize: u32,
    pub dwOldAttributes: u32,
    pub dwNewAttributes: u32,
}
pub const PRINTER_EVENT_CACHE_DELETE: u32 = 6;
pub const PRINTER_EVENT_CACHE_REFRESH: u32 = 5;
pub const PRINTER_EVENT_CONFIGURATION_CHANGE: u32 = 0;
pub const PRINTER_EVENT_CONFIGURATION_UPDATE: u32 = 8;
pub const PRINTER_EVENT_DELETE: u32 = 4;
pub const PRINTER_EVENT_DELETE_CONNECTION: u32 = 2;
pub const PRINTER_EVENT_DELETE_CONNECTION_NO_UI: u32 = 10;
pub const PRINTER_EVENT_FLAG_NO_UI: u32 = 1;
pub const PRINTER_EVENT_INITIALIZE: u32 = 3;
pub const PRINTER_EXECUTE: PRINTER_ACCESS_RIGHTS = 131080;
pub const PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5d5a1704_dfd1_4181_8eee_815c86edad31);
pub const PRINTER_EXTENSION_REASON_DRIVER_EVENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x23bb1328_63de_4293_915b_a6a23d929acb);
pub const PRINTER_EXTENSION_REASON_PRINT_PREFERENCES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec8f261f_267c_469f_b5d6_3933023c29cc);
pub type PRINTER_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_2A {
    pub pServerName: windows_sys::core::PSTR,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pShareName: windows_sys::core::PSTR,
    pub pPortName: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pComment: windows_sys::core::PSTR,
    pub pLocation: windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pSepFile: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for PRINTER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_2W {
    pub pServerName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pShareName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pComment: windows_sys::core::PWSTR,
    pub pLocation: windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pSepFile: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for PRINTER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl Default for PRINTER_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_6 {
    pub dwStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: windows_sys::core::PWSTR,
    pub dwAction: u32,
}
impl Default for PRINTER_INFO_7W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_8A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_8W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_8W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_9A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_9A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_9W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PRINTER_NOTIFY_INFO_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_INFO_DATA_COMPACT: u32 = 1;
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_NOTIFY_INIT {
    pub Size: u32,
    pub Reserved: u32,
    pub PollTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub pTypes: *mut PRINTER_NOTIFY_OPTIONS_TYPE,
}
impl Default for PRINTER_NOTIFY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: u16,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Count: u32,
    pub pFields: *mut u16,
}
impl Default for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_STATUS_ENDPOINT: u32 = 1;
pub const PRINTER_NOTIFY_STATUS_INFO: u32 = 4;
pub const PRINTER_NOTIFY_STATUS_POLL: u32 = 2;
pub const PRINTER_NOTIFY_TYPE: u32 = 0;
pub const PRINTER_OEMINTF_VERSION: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_OPTIONSA {
    pub cbSize: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_OPTIONSW {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = 2;
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = 4;
pub type PRINTER_OPTION_FLAGS = i32;
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = 1;
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = 8;
pub const PRINTER_READ: PRINTER_ACCESS_RIGHTS = 131080;
pub const PRINTER_READ_CONTROL: PRINTER_ACCESS_RIGHTS = 131072;
pub const PRINTER_STANDARD_RIGHTS_EXECUTE: PRINTER_ACCESS_RIGHTS = 131072;
pub const PRINTER_STANDARD_RIGHTS_READ: PRINTER_ACCESS_RIGHTS = 131072;
pub const PRINTER_STANDARD_RIGHTS_REQUIRED: PRINTER_ACCESS_RIGHTS = 983040;
pub const PRINTER_STANDARD_RIGHTS_WRITE: PRINTER_ACCESS_RIGHTS = 131072;
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
pub const PRINTER_SYNCHRONIZE: PRINTER_ACCESS_RIGHTS = 1048576;
pub const PRINTER_WRITE: PRINTER_ACCESS_RIGHTS = 131080;
pub const PRINTER_WRITE_DAC: PRINTER_ACCESS_RIGHTS = 262144;
pub const PRINTER_WRITE_OWNER: PRINTER_ACCESS_RIGHTS = 524288;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTIFI32 {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::Gdi::PANOSE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTIFI32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTPROCESSOROPENDATA {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pDocumentName: windows_sys::core::PWSTR,
    pub JobId: u32,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTPROCESSOROPENDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
impl Default for PRINTPROCESSOR_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
impl Default for PRINTPROCESSOR_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROVIDOR {
    pub fpOpenPrinter: isize,
    pub fpSetJob: isize,
    pub fpGetJob: isize,
    pub fpEnumJobs: isize,
    pub fpAddPrinter: isize,
    pub fpDeletePrinter: isize,
    pub fpSetPrinter: isize,
    pub fpGetPrinter: isize,
    pub fpEnumPrinters: isize,
    pub fpAddPrinterDriver: isize,
    pub fpEnumPrinterDrivers: isize,
    pub fpGetPrinterDriver: isize,
    pub fpGetPrinterDriverDirectory: isize,
    pub fpDeletePrinterDriver: isize,
    pub fpAddPrintProcessor: isize,
    pub fpEnumPrintProcessors: isize,
    pub fpGetPrintProcessorDirectory: isize,
    pub fpDeletePrintProcessor: isize,
    pub fpEnumPrintProcessorDatatypes: isize,
    pub fpStartDocPrinter: isize,
    pub fpStartPagePrinter: isize,
    pub fpWritePrinter: isize,
    pub fpEndPagePrinter: isize,
    pub fpAbortPrinter: isize,
    pub fpReadPrinter: isize,
    pub fpEndDocPrinter: isize,
    pub fpAddJob: isize,
    pub fpScheduleJob: isize,
    pub fpGetPrinterData: isize,
    pub fpSetPrinterData: isize,
    pub fpWaitForPrinterChange: isize,
    pub fpClosePrinter: isize,
    pub fpAddForm: isize,
    pub fpDeleteForm: isize,
    pub fpGetForm: isize,
    pub fpSetForm: isize,
    pub fpEnumForms: isize,
    pub fpEnumMonitors: isize,
    pub fpEnumPorts: isize,
    pub fpAddPort: isize,
    pub fpConfigurePort: isize,
    pub fpDeletePort: isize,
    pub fpCreatePrinterIC: isize,
    pub fpPlayGdiScriptOnPrinterIC: isize,
    pub fpDeletePrinterIC: isize,
    pub fpAddPrinterConnection: isize,
    pub fpDeletePrinterConnection: isize,
    pub fpPrinterMessageBox: isize,
    pub fpAddMonitor: isize,
    pub fpDeleteMonitor: isize,
    pub fpResetPrinter: isize,
    pub fpGetPrinterDriverEx: isize,
    pub fpFindFirstPrinterChangeNotification: isize,
    pub fpFindClosePrinterChangeNotification: isize,
    pub fpAddPortEx: isize,
    pub fpShutDown: isize,
    pub fpRefreshPrinterChangeNotification: isize,
    pub fpOpenPrinterEx: isize,
    pub fpAddPrinterEx: isize,
    pub fpSetPort: isize,
    pub fpEnumPrinterData: isize,
    pub fpDeletePrinterData: isize,
    pub fpClusterSplOpen: isize,
    pub fpClusterSplClose: isize,
    pub fpClusterSplIsAlive: isize,
    pub fpSetPrinterDataEx: isize,
    pub fpGetPrinterDataEx: isize,
    pub fpEnumPrinterDataEx: isize,
    pub fpEnumPrinterKey: isize,
    pub fpDeletePrinterDataEx: isize,
    pub fpDeletePrinterKey: isize,
    pub fpSeekPrinter: isize,
    pub fpDeletePrinterDriverEx: isize,
    pub fpAddPerMachineConnection: isize,
    pub fpDeletePerMachineConnection: isize,
    pub fpEnumPerMachineConnections: isize,
    pub fpXcvData: isize,
    pub fpAddPrinterDriverEx: isize,
    pub fpSplReadPrinter: isize,
    pub fpDriverUnloadComplete: isize,
    pub fpGetSpoolFileInfo: isize,
    pub fpCommitSpoolData: isize,
    pub fpCloseSpoolFileHandle: isize,
    pub fpFlushPrinter: isize,
    pub fpSendRecvBidiData: isize,
    pub fpAddPrinterConnection2: isize,
    pub fpGetPrintClassObject: isize,
    pub fpReportJobProcessingProgress: isize,
    pub fpEnumAndLogProvidorObjects: isize,
    pub fpInternalGetPrinterDriver: isize,
    pub fpFindCompatibleDriver: isize,
    pub fpInstallPrinterDriverPackageFromConnection: isize,
    pub fpGetJobNamedPropertyValue: isize,
    pub fpSetJobNamedProperty: isize,
    pub fpDeleteJobNamedProperty: isize,
    pub fpEnumJobNamedProperties: isize,
    pub fpPowerEvent: isize,
    pub fpGetUserPropertyBag: isize,
    pub fpCanShutdown: isize,
    pub fpLogJobInfoForBranchOffice: isize,
    pub fpRegeneratePrintDeviceCapabilities: isize,
    pub fpPrintSupportOperation: isize,
    pub fpIppCreateJobOnPrinter: isize,
    pub fpIppGetJobAttributes: isize,
    pub fpIppSetJobAttributes: isize,
    pub fpIppGetPrinterAttributes: isize,
    pub fpIppSetPrinterAttributes: isize,
    pub fpIppCreateJobOnPrinterWithAttributes: isize,
}
pub const PRINT_APP_BIDI_NOTIFY_CHANNEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2abad223_b994_4aca_82fc_4571b1b585ac);
pub type PRINT_EXECUTION_CONTEXT = i32;
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = 0;
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = 3;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = 2;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = 1;
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINT_FEATURE_OPTION {
    pub pszFeature: windows_sys::core::PCSTR,
    pub pszOption: windows_sys::core::PCSTR,
}
impl Default for PRINT_FEATURE_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINT_PORT_MONITOR_NOTIFY_CHANNEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25df3b0e_74a9_47f5_80ce_79b4b1eb5c58);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PROPSHEETUI_GETICON_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PROPSHEETUI_INFO {
    pub cbSize: u16,
    pub Version: u16,
    pub Flags: u16,
    pub Reason: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
    pub lParamInit: super::super::Foundation::LPARAM,
    pub UserData: usize,
    pub Result: usize,
}
impl Default for PROPSHEETUI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub hWndParent: super::super::Foundation::HWND,
    pub hInst: super::super::Foundation::HINSTANCE,
    pub Anonymous: PROPSHEETUI_INFO_HEADER_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PROPSHEETUI_INFO_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union PROPSHEETUI_INFO_HEADER_0 {
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub IconID: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PROPSHEETUI_INFO_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPSHEETUI_INFO_VERSION: u32 = 256;
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5;
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2;
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4;
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1;
pub const PROPSHEETUI_REASON_INIT: u32 = 0;
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3;
pub const PROTOCOL_LPR_TYPE: u32 = 2;
pub const PROTOCOL_RAWTCP_TYPE: u32 = 1;
pub const PROTOCOL_UNKNOWN_TYPE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: windows_sys::core::PSTR,
}
impl Default for PROVIDOR_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: windows_sys::core::PWSTR,
}
impl Default for PROVIDOR_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSCRIPT5_PRIVATE_DEVMODE {
    pub wReserved: [u16; 57],
    pub wSize: u16,
}
impl Default for PSCRIPT5_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub hCPSUIPage: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
impl Default for PSPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PSUIHDRF_DEFTITLE: u32 = 16;
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32;
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2;
pub const PSUIHDRF_OBSOLETE: u32 = 1;
pub const PSUIHDRF_PROPTITLE: u32 = 4;
pub const PSUIHDRF_USEHICON: u32 = 8;
pub const PSUIINFO_UNICODE: u32 = 1;
pub const PSUIPAGEINSERT_DLL: u32 = 5;
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0;
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4;
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1;
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2;
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3;
pub const PTSHIM_DEFAULT: SHIMOPTS = 0;
pub const PTSHIM_NOSNAPSHOT: SHIMOPTS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBLISHERINFO {
    pub dwMode: u32,
    pub wMinoutlinePPEM: u16,
    pub wMaxbitmapPPEM: u16,
}
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1;
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0;
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2;
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3;
pub type PageCountType = i32;
pub type PrintAsyncNotifyConversationStyle = i32;
pub type PrintAsyncNotifyError = i32;
pub type PrintAsyncNotifyUserFilter = i32;
pub type PrintJobStatus = i32;
pub const PrintJobStatus_BlockedDeviceQueue: PrintJobStatus = 512;
pub const PrintJobStatus_Complete: PrintJobStatus = 4096;
pub const PrintJobStatus_Deleted: PrintJobStatus = 256;
pub const PrintJobStatus_Deleting: PrintJobStatus = 4;
pub const PrintJobStatus_Error: PrintJobStatus = 2;
pub const PrintJobStatus_Offline: PrintJobStatus = 32;
pub const PrintJobStatus_PaperOut: PrintJobStatus = 64;
pub const PrintJobStatus_Paused: PrintJobStatus = 1;
pub const PrintJobStatus_Printed: PrintJobStatus = 128;
pub const PrintJobStatus_Printing: PrintJobStatus = 16;
pub const PrintJobStatus_Restarted: PrintJobStatus = 2048;
pub const PrintJobStatus_Retained: PrintJobStatus = 8192;
pub const PrintJobStatus_Spooling: PrintJobStatus = 8;
pub const PrintJobStatus_UserIntervention: PrintJobStatus = 1024;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintNamedProperty {
    pub propertyName: windows_sys::core::PWSTR,
    pub propertyValue: PrintPropertyValue,
}
impl Default for PrintNamedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PrintPropertyValue_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PrintSchemaAsyncOperation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43b2f83d_10f2_48ab_831b_55fdbdbd34a4);
pub type PrintSchemaConstrainedSetting = i32;
pub const PrintSchemaConstrainedSetting_Admin: PrintSchemaConstrainedSetting = 2;
pub const PrintSchemaConstrainedSetting_Device: PrintSchemaConstrainedSetting = 3;
pub const PrintSchemaConstrainedSetting_None: PrintSchemaConstrainedSetting = 0;
pub const PrintSchemaConstrainedSetting_PrintTicket: PrintSchemaConstrainedSetting = 1;
pub type PrintSchemaParameterDataType = i32;
pub const PrintSchemaParameterDataType_Integer: PrintSchemaParameterDataType = 0;
pub const PrintSchemaParameterDataType_NumericString: PrintSchemaParameterDataType = 1;
pub const PrintSchemaParameterDataType_String: PrintSchemaParameterDataType = 2;
pub type PrintSchemaSelectionType = i32;
pub const PrintSchemaSelectionType_PickMany: PrintSchemaSelectionType = 1;
pub const PrintSchemaSelectionType_PickOne: PrintSchemaSelectionType = 0;
pub const PrinterExtensionManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x331b60da_9e90_4dd0_9c84_eac4e659b61f);
pub const PrinterQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb54c230_798c_4c9e_b461_29fad04039b1);
pub const PrinterQueueView: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb54c231_798c_4c9e_b461_29fad04039b1);
pub const QCP_DEVICEPROFILE: u32 = 0;
pub const QCP_PROFILEDISK: u32 = 2;
pub const QCP_PROFILEMEMORY: u32 = 1;
pub const QCP_SOURCEPROFILE: u32 = 1;
pub const RAWTCP: u32 = 1;
pub const REMOTE_ONLY_REGISTRATION: PrintAsyncNotifyError = 24;
pub const REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1;
pub const REVERSE_PRINT: u32 = 1;
pub const RIGHT_THEN_DOWN: u32 = 1;
pub type ROUTER_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(dwcommand: u32, pcontext: *const core::ffi::c_void, dwcolor: u32, pnofityinfo: *const PRINTER_NOTIFY_INFO, fdwflags: u32, pdwresult: *mut u32) -> windows_sys::core::BOOL>;
pub const ROUTER_STOP_ROUTING: u32 = 2;
pub const ROUTER_SUCCESS: u32 = 1;
pub const ROUTER_UNKNOWN: u32 = 0;
pub const SERVER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = 1;
pub const SERVER_ACCESS_ENUMERATE: PRINTER_ACCESS_RIGHTS = 2;
pub const SERVER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = 983043;
pub const SERVER_EXECUTE: PRINTER_ACCESS_RIGHTS = 131074;
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0;
pub const SERVER_NOTIFY_TYPE: u32 = 2;
pub const SERVER_READ: PRINTER_ACCESS_RIGHTS = 131074;
pub const SERVER_WRITE: PRINTER_ACCESS_RIGHTS = 131075;
pub const SETOPTIONS_FLAG_KEEP_CONFLICT: u32 = 2;
pub const SETOPTIONS_FLAG_RESOLVE_CONFLICT: u32 = 1;
pub const SETOPTIONS_RESULT_CONFLICT_REMAINED: u32 = 2;
pub const SETOPTIONS_RESULT_CONFLICT_RESOLVED: u32 = 1;
pub const SETOPTIONS_RESULT_NO_CONFLICT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::super::Foundation::HANDLE,
    pub Result: super::super::Foundation::LRESULT,
}
impl Default for SETRESULT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SHIMOPTS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHOWUIPARAMS {
    pub UIType: UI_TYPE,
    pub MessageBoxParams: MESSAGEBOX_PARAMS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIMULATE_CAPS_1 {
    pub dwLevel: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwCollate: u32,
    pub dwNupOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPLCLIENT_INFO_1 {
    pub dwSize: u32,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
}
impl Default for SPLCLIENT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_W2K {
    pub hSplPrinter: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPLCLIENT_INFO_3_VISTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
}
impl Default for SPLCLIENT_INFO_3_VISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPLCLIENT_INFO_INTERNAL {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
    pub dwProcessId: u32,
    pub dwSessionId: u32,
}
impl Default for SPLCLIENT_INFO_INTERNAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPLCLIENT_INFO_INTERNAL_LEVEL: u32 = 100;
pub const SPLDS_ASSET_NUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("assetNumber");
pub const SPLDS_BYTES_PER_MINUTE: windows_sys::core::PCWSTR = windows_sys::core::w!("bytesPerMinute");
pub const SPLDS_DESCRIPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("description");
pub const SPLDS_DRIVER_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("DsDriver");
pub const SPLDS_DRIVER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("driverName");
pub const SPLDS_DRIVER_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("driverVersion");
pub const SPLDS_FLAGS: windows_sys::core::PCWSTR = windows_sys::core::w!("flags");
pub const SPLDS_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("location");
pub const SPLDS_PORT_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("portName");
pub const SPLDS_PRINTER_CLASS: windows_sys::core::PCWSTR = windows_sys::core::w!("printQueue");
pub const SPLDS_PRINTER_LOCATIONS: windows_sys::core::PCWSTR = windows_sys::core::w!("printerLocations");
pub const SPLDS_PRINTER_MODEL: windows_sys::core::PCWSTR = windows_sys::core::w!("printerModel");
pub const SPLDS_PRINTER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("printerName");
pub const SPLDS_PRINTER_NAME_ALIASES: windows_sys::core::PCWSTR = windows_sys::core::w!("printerNameAliases");
pub const SPLDS_PRINT_ATTRIBUTES: windows_sys::core::PCWSTR = windows_sys::core::w!("printAttributes");
pub const SPLDS_PRINT_BIN_NAMES: windows_sys::core::PCWSTR = windows_sys::core::w!("printBinNames");
pub const SPLDS_PRINT_COLLATE: windows_sys::core::PCWSTR = windows_sys::core::w!("printCollate");
pub const SPLDS_PRINT_COLOR: windows_sys::core::PCWSTR = windows_sys::core::w!("printColor");
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("printDuplexSupported");
pub const SPLDS_PRINT_END_TIME: windows_sys::core::PCWSTR = windows_sys::core::w!("printEndTime");
pub const SPLDS_PRINT_IPP_COMPRESSION_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("ippCompressionSupported");
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: windows_sys::core::PCWSTR = windows_sys::core::w!("printKeepPrintedJobs");
pub const SPLDS_PRINT_LANGUAGE: windows_sys::core::PCWSTR = windows_sys::core::w!("printLanguage");
pub const SPLDS_PRINT_MAC_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("printMACAddress");
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("printMaxResolutionSupported");
pub const SPLDS_PRINT_MAX_X_EXTENT: windows_sys::core::PCWSTR = windows_sys::core::w!("printMaxXExtent");
pub const SPLDS_PRINT_MAX_Y_EXTENT: windows_sys::core::PCWSTR = windows_sys::core::w!("printMaxYExtent");
pub const SPLDS_PRINT_MEDIA_READY: windows_sys::core::PCWSTR = windows_sys::core::w!("printMediaReady");
pub const SPLDS_PRINT_MEDIA_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("printMediaSupported");
pub const SPLDS_PRINT_MEMORY: windows_sys::core::PCWSTR = windows_sys::core::w!("printMemory");
pub const SPLDS_PRINT_MIN_X_EXTENT: windows_sys::core::PCWSTR = windows_sys::core::w!("printMinXExtent");
pub const SPLDS_PRINT_MIN_Y_EXTENT: windows_sys::core::PCWSTR = windows_sys::core::w!("printMinYExtent");
pub const SPLDS_PRINT_NETWORK_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("printNetworkAddress");
pub const SPLDS_PRINT_NOTIFY: windows_sys::core::PCWSTR = windows_sys::core::w!("printNotify");
pub const SPLDS_PRINT_NUMBER_UP: windows_sys::core::PCWSTR = windows_sys::core::w!("printNumberUp");
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("printOrientationsSupported");
pub const SPLDS_PRINT_OWNER: windows_sys::core::PCWSTR = windows_sys::core::w!("printOwner");
pub const SPLDS_PRINT_PAGES_PER_MINUTE: windows_sys::core::PCWSTR = windows_sys::core::w!("printPagesPerMinute");
pub const SPLDS_PRINT_RATE: windows_sys::core::PCWSTR = windows_sys::core::w!("printRate");
pub const SPLDS_PRINT_RATE_UNIT: windows_sys::core::PCWSTR = windows_sys::core::w!("printRateUnit");
pub const SPLDS_PRINT_SEPARATOR_FILE: windows_sys::core::PCWSTR = windows_sys::core::w!("printSeparatorFile");
pub const SPLDS_PRINT_SHARE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("printShareName");
pub const SPLDS_PRINT_SPOOLING: windows_sys::core::PCWSTR = windows_sys::core::w!("printSpooling");
pub const SPLDS_PRINT_STAPLING_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("printStaplingSupported");
pub const SPLDS_PRINT_START_TIME: windows_sys::core::PCWSTR = windows_sys::core::w!("printStartTime");
pub const SPLDS_PRINT_STATUS: windows_sys::core::PCWSTR = windows_sys::core::w!("printStatus");
pub const SPLDS_PRIORITY: windows_sys::core::PCWSTR = windows_sys::core::w!("priority");
pub const SPLDS_SERVER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("serverName");
pub const SPLDS_SHORT_SERVER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("shortServerName");
pub const SPLDS_SPOOLER_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("DsSpooler");
pub const SPLDS_UNC_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("uNCName");
pub const SPLDS_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("url");
pub const SPLDS_USER_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("DsUser");
pub const SPLDS_VERSION_NUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("versionNumber");
pub const SPLPRINTER_USER_MODE_PRINTER_DRIVER: windows_sys::core::PCWSTR = windows_sys::core::w!("SPLUserModePrinterDriver");
pub const SPLREG_ALLOW_USER_MANAGEFORMS: windows_sys::core::PCWSTR = windows_sys::core::w!("AllowUserManageForms");
pub const SPLREG_ARCHITECTURE: windows_sys::core::PCWSTR = windows_sys::core::w!("Architecture");
pub const SPLREG_BEEP_ENABLED: windows_sys::core::PCWSTR = windows_sys::core::w!("BeepEnabled");
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultSpoolDirectory");
pub const SPLREG_DNS_MACHINE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DNSMachineName");
pub const SPLREG_DS_PRESENT: windows_sys::core::PCWSTR = windows_sys::core::w!("DsPresent");
pub const SPLREG_DS_PRESENT_FOR_USER: windows_sys::core::PCWSTR = windows_sys::core::w!("DsPresentForUser");
pub const SPLREG_EVENT_LOG: windows_sys::core::PCWSTR = windows_sys::core::w!("EventLog");
pub const SPLREG_MAJOR_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("MajorVersion");
pub const SPLREG_MINOR_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("MinorVersion");
pub const SPLREG_NET_POPUP: windows_sys::core::PCWSTR = windows_sys::core::w!("NetPopup");
pub const SPLREG_NET_POPUP_TO_COMPUTER: windows_sys::core::PCWSTR = windows_sys::core::w!("NetPopupToComputer");
pub const SPLREG_OS_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("OSVersion");
pub const SPLREG_OS_VERSIONEX: windows_sys::core::PCWSTR = windows_sys::core::w!("OSVersionEx");
pub const SPLREG_PORT_THREAD_PRIORITY: windows_sys::core::PCWSTR = windows_sys::core::w!("PortThreadPriority");
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: windows_sys::core::PCWSTR = windows_sys::core::w!("PortThreadPriorityDefault");
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationExecutionPolicy");
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationGroups");
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationIdleTimeout");
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationMaxobjsBeforeRecycle");
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationOverrideCompat");
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDriverIsolationTimeBeforeRecycle");
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintQueueV4DriverDirectory");
pub const SPLREG_REMOTE_FAX: windows_sys::core::PCWSTR = windows_sys::core::w!("RemoteFax");
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: windows_sys::core::PCWSTR = windows_sys::core::w!("RestartJobOnPoolEnabled");
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: windows_sys::core::PCWSTR = windows_sys::core::w!("RestartJobOnPoolError");
pub const SPLREG_RETRY_POPUP: windows_sys::core::PCWSTR = windows_sys::core::w!("RetryPopup");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: windows_sys::core::PCWSTR = windows_sys::core::w!("SchedulerThreadPriority");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: windows_sys::core::PCWSTR = windows_sys::core::w!("SchedulerThreadPriorityDefault");
pub const SPLREG_WEBSHAREMGMT: windows_sys::core::PCWSTR = windows_sys::core::w!("WebShareMgmt");
pub const SPOOL_FILE_PERSISTENT: u32 = 1;
pub const SPOOL_FILE_TEMPORARY: u32 = 2;
pub const SR_OWNER: u32 = 0;
pub const SR_OWNER_PARENT: u32 = 1;
pub const SSP_STDPAGE1: u32 = 10001;
pub const SSP_STDPAGE2: u32 = 10002;
pub const SSP_TVPAGE: u32 = 10000;
pub const STRING_LANGPAIR: u32 = 4;
pub const STRING_MUIDLL: u32 = 2;
pub const STRING_NONE: u32 = 1;
pub const S_CONFLICT_RESOLVED: u32 = 262146;
pub const S_DEVCAP_OUTPUT_FULL_REPLACEMENT: windows_sys::core::HRESULT = 0x4DC01_u32 as _;
pub const S_NO_CONFLICT: u32 = 262145;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRANSDATA {
    pub ubCodePageID: u8,
    pub ubType: u8,
    pub uCode: TRANSDATA_0,
}
impl Default for TRANSDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRANSDATA_0 {
    pub sCode: i16,
    pub ubCode: u8,
    pub ubPairs: [u8; 2],
}
impl Default for TRANSDATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTDOWNLOAD_BITMAP: u32 = 2;
pub const TTDOWNLOAD_DONTCARE: u32 = 0;
pub const TTDOWNLOAD_GRAPHICS: u32 = 1;
pub const TTDOWNLOAD_TTOUTLINE: u32 = 3;
pub const TVOT_2STATES: u32 = 0;
pub const TVOT_3STATES: u32 = 1;
pub const TVOT_CHKBOX: u32 = 9;
pub const TVOT_COMBOBOX: u32 = 6;
pub const TVOT_EDITBOX: u32 = 7;
pub const TVOT_LISTBOX: u32 = 5;
pub const TVOT_NSTATES_EX: u32 = 10;
pub const TVOT_PUSHBUTTON: u32 = 8;
pub const TVOT_SCROLLBAR: u32 = 4;
pub const TVOT_TRACKBAR: u32 = 3;
pub const TVOT_UDARROW: u32 = 2;
pub const TYPE_GLYPHHANDLE: u32 = 3;
pub const TYPE_GLYPHID: u32 = 4;
pub const TYPE_TRANSDATA: u32 = 2;
pub const TYPE_UNICODE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UFF_FILEHEADER {
    pub dwSignature: u32,
    pub dwVersion: u32,
    pub dwSize: u32,
    pub nFonts: u32,
    pub nGlyphSets: u32,
    pub nVarData: u32,
    pub offFontDir: u32,
    pub dwFlags: u32,
    pub dwReserved: [u32; 4],
}
impl Default for UFF_FILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFF_FONTDIRECTORY {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wFontID: u16,
    pub sGlyphID: i16,
    pub wFlags: u16,
    pub dwInstallerSig: u32,
    pub offFontName: u32,
    pub offCartridgeName: u32,
    pub offFontData: u32,
    pub offGlyphData: u32,
    pub offVarData: u32,
}
pub const UFF_VERSION_NUMBER: u32 = 65537;
pub const UFM_CART: u32 = 2;
pub const UFM_SCALABLE: u32 = 4;
pub const UFM_SOFT: u32 = 1;
pub const UFOFLAG_TTDOWNLOAD_BITMAP: u32 = 2;
pub const UFOFLAG_TTDOWNLOAD_TTOUTLINE: u32 = 4;
pub const UFOFLAG_TTFONT: u32 = 1;
pub const UFOFLAG_TTOUTLINE_BOLD_SIM: u32 = 8;
pub const UFOFLAG_TTOUTLINE_ITALIC_SIM: u32 = 16;
pub const UFOFLAG_TTOUTLINE_VERTICAL: u32 = 32;
pub const UFOFLAG_TTSUBSTITUTED: u32 = 64;
pub const UFO_GETINFO_FONTOBJ: u32 = 1;
pub const UFO_GETINFO_GLYPHBITMAP: u32 = 3;
pub const UFO_GETINFO_GLYPHSTRING: u32 = 2;
pub const UFO_GETINFO_GLYPHWIDTH: u32 = 4;
pub const UFO_GETINFO_MEMORY: u32 = 5;
pub const UFO_GETINFO_STDVARIABLE: u32 = 6;
pub type UI_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIDRVINFO {
    pub dwSize: u32,
    pub flGenFlags: u32,
    pub wType: u16,
    pub fCaps: u16,
    pub wXRes: u16,
    pub wYRes: u16,
    pub sYAdjust: i16,
    pub sYMoved: i16,
    pub wPrivateData: u16,
    pub sShift: i16,
    pub SelectFont: INVOC,
    pub UnSelectFont: INVOC,
    pub wReserved: [u16; 4],
}
impl Default for UNIDRVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIDRV_PRIVATE_DEVMODE {
    pub wReserved: [u16; 4],
    pub wSize: u16,
}
impl Default for UNIDRV_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIFM_HDR {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub ulDefaultCodepage: u32,
    pub lGlyphSetDataRCID: i32,
    pub loUnidrvInfo: u32,
    pub loIFIMetrics: u32,
    pub loExtTextMetric: u32,
    pub loWidthTable: u32,
    pub loKernPair: u32,
    pub dwReserved: [u32; 2],
}
impl Default for UNIFM_HDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNIFM_VERSION_1_0: u32 = 65536;
pub const UNIRECTIONAL_NOTIFICATION_LOST: PrintAsyncNotifyError = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNI_CODEPAGEINFO {
    pub dwCodePage: u32,
    pub SelectSymbolSet: INVOC,
    pub UnSelectSymbolSet: INVOC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNI_GLYPHSETDATA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub lPredefinedID: i32,
    pub dwGlyphCount: u32,
    pub dwRunCount: u32,
    pub loRunOffset: u32,
    pub dwCodePageCount: u32,
    pub loCodePageOffset: u32,
    pub loMapTableOffset: u32,
    pub dwReserved: [u32; 2],
}
impl Default for UNI_GLYPHSETDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNI_GLYPHSETDATA_VERSION_1_0: u32 = 65536;
pub const UNKNOWN_PROTOCOL: u32 = 0;
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4;
pub const UPDP_SILENT_UPLOAD: u32 = 1;
pub const UPDP_UPLOAD_ALWAYS: u32 = 2;
pub const USBPRINT_IOCTL_INDEX: u32 = 0;
pub const USB_PRINTER_INTERFACE_CLASSIC: u32 = 1;
pub const USB_PRINTER_INTERFACE_DUAL: u32 = 3;
pub const USB_PRINTER_INTERFACE_IPP: u32 = 2;
pub const USB_PRINT_IPP_COMPAT_ID: u32 = 1;
pub const USB_PRINT_IPP_FAXOUT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USERDATA {
    pub dwSize: u32,
    pub dwItemID: usize,
    pub pKeyWordName: windows_sys::core::PSTR,
    pub dwReserved: [u32; 8],
}
impl Default for USERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIDTHRUN {
    pub wStartGlyph: u16,
    pub wGlyphCount: u16,
    pub loCharWidthOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIDTHTABLE {
    pub dwSize: u32,
    pub dwRunNum: u32,
    pub WidthRun: [WIDTHRUN; 1],
}
impl Default for WIDTHTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_FI_FILENAME: u32 = 900;
pub type XPSRAS_BACKGROUND_COLOR = i32;
pub const XPSRAS_BACKGROUND_COLOR_OPAQUE: XPSRAS_BACKGROUND_COLOR = 1;
pub const XPSRAS_BACKGROUND_COLOR_TRANSPARENT: XPSRAS_BACKGROUND_COLOR = 0;
pub type XPSRAS_PIXEL_FORMAT = i32;
pub const XPSRAS_PIXEL_FORMAT_128BPP_PRGBA_FLOAT_SCRGB: XPSRAS_PIXEL_FORMAT = 3;
pub const XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB: XPSRAS_PIXEL_FORMAT = 1;
pub const XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB: XPSRAS_PIXEL_FORMAT = 2;
pub type XPSRAS_RENDERING_MODE = i32;
pub const XPSRAS_RENDERING_MODE_ALIASED: XPSRAS_RENDERING_MODE = 1;
pub const XPSRAS_RENDERING_MODE_ANTIALIASED: XPSRAS_RENDERING_MODE = 0;
pub const XPS_FP_DRIVER_PROPERTY_BAG: windows_sys::core::PCWSTR = windows_sys::core::w!("DriverPropertyBag");
pub const XPS_FP_FAX_JOB_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("JobFaxProperties");
pub const XPS_FP_JOB_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintJobId");
pub const XPS_FP_JOB_LEVEL_PRINTTICKET: windows_sys::core::PCWSTR = windows_sys::core::w!("JobPrintTicket");
pub const XPS_FP_MERGED_DATAFILE_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("MergedDataFilePath");
pub const XPS_FP_MS_CONTENT_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("DriverMultiContentType");
pub const XPS_FP_MS_CONTENT_TYPE_OPENXPS: windows_sys::core::PCWSTR = windows_sys::core::w!("OpenXPS");
pub const XPS_FP_MS_CONTENT_TYPE_XPS: windows_sys::core::PCWSTR = windows_sys::core::w!("XPS");
pub const XPS_FP_OUTPUT_FILE: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintOutputFileName");
pub const XPS_FP_PRINTDEVICECAPABILITIES: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintDeviceCapabilities");
pub const XPS_FP_PRINTER_HANDLE: windows_sys::core::PCWSTR = windows_sys::core::w!("PrinterHandle");
pub const XPS_FP_PRINTER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PrinterName");
pub const XPS_FP_PRINT_CLASS_FACTORY: windows_sys::core::PCWSTR = windows_sys::core::w!("PrintClassFactory");
pub const XPS_FP_PROGRESS_REPORT: windows_sys::core::PCWSTR = windows_sys::core::w!("ProgressReport");
pub const XPS_FP_QUEUE_PROPERTY_BAG: windows_sys::core::PCWSTR = windows_sys::core::w!("QueuePropertyBag");
pub const XPS_FP_RESOURCE_DLL_PATHS: windows_sys::core::PCWSTR = windows_sys::core::w!("ResourceDLLPaths");
pub const XPS_FP_USER_PRINT_TICKET: windows_sys::core::PCWSTR = windows_sys::core::w!("PerUserPrintTicket");
pub const XPS_FP_USER_TOKEN: windows_sys::core::PCWSTR = windows_sys::core::w!("UserSecurityToken");
pub const XpsJob_DocumentSequenceAdded: EXpsJobConsumption = 0;
pub const XpsJob_FixedDocumentAdded: EXpsJobConsumption = 1;
pub const XpsJob_FixedPageAdded: EXpsJobConsumption = 2;
pub const Xps_Restricted_Font_Editable: EXpsFontRestriction = 8;
pub const Xps_Restricted_Font_Installable: EXpsFontRestriction = 0;
pub const Xps_Restricted_Font_NoEmbedding: EXpsFontRestriction = 2;
pub const Xps_Restricted_Font_PreviewPrint: EXpsFontRestriction = 4;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type _CPSUICALLBACK = Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _SPLCLIENT_INFO_2_V3 {
    pub hSplPrinter: u64,
}
pub const kADT_ASCII: EATTRIBUTE_DATATYPE = 5;
pub const kADT_BINARY: EATTRIBUTE_DATATYPE = 7;
pub const kADT_BOOL: EATTRIBUTE_DATATYPE = 1;
pub const kADT_CUSTOMSIZEPARAMS: EATTRIBUTE_DATATYPE = 10;
pub const kADT_DWORD: EATTRIBUTE_DATATYPE = 4;
pub const kADT_INT: EATTRIBUTE_DATATYPE = 2;
pub const kADT_LONG: EATTRIBUTE_DATATYPE = 3;
pub const kADT_RECT: EATTRIBUTE_DATATYPE = 9;
pub const kADT_SIZE: EATTRIBUTE_DATATYPE = 8;
pub const kADT_UNICODE: EATTRIBUTE_DATATYPE = 6;
pub const kADT_UNKNOWN: EATTRIBUTE_DATATYPE = 0;
pub const kAddingDocumentSequence: EPrintXPSJobProgress = 0;
pub const kAddingFixedDocument: EPrintXPSJobProgress = 2;
pub const kAddingFixedPage: EPrintXPSJobProgress = 4;
pub const kAllUsers: PrintAsyncNotifyUserFilter = 1;
pub const kBiDirectional: PrintAsyncNotifyConversationStyle = 0;
pub const kDocumentSequenceAdded: EPrintXPSJobProgress = 1;
pub const kFixedDocumentAdded: EPrintXPSJobProgress = 3;
pub const kFixedPageAdded: EPrintXPSJobProgress = 5;
pub const kFontAdded: EPrintXPSJobProgress = 7;
pub const kImageAdded: EPrintXPSJobProgress = 8;
pub const kInvalidJobState: EBranchOfficeJobEventType = 0;
pub const kJobConsumption: EPrintXPSJobOperation = 2;
pub const kJobProduction: EPrintXPSJobOperation = 1;
pub const kLogJobError: EBranchOfficeJobEventType = 3;
pub const kLogJobPipelineError: EBranchOfficeJobEventType = 4;
pub const kLogJobPrinted: EBranchOfficeJobEventType = 1;
pub const kLogJobRendered: EBranchOfficeJobEventType = 2;
pub const kLogOfflineFileFull: EBranchOfficeJobEventType = 5;
pub const kMessageBox: UI_TYPE = 0;
pub const kPerUser: PrintAsyncNotifyUserFilter = 0;
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
pub const kUniDirectional: PrintAsyncNotifyConversationStyle = 1;
pub const kXpsDocumentCommitted: EPrintXPSJobProgress = 9;
