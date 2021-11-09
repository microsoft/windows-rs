#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallEnclave<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lproutine: isize, lpparameter: *const ::core::ffi::c_void, fwaitforthread: Param2, lpreturnvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallEnclave(lproutine: isize, lpparameter: *const ::core::ffi::c_void, fwaitforthread: super::super::Foundation::BOOL, lpreturnvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CallEnclave(::core::mem::transmute(lproutine), ::core::mem::transmute(lpparameter), fwaitforthread.into_param().abi(), ::core::mem::transmute(lpreturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnclave<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEnclave(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CreateEnclave(
            hprocess.into_param().abi(),
            ::core::mem::transmute(lpaddress),
            ::core::mem::transmute(dwsize),
            ::core::mem::transmute(dwinitialcommitment),
            ::core::mem::transmute(flenclavetype),
            ::core::mem::transmute(lpenclaveinformation),
            ::core::mem::transmute(dwinfolength),
            ::core::mem::transmute(lpenclaveerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnvironmentBlock<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpenvironment: *mut *mut ::core::ffi::c_void, htoken: Param1, binherit: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEnvironmentBlock(lpenvironment: *mut *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE, binherit: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateEnvironmentBlock(::core::mem::transmute(lpenvironment), htoken.into_param().abi(), binherit.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteEnclave(lpaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteEnclave(lpaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteEnclave(::core::mem::transmute(lpaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyEnvironmentBlock(lpenvironment: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyEnvironmentBlock(lpenvironment: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DestroyEnvironmentBlock(::core::mem::transmute(lpenvironment)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [u8; 32],
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSvn: u32,
    pub SecureKernelSvn: u32,
    pub PlatformSvn: u32,
    pub Flags: u32,
    pub SigningLevel: u32,
    pub EnclaveType: u32,
}
impl ENCLAVE_IDENTITY {}
impl ::core::default::Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_IDENTITY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ENCLAVE_IDENTITY {}
unsafe impl ::windows::runtime::Abi for ENCLAVE_IDENTITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Identity: ENCLAVE_IDENTITY,
}
impl ENCLAVE_INFORMATION {}
impl ::core::default::Default for ENCLAVE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ENCLAVE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ENCLAVE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_REPORT_DATA_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENCLAVE_SEALING_IDENTITY_POLICY(pub i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(0i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(1i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(2i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(3i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(4i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(5i32);
impl ::core::convert::From<i32> for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENCLAVE_SEALING_IDENTITY_POLICY {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: u32,
    pub Flags: u32,
    pub EnclaveSVN: u32,
    pub SystemKeyID: u32,
    pub CurrentSystemKeyID: u32,
}
impl ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl ::core::default::Default for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENCLAVE_VBS_BASIC_KEY_REQUEST").field("RequestSize", &self.RequestSize).field("Flags", &self.Flags).field("EnclaveSVN", &self.EnclaveSVN).field("SystemKeyID", &self.SystemKeyID).field("CurrentSystemKeyID", &self.CurrentSystemKeyID).finish()
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.Flags == other.Flags && self.EnclaveSVN == other.EnclaveSVN && self.SystemKeyID == other.SystemKeyID && self.CurrentSystemKeyID == other.CurrentSystemKeyID
    }
}
impl ::core::cmp::Eq for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
unsafe impl ::windows::runtime::Abi for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Environment`*"]
#[inline]
pub unsafe fn EnclaveGetAttestationReport(enclavedata: *const u8, report: *mut ::core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnclaveGetAttestationReport(enclavedata: *const u8, report: *mut ::core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> ::windows::runtime::HRESULT;
        }
        EnclaveGetAttestationReport(::core::mem::transmute(enclavedata), ::core::mem::transmute(report), ::core::mem::transmute(buffersize), ::core::mem::transmute(outputsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
#[inline]
pub unsafe fn EnclaveGetEnclaveInformation(informationsize: u32) -> ::windows::runtime::Result<ENCLAVE_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnclaveGetEnclaveInformation(informationsize: u32, enclaveinformation: *mut ENCLAVE_INFORMATION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ENCLAVE_INFORMATION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        EnclaveGetEnclaveInformation(::core::mem::transmute(informationsize), &mut result__).from_abi::<ENCLAVE_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
#[inline]
pub unsafe fn EnclaveSealData(datatoencrypt: *const ::core::ffi::c_void, datatoencryptsize: u32, identitypolicy: ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy: u32, protectedblob: *mut ::core::ffi::c_void, buffersize: u32, protectedblobsize: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnclaveSealData(datatoencrypt: *const ::core::ffi::c_void, datatoencryptsize: u32, identitypolicy: ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy: u32, protectedblob: *mut ::core::ffi::c_void, buffersize: u32, protectedblobsize: *mut u32) -> ::windows::runtime::HRESULT;
        }
        EnclaveSealData(::core::mem::transmute(datatoencrypt), ::core::mem::transmute(datatoencryptsize), ::core::mem::transmute(identitypolicy), ::core::mem::transmute(runtimepolicy), ::core::mem::transmute(protectedblob), ::core::mem::transmute(buffersize), ::core::mem::transmute(protectedblobsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
#[inline]
pub unsafe fn EnclaveUnsealData(protectedblob: *const ::core::ffi::c_void, protectedblobsize: u32, decrypteddata: *mut ::core::ffi::c_void, buffersize: u32, decrypteddatasize: *mut u32, sealingidentity: *mut ENCLAVE_IDENTITY, unsealingflags: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnclaveUnsealData(protectedblob: *const ::core::ffi::c_void, protectedblobsize: u32, decrypteddata: *mut ::core::ffi::c_void, buffersize: u32, decrypteddatasize: *mut u32, sealingidentity: *mut ENCLAVE_IDENTITY, unsealingflags: *mut u32) -> ::windows::runtime::HRESULT;
        }
        EnclaveUnsealData(::core::mem::transmute(protectedblob), ::core::mem::transmute(protectedblobsize), ::core::mem::transmute(decrypteddata), ::core::mem::transmute(buffersize), ::core::mem::transmute(decrypteddatasize), ::core::mem::transmute(sealingidentity), ::core::mem::transmute(unsealingflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
#[inline]
pub unsafe fn EnclaveVerifyAttestationReport(enclavetype: u32, report: *const ::core::ffi::c_void, reportsize: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnclaveVerifyAttestationReport(enclavetype: u32, report: *const ::core::ffi::c_void, reportsize: u32) -> ::windows::runtime::HRESULT;
        }
        EnclaveVerifyAttestationReport(::core::mem::transmute(enclavetype), ::core::mem::transmute(report), ::core::mem::transmute(reportsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpsrc: Param0, lpdst: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpandEnvironmentStringsA(lpsrc: super::super::Foundation::PSTR, lpdst: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(ExpandEnvironmentStringsA(lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(htoken: Param0, lpsrc: Param1, lpdest: super::super::Foundation::PSTR, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpandEnvironmentStringsForUserA(htoken: super::super::Foundation::HANDLE, lpsrc: super::super::Foundation::PSTR, lpdest: super::super::Foundation::PSTR, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ExpandEnvironmentStringsForUserA(htoken.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdest), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htoken: Param0, lpsrc: Param1, lpdest: super::super::Foundation::PWSTR, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpandEnvironmentStringsForUserW(htoken: super::super::Foundation::HANDLE, lpsrc: super::super::Foundation::PWSTR, lpdest: super::super::Foundation::PWSTR, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ExpandEnvironmentStringsForUserW(htoken.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdest), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsrc: Param0, lpdst: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpandEnvironmentStringsW(lpsrc: super::super::Foundation::PWSTR, lpdst: super::super::Foundation::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(ExpandEnvironmentStringsW(lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeEnvironmentStringsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(penv: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeEnvironmentStringsA(penv: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeEnvironmentStringsA(penv.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeEnvironmentStringsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(penv: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeEnvironmentStringsW(penv: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeEnvironmentStringsW(penv.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommandLineA() -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommandLineA() -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(GetCommandLineA())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommandLineW() -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommandLineW() -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(GetCommandLineW())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(GetCurrentDirectoryA(::core::mem::transmute(nbufferlength), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(GetCurrentDirectoryW(::core::mem::transmute(nbufferlength), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnvironmentStrings() -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnvironmentStrings() -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(GetEnvironmentStrings())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnvironmentStringsW() -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnvironmentStringsW() -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(GetEnvironmentStringsW())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnvironmentVariableA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpbuffer: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpbuffer: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetEnvironmentVariableA(lpname.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnvironmentVariableW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpbuffer: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpbuffer: super::super::Foundation::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetEnvironmentVariableW(lpname.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeEnclave<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeEnclave(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeEnclave(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(lpenclaveinformation), ::core::mem::transmute(dwinfolength), ::core::mem::transmute(lpenclaveerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsEnclaveTypeSupported(flenclavetype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsEnclaveTypeSupported(flenclavetype: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsEnclaveTypeSupported(::core::mem::transmute(flenclavetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadEnclaveData(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LoadEnclaveData(
            hprocess.into_param().abi(),
            ::core::mem::transmute(lpaddress),
            ::core::mem::transmute(lpbuffer),
            ::core::mem::transmute(nsize),
            ::core::mem::transmute(flprotect),
            ::core::mem::transmute(lppageinformation),
            ::core::mem::transmute(dwinfolength),
            ::core::mem::transmute(lpnumberofbyteswritten),
            ::core::mem::transmute(lpenclaveerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveImageA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadEnclaveImageA(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LoadEnclaveImageA(::core::mem::transmute(lpenclaveaddress), lpimagename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveImageW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadEnclaveImageW(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LoadEnclaveImageW(::core::mem::transmute(lpenclaveaddress), lpimagename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(exename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedCurrentDirectoryForExePathA(exename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NeedCurrentDirectoryForExePathA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(exename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedCurrentDirectoryForExePathW(exename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NeedCurrentDirectoryForExePathW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentDirectoryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lppathname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCurrentDirectoryA(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentDirectoryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lppathname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCurrentDirectoryW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentStringsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(newenvironment: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnvironmentStringsW(newenvironment: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetEnvironmentStringsW(newenvironment.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentVariableA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpvalue: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetEnvironmentVariableA(lpname.into_param().abi(), lpvalue.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentVariableW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpvalue: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetEnvironmentVariableW(lpname.into_param().abi(), lpvalue.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateEnclave<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpaddress: *const ::core::ffi::c_void, fwait: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateEnclave(lpaddress: *const ::core::ffi::c_void, fwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateEnclave(::core::mem::transmute(lpaddress), fwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofbytes: usize, sourceaddress: *const ::core::ffi::c_void, pageprotection: u32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES = unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofbytes: usize) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY = unsafe extern "system" fn(keyrequest: *mut ENCLAVE_VBS_BASIC_KEY_REQUEST, requestedkeysize: u32, returnedkey: *mut u8) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA = unsafe extern "system" fn(buffer: *mut u8, numberofbytes: u32, generation: *mut u64) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT = unsafe extern "system" fn(enclavedata: *const u8, report: *mut ::core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION = unsafe extern "system" fn(enclaveinfo: *mut ENCLAVE_INFORMATION) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES = unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofytes: usize, pageprotection: u32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = unsafe extern "system" fn(returnvalue: usize);
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86_64",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = unsafe extern "system" fn(exceptionrecord: *const VBS_BASIC_ENCLAVE_EXCEPTION_AMD64) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86", target_arch = "aarch64",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = unsafe extern "system" fn(exceptionrecord: *const ::core::ffi::c_void) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
#[cfg(any(target_arch = "x86",))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT = unsafe extern "system" fn(report: *const ::core::ffi::c_void, reportsize: u32) -> i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    pub ExceptionCode: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 3],
    pub ExceptionRAX: usize,
    pub ExceptionRCX: usize,
    pub ExceptionRIP: usize,
    pub ExceptionRFLAGS: usize,
    pub ExceptionRSP: usize,
}
impl VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBS_BASIC_ENCLAVE_EXCEPTION_AMD64")
            .field("ExceptionCode", &self.ExceptionCode)
            .field("NumberParameters", &self.NumberParameters)
            .field("ExceptionInformation", &self.ExceptionInformation)
            .field("ExceptionRAX", &self.ExceptionRAX)
            .field("ExceptionRCX", &self.ExceptionRCX)
            .field("ExceptionRIP", &self.ExceptionRIP)
            .field("ExceptionRFLAGS", &self.ExceptionRFLAGS)
            .field("ExceptionRSP", &self.ExceptionRSP)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation && self.ExceptionRAX == other.ExceptionRAX && self.ExceptionRCX == other.ExceptionRCX && self.ExceptionRIP == other.ExceptionRIP && self.ExceptionRFLAGS == other.ExceptionRFLAGS && self.ExceptionRSP == other.ExceptionRSP
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
unsafe impl ::windows::runtime::Abi for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    pub ReturnFromEnclave: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE>,
    pub ReturnFromException: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION>,
    pub TerminateThread: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD>,
    pub InterruptThread: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD>,
    pub CommitPages: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES>,
    pub DecommitPages: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES>,
    pub ProtectPages: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES>,
    pub CreateThread: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD>,
    pub GetEnclaveInformation: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION>,
    pub GenerateKey: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY>,
    pub GenerateReport: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT>,
    pub VerifyReport: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT>,
    pub GenerateRandomData: ::core::option::Option<VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA>,
}
impl VBS_BASIC_ENCLAVE_SYSCALL_PAGE {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBS_BASIC_ENCLAVE_SYSCALL_PAGE").finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ReturnFromEnclave.map(|f| f as usize) == other.ReturnFromEnclave.map(|f| f as usize)
            && self.ReturnFromException.map(|f| f as usize) == other.ReturnFromException.map(|f| f as usize)
            && self.TerminateThread.map(|f| f as usize) == other.TerminateThread.map(|f| f as usize)
            && self.InterruptThread.map(|f| f as usize) == other.InterruptThread.map(|f| f as usize)
            && self.CommitPages.map(|f| f as usize) == other.CommitPages.map(|f| f as usize)
            && self.DecommitPages.map(|f| f as usize) == other.DecommitPages.map(|f| f as usize)
            && self.ProtectPages.map(|f| f as usize) == other.ProtectPages.map(|f| f as usize)
            && self.CreateThread.map(|f| f as usize) == other.CreateThread.map(|f| f as usize)
            && self.GetEnclaveInformation.map(|f| f as usize) == other.GetEnclaveInformation.map(|f| f as usize)
            && self.GenerateKey.map(|f| f as usize) == other.GenerateKey.map(|f| f as usize)
            && self.GenerateReport.map(|f| f as usize) == other.GenerateReport.map(|f| f as usize)
            && self.VerifyReport.map(|f| f as usize) == other.VerifyReport.map(|f| f as usize)
            && self.GenerateRandomData.map(|f| f as usize) == other.GenerateRandomData.map(|f| f as usize)
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {}
unsafe impl ::windows::runtime::Abi for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [u32; 4],
    pub EntryPoint: u32,
    pub StackPointer: u32,
    pub ExceptionEntryPoint: u32,
    pub ExceptionStack: u32,
    pub ExceptionActive: u32,
}
impl VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32")
            .field("ThreadContext", &self.ThreadContext)
            .field("EntryPoint", &self.EntryPoint)
            .field("StackPointer", &self.StackPointer)
            .field("ExceptionEntryPoint", &self.ExceptionEntryPoint)
            .field("ExceptionStack", &self.ExceptionStack)
            .field("ExceptionActive", &self.ExceptionActive)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
unsafe impl ::windows::runtime::Abi for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [u64; 4],
    pub EntryPoint: u64,
    pub StackPointer: u64,
    pub ExceptionEntryPoint: u64,
    pub ExceptionStack: u64,
    pub ExceptionActive: u32,
}
impl VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64")
            .field("ThreadContext", &self.ThreadContext)
            .field("EntryPoint", &self.EntryPoint)
            .field("StackPointer", &self.StackPointer)
            .field("ExceptionEntryPoint", &self.ExceptionEntryPoint)
            .field("ExceptionStack", &self.ExceptionStack)
            .field("ExceptionActive", &self.ExceptionActive)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
unsafe impl ::windows::runtime::Abi for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: u32,
    pub ReportVersion: u32,
    pub EnclaveData: [u8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
}
impl VBS_ENCLAVE_REPORT {}
impl ::core::default::Default for VBS_ENCLAVE_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT {}
unsafe impl ::windows::runtime::Abi for VBS_ENCLAVE_REPORT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_ENCLAVE_REPORT_MODULE {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub Svn: u32,
    pub ModuleName: [u16; 1],
}
impl VBS_ENCLAVE_REPORT_MODULE {}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_MODULE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_MODULE {}
unsafe impl ::windows::runtime::Abi for VBS_ENCLAVE_REPORT_MODULE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER {
    pub PackageSize: u32,
    pub Version: u32,
    pub SignatureScheme: u32,
    pub SignedStatementSize: u32,
    pub SignatureSize: u32,
    pub Reserved: u32,
}
impl VBS_ENCLAVE_REPORT_PKG_HEADER {}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_PKG_HEADER {}
unsafe impl ::windows::runtime::Abi for VBS_ENCLAVE_REPORT_PKG_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_System_Environment`*"]
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: u32,
    pub Size: u32,
}
impl VBS_ENCLAVE_REPORT_VARDATA_HEADER {}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_VARDATA_HEADER {}
unsafe impl ::windows::runtime::Abi for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Environment`*"]
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
