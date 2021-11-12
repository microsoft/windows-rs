#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
pub struct IKeyboardCapabilities(pub *mut ::core::ffi::c_void);
pub struct IMouseCapabilities(pub *mut ::core::ffi::c_void);
pub struct IMouseDevice(pub *mut ::core::ffi::c_void);
pub struct IMouseDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IMouseEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPenButtonListener(pub *mut ::core::ffi::c_void);
pub struct IPenButtonListenerStatics(pub *mut ::core::ffi::c_void);
pub struct IPenDevice(pub *mut ::core::ffi::c_void);
pub struct IPenDevice2(pub *mut ::core::ffi::c_void);
pub struct IPenDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IPenDockListener(pub *mut ::core::ffi::c_void);
pub struct IPenDockListenerStatics(pub *mut ::core::ffi::c_void);
pub struct IPenDockedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPenTailButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPenTailButtonDoubleClickedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPenTailButtonLongPressedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPenUndockedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPointerDevice(pub *mut ::core::ffi::c_void);
pub struct IPointerDevice2(pub *mut ::core::ffi::c_void);
pub struct IPointerDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct ITouchCapabilities(pub *mut ::core::ffi::c_void);
pub struct KeyboardCapabilities(i32);
pub struct MouseCapabilities(i32);
pub struct MouseDelta(i32);
pub struct MouseDevice(i32);
pub struct MouseEventArgs(i32);
pub struct PenButtonListener(i32);
pub struct PenDevice(i32);
pub struct PenDockListener(i32);
pub struct PenDockedEventArgs(i32);
pub struct PenTailButtonClickedEventArgs(i32);
pub struct PenTailButtonDoubleClickedEventArgs(i32);
pub struct PenTailButtonLongPressedEventArgs(i32);
pub struct PenUndockedEventArgs(i32);
pub struct PointerDevice(i32);
pub struct PointerDeviceType(i32);
pub struct PointerDeviceUsage(i32);
pub struct TouchCapabilities(i32);
