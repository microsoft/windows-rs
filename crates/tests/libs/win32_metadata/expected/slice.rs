windows_link::link!("gdi32.dll" "system" fn BitBlt(hdc : HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : HDC, x1 : i32, y1 : i32, rop : u32) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleBitmap(hdc : HDC, cx : i32, cy : i32) -> HBITMAP);
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleDC(hdc : HDC) -> HDC);
windows_link::link!("gdi32.dll" "system" fn DeleteDC(hdc : HDC) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : HGDIOBJ) -> BOOL);
windows_link::link!("user32.dll" "system" fn GetMessageA(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> BOOL);
windows_link::link!("user32.dll" "system" fn MessageBoxA(hwnd : HWND, lptext : PCSTR, lpcaption : PCSTR, utype : u32) -> i32);
windows_link::link!("gdi32.dll" "system" fn Polyline(hdc : HDC, apt : *const POINT, cpt : i32) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn SelectObject(hdc : HDC, h : HGDIOBJ) -> HGDIOBJ);
#[repr(C, align(16))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for ARM64_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[repr(C, align(16))]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Default)]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ARM64_NT_NEON128 {
    pub Anonymous: ARM64_NT_NEON128_0,
    pub D: [f64; 2],
    pub S: [f32; 4],
    pub H: [u16; 8],
    pub B: [u8; 16],
}
impl Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ARM64_NT_NEON128_0 {
    pub Low: u64,
    pub High: i64,
}
pub type BOOL = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: FLOATING_SAVE_AREA,
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
#[cfg(target_arch = "x86")]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union CONTEXT_0 {
    pub FltSave: XMM_SAVE_AREA32,
    pub Anonymous: CONTEXT_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT_0_0 {
    pub Header: [M128A; 2],
    pub Legacy: [M128A; 8],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type CONTEXT = ARM64_NT_CONTEXT;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[cfg(target_arch = "x86")]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub const GUID_DEVINTERFACE_CDROM: GUID = GUID {
    data1: 0x53f56308,
    data2: 0xb6bf,
    data3: 0x11d0,
    data4: [148, 242, 0, 160, 201, 30, 251, 139],
};
pub type HANDLE = *mut core::ffi::c_void;
pub type HBITMAP = *mut core::ffi::c_void;
pub type HBRUSH = *mut core::ffi::c_void;
pub type HCURSOR = HICON;
pub type HDC = *mut core::ffi::c_void;
pub type HGDIOBJ = *mut core::ffi::c_void;
pub type HICON = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HKEY = *mut core::ffi::c_void;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646 as _;
pub type HWND = *mut core::ffi::c_void;
pub const INVALID_HANDLE_VALUE: HANDLE = -1 as _;
pub type LPARAM = isize;
pub type LRESULT = isize;
#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
pub type MENUTEMPLATEA = core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITORINFOEXA {
    pub Base: MONITORINFO,
    pub szDevice: [i8; 32],
}
impl Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
impl Default for MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type NEON128 = ARM64_NT_NEON128;
pub type PCSTR = *const u8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
pub type PWSTR = *mut u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
impl Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl Default for STRRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STRRET_0 {
    pub pOleStr: PWSTR,
    pub uOffset: u32,
    pub cStr: [i8; 260],
}
impl Default for STRRET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNDCLASSEXA {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: PCSTR,
    pub lpszClassName: PCSTR,
    pub hIconSm: HICON,
}
impl Default for WNDCLASSEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WNDPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
pub type WPARAM = usize;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type XMM_SAVE_AREA32 = XSAVE_FORMAT;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [u8; 224],
}
#[cfg(target_arch = "x86")]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [u8; 96],
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
