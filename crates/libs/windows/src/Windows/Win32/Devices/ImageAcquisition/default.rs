#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDIALOGDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.lIntent == other.lIntent && self.lItemCount == other.lItemCount && self.ppWiaItems == other.ppWiaItems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEDIALOGDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICEDIALOGDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("lIntent", &self.lIntent).field("lItemCount", &self.lItemCount).field("ppWiaItems", &self.ppWiaItems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDIALOGDATA2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.hwndParent == other.hwndParent && self.bstrFolderName == other.bstrFolderName && self.bstrFilename == other.bstrFilename && self.lNumFiles == other.lNumFiles && self.pbstrFilePaths == other.pbstrFilePaths && self.pWiaItem == other.pWiaItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEDIALOGDATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICEDIALOGDATA2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA2").field("cbSize", &self.cbSize).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("hwndParent", &self.hwndParent).field("bstrFolderName", &self.bstrFolderName).field("bstrFilename", &self.bstrFilename).field("lNumFiles", &self.lNumFiles).field("pbstrFilePaths", &self.pbstrFilePaths).field("pWiaItem", &self.pWiaItem).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_CAPS {}
impl ::core::fmt::Debug for IEnumWIA_DEV_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_CAPS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_INFO {}
impl ::core::fmt::Debug for IEnumWIA_DEV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_INFO").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_FORMAT_INFO {}
impl ::core::fmt::Debug for IEnumWIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_FORMAT_INFO").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem {}
impl ::core::fmt::Debug for IEnumWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem2 {}
impl ::core::fmt::Debug for IEnumWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaAppErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaAppErrorHandler {}
impl ::core::fmt::Debug for IWiaAppErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaAppErrorHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaDataCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataCallback {}
impl ::core::fmt::Debug for IWiaDataCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaDataTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataTransfer {}
impl ::core::fmt::Debug for IWiaDataTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataTransfer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr {}
impl ::core::fmt::Debug for IWiaDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr2 {}
impl ::core::fmt::Debug for IWiaDevMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaDrvItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDrvItem {}
impl ::core::fmt::Debug for IWiaDrvItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDrvItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaErrorHandler {}
impl ::core::fmt::Debug for IWiaErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaErrorHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaEventCallback {}
impl ::core::fmt::Debug for IWiaEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaEventCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaImageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaImageFilter {}
impl ::core::fmt::Debug for IWiaImageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaImageFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem {}
impl ::core::fmt::Debug for IWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem2 {}
impl ::core::fmt::Debug for IWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaItemExtras {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItemExtras {}
impl ::core::fmt::Debug for IWiaItemExtras {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItemExtras").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLog {}
impl ::core::fmt::Debug for IWiaLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaLogEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLogEx {}
impl ::core::fmt::Debug for IWiaLogEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLogEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrv {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrv {}
impl ::core::fmt::Debug for IWiaMiniDrv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrv").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvCallBack {}
impl ::core::fmt::Debug for IWiaMiniDrvCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvCallBack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvTransferCallback {}
impl ::core::fmt::Debug for IWiaMiniDrvTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvTransferCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaNotifyDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaNotifyDevMgr {}
impl ::core::fmt::Debug for IWiaNotifyDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaNotifyDevMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPreview {}
impl ::core::fmt::Debug for IWiaPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPreview").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaPropertyStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPropertyStorage {}
impl ::core::fmt::Debug for IWiaPropertyStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPropertyStorage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaSegmentationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaSegmentationFilter {}
impl ::core::fmt::Debug for IWiaSegmentationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaSegmentationFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransfer {}
impl ::core::fmt::Debug for IWiaTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransfer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransferCallback {}
impl ::core::fmt::Debug for IWiaTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransferCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension {}
impl ::core::fmt::Debug for IWiaUIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension2 {}
impl ::core::fmt::Debug for IWiaUIExtension2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWiaVideo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaVideo {}
impl ::core::fmt::Debug for IWiaVideo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaVideo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDRV_TRANSFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MINIDRV_TRANSFER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.lWidthInPixels == other.lWidthInPixels
            && self.lLines == other.lLines
            && self.lDepth == other.lDepth
            && self.lXRes == other.lXRes
            && self.lYRes == other.lYRes
            && self.lCompression == other.lCompression
            && self.guidFormatID == other.guidFormatID
            && self.tymed == other.tymed
            && self.hFile == other.hFile
            && self.cbOffset == other.cbOffset
            && self.lBufferSize == other.lBufferSize
            && self.lActiveBuffer == other.lActiveBuffer
            && self.lNumBuffers == other.lNumBuffers
            && self.pBaseBuffer == other.pBaseBuffer
            && self.pTransferBuffer == other.pTransferBuffer
            && self.bTransferDataCB == other.bTransferDataCB
            && self.bClassDrvAllocBuf == other.bClassDrvAllocBuf
            && self.lClientAddress == other.lClientAddress
            && self.pIWiaMiniDrvCallBack == other.pIWiaMiniDrvCallBack
            && self.lImageSize == other.lImageSize
            && self.lHeaderSize == other.lHeaderSize
            && self.lItemSize == other.lItemSize
            && self.cbWidthInBytes == other.cbWidthInBytes
            && self.lPage == other.lPage
            && self.lCurIfdOffset == other.lCurIfdOffset
            && self.lPrevIfdOffset == other.lPrevIfdOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MINIDRV_TRANSFER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MINIDRV_TRANSFER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDRV_TRANSFER_CONTEXT")
            .field("lSize", &self.lSize)
            .field("lWidthInPixels", &self.lWidthInPixels)
            .field("lLines", &self.lLines)
            .field("lDepth", &self.lDepth)
            .field("lXRes", &self.lXRes)
            .field("lYRes", &self.lYRes)
            .field("lCompression", &self.lCompression)
            .field("guidFormatID", &self.guidFormatID)
            .field("tymed", &self.tymed)
            .field("hFile", &self.hFile)
            .field("cbOffset", &self.cbOffset)
            .field("lBufferSize", &self.lBufferSize)
            .field("lActiveBuffer", &self.lActiveBuffer)
            .field("lNumBuffers", &self.lNumBuffers)
            .field("pBaseBuffer", &self.pBaseBuffer)
            .field("pTransferBuffer", &self.pTransferBuffer)
            .field("bTransferDataCB", &self.bTransferDataCB)
            .field("bClassDrvAllocBuf", &self.bClassDrvAllocBuf)
            .field("lClientAddress", &self.lClientAddress)
            .field("pIWiaMiniDrvCallBack", &self.pIWiaMiniDrvCallBack)
            .field("lImageSize", &self.lImageSize)
            .field("lHeaderSize", &self.lHeaderSize)
            .field("lItemSize", &self.lItemSize)
            .field("cbWidthInBytes", &self.cbWidthInBytes)
            .field("lPage", &self.lPage)
            .field("lCurIfdOffset", &self.lCurIfdOffset)
            .field("lPrevIfdOffset", &self.lPrevIfdOffset)
            .finish()
    }
}
impl ::core::default::Default for RANGEVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RANGEVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lMax == other.lMax && self.lStep == other.lStep
    }
}
impl ::core::cmp::Eq for RANGEVALUE {}
impl ::core::fmt::Debug for RANGEVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RANGEVALUE").field("lMin", &self.lMin).field("lMax", &self.lMax).field("lStep", &self.lStep).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCANINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCANINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ADF == other.ADF
            && self.TPA == other.TPA
            && self.Endorser == other.Endorser
            && self.OpticalXResolution == other.OpticalXResolution
            && self.OpticalYResolution == other.OpticalYResolution
            && self.BedWidth == other.BedWidth
            && self.BedHeight == other.BedHeight
            && self.IntensityRange == other.IntensityRange
            && self.ContrastRange == other.ContrastRange
            && self.SupportedCompressionType == other.SupportedCompressionType
            && self.SupportedDataTypes == other.SupportedDataTypes
            && self.WidthPixels == other.WidthPixels
            && self.WidthBytes == other.WidthBytes
            && self.Lines == other.Lines
            && self.DataType == other.DataType
            && self.PixelBits == other.PixelBits
            && self.Intensity == other.Intensity
            && self.Contrast == other.Contrast
            && self.Xresolution == other.Xresolution
            && self.Yresolution == other.Yresolution
            && self.Window == other.Window
            && self.DitherPattern == other.DitherPattern
            && self.Negative == other.Negative
            && self.Mirror == other.Mirror
            && self.AutoBack == other.AutoBack
            && self.ColorDitherPattern == other.ColorDitherPattern
            && self.ToneMap == other.ToneMap
            && self.Compression == other.Compression
            && self.RawDataFormat == other.RawDataFormat
            && self.RawPixelOrder == other.RawPixelOrder
            && self.bNeedDataAlignment == other.bNeedDataAlignment
            && self.DelayBetweenRead == other.DelayBetweenRead
            && self.MaxBufferSize == other.MaxBufferSize
            && self.DeviceIOHandles == other.DeviceIOHandles
            && self.lReserved == other.lReserved
            && self.pMicroDriverContext == other.pMicroDriverContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCANINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANINFO")
            .field("ADF", &self.ADF)
            .field("TPA", &self.TPA)
            .field("Endorser", &self.Endorser)
            .field("OpticalXResolution", &self.OpticalXResolution)
            .field("OpticalYResolution", &self.OpticalYResolution)
            .field("BedWidth", &self.BedWidth)
            .field("BedHeight", &self.BedHeight)
            .field("IntensityRange", &self.IntensityRange)
            .field("ContrastRange", &self.ContrastRange)
            .field("SupportedCompressionType", &self.SupportedCompressionType)
            .field("SupportedDataTypes", &self.SupportedDataTypes)
            .field("WidthPixels", &self.WidthPixels)
            .field("WidthBytes", &self.WidthBytes)
            .field("Lines", &self.Lines)
            .field("DataType", &self.DataType)
            .field("PixelBits", &self.PixelBits)
            .field("Intensity", &self.Intensity)
            .field("Contrast", &self.Contrast)
            .field("Xresolution", &self.Xresolution)
            .field("Yresolution", &self.Yresolution)
            .field("Window", &self.Window)
            .field("DitherPattern", &self.DitherPattern)
            .field("Negative", &self.Negative)
            .field("Mirror", &self.Mirror)
            .field("AutoBack", &self.AutoBack)
            .field("ColorDitherPattern", &self.ColorDitherPattern)
            .field("ToneMap", &self.ToneMap)
            .field("Compression", &self.Compression)
            .field("RawDataFormat", &self.RawDataFormat)
            .field("RawPixelOrder", &self.RawPixelOrder)
            .field("bNeedDataAlignment", &self.bNeedDataAlignment)
            .field("DelayBetweenRead", &self.DelayBetweenRead)
            .field("MaxBufferSize", &self.MaxBufferSize)
            .field("DeviceIOHandles", &self.DeviceIOHandles)
            .field("lReserved", &self.lReserved)
            .field("pMicroDriverContext", &self.pMicroDriverContext)
            .finish()
    }
}
impl ::core::default::Default for SCANWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCANWINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.xPos == other.xPos && self.yPos == other.yPos && self.xExtent == other.xExtent && self.yExtent == other.yExtent
    }
}
impl ::core::cmp::Eq for SCANWINDOW {}
impl ::core::fmt::Debug for SCANWINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANWINDOW").field("xPos", &self.xPos).field("yPos", &self.yPos).field("xExtent", &self.xExtent).field("yExtent", &self.yExtent).finish()
    }
}
impl ::core::default::Default for TWAIN_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TWAIN_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize && self.lMSG == other.lMSG && self.lCapID == other.lCapID && self.lConType == other.lConType && self.lRC == other.lRC && self.lCC == other.lCC && self.lDataSize == other.lDataSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for TWAIN_CAPABILITY {}
impl ::core::fmt::Debug for TWAIN_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TWAIN_CAPABILITY").field("lSize", &self.lSize).field("lMSG", &self.lMSG).field("lCapID", &self.lCapID).field("lConType", &self.lConType).field("lRC", &self.lRC).field("lCC", &self.lCC).field("lDataSize", &self.lDataSize).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VAL {
    fn eq(&self, other: &Self) -> bool {
        self.lVal == other.lVal && self.dblVal == other.dblVal && self.pGuid == other.pGuid && self.pScanInfo == other.pScanInfo && self.handle == other.handle && self.ppButtonNames == other.ppButtonNames && self.pHandle == other.pHandle && self.lReserved == other.lReserved && self.szVal == other.szVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VAL").field("lVal", &self.lVal).field("dblVal", &self.dblVal).field("pGuid", &self.pGuid).field("pScanInfo", &self.pScanInfo).field("handle", &self.handle).field("ppButtonNames", &self.ppButtonNames).field("pHandle", &self.pHandle).field("lReserved", &self.lReserved).field("szVal", &self.szVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WIAS_DOWN_SAMPLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIAS_DOWN_SAMPLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulOriginalWidth == other.ulOriginalWidth && self.ulOriginalHeight == other.ulOriginalHeight && self.ulBitsPerPixel == other.ulBitsPerPixel && self.ulXRes == other.ulXRes && self.ulYRes == other.ulYRes && self.ulDownSampledWidth == other.ulDownSampledWidth && self.ulDownSampledHeight == other.ulDownSampledHeight && self.ulActualSize == other.ulActualSize && self.ulDestBufSize == other.ulDestBufSize && self.ulSrcBufSize == other.ulSrcBufSize && self.pSrcBuffer == other.pSrcBuffer && self.pDestBuffer == other.pDestBuffer
    }
}
impl ::core::cmp::Eq for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::fmt::Debug for WIAS_DOWN_SAMPLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_DOWN_SAMPLE_INFO")
            .field("ulOriginalWidth", &self.ulOriginalWidth)
            .field("ulOriginalHeight", &self.ulOriginalHeight)
            .field("ulBitsPerPixel", &self.ulBitsPerPixel)
            .field("ulXRes", &self.ulXRes)
            .field("ulYRes", &self.ulYRes)
            .field("ulDownSampledWidth", &self.ulDownSampledWidth)
            .field("ulDownSampledHeight", &self.ulDownSampledHeight)
            .field("ulActualSize", &self.ulActualSize)
            .field("ulDestBufSize", &self.ulDestBufSize)
            .field("ulSrcBufSize", &self.ulSrcBufSize)
            .field("pSrcBuffer", &self.pSrcBuffer)
            .field("pDestBuffer", &self.pDestBuffer)
            .finish()
    }
}
impl ::core::default::Default for WIAS_ENDORSER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulPageCount == other.ulPageCount && self.ulNumEndorserValues == other.ulNumEndorserValues && self.pEndorserValues == other.pEndorserValues
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_INFO {}
impl ::core::fmt::Debug for WIAS_ENDORSER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_INFO").field("ulPageCount", &self.ulPageCount).field("ulNumEndorserValues", &self.ulNumEndorserValues).field("pEndorserValues", &self.pEndorserValues).finish()
    }
}
impl ::core::default::Default for WIAS_ENDORSER_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.wszTokenName == other.wszTokenName && self.wszValue == other.wszValue
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_VALUE {}
impl ::core::fmt::Debug for WIAS_ENDORSER_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_VALUE").field("wszTokenName", &self.wszTokenName).field("wszValue", &self.wszValue).finish()
    }
}
impl ::core::default::Default for WIAVIDEO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIAVIDEO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIAVIDEO_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIA_BARCODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_BARCODES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Version == other.Version && self.Size == other.Size && self.Count == other.Count && self.Barcodes == other.Barcodes
    }
}
impl ::core::cmp::Eq for WIA_BARCODES {}
impl ::core::fmt::Debug for WIA_BARCODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("Barcodes", &self.Barcodes).finish()
    }
}
impl ::core::default::Default for WIA_BARCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_BARCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Page == other.Page && self.Confidence == other.Confidence && self.XOffset == other.XOffset && self.YOffset == other.YOffset && self.Rotation == other.Rotation && self.Length == other.Length && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WIA_BARCODE_INFO {}
impl ::core::fmt::Debug for WIA_BARCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODE_INFO").field("Size", &self.Size).field("Type", &self.Type).field("Page", &self.Page).field("Confidence", &self.Confidence).field("XOffset", &self.XOffset).field("YOffset", &self.YOffset).field("Rotation", &self.Rotation).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
impl ::core::default::Default for WIA_DATA_CALLBACK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_DATA_CALLBACK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize && self.guidFormatID == other.guidFormatID && self.lBufferSize == other.lBufferSize && self.lPageCount == other.lPageCount
    }
}
impl ::core::cmp::Eq for WIA_DATA_CALLBACK_HEADER {}
impl ::core::fmt::Debug for WIA_DATA_CALLBACK_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_CALLBACK_HEADER").field("lSize", &self.lSize).field("guidFormatID", &self.guidFormatID).field("lBufferSize", &self.lBufferSize).field("lPageCount", &self.lPageCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_DATA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_DATA_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.ulSection == other.ulSection && self.ulBufferSize == other.ulBufferSize && self.bDoubleBuffer == other.bDoubleBuffer && self.ulReserved1 == other.ulReserved1 && self.ulReserved2 == other.ulReserved2 && self.ulReserved3 == other.ulReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_DATA_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulSection", &self.ulSection).field("ulBufferSize", &self.ulBufferSize).field("bDoubleBuffer", &self.bDoubleBuffer).field("ulReserved1", &self.ulReserved1).field("ulReserved2", &self.ulReserved2).field("ulReserved3", &self.ulReserved3).finish()
    }
}
impl ::core::default::Default for WIA_DEV_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_DEV_CAP {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.ulFlags == other.ulFlags && self.bstrName == other.bstrName && self.bstrDescription == other.bstrDescription && self.bstrIcon == other.bstrIcon && self.bstrCommandline == other.bstrCommandline
    }
}
impl ::core::cmp::Eq for WIA_DEV_CAP {}
impl ::core::fmt::Debug for WIA_DEV_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("bstrName", &self.bstrName).field("bstrDescription", &self.bstrDescription).field("bstrIcon", &self.bstrIcon).field("bstrCommandline", &self.bstrCommandline).finish()
    }
}
impl ::core::default::Default for WIA_DEV_CAP_DRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_DEV_CAP_DRV {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.ulFlags == other.ulFlags && self.wszName == other.wszName && self.wszDescription == other.wszDescription && self.wszIcon == other.wszIcon
    }
}
impl ::core::cmp::Eq for WIA_DEV_CAP_DRV {}
impl ::core::fmt::Debug for WIA_DEV_CAP_DRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP_DRV").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("wszName", &self.wszName).field("wszDescription", &self.wszDescription).field("wszIcon", &self.wszIcon).finish()
    }
}
impl ::core::default::Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_DITHER_PATTERN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize && self.bstrPatternName == other.bstrPatternName && self.lPatternWidth == other.lPatternWidth && self.lPatternLength == other.lPatternLength && self.cbPattern == other.cbPattern && self.pbPattern == other.pbPattern
    }
}
impl ::core::cmp::Eq for WIA_DITHER_PATTERN_DATA {}
impl ::core::fmt::Debug for WIA_DITHER_PATTERN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DITHER_PATTERN_DATA").field("lSize", &self.lSize).field("bstrPatternName", &self.bstrPatternName).field("lPatternWidth", &self.lPatternWidth).field("lPatternLength", &self.lPatternLength).field("cbPattern", &self.cbPattern).field("pbPattern", &self.pbPattern).finish()
    }
}
impl ::core::default::Default for WIA_EXTENDED_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_EXTENDED_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.ulMinBufferSize == other.ulMinBufferSize && self.ulOptimalBufferSize == other.ulOptimalBufferSize && self.ulMaxBufferSize == other.ulMaxBufferSize && self.ulNumBuffers == other.ulNumBuffers
    }
}
impl ::core::cmp::Eq for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::fmt::Debug for WIA_EXTENDED_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_EXTENDED_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulMinBufferSize", &self.ulMinBufferSize).field("ulOptimalBufferSize", &self.ulOptimalBufferSize).field("ulMaxBufferSize", &self.ulMaxBufferSize).field("ulNumBuffers", &self.ulNumBuffers).finish()
    }
}
impl ::core::default::Default for WIA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidFormatID == other.guidFormatID && self.lTymed == other.lTymed
    }
}
impl ::core::cmp::Eq for WIA_FORMAT_INFO {}
impl ::core::fmt::Debug for WIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_FORMAT_INFO").field("guidFormatID", &self.guidFormatID).field("lTymed", &self.lTymed).finish()
    }
}
impl ::core::default::Default for WIA_MICR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_MICR {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Version == other.Version && self.Size == other.Size && self.Placeholder == other.Placeholder && self.Reserved == other.Reserved && self.Count == other.Count && self.Micr == other.Micr
    }
}
impl ::core::cmp::Eq for WIA_MICR {}
impl ::core::fmt::Debug for WIA_MICR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Placeholder", &self.Placeholder).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Micr", &self.Micr).finish()
    }
}
impl ::core::default::Default for WIA_MICR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_MICR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Page == other.Page && self.Length == other.Length && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WIA_MICR_INFO {}
impl ::core::fmt::Debug for WIA_MICR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR_INFO").field("Size", &self.Size).field("Page", &self.Page).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
impl ::core::default::Default for WIA_PATCH_CODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Version == other.Version && self.Size == other.Size && self.Count == other.Count && self.PatchCodes == other.PatchCodes
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODES {}
impl ::core::fmt::Debug for WIA_PATCH_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("PatchCodes", &self.PatchCodes).finish()
    }
}
impl ::core::default::Default for WIA_PATCH_CODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODE_INFO {}
impl ::core::fmt::Debug for WIA_PATCH_CODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODE_INFO").field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cProps == other.cProps && self.pProps == other.pProps && self.pChanged == other.pChanged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_CONTEXT").field("cProps", &self.cProps).field("pProps", &self.pProps).field("pChanged", &self.pChanged).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for WIA_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_PROPID_TO_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.propid == other.propid && self.pszName == other.pszName
    }
}
impl ::core::cmp::Eq for WIA_PROPID_TO_NAME {}
impl ::core::fmt::Debug for WIA_PROPID_TO_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPID_TO_NAME").field("propid", &self.propid).field("pszName", &self.pszName).finish()
    }
}
impl ::core::default::Default for WIA_RAW_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIA_RAW_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Version == other.Version && self.HeaderSize == other.HeaderSize && self.XRes == other.XRes && self.YRes == other.YRes && self.XExtent == other.XExtent && self.YExtent == other.YExtent && self.BytesPerLine == other.BytesPerLine && self.BitsPerPixel == other.BitsPerPixel && self.ChannelsPerPixel == other.ChannelsPerPixel && self.DataType == other.DataType && self.BitsPerChannel == other.BitsPerChannel && self.Compression == other.Compression && self.PhotometricInterp == other.PhotometricInterp && self.LineOrder == other.LineOrder && self.RawDataOffset == other.RawDataOffset && self.RawDataSize == other.RawDataSize && self.PaletteOffset == other.PaletteOffset && self.PaletteSize == other.PaletteSize
    }
}
impl ::core::cmp::Eq for WIA_RAW_HEADER {}
impl ::core::fmt::Debug for WIA_RAW_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_RAW_HEADER")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("HeaderSize", &self.HeaderSize)
            .field("XRes", &self.XRes)
            .field("YRes", &self.YRes)
            .field("XExtent", &self.XExtent)
            .field("YExtent", &self.YExtent)
            .field("BytesPerLine", &self.BytesPerLine)
            .field("BitsPerPixel", &self.BitsPerPixel)
            .field("ChannelsPerPixel", &self.ChannelsPerPixel)
            .field("DataType", &self.DataType)
            .field("BitsPerChannel", &self.BitsPerChannel)
            .field("Compression", &self.Compression)
            .field("PhotometricInterp", &self.PhotometricInterp)
            .field("LineOrder", &self.LineOrder)
            .field("RawDataOffset", &self.RawDataOffset)
            .field("RawDataSize", &self.RawDataSize)
            .field("PaletteOffset", &self.PaletteOffset)
            .field("PaletteSize", &self.PaletteSize)
            .finish()
    }
}
impl ::core::default::Default for WiaTransferParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WiaTransferParams {
    fn eq(&self, other: &Self) -> bool {
        self.lMessage == other.lMessage && self.lPercentComplete == other.lPercentComplete && self.ulTransferredBytes == other.ulTransferredBytes && self.hrErrorStatus == other.hrErrorStatus
    }
}
impl ::core::cmp::Eq for WiaTransferParams {}
impl ::core::fmt::Debug for WiaTransferParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WiaTransferParams").field("lMessage", &self.lMessage).field("lPercentComplete", &self.lPercentComplete).field("ulTransferredBytes", &self.ulTransferredBytes).field("hrErrorStatus", &self.hrErrorStatus).finish()
    }
}
