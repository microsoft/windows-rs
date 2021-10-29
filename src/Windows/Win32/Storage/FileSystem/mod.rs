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
pub unsafe fn AddLogContainer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hlog: Param0,
    pcbcontainer: *const u64,
    pwszcontainerpath: Param2,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddLogContainer(
                hlog: super::super::Foundation::HANDLE,
                pcbcontainer: *const u64,
                pwszcontainerpath: super::super::Foundation::PWSTR,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddLogContainer(
            hlog.into_param().abi(),
            ::std::mem::transmute(pcbcontainer),
            pwszcontainerpath.into_param().abi(),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddLogContainerSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    ccontainer: u16,
    pcbcontainer: *const u64,
    rgwszcontainerpath: *const super::super::Foundation::PWSTR,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddLogContainerSet(
                hlog: super::super::Foundation::HANDLE,
                ccontainer: u16,
                pcbcontainer: *const u64,
                rgwszcontainerpath: *const super::super::Foundation::PWSTR,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddLogContainerSet(
            hlog.into_param().abi(),
            ::std::mem::transmute(ccontainer),
            ::std::mem::transmute(pcbcontainer),
            ::std::mem::transmute(rgwszcontainerpath),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn AddUsersToEncryptedFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddUsersToEncryptedFile(
                lpfilename: super::super::Foundation::PWSTR,
                pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST,
            ) -> u32;
        }
        ::std::mem::transmute(AddUsersToEncryptedFile(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(pencryptioncertificates),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn AdvanceLogBase(
    pvmarshal: *mut ::std::ffi::c_void,
    plsnbase: *mut CLS_LSN,
    fflags: u32,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdvanceLogBase(
                pvmarshal: *mut ::std::ffi::c_void,
                plsnbase: *mut CLS_LSN,
                fflags: u32,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdvanceLogBase(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(plsnbase),
            ::std::mem::transmute(fflags),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AlignReservedLog(
    pvmarshal: *mut ::std::ffi::c_void,
    creservedrecords: u32,
    rgcbreservation: *mut i64,
    pcbalignreservation: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AlignReservedLog(
                pvmarshal: *mut ::std::ffi::c_void,
                creservedrecords: u32,
                rgcbreservation: *mut i64,
                pcbalignreservation: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AlignReservedLog(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(creservedrecords),
            ::std::mem::transmute(rgcbreservation),
            ::std::mem::transmute(pcbalignreservation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocReservedLog(
    pvmarshal: *mut ::std::ffi::c_void,
    creservedrecords: u32,
    pcbadjustment: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocReservedLog(
                pvmarshal: *mut ::std::ffi::c_void,
                creservedrecords: u32,
                pcbadjustment: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocReservedLog(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(creservedrecords),
            ::std::mem::transmute(pcbadjustment),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreFileApisANSI() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreFileApisANSI() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreFileApisANSI())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreShortNamesEnabled<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    handle: Param0,
    enabled: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreShortNamesEnabled(
                handle: super::super::Foundation::HANDLE,
                enabled: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreShortNamesEnabled(
            handle.into_param().abi(),
            ::std::mem::transmute(enabled),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for BY_HANDLE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BY_HANDLE_FILE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BY_HANDLE_FILE_INFORMATION")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("dwVolumeSerialNumber", &self.dwVolumeSerialNumber)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("nNumberOfLinks", &self.nNumberOfLinks)
            .field("nFileIndexHigh", &self.nFileIndexHigh)
            .field("nFileIndexLow", &self.nFileIndexLow)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BY_HANDLE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.dwVolumeSerialNumber == other.dwVolumeSerialNumber
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.nNumberOfLinks == other.nNumberOfLinks
            && self.nFileIndexHigh == other.nFileIndexHigh
            && self.nFileIndexLow == other.nFileIndexLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BY_HANDLE_FILE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupRead<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfile: Param0,
    lpbuffer: *mut u8,
    nnumberofbytestoread: u32,
    lpnumberofbytesread: *mut u32,
    babort: Param4,
    bprocesssecurity: Param5,
    lpcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupRead(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *mut u8,
                nnumberofbytestoread: u32,
                lpnumberofbytesread: *mut u32,
                babort: super::super::Foundation::BOOL,
                bprocesssecurity: super::super::Foundation::BOOL,
                lpcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BackupRead(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestoread),
            ::std::mem::transmute(lpnumberofbytesread),
            babort.into_param().abi(),
            bprocesssecurity.into_param().abi(),
            ::std::mem::transmute(lpcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupSeek<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    dwlowbytestoseek: u32,
    dwhighbytestoseek: u32,
    lpdwlowbyteseeked: *mut u32,
    lpdwhighbyteseeked: *mut u32,
    lpcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupSeek(
                hfile: super::super::Foundation::HANDLE,
                dwlowbytestoseek: u32,
                dwhighbytestoseek: u32,
                lpdwlowbyteseeked: *mut u32,
                lpdwhighbyteseeked: *mut u32,
                lpcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BackupSeek(
            hfile.into_param().abi(),
            ::std::mem::transmute(dwlowbytestoseek),
            ::std::mem::transmute(dwhighbytestoseek),
            ::std::mem::transmute(lpdwlowbyteseeked),
            ::std::mem::transmute(lpdwhighbyteseeked),
            ::std::mem::transmute(lpcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupWrite<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfile: Param0,
    lpbuffer: *const u8,
    nnumberofbytestowrite: u32,
    lpnumberofbyteswritten: *mut u32,
    babort: Param4,
    bprocesssecurity: Param5,
    lpcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupWrite(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *const u8,
                nnumberofbytestowrite: u32,
                lpnumberofbyteswritten: *mut u32,
                babort: super::super::Foundation::BOOL,
                bprocesssecurity: super::super::Foundation::BOOL,
                lpcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BackupWrite(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestowrite),
            ::std::mem::transmute(lpnumberofbyteswritten),
            babort.into_param().abi(),
            bprocesssecurity.into_param().abi(),
            ::std::mem::transmute(lpcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingCancelRequest<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, IORING_HANDLE_REF>,
>(
    ioring: *const HIORING__,
    file: Param1,
    optocancel: usize,
    userdata: usize,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildIoRingCancelRequest(
                ioring: *const HIORING__,
                file: IORING_HANDLE_REF,
                optocancel: usize,
                userdata: usize,
            ) -> ::windows::runtime::HRESULT;
        }
        BuildIoRingCancelRequest(
            ::std::mem::transmute(ioring),
            file.into_param().abi(),
            ::std::mem::transmute(optocancel),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingReadFile<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, IORING_HANDLE_REF>,
    Param2: ::windows::runtime::IntoParam<'a, IORING_BUFFER_REF>,
>(
    ioring: *const HIORING__,
    fileref: Param1,
    dataref: Param2,
    numberofbytestoread: u32,
    fileoffset: u64,
    userdata: usize,
    flags: IORING_SQE_FLAGS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildIoRingReadFile(
                ioring: *const HIORING__,
                fileref: IORING_HANDLE_REF,
                dataref: IORING_BUFFER_REF,
                numberofbytestoread: u32,
                fileoffset: u64,
                userdata: usize,
                flags: IORING_SQE_FLAGS,
            ) -> ::windows::runtime::HRESULT;
        }
        BuildIoRingReadFile(
            ::std::mem::transmute(ioring),
            fileref.into_param().abi(),
            dataref.into_param().abi(),
            ::std::mem::transmute(numberofbytestoread),
            ::std::mem::transmute(fileoffset),
            ::std::mem::transmute(userdata),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildIoRingRegisterBuffers(
    ioring: *const HIORING__,
    count: u32,
    buffers: *const IORING_BUFFER_INFO,
    userdata: usize,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildIoRingRegisterBuffers(
                ioring: *const HIORING__,
                count: u32,
                buffers: *const IORING_BUFFER_INFO,
                userdata: usize,
            ) -> ::windows::runtime::HRESULT;
        }
        BuildIoRingRegisterBuffers(
            ::std::mem::transmute(ioring),
            ::std::mem::transmute(count),
            ::std::mem::transmute(buffers),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingRegisterFileHandles(
    ioring: *const HIORING__,
    count: u32,
    handles: *const super::super::Foundation::HANDLE,
    userdata: usize,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildIoRingRegisterFileHandles(
                ioring: *const HIORING__,
                count: u32,
                handles: *const super::super::Foundation::HANDLE,
                userdata: usize,
            ) -> ::windows::runtime::HRESULT;
        }
        BuildIoRingRegisterFileHandles(
            ::std::mem::transmute(ioring),
            ::std::mem::transmute(count),
            ::std::mem::transmute(handles),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CACHE_ACCESS_CHECK = unsafe extern "system" fn(
    psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    hclienttoken: super::super::Foundation::HANDLE,
    dwdesiredaccess: u32,
    genericmapping: *mut super::super::Security::GENERIC_MAPPING,
    privilegeset: *mut super::super::Security::PRIVILEGE_SET,
    privilegesetlength: *mut u32,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
) -> super::super::Foundation::BOOL;
pub type CACHE_DESTROY_CALLBACK = unsafe extern "system" fn(cb: u32, lpb: *mut u8);
pub type CACHE_KEY_COMPARE =
    unsafe extern "system" fn(cbkey1: u32, lpbkey1: *mut u8, cbkey2: u32, lpbkey2: *mut u8) -> i32;
pub type CACHE_KEY_HASH = unsafe extern "system" fn(lpbkey: *mut u8, cbkey: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type CACHE_READ_CALLBACK = unsafe extern "system" fn(
    cb: u32,
    lpb: *mut u8,
    lpvcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
pub type CLAIMMEDIALABEL = unsafe extern "system" fn(
    pbuffer: *const u8,
    nbuffersize: u32,
    plabelinfo: *mut MediaLabelInfo,
) -> u32;
pub type CLAIMMEDIALABELEX = unsafe extern "system" fn(
    pbuffer: *const u8,
    nbuffersize: u32,
    plabelinfo: *mut MediaLabelInfo,
    labelguid: *mut ::windows::runtime::GUID,
) -> u32;
pub type CLFS_BLOCK_ALLOCATION = unsafe extern "system" fn(
    cbbufferlength: u32,
    pvusercontext: *mut ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void;
pub type CLFS_BLOCK_DEALLOCATION = unsafe extern "system" fn(
    pvbuffer: *mut ::std::ffi::c_void,
    pvusercontext: *mut ::std::ffi::c_void,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CLFS_CONTEXT_MODE(pub i32);
pub const ClfsContextNone: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(0i32);
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(1i32);
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(2i32);
pub const ClfsContextForward: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(3i32);
impl ::std::convert::From<i32> for CLFS_CONTEXT_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_CONTEXT_MODE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLFS_FLAG(pub u32);
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = CLFS_FLAG(1u32);
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = CLFS_FLAG(2u32);
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = CLFS_FLAG(0u32);
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = CLFS_FLAG(4u32);
impl ::std::convert::From<u32> for CLFS_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CLFS_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CLFS_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CLFS_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CLFS_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CLFS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CLFS_IOSTATS_CLASS(pub i32);
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(0i32);
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(65535i32);
impl ::std::convert::From<i32> for CLFS_IOSTATS_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_IOSTATS_CLASS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLFS_LOG_ARCHIVE_MODE(pub i32);
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(1i32);
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(2i32);
impl ::std::convert::From<i32> for CLFS_LOG_ARCHIVE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_LOG_ARCHIVE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl CLFS_LOG_NAME_INFORMATION {}
impl ::std::default::Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_LOG_NAME_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLFS_LOG_NAME_INFORMATION")
            .field("NameLengthInBytes", &self.NameLengthInBytes)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_LOG_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NameLengthInBytes == other.NameLengthInBytes && self.Name == other.Name
    }
}
impl ::std::cmp::Eq for CLFS_LOG_NAME_INFORMATION {}
unsafe impl ::windows::runtime::Abi for CLFS_LOG_NAME_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: CLS_LSN,
    pub LogIsPinned: u16,
}
impl CLFS_MGMT_NOTIFICATION {}
impl ::std::default::Default for CLFS_MGMT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLFS_MGMT_NOTIFICATION")
            .field("Notification", &self.Notification)
            .field("Lsn", &self.Lsn)
            .field("LogIsPinned", &self.LogIsPinned)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Notification == other.Notification
            && self.Lsn == other.Lsn
            && self.LogIsPinned == other.LogIsPinned
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLFS_MGMT_NOTIFICATION_TYPE(pub i32);
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(0i32);
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(1i32);
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(2i32);
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(3i32);
impl ::std::convert::From<i32> for CLFS_MGMT_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_NOTIFICATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl CLFS_MGMT_POLICY {}
impl ::std::default::Default for CLFS_MGMT_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union CLFS_MGMT_POLICY_0 {
    pub MaximumSize: CLFS_MGMT_POLICY_0_4,
    pub MinimumSize: CLFS_MGMT_POLICY_0_5,
    pub NewContainerSize: CLFS_MGMT_POLICY_0_8,
    pub GrowthRate: CLFS_MGMT_POLICY_0_2,
    pub LogTail: CLFS_MGMT_POLICY_0_3,
    pub AutoShrink: CLFS_MGMT_POLICY_0_1,
    pub AutoGrow: CLFS_MGMT_POLICY_0_0,
    pub NewContainerPrefix: CLFS_MGMT_POLICY_0_7,
    pub NewContainerSuffix: CLFS_MGMT_POLICY_0_9,
    pub NewContainerExtension: CLFS_MGMT_POLICY_0_6,
}
impl CLFS_MGMT_POLICY_0 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Enabled: u32,
}
impl CLFS_MGMT_POLICY_0_0 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AutoGrow_e__Struct")
            .field("Enabled", &self.Enabled)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_0 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Percentage: u32,
}
impl CLFS_MGMT_POLICY_0_1 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AutoShrink_e__Struct")
            .field("Percentage", &self.Percentage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_1 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_2 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
impl CLFS_MGMT_POLICY_0_2 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_GrowthRate_e__Struct")
            .field(
                "AbsoluteGrowthInContainers",
                &self.AbsoluteGrowthInContainers,
            )
            .field("RelativeGrowthPercentage", &self.RelativeGrowthPercentage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteGrowthInContainers == other.AbsoluteGrowthInContainers
            && self.RelativeGrowthPercentage == other.RelativeGrowthPercentage
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_2 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_3 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
impl CLFS_MGMT_POLICY_0_3 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_LogTail_e__Struct")
            .field(
                "MinimumAvailablePercentage",
                &self.MinimumAvailablePercentage,
            )
            .field(
                "MinimumAvailableContainers",
                &self.MinimumAvailableContainers,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumAvailablePercentage == other.MinimumAvailablePercentage
            && self.MinimumAvailableContainers == other.MinimumAvailableContainers
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_3 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_4 {
    pub Containers: u32,
}
impl CLFS_MGMT_POLICY_0_4 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MaximumSize_e__Struct")
            .field("Containers", &self.Containers)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_4 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Containers: u32,
}
impl CLFS_MGMT_POLICY_0_5 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MinimumSize_e__Struct")
            .field("Containers", &self.Containers)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_5 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_6 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl CLFS_MGMT_POLICY_0_6 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NewContainerExtension_e__Struct")
            .field("ExtensionLengthInBytes", &self.ExtensionLengthInBytes)
            .field("ExtensionString", &self.ExtensionString)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionLengthInBytes == other.ExtensionLengthInBytes
            && self.ExtensionString == other.ExtensionString
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_6 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl CLFS_MGMT_POLICY_0_7 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NewContainerPrefix_e__Struct")
            .field("PrefixLengthInBytes", &self.PrefixLengthInBytes)
            .field("PrefixString", &self.PrefixString)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.PrefixLengthInBytes == other.PrefixLengthInBytes
            && self.PrefixString == other.PrefixString
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_7 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_8 {
    pub SizeInBytes: u32,
}
impl CLFS_MGMT_POLICY_0_8 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NewContainerSize_e__Struct")
            .field("SizeInBytes", &self.SizeInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_8 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_8 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_MGMT_POLICY_0_9 {
    pub NextContainerSuffix: u64,
}
impl CLFS_MGMT_POLICY_0_9 {}
impl ::std::default::Default for CLFS_MGMT_POLICY_0_9 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_MGMT_POLICY_0_9 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NewContainerSuffix_e__Struct")
            .field("NextContainerSuffix", &self.NextContainerSuffix)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_MGMT_POLICY_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.NextContainerSuffix == other.NextContainerSuffix
    }
}
impl ::std::cmp::Eq for CLFS_MGMT_POLICY_0_9 {}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_0_9 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLFS_MGMT_POLICY_TYPE(pub i32);
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(0i32);
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(1i32);
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(2i32);
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(3i32);
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(4i32);
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(5i32);
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(6i32);
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(7i32);
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(8i32);
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(9i32);
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(10i32);
impl ::std::convert::From<i32> for CLFS_MGMT_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLFS_MGMT_POLICY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
impl CLFS_NODE_ID {}
impl ::std::default::Default for CLFS_NODE_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_NODE_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLFS_NODE_ID")
            .field("cType", &self.cType)
            .field("cbNode", &self.cbNode)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_NODE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cType == other.cType && self.cbNode == other.cbNode
    }
}
impl ::std::cmp::Eq for CLFS_NODE_ID {}
unsafe impl ::windows::runtime::Abi for CLFS_NODE_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLS_LSN,
    pub PhysicalLsn: CLS_LSN,
}
impl CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::std::default::Default for CLFS_PHYSICAL_LSN_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_PHYSICAL_LSN_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLFS_PHYSICAL_LSN_INFORMATION")
            .field("StreamIdentifier", &self.StreamIdentifier)
            .field("VirtualLsn", &self.VirtualLsn)
            .field("PhysicalLsn", &self.PhysicalLsn)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_PHYSICAL_LSN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
            && self.VirtualLsn == other.VirtualLsn
            && self.PhysicalLsn == other.PhysicalLsn
    }
}
impl ::std::cmp::Eq for CLFS_PHYSICAL_LSN_INFORMATION {}
unsafe impl ::windows::runtime::Abi for CLFS_PHYSICAL_LSN_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
impl CLFS_STREAM_ID_INFORMATION {}
impl ::std::default::Default for CLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLFS_STREAM_ID_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLFS_STREAM_ID_INFORMATION")
            .field("StreamIdentifier", &self.StreamIdentifier)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLFS_STREAM_ID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
    }
}
impl ::std::cmp::Eq for CLFS_STREAM_ID_INFORMATION {}
unsafe impl ::windows::runtime::Abi for CLFS_STREAM_ID_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLSID_DiskQuotaControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2039002481,
    60553,
    4559,
    [156, 0, 0, 170, 0, 161, 79, 86],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
impl CLS_ARCHIVE_DESCRIPTOR {}
impl ::std::default::Default for CLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_ARCHIVE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_ARCHIVE_DESCRIPTOR")
            .field("coffLow", &self.coffLow)
            .field("coffHigh", &self.coffHigh)
            .field("infoContainer", &self.infoContainer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_ARCHIVE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.coffLow == other.coffLow
            && self.coffHigh == other.coffHigh
            && self.infoContainer == other.infoContainer
    }
}
impl ::std::cmp::Eq for CLS_ARCHIVE_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for CLS_ARCHIVE_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: u32,
    pub PhysicalContainerId: u32,
    pub LogicalContainerId: u32,
}
impl CLS_CONTAINER_INFORMATION {}
impl ::std::default::Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_CONTAINER_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_CONTAINER_INFORMATION")
            .field("FileAttributes", &self.FileAttributes)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ContainerSize", &self.ContainerSize)
            .field("FileNameActualLength", &self.FileNameActualLength)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .field("State", &self.State)
            .field("PhysicalContainerId", &self.PhysicalContainerId)
            .field("LogicalContainerId", &self.LogicalContainerId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_CONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ContainerSize == other.ContainerSize
            && self.FileNameActualLength == other.FileNameActualLength
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
            && self.State == other.State
            && self.PhysicalContainerId == other.PhysicalContainerId
            && self.LogicalContainerId == other.LogicalContainerId
    }
}
impl ::std::cmp::Eq for CLS_CONTAINER_INFORMATION {}
unsafe impl ::windows::runtime::Abi for CLS_CONTAINER_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLS_CONTEXT_MODE(pub i32);
pub const ClsContextNone: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(0i32);
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(1i32);
pub const ClsContextPrevious: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(2i32);
pub const ClsContextForward: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(3i32);
impl ::std::convert::From<i32> for CLS_CONTEXT_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLS_CONTEXT_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: ::windows::runtime::GUID,
}
impl CLS_INFORMATION {}
impl ::std::default::Default for CLS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_INFORMATION")
            .field("TotalAvailable", &self.TotalAvailable)
            .field("CurrentAvailable", &self.CurrentAvailable)
            .field("TotalReservation", &self.TotalReservation)
            .field("BaseFileSize", &self.BaseFileSize)
            .field("ContainerSize", &self.ContainerSize)
            .field("TotalContainers", &self.TotalContainers)
            .field("FreeContainers", &self.FreeContainers)
            .field("TotalClients", &self.TotalClients)
            .field("Attributes", &self.Attributes)
            .field("FlushThreshold", &self.FlushThreshold)
            .field("SectorSize", &self.SectorSize)
            .field("MinArchiveTailLsn", &self.MinArchiveTailLsn)
            .field("BaseLsn", &self.BaseLsn)
            .field("LastFlushedLsn", &self.LastFlushedLsn)
            .field("LastLsn", &self.LastLsn)
            .field("RestartLsn", &self.RestartLsn)
            .field("Identity", &self.Identity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalAvailable == other.TotalAvailable
            && self.CurrentAvailable == other.CurrentAvailable
            && self.TotalReservation == other.TotalReservation
            && self.BaseFileSize == other.BaseFileSize
            && self.ContainerSize == other.ContainerSize
            && self.TotalContainers == other.TotalContainers
            && self.FreeContainers == other.FreeContainers
            && self.TotalClients == other.TotalClients
            && self.Attributes == other.Attributes
            && self.FlushThreshold == other.FlushThreshold
            && self.SectorSize == other.SectorSize
            && self.MinArchiveTailLsn == other.MinArchiveTailLsn
            && self.BaseLsn == other.BaseLsn
            && self.LastFlushedLsn == other.LastFlushedLsn
            && self.LastLsn == other.LastLsn
            && self.RestartLsn == other.RestartLsn
            && self.Identity == other.Identity
    }
}
impl ::std::cmp::Eq for CLS_INFORMATION {}
unsafe impl ::windows::runtime::Abi for CLS_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLS_IOSTATS_CLASS(pub i32);
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(0i32);
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(65535i32);
impl ::std::convert::From<i32> for CLS_IOSTATS_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLS_IOSTATS_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
impl CLS_IO_STATISTICS {}
impl ::std::default::Default for CLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_IO_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_IO_STATISTICS")
            .field("hdrIoStats", &self.hdrIoStats)
            .field("cFlush", &self.cFlush)
            .field("cbFlush", &self.cbFlush)
            .field("cMetaFlush", &self.cMetaFlush)
            .field("cbMetaFlush", &self.cbMetaFlush)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_IO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.hdrIoStats == other.hdrIoStats
            && self.cFlush == other.cFlush
            && self.cbFlush == other.cbFlush
            && self.cMetaFlush == other.cMetaFlush
            && self.cbMetaFlush == other.cbMetaFlush
    }
}
impl ::std::cmp::Eq for CLS_IO_STATISTICS {}
unsafe impl ::windows::runtime::Abi for CLS_IO_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
impl CLS_IO_STATISTICS_HEADER {}
impl ::std::default::Default for CLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_IO_STATISTICS_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_IO_STATISTICS_HEADER")
            .field("ubMajorVersion", &self.ubMajorVersion)
            .field("ubMinorVersion", &self.ubMinorVersion)
            .field("eStatsClass", &self.eStatsClass)
            .field("cbLength", &self.cbLength)
            .field("coffData", &self.coffData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_IO_STATISTICS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ubMajorVersion == other.ubMajorVersion
            && self.ubMinorVersion == other.ubMinorVersion
            && self.eStatsClass == other.eStatsClass
            && self.cbLength == other.cbLength
            && self.coffData == other.coffData
    }
}
impl ::std::cmp::Eq for CLS_IO_STATISTICS_HEADER {}
unsafe impl ::windows::runtime::Abi for CLS_IO_STATISTICS_HEADER {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CLS_LOG_INFORMATION_CLASS(pub i32);
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(0i32);
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(1i32);
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(2i32);
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(3i32);
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(4i32);
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(5i32);
impl ::std::convert::From<i32> for CLS_LOG_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLS_LOG_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_LSN {
    pub Internal: u64,
}
impl CLS_LSN {}
impl ::std::default::Default for CLS_LSN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_LSN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_LSN")
            .field("Internal", &self.Internal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_LSN {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
    }
}
impl ::std::cmp::Eq for CLS_LSN {}
unsafe impl ::windows::runtime::Abi for CLS_LSN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::super::Foundation::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: u8,
    pub pinfoContainer: *mut CLS_CONTAINER_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CLS_SCAN_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_SCAN_CONTEXT")
            .field("cidNode", &self.cidNode)
            .field("hLog", &self.hLog)
            .field("cIndex", &self.cIndex)
            .field("cContainers", &self.cContainers)
            .field("cContainersReturned", &self.cContainersReturned)
            .field("eScanMode", &self.eScanMode)
            .field("pinfoContainer", &self.pinfoContainer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLS_SCAN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cidNode == other.cidNode
            && self.hLog == other.hLog
            && self.cIndex == other.cIndex
            && self.cContainers == other.cContainers
            && self.cContainersReturned == other.cContainersReturned
            && self.eScanMode == other.eScanMode
            && self.pinfoContainer == other.pinfoContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLS_SCAN_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: *mut ::std::ffi::c_void,
    pub ByteLength: u32,
}
impl CLS_WRITE_ENTRY {}
impl ::std::default::Default for CLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLS_WRITE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLS_WRITE_ENTRY")
            .field("Buffer", &self.Buffer)
            .field("ByteLength", &self.ByteLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLS_WRITE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.ByteLength == other.ByteLength
    }
}
impl ::std::cmp::Eq for CLS_WRITE_ENTRY {}
unsafe impl ::windows::runtime::Abi for CLS_WRITE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
impl CONNECTION_INFO_0 {}
impl ::std::default::Default for CONNECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONNECTION_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTION_INFO_0")
            .field("coni0_id", &self.coni0_id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONNECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.coni0_id == other.coni0_id
    }
}
impl ::std::cmp::Eq for CONNECTION_INFO_0 {}
unsafe impl ::windows::runtime::Abi for CONNECTION_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: SHARE_TYPE,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: super::super::Foundation::PWSTR,
    pub coni1_netname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CONNECTION_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONNECTION_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONNECTION_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTION_INFO_1")
            .field("coni1_id", &self.coni1_id)
            .field("coni1_type", &self.coni1_type)
            .field("coni1_num_opens", &self.coni1_num_opens)
            .field("coni1_num_users", &self.coni1_num_users)
            .field("coni1_time", &self.coni1_time)
            .field("coni1_username", &self.coni1_username)
            .field("coni1_netname", &self.coni1_netname)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONNECTION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.coni1_id == other.coni1_id
            && self.coni1_type == other.coni1_type
            && self.coni1_num_opens == other.coni1_num_opens
            && self.coni1_num_users == other.coni1_num_users
            && self.coni1_time == other.coni1_time
            && self.coni1_username == other.coni1_username
            && self.coni1_netname == other.coni1_netname
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONNECTION_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONNECTION_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct COPYFILE2_COPY_PHASE(pub i32);
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(0i32);
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(1i32);
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(2i32);
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(3i32);
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(4i32);
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(5i32);
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(6i32);
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(7i32);
impl ::std::convert::From<i32> for COPYFILE2_COPY_PHASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COPYFILE2_COPY_PHASE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: ::std::option::Option<PCOPYFILE2_PROGRESS_ROUTINE>,
    pub pvCallbackContext: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_EXTENDED_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COPYFILE2_EXTENDED_PARAMETERS")
            .field("dwSize", &self.dwSize)
            .field("dwCopyFlags", &self.dwCopyFlags)
            .field("pfCancel", &self.pfCancel)
            .field("pvCallbackContext", &self.pvCallbackContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCopyFlags == other.dwCopyFlags
            && self.pfCancel == other.pfCancel
            && self.pProgressRoutine.map(|f| f as usize)
                == other.pProgressRoutine.map(|f| f as usize)
            && self.pvCallbackContext == other.pvCallbackContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_EXTENDED_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_EXTENDED_PARAMETERS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union COPYFILE2_MESSAGE_0 {
    pub ChunkStarted: COPYFILE2_MESSAGE_0_1,
    pub ChunkFinished: COPYFILE2_MESSAGE_0_0,
    pub StreamStarted: COPYFILE2_MESSAGE_0_5,
    pub StreamFinished: COPYFILE2_MESSAGE_0_4,
    pub PollContinue: COPYFILE2_MESSAGE_0_3,
    pub Error: COPYFILE2_MESSAGE_0_2,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ChunkFinished_e__Struct")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwFlags", &self.dwFlags)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliChunkSize", &self.uliChunkSize)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwFlags == other.dwFlags
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliChunkSize == other.uliChunkSize
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ChunkStarted_e__Struct")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliChunkSize", &self.uliChunkSize)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliChunkSize == other.uliChunkSize
            && self.uliStreamSize == other.uliStreamSize
            && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: ::windows::runtime::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl COPYFILE2_MESSAGE_0_2 {}
impl ::std::default::Default for COPYFILE2_MESSAGE_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Error_e__Struct")
            .field("CopyPhase", &self.CopyPhase)
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("hrFailure", &self.hrFailure)
            .field("dwReserved", &self.dwReserved)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.CopyPhase == other.CopyPhase
            && self.dwStreamNumber == other.dwStreamNumber
            && self.hrFailure == other.hrFailure
            && self.dwReserved == other.dwReserved
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_2 {}
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwReserved: u32,
}
impl COPYFILE2_MESSAGE_0_3 {}
impl ::std::default::Default for COPYFILE2_MESSAGE_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_PollContinue_e__Struct")
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_3 {}
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_StreamFinished_e__Struct")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_4 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl COPYFILE2_MESSAGE_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COPYFILE2_MESSAGE_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COPYFILE2_MESSAGE_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_StreamStarted_e__Struct")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COPYFILE2_MESSAGE_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliStreamSize == other.uliStreamSize
            && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COPYFILE2_MESSAGE_0_5 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_0_5 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct COPYFILE2_MESSAGE_ACTION(pub i32);
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(0i32);
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(1i32);
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(2i32);
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(3i32);
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(4i32);
impl ::std::convert::From<i32> for COPYFILE2_MESSAGE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_ACTION {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct COPYFILE2_MESSAGE_TYPE(pub i32);
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(0i32);
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(1i32);
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(2i32);
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(3i32);
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(4i32);
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(5i32);
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(6i32);
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(7i32);
impl ::std::convert::From<i32> for COPYFILE2_MESSAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COPYFILE2_MESSAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub hTemplateFile: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for CREATEFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for CREATEFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CREATEFILE2_EXTENDED_PARAMETERS")
            .field("dwSize", &self.dwSize)
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwSecurityQosFlags", &self.dwSecurityQosFlags)
            .field("lpSecurityAttributes", &self.lpSecurityAttributes)
            .field("hTemplateFile", &self.hTemplateFile)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for CREATEFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFileAttributes == other.dwFileAttributes
            && self.dwFileFlags == other.dwFileFlags
            && self.dwSecurityQosFlags == other.dwSecurityQosFlags
            && self.lpSecurityAttributes == other.lpSecurityAttributes
            && self.hTemplateFile == other.hTemplateFile
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for CREATEFILE2_EXTENDED_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct CREATE_TAPE_PARTITION_METHOD(pub i32);
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(0i32);
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD =
    CREATE_TAPE_PARTITION_METHOD(2i32);
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(1i32);
impl ::std::convert::From<i32> for CREATE_TAPE_PARTITION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_TAPE_PARTITION_METHOD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
pub const CSC_CACHE_NONE: u32 = 48u32;
pub const CSC_CACHE_VDO: u32 = 32u32;
pub const CSC_MASK: u32 = 48u32;
pub const CSC_MASK_EXT: u32 = 8240u32;
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpname: Param0,
    lpoemname: super::super::Foundation::PSTR,
    oemnamesize: u32,
    pbnamecontainsspaces: *mut super::super::Foundation::BOOL,
    pbnamelegal: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckNameLegalDOS8Dot3A(
                lpname: super::super::Foundation::PSTR,
                lpoemname: super::super::Foundation::PSTR,
                oemnamesize: u32,
                pbnamecontainsspaces: *mut super::super::Foundation::BOOL,
                pbnamelegal: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckNameLegalDOS8Dot3A(
            lpname.into_param().abi(),
            ::std::mem::transmute(lpoemname),
            ::std::mem::transmute(oemnamesize),
            ::std::mem::transmute(pbnamecontainsspaces),
            ::std::mem::transmute(pbnamelegal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpname: Param0,
    lpoemname: super::super::Foundation::PSTR,
    oemnamesize: u32,
    pbnamecontainsspaces: *mut super::super::Foundation::BOOL,
    pbnamelegal: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckNameLegalDOS8Dot3W(
                lpname: super::super::Foundation::PWSTR,
                lpoemname: super::super::Foundation::PSTR,
                oemnamesize: u32,
                pbnamecontainsspaces: *mut super::super::Foundation::BOOL,
                pbnamelegal: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckNameLegalDOS8Dot3W(
            lpname.into_param().abi(),
            ::std::mem::transmute(lpoemname),
            ::std::mem::transmute(oemnamesize),
            ::std::mem::transmute(pbnamecontainsspaces),
            ::std::mem::transmute(pbnamelegal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ClfsClientRecord: u8 = 3u8;
pub const ClfsContainerActive: u32 = 4u32;
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
pub const ClfsContainerInactive: u32 = 2u32;
pub const ClfsContainerInitializing: u32 = 1u32;
pub const ClfsContainerPendingArchive: u32 = 16u32;
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClfsDataRecord: u8 = 1u8;
pub const ClfsNullRecord: u8 = 0u8;
pub const ClfsRestartRecord: u8 = 2u8;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseAndResetLogFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseAndResetLogFile(
                hlog: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseAndResetLogFile(hlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CloseEncryptedFileRaw(pvcontext: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseEncryptedFileRaw(pvcontext: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(CloseEncryptedFileRaw(::std::mem::transmute(pvcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CloseIoRing(ioring: *const HIORING__) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseIoRing(ioring: *const HIORING__) -> ::windows::runtime::HRESULT;
        }
        CloseIoRing(::std::mem::transmute(ioring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ClsContainerActive: u32 = 4u32;
pub const ClsContainerActivePendingDelete: u32 = 8u32;
pub const ClsContainerInactive: u32 = 2u32;
pub const ClsContainerInitializing: u32 = 1u32;
pub const ClsContainerPendingArchive: u32 = 16u32;
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitComplete<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitComplete(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommitComplete(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommitEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitTransaction<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitTransaction(
                transactionhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommitTransaction(transactionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitTransactionAsync<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitTransactionAsync(
                transactionhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommitTransactionAsync(transactionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CompareFileTime(
    lpfiletime1: *const super::super::Foundation::FILETIME,
    lpfiletime2: *const super::super::Foundation::FILETIME,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CompareFileTime(
                lpfiletime1: *const super::super::Foundation::FILETIME,
                lpfiletime2: *const super::super::Foundation::FILETIME,
            ) -> i32;
        }
        ::std::mem::transmute(CompareFileTime(
            ::std::mem::transmute(lpfiletime1),
            ::std::mem::transmute(lpfiletime2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFile2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwszexistingfilename: Param0,
    pwsznewfilename: Param1,
    pextendedparameters: *const COPYFILE2_EXTENDED_PARAMETERS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFile2(
                pwszexistingfilename: super::super::Foundation::PWSTR,
                pwsznewfilename: super::super::Foundation::PWSTR,
                pextendedparameters: *const ::std::mem::ManuallyDrop<COPYFILE2_EXTENDED_PARAMETERS>,
            ) -> ::windows::runtime::HRESULT;
        }
        CopyFile2(
            pwszexistingfilename.into_param().abi(),
            pwsznewfilename.into_param().abi(),
            ::std::mem::transmute(pextendedparameters),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    bfailifexists: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                bfailifexists: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            bfailifexists.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    pbcancel: *mut i32,
    dwcopyflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileExA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                pbcancel: *mut i32,
                dwcopyflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileExA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(pbcancel),
            ::std::mem::transmute(dwcopyflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    pbcancel: *mut i32,
    dwcopyflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileExW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                pbcancel: *mut i32,
                dwcopyflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileExW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(pbcancel),
            ::std::mem::transmute(dwcopyflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    bfailifexists: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileFromAppW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                bfailifexists: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileFromAppW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            bfailifexists.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    pbcancel: *const i32,
    dwcopyflags: u32,
    htransaction: Param6,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileTransactedA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                pbcancel: *const i32,
                dwcopyflags: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileTransactedA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(pbcancel),
            ::std::mem::transmute(dwcopyflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    pbcancel: *const i32,
    dwcopyflags: u32,
    htransaction: Param6,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileTransactedW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                pbcancel: *const i32,
                dwcopyflags: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileTransactedW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(pbcancel),
            ::std::mem::transmute(dwcopyflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    bfailifexists: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyFileW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                bfailifexists: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopyFileW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            bfailifexists.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32;
        }
        ::std::mem::transmute(CopyLZFile(
            ::std::mem::transmute(hfsource),
            ::std::mem::transmute(hfdest),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lppathname: Param0,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryA(
                lppathname: super::super::Foundation::PSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryA(
            lppathname.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lptemplatedirectory: Param0,
    lpnewdirectory: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryExA(
                lptemplatedirectory: super::super::Foundation::PSTR,
                lpnewdirectory: super::super::Foundation::PSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryExA(
            lptemplatedirectory.into_param().abi(),
            lpnewdirectory.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptemplatedirectory: Param0,
    lpnewdirectory: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryExW(
                lptemplatedirectory: super::super::Foundation::PWSTR,
                lpnewdirectory: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryExW(
            lptemplatedirectory.into_param().abi(),
            lpnewdirectory.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryFromAppW(
                lppathname: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryFromAppW(
            lppathname.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lptemplatedirectory: Param0,
    lpnewdirectory: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryTransactedA(
                lptemplatedirectory: super::super::Foundation::PSTR,
                lpnewdirectory: super::super::Foundation::PSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryTransactedA(
            lptemplatedirectory.into_param().abi(),
            lpnewdirectory.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lptemplatedirectory: Param0,
    lpnewdirectory: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryTransactedW(
                lptemplatedirectory: super::super::Foundation::PWSTR,
                lpnewdirectory: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryTransactedW(
            lptemplatedirectory.into_param().abi(),
            lpnewdirectory.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirectoryW(
                lppathname: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDirectoryW(
            lppathname.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEnlistment<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    resourcemanagerhandle: Param1,
    transactionhandle: Param2,
    notificationmask: u32,
    createoptions: u32,
    enlistmentkey: *mut ::std::ffi::c_void,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEnlistment(
                lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                resourcemanagerhandle: super::super::Foundation::HANDLE,
                transactionhandle: super::super::Foundation::HANDLE,
                notificationmask: u32,
                createoptions: u32,
                enlistmentkey: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateEnlistment(
            ::std::mem::transmute(lpenlistmentattributes),
            resourcemanagerhandle.into_param().abi(),
            transactionhandle.into_param().abi(),
            ::std::mem::transmute(notificationmask),
            ::std::mem::transmute(createoptions),
            ::std::mem::transmute(enlistmentkey),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFile2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFile2(
                lpfilename: super::super::Foundation::PWSTR,
                dwdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                dwcreationdisposition: FILE_CREATION_DISPOSITION,
                pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFile2(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(pcreateexparams),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFile2FromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwdesiredaccess: u32,
    dwsharemode: u32,
    dwcreationdisposition: u32,
    pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFile2FromAppW(
                lpfilename: super::super::Foundation::PWSTR,
                dwdesiredaccess: u32,
                dwsharemode: u32,
                dwcreationdisposition: u32,
                pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFile2FromAppW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(pcreateexparams),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: Param6,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileA(
                lpfilename: super::super::Foundation::PSTR,
                dwdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwcreationdisposition: FILE_CREATION_DISPOSITION,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
                htemplatefile: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(dwflagsandattributes),
            htemplatefile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwdesiredaccess: u32,
    dwsharemode: u32,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwcreationdisposition: u32,
    dwflagsandattributes: u32,
    htemplatefile: Param6,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileFromAppW(
                lpfilename: super::super::Foundation::PWSTR,
                dwdesiredaccess: u32,
                dwsharemode: u32,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwcreationdisposition: u32,
                dwflagsandattributes: u32,
                htemplatefile: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileFromAppW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(dwflagsandattributes),
            htemplatefile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwdesiredaccess: u32,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: Param6,
    htransaction: Param7,
    pusminiversion: *const TXFS_MINIVERSION,
    lpextendedparameter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                dwdesiredaccess: u32,
                dwsharemode: FILE_SHARE_MODE,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwcreationdisposition: FILE_CREATION_DISPOSITION,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
                htemplatefile: super::super::Foundation::HANDLE,
                htransaction: super::super::Foundation::HANDLE,
                pusminiversion: *const TXFS_MINIVERSION,
                lpextendedparameter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(dwflagsandattributes),
            htemplatefile.into_param().abi(),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pusminiversion),
            ::std::mem::transmute(lpextendedparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwdesiredaccess: u32,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: Param6,
    htransaction: Param7,
    pusminiversion: *const TXFS_MINIVERSION,
    lpextendedparameter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                dwdesiredaccess: u32,
                dwsharemode: FILE_SHARE_MODE,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwcreationdisposition: FILE_CREATION_DISPOSITION,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
                htemplatefile: super::super::Foundation::HANDLE,
                htransaction: super::super::Foundation::HANDLE,
                pusminiversion: *const TXFS_MINIVERSION,
                lpextendedparameter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(dwflagsandattributes),
            htemplatefile.into_param().abi(),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pusminiversion),
            ::std::mem::transmute(lpextendedparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: Param6,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileW(
                lpfilename: super::super::Foundation::PWSTR,
                dwdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwcreationdisposition: FILE_CREATION_DISPOSITION,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
                htemplatefile: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwcreationdisposition),
            ::std::mem::transmute(dwflagsandattributes),
            htemplatefile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpexistingfilename: Param1,
    lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateHardLinkA(
                lpfilename: super::super::Foundation::PSTR,
                lpexistingfilename: super::super::Foundation::PSTR,
                lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateHardLinkA(
            lpfilename.into_param().abi(),
            lpexistingfilename.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    lpexistingfilename: Param1,
    lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateHardLinkTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                lpexistingfilename: super::super::Foundation::PSTR,
                lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateHardLinkTransactedA(
            lpfilename.into_param().abi(),
            lpexistingfilename.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    lpexistingfilename: Param1,
    lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateHardLinkTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateHardLinkTransactedW(
            lpfilename.into_param().abi(),
            lpexistingfilename.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    lpexistingfilename: Param1,
    lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateHardLinkW(
                lpfilename: super::super::Foundation::PWSTR,
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateHardLinkW(
            lpfilename.into_param().abi(),
            lpexistingfilename.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateIoRing<'a, Param1: ::windows::runtime::IntoParam<'a, IORING_CREATE_FLAGS>>(
    ioringversion: IORING_VERSION,
    flags: Param1,
    submissionqueuesize: u32,
    completionqueuesize: u32,
) -> ::windows::runtime::Result<*mut HIORING__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIoRing(
                ioringversion: IORING_VERSION,
                flags: IORING_CREATE_FLAGS,
                submissionqueuesize: u32,
                completionqueuesize: u32,
                h: *mut *mut HIORING__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HIORING__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateIoRing(
            ::std::mem::transmute(ioringversion),
            flags.into_param().abi(),
            ::std::mem::transmute(submissionqueuesize),
            ::std::mem::transmute(completionqueuesize),
            &mut result__,
        )
        .from_abi::<*mut HIORING__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CreateLogContainerScanContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    cfromcontainer: u32,
    ccontainers: u32,
    escanmode: u8,
    pcxscan: *mut CLS_SCAN_CONTEXT,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateLogContainerScanContext(
                hlog: super::super::Foundation::HANDLE,
                cfromcontainer: u32,
                ccontainers: u32,
                escanmode: u8,
                pcxscan: *mut CLS_SCAN_CONTEXT,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateLogContainerScanContext(
            hlog.into_param().abi(),
            ::std::mem::transmute(cfromcontainer),
            ::std::mem::transmute(ccontainers),
            ::std::mem::transmute(escanmode),
            ::std::mem::transmute(pcxscan),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateLogFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszlogfilename: Param0,
    fdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES,
    fcreatedisposition: FILE_CREATION_DISPOSITION,
    fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateLogFile(
                pszlogfilename: super::super::Foundation::PWSTR,
                fdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES,
                fcreatedisposition: FILE_CREATION_DISPOSITION,
                fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateLogFile(
            pszlogfilename.into_param().abi(),
            ::std::mem::transmute(fdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(psalogfile),
            ::std::mem::transmute(fcreatedisposition),
            ::std::mem::transmute(fflagsandattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateLogMarshallingArea<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pfnallocbuffer: ::std::option::Option<CLFS_BLOCK_ALLOCATION>,
    pfnfreebuffer: ::std::option::Option<CLFS_BLOCK_DEALLOCATION>,
    pvblockalloccontext: *mut ::std::ffi::c_void,
    cbmarshallingbuffer: u32,
    cmaxwritebuffers: u32,
    cmaxreadbuffers: u32,
    ppvmarshal: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateLogMarshallingArea(
                hlog: super::super::Foundation::HANDLE,
                pfnallocbuffer: ::windows::runtime::RawPtr,
                pfnfreebuffer: ::windows::runtime::RawPtr,
                pvblockalloccontext: *mut ::std::ffi::c_void,
                cbmarshallingbuffer: u32,
                cmaxwritebuffers: u32,
                cmaxreadbuffers: u32,
                ppvmarshal: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateLogMarshallingArea(
            hlog.into_param().abi(),
            ::std::mem::transmute(pfnallocbuffer),
            ::std::mem::transmute(pfnfreebuffer),
            ::std::mem::transmute(pvblockalloccontext),
            ::std::mem::transmute(cbmarshallingbuffer),
            ::std::mem::transmute(cmaxwritebuffers),
            ::std::mem::transmute(cmaxreadbuffers),
            ::std::mem::transmute(ppvmarshal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateResourceManager<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    resourcemanagerid: *mut ::windows::runtime::GUID,
    createoptions: u32,
    tmhandle: Param3,
    description: Param4,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateResourceManager(
                lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                resourcemanagerid: *mut ::windows::runtime::GUID,
                createoptions: u32,
                tmhandle: super::super::Foundation::HANDLE,
                description: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateResourceManager(
            ::std::mem::transmute(lpresourcemanagerattributes),
            ::std::mem::transmute(resourcemanagerid),
            ::std::mem::transmute(createoptions),
            tmhandle.into_param().abi(),
            description.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpsymlinkfilename: Param0,
    lptargetfilename: Param1,
    dwflags: SYMBOLIC_LINK_FLAGS,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSymbolicLinkA(
                lpsymlinkfilename: super::super::Foundation::PSTR,
                lptargetfilename: super::super::Foundation::PSTR,
                dwflags: SYMBOLIC_LINK_FLAGS,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CreateSymbolicLinkA(
            lpsymlinkfilename.into_param().abi(),
            lptargetfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpsymlinkfilename: Param0,
    lptargetfilename: Param1,
    dwflags: SYMBOLIC_LINK_FLAGS,
    htransaction: Param3,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSymbolicLinkTransactedA(
                lpsymlinkfilename: super::super::Foundation::PSTR,
                lptargetfilename: super::super::Foundation::PSTR,
                dwflags: SYMBOLIC_LINK_FLAGS,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CreateSymbolicLinkTransactedA(
            lpsymlinkfilename.into_param().abi(),
            lptargetfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpsymlinkfilename: Param0,
    lptargetfilename: Param1,
    dwflags: SYMBOLIC_LINK_FLAGS,
    htransaction: Param3,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSymbolicLinkTransactedW(
                lpsymlinkfilename: super::super::Foundation::PWSTR,
                lptargetfilename: super::super::Foundation::PWSTR,
                dwflags: SYMBOLIC_LINK_FLAGS,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CreateSymbolicLinkTransactedW(
            lpsymlinkfilename.into_param().abi(),
            lptargetfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpsymlinkfilename: Param0,
    lptargetfilename: Param1,
    dwflags: SYMBOLIC_LINK_FLAGS,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSymbolicLinkW(
                lpsymlinkfilename: super::super::Foundation::PWSTR,
                lptargetfilename: super::super::Foundation::PWSTR,
                dwflags: SYMBOLIC_LINK_FLAGS,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CreateSymbolicLinkW(
            lpsymlinkfilename.into_param().abi(),
            lptargetfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTapePartition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD,
    dwcount: u32,
    dwsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTapePartition(
                hdevice: super::super::Foundation::HANDLE,
                dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD,
                dwcount: u32,
                dwsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(CreateTapePartition(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwpartitionmethod),
            ::std::mem::transmute(dwcount),
            ::std::mem::transmute(dwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateTransaction<
    'a,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    uow: *mut ::windows::runtime::GUID,
    createoptions: u32,
    isolationlevel: u32,
    isolationflags: u32,
    timeout: u32,
    description: Param6,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTransaction(
                lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                uow: *mut ::windows::runtime::GUID,
                createoptions: u32,
                isolationlevel: u32,
                isolationflags: u32,
                timeout: u32,
                description: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateTransaction(
            ::std::mem::transmute(lptransactionattributes),
            ::std::mem::transmute(uow),
            ::std::mem::transmute(createoptions),
            ::std::mem::transmute(isolationlevel),
            ::std::mem::transmute(isolationflags),
            ::std::mem::transmute(timeout),
            description.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateTransactionManager<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    logfilename: Param1,
    createoptions: u32,
    commitstrength: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTransactionManager(
                lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
                logfilename: super::super::Foundation::PWSTR,
                createoptions: u32,
                commitstrength: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateTransactionManager(
            ::std::mem::transmute(lptransactionattributes),
            logfilename.into_param().abi(),
            ::std::mem::transmute(createoptions),
            ::std::mem::transmute(commitstrength),
        ))
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
pub struct DEFINE_DOS_DEVICE_FLAGS(pub u32);
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(1u32);
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(2u32);
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(4u32);
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(8u32);
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(16u32);
impl ::std::convert::From<u32> for DEFINE_DOS_DEVICE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEFINE_DOS_DEVICE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DISKQUOTA_USERNAME_RESOLVE(pub u32);
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(2u32);
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(0u32);
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(1u32);
impl ::std::convert::From<u32> for DISKQUOTA_USERNAME_RESOLVE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISKQUOTA_USERNAME_RESOLVE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DISKQUOTA_USERNAME_RESOLVE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DISKQUOTA_USERNAME_RESOLVE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DISKQUOTA_USERNAME_RESOLVE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DISKQUOTA_USERNAME_RESOLVE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DISKQUOTA_USERNAME_RESOLVE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DISKQUOTA_USER_INFORMATION {
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
}
impl DISKQUOTA_USER_INFORMATION {}
impl ::std::default::Default for DISKQUOTA_USER_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISKQUOTA_USER_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISKQUOTA_USER_INFORMATION")
            .field("QuotaUsed", &self.QuotaUsed)
            .field("QuotaThreshold", &self.QuotaThreshold)
            .field("QuotaLimit", &self.QuotaLimit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DISKQUOTA_USER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.QuotaUsed == other.QuotaUsed
            && self.QuotaThreshold == other.QuotaThreshold
            && self.QuotaLimit == other.QuotaLimit
    }
}
impl ::std::cmp::Eq for DISKQUOTA_USER_INFORMATION {}
unsafe impl ::windows::runtime::Abi for DISKQUOTA_USER_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl DISK_SPACE_INFORMATION {}
impl ::std::default::Default for DISK_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISK_SPACE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISK_SPACE_INFORMATION")
            .field(
                "ActualTotalAllocationUnits",
                &self.ActualTotalAllocationUnits,
            )
            .field(
                "ActualAvailableAllocationUnits",
                &self.ActualAvailableAllocationUnits,
            )
            .field(
                "ActualPoolUnavailableAllocationUnits",
                &self.ActualPoolUnavailableAllocationUnits,
            )
            .field(
                "CallerTotalAllocationUnits",
                &self.CallerTotalAllocationUnits,
            )
            .field(
                "CallerAvailableAllocationUnits",
                &self.CallerAvailableAllocationUnits,
            )
            .field(
                "CallerPoolUnavailableAllocationUnits",
                &self.CallerPoolUnavailableAllocationUnits,
            )
            .field("UsedAllocationUnits", &self.UsedAllocationUnits)
            .field(
                "TotalReservedAllocationUnits",
                &self.TotalReservedAllocationUnits,
            )
            .field(
                "VolumeStorageReserveAllocationUnits",
                &self.VolumeStorageReserveAllocationUnits,
            )
            .field(
                "AvailableCommittedAllocationUnits",
                &self.AvailableCommittedAllocationUnits,
            )
            .field(
                "PoolAvailableAllocationUnits",
                &self.PoolAvailableAllocationUnits,
            )
            .field("SectorsPerAllocationUnit", &self.SectorsPerAllocationUnit)
            .field("BytesPerSector", &self.BytesPerSector)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DISK_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ActualTotalAllocationUnits == other.ActualTotalAllocationUnits
            && self.ActualAvailableAllocationUnits == other.ActualAvailableAllocationUnits
            && self.ActualPoolUnavailableAllocationUnits
                == other.ActualPoolUnavailableAllocationUnits
            && self.CallerTotalAllocationUnits == other.CallerTotalAllocationUnits
            && self.CallerAvailableAllocationUnits == other.CallerAvailableAllocationUnits
            && self.CallerPoolUnavailableAllocationUnits
                == other.CallerPoolUnavailableAllocationUnits
            && self.UsedAllocationUnits == other.UsedAllocationUnits
            && self.TotalReservedAllocationUnits == other.TotalReservedAllocationUnits
            && self.VolumeStorageReserveAllocationUnits == other.VolumeStorageReserveAllocationUnits
            && self.AvailableCommittedAllocationUnits == other.AvailableCommittedAllocationUnits
            && self.PoolAvailableAllocationUnits == other.PoolAvailableAllocationUnits
            && self.SectorsPerAllocationUnit == other.SectorsPerAllocationUnit
            && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::std::cmp::Eq for DISK_SPACE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for DISK_SPACE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DecryptFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    dwreserved: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DecryptFileA(
                lpfilename: super::super::Foundation::PSTR,
                dwreserved: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DecryptFileA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DecryptFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwreserved: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DecryptFileW(
                lpfilename: super::super::Foundation::PWSTR,
                dwreserved: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DecryptFileW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefineDosDeviceA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: DEFINE_DOS_DEVICE_FLAGS,
    lpdevicename: Param1,
    lptargetpath: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefineDosDeviceA(
                dwflags: DEFINE_DOS_DEVICE_FLAGS,
                lpdevicename: super::super::Foundation::PSTR,
                lptargetpath: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DefineDosDeviceA(
            ::std::mem::transmute(dwflags),
            lpdevicename.into_param().abi(),
            lptargetpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefineDosDeviceW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: DEFINE_DOS_DEVICE_FLAGS,
    lpdevicename: Param1,
    lptargetpath: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefineDosDeviceW(
                dwflags: DEFINE_DOS_DEVICE_FLAGS,
                lpdevicename: super::super::Foundation::PWSTR,
                lptargetpath: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DefineDosDeviceW(
            ::std::mem::transmute(dwflags),
            lpdevicename.into_param().abi(),
            lptargetpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFileA(
                lpfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteFileA(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFileFromAppW(
                lpfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteFileFromAppW(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    htransaction: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFileTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteFileTransactedA(
            lpfilename.into_param().abi(),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    htransaction: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFileTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteFileTransactedW(
            lpfilename.into_param().abi(),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFileW(
                lpfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteFileW(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogByHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteLogByHandle(
                hlog: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteLogByHandle(hlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszlogfilename: Param0,
    pvreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteLogFile(
                pszlogfilename: super::super::Foundation::PWSTR,
                pvreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteLogFile(
            pszlogfilename.into_param().abi(),
            ::std::mem::transmute(pvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogMarshallingArea(
    pvmarshal: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteLogMarshallingArea(
                pvmarshal: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteLogMarshallingArea(::std::mem::transmute(pvmarshal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteVolumeMountPointA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszvolumemountpoint: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteVolumeMountPointA(
                lpszvolumemountpoint: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteVolumeMountPointA(
            lpszvolumemountpoint.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteVolumeMountPointW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszvolumemountpoint: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteVolumeMountPointW(
                lpszvolumemountpoint: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteVolumeMountPointW(
            lpszvolumemountpoint.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeregisterManageableLogClient<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeregisterManageableLogClient(
                hlog: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeregisterManageableLogClient(hlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DuplicateEncryptionInfoFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    srcfilename: Param0,
    dstfilename: Param1,
    dwcreationdistribution: u32,
    dwattributes: u32,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicateEncryptionInfoFile(
                srcfilename: super::super::Foundation::PWSTR,
                dstfilename: super::super::Foundation::PWSTR,
                dwcreationdistribution: u32,
                dwattributes: u32,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> u32;
        }
        ::std::mem::transmute(DuplicateEncryptionInfoFile(
            srcfilename.into_param().abi(),
            dstfilename.into_param().abi(),
            ::std::mem::transmute(dwcreationdistribution),
            ::std::mem::transmute(dwattributes),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl EFS_CERTIFICATE_BLOB {}
impl ::std::default::Default for EFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_CERTIFICATE_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_CERTIFICATE_BLOB")
            .field("dwCertEncodingType", &self.dwCertEncodingType)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_CERTIFICATE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType
            && self.cbData == other.cbData
            && self.pbData == other.pbData
    }
}
impl ::std::cmp::Eq for EFS_CERTIFICATE_BLOB {}
unsafe impl ::windows::runtime::Abi for EFS_CERTIFICATE_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
impl EFS_COMPATIBILITY_INFO {}
impl ::std::default::Default for EFS_COMPATIBILITY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_COMPATIBILITY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_COMPATIBILITY_INFO")
            .field("EfsVersion", &self.EfsVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_COMPATIBILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion
    }
}
impl ::std::cmp::Eq for EFS_COMPATIBILITY_INFO {}
unsafe impl ::windows::runtime::Abi for EFS_COMPATIBILITY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
impl EFS_DECRYPTION_STATUS_INFO {}
impl ::std::default::Default for EFS_DECRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_DECRYPTION_STATUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_DECRYPTION_STATUS_INFO")
            .field("dwDecryptionError", &self.dwDecryptionError)
            .field("dwHashOffset", &self.dwHashOffset)
            .field("cbHash", &self.cbHash)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_DECRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDecryptionError == other.dwDecryptionError
            && self.dwHashOffset == other.dwHashOffset
            && self.cbHash == other.cbHash
    }
}
impl ::std::cmp::Eq for EFS_DECRYPTION_STATUS_INFO {}
unsafe impl ::windows::runtime::Abi for EFS_DECRYPTION_STATUS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: super::super::Foundation::BOOL,
    pub dwEncryptionError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EFS_ENCRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EFS_ENCRYPTION_STATUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_ENCRYPTION_STATUS_INFO")
            .field("bHasCurrentKey", &self.bHasCurrentKey)
            .field("dwEncryptionError", &self.dwEncryptionError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EFS_ENCRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bHasCurrentKey == other.bHasCurrentKey
            && self.dwEncryptionError == other.dwEncryptionError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EFS_ENCRYPTION_STATUS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl EFS_HASH_BLOB {}
impl ::std::default::Default for EFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_HASH_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_HASH_BLOB")
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_HASH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::std::cmp::Eq for EFS_HASH_BLOB {}
unsafe impl ::windows::runtime::Abi for EFS_HASH_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: u32,
    pub KeyLength: u32,
}
impl EFS_KEY_INFO {}
impl ::std::default::Default for EFS_KEY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_KEY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_KEY_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("Entropy", &self.Entropy)
            .field("Algorithm", &self.Algorithm)
            .field("KeyLength", &self.KeyLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.Entropy == other.Entropy
            && self.Algorithm == other.Algorithm
            && self.KeyLength == other.KeyLength
    }
}
impl ::std::cmp::Eq for EFS_KEY_INFO {}
unsafe impl ::windows::runtime::Abi for EFS_KEY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl EFS_PIN_BLOB {}
impl ::std::default::Default for EFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_PIN_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_PIN_BLOB")
            .field("cbPadding", &self.cbPadding)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_PIN_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbPadding == other.cbPadding
            && self.cbData == other.cbData
            && self.pbData == other.pbData
    }
}
impl ::std::cmp::Eq for EFS_PIN_BLOB {}
unsafe impl ::windows::runtime::Abi for EFS_PIN_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl EFS_RPC_BLOB {}
impl ::std::default::Default for EFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_RPC_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_RPC_BLOB")
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_RPC_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::std::cmp::Eq for EFS_RPC_BLOB {}
unsafe impl ::windows::runtime::Abi for EFS_RPC_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
impl EFS_VERSION_INFO {}
impl ::std::default::Default for EFS_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EFS_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EFS_VERSION_INFO")
            .field("EfsVersion", &self.EfsVersion)
            .field("SubVersion", &self.SubVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EFS_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion && self.SubVersion == other.SubVersion
    }
}
impl ::std::cmp::Eq for EFS_VERSION_INFO {}
unsafe impl ::windows::runtime::Abi for EFS_VERSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: *mut ENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: *mut EFS_RPC_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTED_FILE_METADATA_SIGNATURE")
            .field("dwEfsAccessType", &self.dwEfsAccessType)
            .field("pCertificatesAdded", &self.pCertificatesAdded)
            .field("pEncryptionCertificate", &self.pEncryptionCertificate)
            .field("pEfsStreamSignature", &self.pEfsStreamSignature)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.dwEfsAccessType == other.dwEfsAccessType
            && self.pCertificatesAdded == other.pCertificatesAdded
            && self.pEncryptionCertificate == other.pEncryptionCertificate
            && self.pEfsStreamSignature == other.pEfsStreamSignature
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for ENCRYPTED_FILE_METADATA_SIGNATURE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pCertBlob: *mut EFS_CERTIFICATE_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
impl ::std::default::Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::fmt::Debug for ENCRYPTION_CERTIFICATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_CERTIFICATE")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("pCertBlob", &self.pCertBlob)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::cmp::PartialEq for ENCRYPTION_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.pCertBlob == other.pCertBlob
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::cmp::Eq for ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_CERTIFICATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pHash: *mut EFS_HASH_BLOB,
    pub lpDisplayInformation: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_CERTIFICATE_HASH")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("pHash", &self.pHash)
            .field("lpDisplayInformation", &self.lpDisplayInformation)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.pHash == other.pHash
            && self.lpDisplayInformation == other.lpDisplayInformation
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_CERTIFICATE_HASH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_CERTIFICATE_HASH_LIST")
            .field("nCert_Hash", &self.nCert_Hash)
            .field("pUsers", &self.pUsers)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nCert_Hash == other.nCert_Hash && self.pUsers == other.pUsers
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_CERTIFICATE_HASH_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE,
}
#[cfg(feature = "Win32_Security")]
impl ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::std::default::Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::fmt::Debug for ENCRYPTION_CERTIFICATE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_CERTIFICATE_LIST")
            .field("nUsers", &self.nUsers)
            .field("pUsers", &self.pUsers)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::cmp::PartialEq for ENCRYPTION_CERTIFICATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nUsers == other.nUsers && self.pUsers == other.pUsers
    }
}
#[cfg(feature = "Win32_Security")]
impl ::std::cmp::Eq for ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_CERTIFICATE_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub lpProtectorDescriptor: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ENCRYPTION_PROTECTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for ENCRYPTION_PROTECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_PROTECTOR")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("lpProtectorDescriptor", &self.lpProtectorDescriptor)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for ENCRYPTION_PROTECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.lpProtectorDescriptor == other.lpProtectorDescriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for ENCRYPTION_PROTECTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_PROTECTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut *mut ENCRYPTION_PROTECTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ENCRYPTION_PROTECTOR_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for ENCRYPTION_PROTECTOR_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTION_PROTECTOR_LIST")
            .field("nProtectors", &self.nProtectors)
            .field("pProtectors", &self.pProtectors)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for ENCRYPTION_PROTECTOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nProtectors == other.nProtectors && self.pProtectors == other.pProtectors
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for ENCRYPTION_PROTECTOR_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for ENCRYPTION_PROTECTOR_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ERASE_TAPE_TYPE(pub i32);
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(1i32);
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(0i32);
impl ::std::convert::From<i32> for ERASE_TAPE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ERASE_TAPE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EncryptFileA(
                lpfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EncryptFileA(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EncryptFileW(
                lpfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EncryptFileW(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptionDisable<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    dirpath: Param0,
    disable: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EncryptionDisable(
                dirpath: super::super::Foundation::PWSTR,
                disable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EncryptionDisable(
            dirpath.into_param().abi(),
            disable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EraseTape<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdevice: Param0,
    dwerasetype: ERASE_TAPE_TYPE,
    bimmediate: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EraseTape(
                hdevice: super::super::Foundation::HANDLE,
                dwerasetype: ERASE_TAPE_TYPE,
                bimmediate: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(EraseTape(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwerasetype),
            bimmediate.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_CREATE_CALLBACK = unsafe extern "system" fn(
    lpstrname: super::super::Foundation::PSTR,
    lpvdata: *mut ::std::ffi::c_void,
    cbfilesize: *mut u32,
    cbfilesizehigh: *mut u32,
) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_RICHCREATE_CALLBACK = unsafe extern "system" fn(
    lpstrname: super::super::Foundation::PSTR,
    lpvdata: *mut ::std::ffi::c_void,
    cbfilesize: *mut u32,
    cbfilesizehigh: *mut u32,
    pfdidwescanit: *mut super::super::Foundation::BOOL,
    pfisstuffed: *mut super::super::Foundation::BOOL,
    pfstoredwithdots: *mut super::super::Foundation::BOOL,
    pfstoredwithterminatingdot: *mut super::super::Foundation::BOOL,
)
    -> super::super::Foundation::HANDLE;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FH_OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Offset: u32,
    pub OffsetHigh: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub pfnCompletion: ::std::option::Option<PFN_IO_COMPLETION>,
    pub Reserved1: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl FH_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FH_OVERLAPPED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FH_OVERLAPPED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FH_OVERLAPPED")
            .field("Internal", &self.Internal)
            .field("InternalHigh", &self.InternalHigh)
            .field("Offset", &self.Offset)
            .field("OffsetHigh", &self.OffsetHigh)
            .field("hEvent", &self.hEvent)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FH_OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
            && self.InternalHigh == other.InternalHigh
            && self.Offset == other.Offset
            && self.OffsetHigh == other.OffsetHigh
            && self.hEvent == other.hEvent
            && self.pfnCompletion.map(|f| f as usize) == other.pfnCompletion.map(|f| f as usize)
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FH_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FH_OVERLAPPED {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
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
pub struct FILE_ACCESS_FLAGS(pub u32);
pub const FILE_READ_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_WRITE_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_ADD_FILE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_APPEND_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_READ_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(8u32);
pub const FILE_WRITE_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(16u32);
pub const FILE_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_TRAVERSE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_DELETE_CHILD: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(64u32);
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(128u32);
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(256u32);
pub const READ_CONTROL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const SYNCHRONIZE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1048576u32);
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(983040u32);
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2031616u32);
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(65535u32);
pub const FILE_ALL_ACCESS: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2032127u32);
pub const FILE_GENERIC_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179785u32);
pub const FILE_GENERIC_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179926u32);
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179808u32);
impl ::std::convert::From<u32> for FILE_ACCESS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_ACCESS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_ACCESS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_ACCESS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct FILE_ACTION(pub u32);
pub const FILE_ACTION_ADDED: FILE_ACTION = FILE_ACTION(1u32);
pub const FILE_ACTION_REMOVED: FILE_ACTION = FILE_ACTION(2u32);
pub const FILE_ACTION_MODIFIED: FILE_ACTION = FILE_ACTION(3u32);
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = FILE_ACTION(4u32);
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = FILE_ACTION(5u32);
impl ::std::convert::From<u32> for FILE_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
impl FILE_ALIGNMENT_INFO {}
impl ::std::default::Default for FILE_ALIGNMENT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ALIGNMENT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ALIGNMENT_INFO")
            .field("AlignmentRequirement", &self.AlignmentRequirement)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ALIGNMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AlignmentRequirement == other.AlignmentRequirement
    }
}
impl ::std::cmp::Eq for FILE_ALIGNMENT_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ALIGNMENT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
impl FILE_ALLOCATION_INFO {}
impl ::std::default::Default for FILE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ALLOCATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ALLOCATION_INFO")
            .field("AllocationSize", &self.AllocationSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ALLOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
    }
}
impl ::std::cmp::Eq for FILE_ALLOCATION_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ALLOCATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl FILE_ATTRIBUTE_TAG_INFO {}
impl ::std::default::Default for FILE_ATTRIBUTE_TAG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ATTRIBUTE_TAG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ATTRIBUTE_TAG_INFO")
            .field("FileAttributes", &self.FileAttributes)
            .field("ReparseTag", &self.ReparseTag)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ATTRIBUTE_TAG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.ReparseTag == other.ReparseTag
    }
}
impl ::std::cmp::Eq for FILE_ATTRIBUTE_TAG_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ATTRIBUTE_TAG_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl FILE_BASIC_INFO {}
impl ::std::default::Default for FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_BASIC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_BASIC_INFO")
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("FileAttributes", &self.FileAttributes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.FileAttributes == other.FileAttributes
    }
}
impl ::std::cmp::Eq for FILE_BASIC_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_BASIC_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl FILE_COMPRESSION_INFO {}
impl ::std::default::Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_COMPRESSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_COMPRESSION_INFO")
            .field("CompressedFileSize", &self.CompressedFileSize)
            .field("CompressionFormat", &self.CompressionFormat)
            .field("CompressionUnitShift", &self.CompressionUnitShift)
            .field("ChunkShift", &self.ChunkShift)
            .field("ClusterShift", &self.ClusterShift)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CompressedFileSize == other.CompressedFileSize
            && self.CompressionFormat == other.CompressionFormat
            && self.CompressionUnitShift == other.CompressionUnitShift
            && self.ChunkShift == other.ChunkShift
            && self.ClusterShift == other.ClusterShift
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for FILE_COMPRESSION_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_COMPRESSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_CREATION_DISPOSITION(pub u32);
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(1u32);
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(2u32);
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(3u32);
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(4u32);
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(5u32);
impl ::std::convert::From<u32> for FILE_CREATION_DISPOSITION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_CREATION_DISPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_CREATION_DISPOSITION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_CREATION_DISPOSITION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_CREATION_DISPOSITION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_CREATION_DISPOSITION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_CREATION_DISPOSITION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct FILE_DEVICE_TYPE(pub u32);
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(2u32);
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(7u32);
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(31u32);
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(51u32);
impl ::std::convert::From<u32> for FILE_DEVICE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_DEVICE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_DEVICE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_DEVICE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_DEVICE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_DEVICE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_DEVICE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFileA: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILE_DISPOSITION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FILE_DISPOSITION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_DISPOSITION_INFO")
            .field("DeleteFileA", &self.DeleteFileA)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILE_DISPOSITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeleteFileA == other.DeleteFileA
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILE_DISPOSITION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
impl FILE_END_OF_FILE_INFO {}
impl ::std::default::Default for FILE_END_OF_FILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_END_OF_FILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_END_OF_FILE_INFO")
            .field("EndOfFile", &self.EndOfFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_END_OF_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfFile == other.EndOfFile
    }
}
impl ::std::cmp::Eq for FILE_END_OF_FILE_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_END_OF_FILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
impl FILE_EXTENT {}
impl ::std::default::Default for FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_EXTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_EXTENT")
            .field("VolumeOffset", &self.VolumeOffset)
            .field("ExtentLength", &self.ExtentLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.ExtentLength == other.ExtentLength
    }
}
impl ::std::cmp::Eq for FILE_EXTENT {}
unsafe impl ::windows::runtime::Abi for FILE_EXTENT {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4u32);
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16u32);
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32u32);
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(64u32);
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(128u32);
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(256u32);
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(512u32);
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1024u32);
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2048u32);
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4096u32);
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(8192u32);
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16384u32);
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(32768u32);
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(3u32);
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(0u32);
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(196608u32);
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
impl ::std::convert::From<u32> for FILE_FLAGS_AND_ATTRIBUTES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_FLAGS_AND_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl FILE_FULL_DIR_INFO {}
impl ::std::default::Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_FULL_DIR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_FULL_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_FULL_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_FULL_DIR_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_FULL_DIR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl FILE_ID_128 {}
impl ::std::default::Default for FILE_ID_128 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ID_128 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ID_128")
            .field("Identifier", &self.Identifier)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier
    }
}
impl ::std::cmp::Eq for FILE_ID_128 {}
unsafe impl ::windows::runtime::Abi for FILE_ID_128 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl FILE_ID_BOTH_DIR_INFO {}
impl ::std::default::Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ID_BOTH_DIR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ID_BOTH_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ShortNameLength", &self.ShortNameLength)
            .field("ShortName", &self.ShortName)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ID_BOTH_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.ShortNameLength == other.ShortNameLength
            && self.ShortName == other.ShortName
            && self.FileId == other.FileId
            && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_ID_BOTH_DIR_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ID_BOTH_DIR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
impl FILE_ID_DESCRIPTOR {}
impl ::std::default::Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILE_ID_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILE_ID_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for FILE_ID_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: i64,
    pub ObjectId: ::windows::runtime::GUID,
    pub ExtendedFileId: FILE_ID_128,
}
impl FILE_ID_DESCRIPTOR_0 {}
impl ::std::default::Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILE_ID_DESCRIPTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILE_ID_DESCRIPTOR_0 {}
unsafe impl ::windows::runtime::Abi for FILE_ID_DESCRIPTOR_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl FILE_ID_EXTD_DIR_INFO {}
impl ::std::default::Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ID_EXTD_DIR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ID_EXTD_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ID_EXTD_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.ReparsePointTag == other.ReparsePointTag
            && self.FileId == other.FileId
            && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_ID_EXTD_DIR_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ID_EXTD_DIR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl FILE_ID_INFO {}
impl ::std::default::Default for FILE_ID_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_ID_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_ID_INFO")
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("FileId", &self.FileId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSerialNumber == other.VolumeSerialNumber && self.FileId == other.FileId
    }
}
impl ::std::cmp::Eq for FILE_ID_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_ID_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_ID_TYPE(pub i32);
pub const FileIdType: FILE_ID_TYPE = FILE_ID_TYPE(0i32);
pub const ObjectIdType: FILE_ID_TYPE = FILE_ID_TYPE(1i32);
pub const ExtendedFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(2i32);
pub const MaximumFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(3i32);
impl ::std::convert::From<i32> for FILE_ID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_ID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
impl FILE_INFO_2 {}
impl ::std::default::Default for FILE_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_INFO_2")
            .field("fi2_id", &self.fi2_id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.fi2_id == other.fi2_id
    }
}
impl ::std::cmp::Eq for FILE_INFO_2 {}
unsafe impl ::windows::runtime::Abi for FILE_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: FILE_INFO_FLAGS_PERMISSIONS,
    pub fi3_num_locks: u32,
    pub fi3_pathname: super::super::Foundation::PWSTR,
    pub fi3_username: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FILE_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILE_INFO_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FILE_INFO_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_INFO_3")
            .field("fi3_id", &self.fi3_id)
            .field("fi3_permissions", &self.fi3_permissions)
            .field("fi3_num_locks", &self.fi3_num_locks)
            .field("fi3_pathname", &self.fi3_pathname)
            .field("fi3_username", &self.fi3_username)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.fi3_id == other.fi3_id
            && self.fi3_permissions == other.fi3_permissions
            && self.fi3_num_locks == other.fi3_num_locks
            && self.fi3_pathname == other.fi3_pathname
            && self.fi3_username == other.fi3_username
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILE_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILE_INFO_3 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_INFO_BY_HANDLE_CLASS(pub i32);
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(0i32);
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(1i32);
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(2i32);
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(3i32);
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(4i32);
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(5i32);
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(6i32);
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(7i32);
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(8i32);
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(9i32);
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(10i32);
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(11i32);
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(12i32);
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(13i32);
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(14i32);
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(15i32);
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(16i32);
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(17i32);
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(18i32);
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(19i32);
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(20i32);
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(21i32);
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(22i32);
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(23i32);
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(24i32);
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(25i32);
impl ::std::convert::From<i32> for FILE_INFO_BY_HANDLE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_INFO_BY_HANDLE_CLASS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_INFO_FLAGS_PERMISSIONS(pub u32);
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(1u32);
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(2u32);
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(4u32);
impl ::std::convert::From<u32> for FILE_INFO_FLAGS_PERMISSIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_INFO_FLAGS_PERMISSIONS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
impl FILE_IO_PRIORITY_HINT_INFO {}
impl ::std::default::Default for FILE_IO_PRIORITY_HINT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_IO_PRIORITY_HINT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_IO_PRIORITY_HINT_INFO")
            .field("PriorityHint", &self.PriorityHint)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_IO_PRIORITY_HINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityHint == other.PriorityHint
    }
}
impl ::std::cmp::Eq for FILE_IO_PRIORITY_HINT_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_IO_PRIORITY_HINT_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_NAME(pub u32);
pub const FILE_NAME_NORMALIZED: FILE_NAME = FILE_NAME(0u32);
pub const FILE_NAME_OPENED: FILE_NAME = FILE_NAME(8u32);
impl ::std::convert::From<u32> for FILE_NAME {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_NAME {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_NAME {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_NAME {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_NAME {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_NAME {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_NAME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl FILE_NAME_INFO {}
impl ::std::default::Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_NAME_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_NAME_INFO")
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_NAME_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_NAME_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_NOTIFY_CHANGE(pub u32);
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(1u32);
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(2u32);
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(4u32);
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(8u32);
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(16u32);
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(32u32);
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(64u32);
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(256u32);
impl ::std::convert::From<u32> for FILE_NOTIFY_CHANGE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_NOTIFY_CHANGE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_NOTIFY_CHANGE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_NOTIFY_CHANGE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::std::default::Default for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_NOTIFY_EXTENDED_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Action", &self.Action)
            .field("CreationTime", &self.CreationTime)
            .field("LastModificationTime", &self.LastModificationTime)
            .field("LastChangeTime", &self.LastChangeTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("AllocatedLength", &self.AllocatedLength)
            .field("FileSize", &self.FileSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("ParentFileId", &self.ParentFileId)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.Action == other.Action
            && self.CreationTime == other.CreationTime
            && self.LastModificationTime == other.LastModificationTime
            && self.LastChangeTime == other.LastChangeTime
            && self.LastAccessTime == other.LastAccessTime
            && self.AllocatedLength == other.AllocatedLength
            && self.FileSize == other.FileSize
            && self.FileAttributes == other.FileAttributes
            && self.ReparsePointTag == other.ReparsePointTag
            && self.FileId == other.FileId
            && self.ParentFileId == other.ParentFileId
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_NOTIFY_EXTENDED_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILE_NOTIFY_EXTENDED_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl FILE_NOTIFY_INFORMATION {}
impl ::std::default::Default for FILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_NOTIFY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_NOTIFY_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Action", &self.Action)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_NOTIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.Action == other.Action
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
    }
}
impl ::std::cmp::Eq for FILE_NOTIFY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FILE_NOTIFY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl FILE_REMOTE_PROTOCOL_INFO {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl FILE_REMOTE_PROTOCOL_INFO_0 {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_GenericReserved_e__Struct")
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_0 {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [u32; 16],
}
impl FILE_REMOTE_PROTOCOL_INFO_1 {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1 {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
impl FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Smb2_e__Struct")
            .field("Server", &self.Server)
            .field("Share", &self.Share)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.Share == other.Share
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
impl FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Server_e__Struct")
            .field("Capabilities", &self.Capabilities)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub CachingFlags: u32,
}
impl FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl ::std::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Share_e__Struct")
            .field("Capabilities", &self.Capabilities)
            .field("CachingFlags", &self.CachingFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.CachingFlags == other.CachingFlags
    }
}
impl ::std::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
unsafe impl ::windows::runtime::Abi for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl FILE_RENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILE_RENAME_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILE_RENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILE_RENAME_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: super::super::Foundation::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FILE_RENAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILE_RENAME_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILE_RENAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILE_RENAME_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FILE_SEGMENT_ELEMENT {
    pub Buffer: *mut ::std::ffi::c_void,
    pub Alignment: u64,
}
impl FILE_SEGMENT_ELEMENT {}
impl ::std::default::Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FILE_SEGMENT_ELEMENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FILE_SEGMENT_ELEMENT {}
unsafe impl ::windows::runtime::Abi for FILE_SEGMENT_ELEMENT {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FILE_SHARE_MODE(pub u32);
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = FILE_SHARE_MODE(0u32);
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = FILE_SHARE_MODE(4u32);
pub const FILE_SHARE_READ: FILE_SHARE_MODE = FILE_SHARE_MODE(1u32);
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = FILE_SHARE_MODE(2u32);
impl ::std::convert::From<u32> for FILE_SHARE_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_SHARE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_SHARE_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_SHARE_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_SHARE_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_SHARE_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_SHARE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::Foundation::BOOLEAN,
    pub Directory: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FILE_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FILE_STANDARD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_STANDARD_INFO")
            .field("AllocationSize", &self.AllocationSize)
            .field("EndOfFile", &self.EndOfFile)
            .field("NumberOfLinks", &self.NumberOfLinks)
            .field("DeletePending", &self.DeletePending)
            .field("Directory", &self.Directory)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FILE_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
            && self.EndOfFile == other.EndOfFile
            && self.NumberOfLinks == other.NumberOfLinks
            && self.DeletePending == other.DeletePending
            && self.Directory == other.Directory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FILE_STANDARD_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl FILE_STORAGE_INFO {}
impl ::std::default::Default for FILE_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_STORAGE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_STORAGE_INFO")
            .field("LogicalBytesPerSector", &self.LogicalBytesPerSector)
            .field(
                "PhysicalBytesPerSectorForAtomicity",
                &self.PhysicalBytesPerSectorForAtomicity,
            )
            .field(
                "PhysicalBytesPerSectorForPerformance",
                &self.PhysicalBytesPerSectorForPerformance,
            )
            .field(
                "FileSystemEffectivePhysicalBytesPerSectorForAtomicity",
                &self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity,
            )
            .field("Flags", &self.Flags)
            .field(
                "ByteOffsetForSectorAlignment",
                &self.ByteOffsetForSectorAlignment,
            )
            .field(
                "ByteOffsetForPartitionAlignment",
                &self.ByteOffsetForPartitionAlignment,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalBytesPerSector == other.LogicalBytesPerSector
            && self.PhysicalBytesPerSectorForAtomicity == other.PhysicalBytesPerSectorForAtomicity
            && self.PhysicalBytesPerSectorForPerformance
                == other.PhysicalBytesPerSectorForPerformance
            && self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity
                == other.FileSystemEffectivePhysicalBytesPerSectorForAtomicity
            && self.Flags == other.Flags
            && self.ByteOffsetForSectorAlignment == other.ByteOffsetForSectorAlignment
            && self.ByteOffsetForPartitionAlignment == other.ByteOffsetForPartitionAlignment
    }
}
impl ::std::cmp::Eq for FILE_STORAGE_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_STORAGE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl FILE_STREAM_INFO {}
impl ::std::default::Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_STREAM_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_STREAM_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("StreamNameLength", &self.StreamNameLength)
            .field("StreamSize", &self.StreamSize)
            .field("StreamAllocationSize", &self.StreamAllocationSize)
            .field("StreamName", &self.StreamName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.StreamNameLength == other.StreamNameLength
            && self.StreamSize == other.StreamSize
            && self.StreamAllocationSize == other.StreamAllocationSize
            && self.StreamName == other.StreamName
    }
}
impl ::std::cmp::Eq for FILE_STREAM_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_STREAM_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FINDEX_INFO_LEVELS(pub i32);
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(0i32);
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(1i32);
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(2i32);
impl ::std::convert::From<i32> for FINDEX_INFO_LEVELS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FINDEX_INFO_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FINDEX_SEARCH_OPS(pub i32);
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(0i32);
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(1i32);
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(2i32);
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(3i32);
impl ::std::convert::From<i32> for FINDEX_SEARCH_OPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FINDEX_SEARCH_OPS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct FIND_FIRST_EX_FLAGS(pub u32);
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(1u32);
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(2u32);
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(4u32);
impl ::std::convert::From<u32> for FIND_FIRST_EX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FIND_FIRST_EX_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FIND_FIRST_EX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FIND_FIRST_EX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FIO_CONTEXT {
    pub m_dwTempHack: u32,
    pub m_dwSignature: u32,
    pub m_hFile: super::super::Foundation::HANDLE,
    pub m_dwLinesOffset: u32,
    pub m_dwHeaderLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FIO_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FIO_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FIO_CONTEXT")
            .field("m_dwTempHack", &self.m_dwTempHack)
            .field("m_dwSignature", &self.m_dwSignature)
            .field("m_hFile", &self.m_hFile)
            .field("m_dwLinesOffset", &self.m_dwLinesOffset)
            .field("m_dwHeaderLength", &self.m_dwHeaderLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FIO_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwTempHack == other.m_dwTempHack
            && self.m_dwSignature == other.m_dwSignature
            && self.m_hFile == other.m_hFile
            && self.m_dwLinesOffset == other.m_dwLinesOffset
            && self.m_dwHeaderLength == other.m_dwHeaderLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FIO_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileEncryptionStatusA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpstatus: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileEncryptionStatusA(
                lpfilename: super::super::Foundation::PSTR,
                lpstatus: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FileEncryptionStatusA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileEncryptionStatusW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    lpstatus: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileEncryptionStatusW(
                lpfilename: super::super::Foundation::PWSTR,
                lpstatus: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FileEncryptionStatusW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToLocalFileTime(
    lpfiletime: *const super::super::Foundation::FILETIME,
    lplocalfiletime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileTimeToLocalFileTime(
                lpfiletime: *const super::super::Foundation::FILETIME,
                lplocalfiletime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FileTimeToLocalFileTime(
            ::std::mem::transmute(lpfiletime),
            ::std::mem::transmute(lplocalfiletime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindChangeNotificationHandle(pub isize);
impl ::std::default::Default for FindChangeNotificationHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindChangeNotificationHandle {}
unsafe impl ::windows::runtime::Abi for FindChangeNotificationHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindClose<'a, Param0: ::windows::runtime::IntoParam<'a, FindFileHandle>>(
    hfindfile: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindClose(hfindfile: FindFileHandle) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindClose(hfindfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindCloseChangeNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindChangeNotificationHandle>,
>(
    hchangehandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindCloseChangeNotification(
                hchangehandle: FindChangeNotificationHandle,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindCloseChangeNotification(
            hchangehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindFileHandle(pub isize);
impl ::std::default::Default for FindFileHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindFileHandle {}
unsafe impl ::windows::runtime::Abi for FindFileHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindFileNameHandle(pub isize);
impl ::std::default::Default for FindFileNameHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindFileNameHandle {}
unsafe impl ::windows::runtime::Abi for FindFileNameHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstChangeNotificationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lppathname: Param0,
    bwatchsubtree: Param1,
    dwnotifyfilter: FILE_NOTIFY_CHANGE,
) -> FindChangeNotificationHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstChangeNotificationA(
                lppathname: super::super::Foundation::PSTR,
                bwatchsubtree: super::super::Foundation::BOOL,
                dwnotifyfilter: FILE_NOTIFY_CHANGE,
            ) -> FindChangeNotificationHandle;
        }
        ::std::mem::transmute(FindFirstChangeNotificationA(
            lppathname.into_param().abi(),
            bwatchsubtree.into_param().abi(),
            ::std::mem::transmute(dwnotifyfilter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstChangeNotificationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lppathname: Param0,
    bwatchsubtree: Param1,
    dwnotifyfilter: FILE_NOTIFY_CHANGE,
) -> FindChangeNotificationHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstChangeNotificationW(
                lppathname: super::super::Foundation::PWSTR,
                bwatchsubtree: super::super::Foundation::BOOL,
                dwnotifyfilter: FILE_NOTIFY_CHANGE,
            ) -> FindChangeNotificationHandle;
        }
        ::std::mem::transmute(FindFirstChangeNotificationW(
            lppathname.into_param().abi(),
            bwatchsubtree.into_param().abi(),
            ::std::mem::transmute(dwnotifyfilter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpfindfiledata: *mut WIN32_FIND_DATAA,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileA(
                lpfilename: super::super::Foundation::PSTR,
                lpfindfiledata: *mut WIN32_FIND_DATAA,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfindfiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    finfolevelid: FINDEX_INFO_LEVELS,
    lpfindfiledata: *mut ::std::ffi::c_void,
    fsearchop: FINDEX_SEARCH_OPS,
    lpsearchfilter: *mut ::std::ffi::c_void,
    dwadditionalflags: FIND_FIRST_EX_FLAGS,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileExA(
                lpfilename: super::super::Foundation::PSTR,
                finfolevelid: FINDEX_INFO_LEVELS,
                lpfindfiledata: *mut ::std::ffi::c_void,
                fsearchop: FINDEX_SEARCH_OPS,
                lpsearchfilter: *mut ::std::ffi::c_void,
                dwadditionalflags: FIND_FIRST_EX_FLAGS,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileExA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfindfiledata),
            ::std::mem::transmute(fsearchop),
            ::std::mem::transmute(lpsearchfilter),
            ::std::mem::transmute(dwadditionalflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileExFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    finfolevelid: FINDEX_INFO_LEVELS,
    lpfindfiledata: *mut ::std::ffi::c_void,
    fsearchop: FINDEX_SEARCH_OPS,
    lpsearchfilter: *mut ::std::ffi::c_void,
    dwadditionalflags: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileExFromAppW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: FINDEX_INFO_LEVELS,
                lpfindfiledata: *mut ::std::ffi::c_void,
                fsearchop: FINDEX_SEARCH_OPS,
                lpsearchfilter: *mut ::std::ffi::c_void,
                dwadditionalflags: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(FindFirstFileExFromAppW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfindfiledata),
            ::std::mem::transmute(fsearchop),
            ::std::mem::transmute(lpsearchfilter),
            ::std::mem::transmute(dwadditionalflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    finfolevelid: FINDEX_INFO_LEVELS,
    lpfindfiledata: *mut ::std::ffi::c_void,
    fsearchop: FINDEX_SEARCH_OPS,
    lpsearchfilter: *mut ::std::ffi::c_void,
    dwadditionalflags: FIND_FIRST_EX_FLAGS,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileExW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: FINDEX_INFO_LEVELS,
                lpfindfiledata: *mut ::std::ffi::c_void,
                fsearchop: FINDEX_SEARCH_OPS,
                lpsearchfilter: *mut ::std::ffi::c_void,
                dwadditionalflags: FIND_FIRST_EX_FLAGS,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileExW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfindfiledata),
            ::std::mem::transmute(fsearchop),
            ::std::mem::transmute(lpsearchfilter),
            ::std::mem::transmute(dwadditionalflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileNameTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwflags: u32,
    stringlength: *mut u32,
    linkname: super::super::Foundation::PWSTR,
    htransaction: Param4,
) -> FindFileNameHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileNameTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                dwflags: u32,
                stringlength: *mut u32,
                linkname: super::super::Foundation::PWSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> FindFileNameHandle;
        }
        ::std::mem::transmute(FindFirstFileNameTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(stringlength),
            ::std::mem::transmute(linkname),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwflags: u32,
    stringlength: *mut u32,
    linkname: super::super::Foundation::PWSTR,
) -> FindFileNameHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileNameW(
                lpfilename: super::super::Foundation::PWSTR,
                dwflags: u32,
                stringlength: *mut u32,
                linkname: super::super::Foundation::PWSTR,
            ) -> FindFileNameHandle;
        }
        ::std::mem::transmute(FindFirstFileNameW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(stringlength),
            ::std::mem::transmute(linkname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    finfolevelid: FINDEX_INFO_LEVELS,
    lpfindfiledata: *mut ::std::ffi::c_void,
    fsearchop: FINDEX_SEARCH_OPS,
    lpsearchfilter: *mut ::std::ffi::c_void,
    dwadditionalflags: u32,
    htransaction: Param6,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                finfolevelid: FINDEX_INFO_LEVELS,
                lpfindfiledata: *mut ::std::ffi::c_void,
                fsearchop: FINDEX_SEARCH_OPS,
                lpsearchfilter: *mut ::std::ffi::c_void,
                dwadditionalflags: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfindfiledata),
            ::std::mem::transmute(fsearchop),
            ::std::mem::transmute(lpsearchfilter),
            ::std::mem::transmute(dwadditionalflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    finfolevelid: FINDEX_INFO_LEVELS,
    lpfindfiledata: *mut ::std::ffi::c_void,
    fsearchop: FINDEX_SEARCH_OPS,
    lpsearchfilter: *mut ::std::ffi::c_void,
    dwadditionalflags: u32,
    htransaction: Param6,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: FINDEX_INFO_LEVELS,
                lpfindfiledata: *mut ::std::ffi::c_void,
                fsearchop: FINDEX_SEARCH_OPS,
                lpsearchfilter: *mut ::std::ffi::c_void,
                dwadditionalflags: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfindfiledata),
            ::std::mem::transmute(fsearchop),
            ::std::mem::transmute(lpsearchfilter),
            ::std::mem::transmute(dwadditionalflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    lpfindfiledata: *mut WIN32_FIND_DATAW,
) -> FindFileHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFileW(
                lpfilename: super::super::Foundation::PWSTR,
                lpfindfiledata: *mut WIN32_FIND_DATAW,
            ) -> FindFileHandle;
        }
        ::std::mem::transmute(FindFirstFileW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfindfiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstStreamTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    infolevel: STREAM_INFO_LEVELS,
    lpfindstreamdata: *mut ::std::ffi::c_void,
    dwflags: u32,
    htransaction: Param4,
) -> FindStreamHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstStreamTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                infolevel: STREAM_INFO_LEVELS,
                lpfindstreamdata: *mut ::std::ffi::c_void,
                dwflags: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> FindStreamHandle;
        }
        ::std::mem::transmute(FindFirstStreamTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(infolevel),
            ::std::mem::transmute(lpfindstreamdata),
            ::std::mem::transmute(dwflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstStreamW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    infolevel: STREAM_INFO_LEVELS,
    lpfindstreamdata: *mut ::std::ffi::c_void,
    dwflags: u32,
) -> FindStreamHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstStreamW(
                lpfilename: super::super::Foundation::PWSTR,
                infolevel: STREAM_INFO_LEVELS,
                lpfindstreamdata: *mut ::std::ffi::c_void,
                dwflags: u32,
            ) -> FindStreamHandle;
        }
        ::std::mem::transmute(FindFirstStreamW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(infolevel),
            ::std::mem::transmute(lpfindstreamdata),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstVolumeA(
    lpszvolumename: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> FindVolumeHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstVolumeA(
                lpszvolumename: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> FindVolumeHandle;
        }
        ::std::mem::transmute(FindFirstVolumeA(
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstVolumeMountPointA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszrootpathname: Param0,
    lpszvolumemountpoint: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> FindVolumeMointPointHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstVolumeMountPointA(
                lpszrootpathname: super::super::Foundation::PSTR,
                lpszvolumemountpoint: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> FindVolumeMointPointHandle;
        }
        ::std::mem::transmute(FindFirstVolumeMountPointA(
            lpszrootpathname.into_param().abi(),
            ::std::mem::transmute(lpszvolumemountpoint),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstVolumeMountPointW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszrootpathname: Param0,
    lpszvolumemountpoint: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> FindVolumeMointPointHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstVolumeMountPointW(
                lpszrootpathname: super::super::Foundation::PWSTR,
                lpszvolumemountpoint: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> FindVolumeMointPointHandle;
        }
        ::std::mem::transmute(FindFirstVolumeMountPointW(
            lpszrootpathname.into_param().abi(),
            ::std::mem::transmute(lpszvolumemountpoint),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstVolumeW(
    lpszvolumename: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> FindVolumeHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstVolumeW(
                lpszvolumename: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> FindVolumeHandle;
        }
        ::std::mem::transmute(FindFirstVolumeW(
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextChangeNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindChangeNotificationHandle>,
>(
    hchangehandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextChangeNotification(
                hchangehandle: FindChangeNotificationHandle,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextChangeNotification(hchangehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileA<'a, Param0: ::windows::runtime::IntoParam<'a, FindFileHandle>>(
    hfindfile: Param0,
    lpfindfiledata: *mut WIN32_FIND_DATAA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextFileA(
                hfindfile: FindFileHandle,
                lpfindfiledata: *mut WIN32_FIND_DATAA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextFileA(
            hfindfile.into_param().abi(),
            ::std::mem::transmute(lpfindfiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindFileNameHandle>,
>(
    hfindstream: Param0,
    stringlength: *mut u32,
    linkname: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextFileNameW(
                hfindstream: FindFileNameHandle,
                stringlength: *mut u32,
                linkname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextFileNameW(
            hfindstream.into_param().abi(),
            ::std::mem::transmute(stringlength),
            ::std::mem::transmute(linkname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfindfile: Param0,
    lpfindfiledata: *mut WIN32_FIND_DATAW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextFileW(
                hfindfile: super::super::Foundation::HANDLE,
                lpfindfiledata: *mut WIN32_FIND_DATAW,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextFileW(
            hfindfile.into_param().abi(),
            ::std::mem::transmute(lpfindfiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextStreamW<'a, Param0: ::windows::runtime::IntoParam<'a, FindStreamHandle>>(
    hfindstream: Param0,
    lpfindstreamdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextStreamW(
                hfindstream: FindStreamHandle,
                lpfindstreamdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextStreamW(
            hfindstream.into_param().abi(),
            ::std::mem::transmute(lpfindstreamdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeA<'a, Param0: ::windows::runtime::IntoParam<'a, FindVolumeHandle>>(
    hfindvolume: Param0,
    lpszvolumename: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextVolumeA(
                hfindvolume: FindVolumeHandle,
                lpszvolumename: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextVolumeA(
            hfindvolume.into_param().abi(),
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeMountPointA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindVolumeMointPointHandle>,
>(
    hfindvolumemountpoint: Param0,
    lpszvolumemountpoint: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextVolumeMountPointA(
                hfindvolumemountpoint: FindVolumeMointPointHandle,
                lpszvolumemountpoint: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextVolumeMountPointA(
            hfindvolumemountpoint.into_param().abi(),
            ::std::mem::transmute(lpszvolumemountpoint),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeMountPointW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindVolumeMointPointHandle>,
>(
    hfindvolumemountpoint: Param0,
    lpszvolumemountpoint: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextVolumeMountPointW(
                hfindvolumemountpoint: FindVolumeMointPointHandle,
                lpszvolumemountpoint: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextVolumeMountPointW(
            hfindvolumemountpoint.into_param().abi(),
            ::std::mem::transmute(lpszvolumemountpoint),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeW<'a, Param0: ::windows::runtime::IntoParam<'a, FindVolumeHandle>>(
    hfindvolume: Param0,
    lpszvolumename: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextVolumeW(
                hfindvolume: FindVolumeHandle,
                lpszvolumename: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindNextVolumeW(
            hfindvolume.into_param().abi(),
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindStreamHandle(pub isize);
impl ::std::default::Default for FindStreamHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindStreamHandle {}
unsafe impl ::windows::runtime::Abi for FindStreamHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindVolumeClose<'a, Param0: ::windows::runtime::IntoParam<'a, FindVolumeHandle>>(
    hfindvolume: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindVolumeClose(hfindvolume: FindVolumeHandle) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindVolumeClose(hfindvolume.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindVolumeHandle(pub isize);
impl ::std::default::Default for FindVolumeHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindVolumeHandle {}
unsafe impl ::windows::runtime::Abi for FindVolumeHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct FindVolumeMointPointHandle(pub isize);
impl ::std::default::Default for FindVolumeMointPointHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FindVolumeMointPointHandle {}
unsafe impl ::windows::runtime::Abi for FindVolumeMointPointHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindVolumeMountPointClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, FindVolumeMointPointHandle>,
>(
    hfindvolumemountpoint: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindVolumeMountPointClose(
                hfindvolumemountpoint: FindVolumeMointPointHandle,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindVolumeMountPointClose(
            hfindvolumemountpoint.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushFileBuffers<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushFileBuffers(
                hfile: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlushFileBuffers(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FlushLogBuffers(
    pvmarshal: *mut ::std::ffi::c_void,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushLogBuffers(
                pvmarshal: *mut ::std::ffi::c_void,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlushLogBuffers(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FlushLogToLsn(
    pvmarshalcontext: *mut ::std::ffi::c_void,
    plsnflush: *mut CLS_LSN,
    plsnlastflushed: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushLogToLsn(
                pvmarshalcontext: *mut ::std::ffi::c_void,
                plsnflush: *mut CLS_LSN,
                plsnlastflushed: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlushLogToLsn(
            ::std::mem::transmute(pvmarshalcontext),
            ::std::mem::transmute(plsnflush),
            ::std::mem::transmute(plsnlastflushed),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeEncryptedFileMetadata(pbmetadata: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeEncryptedFileMetadata(pbmetadata: *const u8);
        }
        ::std::mem::transmute(FreeEncryptedFileMetadata(::std::mem::transmute(pbmetadata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST);
        }
        ::std::mem::transmute(FreeEncryptionCertificateHashList(::std::mem::transmute(
            pusers,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeReservedLog(
    pvmarshal: *mut ::std::ffi::c_void,
    creservedrecords: u32,
    pcbadjustment: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeReservedLog(
                pvmarshal: *mut ::std::ffi::c_void,
                creservedrecords: u32,
                pcbadjustment: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FreeReservedLog(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(creservedrecords),
            ::std::mem::transmute(pcbadjustment),
        ))
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
pub struct GET_FILEEX_INFO_LEVELS(pub i32);
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(0i32);
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(1i32);
impl ::std::convert::From<i32> for GET_FILEEX_INFO_LEVELS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_FILEEX_INFO_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct GET_FILE_VERSION_INFO_FLAGS(pub u32);
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(1u32);
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(2u32);
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(4u32);
impl ::std::convert::From<u32> for GET_FILE_VERSION_INFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_FILE_VERSION_INFO_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(pub u32);
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION =
    GET_TAPE_DRIVE_PARAMETERS_OPERATION(1u32);
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION =
    GET_TAPE_DRIVE_PARAMETERS_OPERATION(0u32);
impl ::std::convert::From<u32> for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBinaryTypeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpapplicationname: Param0,
    lpbinarytype: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBinaryTypeA(
                lpapplicationname: super::super::Foundation::PSTR,
                lpbinarytype: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetBinaryTypeA(
            lpapplicationname.into_param().abi(),
            ::std::mem::transmute(lpbinarytype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBinaryTypeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpapplicationname: Param0,
    lpbinarytype: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBinaryTypeW(
                lpapplicationname: super::super::Foundation::PWSTR,
                lpbinarytype: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetBinaryTypeW(
            lpapplicationname.into_param().abi(),
            ::std::mem::transmute(lpbinarytype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpfilesizehigh: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCompressedFileSizeA(
                lpfilename: super::super::Foundation::PSTR,
                lpfilesizehigh: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetCompressedFileSizeA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    lpfilesizehigh: *mut u32,
    htransaction: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCompressedFileSizeTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                lpfilesizehigh: *mut u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetCompressedFileSizeTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfilesizehigh),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    lpfilesizehigh: *mut u32,
    htransaction: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCompressedFileSizeTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                lpfilesizehigh: *mut u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetCompressedFileSizeTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfilesizehigh),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    lpfilesizehigh: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCompressedFileSizeW(
                lpfilename: super::super::Foundation::PWSTR,
                lpfilesizehigh: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetCompressedFileSizeW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentClockTransactionManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionmanagerhandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentClockTransactionManager(
                transactionmanagerhandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentClockTransactionManager(
            transactionmanagerhandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lprootpathname: Param0,
    lpsectorspercluster: *mut u32,
    lpbytespersector: *mut u32,
    lpnumberoffreeclusters: *mut u32,
    lptotalnumberofclusters: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskFreeSpaceA(
                lprootpathname: super::super::Foundation::PSTR,
                lpsectorspercluster: *mut u32,
                lpbytespersector: *mut u32,
                lpnumberoffreeclusters: *mut u32,
                lptotalnumberofclusters: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDiskFreeSpaceA(
            lprootpathname.into_param().abi(),
            ::std::mem::transmute(lpsectorspercluster),
            ::std::mem::transmute(lpbytespersector),
            ::std::mem::transmute(lpnumberoffreeclusters),
            ::std::mem::transmute(lptotalnumberofclusters),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpdirectoryname: Param0,
    lpfreebytesavailabletocaller: *mut u64,
    lptotalnumberofbytes: *mut u64,
    lptotalnumberoffreebytes: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskFreeSpaceExA(
                lpdirectoryname: super::super::Foundation::PSTR,
                lpfreebytesavailabletocaller: *mut u64,
                lptotalnumberofbytes: *mut u64,
                lptotalnumberoffreebytes: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDiskFreeSpaceExA(
            lpdirectoryname.into_param().abi(),
            ::std::mem::transmute(lpfreebytesavailabletocaller),
            ::std::mem::transmute(lptotalnumberofbytes),
            ::std::mem::transmute(lptotalnumberoffreebytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpdirectoryname: Param0,
    lpfreebytesavailabletocaller: *mut u64,
    lptotalnumberofbytes: *mut u64,
    lptotalnumberoffreebytes: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskFreeSpaceExW(
                lpdirectoryname: super::super::Foundation::PWSTR,
                lpfreebytesavailabletocaller: *mut u64,
                lptotalnumberofbytes: *mut u64,
                lptotalnumberoffreebytes: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDiskFreeSpaceExW(
            lpdirectoryname.into_param().abi(),
            ::std::mem::transmute(lpfreebytesavailabletocaller),
            ::std::mem::transmute(lptotalnumberofbytes),
            ::std::mem::transmute(lptotalnumberoffreebytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lprootpathname: Param0,
    lpsectorspercluster: *mut u32,
    lpbytespersector: *mut u32,
    lpnumberoffreeclusters: *mut u32,
    lptotalnumberofclusters: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskFreeSpaceW(
                lprootpathname: super::super::Foundation::PWSTR,
                lpsectorspercluster: *mut u32,
                lpbytespersector: *mut u32,
                lpnumberoffreeclusters: *mut u32,
                lptotalnumberofclusters: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDiskFreeSpaceW(
            lprootpathname.into_param().abi(),
            ::std::mem::transmute(lpsectorspercluster),
            ::std::mem::transmute(lpbytespersector),
            ::std::mem::transmute(lpnumberoffreeclusters),
            ::std::mem::transmute(lptotalnumberofclusters),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskSpaceInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    rootpath: Param0,
) -> ::windows::runtime::Result<DISK_SPACE_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskSpaceInformationA(
                rootpath: super::super::Foundation::PSTR,
                diskspaceinfo: *mut DISK_SPACE_INFORMATION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DISK_SPACE_INFORMATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetDiskSpaceInformationA(rootpath.into_param().abi(), &mut result__)
            .from_abi::<DISK_SPACE_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskSpaceInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    rootpath: Param0,
) -> ::windows::runtime::Result<DISK_SPACE_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskSpaceInformationW(
                rootpath: super::super::Foundation::PWSTR,
                diskspaceinfo: *mut DISK_SPACE_INFORMATION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DISK_SPACE_INFORMATION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetDiskSpaceInformationW(rootpath.into_param().abi(), &mut result__)
            .from_abi::<DISK_SPACE_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDriveTypeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lprootpathname: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDriveTypeA(lprootpathname: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetDriveTypeA(lprootpathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDriveTypeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lprootpathname: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDriveTypeW(lprootpathname: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetDriveTypeW(lprootpathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEncryptedFileMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    pcbmetadata: *mut u32,
    ppbmetadata: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEncryptedFileMetadata(
                lpfilename: super::super::Foundation::PWSTR,
                pcbmetadata: *mut u32,
                ppbmetadata: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(GetEncryptedFileMetadata(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(pcbmetadata),
            ::std::mem::transmute(ppbmetadata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnlistmentId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    enlistmentid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnlistmentId(
                enlistmenthandle: super::super::Foundation::HANDLE,
                enlistmentid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetEnlistmentId(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(enlistmentid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    buffersize: u32,
    buffer: *mut ::std::ffi::c_void,
    bufferused: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnlistmentRecoveryInformation(
                enlistmenthandle: super::super::Foundation::HANDLE,
                buffersize: u32,
                buffer: *mut ::std::ffi::c_void,
                bufferused: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetEnlistmentRecoveryInformation(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(bufferused),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExpandedNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszsource: Param0,
    lpszbuffer: super::super::Foundation::PSTR,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedNameA(
                lpszsource: super::super::Foundation::PSTR,
                lpszbuffer: super::super::Foundation::PSTR,
            ) -> i32;
        }
        ::std::mem::transmute(GetExpandedNameA(
            lpszsource.into_param().abi(),
            ::std::mem::transmute(lpszbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExpandedNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszsource: Param0,
    lpszbuffer: super::super::Foundation::PWSTR,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedNameW(
                lpszsource: super::super::Foundation::PWSTR,
                lpszbuffer: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(GetExpandedNameW(
            lpszsource.into_param().abi(),
            ::std::mem::transmute(lpszbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesA(lpfilename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetFileAttributesA(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    finfolevelid: GET_FILEEX_INFO_LEVELS,
    lpfileinformation: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesExA(
                lpfilename: super::super::Foundation::PSTR,
                finfolevelid: GET_FILEEX_INFO_LEVELS,
                lpfileinformation: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileAttributesExA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfileinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    finfolevelid: GET_FILEEX_INFO_LEVELS,
    lpfileinformation: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesExFromAppW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: GET_FILEEX_INFO_LEVELS,
                lpfileinformation: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileAttributesExFromAppW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfileinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    finfolevelid: GET_FILEEX_INFO_LEVELS,
    lpfileinformation: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesExW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: GET_FILEEX_INFO_LEVELS,
                lpfileinformation: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileAttributesExW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfileinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    finfolevelid: GET_FILEEX_INFO_LEVELS,
    lpfileinformation: *mut ::std::ffi::c_void,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                finfolevelid: GET_FILEEX_INFO_LEVELS,
                lpfileinformation: *mut ::std::ffi::c_void,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileAttributesTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfileinformation),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    finfolevelid: GET_FILEEX_INFO_LEVELS,
    lpfileinformation: *mut ::std::ffi::c_void,
    htransaction: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                finfolevelid: GET_FILEEX_INFO_LEVELS,
                lpfileinformation: *mut ::std::ffi::c_void,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileAttributesTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(finfolevelid),
            ::std::mem::transmute(lpfileinformation),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileAttributesW(lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetFileAttributesW(lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileBandwidthReservation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpperiodmilliseconds: *mut u32,
    lpbytesperperiod: *mut u32,
    pdiscardable: *mut i32,
    lptransfersize: *mut u32,
    lpnumoutstandingrequests: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileBandwidthReservation(
                hfile: super::super::Foundation::HANDLE,
                lpperiodmilliseconds: *mut u32,
                lpbytesperperiod: *mut u32,
                pdiscardable: *mut i32,
                lptransfersize: *mut u32,
                lpnumoutstandingrequests: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileBandwidthReservation(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpperiodmilliseconds),
            ::std::mem::transmute(lpbytesperperiod),
            ::std::mem::transmute(pdiscardable),
            ::std::mem::transmute(lptransfersize),
            ::std::mem::transmute(lpnumoutstandingrequests),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileInformationByHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileInformationByHandle(
                hfile: super::super::Foundation::HANDLE,
                lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileInformationByHandle(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfileinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileInformationByHandleEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    fileinformationclass: FILE_INFO_BY_HANDLE_CLASS,
    lpfileinformation: *mut ::std::ffi::c_void,
    dwbuffersize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileInformationByHandleEx(
                hfile: super::super::Foundation::HANDLE,
                fileinformationclass: FILE_INFO_BY_HANDLE_CLASS,
                lpfileinformation: *mut ::std::ffi::c_void,
                dwbuffersize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileInformationByHandleEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(fileinformationclass),
            ::std::mem::transmute(lpfileinformation),
            ::std::mem::transmute(dwbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpfilesizehigh: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSize(
                hfile: super::super::Foundation::HANDLE,
                lpfilesizehigh: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetFileSize(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSizeEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpfilesize: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSizeEx(
                hfile: super::super::Foundation::HANDLE,
                lpfilesize: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileSizeEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileTime<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpcreationtime: *mut super::super::Foundation::FILETIME,
    lplastaccesstime: *mut super::super::Foundation::FILETIME,
    lplastwritetime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileTime(
                hfile: super::super::Foundation::HANDLE,
                lpcreationtime: *mut super::super::Foundation::FILETIME,
                lplastaccesstime: *mut super::super::Foundation::FILETIME,
                lplastwritetime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileTime(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpcreationtime),
            ::std::mem::transmute(lplastaccesstime),
            ::std::mem::transmute(lplastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileType<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileType(hfile: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetFileType(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lptstrfilename: Param0,
    dwhandle: u32,
    dwlen: u32,
    lpdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoA(
                lptstrfilename: super::super::Foundation::PSTR,
                dwhandle: u32,
                dwlen: u32,
                lpdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileVersionInfoA(
            lptstrfilename.into_param().abi(),
            ::std::mem::transmute(dwhandle),
            ::std::mem::transmute(dwlen),
            ::std::mem::transmute(lpdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoExA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: GET_FILE_VERSION_INFO_FLAGS,
    lpwstrfilename: Param1,
    dwhandle: u32,
    dwlen: u32,
    lpdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoExA(
                dwflags: GET_FILE_VERSION_INFO_FLAGS,
                lpwstrfilename: super::super::Foundation::PSTR,
                dwhandle: u32,
                dwlen: u32,
                lpdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileVersionInfoExA(
            ::std::mem::transmute(dwflags),
            lpwstrfilename.into_param().abi(),
            ::std::mem::transmute(dwhandle),
            ::std::mem::transmute(dwlen),
            ::std::mem::transmute(lpdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: GET_FILE_VERSION_INFO_FLAGS,
    lpwstrfilename: Param1,
    dwhandle: u32,
    dwlen: u32,
    lpdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoExW(
                dwflags: GET_FILE_VERSION_INFO_FLAGS,
                lpwstrfilename: super::super::Foundation::PWSTR,
                dwhandle: u32,
                dwlen: u32,
                lpdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileVersionInfoExW(
            ::std::mem::transmute(dwflags),
            lpwstrfilename.into_param().abi(),
            ::std::mem::transmute(dwhandle),
            ::std::mem::transmute(dwlen),
            ::std::mem::transmute(lpdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoSizeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lptstrfilename: Param0,
    lpdwhandle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoSizeA(
                lptstrfilename: super::super::Foundation::PSTR,
                lpdwhandle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetFileVersionInfoSizeA(
            lptstrfilename.into_param().abi(),
            ::std::mem::transmute(lpdwhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoSizeExA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: GET_FILE_VERSION_INFO_FLAGS,
    lpwstrfilename: Param1,
    lpdwhandle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoSizeExA(
                dwflags: GET_FILE_VERSION_INFO_FLAGS,
                lpwstrfilename: super::super::Foundation::PSTR,
                lpdwhandle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetFileVersionInfoSizeExA(
            ::std::mem::transmute(dwflags),
            lpwstrfilename.into_param().abi(),
            ::std::mem::transmute(lpdwhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoSizeExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: GET_FILE_VERSION_INFO_FLAGS,
    lpwstrfilename: Param1,
    lpdwhandle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoSizeExW(
                dwflags: GET_FILE_VERSION_INFO_FLAGS,
                lpwstrfilename: super::super::Foundation::PWSTR,
                lpdwhandle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetFileVersionInfoSizeExW(
            ::std::mem::transmute(dwflags),
            lpwstrfilename.into_param().abi(),
            ::std::mem::transmute(lpdwhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoSizeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptstrfilename: Param0,
    lpdwhandle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoSizeW(
                lptstrfilename: super::super::Foundation::PWSTR,
                lpdwhandle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetFileVersionInfoSizeW(
            lptstrfilename.into_param().abi(),
            ::std::mem::transmute(lpdwhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptstrfilename: Param0,
    dwhandle: u32,
    dwlen: u32,
    lpdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileVersionInfoW(
                lptstrfilename: super::super::Foundation::PWSTR,
                dwhandle: u32,
                dwlen: u32,
                lpdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileVersionInfoW(
            lptstrfilename.into_param().abi(),
            ::std::mem::transmute(dwhandle),
            ::std::mem::transmute(dwlen),
            ::std::mem::transmute(lpdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpszfilepath: super::super::Foundation::PSTR,
    cchfilepath: u32,
    dwflags: FILE_NAME,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFinalPathNameByHandleA(
                hfile: super::super::Foundation::HANDLE,
                lpszfilepath: super::super::Foundation::PSTR,
                cchfilepath: u32,
                dwflags: FILE_NAME,
            ) -> u32;
        }
        ::std::mem::transmute(GetFinalPathNameByHandleA(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpszfilepath),
            ::std::mem::transmute(cchfilepath),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpszfilepath: super::super::Foundation::PWSTR,
    cchfilepath: u32,
    dwflags: FILE_NAME,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFinalPathNameByHandleW(
                hfile: super::super::Foundation::HANDLE,
                lpszfilepath: super::super::Foundation::PWSTR,
                cchfilepath: u32,
                dwflags: FILE_NAME,
            ) -> u32;
        }
        ::std::mem::transmute(GetFinalPathNameByHandleW(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpszfilepath),
            ::std::mem::transmute(cchfilepath),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PSTR,
    lpfilepart: *mut super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFullPathNameA(
                lpfilename: super::super::Foundation::PSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PSTR,
                lpfilepart: *mut super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetFullPathNameA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PSTR,
    lpfilepart: *mut super::super::Foundation::PSTR,
    htransaction: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFullPathNameTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PSTR,
                lpfilepart: *mut super::super::Foundation::PSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetFullPathNameTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PWSTR,
    lpfilepart: *mut super::super::Foundation::PWSTR,
    htransaction: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFullPathNameTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PWSTR,
                lpfilepart: *mut super::super::Foundation::PWSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetFullPathNameTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PWSTR,
    lpfilepart: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFullPathNameW(
                lpfilename: super::super::Foundation::PWSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PWSTR,
                lpfilepart: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetFullPathNameW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetIoRingInfo(ioring: *const HIORING__) -> ::windows::runtime::Result<IORING_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIoRingInfo(
                ioring: *const HIORING__,
                info: *mut IORING_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IORING_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetIoRingInfo(::std::mem::transmute(ioring), &mut result__)
            .from_abi::<IORING_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogContainerName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hlog: Param0,
    cidlogicalcontainer: u32,
    pwstrcontainername: Param2,
    clencontainername: u32,
    pcactuallencontainername: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogContainerName(
                hlog: super::super::Foundation::HANDLE,
                cidlogicalcontainer: u32,
                pwstrcontainername: super::super::Foundation::PWSTR,
                clencontainername: u32,
                pcactuallencontainername: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogContainerName(
            hlog.into_param().abi(),
            ::std::mem::transmute(cidlogicalcontainer),
            pwstrcontainername.into_param().abi(),
            ::std::mem::transmute(clencontainername),
            ::std::mem::transmute(pcactuallencontainername),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogFileInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pinfobuffer: *mut CLS_INFORMATION,
    cbbuffer: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogFileInformation(
                hlog: super::super::Foundation::HANDLE,
                pinfobuffer: *mut CLS_INFORMATION,
                cbbuffer: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogFileInformation(
            hlog.into_param().abi(),
            ::std::mem::transmute(pinfobuffer),
            ::std::mem::transmute(cbbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogIoStatistics<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pvstatsbuffer: *mut ::std::ffi::c_void,
    cbstatsbuffer: u32,
    estatsclass: CLFS_IOSTATS_CLASS,
    pcbstatswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogIoStatistics(
                hlog: super::super::Foundation::HANDLE,
                pvstatsbuffer: *mut ::std::ffi::c_void,
                cbstatsbuffer: u32,
                estatsclass: CLFS_IOSTATS_CLASS,
                pcbstatswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogIoStatistics(
            hlog.into_param().abi(),
            ::std::mem::transmute(pvstatsbuffer),
            ::std::mem::transmute(cbstatsbuffer),
            ::std::mem::transmute(estatsclass),
            ::std::mem::transmute(pcbstatswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogReservationInfo(
    pvmarshal: *const ::std::ffi::c_void,
    pcbrecordnumber: *mut u32,
    pcbuserreservation: *mut i64,
    pcbcommitreservation: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogReservationInfo(
                pvmarshal: *const ::std::ffi::c_void,
                pcbrecordnumber: *mut u32,
                pcbuserreservation: *mut i64,
                pcbcommitreservation: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogReservationInfo(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(pcbrecordnumber),
            ::std::mem::transmute(pcbuserreservation),
            ::std::mem::transmute(pcbcommitreservation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalDriveStringsA(
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalDriveStringsA(
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetLogicalDriveStringsA(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalDriveStringsW(
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalDriveStringsW(
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetLogicalDriveStringsW(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLogicalDrives() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalDrives() -> u32;
        }
        ::std::mem::transmute(GetLogicalDrives())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszshortpath: Param0,
    lpszlongpath: super::super::Foundation::PSTR,
    cchbuffer: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLongPathNameA(
                lpszshortpath: super::super::Foundation::PSTR,
                lpszlongpath: super::super::Foundation::PSTR,
                cchbuffer: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetLongPathNameA(
            lpszshortpath.into_param().abi(),
            ::std::mem::transmute(lpszlongpath),
            ::std::mem::transmute(cchbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpszshortpath: Param0,
    lpszlongpath: super::super::Foundation::PSTR,
    cchbuffer: u32,
    htransaction: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLongPathNameTransactedA(
                lpszshortpath: super::super::Foundation::PSTR,
                lpszlongpath: super::super::Foundation::PSTR,
                cchbuffer: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetLongPathNameTransactedA(
            lpszshortpath.into_param().abi(),
            ::std::mem::transmute(lpszlongpath),
            ::std::mem::transmute(cchbuffer),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpszshortpath: Param0,
    lpszlongpath: super::super::Foundation::PWSTR,
    cchbuffer: u32,
    htransaction: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLongPathNameTransactedW(
                lpszshortpath: super::super::Foundation::PWSTR,
                lpszlongpath: super::super::Foundation::PWSTR,
                cchbuffer: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(GetLongPathNameTransactedW(
            lpszshortpath.into_param().abi(),
            ::std::mem::transmute(lpszlongpath),
            ::std::mem::transmute(cchbuffer),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszshortpath: Param0,
    lpszlongpath: super::super::Foundation::PWSTR,
    cchbuffer: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLongPathNameW(
                lpszshortpath: super::super::Foundation::PWSTR,
                lpszlongpath: super::super::Foundation::PWSTR,
                cchbuffer: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetLongPathNameW(
            lpszshortpath.into_param().abi(),
            ::std::mem::transmute(lpszlongpath),
            ::std::mem::transmute(cchbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNextLogArchiveExtent(
    pvarchivecontext: *mut ::std::ffi::c_void,
    rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR,
    cdescriptors: u32,
    pcdescriptorsreturned: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNextLogArchiveExtent(
                pvarchivecontext: *mut ::std::ffi::c_void,
                rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR,
                cdescriptors: u32,
                pcdescriptorsreturned: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNextLogArchiveExtent(
            ::std::mem::transmute(pvarchivecontext),
            ::std::mem::transmute(rgadextent),
            ::std::mem::transmute(cdescriptors),
            ::std::mem::transmute(pcdescriptorsreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNotificationResourceManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    resourcemanagerhandle: Param0,
    transactionnotification: *mut TRANSACTION_NOTIFICATION,
    notificationlength: u32,
    dwmilliseconds: u32,
    returnlength: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNotificationResourceManager(
                resourcemanagerhandle: super::super::Foundation::HANDLE,
                transactionnotification: *mut TRANSACTION_NOTIFICATION,
                notificationlength: u32,
                dwmilliseconds: u32,
                returnlength: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNotificationResourceManager(
            resourcemanagerhandle.into_param().abi(),
            ::std::mem::transmute(transactionnotification),
            ::std::mem::transmute(notificationlength),
            ::std::mem::transmute(dwmilliseconds),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    resourcemanagerhandle: Param0,
    transactionnotification: *mut TRANSACTION_NOTIFICATION,
    transactionnotificationlength: u32,
    returnlength: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNotificationResourceManagerAsync(
                resourcemanagerhandle: super::super::Foundation::HANDLE,
                transactionnotification: *mut TRANSACTION_NOTIFICATION,
                transactionnotificationlength: u32,
                returnlength: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNotificationResourceManagerAsync(
            resourcemanagerhandle.into_param().abi(),
            ::std::mem::transmute(transactionnotification),
            ::std::mem::transmute(transactionnotificationlength),
            ::std::mem::transmute(returnlength),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetShortPathNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszlongpath: Param0,
    lpszshortpath: super::super::Foundation::PSTR,
    cchbuffer: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetShortPathNameA(
                lpszlongpath: super::super::Foundation::PSTR,
                lpszshortpath: super::super::Foundation::PSTR,
                cchbuffer: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetShortPathNameA(
            lpszlongpath.into_param().abi(),
            ::std::mem::transmute(lpszshortpath),
            ::std::mem::transmute(cchbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetShortPathNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszlongpath: Param0,
    lpszshortpath: super::super::Foundation::PWSTR,
    cchbuffer: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetShortPathNameW(
                lpszlongpath: super::super::Foundation::PWSTR,
                lpszshortpath: super::super::Foundation::PWSTR,
                cchbuffer: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetShortPathNameW(
            lpszlongpath.into_param().abi(),
            ::std::mem::transmute(lpszshortpath),
            ::std::mem::transmute(cchbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapeParameters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION,
    lpdwsize: *mut u32,
    lptapeinformation: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTapeParameters(
                hdevice: super::super::Foundation::HANDLE,
                dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION,
                lpdwsize: *mut u32,
                lptapeinformation: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(GetTapeParameters(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwoperation),
            ::std::mem::transmute(lpdwsize),
            ::std::mem::transmute(lptapeinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapePosition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    dwpositiontype: TAPE_POSITION_TYPE,
    lpdwpartition: *mut u32,
    lpdwoffsetlow: *mut u32,
    lpdwoffsethigh: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTapePosition(
                hdevice: super::super::Foundation::HANDLE,
                dwpositiontype: TAPE_POSITION_TYPE,
                lpdwpartition: *mut u32,
                lpdwoffsetlow: *mut u32,
                lpdwoffsethigh: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetTapePosition(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwpositiontype),
            ::std::mem::transmute(lpdwpartition),
            ::std::mem::transmute(lpdwoffsetlow),
            ::std::mem::transmute(lpdwoffsethigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapeStatus<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTapeStatus(hdevice: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetTapeStatus(hdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempFileNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lppathname: Param0,
    lpprefixstring: Param1,
    uunique: u32,
    lptempfilename: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempFileNameA(
                lppathname: super::super::Foundation::PSTR,
                lpprefixstring: super::super::Foundation::PSTR,
                uunique: u32,
                lptempfilename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetTempFileNameA(
            lppathname.into_param().abi(),
            lpprefixstring.into_param().abi(),
            ::std::mem::transmute(uunique),
            ::std::mem::transmute(lptempfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
    lpprefixstring: Param1,
    uunique: u32,
    lptempfilename: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempFileNameW(
                lppathname: super::super::Foundation::PWSTR,
                lpprefixstring: super::super::Foundation::PWSTR,
                uunique: u32,
                lptempfilename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetTempFileNameW(
            lppathname.into_param().abi(),
            lpprefixstring.into_param().abi(),
            ::std::mem::transmute(uunique),
            ::std::mem::transmute(lptempfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempPath2A(bufferlength: u32, buffer: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempPath2A(bufferlength: u32, buffer: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetTempPath2A(
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempPath2W(bufferlength: u32, buffer: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempPath2W(bufferlength: u32, buffer: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetTempPath2W(
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempPathA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempPathA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetTempPathA(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTempPathW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTempPathW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetTempPathW(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
    transactionid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTransactionId(
                transactionhandle: super::super::Foundation::HANDLE,
                transactionid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTransactionId(
            transactionhandle.into_param().abi(),
            ::std::mem::transmute(transactionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
    outcome: *mut u32,
    isolationlevel: *mut u32,
    isolationflags: *mut u32,
    timeout: *mut u32,
    bufferlength: u32,
    description: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTransactionInformation(
                transactionhandle: super::super::Foundation::HANDLE,
                outcome: *mut u32,
                isolationlevel: *mut u32,
                isolationflags: *mut u32,
                timeout: *mut u32,
                bufferlength: u32,
                description: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTransactionInformation(
            transactionhandle.into_param().abi(),
            ::std::mem::transmute(outcome),
            ::std::mem::transmute(isolationlevel),
            ::std::mem::transmute(isolationflags),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(description),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionManagerId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionmanagerhandle: Param0,
    transactionmanagerid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTransactionManagerId(
                transactionmanagerhandle: super::super::Foundation::HANDLE,
                transactionmanagerid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTransactionManagerId(
            transactionmanagerhandle.into_param().abi(),
            ::std::mem::transmute(transactionmanagerid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lprootpathname: Param0,
    lpvolumenamebuffer: super::super::Foundation::PSTR,
    nvolumenamesize: u32,
    lpvolumeserialnumber: *mut u32,
    lpmaximumcomponentlength: *mut u32,
    lpfilesystemflags: *mut u32,
    lpfilesystemnamebuffer: super::super::Foundation::PSTR,
    nfilesystemnamesize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumeInformationA(
                lprootpathname: super::super::Foundation::PSTR,
                lpvolumenamebuffer: super::super::Foundation::PSTR,
                nvolumenamesize: u32,
                lpvolumeserialnumber: *mut u32,
                lpmaximumcomponentlength: *mut u32,
                lpfilesystemflags: *mut u32,
                lpfilesystemnamebuffer: super::super::Foundation::PSTR,
                nfilesystemnamesize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumeInformationA(
            lprootpathname.into_param().abi(),
            ::std::mem::transmute(lpvolumenamebuffer),
            ::std::mem::transmute(nvolumenamesize),
            ::std::mem::transmute(lpvolumeserialnumber),
            ::std::mem::transmute(lpmaximumcomponentlength),
            ::std::mem::transmute(lpfilesystemflags),
            ::std::mem::transmute(lpfilesystemnamebuffer),
            ::std::mem::transmute(nfilesystemnamesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationByHandleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpvolumenamebuffer: super::super::Foundation::PWSTR,
    nvolumenamesize: u32,
    lpvolumeserialnumber: *mut u32,
    lpmaximumcomponentlength: *mut u32,
    lpfilesystemflags: *mut u32,
    lpfilesystemnamebuffer: super::super::Foundation::PWSTR,
    nfilesystemnamesize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumeInformationByHandleW(
                hfile: super::super::Foundation::HANDLE,
                lpvolumenamebuffer: super::super::Foundation::PWSTR,
                nvolumenamesize: u32,
                lpvolumeserialnumber: *mut u32,
                lpmaximumcomponentlength: *mut u32,
                lpfilesystemflags: *mut u32,
                lpfilesystemnamebuffer: super::super::Foundation::PWSTR,
                nfilesystemnamesize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumeInformationByHandleW(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpvolumenamebuffer),
            ::std::mem::transmute(nvolumenamesize),
            ::std::mem::transmute(lpvolumeserialnumber),
            ::std::mem::transmute(lpmaximumcomponentlength),
            ::std::mem::transmute(lpfilesystemflags),
            ::std::mem::transmute(lpfilesystemnamebuffer),
            ::std::mem::transmute(nfilesystemnamesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lprootpathname: Param0,
    lpvolumenamebuffer: super::super::Foundation::PWSTR,
    nvolumenamesize: u32,
    lpvolumeserialnumber: *mut u32,
    lpmaximumcomponentlength: *mut u32,
    lpfilesystemflags: *mut u32,
    lpfilesystemnamebuffer: super::super::Foundation::PWSTR,
    nfilesystemnamesize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumeInformationW(
                lprootpathname: super::super::Foundation::PWSTR,
                lpvolumenamebuffer: super::super::Foundation::PWSTR,
                nvolumenamesize: u32,
                lpvolumeserialnumber: *mut u32,
                lpmaximumcomponentlength: *mut u32,
                lpfilesystemflags: *mut u32,
                lpfilesystemnamebuffer: super::super::Foundation::PWSTR,
                nfilesystemnamesize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumeInformationW(
            lprootpathname.into_param().abi(),
            ::std::mem::transmute(lpvolumenamebuffer),
            ::std::mem::transmute(nvolumenamesize),
            ::std::mem::transmute(lpvolumeserialnumber),
            ::std::mem::transmute(lpmaximumcomponentlength),
            ::std::mem::transmute(lpfilesystemflags),
            ::std::mem::transmute(lpfilesystemnamebuffer),
            ::std::mem::transmute(nfilesystemnamesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszvolumemountpoint: Param0,
    lpszvolumename: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumeNameForVolumeMountPointA(
                lpszvolumemountpoint: super::super::Foundation::PSTR,
                lpszvolumename: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumeNameForVolumeMountPointA(
            lpszvolumemountpoint.into_param().abi(),
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszvolumemountpoint: Param0,
    lpszvolumename: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumeNameForVolumeMountPointW(
                lpszvolumemountpoint: super::super::Foundation::PWSTR,
                lpszvolumename: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumeNameForVolumeMountPointW(
            lpszvolumemountpoint.into_param().abi(),
            ::std::mem::transmute(lpszvolumename),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszfilename: Param0,
    lpszvolumepathname: super::super::Foundation::PSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumePathNameA(
                lpszfilename: super::super::Foundation::PSTR,
                lpszvolumepathname: super::super::Foundation::PSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumePathNameA(
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(lpszvolumepathname),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszfilename: Param0,
    lpszvolumepathname: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumePathNameW(
                lpszfilename: super::super::Foundation::PWSTR,
                lpszvolumepathname: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumePathNameW(
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(lpszvolumepathname),
            ::std::mem::transmute(cchbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszvolumename: Param0,
    lpszvolumepathnames: super::super::Foundation::PSTR,
    cchbufferlength: u32,
    lpcchreturnlength: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumePathNamesForVolumeNameA(
                lpszvolumename: super::super::Foundation::PSTR,
                lpszvolumepathnames: super::super::Foundation::PSTR,
                cchbufferlength: u32,
                lpcchreturnlength: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumePathNamesForVolumeNameA(
            lpszvolumename.into_param().abi(),
            ::std::mem::transmute(lpszvolumepathnames),
            ::std::mem::transmute(cchbufferlength),
            ::std::mem::transmute(lpcchreturnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszvolumename: Param0,
    lpszvolumepathnames: super::super::Foundation::PWSTR,
    cchbufferlength: u32,
    lpcchreturnlength: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVolumePathNamesForVolumeNameW(
                lpszvolumename: super::super::Foundation::PWSTR,
                lpszvolumepathnames: super::super::Foundation::PWSTR,
                cchbufferlength: u32,
                lpcchreturnlength: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVolumePathNamesForVolumeNameW(
            lpszvolumename.into_param().abi(),
            ::std::mem::transmute(lpszvolumepathnames),
            ::std::mem::transmute(cchbufferlength),
            ::std::mem::transmute(lpcchreturnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HIORING__ {
    pub unused: i32,
}
impl HIORING__ {}
impl ::std::default::Default for HIORING__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HIORING__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HIORING__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HIORING__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for HIORING__ {}
unsafe impl ::windows::runtime::Abi for HIORING__ {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HandleLogFull<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HandleLogFull(
                hlog: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HandleLogFull(hlog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDiskQuotaControl(::windows::runtime::IUnknown);
impl IDiskQuotaControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumConnectionPoints(
        &self,
    ) -> ::windows::runtime::Result<super::super::System::Com::IEnumConnectionPoints> {
        let mut result__ : < super::super::System::Com:: IEnumConnectionPoints as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::IEnumConnectionPoints>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindConnectionPoint(
        &self,
        riid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<super::super::System::Com::IConnectionPoint> {
        let mut result__ : < super::super::System::Com:: IConnectionPoint as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::IConnectionPoint>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pszpath: Param0,
        breadwrite: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszpath.into_param().abi(),
            breadwrite.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetQuotaState(&self, dwstate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwstate),
        )
        .ok()
    }
    pub unsafe fn GetQuotaState(&self, pdwstate: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwstate),
        )
        .ok()
    }
    pub unsafe fn SetQuotaLogFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
    pub unsafe fn GetQuotaLogFlags(&self, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwflags),
        )
        .ok()
    }
    pub unsafe fn SetDefaultQuotaThreshold(
        &self,
        llthreshold: i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(llthreshold),
        )
        .ok()
    }
    pub unsafe fn GetDefaultQuotaThreshold(
        &self,
        pllthreshold: *mut i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pllthreshold),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultQuotaThresholdText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztext: Param0,
        cchtext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            psztext.into_param().abi(),
            ::std::mem::transmute(cchtext),
        )
        .ok()
    }
    pub unsafe fn SetDefaultQuotaLimit(&self, lllimit: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lllimit),
        )
        .ok()
    }
    pub unsafe fn GetDefaultQuotaLimit(
        &self,
        plllimit: *mut i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plllimit),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultQuotaLimitText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztext: Param0,
        cchtext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            psztext.into_param().abi(),
            ::std::mem::transmute(cchtext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddUserSid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
    >(
        &self,
        pusersid: Param0,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
    ) -> ::windows::runtime::Result<IDiskQuotaUser> {
        let mut result__: <IDiskQuotaUser as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pusersid.into_param().abi(),
            ::std::mem::transmute(fnameresolution),
            &mut result__,
        )
        .from_abi::<IDiskQuotaUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddUserName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszlogonname: Param0,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
    ) -> ::windows::runtime::Result<IDiskQuotaUser> {
        let mut result__: <IDiskQuotaUser as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            pszlogonname.into_param().abi(),
            ::std::mem::transmute(fnameresolution),
            &mut result__,
        )
        .from_abi::<IDiskQuotaUser>(result__)
    }
    pub unsafe fn DeleteUser<'a, Param0: ::windows::runtime::IntoParam<'a, IDiskQuotaUser>>(
        &self,
        puser: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            puser.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindUserSid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
    >(
        &self,
        pusersid: Param0,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
    ) -> ::windows::runtime::Result<IDiskQuotaUser> {
        let mut result__: <IDiskQuotaUser as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pusersid.into_param().abi(),
            ::std::mem::transmute(fnameresolution),
            &mut result__,
        )
        .from_abi::<IDiskQuotaUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindUserName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszlogonname: Param0,
    ) -> ::windows::runtime::Result<IDiskQuotaUser> {
        let mut result__: <IDiskQuotaUser as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            pszlogonname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDiskQuotaUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateEnumUsers(
        &self,
        rgpusersids: *mut super::super::Foundation::PSID,
        cpsids: u32,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
        ppenum: *mut ::std::option::Option<IEnumDiskQuotaUsers>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rgpusersids),
            ::std::mem::transmute(cpsids),
            ::std::mem::transmute(fnameresolution),
            ::std::mem::transmute(ppenum),
        )
        .ok()
    }
    pub unsafe fn CreateUserBatch(&self) -> ::windows::runtime::Result<IDiskQuotaUserBatch> {
        let mut result__: <IDiskQuotaUserBatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDiskQuotaUserBatch>(result__)
    }
    pub unsafe fn InvalidateSidNameCache(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GiveUserNameResolutionPriority<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDiskQuotaUser>,
    >(
        &self,
        puser: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            puser.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ShutdownNameResolution(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiskQuotaControl {
    type Vtable = IDiskQuotaControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2039002482,
        60553,
        4559,
        [156, 0, 0, 170, 0, 161, 79, 86],
    );
}
impl ::std::convert::From<IDiskQuotaControl> for ::windows::runtime::IUnknown {
    fn from(value: IDiskQuotaControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDiskQuotaControl> for ::windows::runtime::IUnknown {
    fn from(value: &IDiskQuotaControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiskQuotaControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDiskQuotaControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IDiskQuotaControl>
    for super::super::System::Com::IConnectionPointContainer
{
    fn from(value: IDiskQuotaControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IDiskQuotaControl>
    for super::super::System::Com::IConnectionPointContainer
{
    fn from(value: &IDiskQuotaControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IConnectionPointContainer>
    for IDiskQuotaControl
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Com::IConnectionPointContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Com::IConnectionPointContainer,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IConnectionPointContainer>
    for &IDiskQuotaControl
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Com::IConnectionPointContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Com::IConnectionPointContainer,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaControl_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppcp: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpath: super::super::Foundation::PWSTR,
        breadwrite: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwstate: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwstate: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        llthreshold: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pllthreshold: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psztext: super::super::Foundation::PWSTR,
        cchtext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lllimit: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plllimit: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psztext: super::super::Foundation::PWSTR,
        cchtext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pusersid: super::super::Foundation::PSID,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
        ppuser: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszlogonname: super::super::Foundation::PWSTR,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
        ppuser: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        puser: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pusersid: super::super::Foundation::PSID,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
        ppuser: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszlogonname: super::super::Foundation::PWSTR,
        ppuser: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rgpusersids: *mut super::super::Foundation::PSID,
        cpsids: u32,
        fnameresolution: DISKQUOTA_USERNAME_RESOLVE,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppbatch: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        puser: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDiskQuotaEvents(::windows::runtime::IUnknown);
impl IDiskQuotaEvents {
    pub unsafe fn OnUserNameChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDiskQuotaUser>,
    >(
        &self,
        puser: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            puser.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiskQuotaEvents {
    type Vtable = IDiskQuotaEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2039002489,
        60553,
        4559,
        [156, 0, 0, 170, 0, 161, 79, 86],
    );
}
impl ::std::convert::From<IDiskQuotaEvents> for ::windows::runtime::IUnknown {
    fn from(value: IDiskQuotaEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDiskQuotaEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IDiskQuotaEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiskQuotaEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDiskQuotaEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        puser: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDiskQuotaUser(::windows::runtime::IUnknown);
impl IDiskQuotaUser {
    pub unsafe fn GetID(&self, pulid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pulid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszaccountcontainer: Param0,
        cchaccountcontainer: u32,
        pszlogonname: Param2,
        cchlogonname: u32,
        pszdisplayname: Param4,
        cchdisplayname: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszaccountcontainer.into_param().abi(),
            ::std::mem::transmute(cchaccountcontainer),
            pszlogonname.into_param().abi(),
            ::std::mem::transmute(cchlogonname),
            pszdisplayname.into_param().abi(),
            ::std::mem::transmute(cchdisplayname),
        )
        .ok()
    }
    pub unsafe fn GetSidLength(&self, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwlength),
        )
        .ok()
    }
    pub unsafe fn GetSid(
        &self,
        pbsidbuffer: *mut u8,
        cbsidbuffer: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbsidbuffer),
            ::std::mem::transmute(cbsidbuffer),
        )
        .ok()
    }
    pub unsafe fn GetQuotaThreshold(
        &self,
        pllthreshold: *mut i64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pllthreshold),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuotaThresholdText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztext: Param0,
        cchtext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            psztext.into_param().abi(),
            ::std::mem::transmute(cchtext),
        )
        .ok()
    }
    pub unsafe fn GetQuotaLimit(&self, plllimit: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plllimit),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuotaLimitText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztext: Param0,
        cchtext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            psztext.into_param().abi(),
            ::std::mem::transmute(cchtext),
        )
        .ok()
    }
    pub unsafe fn GetQuotaUsed(&self, pllused: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pllused),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuotaUsedText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztext: Param0,
        cchtext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            psztext.into_param().abi(),
            ::std::mem::transmute(cchtext),
        )
        .ok()
    }
    pub unsafe fn GetQuotaInformation(
        &self,
        pbquotainfo: *mut ::std::ffi::c_void,
        cbquotainfo: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbquotainfo),
            ::std::mem::transmute(cbquotainfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuotaThreshold<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        llthreshold: i64,
        fwritethrough: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(llthreshold),
            fwritethrough.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuotaLimit<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        lllimit: i64,
        fwritethrough: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lllimit),
            fwritethrough.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Invalidate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetAccountStatus(&self, pdwstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwstatus),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiskQuotaUser {
    type Vtable = IDiskQuotaUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2039002484,
        60553,
        4559,
        [156, 0, 0, 170, 0, 161, 79, 86],
    );
}
impl ::std::convert::From<IDiskQuotaUser> for ::windows::runtime::IUnknown {
    fn from(value: IDiskQuotaUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDiskQuotaUser> for ::windows::runtime::IUnknown {
    fn from(value: &IDiskQuotaUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiskQuotaUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDiskQuotaUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaUser_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pulid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszaccountcontainer: super::super::Foundation::PWSTR,
        cchaccountcontainer: u32,
        pszlogonname: super::super::Foundation::PWSTR,
        cchlogonname: u32,
        pszdisplayname: super::super::Foundation::PWSTR,
        cchdisplayname: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwlength: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbsidbuffer: *mut u8,
        cbsidbuffer: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pllthreshold: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psztext: super::super::Foundation::PWSTR,
        cchtext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plllimit: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psztext: super::super::Foundation::PWSTR,
        cchtext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pllused: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psztext: super::super::Foundation::PWSTR,
        cchtext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbquotainfo: *mut ::std::ffi::c_void,
        cbquotainfo: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        llthreshold: i64,
        fwritethrough: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lllimit: i64,
        fwritethrough: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwstatus: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDiskQuotaUserBatch(::windows::runtime::IUnknown);
impl IDiskQuotaUserBatch {
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IDiskQuotaUser>>(
        &self,
        puser: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            puser.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, IDiskQuotaUser>>(
        &self,
        puser: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            puser.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn FlushToDisk(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiskQuotaUserBatch {
    type Vtable = IDiskQuotaUserBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2039002486,
        60553,
        4559,
        [156, 0, 0, 170, 0, 161, 79, 86],
    );
}
impl ::std::convert::From<IDiskQuotaUserBatch> for ::windows::runtime::IUnknown {
    fn from(value: IDiskQuotaUserBatch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDiskQuotaUserBatch> for ::windows::runtime::IUnknown {
    fn from(value: &IDiskQuotaUserBatch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiskQuotaUserBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDiskQuotaUserBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaUserBatch_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        puser: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        puser: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumDiskQuotaUsers(::windows::runtime::IUnknown);
impl IEnumDiskQuotaUsers {
    pub unsafe fn Next(
        &self,
        cusers: u32,
        rgusers: *mut ::std::option::Option<IDiskQuotaUser>,
        pcusersfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cusers),
            ::std::mem::transmute(rgusers),
            ::std::mem::transmute(pcusersfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, cusers: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cusers),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumDiskQuotaUsers> {
        let mut result__: <IEnumDiskQuotaUsers as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumDiskQuotaUsers>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumDiskQuotaUsers {
    type Vtable = IEnumDiskQuotaUsers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2039002487,
        60553,
        4559,
        [156, 0, 0, 170, 0, 161, 79, 86],
    );
}
impl ::std::convert::From<IEnumDiskQuotaUsers> for ::windows::runtime::IUnknown {
    fn from(value: IEnumDiskQuotaUsers) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumDiskQuotaUsers> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumDiskQuotaUsers) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumDiskQuotaUsers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumDiskQuotaUsers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiskQuotaUsers_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cusers: u32,
        rgusers: *mut ::windows::runtime::RawPtr,
        pcusersfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cusers: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_BUFFER_INFO {
    pub Address: *mut ::std::ffi::c_void,
    pub Length: u32,
}
impl IORING_BUFFER_INFO {}
impl ::std::default::Default for IORING_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_BUFFER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_BUFFER_INFO")
            .field("Address", &self.Address)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for IORING_BUFFER_INFO {}
unsafe impl ::windows::runtime::Abi for IORING_BUFFER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_BUFFER_REF {
    pub Kind: IORING_REF_KIND,
    pub Buffer: IORING_BUFFER_REF_0,
}
impl IORING_BUFFER_REF {}
impl ::std::default::Default for IORING_BUFFER_REF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IORING_BUFFER_REF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IORING_BUFFER_REF {}
unsafe impl ::windows::runtime::Abi for IORING_BUFFER_REF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IORING_BUFFER_REF_0 {
    pub Address: *mut ::std::ffi::c_void,
    pub IndexAndOffset: IORING_REGISTERED_BUFFER,
}
impl IORING_BUFFER_REF_0 {}
impl ::std::default::Default for IORING_BUFFER_REF_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IORING_BUFFER_REF_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IORING_BUFFER_REF_0 {}
unsafe impl ::windows::runtime::Abi for IORING_BUFFER_REF_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_CAPABILITIES {
    pub MaxVersion: IORING_VERSION,
    pub MaxSubmissionQueueSize: u32,
    pub MaxCompletionQueueSize: u32,
    pub FeatureFlags: IORING_FEATURE_FLAGS,
}
impl IORING_CAPABILITIES {}
impl ::std::default::Default for IORING_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_CAPABILITIES")
            .field("MaxVersion", &self.MaxVersion)
            .field("MaxSubmissionQueueSize", &self.MaxSubmissionQueueSize)
            .field("MaxCompletionQueueSize", &self.MaxCompletionQueueSize)
            .field("FeatureFlags", &self.FeatureFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaxVersion == other.MaxVersion
            && self.MaxSubmissionQueueSize == other.MaxSubmissionQueueSize
            && self.MaxCompletionQueueSize == other.MaxCompletionQueueSize
            && self.FeatureFlags == other.FeatureFlags
    }
}
impl ::std::cmp::Eq for IORING_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for IORING_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_CQE {
    pub UserData: usize,
    pub ResultCode: ::windows::runtime::HRESULT,
    pub Information: usize,
}
impl IORING_CQE {}
impl ::std::default::Default for IORING_CQE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_CQE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_CQE")
            .field("UserData", &self.UserData)
            .field("ResultCode", &self.ResultCode)
            .field("Information", &self.Information)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_CQE {
    fn eq(&self, other: &Self) -> bool {
        self.UserData == other.UserData
            && self.ResultCode == other.ResultCode
            && self.Information == other.Information
    }
}
impl ::std::cmp::Eq for IORING_CQE {}
unsafe impl ::windows::runtime::Abi for IORING_CQE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_CREATE_ADVISORY_FLAGS(pub i32);
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS =
    IORING_CREATE_ADVISORY_FLAGS(0i32);
impl ::std::convert::From<i32> for IORING_CREATE_ADVISORY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_CREATE_ADVISORY_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_CREATE_FLAGS {
    pub Required: IORING_CREATE_REQUIRED_FLAGS,
    pub Advisory: IORING_CREATE_ADVISORY_FLAGS,
}
impl IORING_CREATE_FLAGS {}
impl ::std::default::Default for IORING_CREATE_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_CREATE_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_CREATE_FLAGS")
            .field("Required", &self.Required)
            .field("Advisory", &self.Advisory)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_CREATE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Required == other.Required && self.Advisory == other.Advisory
    }
}
impl ::std::cmp::Eq for IORING_CREATE_FLAGS {}
unsafe impl ::windows::runtime::Abi for IORING_CREATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_CREATE_REQUIRED_FLAGS(pub i32);
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS =
    IORING_CREATE_REQUIRED_FLAGS(0i32);
impl ::std::convert::From<i32> for IORING_CREATE_REQUIRED_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_CREATE_REQUIRED_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_FEATURE_FLAGS(pub i32);
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(0i32);
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(1i32);
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(2i32);
impl ::std::convert::From<i32> for IORING_FEATURE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_FEATURE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IORING_HANDLE_REF {
    pub Kind: IORING_REF_KIND,
    pub Handle: IORING_HANDLE_REF_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IORING_HANDLE_REF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IORING_HANDLE_REF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IORING_HANDLE_REF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IORING_HANDLE_REF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IORING_HANDLE_REF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IORING_HANDLE_REF_0 {
    pub Handle: super::super::Foundation::HANDLE,
    pub Index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl IORING_HANDLE_REF_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IORING_HANDLE_REF_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IORING_HANDLE_REF_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IORING_HANDLE_REF_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IORING_HANDLE_REF_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_INFO {
    pub IoRingVersion: IORING_VERSION,
    pub Flags: IORING_CREATE_FLAGS,
    pub SubmissionQueueSize: u32,
    pub CompletionQueueSize: u32,
}
impl IORING_INFO {}
impl ::std::default::Default for IORING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_INFO")
            .field("IoRingVersion", &self.IoRingVersion)
            .field("Flags", &self.Flags)
            .field("SubmissionQueueSize", &self.SubmissionQueueSize)
            .field("CompletionQueueSize", &self.CompletionQueueSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IoRingVersion == other.IoRingVersion
            && self.Flags == other.Flags
            && self.SubmissionQueueSize == other.SubmissionQueueSize
            && self.CompletionQueueSize == other.CompletionQueueSize
    }
}
impl ::std::cmp::Eq for IORING_INFO {}
unsafe impl ::windows::runtime::Abi for IORING_INFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_OP_CODE(pub i32);
pub const IORING_OP_NOP: IORING_OP_CODE = IORING_OP_CODE(0i32);
pub const IORING_OP_READ: IORING_OP_CODE = IORING_OP_CODE(1i32);
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = IORING_OP_CODE(2i32);
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = IORING_OP_CODE(3i32);
pub const IORING_OP_CANCEL: IORING_OP_CODE = IORING_OP_CODE(4i32);
impl ::std::convert::From<i32> for IORING_OP_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_OP_CODE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_REF_KIND(pub i32);
pub const IORING_REF_RAW: IORING_REF_KIND = IORING_REF_KIND(0i32);
pub const IORING_REF_REGISTERED: IORING_REF_KIND = IORING_REF_KIND(1i32);
impl ::std::convert::From<i32> for IORING_REF_KIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_REF_KIND {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IORING_REGISTERED_BUFFER {
    pub BufferIndex: u32,
    pub Offset: u32,
}
impl IORING_REGISTERED_BUFFER {}
impl ::std::default::Default for IORING_REGISTERED_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IORING_REGISTERED_BUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IORING_REGISTERED_BUFFER")
            .field("BufferIndex", &self.BufferIndex)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IORING_REGISTERED_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferIndex == other.BufferIndex && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for IORING_REGISTERED_BUFFER {}
unsafe impl ::windows::runtime::Abi for IORING_REGISTERED_BUFFER {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_SQE_FLAGS(pub i32);
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = IORING_SQE_FLAGS(0i32);
impl ::std::convert::From<i32> for IORING_SQE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_SQE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct IORING_VERSION(pub i32);
pub const IORING_VERSION_INVALID: IORING_VERSION = IORING_VERSION(0i32);
pub const IORING_VERSION_1: IORING_VERSION = IORING_VERSION(1i32);
impl ::std::convert::From<i32> for IORING_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IORING_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallLogPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    ppolicy: *mut CLFS_MGMT_POLICY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallLogPolicy(
                hlog: super::super::Foundation::HANDLE,
                ppolicy: *mut CLFS_MGMT_POLICY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InstallLogPolicy(
            hlog.into_param().abi(),
            ::std::mem::transmute(ppolicy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsIoRingOpSupported(
    ioring: *const HIORING__,
    op: IORING_OP_CODE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsIoRingOpSupported(
                ioring: *const HIORING__,
                op: IORING_OP_CODE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsIoRingOpSupported(
            ::std::mem::transmute(ioring),
            ::std::mem::transmute(op),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
impl KCRM_MARSHAL_HEADER {}
impl ::std::default::Default for KCRM_MARSHAL_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KCRM_MARSHAL_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KCRM_MARSHAL_HEADER")
            .field("VersionMajor", &self.VersionMajor)
            .field("VersionMinor", &self.VersionMinor)
            .field("NumProtocols", &self.NumProtocols)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KCRM_MARSHAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.VersionMajor == other.VersionMajor
            && self.VersionMinor == other.VersionMinor
            && self.NumProtocols == other.NumProtocols
            && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for KCRM_MARSHAL_HEADER {}
unsafe impl ::windows::runtime::Abi for KCRM_MARSHAL_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: ::windows::runtime::GUID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
impl KCRM_PROTOCOL_BLOB {}
impl ::std::default::Default for KCRM_PROTOCOL_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KCRM_PROTOCOL_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KCRM_PROTOCOL_BLOB")
            .field("ProtocolId", &self.ProtocolId)
            .field("StaticInfoLength", &self.StaticInfoLength)
            .field("TransactionIdInfoLength", &self.TransactionIdInfoLength)
            .field("Unused1", &self.Unused1)
            .field("Unused2", &self.Unused2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KCRM_PROTOCOL_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolId == other.ProtocolId
            && self.StaticInfoLength == other.StaticInfoLength
            && self.TransactionIdInfoLength == other.TransactionIdInfoLength
            && self.Unused1 == other.Unused1
            && self.Unused2 == other.Unused2
    }
}
impl ::std::cmp::Eq for KCRM_PROTOCOL_BLOB {}
unsafe impl ::windows::runtime::Abi for KCRM_PROTOCOL_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: ::windows::runtime::GUID,
    pub TmIdentity: ::windows::runtime::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl KCRM_TRANSACTION_BLOB {}
impl ::std::default::Default for KCRM_TRANSACTION_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KCRM_TRANSACTION_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KCRM_TRANSACTION_BLOB")
            .field("UOW", &self.UOW)
            .field("TmIdentity", &self.TmIdentity)
            .field("IsolationLevel", &self.IsolationLevel)
            .field("IsolationFlags", &self.IsolationFlags)
            .field("Timeout", &self.Timeout)
            .field("Description", &self.Description)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KCRM_TRANSACTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.UOW == other.UOW
            && self.TmIdentity == other.TmIdentity
            && self.IsolationLevel == other.IsolationLevel
            && self.IsolationFlags == other.IsolationFlags
            && self.Timeout == other.Timeout
            && self.Description == other.Description
    }
}
impl ::std::cmp::Eq for KCRM_TRANSACTION_BLOB {}
unsafe impl ::windows::runtime::Abi for KCRM_TRANSACTION_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LOCK_FILE_FLAGS(pub u32);
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(2u32);
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(1u32);
impl ::std::convert::From<u32> for LOCK_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCK_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LOCK_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LOCK_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LOCK_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LOG_MANAGEMENT_CALLBACKS {
    pub CallbackContext: *mut ::std::ffi::c_void,
    pub AdvanceTailCallback: ::std::option::Option<PLOG_TAIL_ADVANCE_CALLBACK>,
    pub LogFullHandlerCallback: ::std::option::Option<PLOG_FULL_HANDLER_CALLBACK>,
    pub LogUnpinnedCallback: ::std::option::Option<PLOG_UNPINNED_CALLBACK>,
}
#[cfg(feature = "Win32_Foundation")]
impl LOG_MANAGEMENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LOG_MANAGEMENT_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LOG_MANAGEMENT_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LOG_MANAGEMENT_CALLBACKS")
            .field("CallbackContext", &self.CallbackContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LOG_MANAGEMENT_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.CallbackContext == other.CallbackContext
            && self.AdvanceTailCallback.map(|f| f as usize)
                == other.AdvanceTailCallback.map(|f| f as usize)
            && self.LogFullHandlerCallback.map(|f| f as usize)
                == other.LogFullHandlerCallback.map(|f| f as usize)
            && self.LogUnpinnedCallback.map(|f| f as usize)
                == other.LogUnpinnedCallback.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LOG_MANAGEMENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LOG_MANAGEMENT_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
pub const LOG_POLICY_PERSIST: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPPROGRESS_ROUTINE = unsafe extern "system" fn(
    totalfilesize: i64,
    totalbytestransferred: i64,
    streamsize: i64,
    streambytestransferred: i64,
    dwstreamnumber: u32,
    dwcallbackreason: LPPROGRESS_ROUTINE_CALLBACK_REASON,
    hsourcefile: super::super::Foundation::HANDLE,
    hdestinationfile: super::super::Foundation::HANDLE,
    lpdata: *const ::std::ffi::c_void,
) -> u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(pub u32);
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON =
    LPPROGRESS_ROUTINE_CALLBACK_REASON(0u32);
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON =
    LPPROGRESS_ROUTINE_CALLBACK_REASON(1u32);
impl ::std::convert::From<u32> for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn LZClose(hfile: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZClose(hfile: i32);
        }
        ::std::mem::transmute(LZClose(::std::mem::transmute(hfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LZCopy(hfsource: i32, hfdest: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZCopy(hfsource: i32, hfdest: i32) -> i32;
        }
        ::std::mem::transmute(LZCopy(
            ::std::mem::transmute(hfsource),
            ::std::mem::transmute(hfdest),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LZDone() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZDone();
        }
        ::std::mem::transmute(LZDone())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LZERROR_BADINHANDLE: i32 = -1i32;
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
pub const LZERROR_BADVALUE: i32 = -7i32;
pub const LZERROR_GLOBALLOC: i32 = -5i32;
pub const LZERROR_GLOBLOCK: i32 = -6i32;
pub const LZERROR_READ: i32 = -3i32;
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
pub const LZERROR_WRITE: i32 = -4i32;
#[inline]
pub unsafe fn LZInit(hfsource: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZInit(hfsource: i32) -> i32;
        }
        ::std::mem::transmute(LZInit(::std::mem::transmute(hfsource)))
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
pub struct LZOPENFILE_STYLE(pub u32);
pub const OF_CANCEL: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2048u32);
pub const OF_CREATE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(4096u32);
pub const OF_DELETE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(512u32);
pub const OF_EXIST: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16384u32);
pub const OF_PARSE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(256u32);
pub const OF_PROMPT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(8192u32);
pub const OF_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_READWRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2u32);
pub const OF_REOPEN: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32768u32);
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(64u32);
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(48u32);
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32u32);
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16u32);
pub const OF_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1u32);
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_VERIFY: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1024u32);
impl ::std::convert::From<u32> for LZOPENFILE_STYLE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LZOPENFILE_STYLE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LZOPENFILE_STYLE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LZOPENFILE_STYLE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LZOPENFILE_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LZOpenFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpreopenbuf: *mut OFSTRUCT,
    wstyle: LZOPENFILE_STYLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZOpenFileA(
                lpfilename: super::super::Foundation::PSTR,
                lpreopenbuf: *mut OFSTRUCT,
                wstyle: LZOPENFILE_STYLE,
            ) -> i32;
        }
        ::std::mem::transmute(LZOpenFileA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpreopenbuf),
            ::std::mem::transmute(wstyle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LZOpenFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    lpreopenbuf: *mut OFSTRUCT,
    wstyle: LZOPENFILE_STYLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZOpenFileW(
                lpfilename: super::super::Foundation::PWSTR,
                lpreopenbuf: *mut OFSTRUCT,
                wstyle: LZOPENFILE_STYLE,
            ) -> i32;
        }
        ::std::mem::transmute(LZOpenFileW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpreopenbuf),
            ::std::mem::transmute(wstyle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LZRead(hfile: i32, lpbuffer: super::super::Foundation::PSTR, cbread: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZRead(hfile: i32, lpbuffer: super::super::Foundation::PSTR, cbread: i32) -> i32;
        }
        ::std::mem::transmute(LZRead(
            ::std::mem::transmute(hfile),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(cbread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
        }
        ::std::mem::transmute(LZSeek(
            ::std::mem::transmute(hfile),
            ::std::mem::transmute(loffset),
            ::std::mem::transmute(iorigin),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LZStart() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LZStart() -> i32;
        }
        ::std::mem::transmute(LZStart())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalFileTimeToFileTime(
    lplocalfiletime: *const super::super::Foundation::FILETIME,
    lpfiletime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalFileTimeToFileTime(
                lplocalfiletime: *const super::super::Foundation::FILETIME,
                lpfiletime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LocalFileTimeToFileTime(
            ::std::mem::transmute(lplocalfiletime),
            ::std::mem::transmute(lpfiletime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    dwfileoffsetlow: u32,
    dwfileoffsethigh: u32,
    nnumberofbytestolocklow: u32,
    nnumberofbytestolockhigh: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockFile(
                hfile: super::super::Foundation::HANDLE,
                dwfileoffsetlow: u32,
                dwfileoffsethigh: u32,
                nnumberofbytestolocklow: u32,
                nnumberofbytestolockhigh: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LockFile(
            hfile.into_param().abi(),
            ::std::mem::transmute(dwfileoffsetlow),
            ::std::mem::transmute(dwfileoffsethigh),
            ::std::mem::transmute(nnumberofbytestolocklow),
            ::std::mem::transmute(nnumberofbytestolockhigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn LockFileEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    dwflags: LOCK_FILE_FLAGS,
    dwreserved: u32,
    nnumberofbytestolocklow: u32,
    nnumberofbytestolockhigh: u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockFileEx(
                hfile: super::super::Foundation::HANDLE,
                dwflags: LOCK_FILE_FLAGS,
                dwreserved: u32,
                nnumberofbytestolocklow: u32,
                nnumberofbytestolockhigh: u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LockFileEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(nnumberofbytestolocklow),
            ::std::mem::transmute(nnumberofbytestolockhigh),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogTailAdvanceFailure<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    dwreason: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogTailAdvanceFailure(
                hlog: super::super::Foundation::HANDLE,
                dwreason: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogTailAdvanceFailure(
            hlog.into_param().abi(),
            ::std::mem::transmute(dwreason),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32;
        }
        ::std::mem::transmute(LsnBlockOffset(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LsnContainer(plsn: *const CLS_LSN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnContainer(plsn: *const CLS_LSN) -> u32;
        }
        ::std::mem::transmute(LsnContainer(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN;
        }
        ::std::mem::transmute(LsnCreate(
            ::std::mem::transmute(cidcontainer),
            ::std::mem::transmute(offblock),
            ::std::mem::transmute(crecord),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnEqual(
    plsn1: *const CLS_LSN,
    plsn2: *const CLS_LSN,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnEqual(
                plsn1: *const CLS_LSN,
                plsn2: *const CLS_LSN,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(LsnEqual(
            ::std::mem::transmute(plsn1),
            ::std::mem::transmute(plsn2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnGreater(
    plsn1: *const CLS_LSN,
    plsn2: *const CLS_LSN,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnGreater(
                plsn1: *const CLS_LSN,
                plsn2: *const CLS_LSN,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(LsnGreater(
            ::std::mem::transmute(plsn1),
            ::std::mem::transmute(plsn2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN;
        }
        ::std::mem::transmute(LsnIncrement(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(LsnInvalid(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnLess(
    plsn1: *const CLS_LSN,
    plsn2: *const CLS_LSN,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnLess(
                plsn1: *const CLS_LSN,
                plsn2: *const CLS_LSN,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(LsnLess(
            ::std::mem::transmute(plsn1),
            ::std::mem::transmute(plsn2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(LsnNull(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32;
        }
        ::std::mem::transmute(LsnRecordSequence(::std::mem::transmute(plsn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type MAXMEDIALABEL = unsafe extern "system" fn(pmaxsize: *mut u32) -> u32;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MAX_SID_SIZE: u32 = 256u32;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MOVE_FILE_FLAGS(pub u32);
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(2u32);
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(16u32);
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(4u32);
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(1u32);
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(8u32);
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(32u32);
impl ::std::convert::From<u32> for MOVE_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOVE_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MOVE_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MOVE_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MOVE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MediaLabelInfo {
    pub LabelType: [u16; 64],
    pub LabelIDSize: u32,
    pub LabelID: [u8; 256],
    pub LabelAppDescr: [u16; 256],
}
impl MediaLabelInfo {}
impl ::std::default::Default for MediaLabelInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MediaLabelInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MediaLabelInfo")
            .field("LabelType", &self.LabelType)
            .field("LabelIDSize", &self.LabelIDSize)
            .field("LabelID", &self.LabelID)
            .field("LabelAppDescr", &self.LabelAppDescr)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MediaLabelInfo {
    fn eq(&self, other: &Self) -> bool {
        self.LabelType == other.LabelType
            && self.LabelIDSize == other.LabelIDSize
            && self.LabelID == other.LabelID
            && self.LabelAppDescr == other.LabelAppDescr
    }
}
impl ::std::cmp::Eq for MediaLabelInfo {}
unsafe impl ::windows::runtime::Abi for MediaLabelInfo {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    dwflags: MOVE_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileExA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                dwflags: MOVE_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileExA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    dwflags: MOVE_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileExW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                dwflags: MOVE_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileExW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileFromAppW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileFromAppW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    dwflags: MOVE_FILE_FLAGS,
    htransaction: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileTransactedA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                dwflags: MOVE_FILE_FLAGS,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileTransactedA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(dwflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    dwflags: MOVE_FILE_FLAGS,
    htransaction: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileTransactedW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                dwflags: MOVE_FILE_FLAGS,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileTransactedW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(dwflags),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileWithProgressA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    dwflags: MOVE_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileWithProgressA(
                lpexistingfilename: super::super::Foundation::PSTR,
                lpnewfilename: super::super::Foundation::PSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                dwflags: MOVE_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileWithProgressA(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileWithProgressW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpexistingfilename: Param0,
    lpnewfilename: Param1,
    lpprogressroutine: ::std::option::Option<LPPROGRESS_ROUTINE>,
    lpdata: *const ::std::ffi::c_void,
    dwflags: MOVE_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoveFileWithProgressW(
                lpexistingfilename: super::super::Foundation::PWSTR,
                lpnewfilename: super::super::Foundation::PWSTR,
                lpprogressroutine: ::windows::runtime::RawPtr,
                lpdata: *const ::std::ffi::c_void,
                dwflags: MOVE_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MoveFileWithProgressW(
            lpexistingfilename.into_param().abi(),
            lpnewfilename.into_param().abi(),
            ::std::mem::transmute(lpprogressroutine),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NAME_CACHE_CONTEXT {
    pub m_dwSignature: u32,
}
impl NAME_CACHE_CONTEXT {}
impl ::std::default::Default for NAME_CACHE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NAME_CACHE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NAME_CACHE_CONTEXT")
            .field("m_dwSignature", &self.m_dwSignature)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NAME_CACHE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwSignature == other.m_dwSignature
    }
}
impl ::std::cmp::Eq for NAME_CACHE_CONTEXT {}
unsafe impl ::windows::runtime::Abi for NAME_CACHE_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_ALLOCATION_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::std::ffi::c_void,
    pub AllocatedFrom: ::windows::runtime::GUID,
}
impl NTMS_ALLOCATION_INFORMATION {}
impl ::std::default::Default for NTMS_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_ALLOCATION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_ALLOCATION_INFORMATION")
            .field("dwSize", &self.dwSize)
            .field("lpReserved", &self.lpReserved)
            .field("AllocatedFrom", &self.AllocatedFrom)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_ALLOCATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpReserved == other.lpReserved
            && self.AllocatedFrom == other.AllocatedFrom
    }
}
impl ::std::cmp::Eq for NTMS_ALLOCATION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_ALLOCATION_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_ASYNC_IO {
    pub OperationId: ::windows::runtime::GUID,
    pub EventId: ::windows::runtime::GUID,
    pub dwOperationType: u32,
    pub dwResult: u32,
    pub dwAsyncState: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub bOnStateChange: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_ASYNC_IO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_ASYNC_IO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_ASYNC_IO")
            .field("OperationId", &self.OperationId)
            .field("EventId", &self.EventId)
            .field("dwOperationType", &self.dwOperationType)
            .field("dwResult", &self.dwResult)
            .field("dwAsyncState", &self.dwAsyncState)
            .field("hEvent", &self.hEvent)
            .field("bOnStateChange", &self.bOnStateChange)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_ASYNC_IO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationId == other.OperationId
            && self.EventId == other.EventId
            && self.dwOperationType == other.dwOperationType
            && self.dwResult == other.dwResult
            && self.dwAsyncState == other.dwAsyncState
            && self.hEvent == other.hEvent
            && self.bOnStateChange == other.bOnStateChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_ASYNC_IO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERINFORMATIONA {
    pub Number: u32,
    pub ChangerType: ::windows::runtime::GUID,
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_CHANGERINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_CHANGERINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_CHANGERINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_CHANGERINFORMATIONA")
            .field("Number", &self.Number)
            .field("ChangerType", &self.ChangerType)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("szDeviceName", &self.szDeviceName)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("Library", &self.Library)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_CHANGERINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.ChangerType == other.ChangerType
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.szDeviceName == other.szDeviceName
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.Library == other.Library
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_CHANGERINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_CHANGERINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_CHANGERINFORMATIONW {
    pub Number: u32,
    pub ChangerType: ::windows::runtime::GUID,
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub szDeviceName: [u16; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows::runtime::GUID,
}
impl NTMS_CHANGERINFORMATIONW {}
impl ::std::default::Default for NTMS_CHANGERINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_CHANGERINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_CHANGERINFORMATIONW")
            .field("Number", &self.Number)
            .field("ChangerType", &self.ChangerType)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("szDeviceName", &self.szDeviceName)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_CHANGERINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.ChangerType == other.ChangerType
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.szDeviceName == other.szDeviceName
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.Library == other.Library
    }
}
impl ::std::cmp::Eq for NTMS_CHANGERINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_CHANGERINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERTYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub DeviceType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_CHANGERTYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_CHANGERTYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_CHANGERTYPEINFORMATIONA")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.DeviceType == other.DeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_CHANGERTYPEINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_CHANGERTYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub DeviceType: u32,
}
impl NTMS_CHANGERTYPEINFORMATIONW {}
impl ::std::default::Default for NTMS_CHANGERTYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_CHANGERTYPEINFORMATIONW")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.DeviceType == other.DeviceType
    }
}
impl ::std::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_CHANGERTYPEINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_COMPUTERINFORMATION {
    pub dwLibRequestPurgeTime: u32,
    pub dwOpRequestPurgeTime: u32,
    pub dwLibRequestFlags: u32,
    pub dwOpRequestFlags: u32,
    pub dwMediaPoolPolicy: u32,
}
impl NTMS_COMPUTERINFORMATION {}
impl ::std::default::Default for NTMS_COMPUTERINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_COMPUTERINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_COMPUTERINFORMATION")
            .field("dwLibRequestPurgeTime", &self.dwLibRequestPurgeTime)
            .field("dwOpRequestPurgeTime", &self.dwOpRequestPurgeTime)
            .field("dwLibRequestFlags", &self.dwLibRequestFlags)
            .field("dwOpRequestFlags", &self.dwOpRequestFlags)
            .field("dwMediaPoolPolicy", &self.dwMediaPoolPolicy)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_COMPUTERINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLibRequestPurgeTime == other.dwLibRequestPurgeTime
            && self.dwOpRequestPurgeTime == other.dwOpRequestPurgeTime
            && self.dwLibRequestFlags == other.dwLibRequestFlags
            && self.dwOpRequestFlags == other.dwOpRequestFlags
            && self.dwMediaPoolPolicy == other.dwMediaPoolPolicy
    }
}
impl ::std::cmp::Eq for NTMS_COMPUTERINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_COMPUTERINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONA {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows::runtime::GUID,
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub Reserved: ::windows::runtime::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_DRIVEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_DRIVEINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_DRIVEINFORMATIONA")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_DRIVEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.DriveType == other.DriveType
            && self.szDeviceName == other.szDeviceName
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.dwMountCount == other.dwMountCount
            && self.LastCleanedTs == other.LastCleanedTs
            && self.SavedPartitionId == other.SavedPartitionId
            && self.Library == other.Library
            && self.Reserved == other.Reserved
            && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_DRIVEINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONW {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows::runtime::GUID,
    pub szDeviceName: [u16; 64],
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub Reserved: ::windows::runtime::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_DRIVEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_DRIVEINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_DRIVEINFORMATIONW")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_DRIVEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.DriveType == other.DriveType
            && self.szDeviceName == other.szDeviceName
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.dwMountCount == other.dwMountCount
            && self.LastCleanedTs == other.LastCleanedTs
            && self.SavedPartitionId == other.SavedPartitionId
            && self.Library == other.Library
            && self.Reserved == other.Reserved
            && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_DRIVEINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVETYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_DRIVETYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_DRIVETYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_DRIVETYPEINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_DRIVETYPEINFORMATIONA")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("NumberOfHeads", &self.NumberOfHeads)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.NumberOfHeads == other.NumberOfHeads
            && self.DeviceType == other.DeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_DRIVETYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_DRIVETYPEINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_DRIVETYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl NTMS_DRIVETYPEINFORMATIONW {}
impl ::std::default::Default for NTMS_DRIVETYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_DRIVETYPEINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_DRIVETYPEINFORMATIONW")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("NumberOfHeads", &self.NumberOfHeads)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.NumberOfHeads == other.NumberOfHeads
            && self.DeviceType == other.DeviceType
    }
}
impl ::std::cmp::Eq for NTMS_DRIVETYPEINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_DRIVETYPEINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_FILESYSTEM_INFO {
    pub FileSystemType: [u16; 64],
    pub VolumeName: [u16; 256],
    pub SerialNumber: u32,
}
impl NTMS_FILESYSTEM_INFO {}
impl ::std::default::Default for NTMS_FILESYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_FILESYSTEM_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_FILESYSTEM_INFO")
            .field("FileSystemType", &self.FileSystemType)
            .field("VolumeName", &self.VolumeName)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_FILESYSTEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType
            && self.VolumeName == other.VolumeName
            && self.SerialNumber == other.SerialNumber
    }
}
impl ::std::cmp::Eq for NTMS_FILESYSTEM_INFO {}
unsafe impl ::windows::runtime::Abi for NTMS_FILESYSTEM_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: ::windows::runtime::GUID,
    pub CleanerSlotDefault: ::windows::runtime::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_LIBRARYINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field(
                "LibrarySupportsDriveCleaning",
                &self.LibrarySupportsDriveCleaning,
            )
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_LIBRARYINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows::runtime::GUID,
    pub DriveId: ::windows::runtime::GUID,
    pub PhysMediaId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub SlotId: ::windows::runtime::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_LIBREQUESTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows::runtime::GUID,
    pub DriveId: ::windows::runtime::GUID,
    pub PhysMediaId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub SlotId: ::windows::runtime::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_LIBREQUESTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::runtime::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OBJECTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OBJECTINFORMATIONA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::runtime::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OBJECTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OBJECTINFORMATIONW_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [super::super::Foundation::CHAR; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows::runtime::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows::runtime::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_OPREQUESTINFORMATIONA")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OPREQUESTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows::runtime::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows::runtime::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_OPREQUESTINFORMATIONW")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_OPREQUESTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows::runtime::GUID,
    pub LogicalMedia: ::windows::runtime::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_PARTITIONINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_I1_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows::runtime::GUID,
    pub LogicalMedia: ::windows::runtime::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl NTMS_I1_PARTITIONINFORMATIONW {}
impl ::std::default::Default for NTMS_I1_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::std::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_I1_PARTITIONINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows::runtime::GUID,
    pub MediaPool: ::windows::runtime::GUID,
    pub Location: ::windows::runtime::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::runtime::GUID,
    pub HomeSlot: ::windows::runtime::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_I1_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_I1_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_I1_PMIDINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_I1_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_I1_PMIDINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_I1_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows::runtime::GUID,
    pub MediaPool: ::windows::runtime::GUID,
    pub Location: ::windows::runtime::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::runtime::GUID,
    pub HomeSlot: ::windows::runtime::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl NTMS_I1_PMIDINFORMATIONW {}
impl ::std::default::Default for NTMS_I1_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_I1_PMIDINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_I1_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::std::cmp::Eq for NTMS_I1_PMIDINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_I1_PMIDINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_IEDOORINFORMATION {
    pub Number: u32,
    pub State: NtmsDoorState,
    pub MaxOpenSecs: u16,
    pub Library: ::windows::runtime::GUID,
}
impl NTMS_IEDOORINFORMATION {}
impl ::std::default::Default for NTMS_IEDOORINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_IEDOORINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_IEDOORINFORMATION")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("MaxOpenSecs", &self.MaxOpenSecs)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_IEDOORINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.MaxOpenSecs == other.MaxOpenSecs
            && self.Library == other.Library
    }
}
impl ::std::cmp::Eq for NTMS_IEDOORINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_IEDOORINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_IEPORTINFORMATION {
    pub Number: u32,
    pub Content: NtmsPortContent,
    pub Position: NtmsPortPosition,
    pub MaxExtendSecs: u16,
    pub Library: ::windows::runtime::GUID,
}
impl NTMS_IEPORTINFORMATION {}
impl ::std::default::Default for NTMS_IEPORTINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_IEPORTINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_IEPORTINFORMATION")
            .field("Number", &self.Number)
            .field("Content", &self.Content)
            .field("Position", &self.Position)
            .field("MaxExtendSecs", &self.MaxExtendSecs)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_IEPORTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.Content == other.Content
            && self.Position == other.Position
            && self.MaxExtendSecs == other.MaxExtendSecs
            && self.Library == other.Library
    }
}
impl ::std::cmp::Eq for NTMS_IEPORTINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_IEPORTINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBRARYINFORMATION {
    pub LibraryType: NtmsLibraryType,
    pub CleanerSlot: ::windows::runtime::GUID,
    pub CleanerSlotDefault: ::windows::runtime::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: NtmsInventoryMethod,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows::runtime::GUID,
    pub AutoRecovery: super::super::Foundation::BOOL,
    pub dwFlags: NtmsLibraryFlags,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_LIBRARYINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field(
                "LibrarySupportsDriveCleaning",
                &self.LibrarySupportsDriveCleaning,
            )
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .field("AutoRecovery", &self.AutoRecovery)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
            && self.AutoRecovery == other.AutoRecovery
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_LIBRARYINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONA {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows::runtime::GUID,
    pub DriveId: ::windows::runtime::GUID,
    pub PhysMediaId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub SlotId: ::windows::runtime::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows::runtime::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_LIBREQUESTINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
            && self.dwErrorCode == other.dwErrorCode
            && self.WorkItemId == other.WorkItemId
            && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_LIBREQUESTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONW {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows::runtime::GUID,
    pub DriveId: ::windows::runtime::GUID,
    pub PhysMediaId: ::windows::runtime::GUID,
    pub Library: ::windows::runtime::GUID,
    pub SlotId: ::windows::runtime::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows::runtime::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_LIBREQUESTINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
            && self.dwErrorCode == other.dwErrorCode
            && self.WorkItemId == other.WorkItemId
            && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_LIBREQUESTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_LMIDINFORMATION {
    pub MediaPool: ::windows::runtime::GUID,
    pub dwNumberOfPartitions: u32,
}
impl NTMS_LMIDINFORMATION {}
impl ::std::default::Default for NTMS_LMIDINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_LMIDINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_LMIDINFORMATION")
            .field("MediaPool", &self.MediaPool)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_LMIDINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPool == other.MediaPool && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::std::cmp::Eq for NTMS_LMIDINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_LMIDINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_MEDIAPOOLINFORMATION {
    pub PoolType: u32,
    pub MediaType: ::windows::runtime::GUID,
    pub Parent: ::windows::runtime::GUID,
    pub AllocationPolicy: u32,
    pub DeallocationPolicy: u32,
    pub dwMaxAllocates: u32,
    pub dwNumberOfPhysicalMedia: u32,
    pub dwNumberOfLogicalMedia: u32,
    pub dwNumberOfMediaPools: u32,
}
impl NTMS_MEDIAPOOLINFORMATION {}
impl ::std::default::Default for NTMS_MEDIAPOOLINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_MEDIAPOOLINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_MEDIAPOOLINFORMATION")
            .field("PoolType", &self.PoolType)
            .field("MediaType", &self.MediaType)
            .field("Parent", &self.Parent)
            .field("AllocationPolicy", &self.AllocationPolicy)
            .field("DeallocationPolicy", &self.DeallocationPolicy)
            .field("dwMaxAllocates", &self.dwMaxAllocates)
            .field("dwNumberOfPhysicalMedia", &self.dwNumberOfPhysicalMedia)
            .field("dwNumberOfLogicalMedia", &self.dwNumberOfLogicalMedia)
            .field("dwNumberOfMediaPools", &self.dwNumberOfMediaPools)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_MEDIAPOOLINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolType == other.PoolType
            && self.MediaType == other.MediaType
            && self.Parent == other.Parent
            && self.AllocationPolicy == other.AllocationPolicy
            && self.DeallocationPolicy == other.DeallocationPolicy
            && self.dwMaxAllocates == other.dwMaxAllocates
            && self.dwNumberOfPhysicalMedia == other.dwNumberOfPhysicalMedia
            && self.dwNumberOfLogicalMedia == other.dwNumberOfLogicalMedia
            && self.dwNumberOfMediaPools == other.dwNumberOfMediaPools
    }
}
impl ::std::cmp::Eq for NTMS_MEDIAPOOLINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_MEDIAPOOLINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_MEDIATYPEINFORMATION {
    pub MediaType: u32,
    pub NumberOfSides: u32,
    pub ReadWriteCharacteristics: NtmsReadWriteCharacteristics,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl NTMS_MEDIATYPEINFORMATION {}
impl ::std::default::Default for NTMS_MEDIATYPEINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_MEDIATYPEINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_MEDIATYPEINFORMATION")
            .field("MediaType", &self.MediaType)
            .field("NumberOfSides", &self.NumberOfSides)
            .field("ReadWriteCharacteristics", &self.ReadWriteCharacteristics)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_MEDIATYPEINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType
            && self.NumberOfSides == other.NumberOfSides
            && self.ReadWriteCharacteristics == other.ReadWriteCharacteristics
            && self.DeviceType == other.DeviceType
    }
}
impl ::std::cmp::Eq for NTMS_MEDIATYPEINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_MEDIATYPEINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_MOUNT_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::std::ffi::c_void,
}
impl NTMS_MOUNT_INFORMATION {}
impl ::std::default::Default for NTMS_MOUNT_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_MOUNT_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_MOUNT_INFORMATION")
            .field("dwSize", &self.dwSize)
            .field("lpReserved", &self.lpReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_MOUNT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved
    }
}
impl ::std::cmp::Eq for NTMS_MOUNT_INFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_MOUNT_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_NOTIFICATIONINFORMATION {
    pub dwOperation: NtmsNotificationOperations,
    pub ObjectId: ::windows::runtime::GUID,
}
impl NTMS_NOTIFICATIONINFORMATION {}
impl ::std::default::Default for NTMS_NOTIFICATIONINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_NOTIFICATIONINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_NOTIFICATIONINFORMATION")
            .field("dwOperation", &self.dwOperation)
            .field("ObjectId", &self.ObjectId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_NOTIFICATIONINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwOperation == other.dwOperation && self.ObjectId == other.ObjectId
    }
}
impl ::std::cmp::Eq for NTMS_NOTIFICATIONINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_NOTIFICATIONINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::runtime::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OBJECTINFORMATIONA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OBJECTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONA,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OBJECTINFORMATIONA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OBJECTINFORMATIONA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::runtime::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OBJECTINFORMATIONW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OBJECTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONW,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OBJECTINFORMATIONW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OBJECTINFORMATIONW_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NTMS_OMID_TYPE(pub u32);
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = NTMS_OMID_TYPE(2u32);
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = NTMS_OMID_TYPE(1u32);
impl ::std::convert::From<u32> for NTMS_OMID_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NTMS_OMID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NTMS_OMID_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NTMS_OMID_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NTMS_OMID_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NTMS_OMID_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NTMS_OMID_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONA {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [super::super::Foundation::CHAR; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows::runtime::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows::runtime::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_OPREQUESTINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_OPREQUESTINFORMATIONA")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OPREQUESTINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONW {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [u16; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows::runtime::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows::runtime::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_OPREQUESTINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_OPREQUESTINFORMATIONW")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_OPREQUESTINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows::runtime::GUID,
    pub LogicalMedia: ::windows::runtime::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_PARTITIONINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
            && self.Capacity == other.Capacity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_PARTITIONINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows::runtime::GUID,
    pub LogicalMedia: ::windows::runtime::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl NTMS_PARTITIONINFORMATIONW {}
impl ::std::default::Default for NTMS_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_PARTITIONINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
            && self.Capacity == other.Capacity
    }
}
impl ::std::cmp::Eq for NTMS_PARTITIONINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_PARTITIONINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows::runtime::GUID,
    pub MediaPool: ::windows::runtime::GUID,
    pub Location: ::windows::runtime::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::runtime::GUID,
    pub HomeSlot: ::windows::runtime::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl NTMS_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTMS_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTMS_PMIDINFORMATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTMS_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
            && self.dwMediaTypeCode == other.dwMediaTypeCode
            && self.dwDensityCode == other.dwDensityCode
            && self.MountedPartition == other.MountedPartition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTMS_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTMS_PMIDINFORMATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows::runtime::GUID,
    pub MediaPool: ::windows::runtime::GUID,
    pub Location: ::windows::runtime::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::runtime::GUID,
    pub HomeSlot: ::windows::runtime::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows::runtime::GUID,
}
impl NTMS_PMIDINFORMATIONW {}
impl ::std::default::Default for NTMS_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_PMIDINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
            && self.dwMediaTypeCode == other.dwMediaTypeCode
            && self.dwDensityCode == other.dwDensityCode
            && self.MountedPartition == other.MountedPartition
    }
}
impl ::std::cmp::Eq for NTMS_PMIDINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for NTMS_PMIDINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NTMS_STORAGESLOTINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub Library: ::windows::runtime::GUID,
}
impl NTMS_STORAGESLOTINFORMATION {}
impl ::std::default::Default for NTMS_STORAGESLOTINFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NTMS_STORAGESLOTINFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTMS_STORAGESLOTINFORMATION")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NTMS_STORAGESLOTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.Library == other.Library
    }
}
impl ::std::cmp::Eq for NTMS_STORAGESLOTINFORMATION {}
unsafe impl ::windows::runtime::Abi for NTMS_STORAGESLOTINFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetConnectionEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    qualifier: Param1,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resume_handle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetConnectionEnum(
                servername: super::super::Foundation::PWSTR,
                qualifier: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resume_handle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetConnectionEnum(
            servername.into_param().abi(),
            qualifier.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resume_handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetFileClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    fileid: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetFileClose(servername: super::super::Foundation::PWSTR, fileid: u32) -> u32;
        }
        ::std::mem::transmute(NetFileClose(
            servername.into_param().abi(),
            ::std::mem::transmute(fileid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetFileEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    basepath: Param1,
    username: Param2,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resume_handle: *mut usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetFileEnum(
                servername: super::super::Foundation::PWSTR,
                basepath: super::super::Foundation::PWSTR,
                username: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resume_handle: *mut usize,
            ) -> u32;
        }
        ::std::mem::transmute(NetFileEnum(
            servername.into_param().abi(),
            basepath.into_param().abi(),
            username.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resume_handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetFileGetInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    fileid: u32,
    level: u32,
    bufptr: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetFileGetInfo(
                servername: super::super::Foundation::PWSTR,
                fileid: u32,
                level: u32,
                bufptr: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetFileGetInfo(
            servername.into_param().abi(),
            ::std::mem::transmute(fileid),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerAliasAdd<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    buf: *const u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerAliasAdd(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                buf: *const u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetServerAliasAdd(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(buf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerAliasDel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    buf: *const u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerAliasDel(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                buf: *const u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetServerAliasDel(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(buf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerAliasEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resumehandle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerAliasEnum(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resumehandle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetServerAliasEnum(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resumehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetSessionDel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    uncclientname: Param1,
    username: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetSessionDel(
                servername: super::super::Foundation::PWSTR,
                uncclientname: super::super::Foundation::PWSTR,
                username: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(NetSessionDel(
            servername.into_param().abi(),
            uncclientname.into_param().abi(),
            username.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetSessionEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    uncclientname: Param1,
    username: Param2,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resume_handle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetSessionEnum(
                servername: super::super::Foundation::PWSTR,
                uncclientname: super::super::Foundation::PWSTR,
                username: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resume_handle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetSessionEnum(
            servername.into_param().abi(),
            uncclientname.into_param().abi(),
            username.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resume_handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetSessionGetInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    uncclientname: Param1,
    username: Param2,
    level: u32,
    bufptr: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetSessionGetInfo(
                servername: super::super::Foundation::PWSTR,
                uncclientname: super::super::Foundation::PWSTR,
                username: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetSessionGetInfo(
            servername.into_param().abi(),
            uncclientname.into_param().abi(),
            username.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareAdd<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    buf: *const u8,
    parm_err: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareAdd(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                buf: *const u8,
                parm_err: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareAdd(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(buf),
            ::std::mem::transmute(parm_err),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareCheck<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    device: Param1,
    r#type: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareCheck(
                servername: super::super::Foundation::PWSTR,
                device: super::super::Foundation::PWSTR,
                r#type: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareCheck(
            servername.into_param().abi(),
            device.into_param().abi(),
            ::std::mem::transmute(r#type),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareDel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    netname: Param1,
    reserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareDel(
                servername: super::super::Foundation::PWSTR,
                netname: super::super::Foundation::PWSTR,
                reserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareDel(
            servername.into_param().abi(),
            netname.into_param().abi(),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareDelEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    buf: *const u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareDelEx(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                buf: *const u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareDelEx(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(buf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareDelSticky<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    netname: Param1,
    reserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareDelSticky(
                servername: super::super::Foundation::PWSTR,
                netname: super::super::Foundation::PWSTR,
                reserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareDelSticky(
            servername.into_param().abi(),
            netname.into_param().abi(),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resume_handle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareEnum(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resume_handle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareEnum(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resume_handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareEnumSticky<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    level: u32,
    bufptr: *mut *mut u8,
    prefmaxlen: u32,
    entriesread: *mut u32,
    totalentries: *mut u32,
    resume_handle: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareEnumSticky(
                servername: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
                prefmaxlen: u32,
                entriesread: *mut u32,
                totalentries: *mut u32,
                resume_handle: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareEnumSticky(
            servername.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
            ::std::mem::transmute(prefmaxlen),
            ::std::mem::transmute(entriesread),
            ::std::mem::transmute(totalentries),
            ::std::mem::transmute(resume_handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareGetInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    netname: Param1,
    level: u32,
    bufptr: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareGetInfo(
                servername: super::super::Foundation::PWSTR,
                netname: super::super::Foundation::PWSTR,
                level: u32,
                bufptr: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareGetInfo(
            servername.into_param().abi(),
            netname.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(bufptr),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetShareSetInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    netname: Param1,
    level: u32,
    buf: *const u8,
    parm_err: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetShareSetInfo(
                servername: super::super::Foundation::PWSTR,
                netname: super::super::Foundation::PWSTR,
                level: u32,
                buf: *const u8,
                parm_err: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NetShareSetInfo(
            servername.into_param().abi(),
            netname.into_param().abi(),
            ::std::mem::transmute(level),
            ::std::mem::transmute(buf),
            ::std::mem::transmute(parm_err),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NetStatisticsGet(
    servername: *const i8,
    service: *const i8,
    level: u32,
    options: u32,
    buffer: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetStatisticsGet(
                servername: *const i8,
                service: *const i8,
                level: u32,
                options: u32,
                buffer: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(NetStatisticsGet(
            ::std::mem::transmute(servername),
            ::std::mem::transmute(service),
            ::std::mem::transmute(level),
            ::std::mem::transmute(options),
            ::std::mem::transmute(buffer),
        ))
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
pub struct NtmsAccessMask(pub i32);
pub const NTMS_USE_ACCESS: NtmsAccessMask = NtmsAccessMask(1i32);
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = NtmsAccessMask(2i32);
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = NtmsAccessMask(4i32);
impl ::std::convert::From<i32> for NtmsAccessMask {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsAccessMask {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsAllocateOptions(pub i32);
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = NtmsAllocateOptions(1i32);
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = NtmsAllocateOptions(2i32);
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = NtmsAllocateOptions(4i32);
impl ::std::convert::From<i32> for NtmsAllocateOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsAllocateOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsAllocationPolicy(pub i32);
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = NtmsAllocationPolicy(1i32);
impl ::std::convert::From<i32> for NtmsAllocationPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsAllocationPolicy {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsAsyncOperations(pub i32);
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = NtmsAsyncOperations(1i32);
impl ::std::convert::From<i32> for NtmsAsyncOperations {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsAsyncOperations {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsAsyncStatus(pub i32);
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = NtmsAsyncStatus(0i32);
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = NtmsAsyncStatus(1i32);
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = NtmsAsyncStatus(2i32);
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = NtmsAsyncStatus(3i32);
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = NtmsAsyncStatus(4i32);
impl ::std::convert::From<i32> for NtmsAsyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsAsyncStatus {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsBarCodeState(pub i32);
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = NtmsBarCodeState(1i32);
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = NtmsBarCodeState(2i32);
impl ::std::convert::From<i32> for NtmsBarCodeState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsBarCodeState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsCreateNtmsMediaOptions(pub i32);
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = NtmsCreateNtmsMediaOptions(1i32);
impl ::std::convert::From<i32> for NtmsCreateNtmsMediaOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsCreateNtmsMediaOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsCreateOptions(pub i32);
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = NtmsCreateOptions(1i32);
pub const NTMS_CREATE_NEW: NtmsCreateOptions = NtmsCreateOptions(2i32);
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = NtmsCreateOptions(3i32);
impl ::std::convert::From<i32> for NtmsCreateOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsCreateOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsDeallocationPolicy(pub i32);
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = NtmsDeallocationPolicy(1i32);
impl ::std::convert::From<i32> for NtmsDeallocationPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsDeallocationPolicy {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsDismountOptions(pub i32);
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = NtmsDismountOptions(1i32);
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = NtmsDismountOptions(2i32);
impl ::std::convert::From<i32> for NtmsDismountOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsDismountOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsDoorState(pub i32);
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = NtmsDoorState(0i32);
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = NtmsDoorState(1i32);
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = NtmsDoorState(2i32);
impl ::std::convert::From<i32> for NtmsDoorState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsDoorState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsDriveState(pub i32);
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = NtmsDriveState(0i32);
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = NtmsDriveState(1i32);
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = NtmsDriveState(2i32);
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = NtmsDriveState(5i32);
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = NtmsDriveState(6i32);
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = NtmsDriveState(7i32);
impl ::std::convert::From<i32> for NtmsDriveState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsDriveState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsDriveType(pub i32);
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = NtmsDriveType(0i32);
impl ::std::convert::From<i32> for NtmsDriveType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsDriveType {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsEjectOperation(pub i32);
pub const NTMS_EJECT_START: NtmsEjectOperation = NtmsEjectOperation(0i32);
pub const NTMS_EJECT_STOP: NtmsEjectOperation = NtmsEjectOperation(1i32);
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = NtmsEjectOperation(2i32);
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = NtmsEjectOperation(3i32);
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = NtmsEjectOperation(4i32);
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = NtmsEjectOperation(5i32);
impl ::std::convert::From<i32> for NtmsEjectOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsEjectOperation {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsEnumerateOption(pub i32);
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = NtmsEnumerateOption(0i32);
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = NtmsEnumerateOption(1i32);
impl ::std::convert::From<i32> for NtmsEnumerateOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsEnumerateOption {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsInjectOperation(pub i32);
pub const NTMS_INJECT_START: NtmsInjectOperation = NtmsInjectOperation(0i32);
pub const NTMS_INJECT_STOP: NtmsInjectOperation = NtmsInjectOperation(1i32);
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = NtmsInjectOperation(2i32);
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = NtmsInjectOperation(3i32);
impl ::std::convert::From<i32> for NtmsInjectOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsInjectOperation {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsInventoryMethod(pub i32);
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = NtmsInventoryMethod(0i32);
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = NtmsInventoryMethod(1i32);
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = NtmsInventoryMethod(2i32);
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = NtmsInventoryMethod(3i32);
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = NtmsInventoryMethod(4i32);
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = NtmsInventoryMethod(5i32);
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = NtmsInventoryMethod(6i32);
impl ::std::convert::From<i32> for NtmsInventoryMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsInventoryMethod {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsLibRequestFlags(pub i32);
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(1i32);
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(2i32);
impl ::std::convert::From<i32> for NtmsLibRequestFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsLibRequestFlags {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsLibraryFlags(pub i32);
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = NtmsLibraryFlags(1i32);
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = NtmsLibraryFlags(2i32);
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = NtmsLibraryFlags(4i32);
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = NtmsLibraryFlags(8i32);
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = NtmsLibraryFlags(16i32);
impl ::std::convert::From<i32> for NtmsLibraryFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsLibraryFlags {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsLibraryType(pub i32);
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = NtmsLibraryType(0i32);
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = NtmsLibraryType(1i32);
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = NtmsLibraryType(2i32);
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = NtmsLibraryType(3i32);
impl ::std::convert::From<i32> for NtmsLibraryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsLibraryType {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsLmOperation(pub i32);
pub const NTMS_LM_REMOVE: NtmsLmOperation = NtmsLmOperation(0i32);
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = NtmsLmOperation(3i32);
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = NtmsLmOperation(4i32);
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = NtmsLmOperation(5i32);
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = NtmsLmOperation(6i32);
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = NtmsLmOperation(7i32);
pub const NTMS_LM_INVENTORY: NtmsLmOperation = NtmsLmOperation(8i32);
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = NtmsLmOperation(9i32);
pub const NTMS_LM_EJECT: NtmsLmOperation = NtmsLmOperation(10i32);
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = NtmsLmOperation(11i32);
pub const NTMS_LM_INJECT: NtmsLmOperation = NtmsLmOperation(12i32);
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = NtmsLmOperation(13i32);
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = NtmsLmOperation(14i32);
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = NtmsLmOperation(15i32);
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = NtmsLmOperation(16i32);
pub const NTMS_LM_MOUNT: NtmsLmOperation = NtmsLmOperation(17i32);
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = NtmsLmOperation(18i32);
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = NtmsLmOperation(19i32);
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = NtmsLmOperation(20i32);
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = NtmsLmOperation(21i32);
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = NtmsLmOperation(22i32);
impl ::std::convert::From<i32> for NtmsLmOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsLmOperation {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsLmState(pub i32);
pub const NTMS_LM_QUEUED: NtmsLmState = NtmsLmState(0i32);
pub const NTMS_LM_INPROCESS: NtmsLmState = NtmsLmState(1i32);
pub const NTMS_LM_PASSED: NtmsLmState = NtmsLmState(2i32);
pub const NTMS_LM_FAILED: NtmsLmState = NtmsLmState(3i32);
pub const NTMS_LM_INVALID: NtmsLmState = NtmsLmState(4i32);
pub const NTMS_LM_WAITING: NtmsLmState = NtmsLmState(5i32);
pub const NTMS_LM_DEFERRED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_DEFFERED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_CANCELLED: NtmsLmState = NtmsLmState(7i32);
pub const NTMS_LM_STOPPED: NtmsLmState = NtmsLmState(8i32);
impl ::std::convert::From<i32> for NtmsLmState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsLmState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsMediaPoolPolicy(pub i32);
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(1i32);
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(2i32);
impl ::std::convert::From<i32> for NtmsMediaPoolPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsMediaPoolPolicy {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsMediaState(pub i32);
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = NtmsMediaState(0i32);
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = NtmsMediaState(1i32);
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = NtmsMediaState(2i32);
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = NtmsMediaState(3i32);
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = NtmsMediaState(4i32);
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = NtmsMediaState(5i32);
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = NtmsMediaState(6i32);
impl ::std::convert::From<i32> for NtmsMediaState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsMediaState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsMountOptions(pub i32);
pub const NTMS_MOUNT_READ: NtmsMountOptions = NtmsMountOptions(1i32);
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = NtmsMountOptions(2i32);
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = NtmsMountOptions(16i32);
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = NtmsMountOptions(32i32);
impl ::std::convert::From<i32> for NtmsMountOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsMountOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsMountPriority(pub i32);
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = NtmsMountPriority(15i32);
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = NtmsMountPriority(7i32);
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = NtmsMountPriority(-7i32);
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = NtmsMountPriority(-15i32);
impl ::std::convert::From<i32> for NtmsMountPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsMountPriority {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsNotificationOperations(pub i32);
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = NtmsNotificationOperations(1i32);
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = NtmsNotificationOperations(2i32);
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = NtmsNotificationOperations(3i32);
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = NtmsNotificationOperations(4i32);
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = NtmsNotificationOperations(5i32);
impl ::std::convert::From<i32> for NtmsNotificationOperations {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsNotificationOperations {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsObjectsTypes(pub i32);
pub const NTMS_UNKNOWN: NtmsObjectsTypes = NtmsObjectsTypes(0i32);
pub const NTMS_OBJECT: NtmsObjectsTypes = NtmsObjectsTypes(1i32);
pub const NTMS_CHANGER: NtmsObjectsTypes = NtmsObjectsTypes(2i32);
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(3i32);
pub const NTMS_COMPUTER: NtmsObjectsTypes = NtmsObjectsTypes(4i32);
pub const NTMS_DRIVE: NtmsObjectsTypes = NtmsObjectsTypes(5i32);
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(6i32);
pub const NTMS_IEDOOR: NtmsObjectsTypes = NtmsObjectsTypes(7i32);
pub const NTMS_IEPORT: NtmsObjectsTypes = NtmsObjectsTypes(8i32);
pub const NTMS_LIBRARY: NtmsObjectsTypes = NtmsObjectsTypes(9i32);
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(10i32);
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(11i32);
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = NtmsObjectsTypes(12i32);
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(13i32);
pub const NTMS_PARTITION: NtmsObjectsTypes = NtmsObjectsTypes(14i32);
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(15i32);
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = NtmsObjectsTypes(16i32);
pub const NTMS_OPREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(17i32);
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = NtmsObjectsTypes(18i32);
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = NtmsObjectsTypes(19i32);
impl ::std::convert::From<i32> for NtmsObjectsTypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsObjectsTypes {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsOpRequestFlags(pub i32);
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(1i32);
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(2i32);
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = NtmsOpRequestFlags(16i32);
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = NtmsOpRequestFlags(32i32);
impl ::std::convert::From<i32> for NtmsOpRequestFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsOpRequestFlags {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsOperationalState(pub i32);
pub const NTMS_READY: NtmsOperationalState = NtmsOperationalState(0i32);
pub const NTMS_INITIALIZING: NtmsOperationalState = NtmsOperationalState(10i32);
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = NtmsOperationalState(20i32);
pub const NTMS_NOT_PRESENT: NtmsOperationalState = NtmsOperationalState(21i32);
impl ::std::convert::From<i32> for NtmsOperationalState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsOperationalState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsOpreqCommand(pub i32);
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = NtmsOpreqCommand(0i32);
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(1i32);
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = NtmsOpreqCommand(2i32);
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = NtmsOpreqCommand(3i32);
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(4i32);
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = NtmsOpreqCommand(5i32);
impl ::std::convert::From<i32> for NtmsOpreqCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsOpreqCommand {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsOpreqState(pub i32);
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = NtmsOpreqState(0i32);
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = NtmsOpreqState(1i32);
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = NtmsOpreqState(2i32);
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = NtmsOpreqState(3i32);
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = NtmsOpreqState(4i32);
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = NtmsOpreqState(5i32);
impl ::std::convert::From<i32> for NtmsOpreqState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsOpreqState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsPartitionState(pub i32);
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = NtmsPartitionState(0i32);
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = NtmsPartitionState(1i32);
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = NtmsPartitionState(2i32);
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = NtmsPartitionState(3i32);
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = NtmsPartitionState(4i32);
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = NtmsPartitionState(5i32);
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = NtmsPartitionState(6i32);
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = NtmsPartitionState(7i32);
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = NtmsPartitionState(8i32);
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = NtmsPartitionState(9i32);
impl ::std::convert::From<i32> for NtmsPartitionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsPartitionState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsPoolType(pub i32);
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = NtmsPoolType(0i32);
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = NtmsPoolType(1i32);
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = NtmsPoolType(2i32);
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = NtmsPoolType(3i32);
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = NtmsPoolType(1000i32);
impl ::std::convert::From<i32> for NtmsPoolType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsPoolType {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsPortContent(pub i32);
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = NtmsPortContent(0i32);
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = NtmsPortContent(1i32);
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = NtmsPortContent(2i32);
impl ::std::convert::From<i32> for NtmsPortContent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsPortContent {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsPortPosition(pub i32);
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = NtmsPortPosition(0i32);
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = NtmsPortPosition(1i32);
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = NtmsPortPosition(2i32);
impl ::std::convert::From<i32> for NtmsPortPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsPortPosition {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsReadWriteCharacteristics(pub i32);
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(0i32);
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics =
    NtmsReadWriteCharacteristics(1i32);
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(2i32);
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(3i32);
impl ::std::convert::From<i32> for NtmsReadWriteCharacteristics {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsReadWriteCharacteristics {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsSessionOptions(pub i32);
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = NtmsSessionOptions(1i32);
impl ::std::convert::From<i32> for NtmsSessionOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsSessionOptions {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsSlotState(pub i32);
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = NtmsSlotState(0i32);
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = NtmsSlotState(1i32);
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = NtmsSlotState(2i32);
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = NtmsSlotState(3i32);
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = NtmsSlotState(4i32);
impl ::std::convert::From<i32> for NtmsSlotState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsSlotState {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsUIOperations(pub i32);
pub const NTMS_UIDEST_ADD: NtmsUIOperations = NtmsUIOperations(1i32);
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = NtmsUIOperations(2i32);
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = NtmsUIOperations(3i32);
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = NtmsUIOperations(4i32);
impl ::std::convert::From<i32> for NtmsUIOperations {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsUIOperations {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct NtmsUITypes(pub i32);
pub const NTMS_UITYPE_INVALID: NtmsUITypes = NtmsUITypes(0i32);
pub const NTMS_UITYPE_INFO: NtmsUITypes = NtmsUITypes(1i32);
pub const NTMS_UITYPE_REQ: NtmsUITypes = NtmsUITypes(2i32);
pub const NTMS_UITYPE_ERR: NtmsUITypes = NtmsUITypes(3i32);
pub const NTMS_UITYPE_MAX: NtmsUITypes = NtmsUITypes(4i32);
impl ::std::convert::From<i32> for NtmsUITypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NtmsUITypes {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl OFSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFSTRUCT")
            .field("cBytes", &self.cBytes)
            .field("fFixedDisk", &self.fFixedDisk)
            .field("nErrCode", &self.nErrCode)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("szPathName", &self.szPathName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes
            && self.fFixedDisk == other.fFixedDisk
            && self.nErrCode == other.nErrCode
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.szPathName == other.szPathName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFSTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEncryptedFileRawA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    ulflags: u32,
    pvcontext: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEncryptedFileRawA(
                lpfilename: super::super::Foundation::PSTR,
                ulflags: u32,
                pvcontext: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(OpenEncryptedFileRawA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(ulflags),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEncryptedFileRawW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    ulflags: u32,
    pvcontext: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEncryptedFileRawW(
                lpfilename: super::super::Foundation::PWSTR,
                ulflags: u32,
                pvcontext: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(OpenEncryptedFileRawW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(ulflags),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEnlistment<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwdesiredaccess: u32,
    resourcemanagerhandle: Param1,
    enlistmentid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEnlistment(
                dwdesiredaccess: u32,
                resourcemanagerhandle: super::super::Foundation::HANDLE,
                enlistmentid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenEnlistment(
            ::std::mem::transmute(dwdesiredaccess),
            resourcemanagerhandle.into_param().abi(),
            ::std::mem::transmute(enlistmentid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    lpreopenbuff: *mut OFSTRUCT,
    ustyle: LZOPENFILE_STYLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFile(
                lpfilename: super::super::Foundation::PSTR,
                lpreopenbuff: *mut OFSTRUCT,
                ustyle: LZOPENFILE_STYLE,
            ) -> i32;
        }
        ::std::mem::transmute(OpenFile(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(lpreopenbuff),
            ::std::mem::transmute(ustyle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenFileById<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hvolumehint: Param0,
    lpfileid: *const FILE_ID_DESCRIPTOR,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileById(
                hvolumehint: super::super::Foundation::HANDLE,
                lpfileid: *const FILE_ID_DESCRIPTOR,
                dwdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenFileById(
            hvolumehint.into_param().abi(),
            ::std::mem::transmute(lpfileid),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwflagsandattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenResourceManager<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwdesiredaccess: u32,
    tmhandle: Param1,
    resourcemanagerid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenResourceManager(
                dwdesiredaccess: u32,
                tmhandle: super::super::Foundation::HANDLE,
                resourcemanagerid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenResourceManager(
            ::std::mem::transmute(dwdesiredaccess),
            tmhandle.into_param().abi(),
            ::std::mem::transmute(resourcemanagerid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransaction(
    dwdesiredaccess: u32,
    transactionid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTransaction(
                dwdesiredaccess: u32,
                transactionid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenTransaction(
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(transactionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransactionManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    logfilename: Param0,
    desiredaccess: u32,
    openoptions: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTransactionManager(
                logfilename: super::super::Foundation::PWSTR,
                desiredaccess: u32,
                openoptions: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenTransactionManager(
            logfilename.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(openoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransactionManagerById(
    transactionmanagerid: *const ::windows::runtime::GUID,
    desiredaccess: u32,
    openoptions: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTransactionManagerById(
                transactionmanagerid: *const ::windows::runtime::GUID,
                desiredaccess: u32,
                openoptions: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenTransactionManagerById(
            ::std::mem::transmute(transactionmanagerid),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(openoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const PARTITION_BASIC_DATA_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3956318370,
        47589,
        17459,
        [135, 192, 104, 182, 183, 38, 153, 199],
    );
pub const PARTITION_BSP_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1464029011,
    19961,
    17849,
    [142, 158, 35, 112, 240, 6, 69, 124],
);
pub const PARTITION_CLUSTER_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3684162473,
    2112,
    19374,
    [151, 240, 255, 185, 163, 39, 199, 225],
);
pub const PARTITION_DPP_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1464029011,
    38091,
    17392,
    [165, 51, 215, 60, 16, 207, 165, 125],
);
pub const PARTITION_ENTRY_UNUSED_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const PARTITION_LDM_DATA_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2946195616,
    5169,
    20322,
    [188, 104, 51, 17, 113, 74, 105, 173],
);
pub const PARTITION_LDM_METADATA_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1476970666,
        32399,
        17120,
        [133, 210, 225, 233, 4, 52, 207, 179],
    );
pub const PARTITION_LEGACY_BL_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1112318178,
        31922,
        20409,
        [129, 67, 197, 42, 153, 57, 139, 198],
    );
pub const PARTITION_LEGACY_BL_GUID_BACKUP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1112292972,
        55199,
        18891,
        [147, 93, 54, 215, 20, 103, 162, 136],
    );
pub const PARTITION_MAIN_OS_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1464029011,
    36677,
    16478,
    [138, 35, 24, 109, 138, 67, 48, 211],
);
pub const PARTITION_MSFT_RECOVERY_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3734289316,
        1745,
        19776,
        [161, 106, 191, 213, 1, 121, 214, 172],
    );
pub const PARTITION_MSFT_RESERVED_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3821658902,
        2908,
        19896,
        [129, 125, 249, 45, 240, 2, 21, 174],
    );
pub const PARTITION_MSFT_SNAPSHOT_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3403541489,
        17408,
        19944,
        [177, 3, 18, 17, 125, 207, 60, 207],
    );
pub const PARTITION_OS_DATA_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1464029011,
    9202,
    17621,
    [168, 48, 103, 187, 218, 166, 9, 249],
);
pub const PARTITION_PATCH_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2305271430,
    38570,
    27304,
    [149, 137, 168, 66, 86, 84, 16, 144],
);
pub const PARTITION_PRE_INSTALLED_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        32736,
        16790,
        [155, 66, 66, 123, 81, 100, 52, 132],
    );
pub const PARTITION_SERVICING_FILES_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        17198,
        16404,
        [174, 76, 141, 234, 169, 192, 0, 106],
    );
pub const PARTITION_SERVICING_METADATA_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        50833,
        18949,
        [187, 78, 112, 61, 175, 210, 41, 206],
    );
pub const PARTITION_SERVICING_RESERVE_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        19329,
        17931,
        [163, 25, 255, 182, 254, 19, 109, 20],
    );
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        59469,
        20100,
        [170, 243, 236, 187, 189, 4, 185, 223],
    );
pub const PARTITION_SPACES_DATA_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3886931124,
        56372,
        17721,
        [154, 118, 235, 189, 7, 190, 111, 126],
    );
pub const PARTITION_SPACES_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3881611151,
    63104,
    19694,
    [175, 163, 176, 1, 229, 110, 252, 45],
);
pub const PARTITION_SYSTEM_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3240784680,
    63519,
    4562,
    [186, 75, 0, 160, 201, 62, 201, 59],
);
pub const PARTITION_WINDOWS_SYSTEM_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1464029011,
        58339,
        17969,
        [165, 197, 38, 210, 36, 56, 115, 170],
    );
pub type PCLFS_COMPLETION_ROUTINE =
    unsafe extern "system" fn(pvoverlapped: *mut ::std::ffi::c_void, ulreserved: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PCOPYFILE2_PROGRESS_ROUTINE = unsafe extern "system" fn(
    pmessage: *const COPYFILE2_MESSAGE,
    pvcallbackcontext: *const ::std::ffi::c_void,
) -> COPYFILE2_MESSAGE_ACTION;
pub type PFE_EXPORT_FUNC = unsafe extern "system" fn(
    pbdata: *const u8,
    pvcallbackcontext: *const ::std::ffi::c_void,
    ullength: u32,
) -> u32;
pub type PFE_IMPORT_FUNC = unsafe extern "system" fn(
    pbdata: *mut u8,
    pvcallbackcontext: *const ::std::ffi::c_void,
    ullength: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_IO_COMPLETION = unsafe extern "system" fn(
    pcontext: *mut FIO_CONTEXT,
    lpo: *mut ::std::mem::ManuallyDrop<FH_OVERLAPPED>,
    cb: u32,
    dwcompletionstatus: u32,
);
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_FULL_HANDLER_CALLBACK = unsafe extern "system" fn(
    hlogfile: super::super::Foundation::HANDLE,
    dwerror: u32,
    flogispinned: super::super::Foundation::BOOL,
    pvclientcontext: *mut ::std::ffi::c_void,
);
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_TAIL_ADVANCE_CALLBACK = unsafe extern "system" fn(
    hlogfile: super::super::Foundation::HANDLE,
    lsntarget: CLS_LSN,
    pvclientcontext: *mut ::std::ffi::c_void,
);
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_UNPINNED_CALLBACK = unsafe extern "system" fn(
    hlogfile: super::super::Foundation::HANDLE,
    pvclientcontext: *mut ::std::ffi::c_void,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PREPARE_TAPE_OPERATION(pub i32);
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(5i32);
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(0i32);
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(3i32);
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(2i32);
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(1i32);
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(4i32);
impl ::std::convert::From<i32> for PREPARE_TAPE_OPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PREPARE_TAPE_OPERATION {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct PRIORITY_HINT(pub i32);
pub const IoPriorityHintVeryLow: PRIORITY_HINT = PRIORITY_HINT(0i32);
pub const IoPriorityHintLow: PRIORITY_HINT = PRIORITY_HINT(1i32);
pub const IoPriorityHintNormal: PRIORITY_HINT = PRIORITY_HINT(2i32);
pub const MaximumIoPriorityHintType: PRIORITY_HINT = PRIORITY_HINT(3i32);
impl ::std::convert::From<i32> for PRIORITY_HINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRIORITY_HINT {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn PopIoRingCompletion(
    ioring: *const HIORING__,
) -> ::windows::runtime::Result<IORING_CQE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PopIoRingCompletion(
                ioring: *const HIORING__,
                cqe: *mut IORING_CQE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IORING_CQE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PopIoRingCompletion(::std::mem::transmute(ioring), &mut result__)
            .from_abi::<IORING_CQE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrePrepareComplete<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrePrepareComplete(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrePrepareComplete(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrePrepareEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrePrepareEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrePrepareEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareComplete<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrepareComplete(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrepareComplete(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrepareEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrepareEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareLogArchive<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hlog: Param0,
    pszbaselogfilename: Param1,
    clen: u32,
    plsnlow: *const CLS_LSN,
    plsnhigh: *const CLS_LSN,
    pcactuallength: *mut u32,
    poffbaselogfiledata: *mut u64,
    pcbbaselogfilelength: *mut u64,
    plsnbase: *mut CLS_LSN,
    plsnlast: *mut CLS_LSN,
    plsncurrentarchivetail: *mut CLS_LSN,
    ppvarchivecontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrepareLogArchive(
                hlog: super::super::Foundation::HANDLE,
                pszbaselogfilename: super::super::Foundation::PWSTR,
                clen: u32,
                plsnlow: *const CLS_LSN,
                plsnhigh: *const CLS_LSN,
                pcactuallength: *mut u32,
                poffbaselogfiledata: *mut u64,
                pcbbaselogfilelength: *mut u64,
                plsnbase: *mut CLS_LSN,
                plsnlast: *mut CLS_LSN,
                plsncurrentarchivetail: *mut CLS_LSN,
                ppvarchivecontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrepareLogArchive(
            hlog.into_param().abi(),
            pszbaselogfilename.into_param().abi(),
            ::std::mem::transmute(clen),
            ::std::mem::transmute(plsnlow),
            ::std::mem::transmute(plsnhigh),
            ::std::mem::transmute(pcactuallength),
            ::std::mem::transmute(poffbaselogfiledata),
            ::std::mem::transmute(pcbbaselogfilelength),
            ::std::mem::transmute(plsnbase),
            ::std::mem::transmute(plsnlast),
            ::std::mem::transmute(plsncurrentarchivetail),
            ::std::mem::transmute(ppvarchivecontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareTape<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdevice: Param0,
    dwoperation: PREPARE_TAPE_OPERATION,
    bimmediate: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrepareTape(
                hdevice: super::super::Foundation::HANDLE,
                dwoperation: PREPARE_TAPE_OPERATION,
                bimmediate: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(PrepareTape(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwoperation),
            bimmediate.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDosDeviceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpdevicename: Param0,
    lptargetpath: super::super::Foundation::PSTR,
    ucchmax: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDosDeviceA(
                lpdevicename: super::super::Foundation::PSTR,
                lptargetpath: super::super::Foundation::PSTR,
                ucchmax: u32,
            ) -> u32;
        }
        ::std::mem::transmute(QueryDosDeviceA(
            lpdevicename.into_param().abi(),
            ::std::mem::transmute(lptargetpath),
            ::std::mem::transmute(ucchmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDosDeviceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpdevicename: Param0,
    lptargetpath: super::super::Foundation::PWSTR,
    ucchmax: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDosDeviceW(
                lpdevicename: super::super::Foundation::PWSTR,
                lptargetpath: super::super::Foundation::PWSTR,
                ucchmax: u32,
            ) -> u32;
        }
        ::std::mem::transmute(QueryDosDeviceW(
            lpdevicename.into_param().abi(),
            ::std::mem::transmute(lptargetpath),
            ::std::mem::transmute(ucchmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn QueryIoRingCapabilities() -> ::windows::runtime::Result<IORING_CAPABILITIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIoRingCapabilities(
                capabilities: *mut IORING_CAPABILITIES,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IORING_CAPABILITIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        QueryIoRingCapabilities(&mut result__).from_abi::<IORING_CAPABILITIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryLogPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    epolicytype: CLFS_MGMT_POLICY_TYPE,
    ppolicybuffer: *mut CLFS_MGMT_POLICY,
    pcbpolicybuffer: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryLogPolicy(
                hlog: super::super::Foundation::HANDLE,
                epolicytype: CLFS_MGMT_POLICY_TYPE,
                ppolicybuffer: *mut CLFS_MGMT_POLICY,
                pcbpolicybuffer: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryLogPolicy(
            hlog.into_param().abi(),
            ::std::mem::transmute(epolicytype),
            ::std::mem::transmute(ppolicybuffer),
            ::std::mem::transmute(pcbpolicybuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryRecoveryAgentsOnEncryptedFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryRecoveryAgentsOnEncryptedFile(
                lpfilename: super::super::Foundation::PWSTR,
                precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
            ) -> u32;
        }
        ::std::mem::transmute(QueryRecoveryAgentsOnEncryptedFile(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(precoveryagents),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryUsersOnEncryptedFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUsersOnEncryptedFile(
                lpfilename: super::super::Foundation::PWSTR,
                pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
            ) -> u32;
        }
        ::std::mem::transmute(QueryUsersOnEncryptedFile(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(pusers),
        ))
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
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(pub i32);
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS =
    READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(1i32);
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS =
    READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: ::windows::runtime::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl REPARSE_GUID_DATA_BUFFER {}
impl ::std::default::Default for REPARSE_GUID_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for REPARSE_GUID_DATA_BUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REPARSE_GUID_DATA_BUFFER")
            .field("ReparseTag", &self.ReparseTag)
            .field("ReparseDataLength", &self.ReparseDataLength)
            .field("Reserved", &self.Reserved)
            .field("ReparseGuid", &self.ReparseGuid)
            .field("GenericReparseBuffer", &self.GenericReparseBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ReparseTag == other.ReparseTag
            && self.ReparseDataLength == other.ReparseDataLength
            && self.Reserved == other.Reserved
            && self.ReparseGuid == other.ReparseGuid
            && self.GenericReparseBuffer == other.GenericReparseBuffer
    }
}
impl ::std::cmp::Eq for REPARSE_GUID_DATA_BUFFER {}
unsafe impl ::windows::runtime::Abi for REPARSE_GUID_DATA_BUFFER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl REPARSE_GUID_DATA_BUFFER_0 {}
impl ::std::default::Default for REPARSE_GUID_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for REPARSE_GUID_DATA_BUFFER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_GenericReparseBuffer_e__Struct")
            .field("DataBuffer", &self.DataBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataBuffer == other.DataBuffer
    }
}
impl ::std::cmp::Eq for REPARSE_GUID_DATA_BUFFER_0 {}
unsafe impl ::windows::runtime::Abi for REPARSE_GUID_DATA_BUFFER_0 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct REPLACE_FILE_FLAGS(pub u32);
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(1u32);
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(2u32);
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(4u32);
impl ::std::convert::From<u32> for REPLACE_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REPLACE_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REPLACE_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REPLACE_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReOpenFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    horiginalfile: Param0,
    dwdesiredaccess: FILE_ACCESS_FLAGS,
    dwsharemode: FILE_SHARE_MODE,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReOpenFile(
                horiginalfile: super::super::Foundation::HANDLE,
                dwdesiredaccess: FILE_ACCESS_FLAGS,
                dwsharemode: FILE_SHARE_MODE,
                dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(ReOpenFile(
            horiginalfile.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwflagsandattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadDirectoryChangesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdirectory: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nbufferlength: u32,
    bwatchsubtree: Param3,
    dwnotifyfilter: FILE_NOTIFY_CHANGE,
    lpbytesreturned: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<
        super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    >,
    readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadDirectoryChangesExW(
                hdirectory: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nbufferlength: u32,
                bwatchsubtree: super::super::Foundation::BOOL,
                dwnotifyfilter: FILE_NOTIFY_CHANGE,
                lpbytesreturned: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
                lpcompletionroutine: ::windows::runtime::RawPtr,
                readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadDirectoryChangesExW(
            hdirectory.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nbufferlength),
            bwatchsubtree.into_param().abi(),
            ::std::mem::transmute(dwnotifyfilter),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
            ::std::mem::transmute(readdirectorynotifyinformationclass),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadDirectoryChangesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdirectory: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nbufferlength: u32,
    bwatchsubtree: Param3,
    dwnotifyfilter: FILE_NOTIFY_CHANGE,
    lpbytesreturned: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<
        super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    >,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadDirectoryChangesW(
                hdirectory: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nbufferlength: u32,
                bwatchsubtree: super::super::Foundation::BOOL,
                dwnotifyfilter: FILE_NOTIFY_CHANGE,
                lpbytesreturned: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
                lpcompletionroutine: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadDirectoryChangesW(
            hdirectory.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nbufferlength),
            bwatchsubtree.into_param().abi(),
            ::std::mem::transmute(dwnotifyfilter),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadEncryptedFileRaw(
    pfexportcallback: ::std::option::Option<PFE_EXPORT_FUNC>,
    pvcallbackcontext: *const ::std::ffi::c_void,
    pvcontext: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadEncryptedFileRaw(
                pfexportcallback: ::windows::runtime::RawPtr,
                pvcallbackcontext: *const ::std::ffi::c_void,
                pvcontext: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(ReadEncryptedFileRaw(
            ::std::mem::transmute(pfexportcallback),
            ::std::mem::transmute(pvcallbackcontext),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nnumberofbytestoread: u32,
    lpnumberofbytesread: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadFile(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nnumberofbytestoread: u32,
                lpnumberofbytesread: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadFile(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestoread),
            ::std::mem::transmute(lpnumberofbytesread),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFileEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nnumberofbytestoread: u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<
        super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    >,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadFileEx(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nnumberofbytestoread: u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
                lpcompletionroutine: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadFileEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestoread),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFileScatter<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    asegmentarray: *const FILE_SEGMENT_ELEMENT,
    nnumberofbytestoread: u32,
    lpreserved: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadFileScatter(
                hfile: super::super::Foundation::HANDLE,
                asegmentarray: *const FILE_SEGMENT_ELEMENT,
                nnumberofbytestoread: u32,
                lpreserved: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadFileScatter(
            hfile.into_param().abi(),
            ::std::mem::transmute(asegmentarray),
            ::std::mem::transmute(nnumberofbytestoread),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadLogArchiveMetadata(
    pvarchivecontext: *mut ::std::ffi::c_void,
    cboffset: u32,
    cbbytestoread: u32,
    pbreadbuffer: *mut u8,
    pcbbytesread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadLogArchiveMetadata(
                pvarchivecontext: *mut ::std::ffi::c_void,
                cboffset: u32,
                cbbytestoread: u32,
                pbreadbuffer: *mut u8,
                pcbbytesread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadLogArchiveMetadata(
            ::std::mem::transmute(pvarchivecontext),
            ::std::mem::transmute(cboffset),
            ::std::mem::transmute(cbbytestoread),
            ::std::mem::transmute(pbreadbuffer),
            ::std::mem::transmute(pcbbytesread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pnotification: *mut CLFS_MGMT_NOTIFICATION,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadLogNotification(
                hlog: super::super::Foundation::HANDLE,
                pnotification: *mut CLFS_MGMT_NOTIFICATION,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadLogNotification(
            hlog.into_param().abi(),
            ::std::mem::transmute(pnotification),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogRecord(
    pvmarshal: *mut ::std::ffi::c_void,
    plsnfirst: *mut CLS_LSN,
    econtextmode: CLFS_CONTEXT_MODE,
    ppvreadbuffer: *mut *mut ::std::ffi::c_void,
    pcbreadbuffer: *mut u32,
    perecordtype: *mut u8,
    plsnundonext: *mut CLS_LSN,
    plsnprevious: *mut CLS_LSN,
    ppvreadcontext: *mut *mut ::std::ffi::c_void,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadLogRecord(
                pvmarshal: *mut ::std::ffi::c_void,
                plsnfirst: *mut CLS_LSN,
                econtextmode: CLFS_CONTEXT_MODE,
                ppvreadbuffer: *mut *mut ::std::ffi::c_void,
                pcbreadbuffer: *mut u32,
                perecordtype: *mut u8,
                plsnundonext: *mut CLS_LSN,
                plsnprevious: *mut CLS_LSN,
                ppvreadcontext: *mut *mut ::std::ffi::c_void,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadLogRecord(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(plsnfirst),
            ::std::mem::transmute(econtextmode),
            ::std::mem::transmute(ppvreadbuffer),
            ::std::mem::transmute(pcbreadbuffer),
            ::std::mem::transmute(perecordtype),
            ::std::mem::transmute(plsnundonext),
            ::std::mem::transmute(plsnprevious),
            ::std::mem::transmute(ppvreadcontext),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogRestartArea(
    pvmarshal: *mut ::std::ffi::c_void,
    ppvrestartbuffer: *mut *mut ::std::ffi::c_void,
    pcbrestartbuffer: *mut u32,
    plsn: *mut CLS_LSN,
    ppvcontext: *mut *mut ::std::ffi::c_void,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadLogRestartArea(
                pvmarshal: *mut ::std::ffi::c_void,
                ppvrestartbuffer: *mut *mut ::std::ffi::c_void,
                pcbrestartbuffer: *mut u32,
                plsn: *mut CLS_LSN,
                ppvcontext: *mut *mut ::std::ffi::c_void,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadLogRestartArea(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(ppvrestartbuffer),
            ::std::mem::transmute(pcbrestartbuffer),
            ::std::mem::transmute(plsn),
            ::std::mem::transmute(ppvcontext),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadNextLogRecord(
    pvreadcontext: *mut ::std::ffi::c_void,
    ppvbuffer: *mut *mut ::std::ffi::c_void,
    pcbbuffer: *mut u32,
    perecordtype: *mut u8,
    plsnuser: *mut CLS_LSN,
    plsnundonext: *mut CLS_LSN,
    plsnprevious: *mut CLS_LSN,
    plsnrecord: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadNextLogRecord(
                pvreadcontext: *mut ::std::ffi::c_void,
                ppvbuffer: *mut *mut ::std::ffi::c_void,
                pcbbuffer: *mut u32,
                perecordtype: *mut u8,
                plsnuser: *mut CLS_LSN,
                plsnundonext: *mut CLS_LSN,
                plsnprevious: *mut CLS_LSN,
                plsnrecord: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadNextLogRecord(
            ::std::mem::transmute(pvreadcontext),
            ::std::mem::transmute(ppvbuffer),
            ::std::mem::transmute(pcbbuffer),
            ::std::mem::transmute(perecordtype),
            ::std::mem::transmute(plsnuser),
            ::std::mem::transmute(plsnundonext),
            ::std::mem::transmute(plsnprevious),
            ::std::mem::transmute(plsnrecord),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadOnlyEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadOnlyEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadOnlyEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(
    pvreadcontext: *mut ::std::ffi::c_void,
    ppvrestartbuffer: *mut *mut ::std::ffi::c_void,
    pcbrestartbuffer: *mut u32,
    plsnrestart: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadPreviousLogRestartArea(
                pvreadcontext: *mut ::std::ffi::c_void,
                ppvrestartbuffer: *mut *mut ::std::ffi::c_void,
                pcbrestartbuffer: *mut u32,
                plsnrestart: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadPreviousLogRestartArea(
            ::std::mem::transmute(pvreadcontext),
            ::std::mem::transmute(ppvrestartbuffer),
            ::std::mem::transmute(pcbrestartbuffer),
            ::std::mem::transmute(plsnrestart),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    enlistmentkey: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecoverEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                enlistmentkey: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RecoverEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(enlistmentkey),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverResourceManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    resourcemanagerhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecoverResourceManager(
                resourcemanagerhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RecoverResourceManager(
            resourcemanagerhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverTransactionManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionmanagerhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecoverTransactionManager(
                transactionmanagerhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RecoverTransactionManager(
            transactionmanagerhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterForLogWriteNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hlog: Param0,
    cbthreshold: u32,
    fenable: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterForLogWriteNotification(
                hlog: super::super::Foundation::HANDLE,
                cbthreshold: u32,
                fenable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterForLogWriteNotification(
            hlog.into_param().abi(),
            ::std::mem::transmute(cbthreshold),
            fenable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterManageableLogClient<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pcallbacks: *mut LOG_MANAGEMENT_CALLBACKS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterManageableLogClient(
                hlog: super::super::Foundation::HANDLE,
                pcallbacks: *mut ::std::mem::ManuallyDrop<LOG_MANAGEMENT_CALLBACKS>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterManageableLogClient(
            hlog.into_param().abi(),
            ::std::mem::transmute(pcallbacks),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lppathname: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDirectoryA(
                lppathname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDirectoryA(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDirectoryFromAppW(
                lppathname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDirectoryFromAppW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lppathname: Param0,
    htransaction: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDirectoryTransactedA(
                lppathname: super::super::Foundation::PSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDirectoryTransactedA(
            lppathname.into_param().abi(),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lppathname: Param0,
    htransaction: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDirectoryTransactedW(
                lppathname: super::super::Foundation::PWSTR,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDirectoryTransactedW(
            lppathname.into_param().abi(),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDirectoryW(
                lppathname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDirectoryW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogContainer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hlog: Param0,
    pwszcontainerpath: Param1,
    fforce: Param2,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveLogContainer(
                hlog: super::super::Foundation::HANDLE,
                pwszcontainerpath: super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveLogContainer(
            hlog.into_param().abi(),
            pwszcontainerpath.into_param().abi(),
            fforce.into_param().abi(),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogContainerSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hlog: Param0,
    ccontainer: u16,
    rgwszcontainerpath: *const super::super::Foundation::PWSTR,
    fforce: Param3,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveLogContainerSet(
                hlog: super::super::Foundation::HANDLE,
                ccontainer: u16,
                rgwszcontainerpath: *const super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveLogContainerSet(
            hlog.into_param().abi(),
            ::std::mem::transmute(ccontainer),
            ::std::mem::transmute(rgwszcontainerpath),
            fforce.into_param().abi(),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    epolicytype: CLFS_MGMT_POLICY_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveLogPolicy(
                hlog: super::super::Foundation::HANDLE,
                epolicytype: CLFS_MGMT_POLICY_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveLogPolicy(
            hlog.into_param().abi(),
            ::std::mem::transmute(epolicytype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RemoveUsersFromEncryptedFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveUsersFromEncryptedFile(
                lpfilename: super::super::Foundation::PWSTR,
                phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveUsersFromEncryptedFile(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(phashes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RenameTransactionManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    logfilename: Param0,
    existingtransactionmanagerguid: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RenameTransactionManager(
                logfilename: super::super::Foundation::PWSTR,
                existingtransactionmanagerguid: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RenameTransactionManager(
            logfilename.into_param().abi(),
            ::std::mem::transmute(existingtransactionmanagerguid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpreplacedfilename: Param0,
    lpreplacementfilename: Param1,
    lpbackupfilename: Param2,
    dwreplaceflags: REPLACE_FILE_FLAGS,
    lpexclude: *mut ::std::ffi::c_void,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceFileA(
                lpreplacedfilename: super::super::Foundation::PSTR,
                lpreplacementfilename: super::super::Foundation::PSTR,
                lpbackupfilename: super::super::Foundation::PSTR,
                dwreplaceflags: REPLACE_FILE_FLAGS,
                lpexclude: *mut ::std::ffi::c_void,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReplaceFileA(
            lpreplacedfilename.into_param().abi(),
            lpreplacementfilename.into_param().abi(),
            lpbackupfilename.into_param().abi(),
            ::std::mem::transmute(dwreplaceflags),
            ::std::mem::transmute(lpexclude),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpreplacedfilename: Param0,
    lpreplacementfilename: Param1,
    lpbackupfilename: Param2,
    dwreplaceflags: u32,
    lpexclude: *mut ::std::ffi::c_void,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceFileFromAppW(
                lpreplacedfilename: super::super::Foundation::PWSTR,
                lpreplacementfilename: super::super::Foundation::PWSTR,
                lpbackupfilename: super::super::Foundation::PWSTR,
                dwreplaceflags: u32,
                lpexclude: *mut ::std::ffi::c_void,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReplaceFileFromAppW(
            lpreplacedfilename.into_param().abi(),
            lpreplacementfilename.into_param().abi(),
            lpbackupfilename.into_param().abi(),
            ::std::mem::transmute(dwreplaceflags),
            ::std::mem::transmute(lpexclude),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpreplacedfilename: Param0,
    lpreplacementfilename: Param1,
    lpbackupfilename: Param2,
    dwreplaceflags: REPLACE_FILE_FLAGS,
    lpexclude: *mut ::std::ffi::c_void,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceFileW(
                lpreplacedfilename: super::super::Foundation::PWSTR,
                lpreplacementfilename: super::super::Foundation::PWSTR,
                lpbackupfilename: super::super::Foundation::PWSTR,
                dwreplaceflags: REPLACE_FILE_FLAGS,
                lpexclude: *mut ::std::ffi::c_void,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReplaceFileW(
            lpreplacedfilename.into_param().abi(),
            lpreplacementfilename.into_param().abi(),
            lpbackupfilename.into_param().abi(),
            ::std::mem::transmute(dwreplaceflags),
            ::std::mem::transmute(lpexclude),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReserveAndAppendLog(
    pvmarshal: *mut ::std::ffi::c_void,
    rgwriteentries: *mut CLS_WRITE_ENTRY,
    cwriteentries: u32,
    plsnundonext: *mut CLS_LSN,
    plsnprevious: *mut CLS_LSN,
    creserverecords: u32,
    rgcbreservation: *mut i64,
    fflags: CLFS_FLAG,
    plsn: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReserveAndAppendLog(
                pvmarshal: *mut ::std::ffi::c_void,
                rgwriteentries: *mut CLS_WRITE_ENTRY,
                cwriteentries: u32,
                plsnundonext: *mut CLS_LSN,
                plsnprevious: *mut CLS_LSN,
                creserverecords: u32,
                rgcbreservation: *mut i64,
                fflags: CLFS_FLAG,
                plsn: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReserveAndAppendLog(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(rgwriteentries),
            ::std::mem::transmute(cwriteentries),
            ::std::mem::transmute(plsnundonext),
            ::std::mem::transmute(plsnprevious),
            ::std::mem::transmute(creserverecords),
            ::std::mem::transmute(rgcbreservation),
            ::std::mem::transmute(fflags),
            ::std::mem::transmute(plsn),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(
    pvmarshal: *mut ::std::ffi::c_void,
    rgwriteentries: *mut CLS_WRITE_ENTRY,
    cwriteentries: u32,
    cbentryalignment: u32,
    plsnundonext: *mut CLS_LSN,
    plsnprevious: *mut CLS_LSN,
    creserverecords: u32,
    rgcbreservation: *mut i64,
    fflags: CLFS_FLAG,
    plsn: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReserveAndAppendLogAligned(
                pvmarshal: *mut ::std::ffi::c_void,
                rgwriteentries: *mut CLS_WRITE_ENTRY,
                cwriteentries: u32,
                cbentryalignment: u32,
                plsnundonext: *mut CLS_LSN,
                plsnprevious: *mut CLS_LSN,
                creserverecords: u32,
                rgcbreservation: *mut i64,
                fflags: CLFS_FLAG,
                plsn: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReserveAndAppendLogAligned(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(rgwriteentries),
            ::std::mem::transmute(cwriteentries),
            ::std::mem::transmute(cbentryalignment),
            ::std::mem::transmute(plsnundonext),
            ::std::mem::transmute(plsnprevious),
            ::std::mem::transmute(creserverecords),
            ::std::mem::transmute(rgcbreservation),
            ::std::mem::transmute(fflags),
            ::std::mem::transmute(plsn),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackComplete<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RollbackComplete(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RollbackComplete(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackEnlistment<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RollbackEnlistment(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RollbackEnlistment(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackTransaction<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RollbackTransaction(
                transactionhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RollbackTransaction(transactionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackTransactionAsync<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionhandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RollbackTransactionAsync(
                transactionhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RollbackTransactionAsync(
            transactionhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollforwardTransactionManager<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    transactionmanagerhandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RollforwardTransactionManager(
                transactionmanagerhandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RollforwardTransactionManager(
            transactionmanagerhandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: super::super::Foundation::PWSTR,
    pub srvai0_target: super::super::Foundation::PWSTR,
    pub srvai0_default: super::super::Foundation::BOOLEAN,
    pub srvai0_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVER_ALIAS_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVER_ALIAS_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVER_ALIAS_INFO_0")
            .field("srvai0_alias", &self.srvai0_alias)
            .field("srvai0_target", &self.srvai0_target)
            .field("srvai0_default", &self.srvai0_default)
            .field("srvai0_reserved", &self.srvai0_reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVER_ALIAS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvai0_alias == other.srvai0_alias
            && self.srvai0_target == other.srvai0_target
            && self.srvai0_default == other.srvai0_default
            && self.srvai0_reserved == other.srvai0_reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVER_ALIAS_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_CERTIFICATE_INFO_0 {
    pub srvci0_name: super::super::Foundation::PWSTR,
    pub srvci0_subject: super::super::Foundation::PWSTR,
    pub srvci0_issuer: super::super::Foundation::PWSTR,
    pub srvci0_thumbprint: super::super::Foundation::PWSTR,
    pub srvci0_friendlyname: super::super::Foundation::PWSTR,
    pub srvci0_notbefore: super::super::Foundation::PWSTR,
    pub srvci0_notafter: super::super::Foundation::PWSTR,
    pub srvci0_storelocation: super::super::Foundation::PWSTR,
    pub srvci0_storename: super::super::Foundation::PWSTR,
    pub srvci0_renewalchain: super::super::Foundation::PWSTR,
    pub srvci0_type: u32,
    pub srvci0_flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVER_CERTIFICATE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVER_CERTIFICATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVER_CERTIFICATE_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVER_CERTIFICATE_INFO_0")
            .field("srvci0_name", &self.srvci0_name)
            .field("srvci0_subject", &self.srvci0_subject)
            .field("srvci0_issuer", &self.srvci0_issuer)
            .field("srvci0_thumbprint", &self.srvci0_thumbprint)
            .field("srvci0_friendlyname", &self.srvci0_friendlyname)
            .field("srvci0_notbefore", &self.srvci0_notbefore)
            .field("srvci0_notafter", &self.srvci0_notafter)
            .field("srvci0_storelocation", &self.srvci0_storelocation)
            .field("srvci0_storename", &self.srvci0_storename)
            .field("srvci0_renewalchain", &self.srvci0_renewalchain)
            .field("srvci0_type", &self.srvci0_type)
            .field("srvci0_flags", &self.srvci0_flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVER_CERTIFICATE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvci0_name == other.srvci0_name
            && self.srvci0_subject == other.srvci0_subject
            && self.srvci0_issuer == other.srvci0_issuer
            && self.srvci0_thumbprint == other.srvci0_thumbprint
            && self.srvci0_friendlyname == other.srvci0_friendlyname
            && self.srvci0_notbefore == other.srvci0_notbefore
            && self.srvci0_notafter == other.srvci0_notafter
            && self.srvci0_storelocation == other.srvci0_storelocation
            && self.srvci0_storename == other.srvci0_storename
            && self.srvci0_renewalchain == other.srvci0_renewalchain
            && self.srvci0_type == other.srvci0_type
            && self.srvci0_flags == other.srvci0_flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVER_CERTIFICATE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVER_CERTIFICATE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct SERVER_CERTIFICATE_TYPE(pub i32);
pub const QUIC: SERVER_CERTIFICATE_TYPE = SERVER_CERTIFICATE_TYPE(0i32);
impl ::std::convert::From<i32> for SERVER_CERTIFICATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVER_CERTIFICATE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SESSION_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SESSION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SESSION_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SESSION_INFO_0")
            .field("sesi0_cname", &self.sesi0_cname)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SESSION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi0_cname == other.sesi0_cname
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SESSION_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SESSION_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: super::super::Foundation::PWSTR,
    pub sesi1_username: super::super::Foundation::PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: SESSION_INFO_USER_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl SESSION_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SESSION_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SESSION_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SESSION_INFO_1")
            .field("sesi1_cname", &self.sesi1_cname)
            .field("sesi1_username", &self.sesi1_username)
            .field("sesi1_num_opens", &self.sesi1_num_opens)
            .field("sesi1_time", &self.sesi1_time)
            .field("sesi1_idle_time", &self.sesi1_idle_time)
            .field("sesi1_user_flags", &self.sesi1_user_flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SESSION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi1_cname == other.sesi1_cname
            && self.sesi1_username == other.sesi1_username
            && self.sesi1_num_opens == other.sesi1_num_opens
            && self.sesi1_time == other.sesi1_time
            && self.sesi1_idle_time == other.sesi1_idle_time
            && self.sesi1_user_flags == other.sesi1_user_flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SESSION_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SESSION_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: super::super::Foundation::PWSTR,
    pub sesi10_username: super::super::Foundation::PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SESSION_INFO_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SESSION_INFO_10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SESSION_INFO_10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SESSION_INFO_10")
            .field("sesi10_cname", &self.sesi10_cname)
            .field("sesi10_username", &self.sesi10_username)
            .field("sesi10_time", &self.sesi10_time)
            .field("sesi10_idle_time", &self.sesi10_idle_time)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SESSION_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi10_cname == other.sesi10_cname
            && self.sesi10_username == other.sesi10_username
            && self.sesi10_time == other.sesi10_time
            && self.sesi10_idle_time == other.sesi10_idle_time
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SESSION_INFO_10 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SESSION_INFO_10 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: super::super::Foundation::PWSTR,
    pub sesi2_username: super::super::Foundation::PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi2_cltype_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SESSION_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SESSION_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SESSION_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SESSION_INFO_2")
            .field("sesi2_cname", &self.sesi2_cname)
            .field("sesi2_username", &self.sesi2_username)
            .field("sesi2_num_opens", &self.sesi2_num_opens)
            .field("sesi2_time", &self.sesi2_time)
            .field("sesi2_idle_time", &self.sesi2_idle_time)
            .field("sesi2_user_flags", &self.sesi2_user_flags)
            .field("sesi2_cltype_name", &self.sesi2_cltype_name)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SESSION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi2_cname == other.sesi2_cname
            && self.sesi2_username == other.sesi2_username
            && self.sesi2_num_opens == other.sesi2_num_opens
            && self.sesi2_time == other.sesi2_time
            && self.sesi2_idle_time == other.sesi2_idle_time
            && self.sesi2_user_flags == other.sesi2_user_flags
            && self.sesi2_cltype_name == other.sesi2_cltype_name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SESSION_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SESSION_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: super::super::Foundation::PWSTR,
    pub sesi502_username: super::super::Foundation::PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi502_cltype_name: super::super::Foundation::PWSTR,
    pub sesi502_transport: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SESSION_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SESSION_INFO_502 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SESSION_INFO_502 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SESSION_INFO_502")
            .field("sesi502_cname", &self.sesi502_cname)
            .field("sesi502_username", &self.sesi502_username)
            .field("sesi502_num_opens", &self.sesi502_num_opens)
            .field("sesi502_time", &self.sesi502_time)
            .field("sesi502_idle_time", &self.sesi502_idle_time)
            .field("sesi502_user_flags", &self.sesi502_user_flags)
            .field("sesi502_cltype_name", &self.sesi502_cltype_name)
            .field("sesi502_transport", &self.sesi502_transport)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SESSION_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi502_cname == other.sesi502_cname
            && self.sesi502_username == other.sesi502_username
            && self.sesi502_num_opens == other.sesi502_num_opens
            && self.sesi502_time == other.sesi502_time
            && self.sesi502_idle_time == other.sesi502_idle_time
            && self.sesi502_user_flags == other.sesi502_user_flags
            && self.sesi502_cltype_name == other.sesi502_cltype_name
            && self.sesi502_transport == other.sesi502_transport
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SESSION_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SESSION_INFO_502 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct SESSION_INFO_USER_FLAGS(pub u32);
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(1u32);
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(2u32);
impl ::std::convert::From<u32> for SESSION_INFO_USER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SESSION_INFO_USER_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SESSION_INFO_USER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SESSION_INFO_USER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SESSION_INFO_USER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SESSION_INFO_USER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SESSION_INFO_USER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct SET_FILE_POINTER_MOVE_METHOD(pub u32);
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(0u32);
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(1u32);
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(2u32);
impl ::std::convert::From<u32> for SET_FILE_POINTER_MOVE_METHOD {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SET_FILE_POINTER_MOVE_METHOD {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SET_FILE_POINTER_MOVE_METHOD {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SET_FILE_POINTER_MOVE_METHOD {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SET_FILE_POINTER_MOVE_METHOD {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SET_FILE_POINTER_MOVE_METHOD {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SET_FILE_POINTER_MOVE_METHOD {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_0 {
    pub shi0_netname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SHARE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SHARE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SHARE_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_0")
            .field("shi0_netname", &self.shi0_netname)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SHARE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.shi0_netname == other.shi0_netname
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SHARE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_1 {
    pub shi1_netname: super::super::Foundation::PWSTR,
    pub shi1_type: SHARE_TYPE,
    pub shi1_remark: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SHARE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SHARE_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SHARE_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1")
            .field("shi1_netname", &self.shi1_netname)
            .field("shi1_type", &self.shi1_type)
            .field("shi1_remark", &self.shi1_remark)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SHARE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1_netname == other.shi1_netname
            && self.shi1_type == other.shi1_type
            && self.shi1_remark == other.shi1_remark
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SHARE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SHARE_INFO_1004 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SHARE_INFO_1004 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SHARE_INFO_1004 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1004")
            .field("shi1004_remark", &self.shi1004_remark)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SHARE_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1004_remark == other.shi1004_remark
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SHARE_INFO_1004 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1004 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
impl SHARE_INFO_1005 {}
impl ::std::default::Default for SHARE_INFO_1005 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SHARE_INFO_1005 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1005")
            .field("shi1005_flags", &self.shi1005_flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SHARE_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1005_flags == other.shi1005_flags
    }
}
impl ::std::cmp::Eq for SHARE_INFO_1005 {}
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1005 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
impl SHARE_INFO_1006 {}
impl ::std::default::Default for SHARE_INFO_1006 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SHARE_INFO_1006 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1006")
            .field("shi1006_max_uses", &self.shi1006_max_uses)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SHARE_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1006_max_uses == other.shi1006_max_uses
    }
}
impl ::std::cmp::Eq for SHARE_INFO_1006 {}
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1006 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl SHARE_INFO_1501 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for SHARE_INFO_1501 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for SHARE_INFO_1501 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1501")
            .field("shi1501_reserved", &self.shi1501_reserved)
            .field(
                "shi1501_security_descriptor",
                &self.shi1501_security_descriptor,
            )
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for SHARE_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1501_reserved == other.shi1501_reserved
            && self.shi1501_security_descriptor == other.shi1501_security_descriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for SHARE_INFO_1501 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1501 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: ::windows::runtime::GUID,
}
impl SHARE_INFO_1503 {}
impl ::std::default::Default for SHARE_INFO_1503 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SHARE_INFO_1503 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_1503")
            .field("shi1503_sharefilter", &self.shi1503_sharefilter)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SHARE_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1503_sharefilter == other.shi1503_sharefilter
    }
}
impl ::std::cmp::Eq for SHARE_INFO_1503 {}
unsafe impl ::windows::runtime::Abi for SHARE_INFO_1503 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_2 {
    pub shi2_netname: super::super::Foundation::PWSTR,
    pub shi2_type: SHARE_TYPE,
    pub shi2_remark: super::super::Foundation::PWSTR,
    pub shi2_permissions: SHARE_INFO_PERMISSIONS,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: super::super::Foundation::PWSTR,
    pub shi2_passwd: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SHARE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SHARE_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SHARE_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_2")
            .field("shi2_netname", &self.shi2_netname)
            .field("shi2_type", &self.shi2_type)
            .field("shi2_remark", &self.shi2_remark)
            .field("shi2_permissions", &self.shi2_permissions)
            .field("shi2_max_uses", &self.shi2_max_uses)
            .field("shi2_current_uses", &self.shi2_current_uses)
            .field("shi2_path", &self.shi2_path)
            .field("shi2_passwd", &self.shi2_passwd)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SHARE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.shi2_netname == other.shi2_netname
            && self.shi2_type == other.shi2_type
            && self.shi2_remark == other.shi2_remark
            && self.shi2_permissions == other.shi2_permissions
            && self.shi2_max_uses == other.shi2_max_uses
            && self.shi2_current_uses == other.shi2_current_uses
            && self.shi2_path == other.shi2_path
            && self.shi2_passwd == other.shi2_passwd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SHARE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_501 {
    pub shi501_netname: super::super::Foundation::PWSTR,
    pub shi501_type: SHARE_TYPE,
    pub shi501_remark: super::super::Foundation::PWSTR,
    pub shi501_flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SHARE_INFO_501 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SHARE_INFO_501 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SHARE_INFO_501 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_501")
            .field("shi501_netname", &self.shi501_netname)
            .field("shi501_type", &self.shi501_type)
            .field("shi501_remark", &self.shi501_remark)
            .field("shi501_flags", &self.shi501_flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SHARE_INFO_501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi501_netname == other.shi501_netname
            && self.shi501_type == other.shi501_type
            && self.shi501_remark == other.shi501_remark
            && self.shi501_flags == other.shi501_flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SHARE_INFO_501 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_501 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_502 {
    pub shi502_netname: super::super::Foundation::PWSTR,
    pub shi502_type: SHARE_TYPE,
    pub shi502_remark: super::super::Foundation::PWSTR,
    pub shi502_permissions: SHARE_INFO_PERMISSIONS,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: super::super::Foundation::PWSTR,
    pub shi502_passwd: super::super::Foundation::PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl SHARE_INFO_502 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for SHARE_INFO_502 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for SHARE_INFO_502 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_502")
            .field("shi502_netname", &self.shi502_netname)
            .field("shi502_type", &self.shi502_type)
            .field("shi502_remark", &self.shi502_remark)
            .field("shi502_permissions", &self.shi502_permissions)
            .field("shi502_max_uses", &self.shi502_max_uses)
            .field("shi502_current_uses", &self.shi502_current_uses)
            .field("shi502_path", &self.shi502_path)
            .field("shi502_passwd", &self.shi502_passwd)
            .field("shi502_reserved", &self.shi502_reserved)
            .field(
                "shi502_security_descriptor",
                &self.shi502_security_descriptor,
            )
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for SHARE_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.shi502_netname == other.shi502_netname
            && self.shi502_type == other.shi502_type
            && self.shi502_remark == other.shi502_remark
            && self.shi502_permissions == other.shi502_permissions
            && self.shi502_max_uses == other.shi502_max_uses
            && self.shi502_current_uses == other.shi502_current_uses
            && self.shi502_path == other.shi502_path
            && self.shi502_passwd == other.shi502_passwd
            && self.shi502_reserved == other.shi502_reserved
            && self.shi502_security_descriptor == other.shi502_security_descriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for SHARE_INFO_502 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_502 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_503 {
    pub shi503_netname: super::super::Foundation::PWSTR,
    pub shi503_type: SHARE_TYPE,
    pub shi503_remark: super::super::Foundation::PWSTR,
    pub shi503_permissions: SHARE_INFO_PERMISSIONS,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: super::super::Foundation::PWSTR,
    pub shi503_passwd: super::super::Foundation::PWSTR,
    pub shi503_servername: super::super::Foundation::PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl SHARE_INFO_503 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for SHARE_INFO_503 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for SHARE_INFO_503 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHARE_INFO_503")
            .field("shi503_netname", &self.shi503_netname)
            .field("shi503_type", &self.shi503_type)
            .field("shi503_remark", &self.shi503_remark)
            .field("shi503_permissions", &self.shi503_permissions)
            .field("shi503_max_uses", &self.shi503_max_uses)
            .field("shi503_current_uses", &self.shi503_current_uses)
            .field("shi503_path", &self.shi503_path)
            .field("shi503_passwd", &self.shi503_passwd)
            .field("shi503_servername", &self.shi503_servername)
            .field("shi503_reserved", &self.shi503_reserved)
            .field(
                "shi503_security_descriptor",
                &self.shi503_security_descriptor,
            )
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for SHARE_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi503_netname == other.shi503_netname
            && self.shi503_type == other.shi503_type
            && self.shi503_remark == other.shi503_remark
            && self.shi503_permissions == other.shi503_permissions
            && self.shi503_max_uses == other.shi503_max_uses
            && self.shi503_current_uses == other.shi503_current_uses
            && self.shi503_path == other.shi503_path
            && self.shi503_passwd == other.shi503_passwd
            && self.shi503_servername == other.shi503_servername
            && self.shi503_reserved == other.shi503_reserved
            && self.shi503_security_descriptor == other.shi503_security_descriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for SHARE_INFO_503 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for SHARE_INFO_503 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct SHARE_INFO_PERMISSIONS(pub u32);
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(1u32);
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(2u32);
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(4u32);
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(8u32);
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(16u32);
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32u32);
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(64u32);
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32768u32);
impl ::std::convert::From<u32> for SHARE_INFO_PERMISSIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHARE_INFO_PERMISSIONS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SHARE_INFO_PERMISSIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SHARE_INFO_PERMISSIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SHARE_INFO_PERMISSIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SHARE_INFO_PERMISSIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SHARE_INFO_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SHARE_TYPE(pub u32);
pub const STYPE_DISKTREE: SHARE_TYPE = SHARE_TYPE(0u32);
pub const STYPE_PRINTQ: SHARE_TYPE = SHARE_TYPE(1u32);
pub const STYPE_DEVICE: SHARE_TYPE = SHARE_TYPE(2u32);
pub const STYPE_IPC: SHARE_TYPE = SHARE_TYPE(3u32);
pub const STYPE_SPECIAL: SHARE_TYPE = SHARE_TYPE(2147483648u32);
pub const STYPE_TEMPORARY: SHARE_TYPE = SHARE_TYPE(1073741824u32);
pub const STYPE_MASK: SHARE_TYPE = SHARE_TYPE(255u32);
impl ::std::convert::From<u32> for SHARE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHARE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SHARE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SHARE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SHARE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SHARE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SHARE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
pub const STATSOPT_CLR: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
impl STAT_SERVER_0 {}
impl ::std::default::Default for STAT_SERVER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STAT_SERVER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STAT_SERVER_0")
            .field("sts0_start", &self.sts0_start)
            .field("sts0_fopens", &self.sts0_fopens)
            .field("sts0_devopens", &self.sts0_devopens)
            .field("sts0_jobsqueued", &self.sts0_jobsqueued)
            .field("sts0_sopens", &self.sts0_sopens)
            .field("sts0_stimedout", &self.sts0_stimedout)
            .field("sts0_serrorout", &self.sts0_serrorout)
            .field("sts0_pwerrors", &self.sts0_pwerrors)
            .field("sts0_permerrors", &self.sts0_permerrors)
            .field("sts0_syserrors", &self.sts0_syserrors)
            .field("sts0_bytessent_low", &self.sts0_bytessent_low)
            .field("sts0_bytessent_high", &self.sts0_bytessent_high)
            .field("sts0_bytesrcvd_low", &self.sts0_bytesrcvd_low)
            .field("sts0_bytesrcvd_high", &self.sts0_bytesrcvd_high)
            .field("sts0_avresponse", &self.sts0_avresponse)
            .field("sts0_reqbufneed", &self.sts0_reqbufneed)
            .field("sts0_bigbufneed", &self.sts0_bigbufneed)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STAT_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sts0_start == other.sts0_start
            && self.sts0_fopens == other.sts0_fopens
            && self.sts0_devopens == other.sts0_devopens
            && self.sts0_jobsqueued == other.sts0_jobsqueued
            && self.sts0_sopens == other.sts0_sopens
            && self.sts0_stimedout == other.sts0_stimedout
            && self.sts0_serrorout == other.sts0_serrorout
            && self.sts0_pwerrors == other.sts0_pwerrors
            && self.sts0_permerrors == other.sts0_permerrors
            && self.sts0_syserrors == other.sts0_syserrors
            && self.sts0_bytessent_low == other.sts0_bytessent_low
            && self.sts0_bytessent_high == other.sts0_bytessent_high
            && self.sts0_bytesrcvd_low == other.sts0_bytesrcvd_low
            && self.sts0_bytesrcvd_high == other.sts0_bytesrcvd_high
            && self.sts0_avresponse == other.sts0_avresponse
            && self.sts0_reqbufneed == other.sts0_reqbufneed
            && self.sts0_bigbufneed == other.sts0_bigbufneed
    }
}
impl ::std::cmp::Eq for STAT_SERVER_0 {}
unsafe impl ::windows::runtime::Abi for STAT_SERVER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: i64,
    pub BytesReceived: i64,
    pub SmbsReceived: i64,
    pub PagingReadBytesRequested: i64,
    pub NonPagingReadBytesRequested: i64,
    pub CacheReadBytesRequested: i64,
    pub NetworkReadBytesRequested: i64,
    pub BytesTransmitted: i64,
    pub SmbsTransmitted: i64,
    pub PagingWriteBytesRequested: i64,
    pub NonPagingWriteBytesRequested: i64,
    pub CacheWriteBytesRequested: i64,
    pub NetworkWriteBytesRequested: i64,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
impl STAT_WORKSTATION_0 {}
impl ::std::default::Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STAT_WORKSTATION_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STAT_WORKSTATION_0")
            .field("StatisticsStartTime", &self.StatisticsStartTime)
            .field("BytesReceived", &self.BytesReceived)
            .field("SmbsReceived", &self.SmbsReceived)
            .field("PagingReadBytesRequested", &self.PagingReadBytesRequested)
            .field(
                "NonPagingReadBytesRequested",
                &self.NonPagingReadBytesRequested,
            )
            .field("CacheReadBytesRequested", &self.CacheReadBytesRequested)
            .field("NetworkReadBytesRequested", &self.NetworkReadBytesRequested)
            .field("BytesTransmitted", &self.BytesTransmitted)
            .field("SmbsTransmitted", &self.SmbsTransmitted)
            .field("PagingWriteBytesRequested", &self.PagingWriteBytesRequested)
            .field(
                "NonPagingWriteBytesRequested",
                &self.NonPagingWriteBytesRequested,
            )
            .field("CacheWriteBytesRequested", &self.CacheWriteBytesRequested)
            .field(
                "NetworkWriteBytesRequested",
                &self.NetworkWriteBytesRequested,
            )
            .field("InitiallyFailedOperations", &self.InitiallyFailedOperations)
            .field(
                "FailedCompletionOperations",
                &self.FailedCompletionOperations,
            )
            .field("ReadOperations", &self.ReadOperations)
            .field("RandomReadOperations", &self.RandomReadOperations)
            .field("ReadSmbs", &self.ReadSmbs)
            .field("LargeReadSmbs", &self.LargeReadSmbs)
            .field("SmallReadSmbs", &self.SmallReadSmbs)
            .field("WriteOperations", &self.WriteOperations)
            .field("RandomWriteOperations", &self.RandomWriteOperations)
            .field("WriteSmbs", &self.WriteSmbs)
            .field("LargeWriteSmbs", &self.LargeWriteSmbs)
            .field("SmallWriteSmbs", &self.SmallWriteSmbs)
            .field("RawReadsDenied", &self.RawReadsDenied)
            .field("RawWritesDenied", &self.RawWritesDenied)
            .field("NetworkErrors", &self.NetworkErrors)
            .field("Sessions", &self.Sessions)
            .field("FailedSessions", &self.FailedSessions)
            .field("Reconnects", &self.Reconnects)
            .field("CoreConnects", &self.CoreConnects)
            .field("Lanman20Connects", &self.Lanman20Connects)
            .field("Lanman21Connects", &self.Lanman21Connects)
            .field("LanmanNtConnects", &self.LanmanNtConnects)
            .field("ServerDisconnects", &self.ServerDisconnects)
            .field("HungSessions", &self.HungSessions)
            .field("UseCount", &self.UseCount)
            .field("FailedUseCount", &self.FailedUseCount)
            .field("CurrentCommands", &self.CurrentCommands)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STAT_WORKSTATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StatisticsStartTime == other.StatisticsStartTime
            && self.BytesReceived == other.BytesReceived
            && self.SmbsReceived == other.SmbsReceived
            && self.PagingReadBytesRequested == other.PagingReadBytesRequested
            && self.NonPagingReadBytesRequested == other.NonPagingReadBytesRequested
            && self.CacheReadBytesRequested == other.CacheReadBytesRequested
            && self.NetworkReadBytesRequested == other.NetworkReadBytesRequested
            && self.BytesTransmitted == other.BytesTransmitted
            && self.SmbsTransmitted == other.SmbsTransmitted
            && self.PagingWriteBytesRequested == other.PagingWriteBytesRequested
            && self.NonPagingWriteBytesRequested == other.NonPagingWriteBytesRequested
            && self.CacheWriteBytesRequested == other.CacheWriteBytesRequested
            && self.NetworkWriteBytesRequested == other.NetworkWriteBytesRequested
            && self.InitiallyFailedOperations == other.InitiallyFailedOperations
            && self.FailedCompletionOperations == other.FailedCompletionOperations
            && self.ReadOperations == other.ReadOperations
            && self.RandomReadOperations == other.RandomReadOperations
            && self.ReadSmbs == other.ReadSmbs
            && self.LargeReadSmbs == other.LargeReadSmbs
            && self.SmallReadSmbs == other.SmallReadSmbs
            && self.WriteOperations == other.WriteOperations
            && self.RandomWriteOperations == other.RandomWriteOperations
            && self.WriteSmbs == other.WriteSmbs
            && self.LargeWriteSmbs == other.LargeWriteSmbs
            && self.SmallWriteSmbs == other.SmallWriteSmbs
            && self.RawReadsDenied == other.RawReadsDenied
            && self.RawWritesDenied == other.RawWritesDenied
            && self.NetworkErrors == other.NetworkErrors
            && self.Sessions == other.Sessions
            && self.FailedSessions == other.FailedSessions
            && self.Reconnects == other.Reconnects
            && self.CoreConnects == other.CoreConnects
            && self.Lanman20Connects == other.Lanman20Connects
            && self.Lanman21Connects == other.Lanman21Connects
            && self.LanmanNtConnects == other.LanmanNtConnects
            && self.ServerDisconnects == other.ServerDisconnects
            && self.HungSessions == other.HungSessions
            && self.UseCount == other.UseCount
            && self.FailedUseCount == other.FailedUseCount
            && self.CurrentCommands == other.CurrentCommands
    }
}
impl ::std::cmp::Eq for STAT_WORKSTATION_0 {}
unsafe impl ::windows::runtime::Abi for STAT_WORKSTATION_0 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct STORAGE_BUS_TYPE(pub i32);
pub const BusTypeUnknown: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(0i32);
pub const BusTypeScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(1i32);
pub const BusTypeAtapi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(2i32);
pub const BusTypeAta: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(3i32);
pub const BusType1394: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(4i32);
pub const BusTypeSsa: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(5i32);
pub const BusTypeFibre: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(6i32);
pub const BusTypeUsb: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(7i32);
pub const BusTypeRAID: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(8i32);
pub const BusTypeiScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(9i32);
pub const BusTypeSas: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(10i32);
pub const BusTypeSata: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(11i32);
pub const BusTypeSd: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(12i32);
pub const BusTypeMmc: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(13i32);
pub const BusTypeVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(14i32);
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(15i32);
pub const BusTypeSpaces: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(16i32);
pub const BusTypeNvme: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(17i32);
pub const BusTypeSCM: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(18i32);
pub const BusTypeUfs: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(19i32);
pub const BusTypeMax: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(20i32);
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(127i32);
impl ::std::convert::From<i32> for STORAGE_BUS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STORAGE_BUS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct STREAM_INFO_LEVELS(pub i32);
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(0i32);
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(1i32);
impl ::std::convert::From<i32> for STREAM_INFO_LEVELS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STREAM_INFO_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STYPE_RESERVED1: u32 = 16777216u32;
pub const STYPE_RESERVED2: u32 = 33554432u32;
pub const STYPE_RESERVED3: u32 = 67108864u32;
pub const STYPE_RESERVED4: u32 = 134217728u32;
pub const STYPE_RESERVED5: u32 = 1048576u32;
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SYMBOLIC_LINK_FLAGS(pub u32);
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(1u32);
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS =
    SYMBOLIC_LINK_FLAGS(2u32);
impl ::std::convert::From<u32> for SYMBOLIC_LINK_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYMBOLIC_LINK_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SYMBOLIC_LINK_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SYMBOLIC_LINK_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScanLogContainers(
    pcxscan: *mut CLS_SCAN_CONTEXT,
    escanmode: u8,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScanLogContainers(
                pcxscan: *mut CLS_SCAN_CONTEXT,
                escanmode: u8,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ScanLogContainers(
            ::std::mem::transmute(pcxscan),
            ::std::mem::transmute(escanmode),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SearchPathA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lppath: Param0,
    lpfilename: Param1,
    lpextension: Param2,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PSTR,
    lpfilepart: *mut super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SearchPathA(
                lppath: super::super::Foundation::PSTR,
                lpfilename: super::super::Foundation::PSTR,
                lpextension: super::super::Foundation::PSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PSTR,
                lpfilepart: *mut super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(SearchPathA(
            lppath.into_param().abi(),
            lpfilename.into_param().abi(),
            lpextension.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SearchPathW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppath: Param0,
    lpfilename: Param1,
    lpextension: Param2,
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PWSTR,
    lpfilepart: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SearchPathW(
                lppath: super::super::Foundation::PWSTR,
                lpfilename: super::super::Foundation::PWSTR,
                lpextension: super::super::Foundation::PWSTR,
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PWSTR,
                lpfilepart: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(SearchPathW(
            lppath.into_param().abi(),
            lpfilename.into_param().abi(),
            lpextension.into_param().abi(),
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpfilepart),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SetEncryptedFileMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    pboldmetadata: *const u8,
    pbnewmetadata: *const u8,
    pownerhash: *const ENCRYPTION_CERTIFICATE_HASH,
    dwoperation: u32,
    pcertificatesadded: *const ENCRYPTION_CERTIFICATE_HASH_LIST,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEncryptedFileMetadata(
                lpfilename: super::super::Foundation::PWSTR,
                pboldmetadata: *const u8,
                pbnewmetadata: *const u8,
                pownerhash: *const ENCRYPTION_CERTIFICATE_HASH,
                dwoperation: u32,
                pcertificatesadded: *const ENCRYPTION_CERTIFICATE_HASH_LIST,
            ) -> u32;
        }
        ::std::mem::transmute(SetEncryptedFileMetadata(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(pboldmetadata),
            ::std::mem::transmute(pbnewmetadata),
            ::std::mem::transmute(pownerhash),
            ::std::mem::transmute(dwoperation),
            ::std::mem::transmute(pcertificatesadded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEndOfFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEndOfFile(
                hfile: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetEndOfFile(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn SetEndOfLog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    plsnend: *mut CLS_LSN,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEndOfLog(
                hlog: super::super::Foundation::HANDLE,
                plsnend: *mut CLS_LSN,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetEndOfLog(
            hlog.into_param().abi(),
            ::std::mem::transmute(plsnend),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    buffersize: u32,
    buffer: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnlistmentRecoveryInformation(
                enlistmenthandle: super::super::Foundation::HANDLE,
                buffersize: u32,
                buffer: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetEnlistmentRecoveryInformation(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetFileApisToANSI() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileApisToANSI();
        }
        ::std::mem::transmute(SetFileApisToANSI())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetFileApisToOEM() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileApisToOEM();
        }
        ::std::mem::transmute(SetFileApisToOEM())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileAttributesA(
                lpfilename: super::super::Foundation::PSTR,
                dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileAttributesA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwfileattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesFromAppW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwfileattributes: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileAttributesFromAppW(
                lpfilename: super::super::Foundation::PWSTR,
                dwfileattributes: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileAttributesFromAppW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwfileattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwfileattributes: u32,
    htransaction: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileAttributesTransactedA(
                lpfilename: super::super::Foundation::PSTR,
                dwfileattributes: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileAttributesTransactedA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwfileattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lpfilename: Param0,
    dwfileattributes: u32,
    htransaction: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileAttributesTransactedW(
                lpfilename: super::super::Foundation::PWSTR,
                dwfileattributes: u32,
                htransaction: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileAttributesTransactedW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwfileattributes),
            htransaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileAttributesW(
                lpfilename: super::super::Foundation::PWSTR,
                dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileAttributesW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(dwfileattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileBandwidthReservation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfile: Param0,
    nperiodmilliseconds: u32,
    nbytesperperiod: u32,
    bdiscardable: Param3,
    lptransfersize: *mut u32,
    lpnumoutstandingrequests: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileBandwidthReservation(
                hfile: super::super::Foundation::HANDLE,
                nperiodmilliseconds: u32,
                nbytesperperiod: u32,
                bdiscardable: super::super::Foundation::BOOL,
                lptransfersize: *mut u32,
                lpnumoutstandingrequests: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileBandwidthReservation(
            hfile.into_param().abi(),
            ::std::mem::transmute(nperiodmilliseconds),
            ::std::mem::transmute(nbytesperperiod),
            bdiscardable.into_param().abi(),
            ::std::mem::transmute(lptransfersize),
            ::std::mem::transmute(lpnumoutstandingrequests),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileCompletionNotificationModes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    flags: u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileCompletionNotificationModes(
                filehandle: super::super::Foundation::HANDLE,
                flags: u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileCompletionNotificationModes(
            filehandle.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileInformationByHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    fileinformationclass: FILE_INFO_BY_HANDLE_CLASS,
    lpfileinformation: *const ::std::ffi::c_void,
    dwbuffersize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileInformationByHandle(
                hfile: super::super::Foundation::HANDLE,
                fileinformationclass: FILE_INFO_BY_HANDLE_CLASS,
                lpfileinformation: *const ::std::ffi::c_void,
                dwbuffersize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileInformationByHandle(
            hfile.into_param().abi(),
            ::std::mem::transmute(fileinformationclass),
            ::std::mem::transmute(lpfileinformation),
            ::std::mem::transmute(dwbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileIoOverlappedRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    overlappedrangestart: *const u8,
    length: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileIoOverlappedRange(
                filehandle: super::super::Foundation::HANDLE,
                overlappedrangestart: *const u8,
                length: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileIoOverlappedRange(
            filehandle.into_param().abi(),
            ::std::mem::transmute(overlappedrangestart),
            ::std::mem::transmute(length),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFilePointer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    ldistancetomove: i32,
    lpdistancetomovehigh: *mut i32,
    dwmovemethod: SET_FILE_POINTER_MOVE_METHOD,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFilePointer(
                hfile: super::super::Foundation::HANDLE,
                ldistancetomove: i32,
                lpdistancetomovehigh: *mut i32,
                dwmovemethod: SET_FILE_POINTER_MOVE_METHOD,
            ) -> u32;
        }
        ::std::mem::transmute(SetFilePointer(
            hfile.into_param().abi(),
            ::std::mem::transmute(ldistancetomove),
            ::std::mem::transmute(lpdistancetomovehigh),
            ::std::mem::transmute(dwmovemethod),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFilePointerEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lidistancetomove: i64,
    lpnewfilepointer: *mut i64,
    dwmovemethod: SET_FILE_POINTER_MOVE_METHOD,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFilePointerEx(
                hfile: super::super::Foundation::HANDLE,
                lidistancetomove: i64,
                lpnewfilepointer: *mut i64,
                dwmovemethod: SET_FILE_POINTER_MOVE_METHOD,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFilePointerEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lidistancetomove),
            ::std::mem::transmute(lpnewfilepointer),
            ::std::mem::transmute(dwmovemethod),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileShortNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hfile: Param0,
    lpshortname: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileShortNameA(
                hfile: super::super::Foundation::HANDLE,
                lpshortname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileShortNameA(
            hfile.into_param().abi(),
            lpshortname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileShortNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hfile: Param0,
    lpshortname: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileShortNameW(
                hfile: super::super::Foundation::HANDLE,
                lpshortname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileShortNameW(
            hfile.into_param().abi(),
            lpshortname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileTime<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpcreationtime: *const super::super::Foundation::FILETIME,
    lplastaccesstime: *const super::super::Foundation::FILETIME,
    lplastwritetime: *const super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileTime(
                hfile: super::super::Foundation::HANDLE,
                lpcreationtime: *const super::super::Foundation::FILETIME,
                lplastaccesstime: *const super::super::Foundation::FILETIME,
                lplastwritetime: *const super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileTime(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpcreationtime),
            ::std::mem::transmute(lplastaccesstime),
            ::std::mem::transmute(lplastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileValidData<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    validdatalength: i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileValidData(
                hfile: super::super::Foundation::HANDLE,
                validdatalength: i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileValidData(
            hfile.into_param().abi(),
            ::std::mem::transmute(validdatalength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIoRingCompletionEvent<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    ioring: *const HIORING__,
    hevent: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIoRingCompletionEvent(
                ioring: *const HIORING__,
                hevent: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        SetIoRingCompletionEvent(::std::mem::transmute(ioring), hevent.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogArchiveMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    emode: CLFS_LOG_ARCHIVE_MODE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetLogArchiveMode(
                hlog: super::super::Foundation::HANDLE,
                emode: CLFS_LOG_ARCHIVE_MODE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetLogArchiveMode(
            hlog.into_param().abi(),
            ::std::mem::transmute(emode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogArchiveTail<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    plsnarchivetail: *mut CLS_LSN,
    preserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetLogArchiveTail(
                hlog: super::super::Foundation::HANDLE,
                plsnarchivetail: *mut CLS_LSN,
                preserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetLogArchiveTail(
            hlog.into_param().abi(),
            ::std::mem::transmute(plsnarchivetail),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogFileSizeWithPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hlog: Param0,
    pdesiredsize: *mut u64,
    presultingsize: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetLogFileSizeWithPolicy(
                hlog: super::super::Foundation::HANDLE,
                pdesiredsize: *mut u64,
                presultingsize: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetLogFileSizeWithPolicy(
            hlog.into_param().abi(),
            ::std::mem::transmute(pdesiredsize),
            ::std::mem::transmute(presultingsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetResourceManagerCompletionPort<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    resourcemanagerhandle: Param0,
    iocompletionporthandle: Param1,
    completionkey: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetResourceManagerCompletionPort(
                resourcemanagerhandle: super::super::Foundation::HANDLE,
                iocompletionporthandle: super::super::Foundation::HANDLE,
                completionkey: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetResourceManagerCompletionPort(
            resourcemanagerhandle.into_param().abi(),
            iocompletionporthandle.into_param().abi(),
            ::std::mem::transmute(completionkey),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSearchPathMode(flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSearchPathMode(flags: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSearchPathMode(::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTapeParameters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    dwoperation: TAPE_INFORMATION_TYPE,
    lptapeinformation: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTapeParameters(
                hdevice: super::super::Foundation::HANDLE,
                dwoperation: TAPE_INFORMATION_TYPE,
                lptapeinformation: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(SetTapeParameters(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwoperation),
            ::std::mem::transmute(lptapeinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTapePosition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdevice: Param0,
    dwpositionmethod: TAPE_POSITION_METHOD,
    dwpartition: u32,
    dwoffsetlow: u32,
    dwoffsethigh: u32,
    bimmediate: Param5,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTapePosition(
                hdevice: super::super::Foundation::HANDLE,
                dwpositionmethod: TAPE_POSITION_METHOD,
                dwpartition: u32,
                dwoffsetlow: u32,
                dwoffsethigh: u32,
                bimmediate: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(SetTapePosition(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwpositionmethod),
            ::std::mem::transmute(dwpartition),
            ::std::mem::transmute(dwoffsetlow),
            ::std::mem::transmute(dwoffsethigh),
            bimmediate.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTransactionInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    transactionhandle: Param0,
    isolationlevel: u32,
    isolationflags: u32,
    timeout: u32,
    description: Param4,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTransactionInformation(
                transactionhandle: super::super::Foundation::HANDLE,
                isolationlevel: u32,
                isolationflags: u32,
                timeout: u32,
                description: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetTransactionInformation(
            transactionhandle.into_param().abi(),
            ::std::mem::transmute(isolationlevel),
            ::std::mem::transmute(isolationflags),
            ::std::mem::transmute(timeout),
            description.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKey(
    pencryptioncertificate: *const ENCRYPTION_CERTIFICATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserFileEncryptionKey(
                pencryptioncertificate: *const ENCRYPTION_CERTIFICATE,
            ) -> u32;
        }
        ::std::mem::transmute(SetUserFileEncryptionKey(::std::mem::transmute(
            pencryptioncertificate,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKeyEx(
    pencryptioncertificate: *const ENCRYPTION_CERTIFICATE,
    dwcapabilities: u32,
    dwflags: u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserFileEncryptionKeyEx(
                pencryptioncertificate: *const ENCRYPTION_CERTIFICATE,
                dwcapabilities: u32,
                dwflags: u32,
                pvreserved: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(SetUserFileEncryptionKeyEx(
            ::std::mem::transmute(pencryptioncertificate),
            ::std::mem::transmute(dwcapabilities),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeLabelA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lprootpathname: Param0,
    lpvolumename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVolumeLabelA(
                lprootpathname: super::super::Foundation::PSTR,
                lpvolumename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetVolumeLabelA(
            lprootpathname.into_param().abi(),
            lpvolumename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeLabelW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lprootpathname: Param0,
    lpvolumename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVolumeLabelW(
                lprootpathname: super::super::Foundation::PWSTR,
                lpvolumename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetVolumeLabelW(
            lprootpathname.into_param().abi(),
            lpvolumename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeMountPointA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszvolumemountpoint: Param0,
    lpszvolumename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVolumeMountPointA(
                lpszvolumemountpoint: super::super::Foundation::PSTR,
                lpszvolumename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetVolumeMountPointA(
            lpszvolumemountpoint.into_param().abi(),
            lpszvolumename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeMountPointW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszvolumemountpoint: Param0,
    lpszvolumename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVolumeMountPointW(
                lpszvolumemountpoint: super::super::Foundation::PWSTR,
                lpszvolumename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetVolumeMountPointW(
            lpszvolumemountpoint.into_param().abi(),
            lpszvolumename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SinglePhaseReject<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enlistmenthandle: Param0,
    tmvirtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SinglePhaseReject(
                enlistmenthandle: super::super::Foundation::HANDLE,
                tmvirtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SinglePhaseReject(
            enlistmenthandle.into_param().abi(),
            ::std::mem::transmute(tmvirtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SubmitIoRing(
    ioring: *const HIORING__,
    waitoperations: u32,
    milliseconds: u32,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SubmitIoRing(
                ioring: *const HIORING__,
                waitoperations: u32,
                milliseconds: u32,
                submittedentries: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SubmitIoRing(
            ::std::mem::transmute(ioring),
            ::std::mem::transmute(waitoperations),
            ::std::mem::transmute(milliseconds),
            &mut result__,
        )
        .from_abi::<u32>(result__)
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
pub struct TAPEMARK_TYPE(pub i32);
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(1i32);
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(3i32);
pub const TAPE_SETMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(0i32);
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(2i32);
impl ::std::convert::From<i32> for TAPEMARK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAPEMARK_TYPE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct TAPE_INFORMATION_TYPE(pub u32);
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(1u32);
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(0u32);
impl ::std::convert::From<u32> for TAPE_INFORMATION_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAPE_INFORMATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TAPE_INFORMATION_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TAPE_INFORMATION_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TAPE_INFORMATION_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TAPE_INFORMATION_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TAPE_INFORMATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct TAPE_POSITION_METHOD(pub i32);
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(1i32);
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(2i32);
pub const TAPE_REWIND: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(0i32);
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(4i32);
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(6i32);
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(5i32);
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(7i32);
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(9i32);
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(8i32);
impl ::std::convert::From<i32> for TAPE_POSITION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAPE_POSITION_METHOD {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct TAPE_POSITION_TYPE(pub i32);
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(0i32);
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(1i32);
impl ::std::convert::From<i32> for TAPE_POSITION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAPE_POSITION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: *mut ::std::ffi::c_void,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl TRANSACTION_NOTIFICATION {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION")
            .field("TransactionKey", &self.TransactionKey)
            .field("TransactionNotification", &self.TransactionNotification)
            .field("TmVirtualClock", &self.TmVirtualClock)
            .field("ArgumentLength", &self.ArgumentLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionKey == other.TransactionKey
            && self.TransactionNotification == other.TransactionNotification
            && self.TmVirtualClock == other.TmVirtualClock
            && self.ArgumentLength == other.ArgumentLength
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: ::windows::runtime::GUID,
}
impl TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT")
            .field("MarshalCookie", &self.MarshalCookie)
            .field("UOW", &self.UOW)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.MarshalCookie == other.MarshalCookie && self.UOW == other.UOW
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: ::windows::runtime::GUID,
    pub TmIdentity: ::windows::runtime::GUID,
    pub BufferLength: u32,
}
impl TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT")
            .field("PropagationCookie", &self.PropagationCookie)
            .field("UOW", &self.UOW)
            .field("TmIdentity", &self.TmIdentity)
            .field("BufferLength", &self.BufferLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PropagationCookie == other.PropagationCookie
            && self.UOW == other.UOW
            && self.TmIdentity == other.TmIdentity
            && self.BufferLength == other.BufferLength
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: ::windows::runtime::GUID,
    pub UOW: ::windows::runtime::GUID,
}
impl TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT")
            .field("EnlistmentId", &self.EnlistmentId)
            .field("UOW", &self.UOW)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.UOW == other.UOW
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: u32,
}
impl TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT")
            .field("SavepointId", &self.SavepointId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.SavepointId == other.SavepointId
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: ::windows::runtime::GUID,
    pub Flags: u32,
}
impl TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::std::default::Default for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT")
            .field("TmIdentity", &self.TmIdentity)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.TmIdentity == other.TmIdentity && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
unsafe impl ::windows::runtime::Abi for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TRANSACTION_OUTCOME(pub i32);
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(1i32);
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(2i32);
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(3i32);
impl ::std::convert::From<i32> for TRANSACTION_OUTCOME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRANSACTION_OUTCOME {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct TXFS_MINIVERSION(pub u32);
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(0u32);
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65535u32);
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65534u32);
impl ::std::convert::From<u32> for TXFS_MINIVERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXFS_MINIVERSION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TXFS_MINIVERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TXFS_MINIVERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TXFS_MINIVERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TXFS_MINIVERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TXFS_MINIVERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
impl TXF_ID {}
impl ::std::default::Default for TXF_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TXF_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TXF_ID {}
unsafe impl ::windows::runtime::Abi for TXF_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct TXF_ID_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl TXF_ID_0 {}
impl ::std::default::Default for TXF_ID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TXF_ID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TXF_ID_0 {}
unsafe impl ::windows::runtime::Abi for TXF_ID_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::runtime::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl TXF_LOG_RECORD_AFFECTED_FILE {}
impl ::std::default::Default for TXF_LOG_RECORD_AFFECTED_FILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TXF_LOG_RECORD_AFFECTED_FILE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TXF_LOG_RECORD_AFFECTED_FILE {}
unsafe impl ::windows::runtime::Abi for TXF_LOG_RECORD_AFFECTED_FILE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: TXF_LOG_RECORD_TYPE,
    pub RecordLength: u32,
}
impl TXF_LOG_RECORD_BASE {}
impl ::std::default::Default for TXF_LOG_RECORD_BASE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TXF_LOG_RECORD_BASE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TXF_LOG_RECORD_BASE")
            .field("Version", &self.Version)
            .field("RecordType", &self.RecordType)
            .field("RecordLength", &self.RecordLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TXF_LOG_RECORD_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.RecordType == other.RecordType
            && self.RecordLength == other.RecordLength
    }
}
impl ::std::cmp::Eq for TXF_LOG_RECORD_BASE {}
unsafe impl ::windows::runtime::Abi for TXF_LOG_RECORD_BASE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::runtime::GUID,
    pub NewFileSize: i64,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl TXF_LOG_RECORD_TRUNCATE {}
impl ::std::default::Default for TXF_LOG_RECORD_TRUNCATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TXF_LOG_RECORD_TRUNCATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TXF_LOG_RECORD_TRUNCATE {}
unsafe impl ::windows::runtime::Abi for TXF_LOG_RECORD_TRUNCATE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct TXF_LOG_RECORD_TYPE(pub u16);
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(4u16);
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(2u16);
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(1u16);
impl ::std::convert::From<u16> for TXF_LOG_RECORD_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXF_LOG_RECORD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::runtime::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl TXF_LOG_RECORD_WRITE {}
impl ::std::default::Default for TXF_LOG_RECORD_WRITE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TXF_LOG_RECORD_WRITE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TXF_LOG_RECORD_WRITE {}
unsafe impl ::windows::runtime::Abi for TXF_LOG_RECORD_WRITE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateLogArchive(
    pvarchivecontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateLogArchive(
                pvarchivecontext: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TerminateLogArchive(::std::mem::transmute(pvarchivecontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateReadLog(
    pvcursorcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateReadLog(
                pvcursorcontext: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TerminateReadLog(::std::mem::transmute(pvcursorcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn TruncateLog(
    pvmarshal: *const ::std::ffi::c_void,
    plsnend: *const CLS_LSN,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TruncateLog(
                pvmarshal: *const ::std::ffi::c_void,
                plsnend: *const CLS_LSN,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TruncateLog(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(plsnend),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TxfGetThreadMiniVersionForCreate(miniversion: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfGetThreadMiniVersionForCreate(miniversion: *mut u16);
        }
        ::std::mem::transmute(TxfGetThreadMiniVersionForCreate(::std::mem::transmute(
            miniversion,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogCreateFileReadContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, CLS_LSN>,
    Param2: ::windows::runtime::IntoParam<'a, CLS_LSN>,
>(
    logpath: Param0,
    beginninglsn: Param1,
    endinglsn: Param2,
    txffileid: *const TXF_ID,
    txflogcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogCreateFileReadContext(
                logpath: super::super::Foundation::PWSTR,
                beginninglsn: CLS_LSN,
                endinglsn: CLS_LSN,
                txffileid: *const TXF_ID,
                txflogcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogCreateFileReadContext(
            logpath.into_param().abi(),
            beginninglsn.into_param().abi(),
            endinglsn.into_param().abi(),
            ::std::mem::transmute(txffileid),
            ::std::mem::transmute(txflogcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogCreateRangeReadContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, CLS_LSN>,
    Param2: ::windows::runtime::IntoParam<'a, CLS_LSN>,
>(
    logpath: Param0,
    beginninglsn: Param1,
    endinglsn: Param2,
    beginningvirtualclock: *const i64,
    endingvirtualclock: *const i64,
    recordtypemask: u32,
    txflogcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogCreateRangeReadContext(
                logpath: super::super::Foundation::PWSTR,
                beginninglsn: CLS_LSN,
                endinglsn: CLS_LSN,
                beginningvirtualclock: *const i64,
                endingvirtualclock: *const i64,
                recordtypemask: u32,
                txflogcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogCreateRangeReadContext(
            logpath.into_param().abi(),
            beginninglsn.into_param().abi(),
            endinglsn.into_param().abi(),
            ::std::mem::transmute(beginningvirtualclock),
            ::std::mem::transmute(endingvirtualclock),
            ::std::mem::transmute(recordtypemask),
            ::std::mem::transmute(txflogcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogDestroyReadContext(
    txflogcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogDestroyReadContext(
                txflogcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogDestroyReadContext(::std::mem::transmute(
            txflogcontext,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogReadRecords(
    txflogcontext: *const ::std::ffi::c_void,
    bufferlength: u32,
    buffer: *mut ::std::ffi::c_void,
    bytesused: *mut u32,
    recordcount: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogReadRecords(
                txflogcontext: *const ::std::ffi::c_void,
                bufferlength: u32,
                buffer: *mut ::std::ffi::c_void,
                bytesused: *mut u32,
                recordcount: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogReadRecords(
            ::std::mem::transmute(txflogcontext),
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(bytesused),
            ::std::mem::transmute(recordcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogRecordGetFileName(
    recordbuffer: *const ::std::ffi::c_void,
    recordbufferlengthinbytes: u32,
    namebuffer: super::super::Foundation::PWSTR,
    namebufferlengthinbytes: *mut u32,
    txfid: *mut TXF_ID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogRecordGetFileName(
                recordbuffer: *const ::std::ffi::c_void,
                recordbufferlengthinbytes: u32,
                namebuffer: super::super::Foundation::PWSTR,
                namebufferlengthinbytes: *mut u32,
                txfid: *mut TXF_ID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogRecordGetFileName(
            ::std::mem::transmute(recordbuffer),
            ::std::mem::transmute(recordbufferlengthinbytes),
            ::std::mem::transmute(namebuffer),
            ::std::mem::transmute(namebufferlengthinbytes),
            ::std::mem::transmute(txfid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogRecordGetGenericType(
    recordbuffer: *const ::std::ffi::c_void,
    recordbufferlengthinbytes: u32,
    generictype: *mut u32,
    virtualclock: *mut i64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfLogRecordGetGenericType(
                recordbuffer: *const ::std::ffi::c_void,
                recordbufferlengthinbytes: u32,
                generictype: *mut u32,
                virtualclock: *mut i64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfLogRecordGetGenericType(
            ::std::mem::transmute(recordbuffer),
            ::std::mem::transmute(recordbufferlengthinbytes),
            ::std::mem::transmute(generictype),
            ::std::mem::transmute(virtualclock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfReadMetadataInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    txffileid: *mut TXF_ID,
    lastlsn: *mut CLS_LSN,
    transactionstate: *mut u32,
    lockingtransaction: *mut ::windows::runtime::GUID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfReadMetadataInfo(
                filehandle: super::super::Foundation::HANDLE,
                txffileid: *mut TXF_ID,
                lastlsn: *mut CLS_LSN,
                transactionstate: *mut u32,
                lockingtransaction: *mut ::windows::runtime::GUID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TxfReadMetadataInfo(
            filehandle.into_param().abi(),
            ::std::mem::transmute(txffileid),
            ::std::mem::transmute(lastlsn),
            ::std::mem::transmute(transactionstate),
            ::std::mem::transmute(lockingtransaction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TxfSetThreadMiniVersionForCreate(miniversion: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TxfSetThreadMiniVersionForCreate(miniversion: u16);
        }
        ::std::mem::transmute(TxfSetThreadMiniVersionForCreate(::std::mem::transmute(
            miniversion,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    dwfileoffsetlow: u32,
    dwfileoffsethigh: u32,
    nnumberofbytestounlocklow: u32,
    nnumberofbytestounlockhigh: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockFile(
                hfile: super::super::Foundation::HANDLE,
                dwfileoffsetlow: u32,
                dwfileoffsethigh: u32,
                nnumberofbytestounlocklow: u32,
                nnumberofbytestounlockhigh: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnlockFile(
            hfile.into_param().abi(),
            ::std::mem::transmute(dwfileoffsetlow),
            ::std::mem::transmute(dwfileoffsethigh),
            ::std::mem::transmute(nnumberofbytestounlocklow),
            ::std::mem::transmute(nnumberofbytestounlockhigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn UnlockFileEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    dwreserved: u32,
    nnumberofbytestounlocklow: u32,
    nnumberofbytestounlockhigh: u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockFileEx(
                hfile: super::super::Foundation::HANDLE,
                dwreserved: u32,
                nnumberofbytestounlocklow: u32,
                nnumberofbytestounlockhigh: u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnlockFileEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(nnumberofbytestounlocklow),
            ::std::mem::transmute(nnumberofbytestounlockhigh),
            ::std::mem::transmute(lpoverlapped),
        ))
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
pub struct VER_FIND_FILE_FLAGS(pub u32);
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = VER_FIND_FILE_FLAGS(1u32);
impl ::std::convert::From<u32> for VER_FIND_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VER_FIND_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VER_FIND_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VER_FIND_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VER_FIND_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VER_FIND_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VER_FIND_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct VER_FIND_FILE_STATUS(pub u32);
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(1u32);
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(2u32);
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(4u32);
impl ::std::convert::From<u32> for VER_FIND_FILE_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VER_FIND_FILE_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VER_FIND_FILE_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VER_FIND_FILE_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct VER_INSTALL_FILE_FLAGS(pub u32);
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(1u32);
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(2u32);
impl ::std::convert::From<u32> for VER_INSTALL_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VER_INSTALL_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VER_INSTALL_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VER_INSTALL_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VER_INSTALL_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VER_INSTALL_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VER_INSTALL_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct VER_INSTALL_FILE_STATUS(pub u32);
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1u32);
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2u32);
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4u32);
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8u32);
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16u32);
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32u32);
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(64u32);
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(128u32);
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(256u32);
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(512u32);
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1024u32);
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2048u32);
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4096u32);
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8192u32);
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16384u32);
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32768u32);
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(65536u32);
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(131072u32);
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(262144u32);
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(524288u32);
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1048576u32);
impl ::std::convert::From<u32> for VER_INSTALL_FILE_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VER_INSTALL_FILE_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VER_INSTALL_FILE_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VER_INSTALL_FILE_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_ALLOCATE_BC_STREAM_INPUT")
            .field("Version", &self.Version)
            .field("RequestsPerPeriod", &self.RequestsPerPeriod)
            .field("Period", &self.Period)
            .field("RetryFailures", &self.RetryFailures)
            .field("Discardable", &self.Discardable)
            .field("Reserved1", &self.Reserved1)
            .field("LowestByteOffset", &self.LowestByteOffset)
            .field("HighestByteOffset", &self.HighestByteOffset)
            .field("AccessType", &self.AccessType)
            .field("AccessMode", &self.AccessMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.RequestsPerPeriod == other.RequestsPerPeriod
            && self.Period == other.Period
            && self.RetryFailures == other.RetryFailures
            && self.Discardable == other.Discardable
            && self.Reserved1 == other.Reserved1
            && self.LowestByteOffset == other.LowestByteOffset
            && self.HighestByteOffset == other.HighestByteOffset
            && self.AccessType == other.AccessType
            && self.AccessMode == other.AccessMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::std::default::Default for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_ALLOCATE_BC_STREAM_OUTPUT")
            .field("RequestSize", &self.RequestSize)
            .field("NumOutStandingRequests", &self.NumOutStandingRequests)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize
            && self.NumOutStandingRequests == other.NumOutStandingRequests
    }
}
impl ::std::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
impl VOLUME_ALLOCATION_HINT_INPUT {}
impl ::std::default::Default for VOLUME_ALLOCATION_HINT_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_ALLOCATION_HINT_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_ALLOCATION_HINT_INPUT")
            .field("ClusterSize", &self.ClusterSize)
            .field("NumberOfClusters", &self.NumberOfClusters)
            .field("StartingClusterNumber", &self.StartingClusterNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_ALLOCATION_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterSize == other.ClusterSize
            && self.NumberOfClusters == other.NumberOfClusters
            && self.StartingClusterNumber == other.StartingClusterNumber
    }
}
impl ::std::cmp::Eq for VOLUME_ALLOCATION_HINT_INPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_ALLOCATION_HINT_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::std::default::Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_ALLOCATION_HINT_OUTPUT")
            .field("Bitmap", &self.Bitmap)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Bitmap == other.Bitmap
    }
}
impl ::std::cmp::Eq for VOLUME_ALLOCATION_HINT_OUTPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_ALLOCATION_HINT_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl VOLUME_CRITICAL_IO {}
impl ::std::default::Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_CRITICAL_IO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_CRITICAL_IO")
            .field("AccessType", &self.AccessType)
            .field("ExtentsCount", &self.ExtentsCount)
            .field("Extents", &self.Extents)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_CRITICAL_IO {
    fn eq(&self, other: &Self) -> bool {
        self.AccessType == other.AccessType
            && self.ExtentsCount == other.ExtentsCount
            && self.Extents == other.Extents
    }
}
impl ::std::cmp::Eq for VOLUME_CRITICAL_IO {}
unsafe impl ::windows::runtime::Abi for VOLUME_CRITICAL_IO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl VOLUME_FAILOVER_SET {}
impl ::std::default::Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_FAILOVER_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_FAILOVER_SET")
            .field("NumberOfDisks", &self.NumberOfDisks)
            .field("DiskNumbers", &self.DiskNumbers)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_FAILOVER_SET {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfDisks == other.NumberOfDisks && self.DiskNumbers == other.DiskNumbers
    }
}
impl ::std::cmp::Eq for VOLUME_FAILOVER_SET {}
unsafe impl ::windows::runtime::Abi for VOLUME_FAILOVER_SET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::std::default::Default for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_GET_BC_PROPERTIES_INPUT")
            .field("Version", &self.Version)
            .field("Reserved1", &self.Reserved1)
            .field("LowestByteOffset", &self.LowestByteOffset)
            .field("HighestByteOffset", &self.HighestByteOffset)
            .field("AccessType", &self.AccessType)
            .field("AccessMode", &self.AccessMode)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Reserved1 == other.Reserved1
            && self.LowestByteOffset == other.LowestByteOffset
            && self.HighestByteOffset == other.HighestByteOffset
            && self.AccessType == other.AccessType
            && self.AccessMode == other.AccessMode
    }
}
impl ::std::cmp::Eq for VOLUME_GET_BC_PROPERTIES_INPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_GET_BC_PROPERTIES_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::std::default::Default for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_GET_BC_PROPERTIES_OUTPUT")
            .field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod)
            .field("MinimumPeriod", &self.MinimumPeriod)
            .field("MaximumRequestSize", &self.MaximumRequestSize)
            .field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest)
            .field("NumOutStandingRequests", &self.NumOutStandingRequests)
            .field("RequestSize", &self.RequestSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumRequestsPerPeriod == other.MaximumRequestsPerPeriod
            && self.MinimumPeriod == other.MinimumPeriod
            && self.MaximumRequestSize == other.MaximumRequestSize
            && self.EstimatedTimePerRequest == other.EstimatedTimePerRequest
            && self.NumOutStandingRequests == other.NumOutStandingRequests
            && self.RequestSize == other.RequestSize
    }
}
impl ::std::cmp::Eq for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
impl VOLUME_LOGICAL_OFFSET {}
impl ::std::default::Default for VOLUME_LOGICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_LOGICAL_OFFSET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_LOGICAL_OFFSET")
            .field("LogicalOffset", &self.LogicalOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_LOGICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalOffset == other.LogicalOffset
    }
}
impl ::std::cmp::Eq for VOLUME_LOGICAL_OFFSET {}
unsafe impl ::windows::runtime::Abi for VOLUME_LOGICAL_OFFSET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl VOLUME_NUMBER {}
impl ::std::default::Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_NUMBER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_NUMBER")
            .field("VolumeNumber", &self.VolumeNumber)
            .field("VolumeManagerName", &self.VolumeManagerName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeNumber == other.VolumeNumber && self.VolumeManagerName == other.VolumeManagerName
    }
}
impl ::std::cmp::Eq for VOLUME_NUMBER {}
unsafe impl ::windows::runtime::Abi for VOLUME_NUMBER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
impl VOLUME_PHYSICAL_OFFSET {}
impl ::std::default::Default for VOLUME_PHYSICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_PHYSICAL_OFFSET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_PHYSICAL_OFFSET")
            .field("DiskNumber", &self.DiskNumber)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_PHYSICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for VOLUME_PHYSICAL_OFFSET {}
unsafe impl ::windows::runtime::Abi for VOLUME_PHYSICAL_OFFSET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl VOLUME_PHYSICAL_OFFSETS {}
impl ::std::default::Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_PHYSICAL_OFFSETS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_PHYSICAL_OFFSETS")
            .field("NumberOfPhysicalOffsets", &self.NumberOfPhysicalOffsets)
            .field("PhysicalOffset", &self.PhysicalOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_PHYSICAL_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfPhysicalOffsets == other.NumberOfPhysicalOffsets
            && self.PhysicalOffset == other.PhysicalOffset
    }
}
impl ::std::cmp::Eq for VOLUME_PHYSICAL_OFFSETS {}
unsafe impl ::windows::runtime::Abi for VOLUME_PHYSICAL_OFFSETS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
impl VOLUME_READ_PLEX_INPUT {}
impl ::std::default::Default for VOLUME_READ_PLEX_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_READ_PLEX_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_READ_PLEX_INPUT")
            .field("ByteOffset", &self.ByteOffset)
            .field("Length", &self.Length)
            .field("PlexNumber", &self.PlexNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_READ_PLEX_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset
            && self.Length == other.Length
            && self.PlexNumber == other.PlexNumber
    }
}
impl ::std::cmp::Eq for VOLUME_READ_PLEX_INPUT {}
unsafe impl ::windows::runtime::Abi for VOLUME_READ_PLEX_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: super::super::Foundation::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::super::Foundation::BOOLEAN,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_SET_GPT_ATTRIBUTES_INFORMATION")
            .field("GptAttributes", &self.GptAttributes)
            .field("RevertOnClose", &self.RevertOnClose)
            .field(
                "ApplyToAllConnectedVolumes",
                &self.ApplyToAllConnectedVolumes,
            )
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.GptAttributes == other.GptAttributes
            && self.RevertOnClose == other.RevertOnClose
            && self.ApplyToAllConnectedVolumes == other.ApplyToAllConnectedVolumes
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
impl VOLUME_SHRINK_INFO {}
impl ::std::default::Default for VOLUME_SHRINK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VOLUME_SHRINK_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VOLUME_SHRINK_INFO")
            .field("VolumeSize", &self.VolumeSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VOLUME_SHRINK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSize == other.VolumeSize
    }
}
impl ::std::cmp::Eq for VOLUME_SHRINK_INFO {}
unsafe impl ::windows::runtime::Abi for VOLUME_SHRINK_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: VS_FIXEDFILEINFO_FILE_TYPE,
    pub dwFileSubtype: VS_FIXEDFILEINFO_FILE_SUBTYPE,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
impl VS_FIXEDFILEINFO {}
impl ::std::default::Default for VS_FIXEDFILEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VS_FIXEDFILEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VS_FIXEDFILEINFO")
            .field("dwSignature", &self.dwSignature)
            .field("dwStrucVersion", &self.dwStrucVersion)
            .field("dwFileVersionMS", &self.dwFileVersionMS)
            .field("dwFileVersionLS", &self.dwFileVersionLS)
            .field("dwProductVersionMS", &self.dwProductVersionMS)
            .field("dwProductVersionLS", &self.dwProductVersionLS)
            .field("dwFileFlagsMask", &self.dwFileFlagsMask)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwFileOS", &self.dwFileOS)
            .field("dwFileType", &self.dwFileType)
            .field("dwFileSubtype", &self.dwFileSubtype)
            .field("dwFileDateMS", &self.dwFileDateMS)
            .field("dwFileDateLS", &self.dwFileDateLS)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VS_FIXEDFILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature
            && self.dwStrucVersion == other.dwStrucVersion
            && self.dwFileVersionMS == other.dwFileVersionMS
            && self.dwFileVersionLS == other.dwFileVersionLS
            && self.dwProductVersionMS == other.dwProductVersionMS
            && self.dwProductVersionLS == other.dwProductVersionLS
            && self.dwFileFlagsMask == other.dwFileFlagsMask
            && self.dwFileFlags == other.dwFileFlags
            && self.dwFileOS == other.dwFileOS
            && self.dwFileType == other.dwFileType
            && self.dwFileSubtype == other.dwFileSubtype
            && self.dwFileDateMS == other.dwFileDateMS
            && self.dwFileDateLS == other.dwFileDateLS
    }
}
impl ::std::cmp::Eq for VS_FIXEDFILEINFO {}
unsafe impl ::windows::runtime::Abi for VS_FIXEDFILEINFO {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(1u32);
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(2u32);
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(4u32);
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(8u32);
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(16u32);
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(32u32);
impl ::std::convert::From<u32> for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct VS_FIXEDFILEINFO_FILE_OS(pub i32);
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65536i32);
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131072i32);
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196608i32);
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262144i32);
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(327680i32);
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(1i32);
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(2i32);
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(3i32);
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(4i32);
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65537i32);
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65540i32);
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131074i32);
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196611i32);
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262148i32);
impl ::std::convert::From<i32> for VS_FIXEDFILEINFO_FILE_OS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VS_FIXEDFILEINFO_FILE_OS {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(pub i32);
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(0i32);
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(4i32);
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(5i32);
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(6i32);
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(7i32);
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(8i32);
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(9i32);
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(10i32);
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE =
    VS_FIXEDFILEINFO_FILE_SUBTYPE(11i32);
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE =
    VS_FIXEDFILEINFO_FILE_SUBTYPE(12i32);
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
impl ::std::convert::From<i32> for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct VS_FIXEDFILEINFO_FILE_TYPE(pub i32);
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(0i32);
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(1i32);
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(2i32);
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(3i32);
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(4i32);
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(5i32);
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(7i32);
impl ::std::convert::From<i32> for VS_FIXEDFILEINFO_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VS_FIXEDFILEINFO_FILE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VS_USER_DEFINED: u32 = 100u32;
pub const VS_VERSION_INFO: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ValidateLog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszlogfilename: Param0,
    psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pinfobuffer: *mut CLS_INFORMATION,
    pcbbuffer: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidateLog(
                pszlogfilename: super::super::Foundation::PWSTR,
                psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES,
                pinfobuffer: *mut CLS_INFORMATION,
                pcbbuffer: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ValidateLog(
            pszlogfilename.into_param().abi(),
            ::std::mem::transmute(psalogfile),
            ::std::mem::transmute(pinfobuffer),
            ::std::mem::transmute(pcbbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerFindFileA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    uflags: VER_FIND_FILE_FLAGS,
    szfilename: Param1,
    szwindir: Param2,
    szappdir: Param3,
    szcurdir: super::super::Foundation::PSTR,
    pucurdirlen: *mut u32,
    szdestdir: super::super::Foundation::PSTR,
    pudestdirlen: *mut u32,
) -> VER_FIND_FILE_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerFindFileA(
                uflags: VER_FIND_FILE_FLAGS,
                szfilename: super::super::Foundation::PSTR,
                szwindir: super::super::Foundation::PSTR,
                szappdir: super::super::Foundation::PSTR,
                szcurdir: super::super::Foundation::PSTR,
                pucurdirlen: *mut u32,
                szdestdir: super::super::Foundation::PSTR,
                pudestdirlen: *mut u32,
            ) -> VER_FIND_FILE_STATUS;
        }
        ::std::mem::transmute(VerFindFileA(
            ::std::mem::transmute(uflags),
            szfilename.into_param().abi(),
            szwindir.into_param().abi(),
            szappdir.into_param().abi(),
            ::std::mem::transmute(szcurdir),
            ::std::mem::transmute(pucurdirlen),
            ::std::mem::transmute(szdestdir),
            ::std::mem::transmute(pudestdirlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerFindFileW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    uflags: VER_FIND_FILE_FLAGS,
    szfilename: Param1,
    szwindir: Param2,
    szappdir: Param3,
    szcurdir: super::super::Foundation::PWSTR,
    pucurdirlen: *mut u32,
    szdestdir: super::super::Foundation::PWSTR,
    pudestdirlen: *mut u32,
) -> VER_FIND_FILE_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerFindFileW(
                uflags: VER_FIND_FILE_FLAGS,
                szfilename: super::super::Foundation::PWSTR,
                szwindir: super::super::Foundation::PWSTR,
                szappdir: super::super::Foundation::PWSTR,
                szcurdir: super::super::Foundation::PWSTR,
                pucurdirlen: *mut u32,
                szdestdir: super::super::Foundation::PWSTR,
                pudestdirlen: *mut u32,
            ) -> VER_FIND_FILE_STATUS;
        }
        ::std::mem::transmute(VerFindFileW(
            ::std::mem::transmute(uflags),
            szfilename.into_param().abi(),
            szwindir.into_param().abi(),
            szappdir.into_param().abi(),
            ::std::mem::transmute(szcurdir),
            ::std::mem::transmute(pucurdirlen),
            ::std::mem::transmute(szdestdir),
            ::std::mem::transmute(pudestdirlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerInstallFileA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    uflags: VER_INSTALL_FILE_FLAGS,
    szsrcfilename: Param1,
    szdestfilename: Param2,
    szsrcdir: Param3,
    szdestdir: Param4,
    szcurdir: Param5,
    sztmpfile: super::super::Foundation::PSTR,
    putmpfilelen: *mut u32,
) -> VER_INSTALL_FILE_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerInstallFileA(
                uflags: VER_INSTALL_FILE_FLAGS,
                szsrcfilename: super::super::Foundation::PSTR,
                szdestfilename: super::super::Foundation::PSTR,
                szsrcdir: super::super::Foundation::PSTR,
                szdestdir: super::super::Foundation::PSTR,
                szcurdir: super::super::Foundation::PSTR,
                sztmpfile: super::super::Foundation::PSTR,
                putmpfilelen: *mut u32,
            ) -> VER_INSTALL_FILE_STATUS;
        }
        ::std::mem::transmute(VerInstallFileA(
            ::std::mem::transmute(uflags),
            szsrcfilename.into_param().abi(),
            szdestfilename.into_param().abi(),
            szsrcdir.into_param().abi(),
            szdestdir.into_param().abi(),
            szcurdir.into_param().abi(),
            ::std::mem::transmute(sztmpfile),
            ::std::mem::transmute(putmpfilelen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerInstallFileW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    uflags: VER_INSTALL_FILE_FLAGS,
    szsrcfilename: Param1,
    szdestfilename: Param2,
    szsrcdir: Param3,
    szdestdir: Param4,
    szcurdir: Param5,
    sztmpfile: super::super::Foundation::PWSTR,
    putmpfilelen: *mut u32,
) -> VER_INSTALL_FILE_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerInstallFileW(
                uflags: VER_INSTALL_FILE_FLAGS,
                szsrcfilename: super::super::Foundation::PWSTR,
                szdestfilename: super::super::Foundation::PWSTR,
                szsrcdir: super::super::Foundation::PWSTR,
                szdestdir: super::super::Foundation::PWSTR,
                szcurdir: super::super::Foundation::PWSTR,
                sztmpfile: super::super::Foundation::PWSTR,
                putmpfilelen: *mut u32,
            ) -> VER_INSTALL_FILE_STATUS;
        }
        ::std::mem::transmute(VerInstallFileW(
            ::std::mem::transmute(uflags),
            szsrcfilename.into_param().abi(),
            szdestfilename.into_param().abi(),
            szsrcdir.into_param().abi(),
            szdestdir.into_param().abi(),
            szcurdir.into_param().abi(),
            ::std::mem::transmute(sztmpfile),
            ::std::mem::transmute(putmpfilelen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerLanguageNameA(
    wlang: u32,
    szlang: super::super::Foundation::PSTR,
    cchlang: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerLanguageNameA(
                wlang: u32,
                szlang: super::super::Foundation::PSTR,
                cchlang: u32,
            ) -> u32;
        }
        ::std::mem::transmute(VerLanguageNameA(
            ::std::mem::transmute(wlang),
            ::std::mem::transmute(szlang),
            ::std::mem::transmute(cchlang),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerLanguageNameW(
    wlang: u32,
    szlang: super::super::Foundation::PWSTR,
    cchlang: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerLanguageNameW(
                wlang: u32,
                szlang: super::super::Foundation::PWSTR,
                cchlang: u32,
            ) -> u32;
        }
        ::std::mem::transmute(VerLanguageNameW(
            ::std::mem::transmute(wlang),
            ::std::mem::transmute(szlang),
            ::std::mem::transmute(cchlang),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerQueryValueA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pblock: *const ::std::ffi::c_void,
    lpsubblock: Param1,
    lplpbuffer: *mut *mut ::std::ffi::c_void,
    pulen: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerQueryValueA(
                pblock: *const ::std::ffi::c_void,
                lpsubblock: super::super::Foundation::PSTR,
                lplpbuffer: *mut *mut ::std::ffi::c_void,
                pulen: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VerQueryValueA(
            ::std::mem::transmute(pblock),
            lpsubblock.into_param().abi(),
            ::std::mem::transmute(lplpbuffer),
            ::std::mem::transmute(pulen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerQueryValueW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pblock: *const ::std::ffi::c_void,
    lpsubblock: Param1,
    lplpbuffer: *mut *mut ::std::ffi::c_void,
    pulen: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerQueryValueW(
                pblock: *const ::std::ffi::c_void,
                lpsubblock: super::super::Foundation::PWSTR,
                lplpbuffer: *mut *mut ::std::ffi::c_void,
                pulen: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VerQueryValueW(
            ::std::mem::transmute(pblock),
            lpsubblock.into_param().abi(),
            ::std::mem::transmute(lplpbuffer),
            ::std::mem::transmute(pulen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIM_ENTRY_INFO {
    pub WimEntryInfoSize: u32,
    pub WimType: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows::runtime::GUID,
    pub WimPath: super::super::Foundation::PWSTR,
    pub WimIndex: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WIM_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIM_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIM_ENTRY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIM_ENTRY_INFO")
            .field("WimEntryInfoSize", &self.WimEntryInfoSize)
            .field("WimType", &self.WimType)
            .field("DataSourceId", &self.DataSourceId)
            .field("WimGuid", &self.WimGuid)
            .field("WimPath", &self.WimPath)
            .field("WimIndex", &self.WimIndex)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIM_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.WimEntryInfoSize == other.WimEntryInfoSize
            && self.WimType == other.WimType
            && self.DataSourceId == other.DataSourceId
            && self.WimGuid == other.WimGuid
            && self.WimPath == other.WimPath
            && self.WimIndex == other.WimIndex
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIM_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIM_ENTRY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIM_EXTERNAL_FILE_INFO {
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
    pub Flags: u32,
}
impl WIM_EXTERNAL_FILE_INFO {}
impl ::std::default::Default for WIM_EXTERNAL_FILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIM_EXTERNAL_FILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIM_EXTERNAL_FILE_INFO")
            .field("DataSourceId", &self.DataSourceId)
            .field("ResourceHash", &self.ResourceHash)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIM_EXTERNAL_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId
            && self.ResourceHash == other.ResourceHash
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for WIM_EXTERNAL_FILE_INFO {}
unsafe impl ::windows::runtime::Abi for WIM_EXTERNAL_FILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN32_FILE_ATTRIBUTE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN32_FILE_ATTRIBUTE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_FILE_ATTRIBUTE_DATA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN32_FILE_ATTRIBUTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN32_FILE_ATTRIBUTE_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [super::super::Foundation::CHAR; 260],
    pub cAlternateFileName: [super::super::Foundation::CHAR; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN32_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN32_FIND_DATAA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_FIND_DATAA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN32_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.dwReserved0 == other.dwReserved0
            && self.dwReserved1 == other.dwReserved1
            && self.cFileName == other.cFileName
            && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN32_FIND_DATAA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN32_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN32_FIND_DATAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_FIND_DATAW")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN32_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.dwReserved0 == other.dwReserved0
            && self.dwReserved1 == other.dwReserved1
            && self.cFileName == other.cFileName
            && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN32_FIND_DATAW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl WIN32_FIND_STREAM_DATA {}
impl ::std::default::Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN32_FIND_STREAM_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_FIND_STREAM_DATA")
            .field("StreamSize", &self.StreamSize)
            .field("cStreamName", &self.cStreamName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN32_FIND_STREAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.StreamSize == other.StreamSize && self.cStreamName == other.cStreamName
    }
}
impl ::std::cmp::Eq for WIN32_FIND_STREAM_DATA {}
unsafe impl ::windows::runtime::Abi for WIN32_FIND_STREAM_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: WIN_STREAM_ID,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl WIN32_STREAM_ID {}
impl ::std::default::Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN32_STREAM_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_STREAM_ID")
            .field("dwStreamId", &self.dwStreamId)
            .field("dwStreamAttributes", &self.dwStreamAttributes)
            .field("Size", &self.Size)
            .field("dwStreamNameSize", &self.dwStreamNameSize)
            .field("cStreamName", &self.cStreamName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN32_STREAM_ID {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamId == other.dwStreamId
            && self.dwStreamAttributes == other.dwStreamAttributes
            && self.Size == other.Size
            && self.dwStreamNameSize == other.dwStreamNameSize
            && self.cStreamName == other.cStreamName
    }
}
impl ::std::cmp::Eq for WIN32_STREAM_ID {}
unsafe impl ::windows::runtime::Abi for WIN32_STREAM_ID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WIN_STREAM_ID(pub u32);
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(4u32);
pub const BACKUP_DATA: WIN_STREAM_ID = WIN_STREAM_ID(1u32);
pub const BACKUP_EA_DATA: WIN_STREAM_ID = WIN_STREAM_ID(2u32);
pub const BACKUP_LINK: WIN_STREAM_ID = WIN_STREAM_ID(5u32);
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = WIN_STREAM_ID(7u32);
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(6u32);
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(8u32);
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(3u32);
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = WIN_STREAM_ID(9u32);
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = WIN_STREAM_ID(10u32);
impl ::std::convert::From<u32> for WIN_STREAM_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WIN_STREAM_ID {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WIN_STREAM_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WIN_STREAM_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WIN_STREAM_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WIN_STREAM_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WIN_STREAM_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WOF_FILE_COMPRESSION_INFO_V0 {
    pub Algorithm: u32,
}
impl WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::std::default::Default for WOF_FILE_COMPRESSION_INFO_V0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WOF_FILE_COMPRESSION_INFO_V0")
            .field("Algorithm", &self.Algorithm)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm
    }
}
impl ::std::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V0 {}
unsafe impl ::windows::runtime::Abi for WOF_FILE_COMPRESSION_INFO_V0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WOF_FILE_COMPRESSION_INFO_V1 {
    pub Algorithm: u32,
    pub Flags: u32,
}
impl WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::std::default::Default for WOF_FILE_COMPRESSION_INFO_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WOF_FILE_COMPRESSION_INFO_V1")
            .field("Algorithm", &self.Algorithm)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V1 {}
unsafe impl ::windows::runtime::Abi for WOF_FILE_COMPRESSION_INFO_V1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WOF_PROVIDER_FILE: u32 = 2u32;
pub const WOF_PROVIDER_WIM: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofEnumEntries<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    provider: u32,
    enumproc: ::std::option::Option<WofEnumEntryProc>,
    userdata: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofEnumEntries(
                volumename: super::super::Foundation::PWSTR,
                provider: u32,
                enumproc: ::windows::runtime::RawPtr,
                userdata: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WofEnumEntries(
            volumename.into_param().abi(),
            ::std::mem::transmute(provider),
            ::std::mem::transmute(enumproc),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumEntryProc = unsafe extern "system" fn(
    entryinfo: *const ::std::ffi::c_void,
    userdata: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumFilesProc = unsafe extern "system" fn(
    filepath: super::super::Foundation::PWSTR,
    externalfileinfo: *const ::std::ffi::c_void,
    userdata: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofFileEnumFiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    algorithm: u32,
    enumproc: ::std::option::Option<WofEnumFilesProc>,
    userdata: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofFileEnumFiles(
                volumename: super::super::Foundation::PWSTR,
                algorithm: u32,
                enumproc: ::windows::runtime::RawPtr,
                userdata: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WofFileEnumFiles(
            volumename.into_param().abi(),
            ::std::mem::transmute(algorithm),
            ::std::mem::transmute(enumproc),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofGetDriverVersion<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    fileorvolumehandle: Param0,
    provider: u32,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofGetDriverVersion(
                fileorvolumehandle: super::super::Foundation::HANDLE,
                provider: u32,
                wofversion: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WofGetDriverVersion(
            fileorvolumehandle.into_param().abi(),
            ::std::mem::transmute(provider),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofIsExternalFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filepath: Param0,
    isexternalfile: *mut super::super::Foundation::BOOL,
    provider: *mut u32,
    externalfileinfo: *mut ::std::ffi::c_void,
    bufferlength: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofIsExternalFile(
                filepath: super::super::Foundation::PWSTR,
                isexternalfile: *mut super::super::Foundation::BOOL,
                provider: *mut u32,
                externalfileinfo: *mut ::std::ffi::c_void,
                bufferlength: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WofIsExternalFile(
            filepath.into_param().abi(),
            ::std::mem::transmute(isexternalfile),
            ::std::mem::transmute(provider),
            ::std::mem::transmute(externalfileinfo),
            ::std::mem::transmute(bufferlength),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofSetFileDataLocation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    provider: u32,
    externalfileinfo: *const ::std::ffi::c_void,
    length: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofSetFileDataLocation(
                filehandle: super::super::Foundation::HANDLE,
                provider: u32,
                externalfileinfo: *const ::std::ffi::c_void,
                length: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WofSetFileDataLocation(
            filehandle.into_param().abi(),
            ::std::mem::transmute(provider),
            ::std::mem::transmute(externalfileinfo),
            ::std::mem::transmute(length),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofShouldCompressBinaries<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volume: Param0,
    algorithm: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofShouldCompressBinaries(
                volume: super::super::Foundation::PWSTR,
                algorithm: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WofShouldCompressBinaries(
            volume.into_param().abi(),
            ::std::mem::transmute(algorithm),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimAddEntry<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    wimpath: Param1,
    wimtype: u32,
    wimindex: u32,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofWimAddEntry(
                volumename: super::super::Foundation::PWSTR,
                wimpath: super::super::Foundation::PWSTR,
                wimtype: u32,
                wimindex: u32,
                datasourceid: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WofWimAddEntry(
            volumename.into_param().abi(),
            wimpath.into_param().abi(),
            ::std::mem::transmute(wimtype),
            ::std::mem::transmute(wimindex),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimEnumFiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    datasourceid: i64,
    enumproc: ::std::option::Option<WofEnumFilesProc>,
    userdata: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofWimEnumFiles(
                volumename: super::super::Foundation::PWSTR,
                datasourceid: i64,
                enumproc: ::windows::runtime::RawPtr,
                userdata: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WofWimEnumFiles(
            volumename.into_param().abi(),
            ::std::mem::transmute(datasourceid),
            ::std::mem::transmute(enumproc),
            ::std::mem::transmute(userdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimRemoveEntry<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    datasourceid: i64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofWimRemoveEntry(
                volumename: super::super::Foundation::PWSTR,
                datasourceid: i64,
            ) -> ::windows::runtime::HRESULT;
        }
        WofWimRemoveEntry(
            volumename.into_param().abi(),
            ::std::mem::transmute(datasourceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimSuspendEntry<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    datasourceid: i64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofWimSuspendEntry(
                volumename: super::super::Foundation::PWSTR,
                datasourceid: i64,
            ) -> ::windows::runtime::HRESULT;
        }
        WofWimSuspendEntry(
            volumename.into_param().abi(),
            ::std::mem::transmute(datasourceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimUpdateEntry<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    volumename: Param0,
    datasourceid: i64,
    newwimpath: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WofWimUpdateEntry(
                volumename: super::super::Foundation::PWSTR,
                datasourceid: i64,
                newwimpath: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        WofWimUpdateEntry(
            volumename.into_param().abi(),
            ::std::mem::transmute(datasourceid),
            newwimpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64DisableWow64FsRedirection(
    oldvalue: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64DisableWow64FsRedirection(
                oldvalue: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Wow64DisableWow64FsRedirection(::std::mem::transmute(
            oldvalue,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64EnableWow64FsRedirection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    wow64fsenableredirection: Param0,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64EnableWow64FsRedirection(
                wow64fsenableredirection: super::super::Foundation::BOOLEAN,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(Wow64EnableWow64FsRedirection(
            wow64fsenableredirection.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64RevertWow64FsRedirection(
    olvalue: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64RevertWow64FsRedirection(
                olvalue: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Wow64RevertWow64FsRedirection(::std::mem::transmute(
            olvalue,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteEncryptedFileRaw(
    pfimportcallback: ::std::option::Option<PFE_IMPORT_FUNC>,
    pvcallbackcontext: *const ::std::ffi::c_void,
    pvcontext: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteEncryptedFileRaw(
                pfimportcallback: ::windows::runtime::RawPtr,
                pvcallbackcontext: *const ::std::ffi::c_void,
                pvcontext: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(WriteEncryptedFileRaw(
            ::std::mem::transmute(pfimportcallback),
            ::std::mem::transmute(pvcallbackcontext),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpbuffer: *const ::std::ffi::c_void,
    nnumberofbytestowrite: u32,
    lpnumberofbyteswritten: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteFile(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *const ::std::ffi::c_void,
                nnumberofbytestowrite: u32,
                lpnumberofbyteswritten: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteFile(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestowrite),
            ::std::mem::transmute(lpnumberofbyteswritten),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFileEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpbuffer: *const ::std::ffi::c_void,
    nnumberofbytestowrite: u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
    lpcompletionroutine: ::std::option::Option<
        super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    >,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteFileEx(
                hfile: super::super::Foundation::HANDLE,
                lpbuffer: *const ::std::ffi::c_void,
                nnumberofbytestowrite: u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
                lpcompletionroutine: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteFileEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofbytestowrite),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpcompletionroutine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFileGather<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    asegmentarray: *const FILE_SEGMENT_ELEMENT,
    nnumberofbytestowrite: u32,
    lpreserved: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteFileGather(
                hfile: super::super::Foundation::HANDLE,
                asegmentarray: *const FILE_SEGMENT_ELEMENT,
                nnumberofbytestowrite: u32,
                lpreserved: *mut u32,
                lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteFileGather(
            hfile.into_param().abi(),
            ::std::mem::transmute(asegmentarray),
            ::std::mem::transmute(nnumberofbytestowrite),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteLogRestartArea(
    pvmarshal: *mut ::std::ffi::c_void,
    pvrestartbuffer: *mut ::std::ffi::c_void,
    cbrestartbuffer: u32,
    plsnbase: *mut CLS_LSN,
    fflags: CLFS_FLAG,
    pcbwritten: *mut u32,
    plsnnext: *mut CLS_LSN,
    poverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteLogRestartArea(
                pvmarshal: *mut ::std::ffi::c_void,
                pvrestartbuffer: *mut ::std::ffi::c_void,
                cbrestartbuffer: u32,
                plsnbase: *mut CLS_LSN,
                fflags: CLFS_FLAG,
                pcbwritten: *mut u32,
                plsnnext: *mut CLS_LSN,
                poverlapped: *mut super::super::System::IO::OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteLogRestartArea(
            ::std::mem::transmute(pvmarshal),
            ::std::mem::transmute(pvrestartbuffer),
            ::std::mem::transmute(cbrestartbuffer),
            ::std::mem::transmute(plsnbase),
            ::std::mem::transmute(fflags),
            ::std::mem::transmute(pcbwritten),
            ::std::mem::transmute(plsnnext),
            ::std::mem::transmute(poverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteTapemark<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdevice: Param0,
    dwtapemarktype: TAPEMARK_TYPE,
    dwtapemarkcount: u32,
    bimmediate: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteTapemark(
                hdevice: super::super::Foundation::HANDLE,
                dwtapemarktype: TAPEMARK_TYPE,
                dwtapemarkcount: u32,
                bimmediate: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WriteTapemark(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwtapemarktype),
            ::std::mem::transmute(dwtapemarkcount),
            bimmediate.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
