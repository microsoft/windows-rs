#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_ATTACH: u32 = 14u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_BREAK: u32 = 6u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_DIVOVERFLOW: u32 = 8u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_DLLSTART: u32 = 12u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_DLLSTOP: u32 = 13u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_GPFAULT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_GPFAULT2: u32 = 21u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_INIT: u32 = 20u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_INSTRFAULT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_MODFREE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_MODLOAD: u32 = 3u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_MODMOVE: u32 = 19u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_SEGFREE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_SEGLOAD: u32 = 0u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_SEGMOVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_SINGLESTEP: u32 = 5u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_STACKFAULT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_TASKSTART: u32 = 10u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_TASKSTOP: u32 = 11u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_TEMPBP: u32 = 18u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_TOOLHELP: u32 = 15u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const DBG_WOWINIT: u32 = 17u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type DEBUGEVENTPROC = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<super::Diagnostics::Debug::DEBUG_EVENT>, param1: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_ACCELERATORS: u32 = 9u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_CURSOR: u32 = 12u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_CURSORCOMPONENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_DIALOG: u32 = 5u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_ERRTABLE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_FONT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_FONTDIR: u32 = 7u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_ICON: u32 = 14u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_ICONCOMPONENT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_MAX_RESOURCE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_MENU: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_NAMETABLE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_RCDATA: u32 = 10u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_STRING: u32 = 6u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GD_USERDEFINED: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct GLOBALENTRY {
    pub dwSize: u32,
    pub dwAddress: u32,
    pub dwBlockSize: u32,
    pub hBlock: super::super::Foundation::HANDLE,
    pub wcLock: u16,
    pub wcPageLock: u16,
    pub wFlags: u16,
    pub wHeapPresent: super::super::Foundation::BOOL,
    pub hOwner: super::super::Foundation::HANDLE,
    pub wType: u16,
    pub wData: u16,
    pub dwNext: u32,
    pub dwNextAlt: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GLOBALENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLOBALENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLOBALENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLOBALENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLOBALENTRY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GLOBAL_ALL: u32 = 0u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GLOBAL_FREE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GLOBAL_LRU: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_BURGERMASTER: u32 = 10u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_CODE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_DGROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_FREE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_INTERNAL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_MODULE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_RESOURCE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_SENTINEL: u32 = 9u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_TASK: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const GT_UNKNOWN: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct IMAGE_NOTE {
    pub Module: [super::super::Foundation::CHAR; 10],
    pub FileName: [super::super::Foundation::CHAR; 256],
    pub hModule: u16,
    pub hTask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl IMAGE_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IMAGE_NOTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IMAGE_NOTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMAGE_NOTE").field("Module", &self.Module).field("FileName", &self.FileName).field("hModule", &self.hModule).field("hTask", &self.hTask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IMAGE_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.FileName == other.FileName && self.hModule == other.hModule && self.hTask == other.hTask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IMAGE_NOTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IMAGE_NOTE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const MAX_MODULE_NAME: u32 = 9u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const MAX_PATH16: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct MODULEENTRY {
    pub dwSize: u32,
    pub szModule: [super::super::Foundation::CHAR; 10],
    pub hModule: super::super::Foundation::HANDLE,
    pub wcUsage: u16,
    pub szExePath: [super::super::Foundation::CHAR; 256],
    pub wNext: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl MODULEENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODULEENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODULEENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODULEENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODULEENTRY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PROCESSENUMPROC = unsafe extern "system" fn(dwprocessid: u32, dwattributes: u32, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct SEGMENT_NOTE {
    pub Selector1: u16,
    pub Selector2: u16,
    pub Segment: u16,
    pub Module: [super::super::Foundation::CHAR; 10],
    pub FileName: [super::super::Foundation::CHAR; 256],
    pub Type: u16,
    pub Length: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SEGMENT_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SEGMENT_NOTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SEGMENT_NOTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEGMENT_NOTE").field("Selector1", &self.Selector1).field("Selector2", &self.Selector2).field("Segment", &self.Segment).field("Module", &self.Module).field("FileName", &self.FileName).field("Type", &self.Type).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SEGMENT_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Selector1 == other.Selector1 && self.Selector2 == other.Selector2 && self.Segment == other.Segment && self.Module == other.Module && self.FileName == other.FileName && self.Type == other.Type && self.Length == other.Length
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SEGMENT_NOTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SEGMENT_NOTE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const SN_CODE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const SN_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const SN_V86: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const STATUS_VDM_EVENT: i32 = 1073741829i32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TASKENUMPROC = unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TASKENUMPROCEX = unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, pszmodname: *mut i8, pszfilename: *mut i8, lpuserdefined: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct TEMP_BP_NOTE {
    pub Seg: u16,
    pub Offset: u32,
    pub bPM: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl TEMP_BP_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TEMP_BP_NOTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TEMP_BP_NOTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TEMP_BP_NOTE").field("Seg", &self.Seg).field("Offset", &self.Offset).field("bPM", &self.bPM).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TEMP_BP_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Seg == other.Seg && self.Offset == other.Offset && self.bPM == other.bPM
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TEMP_BP_NOTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TEMP_BP_NOTE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_ALIGNMENT: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_AUXCARRY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_CARRY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_DIRECTION: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_INTERRUPT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_IOPL: u32 = 12288u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_IOPL_BITS: u32 = 18u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_OVERFLOW: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_PARITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_RESUME: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_SIGN: u32 = 128u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_TRACE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_V86: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const V86FLAGS_ZERO: u32 = 64u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMADDR_PM16: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMADDR_PM32: u32 = 16u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMADDR_V86: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMBREAKTHREADPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_System_Kernel`*"]
pub struct VDMCONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: super::Kernel::FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
impl VDMCONTEXT {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::default::Default for VDMCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::fmt::Debug for VDMCONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDMCONTEXT")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .field("ExtendedRegisters", &self.ExtendedRegisters)
            .finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::PartialEq for VDMCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags
            && self.Dr0 == other.Dr0
            && self.Dr1 == other.Dr1
            && self.Dr2 == other.Dr2
            && self.Dr3 == other.Dr3
            && self.Dr6 == other.Dr6
            && self.Dr7 == other.Dr7
            && self.FloatSave == other.FloatSave
            && self.SegGs == other.SegGs
            && self.SegFs == other.SegFs
            && self.SegEs == other.SegEs
            && self.SegDs == other.SegDs
            && self.Edi == other.Edi
            && self.Esi == other.Esi
            && self.Ebx == other.Ebx
            && self.Edx == other.Edx
            && self.Ecx == other.Ecx
            && self.Eax == other.Eax
            && self.Ebp == other.Ebp
            && self.Eip == other.Eip
            && self.SegCs == other.SegCs
            && self.EFlags == other.EFlags
            && self.Esp == other.Esp
            && self.SegSs == other.SegSs
            && self.ExtendedRegisters == other.ExtendedRegisters
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::Eq for VDMCONTEXT {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::runtime::Abi for VDMCONTEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_System_Kernel`*"]
pub struct VDMCONTEXT_WITHOUT_XSAVE {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: super::Kernel::FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl VDMCONTEXT_WITHOUT_XSAVE {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::default::Default for VDMCONTEXT_WITHOUT_XSAVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::fmt::Debug for VDMCONTEXT_WITHOUT_XSAVE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDMCONTEXT_WITHOUT_XSAVE")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::PartialEq for VDMCONTEXT_WITHOUT_XSAVE {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags
            && self.Dr0 == other.Dr0
            && self.Dr1 == other.Dr1
            && self.Dr2 == other.Dr2
            && self.Dr3 == other.Dr3
            && self.Dr6 == other.Dr6
            && self.Dr7 == other.Dr7
            && self.FloatSave == other.FloatSave
            && self.SegGs == other.SegGs
            && self.SegFs == other.SegFs
            && self.SegEs == other.SegEs
            && self.SegDs == other.SegDs
            && self.Edi == other.Edi
            && self.Esi == other.Esi
            && self.Ebx == other.Ebx
            && self.Edx == other.Edx
            && self.Ecx == other.Ecx
            && self.Eax == other.Eax
            && self.Ebp == other.Ebp
            && self.Eip == other.Eip
            && self.SegCs == other.SegCs
            && self.EFlags == other.EFlags
            && self.Esp == other.Esp
            && self.SegSs == other.SegSs
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::Eq for VDMCONTEXT_WITHOUT_XSAVE {}
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::runtime::Abi for VDMCONTEXT_WITHOUT_XSAVE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMCONTEXT_i386: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMCONTEXT_i486: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_DEBUGGER: u32 = 16u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_DIVIDEBYZERO: u32 = 256u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_DOSTASK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_EXCEPTIONS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_LOADDLL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_BREAK_WOWTASK: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_INITIAL_FLAGS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_MAX_SYMBOL_BUFFER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMDBG_TRACE_HISTORY: u32 = 128u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMDETECTWOWPROC = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMPROCESSWOWPROC = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: super::super::Foundation::LPARAM) -> i32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMTASKWOWEXPROC = unsafe extern "system" fn(param0: u32, param1: ::windows::runtime::RawPtr, param2: super::super::Foundation::LPARAM) -> i32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMENUMTASKWOWPROC = unsafe extern "system" fn(param0: u32, param1: ::windows::runtime::RawPtr, param2: super::super::Foundation::LPARAM) -> i32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_ALLFLAGS: u32 = 57344u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_NEEDS_INTERACTIVE: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_PE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_PM16: u32 = 2u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_V86: u32 = 1u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDMEVENT_VERBOSE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETADDREXPRESSIONPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut u16, param3: *mut u32, param4: *mut u16) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Kernel`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type VDMGETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMGETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETDBGFLAGSPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETMODULESELECTORPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: super::super::Foundation::PSTR, param4: *mut u16) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETPOINTERPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: u32, param4: super::super::Foundation::BOOL) -> u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSEGMENTINFOPROC = unsafe extern "system" fn(param0: u16, param1: u32, param2: super::super::Foundation::BOOL, param3: VDM_SEGINFO) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSELECTORMODULEPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: *mut u32, param4: super::super::Foundation::PSTR, param5: u32, param6: super::super::Foundation::PSTR, param7: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETSYMBOLPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: u16, param2: u32, param3: super::super::Foundation::BOOL, param4: super::super::Foundation::BOOL, param5: super::super::Foundation::PSTR, param6: *mut u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub type VDMGETTHREADSELECTORENTRYPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut VDMLDT_ENTRY) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type VDMGETTHREADSELECTORENTRYPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut super::Diagnostics::Debug::LDT_ENTRY) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMGLOBALFIRSTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: ::windows::runtime::RawPtr, param5: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMGLOBALNEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: ::windows::runtime::RawPtr, param5: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMISMODULELOADEDPROC = unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMKILLWOWPROC = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub struct VDMLDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: VDMLDT_ENTRY_0,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl VDMLDT_ENTRY {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::default::Default for VDMLDT_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::PartialEq for VDMLDT_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::Eq for VDMLDT_ENTRY {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for VDMLDT_ENTRY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub union VDMLDT_ENTRY_0 {
    pub Bytes: VDMLDT_ENTRY_0_1,
    pub Bits: VDMLDT_ENTRY_0_0,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl VDMLDT_ENTRY_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::default::Default for VDMLDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::PartialEq for VDMLDT_ENTRY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::Eq for VDMLDT_ENTRY_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for VDMLDT_ENTRY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct VDMLDT_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl VDMLDT_ENTRY_0_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::default::Default for VDMLDT_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::fmt::Debug for VDMLDT_ENTRY_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Bits_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::PartialEq for VDMLDT_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::Eq for VDMLDT_ENTRY_0_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for VDMLDT_ENTRY_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct VDMLDT_ENTRY_0_1 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl VDMLDT_ENTRY_0_1 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::default::Default for VDMLDT_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::fmt::Debug for VDMLDT_ENTRY_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Bytes_e__Struct").field("BaseMid", &self.BaseMid).field("Flags1", &self.Flags1).field("Flags2", &self.Flags2).field("BaseHi", &self.BaseHi).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::PartialEq for VDMLDT_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseMid == other.BaseMid && self.Flags1 == other.Flags1 && self.Flags2 == other.Flags2 && self.BaseHi == other.BaseHi
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::std::cmp::Eq for VDMLDT_ENTRY_0_1 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::runtime::Abi for VDMLDT_ENTRY_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMMODULEFIRSTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: ::windows::runtime::RawPtr, param4: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMMODULENEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: ::windows::runtime::RawPtr, param4: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMPROCESSEXCEPTIONPROC = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<super::Diagnostics::Debug::DEBUG_EVENT>) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Kernel`*"]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type VDMSETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_SystemServices`*"]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
pub type VDMSETCONTEXTPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMSETDBGFLAGSPROC = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMSTARTTASKINWOWPROC = unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::PSTR, param2: u16) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type VDMTERMINATETASKINWOWPROC = unsafe extern "system" fn(param0: u32, param1: u16) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDM_KGDT_R3_CODE: u32 = 24u32;
#[doc = "*Required features: `Win32_System_VirtualDosMachines`*"]
pub const VDM_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_VirtualDosMachines`, `Win32_Foundation`*"]
pub struct VDM_SEGINFO {
    pub Selector: u16,
    pub SegNumber: u16,
    pub Length: u32,
    pub Type: u16,
    pub ModuleName: [super::super::Foundation::CHAR; 9],
    pub FileName: [super::super::Foundation::CHAR; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl VDM_SEGINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDM_SEGINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDM_SEGINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDM_SEGINFO").field("Selector", &self.Selector).field("SegNumber", &self.SegNumber).field("Length", &self.Length).field("Type", &self.Type).field("ModuleName", &self.ModuleName).field("FileName", &self.FileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDM_SEGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Selector == other.Selector && self.SegNumber == other.SegNumber && self.Length == other.Length && self.Type == other.Type && self.ModuleName == other.ModuleName && self.FileName == other.FileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDM_SEGINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDM_SEGINFO {
    type Abi = Self;
}
