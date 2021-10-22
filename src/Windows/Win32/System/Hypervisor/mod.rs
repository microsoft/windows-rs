#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub unsafe fn ApplyGuestMemoryFix(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    virtualaddress: u64,
    fixbuffer: *const ::std::ffi::c_void,
    fixbuffersize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ApplyGuestMemoryFix(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                virtualaddress: u64,
                fixbuffer: *const ::std::ffi::c_void,
                fixbuffersize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ApplyGuestMemoryFix(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(fixbuffer),
            ::std::mem::transmute(fixbuffersize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ApplyPendingSavedStateFileReplayLog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    vmrsfile: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ApplyPendingSavedStateFileReplayLog(
                vmrsfile: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        ApplyPendingSavedStateFileReplayLog(vmrsfile.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CallStackUnwind(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    imageinfo: *const MODULE_INFO,
    imageinfocount: u32,
    framecount: u32,
    callstack: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn CallStackUnwind(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                imageinfo: *const MODULE_INFO,
                imageinfocount: u32,
                framecount: u32,
                callstack: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        CallStackUnwind(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(imageinfo),
            ::std::mem::transmute(imageinfocount),
            ::std::mem::transmute(framecount),
            ::std::mem::transmute(callstack),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOS_IMAGE_INFO {
    pub PdbName: super::super::Foundation::PSTR,
    pub ImageBaseAddress: u64,
    pub ImageSize: u32,
    pub Timestamp: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DOS_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOS_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOS_IMAGE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOS_IMAGE_INFO")
            .field("PdbName", &self.PdbName)
            .field("ImageBaseAddress", &self.ImageBaseAddress)
            .field("ImageSize", &self.ImageSize)
            .field("Timestamp", &self.Timestamp)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOS_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PdbName == other.PdbName
            && self.ImageBaseAddress == other.ImageBaseAddress
            && self.ImageSize == other.ImageSize
            && self.Timestamp == other.Timestamp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOS_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOS_IMAGE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type FOUND_IMAGE_CALLBACK = unsafe extern "system" fn(
    context: *const ::std::ffi::c_void,
    imageinfo: *const DOS_IMAGE_INFO,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FindSavedStateSymbolFieldInType<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    typename: Param2,
    fieldname: Param3,
    offset: *mut u32,
    found: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn FindSavedStateSymbolFieldInType(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                typename: super::super::Foundation::PSTR,
                fieldname: super::super::Foundation::PWSTR,
                offset: *mut u32,
                found: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        FindSavedStateSymbolFieldInType(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            typename.into_param().abi(),
            fieldname.into_param().abi(),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(found),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ForceActiveVirtualTrustLevel(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    virtualtrustlevel: u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ForceActiveVirtualTrustLevel(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                virtualtrustlevel: u8,
            ) -> ::windows::runtime::HRESULT;
        }
        ForceActiveVirtualTrustLevel(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(virtualtrustlevel),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ForceArchitecture(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    architecture: VIRTUAL_PROCESSOR_ARCH,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ForceArchitecture(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                architecture: VIRTUAL_PROCESSOR_ARCH,
            ) -> ::windows::runtime::HRESULT;
        }
        ForceArchitecture(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(architecture),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ForceNestedHostMode<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    hostmode: Param2,
    oldmode: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ForceNestedHostMode(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                hostmode: super::super::Foundation::BOOL,
                oldmode: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        ForceNestedHostMode(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            hostmode.into_param().abi(),
            ::std::mem::transmute(oldmode),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ForcePagingMode(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    pagingmode: PAGING_MODE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ForcePagingMode(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                pagingmode: PAGING_MODE,
            ) -> ::windows::runtime::HRESULT;
        }
        ForcePagingMode(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(pagingmode),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GPA_MEMORY_CHUNK {
    pub GuestPhysicalStartPageIndex: u64,
    pub PageCount: u64,
}
impl GPA_MEMORY_CHUNK {}
impl ::std::default::Default for GPA_MEMORY_CHUNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GPA_MEMORY_CHUNK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GPA_MEMORY_CHUNK")
            .field(
                "GuestPhysicalStartPageIndex",
                &self.GuestPhysicalStartPageIndex,
            )
            .field("PageCount", &self.PageCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GPA_MEMORY_CHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.GuestPhysicalStartPageIndex == other.GuestPhysicalStartPageIndex
            && self.PageCount == other.PageCount
    }
}
impl ::std::cmp::Eq for GPA_MEMORY_CHUNK {}
unsafe impl ::windows::runtime::Abi for GPA_MEMORY_CHUNK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union GUEST_OS_INFO {
    pub AsUINT64: u64,
    pub ClosedSource: GUEST_OS_INFO_0,
    pub OpenSource: GUEST_OS_INFO_1,
}
impl GUEST_OS_INFO {}
impl ::std::default::Default for GUEST_OS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GUEST_OS_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GUEST_OS_INFO {}
unsafe impl ::windows::runtime::Abi for GUEST_OS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GUEST_OS_INFO_0 {
    pub _bitfield: u64,
}
impl GUEST_OS_INFO_0 {}
impl ::std::default::Default for GUEST_OS_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GUEST_OS_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ClosedSource_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GUEST_OS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for GUEST_OS_INFO_0 {}
unsafe impl ::windows::runtime::Abi for GUEST_OS_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GUEST_OS_INFO_1 {
    pub _bitfield: u64,
}
impl GUEST_OS_INFO_1 {}
impl ::std::default::Default for GUEST_OS_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GUEST_OS_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_OpenSource_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GUEST_OS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for GUEST_OS_INFO_1 {}
unsafe impl ::windows::runtime::Abi for GUEST_OS_INFO_1 {
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
pub struct GUEST_OS_MICROSOFT_IDS(pub i32);
pub const GuestOsMicrosoftUndefined: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(0i32);
pub const GuestOsMicrosoftMSDOS: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(1i32);
pub const GuestOsMicrosoftWindows3x: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(2i32);
pub const GuestOsMicrosoftWindows9x: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(3i32);
pub const GuestOsMicrosoftWindowsNT: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(4i32);
pub const GuestOsMicrosoftWindowsCE: GUEST_OS_MICROSOFT_IDS = GUEST_OS_MICROSOFT_IDS(5i32);
impl ::std::convert::From<i32> for GUEST_OS_MICROSOFT_IDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GUEST_OS_MICROSOFT_IDS {
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
pub struct GUEST_OS_OPENSOURCE_IDS(pub i32);
pub const GuestOsOpenSourceUndefined: GUEST_OS_OPENSOURCE_IDS = GUEST_OS_OPENSOURCE_IDS(0i32);
pub const GuestOsOpenSourceLinux: GUEST_OS_OPENSOURCE_IDS = GUEST_OS_OPENSOURCE_IDS(1i32);
pub const GuestOsOpenSourceFreeBSD: GUEST_OS_OPENSOURCE_IDS = GUEST_OS_OPENSOURCE_IDS(2i32);
pub const GuestOsOpenSourceXen: GUEST_OS_OPENSOURCE_IDS = GUEST_OS_OPENSOURCE_IDS(3i32);
pub const GuestOsOpenSourceIllumos: GUEST_OS_OPENSOURCE_IDS = GUEST_OS_OPENSOURCE_IDS(4i32);
impl ::std::convert::From<i32> for GUEST_OS_OPENSOURCE_IDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GUEST_OS_OPENSOURCE_IDS {
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
pub struct GUEST_OS_VENDOR(pub i32);
pub const GuestOsVendorUndefined: GUEST_OS_VENDOR = GUEST_OS_VENDOR(0i32);
pub const GuestOsVendorMicrosoft: GUEST_OS_VENDOR = GUEST_OS_VENDOR(1i32);
pub const GuestOsVendorHPE: GUEST_OS_VENDOR = GUEST_OS_VENDOR(2i32);
pub const GuestOsVendorLANCOM: GUEST_OS_VENDOR = GUEST_OS_VENDOR(512i32);
impl ::std::convert::From<i32> for GUEST_OS_VENDOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GUEST_OS_VENDOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK =
    unsafe extern "system" fn(infomessage: super::super::Foundation::PSTR);
pub const GUID_DEVINTERFACE_VM_GENCOUNTER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1072875819,
        26008,
        20064,
        [142, 28, 12, 207, 73, 39, 227, 25],
    );
pub unsafe fn GetActiveVirtualTrustLevel(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    virtualtrustlevel: *mut u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetActiveVirtualTrustLevel(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                virtualtrustlevel: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        GetActiveVirtualTrustLevel(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(virtualtrustlevel),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetArchitecture(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    architecture: *mut VIRTUAL_PROCESSOR_ARCH,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetArchitecture(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                architecture: *mut VIRTUAL_PROCESSOR_ARCH,
            ) -> ::windows::runtime::HRESULT;
        }
        GetArchitecture(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(architecture),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetEnabledVirtualTrustLevels(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    virtualtrustlevels: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetEnabledVirtualTrustLevels(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                virtualtrustlevels: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        GetEnabledVirtualTrustLevels(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(virtualtrustlevels),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetGuestEnabledVirtualTrustLevels(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    virtualtrustlevels: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetGuestEnabledVirtualTrustLevels(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                virtualtrustlevels: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        GetGuestEnabledVirtualTrustLevels(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(virtualtrustlevels),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetGuestOsInfo(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    virtualtrustlevel: u8,
    guestosinfo: *mut GUEST_OS_INFO,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetGuestOsInfo(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                virtualtrustlevel: u8,
                guestosinfo: *mut GUEST_OS_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        GetGuestOsInfo(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(virtualtrustlevel),
            ::std::mem::transmute(guestosinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetGuestPhysicalMemoryChunks(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    memorychunkpagesize: *mut u64,
    memorychunks: *mut GPA_MEMORY_CHUNK,
    memorychunkcount: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetGuestPhysicalMemoryChunks(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                memorychunkpagesize: *mut u64,
                memorychunks: *mut GPA_MEMORY_CHUNK,
                memorychunkcount: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        GetGuestPhysicalMemoryChunks(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(memorychunkpagesize),
            ::std::mem::transmute(memorychunks),
            ::std::mem::transmute(memorychunkcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetGuestRawSavedMemorySize(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    guestrawsavedmemorysize: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetGuestRawSavedMemorySize(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                guestrawsavedmemorysize: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        GetGuestRawSavedMemorySize(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(guestrawsavedmemorysize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetMemoryBlockCacheLimit(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    memoryblockcachelimit: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetMemoryBlockCacheLimit(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                memoryblockcachelimit: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        GetMemoryBlockCacheLimit(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(memoryblockcachelimit),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNestedVirtualizationMode(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    enabled: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetNestedVirtualizationMode(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                enabled: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        GetNestedVirtualizationMode(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(enabled),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetPagingMode(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    pagingmode: *mut PAGING_MODE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetPagingMode(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                pagingmode: *mut PAGING_MODE,
            ) -> ::windows::runtime::HRESULT;
        }
        GetPagingMode(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(pagingmode),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetRegisterValue(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    registerid: u32,
    registervalue: *mut VIRTUAL_PROCESSOR_REGISTER,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetRegisterValue(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                registerid: u32,
                registervalue: *mut VIRTUAL_PROCESSOR_REGISTER,
            ) -> ::windows::runtime::HRESULT;
        }
        GetRegisterValue(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(registerid),
            ::std::mem::transmute(registervalue),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSavedStateSymbolFieldInfo<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    typename: Param2,
    typefieldinfomap: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetSavedStateSymbolFieldInfo(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                typename: super::super::Foundation::PSTR,
                typefieldinfomap: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        GetSavedStateSymbolFieldInfo(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            typename.into_param().abi(),
            ::std::mem::transmute(typefieldinfomap),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSavedStateSymbolProviderHandle(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetSavedStateSymbolProviderHandle(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(GetSavedStateSymbolProviderHandle(::std::mem::transmute(
            vmsavedstatedumphandle,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSavedStateSymbolTypeSize<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    typename: Param2,
    size: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetSavedStateSymbolTypeSize(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                typename: super::super::Foundation::PSTR,
                size: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        GetSavedStateSymbolTypeSize(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            typename.into_param().abi(),
            ::std::mem::transmute(size),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetVpCount(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GetVpCount(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        GetVpCount(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GuestPhysicalAddressToRawSavedMemoryOffset(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    physicaladdress: u64,
    rawsavedmemoryoffset: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GuestPhysicalAddressToRawSavedMemoryOffset(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                physicaladdress: u64,
                rawsavedmemoryoffset: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        GuestPhysicalAddressToRawSavedMemoryOffset(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(physicaladdress),
            ::std::mem::transmute(rawsavedmemoryoffset),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GuestVirtualAddressToPhysicalAddress(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    virtualaddress: u64,
    physicaladdress: *mut u64,
    unmappedregionsize: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn GuestVirtualAddressToPhysicalAddress(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                virtualaddress: u64,
                physicaladdress: *mut u64,
                unmappedregionsize: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        GuestVirtualAddressToPhysicalAddress(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(physicaladdress),
            ::std::mem::transmute(unmappedregionsize),
        )
        .ok()
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
pub struct HDV_DEVICE_TYPE(pub i32);
pub const HdvDeviceTypeUndefined: HDV_DEVICE_TYPE = HDV_DEVICE_TYPE(0i32);
pub const HdvDeviceTypePCI: HDV_DEVICE_TYPE = HDV_DEVICE_TYPE(1i32);
impl ::std::convert::From<i32> for HDV_DEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDV_DEVICE_TYPE {
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
pub struct HDV_DOORBELL_FLAGS(pub i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_ANY: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(0i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(1i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_WORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(2i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(3i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD: HDV_DOORBELL_FLAGS = HDV_DOORBELL_FLAGS(4i32);
pub const HDV_DOORBELL_FLAG_TRIGGER_ANY_VALUE: HDV_DOORBELL_FLAGS =
    HDV_DOORBELL_FLAGS(-2147483648i32);
impl ::std::convert::From<i32> for HDV_DOORBELL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDV_DOORBELL_FLAGS {
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
pub struct HDV_MMIO_MAPPING_FLAGS(pub u32);
pub const HdvMmioMappingFlagNone: HDV_MMIO_MAPPING_FLAGS = HDV_MMIO_MAPPING_FLAGS(0u32);
pub const HdvMmioMappingFlagWriteable: HDV_MMIO_MAPPING_FLAGS = HDV_MMIO_MAPPING_FLAGS(1u32);
pub const HdvMmioMappingFlagExecutable: HDV_MMIO_MAPPING_FLAGS = HDV_MMIO_MAPPING_FLAGS(2u32);
impl ::std::convert::From<u32> for HDV_MMIO_MAPPING_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDV_MMIO_MAPPING_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HDV_MMIO_MAPPING_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HDV_MMIO_MAPPING_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HDV_PCI_BAR_COUNT: u32 = 6u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HDV_PCI_BAR_SELECTOR(pub i32);
pub const HDV_PCI_BAR0: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(0i32);
pub const HDV_PCI_BAR1: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(1i32);
pub const HDV_PCI_BAR2: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(2i32);
pub const HDV_PCI_BAR3: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(3i32);
pub const HDV_PCI_BAR4: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(4i32);
pub const HDV_PCI_BAR5: HDV_PCI_BAR_SELECTOR = HDV_PCI_BAR_SELECTOR(5i32);
impl ::std::convert::From<i32> for HDV_PCI_BAR_SELECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDV_PCI_BAR_SELECTOR {
    type Abi = Self;
    type DefaultType = Self;
}
pub type HDV_PCI_DEVICE_GET_DETAILS = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
    pnpid: *mut HDV_PCI_PNP_ID,
    probedbarscount: u32,
    probedbars: *mut u32,
) -> ::windows::runtime::HRESULT;
pub type HDV_PCI_DEVICE_INITIALIZE = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HDV_PCI_DEVICE_INTERFACE {
    pub Version: HDV_PCI_INTERFACE_VERSION,
    pub Initialize: ::std::option::Option<HDV_PCI_DEVICE_INITIALIZE>,
    pub Teardown: ::std::option::Option<HDV_PCI_DEVICE_TEARDOWN>,
    pub SetConfiguration: ::std::option::Option<HDV_PCI_DEVICE_SET_CONFIGURATION>,
    pub GetDetails: ::std::option::Option<HDV_PCI_DEVICE_GET_DETAILS>,
    pub Start: ::std::option::Option<HDV_PCI_DEVICE_START>,
    pub Stop: ::std::option::Option<HDV_PCI_DEVICE_STOP>,
    pub ReadConfigSpace: ::std::option::Option<HDV_PCI_READ_CONFIG_SPACE>,
    pub WriteConfigSpace: ::std::option::Option<HDV_PCI_WRITE_CONFIG_SPACE>,
    pub ReadInterceptedMemory: ::std::option::Option<HDV_PCI_READ_INTERCEPTED_MEMORY>,
    pub WriteInterceptedMemory: ::std::option::Option<HDV_PCI_WRITE_INTERCEPTED_MEMORY>,
}
#[cfg(feature = "Win32_Foundation")]
impl HDV_PCI_DEVICE_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HDV_PCI_DEVICE_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HDV_PCI_DEVICE_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDV_PCI_DEVICE_INTERFACE")
            .field("Version", &self.Version)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HDV_PCI_DEVICE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Initialize.map(|f| f as usize) == other.Initialize.map(|f| f as usize)
            && self.Teardown.map(|f| f as usize) == other.Teardown.map(|f| f as usize)
            && self.SetConfiguration.map(|f| f as usize)
                == other.SetConfiguration.map(|f| f as usize)
            && self.GetDetails.map(|f| f as usize) == other.GetDetails.map(|f| f as usize)
            && self.Start.map(|f| f as usize) == other.Start.map(|f| f as usize)
            && self.Stop.map(|f| f as usize) == other.Stop.map(|f| f as usize)
            && self.ReadConfigSpace.map(|f| f as usize) == other.ReadConfigSpace.map(|f| f as usize)
            && self.WriteConfigSpace.map(|f| f as usize)
                == other.WriteConfigSpace.map(|f| f as usize)
            && self.ReadInterceptedMemory.map(|f| f as usize)
                == other.ReadInterceptedMemory.map(|f| f as usize)
            && self.WriteInterceptedMemory.map(|f| f as usize)
                == other.WriteInterceptedMemory.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HDV_PCI_DEVICE_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HDV_PCI_DEVICE_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type HDV_PCI_DEVICE_SET_CONFIGURATION =
    unsafe extern "system" fn(
        devicecontext: *const ::std::ffi::c_void,
        configurationvaluecount: u32,
        configurationvalues: *const super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT;
pub type HDV_PCI_DEVICE_START = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::HRESULT;
pub type HDV_PCI_DEVICE_STOP = unsafe extern "system" fn(devicecontext: *const ::std::ffi::c_void);
pub type HDV_PCI_DEVICE_TEARDOWN =
    unsafe extern "system" fn(devicecontext: *const ::std::ffi::c_void);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HDV_PCI_INTERFACE_VERSION(pub i32);
pub const HdvPciDeviceInterfaceVersionInvalid: HDV_PCI_INTERFACE_VERSION =
    HDV_PCI_INTERFACE_VERSION(0i32);
pub const HdvPciDeviceInterfaceVersion1: HDV_PCI_INTERFACE_VERSION =
    HDV_PCI_INTERFACE_VERSION(1i32);
impl ::std::convert::From<i32> for HDV_PCI_INTERFACE_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDV_PCI_INTERFACE_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HDV_PCI_PNP_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
impl HDV_PCI_PNP_ID {}
impl ::std::default::Default for HDV_PCI_PNP_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HDV_PCI_PNP_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDV_PCI_PNP_ID")
            .field("VendorID", &self.VendorID)
            .field("DeviceID", &self.DeviceID)
            .field("RevisionID", &self.RevisionID)
            .field("ProgIf", &self.ProgIf)
            .field("SubClass", &self.SubClass)
            .field("BaseClass", &self.BaseClass)
            .field("SubVendorID", &self.SubVendorID)
            .field("SubSystemID", &self.SubSystemID)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HDV_PCI_PNP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.VendorID == other.VendorID
            && self.DeviceID == other.DeviceID
            && self.RevisionID == other.RevisionID
            && self.ProgIf == other.ProgIf
            && self.SubClass == other.SubClass
            && self.BaseClass == other.BaseClass
            && self.SubVendorID == other.SubVendorID
            && self.SubSystemID == other.SubSystemID
    }
}
impl ::std::cmp::Eq for HDV_PCI_PNP_ID {}
unsafe impl ::windows::runtime::Abi for HDV_PCI_PNP_ID {
    type Abi = Self;
    type DefaultType = Self;
}
pub type HDV_PCI_READ_CONFIG_SPACE = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
    offset: u32,
    value: *mut u32,
) -> ::windows::runtime::HRESULT;
pub type HDV_PCI_READ_INTERCEPTED_MEMORY = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    offset: u64,
    length: u64,
    value: *mut u8,
)
    -> ::windows::runtime::HRESULT;
pub type HDV_PCI_WRITE_CONFIG_SPACE = unsafe extern "system" fn(
    devicecontext: *const ::std::ffi::c_void,
    offset: u32,
    value: u32,
) -> ::windows::runtime::HRESULT;
pub type HDV_PCI_WRITE_INTERCEPTED_MEMORY =
    unsafe extern "system" fn(
        devicecontext: *const ::std::ffi::c_void,
        barindex: HDV_PCI_BAR_SELECTOR,
        offset: u64,
        length: u64,
        value: *const u8,
    ) -> ::windows::runtime::HRESULT;
pub const HVSOCKET_ADDRESS_FLAG_PASSTHRU: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HVSOCKET_ADDRESS_INFO {
    pub SystemId: ::windows::runtime::GUID,
    pub VirtualMachineId: ::windows::runtime::GUID,
    pub SiloId: ::windows::runtime::GUID,
    pub Flags: u32,
}
impl HVSOCKET_ADDRESS_INFO {}
impl ::std::default::Default for HVSOCKET_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HVSOCKET_ADDRESS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HVSOCKET_ADDRESS_INFO")
            .field("SystemId", &self.SystemId)
            .field("VirtualMachineId", &self.VirtualMachineId)
            .field("SiloId", &self.SiloId)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HVSOCKET_ADDRESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SystemId == other.SystemId
            && self.VirtualMachineId == other.VirtualMachineId
            && self.SiloId == other.SiloId
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for HVSOCKET_ADDRESS_INFO {}
unsafe impl ::windows::runtime::Abi for HVSOCKET_ADDRESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HVSOCKET_CONNECTED_SUSPEND: u32 = 4u32;
pub const HVSOCKET_CONNECT_TIMEOUT: u32 = 1u32;
pub const HVSOCKET_CONNECT_TIMEOUT_MAX: u32 = 300000u32;
pub const HVSOCKET_CONTAINER_PASSTHRU: u32 = 2u32;
pub const HV_GUID_BROADCAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4294967295,
    65535,
    65535,
    [255, 255, 255, 255, 255, 255, 255, 255],
);
pub const HV_GUID_CHILDREN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2430307209,
    3381,
    20345,
    [140, 233, 73, 234, 10, 200, 183, 205],
);
pub const HV_GUID_LOOPBACK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3772866967,
    56662,
    18960,
    [145, 149, 94, 231, 161, 85, 168, 56],
);
pub const HV_GUID_PARENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2754510042,
    53311,
    18444,
    [156, 194, 164, 222, 32, 171, 184, 120],
);
pub const HV_GUID_SILOHOST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    918359132,
    29302,
    16931,
    [136, 186, 125, 3, 182, 84, 197, 104],
);
pub const HV_GUID_VSOCK_TEMPLATE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 64203, 4582, [189, 88, 100, 0, 106, 121, 134, 211]);
pub const HV_GUID_ZERO: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const HV_PROTOCOL_RAW: u32 = 1u32;
pub unsafe fn HdvCreateDeviceInstance(
    devicehosthandle: *const ::std::ffi::c_void,
    devicetype: HDV_DEVICE_TYPE,
    deviceclassid: *const ::windows::runtime::GUID,
    deviceinstanceid: *const ::windows::runtime::GUID,
    deviceinterface: *const ::std::ffi::c_void,
    devicecontext: *const ::std::ffi::c_void,
    devicehandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvCreateDeviceInstance(
                devicehosthandle: *const ::std::ffi::c_void,
                devicetype: HDV_DEVICE_TYPE,
                deviceclassid: *const ::windows::runtime::GUID,
                deviceinstanceid: *const ::windows::runtime::GUID,
                deviceinterface: *const ::std::ffi::c_void,
                devicecontext: *const ::std::ffi::c_void,
                devicehandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvCreateDeviceInstance(
            ::std::mem::transmute(devicehosthandle),
            ::std::mem::transmute(devicetype),
            ::std::mem::transmute(deviceclassid),
            ::std::mem::transmute(deviceinstanceid),
            ::std::mem::transmute(deviceinterface),
            ::std::mem::transmute(devicecontext),
            ::std::mem::transmute(devicehandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HdvCreateGuestMemoryAperture<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    requestor: *const ::std::ffi::c_void,
    guestphysicaladdress: u64,
    bytecount: u32,
    writeprotected: Param3,
    mappedaddress: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvCreateGuestMemoryAperture(
                requestor: *const ::std::ffi::c_void,
                guestphysicaladdress: u64,
                bytecount: u32,
                writeprotected: super::super::Foundation::BOOL,
                mappedaddress: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvCreateGuestMemoryAperture(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(guestphysicaladdress),
            ::std::mem::transmute(bytecount),
            writeprotected.into_param().abi(),
            ::std::mem::transmute(mappedaddress),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HdvCreateSectionBackedMmioRange<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    requestor: *const ::std::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    offsetinpages: u64,
    lengthinpages: u64,
    mappingflags: HDV_MMIO_MAPPING_FLAGS,
    sectionhandle: Param5,
    sectionoffsetinpages: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvCreateSectionBackedMmioRange(
                requestor: *const ::std::ffi::c_void,
                barindex: HDV_PCI_BAR_SELECTOR,
                offsetinpages: u64,
                lengthinpages: u64,
                mappingflags: HDV_MMIO_MAPPING_FLAGS,
                sectionhandle: super::super::Foundation::HANDLE,
                sectionoffsetinpages: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvCreateSectionBackedMmioRange(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(barindex),
            ::std::mem::transmute(offsetinpages),
            ::std::mem::transmute(lengthinpages),
            ::std::mem::transmute(mappingflags),
            sectionhandle.into_param().abi(),
            ::std::mem::transmute(sectionoffsetinpages),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvDeliverGuestInterrupt(
    requestor: *const ::std::ffi::c_void,
    msiaddress: u64,
    msidata: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvDeliverGuestInterrupt(
                requestor: *const ::std::ffi::c_void,
                msiaddress: u64,
                msidata: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvDeliverGuestInterrupt(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(msiaddress),
            ::std::mem::transmute(msidata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvDestroyGuestMemoryAperture(
    requestor: *const ::std::ffi::c_void,
    mappedaddress: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvDestroyGuestMemoryAperture(
                requestor: *const ::std::ffi::c_void,
                mappedaddress: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvDestroyGuestMemoryAperture(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(mappedaddress),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvDestroySectionBackedMmioRange(
    requestor: *const ::std::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    offsetinpages: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvDestroySectionBackedMmioRange(
                requestor: *const ::std::ffi::c_void,
                barindex: HDV_PCI_BAR_SELECTOR,
                offsetinpages: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvDestroySectionBackedMmioRange(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(barindex),
            ::std::mem::transmute(offsetinpages),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_HostComputeSystem")]
pub unsafe fn HdvInitializeDeviceHost<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::HostComputeSystem::HCS_SYSTEM>,
>(
    computesystem: Param0,
    devicehosthandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvInitializeDeviceHost(
                computesystem: super::HostComputeSystem::HCS_SYSTEM,
                devicehosthandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvInitializeDeviceHost(
            computesystem.into_param().abi(),
            ::std::mem::transmute(devicehosthandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvReadGuestMemory(
    requestor: *const ::std::ffi::c_void,
    guestphysicaladdress: u64,
    bytecount: u32,
    buffer: *mut u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvReadGuestMemory(
                requestor: *const ::std::ffi::c_void,
                guestphysicaladdress: u64,
                bytecount: u32,
                buffer: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvReadGuestMemory(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(guestphysicaladdress),
            ::std::mem::transmute(bytecount),
            ::std::mem::transmute(buffer),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HdvRegisterDoorbell<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    requestor: *const ::std::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    baroffset: u64,
    triggervalue: u64,
    flags: u64,
    doorbellevent: Param5,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvRegisterDoorbell(
                requestor: *const ::std::ffi::c_void,
                barindex: HDV_PCI_BAR_SELECTOR,
                baroffset: u64,
                triggervalue: u64,
                flags: u64,
                doorbellevent: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvRegisterDoorbell(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(barindex),
            ::std::mem::transmute(baroffset),
            ::std::mem::transmute(triggervalue),
            ::std::mem::transmute(flags),
            doorbellevent.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvTeardownDeviceHost(
    devicehosthandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvTeardownDeviceHost(
                devicehosthandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvTeardownDeviceHost(::std::mem::transmute(devicehosthandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvUnregisterDoorbell(
    requestor: *const ::std::ffi::c_void,
    barindex: HDV_PCI_BAR_SELECTOR,
    baroffset: u64,
    triggervalue: u64,
    flags: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvUnregisterDoorbell(
                requestor: *const ::std::ffi::c_void,
                barindex: HDV_PCI_BAR_SELECTOR,
                baroffset: u64,
                triggervalue: u64,
                flags: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvUnregisterDoorbell(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(barindex),
            ::std::mem::transmute(baroffset),
            ::std::mem::transmute(triggervalue),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HdvWriteGuestMemory(
    requestor: *const ::std::ffi::c_void,
    guestphysicaladdress: u64,
    bytecount: u32,
    buffer: *const u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmdevicehost")]
        extern "system" {
            fn HdvWriteGuestMemory(
                requestor: *const ::std::ffi::c_void,
                guestphysicaladdress: u64,
                bytecount: u32,
                buffer: *const u8,
            ) -> ::windows::runtime::HRESULT;
        }
        HdvWriteGuestMemory(
            ::std::mem::transmute(requestor),
            ::std::mem::transmute(guestphysicaladdress),
            ::std::mem::transmute(bytecount),
            ::std::mem::transmute(buffer),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const IOCTL_VMGENCOUNTER_READ: u32 = 3325956u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn InKernelSpace(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    inkernelspace: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn InKernelSpace(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                inkernelspace: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        InKernelSpace(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(inkernelspace),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsActiveVirtualTrustLevelEnabled(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    activevirtualtrustlevelenabled: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn IsActiveVirtualTrustLevelEnabled(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                activevirtualtrustlevelenabled: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        IsActiveVirtualTrustLevelEnabled(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(activevirtualtrustlevelenabled),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsNestedVirtualizationEnabled(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    enabled: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn IsNestedVirtualizationEnabled(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                enabled: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        IsNestedVirtualizationEnabled(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(enabled),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadSavedStateFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    vmrsfile: Param0,
    vmsavedstatedumphandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LoadSavedStateFile(
                vmrsfile: super::super::Foundation::PWSTR,
                vmsavedstatedumphandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        LoadSavedStateFile(
            vmrsfile.into_param().abi(),
            ::std::mem::transmute(vmsavedstatedumphandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadSavedStateFiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    binfile: Param0,
    vsvfile: Param1,
    vmsavedstatedumphandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LoadSavedStateFiles(
                binfile: super::super::Foundation::PWSTR,
                vsvfile: super::super::Foundation::PWSTR,
                vmsavedstatedumphandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        LoadSavedStateFiles(
            binfile.into_param().abi(),
            vsvfile.into_param().abi(),
            ::std::mem::transmute(vmsavedstatedumphandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadSavedStateModuleSymbols<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    imagename: Param1,
    modulename: Param2,
    baseaddress: u64,
    sizeofbase: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LoadSavedStateModuleSymbols(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                imagename: super::super::Foundation::PSTR,
                modulename: super::super::Foundation::PSTR,
                baseaddress: u64,
                sizeofbase: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        LoadSavedStateModuleSymbols(
            ::std::mem::transmute(vmsavedstatedumphandle),
            imagename.into_param().abi(),
            modulename.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(sizeofbase),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadSavedStateModuleSymbolsEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    imagename: Param1,
    imagetimestamp: u32,
    modulename: Param3,
    baseaddress: u64,
    sizeofbase: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LoadSavedStateModuleSymbolsEx(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                imagename: super::super::Foundation::PSTR,
                imagetimestamp: u32,
                modulename: super::super::Foundation::PSTR,
                baseaddress: u64,
                sizeofbase: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        LoadSavedStateModuleSymbolsEx(
            ::std::mem::transmute(vmsavedstatedumphandle),
            imagename.into_param().abi(),
            ::std::mem::transmute(imagetimestamp),
            modulename.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(sizeofbase),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadSavedStateSymbolProvider<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    usersymbols: Param1,
    force: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LoadSavedStateSymbolProvider(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                usersymbols: super::super::Foundation::PWSTR,
                force: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        LoadSavedStateSymbolProvider(
            ::std::mem::transmute(vmsavedstatedumphandle),
            usersymbols.into_param().abi(),
            force.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LocateSavedStateFiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    vmname: Param0,
    snapshotname: Param1,
    binpath: *mut super::super::Foundation::PWSTR,
    vsvpath: *mut super::super::Foundation::PWSTR,
    vmrspath: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn LocateSavedStateFiles(
                vmname: super::super::Foundation::PWSTR,
                snapshotname: super::super::Foundation::PWSTR,
                binpath: *mut super::super::Foundation::PWSTR,
                vsvpath: *mut super::super::Foundation::PWSTR,
                vmrspath: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        LocateSavedStateFiles(
            vmname.into_param().abi(),
            snapshotname.into_param().abi(),
            ::std::mem::transmute(binpath),
            ::std::mem::transmute(vsvpath),
            ::std::mem::transmute(vmrspath),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MODULE_INFO {
    pub ProcessImageName: super::super::Foundation::PSTR,
    pub Image: DOS_IMAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl MODULE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODULE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MODULE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODULE_INFO")
            .field("ProcessImageName", &self.ProcessImageName)
            .field("Image", &self.Image)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODULE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessImageName == other.ProcessImageName && self.Image == other.Image
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODULE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODULE_INFO {
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
pub struct PAGING_MODE(pub i32);
pub const Paging_Invalid: PAGING_MODE = PAGING_MODE(0i32);
pub const Paging_NonPaged: PAGING_MODE = PAGING_MODE(1i32);
pub const Paging_32Bit: PAGING_MODE = PAGING_MODE(2i32);
pub const Paging_Pae: PAGING_MODE = PAGING_MODE(3i32);
pub const Paging_Long: PAGING_MODE = PAGING_MODE(4i32);
pub const Paging_Armv8: PAGING_MODE = PAGING_MODE(5i32);
impl ::std::convert::From<i32> for PAGING_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGING_MODE {
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
pub struct REGISTER_ID(pub i32);
pub const X64_RegisterRax: REGISTER_ID = REGISTER_ID(0i32);
pub const X64_RegisterRcx: REGISTER_ID = REGISTER_ID(1i32);
pub const X64_RegisterRdx: REGISTER_ID = REGISTER_ID(2i32);
pub const X64_RegisterRbx: REGISTER_ID = REGISTER_ID(3i32);
pub const X64_RegisterRsp: REGISTER_ID = REGISTER_ID(4i32);
pub const X64_RegisterRbp: REGISTER_ID = REGISTER_ID(5i32);
pub const X64_RegisterRsi: REGISTER_ID = REGISTER_ID(6i32);
pub const X64_RegisterRdi: REGISTER_ID = REGISTER_ID(7i32);
pub const X64_RegisterR8: REGISTER_ID = REGISTER_ID(8i32);
pub const X64_RegisterR9: REGISTER_ID = REGISTER_ID(9i32);
pub const X64_RegisterR10: REGISTER_ID = REGISTER_ID(10i32);
pub const X64_RegisterR11: REGISTER_ID = REGISTER_ID(11i32);
pub const X64_RegisterR12: REGISTER_ID = REGISTER_ID(12i32);
pub const X64_RegisterR13: REGISTER_ID = REGISTER_ID(13i32);
pub const X64_RegisterR14: REGISTER_ID = REGISTER_ID(14i32);
pub const X64_RegisterR15: REGISTER_ID = REGISTER_ID(15i32);
pub const X64_RegisterRip: REGISTER_ID = REGISTER_ID(16i32);
pub const X64_RegisterRFlags: REGISTER_ID = REGISTER_ID(17i32);
pub const X64_RegisterXmm0: REGISTER_ID = REGISTER_ID(18i32);
pub const X64_RegisterXmm1: REGISTER_ID = REGISTER_ID(19i32);
pub const X64_RegisterXmm2: REGISTER_ID = REGISTER_ID(20i32);
pub const X64_RegisterXmm3: REGISTER_ID = REGISTER_ID(21i32);
pub const X64_RegisterXmm4: REGISTER_ID = REGISTER_ID(22i32);
pub const X64_RegisterXmm5: REGISTER_ID = REGISTER_ID(23i32);
pub const X64_RegisterXmm6: REGISTER_ID = REGISTER_ID(24i32);
pub const X64_RegisterXmm7: REGISTER_ID = REGISTER_ID(25i32);
pub const X64_RegisterXmm8: REGISTER_ID = REGISTER_ID(26i32);
pub const X64_RegisterXmm9: REGISTER_ID = REGISTER_ID(27i32);
pub const X64_RegisterXmm10: REGISTER_ID = REGISTER_ID(28i32);
pub const X64_RegisterXmm11: REGISTER_ID = REGISTER_ID(29i32);
pub const X64_RegisterXmm12: REGISTER_ID = REGISTER_ID(30i32);
pub const X64_RegisterXmm13: REGISTER_ID = REGISTER_ID(31i32);
pub const X64_RegisterXmm14: REGISTER_ID = REGISTER_ID(32i32);
pub const X64_RegisterXmm15: REGISTER_ID = REGISTER_ID(33i32);
pub const X64_RegisterFpMmx0: REGISTER_ID = REGISTER_ID(34i32);
pub const X64_RegisterFpMmx1: REGISTER_ID = REGISTER_ID(35i32);
pub const X64_RegisterFpMmx2: REGISTER_ID = REGISTER_ID(36i32);
pub const X64_RegisterFpMmx3: REGISTER_ID = REGISTER_ID(37i32);
pub const X64_RegisterFpMmx4: REGISTER_ID = REGISTER_ID(38i32);
pub const X64_RegisterFpMmx5: REGISTER_ID = REGISTER_ID(39i32);
pub const X64_RegisterFpMmx6: REGISTER_ID = REGISTER_ID(40i32);
pub const X64_RegisterFpMmx7: REGISTER_ID = REGISTER_ID(41i32);
pub const X64_RegisterFpControlStatus: REGISTER_ID = REGISTER_ID(42i32);
pub const X64_RegisterXmmControlStatus: REGISTER_ID = REGISTER_ID(43i32);
pub const X64_RegisterCr0: REGISTER_ID = REGISTER_ID(44i32);
pub const X64_RegisterCr2: REGISTER_ID = REGISTER_ID(45i32);
pub const X64_RegisterCr3: REGISTER_ID = REGISTER_ID(46i32);
pub const X64_RegisterCr4: REGISTER_ID = REGISTER_ID(47i32);
pub const X64_RegisterCr8: REGISTER_ID = REGISTER_ID(48i32);
pub const X64_RegisterEfer: REGISTER_ID = REGISTER_ID(49i32);
pub const X64_RegisterDr0: REGISTER_ID = REGISTER_ID(50i32);
pub const X64_RegisterDr1: REGISTER_ID = REGISTER_ID(51i32);
pub const X64_RegisterDr2: REGISTER_ID = REGISTER_ID(52i32);
pub const X64_RegisterDr3: REGISTER_ID = REGISTER_ID(53i32);
pub const X64_RegisterDr6: REGISTER_ID = REGISTER_ID(54i32);
pub const X64_RegisterDr7: REGISTER_ID = REGISTER_ID(55i32);
pub const X64_RegisterEs: REGISTER_ID = REGISTER_ID(56i32);
pub const X64_RegisterCs: REGISTER_ID = REGISTER_ID(57i32);
pub const X64_RegisterSs: REGISTER_ID = REGISTER_ID(58i32);
pub const X64_RegisterDs: REGISTER_ID = REGISTER_ID(59i32);
pub const X64_RegisterFs: REGISTER_ID = REGISTER_ID(60i32);
pub const X64_RegisterGs: REGISTER_ID = REGISTER_ID(61i32);
pub const X64_RegisterLdtr: REGISTER_ID = REGISTER_ID(62i32);
pub const X64_RegisterTr: REGISTER_ID = REGISTER_ID(63i32);
pub const X64_RegisterIdtr: REGISTER_ID = REGISTER_ID(64i32);
pub const X64_RegisterGdtr: REGISTER_ID = REGISTER_ID(65i32);
pub const X64_RegisterMax: REGISTER_ID = REGISTER_ID(66i32);
pub const ARM64_RegisterX0: REGISTER_ID = REGISTER_ID(67i32);
pub const ARM64_RegisterX1: REGISTER_ID = REGISTER_ID(68i32);
pub const ARM64_RegisterX2: REGISTER_ID = REGISTER_ID(69i32);
pub const ARM64_RegisterX3: REGISTER_ID = REGISTER_ID(70i32);
pub const ARM64_RegisterX4: REGISTER_ID = REGISTER_ID(71i32);
pub const ARM64_RegisterX5: REGISTER_ID = REGISTER_ID(72i32);
pub const ARM64_RegisterX6: REGISTER_ID = REGISTER_ID(73i32);
pub const ARM64_RegisterX7: REGISTER_ID = REGISTER_ID(74i32);
pub const ARM64_RegisterX8: REGISTER_ID = REGISTER_ID(75i32);
pub const ARM64_RegisterX9: REGISTER_ID = REGISTER_ID(76i32);
pub const ARM64_RegisterX10: REGISTER_ID = REGISTER_ID(77i32);
pub const ARM64_RegisterX11: REGISTER_ID = REGISTER_ID(78i32);
pub const ARM64_RegisterX12: REGISTER_ID = REGISTER_ID(79i32);
pub const ARM64_RegisterX13: REGISTER_ID = REGISTER_ID(80i32);
pub const ARM64_RegisterX14: REGISTER_ID = REGISTER_ID(81i32);
pub const ARM64_RegisterX15: REGISTER_ID = REGISTER_ID(82i32);
pub const ARM64_RegisterX16: REGISTER_ID = REGISTER_ID(83i32);
pub const ARM64_RegisterX17: REGISTER_ID = REGISTER_ID(84i32);
pub const ARM64_RegisterX18: REGISTER_ID = REGISTER_ID(85i32);
pub const ARM64_RegisterX19: REGISTER_ID = REGISTER_ID(86i32);
pub const ARM64_RegisterX20: REGISTER_ID = REGISTER_ID(87i32);
pub const ARM64_RegisterX21: REGISTER_ID = REGISTER_ID(88i32);
pub const ARM64_RegisterX22: REGISTER_ID = REGISTER_ID(89i32);
pub const ARM64_RegisterX23: REGISTER_ID = REGISTER_ID(90i32);
pub const ARM64_RegisterX24: REGISTER_ID = REGISTER_ID(91i32);
pub const ARM64_RegisterX25: REGISTER_ID = REGISTER_ID(92i32);
pub const ARM64_RegisterX26: REGISTER_ID = REGISTER_ID(93i32);
pub const ARM64_RegisterX27: REGISTER_ID = REGISTER_ID(94i32);
pub const ARM64_RegisterX28: REGISTER_ID = REGISTER_ID(95i32);
pub const ARM64_RegisterXFp: REGISTER_ID = REGISTER_ID(96i32);
pub const ARM64_RegisterXLr: REGISTER_ID = REGISTER_ID(97i32);
pub const ARM64_RegisterPc: REGISTER_ID = REGISTER_ID(98i32);
pub const ARM64_RegisterSpEl0: REGISTER_ID = REGISTER_ID(99i32);
pub const ARM64_RegisterSpEl1: REGISTER_ID = REGISTER_ID(100i32);
pub const ARM64_RegisterCpsr: REGISTER_ID = REGISTER_ID(101i32);
pub const ARM64_RegisterQ0: REGISTER_ID = REGISTER_ID(102i32);
pub const ARM64_RegisterQ1: REGISTER_ID = REGISTER_ID(103i32);
pub const ARM64_RegisterQ2: REGISTER_ID = REGISTER_ID(104i32);
pub const ARM64_RegisterQ3: REGISTER_ID = REGISTER_ID(105i32);
pub const ARM64_RegisterQ4: REGISTER_ID = REGISTER_ID(106i32);
pub const ARM64_RegisterQ5: REGISTER_ID = REGISTER_ID(107i32);
pub const ARM64_RegisterQ6: REGISTER_ID = REGISTER_ID(108i32);
pub const ARM64_RegisterQ7: REGISTER_ID = REGISTER_ID(109i32);
pub const ARM64_RegisterQ8: REGISTER_ID = REGISTER_ID(110i32);
pub const ARM64_RegisterQ9: REGISTER_ID = REGISTER_ID(111i32);
pub const ARM64_RegisterQ10: REGISTER_ID = REGISTER_ID(112i32);
pub const ARM64_RegisterQ11: REGISTER_ID = REGISTER_ID(113i32);
pub const ARM64_RegisterQ12: REGISTER_ID = REGISTER_ID(114i32);
pub const ARM64_RegisterQ13: REGISTER_ID = REGISTER_ID(115i32);
pub const ARM64_RegisterQ14: REGISTER_ID = REGISTER_ID(116i32);
pub const ARM64_RegisterQ15: REGISTER_ID = REGISTER_ID(117i32);
pub const ARM64_RegisterQ16: REGISTER_ID = REGISTER_ID(118i32);
pub const ARM64_RegisterQ17: REGISTER_ID = REGISTER_ID(119i32);
pub const ARM64_RegisterQ18: REGISTER_ID = REGISTER_ID(120i32);
pub const ARM64_RegisterQ19: REGISTER_ID = REGISTER_ID(121i32);
pub const ARM64_RegisterQ20: REGISTER_ID = REGISTER_ID(122i32);
pub const ARM64_RegisterQ21: REGISTER_ID = REGISTER_ID(123i32);
pub const ARM64_RegisterQ22: REGISTER_ID = REGISTER_ID(124i32);
pub const ARM64_RegisterQ23: REGISTER_ID = REGISTER_ID(125i32);
pub const ARM64_RegisterQ24: REGISTER_ID = REGISTER_ID(126i32);
pub const ARM64_RegisterQ25: REGISTER_ID = REGISTER_ID(127i32);
pub const ARM64_RegisterQ26: REGISTER_ID = REGISTER_ID(128i32);
pub const ARM64_RegisterQ27: REGISTER_ID = REGISTER_ID(129i32);
pub const ARM64_RegisterQ28: REGISTER_ID = REGISTER_ID(130i32);
pub const ARM64_RegisterQ29: REGISTER_ID = REGISTER_ID(131i32);
pub const ARM64_RegisterQ30: REGISTER_ID = REGISTER_ID(132i32);
pub const ARM64_RegisterQ31: REGISTER_ID = REGISTER_ID(133i32);
pub const ARM64_RegisterFpStatus: REGISTER_ID = REGISTER_ID(134i32);
pub const ARM64_RegisterFpControl: REGISTER_ID = REGISTER_ID(135i32);
pub const ARM64_RegisterEsrEl1: REGISTER_ID = REGISTER_ID(136i32);
pub const ARM64_RegisterSpsrEl1: REGISTER_ID = REGISTER_ID(137i32);
pub const ARM64_RegisterFarEl1: REGISTER_ID = REGISTER_ID(138i32);
pub const ARM64_RegisterParEl1: REGISTER_ID = REGISTER_ID(139i32);
pub const ARM64_RegisterElrEl1: REGISTER_ID = REGISTER_ID(140i32);
pub const ARM64_RegisterTtbr0El1: REGISTER_ID = REGISTER_ID(141i32);
pub const ARM64_RegisterTtbr1El1: REGISTER_ID = REGISTER_ID(142i32);
pub const ARM64_RegisterVbarEl1: REGISTER_ID = REGISTER_ID(143i32);
pub const ARM64_RegisterSctlrEl1: REGISTER_ID = REGISTER_ID(144i32);
pub const ARM64_RegisterActlrEl1: REGISTER_ID = REGISTER_ID(145i32);
pub const ARM64_RegisterTcrEl1: REGISTER_ID = REGISTER_ID(146i32);
pub const ARM64_RegisterMairEl1: REGISTER_ID = REGISTER_ID(147i32);
pub const ARM64_RegisterAmairEl1: REGISTER_ID = REGISTER_ID(148i32);
pub const ARM64_RegisterTpidrEl0: REGISTER_ID = REGISTER_ID(149i32);
pub const ARM64_RegisterTpidrroEl0: REGISTER_ID = REGISTER_ID(150i32);
pub const ARM64_RegisterTpidrEl1: REGISTER_ID = REGISTER_ID(151i32);
pub const ARM64_RegisterContextIdrEl1: REGISTER_ID = REGISTER_ID(152i32);
pub const ARM64_RegisterCpacrEl1: REGISTER_ID = REGISTER_ID(153i32);
pub const ARM64_RegisterCsselrEl1: REGISTER_ID = REGISTER_ID(154i32);
pub const ARM64_RegisterCntkctlEl1: REGISTER_ID = REGISTER_ID(155i32);
pub const ARM64_RegisterCntvCvalEl0: REGISTER_ID = REGISTER_ID(156i32);
pub const ARM64_RegisterCntvCtlEl0: REGISTER_ID = REGISTER_ID(157i32);
pub const ARM64_RegisterMax: REGISTER_ID = REGISTER_ID(158i32);
impl ::std::convert::From<i32> for REGISTER_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGISTER_ID {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn ReadGuestPhysicalAddress(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    physicaladdress: u64,
    buffer: *mut ::std::ffi::c_void,
    buffersize: u32,
    bytesread: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ReadGuestPhysicalAddress(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                physicaladdress: u64,
                buffer: *mut ::std::ffi::c_void,
                buffersize: u32,
                bytesread: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ReadGuestPhysicalAddress(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(physicaladdress),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(bytesread),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReadGuestRawSavedMemory(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    rawsavedmemoryoffset: u64,
    buffer: *mut ::std::ffi::c_void,
    buffersize: u32,
    bytesread: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ReadGuestRawSavedMemory(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                rawsavedmemoryoffset: u64,
                buffer: *mut ::std::ffi::c_void,
                buffersize: u32,
                bytesread: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ReadGuestRawSavedMemory(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(rawsavedmemoryoffset),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(bytesread),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReadSavedStateGlobalVariable<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    globalname: Param2,
    buffer: *mut ::std::ffi::c_void,
    buffersize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ReadSavedStateGlobalVariable(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                globalname: super::super::Foundation::PSTR,
                buffer: *mut ::std::ffi::c_void,
                buffersize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ReadSavedStateGlobalVariable(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            globalname.into_param().abi(),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReleaseSavedStateFiles(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ReleaseSavedStateFiles(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        ReleaseSavedStateFiles(::std::mem::transmute(vmsavedstatedumphandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReleaseSavedStateSymbolProvider(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ReleaseSavedStateSymbolProvider(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        ReleaseSavedStateSymbolProvider(::std::mem::transmute(vmsavedstatedumphandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ResolveSavedStateGlobalVariableAddress<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    globalname: Param2,
    virtualaddress: *mut u64,
    size: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ResolveSavedStateGlobalVariableAddress(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                globalname: super::super::Foundation::PSTR,
                virtualaddress: *mut u64,
                size: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ResolveSavedStateGlobalVariableAddress(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            globalname.into_param().abi(),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(size),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SOCKADDR_HV {
    pub Family: u16,
    pub Reserved: u16,
    pub VmId: ::windows::runtime::GUID,
    pub ServiceId: ::windows::runtime::GUID,
}
impl SOCKADDR_HV {}
impl ::std::default::Default for SOCKADDR_HV {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOCKADDR_HV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOCKADDR_HV")
            .field("Family", &self.Family)
            .field("Reserved", &self.Reserved)
            .field("VmId", &self.VmId)
            .field("ServiceId", &self.ServiceId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SOCKADDR_HV {
    fn eq(&self, other: &Self) -> bool {
        self.Family == other.Family
            && self.Reserved == other.Reserved
            && self.VmId == other.VmId
            && self.ServiceId == other.ServiceId
    }
}
impl ::std::cmp::Eq for SOCKADDR_HV {}
unsafe impl ::windows::runtime::Abi for SOCKADDR_HV {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ScanMemoryForDosImages(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    vpid: u32,
    startaddress: u64,
    endaddress: u64,
    callbackcontext: *mut ::std::ffi::c_void,
    foundimagecallback: ::std::option::Option<FOUND_IMAGE_CALLBACK>,
    standaloneaddress: *const u64,
    standaloneaddresscount: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn ScanMemoryForDosImages(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                vpid: u32,
                startaddress: u64,
                endaddress: u64,
                callbackcontext: *mut ::std::ffi::c_void,
                foundimagecallback: ::windows::runtime::RawPtr,
                standaloneaddress: *const u64,
                standaloneaddresscount: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ScanMemoryForDosImages(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(vpid),
            ::std::mem::transmute(startaddress),
            ::std::mem::transmute(endaddress),
            ::std::mem::transmute(callbackcontext),
            ::std::mem::transmute(foundimagecallback),
            ::std::mem::transmute(standaloneaddress),
            ::std::mem::transmute(standaloneaddresscount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetMemoryBlockCacheLimit(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    memoryblockcachelimit: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn SetMemoryBlockCacheLimit(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                memoryblockcachelimit: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        SetMemoryBlockCacheLimit(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(memoryblockcachelimit),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSavedStateSymbolProviderDebugInfoCallback(
    vmsavedstatedumphandle: *mut ::std::ffi::c_void,
    callback: ::std::option::Option<GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "vmsavedstatedumpprovider")]
        extern "system" {
            fn SetSavedStateSymbolProviderDebugInfoCallback(
                vmsavedstatedumphandle: *mut ::std::ffi::c_void,
                callback: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        SetSavedStateSymbolProviderDebugInfoCallback(
            ::std::mem::transmute(vmsavedstatedumphandle),
            ::std::mem::transmute(callback),
        )
        .ok()
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
pub struct VIRTUAL_PROCESSOR_ARCH(pub i32);
pub const Arch_Unknown: VIRTUAL_PROCESSOR_ARCH = VIRTUAL_PROCESSOR_ARCH(0i32);
pub const Arch_x86: VIRTUAL_PROCESSOR_ARCH = VIRTUAL_PROCESSOR_ARCH(1i32);
pub const Arch_x64: VIRTUAL_PROCESSOR_ARCH = VIRTUAL_PROCESSOR_ARCH(2i32);
pub const Arch_Armv8: VIRTUAL_PROCESSOR_ARCH = VIRTUAL_PROCESSOR_ARCH(3i32);
impl ::std::convert::From<i32> for VIRTUAL_PROCESSOR_ARCH {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_ARCH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER {
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub Reg128: VIRTUAL_PROCESSOR_REGISTER_0,
    pub X64: VIRTUAL_PROCESSOR_REGISTER_1,
}
impl VIRTUAL_PROCESSOR_REGISTER {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_0 {
    pub Low64: u64,
    pub High64: u64,
}
impl VIRTUAL_PROCESSOR_REGISTER_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_PROCESSOR_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Reg128_e__Struct")
            .field("Low64", &self.Low64)
            .field("High64", &self.High64)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Low64 == other.Low64 && self.High64 == other.High64
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1 {
    pub Segment: VIRTUAL_PROCESSOR_REGISTER_1_1,
    pub Table: VIRTUAL_PROCESSOR_REGISTER_1_2,
    pub FpControlStatus: VIRTUAL_PROCESSOR_REGISTER_1_0,
    pub XmmControlStatus: VIRTUAL_PROCESSOR_REGISTER_1_3,
}
impl VIRTUAL_PROCESSOR_REGISTER_1 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_0 {
    pub FpControl: u16,
    pub FpStatus: u16,
    pub FpTag: u8,
    pub Reserved: u8,
    pub LastFpOp: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_0_0,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    pub LastFpRip: u64,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_0_0_0,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_0_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    pub LastFpEip: u32,
    pub LastFpCs: u16,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("LastFpEip", &self.LastFpEip)
            .field("LastFpCs", &self.LastFpCs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LastFpEip == other.LastFpEip && self.LastFpCs == other.LastFpCs
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_1 {
    pub Base: u64,
    pub Limit: u32,
    pub Selector: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_1_0,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_1 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_1 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    pub Attributes: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_1_0_0,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_1_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    pub _bitfield: u16,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_2 {
    pub Limit: u16,
    pub Base: u64,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_2 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_PROCESSOR_REGISTER_1_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Table_e__Struct")
            .field("Limit", &self.Limit)
            .field("Base", &self.Base)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Limit == other.Limit && self.Base == other.Base
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_2 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_3 {
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_3_0,
    pub XmmStatusControl: u32,
    pub XmmStatusControlMask: u32,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_3 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_3 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    pub LastFpRdp: u64,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_3_0_0,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_3_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    pub LastFpDp: u32,
    pub LastFpDs: u16,
}
impl VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {}
impl ::std::default::Default for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("LastFpDp", &self.LastFpDp)
            .field("LastFpDs", &self.LastFpDs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LastFpDp == other.LastFpDp && self.LastFpDs == other.LastFpDs
    }
}
impl ::std::cmp::Eq for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
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
pub struct VIRTUAL_PROCESSOR_VENDOR(pub i32);
pub const ProcessorVendor_Unknown: VIRTUAL_PROCESSOR_VENDOR = VIRTUAL_PROCESSOR_VENDOR(0i32);
pub const ProcessorVendor_Amd: VIRTUAL_PROCESSOR_VENDOR = VIRTUAL_PROCESSOR_VENDOR(1i32);
pub const ProcessorVendor_Intel: VIRTUAL_PROCESSOR_VENDOR = VIRTUAL_PROCESSOR_VENDOR(2i32);
pub const ProcessorVendor_Hygon: VIRTUAL_PROCESSOR_VENDOR = VIRTUAL_PROCESSOR_VENDOR(3i32);
pub const ProcessorVendor_Arm: VIRTUAL_PROCESSOR_VENDOR = VIRTUAL_PROCESSOR_VENDOR(4i32);
impl ::std::convert::From<i32> for VIRTUAL_PROCESSOR_VENDOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_PROCESSOR_VENDOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VM_GENCOUNTER {
    pub GenerationCount: u64,
    pub GenerationCountHigh: u64,
}
impl VM_GENCOUNTER {}
impl ::std::default::Default for VM_GENCOUNTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VM_GENCOUNTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VM_GENCOUNTER")
            .field("GenerationCount", &self.GenerationCount)
            .field("GenerationCountHigh", &self.GenerationCountHigh)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VM_GENCOUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.GenerationCount == other.GenerationCount
            && self.GenerationCountHigh == other.GenerationCountHigh
    }
}
impl ::std::cmp::Eq for VM_GENCOUNTER {}
unsafe impl ::windows::runtime::Abi for VM_GENCOUNTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_ACCESS_GPA_CONTROLS {
    pub AsUINT64: u64,
    pub Anonymous: WHV_ACCESS_GPA_CONTROLS_0,
}
impl WHV_ACCESS_GPA_CONTROLS {}
impl ::std::default::Default for WHV_ACCESS_GPA_CONTROLS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_ACCESS_GPA_CONTROLS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_ACCESS_GPA_CONTROLS {}
unsafe impl ::windows::runtime::Abi for WHV_ACCESS_GPA_CONTROLS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_ACCESS_GPA_CONTROLS_0 {
    pub CacheType: WHV_CACHE_TYPE,
    pub Reserved: u32,
}
impl WHV_ACCESS_GPA_CONTROLS_0 {}
impl ::std::default::Default for WHV_ACCESS_GPA_CONTROLS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_ACCESS_GPA_CONTROLS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("CacheType", &self.CacheType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_ACCESS_GPA_CONTROLS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CacheType == other.CacheType && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WHV_ACCESS_GPA_CONTROLS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_ACCESS_GPA_CONTROLS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_ADVISE_GPA_RANGE {
    pub Populate: WHV_ADVISE_GPA_RANGE_POPULATE,
}
impl WHV_ADVISE_GPA_RANGE {}
impl ::std::default::Default for WHV_ADVISE_GPA_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_ADVISE_GPA_RANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_ADVISE_GPA_RANGE {}
unsafe impl ::windows::runtime::Abi for WHV_ADVISE_GPA_RANGE {
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
pub struct WHV_ADVISE_GPA_RANGE_CODE(pub i32);
pub const WHvAdviseGpaRangeCodePopulate: WHV_ADVISE_GPA_RANGE_CODE =
    WHV_ADVISE_GPA_RANGE_CODE(0i32);
pub const WHvAdviseGpaRangeCodePin: WHV_ADVISE_GPA_RANGE_CODE = WHV_ADVISE_GPA_RANGE_CODE(1i32);
pub const WHvAdviseGpaRangeCodeUnpin: WHV_ADVISE_GPA_RANGE_CODE = WHV_ADVISE_GPA_RANGE_CODE(2i32);
impl ::std::convert::From<i32> for WHV_ADVISE_GPA_RANGE_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_ADVISE_GPA_RANGE_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE {
    pub Flags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub AccessType: WHV_MEMORY_ACCESS_TYPE,
}
impl WHV_ADVISE_GPA_RANGE_POPULATE {}
impl ::std::default::Default for WHV_ADVISE_GPA_RANGE_POPULATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_ADVISE_GPA_RANGE_POPULATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_ADVISE_GPA_RANGE_POPULATE {}
unsafe impl ::windows::runtime::Abi for WHV_ADVISE_GPA_RANGE_POPULATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    pub AsUINT32: u32,
    pub Anonymous: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0,
}
impl WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {}
impl ::std::default::Default for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {}
unsafe impl ::windows::runtime::Abi for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    pub _bitfield: u32,
}
impl WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {}
impl ::std::default::Default for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
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
pub struct WHV_ALLOCATE_VPCI_RESOURCE_FLAGS(pub u32);
pub const WHvAllocateVpciResourceFlagNone: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS =
    WHV_ALLOCATE_VPCI_RESOURCE_FLAGS(0u32);
pub const WHvAllocateVpciResourceFlagAllowDirectP2P: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS =
    WHV_ALLOCATE_VPCI_RESOURCE_FLAGS(1u32);
impl ::std::convert::From<u32> for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WHV_ANY_VP: u32 = 4294967295u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WHV_CACHE_TYPE(pub i32);
pub const WHvCacheTypeUncached: WHV_CACHE_TYPE = WHV_CACHE_TYPE(0i32);
pub const WHvCacheTypeWriteCombining: WHV_CACHE_TYPE = WHV_CACHE_TYPE(1i32);
pub const WHvCacheTypeWriteThrough: WHV_CACHE_TYPE = WHV_CACHE_TYPE(4i32);
pub const WHvCacheTypeWriteProtected: WHV_CACHE_TYPE = WHV_CACHE_TYPE(5i32);
pub const WHvCacheTypeWriteBack: WHV_CACHE_TYPE = WHV_CACHE_TYPE(6i32);
impl ::std::convert::From<i32> for WHV_CACHE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_CACHE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WHV_CAPABILITY {
    pub HypervisorPresent: super::super::Foundation::BOOL,
    pub Features: WHV_CAPABILITY_FEATURES,
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorVendor: WHV_PROCESSOR_VENDOR,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub ProcessorClFlushSize: u8,
    pub ExceptionExitBitmap: u64,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ProcessorClockFrequency: u64,
    pub InterruptClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub GpaRangePopulateFlags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub ProcessorFrequencyCap: WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub SchedulerFeatures: WHV_SCHEDULER_FEATURES,
}
#[cfg(feature = "Win32_Foundation")]
impl WHV_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WHV_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WHV_CAPABILITY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WHV_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WHV_CAPABILITY {
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
pub struct WHV_CAPABILITY_CODE(pub i32);
pub const WHvCapabilityCodeHypervisorPresent: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(0i32);
pub const WHvCapabilityCodeFeatures: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(1i32);
pub const WHvCapabilityCodeExtendedVmExits: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(2i32);
pub const WHvCapabilityCodeExceptionExitBitmap: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(3i32);
pub const WHvCapabilityCodeX64MsrExitBitmap: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(4i32);
pub const WHvCapabilityCodeGpaRangePopulateFlags: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(5i32);
pub const WHvCapabilityCodeSchedulerFeatures: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(6i32);
pub const WHvCapabilityCodeProcessorVendor: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(4096i32);
pub const WHvCapabilityCodeProcessorFeatures: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(4097i32);
pub const WHvCapabilityCodeProcessorClFlushSize: WHV_CAPABILITY_CODE = WHV_CAPABILITY_CODE(4098i32);
pub const WHvCapabilityCodeProcessorXsaveFeatures: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4099i32);
pub const WHvCapabilityCodeProcessorClockFrequency: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4100i32);
pub const WHvCapabilityCodeInterruptClockFrequency: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4101i32);
pub const WHvCapabilityCodeProcessorFeaturesBanks: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4102i32);
pub const WHvCapabilityCodeProcessorFrequencyCap: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4103i32);
pub const WHvCapabilityCodeSyntheticProcessorFeaturesBanks: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4104i32);
pub const WHvCapabilityCodeProcessorPerfmonFeatures: WHV_CAPABILITY_CODE =
    WHV_CAPABILITY_CODE(4105i32);
impl ::std::convert::From<i32> for WHV_CAPABILITY_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_CAPABILITY_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_CAPABILITY_FEATURES {
    pub Anonymous: WHV_CAPABILITY_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_CAPABILITY_FEATURES {}
impl ::std::default::Default for WHV_CAPABILITY_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_CAPABILITY_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_CAPABILITY_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_CAPABILITY_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_CAPABILITY_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_CAPABILITY_FEATURES_0 {}
impl ::std::default::Default for WHV_CAPABILITY_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_CAPABILITY_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_CAPABILITY_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_CAPABILITY_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_CAPABILITY_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    pub _bitfield: u32,
    pub HighestFrequencyMhz: u32,
    pub NominalFrequencyMhz: u32,
    pub LowestFrequencyMhz: u32,
    pub FrequencyStepMhz: u32,
}
impl WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {}
impl ::std::default::Default for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP")
            .field("_bitfield", &self._bitfield)
            .field("HighestFrequencyMhz", &self.HighestFrequencyMhz)
            .field("NominalFrequencyMhz", &self.NominalFrequencyMhz)
            .field("LowestFrequencyMhz", &self.LowestFrequencyMhz)
            .field("FrequencyStepMhz", &self.FrequencyStepMhz)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.HighestFrequencyMhz == other.HighestFrequencyMhz
            && self.NominalFrequencyMhz == other.NominalFrequencyMhz
            && self.LowestFrequencyMhz == other.LowestFrequencyMhz
            && self.FrequencyStepMhz == other.FrequencyStepMhz
    }
}
impl ::std::cmp::Eq for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {}
unsafe impl ::windows::runtime::Abi for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_CPUID_OUTPUT {
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
impl WHV_CPUID_OUTPUT {}
impl ::std::default::Default for WHV_CPUID_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_CPUID_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_CPUID_OUTPUT")
            .field("Eax", &self.Eax)
            .field("Ebx", &self.Ebx)
            .field("Ecx", &self.Ecx)
            .field("Edx", &self.Edx)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_CPUID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Eax == other.Eax
            && self.Ebx == other.Ebx
            && self.Ecx == other.Ecx
            && self.Edx == other.Edx
    }
}
impl ::std::cmp::Eq for WHV_CPUID_OUTPUT {}
unsafe impl ::windows::runtime::Abi for WHV_CPUID_OUTPUT {
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
pub struct WHV_CREATE_VPCI_DEVICE_FLAGS(pub u32);
pub const WHvCreateVpciDeviceFlagNone: WHV_CREATE_VPCI_DEVICE_FLAGS =
    WHV_CREATE_VPCI_DEVICE_FLAGS(0u32);
pub const WHvCreateVpciDeviceFlagPhysicallyBacked: WHV_CREATE_VPCI_DEVICE_FLAGS =
    WHV_CREATE_VPCI_DEVICE_FLAGS(1u32);
pub const WHvCreateVpciDeviceFlagUseLogicalInterrupts: WHV_CREATE_VPCI_DEVICE_FLAGS =
    WHV_CREATE_VPCI_DEVICE_FLAGS(2u32);
impl ::std::convert::From<u32> for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_DOORBELL_MATCH_DATA {
    pub GuestAddress: u64,
    pub Value: u64,
    pub Length: u32,
    pub _bitfield: u32,
}
impl WHV_DOORBELL_MATCH_DATA {}
impl ::std::default::Default for WHV_DOORBELL_MATCH_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_DOORBELL_MATCH_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_DOORBELL_MATCH_DATA")
            .field("GuestAddress", &self.GuestAddress)
            .field("Value", &self.Value)
            .field("Length", &self.Length)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_DOORBELL_MATCH_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.GuestAddress == other.GuestAddress
            && self.Value == other.Value
            && self.Length == other.Length
            && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_DOORBELL_MATCH_DATA {}
unsafe impl ::windows::runtime::Abi for WHV_DOORBELL_MATCH_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct WHV_EMULATOR_CALLBACKS {
    pub Size: u32,
    pub Reserved: u32,
    pub WHvEmulatorIoPortCallback: ::std::option::Option<WHV_EMULATOR_IO_PORT_CALLBACK>,
    pub WHvEmulatorMemoryCallback: ::std::option::Option<WHV_EMULATOR_MEMORY_CALLBACK>,
    pub WHvEmulatorGetVirtualProcessorRegisters:
        ::std::option::Option<WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK>,
    pub WHvEmulatorSetVirtualProcessorRegisters:
        ::std::option::Option<WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK>,
    pub WHvEmulatorTranslateGvaPage:
        ::std::option::Option<WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK>,
}
impl WHV_EMULATOR_CALLBACKS {}
impl ::std::default::Default for WHV_EMULATOR_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_EMULATOR_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_EMULATOR_CALLBACKS")
            .field("Size", &self.Size)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_EMULATOR_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Reserved == other.Reserved
            && self.WHvEmulatorIoPortCallback.map(|f| f as usize)
                == other.WHvEmulatorIoPortCallback.map(|f| f as usize)
            && self.WHvEmulatorMemoryCallback.map(|f| f as usize)
                == other.WHvEmulatorMemoryCallback.map(|f| f as usize)
            && self
                .WHvEmulatorGetVirtualProcessorRegisters
                .map(|f| f as usize)
                == other
                    .WHvEmulatorGetVirtualProcessorRegisters
                    .map(|f| f as usize)
            && self
                .WHvEmulatorSetVirtualProcessorRegisters
                .map(|f| f as usize)
                == other
                    .WHvEmulatorSetVirtualProcessorRegisters
                    .map(|f| f as usize)
            && self.WHvEmulatorTranslateGvaPage.map(|f| f as usize)
                == other.WHvEmulatorTranslateGvaPage.map(|f| f as usize)
    }
}
impl ::std::cmp::Eq for WHV_EMULATOR_CALLBACKS {}
unsafe impl ::windows::runtime::Abi for WHV_EMULATOR_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub type WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK =
    unsafe extern "system" fn(
        context: *const ::std::ffi::c_void,
        registernames: *const WHV_REGISTER_NAME,
        registercount: u32,
        registervalues: *mut WHV_REGISTER_VALUE,
    ) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_EMULATOR_IO_ACCESS_INFO {
    pub Direction: u8,
    pub Port: u16,
    pub AccessSize: u16,
    pub Data: u32,
}
impl WHV_EMULATOR_IO_ACCESS_INFO {}
impl ::std::default::Default for WHV_EMULATOR_IO_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_EMULATOR_IO_ACCESS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_EMULATOR_IO_ACCESS_INFO")
            .field("Direction", &self.Direction)
            .field("Port", &self.Port)
            .field("AccessSize", &self.AccessSize)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_EMULATOR_IO_ACCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Direction == other.Direction
            && self.Port == other.Port
            && self.AccessSize == other.AccessSize
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WHV_EMULATOR_IO_ACCESS_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_EMULATOR_IO_ACCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub type WHV_EMULATOR_IO_PORT_CALLBACK = unsafe extern "system" fn(
    context: *const ::std::ffi::c_void,
    ioaccess: *mut WHV_EMULATOR_IO_ACCESS_INFO,
) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_EMULATOR_MEMORY_ACCESS_INFO {
    pub GpaAddress: u64,
    pub Direction: u8,
    pub AccessSize: u8,
    pub Data: [u8; 8],
}
impl WHV_EMULATOR_MEMORY_ACCESS_INFO {}
impl ::std::default::Default for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_EMULATOR_MEMORY_ACCESS_INFO")
            .field("GpaAddress", &self.GpaAddress)
            .field("Direction", &self.Direction)
            .field("AccessSize", &self.AccessSize)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.GpaAddress == other.GpaAddress
            && self.Direction == other.Direction
            && self.AccessSize == other.AccessSize
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WHV_EMULATOR_MEMORY_ACCESS_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub type WHV_EMULATOR_MEMORY_CALLBACK = unsafe extern "system" fn(
    context: *const ::std::ffi::c_void,
    memoryaccess: *mut WHV_EMULATOR_MEMORY_ACCESS_INFO,
) -> ::windows::runtime::HRESULT;
pub type WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK =
    unsafe extern "system" fn(
        context: *const ::std::ffi::c_void,
        registernames: *const WHV_REGISTER_NAME,
        registercount: u32,
        registervalues: *const WHV_REGISTER_VALUE,
    ) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_EMULATOR_STATUS {
    pub Anonymous: WHV_EMULATOR_STATUS_0,
    pub AsUINT32: u32,
}
impl WHV_EMULATOR_STATUS {}
impl ::std::default::Default for WHV_EMULATOR_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_EMULATOR_STATUS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_EMULATOR_STATUS {}
unsafe impl ::windows::runtime::Abi for WHV_EMULATOR_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_EMULATOR_STATUS_0 {
    pub _bitfield: u32,
}
impl WHV_EMULATOR_STATUS_0 {}
impl ::std::default::Default for WHV_EMULATOR_STATUS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_EMULATOR_STATUS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_EMULATOR_STATUS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_EMULATOR_STATUS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_EMULATOR_STATUS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK =
    unsafe extern "system" fn(
        context: *const ::std::ffi::c_void,
        gva: u64,
        translateflags: WHV_TRANSLATE_GVA_FLAGS,
        translationresult: *mut WHV_TRANSLATE_GVA_RESULT_CODE,
        gpa: *mut u64,
    ) -> ::windows::runtime::HRESULT;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WHV_EXCEPTION_TYPE(pub i32);
pub const WHvX64ExceptionTypeDivideErrorFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(0i32);
pub const WHvX64ExceptionTypeDebugTrapOrFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(1i32);
pub const WHvX64ExceptionTypeBreakpointTrap: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(3i32);
pub const WHvX64ExceptionTypeOverflowTrap: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(4i32);
pub const WHvX64ExceptionTypeBoundRangeFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(5i32);
pub const WHvX64ExceptionTypeInvalidOpcodeFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(6i32);
pub const WHvX64ExceptionTypeDeviceNotAvailableFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(7i32);
pub const WHvX64ExceptionTypeDoubleFaultAbort: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(8i32);
pub const WHvX64ExceptionTypeInvalidTaskStateSegmentFault: WHV_EXCEPTION_TYPE =
    WHV_EXCEPTION_TYPE(10i32);
pub const WHvX64ExceptionTypeSegmentNotPresentFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(11i32);
pub const WHvX64ExceptionTypeStackFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(12i32);
pub const WHvX64ExceptionTypeGeneralProtectionFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(13i32);
pub const WHvX64ExceptionTypePageFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(14i32);
pub const WHvX64ExceptionTypeFloatingPointErrorFault: WHV_EXCEPTION_TYPE =
    WHV_EXCEPTION_TYPE(16i32);
pub const WHvX64ExceptionTypeAlignmentCheckFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(17i32);
pub const WHvX64ExceptionTypeMachineCheckAbort: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(18i32);
pub const WHvX64ExceptionTypeSimdFloatingPointFault: WHV_EXCEPTION_TYPE = WHV_EXCEPTION_TYPE(19i32);
impl ::std::convert::From<i32> for WHV_EXCEPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_EXCEPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_EXTENDED_VM_EXITS {
    pub Anonymous: WHV_EXTENDED_VM_EXITS_0,
    pub AsUINT64: u64,
}
impl WHV_EXTENDED_VM_EXITS {}
impl ::std::default::Default for WHV_EXTENDED_VM_EXITS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_EXTENDED_VM_EXITS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_EXTENDED_VM_EXITS {}
unsafe impl ::windows::runtime::Abi for WHV_EXTENDED_VM_EXITS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_EXTENDED_VM_EXITS_0 {
    pub _bitfield: u64,
}
impl WHV_EXTENDED_VM_EXITS_0 {}
impl ::std::default::Default for WHV_EXTENDED_VM_EXITS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_EXTENDED_VM_EXITS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_EXTENDED_VM_EXITS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_EXTENDED_VM_EXITS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_EXTENDED_VM_EXITS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_HYPERCALL_CONTEXT {
    pub Rax: u64,
    pub Rbx: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub R8: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Reserved0: u64,
    pub XmmRegisters: [WHV_UINT128; 6],
    pub Reserved1: [u64; 2],
}
impl WHV_HYPERCALL_CONTEXT {}
impl ::std::default::Default for WHV_HYPERCALL_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_HYPERCALL_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_HYPERCALL_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_HYPERCALL_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_HYPERCALL_CONTEXT_MAX_XMM_REGISTERS: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_INTERNAL_ACTIVITY_REGISTER {
    pub Anonymous: WHV_INTERNAL_ACTIVITY_REGISTER_0,
    pub AsUINT64: u64,
}
impl WHV_INTERNAL_ACTIVITY_REGISTER {}
impl ::std::default::Default for WHV_INTERNAL_ACTIVITY_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_INTERNAL_ACTIVITY_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_INTERNAL_ACTIVITY_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_INTERNAL_ACTIVITY_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    pub _bitfield: u64,
}
impl WHV_INTERNAL_ACTIVITY_REGISTER_0 {}
impl ::std::default::Default for WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_INTERNAL_ACTIVITY_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_INTERRUPT_CONTROL {
    pub _bitfield: u64,
    pub Destination: u32,
    pub Vector: u32,
}
impl WHV_INTERRUPT_CONTROL {}
impl ::std::default::Default for WHV_INTERRUPT_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_INTERRUPT_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_INTERRUPT_CONTROL")
            .field("_bitfield", &self._bitfield)
            .field("Destination", &self.Destination)
            .field("Vector", &self.Vector)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_INTERRUPT_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.Destination == other.Destination
            && self.Vector == other.Vector
    }
}
impl ::std::cmp::Eq for WHV_INTERRUPT_CONTROL {}
unsafe impl ::windows::runtime::Abi for WHV_INTERRUPT_CONTROL {
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
pub struct WHV_INTERRUPT_DESTINATION_MODE(pub i32);
pub const WHvX64InterruptDestinationModePhysical: WHV_INTERRUPT_DESTINATION_MODE =
    WHV_INTERRUPT_DESTINATION_MODE(0i32);
pub const WHvX64InterruptDestinationModeLogical: WHV_INTERRUPT_DESTINATION_MODE =
    WHV_INTERRUPT_DESTINATION_MODE(1i32);
impl ::std::convert::From<i32> for WHV_INTERRUPT_DESTINATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_INTERRUPT_DESTINATION_MODE {
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
pub struct WHV_INTERRUPT_TRIGGER_MODE(pub i32);
pub const WHvX64InterruptTriggerModeEdge: WHV_INTERRUPT_TRIGGER_MODE =
    WHV_INTERRUPT_TRIGGER_MODE(0i32);
pub const WHvX64InterruptTriggerModeLevel: WHV_INTERRUPT_TRIGGER_MODE =
    WHV_INTERRUPT_TRIGGER_MODE(1i32);
impl ::std::convert::From<i32> for WHV_INTERRUPT_TRIGGER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_INTERRUPT_TRIGGER_MODE {
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
pub struct WHV_INTERRUPT_TYPE(pub i32);
pub const WHvX64InterruptTypeFixed: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(0i32);
pub const WHvX64InterruptTypeLowestPriority: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(1i32);
pub const WHvX64InterruptTypeNmi: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(4i32);
pub const WHvX64InterruptTypeInit: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(5i32);
pub const WHvX64InterruptTypeSipi: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(6i32);
pub const WHvX64InterruptTypeLocalInt1: WHV_INTERRUPT_TYPE = WHV_INTERRUPT_TYPE(9i32);
impl ::std::convert::From<i32> for WHV_INTERRUPT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_INTERRUPT_TYPE {
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
pub struct WHV_MAP_GPA_RANGE_FLAGS(pub u32);
pub const WHvMapGpaRangeFlagNone: WHV_MAP_GPA_RANGE_FLAGS = WHV_MAP_GPA_RANGE_FLAGS(0u32);
pub const WHvMapGpaRangeFlagRead: WHV_MAP_GPA_RANGE_FLAGS = WHV_MAP_GPA_RANGE_FLAGS(1u32);
pub const WHvMapGpaRangeFlagWrite: WHV_MAP_GPA_RANGE_FLAGS = WHV_MAP_GPA_RANGE_FLAGS(2u32);
pub const WHvMapGpaRangeFlagExecute: WHV_MAP_GPA_RANGE_FLAGS = WHV_MAP_GPA_RANGE_FLAGS(4u32);
pub const WHvMapGpaRangeFlagTrackDirtyPages: WHV_MAP_GPA_RANGE_FLAGS =
    WHV_MAP_GPA_RANGE_FLAGS(8u32);
impl ::std::convert::From<u32> for WHV_MAP_GPA_RANGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_MAP_GPA_RANGE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_MAP_GPA_RANGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_MAP_GPA_RANGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WHV_MAX_DEVICE_ID_SIZE_IN_CHARS: u32 = 200u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_MEMORY_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_MEMORY_ACCESS_INFO,
    pub Gpa: u64,
    pub Gva: u64,
}
impl WHV_MEMORY_ACCESS_CONTEXT {}
impl ::std::default::Default for WHV_MEMORY_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_MEMORY_ACCESS_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_MEMORY_ACCESS_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_MEMORY_ACCESS_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_MEMORY_ACCESS_INFO {
    pub Anonymous: WHV_MEMORY_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl WHV_MEMORY_ACCESS_INFO {}
impl ::std::default::Default for WHV_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_MEMORY_ACCESS_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_MEMORY_ACCESS_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_MEMORY_ACCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_MEMORY_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl WHV_MEMORY_ACCESS_INFO_0 {}
impl ::std::default::Default for WHV_MEMORY_ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_MEMORY_ACCESS_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_MEMORY_ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_MEMORY_ACCESS_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WHV_MEMORY_ACCESS_INFO_0 {
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
pub struct WHV_MEMORY_ACCESS_TYPE(pub i32);
pub const WHvMemoryAccessRead: WHV_MEMORY_ACCESS_TYPE = WHV_MEMORY_ACCESS_TYPE(0i32);
pub const WHvMemoryAccessWrite: WHV_MEMORY_ACCESS_TYPE = WHV_MEMORY_ACCESS_TYPE(1i32);
pub const WHvMemoryAccessExecute: WHV_MEMORY_ACCESS_TYPE = WHV_MEMORY_ACCESS_TYPE(2i32);
impl ::std::convert::From<i32> for WHV_MEMORY_ACCESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_MEMORY_ACCESS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_MEMORY_RANGE_ENTRY {
    pub GuestAddress: u64,
    pub SizeInBytes: u64,
}
impl WHV_MEMORY_RANGE_ENTRY {}
impl ::std::default::Default for WHV_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_MEMORY_RANGE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_MEMORY_RANGE_ENTRY")
            .field("GuestAddress", &self.GuestAddress)
            .field("SizeInBytes", &self.SizeInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.GuestAddress == other.GuestAddress && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::std::cmp::Eq for WHV_MEMORY_RANGE_ENTRY {}
unsafe impl ::windows::runtime::Abi for WHV_MEMORY_RANGE_ENTRY {
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
pub struct WHV_MSR_ACTION(pub i32);
pub const WHvMsrActionArchitectureDefault: WHV_MSR_ACTION = WHV_MSR_ACTION(0i32);
pub const WHvMsrActionIgnoreWriteReadZero: WHV_MSR_ACTION = WHV_MSR_ACTION(1i32);
pub const WHvMsrActionExit: WHV_MSR_ACTION = WHV_MSR_ACTION(2i32);
impl ::std::convert::From<i32> for WHV_MSR_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_MSR_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_MSR_ACTION_ENTRY {
    pub Index: u32,
    pub ReadAction: u8,
    pub WriteAction: u8,
    pub Reserved: u16,
}
impl WHV_MSR_ACTION_ENTRY {}
impl ::std::default::Default for WHV_MSR_ACTION_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_MSR_ACTION_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_MSR_ACTION_ENTRY")
            .field("Index", &self.Index)
            .field("ReadAction", &self.ReadAction)
            .field("WriteAction", &self.WriteAction)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_MSR_ACTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index
            && self.ReadAction == other.ReadAction
            && self.WriteAction == other.WriteAction
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WHV_MSR_ACTION_ENTRY {}
unsafe impl ::windows::runtime::Abi for WHV_MSR_ACTION_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS {
    pub NotificationPortType: WHV_NOTIFICATION_PORT_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_NOTIFICATION_PORT_PARAMETERS_0,
}
impl WHV_NOTIFICATION_PORT_PARAMETERS {}
impl ::std::default::Default for WHV_NOTIFICATION_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_NOTIFICATION_PORT_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_NOTIFICATION_PORT_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for WHV_NOTIFICATION_PORT_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    pub Doorbell: WHV_DOORBELL_MATCH_DATA,
    pub Event: WHV_NOTIFICATION_PORT_PARAMETERS_0_0,
}
impl WHV_NOTIFICATION_PORT_PARAMETERS_0 {}
impl ::std::default::Default for WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_NOTIFICATION_PORT_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    pub ConnectionId: u32,
}
impl WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {}
impl ::std::default::Default for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Event_e__Struct")
            .field("ConnectionId", &self.ConnectionId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId
    }
}
impl ::std::cmp::Eq for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
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
pub struct WHV_NOTIFICATION_PORT_PROPERTY_CODE(pub i32);
pub const WHvNotificationPortPropertyPreferredTargetVp: WHV_NOTIFICATION_PORT_PROPERTY_CODE =
    WHV_NOTIFICATION_PORT_PROPERTY_CODE(1i32);
pub const WHvNotificationPortPropertyPreferredTargetDuration: WHV_NOTIFICATION_PORT_PROPERTY_CODE =
    WHV_NOTIFICATION_PORT_PROPERTY_CODE(5i32);
impl ::std::convert::From<i32> for WHV_NOTIFICATION_PORT_PROPERTY_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_NOTIFICATION_PORT_PROPERTY_CODE {
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
pub struct WHV_NOTIFICATION_PORT_TYPE(pub i32);
pub const WHvNotificationPortTypeEvent: WHV_NOTIFICATION_PORT_TYPE =
    WHV_NOTIFICATION_PORT_TYPE(2i32);
pub const WHvNotificationPortTypeDoorbell: WHV_NOTIFICATION_PORT_TYPE =
    WHV_NOTIFICATION_PORT_TYPE(4i32);
impl ::std::convert::From<i32> for WHV_NOTIFICATION_PORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_NOTIFICATION_PORT_TYPE {
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
pub struct WHV_PARTITION_COUNTER_SET(pub i32);
pub const WHvPartitionCounterSetMemory: WHV_PARTITION_COUNTER_SET = WHV_PARTITION_COUNTER_SET(0i32);
impl ::std::convert::From<i32> for WHV_PARTITION_COUNTER_SET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_PARTITION_COUNTER_SET {
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
pub struct WHV_PARTITION_HANDLE(pub isize);
impl ::std::default::Default for WHV_PARTITION_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for WHV_PARTITION_HANDLE {}
unsafe impl ::windows::runtime::Abi for WHV_PARTITION_HANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PARTITION_MEMORY_COUNTERS {
    pub Mapped4KPageCount: u64,
    pub Mapped2MPageCount: u64,
    pub Mapped1GPageCount: u64,
}
impl WHV_PARTITION_MEMORY_COUNTERS {}
impl ::std::default::Default for WHV_PARTITION_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PARTITION_MEMORY_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PARTITION_MEMORY_COUNTERS")
            .field("Mapped4KPageCount", &self.Mapped4KPageCount)
            .field("Mapped2MPageCount", &self.Mapped2MPageCount)
            .field("Mapped1GPageCount", &self.Mapped1GPageCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PARTITION_MEMORY_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.Mapped4KPageCount == other.Mapped4KPageCount
            && self.Mapped2MPageCount == other.Mapped2MPageCount
            && self.Mapped1GPageCount == other.Mapped1GPageCount
    }
}
impl ::std::cmp::Eq for WHV_PARTITION_MEMORY_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PARTITION_MEMORY_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WHV_PARTITION_PROPERTY {
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub ProcessorClFlushSize: u8,
    pub ProcessorCount: u32,
    pub CpuidExitList: [u32; 1],
    pub CpuidResultList: [WHV_X64_CPUID_RESULT; 1],
    pub CpuidResultList2: [WHV_X64_CPUID_RESULT2; 1],
    pub MsrActionList: [WHV_MSR_ACTION_ENTRY; 1],
    pub UnimplementedMsrAction: WHV_MSR_ACTION,
    pub ExceptionExitBitmap: u64,
    pub LocalApicEmulationMode: WHV_X64_LOCAL_APIC_EMULATION_MODE,
    pub SeparateSecurityDomain: super::super::Foundation::BOOL,
    pub NestedVirtualization: super::super::Foundation::BOOL,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ProcessorClockFrequency: u64,
    pub InterruptClockFrequency: u64,
    pub ApicRemoteRead: super::super::Foundation::BOOL,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub ReferenceTime: u64,
    pub PrimaryNumaNode: u16,
    pub CpuReserve: u32,
    pub CpuCap: u32,
    pub CpuWeight: u32,
    pub CpuGroupId: u64,
    pub ProcessorFrequencyCap: u32,
    pub AllowDeviceAssignment: super::super::Foundation::BOOL,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub DisableSmt: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WHV_PARTITION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WHV_PARTITION_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WHV_PARTITION_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WHV_PARTITION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WHV_PARTITION_PROPERTY {
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
pub struct WHV_PARTITION_PROPERTY_CODE(pub i32);
pub const WHvPartitionPropertyCodeExtendedVmExits: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(1i32);
pub const WHvPartitionPropertyCodeExceptionExitBitmap: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(2i32);
pub const WHvPartitionPropertyCodeSeparateSecurityDomain: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(3i32);
pub const WHvPartitionPropertyCodeNestedVirtualization: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4i32);
pub const WHvPartitionPropertyCodeX64MsrExitBitmap: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(5i32);
pub const WHvPartitionPropertyCodePrimaryNumaNode: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(6i32);
pub const WHvPartitionPropertyCodeCpuReserve: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(7i32);
pub const WHvPartitionPropertyCodeCpuCap: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(8i32);
pub const WHvPartitionPropertyCodeCpuWeight: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(9i32);
pub const WHvPartitionPropertyCodeCpuGroupId: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(10i32);
pub const WHvPartitionPropertyCodeProcessorFrequencyCap: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(11i32);
pub const WHvPartitionPropertyCodeAllowDeviceAssignment: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(12i32);
pub const WHvPartitionPropertyCodeDisableSmt: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(13i32);
pub const WHvPartitionPropertyCodeProcessorFeatures: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4097i32);
pub const WHvPartitionPropertyCodeProcessorClFlushSize: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4098i32);
pub const WHvPartitionPropertyCodeCpuidExitList: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4099i32);
pub const WHvPartitionPropertyCodeCpuidResultList: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4100i32);
pub const WHvPartitionPropertyCodeLocalApicEmulationMode: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4101i32);
pub const WHvPartitionPropertyCodeProcessorXsaveFeatures: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4102i32);
pub const WHvPartitionPropertyCodeProcessorClockFrequency: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4103i32);
pub const WHvPartitionPropertyCodeInterruptClockFrequency: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4104i32);
pub const WHvPartitionPropertyCodeApicRemoteReadSupport: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4105i32);
pub const WHvPartitionPropertyCodeProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4106i32);
pub const WHvPartitionPropertyCodeReferenceTime: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4107i32);
pub const WHvPartitionPropertyCodeSyntheticProcessorFeaturesBanks: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4108i32);
pub const WHvPartitionPropertyCodeCpuidResultList2: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4109i32);
pub const WHvPartitionPropertyCodeProcessorPerfmonFeatures: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4110i32);
pub const WHvPartitionPropertyCodeMsrActionList: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4111i32);
pub const WHvPartitionPropertyCodeUnimplementedMsrAction: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(4112i32);
pub const WHvPartitionPropertyCodeProcessorCount: WHV_PARTITION_PROPERTY_CODE =
    WHV_PARTITION_PROPERTY_CODE(8191i32);
impl ::std::convert::From<i32> for WHV_PARTITION_PROPERTY_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_PARTITION_PROPERTY_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_APIC_COUNTERS {
    pub MmioAccessCount: u64,
    pub EoiAccessCount: u64,
    pub TprAccessCount: u64,
    pub SentIpiCount: u64,
    pub SelfIpiCount: u64,
}
impl WHV_PROCESSOR_APIC_COUNTERS {}
impl ::std::default::Default for WHV_PROCESSOR_APIC_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_APIC_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_APIC_COUNTERS")
            .field("MmioAccessCount", &self.MmioAccessCount)
            .field("EoiAccessCount", &self.EoiAccessCount)
            .field("TprAccessCount", &self.TprAccessCount)
            .field("SentIpiCount", &self.SentIpiCount)
            .field("SelfIpiCount", &self.SelfIpiCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_APIC_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.MmioAccessCount == other.MmioAccessCount
            && self.EoiAccessCount == other.EoiAccessCount
            && self.TprAccessCount == other.TprAccessCount
            && self.SentIpiCount == other.SentIpiCount
            && self.SelfIpiCount == other.SelfIpiCount
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_APIC_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_APIC_COUNTERS {
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
pub struct WHV_PROCESSOR_COUNTER_SET(pub i32);
pub const WHvProcessorCounterSetRuntime: WHV_PROCESSOR_COUNTER_SET =
    WHV_PROCESSOR_COUNTER_SET(0i32);
pub const WHvProcessorCounterSetIntercepts: WHV_PROCESSOR_COUNTER_SET =
    WHV_PROCESSOR_COUNTER_SET(1i32);
pub const WHvProcessorCounterSetEvents: WHV_PROCESSOR_COUNTER_SET = WHV_PROCESSOR_COUNTER_SET(2i32);
pub const WHvProcessorCounterSetApic: WHV_PROCESSOR_COUNTER_SET = WHV_PROCESSOR_COUNTER_SET(3i32);
pub const WHvProcessorCounterSetSyntheticFeatures: WHV_PROCESSOR_COUNTER_SET =
    WHV_PROCESSOR_COUNTER_SET(4i32);
impl ::std::convert::From<i32> for WHV_PROCESSOR_COUNTER_SET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_COUNTER_SET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_EVENT_COUNTERS {
    pub PageFaultCount: u64,
    pub ExceptionCount: u64,
    pub InterruptCount: u64,
}
impl WHV_PROCESSOR_EVENT_COUNTERS {}
impl ::std::default::Default for WHV_PROCESSOR_EVENT_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_EVENT_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_EVENT_COUNTERS")
            .field("PageFaultCount", &self.PageFaultCount)
            .field("ExceptionCount", &self.ExceptionCount)
            .field("InterruptCount", &self.InterruptCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_EVENT_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.PageFaultCount == other.PageFaultCount
            && self.ExceptionCount == other.ExceptionCount
            && self.InterruptCount == other.InterruptCount
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_EVENT_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_EVENT_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES {
    pub Anonymous: WHV_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_PROCESSOR_FEATURES {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_PROCESSOR_FEATURES_0 {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES1 {
    pub Anonymous: WHV_PROCESSOR_FEATURES1_0,
    pub AsUINT64: u64,
}
impl WHV_PROCESSOR_FEATURES1 {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES1 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES1_0 {
    pub _bitfield: u64,
}
impl WHV_PROCESSOR_FEATURES1_0 {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_FEATURES1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES1_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0,
}
impl WHV_PROCESSOR_FEATURES_BANKS {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES_BANKS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES_BANKS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES_BANKS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 2],
}
impl WHV_PROCESSOR_FEATURES_BANKS_0 {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES_BANKS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES_BANKS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES_BANKS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES_BANKS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_PROCESSOR_FEATURES,
    pub Bank1: WHV_PROCESSOR_FEATURES1,
}
impl WHV_PROCESSOR_FEATURES_BANKS_0_0 {}
impl ::std::default::Default for WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_FEATURES_BANKS_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_INTERCEPT_COUNTER {
    pub Count: u64,
    pub Time100ns: u64,
}
impl WHV_PROCESSOR_INTERCEPT_COUNTER {}
impl ::std::default::Default for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_INTERCEPT_COUNTER")
            .field("Count", &self.Count)
            .field("Time100ns", &self.Time100ns)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Time100ns == other.Time100ns
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_INTERCEPT_COUNTER {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_INTERCEPT_COUNTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_INTERCEPT_COUNTERS {
    pub PageInvalidations: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub ControlRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub IoInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub HaltInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub CpuidInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub MsrAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub OtherIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PendingInterrupts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub EmulatedInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub DebugRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub NestedPageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub Hypercalls: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub RdpmcInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
}
impl WHV_PROCESSOR_INTERCEPT_COUNTERS {}
impl ::std::default::Default for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_INTERCEPT_COUNTERS")
            .field("PageInvalidations", &self.PageInvalidations)
            .field("ControlRegisterAccesses", &self.ControlRegisterAccesses)
            .field("IoInstructions", &self.IoInstructions)
            .field("HaltInstructions", &self.HaltInstructions)
            .field("CpuidInstructions", &self.CpuidInstructions)
            .field("MsrAccesses", &self.MsrAccesses)
            .field("OtherIntercepts", &self.OtherIntercepts)
            .field("PendingInterrupts", &self.PendingInterrupts)
            .field("EmulatedInstructions", &self.EmulatedInstructions)
            .field("DebugRegisterAccesses", &self.DebugRegisterAccesses)
            .field("PageFaultIntercepts", &self.PageFaultIntercepts)
            .field("NestedPageFaultIntercepts", &self.NestedPageFaultIntercepts)
            .field("Hypercalls", &self.Hypercalls)
            .field("RdpmcInstructions", &self.RdpmcInstructions)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.PageInvalidations == other.PageInvalidations
            && self.ControlRegisterAccesses == other.ControlRegisterAccesses
            && self.IoInstructions == other.IoInstructions
            && self.HaltInstructions == other.HaltInstructions
            && self.CpuidInstructions == other.CpuidInstructions
            && self.MsrAccesses == other.MsrAccesses
            && self.OtherIntercepts == other.OtherIntercepts
            && self.PendingInterrupts == other.PendingInterrupts
            && self.EmulatedInstructions == other.EmulatedInstructions
            && self.DebugRegisterAccesses == other.DebugRegisterAccesses
            && self.PageFaultIntercepts == other.PageFaultIntercepts
            && self.NestedPageFaultIntercepts == other.NestedPageFaultIntercepts
            && self.Hypercalls == other.Hypercalls
            && self.RdpmcInstructions == other.RdpmcInstructions
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_INTERCEPT_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_PROCESSOR_PERFMON_FEATURES {
    pub Anonymous: WHV_PROCESSOR_PERFMON_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_PROCESSOR_PERFMON_FEATURES {}
impl ::std::default::Default for WHV_PROCESSOR_PERFMON_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_PERFMON_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_PERFMON_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_PERFMON_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_PERFMON_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_PROCESSOR_PERFMON_FEATURES_0 {}
impl ::std::default::Default for WHV_PROCESSOR_PERFMON_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_PERFMON_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_PERFMON_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_PERFMON_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_PERFMON_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_RUNTIME_COUNTERS {
    pub TotalRuntime100ns: u64,
    pub HypervisorRuntime100ns: u64,
}
impl WHV_PROCESSOR_RUNTIME_COUNTERS {}
impl ::std::default::Default for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_RUNTIME_COUNTERS")
            .field("TotalRuntime100ns", &self.TotalRuntime100ns)
            .field("HypervisorRuntime100ns", &self.HypervisorRuntime100ns)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.TotalRuntime100ns == other.TotalRuntime100ns
            && self.HypervisorRuntime100ns == other.HypervisorRuntime100ns
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_RUNTIME_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_RUNTIME_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    pub SyntheticInterruptsCount: u64,
    pub LongSpinWaitHypercallsCount: u64,
    pub OtherHypercallsCount: u64,
    pub SyntheticInterruptHypercallsCount: u64,
    pub VirtualInterruptHypercallsCount: u64,
    pub VirtualMmuHypercallsCount: u64,
}
impl WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {}
impl ::std::default::Default for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS")
            .field("SyntheticInterruptsCount", &self.SyntheticInterruptsCount)
            .field(
                "LongSpinWaitHypercallsCount",
                &self.LongSpinWaitHypercallsCount,
            )
            .field("OtherHypercallsCount", &self.OtherHypercallsCount)
            .field(
                "SyntheticInterruptHypercallsCount",
                &self.SyntheticInterruptHypercallsCount,
            )
            .field(
                "VirtualInterruptHypercallsCount",
                &self.VirtualInterruptHypercallsCount,
            )
            .field("VirtualMmuHypercallsCount", &self.VirtualMmuHypercallsCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.SyntheticInterruptsCount == other.SyntheticInterruptsCount
            && self.LongSpinWaitHypercallsCount == other.LongSpinWaitHypercallsCount
            && self.OtherHypercallsCount == other.OtherHypercallsCount
            && self.SyntheticInterruptHypercallsCount == other.SyntheticInterruptHypercallsCount
            && self.VirtualInterruptHypercallsCount == other.VirtualInterruptHypercallsCount
            && self.VirtualMmuHypercallsCount == other.VirtualMmuHypercallsCount
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
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
pub struct WHV_PROCESSOR_VENDOR(pub i32);
pub const WHvProcessorVendorAmd: WHV_PROCESSOR_VENDOR = WHV_PROCESSOR_VENDOR(0i32);
pub const WHvProcessorVendorIntel: WHV_PROCESSOR_VENDOR = WHV_PROCESSOR_VENDOR(1i32);
pub const WHvProcessorVendorHygon: WHV_PROCESSOR_VENDOR = WHV_PROCESSOR_VENDOR(2i32);
impl ::std::convert::From<i32> for WHV_PROCESSOR_VENDOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_VENDOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_PROCESSOR_XSAVE_FEATURES {
    pub Anonymous: WHV_PROCESSOR_XSAVE_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_PROCESSOR_XSAVE_FEATURES {}
impl ::std::default::Default for WHV_PROCESSOR_XSAVE_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_XSAVE_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_XSAVE_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_XSAVE_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_PROCESSOR_XSAVE_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_PROCESSOR_XSAVE_FEATURES_0 {}
impl ::std::default::Default for WHV_PROCESSOR_XSAVE_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_PROCESSOR_XSAVE_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_PROCESSOR_XSAVE_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_PROCESSOR_XSAVE_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_PROCESSOR_XSAVE_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_READ_WRITE_GPA_RANGE_MAX_SIZE: u32 = 16u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WHV_REGISTER_NAME(pub i32);
pub const WHvX64RegisterRax: WHV_REGISTER_NAME = WHV_REGISTER_NAME(0i32);
pub const WHvX64RegisterRcx: WHV_REGISTER_NAME = WHV_REGISTER_NAME(1i32);
pub const WHvX64RegisterRdx: WHV_REGISTER_NAME = WHV_REGISTER_NAME(2i32);
pub const WHvX64RegisterRbx: WHV_REGISTER_NAME = WHV_REGISTER_NAME(3i32);
pub const WHvX64RegisterRsp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4i32);
pub const WHvX64RegisterRbp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(5i32);
pub const WHvX64RegisterRsi: WHV_REGISTER_NAME = WHV_REGISTER_NAME(6i32);
pub const WHvX64RegisterRdi: WHV_REGISTER_NAME = WHV_REGISTER_NAME(7i32);
pub const WHvX64RegisterR8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8i32);
pub const WHvX64RegisterR9: WHV_REGISTER_NAME = WHV_REGISTER_NAME(9i32);
pub const WHvX64RegisterR10: WHV_REGISTER_NAME = WHV_REGISTER_NAME(10i32);
pub const WHvX64RegisterR11: WHV_REGISTER_NAME = WHV_REGISTER_NAME(11i32);
pub const WHvX64RegisterR12: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12i32);
pub const WHvX64RegisterR13: WHV_REGISTER_NAME = WHV_REGISTER_NAME(13i32);
pub const WHvX64RegisterR14: WHV_REGISTER_NAME = WHV_REGISTER_NAME(14i32);
pub const WHvX64RegisterR15: WHV_REGISTER_NAME = WHV_REGISTER_NAME(15i32);
pub const WHvX64RegisterRip: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16i32);
pub const WHvX64RegisterRflags: WHV_REGISTER_NAME = WHV_REGISTER_NAME(17i32);
pub const WHvX64RegisterEs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(18i32);
pub const WHvX64RegisterCs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(19i32);
pub const WHvX64RegisterSs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20i32);
pub const WHvX64RegisterDs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(21i32);
pub const WHvX64RegisterFs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(22i32);
pub const WHvX64RegisterGs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(23i32);
pub const WHvX64RegisterLdtr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(24i32);
pub const WHvX64RegisterTr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(25i32);
pub const WHvX64RegisterIdtr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(26i32);
pub const WHvX64RegisterGdtr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(27i32);
pub const WHvX64RegisterCr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(28i32);
pub const WHvX64RegisterCr2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(29i32);
pub const WHvX64RegisterCr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(30i32);
pub const WHvX64RegisterCr4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(31i32);
pub const WHvX64RegisterCr8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(32i32);
pub const WHvX64RegisterDr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(33i32);
pub const WHvX64RegisterDr1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(34i32);
pub const WHvX64RegisterDr2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(35i32);
pub const WHvX64RegisterDr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(36i32);
pub const WHvX64RegisterDr6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(37i32);
pub const WHvX64RegisterDr7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(38i32);
pub const WHvX64RegisterXCr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(39i32);
pub const WHvX64RegisterVirtualCr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(40i32);
pub const WHvX64RegisterVirtualCr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(41i32);
pub const WHvX64RegisterVirtualCr4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(42i32);
pub const WHvX64RegisterVirtualCr8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(43i32);
pub const WHvX64RegisterXmm0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4096i32);
pub const WHvX64RegisterXmm1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4097i32);
pub const WHvX64RegisterXmm2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4098i32);
pub const WHvX64RegisterXmm3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4099i32);
pub const WHvX64RegisterXmm4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4100i32);
pub const WHvX64RegisterXmm5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4101i32);
pub const WHvX64RegisterXmm6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4102i32);
pub const WHvX64RegisterXmm7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4103i32);
pub const WHvX64RegisterXmm8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4104i32);
pub const WHvX64RegisterXmm9: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4105i32);
pub const WHvX64RegisterXmm10: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4106i32);
pub const WHvX64RegisterXmm11: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4107i32);
pub const WHvX64RegisterXmm12: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4108i32);
pub const WHvX64RegisterXmm13: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4109i32);
pub const WHvX64RegisterXmm14: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4110i32);
pub const WHvX64RegisterXmm15: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4111i32);
pub const WHvX64RegisterFpMmx0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4112i32);
pub const WHvX64RegisterFpMmx1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4113i32);
pub const WHvX64RegisterFpMmx2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4114i32);
pub const WHvX64RegisterFpMmx3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4115i32);
pub const WHvX64RegisterFpMmx4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4116i32);
pub const WHvX64RegisterFpMmx5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4117i32);
pub const WHvX64RegisterFpMmx6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4118i32);
pub const WHvX64RegisterFpMmx7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4119i32);
pub const WHvX64RegisterFpControlStatus: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4120i32);
pub const WHvX64RegisterXmmControlStatus: WHV_REGISTER_NAME = WHV_REGISTER_NAME(4121i32);
pub const WHvX64RegisterTsc: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8192i32);
pub const WHvX64RegisterEfer: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8193i32);
pub const WHvX64RegisterKernelGsBase: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8194i32);
pub const WHvX64RegisterApicBase: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8195i32);
pub const WHvX64RegisterPat: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8196i32);
pub const WHvX64RegisterSysenterCs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8197i32);
pub const WHvX64RegisterSysenterEip: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8198i32);
pub const WHvX64RegisterSysenterEsp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8199i32);
pub const WHvX64RegisterStar: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8200i32);
pub const WHvX64RegisterLstar: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8201i32);
pub const WHvX64RegisterCstar: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8202i32);
pub const WHvX64RegisterSfmask: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8203i32);
pub const WHvX64RegisterInitialApicId: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8204i32);
pub const WHvX64RegisterMsrMtrrCap: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8205i32);
pub const WHvX64RegisterMsrMtrrDefType: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8206i32);
pub const WHvX64RegisterMsrMtrrPhysBase0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8208i32);
pub const WHvX64RegisterMsrMtrrPhysBase1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8209i32);
pub const WHvX64RegisterMsrMtrrPhysBase2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8210i32);
pub const WHvX64RegisterMsrMtrrPhysBase3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8211i32);
pub const WHvX64RegisterMsrMtrrPhysBase4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8212i32);
pub const WHvX64RegisterMsrMtrrPhysBase5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8213i32);
pub const WHvX64RegisterMsrMtrrPhysBase6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8214i32);
pub const WHvX64RegisterMsrMtrrPhysBase7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8215i32);
pub const WHvX64RegisterMsrMtrrPhysBase8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8216i32);
pub const WHvX64RegisterMsrMtrrPhysBase9: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8217i32);
pub const WHvX64RegisterMsrMtrrPhysBaseA: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8218i32);
pub const WHvX64RegisterMsrMtrrPhysBaseB: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8219i32);
pub const WHvX64RegisterMsrMtrrPhysBaseC: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8220i32);
pub const WHvX64RegisterMsrMtrrPhysBaseD: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8221i32);
pub const WHvX64RegisterMsrMtrrPhysBaseE: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8222i32);
pub const WHvX64RegisterMsrMtrrPhysBaseF: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8223i32);
pub const WHvX64RegisterMsrMtrrPhysMask0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8256i32);
pub const WHvX64RegisterMsrMtrrPhysMask1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8257i32);
pub const WHvX64RegisterMsrMtrrPhysMask2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8258i32);
pub const WHvX64RegisterMsrMtrrPhysMask3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8259i32);
pub const WHvX64RegisterMsrMtrrPhysMask4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8260i32);
pub const WHvX64RegisterMsrMtrrPhysMask5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8261i32);
pub const WHvX64RegisterMsrMtrrPhysMask6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8262i32);
pub const WHvX64RegisterMsrMtrrPhysMask7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8263i32);
pub const WHvX64RegisterMsrMtrrPhysMask8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8264i32);
pub const WHvX64RegisterMsrMtrrPhysMask9: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8265i32);
pub const WHvX64RegisterMsrMtrrPhysMaskA: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8266i32);
pub const WHvX64RegisterMsrMtrrPhysMaskB: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8267i32);
pub const WHvX64RegisterMsrMtrrPhysMaskC: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8268i32);
pub const WHvX64RegisterMsrMtrrPhysMaskD: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8269i32);
pub const WHvX64RegisterMsrMtrrPhysMaskE: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8270i32);
pub const WHvX64RegisterMsrMtrrPhysMaskF: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8271i32);
pub const WHvX64RegisterMsrMtrrFix64k00000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8304i32);
pub const WHvX64RegisterMsrMtrrFix16k80000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8305i32);
pub const WHvX64RegisterMsrMtrrFix16kA0000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8306i32);
pub const WHvX64RegisterMsrMtrrFix4kC0000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8307i32);
pub const WHvX64RegisterMsrMtrrFix4kC8000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8308i32);
pub const WHvX64RegisterMsrMtrrFix4kD0000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8309i32);
pub const WHvX64RegisterMsrMtrrFix4kD8000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8310i32);
pub const WHvX64RegisterMsrMtrrFix4kE0000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8311i32);
pub const WHvX64RegisterMsrMtrrFix4kE8000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8312i32);
pub const WHvX64RegisterMsrMtrrFix4kF0000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8313i32);
pub const WHvX64RegisterMsrMtrrFix4kF8000: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8314i32);
pub const WHvX64RegisterTscAux: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8315i32);
pub const WHvX64RegisterBndcfgs: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8316i32);
pub const WHvX64RegisterMCount: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8318i32);
pub const WHvX64RegisterACount: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8319i32);
pub const WHvX64RegisterSpecCtrl: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8324i32);
pub const WHvX64RegisterPredCmd: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8325i32);
pub const WHvX64RegisterTscVirtualOffset: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8327i32);
pub const WHvX64RegisterTsxCtrl: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8328i32);
pub const WHvX64RegisterXss: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8331i32);
pub const WHvX64RegisterUCet: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8332i32);
pub const WHvX64RegisterSCet: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8333i32);
pub const WHvX64RegisterSsp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8334i32);
pub const WHvX64RegisterPl0Ssp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8335i32);
pub const WHvX64RegisterPl1Ssp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8336i32);
pub const WHvX64RegisterPl2Ssp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8337i32);
pub const WHvX64RegisterPl3Ssp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8338i32);
pub const WHvX64RegisterInterruptSspTableAddr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8339i32);
pub const WHvX64RegisterTscDeadline: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8341i32);
pub const WHvX64RegisterTscAdjust: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8342i32);
pub const WHvX64RegisterUmwaitControl: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8344i32);
pub const WHvX64RegisterXfd: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8345i32);
pub const WHvX64RegisterXfdErr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(8346i32);
pub const WHvX64RegisterApicId: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12290i32);
pub const WHvX64RegisterApicVersion: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12291i32);
pub const WHvX64RegisterApicTpr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12296i32);
pub const WHvX64RegisterApicPpr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12298i32);
pub const WHvX64RegisterApicEoi: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12299i32);
pub const WHvX64RegisterApicLdr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12301i32);
pub const WHvX64RegisterApicSpurious: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12303i32);
pub const WHvX64RegisterApicIsr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12304i32);
pub const WHvX64RegisterApicIsr1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12305i32);
pub const WHvX64RegisterApicIsr2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12306i32);
pub const WHvX64RegisterApicIsr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12307i32);
pub const WHvX64RegisterApicIsr4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12308i32);
pub const WHvX64RegisterApicIsr5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12309i32);
pub const WHvX64RegisterApicIsr6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12310i32);
pub const WHvX64RegisterApicIsr7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12311i32);
pub const WHvX64RegisterApicTmr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12312i32);
pub const WHvX64RegisterApicTmr1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12313i32);
pub const WHvX64RegisterApicTmr2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12314i32);
pub const WHvX64RegisterApicTmr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12315i32);
pub const WHvX64RegisterApicTmr4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12316i32);
pub const WHvX64RegisterApicTmr5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12317i32);
pub const WHvX64RegisterApicTmr6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12318i32);
pub const WHvX64RegisterApicTmr7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12319i32);
pub const WHvX64RegisterApicIrr0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12320i32);
pub const WHvX64RegisterApicIrr1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12321i32);
pub const WHvX64RegisterApicIrr2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12322i32);
pub const WHvX64RegisterApicIrr3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12323i32);
pub const WHvX64RegisterApicIrr4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12324i32);
pub const WHvX64RegisterApicIrr5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12325i32);
pub const WHvX64RegisterApicIrr6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12326i32);
pub const WHvX64RegisterApicIrr7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12327i32);
pub const WHvX64RegisterApicEse: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12328i32);
pub const WHvX64RegisterApicIcr: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12336i32);
pub const WHvX64RegisterApicLvtTimer: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12338i32);
pub const WHvX64RegisterApicLvtThermal: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12339i32);
pub const WHvX64RegisterApicLvtPerfmon: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12340i32);
pub const WHvX64RegisterApicLvtLint0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12341i32);
pub const WHvX64RegisterApicLvtLint1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12342i32);
pub const WHvX64RegisterApicLvtError: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12343i32);
pub const WHvX64RegisterApicInitCount: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12344i32);
pub const WHvX64RegisterApicCurrentCount: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12345i32);
pub const WHvX64RegisterApicDivide: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12350i32);
pub const WHvX64RegisterApicSelfIpi: WHV_REGISTER_NAME = WHV_REGISTER_NAME(12351i32);
pub const WHvRegisterSint0: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16384i32);
pub const WHvRegisterSint1: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16385i32);
pub const WHvRegisterSint2: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16386i32);
pub const WHvRegisterSint3: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16387i32);
pub const WHvRegisterSint4: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16388i32);
pub const WHvRegisterSint5: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16389i32);
pub const WHvRegisterSint6: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16390i32);
pub const WHvRegisterSint7: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16391i32);
pub const WHvRegisterSint8: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16392i32);
pub const WHvRegisterSint9: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16393i32);
pub const WHvRegisterSint10: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16394i32);
pub const WHvRegisterSint11: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16395i32);
pub const WHvRegisterSint12: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16396i32);
pub const WHvRegisterSint13: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16397i32);
pub const WHvRegisterSint14: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16398i32);
pub const WHvRegisterSint15: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16399i32);
pub const WHvRegisterScontrol: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16400i32);
pub const WHvRegisterSversion: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16401i32);
pub const WHvRegisterSiefp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16402i32);
pub const WHvRegisterSimp: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16403i32);
pub const WHvRegisterEom: WHV_REGISTER_NAME = WHV_REGISTER_NAME(16404i32);
pub const WHvRegisterVpRuntime: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20480i32);
pub const WHvX64RegisterHypercall: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20481i32);
pub const WHvRegisterGuestOsId: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20482i32);
pub const WHvRegisterVpAssistPage: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20499i32);
pub const WHvRegisterReferenceTsc: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20503i32);
pub const WHvRegisterReferenceTscSequence: WHV_REGISTER_NAME = WHV_REGISTER_NAME(20506i32);
pub const WHvRegisterPendingInterruption: WHV_REGISTER_NAME = WHV_REGISTER_NAME(-2147483648i32);
pub const WHvRegisterInterruptState: WHV_REGISTER_NAME = WHV_REGISTER_NAME(-2147483647i32);
pub const WHvRegisterPendingEvent: WHV_REGISTER_NAME = WHV_REGISTER_NAME(-2147483646i32);
pub const WHvX64RegisterDeliverabilityNotifications: WHV_REGISTER_NAME =
    WHV_REGISTER_NAME(-2147483644i32);
