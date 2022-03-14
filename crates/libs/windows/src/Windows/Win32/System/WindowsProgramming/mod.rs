#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AADBE_ADD_ENTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AADBE_DEL_ENTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_2600")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACTCTX_SECTION_KEYED_DATA_2600 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTCTX_SECTION_KEYED_DATA_2600>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut ::core::ffi::c_void,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA").field("lpInformation", &self.lpInformation).field("lpSectionBase", &self.lpSectionBase).field("ulSectionLength", &self.ulSectionLength).field("lpSectionGlobalDataBase", &self.lpSectionGlobalDataBase).field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength).finish()
    }
}
unsafe impl ::windows::core::Abi for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_BASIC_INFORMATION").field("hActCtx", &self.hActCtx).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_BASIC_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_OFFLINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_ONLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_BACKNEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NODELETENEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NOPROGRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_UPDREFCNT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_USEREFCNT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOOVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_QUIET: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_REPLACEONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_WARNIFSKIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_BKINSTALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_CHECKBKDATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_NGCONV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_QUIET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_ROLLBACK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type APPLICATION_RECOVERY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_REGSECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_ARP: u32 = 640u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_NULL: u32 = 642u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn AddDelBackupEntryA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpcszfilelist: Param0, lpcszbackupdir: Param1, lpcszbasename: Param2, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDelBackupEntryA(lpcszfilelist: ::windows::core::PCSTR, lpcszbackupdir: ::windows::core::PCSTR, lpcszbasename: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        AddDelBackupEntryA(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn AddDelBackupEntryW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpcszfilelist: Param0, lpcszbackupdir: Param1, lpcszbasename: Param2, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDelBackupEntryW(lpcszfilelist: ::windows::core::PCWSTR, lpcszbackupdir: ::windows::core::PCWSTR, lpcszbasename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        AddDelBackupEntryW(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, lpszsourcedir: Param1, lpszsourcefile: Param2, lpszdestdir: Param3, lpszdestfile: Param4, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdvInstallFileA(hwnd: super::super::Foundation::HWND, lpszsourcedir: ::windows::core::PCSTR, lpszsourcefile: ::windows::core::PCSTR, lpszdestdir: ::windows::core::PCSTR, lpszdestfile: ::windows::core::PCSTR, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        AdvInstallFileA(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, lpszsourcedir: Param1, lpszsourcefile: Param2, lpszdestdir: Param3, lpszdestfile: Param4, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdvInstallFileW(hwnd: super::super::Foundation::HWND, lpszsourcedir: ::windows::core::PCWSTR, lpszsourcefile: ::windows::core::PCWSTR, lpszdestdir: ::windows::core::PCWSTR, lpszdestfile: ::windows::core::PCWSTR, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        AdvInstallFileW(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApphelpCheckShellObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(objectclsid: *const ::windows::core::GUID, bshimifnecessary: Param1, pullflags: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApphelpCheckShellObject(objectclsid: *const ::windows::core::GUID, bshimifnecessary: super::super::Foundation::BOOL, pullflags: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ApphelpCheckShellObject(::core::mem::transmute(objectclsid), bshimifnecessary.into_param().abi(), ::core::mem::transmute(pullflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BACKUP_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_LOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINFOA {
    pub pszCab: ::windows::core::PSTR,
    pub pszInf: ::windows::core::PSTR,
    pub pszSection: ::windows::core::PSTR,
    pub szSrcPath: [super::super::Foundation::CHAR; 260],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CABINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOA").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CABINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CABINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CABINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct CABINFOW {
    pub pszCab: ::windows::core::PWSTR,
    pub pszInf: ::windows::core::PWSTR,
    pub pszSection: ::windows::core::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CABINFOW {}
