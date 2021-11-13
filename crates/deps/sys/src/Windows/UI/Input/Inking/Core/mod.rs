#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreIncrementalInkStroke(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreIncrementalInkStroke {}
impl ::core::clone::Clone for CoreIncrementalInkStroke {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInkIndependentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInkIndependentInputSource {}
impl ::core::clone::Clone for CoreInkIndependentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInkPresenterHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreInkPresenterHost {}
impl ::core::clone::Clone for CoreInkPresenterHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWetStrokeDisposition {}
impl ::core::clone::Clone for CoreWetStrokeDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWetStrokeUpdateEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWetStrokeUpdateEventArgs {}
impl ::core::clone::Clone for CoreWetStrokeUpdateEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWetStrokeUpdateSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWetStrokeUpdateSource {}
impl ::core::clone::Clone for CoreWetStrokeUpdateSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreIncrementalInkStroke(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreIncrementalInkStroke {}
impl ::core::clone::Clone for ICoreIncrementalInkStroke {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreIncrementalInkStrokeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreIncrementalInkStrokeFactory {}
impl ::core::clone::Clone for ICoreIncrementalInkStrokeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInkIndependentInputSource {}
impl ::core::clone::Clone for ICoreInkIndependentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInkIndependentInputSource2 {}
impl ::core::clone::Clone for ICoreInkIndependentInputSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInkIndependentInputSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInkIndependentInputSourceStatics {}
impl ::core::clone::Clone for ICoreInkIndependentInputSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInkPresenterHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInkPresenterHost {}
impl ::core::clone::Clone for ICoreInkPresenterHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWetStrokeUpdateEventArgs {}
impl ::core::clone::Clone for ICoreWetStrokeUpdateEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWetStrokeUpdateSource {}
impl ::core::clone::Clone for ICoreWetStrokeUpdateSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWetStrokeUpdateSourceStatics {}
impl ::core::clone::Clone for ICoreWetStrokeUpdateSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
