#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(C)]
pub struct MouseDelta {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for MouseDelta {}
impl ::core::clone::Clone for MouseDelta {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PointerDeviceUsage {
    pub UsagePage: u32,
    pub Usage: u32,
    pub MinLogical: i32,
    pub MaxLogical: i32,
    pub MinPhysical: i32,
    pub MaxPhysical: i32,
    pub Unit: u32,
    pub PhysicalMultiplier: f32,
}
impl ::core::marker::Copy for PointerDeviceUsage {}
impl ::core::clone::Clone for PointerDeviceUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TouchCapabilities(pub *mut ::core::ffi::c_void);
