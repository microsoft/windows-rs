#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormA(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormW(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobA(hprinter: super::super::Foundation::HANDLE, level: u32, pdata: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobW(hprinter: super::super::Foundation::HANDLE, level: u32, pdata: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorA(pname: super::super::Foundation::PSTR, level: u32, pmonitors: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorW(pname: super::super::Foundation::PWSTR, level: u32, pmonitors: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortA(pname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, pmonitorname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortW(pname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, pmonitorname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintDeviceObject(hprinter: super::super::Foundation::HANDLE, phdeviceobject: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, ppathname: super::super::Foundation::PSTR, pprintprocessorname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, ppathname: super::super::Foundation::PWSTR, pprintprocessorname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorA(pname: super::super::Foundation::PSTR, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorW(pname: super::super::Foundation::PWSTR, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterA(pname: super::super::Foundation::PSTR, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2A(hwnd: super::super::Foundation::HWND, pszname: super::super::Foundation::PSTR, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2W(hwnd: super::super::Foundation::HWND, pszname: super::super::Foundation::PWSTR, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionA(pname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionW(pname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverA(pname: super::super::Foundation::PSTR, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExA(pname: super::super::Foundation::PSTR, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExW(pname: super::super::Foundation::PWSTR, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverW(pname: super::super::Foundation::PWSTR, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterW(pname: super::super::Foundation::PWSTR, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesA(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevmodeinput: *const super::Gdi::DEVMODEA) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesW(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEW, pdevmodeinput: *const super::Gdi::DEVMODEW) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendPrinterNotifyInfoData(pinfodest: *const PRINTER_NOTIFY_INFO, pdatasrc: *const PRINTER_NOTIFY_INFO_DATA, fdwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallRouterFindFirstPrinterChangeNotification(hprinterrpc: super::super::Foundation::HANDLE, fdwfilterflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: *const PRINTER_NOTIFY_OPTIONS) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseSpoolFileHandle(hprinter: super::super::Foundation::HANDLE, hspoolfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitSpoolData(hprinter: super::super::Foundation::HANDLE, hspoolfile: super::super::Foundation::HANDLE, cbcommit: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIA(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIW(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortA(pname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, pportname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortW(pname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, pportname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConnectToPrinterDlg(hwnd: super::super::Foundation::HWND, flags: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, coredriverguid: ::windows_sys::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, coredriverguid: ::windows_sys::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrintAsyncNotifyChannel(pszname: super::super::Foundation::PWSTR, pnotificationtype: *const ::windows_sys::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: IPrintAsyncNotifyCallback, ppiasynchnotification: *mut IPrintAsyncNotifyChannel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreatePrinterIC(hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEW) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormA(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormW(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteJobNamedProperty(hprinter: super::super::Foundation::HANDLE, jobid: u32, pszname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, pmonitorname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, pmonitorname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortA(pname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, pportname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortW(pname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, pportname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, pprintprocessorname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, pprintprocessorname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, pprintprovidorname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, pprintprovidorname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionA(pname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionW(pname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR, pvaluename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, pdrivername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, pdrivername: super::super::Foundation::PSTR, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, pdrivername: super::super::Foundation::PWSTR, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, pdrivername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterIC(hprinteric: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrint(hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, presid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrintEx(pdqpinfo: *mut DEVQUERYPRINT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesA(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevmodeinput: *const super::Gdi::DEVMODEA, fmode: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesW(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEW, pdevmodeinput: *const super::Gdi::DEVMODEW, fmode: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDocPrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPagePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsA(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsW(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobNamedProperties(hprinter: super::super::Foundation::HANDLE, jobid: u32, pcproperties: *mut u32, ppproperties: *mut *mut PrintNamedProperty) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsA(hprinter: super::super::Foundation::HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsW(hprinter: super::super::Foundation::HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsA(pname: super::super::Foundation::PSTR, level: u32, pmonitor: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsW(pname: super::super::Foundation::PWSTR, level: u32, pmonitor: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsA(pname: super::super::Foundation::PSTR, level: u32, pport: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsW(pname: super::super::Foundation::PWSTR, level: u32, pport: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesA(pname: super::super::Foundation::PSTR, pprintprocessorname: super::super::Foundation::PSTR, level: u32, pdatatypes: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesW(pname: super::super::Foundation::PWSTR, pprintprocessorname: super::super::Foundation::PWSTR, level: u32, pdatatypes: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataA(hprinter: super::super::Foundation::HANDLE, dwindex: u32, pvaluename: super::super::Foundation::PSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: *mut u32, pdata: *mut u8, cbdata: u32, pcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR, penumvalues: *mut u8, cbenumvalues: u32, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR, penumvalues: *mut u8, cbenumvalues: u32, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataW(hprinter: super::super::Foundation::HANDLE, dwindex: u32, pvaluename: super::super::Foundation::PWSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: *mut u32, pdata: *mut u8, cbdata: u32, pcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR, psubkey: super::super::Foundation::PSTR, cbsubkey: u32, pcbsubkey: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR, psubkey: super::super::Foundation::PWSTR, cbsubkey: u32, pcbsubkey: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersA(flags: u32, name: super::super::Foundation::PSTR, level: u32, pprinterenum: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersW(flags: u32, name: super::super::Foundation::PWSTR, level: u32, pprinterenum: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ExtDeviceMode(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HANDLE, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevicename: super::super::Foundation::PSTR, pport: super::super::Foundation::PSTR, pdevmodeinput: *const super::Gdi::DEVMODEA, pprofile: super::super::Foundation::PSTR, fmode: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClosePrinterChangeNotification(hchange: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwfilter: u32, fdwoptions: u32, pprinternotifyoptions: *const ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextPrinterChangeNotification(hchange: super::super::Foundation::HANDLE, pdwchange: *mut u32, pvreserved: *const ::core::ffi::c_void, ppprinternotifyinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushPrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *const ::core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32, csleep: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrintNamedPropertyArray(cproperties: u32, ppproperties: *mut *mut PrintNamedProperty);
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrintPropertyValue(pvalue: *mut PrintPropertyValue);
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrinterNotifyInfo(pprinternotifyinfo: *const PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiDeleteSpoolFileHandle(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndDocEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, dwoptimization: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDC(spoolfilehandle: super::super::Foundation::HANDLE) -> super::Gdi::HDC;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDevmodeForPage(spoolfilehandle: super::super::Foundation::HANDLE, dwpagenumber: u32, pcurrdm: *mut *mut super::Gdi::DEVMODEW, plastdm: *mut *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageCount(spoolfilehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageHandle(spoolfilehandle: super::super::Foundation::HANDLE, page: u32, pdwpagetype: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetSpoolFileHandle(pwszprintername: super::super::Foundation::PWSTR, pdevmode: *mut super::Gdi::DEVMODEW, pwszdocname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiPlayPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, hemf: super::super::Foundation::HANDLE, prectdocument: *mut super::super::Foundation::RECT, prectborder: *mut super::super::Foundation::RECT, prectclip: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiResetDCEMF(spoolfilehandle: super::super::Foundation::HANDLE, pcurrdm: *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn GdiStartDocEMF(spoolfilehandle: super::super::Foundation::HANDLE, pdocinfo: *mut super::super::Storage::Xps::DOCINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiStartPageEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateCopyFilePaths(pszprintername: super::super::Foundation::PWSTR, pszdirectory: super::super::Foundation::PWSTR, psplclientinfo: *const u8, dwlevel: u32, pszsourcedir: super::super::Foundation::PWSTR, pcchsourcedirsize: *mut u32, psztargetdir: super::super::Foundation::PWSTR, pcchtargetdirsize: *mut u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPSUIUserData(hdlg: super::super::Foundation::HWND) -> usize;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, pszzcoredriverdependencies: super::super::Foundation::PSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, pszzcoredriverdependencies: super::super::Foundation::PWSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERW) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterA(pszbuffer: super::super::Foundation::PSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterW(pszbuffer: super::super::Foundation::PWSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormA(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PSTR, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormW(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PWSTR, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobA(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributes(pprintername: super::super::Foundation::PWSTR, pdevmode: *const super::Gdi::DEVMODEW, pattributeinfo: *mut ATTRIBUTE_INFO_3) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributesEx(pprintername: super::super::Foundation::PWSTR, pdevmode: *const super::Gdi::DEVMODEW, dwlevel: u32, pattributeinfo: *mut u8, nsize: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobNamedPropertyValue(hprinter: super::super::Foundation::HANDLE, jobid: u32, pszname: super::super::Foundation::PWSTR, pvalue: *mut PrintPropertyValue) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobW(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintExecutionData(pdata: *mut PRINT_EXECUTION_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintOutputInfo(hwnd: super::super::Foundation::HWND, pszprinter: super::super::Foundation::PWSTR, phfile: *mut super::super::Foundation::HANDLE, ppszoutputfile: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR, pvaluename: super::super::Foundation::PSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2A(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, penvironment: super::super::Foundation::PSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2W(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, penvironment: super::super::Foundation::PWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverA(hprinter: super::super::Foundation::HANDLE, penvironment: super::super::Foundation::PSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryA(pname: super::super::Foundation::PSTR, penvironment: super::super::Foundation::PSTR, level: u32, pdriverdirectory: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryW(pname: super::super::Foundation::PWSTR, penvironment: super::super::Foundation::PWSTR, level: u32, pdriverdirectory: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverPackagePathA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, pszlanguage: super::super::Foundation::PSTR, pszpackageid: super::super::Foundation::PSTR, pszdriverpackagecab: super::super::Foundation::PSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverPackagePathW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, pszlanguage: super::super::Foundation::PWSTR, pszpackageid: super::super::Foundation::PWSTR, pszdriverpackagecab: super::super::Foundation::PWSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverW(hprinter: super::super::Foundation::HANDLE, penvironment: super::super::Foundation::PWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSpoolFileHandle(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonatePrinterClient(htoken: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPrinterDriverFromPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszdrivername: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPrinterDriverFromPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszdrivername: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeA(pdevmode: *const super::Gdi::DEVMODEA, devmodesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeW(pdevmode: *const super::Gdi::DEVMODEW, devmodesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2A(pprintername: super::super::Foundation::PSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA, poptions: *const PRINTER_OPTIONSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2W(pprintername: super::super::Foundation::PWSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW, poptions: *const PRINTER_OPTIONSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterA(pprintername: super::super::Foundation::PSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterW(pprintername: super::super::Foundation::PWSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PartialReplyPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, pdatasrc: *const PRINTER_NOTIFY_INFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayGdiScriptOnPrinterIC(hprinteric: super::super::Foundation::HANDLE, pin: *const u8, cin: u32, pout: *mut u8, cout: u32, ul: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxA(hprinter: super::super::Foundation::HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: super::super::Foundation::PSTR, pcaption: super::super::Foundation::PSTR, dwtype: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxW(hprinter: super::super::Foundation::HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: super::super::Foundation::PWSTR, pcaption: super::super::Foundation::PWSTR, dwtype: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterProperties(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindClosePrinterChangeNotification(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: *const ::core::ffi::c_void, pvreserved1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadPrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *mut ::core::ffi::c_void, cbbuf: u32, pnobytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForPrintAsyncNotifications(pszname: super::super::Foundation::PWSTR, pnotificationtype: *const ::windows_sys::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: IPrintAsyncNotifyCallback, phnotify: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePrintDeviceObject(hdeviceobject: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwchangeflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotificationEx(hnotify: super::super::Foundation::HANDLE, dwcolor: u32, fdwflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportJobProcessingProgress(printerhandle: super::super::Foundation::HANDLE, jobid: u32, joboperation: EPrintXPSJobOperation, jobprogress: EPrintXPSJobProgress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterA(hprinter: super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterW(hprinter: super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToPrinterSelf() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterAllocBidiMem(numbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAllocBidiResponseContainer(count: u32) -> *mut BIDI_RESPONSE_CONTAINER;
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata: u32) -> *mut PRINTER_NOTIFY_INFO;
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterFreeBidiMem(pmempointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreeBidiResponseContainer(pdata: *const BIDI_RESPONSE_CONTAINER) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScheduleJob(hprinter: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCPSUIUserData(hdlg: super::super::Foundation::HWND, cpsuiuserdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterA(pszprinter: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterW(pszprinter: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormA(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PSTR, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormW(hprinter: super::super::Foundation::HANDLE, pformname: super::super::Foundation::PWSTR, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobA(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobNamedProperty(hprinter: super::super::Foundation::HANDLE, jobid: u32, pproperty: *const PrintNamedProperty) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobW(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortA(pname: super::super::Foundation::PSTR, pportname: super::super::Foundation::PSTR, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortW(pname: super::super::Foundation::PWSTR, pportname: super::super::Foundation::PWSTR, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *const u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PSTR, pvaluename: super::super::Foundation::PSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *const u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplIsSessionZero(hprinter: super::super::Foundation::HANDLE, jobid: u32, pissessionzero: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplPromptUIInUsersSession(hprinter: super::super::Foundation::HANDLE, jobid: u32, puiparams: *const SHOWUIPARAMS, presponse: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerCopyFileEvent(pszprintername: super::super::Foundation::PWSTR, pszkey: super::super::Foundation::PWSTR, dwcopyfileevent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindClosePrinterChangeNotification(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwfilterflags: u32, fdwoptions: u32, pprinternotifyoptions: *const ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void, pnotificationconfig: *const ::core::ffi::c_void, phnotify: *mut super::super::Foundation::HANDLE, phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindNextPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, pfdwchange: *mut u32, pprinternotifyoptions: *const ::core::ffi::c_void, ppprinternotifyinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn SpoolerFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO);
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerRefreshPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, dwcolor: u32, poptions: *const PRINTER_NOTIFY_OPTIONS, ppinfo: *mut *mut PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pdocinfo: *const DOC_INFO_1A) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pdocinfo: *const DOC_INFO_1W) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartPagePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnRegisterForPrintAsyncNotifications(param0: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePrintDeviceObject(hprinter: super::super::Foundation::HANDLE, hdeviceobject: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: super::super::Foundation::PSTR, pcchdestinfpath: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: super::super::Foundation::PWSTR, pcchdestinfpath: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForPrinterChange(hprinter: super::super::Foundation::HANDLE, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *const ::core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XcvDataW(hxcv: super::super::Foundation::HANDLE, pszdataname: super::super::Foundation::PWSTR, pinputdata: *const u8, cbinputdata: u32, poutputdata: *mut u8, cboutputdata: u32, pcboutputneeded: *mut u32, pdwstatus: *mut u32) -> super::super::Foundation::BOOL;
}
pub struct ADDJOB_INFO_1A(i32);
pub struct ADDJOB_INFO_1W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APD_COPY_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APD_COPY_FROM_DIRECTORY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APD_COPY_NEW_FILES: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APD_STRICT_DOWNGRADE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APD_STRICT_UPGRADE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2u32;
pub struct ATTRIBUTE_INFO_1(i32);
pub struct ATTRIBUTE_INFO_2(i32);
pub struct ATTRIBUTE_INFO_3(i32);
pub struct ATTRIBUTE_INFO_4(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BIDI_ACCESS_ADMINISTRATOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BIDI_ACCESS_USER: u32 = 2u32;
pub struct BIDI_DATA(i32);
pub struct BIDI_REQUEST_CONTAINER(i32);
pub struct BIDI_REQUEST_DATA(i32);
pub struct BIDI_RESPONSE_CONTAINER(i32);
pub struct BIDI_RESPONSE_DATA(i32);
pub struct BIDI_TYPE(i32);
pub struct BINARY_CONTAINER(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BOOKLET_EDGE_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BOOKLET_EDGE_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BOOKLET_PRINT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const BORDER_PRINT: u32 = 0u32;
pub struct BidiRequest(i32);
pub struct BidiRequestContainer(i32);
pub struct BidiSpl(i32);
pub struct BranchOfficeJobData(i32);
pub struct BranchOfficeJobDataContainer(i32);
pub struct BranchOfficeJobDataError(i32);
pub struct BranchOfficeJobDataPipelineFailed(i32);
pub struct BranchOfficeJobDataPrinted(i32);
pub struct BranchOfficeJobDataRendered(i32);
pub struct BranchOfficeLogOfflineFileFull(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_BIG5: i32 = -10i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_CP437: i32 = -1i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_CP850: i32 = -2i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_CP863: i32 = -3i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_GB2312: i32 = -16i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_ISC: i32 = -11i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_JIS: i32 = -12i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_JIS_ANK: i32 = -13i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_NOPRECNV: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_NS86: i32 = -14i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_SJIS: i32 = -17i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_TCA: i32 = -15i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CC_WANSUNG: i32 = -18i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CDM_CONVERT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CDM_CONVERT351: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CDM_DRIVER_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_FALSE_PDATA: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_FALSE_TRUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_NONE_PDATA: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_NO_PDATA: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_NO_YES: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_OFF_ON: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CHKBOXS_OFF_PDATA: u32 = 5u32;
pub const CLSID_OEMPTPROVIDER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2440181906,
    data2: 17874,
    data3: 18658,
    data4: [158, 201, 86, 35, 121, 218, 249, 146],
};
pub const CLSID_OEMRENDER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1835712294, data2: 40760, data3: 4561, data4: [136, 42, 0, 192, 79, 185, 97, 236] };
pub const CLSID_OEMUI: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2882437335, data2: 40774, data3: 4561, data4: [136, 42, 0, 192, 79, 185, 97, 236] };
pub const CLSID_OEMUIMXDC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1309950720, data2: 23363, data3: 17032, data4: [147, 42, 94, 77, 214, 216, 43, 237] };
pub const CLSID_PTPROVIDER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1185682715,
    data2: 33936,
    data3: 17713,
    data4: [150, 204, 85, 191, 43, 241, 158, 17],
};
pub const CLSID_XPSRASTERIZER_FACTORY: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1346271679,
    data2: 7433,
    data3: 18276,
    data4: [157, 114, 30, 176, 198, 89, 103, 198],
};
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COLOR_OPTIMIZATION: u32 = 1u32;
pub struct COMPROPSHEETUI(i32);
pub struct CONFIG_INFO_DATA_1(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_EVENT_ADD_PRINTER_CONNECTION: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER_CONNECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_EVENT_FILES_CHANGED: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_EVENT_SET_PRINTER_DATAEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_FLAG_CLIENT_SPOOLER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const COPYFILE_FLAG_SERVER_SPOOLER: u32 = 2u32;
pub struct CORE_PRINTER_DRIVERA(i32);
pub struct CORE_PRINTER_DRIVERW(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 17u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_DATABLOCK: u32 = 21u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 19u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSFUNC_SET_RESULT: u32 = 9u32;
pub struct CPSUICBPARAM(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_ACTION_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_ABOUT: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_APPLYNOW: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_DLGPROC: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_EXTPUSH: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_SETACTIVE: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4u32;
pub struct CPSUIDATABLOCK(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUIF_ICONID_AS_HICON: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUI_CANCEL: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUI_OK: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUI_REBOOTSYSTEM: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CPSUI_RESTARTWINDOWS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_HEIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_HEIGHTOFFSET: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_MAX: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_ORIENTATION: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_WIDTH: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const CUSTOMPARAM_WIDTHOFFSET: u32 = 2u32;
pub struct CUSTOMSIZEPARAM(i32);
pub struct DATATYPES_INFO_1A(i32);
pub struct DATATYPES_INFO_1W(i32);
pub struct DATA_HEADER(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DEF_PRIORITY: u32 = 1u32;
pub struct DELETE_PORT_DATA_1(i32);
pub struct DEVICEPROPERTYHEADER(i32);
pub struct DEVQUERYPRINT_INFO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_BKSP_OK: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_NOITALIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_NOUNDER: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_NO_BOLD: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_NO_DOUBLE_UNDERLINE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_NO_STRIKETHRU: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_CAPSL: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_HPINTELLIFONT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_OEM1: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_OEM2: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_PST1: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_TYPE_TRUETYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DF_XM_CR: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT: u32 = 11800u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION: u32 = 12100u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_COUNT: u32 = 12101u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_GETAT: u32 = 12102u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_DRIVERPROPERTIES: u32 = 11803u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTERQUEUE: u32 = 11801u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTSCHEMATICKET: u32 = 11802u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_USERPROPERTIES: u32 = 11804u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENT: u32 = 12200u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS: u32 = 12000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_BIDINOTIFICATION: u32 = 12001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_DETAILEDREASONID: u32 = 12005u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REASONID: u32 = 12002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REQUEST: u32 = 12003u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_SOURCEAPPLICATION: u32 = 12004u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWMODAL: u32 = 12006u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWPARENT: u32 = 12007u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONDRIVEREVENT: u32 = 12201u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONPRINTERQUEUESENUMERATED: u32 = 12202u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST: u32 = 11900u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_CANCEL: u32 = 11901u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_COMPLETE: u32 = 11902u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG: u32 = 11400u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBOOL: u32 = 11401u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBYTES: u32 = 11407u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETINT32: u32 = 11403u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETREADSTREAM: u32 = 11409u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETSTRING: u32 = 11405u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETWRITESTREAM: u32 = 11410u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBOOL: u32 = 11402u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBYTES: u32 = 11408u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETINT32: u32 = 11404u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETSTRING: u32 = 11406u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE: u32 = 11600u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEEVENT: u32 = 11700u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEEVENT_ONBIDIRESPONSERECEIVED: u32 = 11701u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEVIEW: u32 = 12700u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT: u32 = 12800u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT_ONCHANGED: u32 = 12801u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUEVIEW_SETVIEWRANGE: u32 = 12701u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_GETPRINTERQUEUEVIEW: u32 = 11606u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_GETPROPERTIES: u32 = 11604u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_HANDLE: u32 = 11601u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_NAME: u32 = 11602u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDIQUERY: u32 = 11603u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDISETREQUESTASYNC: u32 = 11605u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG: u32 = 11500u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBOOL: u32 = 11501u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBYTES: u32 = 11507u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETINT32: u32 = 11503u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETREADSTREAM: u32 = 11509u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTREAMASXML: u32 = 11411u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTRING: u32 = 11505u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETWRITESTREAM: u32 = 11510u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBOOL: u32 = 11502u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBYTES: u32 = 11508u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETINT32: u32 = 11504u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETSTRING: u32 = 11506u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM: u32 = 11200u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_READ: u32 = 11201u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_WRITE: u32 = 11202u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM: u32 = 11300u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_COMMIT: u32 = 11301u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SEEK: u32 = 11302u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SETSIZE: u32 = 11303u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT: u32 = 12300u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_DRIVERPROPERTIES: u32 = 12301u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_QUEUEPROPERTIES: u32 = 12302u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_USERPROPERTIES: u32 = 12303u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTJOBCOLLECTION: u32 = 12600u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTJOBCOLLECTION_COUNT: u32 = 12601u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTJOBCOLLECTION_GETAT: u32 = 12602u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION: u32 = 10900u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT: u32 = 11100u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT_COMPLETED: u32 = 11101u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_CANCEL: u32 = 10902u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_START: u32 = 10901u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES: u32 = 10800u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE: u32 = 10802u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE_KEYNAME: u32 = 10801u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETOPTIONS: u32 = 10807u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETPARAMETERDEFINITION: u32 = 10808u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETSELECTEDOPTION: u32 = 10806u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMAXVALUE: u32 = 10805u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMINVALUE: u32 = 10804u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_PAGEIMAGEABLESIZE: u32 = 10803u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT: u32 = 10100u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT_DISPLAYNAME: u32 = 10101u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT: u32 = 10000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAME: u32 = 10002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAMESPACEURI: u32 = 10003u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_XMLNODE: u32 = 10001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_FEATURE: u32 = 10600u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_DISPLAYUI: u32 = 10604u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_GETOPTION: u32 = 10603u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTEDOPTION: u32 = 10601u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTIONTYPE: u32 = 10602u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION: u32 = 10400u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION_PAGESPERSHEET: u32 = 10401u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTION: u32 = 10200u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION: u32 = 10500u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_COUNT: u32 = 10501u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_GETAT: u32 = 10502u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTION_CONSTRAINED: u32 = 10202u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTION_GETPROPERTYVALUE: u32 = 10203u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_OPTION_SELECTED: u32 = 10201u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE: u32 = 10700u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_HEIGHT: u32 = 10706u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_WIDTH: u32 = 10705u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_HEIGHT: u32 = 10702u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_WIDTH: u32 = 10701u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_HEIGHT: u32 = 10704u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_WIDTH: u32 = 10703u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION: u32 = 10300u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_HEIGHT: u32 = 10302u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_WIDTH: u32 = 10301u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION: u32 = 12500u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_DATATYPE: u32 = 12503u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMAX: u32 = 12505u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMIN: u32 = 12504u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_UNITTYPE: u32 = 12502u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_USERINPUTREQUIRED: u32 = 12501u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER: u32 = 12400u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER_VALUE: u32 = 12401u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET: u32 = 11000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_COMMITASYNC: u32 = 11004u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETCAPABILITIES: u32 = 11006u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE: u32 = 11002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE_KEYNAME: u32 = 11001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETPARAMETERINITIALIZER: u32 = 11008u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_JOBCOPIESALLDOCUMENTS: u32 = 11007u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_NOTIFYXMLCHANGED: u32 = 11005u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DISPID_PRINTSCHEMA_TICKET_VALIDATEASYNC: u32 = 11003u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DI_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DI_MEMORYMAP_WRITE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DI_READ_SPOOL_JOB: u32 = 3u32;
pub struct DLGPAGE(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_BOOKLET_EDGE: u32 = 21u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_COLOR: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_COPIES_COLLATE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_DEFSOURCE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_DITHERTYPE: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_DUPLEX: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_FIRST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_FORMNAME: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_ICMINTENT: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_ICMMETHOD: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_LAST: u32 = 21u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_MANUAL_DUPLEX: u32 = 19u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_MEDIATYPE: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_NUP: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_NUP_DIRECTION: u32 = 18u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_ORIENTATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_OUTPUTBIN: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_PAGEORDER: u32 = 17u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_PRINTQUALITY: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_QUALITY: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_SCALE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_STAPLE: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_TTOPTION: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DMPUB_USER: u32 = 100u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_ADVANCED: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_INVALIDATE_DRIVER_CACHE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_NOPERMISSION: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_PROMPT_NON_MODAL: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DM_USER_DEFAULT: u32 = 64u32;
pub struct DOCEVENT_CREATEDCPRE(i32);
pub struct DOCEVENT_ESCAPE(i32);
pub struct DOCEVENT_FILTER(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ABORTDOC: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_CREATEDCPOST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_CREATEDCPRE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_DELETEDC: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ENDDOC: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ENDDOCPOST: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ENDDOCPRE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ENDPAGE: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_ESCAPE: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_FAILURE: i32 = -1i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_FIRST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_LAST: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_QUERYFILTER: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_RESETDCPOST: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_RESETDCPRE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_SPOOLED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_STARTDOC: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_STARTDOCPOST: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_STARTDOCPRE: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_STARTPAGE: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_UNSUPPORTED: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DOCUMENTEVENT_XPS_CANCELJOB: u32 = 6u32;
pub struct DOCUMENTPROPERTYHEADER(i32);
pub struct DOC_INFO_1A(i32);
pub struct DOC_INFO_1W(i32);
pub struct DOC_INFO_2A(i32);
pub struct DOC_INFO_2W(i32);
pub struct DOC_INFO_3A(i32);
pub struct DOC_INFO_3W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPD_DELETE_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPD_DELETE_UNUSED_FILES: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPF_ICONID_AS_HICON: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPF_USE_HDLGTEMPLATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DPS_NOPERMISSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DP_STD_DOCPROPPAGE1: u32 = 65533u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DP_STD_DOCPROPPAGE2: u32 = 65534u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DP_STD_RESERVED_START: u32 = 65520u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DP_STD_TREEVIEWPAGE: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DRIVER_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DRIVER_EVENT_INITIALIZE: u32 = 1u32;
pub struct DRIVER_INFO_1A(i32);
pub struct DRIVER_INFO_1W(i32);
pub struct DRIVER_INFO_2A(i32);
pub struct DRIVER_INFO_2W(i32);
pub struct DRIVER_INFO_3A(i32);
pub struct DRIVER_INFO_3W(i32);
pub struct DRIVER_INFO_4A(i32);
pub struct DRIVER_INFO_4W(i32);
pub struct DRIVER_INFO_5A(i32);
pub struct DRIVER_INFO_5W(i32);
pub struct DRIVER_INFO_6A(i32);
pub struct DRIVER_INFO_6W(i32);
pub struct DRIVER_INFO_8A(i32);
pub struct DRIVER_INFO_8W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DRIVER_KERNELMODE: u32 = 1u32;
pub struct DRIVER_UPGRADE_INFO_1(i32);
pub struct DRIVER_UPGRADE_INFO_2(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DRIVER_USERMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DSPRINT_PENDING: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DSPRINT_PUBLISH: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DSPRINT_REPUBLISH: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DSPRINT_UNPUBLISH: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const DSPRINT_UPDATE: u32 = 2u32;
pub struct EATTRIBUTE_DATATYPE(i32);
pub struct EBranchOfficeJobEventType(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_CHECKNAME_ONLY: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8u32;
pub struct EMFPLAYPROC(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EMF_PP_COLOR_OPTIMIZATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_ICONID_AS_HICON: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_INCL_SETUP_TITLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_NO_DOT_DOT_DOT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const EPF_USE_HDLGTEMPLATE: u32 = 128u32;
pub struct EPrintPropertyType(i32);
pub struct EPrintXPSJobOperation(i32);
pub struct EPrintXPSJobProgress(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: u32 = 13016u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: u32 = 13015u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: u32 = 13005u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SCHEMA_READ_ONLY: u32 = 13002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: u32 = 13010u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SERVER_OFFLINE: u32 = 13003u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: u32 = 13006u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: u32 = 13008u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: u32 = 13007u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: u32 = 13009u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_STATUS_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_STATUS_WARNING: u32 = 13001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: u32 = 13017u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: u32 = 13018u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_GETLASTERROR: i32 = -1i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_HINST: i32 = -5i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44i32;
pub struct EXTCHKBOX(i32);
pub struct EXTPUSH(i32);
pub struct EXTTEXTMETRIC(i32);
pub struct EXpsCompressionOptions(i32);
pub struct EXpsFontOptions(i32);
pub struct EXpsFontRestriction(i32);
pub struct EXpsJobConsumption(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const E_VERSION_NOT_SUPPORTED: u32 = 2147745793u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FG_CANCHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FILL_WITH_DEFAULTS: u32 = 1u32;
pub const FMTID_PrinterPropertyBag: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1979297226,
    data2: 2429,
    data3: 17859,
    data4: [166, 228, 186, 178, 158, 39, 111, 62],
};
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_CURRENTFONTID: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTBOLD: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTHEIGHT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTITALIC: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTMAXWIDTH: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTSTRIKETHRU: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTUNDERLINE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_FONTWIDTH: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_GRAYPERCENTAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_MAX: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_NEXTFONTID: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_NEXTGLYPH: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_PRINTDIRINCCDEGREES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_TEXTXRES: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FNT_INFO_TEXTYRES: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_DIR_SORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_DEVICEFONT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_GLYPHSET_GTT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_GLYPHSET_RLE: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_IFI: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_PERMANENT_SF: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_SOFTFONT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FONT_FL_UFM: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FORM_BUILTIN: u32 = 1u32;
pub struct FORM_INFO_1A(i32);
pub struct FORM_INFO_1W(i32);
pub struct FORM_INFO_2A(i32);
pub struct FORM_INFO_2W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FORM_PRINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const FORM_USER: u32 = 0u32;
pub struct GLYPHRUN(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const GPD_OEMCUSTOMDATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_IPPUSB_PRINT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4076077953,
    data2: 62573,
    data3: 20049,
    data4: [188, 231, 98, 222, 108, 242, 208, 152],
};
pub const GUID_DEVINTERFACE_USBPRINT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 685215661, data2: 23058, data3: 4561, data4: [174, 91, 0, 0, 248, 3, 168, 194] };
pub struct IAsyncGetSendNotificationCookie(i32);
pub struct IAsyncGetSrvReferralCookie(i32);
pub struct IBidiAsyncNotifyChannel(i32);
pub struct IBidiRequest(i32);
pub struct IBidiRequestContainer(i32);
pub struct IBidiRequestContainerVtbl(i32);
pub struct IBidiRequestVtbl(i32);
pub struct IBidiSpl(i32);
pub struct IBidiSpl2(i32);
pub struct IBidiSpl2Vtbl(i32);
pub struct IBidiSplVtbl(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ADVANCE: u32 = 64058u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_AUTOSEL: u32 = 64025u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_COLLATE: u32 = 64030u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_COLOR: u32 = 64040u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_COPY: u32 = 64046u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DEVICE: u32 = 64060u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DEVICE2: u32 = 64061u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DOCUMENT: u32 = 64059u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_EMPTY: u32 = 64000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ENVELOPE: u32 = 64010u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ERROR: u32 = 64050u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FALSE: u32 = 64005u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FAX: u32 = 64095u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FONTCART: u32 = 64013u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FONTSUB: u32 = 64081u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_GRAPHIC: u32 = 64057u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_HT_HOST: u32 = 64016u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_MEM: u32 = 64011u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_MONO: u32 = 64039u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_NO: u32 = 64003u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_OFF: u32 = 64007u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ON: u32 = 64008u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_OPTION: u32 = 64066u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_OPTION2: u32 = 64067u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_OUTBIN: u32 = 64055u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_OUTPUT: u32 = 64056u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PORTRAIT: u32 = 64022u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER: u32 = 64062u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER2: u32 = 64063u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER3: u32 = 64064u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER4: u32 = 64065u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_QUESTION: u32 = 64075u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RES_HIGH: u32 = 64037u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RES_LOW: u32 = 64035u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ROT_LAND: u32 = 64024u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_ROT_PORT: u32 = 64110u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_SCALING: u32 = 64045u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_SEL_NONE: u32 = 64001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_STD_FORM: u32 = 64054u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_STOP: u32 = 64068u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TELEPHONE: u32 = 64083u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TRUE: u32 = 64006u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_WARNING: u32 = 64002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_WATERMARK: u32 = 64049u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDI_CPSUI_YES: u32 = 64004u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ABOUT: u32 = 64848u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ADVANCED: u32 = 64722u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ALL: u32 = 64841u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BOND: u32 = 64786u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BOOKLET: u32 = 64873u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_CHANGE: u32 = 64702u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_CHANGED: u32 = 64846u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_CHANGES: u32 = 64845u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COARSE: u32 = 64787u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COLLATE: u32 = 64756u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COLLATED: u32 = 64757u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COLON_SEP: u32 = 64707u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COLOR: u32 = 64764u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COPIES: u32 = 64831u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_COPY: u32 = 64830u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEFAULT: u32 = 64732u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEVICE: u32 = 64842u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DITHERING: u32 = 64752u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DOCUMENT: u32 = 64715u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DRAFT: u32 = 64759u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_DUPLEX: u32 = 64745u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ERROR: u32 = 64733u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_EXIST: u32 = 64736u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FALSE: u32 = 64726u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FAST: u32 = 64838u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FAX: u32 = 64835u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FINE: u32 = 64788u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FORMNAME: u32 = 64747u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_GLOSSY: u32 = 64783u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_GRAPHIC: u32 = 64720u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_HALFTONE: u32 = 64791u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_HIGH: u32 = 64762u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM: u32 = 64748u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICMINTENT: u32 = 64750u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_NO: u32 = 64777u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ICM_YES: u32 = 64778u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LINEART: u32 = 64789u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LOW: u32 = 64760u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MAILBOX: u32 = 64829u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MAKE: u32 = 64833u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MANUALFEED: u32 = 64813u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MEDIA: u32 = 64751u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MEDIUM: u32 = 64761u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MONOCHROME: u32 = 64766u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_MORE: u32 = 64701u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NO: u32 = 64728u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NONE: u32 = 64734u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NOT: u32 = 64735u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NO_NAME: u32 = 64850u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP: u32 = 64864u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OF: u32 = 64704u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OFF: u32 = 64730u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ON: u32 = 64731u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ONLYONE: u32 = 64800u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OPTION: u32 = 64703u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OPTIONS: u32 = 64721u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ORIENTATION: u32 = 64738u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PAGEORDER: u32 = 64855u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PERCENT: u32 = 64711u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PLOT: u32 = 64836u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PORTRAIT: u32 = 64753u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_POSTER: u32 = 64874u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRESENTATION: u32 = 64763u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINT: u32 = 64834u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINTER: u32 = 64717u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_PROPERTIES: u32 = 64713u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_REGULAR: u32 = 64785u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RESET: u32 = 64840u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RESOLUTION: u32 = 64743u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_REVERT: u32 = 64844u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ROTATED: u32 = 64839u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ROT_LAND: u32 = 64755u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_ROT_PORT: u32 = 64886u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SCALING: u32 = 64739u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SETTING: u32 = 64851u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SETTINGS: u32 = 64843u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SETUP: u32 = 64700u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SIDE1: u32 = 64871u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SIDE2: u32 = 64872u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SIMPLEX: u32 = 64767u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLOT1: u32 = 64819u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLOT2: u32 = 64820u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLOT3: u32 = 64821u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLOT4: u32 = 64822u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SLOW: u32 = 64837u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_SOURCE: u32 = 64741u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STACKER: u32 = 64828u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STANDARD: u32 = 64782u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STAPLE: u32 = 64887u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STAPLER: u32 = 64825u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_STRID_LAST: u32 = 64892u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TO: u32 = 64706u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TOTAL: u32 = 64832u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TRUE: u32 = 64727u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TTOPTION: u32 = 64746u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_VERSION: u32 = 64849u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_VERTICAL: u32 = 64769u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_WARNING: u32 = 64847u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_WATERMARK: u32 = 64797u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IDS_CPSUI_YES: u32 = 64729u32;
pub struct IFixedDocument(i32);
pub struct IFixedDocumentSequence(i32);
pub struct IFixedDocumentSequenceVtbl(i32);
pub struct IFixedDocumentVtbl(i32);
pub struct IFixedPage(i32);
pub struct IFixedPageVtbl(i32);
pub struct IImgCreateErrorInfo(i32);
pub struct IImgCreateErrorInfoVtbl(i32);
pub struct IImgErrorInfo(i32);
pub struct IImgErrorInfoVtbl(i32);
pub struct IInterFilterCommunicator(i32);
pub struct IInterFilterCommunicatorVtbl(i32);
pub struct INSERTPSUIPAGE_INFO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3u32;
pub struct INVOC(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_ADD_CHILD_DEVICE: u32 = 2228316u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_ADD_MSIPP_COMPAT_ID: u32 = 2228308u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_CYCLE_PORT: u32 = 2228320u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_GET_1284_ID: u32 = 2228276u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_GET_INTERFACE_TYPE: u32 = 2228300u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_GET_LPT_STATUS: u32 = 2228272u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_GET_PROTOCOL: u32 = 2228292u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_SET_DEVICE_ID: u32 = 2228312u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_SET_PORT_NUMBER: u32 = 2228304u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_SET_PROTOCOL: u32 = 2228296u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_SOFT_RESET: u32 = 2228288u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_VENDOR_GET_COMMAND: u32 = 2228284u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IOCTL_USBPRINT_VENDOR_SET_COMMAND: u32 = 2228280u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const IPDFP_COPY_ALL_FILES: u32 = 1u32;
pub struct IPartBase(i32);
pub struct IPartBaseVtbl(i32);
pub struct IPartColorProfile(i32);
pub struct IPartColorProfileVtbl(i32);
pub struct IPartDiscardControl(i32);
pub struct IPartDiscardControlVtbl(i32);
pub struct IPartFont(i32);
pub struct IPartFont2(i32);
pub struct IPartFont2Vtbl(i32);
pub struct IPartFontVtbl(i32);
pub struct IPartImage(i32);
pub struct IPartImageVtbl(i32);
pub struct IPartPrintTicket(i32);
pub struct IPartPrintTicketVtbl(i32);
pub struct IPartResourceDictionary(i32);
pub struct IPartResourceDictionaryVtbl(i32);
pub struct IPartThumbnail(i32);
pub struct IPartThumbnailVtbl(i32);
pub struct IPrintAsyncCookie(i32);
pub struct IPrintAsyncNewChannelCookie(i32);
pub struct IPrintAsyncNotify(i32);
pub struct IPrintAsyncNotifyCallback(i32);
pub struct IPrintAsyncNotifyChannel(i32);
pub struct IPrintAsyncNotifyDataObject(i32);
pub struct IPrintAsyncNotifyRegistration(i32);
pub struct IPrintAsyncNotifyServerReferral(i32);
pub struct IPrintBidiAsyncNotifyRegistration(i32);
pub struct IPrintClassObjectFactory(i32);
pub struct IPrintClassObjectFactoryVtbl(i32);
pub struct IPrintCoreHelper(i32);
pub struct IPrintCoreHelperPS(i32);
pub struct IPrintCoreHelperUni(i32);
pub struct IPrintCoreHelperUni2(i32);
pub struct IPrintCoreUI2(i32);
pub struct IPrintJob(i32);
pub struct IPrintJobCollection(i32);
pub struct IPrintOemCommon(i32);
pub struct IPrintOemDriverUI(i32);
pub struct IPrintOemUI(i32);
pub struct IPrintOemUI2(i32);
pub struct IPrintOemUIMXDC(i32);
pub struct IPrintPipelineFilter(i32);
pub struct IPrintPipelineFilterVtbl(i32);
pub struct IPrintPipelineManagerControl(i32);
pub struct IPrintPipelineManagerControlVtbl(i32);
pub struct IPrintPipelineProgressReport(i32);
pub struct IPrintPipelineProgressReportVtbl(i32);
pub struct IPrintPipelinePropertyBag(i32);
pub struct IPrintPipelinePropertyBagVtbl(i32);
pub struct IPrintPreviewDxgiPackageTarget(i32);
pub struct IPrintReadStream(i32);
pub struct IPrintReadStreamFactory(i32);
pub struct IPrintReadStreamFactoryVtbl(i32);
pub struct IPrintReadStreamVtbl(i32);
pub struct IPrintSchemaAsyncOperation(i32);
pub struct IPrintSchemaAsyncOperationEvent(i32);
pub struct IPrintSchemaCapabilities(i32);
pub struct IPrintSchemaCapabilities2(i32);
pub struct IPrintSchemaDisplayableElement(i32);
pub struct IPrintSchemaElement(i32);
pub struct IPrintSchemaFeature(i32);
pub struct IPrintSchemaNUpOption(i32);
pub struct IPrintSchemaOption(i32);
pub struct IPrintSchemaOptionCollection(i32);
pub struct IPrintSchemaPageImageableSize(i32);
pub struct IPrintSchemaPageMediaSizeOption(i32);
pub struct IPrintSchemaParameterDefinition(i32);
pub struct IPrintSchemaParameterInitializer(i32);
pub struct IPrintSchemaTicket(i32);
pub struct IPrintSchemaTicket2(i32);
pub struct IPrintTicketProvider(i32);
pub struct IPrintTicketProvider2(i32);
pub struct IPrintUnidiAsyncNotifyRegistration(i32);
pub struct IPrintWriteStream(i32);
pub struct IPrintWriteStreamFlush(i32);
pub struct IPrintWriteStreamFlushVtbl(i32);
pub struct IPrintWriteStreamVtbl(i32);
pub struct IPrinterBidiSetRequestCallback(i32);
pub struct IPrinterExtensionAsyncOperation(i32);
pub struct IPrinterExtensionContext(i32);
pub struct IPrinterExtensionContextCollection(i32);
pub struct IPrinterExtensionEvent(i32);
pub struct IPrinterExtensionEventArgs(i32);
pub struct IPrinterExtensionManager(i32);
pub struct IPrinterExtensionRequest(i32);
pub struct IPrinterPropertyBag(i32);
pub struct IPrinterQueue(i32);
pub struct IPrinterQueue2(i32);
pub struct IPrinterQueueEvent(i32);
pub struct IPrinterQueueView(i32);
pub struct IPrinterQueueViewEvent(i32);
pub struct IPrinterScriptContext(i32);
pub struct IPrinterScriptablePropertyBag(i32);
pub struct IPrinterScriptablePropertyBag2(i32);
pub struct IPrinterScriptableSequentialStream(i32);
pub struct IPrinterScriptableStream(i32);
pub struct IXpsDocument(i32);
pub struct IXpsDocumentConsumer(i32);
pub struct IXpsDocumentConsumerVtbl(i32);
pub struct IXpsDocumentProvider(i32);
pub struct IXpsDocumentProviderVtbl(i32);
pub struct IXpsDocumentVtbl(i32);
pub struct IXpsPartIterator(i32);
pub struct IXpsPartIteratorVtbl(i32);
pub struct IXpsRasterizationFactory(i32);
pub struct IXpsRasterizationFactory1(i32);
pub struct IXpsRasterizationFactory2(i32);
pub struct IXpsRasterizer(i32);
pub struct IXpsRasterizerNotificationCallback(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_ACCESS_ADMINISTER: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_ACCESS_READ: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_CANCEL: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_DELETE: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_LAST_PAGE_EJECTED: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_RELEASE: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_RESTART: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_RETAIN: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_CONTROL_SENT_TO_PRINTER: u32 = 6u32;
pub struct JOB_INFO_1A(i32);
pub struct JOB_INFO_1W(i32);
pub struct JOB_INFO_2A(i32);
pub struct JOB_INFO_2W(i32);
pub struct JOB_INFO_3(i32);
pub struct JOB_INFO_4A(i32);
pub struct JOB_INFO_4W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_DATATYPE: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_DEVMODE: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_DOCUMENT: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: u32 = 21u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PARAMETERS: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PORT_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_POSITION: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: u32 = 24u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_START_TIME: u32 = 17u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_STATUS: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_STATUS_STRING: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_SUBMITTED: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_TIME: u32 = 19u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: u32 = 22u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: u32 = 18u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_FIELD_USER_NAME: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_NOTIFY_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_POSITION_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_BLOCKED_DEVQ: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_DELETED: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_DELETING: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_OFFLINE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_PAPEROUT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_PRINTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_PRINTING: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_RENDERING_LOCALLY: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_RESTART: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_RETAINED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_SPOOLING: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const JOB_STATUS_USER_INTERVENTION: u32 = 1024u32;
pub struct KERNDATA(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const LPR: u32 = 2u32;
pub struct MAPTABLE(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_ADDRESS_STR_LEN: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_CPSFUNC_INDEX: u32 = 26u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_DEVICEDESCRIPTION_STR_LEN: u32 = 257u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_DLGPAGE_COUNT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_IPADDR_STR_LEN: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_NETWORKNAME2_LEN: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_NETWORKNAME_LEN: u32 = 49u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_PORTNAME_LEN: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_PRIORITY: u32 = 99u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_QUEUENAME_LEN: u32 = 33u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_RES_STR_CHARS: u32 = 160u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MAX_SNMP_COMMUNITY_STR_LEN: u32 = 33u32;
pub struct MESSAGEBOX_PARAMS(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MIN_PRIORITY: u32 = 1u32;
pub struct MONITOR(i32);
pub struct MONITOR2(i32);
pub struct MONITOREX(i32);
pub struct MONITORINIT(i32);
pub struct MONITORREG(i32);
pub struct MONITORUI(i32);
pub struct MONITOR_INFO_1A(i32);
pub struct MONITOR_INFO_1W(i32);
pub struct MONITOR_INFO_2A(i32);
pub struct MONITOR_INFO_2W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_ADD: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_COMPOSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_DISABLE: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_DOUBLE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_DOUBLEBYTECHAR_MASK: u32 = 24u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_FORMAT_MASK: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_PAIRED: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_PREDEFIN_MASK: u32 = 224u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_REPLACE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MTYPE_SINGLE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_GRAPHICS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_PHYSICAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_RELATIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_SENDXMOVECMD: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_SENDYMOVECMD: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MV_UPDATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_GET_FILENAME: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC: u32 = 24u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC_SEQ: u32 = 22u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_PRINTTICKET_FIXED_PAGE: u32 = 26u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_SET_S0PAGE: u32 = 28u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_SET_S0PAGE_RESOURCE: u32 = 30u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDCOP_SET_XPSPASSTHRU_MODE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const MXDC_ESCAPE: u32 = 4122u32;
pub struct MxdcEscapeHeader(i32);
pub struct MxdcGetFileNameData(i32);
pub struct MxdcImageTypeEnums(i32);
pub struct MxdcLandscapeRotationEnums(i32);
pub struct MxdcPrintTicketEscape(i32);
pub struct MxdcPrintTicketPassthrough(i32);
pub struct MxdcS0PageData(i32);
pub struct MxdcS0PageEnums(i32);
pub struct MxdcS0PagePassthroughEscape(i32);
pub struct MxdcS0PageResourceEscape(i32);
pub struct MxdcXpsS0PageResource(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const NORMAL_PRINT: u32 = 0u32;
pub struct NOTIFICATION_CALLBACK_COMMANDS(i32);
pub struct NOTIFICATION_CONFIG_1(i32);
pub struct NOTIFICATION_CONFIG_FLAGS(i32);
pub const NOTIFICATION_RELEASE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3130675239, data2: 42766, data3: 19175, data4: [155, 125, 235, 62, 6, 173, 65, 87] };
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const NO_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const NO_COLOR_OPTIMIZATION: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const NO_PRIORITY: u32 = 0u32;
pub struct OEMCUIPCALLBACK(i32);
pub struct OEMCUIPPARAM(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMCUIP_DOCPROP: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMCUIP_PRNPROP: u32 = 2u32;
pub struct OEMDMPARAM(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMDM_CONVERT: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMDM_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMDM_MERGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMDM_SIZE: u32 = 1u32;
pub struct OEMFONTINSTPARAM(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_FREEMEM: u32 = 32769u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_JOBTIMEOUT: u32 = 32770u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_MAX: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_MAXBITMAP: u32 = 32774u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_MINOUTLINE: u32 = 32773u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_MIN_DOCSTICKY: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_MIN_PRINTERSTICKY: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PRINTFLAGS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PROTOCOL: u32 = 32772u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_CUSTOMSIZE: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_DIALECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_NUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_PSLEVEL: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_PSDM_TTDLFMT: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_UNIDM_FLAGS: u32 = 16385u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_UNIDM_GPDVER: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGDS_WAITTIMEOUT: u32 = 32771u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGI_GETINTERFACEVERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGI_GETPUBLISHERINFO: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGI_GETREQUESTEDHELPERINTERFACES: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGI_GETSIGNATURE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMGI_GETVERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMPUBLISH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMPUBLISH_IPRINTCOREHELPER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMTTY_INFO_CODEPAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMTTY_INFO_MARGINS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMTTY_INFO_NUM_UFMS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEMTTY_INFO_UFM_IDS: u32 = 4u32;
pub struct OEMUIOBJ(i32);
pub struct OEMUIPROCS(i32);
pub struct OEMUIPSPARAM(i32);
pub struct OEM_DMEXTRAHEADER(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OEM_MODE_PUBLISHER: u32 = 1u32;
pub struct OIEXT(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OIEXTF_ANSI_STRING: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTCF_HIDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTCF_MASK: u32 = 1u32;
pub struct OPTCOMBO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_CALLBACK: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_CHANGED: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_CHANGEONCE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_COLLAPSE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_DISABLED: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_ECB_CHECKED: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_EXT_DISABLED: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_EXT_HIDE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_HAS_POIEXT: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_HIDE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_INITIAL_TVITEM: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_MASK: i32 = 131071i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTIF_SEL_AS_HICON: i32 = 512i32;
pub struct OPTITEM(i32);
pub struct OPTPARAM(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_HIDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_MASK: u32 = 127u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTTF_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OPTTF_TYPE_DISABLED: u32 = 1u32;
pub struct OPTTYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_LBCB_SORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PDEV_ADJUST_PAPER_MARGIN_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PDEV_HOSTFONT_ENABLED_TYPE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PDEV_USE_TRUE_COLOR_TYPE: u32 = 3u32;
pub struct PFNCOMPROPSHEET(i32);
pub struct PFNPROPSHEETUI(i32);
pub struct PFN_DrvGetDriverSetting(i32);
pub struct PFN_DrvUpdateUISetting(i32);
pub struct PFN_DrvUpgradeRegistrySetting(i32);
pub struct PORT_DATA_1(i32);
pub struct PORT_DATA_2(i32);
pub struct PORT_DATA_LIST_1(i32);
pub struct PORT_INFO_1A(i32);
pub struct PORT_INFO_1W(i32);
pub struct PORT_INFO_2A(i32);
pub struct PORT_INFO_2W(i32);
pub struct PORT_INFO_3A(i32);
pub struct PORT_INFO_3W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_DOOR_OPEN: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_NO_TONER: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_OUTPUT_BIN_FULL: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_OUT_OF_MEMORY: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_PAPER_JAM: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_PAPER_OUT: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_PAPER_PROBLEM: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_POWER_SAVE: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_TONER_LOW: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_TYPE_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_TYPE_INFO: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_TYPE_WARNING: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_USER_INTERVENTION: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_STATUS_WARMING_UP: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_TYPE_NET_ATTACHED: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_TYPE_READ: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_TYPE_REDIRECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PORT_TYPE_WRITE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PPCAPS_BOOKLET_EDGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PPCAPS_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PPCAPS_RIGHT_THEN_DOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PPCAPS_SQUARE_SCALING: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ACCESS_ADMINISTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ACCESS_MANAGE_LIMITED: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ACCESS_USE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_FAX: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_LOCAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_MACHINE: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_NETWORK: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_PER_USER: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_PUBLISHED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_USER: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_QUEUED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_RAW_ONLY: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_SHARED: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_TS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_FORM: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_JOB: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_PORT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_PRINTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_ALL: u32 = 2138570751u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_CONFIGURE_PORT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_FORM: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_JOB: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_PORT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_FORM: u32 = 458752u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_JOB: u32 = 65280u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_PORT: u32 = 7340032u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_PRINTER: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_PRINTER_DRIVER: u32 = 1879048192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_PRINT_PROCESSOR: u32 = 117440512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_SERVER: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_SET_FORM: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_SET_JOB: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_SET_PRINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CHANGE_WRITE_JOB: u32 = 2048u32;
pub struct PRINTER_CONNECTION_INFO_1A(i32);
pub struct PRINTER_CONNECTION_INFO_1W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONNECTION_MISMATCH: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONNECTION_NO_UI: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONTROL_PURGE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_CONTROL_SET_STATUS: u32 = 4u32;
pub struct PRINTER_DEFAULTSA(i32);
pub struct PRINTER_DEFAULTSW(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_3D: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_CLOUD: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_FAX: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_FILE: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_SERVICE: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_CLASS: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_DERIVED: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_NOT_SHAREABLE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_PACKAGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_SANDBOX_DISABLED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_SANDBOX_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_DRIVER_XPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_CATEGORY_3D: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_CATEGORY_ALL: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_CONNECTIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_CONTAINER: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_EXPAND: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_FAVORITE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_HIDE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON1: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON2: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON3: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON4: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON5: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON6: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON7: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICON8: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_ICONMASK: u32 = 16711680u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_NAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_NETWORK: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_REMOTE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ENUM_SHARED: u32 = 32u32;
pub struct PRINTER_ENUM_VALUESA(i32);
pub struct PRINTER_ENUM_VALUESW(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_JAM: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_OUTOFPAPER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_OUTOFTONER: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_SEVERE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_ERROR_WARNING: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_ADD_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_ADD_CONNECTION_NO_UI: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_ATTRIBUTES_CHANGED: u32 = 7u32;
pub struct PRINTER_EVENT_ATTRIBUTES_INFO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_CACHE_DELETE: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_CACHE_REFRESH: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_CONFIGURATION_CHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_CONFIGURATION_UPDATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_DELETE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION_NO_UI: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_FLAG_NO_UI: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_EVENT_INITIALIZE: u32 = 3u32;
pub const PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1566185220,
    data2: 57297,
    data3: 16769,
    data4: [142, 238, 129, 92, 134, 237, 173, 49],
};
pub const PRINTER_EXTENSION_REASON_DRIVER_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 599462696,
    data2: 25566,
    data3: 17043,
    data4: [145, 91, 166, 162, 61, 146, 154, 203],
};
pub const PRINTER_EXTENSION_REASON_PRINT_PREFERENCES: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3968804383, data2: 9852, data3: 18079, data4: [181, 214, 57, 51, 2, 60, 41, 204] };
pub struct PRINTER_INFO_1A(i32);
pub struct PRINTER_INFO_1W(i32);
pub struct PRINTER_INFO_2A(i32);
pub struct PRINTER_INFO_2W(i32);
pub struct PRINTER_INFO_3(i32);
pub struct PRINTER_INFO_4A(i32);
pub struct PRINTER_INFO_4W(i32);
pub struct PRINTER_INFO_5A(i32);
pub struct PRINTER_INFO_5W(i32);
pub struct PRINTER_INFO_6(i32);
pub struct PRINTER_INFO_7A(i32);
pub struct PRINTER_INFO_7W(i32);
pub struct PRINTER_INFO_8A(i32);
pub struct PRINTER_INFO_8W(i32);
pub struct PRINTER_INFO_9A(i32);
pub struct PRINTER_INFO_9W(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_CATEGORY_3D: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_CATEGORY_ALL: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: u32 = 13u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: u32 = 21u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: u32 = 28u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: u32 = 25u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_CJOBS: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_COMMENT: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_DATATYPE: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_DEVMODE: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: u32 = 27u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_LOCATION: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: u32 = 26u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_SEPFILE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_START_TIME: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS: u32 = 18u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: u32 = 19u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: u32 = 24u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: u32 = 22u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: u32 = 17u32;
pub struct PRINTER_NOTIFY_INFO(i32);
pub struct PRINTER_NOTIFY_INFO_DATA(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_INFO_DATA_COMPACT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1u32;
pub struct PRINTER_NOTIFY_INIT(i32);
pub struct PRINTER_NOTIFY_OPTIONS(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1u32;
pub struct PRINTER_NOTIFY_OPTIONS_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_STATUS_ENDPOINT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_STATUS_INFO: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_STATUS_POLL: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_NOTIFY_TYPE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_OEMINTF_VERSION: u32 = 65536u32;
pub struct PRINTER_OPTIONSA(i32);
pub struct PRINTER_OPTIONSW(i32);
pub struct PRINTER_OPTION_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_BUSY: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_DOOR_OPEN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_INITIALIZING: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_IO_ACTIVE: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_MANUAL_FEED: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_NOT_AVAILABLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_NO_TONER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_OFFLINE: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_OUT_OF_MEMORY: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PAGE_PUNT: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PAPER_JAM: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PAPER_OUT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PAPER_PROBLEM: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PENDING_DELETION: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_POWER_SAVE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PRINTING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_PROCESSING: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_SERVER_OFFLINE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_SERVER_UNKNOWN: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_TONER_LOW: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_USER_INTERVENTION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_WAITING: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PRINTER_STATUS_WARMING_UP: u32 = 65536u32;
pub struct PRINTIFI32(i32);
pub struct PRINTPROCESSOROPENDATA(i32);
pub struct PRINTPROCESSOR_CAPS_1(i32);
pub struct PRINTPROCESSOR_CAPS_2(i32);
pub struct PRINTPROCESSOR_INFO_1A(i32);
pub struct PRINTPROCESSOR_INFO_1W(i32);
pub struct PRINTPROVIDOR(i32);
pub const PRINT_APP_BIDI_NOTIFY_CHANNEL: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 716886563,
    data2: 47508,
    data3: 19146,
    data4: [130, 252, 69, 113, 177, 181, 133, 172],
};
pub struct PRINT_EXECUTION_CONTEXT(i32);
pub struct PRINT_EXECUTION_DATA(i32);
pub struct PRINT_FEATURE_OPTION(i32);
pub const PRINT_PORT_MONITOR_NOTIFY_CHANNEL: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 635386638,
    data2: 29865,
    data3: 18421,
    data4: [128, 206, 121, 180, 177, 235, 92, 88],
};
pub struct PROPSHEETUI_GETICON_INFO(i32);
pub struct PROPSHEETUI_INFO(i32);
pub struct PROPSHEETUI_INFO_HEADER(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_INFO_VERSION: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_INIT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROTOCOL_LPR_TYPE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROTOCOL_RAWTCP_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PROTOCOL_UNKNOWN_TYPE: u32 = 0u32;
pub struct PROVIDOR_INFO_1A(i32);
pub struct PROVIDOR_INFO_1W(i32);
pub struct PROVIDOR_INFO_2A(i32);
pub struct PROVIDOR_INFO_2W(i32);
pub struct PSCRIPT5_PRIVATE_DEVMODE(i32);
pub struct PSPINFO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_DEFTITLE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_OBSOLETE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_PROPTITLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIHDRF_USEHICON: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIINFO_UNICODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_DLL: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3u32;
pub struct PUBLISHERINFO(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3u32;
pub struct PageCountType(i32);
pub struct PrintAsyncNotifyConversationStyle(i32);
pub struct PrintAsyncNotifyError(i32);
pub struct PrintAsyncNotifyUserFilter(i32);
pub struct PrintJobStatus(i32);
pub struct PrintNamedProperty(i32);
pub struct PrintPropertiesCollection(i32);
pub struct PrintPropertyValue(i32);
pub struct PrintSchemaAsyncOperation(i32);
pub struct PrintSchemaConstrainedSetting(i32);
pub struct PrintSchemaParameterDataType(i32);
pub struct PrintSchemaSelectionType(i32);
pub struct PrinterExtensionManager(i32);
pub struct PrinterQueue(i32);
pub struct PrinterQueueView(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const QCP_DEVICEPROFILE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const QCP_PROFILEDISK: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const QCP_PROFILEMEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const QCP_SOURCEPROFILE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const RAWTCP: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const REVERSE_PRINT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const RIGHT_THEN_DOWN: u32 = 1u32;
pub struct ROUTER_NOTIFY_CALLBACK(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ROUTER_STOP_ROUTING: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ROUTER_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const ROUTER_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SERVER_ACCESS_ADMINISTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SERVER_ACCESS_ENUMERATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SERVER_NOTIFY_TYPE: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SETOPTIONS_FLAG_KEEP_CONFLICT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SETOPTIONS_FLAG_RESOLVE_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SETOPTIONS_RESULT_CONFLICT_REMAINED: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SETOPTIONS_RESULT_CONFLICT_RESOLVED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SETOPTIONS_RESULT_NO_CONFLICT: u32 = 0u32;
pub struct SETRESULT_INFO(i32);
pub struct SHIMOPTS(i32);
pub struct SHOWUIPARAMS(i32);
pub struct SIMULATE_CAPS_1(i32);
pub struct SPLCLIENT_INFO_1(i32);
pub struct SPLCLIENT_INFO_3_VISTA(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SPOOL_FILE_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SPOOL_FILE_TEMPORARY: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SR_OWNER: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SR_OWNER_PARENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SSP_STDPAGE1: u32 = 10001u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SSP_STDPAGE2: u32 = 10002u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const SSP_TVPAGE: u32 = 10000u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const STRING_LANGPAIR: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const STRING_MUIDLL: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const STRING_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const S_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const S_DEVCAP_OUTPUT_FULL_REPLACEMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(318465i32 as _);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const S_NO_CONFLICT: u32 = 262145u32;
pub struct TRANSDATA(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TTDOWNLOAD_DONTCARE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TTDOWNLOAD_GRAPHICS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TTDOWNLOAD_TTOUTLINE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_2STATES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_3STATES: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_CHKBOX: u32 = 9u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_COMBOBOX: u32 = 6u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_EDITBOX: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_LISTBOX: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_NSTATES_EX: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_PUSHBUTTON: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_SCROLLBAR: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_TRACKBAR: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TVOT_UDARROW: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TYPE_GLYPHHANDLE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TYPE_GLYPHID: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TYPE_TRANSDATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const TYPE_UNICODE: u32 = 1u32;
pub struct UFF_FILEHEADER(i32);
pub struct UFF_FONTDIRECTORY(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFF_VERSION_NUMBER: u32 = 65537u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFM_CART: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFM_SCALABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFM_SOFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTDOWNLOAD_TTOUTLINE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTFONT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTOUTLINE_BOLD_SIM: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTOUTLINE_ITALIC_SIM: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTOUTLINE_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFOFLAG_TTSUBSTITUTED: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_FONTOBJ: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_GLYPHBITMAP: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_GLYPHSTRING: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_GLYPHWIDTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_MEMORY: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UFO_GETINFO_STDVARIABLE: u32 = 6u32;
pub struct UI_TYPE(i32);
pub struct UNIDRVINFO(i32);
pub struct UNIDRV_PRIVATE_DEVMODE(i32);
pub struct UNIFM_HDR(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UNIFM_VERSION_1_0: u32 = 65536u32;
pub struct UNI_CODEPAGEINFO(i32);
pub struct UNI_GLYPHSETDATA(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UNI_GLYPHSETDATA_VERSION_1_0: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UNKNOWN_PROTOCOL: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UPDP_SILENT_UPLOAD: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const UPDP_UPLOAD_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const USBPRINT_IOCTL_INDEX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const USB_PRINTER_INTERFACE_CLASSIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const USB_PRINTER_INTERFACE_DUAL: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const USB_PRINTER_INTERFACE_IPP: u32 = 2u32;
pub struct USERDATA(i32);
pub struct WIDTHRUN(i32);
pub struct WIDTHTABLE(i32);
#[doc = "*Required features: `Win32_Graphics_Printing`*"]
pub const WM_FI_FILENAME: u32 = 900u32;
pub struct XPSRAS_BACKGROUND_COLOR(i32);
pub struct XPSRAS_PIXEL_FORMAT(i32);
pub struct XPSRAS_RENDERING_MODE(i32);
pub struct _CPSUICALLBACK(i32);
pub struct _SPLCLIENT_INFO_2_V1(i32);
pub struct _SPLCLIENT_INFO_2_V2(i32);
pub struct _SPLCLIENT_INFO_2_V2(i32);
pub struct _SPLCLIENT_INFO_2_V3(i32);
pub struct __MIDL___MIDL_itf_imgerror_0000_0000_0001(i32);
