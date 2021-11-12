#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const DBG_ATTACH: u32 = 14u32;
pub const DBG_BREAK: u32 = 6u32;
pub const DBG_DIVOVERFLOW: u32 = 8u32;
pub const DBG_DLLSTART: u32 = 12u32;
pub const DBG_DLLSTOP: u32 = 13u32;
pub const DBG_GPFAULT: u32 = 7u32;
pub const DBG_GPFAULT2: u32 = 21u32;
pub const DBG_INIT: u32 = 20u32;
pub const DBG_INSTRFAULT: u32 = 9u32;
pub const DBG_MODFREE: u32 = 4u32;
pub const DBG_MODLOAD: u32 = 3u32;
pub const DBG_MODMOVE: u32 = 19u32;
pub const DBG_SEGFREE: u32 = 2u32;
pub const DBG_SEGLOAD: u32 = 0u32;
pub const DBG_SEGMOVE: u32 = 1u32;
pub const DBG_SINGLESTEP: u32 = 5u32;
pub const DBG_STACKFAULT: u32 = 16u32;
pub const DBG_TASKSTART: u32 = 10u32;
pub const DBG_TASKSTOP: u32 = 11u32;
pub const DBG_TEMPBP: u32 = 18u32;
pub const DBG_TOOLHELP: u32 = 15u32;
pub const DBG_WOWINIT: u32 = 17u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type DEBUGEVENTPROC = unsafe extern "system" fn(param0: *mut super::Diagnostics::Debug::DEBUG_EVENT, param1: *mut ::core::ffi::c_void) -> u32;
pub const GD_ACCELERATORS: u32 = 9u32;
pub const GD_BITMAP: u32 = 2u32;
pub const GD_CURSOR: u32 = 12u32;
pub const GD_CURSORCOMPONENT: u32 = 1u32;
pub const GD_DIALOG: u32 = 5u32;
pub const GD_ERRTABLE: u32 = 11u32;
pub const GD_FONT: u32 = 8u32;
pub const GD_FONTDIR: u32 = 7u32;
pub const GD_ICON: u32 = 14u32;
pub const GD_ICONCOMPONENT: u32 = 3u32;
pub const GD_MAX_RESOURCE: u32 = 15u32;
pub const GD_MENU: u32 = 4u32;
pub const GD_NAMETABLE: u32 = 15u32;
pub const GD_RCDATA: u32 = 10u32;
pub const GD_STRING: u32 = 6u32;
pub const GD_USERDEFINED: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GLOBALENTRY(i32);
pub const GLOBAL_ALL: u32 = 0u32;
pub const GLOBAL_FREE: u32 = 2u32;
pub const GLOBAL_LRU: u32 = 1u32;
pub const GT_BURGERMASTER: u32 = 10u32;
pub const GT_CODE: u32 = 3u32;
pub const GT_DATA: u32 = 2u32;
pub const GT_DGROUP: u32 = 1u32;
pub const GT_FREE: u32 = 7u32;
pub const GT_INTERNAL: u32 = 8u32;
pub const GT_MODULE: u32 = 6u32;
pub const GT_RESOURCE: u32 = 5u32;
pub const GT_SENTINEL: u32 = 9u32;
pub const GT_TASK: u32 = 4u32;
pub const GT_UNKNOWN: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGE_NOTE(i32);
pub const MAX_MODULE_NAME: u32 = 9u32;
pub const MAX_PATH16: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MODULEENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub type PROCESSENUMPROC = unsafe extern "system" fn(dwprocessid: u32, dwattributes: u32, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SEGMENT_NOTE(i32);
pub const SN_CODE: u32 = 0u32;
pub const SN_DATA: u32 = 1u32;
pub const SN_V86: u32 = 2u32;
pub const STATUS_VDM_EVENT: i32 = 1073741829i32;
#[cfg(feature = "Win32_Foundation")]
pub type TASKENUMPROC = unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type TASKENUMPROCEX = unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, pszmodname: *mut i8, pszfilename: *mut i8, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TEMP_BP_NOTE(i32);
pub const V86FLAGS_ALIGNMENT: u32 = 262144u32;
pub const V86FLAGS_AUXCARRY: u32 = 16u32;
pub const V86FLAGS_CARRY: u32 = 1u32;
pub const V86FLAGS_DIRECTION: u32 = 1024u32;
pub const V86FLAGS_INTERRUPT: u32 = 512u32;
pub const V86FLAGS_IOPL: u32 = 12288u32;
pub const V86FLAGS_IOPL_BITS: u32 = 18u32;
pub const V86FLAGS_OVERFLOW: u32 = 2048u32;
pub const V86FLAGS_PARITY: u32 = 4u32;
pub const V86FLAGS_RESUME: u32 = 65536u32;
pub const V86FLAGS_SIGN: u32 = 128u32;
pub const V86FLAGS_TRACE: u32 = 256u32;
pub const V86FLAGS_V86: u32 = 131072u32;
pub const V86FLAGS_ZERO: u32 = 64u32;
pub const VDMADDR_PM16: u32 = 4u32;
pub const VDMADDR_PM32: u32 = 16u32;
pub const VDMADDR_V86: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMBREAKTHREADPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct VDMCONTEXT(i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct VDMCONTEXT_WITHOUT_XSAVE(i32);
pub const VDMCONTEXT_i386: u32 = 65536u32;
pub const VDMCONTEXT_i486: u32 = 65536u32;
pub const VDMDBG_BREAK_DEBUGGER: u32 = 16u32;
pub const VDMDBG_BREAK_DIVIDEBYZERO: u32 = 256u32;
pub const VDMDBG_BREAK_DOSTASK: u32 = 1u32;
pub const VDMDBG_BREAK_EXCEPTIONS: u32 = 8u32;
pub const VDMDBG_BREAK_LOADDLL: u32 = 4u32;
pub const VDMDBG_BREAK_WOWTASK: u32 = 2u32;
pub const VDMDBG_INITIAL_FLAGS: u32 = 256u32;
pub const VDMDBG_MAX_SYMBOL_BUFFER: u32 = 256u32;
pub const VDMDBG_TRACE_HISTORY: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMDETECTWOWPROC = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMPROCESSWOWPROC = unsafe extern "system" fn(param0: PROCESSENUMPROC, param1: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMTASKWOWEXPROC = unsafe extern "system" fn(param0: u32, param1: TASKENUMPROCEX, param2: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMTASKWOWPROC = unsafe extern "system" fn(param0: u32, param1: TASKENUMPROC, param2: super::super::Foundation::LPARAM) -> i32;
pub const VDMEVENT_ALLFLAGS: u32 = 57344u32;
pub const VDMEVENT_NEEDS_INTERACTIVE: u32 = 32768u32;
pub const VDMEVENT_PE: u32 = 8192u32;
pub const VDMEVENT_PM16: u32 = 2u32;
pub const VDMEVENT_V86: u32 = 1u32;
pub const VDMEVENT_VERBOSE: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETADDREXPRESSIONPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut u16, param3: *mut u32, param4: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type VDMGETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type VDMGETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETDBGFLAGSPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETMODULESELECTORPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: super::super::Foundation::PSTR, param4: *mut u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETPOINTERPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: u32, param4: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSEGMENTINFOPROC = unsafe extern "system" fn(param0: u16, param1: u32, param2: super::super::Foundation::BOOL, param3: VDM_SEGINFO) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSELECTORMODULEPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: *mut u32, param4: super::super::Foundation::PSTR, param5: u32, param6: super::super::Foundation::PSTR, param7: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSYMBOLPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: u16, param2: u32, param3: super::super::Foundation::BOOL, param4: super::super::Foundation::BOOL, param5: super::super::Foundation::PSTR, param6: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETTHREADSELECTORENTRYPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut VDMLDT_ENTRY) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type VDMGETTHREADSELECTORENTRYPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut super::Diagnostics::Debug::LDT_ENTRY) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMGLOBALFIRSTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: DEBUGEVENTPROC, param5: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMGLOBALNEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: DEBUGEVENTPROC, param5: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMISMODULELOADEDPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMKILLWOWPROC = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct VDMLDT_ENTRY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMMODULEFIRSTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: DEBUGEVENTPROC, param4: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMMODULENEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: DEBUGEVENTPROC, param4: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMPROCESSEXCEPTIONPROC = unsafe extern "system" fn(param0: *mut super::Diagnostics::Debug::DEBUG_EVENT) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type VDMSETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type VDMSETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMSETDBGFLAGSPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMSTARTTASKINWOWPROC = unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::PSTR, param2: u16) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type VDMTERMINATETASKINWOWPROC = unsafe extern "system" fn(param0: u32, param1: u16) -> super::super::Foundation::BOOL;
pub const VDM_KGDT_R3_CODE: u32 = 24u32;
pub const VDM_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VDM_SEGINFO(i32);