pub const WHvRegisterInternalActivityState: WHV_REGISTER_NAME = WHV_REGISTER_NAME(-2147483643i32);
pub const WHvX64RegisterPendingDebugException: WHV_REGISTER_NAME =
    WHV_REGISTER_NAME(-2147483642i32);
impl ::std::convert::From<i32> for WHV_REGISTER_NAME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_REGISTER_NAME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_REGISTER_VALUE {
    pub Reg128: WHV_UINT128,
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub Fp: WHV_X64_FP_REGISTER,
    pub FpControlStatus: WHV_X64_FP_CONTROL_STATUS_REGISTER,
    pub XmmControlStatus: WHV_X64_XMM_CONTROL_STATUS_REGISTER,
    pub Segment: WHV_X64_SEGMENT_REGISTER,
    pub Table: WHV_X64_TABLE_REGISTER,
    pub InterruptState: WHV_X64_INTERRUPT_STATE_REGISTER,
    pub PendingInterruption: WHV_X64_PENDING_INTERRUPTION_REGISTER,
    pub DeliverabilityNotifications: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER,
    pub ExceptionEvent: WHV_X64_PENDING_EXCEPTION_EVENT,
    pub ExtIntEvent: WHV_X64_PENDING_EXT_INT_EVENT,
    pub InternalActivity: WHV_INTERNAL_ACTIVITY_REGISTER,
    pub PendingDebugException: WHV_X64_PENDING_DEBUG_EXCEPTION,
}
impl WHV_REGISTER_VALUE {}
impl ::std::default::Default for WHV_REGISTER_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_REGISTER_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_REGISTER_VALUE {}
unsafe impl ::windows::runtime::Abi for WHV_REGISTER_VALUE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_RUN_VP_CANCELED_CONTEXT {
    pub CancelReason: WHV_RUN_VP_CANCEL_REASON,
}
impl WHV_RUN_VP_CANCELED_CONTEXT {}
impl ::std::default::Default for WHV_RUN_VP_CANCELED_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_RUN_VP_CANCELED_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_RUN_VP_CANCELED_CONTEXT")
            .field("CancelReason", &self.CancelReason)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_RUN_VP_CANCELED_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.CancelReason == other.CancelReason
    }
}
impl ::std::cmp::Eq for WHV_RUN_VP_CANCELED_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_RUN_VP_CANCELED_CONTEXT {
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
pub struct WHV_RUN_VP_CANCEL_REASON(pub i32);
pub const WHvRunVpCancelReasonUser: WHV_RUN_VP_CANCEL_REASON = WHV_RUN_VP_CANCEL_REASON(0i32);
impl ::std::convert::From<i32> for WHV_RUN_VP_CANCEL_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_RUN_VP_CANCEL_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_RUN_VP_EXIT_CONTEXT {
    pub ExitReason: WHV_RUN_VP_EXIT_REASON,
    pub Reserved: u32,
    pub VpContext: WHV_VP_EXIT_CONTEXT,
    pub Anonymous: WHV_RUN_VP_EXIT_CONTEXT_0,
}
impl WHV_RUN_VP_EXIT_CONTEXT {}
impl ::std::default::Default for WHV_RUN_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_RUN_VP_EXIT_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_RUN_VP_EXIT_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_RUN_VP_EXIT_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_RUN_VP_EXIT_CONTEXT_0 {
    pub MemoryAccess: WHV_MEMORY_ACCESS_CONTEXT,
    pub IoPortAccess: WHV_X64_IO_PORT_ACCESS_CONTEXT,
    pub MsrAccess: WHV_X64_MSR_ACCESS_CONTEXT,
    pub CpuidAccess: WHV_X64_CPUID_ACCESS_CONTEXT,
    pub VpException: WHV_VP_EXCEPTION_CONTEXT,
    pub InterruptWindow: WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT,
    pub UnsupportedFeature: WHV_X64_UNSUPPORTED_FEATURE_CONTEXT,
    pub CancelReason: WHV_RUN_VP_CANCELED_CONTEXT,
    pub ApicEoi: WHV_X64_APIC_EOI_CONTEXT,
    pub ReadTsc: WHV_X64_RDTSC_CONTEXT,
    pub ApicSmi: WHV_X64_APIC_SMI_CONTEXT,
    pub Hypercall: WHV_HYPERCALL_CONTEXT,
    pub ApicInitSipi: WHV_X64_APIC_INIT_SIPI_CONTEXT,
    pub ApicWrite: WHV_X64_APIC_WRITE_CONTEXT,
    pub SynicSintDeliverable: WHV_SYNIC_SINT_DELIVERABLE_CONTEXT,
}
impl WHV_RUN_VP_EXIT_CONTEXT_0 {}
impl ::std::default::Default for WHV_RUN_VP_EXIT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_RUN_VP_EXIT_CONTEXT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_RUN_VP_EXIT_CONTEXT_0 {}
unsafe impl ::windows::runtime::Abi for WHV_RUN_VP_EXIT_CONTEXT_0 {
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
pub struct WHV_RUN_VP_EXIT_REASON(pub i32);
pub const WHvRunVpExitReasonNone: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(0i32);
pub const WHvRunVpExitReasonMemoryAccess: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(1i32);
pub const WHvRunVpExitReasonX64IoPortAccess: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(2i32);
pub const WHvRunVpExitReasonUnrecoverableException: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(4i32);
pub const WHvRunVpExitReasonInvalidVpRegisterValue: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(5i32);
pub const WHvRunVpExitReasonUnsupportedFeature: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(6i32);
pub const WHvRunVpExitReasonX64InterruptWindow: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(7i32);
pub const WHvRunVpExitReasonX64Halt: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(8i32);
pub const WHvRunVpExitReasonX64ApicEoi: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(9i32);
pub const WHvRunVpExitReasonSynicSintDeliverable: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(10i32);
pub const WHvRunVpExitReasonX64MsrAccess: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(4096i32);
pub const WHvRunVpExitReasonX64Cpuid: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(4097i32);
pub const WHvRunVpExitReasonException: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(4098i32);
pub const WHvRunVpExitReasonX64Rdtsc: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(4099i32);
pub const WHvRunVpExitReasonX64ApicSmiTrap: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(4100i32);
pub const WHvRunVpExitReasonHypercall: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(4101i32);
pub const WHvRunVpExitReasonX64ApicInitSipiTrap: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(4102i32);
pub const WHvRunVpExitReasonX64ApicWriteTrap: WHV_RUN_VP_EXIT_REASON =
    WHV_RUN_VP_EXIT_REASON(4103i32);
pub const WHvRunVpExitReasonCanceled: WHV_RUN_VP_EXIT_REASON = WHV_RUN_VP_EXIT_REASON(8193i32);
impl ::std::convert::From<i32> for WHV_RUN_VP_EXIT_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_RUN_VP_EXIT_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_SCHEDULER_FEATURES {
    pub Anonymous: WHV_SCHEDULER_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_SCHEDULER_FEATURES {}
impl ::std::default::Default for WHV_SCHEDULER_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_SCHEDULER_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_SCHEDULER_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_SCHEDULER_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SCHEDULER_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_SCHEDULER_FEATURES_0 {}
impl ::std::default::Default for WHV_SCHEDULER_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_SCHEDULER_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_SCHEDULER_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_SCHEDULER_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_SCHEDULER_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct WHV_SRIOV_RESOURCE_DESCRIPTOR {
    pub PnpInstanceId: [u16; 200],
    pub VirtualFunctionId: super::SystemServices::LUID,
    pub VirtualFunctionIndex: u16,
    pub Reserved: u16,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl WHV_SRIOV_RESOURCE_DESCRIPTOR {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_SRIOV_RESOURCE_DESCRIPTOR")
            .field("PnpInstanceId", &self.PnpInstanceId)
            .field("VirtualFunctionId", &self.VirtualFunctionId)
            .field("VirtualFunctionIndex", &self.VirtualFunctionIndex)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PnpInstanceId == other.PnpInstanceId
            && self.VirtualFunctionId == other.VirtualFunctionId
            && self.VirtualFunctionIndex == other.VirtualFunctionIndex
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for WHV_SRIOV_RESOURCE_DESCRIPTOR {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SYNIC_EVENT_PARAMETERS {
    pub VpIndex: u32,
    pub TargetSint: u8,
    pub Reserved: u8,
    pub FlagNumber: u16,
}
impl WHV_SYNIC_EVENT_PARAMETERS {}
impl ::std::default::Default for WHV_SYNIC_EVENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_SYNIC_EVENT_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_SYNIC_EVENT_PARAMETERS")
            .field("VpIndex", &self.VpIndex)
            .field("TargetSint", &self.TargetSint)
            .field("Reserved", &self.Reserved)
            .field("FlagNumber", &self.FlagNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_SYNIC_EVENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.VpIndex == other.VpIndex
            && self.TargetSint == other.TargetSint
            && self.Reserved == other.Reserved
            && self.FlagNumber == other.FlagNumber
    }
}
impl ::std::cmp::Eq for WHV_SYNIC_EVENT_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for WHV_SYNIC_EVENT_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_SYNIC_MESSAGE_SIZE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    pub DeliverableSints: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {}
impl ::std::default::Default for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_SYNIC_SINT_DELIVERABLE_CONTEXT")
            .field("DeliverableSints", &self.DeliverableSints)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.DeliverableSints == other.DeliverableSints
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
impl WHV_SYNTHETIC_PROCESSOR_FEATURES {}
impl ::std::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_SYNTHETIC_PROCESSOR_FEATURES {}
unsafe impl ::windows::runtime::Abi for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
impl WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {}
impl ::std::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {}
unsafe impl ::windows::runtime::Abi for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0,
}
impl WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {}
impl ::std::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {}
unsafe impl ::windows::runtime::Abi for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 1],
}
impl WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {}
impl ::std::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_SYNTHETIC_PROCESSOR_FEATURES,
}
impl WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {}
impl ::std::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WHV_TRANSLATE_GVA_FLAGS(pub u32);
pub const WHvTranslateGvaFlagNone: WHV_TRANSLATE_GVA_FLAGS = WHV_TRANSLATE_GVA_FLAGS(0u32);
pub const WHvTranslateGvaFlagValidateRead: WHV_TRANSLATE_GVA_FLAGS = WHV_TRANSLATE_GVA_FLAGS(1u32);
pub const WHvTranslateGvaFlagValidateWrite: WHV_TRANSLATE_GVA_FLAGS = WHV_TRANSLATE_GVA_FLAGS(2u32);
pub const WHvTranslateGvaFlagValidateExecute: WHV_TRANSLATE_GVA_FLAGS =
    WHV_TRANSLATE_GVA_FLAGS(4u32);
pub const WHvTranslateGvaFlagPrivilegeExempt: WHV_TRANSLATE_GVA_FLAGS =
    WHV_TRANSLATE_GVA_FLAGS(8u32);
pub const WHvTranslateGvaFlagSetPageTableBits: WHV_TRANSLATE_GVA_FLAGS =
    WHV_TRANSLATE_GVA_FLAGS(16u32);
pub const WHvTranslateGvaFlagEnforceSmap: WHV_TRANSLATE_GVA_FLAGS = WHV_TRANSLATE_GVA_FLAGS(256u32);
pub const WHvTranslateGvaFlagOverrideSmap: WHV_TRANSLATE_GVA_FLAGS =
    WHV_TRANSLATE_GVA_FLAGS(512u32);
impl ::std::convert::From<u32> for WHV_TRANSLATE_GVA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_TRANSLATE_GVA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_TRANSLATE_GVA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_TRANSLATE_GVA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_TRANSLATE_GVA_RESULT {
    pub ResultCode: WHV_TRANSLATE_GVA_RESULT_CODE,
    pub Reserved: u32,
}
impl WHV_TRANSLATE_GVA_RESULT {}
impl ::std::default::Default for WHV_TRANSLATE_GVA_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_TRANSLATE_GVA_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_TRANSLATE_GVA_RESULT")
            .field("ResultCode", &self.ResultCode)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_TRANSLATE_GVA_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.ResultCode == other.ResultCode && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WHV_TRANSLATE_GVA_RESULT {}
unsafe impl ::windows::runtime::Abi for WHV_TRANSLATE_GVA_RESULT {
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
pub struct WHV_TRANSLATE_GVA_RESULT_CODE(pub i32);
pub const WHvTranslateGvaResultSuccess: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(0i32);
pub const WHvTranslateGvaResultPageNotPresent: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(1i32);
pub const WHvTranslateGvaResultPrivilegeViolation: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(2i32);
pub const WHvTranslateGvaResultInvalidPageTableFlags: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(3i32);
pub const WHvTranslateGvaResultGpaUnmapped: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(4i32);
pub const WHvTranslateGvaResultGpaNoReadAccess: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(5i32);
pub const WHvTranslateGvaResultGpaNoWriteAccess: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(6i32);
pub const WHvTranslateGvaResultGpaIllegalOverlayAccess: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(7i32);
pub const WHvTranslateGvaResultIntercept: WHV_TRANSLATE_GVA_RESULT_CODE =
    WHV_TRANSLATE_GVA_RESULT_CODE(8i32);
impl ::std::convert::From<i32> for WHV_TRANSLATE_GVA_RESULT_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_TRANSLATE_GVA_RESULT_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_TRIGGER_PARAMETERS {
    pub TriggerType: WHV_TRIGGER_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_TRIGGER_PARAMETERS_0,
}
impl WHV_TRIGGER_PARAMETERS {}
impl ::std::default::Default for WHV_TRIGGER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_TRIGGER_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_TRIGGER_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for WHV_TRIGGER_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_TRIGGER_PARAMETERS_0 {
    pub Interrupt: WHV_INTERRUPT_CONTROL,
    pub SynicEvent: WHV_SYNIC_EVENT_PARAMETERS,
    pub DeviceInterrupt: WHV_TRIGGER_PARAMETERS_0_0,
}
impl WHV_TRIGGER_PARAMETERS_0 {}
impl ::std::default::Default for WHV_TRIGGER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_TRIGGER_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_TRIGGER_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for WHV_TRIGGER_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_TRIGGER_PARAMETERS_0_0 {
    pub LogicalDeviceId: u64,
    pub MsiAddress: u64,
    pub MsiData: u32,
    pub Reserved: u32,
}
impl WHV_TRIGGER_PARAMETERS_0_0 {}
impl ::std::default::Default for WHV_TRIGGER_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_TRIGGER_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_DeviceInterrupt_e__Struct")
            .field("LogicalDeviceId", &self.LogicalDeviceId)
            .field("MsiAddress", &self.MsiAddress)
            .field("MsiData", &self.MsiData)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_TRIGGER_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalDeviceId == other.LogicalDeviceId
            && self.MsiAddress == other.MsiAddress
            && self.MsiData == other.MsiData
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WHV_TRIGGER_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_TRIGGER_PARAMETERS_0_0 {
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
pub struct WHV_TRIGGER_TYPE(pub i32);
pub const WHvTriggerTypeInterrupt: WHV_TRIGGER_TYPE = WHV_TRIGGER_TYPE(0i32);
pub const WHvTriggerTypeSynicEvent: WHV_TRIGGER_TYPE = WHV_TRIGGER_TYPE(1i32);
pub const WHvTriggerTypeDeviceInterrupt: WHV_TRIGGER_TYPE = WHV_TRIGGER_TYPE(2i32);
impl ::std::convert::From<i32> for WHV_TRIGGER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_TRIGGER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_UINT128 {
    pub Anonymous: WHV_UINT128_0,
    pub Dword: [u32; 4],
}
impl WHV_UINT128 {}
impl ::std::default::Default for WHV_UINT128 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_UINT128 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_UINT128 {}
unsafe impl ::windows::runtime::Abi for WHV_UINT128 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_UINT128_0 {
    pub Low64: u64,
    pub High64: u64,
}
impl WHV_UINT128_0 {}
impl ::std::default::Default for WHV_UINT128_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_UINT128_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("Low64", &self.Low64)
            .field("High64", &self.High64)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_UINT128_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Low64 == other.Low64 && self.High64 == other.High64
    }
}
impl ::std::cmp::Eq for WHV_UINT128_0 {}
unsafe impl ::windows::runtime::Abi for WHV_UINT128_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY {
    pub PropertyCode: WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE,
    pub Reserved: u32,
    pub Anonymous: WHV_VIRTUAL_PROCESSOR_PROPERTY_0,
}
impl WHV_VIRTUAL_PROCESSOR_PROPERTY {}
impl ::std::default::Default for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VIRTUAL_PROCESSOR_PROPERTY {}
unsafe impl ::windows::runtime::Abi for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    pub NumaNode: u16,
    pub Padding: u64,
}
impl WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {}
impl ::std::default::Default for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {}
unsafe impl ::windows::runtime::Abi for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
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
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE(pub i32);
pub const WHvVirtualProcessorPropertyCodeNumaNode: WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE =
    WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE(0i32);
