#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct InjectedInputButtonChangeKind(pub i32);
impl InjectedInputButtonChangeKind {
    pub const None: Self = Self(0i32);
    pub const FirstButtonDown: Self = Self(1i32);
    pub const FirstButtonUp: Self = Self(2i32);
    pub const SecondButtonDown: Self = Self(3i32);
    pub const SecondButtonUp: Self = Self(4i32);
    pub const ThirdButtonDown: Self = Self(5i32);
    pub const ThirdButtonUp: Self = Self(6i32);
    pub const FourthButtonDown: Self = Self(7i32);
    pub const FourthButtonUp: Self = Self(8i32);
    pub const FifthButtonDown: Self = Self(9i32);
    pub const FifthButtonUp: Self = Self(10i32);
}
#[repr(transparent)]
pub struct InjectedInputGamepadInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: Self = Self(0u32);
    pub const ExtendedKey: Self = Self(1u32);
    pub const KeyUp: Self = Self(2u32);
    pub const ScanCode: Self = Self(8u32);
    pub const Unicode: Self = Self(4u32);
}
#[repr(transparent)]
pub struct InjectedInputKeyboardInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputMouseInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: Self = Self(0u32);
    pub const Move: Self = Self(1u32);
    pub const LeftDown: Self = Self(2u32);
    pub const LeftUp: Self = Self(4u32);
    pub const RightDown: Self = Self(8u32);
    pub const RightUp: Self = Self(16u32);
    pub const MiddleDown: Self = Self(32u32);
    pub const MiddleUp: Self = Self(64u32);
    pub const XDown: Self = Self(128u32);
    pub const XUp: Self = Self(256u32);
    pub const Wheel: Self = Self(2048u32);
    pub const HWheel: Self = Self(4096u32);
    pub const MoveNoCoalesce: Self = Self(8192u32);
    pub const VirtualDesk: Self = Self(16384u32);
    pub const Absolute: Self = Self(32768u32);
}
#[repr(transparent)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: Self = Self(0u32);
    pub const Barrel: Self = Self(1u32);
    pub const Inverted: Self = Self(2u32);
    pub const Eraser: Self = Self(4u32);
}
#[repr(transparent)]
pub struct InjectedInputPenInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: Self = Self(0u32);
    pub const Pressure: Self = Self(1u32);
    pub const Rotation: Self = Self(2u32);
    pub const TiltX: Self = Self(4u32);
    pub const TiltY: Self = Self(8u32);
}
#[repr(C)]
pub struct InjectedInputPoint(i32);
#[repr(C)]
pub struct InjectedInputPointerInfo(i32);
#[repr(transparent)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: Self = Self(0u32);
    pub const New: Self = Self(1u32);
    pub const InRange: Self = Self(2u32);
    pub const InContact: Self = Self(4u32);
    pub const FirstButton: Self = Self(16u32);
    pub const SecondButton: Self = Self(32u32);
    pub const Primary: Self = Self(8192u32);
    pub const Confidence: Self = Self(16384u32);
    pub const Canceled: Self = Self(32768u32);
    pub const PointerDown: Self = Self(65536u32);
    pub const Update: Self = Self(131072u32);
    pub const PointerUp: Self = Self(262144u32);
    pub const CaptureChanged: Self = Self(2097152u32);
}
#[repr(C)]
pub struct InjectedInputRectangle(i32);
#[repr(transparent)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Search: Self = Self(2i32);
}
#[repr(transparent)]
pub struct InjectedInputTouchInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: Self = Self(0u32);
    pub const Contact: Self = Self(1u32);
    pub const Orientation: Self = Self(2u32);
    pub const Pressure: Self = Self(4u32);
}
#[repr(transparent)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Indirect: Self = Self(2i32);
}
#[repr(transparent)]
pub struct InputInjector(pub *mut ::core::ffi::c_void);
