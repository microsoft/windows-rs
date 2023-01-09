impl ::core::default::Default for ADDJOB_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADDJOB_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.Path == other.Path && self.JobId == other.JobId
    }
}
impl ::core::cmp::Eq for ADDJOB_INFO_1A {}
impl ::core::fmt::Debug for ADDJOB_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDJOB_INFO_1A").field("Path", &self.Path).field("JobId", &self.JobId).finish()
    }
}
impl ::core::default::Default for ADDJOB_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADDJOB_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.Path == other.Path && self.JobId == other.JobId
    }
}
impl ::core::cmp::Eq for ADDJOB_INFO_1W {}
impl ::core::fmt::Debug for ADDJOB_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDJOB_INFO_1W").field("Path", &self.Path).field("JobId", &self.JobId).finish()
    }
}
impl ::core::default::Default for ATTRIBUTE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATTRIBUTE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwJobNumberOfPagesPerSide == other.dwJobNumberOfPagesPerSide && self.dwDrvNumberOfPagesPerSide == other.dwDrvNumberOfPagesPerSide && self.dwNupBorderFlags == other.dwNupBorderFlags && self.dwJobPageOrderFlags == other.dwJobPageOrderFlags && self.dwDrvPageOrderFlags == other.dwDrvPageOrderFlags && self.dwJobNumberOfCopies == other.dwJobNumberOfCopies && self.dwDrvNumberOfCopies == other.dwDrvNumberOfCopies
    }
}
impl ::core::cmp::Eq for ATTRIBUTE_INFO_1 {}
impl ::core::fmt::Debug for ATTRIBUTE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTRIBUTE_INFO_1").field("dwJobNumberOfPagesPerSide", &self.dwJobNumberOfPagesPerSide).field("dwDrvNumberOfPagesPerSide", &self.dwDrvNumberOfPagesPerSide).field("dwNupBorderFlags", &self.dwNupBorderFlags).field("dwJobPageOrderFlags", &self.dwJobPageOrderFlags).field("dwDrvPageOrderFlags", &self.dwDrvPageOrderFlags).field("dwJobNumberOfCopies", &self.dwJobNumberOfCopies).field("dwDrvNumberOfCopies", &self.dwDrvNumberOfCopies).finish()
    }
}
impl ::core::default::Default for ATTRIBUTE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATTRIBUTE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwJobNumberOfPagesPerSide == other.dwJobNumberOfPagesPerSide && self.dwDrvNumberOfPagesPerSide == other.dwDrvNumberOfPagesPerSide && self.dwNupBorderFlags == other.dwNupBorderFlags && self.dwJobPageOrderFlags == other.dwJobPageOrderFlags && self.dwDrvPageOrderFlags == other.dwDrvPageOrderFlags && self.dwJobNumberOfCopies == other.dwJobNumberOfCopies && self.dwDrvNumberOfCopies == other.dwDrvNumberOfCopies && self.dwColorOptimization == other.dwColorOptimization
    }
}
impl ::core::cmp::Eq for ATTRIBUTE_INFO_2 {}
impl ::core::fmt::Debug for ATTRIBUTE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTRIBUTE_INFO_2")
            .field("dwJobNumberOfPagesPerSide", &self.dwJobNumberOfPagesPerSide)
            .field("dwDrvNumberOfPagesPerSide", &self.dwDrvNumberOfPagesPerSide)
            .field("dwNupBorderFlags", &self.dwNupBorderFlags)
            .field("dwJobPageOrderFlags", &self.dwJobPageOrderFlags)
            .field("dwDrvPageOrderFlags", &self.dwDrvPageOrderFlags)
            .field("dwJobNumberOfCopies", &self.dwJobNumberOfCopies)
            .field("dwDrvNumberOfCopies", &self.dwDrvNumberOfCopies)
            .field("dwColorOptimization", &self.dwColorOptimization)
            .finish()
    }
}
impl ::core::default::Default for ATTRIBUTE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATTRIBUTE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwJobNumberOfPagesPerSide == other.dwJobNumberOfPagesPerSide && self.dwDrvNumberOfPagesPerSide == other.dwDrvNumberOfPagesPerSide && self.dwNupBorderFlags == other.dwNupBorderFlags && self.dwJobPageOrderFlags == other.dwJobPageOrderFlags && self.dwDrvPageOrderFlags == other.dwDrvPageOrderFlags && self.dwJobNumberOfCopies == other.dwJobNumberOfCopies && self.dwDrvNumberOfCopies == other.dwDrvNumberOfCopies && self.dwColorOptimization == other.dwColorOptimization && self.dmPrintQuality == other.dmPrintQuality && self.dmYResolution == other.dmYResolution
    }
}
impl ::core::cmp::Eq for ATTRIBUTE_INFO_3 {}
impl ::core::fmt::Debug for ATTRIBUTE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTRIBUTE_INFO_3")
            .field("dwJobNumberOfPagesPerSide", &self.dwJobNumberOfPagesPerSide)
            .field("dwDrvNumberOfPagesPerSide", &self.dwDrvNumberOfPagesPerSide)
            .field("dwNupBorderFlags", &self.dwNupBorderFlags)
            .field("dwJobPageOrderFlags", &self.dwJobPageOrderFlags)
            .field("dwDrvPageOrderFlags", &self.dwDrvPageOrderFlags)
            .field("dwJobNumberOfCopies", &self.dwJobNumberOfCopies)
            .field("dwDrvNumberOfCopies", &self.dwDrvNumberOfCopies)
            .field("dwColorOptimization", &self.dwColorOptimization)
            .field("dmPrintQuality", &self.dmPrintQuality)
            .field("dmYResolution", &self.dmYResolution)
            .finish()
    }
}
impl ::core::default::Default for ATTRIBUTE_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATTRIBUTE_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwJobNumberOfPagesPerSide == other.dwJobNumberOfPagesPerSide
            && self.dwDrvNumberOfPagesPerSide == other.dwDrvNumberOfPagesPerSide
            && self.dwNupBorderFlags == other.dwNupBorderFlags
            && self.dwJobPageOrderFlags == other.dwJobPageOrderFlags
            && self.dwDrvPageOrderFlags == other.dwDrvPageOrderFlags
            && self.dwJobNumberOfCopies == other.dwJobNumberOfCopies
            && self.dwDrvNumberOfCopies == other.dwDrvNumberOfCopies
            && self.dwColorOptimization == other.dwColorOptimization
            && self.dmPrintQuality == other.dmPrintQuality
            && self.dmYResolution == other.dmYResolution
            && self.dwDuplexFlags == other.dwDuplexFlags
            && self.dwNupDirection == other.dwNupDirection
            && self.dwBookletFlags == other.dwBookletFlags
            && self.dwScalingPercentX == other.dwScalingPercentX
            && self.dwScalingPercentY == other.dwScalingPercentY
    }
}
impl ::core::cmp::Eq for ATTRIBUTE_INFO_4 {}
impl ::core::fmt::Debug for ATTRIBUTE_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTRIBUTE_INFO_4")
            .field("dwJobNumberOfPagesPerSide", &self.dwJobNumberOfPagesPerSide)
            .field("dwDrvNumberOfPagesPerSide", &self.dwDrvNumberOfPagesPerSide)
            .field("dwNupBorderFlags", &self.dwNupBorderFlags)
            .field("dwJobPageOrderFlags", &self.dwJobPageOrderFlags)
            .field("dwDrvPageOrderFlags", &self.dwDrvPageOrderFlags)
            .field("dwJobNumberOfCopies", &self.dwJobNumberOfCopies)
            .field("dwDrvNumberOfCopies", &self.dwDrvNumberOfCopies)
            .field("dwColorOptimization", &self.dwColorOptimization)
            .field("dmPrintQuality", &self.dmPrintQuality)
            .field("dmYResolution", &self.dmYResolution)
            .field("dwDuplexFlags", &self.dwDuplexFlags)
            .field("dwNupDirection", &self.dwNupDirection)
            .field("dwBookletFlags", &self.dwBookletFlags)
            .field("dwScalingPercentX", &self.dwScalingPercentX)
            .field("dwScalingPercentY", &self.dwScalingPercentY)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIDI_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIDI_REQUEST_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIDI_REQUEST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIDI_RESPONSE_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIDI_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BIDI_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BIDI_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIDI_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINARY_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BINARY_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.cbBuf == other.cbBuf && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for BINARY_CONTAINER {}