impl ::std::convert::From<i32> for WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE {
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
pub struct WHV_VIRTUAL_PROCESSOR_STATE_TYPE(pub i32);
pub const WHvVirtualProcessorStateTypeSynicMessagePage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE =
    WHV_VIRTUAL_PROCESSOR_STATE_TYPE(0i32);
pub const WHvVirtualProcessorStateTypeSynicEventFlagPage: WHV_VIRTUAL_PROCESSOR_STATE_TYPE =
    WHV_VIRTUAL_PROCESSOR_STATE_TYPE(1i32);
pub const WHvVirtualProcessorStateTypeSynicTimerState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE =
    WHV_VIRTUAL_PROCESSOR_STATE_TYPE(2i32);
pub const WHvVirtualProcessorStateTypeInterruptControllerState2: WHV_VIRTUAL_PROCESSOR_STATE_TYPE =
    WHV_VIRTUAL_PROCESSOR_STATE_TYPE(4096i32);
pub const WHvVirtualProcessorStateTypeXsaveState: WHV_VIRTUAL_PROCESSOR_STATE_TYPE =
    WHV_VIRTUAL_PROCESSOR_STATE_TYPE(4097i32);
impl ::std::convert::From<i32> for WHV_VIRTUAL_PROCESSOR_STATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VIRTUAL_PROCESSOR_STATE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_DEVICE_NOTIFICATION {
    pub NotificationType: WHV_VPCI_DEVICE_NOTIFICATION_TYPE,
    pub Reserved1: u32,
    pub Anonymous: WHV_VPCI_DEVICE_NOTIFICATION_0,
}
impl WHV_VPCI_DEVICE_NOTIFICATION {}
impl ::std::default::Default for WHV_VPCI_DEVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_DEVICE_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VPCI_DEVICE_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_VPCI_DEVICE_NOTIFICATION_0 {
    pub Reserved2: u64,
}
impl WHV_VPCI_DEVICE_NOTIFICATION_0 {}
impl ::std::default::Default for WHV_VPCI_DEVICE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_DEVICE_NOTIFICATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VPCI_DEVICE_NOTIFICATION_0 {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_NOTIFICATION_0 {
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
pub struct WHV_VPCI_DEVICE_NOTIFICATION_TYPE(pub i32);
pub const WHvVpciDeviceNotificationUndefined: WHV_VPCI_DEVICE_NOTIFICATION_TYPE =
    WHV_VPCI_DEVICE_NOTIFICATION_TYPE(0i32);
pub const WHvVpciDeviceNotificationMmioRemapping: WHV_VPCI_DEVICE_NOTIFICATION_TYPE =
    WHV_VPCI_DEVICE_NOTIFICATION_TYPE(1i32);
pub const WHvVpciDeviceNotificationSurpriseRemoval: WHV_VPCI_DEVICE_NOTIFICATION_TYPE =
    WHV_VPCI_DEVICE_NOTIFICATION_TYPE(2i32);
impl ::std::convert::From<i32> for WHV_VPCI_DEVICE_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_NOTIFICATION_TYPE {
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
pub struct WHV_VPCI_DEVICE_PROPERTY_CODE(pub i32);
pub const WHvVpciDevicePropertyCodeUndefined: WHV_VPCI_DEVICE_PROPERTY_CODE =
    WHV_VPCI_DEVICE_PROPERTY_CODE(0i32);
pub const WHvVpciDevicePropertyCodeHardwareIDs: WHV_VPCI_DEVICE_PROPERTY_CODE =
    WHV_VPCI_DEVICE_PROPERTY_CODE(1i32);
pub const WHvVpciDevicePropertyCodeProbedBARs: WHV_VPCI_DEVICE_PROPERTY_CODE =
    WHV_VPCI_DEVICE_PROPERTY_CODE(2i32);
impl ::std::convert::From<i32> for WHV_VPCI_DEVICE_PROPERTY_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_PROPERTY_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_DEVICE_REGISTER {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub SizeInBytes: u32,
    pub OffsetInBytes: u64,
}
impl WHV_VPCI_DEVICE_REGISTER {}
impl ::std::default::Default for WHV_VPCI_DEVICE_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VPCI_DEVICE_REGISTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_VPCI_DEVICE_REGISTER")
            .field("Location", &self.Location)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("OffsetInBytes", &self.OffsetInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_DEVICE_REGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
            && self.SizeInBytes == other.SizeInBytes
            && self.OffsetInBytes == other.OffsetInBytes
    }
}
impl ::std::cmp::Eq for WHV_VPCI_DEVICE_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_REGISTER {
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
pub struct WHV_VPCI_DEVICE_REGISTER_SPACE(pub i32);
pub const WHvVpciConfigSpace: WHV_VPCI_DEVICE_REGISTER_SPACE =
    WHV_VPCI_DEVICE_REGISTER_SPACE(-1i32);
pub const WHvVpciBar0: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(0i32);
pub const WHvVpciBar1: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(1i32);
pub const WHvVpciBar2: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(2i32);
pub const WHvVpciBar3: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(3i32);
pub const WHvVpciBar4: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(4i32);
pub const WHvVpciBar5: WHV_VPCI_DEVICE_REGISTER_SPACE = WHV_VPCI_DEVICE_REGISTER_SPACE(5i32);
impl ::std::convert::From<i32> for WHV_VPCI_DEVICE_REGISTER_SPACE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_DEVICE_REGISTER_SPACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_HARDWARE_IDS {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
impl WHV_VPCI_HARDWARE_IDS {}
impl ::std::default::Default for WHV_VPCI_HARDWARE_IDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VPCI_HARDWARE_IDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_VPCI_HARDWARE_IDS")
            .field("VendorID", &self.VendorID)
            .field("DeviceID", &self.DeviceID)
            .field("RevisionID", &self.RevisionID)
            .field("ProgIf", &self.ProgIf)
            .field("SubClass", &self.SubClass)
            .field("BaseClass", &self.BaseClass)
            .field("SubVendorID", &self.SubVendorID)
            .field("SubSystemID", &self.SubSystemID)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_HARDWARE_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.VendorID == other.VendorID
            && self.DeviceID == other.DeviceID
            && self.RevisionID == other.RevisionID
            && self.ProgIf == other.ProgIf
            && self.SubClass == other.SubClass
            && self.BaseClass == other.BaseClass
            && self.SubVendorID == other.SubVendorID
            && self.SubSystemID == other.SubSystemID
    }
}
impl ::std::cmp::Eq for WHV_VPCI_HARDWARE_IDS {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_HARDWARE_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_INTERRUPT_TARGET {
    pub Vector: u32,
    pub Flags: WHV_VPCI_INTERRUPT_TARGET_FLAGS,
    pub ProcessorCount: u32,
    pub Processors: [u32; 1],
}
impl WHV_VPCI_INTERRUPT_TARGET {}
impl ::std::default::Default for WHV_VPCI_INTERRUPT_TARGET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VPCI_INTERRUPT_TARGET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_VPCI_INTERRUPT_TARGET")
            .field("Vector", &self.Vector)
            .field("Flags", &self.Flags)
            .field("ProcessorCount", &self.ProcessorCount)
            .field("Processors", &self.Processors)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_INTERRUPT_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.Vector == other.Vector
            && self.Flags == other.Flags
            && self.ProcessorCount == other.ProcessorCount
            && self.Processors == other.Processors
    }
}
impl ::std::cmp::Eq for WHV_VPCI_INTERRUPT_TARGET {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_INTERRUPT_TARGET {
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
pub struct WHV_VPCI_INTERRUPT_TARGET_FLAGS(pub u32);
pub const WHvVpciInterruptTargetFlagNone: WHV_VPCI_INTERRUPT_TARGET_FLAGS =
    WHV_VPCI_INTERRUPT_TARGET_FLAGS(0u32);
pub const WHvVpciInterruptTargetFlagMulticast: WHV_VPCI_INTERRUPT_TARGET_FLAGS =
    WHV_VPCI_INTERRUPT_TARGET_FLAGS(1u32);
impl ::std::convert::From<u32> for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_MMIO_MAPPING {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub Flags: WHV_VPCI_MMIO_RANGE_FLAGS,
    pub SizeInBytes: u64,
    pub OffsetInBytes: u64,
    pub VirtualAddress: *mut ::std::ffi::c_void,
}
impl WHV_VPCI_MMIO_MAPPING {}
impl ::std::default::Default for WHV_VPCI_MMIO_MAPPING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VPCI_MMIO_MAPPING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_VPCI_MMIO_MAPPING")
            .field("Location", &self.Location)
            .field("Flags", &self.Flags)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("OffsetInBytes", &self.OffsetInBytes)
            .field("VirtualAddress", &self.VirtualAddress)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_MMIO_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
            && self.Flags == other.Flags
            && self.SizeInBytes == other.SizeInBytes
            && self.OffsetInBytes == other.OffsetInBytes
            && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::std::cmp::Eq for WHV_VPCI_MMIO_MAPPING {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_MMIO_MAPPING {
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
pub struct WHV_VPCI_MMIO_RANGE_FLAGS(pub u32);
pub const WHvVpciMmioRangeFlagReadAccess: WHV_VPCI_MMIO_RANGE_FLAGS =
    WHV_VPCI_MMIO_RANGE_FLAGS(1u32);
pub const WHvVpciMmioRangeFlagWriteAccess: WHV_VPCI_MMIO_RANGE_FLAGS =
    WHV_VPCI_MMIO_RANGE_FLAGS(2u32);
impl ::std::convert::From<u32> for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VPCI_PROBED_BARS {
    pub Value: [u32; 6],
}
impl WHV_VPCI_PROBED_BARS {}
impl ::std::default::Default for WHV_VPCI_PROBED_BARS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VPCI_PROBED_BARS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_VPCI_PROBED_BARS")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VPCI_PROBED_BARS {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for WHV_VPCI_PROBED_BARS {}
unsafe impl ::windows::runtime::Abi for WHV_VPCI_PROBED_BARS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHV_VPCI_TYPE0_BAR_COUNT: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VP_EXCEPTION_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub ExceptionInfo: WHV_VP_EXCEPTION_INFO,
    pub ExceptionType: u8,
    pub Reserved2: [u8; 3],
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
impl WHV_VP_EXCEPTION_CONTEXT {}
impl ::std::default::Default for WHV_VP_EXCEPTION_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VP_EXCEPTION_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VP_EXCEPTION_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_VP_EXCEPTION_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_VP_EXCEPTION_INFO {
    pub Anonymous: WHV_VP_EXCEPTION_INFO_0,
    pub AsUINT32: u32,
}
impl WHV_VP_EXCEPTION_INFO {}
impl ::std::default::Default for WHV_VP_EXCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VP_EXCEPTION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VP_EXCEPTION_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_VP_EXCEPTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VP_EXCEPTION_INFO_0 {
    pub _bitfield: u32,
}
impl WHV_VP_EXCEPTION_INFO_0 {}
impl ::std::default::Default for WHV_VP_EXCEPTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_VP_EXCEPTION_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_VP_EXCEPTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_VP_EXCEPTION_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WHV_VP_EXCEPTION_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_VP_EXIT_CONTEXT {
    pub ExecutionState: WHV_X64_VP_EXECUTION_STATE,
    pub _bitfield: u8,
    pub Reserved: u8,
    pub Reserved2: u32,
    pub Cs: WHV_X64_SEGMENT_REGISTER,
    pub Rip: u64,
    pub Rflags: u64,
}
impl WHV_VP_EXIT_CONTEXT {}
impl ::std::default::Default for WHV_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_VP_EXIT_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_VP_EXIT_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_VP_EXIT_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_APIC_EOI_CONTEXT {
    pub InterruptVector: u32,
}
impl WHV_X64_APIC_EOI_CONTEXT {}
impl ::std::default::Default for WHV_X64_APIC_EOI_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_APIC_EOI_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_APIC_EOI_CONTEXT")
            .field("InterruptVector", &self.InterruptVector)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_APIC_EOI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.InterruptVector == other.InterruptVector
    }
}
impl ::std::cmp::Eq for WHV_X64_APIC_EOI_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_APIC_EOI_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_APIC_INIT_SIPI_CONTEXT {
    pub ApicIcr: u64,
}
impl WHV_X64_APIC_INIT_SIPI_CONTEXT {}
impl ::std::default::Default for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_APIC_INIT_SIPI_CONTEXT")
            .field("ApicIcr", &self.ApicIcr)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ApicIcr == other.ApicIcr
    }
}
impl ::std::cmp::Eq for WHV_X64_APIC_INIT_SIPI_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_APIC_SMI_CONTEXT {
    pub ApicIcr: u64,
}
impl WHV_X64_APIC_SMI_CONTEXT {}
impl ::std::default::Default for WHV_X64_APIC_SMI_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_APIC_SMI_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_APIC_SMI_CONTEXT")
            .field("ApicIcr", &self.ApicIcr)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_APIC_SMI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ApicIcr == other.ApicIcr
    }
}
impl ::std::cmp::Eq for WHV_X64_APIC_SMI_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_APIC_SMI_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_APIC_WRITE_CONTEXT {
    pub Type: WHV_X64_APIC_WRITE_TYPE,
    pub Reserved: u32,
    pub WriteValue: u64,
}
impl WHV_X64_APIC_WRITE_CONTEXT {}
impl ::std::default::Default for WHV_X64_APIC_WRITE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_APIC_WRITE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_APIC_WRITE_CONTEXT")
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("WriteValue", &self.WriteValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_APIC_WRITE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.WriteValue == other.WriteValue
    }
}
impl ::std::cmp::Eq for WHV_X64_APIC_WRITE_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_APIC_WRITE_CONTEXT {
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
pub struct WHV_X64_APIC_WRITE_TYPE(pub i32);
pub const WHvX64ApicWriteTypeLdr: WHV_X64_APIC_WRITE_TYPE = WHV_X64_APIC_WRITE_TYPE(208i32);
pub const WHvX64ApicWriteTypeDfr: WHV_X64_APIC_WRITE_TYPE = WHV_X64_APIC_WRITE_TYPE(224i32);
pub const WHvX64ApicWriteTypeSvr: WHV_X64_APIC_WRITE_TYPE = WHV_X64_APIC_WRITE_TYPE(240i32);
pub const WHvX64ApicWriteTypeLint0: WHV_X64_APIC_WRITE_TYPE = WHV_X64_APIC_WRITE_TYPE(848i32);
pub const WHvX64ApicWriteTypeLint1: WHV_X64_APIC_WRITE_TYPE = WHV_X64_APIC_WRITE_TYPE(864i32);
impl ::std::convert::From<i32> for WHV_X64_APIC_WRITE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_APIC_WRITE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_CPUID_ACCESS_CONTEXT {
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub DefaultResultRax: u64,
    pub DefaultResultRcx: u64,
    pub DefaultResultRdx: u64,
    pub DefaultResultRbx: u64,
}
impl WHV_X64_CPUID_ACCESS_CONTEXT {}
impl ::std::default::Default for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_CPUID_ACCESS_CONTEXT")
            .field("Rax", &self.Rax)
            .field("Rcx", &self.Rcx)
            .field("Rdx", &self.Rdx)
            .field("Rbx", &self.Rbx)
            .field("DefaultResultRax", &self.DefaultResultRax)
            .field("DefaultResultRcx", &self.DefaultResultRcx)
            .field("DefaultResultRdx", &self.DefaultResultRdx)
            .field("DefaultResultRbx", &self.DefaultResultRbx)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Rax == other.Rax
            && self.Rcx == other.Rcx
            && self.Rdx == other.Rdx
            && self.Rbx == other.Rbx
            && self.DefaultResultRax == other.DefaultResultRax
            && self.DefaultResultRcx == other.DefaultResultRcx
            && self.DefaultResultRdx == other.DefaultResultRdx
            && self.DefaultResultRbx == other.DefaultResultRbx
    }
}
impl ::std::cmp::Eq for WHV_X64_CPUID_ACCESS_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_CPUID_ACCESS_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_CPUID_RESULT {
    pub Function: u32,
    pub Reserved: [u32; 3],
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
impl WHV_X64_CPUID_RESULT {}
impl ::std::default::Default for WHV_X64_CPUID_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_CPUID_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_CPUID_RESULT")
            .field("Function", &self.Function)
            .field("Reserved", &self.Reserved)
            .field("Eax", &self.Eax)
            .field("Ebx", &self.Ebx)
            .field("Ecx", &self.Ecx)
            .field("Edx", &self.Edx)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_CPUID_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function
            && self.Reserved == other.Reserved
            && self.Eax == other.Eax
            && self.Ebx == other.Ebx
            && self.Ecx == other.Ecx
            && self.Edx == other.Edx
    }
}
impl ::std::cmp::Eq for WHV_X64_CPUID_RESULT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_CPUID_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_CPUID_RESULT2 {
    pub Function: u32,
    pub Index: u32,
    pub VpIndex: u32,
    pub Flags: WHV_X64_CPUID_RESULT2_FLAGS,
    pub Output: WHV_CPUID_OUTPUT,
    pub Mask: WHV_CPUID_OUTPUT,
}
impl WHV_X64_CPUID_RESULT2 {}
impl ::std::default::Default for WHV_X64_CPUID_RESULT2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_CPUID_RESULT2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_CPUID_RESULT2")
            .field("Function", &self.Function)
            .field("Index", &self.Index)
            .field("VpIndex", &self.VpIndex)
            .field("Flags", &self.Flags)
            .field("Output", &self.Output)
            .field("Mask", &self.Mask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_CPUID_RESULT2 {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function
            && self.Index == other.Index
            && self.VpIndex == other.VpIndex
            && self.Flags == other.Flags
            && self.Output == other.Output
            && self.Mask == other.Mask
    }
}
impl ::std::cmp::Eq for WHV_X64_CPUID_RESULT2 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_CPUID_RESULT2 {
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
pub struct WHV_X64_CPUID_RESULT2_FLAGS(pub u32);
pub const WHvX64CpuidResult2FlagSubleafSpecific: WHV_X64_CPUID_RESULT2_FLAGS =
    WHV_X64_CPUID_RESULT2_FLAGS(1u32);
pub const WHvX64CpuidResult2FlagVpSpecific: WHV_X64_CPUID_RESULT2_FLAGS =
    WHV_X64_CPUID_RESULT2_FLAGS(2u32);
impl ::std::convert::From<u32> for WHV_X64_CPUID_RESULT2_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_CPUID_RESULT2_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WHV_X64_CPUID_RESULT2_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WHV_X64_CPUID_RESULT2_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    pub Anonymous: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0,
    pub AsUINT64: u64,
}
impl WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {}
impl ::std::default::Default for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    pub _bitfield: u64,
}
impl WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl WHV_X64_FP_CONTROL_STATUS_REGISTER {}
impl ::std::default::Default for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_CONTROL_STATUS_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    pub FpControl: u16,
    pub FpStatus: u16,
    pub FpTag: u8,
    pub Reserved: u8,
    pub LastFpOp: u16,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0,
}
impl WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRip: u64,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0,
}
impl WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {}
impl ::std::default::Default for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpEip: u32,
    pub LastFpCs: u16,
    pub Reserved2: u16,
}
impl WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {}
impl ::std::default::Default for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("LastFpEip", &self.LastFpEip)
            .field("LastFpCs", &self.LastFpCs)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LastFpEip == other.LastFpEip
            && self.LastFpCs == other.LastFpCs
            && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_FP_REGISTER {
    pub Anonymous: WHV_X64_FP_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl WHV_X64_FP_REGISTER {}
impl ::std::default::Default for WHV_X64_FP_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_FP_REGISTER_0 {
    pub Mantissa: u64,
    pub _bitfield: u64,
}
impl WHV_X64_FP_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_FP_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_FP_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("Mantissa", &self.Mantissa)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_FP_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Mantissa == other.Mantissa && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_FP_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_FP_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    pub DeliverableType: WHV_X64_PENDING_INTERRUPTION_TYPE,
}
impl WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {}
impl ::std::default::Default for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT")
            .field("DeliverableType", &self.DeliverableType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.DeliverableType == other.DeliverableType
    }
}
impl ::std::cmp::Eq for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_INTERRUPT_STATE_REGISTER {
    pub Anonymous: WHV_X64_INTERRUPT_STATE_REGISTER_0,
    pub AsUINT64: u64,
}
impl WHV_X64_INTERRUPT_STATE_REGISTER {}
impl ::std::default::Default for WHV_X64_INTERRUPT_STATE_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_INTERRUPT_STATE_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_INTERRUPT_STATE_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_INTERRUPT_STATE_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    pub _bitfield: u64,
}
impl WHV_X64_INTERRUPT_STATE_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_INTERRUPT_STATE_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_IO_PORT_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_X64_IO_PORT_ACCESS_INFO,
    pub PortNumber: u16,
    pub Reserved2: [u16; 3],
    pub Rax: u64,
    pub Rcx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Ds: WHV_X64_SEGMENT_REGISTER,
    pub Es: WHV_X64_SEGMENT_REGISTER,
}
impl WHV_X64_IO_PORT_ACCESS_CONTEXT {}
impl ::std::default::Default for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_IO_PORT_ACCESS_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_IO_PORT_ACCESS_INFO {
    pub Anonymous: WHV_X64_IO_PORT_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl WHV_X64_IO_PORT_ACCESS_INFO {}
impl ::std::default::Default for WHV_X64_IO_PORT_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_IO_PORT_ACCESS_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_IO_PORT_ACCESS_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_X64_IO_PORT_ACCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_IO_PORT_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl WHV_X64_IO_PORT_ACCESS_INFO_0 {}
impl ::std::default::Default for WHV_X64_IO_PORT_ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_IO_PORT_ACCESS_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_IO_PORT_ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_IO_PORT_ACCESS_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_IO_PORT_ACCESS_INFO_0 {
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
pub struct WHV_X64_LOCAL_APIC_EMULATION_MODE(pub i32);
pub const WHvX64LocalApicEmulationModeNone: WHV_X64_LOCAL_APIC_EMULATION_MODE =
    WHV_X64_LOCAL_APIC_EMULATION_MODE(0i32);
pub const WHvX64LocalApicEmulationModeXApic: WHV_X64_LOCAL_APIC_EMULATION_MODE =
    WHV_X64_LOCAL_APIC_EMULATION_MODE(1i32);
pub const WHvX64LocalApicEmulationModeX2Apic: WHV_X64_LOCAL_APIC_EMULATION_MODE =
    WHV_X64_LOCAL_APIC_EMULATION_MODE(2i32);
impl ::std::convert::From<i32> for WHV_X64_LOCAL_APIC_EMULATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_LOCAL_APIC_EMULATION_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_MSR_ACCESS_CONTEXT {
    pub AccessInfo: WHV_X64_MSR_ACCESS_INFO,
    pub MsrNumber: u32,
    pub Rax: u64,
    pub Rdx: u64,
}
impl WHV_X64_MSR_ACCESS_CONTEXT {}
impl ::std::default::Default for WHV_X64_MSR_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_MSR_ACCESS_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_MSR_ACCESS_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_MSR_ACCESS_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_MSR_ACCESS_INFO {
    pub Anonymous: WHV_X64_MSR_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl WHV_X64_MSR_ACCESS_INFO {}
impl ::std::default::Default for WHV_X64_MSR_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_MSR_ACCESS_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_MSR_ACCESS_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_X64_MSR_ACCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_MSR_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl WHV_X64_MSR_ACCESS_INFO_0 {}
impl ::std::default::Default for WHV_X64_MSR_ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_MSR_ACCESS_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_MSR_ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_MSR_ACCESS_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_MSR_ACCESS_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_MSR_EXIT_BITMAP {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_MSR_EXIT_BITMAP_0,
}
impl WHV_X64_MSR_EXIT_BITMAP {}
impl ::std::default::Default for WHV_X64_MSR_EXIT_BITMAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_MSR_EXIT_BITMAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_MSR_EXIT_BITMAP {}
unsafe impl ::windows::runtime::Abi for WHV_X64_MSR_EXIT_BITMAP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_MSR_EXIT_BITMAP_0 {
    pub _bitfield: u64,
}
impl WHV_X64_MSR_EXIT_BITMAP_0 {}
impl ::std::default::Default for WHV_X64_MSR_EXIT_BITMAP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_MSR_EXIT_BITMAP_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_MSR_EXIT_BITMAP_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_MSR_EXIT_BITMAP_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_MSR_EXIT_BITMAP_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_PENDING_DEBUG_EXCEPTION {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_PENDING_DEBUG_EXCEPTION_0,
}
impl WHV_X64_PENDING_DEBUG_EXCEPTION {}
impl ::std::default::Default for WHV_X64_PENDING_DEBUG_EXCEPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_DEBUG_EXCEPTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_DEBUG_EXCEPTION {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_DEBUG_EXCEPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    pub _bitfield: u64,
}
impl WHV_X64_PENDING_DEBUG_EXCEPTION_0 {}
impl ::std::default::Default for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
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
pub struct WHV_X64_PENDING_EVENT_TYPE(pub i32);
pub const WHvX64PendingEventException: WHV_X64_PENDING_EVENT_TYPE =
    WHV_X64_PENDING_EVENT_TYPE(0i32);
pub const WHvX64PendingEventExtInt: WHV_X64_PENDING_EVENT_TYPE = WHV_X64_PENDING_EVENT_TYPE(5i32);
impl ::std::convert::From<i32> for WHV_X64_PENDING_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_PENDING_EXCEPTION_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXCEPTION_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
impl WHV_X64_PENDING_EXCEPTION_EVENT {}
impl ::std::default::Default for WHV_X64_PENDING_EXCEPTION_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_EXCEPTION_EVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_EXCEPTION_EVENT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_EXCEPTION_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
impl WHV_X64_PENDING_EXCEPTION_EVENT_0 {}
impl ::std::default::Default for WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .field("ErrorCode", &self.ErrorCode)
            .field("ExceptionParameter", &self.ExceptionParameter)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.ErrorCode == other.ErrorCode
            && self.ExceptionParameter == other.ExceptionParameter
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_EXCEPTION_EVENT_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_PENDING_EXT_INT_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXT_INT_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
impl WHV_X64_PENDING_EXT_INT_EVENT {}
impl ::std::default::Default for WHV_X64_PENDING_EXT_INT_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_EXT_INT_EVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_EXT_INT_EVENT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_EXT_INT_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_PENDING_EXT_INT_EVENT_0 {
    pub _bitfield: u64,
    pub Reserved2: u64,
}
impl WHV_X64_PENDING_EXT_INT_EVENT_0 {}
impl ::std::default::Default for WHV_X64_PENDING_EXT_INT_EVENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_PENDING_EXT_INT_EVENT_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_EXT_INT_EVENT_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_EXT_INT_EVENT_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_EXT_INT_EVENT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_PENDING_INTERRUPTION_REGISTER {
    pub Anonymous: WHV_X64_PENDING_INTERRUPTION_REGISTER_0,
    pub AsUINT64: u64,
}
impl WHV_X64_PENDING_INTERRUPTION_REGISTER {}
impl ::std::default::Default for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_INTERRUPTION_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
}
impl WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.ErrorCode == other.ErrorCode
    }
}
impl ::std::cmp::Eq for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
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
pub struct WHV_X64_PENDING_INTERRUPTION_TYPE(pub i32);
pub const WHvX64PendingInterrupt: WHV_X64_PENDING_INTERRUPTION_TYPE =
    WHV_X64_PENDING_INTERRUPTION_TYPE(0i32);
