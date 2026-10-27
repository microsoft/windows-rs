windows_link::link!("gdi32.dll" "system" fn BitBlt(hdc : HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : HDC, x1 : i32, y1 : i32, rop : DWORD) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleBitmap(hdc : HDC, cx : i32, cy : i32) -> HBITMAP);
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleDC(hdc : HDC) -> HDC);
windows_link::link!("gdi32.dll" "system" fn DeleteDC(hdc : HDC) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : HGDIOBJ) -> BOOL);
windows_link::link!("user32.dll" "system" fn GetMessageA(lpmsg : LPMSG, hwnd : HWND, wmsgfiltermin : UINT, wmsgfiltermax : UINT) -> BOOL);
windows_link::link!("user32.dll" "system" fn MessageBoxA(hwnd : HWND, lptext : PCSTR, lpcaption : PCSTR, utype : UINT) -> i32);
windows_link::link!("gdi32.dll" "system" fn Polyline(hdc : HDC, apt : *const POINT, cpt : i32) -> BOOL);
windows_link::link!("gdi32.dll" "system" fn SelectObject(hdc : HDC, h : HGDIOBJ) -> HGDIOBJ);
#[repr(C, align(16))]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: DWORD,
    pub Cpsr: DWORD,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: DWORD64,
    pub Pc: DWORD64,
    pub V: [NEON128; 32],
    pub Fpcr: DWORD,
    pub Fpsr: DWORD,
    pub Bcr: [DWORD; 8],
    pub Bvr: [DWORD64; 8],
    pub Wcr: [DWORD; 2],
    pub Wvr: [DWORD64; 2],
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
    pub X: [DWORD64; 31],
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
    pub X0: DWORD64,
    pub X1: DWORD64,
    pub X2: DWORD64,
    pub X3: DWORD64,
    pub X4: DWORD64,
    pub X5: DWORD64,
    pub X6: DWORD64,
    pub X7: DWORD64,
    pub X8: DWORD64,
    pub X9: DWORD64,
    pub X10: DWORD64,
    pub X11: DWORD64,
    pub X12: DWORD64,
    pub X13: DWORD64,
    pub X14: DWORD64,
    pub X15: DWORD64,
    pub X16: DWORD64,
    pub X17: DWORD64,
    pub X18: DWORD64,
    pub X19: DWORD64,
    pub X20: DWORD64,
    pub X21: DWORD64,
    pub X22: DWORD64,
    pub X23: DWORD64,
    pub X24: DWORD64,
    pub X25: DWORD64,
    pub X26: DWORD64,
    pub X27: DWORD64,
    pub X28: DWORD64,
    pub Fp: DWORD64,
    pub Lr: DWORD64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union ARM64_NT_NEON128 {
    pub Anonymous: ARM64_NT_NEON128_0,
    pub D: [f64; 2],
    pub S: [f32; 4],
    pub H: [WORD; 8],
    pub B: [BYTE; 16],
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Default)]
pub struct ARM64_NT_NEON128_0 {
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type BOOL = i32;
pub type BYTE = u8;
pub type CHAR = i8;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub ContextFlags: DWORD,
    pub Dr0: DWORD,
    pub Dr1: DWORD,
    pub Dr2: DWORD,
    pub Dr3: DWORD,
    pub Dr6: DWORD,
    pub Dr7: DWORD,
    pub FloatSave: FLOATING_SAVE_AREA,
    pub SegGs: DWORD,
    pub SegFs: DWORD,
    pub SegEs: DWORD,
    pub SegDs: DWORD,
    pub Edi: DWORD,
    pub Esi: DWORD,
    pub Ebx: DWORD,
    pub Edx: DWORD,
    pub Ecx: DWORD,
    pub Eax: DWORD,
    pub Ebp: DWORD,
    pub Eip: DWORD,
    pub SegCs: DWORD,
    pub EFlags: DWORD,
    pub Esp: DWORD,
    pub SegSs: DWORD,
    pub ExtendedRegisters: [BYTE; 512],
}
#[cfg(target_arch = "x86")]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(16))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: DWORD64,
    pub P2Home: DWORD64,
    pub P3Home: DWORD64,
    pub P4Home: DWORD64,
    pub P5Home: DWORD64,
    pub P6Home: DWORD64,
    pub ContextFlags: DWORD,
    pub MxCsr: DWORD,
    pub SegCs: WORD,
    pub SegDs: WORD,
    pub SegEs: WORD,
    pub SegFs: WORD,
    pub SegGs: WORD,
    pub SegSs: WORD,
    pub EFlags: DWORD,
    pub Dr0: DWORD64,
    pub Dr1: DWORD64,
    pub Dr2: DWORD64,
    pub Dr3: DWORD64,
    pub Dr6: DWORD64,
    pub Dr7: DWORD64,
    pub Rax: DWORD64,
    pub Rcx: DWORD64,
    pub Rdx: DWORD64,
    pub Rbx: DWORD64,
    pub Rsp: DWORD64,
    pub Rbp: DWORD64,
    pub Rsi: DWORD64,
    pub Rdi: DWORD64,
    pub R8: DWORD64,
    pub R9: DWORD64,
    pub R10: DWORD64,
    pub R11: DWORD64,
    pub R12: DWORD64,
    pub R13: DWORD64,
    pub R14: DWORD64,
    pub R15: DWORD64,
    pub Rip: DWORD64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: DWORD64,
    pub DebugControl: DWORD64,
    pub LastBranchToRip: DWORD64,
    pub LastBranchFromRip: DWORD64,
    pub LastExceptionToRip: DWORD64,
    pub LastExceptionFromRip: DWORD64,
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
pub type DISPATCHERQUEUE_THREAD_APARTMENTTYPE = i32;
pub type DISPATCHERQUEUE_THREAD_TYPE = i32;
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 1;
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 0;
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 2;
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = 2;
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = 1;
pub type DWORD = u32;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type DWORD64 = u64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DispatcherQueueOptions {
    pub dwSize: DWORD,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: DWORD,
    pub StatusWord: DWORD,
    pub TagWord: DWORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: DWORD,
    pub DataOffset: DWORD,
    pub DataSelector: DWORD,
    pub RegisterArea: [BYTE; 80],
    pub Spare0: DWORD,
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
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HBITMAP__ {
    pub unused: i32,
}
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HBRUSH__ {
    pub unused: i32,
}
pub type HCURSOR = *mut HICON__;
pub type HDC = *mut HDC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HDC__ {
    pub unused: i32,
}
pub type HGDIOBJ = *mut core::ffi::c_void;
pub type HICON = *mut HICON__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HICON__ {
    pub unused: i32,
}
pub type HINSTANCE = *mut HINSTANCE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HINSTANCE__ {
    pub unused: i32,
}
pub type HKEY = *mut HKEY__;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HKEY__ {
    pub unused: i32,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HWND__ {
    pub unused: i32,
}
pub const INVALID_HANDLE_VALUE: HANDLE = -1 as _;
pub type LONG = i32;
pub type LONGLONG = i64;
pub type LPARAM = isize;
pub type LPMSG = *mut MSG;
pub type LRESULT = isize;
#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
pub struct M128A {
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
pub type MENUTEMPLATEA = core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MONITORINFO {
    pub cbSize: DWORD,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: DWORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITORINFOEXA {
    pub base: MONITORINFO,
    pub szDevice: [CHAR; 32],
}
impl Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}
#[cfg(target_arch = "aarch64")]
pub type NEON128 = ARM64_NT_NEON128;
pub type PCSTR = *const u8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
pub type PWSTR = *mut u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
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
#[repr(C, packed(8))]
#[derive(Clone, Copy)]
pub struct STRRET {
    pub uType: UINT,
    pub Anonymous: STRRET_0,
}
impl Default for STRRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[derive(Clone, Copy)]
pub union STRRET_0 {
    pub pOleStr: PWSTR,
    pub uOffset: UINT,
    pub cStr: [i8; 260],
}
impl Default for STRRET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UINT = u32;
pub type ULONGLONG = u64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WNDCLASSEXA {
    pub cbSize: UINT,
    pub style: UINT,
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
pub type WNDPROC = Option<
    unsafe extern "system" fn(
        param0: HWND,
        param1: UINT,
        param2: WPARAM,
        param3: LPARAM,
    ) -> LRESULT,
>;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type WORD = u16;
pub type WPARAM = usize;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type XMM_SAVE_AREA32 = XSAVE_FORMAT;
#[repr(C, align(16))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: BYTE,
    pub Reserved1: BYTE,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [BYTE; 96],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