impl ::core::fmt::Debug for BINARY_CONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINARY_CONTAINER").field("cbBuf", &self.cbBuf).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for BranchOfficeJobData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BranchOfficeJobDataContainer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BranchOfficeJobDataError {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BranchOfficeJobDataError {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError && self.pDocumentName == other.pDocumentName && self.pUserName == other.pUserName && self.pPrinterName == other.pPrinterName && self.pDataType == other.pDataType && self.TotalSize == other.TotalSize && self.PrintedSize == other.PrintedSize && self.TotalPages == other.TotalPages && self.PrintedPages == other.PrintedPages && self.pMachineName == other.pMachineName && self.pJobError == other.pJobError && self.pErrorDescription == other.pErrorDescription
    }
}
impl ::core::cmp::Eq for BranchOfficeJobDataError {}
impl ::core::fmt::Debug for BranchOfficeJobDataError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BranchOfficeJobDataError")
            .field("LastError", &self.LastError)
            .field("pDocumentName", &self.pDocumentName)
            .field("pUserName", &self.pUserName)
            .field("pPrinterName", &self.pPrinterName)
            .field("pDataType", &self.pDataType)
            .field("TotalSize", &self.TotalSize)
            .field("PrintedSize", &self.PrintedSize)
            .field("TotalPages", &self.TotalPages)
            .field("PrintedPages", &self.PrintedPages)
            .field("pMachineName", &self.pMachineName)
            .field("pJobError", &self.pJobError)
            .field("pErrorDescription", &self.pErrorDescription)
            .finish()
    }
}
impl ::core::default::Default for BranchOfficeJobDataPipelineFailed {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BranchOfficeJobDataPipelineFailed {
    fn eq(&self, other: &Self) -> bool {
        self.pDocumentName == other.pDocumentName && self.pPrinterName == other.pPrinterName && self.pExtraErrorInfo == other.pExtraErrorInfo
    }
}
impl ::core::cmp::Eq for BranchOfficeJobDataPipelineFailed {}
impl ::core::fmt::Debug for BranchOfficeJobDataPipelineFailed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BranchOfficeJobDataPipelineFailed").field("pDocumentName", &self.pDocumentName).field("pPrinterName", &self.pPrinterName).field("pExtraErrorInfo", &self.pExtraErrorInfo).finish()
    }
}
impl ::core::default::Default for BranchOfficeJobDataPrinted {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BranchOfficeJobDataPrinted {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.pDocumentName == other.pDocumentName && self.pUserName == other.pUserName && self.pMachineName == other.pMachineName && self.pPrinterName == other.pPrinterName && self.pPortName == other.pPortName && self.Size == other.Size && self.TotalPages == other.TotalPages
    }
}
impl ::core::cmp::Eq for BranchOfficeJobDataPrinted {}
impl ::core::fmt::Debug for BranchOfficeJobDataPrinted {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BranchOfficeJobDataPrinted").field("Status", &self.Status).field("pDocumentName", &self.pDocumentName).field("pUserName", &self.pUserName).field("pMachineName", &self.pMachineName).field("pPrinterName", &self.pPrinterName).field("pPortName", &self.pPortName).field("Size", &self.Size).field("TotalPages", &self.TotalPages).finish()
    }
}
impl ::core::default::Default for BranchOfficeJobDataRendered {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BranchOfficeJobDataRendered {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ICMMethod == other.ICMMethod && self.Color == other.Color && self.PrintQuality == other.PrintQuality && self.YResolution == other.YResolution && self.Copies == other.Copies && self.TTOption == other.TTOption
    }
}
impl ::core::cmp::Eq for BranchOfficeJobDataRendered {}
impl ::core::fmt::Debug for BranchOfficeJobDataRendered {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BranchOfficeJobDataRendered").field("Size", &self.Size).field("ICMMethod", &self.ICMMethod).field("Color", &self.Color).field("PrintQuality", &self.PrintQuality).field("YResolution", &self.YResolution).field("Copies", &self.Copies).field("TTOption", &self.TTOption).finish()
    }
}
impl ::core::default::Default for BranchOfficeLogOfflineFileFull {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BranchOfficeLogOfflineFileFull {
    fn eq(&self, other: &Self) -> bool {
        self.pMachineName == other.pMachineName
    }
}
impl ::core::cmp::Eq for BranchOfficeLogOfflineFileFull {}
impl ::core::fmt::Debug for BranchOfficeLogOfflineFileFull {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BranchOfficeLogOfflineFileFull").field("pMachineName", &self.pMachineName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for COMPROPSHEETUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CONFIG_INFO_DATA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIG_INFO_DATA_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.dwVersion == other.dwVersion
    }
}
impl ::core::cmp::Eq for CONFIG_INFO_DATA_1 {}
impl ::core::fmt::Debug for CONFIG_INFO_DATA_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_INFO_DATA_1").field("Reserved", &self.Reserved).field("dwVersion", &self.dwVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CORE_PRINTER_DRIVERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CORE_PRINTER_DRIVERA {
    fn eq(&self, other: &Self) -> bool {
        self.CoreDriverGUID == other.CoreDriverGUID && self.ftDriverDate == other.ftDriverDate && self.dwlDriverVersion == other.dwlDriverVersion && self.szPackageID == other.szPackageID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CORE_PRINTER_DRIVERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CORE_PRINTER_DRIVERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORE_PRINTER_DRIVERA").field("CoreDriverGUID", &self.CoreDriverGUID).field("ftDriverDate", &self.ftDriverDate).field("dwlDriverVersion", &self.dwlDriverVersion).field("szPackageID", &self.szPackageID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CORE_PRINTER_DRIVERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CORE_PRINTER_DRIVERW {
    fn eq(&self, other: &Self) -> bool {
        self.CoreDriverGUID == other.CoreDriverGUID && self.ftDriverDate == other.ftDriverDate && self.dwlDriverVersion == other.dwlDriverVersion && self.szPackageID == other.szPackageID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CORE_PRINTER_DRIVERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CORE_PRINTER_DRIVERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORE_PRINTER_DRIVERW").field("CoreDriverGUID", &self.CoreDriverGUID).field("ftDriverDate", &self.ftDriverDate).field("dwlDriverVersion", &self.dwlDriverVersion).field("szPackageID", &self.szPackageID).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CPSUICBPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CPSUIDATABLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CPSUIDATABLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for CPSUIDATABLOCK {}
impl ::core::fmt::Debug for CPSUIDATABLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPSUIDATABLOCK").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for CUSTOMSIZEPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CUSTOMSIZEPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.dwOrder == other.dwOrder && self.lMinVal == other.lMinVal && self.lMaxVal == other.lMaxVal
    }
}
impl ::core::cmp::Eq for CUSTOMSIZEPARAM {}
impl ::core::fmt::Debug for CUSTOMSIZEPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTOMSIZEPARAM").field("dwOrder", &self.dwOrder).field("lMinVal", &self.lMinVal).field("lMaxVal", &self.lMaxVal).finish()
    }
}
impl ::core::default::Default for DATATYPES_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATATYPES_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DATATYPES_INFO_1A {}
impl ::core::fmt::Debug for DATATYPES_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATATYPES_INFO_1A").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DATATYPES_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATATYPES_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DATATYPES_INFO_1W {}
impl ::core::fmt::Debug for DATATYPES_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATATYPES_INFO_1W").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature && self.wSize == other.wSize && self.wDataID == other.wDataID && self.dwDataSize == other.dwDataSize && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DATA_HEADER {}
impl ::core::fmt::Debug for DATA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATA_HEADER").field("dwSignature", &self.dwSignature).field("wSize", &self.wSize).field("wDataID", &self.wDataID).field("dwDataSize", &self.dwDataSize).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for DELETE_PORT_DATA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DELETE_PORT_DATA_1 {
    fn eq(&self, other: &Self) -> bool {
        self.psztPortName == other.psztPortName && self.Reserved == other.Reserved && self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DELETE_PORT_DATA_1 {}
impl ::core::fmt::Debug for DELETE_PORT_DATA_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELETE_PORT_DATA_1").field("psztPortName", &self.psztPortName).field("Reserved", &self.Reserved).field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEPROPERTYHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEPROPERTYHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.hPrinter == other.hPrinter && self.pszPrinterName == other.pszPrinterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEPROPERTYHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICEPROPERTYHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEPROPERTYHEADER").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("hPrinter", &self.hPrinter).field("pszPrinterName", &self.pszPrinterName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DEVQUERYPRINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DEVQUERYPRINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Level == other.Level && self.hPrinter == other.hPrinter && self.pDevMode == other.pDevMode && self.pszErrorStr == other.pszErrorStr && self.cchErrorStr == other.cchErrorStr && self.cchNeeded == other.cchNeeded
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DEVQUERYPRINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DEVQUERYPRINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVQUERYPRINT_INFO").field("cbSize", &self.cbSize).field("Level", &self.Level).field("hPrinter", &self.hPrinter).field("pDevMode", &self.pDevMode).field("pszErrorStr", &self.pszErrorStr).field("cchErrorStr", &self.cchErrorStr).field("cchNeeded", &self.cchNeeded).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for DLGPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DOCEVENT_CREATEDCPRE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DOCEVENT_CREATEDCPRE {
    fn eq(&self, other: &Self) -> bool {
        self.pszDriver == other.pszDriver && self.pszDevice == other.pszDevice && self.pdm == other.pdm && self.bIC == other.bIC
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DOCEVENT_CREATEDCPRE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DOCEVENT_CREATEDCPRE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCEVENT_CREATEDCPRE").field("pszDriver", &self.pszDriver).field("pszDevice", &self.pszDevice).field("pdm", &self.pdm).field("bIC", &self.bIC).finish()
    }
}
impl ::core::default::Default for DOCEVENT_ESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOCEVENT_ESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.iEscape == other.iEscape && self.cjInput == other.cjInput && self.pvInData == other.pvInData
    }
}
impl ::core::cmp::Eq for DOCEVENT_ESCAPE {}
impl ::core::fmt::Debug for DOCEVENT_ESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCEVENT_ESCAPE").field("iEscape", &self.iEscape).field("cjInput", &self.cjInput).field("pvInData", &self.pvInData).finish()
    }
}
impl ::core::default::Default for DOCEVENT_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOCEVENT_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cElementsAllocated == other.cElementsAllocated && self.cElementsNeeded == other.cElementsNeeded && self.cElementsReturned == other.cElementsReturned && self.aDocEventCall == other.aDocEventCall
    }
}
impl ::core::cmp::Eq for DOCEVENT_FILTER {}
impl ::core::fmt::Debug for DOCEVENT_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCEVENT_FILTER").field("cbSize", &self.cbSize).field("cElementsAllocated", &self.cElementsAllocated).field("cElementsNeeded", &self.cElementsNeeded).field("cElementsReturned", &self.cElementsReturned).field("aDocEventCall", &self.aDocEventCall).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DOCUMENTPROPERTYHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DOCUMENTPROPERTYHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Reserved == other.Reserved && self.hPrinter == other.hPrinter && self.pszPrinterName == other.pszPrinterName && self.pdmIn == other.pdmIn && self.pdmOut == other.pdmOut && self.cbOut == other.cbOut && self.fMode == other.fMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DOCUMENTPROPERTYHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DOCUMENTPROPERTYHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCUMENTPROPERTYHEADER").field("cbSize", &self.cbSize).field("Reserved", &self.Reserved).field("hPrinter", &self.hPrinter).field("pszPrinterName", &self.pszPrinterName).field("pdmIn", &self.pdmIn).field("pdmOut", &self.pdmOut).field("cbOut", &self.cbOut).field("fMode", &self.fMode).finish()
    }
}
impl ::core::default::Default for DOC_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype
    }
}
impl ::core::cmp::Eq for DOC_INFO_1A {}
impl ::core::fmt::Debug for DOC_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_1A").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).finish()
    }
}
impl ::core::default::Default for DOC_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype
    }
}
impl ::core::cmp::Eq for DOC_INFO_1W {}
impl ::core::fmt::Debug for DOC_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_1W").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).finish()
    }
}
impl ::core::default::Default for DOC_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype && self.dwMode == other.dwMode && self.JobId == other.JobId
    }
}
impl ::core::cmp::Eq for DOC_INFO_2A {}
impl ::core::fmt::Debug for DOC_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_2A").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).field("dwMode", &self.dwMode).field("JobId", &self.JobId).finish()
    }
}
impl ::core::default::Default for DOC_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype && self.dwMode == other.dwMode && self.JobId == other.JobId
    }
}
impl ::core::cmp::Eq for DOC_INFO_2W {}
impl ::core::fmt::Debug for DOC_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_2W").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).field("dwMode", &self.dwMode).field("JobId", &self.JobId).finish()
    }
}
impl ::core::default::Default for DOC_INFO_3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_3A {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DOC_INFO_3A {}
impl ::core::fmt::Debug for DOC_INFO_3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_3A").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DOC_INFO_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOC_INFO_3W {
    fn eq(&self, other: &Self) -> bool {
        self.pDocName == other.pDocName && self.pOutputFile == other.pOutputFile && self.pDatatype == other.pDatatype && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DOC_INFO_3W {}
impl ::core::fmt::Debug for DOC_INFO_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOC_INFO_3W").field("pDocName", &self.pDocName).field("pOutputFile", &self.pOutputFile).field("pDatatype", &self.pDatatype).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_1A {}
impl ::core::fmt::Debug for DRIVER_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_1A").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_1W {}
impl ::core::fmt::Debug for DRIVER_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_1W").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_2A {}
impl ::core::fmt::Debug for DRIVER_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_2A").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_2W {}
impl ::core::fmt::Debug for DRIVER_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_2W").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_3A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_3A {}
impl ::core::fmt::Debug for DRIVER_INFO_3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_3A").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).field("pHelpFile", &self.pHelpFile).field("pDependentFiles", &self.pDependentFiles).field("pMonitorName", &self.pMonitorName).field("pDefaultDataType", &self.pDefaultDataType).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_3W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_3W {}
impl ::core::fmt::Debug for DRIVER_INFO_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_3W").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).field("pHelpFile", &self.pHelpFile).field("pDependentFiles", &self.pDependentFiles).field("pMonitorName", &self.pMonitorName).field("pDefaultDataType", &self.pDefaultDataType).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_4A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType && self.pszzPreviousNames == other.pszzPreviousNames
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_4A {}
impl ::core::fmt::Debug for DRIVER_INFO_4A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_4A")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_4W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType && self.pszzPreviousNames == other.pszzPreviousNames
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_4W {}
impl ::core::fmt::Debug for DRIVER_INFO_4W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_4W")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_5A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_5A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.dwDriverAttributes == other.dwDriverAttributes && self.dwConfigVersion == other.dwConfigVersion && self.dwDriverVersion == other.dwDriverVersion
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_5A {}
impl ::core::fmt::Debug for DRIVER_INFO_5A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_5A").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).field("dwDriverAttributes", &self.dwDriverAttributes).field("dwConfigVersion", &self.dwConfigVersion).field("dwDriverVersion", &self.dwDriverVersion).finish()
    }
}
impl ::core::default::Default for DRIVER_INFO_5W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_INFO_5W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.dwDriverAttributes == other.dwDriverAttributes && self.dwConfigVersion == other.dwConfigVersion && self.dwDriverVersion == other.dwDriverVersion
    }
}
impl ::core::cmp::Eq for DRIVER_INFO_5W {}
impl ::core::fmt::Debug for DRIVER_INFO_5W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_5W").field("cVersion", &self.cVersion).field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDriverPath", &self.pDriverPath).field("pDataFile", &self.pDataFile).field("pConfigFile", &self.pConfigFile).field("dwDriverAttributes", &self.dwDriverAttributes).field("dwConfigVersion", &self.dwConfigVersion).field("dwDriverVersion", &self.dwDriverVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVER_INFO_6A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVER_INFO_6A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType && self.pszzPreviousNames == other.pszzPreviousNames && self.ftDriverDate == other.ftDriverDate && self.dwlDriverVersion == other.dwlDriverVersion && self.pszMfgName == other.pszMfgName && self.pszOEMUrl == other.pszOEMUrl && self.pszHardwareID == other.pszHardwareID && self.pszProvider == other.pszProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVER_INFO_6A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVER_INFO_6A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_6A")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .field("ftDriverDate", &self.ftDriverDate)
            .field("dwlDriverVersion", &self.dwlDriverVersion)
            .field("pszMfgName", &self.pszMfgName)
            .field("pszOEMUrl", &self.pszOEMUrl)
            .field("pszHardwareID", &self.pszHardwareID)
            .field("pszProvider", &self.pszProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVER_INFO_6W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVER_INFO_6W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType && self.pszzPreviousNames == other.pszzPreviousNames && self.ftDriverDate == other.ftDriverDate && self.dwlDriverVersion == other.dwlDriverVersion && self.pszMfgName == other.pszMfgName && self.pszOEMUrl == other.pszOEMUrl && self.pszHardwareID == other.pszHardwareID && self.pszProvider == other.pszProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVER_INFO_6W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVER_INFO_6W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_6W")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .field("ftDriverDate", &self.ftDriverDate)
            .field("dwlDriverVersion", &self.dwlDriverVersion)
            .field("pszMfgName", &self.pszMfgName)
            .field("pszOEMUrl", &self.pszOEMUrl)
            .field("pszHardwareID", &self.pszHardwareID)
            .field("pszProvider", &self.pszProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVER_INFO_8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVER_INFO_8A {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion
            && self.pName == other.pName
            && self.pEnvironment == other.pEnvironment
            && self.pDriverPath == other.pDriverPath
            && self.pDataFile == other.pDataFile
            && self.pConfigFile == other.pConfigFile
            && self.pHelpFile == other.pHelpFile
            && self.pDependentFiles == other.pDependentFiles
            && self.pMonitorName == other.pMonitorName
            && self.pDefaultDataType == other.pDefaultDataType
            && self.pszzPreviousNames == other.pszzPreviousNames
            && self.ftDriverDate == other.ftDriverDate
            && self.dwlDriverVersion == other.dwlDriverVersion
            && self.pszMfgName == other.pszMfgName
            && self.pszOEMUrl == other.pszOEMUrl
            && self.pszHardwareID == other.pszHardwareID
            && self.pszProvider == other.pszProvider
            && self.pszPrintProcessor == other.pszPrintProcessor
            && self.pszVendorSetup == other.pszVendorSetup
            && self.pszzColorProfiles == other.pszzColorProfiles
            && self.pszInfPath == other.pszInfPath
            && self.dwPrinterDriverAttributes == other.dwPrinterDriverAttributes
            && self.pszzCoreDriverDependencies == other.pszzCoreDriverDependencies
            && self.ftMinInboxDriverVerDate == other.ftMinInboxDriverVerDate
            && self.dwlMinInboxDriverVerVersion == other.dwlMinInboxDriverVerVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVER_INFO_8A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVER_INFO_8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_8A")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .field("ftDriverDate", &self.ftDriverDate)
            .field("dwlDriverVersion", &self.dwlDriverVersion)
            .field("pszMfgName", &self.pszMfgName)
            .field("pszOEMUrl", &self.pszOEMUrl)
            .field("pszHardwareID", &self.pszHardwareID)
            .field("pszProvider", &self.pszProvider)
            .field("pszPrintProcessor", &self.pszPrintProcessor)
            .field("pszVendorSetup", &self.pszVendorSetup)
            .field("pszzColorProfiles", &self.pszzColorProfiles)
            .field("pszInfPath", &self.pszInfPath)
            .field("dwPrinterDriverAttributes", &self.dwPrinterDriverAttributes)
            .field("pszzCoreDriverDependencies", &self.pszzCoreDriverDependencies)
            .field("ftMinInboxDriverVerDate", &self.ftMinInboxDriverVerDate)
            .field("dwlMinInboxDriverVerVersion", &self.dwlMinInboxDriverVerVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVER_INFO_8W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVER_INFO_8W {
    fn eq(&self, other: &Self) -> bool {
        self.cVersion == other.cVersion
            && self.pName == other.pName
            && self.pEnvironment == other.pEnvironment
            && self.pDriverPath == other.pDriverPath
            && self.pDataFile == other.pDataFile
            && self.pConfigFile == other.pConfigFile
            && self.pHelpFile == other.pHelpFile
            && self.pDependentFiles == other.pDependentFiles
            && self.pMonitorName == other.pMonitorName
            && self.pDefaultDataType == other.pDefaultDataType
            && self.pszzPreviousNames == other.pszzPreviousNames
            && self.ftDriverDate == other.ftDriverDate
            && self.dwlDriverVersion == other.dwlDriverVersion
            && self.pszMfgName == other.pszMfgName
            && self.pszOEMUrl == other.pszOEMUrl
            && self.pszHardwareID == other.pszHardwareID
            && self.pszProvider == other.pszProvider
            && self.pszPrintProcessor == other.pszPrintProcessor
            && self.pszVendorSetup == other.pszVendorSetup
            && self.pszzColorProfiles == other.pszzColorProfiles
            && self.pszInfPath == other.pszInfPath
            && self.dwPrinterDriverAttributes == other.dwPrinterDriverAttributes
            && self.pszzCoreDriverDependencies == other.pszzCoreDriverDependencies
            && self.ftMinInboxDriverVerDate == other.ftMinInboxDriverVerDate
            && self.dwlMinInboxDriverVerVersion == other.dwlMinInboxDriverVerVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVER_INFO_8W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVER_INFO_8W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_INFO_8W")
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .field("ftDriverDate", &self.ftDriverDate)
            .field("dwlDriverVersion", &self.dwlDriverVersion)
            .field("pszMfgName", &self.pszMfgName)
            .field("pszOEMUrl", &self.pszOEMUrl)
            .field("pszHardwareID", &self.pszHardwareID)
            .field("pszProvider", &self.pszProvider)
            .field("pszPrintProcessor", &self.pszPrintProcessor)
            .field("pszVendorSetup", &self.pszVendorSetup)
            .field("pszzColorProfiles", &self.pszzColorProfiles)
            .field("pszInfPath", &self.pszInfPath)
            .field("dwPrinterDriverAttributes", &self.dwPrinterDriverAttributes)
            .field("pszzCoreDriverDependencies", &self.pszzCoreDriverDependencies)
            .field("ftMinInboxDriverVerDate", &self.ftMinInboxDriverVerDate)
            .field("dwlMinInboxDriverVerVersion", &self.dwlMinInboxDriverVerVersion)
            .finish()
    }
}
impl ::core::default::Default for DRIVER_UPGRADE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_UPGRADE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pOldDriverDirectory == other.pOldDriverDirectory
    }
}
impl ::core::cmp::Eq for DRIVER_UPGRADE_INFO_1 {}
impl ::core::fmt::Debug for DRIVER_UPGRADE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_UPGRADE_INFO_1").field("pPrinterName", &self.pPrinterName).field("pOldDriverDirectory", &self.pOldDriverDirectory).finish()
    }
}
impl ::core::default::Default for DRIVER_UPGRADE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_UPGRADE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pOldDriverDirectory == other.pOldDriverDirectory && self.cVersion == other.cVersion && self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDriverPath == other.pDriverPath && self.pDataFile == other.pDataFile && self.pConfigFile == other.pConfigFile && self.pHelpFile == other.pHelpFile && self.pDependentFiles == other.pDependentFiles && self.pMonitorName == other.pMonitorName && self.pDefaultDataType == other.pDefaultDataType && self.pszzPreviousNames == other.pszzPreviousNames
    }
}
impl ::core::cmp::Eq for DRIVER_UPGRADE_INFO_2 {}
impl ::core::fmt::Debug for DRIVER_UPGRADE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_UPGRADE_INFO_2")
            .field("pPrinterName", &self.pPrinterName)
            .field("pOldDriverDirectory", &self.pOldDriverDirectory)
            .field("cVersion", &self.cVersion)
            .field("pName", &self.pName)
            .field("pEnvironment", &self.pEnvironment)
            .field("pDriverPath", &self.pDriverPath)
            .field("pDataFile", &self.pDataFile)
            .field("pConfigFile", &self.pConfigFile)
            .field("pHelpFile", &self.pHelpFile)
            .field("pDependentFiles", &self.pDependentFiles)
            .field("pMonitorName", &self.pMonitorName)
            .field("pDefaultDataType", &self.pDefaultDataType)
            .field("pszzPreviousNames", &self.pszzPreviousNames)
            .finish()
    }
}
impl ::core::default::Default for EATTRIBUTE_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EATTRIBUTE_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EATTRIBUTE_DATATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EBranchOfficeJobEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EBranchOfficeJobEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EBranchOfficeJobEventType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPrintPropertyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPrintPropertyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintPropertyType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPrintXPSJobOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPrintXPSJobOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintXPSJobOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPrintXPSJobProgress {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPrintXPSJobProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintXPSJobProgress").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXTCHKBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTCHKBOX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.pTitle == other.pTitle && self.pSeparator == other.pSeparator && self.pCheckedName == other.pCheckedName && self.IconID == other.IconID && self.wReserved == other.wReserved && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for EXTCHKBOX {}
impl ::core::fmt::Debug for EXTCHKBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTCHKBOX").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("pTitle", &self.pTitle).field("pSeparator", &self.pSeparator).field("pCheckedName", &self.pCheckedName).field("IconID", &self.IconID).field("wReserved", &self.wReserved).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for EXTPUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EXTTEXTMETRIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTTEXTMETRIC {
    fn eq(&self, other: &Self) -> bool {
        self.emSize == other.emSize
            && self.emPointSize == other.emPointSize
            && self.emOrientation == other.emOrientation
            && self.emMasterHeight == other.emMasterHeight
            && self.emMinScale == other.emMinScale
            && self.emMaxScale == other.emMaxScale
            && self.emMasterUnits == other.emMasterUnits
            && self.emCapHeight == other.emCapHeight
            && self.emXHeight == other.emXHeight
            && self.emLowerCaseAscent == other.emLowerCaseAscent
            && self.emLowerCaseDescent == other.emLowerCaseDescent
            && self.emSlant == other.emSlant
            && self.emSuperScript == other.emSuperScript
            && self.emSubScript == other.emSubScript
            && self.emSuperScriptSize == other.emSuperScriptSize
            && self.emSubScriptSize == other.emSubScriptSize
            && self.emUnderlineOffset == other.emUnderlineOffset
            && self.emUnderlineWidth == other.emUnderlineWidth
            && self.emDoubleUpperUnderlineOffset == other.emDoubleUpperUnderlineOffset
            && self.emDoubleLowerUnderlineOffset == other.emDoubleLowerUnderlineOffset
            && self.emDoubleUpperUnderlineWidth == other.emDoubleUpperUnderlineWidth
            && self.emDoubleLowerUnderlineWidth == other.emDoubleLowerUnderlineWidth
            && self.emStrikeOutOffset == other.emStrikeOutOffset
            && self.emStrikeOutWidth == other.emStrikeOutWidth
            && self.emKernPairs == other.emKernPairs
            && self.emKernTracks == other.emKernTracks
    }
}
impl ::core::cmp::Eq for EXTTEXTMETRIC {}
impl ::core::fmt::Debug for EXTTEXTMETRIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTTEXTMETRIC")
            .field("emSize", &self.emSize)
            .field("emPointSize", &self.emPointSize)
            .field("emOrientation", &self.emOrientation)
            .field("emMasterHeight", &self.emMasterHeight)
            .field("emMinScale", &self.emMinScale)
            .field("emMaxScale", &self.emMaxScale)
            .field("emMasterUnits", &self.emMasterUnits)
            .field("emCapHeight", &self.emCapHeight)
            .field("emXHeight", &self.emXHeight)
            .field("emLowerCaseAscent", &self.emLowerCaseAscent)
            .field("emLowerCaseDescent", &self.emLowerCaseDescent)
            .field("emSlant", &self.emSlant)
            .field("emSuperScript", &self.emSuperScript)
            .field("emSubScript", &self.emSubScript)
            .field("emSuperScriptSize", &self.emSuperScriptSize)
            .field("emSubScriptSize", &self.emSubScriptSize)
            .field("emUnderlineOffset", &self.emUnderlineOffset)
            .field("emUnderlineWidth", &self.emUnderlineWidth)
            .field("emDoubleUpperUnderlineOffset", &self.emDoubleUpperUnderlineOffset)
            .field("emDoubleLowerUnderlineOffset", &self.emDoubleLowerUnderlineOffset)
            .field("emDoubleUpperUnderlineWidth", &self.emDoubleUpperUnderlineWidth)
            .field("emDoubleLowerUnderlineWidth", &self.emDoubleLowerUnderlineWidth)
            .field("emStrikeOutOffset", &self.emStrikeOutOffset)
            .field("emStrikeOutWidth", &self.emStrikeOutWidth)
            .field("emKernPairs", &self.emKernPairs)
            .field("emKernTracks", &self.emKernTracks)
            .finish()
    }
}
impl ::core::default::Default for EXpsCompressionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXpsCompressionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXpsCompressionOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXpsFontOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXpsFontOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXpsFontOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXpsFontRestriction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXpsFontRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXpsFontRestriction").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXpsJobConsumption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXpsJobConsumption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXpsJobConsumption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FORM_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FORM_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pName == other.pName && self.Size == other.Size && self.ImageableArea == other.ImageableArea
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FORM_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FORM_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORM_INFO_1A").field("Flags", &self.Flags).field("pName", &self.pName).field("Size", &self.Size).field("ImageableArea", &self.ImageableArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FORM_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FORM_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pName == other.pName && self.Size == other.Size && self.ImageableArea == other.ImageableArea
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FORM_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FORM_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORM_INFO_1W").field("Flags", &self.Flags).field("pName", &self.pName).field("Size", &self.Size).field("ImageableArea", &self.ImageableArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FORM_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FORM_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pName == other.pName && self.Size == other.Size && self.ImageableArea == other.ImageableArea && self.pKeyword == other.pKeyword && self.StringType == other.StringType && self.pMuiDll == other.pMuiDll && self.dwResourceId == other.dwResourceId && self.pDisplayName == other.pDisplayName && self.wLangId == other.wLangId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FORM_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FORM_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORM_INFO_2A").field("Flags", &self.Flags).field("pName", &self.pName).field("Size", &self.Size).field("ImageableArea", &self.ImageableArea).field("pKeyword", &self.pKeyword).field("StringType", &self.StringType).field("pMuiDll", &self.pMuiDll).field("dwResourceId", &self.dwResourceId).field("pDisplayName", &self.pDisplayName).field("wLangId", &self.wLangId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FORM_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FORM_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pName == other.pName && self.Size == other.Size && self.ImageableArea == other.ImageableArea && self.pKeyword == other.pKeyword && self.StringType == other.StringType && self.pMuiDll == other.pMuiDll && self.dwResourceId == other.dwResourceId && self.pDisplayName == other.pDisplayName && self.wLangId == other.wLangId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FORM_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FORM_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORM_INFO_2W").field("Flags", &self.Flags).field("pName", &self.pName).field("Size", &self.Size).field("ImageableArea", &self.ImageableArea).field("pKeyword", &self.pKeyword).field("StringType", &self.StringType).field("pMuiDll", &self.pMuiDll).field("dwResourceId", &self.dwResourceId).field("pDisplayName", &self.pDisplayName).field("wLangId", &self.wLangId).finish()
    }
}
impl ::core::default::Default for GLYPHRUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GLYPHRUN {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.wGlyphCount == other.wGlyphCount
    }
}
impl ::core::cmp::Eq for GLYPHRUN {}
impl ::core::fmt::Debug for GLYPHRUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHRUN").field("wcLow", &self.wcLow).field("wGlyphCount", &self.wGlyphCount).finish()
    }
}
impl ::core::cmp::PartialEq for IAsyncGetSendNotificationCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncGetSendNotificationCookie {}
impl ::core::fmt::Debug for IAsyncGetSendNotificationCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncGetSendNotificationCookie").field(&self.0).finish()
    }
}
impl IAsyncGetSendNotificationCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinishAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn CancelAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
}
impl ::core::cmp::PartialEq for IAsyncGetSrvReferralCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncGetSrvReferralCookie {}
impl ::core::fmt::Debug for IAsyncGetSrvReferralCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncGetSrvReferralCookie").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBidiAsyncNotifyChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBidiAsyncNotifyChannel {}
impl ::core::fmt::Debug for IBidiAsyncNotifyChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBidiAsyncNotifyChannel").field(&self.0).finish()
    }
}
impl IBidiAsyncNotifyChannel {
    pub unsafe fn SendNotification<P0>(&self, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SendNotification)(::windows::core::Vtable::as_raw(self), pdata.into().abi()).ok()
    }
    pub unsafe fn CloseChannel<P0>(&self, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CloseChannel)(::windows::core::Vtable::as_raw(self), pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IBidiRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBidiRequest {}
impl ::core::fmt::Debug for IBidiRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBidiRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBidiRequestContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBidiRequestContainer {}
impl ::core::fmt::Debug for IBidiRequestContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBidiRequestContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBidiSpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBidiSpl {}
impl ::core::fmt::Debug for IBidiSpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBidiSpl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBidiSpl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBidiSpl2 {}
impl ::core::fmt::Debug for IBidiSpl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBidiSpl2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFixedDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFixedDocument {}
impl ::core::fmt::Debug for IFixedDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFixedDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFixedDocumentSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFixedDocumentSequence {}
impl ::core::fmt::Debug for IFixedDocumentSequence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFixedDocumentSequence").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFixedPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFixedPage {}
impl ::core::fmt::Debug for IFixedPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFixedPage").field(&self.0).finish()
    }
}
impl IFixedPage {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IImgCreateErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IImgCreateErrorInfo {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IImgCreateErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImgCreateErrorInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetGUID(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), rguid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetSource<P0>(&self, szsource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSource)(::windows::core::Vtable::as_raw(self), szsource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), szdescription.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetHelpFile<P0>(&self, szhelpfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHelpFile)(::windows::core::Vtable::as_raw(self), szhelpfile.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IImgErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IImgErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IImgErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImgErrorInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHelpFile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHelpContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IInterFilterCommunicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInterFilterCommunicator {}
impl ::core::fmt::Debug for IInterFilterCommunicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInterFilterCommunicator").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSERTPSUIPAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INSERTPSUIPAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Type == other.Type && self.Mode == other.Mode && self.dwData1 == other.dwData1 && self.dwData2 == other.dwData2 && self.dwData3 == other.dwData3
    }
}
impl ::core::cmp::Eq for INSERTPSUIPAGE_INFO {}
impl ::core::fmt::Debug for INSERTPSUIPAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSERTPSUIPAGE_INFO").field("cbSize", &self.cbSize).field("Type", &self.Type).field("Mode", &self.Mode).field("dwData1", &self.dwData1).field("dwData2", &self.dwData2).field("dwData3", &self.dwData3).finish()
    }
}
impl ::core::default::Default for INVOC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INVOC {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.loOffset == other.loOffset
    }
}
impl ::core::cmp::Eq for INVOC {}
impl ::core::fmt::Debug for INVOC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INVOC").field("dwCount", &self.dwCount).field("loOffset", &self.loOffset).finish()
    }
}
impl ::core::cmp::PartialEq for IPartBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartBase {}
impl ::core::fmt::Debug for IPartBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartBase").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPartColorProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartColorProfile {}
impl ::core::fmt::Debug for IPartColorProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartColorProfile").field(&self.0).finish()
    }
}
impl IPartColorProfile {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPartDiscardControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartDiscardControl {}
impl ::core::fmt::Debug for IPartDiscardControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartDiscardControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPartFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartFont {}
impl ::core::fmt::Debug for IPartFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartFont").field(&self.0).finish()
    }
}
impl IPartFont {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPartFont2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartFont2 {}
impl ::core::fmt::Debug for IPartFont2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartFont2").field(&self.0).finish()
    }
}
impl IPartFont2 {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
    pub unsafe fn GetFontProperties(&self, pcontenttype: *mut ::windows::core::BSTR, pfontoptions: *mut EXpsFontOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pcontenttype), pfontoptions).ok()
    }
    pub unsafe fn SetFontContent<P0>(&self, pcontenttype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontContent)(::windows::core::Vtable::as_raw(self), pcontenttype.into().abi()).ok()
    }
    pub unsafe fn SetFontOptions(&self, options: EXpsFontOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFontOptions)(::windows::core::Vtable::as_raw(self), options).ok()
    }
}
impl ::core::cmp::PartialEq for IPartImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartImage {}
impl ::core::fmt::Debug for IPartImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartImage").field(&self.0).finish()
    }
}
impl IPartImage {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPartPrintTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartPrintTicket {}
impl ::core::fmt::Debug for IPartPrintTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartPrintTicket").field(&self.0).finish()
    }
}
impl IPartPrintTicket {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPartResourceDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartResourceDictionary {}
impl ::core::fmt::Debug for IPartResourceDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartResourceDictionary").field(&self.0).finish()
    }
}
impl IPartResourceDictionary {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPartThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartThumbnail {}
impl ::core::fmt::Debug for IPartThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartThumbnail").field(&self.0).finish()
    }
}
impl IPartThumbnail {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncCookie {}
impl ::core::fmt::Debug for IPrintAsyncCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncCookie").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNewChannelCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNewChannelCookie {}
impl ::core::fmt::Debug for IPrintAsyncNewChannelCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNewChannelCookie").field(&self.0).finish()
    }
}
impl IPrintAsyncNewChannelCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinishAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn CancelAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotify {}
impl ::core::fmt::Debug for IPrintAsyncNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotifyCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotifyCallback {}
impl ::core::fmt::Debug for IPrintAsyncNotifyCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotifyCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotifyChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotifyChannel {}
impl ::core::fmt::Debug for IPrintAsyncNotifyChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotifyChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotifyDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotifyDataObject {}
impl ::core::fmt::Debug for IPrintAsyncNotifyDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotifyDataObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotifyRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotifyRegistration {}
impl ::core::fmt::Debug for IPrintAsyncNotifyRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotifyRegistration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintAsyncNotifyServerReferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintAsyncNotifyServerReferral {}
impl ::core::fmt::Debug for IPrintAsyncNotifyServerReferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintAsyncNotifyServerReferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintBidiAsyncNotifyRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintBidiAsyncNotifyRegistration {}
impl ::core::fmt::Debug for IPrintBidiAsyncNotifyRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintBidiAsyncNotifyRegistration").field(&self.0).finish()
    }
}
impl IPrintBidiAsyncNotifyRegistration {
    pub unsafe fn RegisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RegisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnregisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintClassObjectFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintClassObjectFactory {}
impl ::core::fmt::Debug for IPrintClassObjectFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintClassObjectFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintCoreHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCoreHelper {}
impl ::core::fmt::Debug for IPrintCoreHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCoreHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintCoreHelperPS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCoreHelperPS {}
impl ::core::fmt::Debug for IPrintCoreHelperPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCoreHelperPS").field(&self.0).finish()
    }
}
impl IPrintCoreHelperPS {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetOption<P0>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturerequested: P0) -> ::windows::core::Result<::windows::core::PSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturerequested.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetOptions<P0>(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: P0, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, bresolveconflicts.into(), pfopairs, cpairs, pcpairswritten, pdwresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn EnumConstrainedOptions<P0>(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: P0, pconstrainedoptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumConstrainedOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, pszfeaturekeyword.into().abi(), pconstrainedoptionlist, pdwnumoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn WhyConstrained<P0, P1>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WhyConstrained)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), ppfoconstraints, pdwnumoptions).ok()
    }
    pub unsafe fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut ::windows::core::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumFeatures)(::windows::core::Vtable::as_raw(self), pfeaturelist, pdwnumfeatures).ok()
    }
    pub unsafe fn EnumOptions<P0>(&self, pszfeaturekeyword: P0, poptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumOptions)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), poptionlist, pdwnumoptions).ok()
    }
    pub unsafe fn GetFontSubstitution<P0>(&self, psztruetypefontname: P0, ppszdevfontname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), ppszdevfontname).ok()
    }
    pub unsafe fn SetFontSubstitution<P0, P1>(&self, psztruetypefontname: P0, pszdevfontname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), pszdevfontname.into().abi()).ok()
    }
    pub unsafe fn CreateInstanceOfMSXMLObject<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateInstanceOfMSXMLObject)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, ppv).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintCoreHelperUni {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCoreHelperUni {}
impl ::core::fmt::Debug for IPrintCoreHelperUni {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCoreHelperUni").field(&self.0).finish()
    }
}
impl IPrintCoreHelperUni {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetOption<P0>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturerequested: P0) -> ::windows::core::Result<::windows::core::PSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturerequested.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetOptions<P0>(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: P0, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, bresolveconflicts.into(), pfopairs, cpairs, pcpairswritten, pdwresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn EnumConstrainedOptions<P0>(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: P0, pconstrainedoptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumConstrainedOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, pszfeaturekeyword.into().abi(), pconstrainedoptionlist, pdwnumoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn WhyConstrained<P0, P1>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WhyConstrained)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), ppfoconstraints, pdwnumoptions).ok()
    }
    pub unsafe fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut ::windows::core::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumFeatures)(::windows::core::Vtable::as_raw(self), pfeaturelist, pdwnumfeatures).ok()
    }
    pub unsafe fn EnumOptions<P0>(&self, pszfeaturekeyword: P0, poptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumOptions)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), poptionlist, pdwnumoptions).ok()
    }
    pub unsafe fn GetFontSubstitution<P0>(&self, psztruetypefontname: P0, ppszdevfontname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), ppszdevfontname).ok()
    }
    pub unsafe fn SetFontSubstitution<P0, P1>(&self, psztruetypefontname: P0, pszdevfontname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), pszdevfontname.into().abi()).ok()
    }
    pub unsafe fn CreateInstanceOfMSXMLObject<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateInstanceOfMSXMLObject)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, ppv).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintCoreHelperUni2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCoreHelperUni2 {}