pub const WHvX64PendingNmi: WHV_X64_PENDING_INTERRUPTION_TYPE =
    WHV_X64_PENDING_INTERRUPTION_TYPE(2i32);
pub const WHvX64PendingException: WHV_X64_PENDING_INTERRUPTION_TYPE =
    WHV_X64_PENDING_INTERRUPTION_TYPE(3i32);
impl ::std::convert::From<i32> for WHV_X64_PENDING_INTERRUPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_PENDING_INTERRUPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_RDTSC_CONTEXT {
    pub TscAux: u64,
    pub VirtualOffset: u64,
    pub Tsc: u64,
    pub ReferenceTime: u64,
    pub RdtscInfo: WHV_X64_RDTSC_INFO,
}
impl WHV_X64_RDTSC_CONTEXT {}
impl ::std::default::Default for WHV_X64_RDTSC_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_RDTSC_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_RDTSC_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_RDTSC_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_RDTSC_INFO {
    pub Anonymous: WHV_X64_RDTSC_INFO_0,
    pub AsUINT64: u64,
}
impl WHV_X64_RDTSC_INFO {}
impl ::std::default::Default for WHV_X64_RDTSC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_RDTSC_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_RDTSC_INFO {}
unsafe impl ::windows::runtime::Abi for WHV_X64_RDTSC_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_RDTSC_INFO_0 {
    pub _bitfield: u64,
}
impl WHV_X64_RDTSC_INFO_0 {}
impl ::std::default::Default for WHV_X64_RDTSC_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_RDTSC_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_RDTSC_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_RDTSC_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_RDTSC_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_SEGMENT_REGISTER {
    pub Base: u64,
    pub Limit: u32,
    pub Selector: u16,
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0,
}
impl WHV_X64_SEGMENT_REGISTER {}
impl ::std::default::Default for WHV_X64_SEGMENT_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_SEGMENT_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_SEGMENT_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_SEGMENT_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_SEGMENT_REGISTER_0 {
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0_0,
    pub Attributes: u16,
}
impl WHV_X64_SEGMENT_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_SEGMENT_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_SEGMENT_REGISTER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_SEGMENT_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_SEGMENT_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_SEGMENT_REGISTER_0_0 {
    pub _bitfield: u16,
}
impl WHV_X64_SEGMENT_REGISTER_0_0 {}
impl ::std::default::Default for WHV_X64_SEGMENT_REGISTER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_SEGMENT_REGISTER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_SEGMENT_REGISTER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_SEGMENT_REGISTER_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_SEGMENT_REGISTER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_TABLE_REGISTER {
    pub Pad: [u16; 3],
    pub Limit: u16,
    pub Base: u64,
}
impl WHV_X64_TABLE_REGISTER {}
impl ::std::default::Default for WHV_X64_TABLE_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_TABLE_REGISTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_TABLE_REGISTER")
            .field("Pad", &self.Pad)
            .field("Limit", &self.Limit)
            .field("Base", &self.Base)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_TABLE_REGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Pad == other.Pad && self.Limit == other.Limit && self.Base == other.Base
    }
}
impl ::std::cmp::Eq for WHV_X64_TABLE_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_TABLE_REGISTER {
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
pub struct WHV_X64_UNSUPPORTED_FEATURE_CODE(pub i32);
pub const WHvUnsupportedFeatureIntercept: WHV_X64_UNSUPPORTED_FEATURE_CODE =
    WHV_X64_UNSUPPORTED_FEATURE_CODE(1i32);
pub const WHvUnsupportedFeatureTaskSwitchTss: WHV_X64_UNSUPPORTED_FEATURE_CODE =
    WHV_X64_UNSUPPORTED_FEATURE_CODE(2i32);
impl ::std::convert::From<i32> for WHV_X64_UNSUPPORTED_FEATURE_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WHV_X64_UNSUPPORTED_FEATURE_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    pub FeatureCode: WHV_X64_UNSUPPORTED_FEATURE_CODE,
    pub Reserved: u32,
    pub FeatureParameter: u64,
}
impl WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {}
impl ::std::default::Default for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WHV_X64_UNSUPPORTED_FEATURE_CONTEXT")
            .field("FeatureCode", &self.FeatureCode)
            .field("Reserved", &self.Reserved)
            .field("FeatureParameter", &self.FeatureParameter)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.FeatureCode == other.FeatureCode
            && self.Reserved == other.Reserved
            && self.FeatureParameter == other.FeatureParameter
    }
}
impl ::std::cmp::Eq for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {}
unsafe impl ::windows::runtime::Abi for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_VP_EXECUTION_STATE {
    pub Anonymous: WHV_X64_VP_EXECUTION_STATE_0,
    pub AsUINT16: u16,
}
impl WHV_X64_VP_EXECUTION_STATE {}
impl ::std::default::Default for WHV_X64_VP_EXECUTION_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_VP_EXECUTION_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_VP_EXECUTION_STATE {}
unsafe impl ::windows::runtime::Abi for WHV_X64_VP_EXECUTION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_VP_EXECUTION_STATE_0 {
    pub _bitfield: u16,
}
impl WHV_X64_VP_EXECUTION_STATE_0 {}
impl ::std::default::Default for WHV_X64_VP_EXECUTION_STATE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_VP_EXECUTION_STATE_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_VP_EXECUTION_STATE_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WHV_X64_VP_EXECUTION_STATE_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_VP_EXECUTION_STATE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl WHV_X64_XMM_CONTROL_STATUS_REGISTER {}
impl ::std::default::Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_XMM_CONTROL_STATUS_REGISTER {}
unsafe impl ::windows::runtime::Abi for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0,
    pub XmmStatusControl: u32,
    pub XmmStatusControlMask: u32,
}
impl WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {}
impl ::std::default::Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRdp: u64,
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0,
}
impl WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {}
impl ::std::default::Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpDp: u32,
    pub LastFpDs: u16,
    pub Reserved: u16,
}
impl WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {}
impl ::std::default::Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("LastFpDp", &self.LastFpDp)
            .field("LastFpDs", &self.LastFpDs)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LastFpDp == other.LastFpDp
            && self.LastFpDs == other.LastFpDs
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {}
unsafe impl ::windows::runtime::Abi for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvAcceptPartitionMigration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    migrationhandle: Param0,
) -> ::windows::runtime::Result<WHV_PARTITION_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvAcceptPartitionMigration(
                migrationhandle: super::super::Foundation::HANDLE,
                partition: *mut WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WHV_PARTITION_HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvAcceptPartitionMigration(migrationhandle.into_param().abi(), &mut result__)
            .from_abi::<WHV_PARTITION_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvAdviseGpaRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    gparanges: *const WHV_MEMORY_RANGE_ENTRY,
    gparangescount: u32,
    advice: WHV_ADVISE_GPA_RANGE_CODE,
    advicebuffer: *const ::std::ffi::c_void,
    advicebuffersizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvAdviseGpaRange(
                partition: WHV_PARTITION_HANDLE,
                gparanges: *const WHV_MEMORY_RANGE_ENTRY,
                gparangescount: u32,
                advice: WHV_ADVISE_GPA_RANGE_CODE,
                advicebuffer: *const ::std::ffi::c_void,
                advicebuffersizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvAdviseGpaRange(
            partition.into_param().abi(),
            ::std::mem::transmute(gparanges),
            ::std::mem::transmute(gparangescount),
            ::std::mem::transmute(advice),
            ::std::mem::transmute(advicebuffer),
            ::std::mem::transmute(advicebuffersizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvAllocateVpciResource(
    providerid: *const ::windows::runtime::GUID,
    flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS,
    resourcedescriptor: *const ::std::ffi::c_void,
    resourcedescriptorsizeinbytes: u32,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvAllocateVpciResource(
                providerid: *const ::windows::runtime::GUID,
                flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS,
                resourcedescriptor: *const ::std::ffi::c_void,
                resourcedescriptorsizeinbytes: u32,
                vpciresource: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvAllocateVpciResource(
            ::std::mem::transmute(providerid),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(resourcedescriptor),
            ::std::mem::transmute(resourcedescriptorsizeinbytes),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCancelPartitionMigration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCancelPartitionMigration(
                partition: WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCancelPartitionMigration(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCancelRunVirtualProcessor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    flags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCancelRunVirtualProcessor(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                flags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCancelRunVirtualProcessor(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCompletePartitionMigration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCompletePartitionMigration(
                partition: WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCompletePartitionMigration(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvCreateNotificationPort<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    partition: Param0,
    parameters: *const WHV_NOTIFICATION_PORT_PARAMETERS,
    eventhandle: Param2,
    porthandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreateNotificationPort(
                partition: WHV_PARTITION_HANDLE,
                parameters: *const WHV_NOTIFICATION_PORT_PARAMETERS,
                eventhandle: super::super::Foundation::HANDLE,
                porthandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCreateNotificationPort(
            partition.into_param().abi(),
            ::std::mem::transmute(parameters),
            eventhandle.into_param().abi(),
            ::std::mem::transmute(porthandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCreatePartition() -> ::windows::runtime::Result<WHV_PARTITION_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreatePartition(
                partition: *mut WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WHV_PARTITION_HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvCreatePartition(&mut result__).from_abi::<WHV_PARTITION_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvCreateTrigger<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    parameters: *const WHV_TRIGGER_PARAMETERS,
    triggerhandle: *mut *mut ::std::ffi::c_void,
    eventhandle: *mut super::super::Foundation::HANDLE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreateTrigger(
                partition: WHV_PARTITION_HANDLE,
                parameters: *const WHV_TRIGGER_PARAMETERS,
                triggerhandle: *mut *mut ::std::ffi::c_void,
                eventhandle: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCreateTrigger(
            partition.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(triggerhandle),
            ::std::mem::transmute(eventhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCreateVirtualProcessor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    flags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreateVirtualProcessor(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                flags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCreateVirtualProcessor(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvCreateVirtualProcessor2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    properties: *const WHV_VIRTUAL_PROCESSOR_PROPERTY,
    propertycount: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreateVirtualProcessor2(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                properties: *const WHV_VIRTUAL_PROCESSOR_PROPERTY,
                propertycount: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCreateVirtualProcessor2(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(properties),
            ::std::mem::transmute(propertycount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvCreateVpciDevice<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    vpciresource: Param2,
    flags: WHV_CREATE_VPCI_DEVICE_FLAGS,
    notificationeventhandle: Param4,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvCreateVpciDevice(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                vpciresource: super::super::Foundation::HANDLE,
                flags: WHV_CREATE_VPCI_DEVICE_FLAGS,
                notificationeventhandle: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvCreateVpciDevice(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            vpciresource.into_param().abi(),
            ::std::mem::transmute(flags),
            notificationeventhandle.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvDeleteNotificationPort<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    porthandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvDeleteNotificationPort(
                partition: WHV_PARTITION_HANDLE,
                porthandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvDeleteNotificationPort(
            partition.into_param().abi(),
            ::std::mem::transmute(porthandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvDeletePartition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvDeletePartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
        }
        WHvDeletePartition(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvDeleteTrigger<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    triggerhandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvDeleteTrigger(
                partition: WHV_PARTITION_HANDLE,
                triggerhandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvDeleteTrigger(
            partition.into_param().abi(),
            ::std::mem::transmute(triggerhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvDeleteVirtualProcessor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvDeleteVirtualProcessor(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvDeleteVirtualProcessor(partition.into_param().abi(), ::std::mem::transmute(vpindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvDeleteVpciDevice<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvDeleteVpciDevice(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvDeleteVpciDevice(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvEmulatorCreateEmulator(
    callbacks: *const WHV_EMULATOR_CALLBACKS,
    emulator: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvemulation")]
        extern "system" {
            fn WHvEmulatorCreateEmulator(
                callbacks: *const ::std::mem::ManuallyDrop<WHV_EMULATOR_CALLBACKS>,
                emulator: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvEmulatorCreateEmulator(
            ::std::mem::transmute(callbacks),
            ::std::mem::transmute(emulator),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvEmulatorDestroyEmulator(
    emulator: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvemulation")]
        extern "system" {
            fn WHvEmulatorDestroyEmulator(
                emulator: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvEmulatorDestroyEmulator(::std::mem::transmute(emulator)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvEmulatorTryIoEmulation(
    emulator: *const ::std::ffi::c_void,
    context: *const ::std::ffi::c_void,
    vpcontext: *const WHV_VP_EXIT_CONTEXT,
    ioinstructioncontext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT,
) -> ::windows::runtime::Result<WHV_EMULATOR_STATUS> {
    #[cfg(windows)]
    {
        #[link(name = "winhvemulation")]
        extern "system" {
            fn WHvEmulatorTryIoEmulation(
                emulator: *const ::std::ffi::c_void,
                context: *const ::std::ffi::c_void,
                vpcontext: *const WHV_VP_EXIT_CONTEXT,
                ioinstructioncontext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT,
                emulatorreturnstatus: *mut WHV_EMULATOR_STATUS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WHV_EMULATOR_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvEmulatorTryIoEmulation(
            ::std::mem::transmute(emulator),
            ::std::mem::transmute(context),
            ::std::mem::transmute(vpcontext),
            ::std::mem::transmute(ioinstructioncontext),
            &mut result__,
        )
        .from_abi::<WHV_EMULATOR_STATUS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvEmulatorTryMmioEmulation(
    emulator: *const ::std::ffi::c_void,
    context: *const ::std::ffi::c_void,
    vpcontext: *const WHV_VP_EXIT_CONTEXT,
    mmioinstructioncontext: *const WHV_MEMORY_ACCESS_CONTEXT,
) -> ::windows::runtime::Result<WHV_EMULATOR_STATUS> {
    #[cfg(windows)]
    {
        #[link(name = "winhvemulation")]
        extern "system" {
            fn WHvEmulatorTryMmioEmulation(
                emulator: *const ::std::ffi::c_void,
                context: *const ::std::ffi::c_void,
                vpcontext: *const WHV_VP_EXIT_CONTEXT,
                mmioinstructioncontext: *const WHV_MEMORY_ACCESS_CONTEXT,
                emulatorreturnstatus: *mut WHV_EMULATOR_STATUS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WHV_EMULATOR_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvEmulatorTryMmioEmulation(
            ::std::mem::transmute(emulator),
            ::std::mem::transmute(context),
            ::std::mem::transmute(vpcontext),
            ::std::mem::transmute(mmioinstructioncontext),
            &mut result__,
        )
        .from_abi::<WHV_EMULATOR_STATUS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetCapability(
    capabilitycode: WHV_CAPABILITY_CODE,
    capabilitybuffer: *mut ::std::ffi::c_void,
    capabilitybuffersizeinbytes: u32,
    writtensizeinbytes: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetCapability(
                capabilitycode: WHV_CAPABILITY_CODE,
                capabilitybuffer: *mut ::std::ffi::c_void,
                capabilitybuffersizeinbytes: u32,
                writtensizeinbytes: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetCapability(
            ::std::mem::transmute(capabilitycode),
            ::std::mem::transmute(capabilitybuffer),
            ::std::mem::transmute(capabilitybuffersizeinbytes),
            ::std::mem::transmute(writtensizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetInterruptTargetVpSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    destination: u64,
    destinationmode: WHV_INTERRUPT_DESTINATION_MODE,
    targetvps: *mut u32,
    vpcount: u32,
    targetvpcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetInterruptTargetVpSet(
                partition: WHV_PARTITION_HANDLE,
                destination: u64,
                destinationmode: WHV_INTERRUPT_DESTINATION_MODE,
                targetvps: *mut u32,
                vpcount: u32,
                targetvpcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetInterruptTargetVpSet(
            partition.into_param().abi(),
            ::std::mem::transmute(destination),
            ::std::mem::transmute(destinationmode),
            ::std::mem::transmute(targetvps),
            ::std::mem::transmute(vpcount),
            ::std::mem::transmute(targetvpcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetPartitionCounters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    counterset: WHV_PARTITION_COUNTER_SET,
    buffer: *mut ::std::ffi::c_void,
    buffersizeinbytes: u32,
    byteswritten: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetPartitionCounters(
                partition: WHV_PARTITION_HANDLE,
                counterset: WHV_PARTITION_COUNTER_SET,
                buffer: *mut ::std::ffi::c_void,
                buffersizeinbytes: u32,
                byteswritten: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetPartitionCounters(
            partition.into_param().abi(),
            ::std::mem::transmute(counterset),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
            ::std::mem::transmute(byteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetPartitionProperty<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    propertycode: WHV_PARTITION_PROPERTY_CODE,
    propertybuffer: *mut ::std::ffi::c_void,
    propertybuffersizeinbytes: u32,
    writtensizeinbytes: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetPartitionProperty(
                partition: WHV_PARTITION_HANDLE,
                propertycode: WHV_PARTITION_PROPERTY_CODE,
                propertybuffer: *mut ::std::ffi::c_void,
                propertybuffersizeinbytes: u32,
                writtensizeinbytes: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetPartitionProperty(
            partition.into_param().abi(),
            ::std::mem::transmute(propertycode),
            ::std::mem::transmute(propertybuffer),
            ::std::mem::transmute(propertybuffersizeinbytes),
            ::std::mem::transmute(writtensizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorCounters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    counterset: WHV_PROCESSOR_COUNTER_SET,
    buffer: *mut ::std::ffi::c_void,
    buffersizeinbytes: u32,
    byteswritten: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorCounters(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                counterset: WHV_PROCESSOR_COUNTER_SET,
                buffer: *mut ::std::ffi::c_void,
                buffersizeinbytes: u32,
                byteswritten: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorCounters(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(counterset),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
            ::std::mem::transmute(byteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorCpuidOutput<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    eax: u32,
    ecx: u32,
) -> ::windows::runtime::Result<WHV_CPUID_OUTPUT> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorCpuidOutput(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                eax: u32,
                ecx: u32,
                cpuidoutput: *mut WHV_CPUID_OUTPUT,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WHV_CPUID_OUTPUT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WHvGetVirtualProcessorCpuidOutput(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(eax),
            ::std::mem::transmute(ecx),
            &mut result__,
        )
        .from_abi::<WHV_CPUID_OUTPUT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorInterruptControllerState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    state: *mut ::std::ffi::c_void,
    statesize: u32,
    writtensize: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorInterruptControllerState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                state: *mut ::std::ffi::c_void,
                statesize: u32,
                writtensize: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorInterruptControllerState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(state),
            ::std::mem::transmute(statesize),
            ::std::mem::transmute(writtensize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorInterruptControllerState2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    state: *mut ::std::ffi::c_void,
    statesize: u32,
    writtensize: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorInterruptControllerState2(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                state: *mut ::std::ffi::c_void,
                statesize: u32,
                writtensize: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorInterruptControllerState2(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(state),
            ::std::mem::transmute(statesize),
            ::std::mem::transmute(writtensize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorRegisters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    registernames: *const WHV_REGISTER_NAME,
    registercount: u32,
    registervalues: *mut WHV_REGISTER_VALUE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorRegisters(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                registernames: *const WHV_REGISTER_NAME,
                registercount: u32,
                registervalues: *mut WHV_REGISTER_VALUE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorRegisters(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(registernames),
            ::std::mem::transmute(registercount),
            ::std::mem::transmute(registervalues),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE,
    buffer: *mut ::std::ffi::c_void,
    buffersizeinbytes: u32,
    byteswritten: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE,
                buffer: *mut ::std::ffi::c_void,
                buffersizeinbytes: u32,
                byteswritten: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(statetype),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
            ::std::mem::transmute(byteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVirtualProcessorXsaveState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    buffer: *mut ::std::ffi::c_void,
    buffersizeinbytes: u32,
    byteswritten: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVirtualProcessorXsaveState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                buffer: *mut ::std::ffi::c_void,
                buffersizeinbytes: u32,
                byteswritten: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVirtualProcessorXsaveState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
            ::std::mem::transmute(byteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVpciDeviceInterruptTarget<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    index: u32,
    multimessagenumber: u32,
    target: *mut WHV_VPCI_INTERRUPT_TARGET,
    targetsizeinbytes: u32,
    byteswritten: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVpciDeviceInterruptTarget(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                index: u32,
                multimessagenumber: u32,
                target: *mut WHV_VPCI_INTERRUPT_TARGET,
                targetsizeinbytes: u32,
                byteswritten: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVpciDeviceInterruptTarget(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(index),
            ::std::mem::transmute(multimessagenumber),
            ::std::mem::transmute(target),
            ::std::mem::transmute(targetsizeinbytes),
            ::std::mem::transmute(byteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVpciDeviceNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    notification: *mut WHV_VPCI_DEVICE_NOTIFICATION,
    notificationsizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVpciDeviceNotification(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                notification: *mut WHV_VPCI_DEVICE_NOTIFICATION,
                notificationsizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVpciDeviceNotification(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(notification),
            ::std::mem::transmute(notificationsizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvGetVpciDeviceProperty<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    propertycode: WHV_VPCI_DEVICE_PROPERTY_CODE,
    propertybuffer: *mut ::std::ffi::c_void,
    propertybuffersizeinbytes: u32,
    writtensizeinbytes: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvGetVpciDeviceProperty(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                propertycode: WHV_VPCI_DEVICE_PROPERTY_CODE,
                propertybuffer: *mut ::std::ffi::c_void,
                propertybuffersizeinbytes: u32,
                writtensizeinbytes: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvGetVpciDeviceProperty(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(propertycode),
            ::std::mem::transmute(propertybuffer),
            ::std::mem::transmute(propertybuffersizeinbytes),
            ::std::mem::transmute(writtensizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvMapGpaRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    sourceaddress: *const ::std::ffi::c_void,
    guestaddress: u64,
    sizeinbytes: u64,
    flags: WHV_MAP_GPA_RANGE_FLAGS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvMapGpaRange(
                partition: WHV_PARTITION_HANDLE,
                sourceaddress: *const ::std::ffi::c_void,
                guestaddress: u64,
                sizeinbytes: u64,
                flags: WHV_MAP_GPA_RANGE_FLAGS,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvMapGpaRange(
            partition.into_param().abi(),
            ::std::mem::transmute(sourceaddress),
            ::std::mem::transmute(guestaddress),
            ::std::mem::transmute(sizeinbytes),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvMapGpaRange2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    partition: Param0,
    process: Param1,
    sourceaddress: *const ::std::ffi::c_void,
    guestaddress: u64,
    sizeinbytes: u64,
    flags: WHV_MAP_GPA_RANGE_FLAGS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvMapGpaRange2(
                partition: WHV_PARTITION_HANDLE,
                process: super::super::Foundation::HANDLE,
                sourceaddress: *const ::std::ffi::c_void,
                guestaddress: u64,
                sizeinbytes: u64,
                flags: WHV_MAP_GPA_RANGE_FLAGS,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvMapGpaRange2(
            partition.into_param().abi(),
            process.into_param().abi(),
            ::std::mem::transmute(sourceaddress),
            ::std::mem::transmute(guestaddress),
            ::std::mem::transmute(sizeinbytes),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvMapVpciDeviceInterrupt<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    index: u32,
    messagecount: u32,
    target: *const WHV_VPCI_INTERRUPT_TARGET,
    msiaddress: *mut u64,
    msidata: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvMapVpciDeviceInterrupt(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                index: u32,
                messagecount: u32,
                target: *const WHV_VPCI_INTERRUPT_TARGET,
                msiaddress: *mut u64,
                msidata: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvMapVpciDeviceInterrupt(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(index),
            ::std::mem::transmute(messagecount),
            ::std::mem::transmute(target),
            ::std::mem::transmute(msiaddress),
            ::std::mem::transmute(msidata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvMapVpciDeviceMmioRanges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    mappingcount: *mut u32,
    mappings: *mut *mut WHV_VPCI_MMIO_MAPPING,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvMapVpciDeviceMmioRanges(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                mappingcount: *mut u32,
                mappings: *mut *mut WHV_VPCI_MMIO_MAPPING,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvMapVpciDeviceMmioRanges(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(mappingcount),
            ::std::mem::transmute(mappings),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvPostVirtualProcessorSynicMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    sintindex: u32,
    message: *const ::std::ffi::c_void,
    messagesizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvPostVirtualProcessorSynicMessage(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                sintindex: u32,
                message: *const ::std::ffi::c_void,
                messagesizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvPostVirtualProcessorSynicMessage(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(sintindex),
            ::std::mem::transmute(message),
            ::std::mem::transmute(messagesizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvQueryGpaRangeDirtyBitmap<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    guestaddress: u64,
    rangesizeinbytes: u64,
    bitmap: *mut u64,
    bitmapsizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvQueryGpaRangeDirtyBitmap(
                partition: WHV_PARTITION_HANDLE,
                guestaddress: u64,
                rangesizeinbytes: u64,
                bitmap: *mut u64,
                bitmapsizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvQueryGpaRangeDirtyBitmap(
            partition.into_param().abi(),
            ::std::mem::transmute(guestaddress),
            ::std::mem::transmute(rangesizeinbytes),
            ::std::mem::transmute(bitmap),
            ::std::mem::transmute(bitmapsizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvReadGpaRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, WHV_ACCESS_GPA_CONTROLS>,
>(
    partition: Param0,
    vpindex: u32,
    guestaddress: u64,
    controls: Param3,
    data: *mut ::std::ffi::c_void,
    datasizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvReadGpaRange(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                guestaddress: u64,
                controls: WHV_ACCESS_GPA_CONTROLS,
                data: *mut ::std::ffi::c_void,
                datasizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvReadGpaRange(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(guestaddress),
            controls.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datasizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvReadVpciDeviceRegister<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    register: *const WHV_VPCI_DEVICE_REGISTER,
    data: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvReadVpciDeviceRegister(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                register: *const WHV_VPCI_DEVICE_REGISTER,
                data: *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvReadVpciDeviceRegister(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(register),
            ::std::mem::transmute(data),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvRegisterPartitionDoorbellEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    partition: Param0,
    matchdata: *const WHV_DOORBELL_MATCH_DATA,
    eventhandle: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvRegisterPartitionDoorbellEvent(
                partition: WHV_PARTITION_HANDLE,
                matchdata: *const WHV_DOORBELL_MATCH_DATA,
                eventhandle: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvRegisterPartitionDoorbellEvent(
            partition.into_param().abi(),
            ::std::mem::transmute(matchdata),
            eventhandle.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvRequestInterrupt<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    interrupt: *const WHV_INTERRUPT_CONTROL,
    interruptcontrolsize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvRequestInterrupt(
                partition: WHV_PARTITION_HANDLE,
                interrupt: *const WHV_INTERRUPT_CONTROL,
                interruptcontrolsize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvRequestInterrupt(
            partition.into_param().abi(),
            ::std::mem::transmute(interrupt),
            ::std::mem::transmute(interruptcontrolsize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvRequestVpciDeviceInterrupt<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    msiaddress: u64,
    msidata: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvRequestVpciDeviceInterrupt(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                msiaddress: u64,
                msidata: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvRequestVpciDeviceInterrupt(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(msiaddress),
            ::std::mem::transmute(msidata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvResetPartition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvResetPartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
        }
        WHvResetPartition(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvResumePartitionTime<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvResumePartitionTime(
                partition: WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvResumePartitionTime(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvRetargetVpciDeviceInterrupt<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    msiaddress: u64,
    msidata: u32,
    target: *const WHV_VPCI_INTERRUPT_TARGET,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvRetargetVpciDeviceInterrupt(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                msiaddress: u64,
                msidata: u32,
                target: *const WHV_VPCI_INTERRUPT_TARGET,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvRetargetVpciDeviceInterrupt(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(msiaddress),
            ::std::mem::transmute(msidata),
            ::std::mem::transmute(target),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvRunVirtualProcessor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    exitcontext: *mut ::std::ffi::c_void,
    exitcontextsizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvRunVirtualProcessor(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                exitcontext: *mut ::std::ffi::c_void,
                exitcontextsizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvRunVirtualProcessor(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(exitcontext),
            ::std::mem::transmute(exitcontextsizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetNotificationPortProperty<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    porthandle: *const ::std::ffi::c_void,
    propertycode: WHV_NOTIFICATION_PORT_PROPERTY_CODE,
    propertyvalue: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetNotificationPortProperty(
                partition: WHV_PARTITION_HANDLE,
                porthandle: *const ::std::ffi::c_void,
                propertycode: WHV_NOTIFICATION_PORT_PROPERTY_CODE,
                propertyvalue: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetNotificationPortProperty(
            partition.into_param().abi(),
            ::std::mem::transmute(porthandle),
            ::std::mem::transmute(propertycode),
            ::std::mem::transmute(propertyvalue),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetPartitionProperty<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    propertycode: WHV_PARTITION_PROPERTY_CODE,
    propertybuffer: *const ::std::ffi::c_void,
    propertybuffersizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetPartitionProperty(
                partition: WHV_PARTITION_HANDLE,
                propertycode: WHV_PARTITION_PROPERTY_CODE,
                propertybuffer: *const ::std::ffi::c_void,
                propertybuffersizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetPartitionProperty(
            partition.into_param().abi(),
            ::std::mem::transmute(propertycode),
            ::std::mem::transmute(propertybuffer),
            ::std::mem::transmute(propertybuffersizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetVirtualProcessorInterruptControllerState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    state: *const ::std::ffi::c_void,
    statesize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVirtualProcessorInterruptControllerState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                state: *const ::std::ffi::c_void,
                statesize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVirtualProcessorInterruptControllerState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(state),
            ::std::mem::transmute(statesize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetVirtualProcessorInterruptControllerState2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    state: *const ::std::ffi::c_void,
    statesize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVirtualProcessorInterruptControllerState2(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                state: *const ::std::ffi::c_void,
                statesize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVirtualProcessorInterruptControllerState2(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(state),
            ::std::mem::transmute(statesize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetVirtualProcessorRegisters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    registernames: *const WHV_REGISTER_NAME,
    registercount: u32,
    registervalues: *const WHV_REGISTER_VALUE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVirtualProcessorRegisters(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                registernames: *const WHV_REGISTER_NAME,
                registercount: u32,
                registervalues: *const WHV_REGISTER_VALUE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVirtualProcessorRegisters(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(registernames),
            ::std::mem::transmute(registercount),
            ::std::mem::transmute(registervalues),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetVirtualProcessorState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE,
    buffer: *const ::std::ffi::c_void,
    buffersizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVirtualProcessorState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE,
                buffer: *const ::std::ffi::c_void,
                buffersizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVirtualProcessorState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(statetype),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetVirtualProcessorXsaveState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    buffer: *const ::std::ffi::c_void,
    buffersizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVirtualProcessorXsaveState(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                buffer: *const ::std::ffi::c_void,
                buffersizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVirtualProcessorXsaveState(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn WHvSetVpciDevicePowerState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    powerstate: super::SystemServices::DEVICE_POWER_STATE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetVpciDevicePowerState(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                powerstate: super::SystemServices::DEVICE_POWER_STATE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSetVpciDevicePowerState(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(powerstate),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSetupPartition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSetupPartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
        }
        WHvSetupPartition(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvSignalVirtualProcessorSynicEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, WHV_SYNIC_EVENT_PARAMETERS>,
>(
    partition: Param0,
    synicevent: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSignalVirtualProcessorSynicEvent(
                partition: WHV_PARTITION_HANDLE,
                synicevent: WHV_SYNIC_EVENT_PARAMETERS,
                newlysignaled: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvSignalVirtualProcessorSynicEvent(
            partition.into_param().abi(),
            synicevent.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WHvStartPartitionMigration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvStartPartitionMigration(
                partition: WHV_PARTITION_HANDLE,
                migrationhandle: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        WHvStartPartitionMigration(partition.into_param().abi(), &mut result__)
            .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvSuspendPartitionTime<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvSuspendPartitionTime(
                partition: WHV_PARTITION_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvSuspendPartitionTime(partition.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvTranslateGva<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    vpindex: u32,
    gva: u64,
    translateflags: WHV_TRANSLATE_GVA_FLAGS,
    translationresult: *mut WHV_TRANSLATE_GVA_RESULT,
    gpa: *mut u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvTranslateGva(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                gva: u64,
                translateflags: WHV_TRANSLATE_GVA_FLAGS,
                translationresult: *mut WHV_TRANSLATE_GVA_RESULT,
                gpa: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvTranslateGva(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(gva),
            ::std::mem::transmute(translateflags),
            ::std::mem::transmute(translationresult),
            ::std::mem::transmute(gpa),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvUnmapGpaRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    guestaddress: u64,
    sizeinbytes: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvUnmapGpaRange(
                partition: WHV_PARTITION_HANDLE,
                guestaddress: u64,
                sizeinbytes: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvUnmapGpaRange(
            partition.into_param().abi(),
            ::std::mem::transmute(guestaddress),
            ::std::mem::transmute(sizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvUnmapVpciDeviceInterrupt<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    index: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvUnmapVpciDeviceInterrupt(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                index: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvUnmapVpciDeviceInterrupt(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(index),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvUnmapVpciDeviceMmioRanges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvUnmapVpciDeviceMmioRanges(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvUnmapVpciDeviceMmioRanges(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvUnregisterPartitionDoorbellEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    matchdata: *const WHV_DOORBELL_MATCH_DATA,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvUnregisterPartitionDoorbellEvent(
                partition: WHV_PARTITION_HANDLE,
                matchdata: *const WHV_DOORBELL_MATCH_DATA,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvUnregisterPartitionDoorbellEvent(
            partition.into_param().abi(),
            ::std::mem::transmute(matchdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvUpdateTriggerParameters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    parameters: *const WHV_TRIGGER_PARAMETERS,
    triggerhandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvUpdateTriggerParameters(
                partition: WHV_PARTITION_HANDLE,
                parameters: *const WHV_TRIGGER_PARAMETERS,
                triggerhandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvUpdateTriggerParameters(
            partition.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(triggerhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvWriteGpaRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, WHV_ACCESS_GPA_CONTROLS>,
>(
    partition: Param0,
    vpindex: u32,
    guestaddress: u64,
    controls: Param3,
    data: *const ::std::ffi::c_void,
    datasizeinbytes: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvWriteGpaRange(
                partition: WHV_PARTITION_HANDLE,
                vpindex: u32,
                guestaddress: u64,
                controls: WHV_ACCESS_GPA_CONTROLS,
                data: *const ::std::ffi::c_void,
                datasizeinbytes: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvWriteGpaRange(
            partition.into_param().abi(),
            ::std::mem::transmute(vpindex),
            ::std::mem::transmute(guestaddress),
            controls.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(datasizeinbytes),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WHvWriteVpciDeviceRegister<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, WHV_PARTITION_HANDLE>,
>(
    partition: Param0,
    logicaldeviceid: u64,
    register: *const WHV_VPCI_DEVICE_REGISTER,
    data: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "winhvplatform")]
        extern "system" {
            fn WHvWriteVpciDeviceRegister(
                partition: WHV_PARTITION_HANDLE,
                logicaldeviceid: u64,
                register: *const WHV_VPCI_DEVICE_REGISTER,
                data: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        WHvWriteVpciDeviceRegister(
            partition.into_param().abi(),
            ::std::mem::transmute(logicaldeviceid),
            ::std::mem::transmute(register),
            ::std::mem::transmute(data),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
