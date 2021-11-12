#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct PrintDocumentPackageCompletion(i32);
#[repr(C)]
pub struct PrintDocumentPackageStatus(i32);
#[repr(C)]
pub struct PrintDocumentPackageTarget(i32);
#[repr(C)]
pub struct PrintDocumentPackageTargetFactory(i32);
#[repr(C)]
pub struct XPS_JOB_COMPLETION(i32);
#[repr(C)]
pub struct XPS_JOB_STATUS(i32);