impl ::core::fmt::Debug for IPrintCoreHelperUni2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCoreHelperUni2").field(&self.0).finish()
    }
}
impl IPrintCoreHelperUni2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetOption<P0>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturerequested: P0) -> ::windows::core::Result<::windows::core::PSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturerequested.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetOptions<P0>(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: P0, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, bresolveconflicts.into(), pfopairs, cpairs, pcpairswritten, pdwresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn EnumConstrainedOptions<P0>(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: P0, pconstrainedoptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumConstrainedOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, pszfeaturekeyword.into().abi(), pconstrainedoptionlist, pdwnumoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn WhyConstrained<P0, P1>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.WhyConstrained)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), ppfoconstraints, pdwnumoptions).ok()
    }
    pub unsafe fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut ::windows::core::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumFeatures)(::windows::core::Vtable::as_raw(self), pfeaturelist, pdwnumfeatures).ok()
    }
    pub unsafe fn EnumOptions<P0>(&self, pszfeaturekeyword: P0, poptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumOptions)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), poptionlist, pdwnumoptions).ok()
    }
    pub unsafe fn GetFontSubstitution<P0>(&self, psztruetypefontname: P0, ppszdevfontname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), ppszdevfontname).ok()
    }
    pub unsafe fn SetFontSubstitution<P0, P1>(&self, psztruetypefontname: P0, pszdevfontname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), pszdevfontname.into().abi()).ok()
    }
    pub unsafe fn CreateInstanceOfMSXMLObject<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateInstanceOfMSXMLObject)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateGDLSnapshot)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, dwflags, ::core::mem::transmute(ppsnapshotstream)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDefaultGDLSnapshot)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPrintCoreUI2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCoreUI2 {}
impl ::core::fmt::Debug for IPrintCoreUI2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCoreUI2").field(&self.0).finish()
    }
}
impl IPrintCoreUI2 {
    pub unsafe fn DrvGetDriverSetting<P0>(&self, pci: *mut ::core::ffi::c_void, feature: P0, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrvGetDriverSetting)(::windows::core::Vtable::as_raw(self), pci, feature.into().abi(), poutput, cbsize, pcbneeded, pdwoptionsreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrvUpgradeRegistrySetting<P0, P1, P2>(&self, hprinter: P0, pfeature: P1, poption: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrvUpgradeRegistrySetting)(::windows::core::Vtable::as_raw(self), hprinter.into(), pfeature.into().abi(), poption.into().abi()).ok()
    }
    pub unsafe fn DrvUpdateUISetting(&self, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrvUpdateUISetting)(::windows::core::Vtable::as_raw(self), pci, poptitem, dwpreviousselection, dwmode).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintJob {}
impl ::core::fmt::Debug for IPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintJobCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintJobCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintJobCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintJobCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintOemCommon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOemCommon {}
impl ::core::fmt::Debug for IPrintOemCommon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOemCommon").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintOemDriverUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOemDriverUI {}
impl ::core::fmt::Debug for IPrintOemDriverUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOemDriverUI").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintOemUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOemUI {}
impl ::core::fmt::Debug for IPrintOemUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOemUI").field(&self.0).finish()
    }
}
impl IPrintOemUI {
    pub unsafe fn GetInfo(&self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self), dwmode, pbuffer, cbsize, pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DevMode)(::windows::core::Vtable::as_raw(self), dwmode, poemdmparam).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintOemUI2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOemUI2 {}
