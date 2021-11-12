#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CoreIncrementalInkStroke(i32);
pub struct CoreInkIndependentInputSource(i32);
pub struct CoreInkPresenterHost(i32);
pub struct CoreWetStrokeDisposition(i32);
pub struct CoreWetStrokeUpdateEventArgs(i32);
pub struct CoreWetStrokeUpdateSource(i32);
pub struct ICoreIncrementalInkStroke(pub *mut ::core::ffi::c_void);
pub struct ICoreIncrementalInkStrokeFactory(pub *mut ::core::ffi::c_void);
pub struct ICoreInkIndependentInputSource(pub *mut ::core::ffi::c_void);
pub struct ICoreInkIndependentInputSource2(pub *mut ::core::ffi::c_void);
pub struct ICoreInkIndependentInputSourceStatics(pub *mut ::core::ffi::c_void);
pub struct ICoreInkPresenterHost(pub *mut ::core::ffi::c_void);
pub struct ICoreWetStrokeUpdateEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICoreWetStrokeUpdateSource(pub *mut ::core::ffi::c_void);
pub struct ICoreWetStrokeUpdateSourceStatics(pub *mut ::core::ffi::c_void);
