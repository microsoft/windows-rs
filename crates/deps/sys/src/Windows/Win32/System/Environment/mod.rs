#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallEnclave();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnclave();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnvironmentBlock();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnclave();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyEnvironmentBlock();
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveGetAttestationReport();
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveGetEnclaveInformation();
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveSealData();
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveUnsealData();
    #[doc = "*Required features: `Win32_System_Environment`*"]
    pub fn EnclaveVerifyAttestationReport();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsForUserA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsForUserW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpandEnvironmentStringsW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeEnvironmentStringsA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeEnvironmentStringsW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommandLineA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommandLineW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentDirectoryA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentDirectoryW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentStrings();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentStringsW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentVariableA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnvironmentVariableW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeEnclave();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsEnclaveTypeSupported();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveData();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveImageA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadEnclaveImageW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedCurrentDirectoryForExePathA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedCurrentDirectoryForExePathW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentDirectoryA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentDirectoryW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentStringsW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentVariableA();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentVariableW();
    #[doc = "*Required features: `Win32_System_Environment`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateEnclave();
}