impl ::core::fmt::Debug for IPrintOemUI2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOemUI2").field(&self.0).finish()
    }
}
impl IPrintOemUI2 {
    pub unsafe fn GetInfo(&self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInfo)(::windows::core::Vtable::as_raw(self), dwmode, pbuffer, cbsize, pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DevMode)(::windows::core::Vtable::as_raw(self), dwmode, poemdmparam).ok()
    }
    pub unsafe fn PublishDriverInterface<P0>(&self, piunknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PublishDriverInterface)(::windows::core::Vtable::as_raw(self), piunknown.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CommonUIProp)(::windows::core::Vtable::as_raw(self), dwmode, poemcuipparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentPropertySheets<P0>(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.DocumentPropertySheets)(::windows::core::Vtable::as_raw(self), ppsuiinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DevicePropertySheets<P0>(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.DevicePropertySheets)(::windows::core::Vtable::as_raw(self), ppsuiinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DevQueryPrintEx)(::windows::core::Vtable::as_raw(self), poemuiobj, pdqpinfo, ppublicdm, poemdm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DeviceCapabilitiesA<P0, P1>(&self, poemuiobj: *mut OEMUIOBJ, hprinter: P0, pdevicename: P1, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeviceCapabilitiesA)(::windows::core::Vtable::as_raw(self), poemuiobj, hprinter.into(), pdevicename.into().abi(), wcapability, poutput, ppublicdm, poemdm, dwold, dwresult).ok()
    }
    pub unsafe fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UpgradePrinter)(::windows::core::Vtable::as_raw(self), dwlevel, pdriverupgradeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrinterEvent<P0, P1>(&self, pprintername: P0, idriverevent: i32, dwflags: u32, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.PrinterEvent)(::windows::core::Vtable::as_raw(self), pprintername.into().abi(), idriverevent, dwflags, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverEvent<P0>(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.DriverEvent)(::windows::core::Vtable::as_raw(self), dwdriverevent, dwlevel, pdriverinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn QueryColorProfile<P0>(&self, hprinter: P0, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.QueryColorProfile)(::windows::core::Vtable::as_raw(self), hprinter.into(), poemuiobj, ppublicdm, poemdm, ulquerymode, pvprofiledata, pcbprofiledata, pflprofiledata).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FontInstallerDlgProc<P0, P1, P2>(&self, hwnd: P0, usmsg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.FontInstallerDlgProc)(::windows::core::Vtable::as_raw(self), hwnd.into(), usmsg, wparam.into(), lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateExternalFonts<P0, P1, P2>(&self, hprinter: P0, hheap: P1, pwstrcartridges: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateExternalFonts)(::windows::core::Vtable::as_raw(self), hprinter.into(), hheap.into(), pwstrcartridges.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintOemUIMXDC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOemUIMXDC {}
impl ::core::fmt::Debug for IPrintOemUIMXDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOemUIMXDC").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintPipelineFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintPipelineFilter {}
impl ::core::fmt::Debug for IPrintPipelineFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintPipelineFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintPipelineManagerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintPipelineManagerControl {}
impl ::core::fmt::Debug for IPrintPipelineManagerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintPipelineManagerControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintPipelineProgressReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintPipelineProgressReport {}
impl ::core::fmt::Debug for IPrintPipelineProgressReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintPipelineProgressReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintPipelinePropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintPipelinePropertyBag {}
impl ::core::fmt::Debug for IPrintPipelinePropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintPipelinePropertyBag").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintPreviewDxgiPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintPreviewDxgiPackageTarget {}
impl ::core::fmt::Debug for IPrintPreviewDxgiPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintPreviewDxgiPackageTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintReadStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintReadStream {}
impl ::core::fmt::Debug for IPrintReadStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintReadStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintReadStreamFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintReadStreamFactory {}
impl ::core::fmt::Debug for IPrintReadStreamFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintReadStreamFactory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaAsyncOperation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaAsyncOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaAsyncOperationEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaAsyncOperationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaAsyncOperationEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaAsyncOperationEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaCapabilities").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaCapabilities2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaCapabilities2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaCapabilities2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaCapabilities2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities2 {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFeatureByKeyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrkeyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeature(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFeature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PageImageableSize(&self) -> ::windows::core::Result<IPrintSchemaPageImageableSize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PageImageableSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocumentsMinValue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.JobCopiesAllDocumentsMinValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocumentsMaxValue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.JobCopiesAllDocumentsMaxValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedOptionInPrintTicket<P0>(&self, pfeature: P0) -> ::windows::core::Result<IPrintSchemaOption>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaFeature>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelectedOptionInPrintTicket)(::windows::core::Vtable::as_raw(self), pfeature.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOptions<P0>(&self, pfeature: P0) -> ::windows::core::Result<IPrintSchemaOptionCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaFeature>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptions)(::windows::core::Vtable::as_raw(self), pfeature.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaDisplayableElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaDisplayableElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaDisplayableElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaDisplayableElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaDisplayableElement {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaFeature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaFeature {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaFeature").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaFeature {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaNUpOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaNUpOption {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaNUpOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaNUpOption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaNUpOption {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Selected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Constrained(&self) -> ::windows::core::Result<PrintSchemaConstrainedSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Constrained)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValue(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaOption {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaOption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOption {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaOptionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaOptionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaOptionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaOptionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaPageImageableSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaPageImageableSize {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaPageImageableSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaPageImageableSize").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageImageableSize {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaPageMediaSizeOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaPageMediaSizeOption {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaPageMediaSizeOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaPageMediaSizeOption").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageMediaSizeOption {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Selected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Constrained(&self) -> ::windows::core::Result<PrintSchemaConstrainedSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Constrained)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValue(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaParameterDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaParameterDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaParameterDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaParameterDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterDefinition {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaParameterInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaParameterInitializer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaParameterInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaParameterInitializer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterInitializer {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaTicket {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaTicket").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintSchemaTicket2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintSchemaTicket2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintSchemaTicket2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintSchemaTicket2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket2 {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFeatureByKeyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrkeyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeature(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFeature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValidateAsync(&self) -> ::windows::core::Result<IPrintSchemaAsyncOperation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ValidateAsync)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAsync<P0>(&self, pprintticketcommit: P0) -> ::windows::core::Result<IPrintSchemaAsyncOperation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaTicket>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CommitAsync)(::windows::core::Vtable::as_raw(self), pprintticketcommit.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyXmlChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NotifyXmlChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<IPrintSchemaCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocuments(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.JobCopiesAllDocuments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetJobCopiesAllDocuments)(::windows::core::Vtable::as_raw(self), uljobcopiesalldocuments).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintTicketProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTicketProvider {}
impl ::core::fmt::Debug for IPrintTicketProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTicketProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintTicketProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTicketProvider2 {}
impl ::core::fmt::Debug for IPrintTicketProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTicketProvider2").field(&self.0).finish()
    }
}
impl IPrintTicketProvider2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedVersions<P0>(&self, hprinter: P0, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSupportedVersions)(::windows::core::Vtable::as_raw(self), hprinter.into(), ppversions, cversions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BindPrinter<P0>(&self, hprinter: P0, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.BindPrinter)(::windows::core::Vtable::as_raw(self), hprinter.into(), version, poptions, pdevmodeflags, cnamespaces, ppnamespaces).ok()
    }
    pub unsafe fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryDeviceNamespace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdefaultnamespace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertPrintTicketToDevMode<P0>(&self, pprintticket: P0, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertPrintTicketToDevMode)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi(), cbdevmodein, pdevmodein, pcbdevmodeout, ppdevmodeout).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertDevModeToPrintTicket<P0>(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertDevModeToPrintTicket)(::windows::core::Vtable::as_raw(self), cbdevmode, pdevmode, pprintticket.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintCapabilities<P0>(&self, pprintticket: P0) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPrintCapabilities)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn ValidatePrintTicket<P0>(&self, pbaseticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ValidatePrintTicket)(::windows::core::Vtable::as_raw(self), pbaseticket.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintUnidiAsyncNotifyRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintUnidiAsyncNotifyRegistration {}
impl ::core::fmt::Debug for IPrintUnidiAsyncNotifyRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintUnidiAsyncNotifyRegistration").field(&self.0).finish()
    }
}
impl IPrintUnidiAsyncNotifyRegistration {
    pub unsafe fn RegisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RegisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnregisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IPrintWriteStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWriteStream {}
impl ::core::fmt::Debug for IPrintWriteStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWriteStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintWriteStreamFlush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWriteStreamFlush {}
impl ::core::fmt::Debug for IPrintWriteStreamFlush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWriteStreamFlush").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrinterBidiSetRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinterBidiSetRequestCallback {}
impl ::core::fmt::Debug for IPrinterBidiSetRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterBidiSetRequestCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrinterExtensionAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinterExtensionAsyncOperation {}
impl ::core::fmt::Debug for IPrinterExtensionAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionAsyncOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterExtensionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterExtensionContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterExtensionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterExtensionContextCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterExtensionContextCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterExtensionContextCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionContextCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterExtensionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterExtensionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterExtensionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterExtensionEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterExtensionEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterExtensionEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventArgs {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrinterQueue(&self) -> ::windows::core::Result<IPrinterQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrinterQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrintSchemaTicket(&self) -> ::windows::core::Result<IPrintSchemaTicket> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrintSchemaTicket)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DriverProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DriverProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UserProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPrinterExtensionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinterExtensionManager {}
impl ::core::fmt::Debug for IPrinterExtensionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterExtensionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterExtensionRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterExtensionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterExtensionRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterQueue2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterQueue2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterQueue2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterQueue2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Handle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendBidiQuery(&self, bstrbidiquery: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendBidiQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbidiquery)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterQueueEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterQueueEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterQueueEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterQueueEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterQueueView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterQueueView {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterQueueView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterQueueView").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterQueueViewEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterQueueViewEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterQueueViewEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterQueueViewEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterScriptContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterScriptContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterScriptContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterScriptContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterScriptablePropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterScriptablePropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterScriptablePropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterScriptablePropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterScriptablePropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterScriptablePropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterScriptablePropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterScriptablePropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBool(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBool<P0>(&self, bstrname: &::windows::core::BSTR, bvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), bvalue.into()).ok()
    }
    pub unsafe fn GetInt32(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInt32(&self, bstrname: &::windows::core::BSTR, nvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), nvalue).ok()
    }
    pub unsafe fn GetString(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetString(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBytes(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBytes<P0>(&self, bstrname: &::windows::core::BSTR, parray: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), parray.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReadStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IPrinterScriptableStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReadStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWriteStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IPrinterScriptableStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWriteStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterScriptableSequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterScriptableSequentialStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterScriptableSequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterScriptableSequentialStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrinterScriptableStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrinterScriptableStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrinterScriptableStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinterScriptableStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, cbread: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), cbread, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0>(&self, parray: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), parray.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocument {}
impl ::core::fmt::Debug for IXpsDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentConsumer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentConsumer {}
impl ::core::fmt::Debug for IXpsDocumentConsumer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentConsumer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentProvider {}
impl ::core::fmt::Debug for IXpsDocumentProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsPartIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsPartIterator {}
impl ::core::fmt::Debug for IXpsPartIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsPartIterator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsRasterizationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsRasterizationFactory {}
impl ::core::fmt::Debug for IXpsRasterizationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsRasterizationFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsRasterizationFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsRasterizationFactory1 {}
impl ::core::fmt::Debug for IXpsRasterizationFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsRasterizationFactory1").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsRasterizationFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsRasterizationFactory2 {}
impl ::core::fmt::Debug for IXpsRasterizationFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsRasterizationFactory2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsRasterizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsRasterizer {}
impl ::core::fmt::Debug for IXpsRasterizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsRasterizer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsRasterizerNotificationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsRasterizerNotificationCallback {}
impl ::core::fmt::Debug for IXpsRasterizerNotificationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsRasterizerNotificationCallback").field(&self.0).finish()
    }
}
impl ::core::default::Default for ImgErrorInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ImgErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description && self.guid == other.guid && self.helpContext == other.helpContext && self.helpFile == other.helpFile && self.source == other.source && self.devDescription == other.devDescription && self.errorID == other.errorID && self.cUserParameters == other.cUserParameters && self.aUserParameters == other.aUserParameters && self.userFallback == other.userFallback && self.exceptionID == other.exceptionID
    }
}
impl ::core::cmp::Eq for ImgErrorInfo {}
impl ::core::fmt::Debug for ImgErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ImgErrorInfo").field("description", &self.description).field("guid", &self.guid).field("helpContext", &self.helpContext).field("helpFile", &self.helpFile).field("source", &self.source).field("devDescription", &self.devDescription).field("errorID", &self.errorID).field("cUserParameters", &self.cUserParameters).field("aUserParameters", &self.aUserParameters).field("userFallback", &self.userFallback).field("exceptionID", &self.exceptionID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOB_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOB_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.pPrinterName == other.pPrinterName && self.pMachineName == other.pMachineName && self.pUserName == other.pUserName && self.pDocument == other.pDocument && self.pDatatype == other.pDatatype && self.pStatus == other.pStatus && self.Status == other.Status && self.Priority == other.Priority && self.Position == other.Position && self.TotalPages == other.TotalPages && self.PagesPrinted == other.PagesPrinted && self.Submitted == other.Submitted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOB_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOB_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_1A")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pDatatype", &self.pDatatype)
            .field("pStatus", &self.pStatus)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("TotalPages", &self.TotalPages)
            .field("PagesPrinted", &self.PagesPrinted)
            .field("Submitted", &self.Submitted)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOB_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOB_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.pPrinterName == other.pPrinterName && self.pMachineName == other.pMachineName && self.pUserName == other.pUserName && self.pDocument == other.pDocument && self.pDatatype == other.pDatatype && self.pStatus == other.pStatus && self.Status == other.Status && self.Priority == other.Priority && self.Position == other.Position && self.TotalPages == other.TotalPages && self.PagesPrinted == other.PagesPrinted && self.Submitted == other.Submitted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOB_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOB_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_1W")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pDatatype", &self.pDatatype)
            .field("pStatus", &self.pStatus)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("TotalPages", &self.TotalPages)
            .field("PagesPrinted", &self.PagesPrinted)
            .field("Submitted", &self.Submitted)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for JOB_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOB_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId
            && self.pPrinterName == other.pPrinterName
            && self.pMachineName == other.pMachineName
            && self.pUserName == other.pUserName
            && self.pDocument == other.pDocument
            && self.pNotifyName == other.pNotifyName
            && self.pDatatype == other.pDatatype
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pParameters == other.pParameters
            && self.pDriverName == other.pDriverName
            && self.pDevMode == other.pDevMode
            && self.pStatus == other.pStatus
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Status == other.Status
            && self.Priority == other.Priority
            && self.Position == other.Position
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.TotalPages == other.TotalPages
            && self.Size == other.Size
            && self.Submitted == other.Submitted
            && self.Time == other.Time
            && self.PagesPrinted == other.PagesPrinted
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOB_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOB_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_2A")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pNotifyName", &self.pNotifyName)
            .field("pDatatype", &self.pDatatype)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pParameters", &self.pParameters)
            .field("pDriverName", &self.pDriverName)
            .field("pDevMode", &self.pDevMode)
            .field("pStatus", &self.pStatus)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("TotalPages", &self.TotalPages)
            .field("Size", &self.Size)
            .field("Submitted", &self.Submitted)
            .field("Time", &self.Time)
            .field("PagesPrinted", &self.PagesPrinted)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for JOB_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOB_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId
            && self.pPrinterName == other.pPrinterName
            && self.pMachineName == other.pMachineName
            && self.pUserName == other.pUserName
            && self.pDocument == other.pDocument
            && self.pNotifyName == other.pNotifyName
            && self.pDatatype == other.pDatatype
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pParameters == other.pParameters
            && self.pDriverName == other.pDriverName
            && self.pDevMode == other.pDevMode
            && self.pStatus == other.pStatus
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Status == other.Status
            && self.Priority == other.Priority
            && self.Position == other.Position
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.TotalPages == other.TotalPages
            && self.Size == other.Size
            && self.Submitted == other.Submitted
            && self.Time == other.Time
            && self.PagesPrinted == other.PagesPrinted
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOB_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOB_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_2W")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pNotifyName", &self.pNotifyName)
            .field("pDatatype", &self.pDatatype)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pParameters", &self.pParameters)
            .field("pDriverName", &self.pDriverName)
            .field("pDevMode", &self.pDevMode)
            .field("pStatus", &self.pStatus)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("TotalPages", &self.TotalPages)
            .field("Size", &self.Size)
            .field("Submitted", &self.Submitted)
            .field("Time", &self.Time)
            .field("PagesPrinted", &self.PagesPrinted)
            .finish()
    }
}
impl ::core::default::Default for JOB_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOB_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.NextJobId == other.NextJobId && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for JOB_INFO_3 {}
impl ::core::fmt::Debug for JOB_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_3").field("JobId", &self.JobId).field("NextJobId", &self.NextJobId).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for JOB_INFO_4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOB_INFO_4A {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId
            && self.pPrinterName == other.pPrinterName
            && self.pMachineName == other.pMachineName
            && self.pUserName == other.pUserName
            && self.pDocument == other.pDocument
            && self.pNotifyName == other.pNotifyName
            && self.pDatatype == other.pDatatype
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pParameters == other.pParameters
            && self.pDriverName == other.pDriverName
            && self.pDevMode == other.pDevMode
            && self.pStatus == other.pStatus
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Status == other.Status
            && self.Priority == other.Priority
            && self.Position == other.Position
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.TotalPages == other.TotalPages
            && self.Size == other.Size
            && self.Submitted == other.Submitted
            && self.Time == other.Time
            && self.PagesPrinted == other.PagesPrinted
            && self.SizeHigh == other.SizeHigh
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOB_INFO_4A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOB_INFO_4A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_4A")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pNotifyName", &self.pNotifyName)
            .field("pDatatype", &self.pDatatype)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pParameters", &self.pParameters)
            .field("pDriverName", &self.pDriverName)
            .field("pDevMode", &self.pDevMode)
            .field("pStatus", &self.pStatus)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("TotalPages", &self.TotalPages)
            .field("Size", &self.Size)
            .field("Submitted", &self.Submitted)
            .field("Time", &self.Time)
            .field("PagesPrinted", &self.PagesPrinted)
            .field("SizeHigh", &self.SizeHigh)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for JOB_INFO_4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOB_INFO_4W {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId
            && self.pPrinterName == other.pPrinterName
            && self.pMachineName == other.pMachineName
            && self.pUserName == other.pUserName
            && self.pDocument == other.pDocument
            && self.pNotifyName == other.pNotifyName
            && self.pDatatype == other.pDatatype
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pParameters == other.pParameters
            && self.pDriverName == other.pDriverName
            && self.pDevMode == other.pDevMode
            && self.pStatus == other.pStatus
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Status == other.Status
            && self.Priority == other.Priority
            && self.Position == other.Position
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.TotalPages == other.TotalPages
            && self.Size == other.Size
            && self.Submitted == other.Submitted
            && self.Time == other.Time
            && self.PagesPrinted == other.PagesPrinted
            && self.SizeHigh == other.SizeHigh
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOB_INFO_4W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOB_INFO_4W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_INFO_4W")
            .field("JobId", &self.JobId)
            .field("pPrinterName", &self.pPrinterName)
            .field("pMachineName", &self.pMachineName)
            .field("pUserName", &self.pUserName)
            .field("pDocument", &self.pDocument)
            .field("pNotifyName", &self.pNotifyName)
            .field("pDatatype", &self.pDatatype)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pParameters", &self.pParameters)
            .field("pDriverName", &self.pDriverName)
            .field("pDevMode", &self.pDevMode)
            .field("pStatus", &self.pStatus)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Status", &self.Status)
            .field("Priority", &self.Priority)
            .field("Position", &self.Position)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("TotalPages", &self.TotalPages)
            .field("Size", &self.Size)
            .field("Submitted", &self.Submitted)
            .field("Time", &self.Time)
            .field("PagesPrinted", &self.PagesPrinted)
            .field("SizeHigh", &self.SizeHigh)
            .finish()
    }
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::default::Default for KERNDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::cmp::PartialEq for KERNDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwKernPairNum == other.dwKernPairNum && self.KernPair == other.KernPair
    }
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::cmp::Eq for KERNDATA {}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::fmt::Debug for KERNDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERNDATA").field("dwSize", &self.dwSize).field("dwKernPairNum", &self.dwKernPairNum).field("KernPair", &self.KernPair).finish()
    }
}
impl ::core::default::Default for MAPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MESSAGEBOX_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MESSAGEBOX_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pTitle == other.pTitle && self.pMessage == other.pMessage && self.Style == other.Style && self.dwTimeout == other.dwTimeout && self.bWait == other.bWait
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MESSAGEBOX_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MESSAGEBOX_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGEBOX_PARAMS").field("cbSize", &self.cbSize).field("pTitle", &self.pTitle).field("pMessage", &self.pMessage).field("Style", &self.Style).field("dwTimeout", &self.dwTimeout).field("bWait", &self.bWait).finish()
    }
}
impl ::core::default::Default for MONITOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR {
    fn eq(&self, other: &Self) -> bool {
        self.pfnEnumPorts == other.pfnEnumPorts && self.pfnOpenPort == other.pfnOpenPort && self.pfnOpenPortEx == other.pfnOpenPortEx && self.pfnStartDocPort == other.pfnStartDocPort && self.pfnWritePort == other.pfnWritePort && self.pfnReadPort == other.pfnReadPort && self.pfnEndDocPort == other.pfnEndDocPort && self.pfnClosePort == other.pfnClosePort && self.pfnAddPort == other.pfnAddPort && self.pfnAddPortEx == other.pfnAddPortEx && self.pfnConfigurePort == other.pfnConfigurePort && self.pfnDeletePort == other.pfnDeletePort && self.pfnGetPrinterDataFromPort == other.pfnGetPrinterDataFromPort && self.pfnSetPortTimeOuts == other.pfnSetPortTimeOuts && self.pfnXcvOpenPort == other.pfnXcvOpenPort && self.pfnXcvDataPort == other.pfnXcvDataPort && self.pfnXcvClosePort == other.pfnXcvClosePort
    }
}
impl ::core::cmp::Eq for MONITOR {}
impl ::core::fmt::Debug for MONITOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR")
            .field("pfnEnumPorts", &self.pfnEnumPorts)
            .field("pfnOpenPort", &self.pfnOpenPort)
            .field("pfnOpenPortEx", &self.pfnOpenPortEx)
            .field("pfnStartDocPort", &self.pfnStartDocPort)
            .field("pfnWritePort", &self.pfnWritePort)
            .field("pfnReadPort", &self.pfnReadPort)
            .field("pfnEndDocPort", &self.pfnEndDocPort)
            .field("pfnClosePort", &self.pfnClosePort)
            .field("pfnAddPort", &self.pfnAddPort)
            .field("pfnAddPortEx", &self.pfnAddPortEx)
            .field("pfnConfigurePort", &self.pfnConfigurePort)
            .field("pfnDeletePort", &self.pfnDeletePort)
            .field("pfnGetPrinterDataFromPort", &self.pfnGetPrinterDataFromPort)
            .field("pfnSetPortTimeOuts", &self.pfnSetPortTimeOuts)
            .field("pfnXcvOpenPort", &self.pfnXcvOpenPort)
            .field("pfnXcvDataPort", &self.pfnXcvDataPort)
            .field("pfnXcvClosePort", &self.pfnXcvClosePort)
            .finish()
    }
}
impl ::core::default::Default for MONITOR2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.pfnEnumPorts == other.pfnEnumPorts
            && self.pfnOpenPort == other.pfnOpenPort
            && self.pfnOpenPortEx == other.pfnOpenPortEx
            && self.pfnStartDocPort == other.pfnStartDocPort
            && self.pfnWritePort == other.pfnWritePort
            && self.pfnReadPort == other.pfnReadPort
            && self.pfnEndDocPort == other.pfnEndDocPort
            && self.pfnClosePort == other.pfnClosePort
            && self.pfnAddPort == other.pfnAddPort
            && self.pfnAddPortEx == other.pfnAddPortEx
            && self.pfnConfigurePort == other.pfnConfigurePort
            && self.pfnDeletePort == other.pfnDeletePort
            && self.pfnGetPrinterDataFromPort == other.pfnGetPrinterDataFromPort
            && self.pfnSetPortTimeOuts == other.pfnSetPortTimeOuts
            && self.pfnXcvOpenPort == other.pfnXcvOpenPort
            && self.pfnXcvDataPort == other.pfnXcvDataPort
            && self.pfnXcvClosePort == other.pfnXcvClosePort
            && self.pfnShutdown == other.pfnShutdown
            && self.pfnSendRecvBidiDataFromPort == other.pfnSendRecvBidiDataFromPort
            && self.pfnNotifyUsedPorts == other.pfnNotifyUsedPorts
            && self.pfnNotifyUnusedPorts == other.pfnNotifyUnusedPorts
            && self.pfnPowerEvent == other.pfnPowerEvent
    }
}
impl ::core::cmp::Eq for MONITOR2 {}
impl ::core::fmt::Debug for MONITOR2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR2")
            .field("cbSize", &self.cbSize)
            .field("pfnEnumPorts", &self.pfnEnumPorts)
            .field("pfnOpenPort", &self.pfnOpenPort)
            .field("pfnOpenPortEx", &self.pfnOpenPortEx)
            .field("pfnStartDocPort", &self.pfnStartDocPort)
            .field("pfnWritePort", &self.pfnWritePort)
            .field("pfnReadPort", &self.pfnReadPort)
            .field("pfnEndDocPort", &self.pfnEndDocPort)
            .field("pfnClosePort", &self.pfnClosePort)
            .field("pfnAddPort", &self.pfnAddPort)
            .field("pfnAddPortEx", &self.pfnAddPortEx)
            .field("pfnConfigurePort", &self.pfnConfigurePort)
            .field("pfnDeletePort", &self.pfnDeletePort)
            .field("pfnGetPrinterDataFromPort", &self.pfnGetPrinterDataFromPort)
            .field("pfnSetPortTimeOuts", &self.pfnSetPortTimeOuts)
            .field("pfnXcvOpenPort", &self.pfnXcvOpenPort)
            .field("pfnXcvDataPort", &self.pfnXcvDataPort)
            .field("pfnXcvClosePort", &self.pfnXcvClosePort)
            .field("pfnShutdown", &self.pfnShutdown)
            .field("pfnSendRecvBidiDataFromPort", &self.pfnSendRecvBidiDataFromPort)
            .field("pfnNotifyUsedPorts", &self.pfnNotifyUsedPorts)
            .field("pfnNotifyUnusedPorts", &self.pfnNotifyUnusedPorts)
            .field("pfnPowerEvent", &self.pfnPowerEvent)
            .finish()
    }
}
impl ::core::default::Default for MONITOREX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOREX {
    fn eq(&self, other: &Self) -> bool {
        self.dwMonitorSize == other.dwMonitorSize && self.Monitor == other.Monitor
    }
}
impl ::core::cmp::Eq for MONITOREX {}
impl ::core::fmt::Debug for MONITOREX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOREX").field("dwMonitorSize", &self.dwMonitorSize).field("Monitor", &self.Monitor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for MONITORINIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::cmp::PartialEq for MONITORINIT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hSpooler == other.hSpooler && self.hckRegistryRoot == other.hckRegistryRoot && self.pMonitorReg == other.pMonitorReg && self.bLocal == other.bLocal && self.pszServerName == other.pszServerName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::cmp::Eq for MONITORINIT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::fmt::Debug for MONITORINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINIT").field("cbSize", &self.cbSize).field("hSpooler", &self.hSpooler).field("hckRegistryRoot", &self.hckRegistryRoot).field("pMonitorReg", &self.pMonitorReg).field("bLocal", &self.bLocal).field("pszServerName", &self.pszServerName).finish()
    }
}
impl ::core::default::Default for MONITORREG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITORREG {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fpCreateKey == other.fpCreateKey && self.fpOpenKey == other.fpOpenKey && self.fpCloseKey == other.fpCloseKey && self.fpDeleteKey == other.fpDeleteKey && self.fpEnumKey == other.fpEnumKey && self.fpQueryInfoKey == other.fpQueryInfoKey && self.fpSetValue == other.fpSetValue && self.fpDeleteValue == other.fpDeleteValue && self.fpEnumValue == other.fpEnumValue && self.fpQueryValue == other.fpQueryValue
    }
}
impl ::core::cmp::Eq for MONITORREG {}
impl ::core::fmt::Debug for MONITORREG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORREG").field("cbSize", &self.cbSize).field("fpCreateKey", &self.fpCreateKey).field("fpOpenKey", &self.fpOpenKey).field("fpCloseKey", &self.fpCloseKey).field("fpDeleteKey", &self.fpDeleteKey).field("fpEnumKey", &self.fpEnumKey).field("fpQueryInfoKey", &self.fpQueryInfoKey).field("fpSetValue", &self.fpSetValue).field("fpDeleteValue", &self.fpDeleteValue).field("fpEnumValue", &self.fpEnumValue).field("fpQueryValue", &self.fpQueryValue).finish()
    }
}
impl ::core::default::Default for MONITORUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITORUI {
    fn eq(&self, other: &Self) -> bool {
        self.dwMonitorUISize == other.dwMonitorUISize && self.pfnAddPortUI == other.pfnAddPortUI && self.pfnConfigurePortUI == other.pfnConfigurePortUI && self.pfnDeletePortUI == other.pfnDeletePortUI
    }
}
impl ::core::cmp::Eq for MONITORUI {}
impl ::core::fmt::Debug for MONITORUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORUI").field("dwMonitorUISize", &self.dwMonitorUISize).field("pfnAddPortUI", &self.pfnAddPortUI).field("pfnConfigurePortUI", &self.pfnConfigurePortUI).field("pfnDeletePortUI", &self.pfnDeletePortUI).finish()
    }
}
impl ::core::default::Default for MONITOR_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for MONITOR_INFO_1A {}
impl ::core::fmt::Debug for MONITOR_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR_INFO_1A").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for MONITOR_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for MONITOR_INFO_1W {}
impl ::core::fmt::Debug for MONITOR_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR_INFO_1W").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for MONITOR_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDLLName == other.pDLLName
    }
}
impl ::core::cmp::Eq for MONITOR_INFO_2A {}
impl ::core::fmt::Debug for MONITOR_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR_INFO_2A").field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDLLName", &self.pDLLName).finish()
    }
}
impl ::core::default::Default for MONITOR_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MONITOR_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDLLName == other.pDLLName
    }
}
impl ::core::cmp::Eq for MONITOR_INFO_2W {}
impl ::core::fmt::Debug for MONITOR_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR_INFO_2W").field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDLLName", &self.pDLLName).finish()
    }
}
impl ::core::default::Default for MXDC_ESCAPE_HEADER_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_GET_FILENAME_DATA_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_IMAGE_TYPE_ENUMS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MXDC_IMAGE_TYPE_ENUMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MXDC_IMAGE_TYPE_ENUMS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MXDC_LANDSCAPE_ROTATION_ENUMS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MXDC_LANDSCAPE_ROTATION_ENUMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MXDC_LANDSCAPE_ROTATION_ENUMS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MXDC_PRINTTICKET_DATA_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_PRINTTICKET_ESCAPE_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_S0PAGE_DATA_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MXDC_S0_PAGE_ENUMS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MXDC_S0_PAGE_ENUMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MXDC_S0_PAGE_ENUMS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MXDC_XPS_S0PAGE_RESOURCE_T {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NOTIFICATION_CALLBACK_COMMANDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFICATION_CALLBACK_COMMANDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFICATION_CALLBACK_COMMANDS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFICATION_CONFIG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NOTIFICATION_CONFIG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFICATION_CONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFICATION_CONFIG_FLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OEMCUIPPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for OEMDMPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for OEMDMPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pdriverobj == other.pdriverobj && self.hPrinter == other.hPrinter && self.hModule == other.hModule && self.pPublicDMIn == other.pPublicDMIn && self.pPublicDMOut == other.pPublicDMOut && self.pOEMDMIn == other.pOEMDMIn && self.pOEMDMOut == other.pOEMDMOut && self.cbBufSize == other.cbBufSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for OEMDMPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for OEMDMPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OEMDMPARAM").field("cbSize", &self.cbSize).field("pdriverobj", &self.pdriverobj).field("hPrinter", &self.hPrinter).field("hModule", &self.hModule).field("pPublicDMIn", &self.pPublicDMIn).field("pPublicDMOut", &self.pPublicDMOut).field("pOEMDMIn", &self.pOEMDMIn).field("pOEMDMOut", &self.pOEMDMOut).field("cbBufSize", &self.cbBufSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OEMFONTINSTPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OEMFONTINSTPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hPrinter == other.hPrinter && self.hModule == other.hModule && self.hHeap == other.hHeap && self.dwFlags == other.dwFlags && self.pFontInstallerName == other.pFontInstallerName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OEMFONTINSTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OEMFONTINSTPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OEMFONTINSTPARAM").field("cbSize", &self.cbSize).field("hPrinter", &self.hPrinter).field("hModule", &self.hModule).field("hHeap", &self.hHeap).field("dwFlags", &self.dwFlags).field("pFontInstallerName", &self.pFontInstallerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OEMUIOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OEMUIOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pOemUIProcs == other.pOemUIProcs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OEMUIOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OEMUIOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OEMUIOBJ").field("cbSize", &self.cbSize).field("pOemUIProcs", &self.pOemUIProcs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OEMUIPROCS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for OEMUIPSPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for OEMUIPSPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.poemuiobj == other.poemuiobj && self.hPrinter == other.hPrinter && self.pPrinterName == other.pPrinterName && self.hModule == other.hModule && self.hOEMHeap == other.hOEMHeap && self.pPublicDM == other.pPublicDM && self.pOEMDM == other.pOEMDM && self.pOEMUserData == other.pOEMUserData && self.dwFlags == other.dwFlags && self.pOemEntry == other.pOemEntry
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for OEMUIPSPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for OEMUIPSPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OEMUIPSPARAM").field("cbSize", &self.cbSize).field("poemuiobj", &self.poemuiobj).field("hPrinter", &self.hPrinter).field("pPrinterName", &self.pPrinterName).field("hModule", &self.hModule).field("hOEMHeap", &self.hOEMHeap).field("pPublicDM", &self.pPublicDM).field("pOEMDM", &self.pOEMDM).field("pOEMUserData", &self.pOEMUserData).field("dwFlags", &self.dwFlags).field("pOemEntry", &self.pOemEntry).finish()
    }
}
impl ::core::default::Default for OEM_DMEXTRAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OEM_DMEXTRAHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwSignature == other.dwSignature && self.dwVersion == other.dwVersion
    }
}
impl ::core::cmp::Eq for OEM_DMEXTRAHEADER {}
impl ::core::fmt::Debug for OEM_DMEXTRAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OEM_DMEXTRAHEADER").field("dwSize", &self.dwSize).field("dwSignature", &self.dwSignature).field("dwVersion", &self.dwVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OIEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OIEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.hInstCaller == other.hInstCaller && self.pHelpFile == other.pHelpFile && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OIEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OIEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OIEXT").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("hInstCaller", &self.hInstCaller).field("pHelpFile", &self.pHelpFile).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPTCOMBO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPTCOMBO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.cListItem == other.cListItem && self.pListItem == other.pListItem && self.Sel == other.Sel && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPTCOMBO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPTCOMBO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPTCOMBO").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("cListItem", &self.cListItem).field("pListItem", &self.pListItem).field("Sel", &self.Sel).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPTPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPTPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.Style == other.Style && self.pData == other.pData && self.IconID == other.IconID && self.lParam == other.lParam && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPTPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPTPARAM").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("Style", &self.Style).field("pData", &self.pData).field("IconID", &self.IconID).field("lParam", &self.lParam).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPTTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPTTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Type == other.Type && self.Flags == other.Flags && self.Count == other.Count && self.BegCtrlID == other.BegCtrlID && self.pOptParam == other.pOptParam && self.Style == other.Style && self.wReserved == other.wReserved && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPTTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPTTYPE").field("cbSize", &self.cbSize).field("Type", &self.Type).field("Flags", &self.Flags).field("Count", &self.Count).field("BegCtrlID", &self.BegCtrlID).field("pOptParam", &self.pOptParam).field("Style", &self.Style).field("wReserved", &self.wReserved).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for PORT_DATA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_DATA_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sztPortName == other.sztPortName && self.dwVersion == other.dwVersion && self.dwProtocol == other.dwProtocol && self.cbSize == other.cbSize && self.dwReserved == other.dwReserved && self.sztHostAddress == other.sztHostAddress && self.sztSNMPCommunity == other.sztSNMPCommunity && self.dwDoubleSpool == other.dwDoubleSpool && self.sztQueue == other.sztQueue && self.sztIPAddress == other.sztIPAddress && self.Reserved == other.Reserved && self.dwPortNumber == other.dwPortNumber && self.dwSNMPEnabled == other.dwSNMPEnabled && self.dwSNMPDevIndex == other.dwSNMPDevIndex
    }
}
impl ::core::cmp::Eq for PORT_DATA_1 {}
impl ::core::fmt::Debug for PORT_DATA_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_DATA_1")
            .field("sztPortName", &self.sztPortName)
            .field("dwVersion", &self.dwVersion)
            .field("dwProtocol", &self.dwProtocol)
            .field("cbSize", &self.cbSize)
            .field("dwReserved", &self.dwReserved)
            .field("sztHostAddress", &self.sztHostAddress)
            .field("sztSNMPCommunity", &self.sztSNMPCommunity)
            .field("dwDoubleSpool", &self.dwDoubleSpool)
            .field("sztQueue", &self.sztQueue)
            .field("sztIPAddress", &self.sztIPAddress)
            .field("Reserved", &self.Reserved)
            .field("dwPortNumber", &self.dwPortNumber)
            .field("dwSNMPEnabled", &self.dwSNMPEnabled)
            .field("dwSNMPDevIndex", &self.dwSNMPDevIndex)
            .finish()
    }
}
impl ::core::default::Default for PORT_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.sztPortName == other.sztPortName && self.dwVersion == other.dwVersion && self.dwProtocol == other.dwProtocol && self.cbSize == other.cbSize && self.dwReserved == other.dwReserved && self.sztHostAddress == other.sztHostAddress && self.sztSNMPCommunity == other.sztSNMPCommunity && self.dwDoubleSpool == other.dwDoubleSpool && self.sztQueue == other.sztQueue && self.Reserved == other.Reserved && self.dwPortNumber == other.dwPortNumber && self.dwSNMPEnabled == other.dwSNMPEnabled && self.dwSNMPDevIndex == other.dwSNMPDevIndex && self.dwPortMonitorMibIndex == other.dwPortMonitorMibIndex
    }
}
impl ::core::cmp::Eq for PORT_DATA_2 {}
impl ::core::fmt::Debug for PORT_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_DATA_2")
            .field("sztPortName", &self.sztPortName)
            .field("dwVersion", &self.dwVersion)
            .field("dwProtocol", &self.dwProtocol)
            .field("cbSize", &self.cbSize)
            .field("dwReserved", &self.dwReserved)
            .field("sztHostAddress", &self.sztHostAddress)
            .field("sztSNMPCommunity", &self.sztSNMPCommunity)
            .field("dwDoubleSpool", &self.dwDoubleSpool)
            .field("sztQueue", &self.sztQueue)
            .field("Reserved", &self.Reserved)
            .field("dwPortNumber", &self.dwPortNumber)
            .field("dwSNMPEnabled", &self.dwSNMPEnabled)
            .field("dwSNMPDevIndex", &self.dwSNMPDevIndex)
            .field("dwPortMonitorMibIndex", &self.dwPortMonitorMibIndex)
            .finish()
    }
}
impl ::core::default::Default for PORT_DATA_LIST_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_DATA_LIST_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cPortData == other.cPortData && self.pPortData == other.pPortData
    }
}
impl ::core::cmp::Eq for PORT_DATA_LIST_1 {}
impl ::core::fmt::Debug for PORT_DATA_LIST_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_DATA_LIST_1").field("dwVersion", &self.dwVersion).field("cPortData", &self.cPortData).field("pPortData", &self.pPortData).finish()
    }
}
impl ::core::default::Default for PORT_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for PORT_INFO_1A {}
impl ::core::fmt::Debug for PORT_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_1A").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for PORT_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for PORT_INFO_1W {}
impl ::core::fmt::Debug for PORT_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_1W").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for PORT_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.pPortName == other.pPortName && self.pMonitorName == other.pMonitorName && self.pDescription == other.pDescription && self.fPortType == other.fPortType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PORT_INFO_2A {}
impl ::core::fmt::Debug for PORT_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_2A").field("pPortName", &self.pPortName).field("pMonitorName", &self.pMonitorName).field("pDescription", &self.pDescription).field("fPortType", &self.fPortType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for PORT_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.pPortName == other.pPortName && self.pMonitorName == other.pMonitorName && self.pDescription == other.pDescription && self.fPortType == other.fPortType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PORT_INFO_2W {}
impl ::core::fmt::Debug for PORT_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_2W").field("pPortName", &self.pPortName).field("pMonitorName", &self.pMonitorName).field("pDescription", &self.pDescription).field("fPortType", &self.fPortType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for PORT_INFO_3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_3A {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.pszStatus == other.pszStatus && self.dwSeverity == other.dwSeverity
    }
}
impl ::core::cmp::Eq for PORT_INFO_3A {}
impl ::core::fmt::Debug for PORT_INFO_3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_3A").field("dwStatus", &self.dwStatus).field("pszStatus", &self.pszStatus).field("dwSeverity", &self.dwSeverity).finish()
    }
}
impl ::core::default::Default for PORT_INFO_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PORT_INFO_3W {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.pszStatus == other.pszStatus && self.dwSeverity == other.dwSeverity
    }
}
impl ::core::cmp::Eq for PORT_INFO_3W {}
impl ::core::fmt::Debug for PORT_INFO_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PORT_INFO_3W").field("dwStatus", &self.dwStatus).field("pszStatus", &self.pszStatus).field("dwSeverity", &self.dwSeverity).finish()
    }
}
impl ::core::default::Default for PRINTER_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRINTER_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTER_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRINTER_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRINTER_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PRINTER_CONNECTION_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_CONNECTION_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pszDriverName == other.pszDriverName
    }
}
impl ::core::cmp::Eq for PRINTER_CONNECTION_INFO_1A {}
impl ::core::fmt::Debug for PRINTER_CONNECTION_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_CONNECTION_INFO_1A").field("dwFlags", &self.dwFlags).field("pszDriverName", &self.pszDriverName).finish()
    }
}
impl ::core::default::Default for PRINTER_CONNECTION_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_CONNECTION_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pszDriverName == other.pszDriverName
    }
}
impl ::core::cmp::Eq for PRINTER_CONNECTION_INFO_1W {}
impl ::core::fmt::Debug for PRINTER_CONNECTION_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_CONNECTION_INFO_1W").field("dwFlags", &self.dwFlags).field("pszDriverName", &self.pszDriverName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_DEFAULTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_DEFAULTSA {
    fn eq(&self, other: &Self) -> bool {
        self.pDatatype == other.pDatatype && self.pDevMode == other.pDevMode && self.DesiredAccess == other.DesiredAccess
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_DEFAULTSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_DEFAULTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_DEFAULTSA").field("pDatatype", &self.pDatatype).field("pDevMode", &self.pDevMode).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_DEFAULTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_DEFAULTSW {
    fn eq(&self, other: &Self) -> bool {
        self.pDatatype == other.pDatatype && self.pDevMode == other.pDevMode && self.DesiredAccess == other.DesiredAccess
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_DEFAULTSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_DEFAULTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_DEFAULTSW").field("pDatatype", &self.pDatatype).field("pDevMode", &self.pDevMode).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
impl ::core::default::Default for PRINTER_ENUM_VALUESA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_ENUM_VALUESA {
    fn eq(&self, other: &Self) -> bool {
        self.pValueName == other.pValueName && self.cbValueName == other.cbValueName && self.dwType == other.dwType && self.pData == other.pData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for PRINTER_ENUM_VALUESA {}
impl ::core::fmt::Debug for PRINTER_ENUM_VALUESA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_ENUM_VALUESA").field("pValueName", &self.pValueName).field("cbValueName", &self.cbValueName).field("dwType", &self.dwType).field("pData", &self.pData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::default::Default for PRINTER_ENUM_VALUESW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_ENUM_VALUESW {
    fn eq(&self, other: &Self) -> bool {
        self.pValueName == other.pValueName && self.cbValueName == other.cbValueName && self.dwType == other.dwType && self.pData == other.pData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for PRINTER_ENUM_VALUESW {}
impl ::core::fmt::Debug for PRINTER_ENUM_VALUESW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_ENUM_VALUESW").field("pValueName", &self.pValueName).field("cbValueName", &self.cbValueName).field("dwType", &self.dwType).field("pData", &self.pData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::default::Default for PRINTER_EVENT_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_EVENT_ATTRIBUTES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwOldAttributes == other.dwOldAttributes && self.dwNewAttributes == other.dwNewAttributes
    }
}
impl ::core::cmp::Eq for PRINTER_EVENT_ATTRIBUTES_INFO {}
impl ::core::fmt::Debug for PRINTER_EVENT_ATTRIBUTES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_EVENT_ATTRIBUTES_INFO").field("cbSize", &self.cbSize).field("dwOldAttributes", &self.dwOldAttributes).field("dwNewAttributes", &self.dwNewAttributes).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pDescription == other.pDescription && self.pName == other.pName && self.pComment == other.pComment
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_1A {}
impl ::core::fmt::Debug for PRINTER_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_1A").field("Flags", &self.Flags).field("pDescription", &self.pDescription).field("pName", &self.pName).field("pComment", &self.pComment).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pDescription == other.pDescription && self.pName == other.pName && self.pComment == other.pComment
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_1W {}
impl ::core::fmt::Debug for PRINTER_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_1W").field("Flags", &self.Flags).field("pDescription", &self.pDescription).field("pName", &self.pName).field("pComment", &self.pComment).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for PRINTER_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
            && self.pPrinterName == other.pPrinterName
            && self.pShareName == other.pShareName
            && self.pPortName == other.pPortName
            && self.pDriverName == other.pDriverName
            && self.pComment == other.pComment
            && self.pLocation == other.pLocation
            && self.pDevMode == other.pDevMode
            && self.pSepFile == other.pSepFile
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pDatatype == other.pDatatype
            && self.pParameters == other.pParameters
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Attributes == other.Attributes
            && self.Priority == other.Priority
            && self.DefaultPriority == other.DefaultPriority
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.Status == other.Status
            && self.cJobs == other.cJobs
            && self.AveragePPM == other.AveragePPM
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for PRINTER_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for PRINTER_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_2A")
            .field("pServerName", &self.pServerName)
            .field("pPrinterName", &self.pPrinterName)
            .field("pShareName", &self.pShareName)
            .field("pPortName", &self.pPortName)
            .field("pDriverName", &self.pDriverName)
            .field("pComment", &self.pComment)
            .field("pLocation", &self.pLocation)
            .field("pDevMode", &self.pDevMode)
            .field("pSepFile", &self.pSepFile)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pDatatype", &self.pDatatype)
            .field("pParameters", &self.pParameters)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Attributes", &self.Attributes)
            .field("Priority", &self.Priority)
            .field("DefaultPriority", &self.DefaultPriority)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("Status", &self.Status)
            .field("cJobs", &self.cJobs)
            .field("AveragePPM", &self.AveragePPM)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::default::Default for PRINTER_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
            && self.pPrinterName == other.pPrinterName
            && self.pShareName == other.pShareName
            && self.pPortName == other.pPortName
            && self.pDriverName == other.pDriverName
            && self.pComment == other.pComment
            && self.pLocation == other.pLocation
            && self.pDevMode == other.pDevMode
            && self.pSepFile == other.pSepFile
            && self.pPrintProcessor == other.pPrintProcessor
            && self.pDatatype == other.pDatatype
            && self.pParameters == other.pParameters
            && self.pSecurityDescriptor == other.pSecurityDescriptor
            && self.Attributes == other.Attributes
            && self.Priority == other.Priority
            && self.DefaultPriority == other.DefaultPriority
            && self.StartTime == other.StartTime
            && self.UntilTime == other.UntilTime
            && self.Status == other.Status
            && self.cJobs == other.cJobs
            && self.AveragePPM == other.AveragePPM
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::cmp::Eq for PRINTER_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::fmt::Debug for PRINTER_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_2W")
            .field("pServerName", &self.pServerName)
            .field("pPrinterName", &self.pPrinterName)
            .field("pShareName", &self.pShareName)
            .field("pPortName", &self.pPortName)
            .field("pDriverName", &self.pDriverName)
            .field("pComment", &self.pComment)
            .field("pLocation", &self.pLocation)
            .field("pDevMode", &self.pDevMode)
            .field("pSepFile", &self.pSepFile)
            .field("pPrintProcessor", &self.pPrintProcessor)
            .field("pDatatype", &self.pDatatype)
            .field("pParameters", &self.pParameters)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .field("Attributes", &self.Attributes)
            .field("Priority", &self.Priority)
            .field("DefaultPriority", &self.DefaultPriority)
            .field("StartTime", &self.StartTime)
            .field("UntilTime", &self.UntilTime)
            .field("Status", &self.Status)
            .field("cJobs", &self.cJobs)
            .field("AveragePPM", &self.AveragePPM)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for PRINTER_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for PRINTER_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for PRINTER_INFO_3 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for PRINTER_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_3").field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_4A {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pServerName == other.pServerName && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_4A {}
impl ::core::fmt::Debug for PRINTER_INFO_4A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_4A").field("pPrinterName", &self.pPrinterName).field("pServerName", &self.pServerName).field("Attributes", &self.Attributes).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_4W {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pServerName == other.pServerName && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_4W {}
impl ::core::fmt::Debug for PRINTER_INFO_4W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_4W").field("pPrinterName", &self.pPrinterName).field("pServerName", &self.pServerName).field("Attributes", &self.Attributes).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_5A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_5A {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pPortName == other.pPortName && self.Attributes == other.Attributes && self.DeviceNotSelectedTimeout == other.DeviceNotSelectedTimeout && self.TransmissionRetryTimeout == other.TransmissionRetryTimeout
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_5A {}
impl ::core::fmt::Debug for PRINTER_INFO_5A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_5A").field("pPrinterName", &self.pPrinterName).field("pPortName", &self.pPortName).field("Attributes", &self.Attributes).field("DeviceNotSelectedTimeout", &self.DeviceNotSelectedTimeout).field("TransmissionRetryTimeout", &self.TransmissionRetryTimeout).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_5W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_5W {
    fn eq(&self, other: &Self) -> bool {
        self.pPrinterName == other.pPrinterName && self.pPortName == other.pPortName && self.Attributes == other.Attributes && self.DeviceNotSelectedTimeout == other.DeviceNotSelectedTimeout && self.TransmissionRetryTimeout == other.TransmissionRetryTimeout
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_5W {}
impl ::core::fmt::Debug for PRINTER_INFO_5W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_5W").field("pPrinterName", &self.pPrinterName).field("pPortName", &self.pPortName).field("Attributes", &self.Attributes).field("DeviceNotSelectedTimeout", &self.DeviceNotSelectedTimeout).field("TransmissionRetryTimeout", &self.TransmissionRetryTimeout).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_6 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_6 {}
impl ::core::fmt::Debug for PRINTER_INFO_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_6").field("dwStatus", &self.dwStatus).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_7A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_7A {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjectGUID == other.pszObjectGUID && self.dwAction == other.dwAction
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_7A {}
impl ::core::fmt::Debug for PRINTER_INFO_7A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_7A").field("pszObjectGUID", &self.pszObjectGUID).field("dwAction", &self.dwAction).finish()
    }
}
impl ::core::default::Default for PRINTER_INFO_7W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_INFO_7W {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjectGUID == other.pszObjectGUID && self.dwAction == other.dwAction
    }
}
impl ::core::cmp::Eq for PRINTER_INFO_7W {}
impl ::core::fmt::Debug for PRINTER_INFO_7W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_7W").field("pszObjectGUID", &self.pszObjectGUID).field("dwAction", &self.dwAction).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_INFO_8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_8A {
    fn eq(&self, other: &Self) -> bool {
        self.pDevMode == other.pDevMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_INFO_8A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_INFO_8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_8A").field("pDevMode", &self.pDevMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_INFO_8W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_8W {
    fn eq(&self, other: &Self) -> bool {
        self.pDevMode == other.pDevMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_INFO_8W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_INFO_8W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_8W").field("pDevMode", &self.pDevMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_INFO_9A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_9A {
    fn eq(&self, other: &Self) -> bool {
        self.pDevMode == other.pDevMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_INFO_9A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_INFO_9A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_9A").field("pDevMode", &self.pDevMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTER_INFO_9W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTER_INFO_9W {
    fn eq(&self, other: &Self) -> bool {
        self.pDevMode == other.pDevMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTER_INFO_9W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTER_INFO_9W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_INFO_9W").field("pDevMode", &self.pDevMode).finish()
    }
}
impl ::core::default::Default for PRINTER_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PRINTER_NOTIFY_INFO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PRINTER_NOTIFY_INIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_NOTIFY_INIT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Reserved == other.Reserved && self.PollTime == other.PollTime
    }
}
impl ::core::cmp::Eq for PRINTER_NOTIFY_INIT {}
impl ::core::fmt::Debug for PRINTER_NOTIFY_INIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_NOTIFY_INIT").field("Size", &self.Size).field("Reserved", &self.Reserved).field("PollTime", &self.PollTime).finish()
    }
}
impl ::core::default::Default for PRINTER_NOTIFY_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_NOTIFY_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Count == other.Count && self.pTypes == other.pTypes
    }
}
impl ::core::cmp::Eq for PRINTER_NOTIFY_OPTIONS {}
impl ::core::fmt::Debug for PRINTER_NOTIFY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_NOTIFY_OPTIONS").field("Version", &self.Version).field("Flags", &self.Flags).field("Count", &self.Count).field("pTypes", &self.pTypes).finish()
    }
}
impl ::core::default::Default for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Count == other.Count && self.pFields == other.pFields
    }
}
impl ::core::cmp::Eq for PRINTER_NOTIFY_OPTIONS_TYPE {}
impl ::core::fmt::Debug for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_NOTIFY_OPTIONS_TYPE").field("Type", &self.Type).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Count", &self.Count).field("pFields", &self.pFields).finish()
    }
}
impl ::core::default::Default for PRINTER_OPTIONSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_OPTIONSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for PRINTER_OPTIONSA {}
impl ::core::fmt::Debug for PRINTER_OPTIONSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_OPTIONSA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for PRINTER_OPTIONSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTER_OPTIONSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for PRINTER_OPTIONSW {}
impl ::core::fmt::Debug for PRINTER_OPTIONSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTER_OPTIONSW").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for PRINTER_OPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRINTER_OPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTER_OPTION_FLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTIFI32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTIFI32 {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis
            && self.cjIfiExtra == other.cjIfiExtra
            && self.dpwszFamilyName == other.dpwszFamilyName
            && self.dpwszStyleName == other.dpwszStyleName
            && self.dpwszFaceName == other.dpwszFaceName
            && self.dpwszUniqueName == other.dpwszUniqueName
            && self.dpFontSim == other.dpFontSim
            && self.lEmbedId == other.lEmbedId
            && self.lItalicAngle == other.lItalicAngle
            && self.lCharBias == other.lCharBias
            && self.dpCharSets == other.dpCharSets
            && self.jWinCharSet == other.jWinCharSet
            && self.jWinPitchAndFamily == other.jWinPitchAndFamily
            && self.usWinWeight == other.usWinWeight
            && self.flInfo == other.flInfo
            && self.fsSelection == other.fsSelection
            && self.fsType == other.fsType
            && self.fwdUnitsPerEm == other.fwdUnitsPerEm
            && self.fwdLowestPPEm == other.fwdLowestPPEm
            && self.fwdWinAscender == other.fwdWinAscender
            && self.fwdWinDescender == other.fwdWinDescender
            && self.fwdMacAscender == other.fwdMacAscender
            && self.fwdMacDescender == other.fwdMacDescender
            && self.fwdMacLineGap == other.fwdMacLineGap
            && self.fwdTypoAscender == other.fwdTypoAscender
            && self.fwdTypoDescender == other.fwdTypoDescender
            && self.fwdTypoLineGap == other.fwdTypoLineGap
            && self.fwdAveCharWidth == other.fwdAveCharWidth
            && self.fwdMaxCharInc == other.fwdMaxCharInc
            && self.fwdCapHeight == other.fwdCapHeight
            && self.fwdXHeight == other.fwdXHeight
            && self.fwdSubscriptXSize == other.fwdSubscriptXSize
            && self.fwdSubscriptYSize == other.fwdSubscriptYSize
            && self.fwdSubscriptXOffset == other.fwdSubscriptXOffset
            && self.fwdSubscriptYOffset == other.fwdSubscriptYOffset
            && self.fwdSuperscriptXSize == other.fwdSuperscriptXSize
            && self.fwdSuperscriptYSize == other.fwdSuperscriptYSize
            && self.fwdSuperscriptXOffset == other.fwdSuperscriptXOffset
            && self.fwdSuperscriptYOffset == other.fwdSuperscriptYOffset
            && self.fwdUnderscoreSize == other.fwdUnderscoreSize
            && self.fwdUnderscorePosition == other.fwdUnderscorePosition
            && self.fwdStrikeoutSize == other.fwdStrikeoutSize
            && self.fwdStrikeoutPosition == other.fwdStrikeoutPosition
            && self.chFirstChar == other.chFirstChar
            && self.chLastChar == other.chLastChar
            && self.chDefaultChar == other.chDefaultChar
            && self.chBreakChar == other.chBreakChar
            && self.wcFirstChar == other.wcFirstChar
            && self.wcLastChar == other.wcLastChar
            && self.wcDefaultChar == other.wcDefaultChar
            && self.wcBreakChar == other.wcBreakChar
            && self.ptlBaseline == other.ptlBaseline
            && self.ptlAspect == other.ptlAspect
            && self.ptlCaret == other.ptlCaret
            && self.rclFontBox == other.rclFontBox
            && self.achVendId == other.achVendId
            && self.cKerningPairs == other.cKerningPairs
            && self.ulPanoseCulture == other.ulPanoseCulture
            && self.panose == other.panose
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTIFI32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTIFI32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTIFI32")
            .field("cjThis", &self.cjThis)
            .field("cjIfiExtra", &self.cjIfiExtra)
            .field("dpwszFamilyName", &self.dpwszFamilyName)
            .field("dpwszStyleName", &self.dpwszStyleName)
            .field("dpwszFaceName", &self.dpwszFaceName)
            .field("dpwszUniqueName", &self.dpwszUniqueName)
            .field("dpFontSim", &self.dpFontSim)
            .field("lEmbedId", &self.lEmbedId)
            .field("lItalicAngle", &self.lItalicAngle)
            .field("lCharBias", &self.lCharBias)
            .field("dpCharSets", &self.dpCharSets)
            .field("jWinCharSet", &self.jWinCharSet)
            .field("jWinPitchAndFamily", &self.jWinPitchAndFamily)
            .field("usWinWeight", &self.usWinWeight)
            .field("flInfo", &self.flInfo)
            .field("fsSelection", &self.fsSelection)
            .field("fsType", &self.fsType)
            .field("fwdUnitsPerEm", &self.fwdUnitsPerEm)
            .field("fwdLowestPPEm", &self.fwdLowestPPEm)
            .field("fwdWinAscender", &self.fwdWinAscender)
            .field("fwdWinDescender", &self.fwdWinDescender)
            .field("fwdMacAscender", &self.fwdMacAscender)
            .field("fwdMacDescender", &self.fwdMacDescender)
            .field("fwdMacLineGap", &self.fwdMacLineGap)
            .field("fwdTypoAscender", &self.fwdTypoAscender)
            .field("fwdTypoDescender", &self.fwdTypoDescender)
            .field("fwdTypoLineGap", &self.fwdTypoLineGap)
            .field("fwdAveCharWidth", &self.fwdAveCharWidth)
            .field("fwdMaxCharInc", &self.fwdMaxCharInc)
            .field("fwdCapHeight", &self.fwdCapHeight)
            .field("fwdXHeight", &self.fwdXHeight)
            .field("fwdSubscriptXSize", &self.fwdSubscriptXSize)
            .field("fwdSubscriptYSize", &self.fwdSubscriptYSize)
            .field("fwdSubscriptXOffset", &self.fwdSubscriptXOffset)
            .field("fwdSubscriptYOffset", &self.fwdSubscriptYOffset)
            .field("fwdSuperscriptXSize", &self.fwdSuperscriptXSize)
            .field("fwdSuperscriptYSize", &self.fwdSuperscriptYSize)
            .field("fwdSuperscriptXOffset", &self.fwdSuperscriptXOffset)
            .field("fwdSuperscriptYOffset", &self.fwdSuperscriptYOffset)
            .field("fwdUnderscoreSize", &self.fwdUnderscoreSize)
            .field("fwdUnderscorePosition", &self.fwdUnderscorePosition)
            .field("fwdStrikeoutSize", &self.fwdStrikeoutSize)
            .field("fwdStrikeoutPosition", &self.fwdStrikeoutPosition)
            .field("chFirstChar", &self.chFirstChar)
            .field("chLastChar", &self.chLastChar)
            .field("chDefaultChar", &self.chDefaultChar)
            .field("chBreakChar", &self.chBreakChar)
            .field("wcFirstChar", &self.wcFirstChar)
            .field("wcLastChar", &self.wcLastChar)
            .field("wcDefaultChar", &self.wcDefaultChar)
            .field("wcBreakChar", &self.wcBreakChar)
            .field("ptlBaseline", &self.ptlBaseline)
            .field("ptlAspect", &self.ptlAspect)
            .field("ptlCaret", &self.ptlCaret)
            .field("rclFontBox", &self.rclFontBox)
            .field("achVendId", &self.achVendId)
            .field("cKerningPairs", &self.cKerningPairs)
            .field("ulPanoseCulture", &self.ulPanoseCulture)
            .field("panose", &self.panose)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTPROCESSOROPENDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTPROCESSOROPENDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pDevMode == other.pDevMode && self.pDatatype == other.pDatatype && self.pParameters == other.pParameters && self.pDocumentName == other.pDocumentName && self.JobId == other.JobId && self.pOutputFile == other.pOutputFile && self.pPrinterName == other.pPrinterName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTPROCESSOROPENDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTPROCESSOROPENDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROCESSOROPENDATA").field("pDevMode", &self.pDevMode).field("pDatatype", &self.pDatatype).field("pParameters", &self.pParameters).field("pDocumentName", &self.pDocumentName).field("JobId", &self.JobId).field("pOutputFile", &self.pOutputFile).field("pPrinterName", &self.pPrinterName).finish()
    }
}
impl ::core::default::Default for PRINTPROCESSOR_CAPS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTPROCESSOR_CAPS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwLevel == other.dwLevel && self.dwNupOptions == other.dwNupOptions && self.dwPageOrderFlags == other.dwPageOrderFlags && self.dwNumberOfCopies == other.dwNumberOfCopies
    }
}
impl ::core::cmp::Eq for PRINTPROCESSOR_CAPS_1 {}
impl ::core::fmt::Debug for PRINTPROCESSOR_CAPS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROCESSOR_CAPS_1").field("dwLevel", &self.dwLevel).field("dwNupOptions", &self.dwNupOptions).field("dwPageOrderFlags", &self.dwPageOrderFlags).field("dwNumberOfCopies", &self.dwNumberOfCopies).finish()
    }
}
impl ::core::default::Default for PRINTPROCESSOR_CAPS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTPROCESSOR_CAPS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwLevel == other.dwLevel && self.dwNupOptions == other.dwNupOptions && self.dwPageOrderFlags == other.dwPageOrderFlags && self.dwNumberOfCopies == other.dwNumberOfCopies && self.dwDuplexHandlingCaps == other.dwDuplexHandlingCaps && self.dwNupDirectionCaps == other.dwNupDirectionCaps && self.dwNupBorderCaps == other.dwNupBorderCaps && self.dwBookletHandlingCaps == other.dwBookletHandlingCaps && self.dwScalingCaps == other.dwScalingCaps
    }
}
impl ::core::cmp::Eq for PRINTPROCESSOR_CAPS_2 {}
impl ::core::fmt::Debug for PRINTPROCESSOR_CAPS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROCESSOR_CAPS_2")
            .field("dwLevel", &self.dwLevel)
            .field("dwNupOptions", &self.dwNupOptions)
            .field("dwPageOrderFlags", &self.dwPageOrderFlags)
            .field("dwNumberOfCopies", &self.dwNumberOfCopies)
            .field("dwDuplexHandlingCaps", &self.dwDuplexHandlingCaps)
            .field("dwNupDirectionCaps", &self.dwNupDirectionCaps)
            .field("dwNupBorderCaps", &self.dwNupBorderCaps)
            .field("dwBookletHandlingCaps", &self.dwBookletHandlingCaps)
            .field("dwScalingCaps", &self.dwScalingCaps)
            .finish()
    }
}
impl ::core::default::Default for PRINTPROCESSOR_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTPROCESSOR_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for PRINTPROCESSOR_INFO_1A {}
impl ::core::fmt::Debug for PRINTPROCESSOR_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROCESSOR_INFO_1A").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for PRINTPROCESSOR_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTPROCESSOR_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName
    }
}
impl ::core::cmp::Eq for PRINTPROCESSOR_INFO_1W {}
impl ::core::fmt::Debug for PRINTPROCESSOR_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROCESSOR_INFO_1W").field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for PRINTPROVIDOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINTPROVIDOR {
    fn eq(&self, other: &Self) -> bool {
        self.fpOpenPrinter == other.fpOpenPrinter
            && self.fpSetJob == other.fpSetJob
            && self.fpGetJob == other.fpGetJob
            && self.fpEnumJobs == other.fpEnumJobs
            && self.fpAddPrinter == other.fpAddPrinter
            && self.fpDeletePrinter == other.fpDeletePrinter
            && self.fpSetPrinter == other.fpSetPrinter
            && self.fpGetPrinter == other.fpGetPrinter
            && self.fpEnumPrinters == other.fpEnumPrinters
            && self.fpAddPrinterDriver == other.fpAddPrinterDriver
            && self.fpEnumPrinterDrivers == other.fpEnumPrinterDrivers
            && self.fpGetPrinterDriver == other.fpGetPrinterDriver
            && self.fpGetPrinterDriverDirectory == other.fpGetPrinterDriverDirectory
            && self.fpDeletePrinterDriver == other.fpDeletePrinterDriver
            && self.fpAddPrintProcessor == other.fpAddPrintProcessor
            && self.fpEnumPrintProcessors == other.fpEnumPrintProcessors
            && self.fpGetPrintProcessorDirectory == other.fpGetPrintProcessorDirectory
            && self.fpDeletePrintProcessor == other.fpDeletePrintProcessor
            && self.fpEnumPrintProcessorDatatypes == other.fpEnumPrintProcessorDatatypes
            && self.fpStartDocPrinter == other.fpStartDocPrinter
            && self.fpStartPagePrinter == other.fpStartPagePrinter
            && self.fpWritePrinter == other.fpWritePrinter
            && self.fpEndPagePrinter == other.fpEndPagePrinter
            && self.fpAbortPrinter == other.fpAbortPrinter
            && self.fpReadPrinter == other.fpReadPrinter
            && self.fpEndDocPrinter == other.fpEndDocPrinter
            && self.fpAddJob == other.fpAddJob
            && self.fpScheduleJob == other.fpScheduleJob
            && self.fpGetPrinterData == other.fpGetPrinterData
            && self.fpSetPrinterData == other.fpSetPrinterData
            && self.fpWaitForPrinterChange == other.fpWaitForPrinterChange
            && self.fpClosePrinter == other.fpClosePrinter
            && self.fpAddForm == other.fpAddForm
            && self.fpDeleteForm == other.fpDeleteForm
            && self.fpGetForm == other.fpGetForm
            && self.fpSetForm == other.fpSetForm
            && self.fpEnumForms == other.fpEnumForms
            && self.fpEnumMonitors == other.fpEnumMonitors
            && self.fpEnumPorts == other.fpEnumPorts
            && self.fpAddPort == other.fpAddPort
            && self.fpConfigurePort == other.fpConfigurePort
            && self.fpDeletePort == other.fpDeletePort
            && self.fpCreatePrinterIC == other.fpCreatePrinterIC
            && self.fpPlayGdiScriptOnPrinterIC == other.fpPlayGdiScriptOnPrinterIC
            && self.fpDeletePrinterIC == other.fpDeletePrinterIC
            && self.fpAddPrinterConnection == other.fpAddPrinterConnection
            && self.fpDeletePrinterConnection == other.fpDeletePrinterConnection
            && self.fpPrinterMessageBox == other.fpPrinterMessageBox
            && self.fpAddMonitor == other.fpAddMonitor
            && self.fpDeleteMonitor == other.fpDeleteMonitor
            && self.fpResetPrinter == other.fpResetPrinter
            && self.fpGetPrinterDriverEx == other.fpGetPrinterDriverEx
            && self.fpFindFirstPrinterChangeNotification == other.fpFindFirstPrinterChangeNotification
            && self.fpFindClosePrinterChangeNotification == other.fpFindClosePrinterChangeNotification
            && self.fpAddPortEx == other.fpAddPortEx
            && self.fpShutDown == other.fpShutDown
            && self.fpRefreshPrinterChangeNotification == other.fpRefreshPrinterChangeNotification
            && self.fpOpenPrinterEx == other.fpOpenPrinterEx
            && self.fpAddPrinterEx == other.fpAddPrinterEx
            && self.fpSetPort == other.fpSetPort
            && self.fpEnumPrinterData == other.fpEnumPrinterData
            && self.fpDeletePrinterData == other.fpDeletePrinterData
            && self.fpClusterSplOpen == other.fpClusterSplOpen
            && self.fpClusterSplClose == other.fpClusterSplClose
            && self.fpClusterSplIsAlive == other.fpClusterSplIsAlive
            && self.fpSetPrinterDataEx == other.fpSetPrinterDataEx
            && self.fpGetPrinterDataEx == other.fpGetPrinterDataEx
            && self.fpEnumPrinterDataEx == other.fpEnumPrinterDataEx
            && self.fpEnumPrinterKey == other.fpEnumPrinterKey
            && self.fpDeletePrinterDataEx == other.fpDeletePrinterDataEx
            && self.fpDeletePrinterKey == other.fpDeletePrinterKey
            && self.fpSeekPrinter == other.fpSeekPrinter
            && self.fpDeletePrinterDriverEx == other.fpDeletePrinterDriverEx
            && self.fpAddPerMachineConnection == other.fpAddPerMachineConnection
            && self.fpDeletePerMachineConnection == other.fpDeletePerMachineConnection
            && self.fpEnumPerMachineConnections == other.fpEnumPerMachineConnections
            && self.fpXcvData == other.fpXcvData
            && self.fpAddPrinterDriverEx == other.fpAddPrinterDriverEx
            && self.fpSplReadPrinter == other.fpSplReadPrinter
            && self.fpDriverUnloadComplete == other.fpDriverUnloadComplete
            && self.fpGetSpoolFileInfo == other.fpGetSpoolFileInfo
            && self.fpCommitSpoolData == other.fpCommitSpoolData
            && self.fpCloseSpoolFileHandle == other.fpCloseSpoolFileHandle
            && self.fpFlushPrinter == other.fpFlushPrinter
            && self.fpSendRecvBidiData == other.fpSendRecvBidiData
            && self.fpAddPrinterConnection2 == other.fpAddPrinterConnection2
            && self.fpGetPrintClassObject == other.fpGetPrintClassObject
            && self.fpReportJobProcessingProgress == other.fpReportJobProcessingProgress
            && self.fpEnumAndLogProvidorObjects == other.fpEnumAndLogProvidorObjects
            && self.fpInternalGetPrinterDriver == other.fpInternalGetPrinterDriver
            && self.fpFindCompatibleDriver == other.fpFindCompatibleDriver
            && self.fpGetJobNamedPropertyValue == other.fpGetJobNamedPropertyValue
            && self.fpSetJobNamedProperty == other.fpSetJobNamedProperty
            && self.fpDeleteJobNamedProperty == other.fpDeleteJobNamedProperty
            && self.fpEnumJobNamedProperties == other.fpEnumJobNamedProperties
            && self.fpPowerEvent == other.fpPowerEvent
            && self.fpGetUserPropertyBag == other.fpGetUserPropertyBag
            && self.fpCanShutdown == other.fpCanShutdown
            && self.fpLogJobInfoForBranchOffice == other.fpLogJobInfoForBranchOffice
            && self.fpRegeneratePrintDeviceCapabilities == other.fpRegeneratePrintDeviceCapabilities
            && self.fpPrintSupportOperation == other.fpPrintSupportOperation
            && self.fpIppCreateJobOnPrinter == other.fpIppCreateJobOnPrinter
            && self.fpIppGetJobAttributes == other.fpIppGetJobAttributes
            && self.fpIppSetJobAttributes == other.fpIppSetJobAttributes
            && self.fpIppGetPrinterAttributes == other.fpIppGetPrinterAttributes
            && self.fpIppSetPrinterAttributes == other.fpIppSetPrinterAttributes
    }
}
impl ::core::cmp::Eq for PRINTPROVIDOR {}
impl ::core::fmt::Debug for PRINTPROVIDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPROVIDOR")
            .field("fpOpenPrinter", &self.fpOpenPrinter)
            .field("fpSetJob", &self.fpSetJob)
            .field("fpGetJob", &self.fpGetJob)
            .field("fpEnumJobs", &self.fpEnumJobs)
            .field("fpAddPrinter", &self.fpAddPrinter)
            .field("fpDeletePrinter", &self.fpDeletePrinter)
            .field("fpSetPrinter", &self.fpSetPrinter)
            .field("fpGetPrinter", &self.fpGetPrinter)
            .field("fpEnumPrinters", &self.fpEnumPrinters)
            .field("fpAddPrinterDriver", &self.fpAddPrinterDriver)
            .field("fpEnumPrinterDrivers", &self.fpEnumPrinterDrivers)
            .field("fpGetPrinterDriver", &self.fpGetPrinterDriver)
            .field("fpGetPrinterDriverDirectory", &self.fpGetPrinterDriverDirectory)
            .field("fpDeletePrinterDriver", &self.fpDeletePrinterDriver)
            .field("fpAddPrintProcessor", &self.fpAddPrintProcessor)
            .field("fpEnumPrintProcessors", &self.fpEnumPrintProcessors)
            .field("fpGetPrintProcessorDirectory", &self.fpGetPrintProcessorDirectory)
            .field("fpDeletePrintProcessor", &self.fpDeletePrintProcessor)
            .field("fpEnumPrintProcessorDatatypes", &self.fpEnumPrintProcessorDatatypes)
            .field("fpStartDocPrinter", &self.fpStartDocPrinter)
            .field("fpStartPagePrinter", &self.fpStartPagePrinter)
            .field("fpWritePrinter", &self.fpWritePrinter)
            .field("fpEndPagePrinter", &self.fpEndPagePrinter)
            .field("fpAbortPrinter", &self.fpAbortPrinter)
            .field("fpReadPrinter", &self.fpReadPrinter)
            .field("fpEndDocPrinter", &self.fpEndDocPrinter)
            .field("fpAddJob", &self.fpAddJob)
            .field("fpScheduleJob", &self.fpScheduleJob)
            .field("fpGetPrinterData", &self.fpGetPrinterData)
            .field("fpSetPrinterData", &self.fpSetPrinterData)
            .field("fpWaitForPrinterChange", &self.fpWaitForPrinterChange)
            .field("fpClosePrinter", &self.fpClosePrinter)
            .field("fpAddForm", &self.fpAddForm)
            .field("fpDeleteForm", &self.fpDeleteForm)
            .field("fpGetForm", &self.fpGetForm)
            .field("fpSetForm", &self.fpSetForm)
            .field("fpEnumForms", &self.fpEnumForms)
            .field("fpEnumMonitors", &self.fpEnumMonitors)
            .field("fpEnumPorts", &self.fpEnumPorts)
            .field("fpAddPort", &self.fpAddPort)
            .field("fpConfigurePort", &self.fpConfigurePort)
            .field("fpDeletePort", &self.fpDeletePort)
            .field("fpCreatePrinterIC", &self.fpCreatePrinterIC)
            .field("fpPlayGdiScriptOnPrinterIC", &self.fpPlayGdiScriptOnPrinterIC)
            .field("fpDeletePrinterIC", &self.fpDeletePrinterIC)
            .field("fpAddPrinterConnection", &self.fpAddPrinterConnection)
            .field("fpDeletePrinterConnection", &self.fpDeletePrinterConnection)
            .field("fpPrinterMessageBox", &self.fpPrinterMessageBox)
            .field("fpAddMonitor", &self.fpAddMonitor)
            .field("fpDeleteMonitor", &self.fpDeleteMonitor)
            .field("fpResetPrinter", &self.fpResetPrinter)
            .field("fpGetPrinterDriverEx", &self.fpGetPrinterDriverEx)
            .field("fpFindFirstPrinterChangeNotification", &self.fpFindFirstPrinterChangeNotification)
            .field("fpFindClosePrinterChangeNotification", &self.fpFindClosePrinterChangeNotification)
            .field("fpAddPortEx", &self.fpAddPortEx)
            .field("fpShutDown", &self.fpShutDown)
            .field("fpRefreshPrinterChangeNotification", &self.fpRefreshPrinterChangeNotification)
            .field("fpOpenPrinterEx", &self.fpOpenPrinterEx)
            .field("fpAddPrinterEx", &self.fpAddPrinterEx)
            .field("fpSetPort", &self.fpSetPort)
            .field("fpEnumPrinterData", &self.fpEnumPrinterData)
            .field("fpDeletePrinterData", &self.fpDeletePrinterData)
            .field("fpClusterSplOpen", &self.fpClusterSplOpen)
            .field("fpClusterSplClose", &self.fpClusterSplClose)
            .field("fpClusterSplIsAlive", &self.fpClusterSplIsAlive)
            .field("fpSetPrinterDataEx", &self.fpSetPrinterDataEx)
            .field("fpGetPrinterDataEx", &self.fpGetPrinterDataEx)
            .field("fpEnumPrinterDataEx", &self.fpEnumPrinterDataEx)
            .field("fpEnumPrinterKey", &self.fpEnumPrinterKey)
            .field("fpDeletePrinterDataEx", &self.fpDeletePrinterDataEx)
            .field("fpDeletePrinterKey", &self.fpDeletePrinterKey)
            .field("fpSeekPrinter", &self.fpSeekPrinter)
            .field("fpDeletePrinterDriverEx", &self.fpDeletePrinterDriverEx)
            .field("fpAddPerMachineConnection", &self.fpAddPerMachineConnection)
            .field("fpDeletePerMachineConnection", &self.fpDeletePerMachineConnection)
            .field("fpEnumPerMachineConnections", &self.fpEnumPerMachineConnections)
            .field("fpXcvData", &self.fpXcvData)
            .field("fpAddPrinterDriverEx", &self.fpAddPrinterDriverEx)
            .field("fpSplReadPrinter", &self.fpSplReadPrinter)
            .field("fpDriverUnloadComplete", &self.fpDriverUnloadComplete)
            .field("fpGetSpoolFileInfo", &self.fpGetSpoolFileInfo)
            .field("fpCommitSpoolData", &self.fpCommitSpoolData)
            .field("fpCloseSpoolFileHandle", &self.fpCloseSpoolFileHandle)
            .field("fpFlushPrinter", &self.fpFlushPrinter)
            .field("fpSendRecvBidiData", &self.fpSendRecvBidiData)
            .field("fpAddPrinterConnection2", &self.fpAddPrinterConnection2)
            .field("fpGetPrintClassObject", &self.fpGetPrintClassObject)
            .field("fpReportJobProcessingProgress", &self.fpReportJobProcessingProgress)
            .field("fpEnumAndLogProvidorObjects", &self.fpEnumAndLogProvidorObjects)
            .field("fpInternalGetPrinterDriver", &self.fpInternalGetPrinterDriver)
            .field("fpFindCompatibleDriver", &self.fpFindCompatibleDriver)
            .field("fpGetJobNamedPropertyValue", &self.fpGetJobNamedPropertyValue)
            .field("fpSetJobNamedProperty", &self.fpSetJobNamedProperty)
            .field("fpDeleteJobNamedProperty", &self.fpDeleteJobNamedProperty)
            .field("fpEnumJobNamedProperties", &self.fpEnumJobNamedProperties)
            .field("fpPowerEvent", &self.fpPowerEvent)
            .field("fpGetUserPropertyBag", &self.fpGetUserPropertyBag)
            .field("fpCanShutdown", &self.fpCanShutdown)
            .field("fpLogJobInfoForBranchOffice", &self.fpLogJobInfoForBranchOffice)
            .field("fpRegeneratePrintDeviceCapabilities", &self.fpRegeneratePrintDeviceCapabilities)
            .field("fpPrintSupportOperation", &self.fpPrintSupportOperation)
            .field("fpIppCreateJobOnPrinter", &self.fpIppCreateJobOnPrinter)
            .field("fpIppGetJobAttributes", &self.fpIppGetJobAttributes)
            .field("fpIppSetJobAttributes", &self.fpIppSetJobAttributes)
            .field("fpIppGetPrinterAttributes", &self.fpIppGetPrinterAttributes)
            .field("fpIppSetPrinterAttributes", &self.fpIppSetPrinterAttributes)
            .finish()
    }
}
impl ::core::default::Default for PRINT_EXECUTION_CONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRINT_EXECUTION_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINT_EXECUTION_CONTEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRINT_EXECUTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINT_EXECUTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context && self.clientAppPID == other.clientAppPID
    }
}
impl ::core::cmp::Eq for PRINT_EXECUTION_DATA {}
impl ::core::fmt::Debug for PRINT_EXECUTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINT_EXECUTION_DATA").field("context", &self.context).field("clientAppPID", &self.clientAppPID).finish()
    }
}
impl ::core::default::Default for PRINT_FEATURE_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINT_FEATURE_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.pszFeature == other.pszFeature && self.pszOption == other.pszOption
    }
}
impl ::core::cmp::Eq for PRINT_FEATURE_OPTION {}
impl ::core::fmt::Debug for PRINT_FEATURE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINT_FEATURE_OPTION").field("pszFeature", &self.pszFeature).field("pszOption", &self.pszOption).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for PROPSHEETUI_GETICON_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for PROPSHEETUI_GETICON_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Flags == other.Flags && self.cxIcon == other.cxIcon && self.cyIcon == other.cyIcon && self.hIcon == other.hIcon
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for PROPSHEETUI_GETICON_INFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for PROPSHEETUI_GETICON_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPSHEETUI_GETICON_INFO").field("cbSize", &self.cbSize).field("Flags", &self.Flags).field("cxIcon", &self.cxIcon).field("cyIcon", &self.cyIcon).field("hIcon", &self.hIcon).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPSHEETUI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETUI_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROVIDOR_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROVIDOR_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDLLName == other.pDLLName
    }
}
impl ::core::cmp::Eq for PROVIDOR_INFO_1A {}
impl ::core::fmt::Debug for PROVIDOR_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDOR_INFO_1A").field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDLLName", &self.pDLLName).finish()
    }
}
impl ::core::default::Default for PROVIDOR_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROVIDOR_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pEnvironment == other.pEnvironment && self.pDLLName == other.pDLLName
    }
}
impl ::core::cmp::Eq for PROVIDOR_INFO_1W {}
impl ::core::fmt::Debug for PROVIDOR_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDOR_INFO_1W").field("pName", &self.pName).field("pEnvironment", &self.pEnvironment).field("pDLLName", &self.pDLLName).finish()
    }
}
impl ::core::default::Default for PROVIDOR_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROVIDOR_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.pOrder == other.pOrder
    }
}
impl ::core::cmp::Eq for PROVIDOR_INFO_2A {}
impl ::core::fmt::Debug for PROVIDOR_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDOR_INFO_2A").field("pOrder", &self.pOrder).finish()
    }
}
impl ::core::default::Default for PROVIDOR_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROVIDOR_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.pOrder == other.pOrder
    }
}
impl ::core::cmp::Eq for PROVIDOR_INFO_2W {}
impl ::core::fmt::Debug for PROVIDOR_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDOR_INFO_2W").field("pOrder", &self.pOrder).finish()
    }
}
impl ::core::default::Default for PSCRIPT5_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSCRIPT5_PRIVATE_DEVMODE {
    fn eq(&self, other: &Self) -> bool {
        self.wReserved == other.wReserved && self.wSize == other.wSize
    }
}
impl ::core::cmp::Eq for PSCRIPT5_PRIVATE_DEVMODE {}
impl ::core::fmt::Debug for PSCRIPT5_PRIVATE_DEVMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSCRIPT5_PRIVATE_DEVMODE").field("wReserved", &self.wReserved).field("wSize", &self.wSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PUBLISHERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PUBLISHERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMode == other.dwMode && self.wMinoutlinePPEM == other.wMinoutlinePPEM && self.wMaxbitmapPPEM == other.wMaxbitmapPPEM
    }
}
impl ::core::cmp::Eq for PUBLISHERINFO {}
impl ::core::fmt::Debug for PUBLISHERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLISHERINFO").field("dwMode", &self.dwMode).field("wMinoutlinePPEM", &self.wMinoutlinePPEM).field("wMaxbitmapPPEM", &self.wMaxbitmapPPEM).finish()
    }
}
impl ::core::default::Default for PageCountType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PageCountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageCountType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintAsyncNotifyConversationStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintAsyncNotifyConversationStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintAsyncNotifyConversationStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintAsyncNotifyError {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintAsyncNotifyError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintAsyncNotifyError").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintAsyncNotifyUserFilter {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintAsyncNotifyUserFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintAsyncNotifyUserFilter").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintJobStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintJobStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintNamedProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PrintPropertiesCollection {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PrintPropertiesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.numberOfProperties == other.numberOfProperties && self.propertiesCollection == other.propertiesCollection
    }
}
impl ::core::cmp::Eq for PrintPropertiesCollection {}
impl ::core::fmt::Debug for PrintPropertiesCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintPropertiesCollection").field("numberOfProperties", &self.numberOfProperties).field("propertiesCollection", &self.propertiesCollection).finish()
    }
}
impl ::core::default::Default for PrintPropertyValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PrintSchemaConstrainedSetting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintSchemaConstrainedSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchemaConstrainedSetting").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintSchemaParameterDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintSchemaParameterDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchemaParameterDataType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintSchemaSelectionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintSchemaSelectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchemaSelectionType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SETRESULT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SETRESULT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wReserved == other.wReserved && self.hSetResult == other.hSetResult && self.Result == other.Result
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SETRESULT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SETRESULT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SETRESULT_INFO").field("cbSize", &self.cbSize).field("wReserved", &self.wReserved).field("hSetResult", &self.hSetResult).field("Result", &self.Result).finish()
    }
}
impl ::core::default::Default for SHIMOPTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHIMOPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHIMOPTS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHOWUIPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SHOWUIPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.UIType == other.UIType && self.MessageBoxParams == other.MessageBoxParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SHOWUIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SHOWUIPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHOWUIPARAMS").field("UIType", &self.UIType).field("MessageBoxParams", &self.MessageBoxParams).finish()
    }
}
impl ::core::default::Default for SIMULATE_CAPS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SIMULATE_CAPS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwLevel == other.dwLevel && self.dwPageOrderFlags == other.dwPageOrderFlags && self.dwNumberOfCopies == other.dwNumberOfCopies && self.dwCollate == other.dwCollate && self.dwNupOptions == other.dwNupOptions
    }
}
impl ::core::cmp::Eq for SIMULATE_CAPS_1 {}
impl ::core::fmt::Debug for SIMULATE_CAPS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SIMULATE_CAPS_1").field("dwLevel", &self.dwLevel).field("dwPageOrderFlags", &self.dwPageOrderFlags).field("dwNumberOfCopies", &self.dwNumberOfCopies).field("dwCollate", &self.dwCollate).field("dwNupOptions", &self.dwNupOptions).finish()
    }
}
impl ::core::default::Default for SPLCLIENT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPLCLIENT_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pMachineName == other.pMachineName && self.pUserName == other.pUserName && self.dwBuildNum == other.dwBuildNum && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.wProcessorArchitecture == other.wProcessorArchitecture
    }
}
impl ::core::cmp::Eq for SPLCLIENT_INFO_1 {}
impl ::core::fmt::Debug for SPLCLIENT_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPLCLIENT_INFO_1").field("dwSize", &self.dwSize).field("pMachineName", &self.pMachineName).field("pUserName", &self.pUserName).field("dwBuildNum", &self.dwBuildNum).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("wProcessorArchitecture", &self.wProcessorArchitecture).finish()
    }
}
impl ::core::default::Default for SPLCLIENT_INFO_2_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPLCLIENT_INFO_2_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.hSplPrinter == other.hSplPrinter
    }
}
impl ::core::cmp::Eq for SPLCLIENT_INFO_2_W2K {}
impl ::core::fmt::Debug for SPLCLIENT_INFO_2_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPLCLIENT_INFO_2_W2K").field("hSplPrinter", &self.hSplPrinter).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SPLCLIENT_INFO_2_WINXP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SPLCLIENT_INFO_2_WINXP {
    fn eq(&self, other: &Self) -> bool {
        self.hSplPrinter == other.hSplPrinter
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SPLCLIENT_INFO_2_WINXP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SPLCLIENT_INFO_2_WINXP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPLCLIENT_INFO_2_WINXP").field("hSplPrinter", &self.hSplPrinter).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SPLCLIENT_INFO_2_WINXP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SPLCLIENT_INFO_2_WINXP {
    fn eq(&self, other: &Self) -> bool {
        self.hSplPrinter == other.hSplPrinter
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SPLCLIENT_INFO_2_WINXP {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for SPLCLIENT_INFO_2_WINXP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPLCLIENT_INFO_2_WINXP").field("hSplPrinter", &self.hSplPrinter).finish()
    }
}
impl ::core::default::Default for SPLCLIENT_INFO_3_VISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPLCLIENT_INFO_3_VISTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.dwSize == other.dwSize && self.pMachineName == other.pMachineName && self.pUserName == other.pUserName && self.dwBuildNum == other.dwBuildNum && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.wProcessorArchitecture == other.wProcessorArchitecture && self.hSplPrinter == other.hSplPrinter
    }
}
impl ::core::cmp::Eq for SPLCLIENT_INFO_3_VISTA {}
impl ::core::fmt::Debug for SPLCLIENT_INFO_3_VISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPLCLIENT_INFO_3_VISTA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("dwSize", &self.dwSize).field("pMachineName", &self.pMachineName).field("pUserName", &self.pUserName).field("dwBuildNum", &self.dwBuildNum).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("wProcessorArchitecture", &self.wProcessorArchitecture).field("hSplPrinter", &self.hSplPrinter).finish()
    }
}
impl ::core::default::Default for TRANSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UFF_FILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UFF_FILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature && self.dwVersion == other.dwVersion && self.dwSize == other.dwSize && self.nFonts == other.nFonts && self.nGlyphSets == other.nGlyphSets && self.nVarData == other.nVarData && self.offFontDir == other.offFontDir && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for UFF_FILEHEADER {}
impl ::core::fmt::Debug for UFF_FILEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UFF_FILEHEADER").field("dwSignature", &self.dwSignature).field("dwVersion", &self.dwVersion).field("dwSize", &self.dwSize).field("nFonts", &self.nFonts).field("nGlyphSets", &self.nGlyphSets).field("nVarData", &self.nVarData).field("offFontDir", &self.offFontDir).field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for UFF_FONTDIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UFF_FONTDIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature && self.wSize == other.wSize && self.wFontID == other.wFontID && self.sGlyphID == other.sGlyphID && self.wFlags == other.wFlags && self.dwInstallerSig == other.dwInstallerSig && self.offFontName == other.offFontName && self.offCartridgeName == other.offCartridgeName && self.offFontData == other.offFontData && self.offGlyphData == other.offGlyphData && self.offVarData == other.offVarData
    }
}
impl ::core::cmp::Eq for UFF_FONTDIRECTORY {}
impl ::core::fmt::Debug for UFF_FONTDIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UFF_FONTDIRECTORY").field("dwSignature", &self.dwSignature).field("wSize", &self.wSize).field("wFontID", &self.wFontID).field("sGlyphID", &self.sGlyphID).field("wFlags", &self.wFlags).field("dwInstallerSig", &self.dwInstallerSig).field("offFontName", &self.offFontName).field("offCartridgeName", &self.offCartridgeName).field("offFontData", &self.offFontData).field("offGlyphData", &self.offGlyphData).field("offVarData", &self.offVarData).finish()
    }
}
impl ::core::default::Default for UI_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UI_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNIDRVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNIDRVINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.flGenFlags == other.flGenFlags && self.wType == other.wType && self.fCaps == other.fCaps && self.wXRes == other.wXRes && self.wYRes == other.wYRes && self.sYAdjust == other.sYAdjust && self.sYMoved == other.sYMoved && self.wPrivateData == other.wPrivateData && self.sShift == other.sShift && self.SelectFont == other.SelectFont && self.UnSelectFont == other.UnSelectFont && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for UNIDRVINFO {}
impl ::core::fmt::Debug for UNIDRVINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIDRVINFO").field("dwSize", &self.dwSize).field("flGenFlags", &self.flGenFlags).field("wType", &self.wType).field("fCaps", &self.fCaps).field("wXRes", &self.wXRes).field("wYRes", &self.wYRes).field("sYAdjust", &self.sYAdjust).field("sYMoved", &self.sYMoved).field("wPrivateData", &self.wPrivateData).field("sShift", &self.sShift).field("SelectFont", &self.SelectFont).field("UnSelectFont", &self.UnSelectFont).field("wReserved", &self.wReserved).finish()
    }
}
impl ::core::default::Default for UNIDRV_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNIDRV_PRIVATE_DEVMODE {
    fn eq(&self, other: &Self) -> bool {
        self.wReserved == other.wReserved && self.wSize == other.wSize
    }
}
impl ::core::cmp::Eq for UNIDRV_PRIVATE_DEVMODE {}
impl ::core::fmt::Debug for UNIDRV_PRIVATE_DEVMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIDRV_PRIVATE_DEVMODE").field("wReserved", &self.wReserved).field("wSize", &self.wSize).finish()
    }
}
impl ::core::default::Default for UNIFM_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNIFM_HDR {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVersion == other.dwVersion && self.ulDefaultCodepage == other.ulDefaultCodepage && self.lGlyphSetDataRCID == other.lGlyphSetDataRCID && self.loUnidrvInfo == other.loUnidrvInfo && self.loIFIMetrics == other.loIFIMetrics && self.loExtTextMetric == other.loExtTextMetric && self.loWidthTable == other.loWidthTable && self.loKernPair == other.loKernPair && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for UNIFM_HDR {}
impl ::core::fmt::Debug for UNIFM_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIFM_HDR").field("dwSize", &self.dwSize).field("dwVersion", &self.dwVersion).field("ulDefaultCodepage", &self.ulDefaultCodepage).field("lGlyphSetDataRCID", &self.lGlyphSetDataRCID).field("loUnidrvInfo", &self.loUnidrvInfo).field("loIFIMetrics", &self.loIFIMetrics).field("loExtTextMetric", &self.loExtTextMetric).field("loWidthTable", &self.loWidthTable).field("loKernPair", &self.loKernPair).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for UNI_CODEPAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNI_CODEPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwCodePage == other.dwCodePage && self.SelectSymbolSet == other.SelectSymbolSet && self.UnSelectSymbolSet == other.UnSelectSymbolSet
    }
}
impl ::core::cmp::Eq for UNI_CODEPAGEINFO {}
impl ::core::fmt::Debug for UNI_CODEPAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNI_CODEPAGEINFO").field("dwCodePage", &self.dwCodePage).field("SelectSymbolSet", &self.SelectSymbolSet).field("UnSelectSymbolSet", &self.UnSelectSymbolSet).finish()
    }
}
impl ::core::default::Default for UNI_GLYPHSETDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNI_GLYPHSETDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags && self.lPredefinedID == other.lPredefinedID && self.dwGlyphCount == other.dwGlyphCount && self.dwRunCount == other.dwRunCount && self.loRunOffset == other.loRunOffset && self.dwCodePageCount == other.dwCodePageCount && self.loCodePageOffset == other.loCodePageOffset && self.loMapTableOffset == other.loMapTableOffset && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for UNI_GLYPHSETDATA {}
impl ::core::fmt::Debug for UNI_GLYPHSETDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNI_GLYPHSETDATA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("lPredefinedID", &self.lPredefinedID)
            .field("dwGlyphCount", &self.dwGlyphCount)
            .field("dwRunCount", &self.dwRunCount)
            .field("loRunOffset", &self.loRunOffset)
            .field("dwCodePageCount", &self.dwCodePageCount)
            .field("loCodePageOffset", &self.loCodePageOffset)
            .field("loMapTableOffset", &self.loMapTableOffset)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for USERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwItemID == other.dwItemID && self.pKeyWordName == other.pKeyWordName && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for USERDATA {}
