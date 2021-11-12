#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInjectedInputGamepadInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInjectedInputGamepadInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInjectedInputKeyboardInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInjectedInputMouseInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInjectedInputPenInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInjectedInputTouchInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputInjector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputInjector2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputInjectorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputInjectorStatics2(pub *mut ::core::ffi::c_void);
pub struct InjectedInputButtonChangeKind(i32);
#[repr(transparent)]
pub struct InjectedInputGamepadInfo(pub *mut ::core::ffi::c_void);
pub struct InjectedInputKeyOptions(i32);
#[repr(transparent)]
pub struct InjectedInputKeyboardInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputMouseInfo(pub *mut ::core::ffi::c_void);
pub struct InjectedInputMouseOptions(i32);
pub struct InjectedInputPenButtons(i32);
#[repr(transparent)]
pub struct InjectedInputPenInfo(pub *mut ::core::ffi::c_void);
pub struct InjectedInputPenParameters(i32);
pub struct InjectedInputPoint(i32);
pub struct InjectedInputPointerInfo(i32);
pub struct InjectedInputPointerOptions(i32);
pub struct InjectedInputRectangle(i32);
pub struct InjectedInputShortcut(i32);
#[repr(transparent)]
pub struct InjectedInputTouchInfo(pub *mut ::core::ffi::c_void);
pub struct InjectedInputTouchParameters(i32);
pub struct InjectedInputVisualizationMode(i32);
#[repr(transparent)]
pub struct InputInjector(pub *mut ::core::ffi::c_void);
