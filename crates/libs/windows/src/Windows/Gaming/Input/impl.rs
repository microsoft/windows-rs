#[cfg(feature = "implement_exclusive")]
pub trait IArcadeStickImpl: Sized + IGameControllerImpl {
    fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<ArcadeStickReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcadeStickStaticsImpl: Sized {
    fn ArcadeStickAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeStickRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeSticks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcadeStickStatics2Impl: Sized + IArcadeStickStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<ArcadeStick>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlightStickImpl: Sized + IGameControllerImpl {
    fn HatSwitchKind(&self) -> ::windows::core::Result<GameControllerSwitchKind>;
    fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<FlightStickReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlightStickStaticsImpl: Sized {
    fn FlightStickAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightStickRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightSticks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>>;
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<FlightStick>;
}
pub trait IGameControllerImpl: Sized {
    fn HeadsetConnected(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetConnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadsetDisconnected(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Headset(&self) -> ::windows::core::Result<Headset>;
    fn IsWireless(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
pub trait IGameControllerBatteryInfoImpl: Sized {
    fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGamepadImpl: Sized + IGameControllerImpl {
    fn Vibration(&self) -> ::windows::core::Result<GamepadVibration>;
    fn SetVibration(&self, value: &GamepadVibration) -> ::windows::core::Result<()>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<GamepadReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGamepad2Impl: Sized + IGameControllerImpl + IGamepadImpl {
    fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGamepadStaticsImpl: Sized {
    fn GamepadAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GamepadRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Gamepads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGamepadStatics2Impl: Sized + IGamepadStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<Gamepad>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHeadsetImpl: Sized {
    fn CaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRacingWheelImpl: Sized + IGameControllerImpl {
    fn HasClutch(&self) -> ::windows::core::Result<bool>;
    fn HasHandbrake(&self) -> ::windows::core::Result<bool>;
    fn HasPatternShifter(&self) -> ::windows::core::Result<bool>;
    fn MaxPatternShifterGear(&self) -> ::windows::core::Result<i32>;
    fn MaxWheelAngle(&self) -> ::windows::core::Result<f64>;
    fn WheelMotor(&self) -> ::windows::core::Result<ForceFeedback::ForceFeedbackMotor>;
    fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<RacingWheelReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRacingWheelStaticsImpl: Sized {
    fn RacingWheelAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheelRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheels(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRacingWheelStatics2Impl: Sized + IRacingWheelStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RacingWheel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawGameControllerImpl: Sized + IGameControllerImpl {
    fn AxisCount(&self) -> ::windows::core::Result<i32>;
    fn ButtonCount(&self) -> ::windows::core::Result<i32>;
    fn ForceFeedbackMotors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn SwitchCount(&self) -> ::windows::core::Result<i32>;
    fn GetButtonLabel(&self, buttonindex: i32) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self, buttonarray: &mut [<bool as ::windows::core::DefaultType>::DefaultType], switcharray: &mut [<GameControllerSwitchPosition as ::windows::core::DefaultType>::DefaultType], axisarray: &mut [<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u64>;
    fn GetSwitchKind(&self, switchindex: i32) -> ::windows::core::Result<GameControllerSwitchKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawGameController2Impl: Sized + IGameControllerImpl + IRawGameControllerImpl {
    fn SimpleHapticsControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>;
    fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawGameControllerStaticsImpl: Sized {
    fn RawGameControllerAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllerRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>>;
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RawGameController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUINavigationControllerImpl: Sized + IGameControllerImpl {
    fn GetCurrentReading(&self) -> ::windows::core::Result<UINavigationReading>;
    fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUINavigationControllerStaticsImpl: Sized {
    fn UINavigationControllerAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllerRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUINavigationControllerStatics2Impl: Sized + IUINavigationControllerStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<UINavigationController>;
}
