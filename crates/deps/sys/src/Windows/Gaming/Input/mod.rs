#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {
    fn ArcadeStick();
    fn ArcadeStickButtons();
    fn ArcadeStickReading();
    fn FlightStick();
    fn FlightStickButtons();
    fn FlightStickReading();
    fn GameControllerButtonLabel();
    fn GameControllerSwitchKind();
    fn GameControllerSwitchPosition();
    fn Gamepad();
    fn GamepadButtons();
    fn GamepadReading();
    fn GamepadVibration();
    fn GamingInputPreviewContract();
    fn Headset();
    fn IArcadeStick();
    fn IArcadeStickStatics();
    fn IArcadeStickStatics2();
    fn IFlightStick();
    fn IFlightStickStatics();
    fn IGameController();
    fn IGameControllerBatteryInfo();
    fn IGamepad();
    fn IGamepad2();
    fn IGamepadStatics();
    fn IGamepadStatics2();
    fn IHeadset();
    fn IRacingWheel();
    fn IRacingWheelStatics();
    fn IRacingWheelStatics2();
    fn IRawGameController();
    fn IRawGameController2();
    fn IRawGameControllerStatics();
    fn IUINavigationController();
    fn IUINavigationControllerStatics();
    fn IUINavigationControllerStatics2();
    fn OptionalUINavigationButtons();
    fn RacingWheel();
    fn RacingWheelButtons();
    fn RacingWheelReading();
    fn RawGameController();
    fn RequiredUINavigationButtons();
    fn UINavigationController();
    fn UINavigationReading();
}
