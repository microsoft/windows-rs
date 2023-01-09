impl ::core::default::Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::core::fmt::Debug for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ADAPTER_INTERFACE_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for WINBIO_ANTI_SPOOF_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_ANTI_SPOOF_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.Source == other.Source
    }
}
impl ::core::cmp::Eq for WINBIO_ANTI_SPOOF_POLICY {}
impl ::core::fmt::Debug for WINBIO_ANTI_SPOOF_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ANTI_SPOOF_POLICY").field("Action", &self.Action).field("Source", &self.Source).finish()
    }
}
impl ::core::default::Default for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_ANTI_SPOOF_POLICY_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_ASYNC_NOTIFICATION_METHOD").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength && self.FormatIdentifier == other.FormatIdentifier && self.VersionNumber == other.VersionNumber && self.ProductId == other.ProductId && self.CaptureDeviceId == other.CaptureDeviceId && self.ImageAcquisitionLevel == other.ImageAcquisitionLevel && self.HorizontalScanResolution == other.HorizontalScanResolution && self.VerticalScanResolution == other.VerticalScanResolution && self.HorizontalImageResolution == other.HorizontalImageResolution && self.VerticalImageResolution == other.VerticalImageResolution && self.ElementCount == other.ElementCount && self.ScaleUnits == other.ScaleUnits && self.PixelDepth == other.PixelDepth && self.ImageCompressionAlg == other.ImageCompressionAlg && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_HEADER {}
impl ::core::fmt::Debug for WINBIO_BDB_ANSI_381_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BDB_ANSI_381_HEADER")
            .field("RecordLength", &self.RecordLength)
            .field("FormatIdentifier", &self.FormatIdentifier)
            .field("VersionNumber", &self.VersionNumber)
            .field("ProductId", &self.ProductId)
            .field("CaptureDeviceId", &self.CaptureDeviceId)
            .field("ImageAcquisitionLevel", &self.ImageAcquisitionLevel)
            .field("HorizontalScanResolution", &self.HorizontalScanResolution)
            .field("VerticalScanResolution", &self.VerticalScanResolution)
            .field("HorizontalImageResolution", &self.HorizontalImageResolution)
            .field("VerticalImageResolution", &self.VerticalImageResolution)
            .field("ElementCount", &self.ElementCount)
            .field("ScaleUnits", &self.ScaleUnits)
            .field("PixelDepth", &self.PixelDepth)
            .field("ImageCompressionAlg", &self.ImageCompressionAlg)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.BlockLength == other.BlockLength && self.HorizontalLineLength == other.HorizontalLineLength && self.VerticalLineLength == other.VerticalLineLength && self.Position == other.Position && self.CountOfViews == other.CountOfViews && self.ViewNumber == other.ViewNumber && self.ImageQuality == other.ImageQuality && self.ImpressionType == other.ImpressionType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_RECORD {}
