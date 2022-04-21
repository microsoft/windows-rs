#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormA(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormW(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobA(hprinter: super::super::Foundation::HANDLE, level: u32, pdata: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobW(hprinter: super::super::Foundation::HANDLE, level: u32, pdata: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorA(pname: ::windows_sys::core::PCSTR, level: u32, pmonitors: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorW(pname: ::windows_sys::core::PCWSTR, level: u32, pmonitors: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortA(pname: ::windows_sys::core::PCSTR, hwnd: super::super::Foundation::HWND, pmonitorname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortW(pname: ::windows_sys::core::PCWSTR, hwnd: super::super::Foundation::HWND, pmonitorname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintDeviceObject(hprinter: super::super::Foundation::HANDLE, phdeviceobject: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, ppathname: ::windows_sys::core::PCSTR, pprintprocessorname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, ppathname: ::windows_sys::core::PCWSTR, pprintprocessorname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorA(pname: ::windows_sys::core::PCSTR, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorW(pname: ::windows_sys::core::PCWSTR, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterA(pname: ::windows_sys::core::PCSTR, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2A(hwnd: super::super::Foundation::HWND, pszname: ::windows_sys::core::PCSTR, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2W(hwnd: super::super::Foundation::HWND, pszname: ::windows_sys::core::PCWSTR, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionA(pname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionW(pname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverA(pname: ::windows_sys::core::PCSTR, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExA(pname: ::windows_sys::core::PCSTR, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExW(pname: ::windows_sys::core::PCWSTR, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverW(pname: ::windows_sys::core::PCWSTR, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterW(pname: ::windows_sys::core::PCWSTR, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesA(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_sys::core::PCSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevmodeinput: *const super::Gdi::DEVMODEA) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesW(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_sys::core::PCWSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEW, pdevmodeinput: *const super::Gdi::DEVMODEW) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendPrinterNotifyInfoData(pinfodest: *const PRINTER_NOTIFY_INFO, pdatasrc: *const PRINTER_NOTIFY_INFO_DATA, fdwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallRouterFindFirstPrinterChangeNotification(hprinterrpc: super::super::Foundation::HANDLE, fdwfilterflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: *const PRINTER_NOTIFY_OPTIONS) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseSpoolFileHandle(hprinter: super::super::Foundation::HANDLE, hspoolfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitSpoolData(hprinter: super::super::Foundation::HANDLE, hspoolfile: super::super::Foundation::HANDLE, cbcommit: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIA(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIW(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortA(pname: ::windows_sys::core::PCSTR, hwnd: super::super::Foundation::HWND, pportname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortW(pname: ::windows_sys::core::PCWSTR, hwnd: super::super::Foundation::HWND, pportname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConnectToPrinterDlg(hwnd: super::super::Foundation::HWND, flags: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledA(pszserver: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR, coredriverguid: ::windows_sys::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledW(pszserver: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR, coredriverguid: ::windows_sys::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64, pbdriverinstalled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn CreatePrintAsyncNotifyChannel(pszname: ::windows_sys::core::PCWSTR, pnotificationtype: *const ::windows_sys::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: IPrintAsyncNotifyCallback, ppiasynchnotification: *mut IPrintAsyncNotifyChannel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreatePrinterIC(hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEW) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormA(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormW(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteJobNamedProperty(hprinter: super::super::Foundation::HANDLE, jobid: u32, pszname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, pmonitorname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, pmonitorname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortA(pname: ::windows_sys::core::PCSTR, hwnd: super::super::Foundation::HWND, pportname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortW(pname: ::windows_sys::core::PCWSTR, hwnd: super::super::Foundation::HWND, pportname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, pprintprocessorname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, pprintprocessorname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, pprintprovidorname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, pprintprovidorname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionA(pname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionW(pname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR, pvaluename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR, pvaluename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, pdrivername: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, pdrivername: ::windows_sys::core::PCSTR, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, pdrivername: ::windows_sys::core::PCWSTR, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn DeletePrinterDriverPackageA(pszserver: ::windows_sys::core::PCSTR, pszinfpath: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn DeletePrinterDriverPackageW(pszserver: ::windows_sys::core::PCWSTR, pszinfpath: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, pdrivername: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterIC(hprinteric: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrint(hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, presid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrintEx(pdqpinfo: *mut DEVQUERYPRINT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesA(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_sys::core::PCSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevmodeinput: *const super::Gdi::DEVMODEA, fmode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesW(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_sys::core::PCWSTR, pdevmodeoutput: *mut super::Gdi::DEVMODEW, pdevmodeinput: *const super::Gdi::DEVMODEW, fmode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDocPrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPagePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsA(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsW(hprinter: super::super::Foundation::HANDLE, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobNamedProperties(hprinter: super::super::Foundation::HANDLE, jobid: u32, pcproperties: *mut u32, ppproperties: *mut *mut PrintNamedProperty) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsA(hprinter: super::super::Foundation::HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsW(hprinter: super::super::Foundation::HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsA(pname: ::windows_sys::core::PCSTR, level: u32, pmonitor: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsW(pname: ::windows_sys::core::PCWSTR, level: u32, pmonitor: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsA(pname: ::windows_sys::core::PCSTR, level: u32, pport: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsW(pname: ::windows_sys::core::PCWSTR, level: u32, pport: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesA(pname: ::windows_sys::core::PCSTR, pprintprocessorname: ::windows_sys::core::PCSTR, level: u32, pdatatypes: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesW(pname: ::windows_sys::core::PCWSTR, pprintprocessorname: ::windows_sys::core::PCWSTR, level: u32, pdatatypes: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataA(hprinter: super::super::Foundation::HANDLE, dwindex: u32, pvaluename: ::windows_sys::core::PSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: *mut u32, pdata: *mut u8, cbdata: u32, pcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR, penumvalues: *mut u8, cbenumvalues: u32, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR, penumvalues: *mut u8, cbenumvalues: u32, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataW(hprinter: super::super::Foundation::HANDLE, dwindex: u32, pvaluename: ::windows_sys::core::PWSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: *mut u32, pdata: *mut u8, cbdata: u32, pcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR, psubkey: ::windows_sys::core::PSTR, cbsubkey: u32, pcbsubkey: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR, psubkey: ::windows_sys::core::PWSTR, cbsubkey: u32, pcbsubkey: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersA(flags: u32, name: ::windows_sys::core::PCSTR, level: u32, pprinterenum: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersW(flags: u32, name: ::windows_sys::core::PCWSTR, level: u32, pprinterenum: *mut u8, cbbuf: u32, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ExtDeviceMode(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HANDLE, pdevmodeoutput: *mut super::Gdi::DEVMODEA, pdevicename: ::windows_sys::core::PCSTR, pport: ::windows_sys::core::PCSTR, pdevmodeinput: *const super::Gdi::DEVMODEA, pprofile: ::windows_sys::core::PCSTR, fmode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClosePrinterChangeNotification(hchange: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwfilter: u32, fdwoptions: u32, pprinternotifyoptions: *const ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextPrinterChangeNotification(hchange: super::super::Foundation::HANDLE, pdwchange: *mut u32, pvreserved: *const ::core::ffi::c_void, ppprinternotifyinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushPrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *const ::core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32, csleep: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn FreePrintNamedPropertyArray(cproperties: u32, ppproperties: *mut *mut PrintNamedProperty);
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn FreePrintPropertyValue(pvalue: *mut PrintPropertyValue);
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrinterNotifyInfo(pprinternotifyinfo: *const PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiDeleteSpoolFileHandle(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndDocEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, dwoptimization: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDC(spoolfilehandle: super::super::Foundation::HANDLE) -> super::Gdi::HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDevmodeForPage(spoolfilehandle: super::super::Foundation::HANDLE, dwpagenumber: u32, pcurrdm: *mut *mut super::Gdi::DEVMODEW, plastdm: *mut *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageCount(spoolfilehandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageHandle(spoolfilehandle: super::super::Foundation::HANDLE, page: u32, pdwpagetype: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetSpoolFileHandle(pwszprintername: ::windows_sys::core::PCWSTR, pdevmode: *mut super::Gdi::DEVMODEW, pwszdocname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiPlayPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, hemf: super::super::Foundation::HANDLE, prectdocument: *mut super::super::Foundation::RECT, prectborder: *mut super::super::Foundation::RECT, prectclip: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiResetDCEMF(spoolfilehandle: super::super::Foundation::HANDLE, pcurrdm: *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn GdiStartDocEMF(spoolfilehandle: super::super::Foundation::HANDLE, pdocinfo: *mut super::super::Storage::Xps::DOCINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiStartPageEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn GenerateCopyFilePaths(pszprintername: ::windows_sys::core::PCWSTR, pszdirectory: ::windows_sys::core::PCWSTR, psplclientinfo: *const u8, dwlevel: u32, pszsourcedir: ::windows_sys::core::PWSTR, pcchsourcedirsize: *mut u32, psztargetdir: ::windows_sys::core::PWSTR, pcchtargetdirsize: *mut u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPSUIUserData(hdlg: super::super::Foundation::HWND) -> usize;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversA(pszserver: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR, pszzcoredriverdependencies: ::windows_sys::core::PCSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversW(pszserver: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR, pszzcoredriverdependencies: ::windows_sys::core::PCWSTR, ccoreprinterdrivers: u32, pcoreprinterdrivers: *mut CORE_PRINTER_DRIVERW) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterA(pszbuffer: ::windows_sys::core::PSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterW(pszbuffer: ::windows_sys::core::PWSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormA(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCSTR, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormW(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCWSTR, level: u32, pform: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobA(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributes(pprintername: ::windows_sys::core::PCWSTR, pdevmode: *const super::Gdi::DEVMODEW, pattributeinfo: *mut ATTRIBUTE_INFO_3) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributesEx(pprintername: ::windows_sys::core::PCWSTR, pdevmode: *const super::Gdi::DEVMODEW, dwlevel: u32, pattributeinfo: *mut u8, nsize: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobNamedPropertyValue(hprinter: super::super::Foundation::HANDLE, jobid: u32, pszname: ::windows_sys::core::PCWSTR, pvalue: *mut PrintPropertyValue) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobW(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintExecutionData(pdata: *mut PRINT_EXECUTION_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintOutputInfo(hwnd: super::super::Foundation::HWND, pszprinter: ::windows_sys::core::PCWSTR, phfile: *mut super::super::Foundation::HANDLE, ppszoutputfile: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, level: u32, pprintprocessorinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR, pvaluename: ::windows_sys::core::PCSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR, pvaluename: ::windows_sys::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, nsize: u32, pcbneeded: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2A(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, penvironment: ::windows_sys::core::PCSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2W(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE, penvironment: ::windows_sys::core::PCWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverA(hprinter: super::super::Foundation::HANDLE, penvironment: ::windows_sys::core::PCSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryA(pname: ::windows_sys::core::PCSTR, penvironment: ::windows_sys::core::PCSTR, level: u32, pdriverdirectory: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryW(pname: ::windows_sys::core::PCWSTR, penvironment: ::windows_sys::core::PCWSTR, level: u32, pdriverdirectory: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn GetPrinterDriverPackagePathA(pszserver: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR, pszlanguage: ::windows_sys::core::PCSTR, pszpackageid: ::windows_sys::core::PCSTR, pszdriverpackagecab: ::windows_sys::core::PSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn GetPrinterDriverPackagePathW(pszserver: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR, pszlanguage: ::windows_sys::core::PCWSTR, pszpackageid: ::windows_sys::core::PCWSTR, pszdriverpackagecab: ::windows_sys::core::PWSTR, cchdriverpackagecab: u32, pcchrequiredsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverW(hprinter: super::super::Foundation::HANDLE, penvironment: ::windows_sys::core::PCWSTR, level: u32, pdriverinfo: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *mut u8, cbbuf: u32, pcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSpoolFileHandle(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonatePrinterClient(htoken: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn InstallPrinterDriverFromPackageA(pszserver: ::windows_sys::core::PCSTR, pszinfpath: ::windows_sys::core::PCSTR, pszdrivername: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn InstallPrinterDriverFromPackageW(pszserver: ::windows_sys::core::PCWSTR, pszinfpath: ::windows_sys::core::PCWSTR, pszdrivername: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeA(pdevmode: *const super::Gdi::DEVMODEA, devmodesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeW(pdevmode: *const super::Gdi::DEVMODEW, devmodesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2A(pprintername: ::windows_sys::core::PCSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA, poptions: *const PRINTER_OPTIONSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2W(pprintername: ::windows_sys::core::PCWSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW, poptions: *const PRINTER_OPTIONSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterA(pprintername: ::windows_sys::core::PCSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterW(pprintername: ::windows_sys::core::PCWSTR, phprinter: *mut super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PartialReplyPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, pdatasrc: *const PRINTER_NOTIFY_INFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayGdiScriptOnPrinterIC(hprinteric: super::super::Foundation::HANDLE, pin: *const u8, cin: u32, pout: *mut u8, cout: u32, ul: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxA(hprinter: super::super::Foundation::HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: ::windows_sys::core::PCSTR, pcaption: ::windows_sys::core::PCSTR, dwtype: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxW(hprinter: super::super::Foundation::HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: ::windows_sys::core::PCWSTR, pcaption: ::windows_sys::core::PCWSTR, dwtype: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterProperties(hwnd: super::super::Foundation::HWND, hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindClosePrinterChangeNotification(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: *const ::core::ffi::c_void, pvreserved1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadPrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *mut ::core::ffi::c_void, cbbuf: u32, pnobytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForPrintAsyncNotifications(pszname: ::windows_sys::core::PCWSTR, pnotificationtype: *const ::windows_sys::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: IPrintAsyncNotifyCallback, phnotify: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePrintDeviceObject(hdeviceobject: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwchangeflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotificationEx(hnotify: super::super::Foundation::HANDLE, dwcolor: u32, fdwflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportJobProcessingProgress(printerhandle: super::super::Foundation::HANDLE, jobid: u32, joboperation: EPrintXPSJobOperation, jobprogress: EPrintXPSJobProgress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterA(hprinter: super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterW(hprinter: super::super::Foundation::HANDLE, pdefault: *const PRINTER_DEFAULTSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToPrinterSelf() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn RouterAllocBidiMem(numbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAllocBidiResponseContainer(count: u32) -> *mut BIDI_RESPONSE_CONTAINER;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata: u32) -> *mut PRINTER_NOTIFY_INFO;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn RouterFreeBidiMem(pmempointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreeBidiResponseContainer(pdata: *const BIDI_RESPONSE_CONTAINER) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScheduleJob(hprinter: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCPSUIUserData(hdlg: super::super::Foundation::HWND, cpsuiuserdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterA(pszprinter: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterW(pszprinter: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormA(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCSTR, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormW(hprinter: super::super::Foundation::HANDLE, pformname: ::windows_sys::core::PCWSTR, level: u32, pform: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobA(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobNamedProperty(hprinter: super::super::Foundation::HANDLE, jobid: u32, pproperty: *const PrintNamedProperty) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobW(hprinter: super::super::Foundation::HANDLE, jobid: u32, level: u32, pjob: *mut u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortA(pname: ::windows_sys::core::PCSTR, pportname: ::windows_sys::core::PCSTR, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortW(pname: ::windows_sys::core::PCWSTR, pportname: ::windows_sys::core::PCWSTR, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *const u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataA(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExA(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCSTR, pvaluename: ::windows_sys::core::PCSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExW(hprinter: super::super::Foundation::HANDLE, pkeyname: ::windows_sys::core::PCWSTR, pvaluename: ::windows_sys::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataW(hprinter: super::super::Foundation::HANDLE, pvaluename: ::windows_sys::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pprinter: *const u8, command: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplIsSessionZero(hprinter: super::super::Foundation::HANDLE, jobid: u32, pissessionzero: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplPromptUIInUsersSession(hprinter: super::super::Foundation::HANDLE, jobid: u32, puiparams: *const SHOWUIPARAMS, presponse: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerCopyFileEvent(pszprintername: ::windows_sys::core::PCWSTR, pszkey: ::windows_sys::core::PCWSTR, dwcopyfileevent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindClosePrinterChangeNotification(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindFirstPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, fdwfilterflags: u32, fdwoptions: u32, pprinternotifyoptions: *const ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void, pnotificationconfig: *const ::core::ffi::c_void, phnotify: *mut super::super::Foundation::HANDLE, phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindNextPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, pfdwchange: *mut u32, pprinternotifyoptions: *const ::core::ffi::c_void, ppprinternotifyinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
    pub fn SpoolerFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO);
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerRefreshPrinterChangeNotification(hprinter: super::super::Foundation::HANDLE, dwcolor: u32, poptions: *const PRINTER_NOTIFY_OPTIONS, ppinfo: *mut *mut PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterA(hprinter: super::super::Foundation::HANDLE, level: u32, pdocinfo: *const DOC_INFO_1A) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterW(hprinter: super::super::Foundation::HANDLE, level: u32, pdocinfo: *const DOC_INFO_1W) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartPagePrinter(hprinter: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnRegisterForPrintAsyncNotifications(param0: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePrintDeviceObject(hprinter: super::super::Foundation::HANDLE, hdeviceobject: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageA(pszserver: ::windows_sys::core::PCSTR, pszinfpath: ::windows_sys::core::PCSTR, pszenvironment: ::windows_sys::core::PCSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: ::windows_sys::core::PSTR, pcchdestinfpath: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageW(pszserver: ::windows_sys::core::PCWSTR, pszinfpath: ::windows_sys::core::PCWSTR, pszenvironment: ::windows_sys::core::PCWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: ::windows_sys::core::PWSTR, pcchdestinfpath: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForPrinterChange(hprinter: super::super::Foundation::HANDLE, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrinter(hprinter: super::super::Foundation::HANDLE, pbuf: *const ::core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XcvDataW(hxcv: super::super::Foundation::HANDLE, pszdataname: ::windows_sys::core::PCWSTR, pinputdata: *const u8, cbinputdata: u32, poutputdata: *mut u8, cboutputdata: u32, pcboutputneeded: *mut u32, pdwstatus: *mut u32) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ADDJOB_INFO_1A {
    pub Path: ::windows_sys::core::PSTR,
    pub JobId: u32,
}
impl ::core::marker::Copy for ADDJOB_INFO_1A {}
impl ::core::clone::Clone for ADDJOB_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ADDJOB_INFO_1W {
    pub Path: ::windows_sys::core::PWSTR,
    pub JobId: u32,
}
impl ::core::marker::Copy for ADDJOB_INFO_1W {}
impl ::core::clone::Clone for ADDJOB_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_FROM_DIRECTORY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_NEW_FILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_STRICT_DOWNGRADE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_STRICT_UPGRADE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ATTRIBUTE_INFO_1 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
}
impl ::core::marker::Copy for ATTRIBUTE_INFO_1 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for ATTRIBUTE_INFO_2 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for ATTRIBUTE_INFO_3 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for ATTRIBUTE_INFO_4 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACCESS_ADMINISTRATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACCESS_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_ENUM_SCHEMA: &str = "EnumSchema";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET: &str = "Get";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET_ALL: &str = "GetAll";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET_WITH_ARGUMENT: &str = "GetWithArgument";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_SET: &str = "Set";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_DATA {
    pub dwBidiType: u32,
    pub u: BIDI_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union BIDI_DATA_0 {
    pub bData: super::super::Foundation::BOOL,
    pub iData: i32,
    pub sData: ::windows_sys::core::PWSTR,
    pub fData: f32,
    pub biData: BINARY_CONTAINER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_REQUEST_CONTAINER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_REQUEST_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: u32,
    pub pSchema: ::windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_REQUEST_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_REQUEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_RESPONSE_CONTAINER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_RESPONSE_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: u32,
    pub dwReqNumber: u32,
    pub pSchema: ::windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_RESPONSE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_RESPONSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type BIDI_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_NULL: BIDI_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_INT: BIDI_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_FLOAT: BIDI_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_BOOL: BIDI_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_STRING: BIDI_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_TEXT: BIDI_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ENUM: BIDI_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_BLOB: BIDI_TYPE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BINARY_CONTAINER {
    pub cbBuf: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BINARY_CONTAINER {}
impl ::core::clone::Clone for BINARY_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_EDGE_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_EDGE_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_PRINT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BORDER_PRINT: u32 = 0u32;
pub const BidiRequest: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3105237539, data2: 17913, data3: 18380, data4: [128, 245, 254, 15, 233, 185, 225, 162] };
pub const BidiRequestContainer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4233857572, data2: 56069, data3: 18945, data4: [131, 136, 34, 237, 246, 194, 187, 186] };
pub const BidiSpl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 711017024, data2: 42181, data3: 19507, data4: [189, 135, 27, 199, 9, 51, 22, 57] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobData {
    pub eEventType: EBranchOfficeJobEventType,
    pub JobId: u32,
    pub JobInfo: BranchOfficeJobData_0,
}
impl ::core::marker::Copy for BranchOfficeJobData {}
impl ::core::clone::Clone for BranchOfficeJobData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union BranchOfficeJobData_0 {
    pub LogJobPrinted: BranchOfficeJobDataPrinted,
    pub LogJobRendered: BranchOfficeJobDataRendered,
    pub LogJobError: BranchOfficeJobDataError,
    pub LogPipelineFailed: BranchOfficeJobDataPipelineFailed,
    pub LogOfflineFileFull: BranchOfficeLogOfflineFileFull,
}
impl ::core::marker::Copy for BranchOfficeJobData_0 {}
impl ::core::clone::Clone for BranchOfficeJobData_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataContainer {
    pub cJobDataEntries: u32,
    pub JobData: [BranchOfficeJobData; 1],
}
impl ::core::marker::Copy for BranchOfficeJobDataContainer {}
impl ::core::clone::Clone for BranchOfficeJobDataContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataError {
    pub LastError: u32,
    pub pDocumentName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pDataType: ::windows_sys::core::PWSTR,
    pub TotalSize: i64,
    pub PrintedSize: i64,
    pub TotalPages: u32,
    pub PrintedPages: u32,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pJobError: ::windows_sys::core::PWSTR,
    pub pErrorDescription: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeJobDataError {}
impl ::core::clone::Clone for BranchOfficeJobDataError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataPipelineFailed {
    pub pDocumentName: ::windows_sys::core::PWSTR,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pExtraErrorInfo: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeJobDataPipelineFailed {}
impl ::core::clone::Clone for BranchOfficeJobDataPipelineFailed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataPrinted {
    pub Status: u32,
    pub pDocumentName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pPortName: ::windows_sys::core::PWSTR,
    pub Size: i64,
    pub TotalPages: u32,
}
impl ::core::marker::Copy for BranchOfficeJobDataPrinted {}
impl ::core::clone::Clone for BranchOfficeJobDataPrinted {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataRendered {
    pub Size: i64,
    pub ICMMethod: u32,
    pub Color: i16,
    pub PrintQuality: i16,
    pub YResolution: i16,
    pub Copies: i16,
    pub TTOption: i16,
}
impl ::core::marker::Copy for BranchOfficeJobDataRendered {}
impl ::core::clone::Clone for BranchOfficeJobDataRendered {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeLogOfflineFileFull {
    pub pMachineName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeLogOfflineFileFull {}
impl ::core::clone::Clone for BranchOfficeLogOfflineFileFull {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_BIG5: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP437: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP850: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP863: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_GB2312: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_ISC: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_JIS: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_JIS_ANK: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_NOPRECNV: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_NS86: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_SJIS: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_TCA: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_WANSUNG: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_CONVERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_CONVERT351: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_DRIVER_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_FALSE_PDATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_FALSE_TRUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NONE_PDATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NO_PDATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NO_YES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_OFF_ON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_OFF_PDATA: u32 = 5u32;
pub const CLSID_OEMPTPROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2440181906, data2: 17874, data3: 18658, data4: [158, 201, 86, 35, 121, 218, 249, 146] };
pub const CLSID_OEMRENDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1835712294, data2: 40760, data3: 4561, data4: [136, 42, 0, 192, 79, 185, 97, 236] };
pub const CLSID_OEMUI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2882437335, data2: 40774, data3: 4561, data4: [136, 42, 0, 192, 79, 185, 97, 236] };
pub const CLSID_OEMUIMXDC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309950720, data2: 23363, data3: 17032, data4: [147, 42, 94, 77, 214, 216, 43, 237] };
pub const CLSID_PTPROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1185682715, data2: 33936, data3: 17713, data4: [150, 204, 85, 191, 43, 241, 158, 17] };
pub const CLSID_XPSRASTERIZER_FACTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1346271679, data2: 7433, data3: 18276, data4: [157, 114, 30, 176, 198, 89, 103, 198] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COLOR_OPTIMIZATION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for COMPROPSHEETUI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for COMPROPSHEETUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CONFIG_INFO_DATA_1 {
    pub Reserved: [u8; 128],
    pub dwVersion: u32,
}
impl ::core::marker::Copy for CONFIG_INFO_DATA_1 {}
impl ::core::clone::Clone for CONFIG_INFO_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_ADD_PRINTER_CONNECTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER_CONNECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_FILES_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_SET_PRINTER_DATAEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_FLAG_CLIENT_SPOOLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_FLAG_SERVER_SPOOLER: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: ::windows_sys::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORE_PRINTER_DRIVERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORE_PRINTER_DRIVERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: ::windows_sys::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORE_PRINTER_DRIVERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORE_PRINTER_DRIVERW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_DATABLOCK: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_RESULT: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CPSUICBPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CPSUICBPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union CPSUICBPARAM_0 {
    pub OldSel: i32,
    pub pOldSel: *mut i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CPSUICBPARAM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CPSUICBPARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ABOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_APPLYNOW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_DLGPROC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_EXTPUSH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_SETACTIVE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for CPSUIDATABLOCK {}
impl ::core::clone::Clone for CPSUIDATABLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_ICONID_AS_HICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_REBOOTSYSTEM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_RESTARTWINDOWS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_HEIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_HEIGHTOFFSET: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_ORIENTATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_WIDTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_WIDTHOFFSET: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CUSTOMSIZEPARAM {
    pub dwOrder: i32,
    pub lMinVal: i32,
    pub lMaxVal: i32,
}
impl ::core::marker::Copy for CUSTOMSIZEPARAM {}
impl ::core::clone::Clone for CUSTOMSIZEPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATATYPES_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DATATYPES_INFO_1A {}
impl ::core::clone::Clone for DATATYPES_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATATYPES_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DATATYPES_INFO_1W {}
impl ::core::clone::Clone for DATATYPES_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATA_HEADER {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wDataID: u16,
    pub dwDataSize: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DATA_HEADER {}
impl ::core::clone::Clone for DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DEF_PRIORITY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DELETE_PORT_DATA_1 {
    pub psztPortName: [u16; 64],
    pub Reserved: [u8; 98],
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DELETE_PORT_DATA_1 {}
impl ::core::clone::Clone for DELETE_PORT_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEPROPERTYHEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICEPROPERTYHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEPROPERTYHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DEVQUERYPRINT_INFO {
    pub cbSize: u16,
    pub Level: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pszErrorStr: ::windows_sys::core::PWSTR,
    pub cchErrorStr: u32,
    pub cchNeeded: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DEVQUERYPRINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DEVQUERYPRINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_BKSP_OK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NOITALIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NOUNDER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_DOUBLE_UNDERLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_STRIKETHRU: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_CAPSL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_HPINTELLIFONT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_OEM1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_OEM2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_PST1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_TRUETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_XM_CR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT: u32 = 11800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION: u32 = 12100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_COUNT: u32 = 12101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_GETAT: u32 = 12102u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_DRIVERPROPERTIES: u32 = 11803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTERQUEUE: u32 = 11801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTSCHEMATICKET: u32 = 11802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_USERPROPERTIES: u32 = 11804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT: u32 = 12200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS: u32 = 12000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_BIDINOTIFICATION: u32 = 12001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_DETAILEDREASONID: u32 = 12005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REASONID: u32 = 12002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REQUEST: u32 = 12003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_SOURCEAPPLICATION: u32 = 12004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWMODAL: u32 = 12006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWPARENT: u32 = 12007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONDRIVEREVENT: u32 = 12201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONPRINTERQUEUESENUMERATED: u32 = 12202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST: u32 = 11900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_CANCEL: u32 = 11901u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_COMPLETE: u32 = 11902u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG: u32 = 11400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBOOL: u32 = 11401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBYTES: u32 = 11407u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETINT32: u32 = 11403u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETREADSTREAM: u32 = 11409u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETSTRING: u32 = 11405u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETWRITESTREAM: u32 = 11410u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBOOL: u32 = 11402u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBYTES: u32 = 11408u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETINT32: u32 = 11404u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETSTRING: u32 = 11406u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE: u32 = 11600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEEVENT: u32 = 11700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEEVENT_ONBIDIRESPONSERECEIVED: u32 = 11701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW: u32 = 12700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT: u32 = 12800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT_ONCHANGED: u32 = 12801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_SETVIEWRANGE: u32 = 12701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_GETPRINTERQUEUEVIEW: u32 = 11606u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_GETPROPERTIES: u32 = 11604u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_HANDLE: u32 = 11601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_NAME: u32 = 11602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDIQUERY: u32 = 11603u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDISETREQUESTASYNC: u32 = 11605u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG: u32 = 11500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBOOL: u32 = 11501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBYTES: u32 = 11507u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETINT32: u32 = 11503u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETREADSTREAM: u32 = 11509u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTREAMASXML: u32 = 11411u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTRING: u32 = 11505u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETWRITESTREAM: u32 = 11510u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBOOL: u32 = 11502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBYTES: u32 = 11508u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETINT32: u32 = 11504u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETSTRING: u32 = 11506u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM: u32 = 11200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_READ: u32 = 11201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_WRITE: u32 = 11202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM: u32 = 11300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_COMMIT: u32 = 11301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SEEK: u32 = 11302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SETSIZE: u32 = 11303u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT: u32 = 12300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_DRIVERPROPERTIES: u32 = 12301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_QUEUEPROPERTIES: u32 = 12302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_USERPROPERTIES: u32 = 12303u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION: u32 = 12600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION_COUNT: u32 = 12601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION_GETAT: u32 = 12602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION: u32 = 10900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT: u32 = 11100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT_COMPLETED: u32 = 11101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_CANCEL: u32 = 10902u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_START: u32 = 10901u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES: u32 = 10800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE: u32 = 10802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE_KEYNAME: u32 = 10801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETOPTIONS: u32 = 10807u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETPARAMETERDEFINITION: u32 = 10808u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETSELECTEDOPTION: u32 = 10806u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMAXVALUE: u32 = 10805u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMINVALUE: u32 = 10804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_PAGEIMAGEABLESIZE: u32 = 10803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT: u32 = 10100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT_DISPLAYNAME: u32 = 10101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAME: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAMESPACEURI: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_XMLNODE: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE: u32 = 10600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_DISPLAYUI: u32 = 10604u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_GETOPTION: u32 = 10603u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTEDOPTION: u32 = 10601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTIONTYPE: u32 = 10602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION: u32 = 10400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION_PAGESPERSHEET: u32 = 10401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION: u32 = 10200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION: u32 = 10500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_COUNT: u32 = 10501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_GETAT: u32 = 10502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_CONSTRAINED: u32 = 10202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_GETPROPERTYVALUE: u32 = 10203u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_SELECTED: u32 = 10201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE: u32 = 10700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_HEIGHT: u32 = 10706u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_WIDTH: u32 = 10705u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_HEIGHT: u32 = 10702u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_WIDTH: u32 = 10701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_HEIGHT: u32 = 10704u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_WIDTH: u32 = 10703u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION: u32 = 10300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_HEIGHT: u32 = 10302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_WIDTH: u32 = 10301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION: u32 = 12500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_DATATYPE: u32 = 12503u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMAX: u32 = 12505u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMIN: u32 = 12504u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_UNITTYPE: u32 = 12502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_USERINPUTREQUIRED: u32 = 12501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER: u32 = 12400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER_VALUE: u32 = 12401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_COMMITASYNC: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETCAPABILITIES: u32 = 11006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE_KEYNAME: u32 = 11001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETPARAMETERINITIALIZER: u32 = 11008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_JOBCOPIESALLDOCUMENTS: u32 = 11007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_NOTIFYXMLCHANGED: u32 = 11005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_VALIDATEASYNC: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_MEMORYMAP_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_READ_SPOOL_JOB: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pTabName: *mut i8,
    pub IconID: usize,
    pub Anonymous: DLGPAGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for DLGPAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for DLGPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union DLGPAGE_0 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for DLGPAGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for DLGPAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_BOOKLET_EDGE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_COLOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_COPIES_COLLATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DEFSOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DITHERTYPE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DUPLEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_FORMNAME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ICMINTENT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ICMMETHOD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_LAST: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_MANUAL_DUPLEX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_MEDIATYPE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NUP_DIRECTION: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ORIENTATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OUTPUTBIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_PAGEORDER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_PRINTQUALITY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_QUALITY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_SCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_STAPLE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_TTOPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_USER: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_ADVANCED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_INVALIDATE_DRIVER_CACHE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_NOPERMISSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_PROMPT_NON_MODAL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_USER_DEFAULT: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DOCEVENT_CREATEDCPRE {
    pub pszDriver: ::windows_sys::core::PWSTR,
    pub pszDevice: ::windows_sys::core::PWSTR,
    pub pdm: *mut super::Gdi::DEVMODEW,
    pub bIC: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DOCEVENT_CREATEDCPRE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DOCEVENT_CREATEDCPRE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOCEVENT_ESCAPE {
    pub iEscape: i32,
    pub cjInput: i32,
    pub pvInData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DOCEVENT_ESCAPE {}
impl ::core::clone::Clone for DOCEVENT_ESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOCEVENT_FILTER {
    pub cbSize: u32,
    pub cElementsAllocated: u32,
    pub cElementsNeeded: u32,
    pub cElementsReturned: u32,
    pub aDocEventCall: [u32; 1],
}
impl ::core::marker::Copy for DOCEVENT_FILTER {}
impl ::core::clone::Clone for DOCEVENT_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ABORTDOC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_CREATEDCPOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_CREATEDCPRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_DELETEDC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOCPOST: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOCPRE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDPAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ESCAPE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_FAILURE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_LAST: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_QUERYFILTER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_RESETDCPOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_RESETDCPRE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_SPOOLED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOCPOST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOCPRE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTPAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_UNSUPPORTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_CANCELJOB: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DOCUMENTPROPERTYHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DOCUMENTPROPERTYHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_1A {
    pub pDocName: ::windows_sys::core::PSTR,
    pub pOutputFile: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DOC_INFO_1A {}
impl ::core::clone::Clone for DOC_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_1W {
    pub pDocName: ::windows_sys::core::PWSTR,
    pub pOutputFile: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DOC_INFO_1W {}
impl ::core::clone::Clone for DOC_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_2A {
    pub pDocName: ::windows_sys::core::PSTR,
    pub pOutputFile: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl ::core::marker::Copy for DOC_INFO_2A {}
impl ::core::clone::Clone for DOC_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_2W {
    pub pDocName: ::windows_sys::core::PWSTR,
    pub pOutputFile: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl ::core::marker::Copy for DOC_INFO_2W {}
impl ::core::clone::Clone for DOC_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_3A {
    pub pDocName: ::windows_sys::core::PSTR,
    pub pOutputFile: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DOC_INFO_3A {}
impl ::core::clone::Clone for DOC_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_3W {
    pub pDocName: ::windows_sys::core::PWSTR,
    pub pOutputFile: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DOC_INFO_3W {}
impl ::core::clone::Clone for DOC_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_UNUSED_FILES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPF_ICONID_AS_HICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPF_USE_HDLGTEMPLATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPS_NOPERMISSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_DOCPROPPAGE1: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_DOCPROPPAGE2: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_RESERVED_START: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_TREEVIEWPAGE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_EVENT_INITIALIZE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_1A {}
impl ::core::clone::Clone for DRIVER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_1W {}
impl ::core::clone::Clone for DRIVER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_2A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_2A {}
impl ::core::clone::Clone for DRIVER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_2W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_2W {}
impl ::core::clone::Clone for DRIVER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_3A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
    pub pHelpFile: ::windows_sys::core::PSTR,
    pub pDependentFiles: ::windows_sys::core::PSTR,
    pub pMonitorName: ::windows_sys::core::PSTR,
    pub pDefaultDataType: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_3A {}
impl ::core::clone::Clone for DRIVER_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_3W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
    pub pHelpFile: ::windows_sys::core::PWSTR,
    pub pDependentFiles: ::windows_sys::core::PWSTR,
    pub pMonitorName: ::windows_sys::core::PWSTR,
    pub pDefaultDataType: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_3W {}
impl ::core::clone::Clone for DRIVER_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_4A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
    pub pHelpFile: ::windows_sys::core::PSTR,
    pub pDependentFiles: ::windows_sys::core::PSTR,
    pub pMonitorName: ::windows_sys::core::PSTR,
    pub pDefaultDataType: ::windows_sys::core::PSTR,
    pub pszzPreviousNames: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_4A {}
impl ::core::clone::Clone for DRIVER_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_4W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
    pub pHelpFile: ::windows_sys::core::PWSTR,
    pub pDependentFiles: ::windows_sys::core::PWSTR,
    pub pMonitorName: ::windows_sys::core::PWSTR,
    pub pDefaultDataType: ::windows_sys::core::PWSTR,
    pub pszzPreviousNames: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_4W {}
impl ::core::clone::Clone for DRIVER_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_5A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl ::core::marker::Copy for DRIVER_INFO_5A {}
impl ::core::clone::Clone for DRIVER_INFO_5A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_5W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl ::core::marker::Copy for DRIVER_INFO_5W {}
impl ::core::clone::Clone for DRIVER_INFO_5W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_6A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
    pub pHelpFile: ::windows_sys::core::PSTR,
    pub pDependentFiles: ::windows_sys::core::PSTR,
    pub pMonitorName: ::windows_sys::core::PSTR,
    pub pDefaultDataType: ::windows_sys::core::PSTR,
    pub pszzPreviousNames: ::windows_sys::core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows_sys::core::PSTR,
    pub pszOEMUrl: ::windows_sys::core::PSTR,
    pub pszHardwareID: ::windows_sys::core::PSTR,
    pub pszProvider: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_6A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_6A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_6W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
    pub pHelpFile: ::windows_sys::core::PWSTR,
    pub pDependentFiles: ::windows_sys::core::PWSTR,
    pub pMonitorName: ::windows_sys::core::PWSTR,
    pub pDefaultDataType: ::windows_sys::core::PWSTR,
    pub pszzPreviousNames: ::windows_sys::core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows_sys::core::PWSTR,
    pub pszOEMUrl: ::windows_sys::core::PWSTR,
    pub pszHardwareID: ::windows_sys::core::PWSTR,
    pub pszProvider: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_6W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_6W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_8A {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDriverPath: ::windows_sys::core::PSTR,
    pub pDataFile: ::windows_sys::core::PSTR,
    pub pConfigFile: ::windows_sys::core::PSTR,
    pub pHelpFile: ::windows_sys::core::PSTR,
    pub pDependentFiles: ::windows_sys::core::PSTR,
    pub pMonitorName: ::windows_sys::core::PSTR,
    pub pDefaultDataType: ::windows_sys::core::PSTR,
    pub pszzPreviousNames: ::windows_sys::core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows_sys::core::PSTR,
    pub pszOEMUrl: ::windows_sys::core::PSTR,
    pub pszHardwareID: ::windows_sys::core::PSTR,
    pub pszProvider: ::windows_sys::core::PSTR,
    pub pszPrintProcessor: ::windows_sys::core::PSTR,
    pub pszVendorSetup: ::windows_sys::core::PSTR,
    pub pszzColorProfiles: ::windows_sys::core::PSTR,
    pub pszInfPath: ::windows_sys::core::PSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: ::windows_sys::core::PSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_8A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_8W {
    pub cVersion: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDriverPath: ::windows_sys::core::PWSTR,
    pub pDataFile: ::windows_sys::core::PWSTR,
    pub pConfigFile: ::windows_sys::core::PWSTR,
    pub pHelpFile: ::windows_sys::core::PWSTR,
    pub pDependentFiles: ::windows_sys::core::PWSTR,
    pub pMonitorName: ::windows_sys::core::PWSTR,
    pub pDefaultDataType: ::windows_sys::core::PWSTR,
    pub pszzPreviousNames: ::windows_sys::core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows_sys::core::PWSTR,
    pub pszOEMUrl: ::windows_sys::core::PWSTR,
    pub pszHardwareID: ::windows_sys::core::PWSTR,
    pub pszProvider: ::windows_sys::core::PWSTR,
    pub pszPrintProcessor: ::windows_sys::core::PWSTR,
    pub pszVendorSetup: ::windows_sys::core::PWSTR,
    pub pszzColorProfiles: ::windows_sys::core::PWSTR,
    pub pszInfPath: ::windows_sys::core::PWSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: ::windows_sys::core::PWSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_8W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_8W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_KERNELMODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_UPGRADE_INFO_1 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
}
impl ::core::marker::Copy for DRIVER_UPGRADE_INFO_1 {}
impl ::core::clone::Clone for DRIVER_UPGRADE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for DRIVER_UPGRADE_INFO_2 {}
impl ::core::clone::Clone for DRIVER_UPGRADE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_USERMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_PENDING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_PUBLISH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_REPUBLISH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_UNPUBLISH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EATTRIBUTE_DATATYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_UNKNOWN: EATTRIBUTE_DATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_BOOL: EATTRIBUTE_DATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_INT: EATTRIBUTE_DATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_LONG: EATTRIBUTE_DATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_DWORD: EATTRIBUTE_DATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_ASCII: EATTRIBUTE_DATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_UNICODE: EATTRIBUTE_DATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_BINARY: EATTRIBUTE_DATATYPE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_SIZE: EATTRIBUTE_DATATYPE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_RECT: EATTRIBUTE_DATATYPE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_CUSTOMSIZEPARAMS: EATTRIBUTE_DATATYPE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EBranchOfficeJobEventType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kInvalidJobState: EBranchOfficeJobEventType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobPrinted: EBranchOfficeJobEventType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobRendered: EBranchOfficeJobEventType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobError: EBranchOfficeJobEventType = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobPipelineError: EBranchOfficeJobEventType = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogOfflineFileFull: EBranchOfficeJobEventType = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_ONLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type EMFPLAYPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::HANDLE) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EMF_PP_COLOR_OPTIMIZATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_ICONID_AS_HICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_INCL_SETUP_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_NO_DOT_DOT_DOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_USE_HDLGTEMPLATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EPrintPropertyType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeString: EPrintPropertyType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeInt32: EPrintPropertyType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeInt64: EPrintPropertyType = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeByte: EPrintPropertyType = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeTime: EPrintPropertyType = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeDevMode: EPrintPropertyType = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeSD: EPrintPropertyType = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeNotificationReply: EPrintPropertyType = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeNotificationOptions: EPrintPropertyType = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeBuffer: EPrintPropertyType = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EPrintXPSJobOperation = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kJobProduction: EPrintXPSJobOperation = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kJobConsumption: EPrintXPSJobOperation = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EPrintXPSJobProgress = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingDocumentSequence: EPrintXPSJobProgress = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kDocumentSequenceAdded: EPrintXPSJobProgress = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingFixedDocument: EPrintXPSJobProgress = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFixedDocumentAdded: EPrintXPSJobProgress = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingFixedPage: EPrintXPSJobProgress = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFixedPageAdded: EPrintXPSJobProgress = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kResourceAdded: EPrintXPSJobProgress = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFontAdded: EPrintXPSJobProgress = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kImageAdded: EPrintXPSJobProgress = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kXpsDocumentCommitted: EPrintXPSJobProgress = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: u32 = 13016u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: u32 = 13015u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: u32 = 13005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_READ_ONLY: u32 = 13002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: u32 = 13010u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SERVER_OFFLINE: u32 = 13003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: u32 = 13006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: u32 = 13008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: u32 = 13007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: u32 = 13009u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_STATUS_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_STATUS_WARNING: u32 = 13001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: u32 = 13017u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: u32 = 13018u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_GETLASTERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_HINST: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for EXTCHKBOX {}
impl ::core::clone::Clone for EXTCHKBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct EXTPUSH {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub Anonymous1: EXTPUSH_0,
    pub IconID: usize,
    pub Anonymous2: EXTPUSH_1,
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union EXTPUSH_0 {
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pfnCallBack: super::super::Foundation::FARPROC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union EXTPUSH_1 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for EXTTEXTMETRIC {}
impl ::core::clone::Clone for EXTTEXTMETRIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EXpsCompressionOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_NotCompressed: EXpsCompressionOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Normal: EXpsCompressionOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Small: EXpsCompressionOptions = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Fast: EXpsCompressionOptions = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EXpsFontOptions = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Font_Normal: EXpsFontOptions = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Font_Obfusticate: EXpsFontOptions = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EXpsFontRestriction = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_Installable: EXpsFontRestriction = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_NoEmbedding: EXpsFontRestriction = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_PreviewPrint: EXpsFontRestriction = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_Editable: EXpsFontRestriction = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type EXpsJobConsumption = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_DocumentSequenceAdded: EXpsJobConsumption = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_FixedDocumentAdded: EXpsJobConsumption = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_FixedPageAdded: EXpsJobConsumption = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const E_VERSION_NOT_SUPPORTED: u32 = 2147745793u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FG_CANCHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FILL_WITH_DEFAULTS: u32 = 1u32;
pub const FMTID_PrinterPropertyBag: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1979297226, data2: 2429, data3: 17859, data4: [166, 228, 186, 178, 158, 39, 111, 62] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_CURRENTFONTID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTBOLD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTHEIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTITALIC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTMAXWIDTH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTSTRIKETHRU: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTUNDERLINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTWIDTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_GRAYPERCENTAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_MAX: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_NEXTFONTID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_NEXTGLYPH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_PRINTDIRINCCDEGREES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_TEXTXRES: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_TEXTYRES: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_DIR_SORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_DEVICEFONT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_GLYPHSET_GTT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_GLYPHSET_RLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_IFI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_PERMANENT_SF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_SOFTFONT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_UFM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_BUILTIN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_1A {
    pub Flags: u32,
    pub pName: ::windows_sys::core::PSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_1W {
    pub Flags: u32,
    pub pName: ::windows_sys::core::PWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_2A {
    pub Flags: u32,
    pub pName: ::windows_sys::core::PCSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: ::windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: ::windows_sys::core::PCSTR,
    pub dwResourceId: u32,
    pub pDisplayName: ::windows_sys::core::PCSTR,
    pub wLangId: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_2W {
    pub Flags: u32,
    pub pName: ::windows_sys::core::PCWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: ::windows_sys::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: ::windows_sys::core::PCWSTR,
    pub dwResourceId: u32,
    pub pDisplayName: ::windows_sys::core::PCWSTR,
    pub wLangId: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_USER: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct GLYPHRUN {
    pub wcLow: u16,
    pub wGlyphCount: u16,
}
impl ::core::marker::Copy for GLYPHRUN {}
impl ::core::clone::Clone for GLYPHRUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const GPD_OEMCUSTOMDATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_IPPUSB_PRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4076077953, data2: 62573, data3: 20049, data4: [188, 231, 98, 222, 108, 242, 208, 152] };
pub const GUID_DEVINTERFACE_USBPRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 685215661, data2: 23058, data3: 4561, data4: [174, 91, 0, 0, 248, 3, 168, 194] };
pub type IAsyncGetSendNotificationCookie = *mut ::core::ffi::c_void;
pub type IAsyncGetSrvReferralCookie = *mut ::core::ffi::c_void;
pub type IBidiAsyncNotifyChannel = *mut ::core::ffi::c_void;
pub type IBidiRequest = *mut ::core::ffi::c_void;
pub type IBidiRequestContainer = *mut ::core::ffi::c_void;
pub type IBidiSpl = *mut ::core::ffi::c_void;
pub type IBidiSpl2 = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ADVANCE: u32 = 64058u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_AUTOSEL: u32 = 64025u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COLLATE: u32 = 64030u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COLOR: u32 = 64040u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COPY: u32 = 64046u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE: u32 = 64060u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE2: u32 = 64061u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DOCUMENT: u32 = 64059u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_EMPTY: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ENVELOPE: u32 = 64010u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ERROR: u32 = 64050u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FALSE: u32 = 64005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FAX: u32 = 64095u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCART: u32 = 64013u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTSUB: u32 = 64081u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GRAPHIC: u32 = 64057u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HT_HOST: u32 = 64016u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MEM: u32 = 64011u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MONO: u32 = 64039u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NO: u32 = 64003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OFF: u32 = 64007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ON: u32 = 64008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OPTION: u32 = 64066u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OPTION2: u32 = 64067u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OUTBIN: u32 = 64055u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OUTPUT: u32 = 64056u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PORTRAIT: u32 = 64022u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER: u32 = 64062u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER2: u32 = 64063u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER3: u32 = 64064u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER4: u32 = 64065u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_QUESTION: u32 = 64075u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_HIGH: u32 = 64037u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_LOW: u32 = 64035u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROT_LAND: u32 = 64024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROT_PORT: u32 = 64110u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SCALING: u32 = 64045u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SEL_NONE: u32 = 64001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STD_FORM: u32 = 64054u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STOP: u32 = 64068u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TELEPHONE: u32 = 64083u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TRUE: u32 = 64006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WARNING: u32 = 64002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WATERMARK: u32 = 64049u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_YES: u32 = 64004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ABOUT: u32 = 64848u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ADVANCED: u32 = 64722u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ALL: u32 = 64841u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOND: u32 = 64786u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET: u32 = 64873u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGE: u32 = 64702u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGED: u32 = 64846u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGES: u32 = 64845u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COARSE: u32 = 64787u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLLATE: u32 = 64756u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLLATED: u32 = 64757u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLON_SEP: u32 = 64707u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLOR: u32 = 64764u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COPIES: u32 = 64831u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COPY: u32 = 64830u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULT: u32 = 64732u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICE: u32 = 64842u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DITHERING: u32 = 64752u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOCUMENT: u32 = 64715u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DRAFT: u32 = 64759u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DUPLEX: u32 = 64745u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ERROR: u32 = 64733u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_EXIST: u32 = 64736u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FALSE: u32 = 64726u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FAST: u32 = 64838u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FAX: u32 = 64835u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FINE: u32 = 64788u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMNAME: u32 = 64747u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GLOSSY: u32 = 64783u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GRAPHIC: u32 = 64720u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HALFTONE: u32 = 64791u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HIGH: u32 = 64762u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM: u32 = 64748u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICMINTENT: u32 = 64750u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_NO: u32 = 64777u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_YES: u32 = 64778u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LINEART: u32 = 64789u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LOW: u32 = 64760u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MAILBOX: u32 = 64829u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MAKE: u32 = 64833u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUALFEED: u32 = 64813u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MEDIA: u32 = 64751u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MEDIUM: u32 = 64761u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MONOCHROME: u32 = 64766u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MORE: u32 = 64701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NO: u32 = 64728u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NONE: u32 = 64734u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NOT: u32 = 64735u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NO_NAME: u32 = 64850u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP: u32 = 64864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OF: u32 = 64704u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OFF: u32 = 64730u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ON: u32 = 64731u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ONLYONE: u32 = 64800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OPTION: u32 = 64703u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OPTIONS: u32 = 64721u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ORIENTATION: u32 = 64738u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAGEORDER: u32 = 64855u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PERCENT: u32 = 64711u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PLOT: u32 = 64836u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PORTRAIT: u32 = 64753u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER: u32 = 64874u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRESENTATION: u32 = 64763u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINT: u32 = 64834u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTER: u32 = 64717u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PROPERTIES: u32 = 64713u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_REGULAR: u32 = 64785u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RESET: u32 = 64840u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RESOLUTION: u32 = 64743u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_REVERT: u32 = 64844u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROTATED: u32 = 64839u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROT_LAND: u32 = 64755u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROT_PORT: u32 = 64886u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SCALING: u32 = 64739u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETTING: u32 = 64851u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETTINGS: u32 = 64843u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETUP: u32 = 64700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIDE1: u32 = 64871u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIDE2: u32 = 64872u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIMPLEX: u32 = 64767u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT1: u32 = 64819u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT2: u32 = 64820u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT3: u32 = 64821u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT4: u32 = 64822u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOW: u32 = 64837u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SOURCE: u32 = 64741u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STACKER: u32 = 64828u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STANDARD: u32 = 64782u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLE: u32 = 64887u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER: u32 = 64825u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STRID_LAST: u32 = 64892u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TO: u32 = 64706u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TOTAL: u32 = 64832u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRUE: u32 = 64727u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TTOPTION: u32 = 64746u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_VERSION: u32 = 64849u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_VERTICAL: u32 = 64769u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_WARNING: u32 = 64847u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_WATERMARK: u32 = 64797u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_YES: u32 = 64729u32;
pub type IFixedDocument = *mut ::core::ffi::c_void;
pub type IFixedDocumentSequence = *mut ::core::ffi::c_void;
pub type IFixedPage = *mut ::core::ffi::c_void;
pub type IImgCreateErrorInfo = *mut ::core::ffi::c_void;
pub type IImgErrorInfo = *mut ::core::ffi::c_void;
pub type IInterFilterCommunicator = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
impl ::core::marker::Copy for INSERTPSUIPAGE_INFO {}
impl ::core::clone::Clone for INSERTPSUIPAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct INVOC {
    pub dwCount: u32,
    pub loOffset: u32,
}
impl ::core::marker::Copy for INVOC {}
impl ::core::clone::Clone for INVOC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_ADD_CHILD_DEVICE: u32 = 2228316u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_ADD_MSIPP_COMPAT_ID: u32 = 2228308u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_CYCLE_PORT: u32 = 2228320u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_1284_ID: u32 = 2228276u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_INTERFACE_TYPE: u32 = 2228300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_LPT_STATUS: u32 = 2228272u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_PROTOCOL: u32 = 2228292u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_DEVICE_ID: u32 = 2228312u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_PORT_NUMBER: u32 = 2228304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_PROTOCOL: u32 = 2228296u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SOFT_RESET: u32 = 2228288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_VENDOR_GET_COMMAND: u32 = 2228284u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_VENDOR_SET_COMMAND: u32 = 2228280u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IPDFP_COPY_ALL_FILES: u32 = 1u32;
pub type IPartBase = *mut ::core::ffi::c_void;
pub type IPartColorProfile = *mut ::core::ffi::c_void;
pub type IPartDiscardControl = *mut ::core::ffi::c_void;
pub type IPartFont = *mut ::core::ffi::c_void;
pub type IPartFont2 = *mut ::core::ffi::c_void;
pub type IPartImage = *mut ::core::ffi::c_void;
pub type IPartPrintTicket = *mut ::core::ffi::c_void;
pub type IPartResourceDictionary = *mut ::core::ffi::c_void;
pub type IPartThumbnail = *mut ::core::ffi::c_void;
pub type IPrintAsyncCookie = *mut ::core::ffi::c_void;
pub type IPrintAsyncNewChannelCookie = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotify = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotifyCallback = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotifyChannel = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotifyDataObject = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotifyRegistration = *mut ::core::ffi::c_void;
pub type IPrintAsyncNotifyServerReferral = *mut ::core::ffi::c_void;
pub type IPrintBidiAsyncNotifyRegistration = *mut ::core::ffi::c_void;
pub type IPrintClassObjectFactory = *mut ::core::ffi::c_void;
pub type IPrintCoreHelper = *mut ::core::ffi::c_void;
pub type IPrintCoreHelperPS = *mut ::core::ffi::c_void;
pub type IPrintCoreHelperUni = *mut ::core::ffi::c_void;
pub type IPrintCoreHelperUni2 = *mut ::core::ffi::c_void;
pub type IPrintCoreUI2 = *mut ::core::ffi::c_void;
pub type IPrintJob = *mut ::core::ffi::c_void;
pub type IPrintJobCollection = *mut ::core::ffi::c_void;
pub type IPrintOemCommon = *mut ::core::ffi::c_void;
pub type IPrintOemDriverUI = *mut ::core::ffi::c_void;
pub type IPrintOemUI = *mut ::core::ffi::c_void;
pub type IPrintOemUI2 = *mut ::core::ffi::c_void;
pub type IPrintOemUIMXDC = *mut ::core::ffi::c_void;
pub type IPrintPipelineFilter = *mut ::core::ffi::c_void;
pub type IPrintPipelineManagerControl = *mut ::core::ffi::c_void;
pub type IPrintPipelineProgressReport = *mut ::core::ffi::c_void;
pub type IPrintPipelinePropertyBag = *mut ::core::ffi::c_void;
pub type IPrintPreviewDxgiPackageTarget = *mut ::core::ffi::c_void;
pub type IPrintReadStream = *mut ::core::ffi::c_void;
pub type IPrintReadStreamFactory = *mut ::core::ffi::c_void;
pub type IPrintSchemaAsyncOperation = *mut ::core::ffi::c_void;
pub type IPrintSchemaAsyncOperationEvent = *mut ::core::ffi::c_void;
pub type IPrintSchemaCapabilities = *mut ::core::ffi::c_void;
pub type IPrintSchemaCapabilities2 = *mut ::core::ffi::c_void;
pub type IPrintSchemaDisplayableElement = *mut ::core::ffi::c_void;
pub type IPrintSchemaElement = *mut ::core::ffi::c_void;
pub type IPrintSchemaFeature = *mut ::core::ffi::c_void;
pub type IPrintSchemaNUpOption = *mut ::core::ffi::c_void;
pub type IPrintSchemaOption = *mut ::core::ffi::c_void;
pub type IPrintSchemaOptionCollection = *mut ::core::ffi::c_void;
pub type IPrintSchemaPageImageableSize = *mut ::core::ffi::c_void;
pub type IPrintSchemaPageMediaSizeOption = *mut ::core::ffi::c_void;
pub type IPrintSchemaParameterDefinition = *mut ::core::ffi::c_void;
pub type IPrintSchemaParameterInitializer = *mut ::core::ffi::c_void;
pub type IPrintSchemaTicket = *mut ::core::ffi::c_void;
pub type IPrintSchemaTicket2 = *mut ::core::ffi::c_void;
pub type IPrintTicketProvider = *mut ::core::ffi::c_void;
pub type IPrintTicketProvider2 = *mut ::core::ffi::c_void;
pub type IPrintUnidiAsyncNotifyRegistration = *mut ::core::ffi::c_void;
pub type IPrintWriteStream = *mut ::core::ffi::c_void;
pub type IPrintWriteStreamFlush = *mut ::core::ffi::c_void;
pub type IPrinterBidiSetRequestCallback = *mut ::core::ffi::c_void;
pub type IPrinterExtensionAsyncOperation = *mut ::core::ffi::c_void;
pub type IPrinterExtensionContext = *mut ::core::ffi::c_void;
pub type IPrinterExtensionContextCollection = *mut ::core::ffi::c_void;
pub type IPrinterExtensionEvent = *mut ::core::ffi::c_void;
pub type IPrinterExtensionEventArgs = *mut ::core::ffi::c_void;
pub type IPrinterExtensionManager = *mut ::core::ffi::c_void;
pub type IPrinterExtensionRequest = *mut ::core::ffi::c_void;
pub type IPrinterPropertyBag = *mut ::core::ffi::c_void;
pub type IPrinterQueue = *mut ::core::ffi::c_void;
pub type IPrinterQueue2 = *mut ::core::ffi::c_void;
pub type IPrinterQueueEvent = *mut ::core::ffi::c_void;
pub type IPrinterQueueView = *mut ::core::ffi::c_void;
pub type IPrinterQueueViewEvent = *mut ::core::ffi::c_void;
pub type IPrinterScriptContext = *mut ::core::ffi::c_void;
pub type IPrinterScriptablePropertyBag = *mut ::core::ffi::c_void;
pub type IPrinterScriptablePropertyBag2 = *mut ::core::ffi::c_void;
pub type IPrinterScriptableSequentialStream = *mut ::core::ffi::c_void;
pub type IPrinterScriptableStream = *mut ::core::ffi::c_void;
pub type IXpsDocument = *mut ::core::ffi::c_void;
pub type IXpsDocumentConsumer = *mut ::core::ffi::c_void;
pub type IXpsDocumentProvider = *mut ::core::ffi::c_void;
pub type IXpsPartIterator = *mut ::core::ffi::c_void;
pub type IXpsRasterizationFactory = *mut ::core::ffi::c_void;
pub type IXpsRasterizationFactory1 = *mut ::core::ffi::c_void;
pub type IXpsRasterizationFactory2 = *mut ::core::ffi::c_void;
pub type IXpsRasterizer = *mut ::core::ffi::c_void;
pub type IXpsRasterizerNotificationCallback = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_ACCESS_ADMINISTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_ACCESS_READ: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_CANCEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_DELETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_LAST_PAGE_EJECTED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RELEASE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RETAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_SENT_TO_PRINTER: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_INFO_1A {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pMachineName: ::windows_sys::core::PSTR,
    pub pUserName: ::windows_sys::core::PSTR,
    pub pDocument: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub pStatus: ::windows_sys::core::PSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOB_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOB_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_INFO_1W {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pDocument: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pStatus: ::windows_sys::core::PWSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOB_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOB_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_2A {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pMachineName: ::windows_sys::core::PSTR,
    pub pUserName: ::windows_sys::core::PSTR,
    pub pDocument: ::windows_sys::core::PSTR,
    pub pNotifyName: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub pPrintProcessor: ::windows_sys::core::PSTR,
    pub pParameters: ::windows_sys::core::PSTR,
    pub pDriverName: ::windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: ::windows_sys::core::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_2W {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pDocument: ::windows_sys::core::PWSTR,
    pub pNotifyName: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pPrintProcessor: ::windows_sys::core::PWSTR,
    pub pParameters: ::windows_sys::core::PWSTR,
    pub pDriverName: ::windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: ::windows_sys::core::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct JOB_INFO_3 {
    pub JobId: u32,
    pub NextJobId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for JOB_INFO_3 {}
impl ::core::clone::Clone for JOB_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_4A {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pMachineName: ::windows_sys::core::PSTR,
    pub pUserName: ::windows_sys::core::PSTR,
    pub pDocument: ::windows_sys::core::PSTR,
    pub pNotifyName: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub pPrintProcessor: ::windows_sys::core::PSTR,
    pub pParameters: ::windows_sys::core::PSTR,
    pub pDriverName: ::windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: ::windows_sys::core::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_4A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_4W {
    pub JobId: u32,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pDocument: ::windows_sys::core::PWSTR,
    pub pNotifyName: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pPrintProcessor: ::windows_sys::core::PWSTR,
    pub pParameters: ::windows_sys::core::PWSTR,
    pub pDriverName: ::windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: ::windows_sys::core::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_4W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DATATYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DEVMODE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DOCUMENT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PARAMETERS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PORT_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_POSITION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_START_TIME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_STATUS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_STATUS_STRING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_SUBMITTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TIME: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_USER_NAME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_POSITION_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_BLOCKED_DEVQ: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_DELETED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_DELETING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_OFFLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PAPEROUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PRINTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PRINTING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RENDERING_LOCALLY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RESTART: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RETAINED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_SPOOLING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_USER_INTERVENTION: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Devices_Display\"`*"]
#[cfg(feature = "Win32_Devices_Display")]
pub struct KERNDATA {
    pub dwSize: u32,
    pub dwKernPairNum: u32,
    pub KernPair: [super::super::Devices::Display::FD_KERNINGPAIR; 1],
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::marker::Copy for KERNDATA {}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::clone::Clone for KERNDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const LPR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MAPTABLE {
    pub dwSize: u32,
    pub dwGlyphNum: u32,
    pub Trans: [TRANSDATA; 1],
}
impl ::core::marker::Copy for MAPTABLE {}
impl ::core::clone::Clone for MAPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_ADDRESS_STR_LEN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_CPSFUNC_INDEX: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_DEVICEDESCRIPTION_STR_LEN: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_DLGPAGE_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_IPADDR_STR_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NETWORKNAME2_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NETWORKNAME_LEN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PORTNAME_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PRIORITY: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_QUEUENAME_LEN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_RES_STR_CHARS: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_SNMP_COMMUNITY_STR_LEN: u32 = 33u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MESSAGEBOX_PARAMS {
    pub cbSize: u32,
    pub pTitle: ::windows_sys::core::PWSTR,
    pub pMessage: ::windows_sys::core::PWSTR,
    pub Style: u32,
    pub dwTimeout: u32,
    pub bWait: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MESSAGEBOX_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MESSAGEBOX_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MIN_PRIORITY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR {
    pub pfnEnumPorts: isize,
    pub pfnOpenPort: isize,
    pub pfnOpenPortEx: isize,
    pub pfnStartDocPort: isize,
    pub pfnWritePort: isize,
    pub pfnReadPort: isize,
    pub pfnEndDocPort: isize,
    pub pfnClosePort: isize,
    pub pfnAddPort: isize,
    pub pfnAddPortEx: isize,
    pub pfnConfigurePort: isize,
    pub pfnDeletePort: isize,
    pub pfnGetPrinterDataFromPort: isize,
    pub pfnSetPortTimeOuts: isize,
    pub pfnXcvOpenPort: isize,
    pub pfnXcvDataPort: isize,
    pub pfnXcvClosePort: isize,
}
impl ::core::marker::Copy for MONITOR {}
impl ::core::clone::Clone for MONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR2 {
    pub cbSize: u32,
    pub pfnEnumPorts: isize,
    pub pfnOpenPort: isize,
    pub pfnOpenPortEx: isize,
    pub pfnStartDocPort: isize,
    pub pfnWritePort: isize,
    pub pfnReadPort: isize,
    pub pfnEndDocPort: isize,
    pub pfnClosePort: isize,
    pub pfnAddPort: isize,
    pub pfnAddPortEx: isize,
    pub pfnConfigurePort: isize,
    pub pfnDeletePort: isize,
    pub pfnGetPrinterDataFromPort: isize,
    pub pfnSetPortTimeOuts: isize,
    pub pfnXcvOpenPort: isize,
    pub pfnXcvDataPort: isize,
    pub pfnXcvClosePort: isize,
    pub pfnShutdown: isize,
    pub pfnSendRecvBidiDataFromPort: isize,
    pub pfnNotifyUsedPorts: isize,
    pub pfnNotifyUnusedPorts: isize,
    pub pfnPowerEvent: isize,
}
impl ::core::marker::Copy for MONITOR2 {}
impl ::core::clone::Clone for MONITOR2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOREX {
    pub dwMonitorSize: u32,
    pub Monitor: MONITOR,
}
impl ::core::marker::Copy for MONITOREX {}
impl ::core::clone::Clone for MONITOREX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct MONITORINIT {
    pub cbSize: u32,
    pub hSpooler: super::super::Foundation::HANDLE,
    pub hckRegistryRoot: super::super::System::Registry::HKEY,
    pub pMonitorReg: *mut MONITORREG,
    pub bLocal: super::super::Foundation::BOOL,
    pub pszServerName: ::windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for MONITORINIT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for MONITORINIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for MONITORREG {}
impl ::core::clone::Clone for MONITORREG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITORUI {
    pub dwMonitorUISize: u32,
    pub pfnAddPortUI: isize,
    pub pfnConfigurePortUI: isize,
    pub pfnDeletePortUI: isize,
}
impl ::core::marker::Copy for MONITORUI {}
impl ::core::clone::Clone for MONITORUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_1A {}
impl ::core::clone::Clone for MONITOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_1W {}
impl ::core::clone::Clone for MONITOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_2A {
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDLLName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_2A {}
impl ::core::clone::Clone for MONITOR_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_2W {
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDLLName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_2W {}
impl ::core::clone::Clone for MONITOR_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MS_PRINT_JOB_OUTPUT_FILE: &str = "MsPrintJobOutputFile";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_ADD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_COMPOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DISABLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DOUBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DOUBLEBYTECHAR_MASK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_FORMAT_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_PAIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_PREDEFIN_MASK: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_REPLACE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_SINGLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_GRAPHICS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_PHYSICAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_RELATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_SENDXMOVECMD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_SENDYMOVECMD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_UPDATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_GET_FILENAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC_SEQ: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_PAGE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_S0PAGE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_S0PAGE_RESOURCE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_XPSPASSTHRU_MODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_ESCAPE: u32 = 4122u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcEscapeHeader {
    pub cbInput: u32,
    pub cbOutput: u32,
    pub opCode: u32,
}
impl ::core::marker::Copy for MxdcEscapeHeader {}
impl ::core::clone::Clone for MxdcEscapeHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcGetFileNameData {
    pub cbOutput: u32,
    pub wszData: [u16; 1],
}
impl ::core::marker::Copy for MxdcGetFileNameData {}
impl ::core::clone::Clone for MxdcGetFileNameData {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type MxdcImageTypeEnums = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGHIGH_COMPRESSION: MxdcImageTypeEnums = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGMEDIUM_COMPRESSION: MxdcImageTypeEnums = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGLOW_COMPRESSION: MxdcImageTypeEnums = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_PNG: MxdcImageTypeEnums = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type MxdcLandscapeRotationEnums = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_90_DEGREES: MxdcLandscapeRotationEnums = 90i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_NONE: MxdcLandscapeRotationEnums = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_270_DEGREES: MxdcLandscapeRotationEnums = -90i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcPrintTicketEscape {
    pub mxdcEscape: MxdcEscapeHeader,
    pub printTicketData: MxdcPrintTicketPassthrough,
}
impl ::core::marker::Copy for MxdcPrintTicketEscape {}
impl ::core::clone::Clone for MxdcPrintTicketEscape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcPrintTicketPassthrough {
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MxdcPrintTicketPassthrough {}
impl ::core::clone::Clone for MxdcPrintTicketPassthrough {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcS0PageData {
    pub dwSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MxdcS0PageData {}
impl ::core::clone::Clone for MxdcS0PageData {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type MxdcS0PageEnums = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_TTF: MxdcS0PageEnums = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_JPEG: MxdcS0PageEnums = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_PNG: MxdcS0PageEnums = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_TIFF: MxdcS0PageEnums = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_WDP: MxdcS0PageEnums = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_DICTIONARY: MxdcS0PageEnums = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_ICC_PROFILE: MxdcS0PageEnums = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_JPEG_THUMBNAIL: MxdcS0PageEnums = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_PNG_THUMBNAIL: MxdcS0PageEnums = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_MAX: MxdcS0PageEnums = 9i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcS0PagePassthroughEscape {
    pub mxdcEscape: MxdcEscapeHeader,
    pub xpsS0PageData: MxdcS0PageData,
}
impl ::core::marker::Copy for MxdcS0PagePassthroughEscape {}
impl ::core::clone::Clone for MxdcS0PagePassthroughEscape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcS0PageResourceEscape {
    pub mxdcEscape: MxdcEscapeHeader,
    pub xpsS0PageResourcePassthrough: MxdcXpsS0PageResource,
}
impl ::core::marker::Copy for MxdcS0PageResourceEscape {}
impl ::core::clone::Clone for MxdcS0PageResourceEscape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MxdcXpsS0PageResource {
    pub dwSize: u32,
    pub dwResourceType: u32,
    pub szUri: [u8; 260],
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MxdcXpsS0PageResource {}
impl ::core::clone::Clone for MxdcXpsS0PageResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NORMAL_PRINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type NOTIFICATION_CALLBACK_COMMANDS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_NOTIFY: NOTIFICATION_CALLBACK_COMMANDS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_CONTEXT_ACQUIRE: NOTIFICATION_CALLBACK_COMMANDS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_CONTEXT_RELEASE: NOTIFICATION_CALLBACK_COMMANDS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_CONFIG_1 {
    pub cbSize: u32,
    pub fdwFlags: u32,
    pub pfnNotifyCallback: ROUTER_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFICATION_CONFIG_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFICATION_CONFIG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type NOTIFICATION_CONFIG_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_CREATE_EVENT: NOTIFICATION_CONFIG_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_REGISTER_CALLBACK: NOTIFICATION_CONFIG_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_EVENT_TRIGGER: NOTIFICATION_CONFIG_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_ASYNC_CHANNEL: NOTIFICATION_CONFIG_FLAGS = 8i32;
pub const NOTIFICATION_RELEASE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3130675239, data2: 42766, data3: 19175, data4: [155, 125, 235, 62, 6, 173, 65, 87] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_COLOR_OPTIMIZATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_PRIORITY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type OEMCUIPCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut CPSUICBPARAM, param1: *mut OEMCUIPPARAM) -> i32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OEMCUIPPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub pDrvOptItems: *mut OPTITEM,
    pub cDrvOptItems: u32,
    pub pOEMOptItems: *mut OPTITEM,
    pub cOEMOptItems: u32,
    pub pOEMUserData: *mut ::core::ffi::c_void,
    pub OEMCUIPCallback: OEMCUIPCALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OEMCUIPPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OEMCUIPPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMCUIP_DOCPROP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMCUIP_PRNPROP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct OEMDMPARAM {
    pub cbSize: u32,
    pub pdriverobj: *mut ::core::ffi::c_void,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub pPublicDMIn: *mut super::Gdi::DEVMODEA,
    pub pPublicDMOut: *mut super::Gdi::DEVMODEA,
    pub pOEMDMIn: *mut ::core::ffi::c_void,
    pub pOEMDMOut: *mut ::core::ffi::c_void,
    pub cbBufSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for OEMDMPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for OEMDMPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_CONVERT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_MERGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_SIZE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMFONTINSTPARAM {
    pub cbSize: u32,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub hHeap: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
    pub pFontInstallerName: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMFONTINSTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMFONTINSTPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_FREEMEM: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_JOBTIMEOUT: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MAX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MAXBITMAP: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MINOUTLINE: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MIN_DOCSTICKY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MIN_PRINTERSTICKY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PRINTFLAGS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PROTOCOL: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_CUSTOMSIZE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_DIALECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_NUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_PSLEVEL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_TTDLFMT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_UNIDM_FLAGS: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_UNIDM_GPDVER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_WAITTIMEOUT: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETINTERFACEVERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETPUBLISHERINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETREQUESTEDHELPERINTERFACES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETSIGNATURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETVERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMPUBLISH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMPUBLISH_IPRINTCOREHELPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_CODEPAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_MARGINS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_NUM_UFMS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_UFM_IDS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMUIOBJ {
    pub cbSize: u32,
    pub pOemUIProcs: *mut OEMUIPROCS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMUIOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMUIOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMUIPROCS {
    pub DrvGetDriverSetting: PFN_DrvGetDriverSetting,
    pub DrvUpdateUISetting: PFN_DrvUpdateUISetting,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMUIPROCS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMUIPROCS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct OEMUIPSPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut ::core::ffi::c_void,
    pub pOEMUserData: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub pOemEntry: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for OEMUIPSPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for OEMUIPSPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct OEM_DMEXTRAHEADER {
    pub dwSize: u32,
    pub dwSignature: u32,
    pub dwVersion: u32,
}
impl ::core::marker::Copy for OEM_DMEXTRAHEADER {}
impl ::core::clone::Clone for OEM_DMEXTRAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEM_MODE_PUBLISHER: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pHelpFile: *mut i8,
    pub dwReserved: [usize; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OIEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OIEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OIEXTF_ANSI_STRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTCF_HIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTCF_MASK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPTCOMBO {
    pub cbSize: u16,
    pub Flags: u8,
    pub cListItem: u16,
    pub pListItem: *mut OPTPARAM,
    pub Sel: i32,
    pub dwReserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTCOMBO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTCOMBO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CALLBACK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CHANGED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CHANGEONCE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_COLLAPSE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_DISABLED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_ECB_CHECKED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_DISABLED: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_HIDE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_HAS_POIEXT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_HIDE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_INITIAL_TVITEM: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_MASK: i32 = 131071i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_SEL_AS_HICON: i32 = 512i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union OPTITEM_0 {
    pub Sel: i32,
    pub pSel: *mut i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union OPTITEM_1 {
    pub pExtChkBox: *mut EXTCHKBOX,
    pub pExtPush: *mut EXTPUSH,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: *mut i8,
    pub IconID: usize,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_HIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_TYPE_DISABLED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_SORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_ADJUST_PAPER_MARGIN_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_HOSTFONT_ENABLED_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_USE_TRUE_COLOR_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCOMPROPSHEET = ::core::option::Option<unsafe extern "system" fn(hcompropsheet: super::super::Foundation::HANDLE, function: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> isize>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPROPSHEETUI = ::core::option::Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvGetDriverSetting = ::core::option::Option<unsafe extern "system" fn(pdriverobj: *mut ::core::ffi::c_void, feature: ::windows_sys::core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUpdateUISetting = ::core::option::Option<unsafe extern "system" fn(pdriverobj: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUpgradeRegistrySetting = ::core::option::Option<unsafe extern "system" fn(hprinter: super::super::Foundation::HANDLE, pfeature: ::windows_sys::core::PCSTR, poption: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for PORT_DATA_1 {}
impl ::core::clone::Clone for PORT_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for PORT_DATA_2 {}
impl ::core::clone::Clone for PORT_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_DATA_LIST_1 {
    pub dwVersion: u32,
    pub cPortData: u32,
    pub pPortData: [PORT_DATA_2; 1],
}
impl ::core::marker::Copy for PORT_DATA_LIST_1 {}
impl ::core::clone::Clone for PORT_DATA_LIST_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PORT_INFO_1A {}
impl ::core::clone::Clone for PORT_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PORT_INFO_1W {}
impl ::core::clone::Clone for PORT_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_2A {
    pub pPortName: ::windows_sys::core::PSTR,
    pub pMonitorName: ::windows_sys::core::PSTR,
    pub pDescription: ::windows_sys::core::PSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PORT_INFO_2A {}
impl ::core::clone::Clone for PORT_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_2W {
    pub pPortName: ::windows_sys::core::PWSTR,
    pub pMonitorName: ::windows_sys::core::PWSTR,
    pub pDescription: ::windows_sys::core::PWSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PORT_INFO_2W {}
impl ::core::clone::Clone for PORT_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_3A {
    pub dwStatus: u32,
    pub pszStatus: ::windows_sys::core::PSTR,
    pub dwSeverity: u32,
}
impl ::core::marker::Copy for PORT_INFO_3A {}
impl ::core::clone::Clone for PORT_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_3W {
    pub dwStatus: u32,
    pub pszStatus: ::windows_sys::core::PWSTR,
    pub dwSeverity: u32,
}
impl ::core::marker::Copy for PORT_INFO_3W {}
impl ::core::clone::Clone for PORT_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_DOOR_OPEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_NO_TONER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OUTPUT_BIN_FULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OUT_OF_MEMORY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_PROBLEM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_POWER_SAVE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TONER_LOW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_INFO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_USER_INTERVENTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_WARMING_UP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_NET_ATTACHED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_READ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_REDIRECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_BOOKLET_EDGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_RIGHT_THEN_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_SQUARE_SCALING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PRINTER_ACCESS_RIGHTS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = 983052u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_READ: PRINTER_ACCESS_RIGHTS = 131080u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE: PRINTER_ACCESS_RIGHTS = 131080u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EXECUTE: PRINTER_ACCESS_RIGHTS = 131080u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = 983043u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_READ: PRINTER_ACCESS_RIGHTS = 131074u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_WRITE: PRINTER_ACCESS_RIGHTS = 131075u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_EXECUTE: PRINTER_ACCESS_RIGHTS = 131074u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DELETE: PRINTER_ACCESS_RIGHTS = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_READ_CONTROL: PRINTER_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE_DAC: PRINTER_ACCESS_RIGHTS = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE_OWNER: PRINTER_ACCESS_RIGHTS = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_SYNCHRONIZE: PRINTER_ACCESS_RIGHTS = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_REQUIRED: PRINTER_ACCESS_RIGHTS = 983040u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_READ: PRINTER_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_WRITE: PRINTER_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_EXECUTE: PRINTER_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ACCESS_ENUMERATE: PRINTER_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_USE: PRINTER_ACCESS_RIGHTS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_MANAGE_LIMITED: PRINTER_ACCESS_RIGHTS = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_FAX: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_LOCAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_MACHINE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_NETWORK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PER_USER: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUBLISHED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_USER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_QUEUED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_RAW_ONLY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_SHARED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_TS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_FORM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_JOB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PORT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ALL: u32 = 2138570751u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_CONFIGURE_PORT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_FORM: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_JOB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PORT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_FORM: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_JOB: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PORT: u32 = 7340032u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINTER: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINTER_DRIVER: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINT_PROCESSOR: u32 = 117440512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SERVER: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_FORM: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_JOB: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_WRITE_JOB: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_CONNECTION_INFO_1A {
    pub dwFlags: u32,
    pub pszDriverName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PRINTER_CONNECTION_INFO_1A {}
impl ::core::clone::Clone for PRINTER_CONNECTION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_CONNECTION_INFO_1W {
    pub dwFlags: u32,
    pub pszDriverName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PRINTER_CONNECTION_INFO_1W {}
impl ::core::clone::Clone for PRINTER_CONNECTION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONNECTION_MISMATCH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONNECTION_NO_UI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_PURGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_SET_STATUS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: ::windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_DEFAULTSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_DEFAULTSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_DEFAULTSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_DEFAULTSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_3D: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_CLOUD: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_FAX: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_FILE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_SERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_DERIVED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_NOT_SHAREABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_PACKAGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SANDBOX_DISABLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SANDBOX_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_XPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CATEGORY_3D: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CATEGORY_ALL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CONNECTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CONTAINER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_EXPAND: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_FAVORITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_HIDE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON2: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON3: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON4: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON6: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON7: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON8: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICONMASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_NETWORK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_REMOTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_SHARED: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: ::windows_sys::core::PSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for PRINTER_ENUM_VALUESA {}
impl ::core::clone::Clone for PRINTER_ENUM_VALUESA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: ::windows_sys::core::PWSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for PRINTER_ENUM_VALUESW {}
impl ::core::clone::Clone for PRINTER_ENUM_VALUESW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_OUTOFPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_OUTOFTONER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_SEVERE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_WARNING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ADD_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ADD_CONNECTION_NO_UI: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ATTRIBUTES_CHANGED: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_EVENT_ATTRIBUTES_INFO {
    pub cbSize: u32,
    pub dwOldAttributes: u32,
    pub dwNewAttributes: u32,
}
impl ::core::marker::Copy for PRINTER_EVENT_ATTRIBUTES_INFO {}
impl ::core::clone::Clone for PRINTER_EVENT_ATTRIBUTES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CACHE_DELETE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CACHE_REFRESH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CONFIGURATION_CHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CONFIGURATION_UPDATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION_NO_UI: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_FLAG_NO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_INITIALIZE: u32 = 3u32;
pub const PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1566185220, data2: 57297, data3: 16769, data4: [142, 238, 129, 92, 134, 237, 173, 49] };
pub const PRINTER_EXTENSION_REASON_DRIVER_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 599462696, data2: 25566, data3: 17043, data4: [145, 91, 166, 162, 61, 146, 154, 203] };
pub const PRINTER_EXTENSION_REASON_PRINT_PREFERENCES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3968804383, data2: 9852, data3: 18079, data4: [181, 214, 57, 51, 2, 60, 41, 204] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_1A {
    pub Flags: u32,
    pub pDescription: ::windows_sys::core::PSTR,
    pub pName: ::windows_sys::core::PSTR,
    pub pComment: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PRINTER_INFO_1A {}
impl ::core::clone::Clone for PRINTER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_1W {
    pub Flags: u32,
    pub pDescription: ::windows_sys::core::PWSTR,
    pub pName: ::windows_sys::core::PWSTR,
    pub pComment: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PRINTER_INFO_1W {}
impl ::core::clone::Clone for PRINTER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct PRINTER_INFO_2A {
    pub pServerName: ::windows_sys::core::PSTR,
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pShareName: ::windows_sys::core::PSTR,
    pub pPortName: ::windows_sys::core::PSTR,
    pub pDriverName: ::windows_sys::core::PSTR,
    pub pComment: ::windows_sys::core::PSTR,
    pub pLocation: ::windows_sys::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pSepFile: ::windows_sys::core::PSTR,
    pub pPrintProcessor: ::windows_sys::core::PSTR,
    pub pDatatype: ::windows_sys::core::PSTR,
    pub pParameters: ::windows_sys::core::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for PRINTER_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for PRINTER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct PRINTER_INFO_2W {
    pub pServerName: ::windows_sys::core::PWSTR,
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pShareName: ::windows_sys::core::PWSTR,
    pub pPortName: ::windows_sys::core::PWSTR,
    pub pDriverName: ::windows_sys::core::PWSTR,
    pub pComment: ::windows_sys::core::PWSTR,
    pub pLocation: ::windows_sys::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pSepFile: ::windows_sys::core::PWSTR,
    pub pPrintProcessor: ::windows_sys::core::PWSTR,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pParameters: ::windows_sys::core::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for PRINTER_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for PRINTER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for PRINTER_INFO_3 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for PRINTER_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_4A {
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pServerName: ::windows_sys::core::PSTR,
    pub Attributes: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_4A {}
impl ::core::clone::Clone for PRINTER_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_4W {
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pServerName: ::windows_sys::core::PWSTR,
    pub Attributes: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_4W {}
impl ::core::clone::Clone for PRINTER_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_5A {
    pub pPrinterName: ::windows_sys::core::PSTR,
    pub pPortName: ::windows_sys::core::PSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_5A {}
impl ::core::clone::Clone for PRINTER_INFO_5A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_5W {
    pub pPrinterName: ::windows_sys::core::PWSTR,
    pub pPortName: ::windows_sys::core::PWSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_5W {}
impl ::core::clone::Clone for PRINTER_INFO_5W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_6 {
    pub dwStatus: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_6 {}
impl ::core::clone::Clone for PRINTER_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_7A {
    pub pszObjectGUID: ::windows_sys::core::PSTR,
    pub dwAction: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_7A {}
impl ::core::clone::Clone for PRINTER_INFO_7A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: ::windows_sys::core::PWSTR,
    pub dwAction: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_7W {}
impl ::core::clone::Clone for PRINTER_INFO_7W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_8A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_8A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_8W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_8W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_8W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_9A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_9A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_9A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_9W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_9W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_9W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_CATEGORY_3D: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_CATEGORY_ALL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_CJOBS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_COMMENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DATATYPE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DEVMODE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_LOCATION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SEPFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_START_TIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO_DATA {
    pub Type: u16,
    pub Field: u16,
    pub Reserved: u32,
    pub Id: u32,
    pub NotifyData: PRINTER_NOTIFY_INFO_DATA_0,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union PRINTER_NOTIFY_INFO_DATA_0 {
    pub adwData: [u32; 2],
    pub Data: PRINTER_NOTIFY_INFO_DATA_0_0,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA_0 {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA_0_0 {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_INFO_DATA_COMPACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INIT {
    pub Size: u32,
    pub Reserved: u32,
    pub PollTime: u32,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INIT {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub pTypes: *mut PRINTER_NOTIFY_OPTIONS_TYPE,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_OPTIONS {}
impl ::core::clone::Clone for PRINTER_NOTIFY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: u16,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Count: u32,
    pub pFields: *mut u16,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_OPTIONS_TYPE {}
impl ::core::clone::Clone for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_ENDPOINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_POLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OEMINTF_VERSION: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_OPTIONSA {
    pub cbSize: u32,
    pub dwFlags: PRINTER_OPTION_FLAGS,
}
impl ::core::marker::Copy for PRINTER_OPTIONSA {}
impl ::core::clone::Clone for PRINTER_OPTIONSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_OPTIONSW {
    pub cbSize: u32,
    pub dwFlags: PRINTER_OPTION_FLAGS,
}
impl ::core::marker::Copy for PRINTER_OPTIONSW {}
impl ::core::clone::Clone for PRINTER_OPTIONSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PRINTER_OPTION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_BUSY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_DOOR_OPEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_INITIALIZING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_IO_ACTIVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_MANUAL_FEED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_NOT_AVAILABLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_NO_TONER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OFFLINE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OUT_OF_MEMORY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAGE_PUNT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_JAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_OUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_PROBLEM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PENDING_DELETION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_POWER_SAVE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PRINTING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PROCESSING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_SERVER_OFFLINE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_SERVER_UNKNOWN: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_TONER_LOW: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_USER_INTERVENTION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_WAITING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_WARMING_UP: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTIFI32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTIFI32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTPROCESSOROPENDATA {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pDatatype: ::windows_sys::core::PWSTR,
    pub pParameters: ::windows_sys::core::PWSTR,
    pub pDocumentName: ::windows_sys::core::PWSTR,
    pub JobId: u32,
    pub pOutputFile: ::windows_sys::core::PWSTR,
    pub pPrinterName: ::windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTPROCESSOROPENDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTPROCESSOROPENDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
}
impl ::core::marker::Copy for PRINTPROCESSOR_CAPS_1 {}
impl ::core::clone::Clone for PRINTPROCESSOR_CAPS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for PRINTPROCESSOR_CAPS_2 {}
impl ::core::clone::Clone for PRINTPROCESSOR_CAPS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PRINTPROCESSOR_INFO_1A {}
impl ::core::clone::Clone for PRINTPROCESSOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PRINTPROCESSOR_INFO_1W {}
impl ::core::clone::Clone for PRINTPROCESSOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
}
impl ::core::marker::Copy for PRINTPROVIDOR {}
impl ::core::clone::Clone for PRINTPROVIDOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PRINT_APP_BIDI_NOTIFY_CHANNEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 716886563, data2: 47508, data3: 19146, data4: [130, 252, 69, 113, 177, 181, 133, 172] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PRINT_EXECUTION_CONTEXT = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: u32,
}
impl ::core::marker::Copy for PRINT_EXECUTION_DATA {}
impl ::core::clone::Clone for PRINT_EXECUTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINT_FEATURE_OPTION {
    pub pszFeature: ::windows_sys::core::PCSTR,
    pub pszOption: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for PRINT_FEATURE_OPTION {}
impl ::core::clone::Clone for PRINT_FEATURE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PRINT_PORT_MONITOR_NOTIFY_CHANNEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635386638, data2: 29865, data3: 18421, data4: [128, 206, 121, 180, 177, 235, 92, 88] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for PROPSHEETUI_GETICON_INFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for PROPSHEETUI_GETICON_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSHEETUI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSHEETUI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub hWndParent: super::super::Foundation::HWND,
    pub hInst: super::super::Foundation::HINSTANCE,
    pub Anonymous: PROPSHEETUI_INFO_HEADER_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETUI_INFO_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETUI_INFO_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETUI_INFO_HEADER_0 {
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub IconID: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETUI_INFO_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETUI_INFO_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_INFO_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_INIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_LPR_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_RAWTCP_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_UNKNOWN_TYPE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_1A {
    pub pName: ::windows_sys::core::PSTR,
    pub pEnvironment: ::windows_sys::core::PSTR,
    pub pDLLName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_1A {}
impl ::core::clone::Clone for PROVIDOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_1W {
    pub pName: ::windows_sys::core::PWSTR,
    pub pEnvironment: ::windows_sys::core::PWSTR,
    pub pDLLName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_1W {}
impl ::core::clone::Clone for PROVIDOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_2A {}
impl ::core::clone::Clone for PROVIDOR_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_2W {}
impl ::core::clone::Clone for PROVIDOR_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PSCRIPT5_PRIVATE_DEVMODE {
    pub wReserved: [u16; 57],
    pub wSize: u16,
}
impl ::core::marker::Copy for PSCRIPT5_PRIVATE_DEVMODE {}
impl ::core::clone::Clone for PSCRIPT5_PRIVATE_DEVMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub hCPSUIPage: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_DEFTITLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_OBSOLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_PROPTITLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_USEHICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIINFO_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_DLL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PUBLISHERINFO {
    pub dwMode: u32,
    pub wMinoutlinePPEM: u16,
    pub wMaxbitmapPPEM: u16,
}
impl ::core::marker::Copy for PUBLISHERINFO {}
impl ::core::clone::Clone for PUBLISHERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PageCountType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FinalPageCount: PageCountType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IntermediatePageCount: PageCountType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintAsyncNotifyConversationStyle = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kBiDirectional: PrintAsyncNotifyConversationStyle = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kUniDirectional: PrintAsyncNotifyConversationStyle = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintAsyncNotifyError = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_SERVER: PrintAsyncNotifyError = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_ANOTHER_LISTENER: PrintAsyncNotifyError = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_SAME_LISTENER: PrintAsyncNotifyError = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_RELEASED_BY_LISTENER: PrintAsyncNotifyError = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNIRECTIONAL_NOTIFICATION_LOST: PrintAsyncNotifyError = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_NOTIFICATION_FAILURE: PrintAsyncNotifyError = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_LISTENERS: PrintAsyncNotifyError = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ALREADY_CLOSED: PrintAsyncNotifyError = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ALREADY_OPENED: PrintAsyncNotifyError = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_WAITING_FOR_CLIENT_NOTIFICATION: PrintAsyncNotifyError = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_NOT_OPENED: PrintAsyncNotifyError = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_CALL_ALREADY_PARKED: PrintAsyncNotifyError = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOT_REGISTERED: PrintAsyncNotifyError = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ALREADY_UNREGISTERED: PrintAsyncNotifyError = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ALREADY_REGISTERED: PrintAsyncNotifyError = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ACQUIRED: PrintAsyncNotifyError = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_CALL_IN_PROGRESS: PrintAsyncNotifyError = 17i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NOTIFICATION_SIZE_EXCEEDED: PrintAsyncNotifyError = 18i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INTERNAL_NOTIFICATION_QUEUE_IS_FULL: PrintAsyncNotifyError = 19i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INVALID_NOTIFICATION_TYPE: PrintAsyncNotifyError = 20i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_REGISTRATION_COUNT_EXCEEDED: PrintAsyncNotifyError = 21i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_CHANNEL_COUNT_EXCEEDED: PrintAsyncNotifyError = 22i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const LOCAL_ONLY_REGISTRATION: PrintAsyncNotifyError = 23i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REMOTE_ONLY_REGISTRATION: PrintAsyncNotifyError = 24i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintAsyncNotifyUserFilter = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPerUser: PrintAsyncNotifyUserFilter = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAllUsers: PrintAsyncNotifyUserFilter = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintJobStatus = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Paused: PrintJobStatus = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Error: PrintJobStatus = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Deleting: PrintJobStatus = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Spooling: PrintJobStatus = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Printing: PrintJobStatus = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Offline: PrintJobStatus = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_PaperOut: PrintJobStatus = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Printed: PrintJobStatus = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Deleted: PrintJobStatus = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_BlockedDeviceQueue: PrintJobStatus = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_UserIntervention: PrintJobStatus = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Restarted: PrintJobStatus = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Complete: PrintJobStatus = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Retained: PrintJobStatus = 8192i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintNamedProperty {
    pub propertyName: ::windows_sys::core::PWSTR,
    pub propertyValue: PrintPropertyValue,
}
impl ::core::marker::Copy for PrintNamedProperty {}
impl ::core::clone::Clone for PrintNamedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertiesCollection {
    pub numberOfProperties: u32,
    pub propertiesCollection: *mut PrintNamedProperty,
}
impl ::core::marker::Copy for PrintPropertiesCollection {}
impl ::core::clone::Clone for PrintPropertiesCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertyValue {
    pub ePropertyType: EPrintPropertyType,
    pub value: PrintPropertyValue_0,
}
impl ::core::marker::Copy for PrintPropertyValue {}
impl ::core::clone::Clone for PrintPropertyValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union PrintPropertyValue_0 {
    pub propertyByte: u8,
    pub propertyString: ::windows_sys::core::PWSTR,
    pub propertyInt32: i32,
    pub propertyInt64: i64,
    pub propertyBlob: PrintPropertyValue_0_0,
}
impl ::core::marker::Copy for PrintPropertyValue_0 {}
impl ::core::clone::Clone for PrintPropertyValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PrintPropertyValue_0_0 {}
impl ::core::clone::Clone for PrintPropertyValue_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PrintSchemaAsyncOperation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1135802429, data2: 4338, data3: 18603, data4: [131, 27, 85, 253, 189, 189, 52, 164] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintSchemaConstrainedSetting = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_None: PrintSchemaConstrainedSetting = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_PrintTicket: PrintSchemaConstrainedSetting = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_Admin: PrintSchemaConstrainedSetting = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_Device: PrintSchemaConstrainedSetting = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintSchemaParameterDataType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_Integer: PrintSchemaParameterDataType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_NumericString: PrintSchemaParameterDataType = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_String: PrintSchemaParameterDataType = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type PrintSchemaSelectionType = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaSelectionType_PickOne: PrintSchemaSelectionType = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaSelectionType_PickMany: PrintSchemaSelectionType = 1i32;
pub const PrinterExtensionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 857432282, data2: 40592, data3: 19920, data4: [156, 132, 234, 196, 230, 89, 182, 31] };
pub const PrinterQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3948200496, data2: 31116, data3: 19614, data4: [180, 97, 41, 250, 208, 64, 57, 177] };
pub const PrinterQueueView: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3948200497, data2: 31116, data3: 19614, data4: [180, 97, 41, 250, 208, 64, 57, 177] };
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_DEVICEPROFILE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_PROFILEDISK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_PROFILEMEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_SOURCEPROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const RAWTCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REVERSE_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const RIGHT_THEN_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ROUTER_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcommand: u32, pcontext: *const ::core::ffi::c_void, dwcolor: u32, pnofityinfo: *const PRINTER_NOTIFY_INFO, fdwflags: u32, pdwresult: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_STOP_ROUTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_NOTIFY_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_FLAG_KEEP_CONFLICT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_FLAG_RESOLVE_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_CONFLICT_REMAINED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_CONFLICT_RESOLVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_NO_CONFLICT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::super::Foundation::HANDLE,
    pub Result: super::super::Foundation::LRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SETRESULT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SETRESULT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type SHIMOPTS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PTSHIM_DEFAULT: SHIMOPTS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PTSHIM_NOSNAPSHOT: SHIMOPTS = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SHOWUIPARAMS {
    pub UIType: UI_TYPE,
    pub MessageBoxParams: MESSAGEBOX_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHOWUIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHOWUIPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SIMULATE_CAPS_1 {
    pub dwLevel: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwCollate: u32,
    pub dwNupOptions: u32,
}
impl ::core::marker::Copy for SIMULATE_CAPS_1 {}
impl ::core::clone::Clone for SIMULATE_CAPS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SPLCLIENT_INFO_1 {
    pub dwSize: u32,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
}
impl ::core::marker::Copy for SPLCLIENT_INFO_1 {}
impl ::core::clone::Clone for SPLCLIENT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SPLCLIENT_INFO_3_VISTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
}
impl ::core::marker::Copy for SPLCLIENT_INFO_3_VISTA {}
impl ::core::clone::Clone for SPLCLIENT_INFO_3_VISTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_ASSET_NUMBER: &str = "assetNumber";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_BYTES_PER_MINUTE: &str = "bytesPerMinute";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DESCRIPTION: &str = "description";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_KEY: &str = "DsDriver";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_NAME: &str = "driverName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_VERSION: &str = "driverVersion";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_FLAGS: &str = "flags";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_LOCATION: &str = "location";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PORT_NAME: &str = "portName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_CLASS: &str = "printQueue";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_LOCATIONS: &str = "printerLocations";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_MODEL: &str = "printerModel";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_NAME: &str = "printerName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_NAME_ALIASES: &str = "printerNameAliases";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_ATTRIBUTES: &str = "printAttributes";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_BIN_NAMES: &str = "printBinNames";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_COLLATE: &str = "printCollate";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_COLOR: &str = "printColor";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: &str = "printDuplexSupported";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_END_TIME: &str = "printEndTime";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: &str = "printKeepPrintedJobs";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_LANGUAGE: &str = "printLanguage";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAC_ADDRESS: &str = "printMACAddress";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: &str = "printMaxResolutionSupported";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_X_EXTENT: &str = "printMaxXExtent";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_Y_EXTENT: &str = "printMaxYExtent";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEDIA_READY: &str = "printMediaReady";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEDIA_SUPPORTED: &str = "printMediaSupported";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEMORY: &str = "printMemory";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MIN_X_EXTENT: &str = "printMinXExtent";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MIN_Y_EXTENT: &str = "printMinYExtent";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NETWORK_ADDRESS: &str = "printNetworkAddress";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NOTIFY: &str = "printNotify";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NUMBER_UP: &str = "printNumberUp";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: &str = "printOrientationsSupported";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_OWNER: &str = "printOwner";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_PAGES_PER_MINUTE: &str = "printPagesPerMinute";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_RATE: &str = "printRate";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_RATE_UNIT: &str = "printRateUnit";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SEPARATOR_FILE: &str = "printSeparatorFile";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SHARE_NAME: &str = "printShareName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SPOOLING: &str = "printSpooling";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_STAPLING_SUPPORTED: &str = "printStaplingSupported";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_START_TIME: &str = "printStartTime";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_STATUS: &str = "printStatus";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRIORITY: &str = "priority";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SERVER_NAME: &str = "serverName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SHORT_SERVER_NAME: &str = "shortServerName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SPOOLER_KEY: &str = "DsSpooler";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_UNC_NAME: &str = "uNCName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_URL: &str = "url";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_USER_KEY: &str = "DsUser";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_VERSION_NUMBER: &str = "versionNumber";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLPRINTER_USER_MODE_PRINTER_DRIVER: &str = "SPLUserModePrinterDriver";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_ALLOW_USER_MANAGEFORMS: &str = "AllowUserManageForms";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_ARCHITECTURE: &str = "Architecture";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_BEEP_ENABLED: &str = "BeepEnabled";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: &str = "DefaultSpoolDirectory";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DNS_MACHINE_NAME: &str = "DNSMachineName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DS_PRESENT: &str = "DsPresent";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DS_PRESENT_FOR_USER: &str = "DsPresentForUser";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_EVENT_LOG: &str = "EventLog";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_MAJOR_VERSION: &str = "MajorVersion";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_MINOR_VERSION: &str = "MinorVersion";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_NET_POPUP: &str = "NetPopup";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_NET_POPUP_TO_COMPUTER: &str = "NetPopupToComputer";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_OS_VERSION: &str = "OSVersion";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_OS_VERSIONEX: &str = "OSVersionEx";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PORT_THREAD_PRIORITY: &str = "PortThreadPriority";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: &str = "PortThreadPriorityDefault";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: &str = "PrintDriverIsolationExecutionPolicy";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: &str = "PrintDriverIsolationGroups";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: &str = "PrintDriverIsolationIdleTimeout";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: &str = "PrintDriverIsolationMaxobjsBeforeRecycle";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: &str = "PrintDriverIsolationOverrideCompat";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: &str = "PrintDriverIsolationTimeBeforeRecycle";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: &str = "PrintQueueV4DriverDirectory";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_REMOTE_FAX: &str = "RemoteFax";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: &str = "RestartJobOnPoolEnabled";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: &str = "RestartJobOnPoolError";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RETRY_POPUP: &str = "RetryPopup";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: &str = "SchedulerThreadPriority";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: &str = "SchedulerThreadPriorityDefault";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_WEBSHAREMGMT: &str = "WebShareMgmt";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPOOL_FILE_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPOOL_FILE_TEMPORARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SR_OWNER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SR_OWNER_PARENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_STDPAGE1: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_STDPAGE2: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_TVPAGE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_LANGPAIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_MUIDLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_DEVCAP_OUTPUT_FULL_REPLACEMENT: ::windows_sys::core::HRESULT = 318465i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_NO_CONFLICT: u32 = 262145u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct TRANSDATA {
    pub ubCodePageID: u8,
    pub ubType: u8,
    pub uCode: TRANSDATA_0,
}
impl ::core::marker::Copy for TRANSDATA {}
impl ::core::clone::Clone for TRANSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union TRANSDATA_0 {
    pub sCode: i16,
    pub ubCode: u8,
    pub ubPairs: [u8; 2],
}
impl ::core::marker::Copy for TRANSDATA_0 {}
impl ::core::clone::Clone for TRANSDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_DONTCARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_GRAPHICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_TTOUTLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_2STATES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_3STATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_CHKBOX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_COMBOBOX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_EDITBOX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_LISTBOX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_NSTATES_EX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_PUSHBUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_SCROLLBAR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_TRACKBAR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_UDARROW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_GLYPHHANDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_GLYPHID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_TRANSDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_UNICODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for UFF_FILEHEADER {}
impl ::core::clone::Clone for UFF_FILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for UFF_FONTDIRECTORY {}
impl ::core::clone::Clone for UFF_FONTDIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFF_VERSION_NUMBER: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_CART: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_SCALABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_SOFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTDOWNLOAD_TTOUTLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTFONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_BOLD_SIM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_ITALIC_SIM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTSUBSTITUTED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_FONTOBJ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHBITMAP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHSTRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHWIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_MEMORY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_STDVARIABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type UI_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kMessageBox: UI_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for UNIDRVINFO {}
impl ::core::clone::Clone for UNIDRVINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNIDRV_PRIVATE_DEVMODE {
    pub wReserved: [u16; 4],
    pub wSize: u16,
}
impl ::core::marker::Copy for UNIDRV_PRIVATE_DEVMODE {}
impl ::core::clone::Clone for UNIDRV_PRIVATE_DEVMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for UNIFM_HDR {}
impl ::core::clone::Clone for UNIFM_HDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNIFM_VERSION_1_0: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNI_CODEPAGEINFO {
    pub dwCodePage: u32,
    pub SelectSymbolSet: INVOC,
    pub UnSelectSymbolSet: INVOC,
}
impl ::core::marker::Copy for UNI_CODEPAGEINFO {}
impl ::core::clone::Clone for UNI_CODEPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
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
impl ::core::marker::Copy for UNI_GLYPHSETDATA {}
impl ::core::clone::Clone for UNI_GLYPHSETDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNI_GLYPHSETDATA_VERSION_1_0: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNKNOWN_PROTOCOL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_SILENT_UPLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_UPLOAD_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USBPRINT_IOCTL_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_CLASSIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_DUAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_IPP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct USERDATA {
    pub dwSize: u32,
    pub dwItemID: usize,
    pub pKeyWordName: ::windows_sys::core::PSTR,
    pub dwReserved: [u32; 8],
}
impl ::core::marker::Copy for USERDATA {}
impl ::core::clone::Clone for USERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct WIDTHRUN {
    pub wStartGlyph: u16,
    pub wGlyphCount: u16,
    pub loCharWidthOffset: u32,
}
impl ::core::marker::Copy for WIDTHRUN {}
impl ::core::clone::Clone for WIDTHRUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct WIDTHTABLE {
    pub dwSize: u32,
    pub dwRunNum: u32,
    pub WidthRun: [WIDTHRUN; 1],
}
impl ::core::marker::Copy for WIDTHTABLE {}
impl ::core::clone::Clone for WIDTHTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const WM_FI_FILENAME: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type XPSRAS_BACKGROUND_COLOR = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_BACKGROUND_COLOR_TRANSPARENT: XPSRAS_BACKGROUND_COLOR = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_BACKGROUND_COLOR_OPAQUE: XPSRAS_BACKGROUND_COLOR = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type XPSRAS_PIXEL_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB: XPSRAS_PIXEL_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB: XPSRAS_PIXEL_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_128BPP_PRGBA_FLOAT_SCRGB: XPSRAS_PIXEL_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub type XPSRAS_RENDERING_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_RENDERING_MODE_ANTIALIASED: XPSRAS_RENDERING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_RENDERING_MODE_ALIASED: XPSRAS_RENDERING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_DRIVER_PROPERTY_BAG: &str = "DriverPropertyBag";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_JOB_ID: &str = "PrintJobId";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_JOB_LEVEL_PRINTTICKET: &str = "JobPrintTicket";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MERGED_DATAFILE_PATH: &str = "MergedDataFilePath";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE: &str = "DriverMultiContentType";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE_OPENXPS: &str = "OpenXPS";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE_XPS: &str = "XPS";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_OUTPUT_FILE: &str = "PrintOutputFileName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTDEVICECAPABILITIES: &str = "PrintDeviceCapabilities";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTER_HANDLE: &str = "PrinterHandle";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTER_NAME: &str = "PrinterName";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINT_CLASS_FACTORY: &str = "PrintClassFactory";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PROGRESS_REPORT: &str = "ProgressReport";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_QUEUE_PROPERTY_BAG: &str = "QueuePropertyBag";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_RESOURCE_DLL_PATHS: &str = "ResourceDLLPaths";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_USER_PRINT_TICKET: &str = "PerUserPrintTicket";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_USER_TOKEN: &str = "UserSecurityToken";
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type _CPSUICALLBACK = ::core::option::Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct _SPLCLIENT_INFO_2_V1 {
    pub hSplPrinter: usize,
}
impl ::core::marker::Copy for _SPLCLIENT_INFO_2_V1 {}
impl ::core::clone::Clone for _SPLCLIENT_INFO_2_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct _SPLCLIENT_INFO_2_V2 {
    pub hSplPrinter: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for _SPLCLIENT_INFO_2_V2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for _SPLCLIENT_INFO_2_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[cfg(target_arch = "x86")]
pub struct _SPLCLIENT_INFO_2_V2 {
    pub hSplPrinter: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for _SPLCLIENT_INFO_2_V2 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for _SPLCLIENT_INFO_2_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct _SPLCLIENT_INFO_2_V3 {
    pub hSplPrinter: u64,
}
impl ::core::marker::Copy for _SPLCLIENT_INFO_2_V3 {}
impl ::core::clone::Clone for _SPLCLIENT_INFO_2_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct __MIDL___MIDL_itf_imgerror_0000_0000_0001 {
    pub description: super::super::Foundation::BSTR,
    pub guid: ::windows_sys::core::GUID,
    pub helpContext: u32,
    pub helpFile: super::super::Foundation::BSTR,
    pub source: super::super::Foundation::BSTR,
    pub devDescription: super::super::Foundation::BSTR,
    pub errorID: ::windows_sys::core::GUID,
    pub cUserParameters: u32,
    pub aUserParameters: *mut super::super::Foundation::BSTR,
    pub userFallback: super::super::Foundation::BSTR,
    pub exceptionID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for __MIDL___MIDL_itf_imgerror_0000_0000_0001 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for __MIDL___MIDL_itf_imgerror_0000_0000_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
