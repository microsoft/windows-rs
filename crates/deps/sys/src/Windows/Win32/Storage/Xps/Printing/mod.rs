#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob(
        printername: super::super::super::Foundation::PWSTR,
        jobname: super::super::super::Foundation::PWSTR,
        outputfilename: super::super::super::Foundation::PWSTR,
        progressevent: super::super::super::Foundation::HANDLE,
        completionevent: super::super::super::Foundation::HANDLE,
        printablepageson: *const u8,
        printablepagesoncount: u32,
        xpsprintjob: *mut IXpsPrintJob,
        documentstream: *mut IXpsPrintJobStream,
        printticketstream: *mut IXpsPrintJobStream,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut IXpsPrintJob, printcontentreceiver: *mut super::IXpsOMPackageTarget) -> ::windows_sys::core::HRESULT;
}
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2628665512,
    data2: 57041,
    data3: 16841,
    data4: [169, 253, 215, 53, 239, 51, 174, 218],
};
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 5684082, data2: 35996, data3: 17938, data4: [189, 15, 147, 1, 42, 135, 9, 157] };
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1675351840,
    data2: 35604,
    data3: 17783,
    data4: [176, 116, 123, 177, 27, 89, 109, 40],
};
#[repr(transparent)]
pub struct IPrintDocumentPackageStatusEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDocumentPackageTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDocumentPackageTargetFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXpsPrintJobStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintDocumentPackageCompletion(pub i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
#[repr(C)]
pub struct PrintDocumentPackageStatus(i32);
pub const PrintDocumentPackageTarget: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1212311198,
    data2: 39239,
    data3: 18154,
    data4: [139, 162, 216, 204, 228, 50, 194, 202],
};
pub const PrintDocumentPackageTargetFactory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 881783165,
    data2: 27777,
    data3: 18818,
    data4: [146, 180, 238, 24, 138, 67, 134, 122],
};
#[repr(transparent)]
pub struct XPS_JOB_COMPLETION(pub i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
#[repr(C)]
pub struct XPS_JOB_STATUS(i32);
