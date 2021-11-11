#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFormW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddJobW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMonitorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPortW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintDeviceObject();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProcessorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrintProvidorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2A();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnection2W();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterConnectionW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterDriverW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn AdvancedDocumentPropertiesW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendPrinterNotifyInfoData();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallRouterFindFirstPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseSpoolFileHandle();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitSpoolData();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommonPropertySheetUIW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConfigurePortW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConnectToPrinterDlg();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CorePrinterDriverInstalledW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrintAsyncNotifyChannel();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreatePrinterIC();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFormW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteJobNamedProperty();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMonitorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePortW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProcessorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrintProvidorW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterConnectionW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDataW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverPackageA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverPackageW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterDriverW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterIC();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePrinterKeyW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrint();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DevQueryPrintEx();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DocumentPropertiesW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDocPrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPagePrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFormsW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobNamedProperties();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumJobsW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMonitorsW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPortsW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorDatatypesW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintProcessorsW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDataW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterDriversW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrinterKeyW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPrintersW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ExtDeviceMode();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClosePrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushPrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrintNamedPropertyArray();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrintPropertyValue();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePrinterNotifyInfo();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiDeleteSpoolFileHandle();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndDocEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiEndPageEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDC();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetDevmodeForPage();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageCount();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGetPageHandle();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiGetSpoolFileHandle();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiPlayPageEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GdiResetDCEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn GdiStartDocEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiStartPageEMF();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateCopyFilePaths();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPSUIUserData();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCorePrinterDriversW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFormW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributes();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetJobAttributesEx();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobNamedPropertyValue();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintExecutionData();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintOutputInfo();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrintProcessorDirectoryW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDataW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2A();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriver2W();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverDirectoryW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverPackagePathA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverPackagePathW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterDriverW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSpoolFileHandle();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonatePrinterClient();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPrinterDriverFromPackageA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPrinterDriverFromPackageW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn IsValidDevmodeW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2A();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinter2W();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OpenPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PartialReplyPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayGdiScriptOnPrinterIC();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterMessageBoxW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrinterProperties();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindClosePrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProvidorFindFirstPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadPrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForPrintAsyncNotifications();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePrintDeviceObject();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyPrinterChangeNotificationEx();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportJobProcessingProgress();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ResetPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToPrinterSelf();
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterAllocBidiMem();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAllocBidiResponseContainer();
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterAllocPrinterNotifyInfo();
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn RouterFreeBidiMem();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreeBidiResponseContainer();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterFreePrinterNotifyInfo();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScheduleJob();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCPSUIUserData();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFormW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobNamedProperty();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPortW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataExW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterDataW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplIsSessionZero();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SplPromptUIInUsersSession();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerCopyFileEvent();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindClosePrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindFirstPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerFindNextPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`*"]
    pub fn SpoolerFreePrinterNotifyInfo();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SpoolerRefreshPrinterChangeNotification();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartDocPrinterW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartPagePrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnRegisterForPrintAsyncNotifications();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePrintDeviceObject();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageA();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UploadPrinterDriverPackageW();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForPrinterChange();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrinter();
    #[doc = "*Required features: `Win32_Graphics_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XcvDataW();
}
