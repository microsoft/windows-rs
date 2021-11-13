#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreDragDropManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDragDropManager {}
impl ::core::clone::Clone for CoreDragDropManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDragInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDragInfo {}
impl ::core::clone::Clone for CoreDragInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDragOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDragOperation {}
impl ::core::clone::Clone for CoreDragOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
impl ::core::marker::Copy for CoreDragUIContentMode {}
impl ::core::clone::Clone for CoreDragUIContentMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDragUIOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDragUIOverride {}
impl ::core::clone::Clone for CoreDragUIOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDropOperationTargetRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDropOperationTargetRequestedEventArgs {}
impl ::core::clone::Clone for CoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragDropManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragDropManager {}
impl ::core::clone::Clone for ICoreDragDropManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragDropManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragDropManagerStatics {}
impl ::core::clone::Clone for ICoreDragDropManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragInfo {}
impl ::core::clone::Clone for ICoreDragInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragInfo2 {}
impl ::core::clone::Clone for ICoreDragInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragOperation {}
impl ::core::clone::Clone for ICoreDragOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragOperation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragOperation2 {}
impl ::core::clone::Clone for ICoreDragOperation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDragUIOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDragUIOverride {}
impl ::core::clone::Clone for ICoreDragUIOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDropOperationTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDropOperationTarget {}
impl ::core::clone::Clone for ICoreDropOperationTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDropOperationTargetRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDropOperationTargetRequestedEventArgs {}
impl ::core::clone::Clone for ICoreDropOperationTargetRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