impl ::core::clone::Clone for CABINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOW").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for CABINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CABINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for CABINFOW {}
impl ::core::default::Default for CABINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CATID_DeleteBrowsingHistory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31caf6e4_d6aa_4090_a050_a5ac8972e9ef);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_110: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_115200: u32 = 115200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_1200: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_128000: u32 = 128000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_14400: u32 = 14400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_19200: u32 = 19200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_2400: u32 = 2400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_256000: u32 = 256000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_300: u32 = 300u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_38400: u32 = 38400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_4800: u32 = 4800u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_56000: u32 = 56000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_57600: u32 = 57600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_600: u32 = 600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_9600: u32 = 9600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_DNS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_IOE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_MODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_OOP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_PTO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_TXFULL: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_ID").field("UniqueProcess", &self.UniqueProcess).field("UniqueThread", &self.UniqueThread).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLIENT_ID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIENT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIENT_ID>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_IP: u32 = 771u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_IPX: u32 = 769u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_NBF: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_UDP: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CONTEXT_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_RATE_MIN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DIRECTORY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_NO_BUFFERING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_RESTARTABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_NBF: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_SPP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_SPX: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_TCP: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_HWND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_LEVEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CREATE_FOR_DIR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CREATE_FOR_IMPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::clone::Clone for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG").field("Size", &self.Size).field("TriggerId", &self.TriggerId).finish()
    }
}
unsafe impl ::windows::core::Abi for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG>()) == 0 }
    }
}
impl ::core::cmp::Eq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::default::Default for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CameraUIControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16d5a2be_b1c5_47b3_8eae_ccbcf452c7e8);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlCaptureMode {}
impl ::core::clone::Clone for CameraUIControlCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlCaptureMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlCaptureMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlLinearSelectionMode {}
impl ::core::clone::Clone for CameraUIControlLinearSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlLinearSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlLinearSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlLinearSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlLinearSelectionMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlMode {}
impl ::core::clone::Clone for CameraUIControlMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlPhotoFormat {}
impl ::core::clone::Clone for CameraUIControlPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlPhotoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlPhotoFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlVideoFormat {}
impl ::core::clone::Clone for CameraUIControlVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlVideoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlVideoFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlViewType {}
impl ::core::clone::Clone for CameraUIControlViewType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlViewType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraUIControlViewType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraUIControlViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlViewType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelDeviceWakeupRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelDeviceWakeupRequest(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelDeviceWakeupRequest(hdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelTimerQueueTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0, timer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn CloseINFEngine(hinf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseINFEngine(hinf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CloseINFEngine(::core::mem::transmute(hinf)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::core::HRESULT;
        }
        ConvertAuxiliaryCounterToPerformanceCounter(::core::mem::transmute(ullauxiliarycountervalue), ::core::mem::transmute(lpperformancecountervalue), ::core::mem::transmute(lpconversionerror)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::core::HRESULT;
        }
        ConvertPerformanceCounterToAuxiliaryCounter(::core::mem::transmute(ullperformancecountervalue), ::core::mem::transmute(lpauxiliarycountervalue), ::core::mem::transmute(lpconversionerror)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateWaitableTimerA(::core::mem::transmute(lptimerattributes), bmanualreset.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: Param1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerExA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: ::windows::core::PCSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateWaitableTimerExA(::core::mem::transmute(lptimerattributes), lptimername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
impl ::core::marker::Copy for DATETIME {}
impl ::core::clone::Clone for DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIME").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).finish()
    }
}
unsafe impl ::windows::core::Abi for DATETIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DATETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DATETIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DATETIME {}
impl ::core::default::Default for DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32;
        }
        ::core::mem::transmute(DCIBeginAccess(::core::mem::transmute(pdci), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(dx), ::core::mem::transmute(dy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DCICMD {}
impl ::core::clone::Clone for DCICMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICMD").field("dwCommand", &self.dwCommand).field("dwParam1", &self.dwParam1).field("dwParam2", &self.dwParam2).field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DCICMD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCICMD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCICMD>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCICMD {}
impl ::core::default::Default for DCICMD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DCICREATEINPUT {}
impl ::core::clone::Clone for DCICREATEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICREATEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICREATEINPUT").field("cmd", &self.cmd).field("dwCompression", &self.dwCompression).field("dwMask", &self.dwMask).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwDCICaps", &self.dwDCICaps).field("dwBitCount", &self.dwBitCount).field("lpSurface", &self.lpSurface).finish()
    }
}
unsafe impl ::windows::core::Abi for DCICREATEINPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCICREATEINPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCICREATEINPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCICREATEINPUT {}
impl ::core::default::Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICloseProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICloseProvider(hdc: super::super::Graphics::Gdi::HDC);
        }
        DCICloseProvider(hdc.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOffscreen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreateOffscreen(hdc: super::super::Graphics::Gdi::HDC, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32;
        }
        ::core::mem::transmute(DCICreateOffscreen(hdc.into_param().abi(), ::core::mem::transmute(dwcompression), ::core::mem::transmute(dwredmask), ::core::mem::transmute(dwgreenmask), ::core::mem::transmute(dwbluemask), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwdcicaps), ::core::mem::transmute(dwbitcount), ::core::mem::transmute(lplpsurface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOverlay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpoffscreensurf: *mut ::core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreateOverlay(hdc: super::super::Graphics::Gdi::HDC, lpoffscreensurf: *mut ::core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32;
        }
        ::core::mem::transmute(DCICreateOverlay(hdc.into_param().abi(), ::core::mem::transmute(lpoffscreensurf), ::core::mem::transmute(lplpsurface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreatePrimary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lplpsurface: *mut *mut DCISURFACEINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreatePrimary(hdc: super::super::Graphics::Gdi::HDC, lplpsurface: *mut *mut DCISURFACEINFO) -> i32;
        }
        ::core::mem::transmute(DCICreatePrimary(hdc.into_param().abi(), ::core::mem::transmute(lplpsurface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIDestroy(pdci: *mut DCISURFACEINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIDestroy(pdci: *mut DCISURFACEINFO);
        }
        DCIDestroy(::core::mem::transmute(pdci))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32;
        }
        ::core::mem::transmute(DCIDraw(::core::mem::transmute(pdci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DCIENUMINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DCIENUMINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIENUMINPUT").field("cmd", &self.cmd).field("rSrc", &self.rSrc).field("rDst", &self.rDst).field("EnumCallback", &self.EnumCallback).field("lpContext", &self.lpContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DCIENUMINPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DCIENUMINPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCIENUMINPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCIENUMSURFACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCIESCAPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIEndAccess(pdci: *mut DCISURFACEINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIEndAccess(pdci: *mut DCISURFACEINFO);
        }
        DCIEndAccess(::core::mem::transmute(pdci))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCIEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIEnum(hdc: super::super::Graphics::Gdi::HDC, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(DCIEnum(hdc.into_param().abi(), ::core::mem::transmute(lprdst), ::core::mem::transmute(lprsrc), ::core::mem::transmute(lpfncallback), ::core::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
impl ::core::marker::Copy for DCIOFFSCREEN {}
impl ::core::clone::Clone for DCIOFFSCREEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOFFSCREEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOFFSCREEN").field("dciInfo", &self.dciInfo).field("Draw", &self.Draw).field("SetClipList", &self.SetClipList).field("SetDestination", &self.SetDestination).finish()
    }
}
unsafe impl ::windows::core::Abi for DCIOFFSCREEN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCIOFFSCREEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCIOFFSCREEN>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCIOFFSCREEN {}
impl ::core::default::Default for DCIOFFSCREEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
impl ::core::marker::Copy for DCIOVERLAY {}
impl ::core::clone::Clone for DCIOVERLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOVERLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOVERLAY").field("dciInfo", &self.dciInfo).field("dwChromakeyValue", &self.dwChromakeyValue).field("dwChromakeyMask", &self.dwChromakeyMask).finish()
    }
}
unsafe impl ::windows::core::Abi for DCIOVERLAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCIOVERLAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCIOVERLAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCIOVERLAY {}
impl ::core::default::Default for DCIOVERLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC;
        }
        ::core::mem::transmute(DCIOpenProvider())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl ::core::marker::Copy for DCISURFACEINFO {}
impl ::core::clone::Clone for DCISURFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCISURFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCISURFACEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lStride", &self.lStride)
            .field("dwBitCount", &self.dwBitCount)
            .field("dwOffSurface", &self.dwOffSurface)
            .field("wSelSurface", &self.wSelSurface)
            .field("wReserved", &self.wReserved)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("BeginAccess", &self.BeginAccess)
            .field("EndAccess", &self.EndAccess)
            .field("DestroySurface", &self.DestroySurface)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DCISURFACEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCISURFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCISURFACEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCISURFACEINFO {}
impl ::core::default::Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
        }
        ::core::mem::transmute(DCISetClipList(::core::mem::transmute(pdci), ::core::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32;
        }
        ::core::mem::transmute(DCISetDestination(::core::mem::transmute(pdci), ::core::mem::transmute(dst), ::core::mem::transmute(src)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
        }
        ::core::mem::transmute(DCISetSrcDestClip(::core::mem::transmute(pdci), ::core::mem::transmute(srcrc), ::core::mem::transmute(destrc), ::core::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_1632_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ASYNC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CANOVERLAY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CHROMAKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_DWORDALIGN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_DWORDSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_XALIGN: i32 = -17i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_XYALIGN: i32 = -19i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_YALIGN: i32 = -18i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_GENERIC: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OFFSCREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OVERLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_PRIMARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_SURFACE_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_VISIBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_WRITEONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DECISION_LOCATION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = DECISION_LOCATION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = DECISION_LOCATION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = DECISION_LOCATION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = DECISION_LOCATION(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = DECISION_LOCATION(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(6i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = DECISION_LOCATION(7i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = DECISION_LOCATION(8i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = DECISION_LOCATION(9i32);
impl ::core::marker::Copy for DECISION_LOCATION {}
impl ::core::clone::Clone for DECISION_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DECISION_LOCATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DECISION_LOCATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DECISION_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DECISION_LOCATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: ::windows::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DELAYLOAD_INFO {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DELAYLOAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELAYLOAD_INFO>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DELAYLOAD_INFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(target_arch = "x86")]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: ::windows::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for DELAYLOAD_INFO {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for DELAYLOAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELAYLOAD_INFO>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for DELAYLOAD_INFO {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DELAYLOAD_PROC_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELAYLOAD_PROC_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR {}
impl ::core::default::Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: ::windows::core::PCSTR,
    pub Ordinal: u32,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR_0 {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DELAYLOAD_PROC_DESCRIPTOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELAYLOAD_PROC_DESCRIPTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR_0 {}
impl ::core::default::Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_DOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_CDROM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_FIXED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_RAMDISK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
pub const DefaultBrowserSyncSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac83423_3112_4aa6_9b5b_1feb23d0c5f9);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DelNodeA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszfileordirname: Param0, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeA(pszfileordirname: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        DelNodeA(pszfileordirname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DelNodeRunDLL32W<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hwnd: Param0, hinstance: Param1, pszparms: ::windows::core::PWSTR, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeRunDLL32W(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PWSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        DelNodeRunDLL32W(hwnd.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(pszparms), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DelNodeW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszfileordirname: Param0, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeW(pszfileordirname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        DelNodeW(pszfileordirname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hostname: Param0, computername: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameA(hostname: ::windows::core::PCSTR, computername: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DnsHostnameToComputerNameA(hostname.into_param().abi(), ::core::mem::transmute(computername), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hostname: Param0, computername: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameW(hostname: ::windows::core::PCWSTR, computername: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DnsHostnameToComputerNameW(hostname.into_param().abi(), ::core::mem::transmute(computername), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DosDateTimeToFileTime(::core::mem::transmute(wfatdate), ::core::mem::transmute(wfattime), ::core::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ENTITY_LIST_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ENTITY_TYPE_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type ENUM_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ER_ICMP: u32 = 896u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4270827_4f39_45df_9288_12ff6b85a921);
pub const EditionUpgradeHelper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01776df3_b9af_4e50_9b1c_56e93116d704);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnableProcessOptionalXStateFeatures(::core::mem::transmute(features)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pcab: *mut CABINFOA, preserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteCabA(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOA, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ExecuteCabA(hwnd.into_param().abi(), ::core::mem::transmute(pcab), ::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pcab: *mut CABINFOW, preserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteCabW(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOW, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ExecuteCabW(hwnd.into_param().abi(), ::core::mem::transmute(pcab), ::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ExtractFilesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszcabname: Param0, pszexpanddir: Param1, dwflags: u32, pszfilelist: Param3, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractFilesA(pszcabname: ::windows::core::PCSTR, pszexpanddir: ::windows::core::PCSTR, dwflags: u32, pszfilelist: ::windows::core::PCSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        ExtractFilesA(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), ::core::mem::transmute(dwflags), pszfilelist.into_param().abi(), ::core::mem::transmute(lpreserved), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ExtractFilesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszcabname: Param0, pszexpanddir: Param1, dwflags: u32, pszfilelist: Param3, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractFilesW(pszcabname: ::windows::core::PCWSTR, pszexpanddir: ::windows::core::PCWSTR, dwflags: u32, pszfilelist: ::windows::core::PCWSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        ExtractFilesW(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), ::core::mem::transmute(dwflags), pszfilelist.into_param().abi(), ::core::mem::transmute(lpreserved), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_CHANGE_TIME(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(3i32);
impl ::core::marker::Copy for FEATURE_CHANGE_TIME {}
impl ::core::clone::Clone for FEATURE_CHANGE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_CHANGE_TIME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEATURE_CHANGE_TIME {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEATURE_CHANGE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_CHANGE_TIME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_ENABLED_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(2i32);
impl ::core::marker::Copy for FEATURE_ENABLED_STATE {}
impl ::core::clone::Clone for FEATURE_ENABLED_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FEATURE_ENABLED_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEATURE_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_ENABLED_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FEATURE_ERROR {
    pub hr: ::windows::core::HRESULT,
    pub lineNumber: u16,
    pub file: ::windows::core::PCSTR,
    pub process: ::windows::core::PCSTR,
    pub module: ::windows::core::PCSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: ::windows::core::PCSTR,
    pub message: ::windows::core::PCSTR,
    pub originLineNumber: u16,
    pub originFile: ::windows::core::PCSTR,
    pub originModule: ::windows::core::PCSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: ::windows::core::PCSTR,
    pub originName: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for FEATURE_ERROR {}
impl ::core::clone::Clone for FEATURE_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FEATURE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FEATURE_ERROR")
            .field("hr", &self.hr)
            .field("lineNumber", &self.lineNumber)
            .field("file", &self.file)
            .field("process", &self.process)
            .field("module", &self.module)
            .field("callerReturnAddressOffset", &self.callerReturnAddressOffset)
            .field("callerModule", &self.callerModule)
            .field("message", &self.message)
            .field("originLineNumber", &self.originLineNumber)
            .field("originFile", &self.originFile)
            .field("originModule", &self.originModule)
            .field("originCallerReturnAddressOffset", &self.originCallerReturnAddressOffset)
            .field("originCallerModule", &self.originCallerModule)
            .field("originName", &self.originName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FEATURE_ERROR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FEATURE_ERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FEATURE_ERROR>()) == 0 }
    }
}
impl ::core::cmp::Eq for FEATURE_ERROR {}
impl ::core::default::Default for FEATURE_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_STATE_CHANGE_SUBSCRIPTION(pub isize);
impl FEATURE_STATE_CHANGE_SUBSCRIPTION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FEATURE_STATE_CHANGE_SUBSCRIPTION {}
impl ::core::fmt::Debug for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_STATE_CHANGE_SUBSCRIPTION").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_SERVICE_PIPE_HANDLE(pub isize);
impl FH_SERVICE_PIPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FH_SERVICE_PIPE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FH_SERVICE_PIPE_HANDLE {}
impl ::core::fmt::Debug for FH_SERVICE_PIPE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_SERVICE_PIPE_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FH_SERVICE_PIPE_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_CASE_SENSITIVE_INFO {}
impl ::core::clone::Clone for FILE_CASE_SENSITIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_CASE_SENSITIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_CASE_SENSITIVE_INFO").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_CASE_SENSITIVE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_CASE_SENSITIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_CASE_SENSITIVE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_CASE_SENSITIVE_INFO {}
impl ::core::default::Default for FILE_CASE_SENSITIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_CREATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DELETE_ON_CLOSE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DIRECTORY_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFO_EX {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO_EX").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_DISPOSITION_INFO_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_DISPOSITION_INFO_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO_EX {}
impl ::core::default::Default for FILE_DISPOSITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_ENCRYPTABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_EXISTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for FILE_INFORMATION_CLASS {}
impl ::core::clone::Clone for FILE_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILE_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NON_DIRECTORY_FILE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPENED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OVERWRITTEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RANDOM_ACCESS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_READ_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RESERVE_OPFILTER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_ROOT_DIR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SEQUENTIAL_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SUPERSEDED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_DIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_TYPE_CHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_TYPE_DISK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_TYPE_PIPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_TYPE_REMOTE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_UNKNOWN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_USER_DISALLOWED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_WRITE_THROUGH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_CASE_SENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_FILE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn FileSaveMarkNotExistA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpfilelist: Param0, lpdir: Param1, lpbasename: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveMarkNotExistA(lpfilelist: ::windows::core::PCSTR, lpdir: ::windows::core::PCSTR, lpbasename: ::windows::core::PCSTR) -> ::windows::core::HRESULT;
        }
        FileSaveMarkNotExistA(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn FileSaveMarkNotExistW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfilelist: Param0, lpdir: Param1, lpbasename: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveMarkNotExistW(lpfilelist: ::windows::core::PCWSTR, lpdir: ::windows::core::PCWSTR, lpbasename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        FileSaveMarkNotExistW(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, psztitle: Param1, pszinf: Param2, pszsection: Param3, pszbackupdir: Param4, pszbasebackupfile: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: ::windows::core::PCSTR, pszinf: ::windows::core::PCSTR, pszsection: ::windows::core::PCSTR, pszbackupdir: ::windows::core::PCSTR, pszbasebackupfile: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        FileSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, psztitle: Param1, pszinf: Param2, pszsection: Param3, pszbackupdir: Param4, pszbasebackupfile: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, pszinf: ::windows::core::PCWSTR, pszsection: ::windows::core::PCWSTR, pszbackupdir: ::windows::core::PCWSTR, pszbasebackupfile: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        FileSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdlg: Param0, lpfilelist: Param1, lpdir: Param2, lpbasename: Param3, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreW(hdlg: super::super::Foundation::HWND, lpfilelist: ::windows::core::PCWSTR, lpdir: ::windows::core::PCWSTR, lpbasename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        FileSaveRestoreW(hdlg.into_param().abi(), lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FileTimeToDosDateTime(::core::mem::transmute(lpfiletime), ::core::mem::transmute(lpfatdate), ::core::mem::transmute(lpfattime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: &'static str = "GetSystemWow64DirectoryA";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: &'static str = "GetSystemWow64DirectoryA";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: &'static str = "GetSystemWow64DirectoryA";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: &'static str = "GetSystemWow64DirectoryW";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DDESHARE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DISCARDABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DISCARDED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_LOCKCOUNT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_LOWER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_MODIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOCOMPACT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NODISCARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOTIFY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOT_BANKED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_SHARE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GdiEntry13() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GdiEntry13() -> u32;
        }
        ::core::mem::transmute(GdiEntry13())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameA(lpbuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameA(lpbuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetComputerNameA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameW(lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameW(lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetComputerNameW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCurrentHwProfileA(::core::mem::transmute(lphwprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCurrentHwProfileW(::core::mem::transmute(lphwprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetDCRegionData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDCRegionData(hdc: super::super::Graphics::Gdi::HDC, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::core::mem::transmute(GetDCRegionData(hdc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE;
        }
        ::core::mem::transmute(GetFeatureEnabledState(::core::mem::transmute(featureid), ::core::mem::transmute(changetime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetFeatureVariant(::core::mem::transmute(featureid), ::core::mem::transmute(changetime), ::core::mem::transmute(payloadid), ::core::mem::transmute(hasnotification)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableA(lpname: ::windows::core::PCSTR, lpguid: ::windows::core::PCSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableExA(lpname: ::windows::core::PCSTR, lpguid: ::windows::core::PCSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(nsize), ::core::mem::transmute(pdwattribubutes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableExW(lpname: ::windows::core::PCWSTR, lpguid: ::windows::core::PCWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(nsize), ::core::mem::transmute(pdwattribubutes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableW(lpname: ::windows::core::PCWSTR, lpguid: ::windows::core::PCWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileIntA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileIntA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, ndefault: i32, lpfilename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::core::mem::transmute(ndefault), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileIntW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileIntW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, ndefault: i32, lpfilename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::core::mem::transmute(ndefault), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpreturnedstring: &mut [u8], lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionA(lpappname: ::windows::core::PCSTR, lpreturnedstring: ::windows::core::PSTR, nsize: u32, lpfilename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileSectionA(lpappname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszreturnbuffer: &mut [u8], lpfilename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionNamesA(lpszreturnbuffer: ::windows::core::PSTR, nsize: u32, lpfilename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileSectionNamesA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszreturnbuffer)), lpszreturnbuffer.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszreturnbuffer: &mut [u16], lpfilename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionNamesW(lpszreturnbuffer: ::windows::core::PWSTR, nsize: u32, lpfilename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileSectionNamesW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszreturnbuffer)), lpszreturnbuffer.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpreturnedstring: &mut [u16], lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionW(lpappname: ::windows::core::PCWSTR, lpreturnedstring: ::windows::core::PWSTR, nsize: u32, lpfilename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileSectionW(lpappname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: &mut [u8], lpfilename: Param5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStringA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, lpdefault: ::windows::core::PCSTR, lpreturnedstring: ::windows::core::PSTR, nsize: u32, lpfilename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: &mut [u16], lpfilename: Param5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStringW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, lpdefault: ::windows::core::PCWSTR, lpreturnedstring: ::windows::core::PWSTR, nsize: u32, lpfilename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetPrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _, lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStructA(lpszsection: ::windows::core::PCSTR, lpszkey: ::windows::core::PCSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct), ::core::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStructW(lpszsection: ::windows::core::PCWSTR, lpszkey: ::windows::core::PCWSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct), ::core::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileIntA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileIntA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, ndefault: i32) -> u32;
        }
        ::core::mem::transmute(GetProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::core::mem::transmute(ndefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileIntW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileIntW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, ndefault: i32) -> u32;
        }
        ::core::mem::transmute(GetProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::core::mem::transmute(ndefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileSectionA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpreturnedstring: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileSectionA(lpappname: ::windows::core::PCSTR, lpreturnedstring: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetProfileSectionA(lpappname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileSectionW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpreturnedstring: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileSectionW(lpappname: ::windows::core::PCWSTR, lpreturnedstring: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetProfileSectionW(lpappname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileStringA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, lpdefault: ::windows::core::PCSTR, lpreturnedstring: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileStringW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, lpdefault: ::windows::core::PCWSTR, lpreturnedstring: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpreturnedstring)), lpreturnedstring.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemRegistryQuota(::core::mem::transmute(pdwquotaallowed), ::core::mem::transmute(pdwquotaused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetThreadEnabledXStateFeatures() -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadEnabledXStateFeatures() -> u64;
        }
        ::core::mem::transmute(GetThreadEnabledXStateFeatures())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameA(lpbuffer: ::windows::core::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserNameA(lpbuffer: ::windows::core::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUserNameA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(pcbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameW(lpbuffer: ::windows::core::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserNameW(lpbuffer: ::windows::core::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUserNameW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(pcbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileA(lpszfilename: ::windows::core::PCSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetVersionFromFileA(lpszfilename.into_param().abi(), ::core::mem::transmute(pdwmsver), ::core::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileExA(lpszfilename: ::windows::core::PCSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetVersionFromFileExA(lpszfilename.into_param().abi(), ::core::mem::transmute(pdwmsver), ::core::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileExW(lpszfilename: ::windows::core::PCWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetVersionFromFileExW(lpszfilename.into_param().abi(), ::core::mem::transmute(pdwmsver), ::core::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileW(lpszfilename: ::windows::core::PCWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetVersionFromFileW(lpszfilename.into_param().abi(), ::core::mem::transmute(pdwmsver), ::core::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetWindowRegionData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowRegionData(hwnd: super::super::Foundation::HWND, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::core::mem::transmute(GetWindowRegionData(hwnd.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GlobalCompact(dwminfree: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalCompact(dwminfree: u32) -> usize;
        }
        ::core::mem::transmute(GlobalCompact(::core::mem::transmute(dwminfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GlobalFix(hmem: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFix(hmem: isize);
        }
        GlobalFix(::core::mem::transmute(hmem))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GlobalUnWire(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GlobalUnfix(hmem: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnfix(hmem: isize);
        }
        GlobalUnfix(::core::mem::transmute(hmem))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GlobalWire(hmem: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalWire(hmem: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GlobalWire(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HANJA_WINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HINSTANCE_ERROR: u32 = 32u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWINWATCH(pub isize);
impl HWINWATCH {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HWINWATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWINWATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWINWATCH {}
impl ::core::fmt::Debug for HWINWATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWINWATCH").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HWINWATCH {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [super::super::Foundation::CHAR; 39],
    pub szHwProfileName: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HW_PROFILE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HW_PROFILE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOA").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HW_PROFILE_INFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HW_PROFILE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HW_PROFILE_INFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOW {}
impl ::core::clone::Clone for HW_PROFILE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HW_PROFILE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOW").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
unsafe impl ::windows::core::Abi for HW_PROFILE_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HW_PROFILE_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOW {}
impl ::core::default::Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct ICameraUIControl(::windows::core::IUnknown);
impl ICameraUIControl {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::core::IntoParam<'a, ICameraUIControlEventCallback>>(&self, pwindow: Param0, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: Param6, peventcallback: Param7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Show)(::core::mem::transmute_copy(self), pwindow.into_param().abi(), ::core::mem::transmute(mode), ::core::mem::transmute(selectionmode), ::core::mem::transmute(capturemode), ::core::mem::transmute(photoformat), ::core::mem::transmute(videoformat), bhasclosebutton.into_param().abi(), peventcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Suspend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn GetCurrentViewType(&self) -> ::windows::core::Result<CameraUIControlViewType> {
        let mut result__: CameraUIControlViewType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentViewType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CameraUIControlViewType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetActiveItem(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetActiveItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedItems(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSelectedItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn RemoveCapturedItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveCapturedItem)(::core::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICameraUIControl> for ::windows::core::IUnknown {
    fn from(value: ICameraUIControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraUIControl> for ::windows::core::IUnknown {
    fn from(value: &ICameraUIControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICameraUIControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICameraUIControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICameraUIControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICameraUIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControl {}
impl ::core::fmt::Debug for ICameraUIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICameraUIControl {
    type Vtable = ICameraUIControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8733adf_3d68_4b8f_bb08_e28a0bed0376);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Suspend: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentViewType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetActiveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetActiveItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelectedItems: usize,
    pub RemoveCapturedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct ICameraUIControlEventCallback(::windows::core::IUnknown);
impl ICameraUIControlEventCallback {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn OnStartupComplete(&self) {
        (::windows::core::Interface::vtable(self).OnStartupComplete)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn OnSuspendComplete(&self) {
        (::windows::core::Interface::vtable(self).OnSuspendComplete)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn OnItemCaptured<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpath: Param0) {
        (::windows::core::Interface::vtable(self).OnItemCaptured)(::core::mem::transmute_copy(self), pszpath.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn OnItemDeleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpath: Param0) {
        (::windows::core::Interface::vtable(self).OnItemDeleted)(::core::mem::transmute_copy(self), pszpath.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn OnClosed(&self) {
        (::windows::core::Interface::vtable(self).OnClosed)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<ICameraUIControlEventCallback> for ::windows::core::IUnknown {
    fn from(value: ICameraUIControlEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraUIControlEventCallback> for ::windows::core::IUnknown {
    fn from(value: &ICameraUIControlEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICameraUIControlEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICameraUIControlEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICameraUIControlEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICameraUIControlEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControlEventCallback {}
impl ::core::fmt::Debug for ICameraUIControlEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControlEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICameraUIControlEventCallback {
    type Vtable = ICameraUIControlEventCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bfa0c2c_fbcd_4776_bda4_88bf974e74f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControlEventCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStartupComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnSuspendComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnItemCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR),
    pub OnItemDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR),
    pub OnClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IClipServiceNotificationHelper(::windows::core::IUnknown);
impl IClipServiceNotificationHelper {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowToast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, titletext: Param0, bodytext: Param1, packagename: Param2, appid: Param3, launchcommand: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowToast)(::core::mem::transmute_copy(self), titletext.into_param().abi(), bodytext.into_param().abi(), packagename.into_param().abi(), appid.into_param().abi(), launchcommand.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IClipServiceNotificationHelper> for ::windows::core::IUnknown {
    fn from(value: IClipServiceNotificationHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClipServiceNotificationHelper> for ::windows::core::IUnknown {
    fn from(value: &IClipServiceNotificationHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClipServiceNotificationHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClipServiceNotificationHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClipServiceNotificationHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClipServiceNotificationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClipServiceNotificationHelper {}
impl ::core::fmt::Debug for IClipServiceNotificationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClipServiceNotificationHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClipServiceNotificationHelper {
    type Vtable = IClipServiceNotificationHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc39948f0_6142_44fd_98ca_e1681a8d68b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipServiceNotificationHelper_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titletext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bodytext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, launchcommand: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowToast: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IContainerActivationHelper(::windows::core::IUnknown);
impl IContainerActivationHelper {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn CanActivateClientVM(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CanActivateClientVM)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IContainerActivationHelper> for ::windows::core::IUnknown {
    fn from(value: IContainerActivationHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContainerActivationHelper> for ::windows::core::IUnknown {
    fn from(value: &IContainerActivationHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContainerActivationHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContainerActivationHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContainerActivationHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContainerActivationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContainerActivationHelper {}
impl ::core::fmt::Debug for IContainerActivationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContainerActivationHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContainerActivationHelper {
    type Vtable = IContainerActivationHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb524f93f_80d5_4ec7_ae9e_d66e93ade1fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerActivationHelper_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CanActivateClientVM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isallowed: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IDefaultBrowserSyncSettings(::windows::core::IUnknown);
impl IDefaultBrowserSyncSettings {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsEnabled)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDefaultBrowserSyncSettings> for ::windows::core::IUnknown {
    fn from(value: IDefaultBrowserSyncSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDefaultBrowserSyncSettings> for ::windows::core::IUnknown {
    fn from(value: &IDefaultBrowserSyncSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDefaultBrowserSyncSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDefaultBrowserSyncSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDefaultBrowserSyncSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDefaultBrowserSyncSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultBrowserSyncSettings {}
impl ::core::fmt::Debug for IDefaultBrowserSyncSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultBrowserSyncSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDefaultBrowserSyncSettings {
    type Vtable = IDefaultBrowserSyncSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a27faad_5ae6_4255_9030_c530936292e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultBrowserSyncSettings_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IDeleteBrowsingHistory(::windows::core::IUnknown);
impl IDeleteBrowsingHistory {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn DeleteBrowsingHistory(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteBrowsingHistory)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IDeleteBrowsingHistory> for ::windows::core::IUnknown {
    fn from(value: IDeleteBrowsingHistory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeleteBrowsingHistory> for ::windows::core::IUnknown {
    fn from(value: &IDeleteBrowsingHistory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeleteBrowsingHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeleteBrowsingHistory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeleteBrowsingHistory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeleteBrowsingHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeleteBrowsingHistory {}
impl ::core::fmt::Debug for IDeleteBrowsingHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeleteBrowsingHistory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeleteBrowsingHistory {
    type Vtable = IDeleteBrowsingHistory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf38ed4b_2be7_4461_8b5e_9a466dc82ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteBrowsingHistory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub DeleteBrowsingHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_BACKNEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_FRDOALL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NODELETENEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOENUMKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOPROGRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_REGSECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_UPDREFCNT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_USEREFCNT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BADID: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BAUDRATE: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BYTESIZE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_DEFAULT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_HARDWARE: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_MEMORY: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_NOPEN: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_OPEN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IEditionUpgradeBroker(::windows::core::IUnknown);
impl IEditionUpgradeBroker {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn InitializeParentWindow(&self, parenthandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeParentWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(parenthandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOperatingSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parameter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOperatingSystem)(::core::mem::transmute_copy(self), parameter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowProductKeyUI)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn CanUpgrade(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CanUpgrade)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IEditionUpgradeBroker> for ::windows::core::IUnknown {
    fn from(value: IEditionUpgradeBroker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEditionUpgradeBroker> for ::windows::core::IUnknown {
    fn from(value: &IEditionUpgradeBroker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEditionUpgradeBroker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEditionUpgradeBroker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEditionUpgradeBroker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEditionUpgradeBroker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeBroker {}
impl ::core::fmt::Debug for IEditionUpgradeBroker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeBroker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEditionUpgradeBroker {
    type Vtable = IEditionUpgradeBroker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff19cbcf_9455_4937_b872_6b7929a460af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeBroker_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitializeParentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parenthandle: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOperatingSystem: usize,
    pub ShowProductKeyUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IEditionUpgradeHelper(::windows::core::IUnknown);
impl IEditionUpgradeHelper {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanUpgrade(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CanUpgrade)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn UpdateOperatingSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, contentid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOperatingSystem)(::core::mem::transmute_copy(self), contentid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowProductKeyUI)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
    pub unsafe fn GetOsProductContentId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOsProductContentId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGenuineLocalStatus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGenuineLocalStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IEditionUpgradeHelper> for ::windows::core::IUnknown {
    fn from(value: IEditionUpgradeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEditionUpgradeHelper> for ::windows::core::IUnknown {
    fn from(value: &IEditionUpgradeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEditionUpgradeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEditionUpgradeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEditionUpgradeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEditionUpgradeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeHelper {}
impl ::core::fmt::Debug for IEditionUpgradeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEditionUpgradeHelper {
    type Vtable = IEditionUpgradeHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e9e342_5deb_43b6_849e_6913b85d503a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeHelper_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CanUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanUpgrade: usize,
    pub UpdateOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ShowProductKeyUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOsProductContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGenuineLocalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGenuineLocalStatus: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_GENERIC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_MIB: u32 = 514u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IGNORE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_DELAYLOAD_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_DELAYLOAD_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_DELAYLOAD_DESCRIPTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DELAYLOAD_DESCRIPTOR_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_DELAYLOAD_DESCRIPTOR_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_THUNK_DATA32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_THUNK_DATA32>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA32 {}
impl ::core::default::Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_THUNK_DATA32_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA32_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_THUNK_DATA32_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA32_0 {}
impl ::core::default::Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_THUNK_DATA64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_THUNK_DATA64>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA64 {}
impl ::core::default::Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_THUNK_DATA64_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA64_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGE_THUNK_DATA64_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA64_0 {}
impl ::core::default::Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_INIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_PREV: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROA").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEPROA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEPROA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROW").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEPROW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEPROW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRUCT").field("fnc", &self.fnc).field("wParam", &self.wParam).field("wCount", &self.wCount).field("dchSource", &self.dchSource).field("dchDest", &self.dchDest).field("lParam1", &self.lParam1).field("lParam2", &self.lParam2).field("lParam3", &self.lParam3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMESTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_BANJAtoJUNJA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_ENABLE_CONVERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETIMECAPS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETOPEN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETVERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_JOHABtoKS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_JUNJAtoBANJA: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_KStoJOHAB: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MAXPROCESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_CODEINPUT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_HIRAGANA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_KATAKANA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_NOROMAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_ROMAN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_REQUEST_CONVERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_DISKERROR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_ILLEGAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_INVALID: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NEST: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOROOM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOTFOUND: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_TOOLONG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SENDVKEY: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETOPEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SET_MODE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPGetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPGetIMEA(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPGetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPGetIMEW(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPQueryIMEA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPQueryIMEW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPSetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPSetIMEA(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPSetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IMPSetIMEW(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFINITE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_GENERIC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INTERIM_WINDOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IO_STATUS_BLOCK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IO_STATUS_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_STATUS_BLOCK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IO_STATUS_BLOCK_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IO_STATUS_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_STATUS_BLOCK_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_CHANGECONVERT: u32 = 289u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_CLOSECONVERT: u32 = 290u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_DBCSCHAR: u32 = 352u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_FULLCONVERT: u32 = 291u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_IMESELECT: u32 = 304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_MODEINFO: u32 = 400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_OPENCONVERT: u32 = 288u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRING: u32 = 320u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGEND: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGEX: u32 = 384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGSTART: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_UNDETERMINE: u32 = 368u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IWindowsLockModeHelper(::windows::core::IUnknown);
impl IWindowsLockModeHelper {
    #[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWindowsLockModeHelper> for ::windows::core::IUnknown {
    fn from(value: IWindowsLockModeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsLockModeHelper> for ::windows::core::IUnknown {
    fn from(value: &IWindowsLockModeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsLockModeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsLockModeHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsLockModeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsLockModeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsLockModeHelper {}
impl ::core::fmt::Debug for IWindowsLockModeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsLockModeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsLockModeHelper {
    type Vtable = IWindowsLockModeHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf342d19e_cc22_4648_bb5d_03ccf75b47c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsLockModeHelper_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSMode: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsApiSetImplemented<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(contract: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsApiSetImplemented(contract: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsApiSetImplemented(contract.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadHugeReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadHugeReadPtr(::core::mem::transmute(lp), ::core::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadHugeWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadHugeWritePtr(::core::mem::transmute(lp), ::core::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsNTAdmin(::core::mem::transmute(dwreserved), ::core::mem::transmute(lpdwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsNativeVhdBoot(::core::mem::transmute(nativevhdboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTokenUntrusted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(tokenhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTokenUntrusted(tokenhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsTokenUntrusted(tokenhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: super::super::Foundation::BOOL,
    pub fAllPermissions: super::super::Foundation::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: ::windows::core::PCWSTR,
    pub guidZone: ::windows::core::GUID,
    pub hVerify: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JAVA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JAVA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JAVA_TRUST")
            .field("cbSize", &self.cbSize)
            .field("flag", &self.flag)
            .field("fAllActiveXPermissions", &self.fAllActiveXPermissions)
            .field("fAllPermissions", &self.fAllPermissions)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("pbJavaPermissions", &self.pbJavaPermissions)
            .field("cbJavaPermissions", &self.cbJavaPermissions)
            .field("pbSigner", &self.pbSigner)
            .field("cbSigner", &self.cbSigner)
            .field("pwszZone", &self.pwszZone)
            .field("guidZone", &self.guidZone)
            .field("hVerify", &self.hVerify)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JAVA_TRUST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JAVA_TRUST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JAVA_TRUST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
impl ::core::marker::Copy for JIT_DEBUG_INFO {}
impl ::core::clone::Clone for JIT_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JIT_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JIT_DEBUG_INFO").field("dwSize", &self.dwSize).field("dwProcessorArchitecture", &self.dwProcessorArchitecture).field("dwThreadID", &self.dwThreadID).field("dwReserved0", &self.dwReserved0).field("lpExceptionAddress", &self.lpExceptionAddress).field("lpExceptionRecord", &self.lpExceptionRecord).field("lpContextRecord", &self.lpContextRecord).finish()
    }
}
unsafe impl ::windows::core::Abi for JIT_DEBUG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JIT_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JIT_DEBUG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for JIT_DEBUG_INFO {}
impl ::core::default::Default for JIT_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
impl ::core::marker::Copy for KEY_SET_INFORMATION_CLASS {}
impl ::core::clone::Clone for KEY_SET_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEY_SET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KEY_SET_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_VALUE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_VALUE_ENTRY").field("ValueName", &self.ValueName).field("DataLength", &self.DataLength).field("DataOffset", &self.DataOffset).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KEY_VALUE_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KEY_VALUE_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub DllBase: *mut ::core::ffi::c_void,
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut ::core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for LDR_DATA_TABLE_ENTRY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for LDR_DATA_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDR_DATA_TABLE_ENTRY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for LDR_DATA_TABLE_ENTRY_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for LDR_DATA_TABLE_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDR_DATA_TABLE_ENTRY_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for LDR_DATA_TABLE_ENTRY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LIS_NOGRPCONV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LIS_QUIET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LPTx: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LaunchINFSectionExW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PCWSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        LaunchINFSectionExW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hwndowner: Param0, hinstance: Param1, pszparams: ::windows::core::PWSTR, nshow: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LaunchINFSectionW(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparams: ::windows::core::PWSTR, nshow: i32) -> i32;
        }
        ::core::mem::transmute(LaunchINFSectionW(hwndowner.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(pszparams), ::core::mem::transmute(nshow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn LocalCompact(uminfree: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalCompact(uminfree: u32) -> usize;
        }
        ::core::mem::transmute(LocalCompact(::core::mem::transmute(uminfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize;
        }
        ::core::mem::transmute(LocalShrink(::core::mem::transmute(hmem), ::core::mem::transmute(cbnewsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAXINTATOM: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_HIDDEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_RECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_SCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_VERTICAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_WINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MODE_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32;
        }
        ::core::mem::transmute(MulDiv(::core::mem::transmute(nnumber), ::core::mem::transmute(nnumerator), ::core::mem::transmute(ndenominator)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NeedReboot(::core::mem::transmute(dwrebootcheck)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn NeedRebootInit() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedRebootInit() -> u32;
        }
        ::core::mem::transmute(NeedRebootInit())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtClose(handle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NtClose(handle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtDeviceIoControlFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, event: Param1, apcroutine: PIO_APC_ROUTINE, apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtDeviceIoControlFile(filehandle: super::super::Foundation::HANDLE, event: super::super::Foundation::HANDLE, apcroutine: ::windows::core::RawPtr, apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtDeviceIoControlFile(filehandle.into_param().abi(), event.into_param().abi(), ::core::mem::transmute(apcroutine), ::core::mem::transmute(apccontext), ::core::mem::transmute(iostatusblock), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(inputbufferlength), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(outputbufferlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param11: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(masterkeyhandle: Param0, subordinateobjects: &[OBJECT_ATTRIBUTES], event: Param3, apcroutine: PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, completionfilter: u32, watchtree: Param8, buffer: *mut ::core::ffi::c_void, buffersize: u32, asynchronous: Param11) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtNotifyChangeMultipleKeys(masterkeyhandle: super::super::Foundation::HANDLE, count: u32, subordinateobjects: *const OBJECT_ATTRIBUTES, event: super::super::Foundation::HANDLE, apcroutine: ::windows::core::RawPtr, apccontext: *const ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, completionfilter: u32, watchtree: super::super::Foundation::BOOLEAN, buffer: *mut ::core::ffi::c_void, buffersize: u32, asynchronous: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        NtNotifyChangeMultipleKeys(masterkeyhandle.into_param().abi(), subordinateobjects.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(subordinateobjects)), event.into_param().abi(), ::core::mem::transmute(apcroutine), ::core::mem::transmute(apccontext), ::core::mem::transmute(iostatusblock), ::core::mem::transmute(completionfilter), watchtree.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), asynchronous.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtOpenFile(::core::mem::transmute(filehandle), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(objectattributes), ::core::mem::transmute(iostatusblock), ::core::mem::transmute(shareaccess), ::core::mem::transmute(openoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryMultipleValueKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryMultipleValueKey(keyhandle: super::super::Foundation::HANDLE, valueentries: *mut KEY_VALUE_ENTRY, entrycount: u32, valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryMultipleValueKey(keyhandle.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(valueentries)), valueentries.len() as _, ::core::mem::transmute(valuebuffer), ::core::mem::transmute(bufferlength), ::core::mem::transmute(requiredbufferlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::core::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryObject(handle: super::super::Foundation::HANDLE, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::core::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryObject(handle.into_param().abi(), ::core::mem::transmute(objectinformationclass), ::core::mem::transmute(objectinformation), ::core::mem::transmute(objectinformationlength), ::core::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQuerySystemInformation(::core::mem::transmute(systeminformationclass), ::core::mem::transmute(systeminformation), ::core::mem::transmute(systeminformationlength), ::core::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemTime(systemtime: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQuerySystemTime(systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        NtQuerySystemTime(::core::mem::transmute(systemtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryTimerResolution(::core::mem::transmute(maximumtime), ::core::mem::transmute(minimumtime), ::core::mem::transmute(currenttime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtRenameKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, newname: *const super::super::Foundation::UNICODE_STRING) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtRenameKey(keyhandle: super::super::Foundation::HANDLE, newname: *const super::super::Foundation::UNICODE_STRING) -> super::super::Foundation::NTSTATUS;
        }
        NtRenameKey(keyhandle.into_param().abi(), ::core::mem::transmute(newname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtSetInformationKey(keyhandle: super::super::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtSetInformationKey(keyhandle.into_param().abi(), ::core::mem::transmute(keysetinformationclass), ::core::mem::transmute(keysetinformation), ::core::mem::transmute(keysetinformationlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtWaitForSingleObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(handle: Param0, alertable: Param1, timeout: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtWaitForSingleObject(handle: super::super::Foundation::HANDLE, alertable: super::super::Foundation::BOOLEAN, timeout: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        NtWaitForSingleObject(handle.into_param().abi(), alertable.into_param().abi(), ::core::mem::transmute(timeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub ObjectName: *mut super::super::Foundation::UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub SecurityQualityOfService: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OBJECT_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECT_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OBJECT_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for OBJECT_INFORMATION_CLASS {}
impl ::core::clone::Clone for OBJECT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OBJECT_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for OBJECT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OFS_MAXPATHNAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OPERATION_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OVERWRITE_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn OpenINFEngineA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszinffilename: Param0, pszinstallsection: Param1, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenINFEngineA(pszinffilename: ::windows::core::PCSTR, pszinstallsection: ::windows::core::PCSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        OpenINFEngineA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(phinf), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn OpenINFEngineW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszinffilename: Param0, pszinstallsection: Param1, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenINFEngineW(pszinffilename: ::windows::core::PCWSTR, pszinstallsection: ::windows::core::PCWSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        OpenINFEngineW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(phinf), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenMutexA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenMutexA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenMutexA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenSemaphoreA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSemaphoreA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenSemaphoreA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWaitableTimerA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWaitableTimerA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenWaitableTimerA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut ::core::ffi::c_void>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONA {
    pub szGUID: [super::super::Foundation::CHAR; 59],
    pub szDispName: [super::super::Foundation::CHAR; 128],
    pub szLocale: [super::super::Foundation::CHAR; 10],
    pub szStub: [super::super::Foundation::CHAR; 1040],
    pub szVersion: [super::super::Foundation::CHAR; 32],
    pub szCompID: [super::super::Foundation::CHAR; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONA").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERUSERSECTIONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERUSERSECTIONA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONW").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERUSERSECTIONW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERUSERSECTIONW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PFEATURE_STATE_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PFIBER_CALLOUT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpparameter: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIO_APC_ROUTINE = ::core::option::Option<unsafe extern "system" fn(apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PQUERYACTCTXW_FUNC = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_CANCEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_CONTINUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_QUIET: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION").field("Attributes", &self.Attributes).field("GrantedAccess", &self.GrantedAccess).field("HandleCount", &self.HandleCount).field("PointerCount", &self.PointerCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PUBLIC_OBJECT_BASIC_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PUBLIC_OBJECT_BASIC_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::default::Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION").field("TypeName", &self.TypeName).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PUBLIC_OBJECT_TYPE_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PUBLIC_OBJECT_TYPE_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINSTATIONQUERYINFORMATIONW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut ::core::ffi::c_void, param4: u32, param5: *mut u32) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = ::core::option::Option<unsafe extern "system" fn(packagefamilyname: ::windows::core::PCWSTR, packageversion: u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = ::core::option::Option<unsafe extern "system" fn(pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = ::core::option::Option<unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDYNAMICODETRUST_API = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = ::core::option::Option<unsafe extern "system" fn(setting: ::windows::core::PCWSTR, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = ::core::option::Option<unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = ::core::option::Option<unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_SETDYNAMICCODETRUST_API = ::core::option::Option<unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency() -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        QueryAuxiliaryCounterFrequency(::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryIdleProcessorCycleTime(::core::mem::transmute(bufferlength), ::core::mem::transmute(processoridlecycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryIdleProcessorCycleTimeEx(::core::mem::transmute(group), ::core::mem::transmute(bufferlength), ::core::mem::transmute(processoridlecycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryInterruptTime(lpinterrupttime: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInterruptTime(lpinterrupttime: *mut u64);
        }
        QueryInterruptTime(::core::mem::transmute(lpinterrupttime))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64);
        }
        QueryInterruptTimePrecise(::core::mem::transmute(lpinterrupttimeprecise))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProcessCycleTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, cycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProcessCycleTime(processhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryProcessCycleTime(processhandle.into_param().abi(), ::core::mem::transmute(cycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadCycleTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, cycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadCycleTime(threadhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryThreadCycleTime(threadhandle.into_param().abi(), ::core::mem::transmute(cycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryUnbiasedInterruptTime(::core::mem::transmute(unbiasedtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64);
        }
        QueryUnbiasedInterruptTimePrecise(::core::mem::transmute(lpunbiasedinterrupttimeprecise))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type REGINSTALLA = ::core::option::Option<unsafe extern "system" fn(hm: super::super::Foundation::HINSTANCE, pszsection: ::windows::core::PCSTR, psttable: *mut STRTABLEA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REG_RESTORE_LOG_KEY: &'static str = "RegRestoreLogFile";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REG_SAVE_LOG_KEY: &'static str = "RegSaveLogFile";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RESETDEV: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_INF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_NGCONV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_QUIET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_NOWAIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_QUIET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
        }
        ::core::mem::transmute(RaiseCustomSystemEventTrigger(::core::mem::transmute(customsystemeventtriggerconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, pszinf: Param1, pszsec: Param2, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RebootCheckOnInstallA(hwnd: super::super::Foundation::HWND, pszinf: ::windows::core::PCSTR, pszsec: ::windows::core::PCSTR, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        RebootCheckOnInstallA(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, pszinf: Param1, pszsec: Param2, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RebootCheckOnInstallW(hwnd: super::super::Foundation::HWND, pszinf: ::windows::core::PCWSTR, pszsec: ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        RebootCheckOnInstallW(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR);
        }
        RecordFeatureError(::core::mem::transmute(featureid), ::core::mem::transmute(error))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RecordFeatureUsage<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(featureid: u32, kind: u32, addend: u32, originname: Param3) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecordFeatureUsage(featureid: u32, kind: u32, addend: u32, originname: ::windows::core::PCSTR);
        }
        RecordFeatureUsage(::core::mem::transmute(featureid), ::core::mem::transmute(kind), ::core::mem::transmute(addend), originname.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hmod: Param0, pszsection: Param1, psttable: *const STRTABLEA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegInstallA(hmod: super::super::Foundation::HINSTANCE, pszsection: ::windows::core::PCSTR, psttable: *const STRTABLEA) -> ::windows::core::HRESULT;
        }
        RegInstallA(hmod.into_param().abi(), pszsection.into_param().abi(), ::core::mem::transmute(psttable)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hmod: Param0, pszsection: Param1, psttable: *const STRTABLEW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegInstallW(hmod: super::super::Foundation::HINSTANCE, pszsection: ::windows::core::PCWSTR, psttable: *const STRTABLEW) -> ::windows::core::HRESULT;
        }
        RegInstallW(hmod.into_param().abi(), pszsection.into_param().abi(), ::core::mem::transmute(psttable)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreAllA(hwnd: super::super::Foundation::HWND, psztitlestring: ::windows::core::PCSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::core::HRESULT;
        }
        RegRestoreAllA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreAllW(hwnd: super::super::Foundation::HWND, psztitlestring: ::windows::core::PCWSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::core::HRESULT;
        }
        RegRestoreAllW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::Registry::HKEY>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2, pcszrootkey: Param3, pcszsubkey: Param4, pcszvaluename: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreA(hwnd: super::super::Foundation::HWND, psztitlestring: ::windows::core::PCSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: ::windows::core::PCSTR, pcszsubkey: ::windows::core::PCSTR, pcszvaluename: ::windows::core::PCSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        RegSaveRestoreA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, super::Registry::HKEY>, Param5: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitle: Param1, pszinf: Param2, pszsection: Param3, hhklmbackkey: Param4, hhkcubackkey: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: ::windows::core::PCSTR, pszinf: ::windows::core::PCSTR, pszsection: ::windows::core::PCSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::core::HRESULT;
        }
        RegSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::Registry::HKEY>, Param5: ::windows::core::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitle: Param1, pszinf: Param2, pszsection: Param3, hhklmbackkey: Param4, hhkcubackkey: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, pszinf: ::windows::core::PCWSTR, pszsection: ::windows::core::PCWSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::core::HRESULT;
        }
        RegSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::Registry::HKEY>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2, pcszrootkey: Param3, pcszsubkey: Param4, pcszvaluename: Param5, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreW(hwnd: super::super::Foundation::HWND, psztitlestring: ::windows::core::PCWSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: ::windows::core::PCWSTR, pcszsubkey: ::windows::core::PCWSTR, pcszvaluename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        RegSaveRestoreW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplacePartitionUnit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(targetpartition: Param0, sparepartition: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplacePartitionUnit(targetpartition: ::windows::core::PCWSTR, sparepartition: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReplacePartitionUnit(targetpartition.into_param().abi(), sparepartition.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RequestDeviceWakeup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RequestDeviceWakeup(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RequestDeviceWakeup(hdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlAnsiStringToUnicodeString<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlAnsiStringToUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlAnsiStringToUnicodeString(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        RtlCharToInteger(::core::mem::transmute(string), ::core::mem::transmute(base), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING);
        }
        RtlFreeAnsiString(::core::mem::transmute(ansistring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING);
        }
        RtlFreeOemString(::core::mem::transmute(oemstring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING);
        }
        RtlFreeUnicodeString(::core::mem::transmute(unicodestring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlGetReturnAddressHijackTarget() -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlGetReturnAddressHijackTarget() -> usize;
        }
        ::core::mem::transmute(RtlGetReturnAddressHijackTarget())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
        }
        RtlInitAnsiString(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
        }
        RtlInitAnsiStringEx(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
        }
        RtlInitString(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
        }
        RtlInitStringEx(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInitUnicodeString<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: ::windows::core::PCWSTR);
        }
        RtlInitUnicodeString(::core::mem::transmute(destinationstring), sourcestring.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(RtlIsNameLegalDOS8Dot3(::core::mem::transmute(name), ::core::mem::transmute(oemname), ::core::mem::transmute(namecontainsspaces)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        RtlLocalTimeToSystemTime(::core::mem::transmute(localtime), ::core::mem::transmute(systemtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
        }
        ::core::mem::transmute(RtlRaiseCustomSystemEventTrigger(::core::mem::transmute(triggerconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(RtlTimeToSecondsSince1970(::core::mem::transmute(time), ::core::mem::transmute(elapsedseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToAnsiString<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeStringToAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeStringToAnsiString(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToOemString<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeStringToOemString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeStringToOemString(::core::mem::transmute(destinationstring), ::core::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteSize<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(bytesinmultibytestring: *mut u32, unicodestring: Param1, bytesinunicodestring: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeToMultiByteSize(bytesinmultibytestring: *mut u32, unicodestring: ::windows::core::PCWSTR, bytesinunicodestring: u32) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeToMultiByteSize(::core::mem::transmute(bytesinmultibytestring), unicodestring.into_param().abi(), ::core::mem::transmute(bytesinunicodestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlUniform(seed: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUniform(seed: *mut u32) -> u32;
        }
        ::core::mem::transmute(RtlUniform(::core::mem::transmute(seed)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, szcmdname: Param1, szinfsection: Param2, szdir: Param3, lpsztitle: Param4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RunSetupCommandA(hwnd: super::super::Foundation::HWND, szcmdname: ::windows::core::PCSTR, szinfsection: ::windows::core::PCSTR, szdir: ::windows::core::PCSTR, lpsztitle: ::windows::core::PCSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        RunSetupCommandA(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), ::core::mem::transmute(phexe), ::core::mem::transmute(dwflags), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, szcmdname: Param1, szinfsection: Param2, szdir: Param3, lpsztitle: Param4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RunSetupCommandW(hwnd: super::super::Foundation::HWND, szcmdname: ::windows::core::PCWSTR, szinfsection: ::windows::core::PCWSTR, szdir: ::windows::core::PCWSTR, lpsztitle: ::windows::core::PCWSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        RunSetupCommandW(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), ::core::mem::transmute(phexe), ::core::mem::transmute(dwflags), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_32BIT_BINARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_64BIT_BINARY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_DOS_BINARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_OS216_BINARY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_PIF_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_POSIX_BINARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_WOW_BINARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SHUTDOWN_NORETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRENTRYA {
    pub pszName: ::windows::core::PSTR,
    pub pszValue: ::windows::core::PSTR,
}
impl ::core::marker::Copy for STRENTRYA {}
impl ::core::clone::Clone for STRENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYA").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
unsafe impl ::windows::core::Abi for STRENTRYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRENTRYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRENTRYA {}
impl ::core::default::Default for STRENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRENTRYW {
    pub pszName: ::windows::core::PWSTR,
    pub pszValue: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for STRENTRYW {}
impl ::core::clone::Clone for STRENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYW").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
unsafe impl ::windows::core::Abi for STRENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRENTRYW {}
impl ::core::default::Default for STRENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for STRINGEXSTRUCT {}
impl ::core::clone::Clone for STRINGEXSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRINGEXSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRINGEXSTRUCT").field("dwSize", &self.dwSize).field("uDeterminePos", &self.uDeterminePos).field("uDetermineDelimPos", &self.uDetermineDelimPos).field("uYomiPos", &self.uYomiPos).field("uYomiDelimPos", &self.uYomiDelimPos).finish()
    }
}
unsafe impl ::windows::core::Abi for STRINGEXSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRINGEXSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRINGEXSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRINGEXSTRUCT {}
impl ::core::default::Default for STRINGEXSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
impl ::core::marker::Copy for STRTABLEA {}
impl ::core::clone::Clone for STRTABLEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEA").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
unsafe impl ::windows::core::Abi for STRTABLEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRTABLEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRTABLEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRTABLEA {}
impl ::core::default::Default for STRTABLEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
impl ::core::marker::Copy for STRTABLEW {}
impl ::core::clone::Clone for STRTABLEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEW").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
unsafe impl ::windows::core::Abi for STRTABLEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRTABLEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRTABLEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRTABLEW {}
impl ::core::default::Default for STRTABLEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut ::core::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl ::core::marker::Copy for SYSTEM_BASIC_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("NumberOfProcessors", &self.NumberOfProcessors).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_BASIC_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_BASIC_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_BASIC_INFORMATION {}
impl ::core::default::Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
impl ::core::marker::Copy for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CODEINTEGRITY_INFORMATION").field("Length", &self.Length).field("CodeIntegrityOptions", &self.CodeIntegrityOptions).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_CODEINTEGRITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_CODEINTEGRITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::default::Default for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl ::core::marker::Copy for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EXCEPTION_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_EXCEPTION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_EXCEPTION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::default::Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(23i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(33i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(37i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(45i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(103i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(134i32);
impl ::core::marker::Copy for SYSTEM_INFORMATION_CLASS {}
impl ::core::clone::Clone for SYSTEM_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl ::core::marker::Copy for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_INTERRUPT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INTERRUPT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INTERRUPT_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_INTERRUPT_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_INTERRUPT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_INTERRUPT_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::default::Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl ::core::marker::Copy for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOOKASIDE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOOKASIDE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOOKASIDE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_LOOKASIDE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_LOOKASIDE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_LOOKASIDE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::default::Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl ::core::marker::Copy for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PERFORMANCE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_PERFORMANCE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_PERFORMANCE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::default::Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl ::core::marker::Copy for SYSTEM_POLICY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POLICY_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_POLICY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_POLICY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_POLICY_INFORMATION {}
impl ::core::default::Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION").field("IdleTime", &self.IdleTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::default::Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut ::core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("Reserved1", &self.Reserved1)
            .field("ImageName", &self.ImageName)
            .field("BasePriority", &self.BasePriority)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved2", &self.Reserved2)
            .field("HandleCount", &self.HandleCount)
            .field("SessionId", &self.SessionId)
            .field("Reserved3", &self.Reserved3)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("Reserved4", &self.Reserved4)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("Reserved5", &self.Reserved5)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("Reserved6", &self.Reserved6)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivatePageCount", &self.PrivatePageCount)
            .field("Reserved7", &self.Reserved7)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_PROCESS_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_PROCESS_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_REGISTRY_QUOTA_INFORMATION").field("RegistryQuotaAllowed", &self.RegistryQuotaAllowed).field("RegistryQuotaUsed", &self.RegistryQuotaUsed).field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_REGISTRY_QUOTA_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::default::Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut ::core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_THREAD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_THREAD_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("StartAddress", &self.StartAddress).field("ClientId", &self.ClientId).field("Priority", &self.Priority).field("BasePriority", &self.BasePriority).field("Reserved3", &self.Reserved3).field("ThreadState", &self.ThreadState).field("WaitReason", &self.WaitReason).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_THREAD_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_THREAD_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl ::core::marker::Copy for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_TIMEOFDAY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_TIMEOFDAY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_TIMEOFDAY_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_TIMEOFDAY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_TIMEOFDAY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_TIMEOFDAY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::default::Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_ALLTHRESHOLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_LEGATO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD1024: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD2048: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD512: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIODVOICE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_QUEUEEMPTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERBDNT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDCC: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDDR: i32 = -14i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDFQ: i32 = -13i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDLN: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDMD: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDPT: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDSH: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDSR: i32 = -15i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDST: i32 = -16i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDTP: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDVL: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDVNA: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERMACT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SEROFM: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERQFUL: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_STACCATO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE1024: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE2048: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE512: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITEVOICE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendIMEMessageExA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(SendIMEMessageExA(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendIMEMessageExW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(SendIMEMessageExW(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentStringsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(newenvironment: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnvironmentStringsA(newenvironment: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetEnvironmentStringsA(newenvironment.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableA(lpname: ::windows::core::PCSTR, lpguid: ::windows::core::PCSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableExA(lpname: ::windows::core::PCSTR, lpguid: ::windows::core::PCSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue), ::core::mem::transmute(nsize), ::core::mem::transmute(dwattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableExW(lpname: ::windows::core::PCWSTR, lpguid: ::windows::core::PCWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue), ::core::mem::transmute(nsize), ::core::mem::transmute(dwattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableW(lpname: ::windows::core::PCWSTR, lpguid: ::windows::core::PCWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn SetHandleCount(unumber: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetHandleCount(unumber: u32) -> u32;
        }
        ::core::mem::transmute(SetHandleCount(::core::mem::transmute(unumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMessageWaitingIndicator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmsgindicator: Param0, ulmsgcount: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMessageWaitingIndicator(hmsgindicator: super::super::Foundation::HANDLE, ulmsgcount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetMessageWaitingIndicator(hmsgindicator.into_param().abi(), ::core::mem::transmute(ulmsgcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::core::HRESULT;
        }
        SetPerUserSecValuesA(::core::mem::transmute(pperuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::core::HRESULT;
        }
        SetPerUserSecValuesW(::core::mem::transmute(pperuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SignalObjectAndWait<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hobjecttosignal: Param0, hobjecttowaiton: Param1, dwmilliseconds: u32, balertable: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SignalObjectAndWait(hobjecttosignal: super::super::Foundation::HANDLE, hobjecttowaiton: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(SignalObjectAndWait(hobjecttosignal.into_param().abi(), hobjecttowaiton.into_param().abi(), ::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: PFEATURE_STATE_CHANGE_CALLBACK, context: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void);
        }
        SubscribeFeatureStateChangeNotification(::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_GP_TRAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_HARDERR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_SIGNAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TDIENTITY_ENTITY_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(640u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(769u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(768u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1025u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1024u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(896u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(512u32);
impl ::core::marker::Copy for TDIENTITY_ENTITY_TYPE {}
impl ::core::clone::Clone for TDIENTITY_ENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDIENTITY_ENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TDIENTITY_ENTITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TDIENTITY_ENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDIENTITY_ENTITY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
impl ::core::marker::Copy for TDIEntityID {}
impl ::core::clone::Clone for TDIEntityID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIEntityID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIEntityID").field("tei_entity", &self.tei_entity).field("tei_instance", &self.tei_instance).finish()
    }
}
unsafe impl ::windows::core::Abi for TDIEntityID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TDIEntityID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TDIEntityID>()) == 0 }
    }
}
impl ::core::cmp::Eq for TDIEntityID {}
impl ::core::default::Default for TDIEntityID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
impl ::core::marker::Copy for TDIObjectID {}
impl ::core::clone::Clone for TDIObjectID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIObjectID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIObjectID").field("toi_entity", &self.toi_entity).field("toi_class", &self.toi_class).field("toi_type", &self.toi_type).field("toi_id", &self.toi_id).finish()
    }
}
unsafe impl ::windows::core::Abi for TDIObjectID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TDIObjectID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TDIObjectID>()) == 0 }
    }
}
impl ::core::cmp::Eq for TDIObjectID {}
impl ::core::default::Default for TDIObjectID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut ::core::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut ::core::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TDI_TL_IO_CONTROL_ENDPOINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TDI_TL_IO_CONTROL_ENDPOINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::core::default::Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TDI_TL_IO_CONTROL_ENDPOINT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::core::default::Default for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TDI_TL_IO_CONTROL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(3i32);
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_TYPE {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TDI_TL_IO_CONTROL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TDI_TL_IO_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDI_TL_IO_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for THREAD_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for THREAD_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_NAME_INFORMATION").field("ThreadName", &self.ThreadName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for THREAD_NAME_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for THREAD_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<THREAD_NAME_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for THREAD_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszinffilename: Param0, pszinstallsection: Param1, psztranslatesection: Param2, psztranslatekey: Param3, pszbuffer: &mut [u8], pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringA(pszinffilename: ::windows::core::PCSTR, pszinstallsection: ::windows::core::PCSTR, psztranslatesection: ::windows::core::PCSTR, psztranslatekey: ::windows::core::PCSTR, pszbuffer: ::windows::core::PSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        TranslateInfStringA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, ::core::mem::transmute(pdwrequiredsize), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hinf: *mut ::core::ffi::c_void, pszinffilename: Param1, psztranslatesection: Param2, psztranslatekey: Param3, pszbuffer: &mut [u8], pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringExA(hinf: *mut ::core::ffi::c_void, pszinffilename: ::windows::core::PCSTR, psztranslatesection: ::windows::core::PCSTR, psztranslatekey: ::windows::core::PCSTR, pszbuffer: ::windows::core::PSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        TranslateInfStringExA(::core::mem::transmute(hinf), pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, ::core::mem::transmute(pdwrequiredsize), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hinf: *mut ::core::ffi::c_void, pszinffilename: Param1, psztranslatesection: Param2, psztranslatekey: Param3, pszbuffer: &mut [u16], pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringExW(hinf: *mut ::core::ffi::c_void, pszinffilename: ::windows::core::PCWSTR, psztranslatesection: ::windows::core::PCWSTR, psztranslatekey: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        TranslateInfStringExW(::core::mem::transmute(hinf), pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, ::core::mem::transmute(pdwrequiredsize), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszinffilename: Param0, pszinstallsection: Param1, psztranslatesection: Param2, psztranslatekey: Param3, pszbuffer: &mut [u16], pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringW(pszinffilename: ::windows::core::PCWSTR, pszinstallsection: ::windows::core::PCWSTR, psztranslatesection: ::windows::core::PCWSTR, psztranslatekey: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        TranslateInfStringW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, ::core::mem::transmute(pdwrequiredsize), ::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const UMS_VERSION: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for UNDETERMINESTRUCT {}
impl ::core::clone::Clone for UNDETERMINESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNDETERMINESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNDETERMINESTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDefIMESize", &self.uDefIMESize)
            .field("uDefIMEPos", &self.uDefIMEPos)
            .field("uUndetTextLen", &self.uUndetTextLen)
            .field("uUndetTextPos", &self.uUndetTextPos)
            .field("uUndetAttrPos", &self.uUndetAttrPos)
            .field("uCursorPos", &self.uCursorPos)
            .field("uDeltaStart", &self.uDeltaStart)
            .field("uDetermineTextLen", &self.uDetermineTextLen)
            .field("uDetermineTextPos", &self.uDetermineTextPos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiTextLen", &self.uYomiTextLen)
            .field("uYomiTextPos", &self.uYomiTextPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for UNDETERMINESTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNDETERMINESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNDETERMINESTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNDETERMINESTRUCT {}
impl ::core::default::Default for UNDETERMINESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn UnsubscribeFeatureStateChangeNotification<'a, Param0: ::windows::core::IntoParam<'a, FEATURE_STATE_CHANGE_SUBSCRIPTION>>(subscription: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnsubscribeFeatureStateChangeNotification(subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION);
        }
        UnsubscribeFeatureStateChangeNotification(subscription.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PCSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        UserInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PCWSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        UserInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserUnInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PCSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        UserUnInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserUnInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: ::windows::core::PCWSTR, nshow: i32) -> ::windows::core::HRESULT;
        }
        UserUnInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::core::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VALUENAME(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_UNKNOWN: VALUENAME = VALUENAME(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = VALUENAME(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = VALUENAME(2i32);
impl ::core::marker::Copy for VALUENAME {}
impl ::core::clone::Clone for VALUENAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VALUENAME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VALUENAME {
    type Abi = Self;
}
impl ::core::fmt::Debug for VALUENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALUENAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_DOS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_GUID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_NONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_NT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSEnableIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSEnableIME(param0: super::super::Foundation::HWND, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WINNLSEnableIME(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetEnableStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSGetEnableStatus(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WINNLSGetEnableStatus(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetIMEHotkey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSGetIMEHotkey(param0: super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(WINNLSGetIMEHotkey(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSTATIONINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WinStationInformation: WINSTATIONINFOCLASS = WINSTATIONINFOCLASS(8i32);
impl ::core::marker::Copy for WINSTATIONINFOCLASS {}
impl ::core::clone::Clone for WINSTATIONINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSTATIONINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINSTATIONINFOCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSTATIONINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSTATIONINFOCLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl ::core::marker::Copy for WINSTATIONINFORMATIONW {}
impl ::core::clone::Clone for WINSTATIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINSTATIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINSTATIONINFORMATIONW").field("Reserved2", &self.Reserved2).field("LogonId", &self.LogonId).field("Reserved3", &self.Reserved3).finish()
    }
}
unsafe impl ::windows::core::Abi for WINSTATIONINFORMATIONW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINSTATIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINSTATIONINFORMATIONW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINSTATIONINFORMATIONW {}
impl ::core::default::Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINWATCHNOTIFYPROC = ::core::option::Option<unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM)>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::clone::Clone for WLDP_DEVICE_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLDP_DEVICE_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_DEVICE_SECURITY_INFORMATION").field("UnlockIdSize", &self.UnlockIdSize).field("UnlockId", &self.UnlockId).field("ManufacturerIDLength", &self.ManufacturerIDLength).field("ManufacturerID", &self.ManufacturerID).finish()
    }
}
unsafe impl ::windows::core::Abi for WLDP_DEVICE_SECURITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLDP_DEVICE_SECURITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLDP_DEVICE_SECURITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::default::Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_DLL: &'static str = "WLDP.DLL";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_GETLOCKDOWNPOLICY_FN: &'static str = "WldpGetLockdownPolicy";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_HOST(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = WLDP_HOST(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_SVCHOST: WLDP_HOST = WLDP_HOST(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_MAX: WLDP_HOST = WLDP_HOST(2i32);
impl ::core::marker::Copy for WLDP_HOST {}
impl ::core::clone::Clone for WLDP_HOST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_HOST {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_HOST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_HOST_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = WLDP_HOST_ID(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = WLDP_HOST_ID(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = WLDP_HOST_ID(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = WLDP_HOST_ID(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = WLDP_HOST_ID(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = WLDP_HOST_ID(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = WLDP_HOST_ID(6i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = WLDP_HOST_ID(7i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = WLDP_HOST_ID(8i32);
impl ::core::marker::Copy for WLDP_HOST_ID {}
impl ::core::clone::Clone for WLDP_HOST_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_HOST_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_HOST_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: ::windows::core::PCWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLDP_HOST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLDP_HOST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_HOST_INFORMATION").field("dwRevision", &self.dwRevision).field("dwHostId", &self.dwHostId).field("szSource", &self.szSource).field("hSource", &self.hSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLDP_HOST_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLDP_HOST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLDP_HOST_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLDP_HOST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISAPPAPPROVEDBYPOLICY_FN: &'static str = "WldpIsAppApprovedByPolicy";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISCLASSINAPPROVEDLIST_FN: &'static str = "WldpIsClassInApprovedList";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISDYNAMICCODEPOLICYENABLED_FN: &'static str = "WldpIsDynamicCodePolicyEnabled";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISPRODUCTIONCONFIGURATION_FN: &'static str = "WldpIsProductionConfiguration";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN: &'static str = "WldpIsWcosProductionConfiguration";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_KEY(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_UNKNOWN: WLDP_KEY = WLDP_KEY(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_OVERRIDE: WLDP_KEY = WLDP_KEY(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_ALL_KEYS: WLDP_KEY = WLDP_KEY(2i32);
impl ::core::marker::Copy for WLDP_KEY {}
impl ::core::clone::Clone for WLDP_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_KEY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_KEY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_KEY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_POLICY_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = WLDP_POLICY_SETTING(1000i32);
impl ::core::marker::Copy for WLDP_POLICY_SETTING {}
impl ::core::clone::Clone for WLDP_POLICY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_POLICY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_POLICY_SETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_POLICY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_POLICY_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDANAMICCODETRUST_FN: &'static str = "WldpQueryDynamicCodeTrust";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDEVICESECURITYINFORMATION_FN: &'static str = "WldpQueryDeviceSecurityInformation";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDYNAMICCODETRUST_FN: &'static str = "WldpQueryDynamicCodeTrust";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYPOLICYSETTINGENABLED2_FN: &'static str = "WldpQueryPolicySettingEnabled2";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYPOLICYSETTINGENABLED_FN: &'static str = "WldpQueryPolicySettingEnabled";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYWINDOWSLOCKDOWNMODE_FN: &'static str = "WldpQueryWindowsLockdownMode";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN: &'static str = "WldpQueryWindowsLockdownRestriction";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_RESETPRODUCTIONCONFIGURATION_FN: &'static str = "WldpResetProductionConfiguration";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN: &'static str = "WldpResetWcosProductionConfiguration";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_SETDYNAMICCODETRUST_FN: &'static str = "WldpSetDynamicCodeTrust";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN: &'static str = "WldpSetWindowsLockdownRestriction";
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_MODE {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_WINDOWS_LOCKDOWN_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_RESTRICTION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_RESTRICTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_CONVERTREQUEST: u32 = 266u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_CONVERTRESULT: u32 = 267u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IMEKEYDOWN: u32 = 656u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IMEKEYUP: u32 = 657u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IME_REPORT: u32 = 640u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_INTERIM: u32 = 268u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn WinWatchClose<'a, Param0: ::windows::core::IntoParam<'a, HWINWATCH>>(hww: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchClose(hww: HWINWATCH);
        }
        WinWatchClose(hww.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchDidStatusChange<'a, Param0: ::windows::core::IntoParam<'a, HWINWATCH>>(hww: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchDidStatusChange(hww: HWINWATCH) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WinWatchDidStatusChange(hww.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn WinWatchGetClipList<'a, Param0: ::windows::core::IntoParam<'a, HWINWATCH>>(hww: Param0, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchGetClipList(hww: HWINWATCH, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::core::mem::transmute(WinWatchGetClipList(hww.into_param().abi(), ::core::mem::transmute(prc), ::core::mem::transmute(size), ::core::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchNotify<'a, Param0: ::windows::core::IntoParam<'a, HWINWATCH>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hww: Param0, notifycallback: WINWATCHNOTIFYPROC, notifyparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchNotify(hww: HWINWATCH, notifycallback: ::windows::core::RawPtr, notifyparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WinWatchNotify(hww.into_param().abi(), ::core::mem::transmute(notifycallback), notifyparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchOpen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> HWINWATCH {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchOpen(hwnd: super::super::Foundation::HWND) -> HWINWATCH;
        }
        ::core::mem::transmute(WinWatchOpen(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::core::HRESULT;
        }
        WldpGetLockdownPolicy(::core::mem::transmute(hostinformation), ::core::mem::transmute(lockdownstate), ::core::mem::transmute(lockdownflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsClassInApprovedList(classid: *const ::windows::core::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpIsClassInApprovedList(classid: *const ::windows::core::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::core::HRESULT;
        }
        WldpIsClassInApprovedList(::core::mem::transmute(classid), ::core::mem::transmute(hostinformation), ::core::mem::transmute(isapproved), ::core::mem::transmute(optionalflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsDynamicCodePolicyEnabled() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpIsDynamicCodePolicyEnabled(isenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        WldpIsDynamicCodePolicyEnabled(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn WldpQueryDeviceSecurityInformation(information: &mut [WLDP_DEVICE_SECURITY_INFORMATION], returnlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpQueryDeviceSecurityInformation(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::core::HRESULT;
        }
        WldpQueryDeviceSecurityInformation(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(information)), information.len() as _, ::core::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpQueryDynamicCodeTrust<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpQueryDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows::core::HRESULT;
        }
        WldpQueryDynamicCodeTrust(filehandle.into_param().abi(), ::core::mem::transmute(baseimage), ::core::mem::transmute(imagesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpSetDynamicCodeTrust<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpSetDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        WldpSetDynamicCodeTrust(filehandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpstring: Param1, lpfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileSectionA(lpappname: ::windows::core::PCSTR, lpstring: ::windows::core::PCSTR, lpfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpstring: Param1, lpfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileSectionW(lpappname: ::windows::core::PCWSTR, lpstring: ::windows::core::PCWSTR, lpfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2, lpfilename: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStringA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, lpstring: ::windows::core::PCSTR, lpfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2, lpfilename: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStringW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, lpstring: ::windows::core::PCWSTR, lpfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStructA(lpszsection: ::windows::core::PCSTR, lpszkey: ::windows::core::PCSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct), ::core::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStructW(lpszsection: ::windows::core::PCWSTR, lpszkey: ::windows::core::PCWSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WritePrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct), ::core::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpstring: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileSectionA(lpappname: ::windows::core::PCSTR, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WriteProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpstring: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileSectionW(lpappname: ::windows::core::PCWSTR, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WriteProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileStringA(lpappname: ::windows::core::PCSTR, lpkeyname: ::windows::core::PCSTR, lpstring: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WriteProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileStringW(lpappname: ::windows::core::PCWSTR, lpkeyname: ::windows::core::PCWSTR, lpstring: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WriteProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct _D3DHAL_CALLBACKS(pub u8);
#[repr(C)]
pub struct _D3DHAL_GLOBALDRIVERDATA(pub u8);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _hread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, lbytes: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _hread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, lbytes: i32) -> i32;
        }
        ::core::mem::transmute(_hread(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _hwrite<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfile: i32, lpbuffer: Param1, lbytes: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _hwrite(hfile: i32, lpbuffer: ::windows::core::PCSTR, lbytes: i32) -> i32;
        }
        ::core::mem::transmute(_hwrite(::core::mem::transmute(hfile), lpbuffer.into_param().abi(), ::core::mem::transmute(lbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lclose(hfile: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lclose(hfile: i32) -> i32;
        }
        ::core::mem::transmute(_lclose(::core::mem::transmute(hfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lcreat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lppathname: Param0, iattribute: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lcreat(lppathname: ::windows::core::PCSTR, iattribute: i32) -> i32;
        }
        ::core::mem::transmute(_lcreat(lppathname.into_param().abi(), ::core::mem::transmute(iattribute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
        }
        ::core::mem::transmute(_llseek(::core::mem::transmute(hfile), ::core::mem::transmute(loffset), ::core::mem::transmute(iorigin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lopen<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lppathname: Param0, ireadwrite: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lopen(lppathname: ::windows::core::PCSTR, ireadwrite: i32) -> i32;
        }
        ::core::mem::transmute(_lopen(lppathname.into_param().abi(), ::core::mem::transmute(ireadwrite)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, ubytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, ubytes: u32) -> u32;
        }
        ::core::mem::transmute(_lread(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(ubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lwrite<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfile: i32, lpbuffer: Param1, ubytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lwrite(hfile: i32, lpbuffer: ::windows::core::PCSTR, ubytes: u32) -> u32;
        }
        ::core::mem::transmute(_lwrite(::core::mem::transmute(hfile), lpbuffer.into_param().abi(), ::core::mem::transmute(ubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct tcp_request_query_information_ex32_xp {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for tcp_request_query_information_ex32_xp {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for tcp_request_query_information_ex32_xp {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for tcp_request_query_information_ex32_xp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex32_xp").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for tcp_request_query_information_ex32_xp {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for tcp_request_query_information_ex32_xp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_request_query_information_ex32_xp>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for tcp_request_query_information_ex32_xp {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for tcp_request_query_information_ex32_xp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct tcp_request_query_information_ex_w2k {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_w2k {}
impl ::core::clone::Clone for tcp_request_query_information_ex_w2k {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_query_information_ex_w2k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex_w2k").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_request_query_information_ex_w2k {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_request_query_information_ex_w2k {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_request_query_information_ex_w2k>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_request_query_information_ex_w2k {}
impl ::core::default::Default for tcp_request_query_information_ex_w2k {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct tcp_request_query_information_ex_xp {
    pub ID: TDIObjectID,
    pub Context: [usize; 2],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_xp {}
impl ::core::clone::Clone for tcp_request_query_information_ex_xp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_query_information_ex_xp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex_xp").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_request_query_information_ex_xp {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_request_query_information_ex_xp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_request_query_information_ex_xp>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_request_query_information_ex_xp {}
impl ::core::default::Default for tcp_request_query_information_ex_xp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct tcp_request_set_information_ex {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for tcp_request_set_information_ex {}
impl ::core::clone::Clone for tcp_request_set_information_ex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_set_information_ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_set_information_ex").field("ID", &self.ID).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_request_set_information_ex {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_request_set_information_ex {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_request_set_information_ex>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_request_set_information_ex {}
impl ::core::default::Default for tcp_request_set_information_ex {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32;
        }
        ::core::mem::transmute(uaw_lstrcmpW(::core::mem::transmute(string1), ::core::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32;
        }
        ::core::mem::transmute(uaw_lstrcmpiW(::core::mem::transmute(string1), ::core::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrlenW(string: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrlenW(string: *const u16) -> i32;
        }
        ::core::mem::transmute(uaw_lstrlenW(::core::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16;
        }
        ::core::mem::transmute(uaw_wcschr(::core::mem::transmute(string), ::core::mem::transmute(character)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16;
        }
        ::core::mem::transmute(uaw_wcscpy(::core::mem::transmute(destination), ::core::mem::transmute(source)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32;
        }
        ::core::mem::transmute(uaw_wcsicmp(::core::mem::transmute(string1), ::core::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcslen(string: *const u16) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcslen(string: *const u16) -> usize;
        }
        ::core::mem::transmute(uaw_wcslen(::core::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16;
        }
        ::core::mem::transmute(uaw_wcsrchr(::core::mem::transmute(string), ::core::mem::transmute(character)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
