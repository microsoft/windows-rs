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
    pub fn AddPrintDeviceObject(hprinter: super::super::Foundation::HANDLE, phdeviceobject: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
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
    pub fn CommonPropertySheetUIA(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIW(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
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
    pub fn CorePrinterDriverInstalledA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, coredriverguid: ::windows::runtime::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, coredriverguid: ::windows::runtime::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrintAsyncNotifyChannel(pszname: super::super::Foundation::PWSTR, pnotificationtype: *const ::windows::runtime::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: ::windows::runtime::RawPtr, ppiasynchnotification: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
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
    pub fn DeletePrinterDriverPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
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
    pub fn GetCorePrinterDriversA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, pszzcoredriverdependencies: super::super::Foundation::PSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, pszzcoredriverdependencies: super::super::Foundation::PWSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERW) -> ::windows::runtime::HRESULT;
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
    pub fn GetPrintOutputInfo(hwnd: super::super::Foundation::HWND, pszprinter: super::super::Foundation::PWSTR, phfile: *mut super::super::Foundation::HANDLE, ppszoutputfile: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
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
    pub fn GetPrinterDriverPackagePathA(pszserver: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, pszlanguage: super::super::Foundation::PSTR, pszpackageid: super::super::Foundation::PSTR, pszdriverpackagecab: super::super::Foundation::PSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverPackagePathW(pszserver: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, pszlanguage: super::super::Foundation::PWSTR, pszpackageid: super::super::Foundation::PWSTR, pszdriverpackagecab: super::super::Foundation::PWSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows::runtime::HRESULT;
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
    pub fn InstallPrinterDriverFromPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszdrivername: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPrinterDriverFromPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszdrivername: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
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
    pub fn RegisterForPrintAsyncNotifications(pszname: super::super::Foundation::PWSTR, pnotificationtype: *const ::windows::runtime::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: ::windows::runtime::RawPtr, phnotify: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePrintDeviceObject(hdeviceobject: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwchangeflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotificationEx(hnotify: super::super::Foundation::HANDLE, dwcolor: u32, fdwflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportJobProcessingProgress(printerhandle: super::super::Foundation::HANDLE, jobid: u32, joboperation: EPrintXPSJobOperation, jobprogress: EPrintXPSJobProgress) -> ::windows::runtime::HRESULT;
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
    pub fn UnRegisterForPrintAsyncNotifications(param0: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePrintDeviceObject(hprinter: super::super::Foundation::HANDLE, hdeviceobject: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageA(pszserver: super::super::Foundation::PSTR, pszinfpath: super::super::Foundation::PSTR, pszenvironment: super::super::Foundation::PSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: super::super::Foundation::PSTR, pcchdestinfpath: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageW(pszserver: super::super::Foundation::PWSTR, pszinfpath: super::super::Foundation::PWSTR, pszenvironment: super::super::Foundation::PWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: super::super::Foundation::PWSTR, pcchdestinfpath: *mut u32) -> ::windows::runtime::HRESULT;
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