impl ::core::fmt::Debug for USERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USERDATA").field("dwSize", &self.dwSize).field("dwItemID", &self.dwItemID).field("pKeyWordName", &self.pKeyWordName).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for WIDTHRUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIDTHRUN {
    fn eq(&self, other: &Self) -> bool {
        self.wStartGlyph == other.wStartGlyph && self.wGlyphCount == other.wGlyphCount && self.loCharWidthOffset == other.loCharWidthOffset
    }
}
impl ::core::cmp::Eq for WIDTHRUN {}
impl ::core::fmt::Debug for WIDTHRUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIDTHRUN").field("wStartGlyph", &self.wStartGlyph).field("wGlyphCount", &self.wGlyphCount).field("loCharWidthOffset", &self.loCharWidthOffset).finish()
    }
}
impl ::core::default::Default for WIDTHTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIDTHTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwRunNum == other.dwRunNum && self.WidthRun == other.WidthRun
    }
}
impl ::core::cmp::Eq for WIDTHTABLE {}
impl ::core::fmt::Debug for WIDTHTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIDTHTABLE").field("dwSize", &self.dwSize).field("dwRunNum", &self.dwRunNum).field("WidthRun", &self.WidthRun).finish()
    }
}
impl ::core::default::Default for XPSRAS_BACKGROUND_COLOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPSRAS_BACKGROUND_COLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPSRAS_BACKGROUND_COLOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPSRAS_PIXEL_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPSRAS_PIXEL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPSRAS_PIXEL_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPSRAS_RENDERING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPSRAS_RENDERING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPSRAS_RENDERING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SPLCLIENT_INFO_2_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _SPLCLIENT_INFO_2_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.hSplPrinter == other.hSplPrinter
    }
}
impl ::core::cmp::Eq for _SPLCLIENT_INFO_2_V3 {}
impl ::core::fmt::Debug for _SPLCLIENT_INFO_2_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_SPLCLIENT_INFO_2_V3").field("hSplPrinter", &self.hSplPrinter).finish()
    }
}