impl ::core::fmt::Debug for WINBIO_BDB_ANSI_381_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BDB_ANSI_381_RECORD").field("BlockLength", &self.BlockLength).field("HorizontalLineLength", &self.HorizontalLineLength).field("VerticalLineLength", &self.VerticalLineLength).field("Position", &self.Position).field("CountOfViews", &self.CountOfViews).field("ViewNumber", &self.ViewNumber).field("ImageQuality", &self.ImageQuality).field("ImpressionType", &self.ImpressionType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WINBIO_BIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BIR {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderBlock == other.HeaderBlock && self.StandardDataBlock == other.StandardDataBlock && self.VendorDataBlock == other.VendorDataBlock && self.SignatureBlock == other.SignatureBlock
    }
}
impl ::core::cmp::Eq for WINBIO_BIR {}
impl ::core::fmt::Debug for WINBIO_BIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR").field("HeaderBlock", &self.HeaderBlock).field("StandardDataBlock", &self.StandardDataBlock).field("VendorDataBlock", &self.VendorDataBlock).field("SignatureBlock", &self.SignatureBlock).finish()
    }
}
impl ::core::default::Default for WINBIO_BIR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BIR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_DATA {}
impl ::core::fmt::Debug for WINBIO_BIR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_DATA").field("Size", &self.Size).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for WINBIO_BIR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields && self.HeaderVersion == other.HeaderVersion && self.PatronHeaderVersion == other.PatronHeaderVersion && self.DataFlags == other.DataFlags && self.Type == other.Type && self.Subtype == other.Subtype && self.Purpose == other.Purpose && self.DataQuality == other.DataQuality && self.CreationDate == other.CreationDate && self.ValidityPeriod == other.ValidityPeriod && self.BiometricDataFormat == other.BiometricDataFormat && self.ProductId == other.ProductId
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER {}
impl ::core::fmt::Debug for WINBIO_BIR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_HEADER")
            .field("ValidFields", &self.ValidFields)
            .field("HeaderVersion", &self.HeaderVersion)
            .field("PatronHeaderVersion", &self.PatronHeaderVersion)
            .field("DataFlags", &self.DataFlags)
            .field("Type", &self.Type)
            .field("Subtype", &self.Subtype)
            .field("Purpose", &self.Purpose)
            .field("DataQuality", &self.DataQuality)
            .field("CreationDate", &self.CreationDate)
            .field("ValidityPeriod", &self.ValidityPeriod)
            .field("BiometricDataFormat", &self.BiometricDataFormat)
            .field("ProductId", &self.ProductId)
            .finish()
    }
}
impl ::core::default::Default for WINBIO_BIR_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginDate == other.BeginDate && self.EndDate == other.EndDate
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER_0 {}
impl ::core::fmt::Debug for WINBIO_BIR_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_HEADER_0").field("BeginDate", &self.BeginDate).field("EndDate", &self.EndDate).finish()
    }
}
impl ::core::default::Default for WINBIO_BLANK_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BLANK_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult
    }
}
impl ::core::cmp::Eq for WINBIO_BLANK_PAYLOAD {}
impl ::core::fmt::Debug for WINBIO_BLANK_PAYLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BLANK_PAYLOAD").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).finish()
    }
}
impl ::core::default::Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_BSP_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.BspId == other.BspId && self.Description == other.Description && self.Vendor == other.Vendor && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for WINBIO_BSP_SCHEMA {}
impl ::core::fmt::Debug for WINBIO_BSP_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BSP_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("BspId", &self.BspId).field("Description", &self.Description).field("Vendor", &self.Vendor).field("Version", &self.Version).finish()
    }
}
impl ::core::default::Default for WINBIO_CALIBRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_CALIBRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.CalibrationData == other.CalibrationData
    }
}
impl ::core::cmp::Eq for WINBIO_CALIBRATION_INFO {}
impl ::core::fmt::Debug for WINBIO_CALIBRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CALIBRATION_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("CalibrationData", &self.CalibrationData).finish()
    }
}
impl ::core::default::Default for WINBIO_CAPTURE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.RejectDetail == other.RejectDetail && self.CaptureData == other.CaptureData
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_DATA {}
impl ::core::fmt::Debug for WINBIO_CAPTURE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CAPTURE_DATA").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("RejectDetail", &self.RejectDetail).field("CaptureData", &self.CaptureData).finish()
    }
}
impl ::core::default::Default for WINBIO_CAPTURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_PARAMETERS {}
impl ::core::fmt::Debug for WINBIO_CAPTURE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CAPTURE_PARAMETERS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for WINBIO_COMPONENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_COMPONENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_COMPONENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WINBIO_DATA {}
impl ::core::fmt::Debug for WINBIO_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_DATA").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for WINBIO_DIAGNOSTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_DIAGNOSTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.VendorDiagnostics == other.VendorDiagnostics
    }
}
impl ::core::cmp::Eq for WINBIO_DIAGNOSTICS {}
impl ::core::fmt::Debug for WINBIO_DIAGNOSTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_DIAGNOSTICS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("VendorDiagnostics", &self.VendorDiagnostics).finish()
    }
}
impl ::core::default::Default for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags && self.NonceSize == other.NonceSize
    }
}
impl ::core::cmp::Eq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::core::fmt::Debug for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ENCRYPTED_CAPTURE_PARAMS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).field("NonceSize", &self.NonceSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_ENGINE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SubFactor == other.SubFactor
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_PARAMETERS").field("Size", &self.Size).field("SubFactor", &self.SubFactor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_EXTENDED_UNIT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_UNIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Availability == other.Availability && self.ReasonCode == other.ReasonCode
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_UNIT_STATUS {}
impl ::core::fmt::Debug for WINBIO_EXTENDED_UNIT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_UNIT_STATUS").field("Availability", &self.Availability).field("ReasonCode", &self.ReasonCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_FP_BU_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_FP_BU_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorAttached == other.SensorAttached && self.CreationResult == other.CreationResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_FP_BU_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_FP_BU_STATE").field("SensorAttached", &self.SensorAttached).field("CreationResult", &self.CreationResult).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_FRAMEWORK_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_GESTURE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_GESTURE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BiometricType == other.BiometricType && self.MatchType == other.MatchType && self.ProtectionType == other.ProtectionType
    }
}
impl ::core::cmp::Eq for WINBIO_GESTURE_METADATA {}
impl ::core::fmt::Debug for WINBIO_GESTURE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_GESTURE_METADATA").field("Size", &self.Size).field("BiometricType", &self.BiometricType).field("MatchType", &self.MatchType).field("ProtectionType", &self.ProtectionType).finish()
    }
}
impl ::core::default::Default for WINBIO_GET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_GET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::core::cmp::Eq for WINBIO_GET_INDICATOR {}
impl ::core::fmt::Debug for WINBIO_GET_INDICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_GET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
impl ::core::default::Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_NOTIFY_WAKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_NOTIFY_WAKE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.Reason == other.Reason
    }
}
impl ::core::cmp::Eq for WINBIO_NOTIFY_WAKE {}
impl ::core::fmt::Debug for WINBIO_NOTIFY_WAKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_NOTIFY_WAKE").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("Reason", &self.Reason).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_PIPELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_PIPELINE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorHandle == other.SensorHandle && self.EngineHandle == other.EngineHandle && self.StorageHandle == other.StorageHandle && self.SensorInterface == other.SensorInterface && self.EngineInterface == other.EngineInterface && self.StorageInterface == other.StorageInterface && self.SensorContext == other.SensorContext && self.EngineContext == other.EngineContext && self.StorageContext == other.StorageContext && self.FrameworkInterface == other.FrameworkInterface
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_PIPELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PIPELINE")
            .field("SensorHandle", &self.SensorHandle)
            .field("EngineHandle", &self.EngineHandle)
            .field("StorageHandle", &self.StorageHandle)
            .field("SensorInterface", &self.SensorInterface)
            .field("EngineInterface", &self.EngineInterface)
            .field("StorageInterface", &self.StorageInterface)
            .field("SensorContext", &self.SensorContext)
            .field("EngineContext", &self.EngineContext)
            .field("StorageContext", &self.StorageContext)
            .field("FrameworkInterface", &self.FrameworkInterface)
            .finish()
    }
}
impl ::core::default::Default for WINBIO_POLICY_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_POLICY_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_POLICY_SOURCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_POOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_POOL").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.PrivateSensorTypeInfo == other.PrivateSensorTypeInfo
    }
}
impl ::core::cmp::Eq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::core::fmt::Debug for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRIVATE_SENSOR_TYPE_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("PrivateSensorTypeInfo", &self.PrivateSensorTypeInfo).finish()
    }
}
impl ::core::default::Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_REGISTERED_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_REGISTERED_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for WINBIO_REGISTERED_FORMAT {}
impl ::core::fmt::Debug for WINBIO_REGISTERED_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_REGISTERED_FORMAT").field("Owner", &self.Owner).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.Flags == other.Flags && self.ValidationTag == other.ValidationTag
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::core::fmt::Debug for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_BUFFER_HEADER_V1").field("Type", &self.Type).field("Size", &self.Size).field("Flags", &self.Flags).field("ValidationTag", &self.ValidationTag).finish()
    }
}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Flags == other.Flags && self.ModelCertificateSize == other.ModelCertificateSize && self.IntermediateCA1Size == other.IntermediateCA1Size && self.IntermediateCA2Size == other.IntermediateCA2Size
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_DATA {}
impl ::core::fmt::Debug for WINBIO_SECURE_CONNECTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_CONNECTION_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Flags", &self.Flags).field("ModelCertificateSize", &self.ModelCertificateSize).field("IntermediateCA1Size", &self.IntermediateCA1Size).field("IntermediateCA2Size", &self.IntermediateCA2Size).finish()
    }
}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Version == other.Version && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::core::fmt::Debug for WINBIO_SECURE_CONNECTION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_CONNECTION_PARAMS").field("PayloadSize", &self.PayloadSize).field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for WINBIO_SENSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SENSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.WinBioVersion == other.WinBioVersion && self.SensorType == other.SensorType && self.SensorSubType == other.SensorSubType && self.Capabilities == other.Capabilities && self.ManufacturerName == other.ManufacturerName && self.ModelName == other.ModelName && self.SerialNumber == other.SerialNumber && self.FirmwareVersion == other.FirmwareVersion && self.SupportedFormatEntries == other.SupportedFormatEntries && self.SupportedFormat == other.SupportedFormat
    }
}
impl ::core::cmp::Eq for WINBIO_SENSOR_ATTRIBUTES {}
impl ::core::fmt::Debug for WINBIO_SENSOR_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SENSOR_ATTRIBUTES")
            .field("PayloadSize", &self.PayloadSize)
            .field("WinBioHresult", &self.WinBioHresult)
            .field("WinBioVersion", &self.WinBioVersion)
            .field("SensorType", &self.SensorType)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("ManufacturerName", &self.ManufacturerName)
            .field("ModelName", &self.ModelName)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .field("SupportedFormatEntries", &self.SupportedFormatEntries)
            .field("SupportedFormat", &self.SupportedFormat)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_SENSOR_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_SETTING_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINBIO_SETTING_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_SETTING_SOURCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINBIO_SET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::core::cmp::Eq for WINBIO_SET_INDICATOR {}
