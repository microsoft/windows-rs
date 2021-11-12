#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreIncrementalInkStroke(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInkIndependentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInkPresenterHost(pub *mut ::core::ffi::c_void);
pub struct CoreWetStrokeDisposition(i32);
#[repr(transparent)]
pub struct CoreWetStrokeUpdateEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWetStrokeUpdateSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreIncrementalInkStroke(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreIncrementalInkStrokeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInkIndependentInputSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInkPresenterHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSourceStatics(pub *mut ::core::ffi::c_void);
