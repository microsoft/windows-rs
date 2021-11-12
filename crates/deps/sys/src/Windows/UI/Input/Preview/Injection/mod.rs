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
    pub const None: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(0i32);
    pub const FirstButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(1i32);
    pub const FirstButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(2i32);
    pub const SecondButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(3i32);
    pub const SecondButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(4i32);
    pub const ThirdButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(5i32);
    pub const ThirdButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(6i32);
    pub const FourthButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(7i32);
    pub const FourthButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(8i32);
    pub const FifthButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(9i32);
    pub const FifthButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(10i32);
}
#[repr(transparent)]
pub struct InjectedInputGamepadInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: InjectedInputKeyOptions = InjectedInputKeyOptions(0u32);
    pub const ExtendedKey: InjectedInputKeyOptions = InjectedInputKeyOptions(1u32);
    pub const KeyUp: InjectedInputKeyOptions = InjectedInputKeyOptions(2u32);
    pub const ScanCode: InjectedInputKeyOptions = InjectedInputKeyOptions(8u32);
    pub const Unicode: InjectedInputKeyOptions = InjectedInputKeyOptions(4u32);
}
#[repr(transparent)]
pub struct InjectedInputKeyboardInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputMouseInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: InjectedInputMouseOptions = InjectedInputMouseOptions(0u32);
    pub const Move: InjectedInputMouseOptions = InjectedInputMouseOptions(1u32);
    pub const LeftDown: InjectedInputMouseOptions = InjectedInputMouseOptions(2u32);
    pub const LeftUp: InjectedInputMouseOptions = InjectedInputMouseOptions(4u32);
    pub const RightDown: InjectedInputMouseOptions = InjectedInputMouseOptions(8u32);
    pub const RightUp: InjectedInputMouseOptions = InjectedInputMouseOptions(16u32);
    pub const MiddleDown: InjectedInputMouseOptions = InjectedInputMouseOptions(32u32);
    pub const MiddleUp: InjectedInputMouseOptions = InjectedInputMouseOptions(64u32);
    pub const XDown: InjectedInputMouseOptions = InjectedInputMouseOptions(128u32);
    pub const XUp: InjectedInputMouseOptions = InjectedInputMouseOptions(256u32);
    pub const Wheel: InjectedInputMouseOptions = InjectedInputMouseOptions(2048u32);
    pub const HWheel: InjectedInputMouseOptions = InjectedInputMouseOptions(4096u32);
    pub const MoveNoCoalesce: InjectedInputMouseOptions = InjectedInputMouseOptions(8192u32);
    pub const VirtualDesk: InjectedInputMouseOptions = InjectedInputMouseOptions(16384u32);
    pub const Absolute: InjectedInputMouseOptions = InjectedInputMouseOptions(32768u32);
}
#[repr(transparent)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: InjectedInputPenButtons = InjectedInputPenButtons(0u32);
    pub const Barrel: InjectedInputPenButtons = InjectedInputPenButtons(1u32);
    pub const Inverted: InjectedInputPenButtons = InjectedInputPenButtons(2u32);
    pub const Eraser: InjectedInputPenButtons = InjectedInputPenButtons(4u32);
}
#[repr(transparent)]
pub struct InjectedInputPenInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: InjectedInputPenParameters = InjectedInputPenParameters(0u32);
    pub const Pressure: InjectedInputPenParameters = InjectedInputPenParameters(1u32);
    pub const Rotation: InjectedInputPenParameters = InjectedInputPenParameters(2u32);
    pub const TiltX: InjectedInputPenParameters = InjectedInputPenParameters(4u32);
    pub const TiltY: InjectedInputPenParameters = InjectedInputPenParameters(8u32);
}
#[repr(C)]
pub struct InjectedInputPoint(i32);
#[repr(C)]
pub struct InjectedInputPointerInfo(i32);
#[repr(transparent)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: InjectedInputPointerOptions = InjectedInputPointerOptions(0u32);
    pub const New: InjectedInputPointerOptions = InjectedInputPointerOptions(1u32);
    pub const InRange: InjectedInputPointerOptions = InjectedInputPointerOptions(2u32);
    pub const InContact: InjectedInputPointerOptions = InjectedInputPointerOptions(4u32);
    pub const FirstButton: InjectedInputPointerOptions = InjectedInputPointerOptions(16u32);
    pub const SecondButton: InjectedInputPointerOptions = InjectedInputPointerOptions(32u32);
    pub const Primary: InjectedInputPointerOptions = InjectedInputPointerOptions(8192u32);
    pub const Confidence: InjectedInputPointerOptions = InjectedInputPointerOptions(16384u32);
    pub const Canceled: InjectedInputPointerOptions = InjectedInputPointerOptions(32768u32);
    pub const PointerDown: InjectedInputPointerOptions = InjectedInputPointerOptions(65536u32);
    pub const Update: InjectedInputPointerOptions = InjectedInputPointerOptions(131072u32);
    pub const PointerUp: InjectedInputPointerOptions = InjectedInputPointerOptions(262144u32);
    pub const CaptureChanged: InjectedInputPointerOptions = InjectedInputPointerOptions(2097152u32);
}
#[repr(C)]
pub struct InjectedInputRectangle(i32);
#[repr(transparent)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: InjectedInputShortcut = InjectedInputShortcut(0i32);
    pub const Start: InjectedInputShortcut = InjectedInputShortcut(1i32);
    pub const Search: InjectedInputShortcut = InjectedInputShortcut(2i32);
}
#[repr(transparent)]
pub struct InjectedInputTouchInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: InjectedInputTouchParameters = InjectedInputTouchParameters(0u32);
    pub const Contact: InjectedInputTouchParameters = InjectedInputTouchParameters(1u32);
    pub const Orientation: InjectedInputTouchParameters = InjectedInputTouchParameters(2u32);
    pub const Pressure: InjectedInputTouchParameters = InjectedInputTouchParameters(4u32);
}
#[repr(transparent)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: InjectedInputVisualizationMode = InjectedInputVisualizationMode(0i32);
    pub const Default: InjectedInputVisualizationMode = InjectedInputVisualizationMode(1i32);
    pub const Indirect: InjectedInputVisualizationMode = InjectedInputVisualizationMode(2i32);
}
#[repr(transparent)]
pub struct InputInjector(pub *mut ::core::ffi::c_void);
