#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CoreDragDropManager(i32);
pub struct CoreDragInfo(i32);
pub struct CoreDragOperation(i32);
pub struct CoreDragUIContentMode(i32);
pub struct CoreDragUIOverride(i32);
pub struct CoreDropOperationTargetRequestedEventArgs(i32);
pub struct ICoreDragDropManager(pub *mut ::core::ffi::c_void);
pub struct ICoreDragDropManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ICoreDragInfo(pub *mut ::core::ffi::c_void);
pub struct ICoreDragInfo2(pub *mut ::core::ffi::c_void);
pub struct ICoreDragOperation(pub *mut ::core::ffi::c_void);
pub struct ICoreDragOperation2(pub *mut ::core::ffi::c_void);
pub struct ICoreDragUIOverride(pub *mut ::core::ffi::c_void);
pub struct ICoreDropOperationTarget(pub *mut ::core::ffi::c_void);
pub struct ICoreDropOperationTargetRequestedEventArgs(pub *mut ::core::ffi::c_void);
