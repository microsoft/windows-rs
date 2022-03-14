#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallNamedPipeA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpnamedpipename: Param0, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNamedPipeA(lpnamedpipename: ::windows::core::PCSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CallNamedPipeA(lpnamedpipename.into_param().abi(), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesread), ::core::mem::transmute(ntimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallNamedPipeW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpnamedpipename: Param0, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNamedPipeW(lpnamedpipename: ::windows::core::PCWSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CallNamedPipeW(lpnamedpipename.into_param().abi(), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesread), ::core::mem::transmute(ntimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ConnectNamedPipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ConnectNamedPipe(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpname: Param0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateNamedPipeA(lpname: ::windows::core::PCSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateNamedPipeA(lpname.into_param().abi(), ::core::mem::transmute(dwopenmode), ::core::mem::transmute(dwpipemode), ::core::mem::transmute(nmaxinstances), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(ndefaulttimeout), ::core::mem::transmute(lpsecurityattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpname: Param0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateNamedPipeW(lpname: ::windows::core::PCWSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateNamedPipeW(lpname.into_param().abi(), ::core::mem::transmute(dwopenmode), ::core::mem::transmute(dwpipemode), ::core::mem::transmute(nmaxinstances), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(ndefaulttimeout), ::core::mem::transmute(lpsecurityattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES, nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES, nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreatePipe(::core::mem::transmute(hreadpipe), ::core::mem::transmute(hwritepipe), ::core::mem::transmute(lppipeattributes), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisconnectNamedPipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisconnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DisconnectNamedPipe(hnamedpipe.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, clientcomputername: ::windows::core::PSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientComputerNameA(pipe: super::super::Foundation::HANDLE, clientcomputername: ::windows::core::PSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeClientComputerNameA(pipe.into_param().abi(), ::core::mem::transmute(clientcomputername), ::core::mem::transmute(clientcomputernamelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, clientcomputername: ::windows::core::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientComputerNameW(pipe: super::super::Foundation::HANDLE, clientcomputername: ::windows::core::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeClientComputerNameW(pipe.into_param().abi(), ::core::mem::transmute(clientcomputername), ::core::mem::transmute(clientcomputernamelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientProcessId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, clientprocessid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientProcessId(pipe: super::super::Foundation::HANDLE, clientprocessid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeClientProcessId(pipe.into_param().abi(), ::core::mem::transmute(clientprocessid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientSessionId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, clientsessionid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientSessionId(pipe: super::super::Foundation::HANDLE, clientsessionid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeClientSessionId(pipe.into_param().abi(), ::core::mem::transmute(clientsessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeHandleStateA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: &mut [u8]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeHandleStateA(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows::core::PSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeHandleStateA(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpstate), ::core::mem::transmute(lpcurinstances), ::core::mem::transmute(lpmaxcollectioncount), ::core::mem::transmute(lpcollectdatatimeout), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpusername)), lpusername.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeHandleStateW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: &mut [u16]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeHandleStateW(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows::core::PWSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeHandleStateW(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpstate), ::core::mem::transmute(lpcurinstances), ::core::mem::transmute(lpmaxcollectioncount), ::core::mem::transmute(lpcollectdatatimeout), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpusername)), lpusername.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeInfo(hnamedpipe: super::super::Foundation::HANDLE, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeInfo(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpflags), ::core::mem::transmute(lpoutbuffersize), ::core::mem::transmute(lpinbuffersize), ::core::mem::transmute(lpmaxinstances)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeServerProcessId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, serverprocessid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeServerProcessId(pipe: super::super::Foundation::HANDLE, serverprocessid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeServerProcessId(pipe.into_param().abi(), ::core::mem::transmute(serverprocessid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeServerSessionId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pipe: Param0, serversessionid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeServerSessionId(pipe: super::super::Foundation::HANDLE, serversessionid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedPipeServerSessionId(pipe.into_param().abi(), ::core::mem::transmute(serversessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateNamedPipeClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateNamedPipeClient(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImpersonateNamedPipeClient(hnamedpipe.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAMED_PIPE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_WAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(2u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(4u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(8u32);
impl ::core::marker::Copy for NAMED_PIPE_MODE {}
impl ::core::clone::Clone for NAMED_PIPE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAMED_PIPE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NAMED_PIPE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NAMED_PIPE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAMED_PIPE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NAMED_PIPE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NAMED_PIPE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NAMED_PIPE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const NMPWAIT_NOWAIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Pipes\"`*"]
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekNamedPipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpbuffer: *mut ::core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PeekNamedPipe(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nbuffersize), ::core::mem::transmute(lpbytesread), ::core::mem::transmute(lptotalbytesavail), ::core::mem::transmute(lpbytesleftthismessage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNamedPipeHandleState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNamedPipeHandleState(hnamedpipe: super::super::Foundation::HANDLE, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetNamedPipeHandleState(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpmode), ::core::mem::transmute(lpmaxcollectioncount), ::core::mem::transmute(lpcollectdatatimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn TransactNamedPipe<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hnamedpipe: Param0, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TransactNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TransactNamedPipe(hnamedpipe.into_param().abi(), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesread), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitNamedPipeA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpnamedpipename: Param0, ntimeout: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitNamedPipeA(lpnamedpipename: ::windows::core::PCSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WaitNamedPipeA(lpnamedpipename.into_param().abi(), ::core::mem::transmute(ntimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Pipes\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitNamedPipeW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpnamedpipename: Param0, ntimeout: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitNamedPipeW(lpnamedpipename: ::windows::core::PCWSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WaitNamedPipeW(lpnamedpipename.into_param().abi(), ::core::mem::transmute(ntimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
