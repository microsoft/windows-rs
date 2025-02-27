#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type ADDRESS_FAMILY = u16;
pub type HANDLE = *mut core::ffi::c_void;
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<
    unsafe extern "system" fn(
        dwerror: u32,
        cbtransferred: u32,
        lpoverlapped: *mut OVERLAPPED,
        dwflags: u32,
    ),
>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: HANDLE,
}
impl Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut core::ffi::c_void,
}
impl Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
pub type PSTR = *mut u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOCKADDR {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [i8; 14],
}
impl Default for SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSABUF {
    pub len: u32,
    pub buf: PSTR,
}
impl Default for WSABUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSAMSG {
    pub name: *mut SOCKADDR,
    pub namelen: i32,
    pub lpBuffers: *mut WSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
impl Default for WSAMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSASENDMSG {
    pub lpMsg: *mut WSAMSG,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: *mut u32,
    pub lpOverlapped: *mut OVERLAPPED,
    pub lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
impl Default for WSASENDMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
