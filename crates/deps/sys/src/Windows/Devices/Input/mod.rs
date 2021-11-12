#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKeyboardCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMouseCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMouseDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMouseDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMouseEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenButtonListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenButtonListenerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDockListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDockListenerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenDockedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenTailButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenTailButtonDoubleClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenTailButtonLongPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenUndockedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITouchCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MouseCapabilities(pub *mut ::core::ffi::c_void);
pub struct MouseDelta(i32);
#[repr(transparent)]
pub struct MouseDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MouseEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenButtonListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenDockListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenDockedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenTailButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenTailButtonDoubleClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenTailButtonLongPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenUndockedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerDevice(pub *mut ::core::ffi::c_void);
pub struct PointerDeviceType(i32);
pub struct PointerDeviceUsage(i32);
#[repr(transparent)]
pub struct TouchCapabilities(pub *mut ::core::ffi::c_void);
