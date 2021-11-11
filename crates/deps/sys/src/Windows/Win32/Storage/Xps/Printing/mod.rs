#![allow(non_snake_case, non_camel_case_types)]
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
        xpsprintjob: *mut ::windows::runtime::RawPtr,
        documentstream: *mut ::windows::runtime::RawPtr,
        printticketstream: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut ::windows::runtime::RawPtr, printcontentreceiver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
