#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallEnclave(lproutine: isize, lpparameter: *const ::core::ffi::c_void, fwaitforthread: super::super::Foundation::BOOL, lpreturnvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnclave(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnvironmentBlock(lpenvironment: *mut *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE, binherit: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnclave(lpaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyEnvironmentBlock(lpenvironment: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveGetAttestationReport(enclavedata: *const u8, report: *mut ::core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveGetEnclaveInformation(informationsize: u32, enclaveinformation: *mut ENCLAVE_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveSealData(datatoencrypt: *const ::core::ffi::c_void, datatoencryptsize: u32, identitypolicy: ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy: u32, protectedblob: *mut ::core::ffi::c_void, buffersize: u32, protectedblobsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveUnsealData(protectedblob: *const ::core::ffi::c_void, protectedblobsize: u32, decrypteddata: *mut ::core::ffi::c_void, buffersize: u32, decrypteddatasize: *mut u32, sealingidentity: *mut ENCLAVE_IDENTITY, unsealingflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveVerifyAttestationReport(enclavetype: u32, report: *const ::core::ffi::c_void, reportsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsA(lpsrc: super::super::Foundation::PSTR, lpdst: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsForUserA(htoken: super::super::Foundation::HANDLE, lpsrc: super::super::Foundation::PSTR, lpdest: super::super::Foundation::PSTR, dwsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsForUserW(htoken: super::super::Foundation::HANDLE, lpsrc: super::super::Foundation::PWSTR, lpdest: super::super::Foundation::PWSTR, dwsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsW(lpsrc: super::super::Foundation::PWSTR, lpdst: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeEnvironmentStringsA(penv: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeEnvironmentStringsW(penv: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommandLineA() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommandLineW() -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentStrings() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentStringsW() -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpbuffer: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpbuffer: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeEnclave(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsEnclaveTypeSupported(flenclavetype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveData(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveImageA(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveImageW(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedCurrentDirectoryForExePathA(exename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedCurrentDirectoryForExePathW(exename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentStringsW(newenvironment: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateEnclave(lpaddress: *const ::core::ffi::c_void, fwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
pub struct ENCLAVE_IDENTITY(i32);
pub struct ENCLAVE_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_REPORT_DATA_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: u32 = 1u32;
pub struct ENCLAVE_SEALING_IDENTITY_POLICY(i32);
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_UNSEAL_FLAG_STALE_KEY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_DEBUG_KEY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_FAMILY_ID: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_IMAGE_ID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_MEASUREMENT: u32 = 1u32;
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD(i32);
pub struct VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT(i32);
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64(i32);
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE(i32);
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32(i32);
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64(i32);
pub struct VBS_ENCLAVE_REPORT(i32);
pub struct VBS_ENCLAVE_REPORT_MODULE(i32);
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER(i32);
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER(i32);
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
