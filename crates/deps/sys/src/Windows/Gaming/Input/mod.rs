#![allow(non_snake_case, non_camel_case_types)]
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
pub struct ArcadeStickButtons(i32);
pub struct ArcadeStickReading(i32);
#[repr(transparent)]
pub struct FlightStick(pub *mut ::core::ffi::c_void);
pub struct FlightStickButtons(i32);
pub struct FlightStickReading(i32);
pub struct GameControllerButtonLabel(i32);
pub struct GameControllerSwitchKind(i32);
pub struct GameControllerSwitchPosition(i32);
#[repr(transparent)]
pub struct Gamepad(pub *mut ::core::ffi::c_void);
pub struct GamepadButtons(i32);
pub struct GamepadReading(i32);
pub struct GamepadVibration(i32);
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
pub struct OptionalUINavigationButtons(i32);
#[repr(transparent)]
pub struct RacingWheel(pub *mut ::core::ffi::c_void);
pub struct RacingWheelButtons(i32);
pub struct RacingWheelReading(i32);
#[repr(transparent)]
pub struct RawGameController(pub *mut ::core::ffi::c_void);
pub struct RequiredUINavigationButtons(i32);
#[repr(transparent)]
pub struct UINavigationController(pub *mut ::core::ffi::c_void);
pub struct UINavigationReading(i32);
