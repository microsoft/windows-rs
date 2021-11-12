#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreDragDropManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreDragInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreDragOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
#[repr(transparent)]
pub struct CoreDragUIOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreDropOperationTargetRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragDropManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragDropManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDragUIOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDropOperationTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDropOperationTargetRequestedEventArgs(pub *mut ::core::ffi::c_void);
