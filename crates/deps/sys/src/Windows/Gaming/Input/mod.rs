#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
pub struct ArcadeStick(i32);
pub struct ArcadeStickButtons(i32);
pub struct ArcadeStickReading(i32);
pub struct FlightStick(i32);
pub struct FlightStickButtons(i32);
pub struct FlightStickReading(i32);
pub struct GameControllerButtonLabel(i32);
pub struct GameControllerSwitchKind(i32);
pub struct GameControllerSwitchPosition(i32);
pub struct Gamepad(i32);
pub struct GamepadButtons(i32);
pub struct GamepadReading(i32);
pub struct GamepadVibration(i32);
pub struct GamingInputPreviewContract(i32);
pub struct Headset(i32);
pub struct IArcadeStick(pub *mut ::core::ffi::c_void);
pub struct IArcadeStickStatics(pub *mut ::core::ffi::c_void);
pub struct IArcadeStickStatics2(pub *mut ::core::ffi::c_void);
pub struct IFlightStick(pub *mut ::core::ffi::c_void);
pub struct IFlightStickStatics(pub *mut ::core::ffi::c_void);
pub struct IGameController(pub *mut ::core::ffi::c_void);
pub struct IGameControllerBatteryInfo(pub *mut ::core::ffi::c_void);
pub struct IGamepad(pub *mut ::core::ffi::c_void);
pub struct IGamepad2(pub *mut ::core::ffi::c_void);
pub struct IGamepadStatics(pub *mut ::core::ffi::c_void);
pub struct IGamepadStatics2(pub *mut ::core::ffi::c_void);
pub struct IHeadset(pub *mut ::core::ffi::c_void);
pub struct IRacingWheel(pub *mut ::core::ffi::c_void);
pub struct IRacingWheelStatics(pub *mut ::core::ffi::c_void);
pub struct IRacingWheelStatics2(pub *mut ::core::ffi::c_void);
pub struct IRawGameController(pub *mut ::core::ffi::c_void);
pub struct IRawGameController2(pub *mut ::core::ffi::c_void);
pub struct IRawGameControllerStatics(pub *mut ::core::ffi::c_void);
pub struct IUINavigationController(pub *mut ::core::ffi::c_void);
pub struct IUINavigationControllerStatics(pub *mut ::core::ffi::c_void);
pub struct IUINavigationControllerStatics2(pub *mut ::core::ffi::c_void);
pub struct OptionalUINavigationButtons(i32);
pub struct RacingWheel(i32);
pub struct RacingWheelButtons(i32);
pub struct RacingWheelReading(i32);
pub struct RawGameController(i32);
pub struct RequiredUINavigationButtons(i32);
pub struct UINavigationController(i32);
pub struct UINavigationReading(i32);
