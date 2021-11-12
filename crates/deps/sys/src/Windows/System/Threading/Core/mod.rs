#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPreallocatedWorkItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPreallocatedWorkItem {}
impl ::core::clone::Clone for IPreallocatedWorkItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPreallocatedWorkItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPreallocatedWorkItemFactory {}
impl ::core::clone::Clone for IPreallocatedWorkItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignalNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignalNotifier {}
impl ::core::clone::Clone for ISignalNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignalNotifierStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignalNotifierStatics {}
impl ::core::clone::Clone for ISignalNotifierStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PreallocatedWorkItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PreallocatedWorkItem {}
impl ::core::clone::Clone for PreallocatedWorkItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SignalHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SignalHandler {}
impl ::core::clone::Clone for SignalHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SignalNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SignalNotifier {}
impl ::core::clone::Clone for SignalNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
