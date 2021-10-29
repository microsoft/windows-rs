#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallNamedPipeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpnamedpipename: Param0,
    lpinbuffer: *const ::std::ffi::c_void,
    ninbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    noutbuffersize: u32,
    lpbytesread: *mut u32,
    ntimeout: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNamedPipeA(
                lpnamedpipename: super::super::Foundation::PSTR,
                lpinbuffer: *const ::std::ffi::c_void,
                ninbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                noutbuffersize: u32,
                lpbytesread: *mut u32,
                ntimeout: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CallNamedPipeA(
            lpnamedpipename.into_param().abi(),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(lpbytesread),
            ::std::mem::transmute(ntimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallNamedPipeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpnamedpipename: Param0,
    lpinbuffer: *const ::std::ffi::c_void,
    ninbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    noutbuffersize: u32,
    lpbytesread: *mut u32,
    ntimeout: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallNamedPipeW(
                lpnamedpipename: super::super::Foundation::PWSTR,
                lpinbuffer: *const ::std::ffi::c_void,
                ninbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                noutbuffersize: u32,
                lpbytesread: *mut u32,
                ntimeout: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CallNamedPipeW(
            lpnamedpipename.into_param().abi(),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(lpbytesread),
            ::std::mem::transmute(ntimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ConnectNamedPipe<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpoverlapped: *mut super::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConnectNamedPipe(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpoverlapped: *mut super::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConnectNamedPipe(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_Storage_FileSystem"
))]
#[inline]
pub unsafe fn CreateNamedPipeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpname: Param0,
    dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
    dwpipemode: NAMED_PIPE_MODE,
    nmaxinstances: u32,
    noutbuffersize: u32,
    ninbuffersize: u32,
    ndefaulttimeout: u32,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateNamedPipeA(
                lpname: super::super::Foundation::PSTR,
                dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
                dwpipemode: NAMED_PIPE_MODE,
                nmaxinstances: u32,
                noutbuffersize: u32,
                ninbuffersize: u32,
                ndefaulttimeout: u32,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateNamedPipeA(
            lpname.into_param().abi(),
            ::std::mem::transmute(dwopenmode),
            ::std::mem::transmute(dwpipemode),
            ::std::mem::transmute(nmaxinstances),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(ndefaulttimeout),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_Storage_FileSystem"
))]
#[inline]
pub unsafe fn CreateNamedPipeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpname: Param0,
    dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
    dwpipemode: NAMED_PIPE_MODE,
    nmaxinstances: u32,
    noutbuffersize: u32,
    ninbuffersize: u32,
    ndefaulttimeout: u32,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateNamedPipeW(
                lpname: super::super::Foundation::PWSTR,
                dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
                dwpipemode: NAMED_PIPE_MODE,
                nmaxinstances: u32,
                noutbuffersize: u32,
                ninbuffersize: u32,
                ndefaulttimeout: u32,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateNamedPipeW(
            lpname.into_param().abi(),
            ::std::mem::transmute(dwopenmode),
            ::std::mem::transmute(dwpipemode),
            ::std::mem::transmute(nmaxinstances),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(ndefaulttimeout),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePipe(
    hreadpipe: *mut super::super::Foundation::HANDLE,
    hwritepipe: *mut super::super::Foundation::HANDLE,
    lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    nsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePipe(
                hreadpipe: *mut super::super::Foundation::HANDLE,
                hwritepipe: *mut super::super::Foundation::HANDLE,
                lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                nsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePipe(
            ::std::mem::transmute(hreadpipe),
            ::std::mem::transmute(hwritepipe),
            ::std::mem::transmute(lppipeattributes),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisconnectNamedPipe<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisconnectNamedPipe(
                hnamedpipe: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DisconnectNamedPipe(hnamedpipe.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    clientcomputername: super::super::Foundation::PSTR,
    clientcomputernamelength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientComputerNameA(
                pipe: super::super::Foundation::HANDLE,
                clientcomputername: super::super::Foundation::PSTR,
                clientcomputernamelength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeClientComputerNameA(
            pipe.into_param().abi(),
            ::std::mem::transmute(clientcomputername),
            ::std::mem::transmute(clientcomputernamelength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    clientcomputername: super::super::Foundation::PWSTR,
    clientcomputernamelength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientComputerNameW(
                pipe: super::super::Foundation::HANDLE,
                clientcomputername: super::super::Foundation::PWSTR,
                clientcomputernamelength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeClientComputerNameW(
            pipe.into_param().abi(),
            ::std::mem::transmute(clientcomputername),
            ::std::mem::transmute(clientcomputernamelength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientProcessId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    clientprocessid: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientProcessId(
                pipe: super::super::Foundation::HANDLE,
                clientprocessid: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeClientProcessId(
            pipe.into_param().abi(),
            ::std::mem::transmute(clientprocessid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeClientSessionId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    clientsessionid: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeClientSessionId(
                pipe: super::super::Foundation::HANDLE,
                clientsessionid: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeClientSessionId(
            pipe.into_param().abi(),
            ::std::mem::transmute(clientsessionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeHandleStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpstate: *mut NAMED_PIPE_MODE,
    lpcurinstances: *mut u32,
    lpmaxcollectioncount: *mut u32,
    lpcollectdatatimeout: *mut u32,
    lpusername: super::super::Foundation::PSTR,
    nmaxusernamesize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeHandleStateA(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpstate: *mut NAMED_PIPE_MODE,
                lpcurinstances: *mut u32,
                lpmaxcollectioncount: *mut u32,
                lpcollectdatatimeout: *mut u32,
                lpusername: super::super::Foundation::PSTR,
                nmaxusernamesize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeHandleStateA(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpstate),
            ::std::mem::transmute(lpcurinstances),
            ::std::mem::transmute(lpmaxcollectioncount),
            ::std::mem::transmute(lpcollectdatatimeout),
            ::std::mem::transmute(lpusername),
            ::std::mem::transmute(nmaxusernamesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeHandleStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpstate: *mut NAMED_PIPE_MODE,
    lpcurinstances: *mut u32,
    lpmaxcollectioncount: *mut u32,
    lpcollectdatatimeout: *mut u32,
    lpusername: super::super::Foundation::PWSTR,
    nmaxusernamesize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeHandleStateW(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpstate: *mut NAMED_PIPE_MODE,
                lpcurinstances: *mut u32,
                lpmaxcollectioncount: *mut u32,
                lpcollectdatatimeout: *mut u32,
                lpusername: super::super::Foundation::PWSTR,
                nmaxusernamesize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeHandleStateW(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpstate),
            ::std::mem::transmute(lpcurinstances),
            ::std::mem::transmute(lpmaxcollectioncount),
            ::std::mem::transmute(lpcollectdatatimeout),
            ::std::mem::transmute(lpusername),
            ::std::mem::transmute(nmaxusernamesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpflags: *mut NAMED_PIPE_MODE,
    lpoutbuffersize: *mut u32,
    lpinbuffersize: *mut u32,
    lpmaxinstances: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeInfo(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpflags: *mut NAMED_PIPE_MODE,
                lpoutbuffersize: *mut u32,
                lpinbuffersize: *mut u32,
                lpmaxinstances: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeInfo(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpflags),
            ::std::mem::transmute(lpoutbuffersize),
            ::std::mem::transmute(lpinbuffersize),
            ::std::mem::transmute(lpmaxinstances),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeServerProcessId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    serverprocessid: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeServerProcessId(
                pipe: super::super::Foundation::HANDLE,
                serverprocessid: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeServerProcessId(
            pipe.into_param().abi(),
            ::std::mem::transmute(serverprocessid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedPipeServerSessionId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pipe: Param0,
    serversessionid: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedPipeServerSessionId(
                pipe: super::super::Foundation::HANDLE,
                serversessionid: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedPipeServerSessionId(
            pipe.into_param().abi(),
            ::std::mem::transmute(serversessionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateNamedPipeClient<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateNamedPipeClient(
                hnamedpipe: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateNamedPipeClient(hnamedpipe.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NAMED_PIPE_MODE(pub u32);
pub const PIPE_WAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(2u32);
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(4u32);
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(8u32);
impl ::std::convert::From<u32> for NAMED_PIPE_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NAMED_PIPE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NAMED_PIPE_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NAMED_PIPE_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NAMED_PIPE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekNamedPipe<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nbuffersize: u32,
    lpbytesread: *mut u32,
    lptotalbytesavail: *mut u32,
    lpbytesleftthismessage: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekNamedPipe(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nbuffersize: u32,
                lpbytesread: *mut u32,
                lptotalbytesavail: *mut u32,
                lpbytesleftthismessage: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PeekNamedPipe(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nbuffersize),
            ::std::mem::transmute(lpbytesread),
            ::std::mem::transmute(lptotalbytesavail),
            ::std::mem::transmute(lpbytesleftthismessage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNamedPipeHandleState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpmode: *const NAMED_PIPE_MODE,
    lpmaxcollectioncount: *const u32,
    lpcollectdatatimeout: *const u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNamedPipeHandleState(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpmode: *const NAMED_PIPE_MODE,
                lpmaxcollectioncount: *const u32,
                lpcollectdatatimeout: *const u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetNamedPipeHandleState(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpmode),
            ::std::mem::transmute(lpmaxcollectioncount),
            ::std::mem::transmute(lpcollectdatatimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn TransactNamedPipe<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hnamedpipe: Param0,
    lpinbuffer: *const ::std::ffi::c_void,
    ninbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    noutbuffersize: u32,
    lpbytesread: *mut u32,
    lpoverlapped: *mut super::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TransactNamedPipe(
                hnamedpipe: super::super::Foundation::HANDLE,
                lpinbuffer: *const ::std::ffi::c_void,
                ninbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                noutbuffersize: u32,
                lpbytesread: *mut u32,
                lpoverlapped: *mut super::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TransactNamedPipe(
            hnamedpipe.into_param().abi(),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(lpbytesread),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitNamedPipeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpnamedpipename: Param0,
    ntimeout: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitNamedPipeA(
                lpnamedpipename: super::super::Foundation::PSTR,
                ntimeout: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WaitNamedPipeA(
            lpnamedpipename.into_param().abi(),
            ::std::mem::transmute(ntimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitNamedPipeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpnamedpipename: Param0,
    ntimeout: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitNamedPipeW(
                lpnamedpipename: super::super::Foundation::PWSTR,
                ntimeout: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WaitNamedPipeW(
            lpnamedpipename.into_param().abi(),
            ::std::mem::transmute(ntimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