impl ::core::fmt::Debug for WINBIO_SET_INDICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_STORAGE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINBIO_STORAGE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.SubFactor == other.SubFactor && self.IndexVector == other.IndexVector && self.IndexElementCount == other.IndexElementCount && self.TemplateBlob == other.TemplateBlob && self.TemplateBlobSize == other.TemplateBlobSize && self.PayloadBlob == other.PayloadBlob && self.PayloadBlobSize == other.PayloadBlobSize
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_RECORD {}
impl ::core::fmt::Debug for WINBIO_STORAGE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_STORAGE_RECORD").field("Identity", &self.Identity).field("SubFactor", &self.SubFactor).field("IndexVector", &self.IndexVector).field("IndexElementCount", &self.IndexElementCount).field("TemplateBlob", &self.TemplateBlob).field("TemplateBlobSize", &self.TemplateBlobSize).field("PayloadBlob", &self.PayloadBlob).field("PayloadBlobSize", &self.PayloadBlobSize).finish()
    }
}
impl ::core::default::Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.DatabaseId == other.DatabaseId && self.DataFormat == other.DataFormat && self.Attributes == other.Attributes && self.FilePath == other.FilePath && self.ConnectionString == other.ConnectionString
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_SCHEMA {}
impl ::core::fmt::Debug for WINBIO_STORAGE_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_STORAGE_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("DatabaseId", &self.DatabaseId).field("DataFormat", &self.DataFormat).field("Attributes", &self.Attributes).field("FilePath", &self.FilePath).field("ConnectionString", &self.ConnectionString).finish()
    }
}
impl ::core::default::Default for WINBIO_SUPPORTED_ALGORITHMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_SUPPORTED_ALGORITHMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.NumberOfAlgorithms == other.NumberOfAlgorithms && self.AlgorithmData == other.AlgorithmData
    }
}
impl ::core::cmp::Eq for WINBIO_SUPPORTED_ALGORITHMS {}
impl ::core::fmt::Debug for WINBIO_SUPPORTED_ALGORITHMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SUPPORTED_ALGORITHMS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("NumberOfAlgorithms", &self.NumberOfAlgorithms).field("AlgorithmData", &self.AlgorithmData).finish()
    }
}
impl ::core::default::Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_UNIT_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.PoolType == other.PoolType && self.BiometricFactor == other.BiometricFactor && self.SensorSubType == other.SensorSubType && self.Capabilities == other.Capabilities && self.DeviceInstanceId == other.DeviceInstanceId && self.Description == other.Description && self.Manufacturer == other.Manufacturer && self.Model == other.Model && self.SerialNumber == other.SerialNumber && self.FirmwareVersion == other.FirmwareVersion
    }
}
impl ::core::cmp::Eq for WINBIO_UNIT_SCHEMA {}
impl ::core::fmt::Debug for WINBIO_UNIT_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_UNIT_SCHEMA")
            .field("UnitId", &self.UnitId)
            .field("PoolType", &self.PoolType)
            .field("BiometricFactor", &self.BiometricFactor)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("DeviceInstanceId", &self.DeviceInstanceId)
            .field("Description", &self.Description)
            .field("Manufacturer", &self.Manufacturer)
            .field("Model", &self.Model)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .finish()
    }
}
impl ::core::default::Default for WINBIO_UPDATE_FIRMWARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_UPDATE_FIRMWARE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.FirmwareData == other.FirmwareData
    }
}
impl ::core::cmp::Eq for WINBIO_UPDATE_FIRMWARE {}
impl ::core::fmt::Debug for WINBIO_UPDATE_FIRMWARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_UPDATE_FIRMWARE").field("PayloadSize", &self.PayloadSize).field("FirmwareData", &self.FirmwareData).finish()
    }
}
impl ::core::default::Default for WINBIO_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINBIO_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for WINBIO_VERSION {}
impl ::core::fmt::Debug for WINBIO_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
