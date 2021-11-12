#![allow(non_snake_case, non_camel_case_types)]
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
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
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
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut IXpsPrintJob, printcontentreceiver: *mut super::IXpsOMPackageTarget) -> ::windows_sys::core::HRESULT;
}
