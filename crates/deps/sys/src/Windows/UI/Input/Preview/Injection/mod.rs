#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInjectedInputGamepadInfo(pub *mut ::core::ffi::c_void);
pub struct IInjectedInputGamepadInfoFactory(pub *mut ::core::ffi::c_void);
pub struct IInjectedInputKeyboardInfo(pub *mut ::core::ffi::c_void);
pub struct IInjectedInputMouseInfo(pub *mut ::core::ffi::c_void);
pub struct IInjectedInputPenInfo(pub *mut ::core::ffi::c_void);
pub struct IInjectedInputTouchInfo(pub *mut ::core::ffi::c_void);
pub struct IInputInjector(pub *mut ::core::ffi::c_void);
pub struct IInputInjector2(pub *mut ::core::ffi::c_void);
pub struct IInputInjectorStatics(pub *mut ::core::ffi::c_void);
pub struct IInputInjectorStatics2(pub *mut ::core::ffi::c_void);
pub struct InjectedInputButtonChangeKind(i32);
pub struct InjectedInputGamepadInfo(i32);
pub struct InjectedInputKeyOptions(i32);
pub struct InjectedInputKeyboardInfo(i32);
pub struct InjectedInputMouseInfo(i32);
pub struct InjectedInputMouseOptions(i32);
pub struct InjectedInputPenButtons(i32);
pub struct InjectedInputPenInfo(i32);
pub struct InjectedInputPenParameters(i32);
pub struct InjectedInputPoint(i32);
pub struct InjectedInputPointerInfo(i32);
pub struct InjectedInputPointerOptions(i32);
pub struct InjectedInputRectangle(i32);
pub struct InjectedInputShortcut(i32);
pub struct InjectedInputTouchInfo(i32);
pub struct InjectedInputTouchParameters(i32);
pub struct InjectedInputVisualizationMode(i32);
pub struct InputInjector(i32);
