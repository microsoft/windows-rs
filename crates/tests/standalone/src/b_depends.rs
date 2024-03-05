#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub type ADDRESS_FAMILY = u16;
pub type HANDLE = isize;
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<
    unsafe extern "system" fn(
        dwerror: u32,
        cbtransferred: u32,
        lpoverlapped: *mut OVERLAPPED,
        dwflags: u32,
    ),
>;
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: HANDLE,
}
impl Copy for OVERLAPPED {}
impl Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut core::ffi::c_void,
}
impl Copy for OVERLAPPED_0 {}
impl Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl Copy for OVERLAPPED_0_0 {}
impl Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PSTR = *mut u8;
#[repr(C)]
pub struct SOCKADDR {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [i8; 14],
}
impl Copy for SOCKADDR {}
impl Clone for SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSABUF {
    pub len: u32,
    pub buf: PSTR,
}
impl Copy for WSABUF {}
impl Clone for WSABUF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAMSG {
    pub name: *mut SOCKADDR,
    pub namelen: i32,
    pub lpBuffers: *mut WSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
impl Copy for WSAMSG {}
impl Clone for WSAMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSASENDMSG {
    pub lpMsg: *mut WSAMSG,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: *mut u32,
    pub lpOverlapped: *mut OVERLAPPED,
    pub lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
impl Copy for WSASENDMSG {}
impl Clone for WSASENDMSG {
    fn clone(&self) -> Self {
        *self
    }
}
