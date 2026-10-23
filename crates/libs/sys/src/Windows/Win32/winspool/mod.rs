#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn AbortPrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddFormA(hprinter : super::HANDLE, level : super::DWORD, pform : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddFormW(hprinter : super::HANDLE, level : super::DWORD, pform : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddJobA(hprinter : super::HANDLE, level : super::DWORD, pdata : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddJobW(hprinter : super::HANDLE, level : super::DWORD, pdata : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddMonitorA(pname : windows_sys::core::PCSTR, level : super::DWORD, pmonitors : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddMonitorW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pmonitors : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn AddPortA(pname : windows_sys::core::PCSTR, hwnd : super::HWND, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn AddPortW(pname : windows_sys::core::PCWSTR, hwnd : super::HWND, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, ppathname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, ppathname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorA(pname : windows_sys::core::PCSTR, level : super::DWORD, pprovidorinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrintProvidorW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pprovidorinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddPrinterA(pname : windows_sys::core::PCSTR, level : super::DWORD, pprinter : super::LPBYTE) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2A(hwnd : super::HWND, pszname : windows_sys::core::PCSTR, dwlevel : super::DWORD, pconnectioninfo : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddPrinterConnection2W(hwnd : super::HWND, pszname : windows_sys::core::PCWSTR, dwlevel : super::DWORD, pconnectioninfo : super::PVOID) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn AddPrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverA(pname : windows_sys::core::PCSTR, level : super::DWORD, pdriverinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExA(pname : windows_sys::core::PCSTR, level : super::DWORD, lpbdriverinfo : super::PBYTE, dwfilecopyflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverExW(pname : windows_sys::core::PCWSTR, level : super::DWORD, lpbdriverinfo : super::PBYTE, dwfilecopyflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn AddPrinterDriverW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pdriverinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AddPrinterW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pprinter : super::LPBYTE) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesA(hwnd : super::HWND, hprinter : super::HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : super::PDEVMODEA, pdevmodeinput : super::PDEVMODEA) -> super::LONG);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesW(hwnd : super::HWND, hprinter : super::HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : super::PDEVMODEW, pdevmodeinput : super::PDEVMODEW) -> super::LONG);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn ClosePrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn CloseSpoolFileHandle(hprinter : super::HANDLE, hspoolfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn CommitSpoolData(hprinter : super::HANDLE, hspoolfile : super::HANDLE, cbcommit : super::DWORD) -> super::HANDLE);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn ConfigurePortA(pname : windows_sys::core::PCSTR, hwnd : super::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn ConfigurePortW(pname : windows_sys::core::PCWSTR, hwnd : super::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ConnectToPrinterDlg(hwnd : super::HWND, flags : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::FILETIME, dwldriverversion : super::DWORDLONG, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn CorePrinterDriverInstalledW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, coredriverguid : windows_sys::core::GUID, ftdriverdate : super::FILETIME, dwldriverversion : super::DWORDLONG, pbdriverinstalled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn DeleteFormA(hprinter : super::HANDLE, pformname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn DeleteFormW(hprinter : super::HANDLE, pformname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeleteJobNamedProperty(hprinter : super::HANDLE, jobid : super::DWORD, pszname : windows_sys::core::PCWSTR) -> super::DWORD);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pmonitorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeleteMonitorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pmonitorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn DeletePortA(pname : windows_sys::core::PCSTR, hwnd : super::HWND, pportname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn DeletePortW(pname : windows_sys::core::PCWSTR, hwnd : super::HWND, pportname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProcessorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pprintprovidorname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrintProvidorW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pprintprovidorname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn DeletePrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionA(pname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterConnectionW(pname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataA(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataExW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDataW(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCWSTR) -> super::DWORD);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, pdrivername : windows_sys::core::PCSTR, dwdeleteflag : super::DWORD, dwversionflag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverExW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR, dwdeleteflag : super::DWORD, dwversionflag : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("winspool.drv" "system" fn DeletePrinterDriverW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, pdrivername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeletePrinterKeyW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesA(hwnd : super::HWND, hprinter : super::HANDLE, pdevicename : windows_sys::core::PCSTR, pdevmodeoutput : super::PDEVMODEA, pdevmodeinput : super::PDEVMODEA, fmode : super::DWORD) -> super::LONG);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DocumentPropertiesW(hwnd : super::HWND, hprinter : super::HANDLE, pdevicename : windows_sys::core::PCWSTR, pdevmodeoutput : super::PDEVMODEW, pdevmodeinput : super::PDEVMODEW, fmode : super::DWORD) -> super::LONG);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn EndDocPrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn EndPagePrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumFormsA(hprinter : super::HANDLE, level : super::DWORD, pform : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumFormsW(hprinter : super::HANDLE, level : super::DWORD, pform : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumJobNamedProperties(hprinter : super::HANDLE, jobid : super::DWORD, pcproperties : *mut super::DWORD, ppproperties : *mut *mut PrintNamedProperty) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumJobsA(hprinter : super::HANDLE, firstjob : super::DWORD, nojobs : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumJobsW(hprinter : super::HANDLE, firstjob : super::DWORD, nojobs : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumMonitorsA(pname : windows_sys::core::PCSTR, level : super::DWORD, pmonitor : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumMonitorsW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pmonitor : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPortsA(pname : windows_sys::core::PCSTR, level : super::DWORD, pport : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPortsW(pname : windows_sys::core::PCWSTR, level : super::DWORD, pport : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesA(pname : windows_sys::core::PCSTR, pprintprocessorname : windows_sys::core::PCSTR, level : super::DWORD, pdatatypes : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesW(pname : windows_sys::core::PCWSTR, pprintprocessorname : windows_sys::core::PCWSTR, level : super::DWORD, pdatatypes : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pprintprocessorinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintProcessorsW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pprintprocessorinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataA(hprinter : super::HANDLE, dwindex : super::DWORD, pvaluename : windows_sys::core::PSTR, cbvaluename : super::DWORD, pcbvaluename : super::LPDWORD, ptype : super::LPDWORD, pdata : super::LPBYTE, cbdata : super::DWORD, pcbdata : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR, penumvalues : super::LPBYTE, cbenumvalues : super::DWORD, pcbenumvalues : super::LPDWORD, pnenumvalues : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataExW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR, penumvalues : super::LPBYTE, cbenumvalues : super::DWORD, pcbenumvalues : super::LPDWORD, pnenumvalues : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDataW(hprinter : super::HANDLE, dwindex : super::DWORD, pvaluename : windows_sys::core::PWSTR, cbvaluename : super::DWORD, pcbvaluename : super::LPDWORD, ptype : super::LPDWORD, pdata : super::LPBYTE, cbdata : super::DWORD, pcbdata : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterDriversW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR, psubkey : windows_sys::core::PSTR, cbsubkey : super::DWORD, pcbsubkey : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn EnumPrinterKeyW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR, psubkey : windows_sys::core::PWSTR, cbsubkey : super::DWORD, pcbsubkey : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintersA(flags : super::DWORD, name : windows_sys::core::PCSTR, level : super::DWORD, pprinterenum : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn EnumPrintersW(flags : super::DWORD, name : windows_sys::core::PCWSTR, level : super::DWORD, pprinterenum : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD, pcreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "C" fn ExtDeviceMode(hwnd : super::HWND, hinst : super::HANDLE, pdevmodeoutput : super::LPDEVMODEA, pdevicename : windows_sys::core::PCSTR, pport : windows_sys::core::PCSTR, pdevmodeinput : super::LPDEVMODEA, pprofile : windows_sys::core::PCSTR, fmode : super::DWORD) -> super::LONG);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn FindClosePrinterChangeNotification(hchange : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn FindFirstPrinterChangeNotification(hprinter : super::HANDLE, fdwfilter : super::DWORD, fdwoptions : super::DWORD, pprinternotifyoptions : super::PVOID) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn FindNextPrinterChangeNotification(hchange : super::HANDLE, pdwchange : super::PDWORD, pvreserved : super::LPVOID, ppprinternotifyinfo : *mut super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn FlushPrinter(hprinter : super::HANDLE, pbuf : super::LPVOID, cbbuf : super::DWORD, pcwritten : super::LPDWORD, csleep : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn FreePrintNamedPropertyArray(cproperties : super::DWORD, ppproperties : *mut *mut PrintNamedProperty));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn FreePrintPropertyValue(pvalue : *mut PrintPropertyValue));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn FreePrinterNotifyInfo(pprinternotifyinfo : PPRINTER_NOTIFY_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszzcoredriverdependencies : windows_sys::core::PCSTR, ccoreprinterdrivers : super::DWORD, pcoreprinterdrivers : PCORE_PRINTER_DRIVERA) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetCorePrinterDriversW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszzcoredriverdependencies : windows_sys::core::PCWSTR, ccoreprinterdrivers : super::DWORD, pcoreprinterdrivers : PCORE_PRINTER_DRIVERW) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterA(pszbuffer : windows_sys::core::PSTR, pcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("winspool.drv" "system" fn GetDefaultPrinterW(pszbuffer : windows_sys::core::PWSTR, pcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetFormA(hprinter : super::HANDLE, pformname : windows_sys::core::PCSTR, level : super::DWORD, pform : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetFormW(hprinter : super::HANDLE, pformname : windows_sys::core::PCWSTR, level : super::DWORD, pform : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetJobA(hprinter : super::HANDLE, jobid : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetJobNamedPropertyValue(hprinter : super::HANDLE, jobid : super::DWORD, pszname : windows_sys::core::PCWSTR, pvalue : *mut PrintPropertyValue) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetJobW(hprinter : super::HANDLE, jobid : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn GetPrintExecutionData(pdata : *mut PRINT_EXECUTION_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrintOutputInfo(hwnd : super::HWND, pszprinter : windows_sys::core::PCWSTR, phfile : *mut super::HANDLE, ppszoutputfile : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pprintprocessorinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pprintprocessorinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterA(hprinter : super::HANDLE, level : super::DWORD, pprinter : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataA(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCSTR, ptype : super::LPDWORD, pdata : super::LPBYTE, nsize : super::DWORD, pcbneeded : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, ptype : super::LPDWORD, pdata : super::LPBYTE, nsize : super::DWORD, pcbneeded : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataExW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, ptype : super::LPDWORD, pdata : super::LPBYTE, nsize : super::DWORD, pcbneeded : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDataW(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCWSTR, ptype : super::LPDWORD, pdata : super::LPBYTE, nsize : super::DWORD, pcbneeded : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2A(hwnd : super::HWND, hprinter : super::HANDLE, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriver2W(hwnd : super::HWND, hprinter : super::HANDLE, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverA(hprinter : super::HANDLE, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryA(pname : windows_sys::core::PCSTR, penvironment : windows_sys::core::PCSTR, level : super::DWORD, pdriverdirectory : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryW(pname : windows_sys::core::PCWSTR, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pdriverdirectory : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathA(pszserver : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, pszlanguage : windows_sys::core::PCSTR, pszpackageid : windows_sys::core::PCSTR, pszdriverpackagecab : windows_sys::core::PSTR, cchdriverpackagecab : super::DWORD, pcchrequiredsize : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathW(pszserver : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, pszlanguage : windows_sys::core::PCWSTR, pszpackageid : windows_sys::core::PCWSTR, pszdriverpackagecab : windows_sys::core::PWSTR, cchdriverpackagecab : super::DWORD, pcchrequiredsize : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterDriverW(hprinter : super::HANDLE, penvironment : windows_sys::core::PCWSTR, level : super::DWORD, pdriverinfo : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn GetPrinterW(hprinter : super::HANDLE, level : super::DWORD, pprinter : super::LPBYTE, cbbuf : super::DWORD, pcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn GetSpoolFileHandle(hprinter : super::HANDLE) -> super::HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszdrivername : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszdrivername : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeA(pdevmode : super::PDEVMODEA, devmodesize : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn IsValidDevmodeW(pdevmode : super::PDEVMODEW, devmodesize : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2A(pprintername : windows_sys::core::PCSTR, phprinter : super::LPHANDLE, pdefault : PPRINTER_DEFAULTSA, poptions : PPRINTER_OPTIONSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinter2W(pprintername : windows_sys::core::PCWSTR, phprinter : super::LPHANDLE, pdefault : PPRINTER_DEFAULTSW, poptions : PPRINTER_OPTIONSW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinterA(pprintername : windows_sys::core::PCSTR, phprinter : super::LPHANDLE, pdefault : LPPRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn OpenPrinterW(pprintername : windows_sys::core::PCWSTR, phprinter : super::LPHANDLE, pdefault : LPPRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxA(hprinter : super::HANDLE, error : super::DWORD, hwnd : super::HWND, ptext : windows_sys::core::PCSTR, pcaption : windows_sys::core::PCSTR, dwtype : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterMessageBoxW(hprinter : super::HANDLE, error : super::DWORD, hwnd : super::HWND, ptext : windows_sys::core::PCWSTR, pcaption : windows_sys::core::PCWSTR, dwtype : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn PrinterProperties(hwnd : super::HWND, hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ReadPrinter(hprinter : super::HANDLE, pbuf : super::LPVOID, cbbuf : super::DWORD, pnobytesread : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ReportJobProcessingProgress(printerhandle : super::HANDLE, jobid : super::ULONG, joboperation : EPrintXPSJobOperation, jobprogress : EPrintXPSJobProgress) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ResetPrinterA(hprinter : super::HANDLE, pdefault : LPPRINTER_DEFAULTSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ResetPrinterW(hprinter : super::HANDLE, pdefault : LPPRINTER_DEFAULTSW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn ScheduleJob(hprinter : super::HANDLE, jobid : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterA(pszprinter : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("winspool.drv" "system" fn SetDefaultPrinterW(pszprinter : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetFormA(hprinter : super::HANDLE, pformname : windows_sys::core::PCSTR, level : super::DWORD, pform : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetFormW(hprinter : super::HANDLE, pformname : windows_sys::core::PCWSTR, level : super::DWORD, pform : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetJobA(hprinter : super::HANDLE, jobid : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, command : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetJobNamedProperty(hprinter : super::HANDLE, jobid : super::DWORD, pproperty : *const PrintNamedProperty) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetJobW(hprinter : super::HANDLE, jobid : super::DWORD, level : super::DWORD, pjob : super::LPBYTE, command : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn SetPortA(pname : windows_sys::core::PCSTR, pportname : windows_sys::core::PCSTR, dwlevel : super::DWORD, pportinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winspool.drv" "system" fn SetPortW(pname : windows_sys::core::PCWSTR, pportname : windows_sys::core::PCWSTR, dwlevel : super::DWORD, pportinfo : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterA(hprinter : super::HANDLE, level : super::DWORD, pprinter : super::LPBYTE, command : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataA(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCSTR, r#type : super::DWORD, pdata : super::LPBYTE, cbdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExA(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCSTR, pvaluename : windows_sys::core::PCSTR, r#type : super::DWORD, pdata : super::LPBYTE, cbdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataExW(hprinter : super::HANDLE, pkeyname : windows_sys::core::PCWSTR, pvaluename : windows_sys::core::PCWSTR, r#type : super::DWORD, pdata : super::LPBYTE, cbdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterDataW(hprinter : super::HANDLE, pvaluename : windows_sys::core::PCWSTR, r#type : super::DWORD, pdata : super::LPBYTE, cbdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn SetPrinterW(hprinter : super::HANDLE, level : super::DWORD, pprinter : super::LPBYTE, command : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn StartDocPrinterA(hprinter : super::HANDLE, level : super::DWORD, pdocinfo : super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn StartDocPrinterW(hprinter : super::HANDLE, level : super::DWORD, pdocinfo : super::LPBYTE) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("winspool.drv" "system" fn StartPagePrinter(hprinter : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageA(pszserver : windows_sys::core::PCSTR, pszinfpath : windows_sys::core::PCSTR, pszenvironment : windows_sys::core::PCSTR, dwflags : super::DWORD, hwnd : super::HWND, pszdestinfpath : windows_sys::core::PSTR, pcchdestinfpath : super::PULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("winspool.drv" "system" fn UploadPrinterDriverPackageW(pszserver : windows_sys::core::PCWSTR, pszinfpath : windows_sys::core::PCWSTR, pszenvironment : windows_sys::core::PCWSTR, dwflags : super::DWORD, hwnd : super::HWND, pszdestinfpath : windows_sys::core::PWSTR, pcchdestinfpath : super::PULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn WaitForPrinterChange(hprinter : super::HANDLE, flags : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn WritePrinter(hprinter : super::HANDLE, pbuf : super::LPVOID, cbbuf : super::DWORD, pcwritten : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn XcvDataW(hxcv : super::HANDLE, pszdataname : windows_sys::core::PCWSTR, pinputdata : super::PBYTE, cbinputdata : super::DWORD, poutputdata : super::PBYTE, cboutputdata : super::DWORD, pcboutputneeded : super::PDWORD, pdwstatus : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
pub type ADDJOB_INFO_1 = ADDJOB_INFO_1A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ADDJOB_INFO_1A {
    pub Path: windows_sys::core::PSTR,
    pub JobId: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ADDJOB_INFO_1W {
    pub Path: windows_sys::core::PWSTR,
    pub JobId: super::DWORD,
}
pub const APD_COPY_ALL_FILES: i32 = 4;
pub const APD_COPY_FROM_DIRECTORY: i32 = 16;
pub const APD_COPY_NEW_FILES: i32 = 8;
pub const APD_STRICT_DOWNGRADE: i32 = 2;
pub const APD_STRICT_UPGRADE: i32 = 1;
pub const BIDI_ACCESS_ADMINISTRATOR: i32 = 1;
pub const BIDI_ACCESS_USER: i32 = 2;
pub const BIDI_ACTION_ENUM_SCHEMA: windows_sys::core::PCWSTR = windows_sys::core::w!("EnumSchema");
pub const BIDI_ACTION_GET: windows_sys::core::PCWSTR = windows_sys::core::w!("Get");
pub const BIDI_ACTION_GET_ALL: windows_sys::core::PCWSTR = windows_sys::core::w!("GetAll");
pub const BIDI_ACTION_GET_WITH_ARGUMENT: windows_sys::core::PCWSTR = windows_sys::core::w!("GetWithArgument");
pub const BIDI_ACTION_SET: windows_sys::core::PCWSTR = windows_sys::core::w!("Set");
pub const BIDI_BLOB: BIDI_TYPE = 7;
pub const BIDI_BOOL: BIDI_TYPE = 3;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct BIDI_DATA {
    pub dwBidiType: super::DWORD,
    pub u: BIDI_DATA_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for BIDI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union BIDI_DATA_0 {
    pub bData: windows_sys::core::BOOL,
    pub iData: super::LONG,
    pub sData: windows_sys::core::PWSTR,
    pub fData: super::FLOAT,
    pub biData: BINARY_CONTAINER,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub Count: super::DWORD,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for BIDI_REQUEST_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: super::DWORD,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for BIDI_REQUEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub Count: super::DWORD,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for BIDI_RESPONSE_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: super::DWORD,
    pub dwReqNumber: super::DWORD,
    pub pSchema: windows_sys::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for BIDI_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_STRING: BIDI_TYPE = 4;
pub const BIDI_TEXT: BIDI_TYPE = 5;
pub type BIDI_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BINARY_CONTAINER {
    pub cbBuf: super::DWORD,
    pub pData: super::LPBYTE,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type CORE_PRINTER_DRIVER = CORE_PRINTER_DRIVERA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub szPackageID: [super::CHAR; 260],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for CORE_PRINTER_DRIVERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: windows_sys::core::GUID,
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub szPackageID: [super::WCHAR; 260],
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for CORE_PRINTER_DRIVERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DATATYPES_INFO_1 = DATATYPES_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DATATYPES_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DATATYPES_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
pub const DEF_PRIORITY: i32 = 1;
pub const DI_CHANNEL: i32 = 1;
pub const DI_MEMORYMAP_WRITE: i32 = 1;
pub const DI_READ_SPOOL_JOB: i32 = 3;
pub type DOC_INFO_1 = DOC_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_1A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_1W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub type DOC_INFO_2 = DOC_INFO_2A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_2A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub dwMode: super::DWORD,
    pub JobId: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_2W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub dwMode: super::DWORD,
    pub JobId: super::DWORD,
}
#[cfg(feature = "shared")]
pub type DOC_INFO_3 = DOC_INFO_3A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_3A {
    pub pDocName: windows_sys::core::PSTR,
    pub pOutputFile: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub dwFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOC_INFO_3W {
    pub pDocName: windows_sys::core::PWSTR,
    pub pOutputFile: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub dwFlags: super::DWORD,
}
pub const DPD_DELETE_ALL_FILES: i32 = 4;
pub const DPD_DELETE_SPECIFIC_VERSION: i32 = 2;
pub const DPD_DELETE_UNUSED_FILES: i32 = 1;
pub type DRIVER_INFO_1 = DRIVER_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub type DRIVER_INFO_2 = DRIVER_INFO_2A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_2A {
    pub cVersion: super::DWORD,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_2W {
    pub cVersion: super::DWORD,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub type DRIVER_INFO_3 = DRIVER_INFO_3A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_3A {
    pub cVersion: super::DWORD,
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
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_3W {
    pub cVersion: super::DWORD,
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
#[cfg(feature = "shared")]
pub type DRIVER_INFO_4 = DRIVER_INFO_4A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_4A {
    pub cVersion: super::DWORD,
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
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_4W {
    pub cVersion: super::DWORD,
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
#[cfg(feature = "shared")]
pub type DRIVER_INFO_5 = DRIVER_INFO_5A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_5A {
    pub cVersion: super::DWORD,
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDriverPath: windows_sys::core::PSTR,
    pub pDataFile: windows_sys::core::PSTR,
    pub pConfigFile: windows_sys::core::PSTR,
    pub dwDriverAttributes: super::DWORD,
    pub dwConfigVersion: super::DWORD,
    pub dwDriverVersion: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_5W {
    pub cVersion: super::DWORD,
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDriverPath: windows_sys::core::PWSTR,
    pub pDataFile: windows_sys::core::PWSTR,
    pub pConfigFile: windows_sys::core::PWSTR,
    pub dwDriverAttributes: super::DWORD,
    pub dwConfigVersion: super::DWORD,
    pub dwDriverVersion: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type DRIVER_INFO_6 = DRIVER_INFO_6A;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_6A {
    pub cVersion: super::DWORD,
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
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub pszMfgName: windows_sys::core::PSTR,
    pub pszOEMUrl: windows_sys::core::PSTR,
    pub pszHardwareID: windows_sys::core::PSTR,
    pub pszProvider: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_6W {
    pub cVersion: super::DWORD,
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
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub pszMfgName: windows_sys::core::PWSTR,
    pub pszOEMUrl: windows_sys::core::PWSTR,
    pub pszHardwareID: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type DRIVER_INFO_8 = DRIVER_INFO_8A;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_8A {
    pub cVersion: super::DWORD,
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
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub pszMfgName: windows_sys::core::PSTR,
    pub pszOEMUrl: windows_sys::core::PSTR,
    pub pszHardwareID: windows_sys::core::PSTR,
    pub pszProvider: windows_sys::core::PSTR,
    pub pszPrintProcessor: windows_sys::core::PSTR,
    pub pszVendorSetup: windows_sys::core::PSTR,
    pub pszzColorProfiles: windows_sys::core::PSTR,
    pub pszInfPath: windows_sys::core::PSTR,
    pub dwPrinterDriverAttributes: super::DWORD,
    pub pszzCoreDriverDependencies: windows_sys::core::PSTR,
    pub ftMinInboxDriverVerDate: super::FILETIME,
    pub dwlMinInboxDriverVerVersion: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_INFO_8W {
    pub cVersion: super::DWORD,
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
    pub ftDriverDate: super::FILETIME,
    pub dwlDriverVersion: super::DWORDLONG,
    pub pszMfgName: windows_sys::core::PWSTR,
    pub pszOEMUrl: windows_sys::core::PWSTR,
    pub pszHardwareID: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
    pub pszPrintProcessor: windows_sys::core::PWSTR,
    pub pszVendorSetup: windows_sys::core::PWSTR,
    pub pszzColorProfiles: windows_sys::core::PWSTR,
    pub pszInfPath: windows_sys::core::PWSTR,
    pub dwPrinterDriverAttributes: super::DWORD,
    pub pszzCoreDriverDependencies: windows_sys::core::PWSTR,
    pub ftMinInboxDriverVerDate: super::FILETIME,
    pub dwlMinInboxDriverVerVersion: super::DWORDLONG,
}
pub const DRIVER_KERNELMODE: i32 = 1;
pub const DRIVER_USERMODE: i32 = 2;
pub const DSPRINT_PENDING: u32 = 2147483648;
pub const DSPRINT_PUBLISH: i32 = 1;
pub const DSPRINT_REPUBLISH: i32 = 8;
pub const DSPRINT_UNPUBLISH: i32 = 4;
pub const DSPRINT_UPDATE: i32 = 2;
pub type EPrintPropertyType = i32;
pub type EPrintXPSJobOperation = i32;
pub type EPrintXPSJobProgress = i32;
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: i32 = 13014;
pub const ERROR_BIDI_DEVICE_OFFLINE: i32 = 13004;
pub const ERROR_BIDI_ERROR_BASE: i32 = 13000;
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: i32 = 13012;
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: i32 = 13013;
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: i32 = 13011;
pub const ERROR_BIDI_NOT_SUPPORTED: i32 = 50;
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: i32 = 13016;
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: i32 = 13015;
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: i32 = 13005;
pub const ERROR_BIDI_SCHEMA_READ_ONLY: i32 = 13002;
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: i32 = 13010;
pub const ERROR_BIDI_SERVER_OFFLINE: i32 = 13003;
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: i32 = 13006;
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: i32 = 13008;
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: i32 = 13007;
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: i32 = 13009;
pub const ERROR_BIDI_STATUS_OK: i32 = 0;
pub const ERROR_BIDI_STATUS_WARNING: i32 = 13001;
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: i32 = 13017;
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: i32 = 13018;
pub const FORM_BUILTIN: i32 = 1;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type FORM_INFO_1 = FORM_INFO_1A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FORM_INFO_1A {
    pub Flags: super::DWORD,
    pub pName: windows_sys::core::PSTR,
    pub Size: super::SIZEL,
    pub ImageableArea: super::RECTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FORM_INFO_1W {
    pub Flags: super::DWORD,
    pub pName: windows_sys::core::PWSTR,
    pub Size: super::SIZEL,
    pub ImageableArea: super::RECTL,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type FORM_INFO_2 = FORM_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FORM_INFO_2A {
    pub Flags: super::DWORD,
    pub pName: windows_sys::core::PCSTR,
    pub Size: super::SIZEL,
    pub ImageableArea: super::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: super::DWORD,
    pub pMuiDll: windows_sys::core::PCSTR,
    pub dwResourceId: super::DWORD,
    pub pDisplayName: windows_sys::core::PCSTR,
    pub wLangId: super::LANGID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FORM_INFO_2W {
    pub Flags: super::DWORD,
    pub pName: windows_sys::core::PCWSTR,
    pub Size: super::SIZEL,
    pub ImageableArea: super::RECTL,
    pub pKeyword: windows_sys::core::PCSTR,
    pub StringType: super::DWORD,
    pub pMuiDll: windows_sys::core::PCWSTR,
    pub dwResourceId: super::DWORD,
    pub pDisplayName: windows_sys::core::PCWSTR,
    pub wLangId: super::LANGID,
}
pub const FORM_PRINTER: i32 = 2;
pub const FORM_USER: i32 = 0;
pub const IPDFP_COPY_ALL_FILES: i32 = 1;
pub const JOB_ACCESS_ADMINISTER: i32 = 16;
pub const JOB_ACCESS_READ: i32 = 32;
pub const JOB_ALL_ACCESS: i32 = 983088;
pub const JOB_CONTROL_CANCEL: i32 = 3;
pub const JOB_CONTROL_DELETE: i32 = 5;
pub const JOB_CONTROL_LAST_PAGE_EJECTED: i32 = 7;
pub const JOB_CONTROL_PAUSE: i32 = 1;
pub const JOB_CONTROL_PENDING_ON_DEVICE: i32 = 11;
pub const JOB_CONTROL_RELEASE: i32 = 9;
pub const JOB_CONTROL_RESTART: i32 = 4;
pub const JOB_CONTROL_RESUME: i32 = 2;
pub const JOB_CONTROL_RETAIN: i32 = 8;
pub const JOB_CONTROL_SEND_TOAST: i32 = 10;
pub const JOB_CONTROL_SENT_TO_PRINTER: i32 = 6;
pub const JOB_EXECUTE: i32 = 131088;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type JOB_INFO_1 = JOB_INFO_1A;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_1A {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pStatus: windows_sys::core::PSTR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub TotalPages: super::DWORD,
    pub PagesPrinted: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_1W {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pStatus: windows_sys::core::PWSTR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub TotalPages: super::DWORD,
    pub PagesPrinted: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type JOB_INFO_2 = JOB_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_2A {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pNotifyName: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pDevMode: super::LPDEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub TotalPages: super::DWORD,
    pub Size: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
    pub Time: super::DWORD,
    pub PagesPrinted: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_2W {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pNotifyName: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pDevMode: super::LPDEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub TotalPages: super::DWORD,
    pub Size: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
    pub Time: super::DWORD,
    pub PagesPrinted: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_3 {
    pub JobId: super::DWORD,
    pub NextJobId: super::DWORD,
    pub Reserved: super::DWORD,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type JOB_INFO_4 = JOB_INFO_4A;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_4A {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pMachineName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDocument: windows_sys::core::PSTR,
    pub pNotifyName: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pDevMode: super::LPDEVMODEA,
    pub pStatus: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub TotalPages: super::DWORD,
    pub Size: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
    pub Time: super::DWORD,
    pub PagesPrinted: super::DWORD,
    pub SizeHigh: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct JOB_INFO_4W {
    pub JobId: super::DWORD,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pMachineName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDocument: windows_sys::core::PWSTR,
    pub pNotifyName: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pDevMode: super::LPDEVMODEW,
    pub pStatus: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Status: super::DWORD,
    pub Priority: super::DWORD,
    pub Position: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub TotalPages: super::DWORD,
    pub Size: super::DWORD,
    pub Submitted: super::SYSTEMTIME,
    pub Time: super::DWORD,
    pub PagesPrinted: super::DWORD,
    pub SizeHigh: super::LONG,
}
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: i32 = 23;
pub const JOB_NOTIFY_FIELD_DATATYPE: i32 = 5;
pub const JOB_NOTIFY_FIELD_DEVMODE: i32 = 9;
pub const JOB_NOTIFY_FIELD_DOCUMENT: i32 = 13;
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: i32 = 8;
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: i32 = 1;
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: i32 = 4;
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: i32 = 21;
pub const JOB_NOTIFY_FIELD_PARAMETERS: i32 = 7;
pub const JOB_NOTIFY_FIELD_PORT_NAME: i32 = 2;
pub const JOB_NOTIFY_FIELD_POSITION: i32 = 15;
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: i32 = 0;
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: i32 = 6;
pub const JOB_NOTIFY_FIELD_PRIORITY: i32 = 14;
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: i32 = 24;
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: i32 = 12;
pub const JOB_NOTIFY_FIELD_START_TIME: i32 = 17;
pub const JOB_NOTIFY_FIELD_STATUS: i32 = 10;
pub const JOB_NOTIFY_FIELD_STATUS_STRING: i32 = 11;
pub const JOB_NOTIFY_FIELD_SUBMITTED: i32 = 16;
pub const JOB_NOTIFY_FIELD_TIME: i32 = 19;
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: i32 = 22;
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: i32 = 20;
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: i32 = 18;
pub const JOB_NOTIFY_FIELD_USER_NAME: i32 = 3;
pub const JOB_NOTIFY_TYPE: i32 = 1;
pub const JOB_POSITION_UNSPECIFIED: i32 = 0;
pub const JOB_READ: i32 = 131104;
pub const JOB_STATUS_BLOCKED_DEVQ: i32 = 512;
pub const JOB_STATUS_COMPLETE: i32 = 4096;
pub const JOB_STATUS_DELETED: i32 = 256;
pub const JOB_STATUS_DELETING: i32 = 4;
pub const JOB_STATUS_ERROR: i32 = 2;
pub const JOB_STATUS_OFFLINE: i32 = 32;
pub const JOB_STATUS_PAPEROUT: i32 = 64;
pub const JOB_STATUS_PAUSED: i32 = 1;
pub const JOB_STATUS_PRINTED: i32 = 128;
pub const JOB_STATUS_PRINTING: i32 = 16;
pub const JOB_STATUS_RENDERING_LOCALLY: i32 = 16384;
pub const JOB_STATUS_RESTART: i32 = 2048;
pub const JOB_STATUS_RETAINED: i32 = 8192;
pub const JOB_STATUS_SPOOLING: i32 = 8;
pub const JOB_STATUS_USER_INTERVENTION: i32 = 1024;
pub const JOB_WRITE: i32 = 131088;
#[cfg(feature = "shared")]
pub type LPADDJOB_INFO_1 = *mut ADDJOB_INFO_1A;
#[cfg(feature = "shared")]
pub type LPADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
#[cfg(feature = "shared")]
pub type LPADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBIDI_DATA = *mut BIDI_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
pub type LPDATATYPES_INFO_1 = *mut DATATYPES_INFO_1A;
pub type LPDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
pub type LPDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
pub type LPDOC_INFO_1 = *mut DOC_INFO_1A;
pub type LPDOC_INFO_1A = *mut DOC_INFO_1A;
pub type LPDOC_INFO_1W = *mut DOC_INFO_1W;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_2 = *mut DOC_INFO_2A;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_2A = *mut DOC_INFO_2A;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_2W = *mut DOC_INFO_2W;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_3 = *mut DOC_INFO_3A;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_3A = *mut DOC_INFO_3A;
#[cfg(feature = "shared")]
pub type LPDOC_INFO_3W = *mut DOC_INFO_3W;
pub type LPDRIVER_INFO_1 = *mut DRIVER_INFO_1A;
pub type LPDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
pub type LPDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_2 = *mut DRIVER_INFO_2A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_3 = *mut DRIVER_INFO_3A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_4 = *mut DRIVER_INFO_4A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_5 = *mut DRIVER_INFO_5A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
#[cfg(feature = "shared")]
pub type LPDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_6 = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_8 = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_1 = *mut FORM_INFO_1A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_1A = *mut FORM_INFO_1A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_1W = *mut FORM_INFO_1W;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_2 = *mut FORM_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_2A = *mut FORM_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFORM_INFO_2W = *mut FORM_INFO_2W;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type LPJOB_INFO_1 = *mut JOB_INFO_1A;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type LPJOB_INFO_1A = *mut JOB_INFO_1A;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type LPJOB_INFO_1W = *mut JOB_INFO_1W;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_2 = *mut JOB_INFO_2A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_2A = *mut JOB_INFO_2A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_2W = *mut JOB_INFO_2W;
#[cfg(feature = "shared")]
pub type LPJOB_INFO_3 = *mut JOB_INFO_3;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_4 = *mut JOB_INFO_4A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_4A = *mut JOB_INFO_4A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPJOB_INFO_4W = *mut JOB_INFO_4W;
pub type LPMONITOR_INFO_1 = *mut MONITOR_INFO_1A;
pub type LPMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
pub type LPMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
pub type LPMONITOR_INFO_2 = *mut MONITOR_INFO_2A;
pub type LPMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
pub type LPMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
pub type LPPORT_INFO_1 = *mut PORT_INFO_1A;
pub type LPPORT_INFO_1A = *mut PORT_INFO_1A;
pub type LPPORT_INFO_1W = *mut PORT_INFO_1W;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_2 = *mut PORT_INFO_2A;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_2A = *mut PORT_INFO_2A;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_2W = *mut PORT_INFO_2W;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_3 = *mut PORT_INFO_3A;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_3A = *mut PORT_INFO_3A;
#[cfg(feature = "shared")]
pub type LPPORT_INFO_3W = *mut PORT_INFO_3W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_DEFAULTS = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
#[cfg(feature = "shared")]
pub type LPPRINTER_ENUM_VALUES = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "shared")]
pub type LPPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "shared")]
pub type LPPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_1 = *mut PRINTER_INFO_1A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_INFO_2 = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
#[cfg(feature = "winnt")]
pub type LPPRINTER_INFO_3 = *mut PRINTER_INFO_3;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_4 = *mut PRINTER_INFO_4A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_5 = *mut PRINTER_INFO_5A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_6 = *mut PRINTER_INFO_6;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_7 = *mut PRINTER_INFO_7A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
#[cfg(feature = "shared")]
pub type LPPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_INFO_8 = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_INFO_9 = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
#[cfg(feature = "shared")]
pub type LPPRINTER_OPTIONS = *mut PRINTER_OPTIONSA;
#[cfg(feature = "shared")]
pub type LPPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
#[cfg(feature = "shared")]
pub type LPPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type LPPRINTPROCESSOR_INFO_1 = *mut PRINTPROCESSOR_INFO_1A;
pub type LPPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
pub type LPPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
pub type LPPROVIDOR_INFO_1 = *mut PROVIDOR_INFO_1A;
pub type LPPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
pub type LPPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
pub type LPPROVIDOR_INFO_2 = *mut PROVIDOR_INFO_2A;
pub type LPPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
pub type LPPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
pub const MAX_FORM_KEYWORD_LENGTH: i32 = 64;
pub const MAX_PRIORITY: i32 = 99;
pub const MIN_PRIORITY: i32 = 1;
pub type MONITOR_INFO_1 = MONITOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
pub type MONITOR_INFO_2 = MONITOR_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITOR_INFO_2A {
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDLLName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITOR_INFO_2W {
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDLLName: windows_sys::core::PWSTR,
}
pub const MS_PRINT_JOB_OUTPUT_FILE: windows_sys::core::PCWSTR = windows_sys::core::w!("MsPrintJobOutputFile");
pub const NORMAL_PRINT: i32 = 0;
pub const NO_PRIORITY: i32 = 0;
#[cfg(feature = "shared")]
pub type PADDJOB_INFO_1 = *mut ADDJOB_INFO_1A;
#[cfg(feature = "shared")]
pub type PADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
#[cfg(feature = "shared")]
pub type PADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBIDI_DATA = *mut BIDI_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
#[cfg(feature = "shared")]
pub type PBINARY_CONTAINER = *mut BINARY_CONTAINER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCORE_PRINTER_DRIVER = *mut CORE_PRINTER_DRIVERA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCORE_PRINTER_DRIVERA = *mut CORE_PRINTER_DRIVERA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PCORE_PRINTER_DRIVERW = *mut CORE_PRINTER_DRIVERW;
pub type PDATATYPES_INFO_1 = *mut DATATYPES_INFO_1A;
pub type PDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
pub type PDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
pub type PDOC_INFO_1 = *mut DOC_INFO_1A;
pub type PDOC_INFO_1A = *mut DOC_INFO_1A;
pub type PDOC_INFO_1W = *mut DOC_INFO_1W;
#[cfg(feature = "shared")]
pub type PDOC_INFO_2 = *mut DOC_INFO_2A;
#[cfg(feature = "shared")]
pub type PDOC_INFO_2A = *mut DOC_INFO_2A;
#[cfg(feature = "shared")]
pub type PDOC_INFO_2W = *mut DOC_INFO_2W;
#[cfg(feature = "shared")]
pub type PDOC_INFO_3 = *mut DOC_INFO_3A;
#[cfg(feature = "shared")]
pub type PDOC_INFO_3A = *mut DOC_INFO_3A;
#[cfg(feature = "shared")]
pub type PDOC_INFO_3W = *mut DOC_INFO_3W;
pub type PDRIVER_INFO_1 = *mut DRIVER_INFO_1A;
pub type PDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
pub type PDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_2 = *mut DRIVER_INFO_2A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_3 = *mut DRIVER_INFO_3A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_4 = *mut DRIVER_INFO_4A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_5 = *mut DRIVER_INFO_5A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
#[cfg(feature = "shared")]
pub type PDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_6 = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_8 = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_1 = *mut FORM_INFO_1A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_1A = *mut FORM_INFO_1A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_1W = *mut FORM_INFO_1W;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_2 = *mut FORM_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_2A = *mut FORM_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PFORM_INFO_2W = *mut FORM_INFO_2W;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type PJOB_INFO_1 = *mut JOB_INFO_1A;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type PJOB_INFO_1A = *mut JOB_INFO_1A;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type PJOB_INFO_1W = *mut JOB_INFO_1W;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_2 = *mut JOB_INFO_2A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_2A = *mut JOB_INFO_2A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_2W = *mut JOB_INFO_2W;
#[cfg(feature = "shared")]
pub type PJOB_INFO_3 = *mut JOB_INFO_3;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_4 = *mut JOB_INFO_4A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_4A = *mut JOB_INFO_4A;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PJOB_INFO_4W = *mut JOB_INFO_4W;
pub type PMONITOR_INFO_1 = *mut MONITOR_INFO_1A;
pub type PMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
pub type PMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
pub type PMONITOR_INFO_2 = *mut MONITOR_INFO_2A;
pub type PMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
pub type PMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
pub type PORT_INFO_1 = PORT_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub type PORT_INFO_2 = PORT_INFO_2A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_2A {
    pub pPortName: windows_sys::core::PSTR,
    pub pMonitorName: windows_sys::core::PSTR,
    pub pDescription: windows_sys::core::PSTR,
    pub fPortType: super::DWORD,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_2W {
    pub pPortName: windows_sys::core::PWSTR,
    pub pMonitorName: windows_sys::core::PWSTR,
    pub pDescription: windows_sys::core::PWSTR,
    pub fPortType: super::DWORD,
    pub Reserved: super::DWORD,
}
#[cfg(feature = "shared")]
pub type PORT_INFO_3 = PORT_INFO_3A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_3A {
    pub dwStatus: super::DWORD,
    pub pszStatus: windows_sys::core::PSTR,
    pub dwSeverity: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PORT_INFO_3W {
    pub dwStatus: super::DWORD,
    pub pszStatus: windows_sys::core::PWSTR,
    pub dwSeverity: super::DWORD,
}
pub const PORT_STATUS_DOOR_OPEN: i32 = 7;
pub const PORT_STATUS_NO_TONER: i32 = 6;
pub const PORT_STATUS_OFFLINE: i32 = 1;
pub const PORT_STATUS_OUTPUT_BIN_FULL: i32 = 4;
pub const PORT_STATUS_OUT_OF_MEMORY: i32 = 9;
pub const PORT_STATUS_PAPER_JAM: i32 = 2;
pub const PORT_STATUS_PAPER_OUT: i32 = 3;
pub const PORT_STATUS_PAPER_PROBLEM: i32 = 5;
pub const PORT_STATUS_POWER_SAVE: i32 = 12;
pub const PORT_STATUS_TONER_LOW: i32 = 10;
pub const PORT_STATUS_TYPE_ERROR: i32 = 1;
pub const PORT_STATUS_TYPE_INFO: i32 = 3;
pub const PORT_STATUS_TYPE_WARNING: i32 = 2;
pub const PORT_STATUS_USER_INTERVENTION: i32 = 8;
pub const PORT_STATUS_WARMING_UP: i32 = 11;
pub const PORT_TYPE_NET_ATTACHED: i32 = 8;
pub const PORT_TYPE_READ: i32 = 2;
pub const PORT_TYPE_REDIRECTED: i32 = 4;
pub const PORT_TYPE_WRITE: i32 = 1;
pub const PPCAPS_BOOKLET_EDGE: i32 = 1;
pub const PPCAPS_BORDER_PRINT: i32 = 1;
pub const PPCAPS_DONT_SEND_EXTRA_PAGES_FOR_DUPLEX: i32 = 2;
pub const PPCAPS_DOWN_THEN_LEFT: i32 = 8;
pub const PPCAPS_DOWN_THEN_RIGHT: i32 = 2;
pub const PPCAPS_LEFT_THEN_DOWN: i32 = 4;
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: i32 = 1;
pub const PPCAPS_RIGHT_THEN_DOWN: i32 = 1;
pub const PPCAPS_SQUARE_SCALING: i32 = 1;
pub type PPORT_INFO_1 = *mut PORT_INFO_1A;
pub type PPORT_INFO_1A = *mut PORT_INFO_1A;
pub type PPORT_INFO_1W = *mut PORT_INFO_1W;
#[cfg(feature = "shared")]
pub type PPORT_INFO_2 = *mut PORT_INFO_2A;
#[cfg(feature = "shared")]
pub type PPORT_INFO_2A = *mut PORT_INFO_2A;
#[cfg(feature = "shared")]
pub type PPORT_INFO_2W = *mut PORT_INFO_2W;
#[cfg(feature = "shared")]
pub type PPORT_INFO_3 = *mut PORT_INFO_3A;
#[cfg(feature = "shared")]
pub type PPORT_INFO_3A = *mut PORT_INFO_3A;
#[cfg(feature = "shared")]
pub type PPORT_INFO_3W = *mut PORT_INFO_3W;
#[cfg(feature = "shared")]
pub type PPRINTER_CONNECTION_INFO_1 = *mut PRINTER_CONNECTION_INFO_1A;
#[cfg(feature = "shared")]
pub type PPRINTER_CONNECTION_INFO_1A = *mut PRINTER_CONNECTION_INFO_1A;
#[cfg(feature = "shared")]
pub type PPRINTER_CONNECTION_INFO_1W = *mut PRINTER_CONNECTION_INFO_1W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_DEFAULTS = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
#[cfg(feature = "shared")]
pub type PPRINTER_ENUM_VALUES = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "shared")]
pub type PPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
#[cfg(feature = "shared")]
pub type PPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_1 = *mut PRINTER_INFO_1A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_INFO_2 = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
#[cfg(feature = "winnt")]
pub type PPRINTER_INFO_3 = *mut PRINTER_INFO_3;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_4 = *mut PRINTER_INFO_4A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_5 = *mut PRINTER_INFO_5A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_6 = *mut PRINTER_INFO_6;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_7 = *mut PRINTER_INFO_7A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
#[cfg(feature = "shared")]
pub type PPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_INFO_8 = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_INFO_9 = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
#[cfg(feature = "shared")]
pub type PPRINTER_OPTIONS = *mut PRINTER_OPTIONSA;
#[cfg(feature = "shared")]
pub type PPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
#[cfg(feature = "shared")]
pub type PPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
#[cfg(feature = "shared")]
pub type PPRINTPROCESSOR_CAPS_1 = *mut PRINTPROCESSOR_CAPS_1;
#[cfg(feature = "shared")]
pub type PPRINTPROCESSOR_CAPS_2 = *mut PRINTPROCESSOR_CAPS_2;
pub type PPRINTPROCESSOR_INFO_1 = *mut PRINTPROCESSOR_INFO_1A;
pub type PPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
pub type PPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
pub type PPROVIDOR_INFO_1 = *mut PROVIDOR_INFO_1A;
pub type PPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
pub type PPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
pub type PPROVIDOR_INFO_2 = *mut PROVIDOR_INFO_2A;
pub type PPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
pub type PPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
pub const PRINTER_ACCESS_ADMINISTER: i32 = 4;
pub const PRINTER_ACCESS_MANAGE_LIMITED: i32 = 64;
pub const PRINTER_ACCESS_USE: i32 = 8;
pub const PRINTER_ALL_ACCESS: i32 = 983052;
pub const PRINTER_ATTRIBUTE_DEFAULT: i32 = 4;
pub const PRINTER_ATTRIBUTE_DIRECT: i32 = 2;
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: i32 = 512;
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: i32 = 2048;
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: i32 = 128;
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: i32 = 8388608;
pub const PRINTER_ATTRIBUTE_FAX: i32 = 16384;
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: i32 = 1048576;
pub const PRINTER_ATTRIBUTE_HIDDEN: i32 = 32;
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: i32 = 256;
pub const PRINTER_ATTRIBUTE_LOCAL: i32 = 64;
pub const PRINTER_ATTRIBUTE_MACHINE: i32 = 524288;
pub const PRINTER_ATTRIBUTE_NETWORK: i32 = 16;
pub const PRINTER_ATTRIBUTE_PER_USER: i32 = 4194304;
pub const PRINTER_ATTRIBUTE_PUBLISHED: i32 = 8192;
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: i32 = 262144;
pub const PRINTER_ATTRIBUTE_PUSHED_USER: i32 = 131072;
pub const PRINTER_ATTRIBUTE_QUEUED: i32 = 1;
pub const PRINTER_ATTRIBUTE_RAW_ONLY: i32 = 4096;
pub const PRINTER_ATTRIBUTE_SHARED: i32 = 8;
pub const PRINTER_ATTRIBUTE_TS: i32 = 32768;
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: i32 = 2097152;
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: i32 = 1024;
pub const PRINTER_CHANGE_ADD_FORM: i32 = 65536;
pub const PRINTER_CHANGE_ADD_JOB: i32 = 256;
pub const PRINTER_CHANGE_ADD_PORT: i32 = 1048576;
pub const PRINTER_CHANGE_ADD_PRINTER: i32 = 1;
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: i32 = 268435456;
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: i32 = 16777216;
pub const PRINTER_CHANGE_ALL: i32 = 2138570751;
pub const PRINTER_CHANGE_CONFIGURE_PORT: i32 = 2097152;
pub const PRINTER_CHANGE_DELETE_FORM: i32 = 262144;
pub const PRINTER_CHANGE_DELETE_JOB: i32 = 1024;
pub const PRINTER_CHANGE_DELETE_PORT: i32 = 4194304;
pub const PRINTER_CHANGE_DELETE_PRINTER: i32 = 4;
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: i32 = 1073741824;
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: i32 = 67108864;
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: i32 = 8;
pub const PRINTER_CHANGE_FORM: i32 = 458752;
pub const PRINTER_CHANGE_JOB: i32 = 65280;
pub const PRINTER_CHANGE_PORT: i32 = 7340032;
pub const PRINTER_CHANGE_PRINTER: i32 = 255;
pub const PRINTER_CHANGE_PRINTER_DRIVER: i32 = 1879048192;
pub const PRINTER_CHANGE_PRINT_PROCESSOR: i32 = 117440512;
pub const PRINTER_CHANGE_SERVER: i32 = 134217728;
pub const PRINTER_CHANGE_SET_FORM: i32 = 131072;
pub const PRINTER_CHANGE_SET_JOB: i32 = 512;
pub const PRINTER_CHANGE_SET_PRINTER: i32 = 2;
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: i32 = 536870912;
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648;
pub const PRINTER_CHANGE_WRITE_JOB: i32 = 2048;
#[cfg(feature = "shared")]
pub type PRINTER_CONNECTION_INFO_1 = PRINTER_CONNECTION_INFO_1A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_CONNECTION_INFO_1A {
    pub dwFlags: super::DWORD,
    pub pszDriverName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_CONNECTION_INFO_1W {
    pub dwFlags: super::DWORD,
    pub pszDriverName: windows_sys::core::PWSTR,
}
pub const PRINTER_CONNECTION_MISMATCH: i32 = 32;
pub const PRINTER_CONNECTION_NO_UI: i32 = 64;
pub const PRINTER_CONTROL_PAUSE: i32 = 1;
pub const PRINTER_CONTROL_PURGE: i32 = 3;
pub const PRINTER_CONTROL_RESUME: i32 = 2;
pub const PRINTER_CONTROL_SET_STATUS: i32 = 4;
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PRINTER_DEFAULTS = PRINTER_DEFAULTSA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: windows_sys::core::PSTR,
    pub pDevMode: super::LPDEVMODEA,
    pub DesiredAccess: super::ACCESS_MASK,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: windows_sys::core::PWSTR,
    pub pDevMode: super::LPDEVMODEW,
    pub DesiredAccess: super::ACCESS_MASK,
}
pub const PRINTER_DRIVER_CATEGORY_3D: i32 = 4096;
pub const PRINTER_DRIVER_CATEGORY_CLOUD: i32 = 8192;
pub const PRINTER_DRIVER_CATEGORY_FAX: i32 = 64;
pub const PRINTER_DRIVER_CATEGORY_FILE: i32 = 128;
pub const PRINTER_DRIVER_CATEGORY_SERVICE: i32 = 512;
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: i32 = 256;
pub const PRINTER_DRIVER_CLASS: i32 = 8;
pub const PRINTER_DRIVER_DERIVED: i32 = 16;
pub const PRINTER_DRIVER_NOT_SHAREABLE: i32 = 32;
pub const PRINTER_DRIVER_PACKAGE_AWARE: i32 = 1;
pub const PRINTER_DRIVER_SANDBOX_DISABLED: i32 = 2048;
pub const PRINTER_DRIVER_SANDBOX_ENABLED: i32 = 4;
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: i32 = 1024;
pub const PRINTER_DRIVER_XPS: i32 = 2;
pub const PRINTER_ENUM_CATEGORY_3D: i32 = 67108864;
pub const PRINTER_ENUM_CATEGORY_ALL: i32 = 33554432;
pub const PRINTER_ENUM_CONNECTIONS: i32 = 4;
pub const PRINTER_ENUM_CONTAINER: i32 = 32768;
pub const PRINTER_ENUM_DEFAULT: i32 = 1;
pub const PRINTER_ENUM_EXPAND: i32 = 16384;
pub const PRINTER_ENUM_FAVORITE: i32 = 4;
pub const PRINTER_ENUM_HIDE: i32 = 16777216;
pub const PRINTER_ENUM_ICON1: i32 = 65536;
pub const PRINTER_ENUM_ICON2: i32 = 131072;
pub const PRINTER_ENUM_ICON3: i32 = 262144;
pub const PRINTER_ENUM_ICON4: i32 = 524288;
pub const PRINTER_ENUM_ICON5: i32 = 1048576;
pub const PRINTER_ENUM_ICON6: i32 = 2097152;
pub const PRINTER_ENUM_ICON7: i32 = 4194304;
pub const PRINTER_ENUM_ICON8: i32 = 8388608;
pub const PRINTER_ENUM_ICONMASK: i32 = 16711680;
pub const PRINTER_ENUM_LOCAL: i32 = 2;
pub const PRINTER_ENUM_NAME: i32 = 8;
pub const PRINTER_ENUM_NETWORK: i32 = 64;
pub const PRINTER_ENUM_REMOTE: i32 = 16;
pub const PRINTER_ENUM_SHARED: i32 = 32;
#[cfg(feature = "shared")]
pub type PRINTER_ENUM_VALUES = PRINTER_ENUM_VALUESA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: windows_sys::core::PSTR,
    pub cbValueName: super::DWORD,
    pub dwType: super::DWORD,
    pub pData: super::LPBYTE,
    pub cbData: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: windows_sys::core::PWSTR,
    pub cbValueName: super::DWORD,
    pub dwType: super::DWORD,
    pub pData: super::LPBYTE,
    pub cbData: super::DWORD,
}
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648;
pub const PRINTER_ERROR_JAM: i32 = 2;
pub const PRINTER_ERROR_OUTOFPAPER: i32 = 1;
pub const PRINTER_ERROR_OUTOFTONER: i32 = 4;
pub const PRINTER_ERROR_SEVERE: i32 = 536870912;
pub const PRINTER_ERROR_WARNING: i32 = 1073741824;
pub const PRINTER_EXECUTE: i32 = 131080;
#[cfg(feature = "shared")]
pub type PRINTER_INFO_1 = PRINTER_INFO_1A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_1A {
    pub Flags: super::DWORD,
    pub pDescription: windows_sys::core::PSTR,
    pub pName: windows_sys::core::PSTR,
    pub pComment: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_1W {
    pub Flags: super::DWORD,
    pub pDescription: windows_sys::core::PWSTR,
    pub pName: windows_sys::core::PWSTR,
    pub pComment: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PRINTER_INFO_2 = PRINTER_INFO_2A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_2A {
    pub pServerName: windows_sys::core::PSTR,
    pub pPrinterName: windows_sys::core::PSTR,
    pub pShareName: windows_sys::core::PSTR,
    pub pPortName: windows_sys::core::PSTR,
    pub pDriverName: windows_sys::core::PSTR,
    pub pComment: windows_sys::core::PSTR,
    pub pLocation: windows_sys::core::PSTR,
    pub pDevMode: super::LPDEVMODEA,
    pub pSepFile: windows_sys::core::PSTR,
    pub pPrintProcessor: windows_sys::core::PSTR,
    pub pDatatype: windows_sys::core::PSTR,
    pub pParameters: windows_sys::core::PSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Attributes: super::DWORD,
    pub Priority: super::DWORD,
    pub DefaultPriority: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub Status: super::DWORD,
    pub cJobs: super::DWORD,
    pub AveragePPM: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_2W {
    pub pServerName: windows_sys::core::PWSTR,
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pShareName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub pDriverName: windows_sys::core::PWSTR,
    pub pComment: windows_sys::core::PWSTR,
    pub pLocation: windows_sys::core::PWSTR,
    pub pDevMode: super::LPDEVMODEW,
    pub pSepFile: windows_sys::core::PWSTR,
    pub pPrintProcessor: windows_sys::core::PWSTR,
    pub pDatatype: windows_sys::core::PWSTR,
    pub pParameters: windows_sys::core::PWSTR,
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
    pub Attributes: super::DWORD,
    pub Priority: super::DWORD,
    pub DefaultPriority: super::DWORD,
    pub StartTime: super::DWORD,
    pub UntilTime: super::DWORD,
    pub Status: super::DWORD,
    pub cJobs: super::DWORD,
    pub AveragePPM: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "shared")]
pub type PRINTER_INFO_4 = PRINTER_INFO_4A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_4A {
    pub pPrinterName: windows_sys::core::PSTR,
    pub pServerName: windows_sys::core::PSTR,
    pub Attributes: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_4W {
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pServerName: windows_sys::core::PWSTR,
    pub Attributes: super::DWORD,
}
#[cfg(feature = "shared")]
pub type PRINTER_INFO_5 = PRINTER_INFO_5A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_5A {
    pub pPrinterName: windows_sys::core::PSTR,
    pub pPortName: windows_sys::core::PSTR,
    pub Attributes: super::DWORD,
    pub DeviceNotSelectedTimeout: super::DWORD,
    pub TransmissionRetryTimeout: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_5W {
    pub pPrinterName: windows_sys::core::PWSTR,
    pub pPortName: windows_sys::core::PWSTR,
    pub Attributes: super::DWORD,
    pub DeviceNotSelectedTimeout: super::DWORD,
    pub TransmissionRetryTimeout: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_6 {
    pub dwStatus: super::DWORD,
}
#[cfg(feature = "shared")]
pub type PRINTER_INFO_7 = PRINTER_INFO_7A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_7A {
    pub pszObjectGUID: windows_sys::core::PSTR,
    pub dwAction: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: windows_sys::core::PWSTR,
    pub dwAction: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PRINTER_INFO_8 = PRINTER_INFO_8A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_8A {
    pub pDevMode: super::LPDEVMODEA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_8W {
    pub pDevMode: super::LPDEVMODEW,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PRINTER_INFO_9 = PRINTER_INFO_9A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_9A {
    pub pDevMode: super::LPDEVMODEA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_INFO_9W {
    pub pDevMode: super::LPDEVMODEW,
}
pub const PRINTER_NOTIFY_CATEGORY_3D: i32 = 8192;
pub const PRINTER_NOTIFY_CATEGORY_ALL: i32 = 4096;
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: i32 = 13;
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: i32 = 21;
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: i32 = 28;
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: i32 = 25;
pub const PRINTER_NOTIFY_FIELD_CJOBS: i32 = 20;
pub const PRINTER_NOTIFY_FIELD_COMMENT: i32 = 5;
pub const PRINTER_NOTIFY_FIELD_DATATYPE: i32 = 11;
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: i32 = 15;
pub const PRINTER_NOTIFY_FIELD_DEVMODE: i32 = 7;
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: i32 = 4;
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: i32 = 27;
pub const PRINTER_NOTIFY_FIELD_LOCATION: i32 = 6;
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: i32 = 26;
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: i32 = 23;
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: i32 = 10;
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: i32 = 3;
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: i32 = 1;
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: i32 = 9;
pub const PRINTER_NOTIFY_FIELD_PRIORITY: i32 = 14;
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: i32 = 12;
pub const PRINTER_NOTIFY_FIELD_SEPFILE: i32 = 8;
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: i32 = 0;
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: i32 = 2;
pub const PRINTER_NOTIFY_FIELD_START_TIME: i32 = 16;
pub const PRINTER_NOTIFY_FIELD_STATUS: i32 = 18;
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: i32 = 19;
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: i32 = 24;
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: i32 = 22;
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: i32 = 17;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub Count: super::DWORD,
    pub aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PRINTER_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO_DATA {
    pub Type: super::WORD,
    pub Field: super::WORD,
    pub Reserved: super::DWORD,
    pub Id: super::DWORD,
    pub NotifyData: PRINTER_NOTIFY_INFO_DATA_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PRINTER_NOTIFY_INFO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union PRINTER_NOTIFY_INFO_DATA_0 {
    pub adwData: [super::DWORD; 2],
    pub Data: PRINTER_NOTIFY_INFO_DATA_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PRINTER_NOTIFY_INFO_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: super::DWORD,
    pub pBuf: super::LPVOID,
}
pub const PRINTER_NOTIFY_INFO_DISCARDED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub Count: super::DWORD,
    pub pTypes: PPRINTER_NOTIFY_OPTIONS_TYPE,
}
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: super::WORD,
    pub Reserved0: super::WORD,
    pub Reserved1: super::DWORD,
    pub Reserved2: super::DWORD,
    pub Count: super::DWORD,
    pub pFields: super::PWORD,
}
pub const PRINTER_NOTIFY_TYPE: i32 = 0;
#[cfg(feature = "shared")]
pub type PRINTER_OPTIONS = PRINTER_OPTIONSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_OPTIONSA {
    pub cbSize: super::UINT,
    pub dwFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTER_OPTIONSW {
    pub cbSize: super::UINT,
    pub dwFlags: super::DWORD,
}
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = 2;
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = 4;
pub type PRINTER_OPTION_FLAGS = i32;
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = 1;
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = 8;
pub const PRINTER_READ: i32 = 131080;
pub const PRINTER_STATUS_BUSY: i32 = 512;
pub const PRINTER_STATUS_DOOR_OPEN: i32 = 4194304;
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: i32 = 67108864;
pub const PRINTER_STATUS_ERROR: i32 = 2;
pub const PRINTER_STATUS_INITIALIZING: i32 = 32768;
pub const PRINTER_STATUS_IO_ACTIVE: i32 = 256;
pub const PRINTER_STATUS_MANUAL_FEED: i32 = 32;
pub const PRINTER_STATUS_NOT_AVAILABLE: i32 = 4096;
pub const PRINTER_STATUS_NO_TONER: i32 = 262144;
pub const PRINTER_STATUS_OFFLINE: i32 = 128;
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: i32 = 2048;
pub const PRINTER_STATUS_OUT_OF_MEMORY: i32 = 2097152;
pub const PRINTER_STATUS_PAGE_PUNT: i32 = 524288;
pub const PRINTER_STATUS_PAPER_JAM: i32 = 8;
pub const PRINTER_STATUS_PAPER_OUT: i32 = 16;
pub const PRINTER_STATUS_PAPER_PROBLEM: i32 = 64;
pub const PRINTER_STATUS_PAUSED: i32 = 1;
pub const PRINTER_STATUS_PENDING_DELETION: i32 = 4;
pub const PRINTER_STATUS_POWER_SAVE: i32 = 16777216;
pub const PRINTER_STATUS_PRINTING: i32 = 1024;
pub const PRINTER_STATUS_PROCESSING: i32 = 16384;
pub const PRINTER_STATUS_SERVER_OFFLINE: i32 = 33554432;
pub const PRINTER_STATUS_SERVER_UNKNOWN: i32 = 8388608;
pub const PRINTER_STATUS_TONER_LOW: i32 = 131072;
pub const PRINTER_STATUS_USER_INTERVENTION: i32 = 1048576;
pub const PRINTER_STATUS_WAITING: i32 = 8192;
pub const PRINTER_STATUS_WARMING_UP: i32 = 65536;
pub const PRINTER_WRITE: i32 = 131080;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: super::DWORD,
    pub dwNupOptions: super::DWORD,
    pub dwPageOrderFlags: super::DWORD,
    pub dwNumberOfCopies: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_CAPS_2 {
    pub dwLevel: super::DWORD,
    pub dwNupOptions: super::DWORD,
    pub dwPageOrderFlags: super::DWORD,
    pub dwNumberOfCopies: super::DWORD,
    pub dwDuplexHandlingCaps: super::DWORD,
    pub dwNupDirectionCaps: super::DWORD,
    pub dwNupBorderCaps: super::DWORD,
    pub dwBookletHandlingCaps: super::DWORD,
    pub dwScalingCaps: super::DWORD,
}
pub type PRINTPROCESSOR_INFO_1 = PRINTPROCESSOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
}
pub type PRINT_EXECUTION_CONTEXT = i32;
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = 0;
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = 3;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = 2;
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = 1;
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: super::DWORD,
}
pub type PROVIDOR_INFO_1 = PROVIDOR_INFO_1A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROVIDOR_INFO_1A {
    pub pName: windows_sys::core::PSTR,
    pub pEnvironment: windows_sys::core::PSTR,
    pub pDLLName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROVIDOR_INFO_1W {
    pub pName: windows_sys::core::PWSTR,
    pub pEnvironment: windows_sys::core::PWSTR,
    pub pDLLName: windows_sys::core::PWSTR,
}
pub type PROVIDOR_INFO_2 = PROVIDOR_INFO_2A;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PrintNamedProperty {
    pub propertyName: *mut super::WCHAR,
    pub propertyValue: PrintPropertyValue,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for PrintNamedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PrintPropertiesCollection {
    pub numberOfProperties: super::ULONG,
    pub propertiesCollection: *mut PrintNamedProperty,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PrintPropertyValue {
    pub ePropertyType: EPrintPropertyType,
    pub value: PrintPropertyValue_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PrintPropertyValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union PrintPropertyValue_0 {
    pub propertyByte: super::BYTE,
    pub propertyString: windows_sys::core::PWSTR,
    pub propertyInt32: super::LONG,
    pub propertyInt64: super::LONGLONG,
    pub propertyBlob: PrintPropertyValue_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PrintPropertyValue_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: super::DWORD,
    pub pBuf: super::LPVOID,
}
pub const REVERSE_PRINT: i32 = 1;
pub const SERVER_ACCESS_ADMINISTER: i32 = 1;
pub const SERVER_ACCESS_ENUMERATE: i32 = 2;
pub const SERVER_ALL_ACCESS: i32 = 983043;
pub const SERVER_EXECUTE: i32 = 131074;
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: i32 = 0;
pub const SERVER_NOTIFY_TYPE: i32 = 2;
pub const SERVER_READ: i32 = 131074;
pub const SERVER_WRITE: i32 = 131075;
pub const SPLDS_ASSET_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("assetNumber");
pub const SPLDS_BYTES_PER_MINUTE: windows_sys::core::PCSTR = windows_sys::core::s!("bytesPerMinute");
pub const SPLDS_DESCRIPTION: windows_sys::core::PCSTR = windows_sys::core::s!("description");
pub const SPLDS_DRIVER_KEY: windows_sys::core::PCSTR = windows_sys::core::s!("DsDriver");
pub const SPLDS_DRIVER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("driverName");
pub const SPLDS_DRIVER_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("driverVersion");
pub const SPLDS_FLAGS: windows_sys::core::PCSTR = windows_sys::core::s!("flags");
pub const SPLDS_LOCATION: windows_sys::core::PCSTR = windows_sys::core::s!("location");
pub const SPLDS_PORT_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("portName");
pub const SPLDS_PRINTER_CLASS: windows_sys::core::PCSTR = windows_sys::core::s!("printQueue");
pub const SPLDS_PRINTER_LOCATIONS: windows_sys::core::PCSTR = windows_sys::core::s!("printerLocations");
pub const SPLDS_PRINTER_MODEL: windows_sys::core::PCSTR = windows_sys::core::s!("printerModel");
pub const SPLDS_PRINTER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("printerName");
pub const SPLDS_PRINTER_NAME_ALIASES: windows_sys::core::PCSTR = windows_sys::core::s!("printerNameAliases");
pub const SPLDS_PRINT_ATTRIBUTES: windows_sys::core::PCSTR = windows_sys::core::s!("printAttributes");
pub const SPLDS_PRINT_BIN_NAMES: windows_sys::core::PCSTR = windows_sys::core::s!("printBinNames");
pub const SPLDS_PRINT_COLLATE: windows_sys::core::PCSTR = windows_sys::core::s!("printCollate");
pub const SPLDS_PRINT_COLOR: windows_sys::core::PCSTR = windows_sys::core::s!("printColor");
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("printDuplexSupported");
pub const SPLDS_PRINT_END_TIME: windows_sys::core::PCSTR = windows_sys::core::s!("printEndTime");
pub const SPLDS_PRINT_IPP_COMPRESSION_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("ippCompressionSupported");
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: windows_sys::core::PCSTR = windows_sys::core::s!("printKeepPrintedJobs");
pub const SPLDS_PRINT_LANGUAGE: windows_sys::core::PCSTR = windows_sys::core::s!("printLanguage");
pub const SPLDS_PRINT_MAC_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("printMACAddress");
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("printMaxResolutionSupported");
pub const SPLDS_PRINT_MAX_X_EXTENT: windows_sys::core::PCSTR = windows_sys::core::s!("printMaxXExtent");
pub const SPLDS_PRINT_MAX_Y_EXTENT: windows_sys::core::PCSTR = windows_sys::core::s!("printMaxYExtent");
pub const SPLDS_PRINT_MEDIA_READY: windows_sys::core::PCSTR = windows_sys::core::s!("printMediaReady");
pub const SPLDS_PRINT_MEDIA_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("printMediaSupported");
pub const SPLDS_PRINT_MEMORY: windows_sys::core::PCSTR = windows_sys::core::s!("printMemory");
pub const SPLDS_PRINT_MIN_X_EXTENT: windows_sys::core::PCSTR = windows_sys::core::s!("printMinXExtent");
pub const SPLDS_PRINT_MIN_Y_EXTENT: windows_sys::core::PCSTR = windows_sys::core::s!("printMinYExtent");
pub const SPLDS_PRINT_NETWORK_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("printNetworkAddress");
pub const SPLDS_PRINT_NOTIFY: windows_sys::core::PCSTR = windows_sys::core::s!("printNotify");
pub const SPLDS_PRINT_NUMBER_UP: windows_sys::core::PCSTR = windows_sys::core::s!("printNumberUp");
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("printOrientationsSupported");
pub const SPLDS_PRINT_OWNER: windows_sys::core::PCSTR = windows_sys::core::s!("printOwner");
pub const SPLDS_PRINT_PAGES_PER_MINUTE: windows_sys::core::PCSTR = windows_sys::core::s!("printPagesPerMinute");
pub const SPLDS_PRINT_RATE: windows_sys::core::PCSTR = windows_sys::core::s!("printRate");
pub const SPLDS_PRINT_RATE_UNIT: windows_sys::core::PCSTR = windows_sys::core::s!("printRateUnit");
pub const SPLDS_PRINT_SEPARATOR_FILE: windows_sys::core::PCSTR = windows_sys::core::s!("printSeparatorFile");
pub const SPLDS_PRINT_SHARE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("printShareName");
pub const SPLDS_PRINT_SPOOLING: windows_sys::core::PCSTR = windows_sys::core::s!("printSpooling");
pub const SPLDS_PRINT_STAPLING_SUPPORTED: windows_sys::core::PCSTR = windows_sys::core::s!("printStaplingSupported");
pub const SPLDS_PRINT_START_TIME: windows_sys::core::PCSTR = windows_sys::core::s!("printStartTime");
pub const SPLDS_PRINT_STATUS: windows_sys::core::PCSTR = windows_sys::core::s!("printStatus");
pub const SPLDS_PRIORITY: windows_sys::core::PCSTR = windows_sys::core::s!("priority");
pub const SPLDS_SERVER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("serverName");
pub const SPLDS_SHORT_SERVER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("shortServerName");
pub const SPLDS_SPOOLER_KEY: windows_sys::core::PCSTR = windows_sys::core::s!("DsSpooler");
pub const SPLDS_UNC_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("uNCName");
pub const SPLDS_URL: windows_sys::core::PCSTR = windows_sys::core::s!("url");
pub const SPLDS_USER_KEY: windows_sys::core::PCSTR = windows_sys::core::s!("DsUser");
pub const SPLDS_VERSION_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("versionNumber");
pub const SPLREG_ALLOW_USER_MANAGEFORMS: windows_sys::core::PCSTR = windows_sys::core::s!("AllowUserManageForms");
pub const SPLREG_ARCHITECTURE: windows_sys::core::PCSTR = windows_sys::core::s!("Architecture");
pub const SPLREG_BEEP_ENABLED: windows_sys::core::PCSTR = windows_sys::core::s!("BeepEnabled");
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: windows_sys::core::PCSTR = windows_sys::core::s!("DefaultSpoolDirectory");
pub const SPLREG_DNS_MACHINE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("DNSMachineName");
pub const SPLREG_DS_PRESENT: windows_sys::core::PCSTR = windows_sys::core::s!("DsPresent");
pub const SPLREG_DS_PRESENT_FOR_USER: windows_sys::core::PCSTR = windows_sys::core::s!("DsPresentForUser");
pub const SPLREG_EVENT_LOG: windows_sys::core::PCSTR = windows_sys::core::s!("EventLog");
pub const SPLREG_MAJOR_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("MajorVersion");
pub const SPLREG_MINOR_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("MinorVersion");
pub const SPLREG_NET_POPUP: windows_sys::core::PCSTR = windows_sys::core::s!("NetPopup");
pub const SPLREG_NET_POPUP_TO_COMPUTER: windows_sys::core::PCSTR = windows_sys::core::s!("NetPopupToComputer");
pub const SPLREG_OS_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("OSVersion");
pub const SPLREG_OS_VERSIONEX: windows_sys::core::PCSTR = windows_sys::core::s!("OSVersionEx");
pub const SPLREG_PORT_THREAD_PRIORITY: windows_sys::core::PCSTR = windows_sys::core::s!("PortThreadPriority");
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: windows_sys::core::PCSTR = windows_sys::core::s!("PortThreadPriorityDefault");
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationExecutionPolicy");
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationGroups");
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS_SEPARATOR: u16 = 92;
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationIdleTimeout");
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationMaxobjsBeforeRecycle");
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationOverrideCompat");
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: windows_sys::core::PCSTR = windows_sys::core::s!("PrintDriverIsolationTimeBeforeRecycle");
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: windows_sys::core::PCSTR = windows_sys::core::s!("PrintQueueV4DriverDirectory");
pub const SPLREG_REMOTE_FAX: windows_sys::core::PCSTR = windows_sys::core::s!("RemoteFax");
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: windows_sys::core::PCSTR = windows_sys::core::s!("RestartJobOnPoolEnabled");
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: windows_sys::core::PCSTR = windows_sys::core::s!("RestartJobOnPoolError");
pub const SPLREG_RETRY_POPUP: windows_sys::core::PCSTR = windows_sys::core::s!("RetryPopup");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: windows_sys::core::PCSTR = windows_sys::core::s!("SchedulerThreadPriority");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: windows_sys::core::PCSTR = windows_sys::core::s!("SchedulerThreadPriorityDefault");
pub const SPLREG_WEBSHAREMGMT: windows_sys::core::PCSTR = windows_sys::core::s!("WebShareMgmt");
pub const SPOOL_FILE_PERSISTENT: i32 = 1;
pub const SPOOL_FILE_TEMPORARY: i32 = 2;
pub const STRING_LANGPAIR: i32 = 4;
pub const STRING_MUIDLL: i32 = 2;
pub const STRING_NONE: i32 = 1;
pub const UPDP_CHECK_DRIVERSTORE: i32 = 4;
pub const UPDP_SILENT_UPLOAD: i32 = 1;
pub const UPDP_UPLOAD_ALWAYS: i32 = 2;
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
