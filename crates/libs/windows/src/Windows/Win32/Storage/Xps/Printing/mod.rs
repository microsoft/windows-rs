pub const ID_DOCUMENTPACKAGETARGET_MSXPS: windows_core::GUID = windows_core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: windows_core::GUID = windows_core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: windows_core::GUID = windows_core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = 2i32;
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = 1i32;
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = 3i32;
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = 0i32;
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2i32;
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1i32;
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3i32;
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PrintDocumentPackageCompletion(pub i32);
impl windows_core::TypeKind for PrintDocumentPackageCompletion {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_JOB_COMPLETION(pub i32);
impl windows_core::TypeKind for XPS_JOB_COMPLETION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: windows_core::HRESULT,
}
impl Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PrintDocumentPackageStatus {
    type TypeKind = windows_core::CopyType;
}
pub const PrintDocumentPackageTarget: windows_core::GUID = windows_core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
pub const PrintDocumentPackageTargetFactory: windows_core::GUID = windows_core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_core::HRESULT,
}
impl Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_JOB_STATUS {
    type TypeKind = windows_core::CopyType;
}
