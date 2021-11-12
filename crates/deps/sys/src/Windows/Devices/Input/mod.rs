#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKeyboardCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyboardCapabilities {}
impl ::core::clone::Clone for IKeyboardCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMouseCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMouseCapabilities {}
impl ::core::clone::Clone for IMouseCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMouseDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMouseDevice {}
impl ::core::clone::Clone for IMouseDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMouseDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMouseDeviceStatics {}
impl ::core::clone::Clone for IMouseDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMouseEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMouseEventArgs {}
impl ::core::clone::Clone for IMouseEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenButtonListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenButtonListener {}
impl ::core::clone::Clone for IPenButtonListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenButtonListenerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenButtonListenerStatics {}
impl ::core::clone::Clone for IPenButtonListenerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDevice {}
impl ::core::clone::Clone for IPenDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDevice2 {}
impl ::core::clone::Clone for IPenDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDeviceStatics {}
impl ::core::clone::Clone for IPenDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDockListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDockListener {}
impl ::core::clone::Clone for IPenDockListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDockListenerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDockListenerStatics {}
impl ::core::clone::Clone for IPenDockListenerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenDockedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenDockedEventArgs {}
impl ::core::clone::Clone for IPenDockedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenTailButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenTailButtonClickedEventArgs {}
impl ::core::clone::Clone for IPenTailButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenTailButtonDoubleClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenTailButtonDoubleClickedEventArgs {}
impl ::core::clone::Clone for IPenTailButtonDoubleClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenTailButtonLongPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenTailButtonLongPressedEventArgs {}
impl ::core::clone::Clone for IPenTailButtonLongPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenUndockedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenUndockedEventArgs {}
impl ::core::clone::Clone for IPenUndockedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerDevice {}
impl ::core::clone::Clone for IPointerDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerDevice2 {}
impl ::core::clone::Clone for IPointerDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerDeviceStatics {}
impl ::core::clone::Clone for IPointerDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITouchCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITouchCapabilities {}
impl ::core::clone::Clone for ITouchCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyboardCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyboardCapabilities {}
impl ::core::clone::Clone for KeyboardCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MouseCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MouseCapabilities {}
impl ::core::clone::Clone for MouseCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MouseDevice {}
impl ::core::clone::Clone for MouseDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MouseEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MouseEventArgs {}
impl ::core::clone::Clone for MouseEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenButtonListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenButtonListener {}
impl ::core::clone::Clone for PenButtonListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenDevice {}
impl ::core::clone::Clone for PenDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenDockListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenDockListener {}
impl ::core::clone::Clone for PenDockListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenDockedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenDockedEventArgs {}
impl ::core::clone::Clone for PenDockedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenTailButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenTailButtonClickedEventArgs {}
impl ::core::clone::Clone for PenTailButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenTailButtonDoubleClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenTailButtonDoubleClickedEventArgs {}
impl ::core::clone::Clone for PenTailButtonDoubleClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenTailButtonLongPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenTailButtonLongPressedEventArgs {}
impl ::core::clone::Clone for PenTailButtonLongPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenUndockedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenUndockedEventArgs {}
impl ::core::clone::Clone for PenUndockedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerDevice {}
impl ::core::clone::Clone for PointerDevice {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TouchCapabilities {}
impl ::core::clone::Clone for TouchCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
