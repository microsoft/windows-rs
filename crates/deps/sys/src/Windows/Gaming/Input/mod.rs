#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ArcadeStick(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: ArcadeStickButtons = ArcadeStickButtons(0u32);
    pub const StickUp: ArcadeStickButtons = ArcadeStickButtons(1u32);
    pub const StickDown: ArcadeStickButtons = ArcadeStickButtons(2u32);
    pub const StickLeft: ArcadeStickButtons = ArcadeStickButtons(4u32);
    pub const StickRight: ArcadeStickButtons = ArcadeStickButtons(8u32);
    pub const Action1: ArcadeStickButtons = ArcadeStickButtons(16u32);
    pub const Action2: ArcadeStickButtons = ArcadeStickButtons(32u32);
    pub const Action3: ArcadeStickButtons = ArcadeStickButtons(64u32);
    pub const Action4: ArcadeStickButtons = ArcadeStickButtons(128u32);
    pub const Action5: ArcadeStickButtons = ArcadeStickButtons(256u32);
    pub const Action6: ArcadeStickButtons = ArcadeStickButtons(512u32);
    pub const Special1: ArcadeStickButtons = ArcadeStickButtons(1024u32);
    pub const Special2: ArcadeStickButtons = ArcadeStickButtons(2048u32);
}
#[repr(C)]
pub struct ArcadeStickReading(i32);
#[repr(transparent)]
pub struct FlightStick(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: FlightStickButtons = FlightStickButtons(0u32);
    pub const FirePrimary: FlightStickButtons = FlightStickButtons(1u32);
    pub const FireSecondary: FlightStickButtons = FlightStickButtons(2u32);
}
#[repr(C)]
pub struct FlightStickReading(i32);
#[repr(transparent)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: GameControllerButtonLabel = GameControllerButtonLabel(0i32);
    pub const XboxBack: GameControllerButtonLabel = GameControllerButtonLabel(1i32);
    pub const XboxStart: GameControllerButtonLabel = GameControllerButtonLabel(2i32);
    pub const XboxMenu: GameControllerButtonLabel = GameControllerButtonLabel(3i32);
    pub const XboxView: GameControllerButtonLabel = GameControllerButtonLabel(4i32);
    pub const XboxUp: GameControllerButtonLabel = GameControllerButtonLabel(5i32);
    pub const XboxDown: GameControllerButtonLabel = GameControllerButtonLabel(6i32);
    pub const XboxLeft: GameControllerButtonLabel = GameControllerButtonLabel(7i32);
    pub const XboxRight: GameControllerButtonLabel = GameControllerButtonLabel(8i32);
    pub const XboxA: GameControllerButtonLabel = GameControllerButtonLabel(9i32);
    pub const XboxB: GameControllerButtonLabel = GameControllerButtonLabel(10i32);
    pub const XboxX: GameControllerButtonLabel = GameControllerButtonLabel(11i32);
    pub const XboxY: GameControllerButtonLabel = GameControllerButtonLabel(12i32);
    pub const XboxLeftBumper: GameControllerButtonLabel = GameControllerButtonLabel(13i32);
    pub const XboxLeftTrigger: GameControllerButtonLabel = GameControllerButtonLabel(14i32);
    pub const XboxLeftStickButton: GameControllerButtonLabel = GameControllerButtonLabel(15i32);
    pub const XboxRightBumper: GameControllerButtonLabel = GameControllerButtonLabel(16i32);
    pub const XboxRightTrigger: GameControllerButtonLabel = GameControllerButtonLabel(17i32);
    pub const XboxRightStickButton: GameControllerButtonLabel = GameControllerButtonLabel(18i32);
    pub const XboxPaddle1: GameControllerButtonLabel = GameControllerButtonLabel(19i32);
    pub const XboxPaddle2: GameControllerButtonLabel = GameControllerButtonLabel(20i32);
    pub const XboxPaddle3: GameControllerButtonLabel = GameControllerButtonLabel(21i32);
    pub const XboxPaddle4: GameControllerButtonLabel = GameControllerButtonLabel(22i32);
    pub const Mode: GameControllerButtonLabel = GameControllerButtonLabel(23i32);
    pub const Select: GameControllerButtonLabel = GameControllerButtonLabel(24i32);
    pub const Menu: GameControllerButtonLabel = GameControllerButtonLabel(25i32);
    pub const View: GameControllerButtonLabel = GameControllerButtonLabel(26i32);
    pub const Back: GameControllerButtonLabel = GameControllerButtonLabel(27i32);
    pub const Start: GameControllerButtonLabel = GameControllerButtonLabel(28i32);
    pub const Options: GameControllerButtonLabel = GameControllerButtonLabel(29i32);
    pub const Share: GameControllerButtonLabel = GameControllerButtonLabel(30i32);
    pub const Up: GameControllerButtonLabel = GameControllerButtonLabel(31i32);
    pub const Down: GameControllerButtonLabel = GameControllerButtonLabel(32i32);
    pub const Left: GameControllerButtonLabel = GameControllerButtonLabel(33i32);
    pub const Right: GameControllerButtonLabel = GameControllerButtonLabel(34i32);
    pub const LetterA: GameControllerButtonLabel = GameControllerButtonLabel(35i32);
    pub const LetterB: GameControllerButtonLabel = GameControllerButtonLabel(36i32);
    pub const LetterC: GameControllerButtonLabel = GameControllerButtonLabel(37i32);
    pub const LetterL: GameControllerButtonLabel = GameControllerButtonLabel(38i32);
    pub const LetterR: GameControllerButtonLabel = GameControllerButtonLabel(39i32);
    pub const LetterX: GameControllerButtonLabel = GameControllerButtonLabel(40i32);
    pub const LetterY: GameControllerButtonLabel = GameControllerButtonLabel(41i32);
    pub const LetterZ: GameControllerButtonLabel = GameControllerButtonLabel(42i32);
    pub const Cross: GameControllerButtonLabel = GameControllerButtonLabel(43i32);
    pub const Circle: GameControllerButtonLabel = GameControllerButtonLabel(44i32);
    pub const Square: GameControllerButtonLabel = GameControllerButtonLabel(45i32);
    pub const Triangle: GameControllerButtonLabel = GameControllerButtonLabel(46i32);
    pub const LeftBumper: GameControllerButtonLabel = GameControllerButtonLabel(47i32);
    pub const LeftTrigger: GameControllerButtonLabel = GameControllerButtonLabel(48i32);
    pub const LeftStickButton: GameControllerButtonLabel = GameControllerButtonLabel(49i32);
    pub const Left1: GameControllerButtonLabel = GameControllerButtonLabel(50i32);
    pub const Left2: GameControllerButtonLabel = GameControllerButtonLabel(51i32);
    pub const Left3: GameControllerButtonLabel = GameControllerButtonLabel(52i32);
    pub const RightBumper: GameControllerButtonLabel = GameControllerButtonLabel(53i32);
    pub const RightTrigger: GameControllerButtonLabel = GameControllerButtonLabel(54i32);
    pub const RightStickButton: GameControllerButtonLabel = GameControllerButtonLabel(55i32);
    pub const Right1: GameControllerButtonLabel = GameControllerButtonLabel(56i32);
    pub const Right2: GameControllerButtonLabel = GameControllerButtonLabel(57i32);
    pub const Right3: GameControllerButtonLabel = GameControllerButtonLabel(58i32);
    pub const Paddle1: GameControllerButtonLabel = GameControllerButtonLabel(59i32);
    pub const Paddle2: GameControllerButtonLabel = GameControllerButtonLabel(60i32);
    pub const Paddle3: GameControllerButtonLabel = GameControllerButtonLabel(61i32);
    pub const Paddle4: GameControllerButtonLabel = GameControllerButtonLabel(62i32);
    pub const Plus: GameControllerButtonLabel = GameControllerButtonLabel(63i32);
    pub const Minus: GameControllerButtonLabel = GameControllerButtonLabel(64i32);
    pub const DownLeftArrow: GameControllerButtonLabel = GameControllerButtonLabel(65i32);
    pub const DialLeft: GameControllerButtonLabel = GameControllerButtonLabel(66i32);
    pub const DialRight: GameControllerButtonLabel = GameControllerButtonLabel(67i32);
    pub const Suspension: GameControllerButtonLabel = GameControllerButtonLabel(68i32);
}
#[repr(transparent)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: GameControllerSwitchKind = GameControllerSwitchKind(0i32);
    pub const FourWay: GameControllerSwitchKind = GameControllerSwitchKind(1i32);
    pub const EightWay: GameControllerSwitchKind = GameControllerSwitchKind(2i32);
}
#[repr(transparent)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: GameControllerSwitchPosition = GameControllerSwitchPosition(0i32);
    pub const Up: GameControllerSwitchPosition = GameControllerSwitchPosition(1i32);
    pub const UpRight: GameControllerSwitchPosition = GameControllerSwitchPosition(2i32);
    pub const Right: GameControllerSwitchPosition = GameControllerSwitchPosition(3i32);
    pub const DownRight: GameControllerSwitchPosition = GameControllerSwitchPosition(4i32);
    pub const Down: GameControllerSwitchPosition = GameControllerSwitchPosition(5i32);
    pub const DownLeft: GameControllerSwitchPosition = GameControllerSwitchPosition(6i32);
    pub const Left: GameControllerSwitchPosition = GameControllerSwitchPosition(7i32);
    pub const UpLeft: GameControllerSwitchPosition = GameControllerSwitchPosition(8i32);
}
#[repr(transparent)]
pub struct Gamepad(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: GamepadButtons = GamepadButtons(0u32);
    pub const Menu: GamepadButtons = GamepadButtons(1u32);
    pub const View: GamepadButtons = GamepadButtons(2u32);
    pub const A: GamepadButtons = GamepadButtons(4u32);
    pub const B: GamepadButtons = GamepadButtons(8u32);
    pub const X: GamepadButtons = GamepadButtons(16u32);
    pub const Y: GamepadButtons = GamepadButtons(32u32);
    pub const DPadUp: GamepadButtons = GamepadButtons(64u32);
    pub const DPadDown: GamepadButtons = GamepadButtons(128u32);
    pub const DPadLeft: GamepadButtons = GamepadButtons(256u32);
    pub const DPadRight: GamepadButtons = GamepadButtons(512u32);
    pub const LeftShoulder: GamepadButtons = GamepadButtons(1024u32);
    pub const RightShoulder: GamepadButtons = GamepadButtons(2048u32);
    pub const LeftThumbstick: GamepadButtons = GamepadButtons(4096u32);
    pub const RightThumbstick: GamepadButtons = GamepadButtons(8192u32);
    pub const Paddle1: GamepadButtons = GamepadButtons(16384u32);
    pub const Paddle2: GamepadButtons = GamepadButtons(32768u32);
    pub const Paddle3: GamepadButtons = GamepadButtons(65536u32);
    pub const Paddle4: GamepadButtons = GamepadButtons(131072u32);
}
#[repr(C)]
pub struct GamepadReading(i32);
#[repr(C)]
pub struct GamepadVibration(i32);
#[repr(C)]
pub struct GamingInputPreviewContract(i32);
#[repr(transparent)]
pub struct Headset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IArcadeStick(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IArcadeStickStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IArcadeStickStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlightStick(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlightStickStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameControllerBatteryInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGamepad(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGamepad2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGamepadStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGamepadStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeadset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRacingWheel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRacingWheelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRacingWheelStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawGameController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawGameController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawGameControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUINavigationController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUINavigationControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUINavigationControllerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: OptionalUINavigationButtons = OptionalUINavigationButtons(0u32);
    pub const Context1: OptionalUINavigationButtons = OptionalUINavigationButtons(1u32);
    pub const Context2: OptionalUINavigationButtons = OptionalUINavigationButtons(2u32);
    pub const Context3: OptionalUINavigationButtons = OptionalUINavigationButtons(4u32);
    pub const Context4: OptionalUINavigationButtons = OptionalUINavigationButtons(8u32);
    pub const PageUp: OptionalUINavigationButtons = OptionalUINavigationButtons(16u32);
    pub const PageDown: OptionalUINavigationButtons = OptionalUINavigationButtons(32u32);
    pub const PageLeft: OptionalUINavigationButtons = OptionalUINavigationButtons(64u32);
    pub const PageRight: OptionalUINavigationButtons = OptionalUINavigationButtons(128u32);
    pub const ScrollUp: OptionalUINavigationButtons = OptionalUINavigationButtons(256u32);
    pub const ScrollDown: OptionalUINavigationButtons = OptionalUINavigationButtons(512u32);
    pub const ScrollLeft: OptionalUINavigationButtons = OptionalUINavigationButtons(1024u32);
    pub const ScrollRight: OptionalUINavigationButtons = OptionalUINavigationButtons(2048u32);
}
#[repr(transparent)]
pub struct RacingWheel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: RacingWheelButtons = RacingWheelButtons(0u32);
    pub const PreviousGear: RacingWheelButtons = RacingWheelButtons(1u32);
    pub const NextGear: RacingWheelButtons = RacingWheelButtons(2u32);
    pub const DPadUp: RacingWheelButtons = RacingWheelButtons(4u32);
    pub const DPadDown: RacingWheelButtons = RacingWheelButtons(8u32);
    pub const DPadLeft: RacingWheelButtons = RacingWheelButtons(16u32);
    pub const DPadRight: RacingWheelButtons = RacingWheelButtons(32u32);
    pub const Button1: RacingWheelButtons = RacingWheelButtons(64u32);
    pub const Button2: RacingWheelButtons = RacingWheelButtons(128u32);
    pub const Button3: RacingWheelButtons = RacingWheelButtons(256u32);
    pub const Button4: RacingWheelButtons = RacingWheelButtons(512u32);
    pub const Button5: RacingWheelButtons = RacingWheelButtons(1024u32);
    pub const Button6: RacingWheelButtons = RacingWheelButtons(2048u32);
    pub const Button7: RacingWheelButtons = RacingWheelButtons(4096u32);
    pub const Button8: RacingWheelButtons = RacingWheelButtons(8192u32);
    pub const Button9: RacingWheelButtons = RacingWheelButtons(16384u32);
    pub const Button10: RacingWheelButtons = RacingWheelButtons(32768u32);
    pub const Button11: RacingWheelButtons = RacingWheelButtons(65536u32);
    pub const Button12: RacingWheelButtons = RacingWheelButtons(131072u32);
    pub const Button13: RacingWheelButtons = RacingWheelButtons(262144u32);
    pub const Button14: RacingWheelButtons = RacingWheelButtons(524288u32);
    pub const Button15: RacingWheelButtons = RacingWheelButtons(1048576u32);
    pub const Button16: RacingWheelButtons = RacingWheelButtons(2097152u32);
}
#[repr(C)]
pub struct RacingWheelReading(i32);
#[repr(transparent)]
pub struct RawGameController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: RequiredUINavigationButtons = RequiredUINavigationButtons(0u32);
    pub const Menu: RequiredUINavigationButtons = RequiredUINavigationButtons(1u32);
    pub const View: RequiredUINavigationButtons = RequiredUINavigationButtons(2u32);
    pub const Accept: RequiredUINavigationButtons = RequiredUINavigationButtons(4u32);
    pub const Cancel: RequiredUINavigationButtons = RequiredUINavigationButtons(8u32);
    pub const Up: RequiredUINavigationButtons = RequiredUINavigationButtons(16u32);
    pub const Down: RequiredUINavigationButtons = RequiredUINavigationButtons(32u32);
    pub const Left: RequiredUINavigationButtons = RequiredUINavigationButtons(64u32);
    pub const Right: RequiredUINavigationButtons = RequiredUINavigationButtons(128u32);
}
#[repr(transparent)]
pub struct UINavigationController(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UINavigationReading(i32);
