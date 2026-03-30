pub type DODownloadCostPolicy = i32;
pub const DODownloadCostPolicy_Always: DODownloadCostPolicy = 0i32;
pub const DODownloadCostPolicy_NoCellular: DODownloadCostPolicy = 5i32;
pub const DODownloadCostPolicy_NoRoaming: DODownloadCostPolicy = 3i32;
pub const DODownloadCostPolicy_NoSurcharge: DODownloadCostPolicy = 4i32;
pub const DODownloadCostPolicy_Standard: DODownloadCostPolicy = 2i32;
pub const DODownloadCostPolicy_Unrestricted: DODownloadCostPolicy = 1i32;
pub type DODownloadProperty = i32;
pub const DODownloadProperty_BlockingMode: DODownloadProperty = 12i32;
pub const DODownloadProperty_CallbackFreqPercent: DODownloadProperty = 8i32;
pub const DODownloadProperty_CallbackFreqSeconds: DODownloadProperty = 9i32;
pub const DODownloadProperty_CallbackInterface: DODownloadProperty = 13i32;
pub const DODownloadProperty_ContentId: DODownloadProperty = 2i32;
pub const DODownloadProperty_CorrelationVector: DODownloadProperty = 17i32;
pub const DODownloadProperty_CostPolicy: DODownloadProperty = 6i32;
pub const DODownloadProperty_DecryptionInfo: DODownloadProperty = 18i32;
pub const DODownloadProperty_DisallowOnCellular: DODownloadProperty = 22i32;
pub const DODownloadProperty_DisplayName: DODownloadProperty = 3i32;
pub const DODownloadProperty_ForegroundPriority: DODownloadProperty = 11i32;
pub const DODownloadProperty_HttpAllowSecureToNonSecureRedirect: DODownloadProperty = 24i32;
pub const DODownloadProperty_HttpCustomAuthHeaders: DODownloadProperty = 23i32;
pub const DODownloadProperty_HttpCustomHeaders: DODownloadProperty = 5i32;
pub const DODownloadProperty_HttpRedirectionTarget: DODownloadProperty = 26i32;
pub const DODownloadProperty_HttpResponseHeaders: DODownloadProperty = 27i32;
pub const DODownloadProperty_HttpServerIPAddress: DODownloadProperty = 28i32;
pub const DODownloadProperty_HttpStatusCode: DODownloadProperty = 29i32;
pub const DODownloadProperty_Id: DODownloadProperty = 0i32;
pub const DODownloadProperty_IntegrityCheckInfo: DODownloadProperty = 19i32;
pub const DODownloadProperty_IntegrityCheckMandatory: DODownloadProperty = 20i32;
pub const DODownloadProperty_LocalPath: DODownloadProperty = 4i32;
pub const DODownloadProperty_NetworkToken: DODownloadProperty = 16i32;
pub const DODownloadProperty_NoProgressTimeoutSeconds: DODownloadProperty = 10i32;
pub const DODownloadProperty_NonVolatile: DODownloadProperty = 25i32;
pub const DODownloadProperty_SecurityContext: DODownloadProperty = 15i32;
pub const DODownloadProperty_SecurityFlags: DODownloadProperty = 7i32;
pub const DODownloadProperty_StreamInterface: DODownloadProperty = 14i32;
pub const DODownloadProperty_TotalSizeBytes: DODownloadProperty = 21i32;
pub const DODownloadProperty_Uri: DODownloadProperty = 1i32;
pub type DODownloadState = i32;
pub const DODownloadState_Aborted: DODownloadState = 4i32;
pub const DODownloadState_Created: DODownloadState = 0i32;
pub const DODownloadState_Finalized: DODownloadState = 3i32;
pub const DODownloadState_Paused: DODownloadState = 5i32;
pub const DODownloadState_Transferred: DODownloadState = 2i32;
pub const DODownloadState_Transferring: DODownloadState = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DO_DOWNLOAD_ENUM_CATEGORY {
    pub Property: DODownloadProperty,
    pub Value: windows_sys::core::PCWSTR,
}
impl Default for DO_DOWNLOAD_ENUM_CATEGORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DO_DOWNLOAD_RANGE {
    pub Offset: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DO_DOWNLOAD_RANGES_INFO {
    pub RangeCount: u32,
    pub Ranges: [DO_DOWNLOAD_RANGE; 1],
}
impl Default for DO_DOWNLOAD_RANGES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DO_DOWNLOAD_STATUS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub State: DODownloadState,
    pub Error: windows_sys::core::HRESULT,
    pub ExtendedError: windows_sys::core::HRESULT,
}
pub const DecryptionInfo_AlgorithmName: windows_sys::core::PCWSTR = windows_sys::core::w!("AlgorithmName");
pub const DecryptionInfo_ChainingMode: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingMode");
pub const DecryptionInfo_EncryptionBufferSize: windows_sys::core::PCWSTR = windows_sys::core::w!("EncryptionBufferSize");
pub const DecryptionInfo_KeyData: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyData");
pub const DeliveryOptimization: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b99fa76_721c_423c_adac_56d03c8a8007);
pub const IntegrityCheckInfo_HashOfHashes: windows_sys::core::PCWSTR = windows_sys::core::w!("HashOfHashes");
pub const IntegrityCheckInfo_PiecesHashFileDigest: windows_sys::core::PCWSTR = windows_sys::core::w!("PiecesHashFileDigest");
pub const IntegrityCheckInfo_PiecesHashFileDigestAlgorithm: windows_sys::core::PCWSTR = windows_sys::core::w!("PiecesHashFileDigestAlgorithm");
pub const IntegrityCheckInfo_PiecesHashFileUrl: windows_sys::core::PCWSTR = windows_sys::core::w!("PiecesHashFileUrl");
