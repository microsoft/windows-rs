#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNamedPipeA();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNamedPipeW();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ConnectNamedPipe();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeA();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeW();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePipe();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisconnectNamedPipe();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientComputerNameA();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientComputerNameW();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientProcessId();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientSessionId();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeHandleStateA();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeHandleStateW();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeInfo();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeServerProcessId();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeServerSessionId();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateNamedPipeClient();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekNamedPipe();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedPipeHandleState();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TransactNamedPipe();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitNamedPipeA();
    #[doc = "*Required features: `Win32_System_Pipes`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitNamedPipeW();
}
