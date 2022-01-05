#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputGamepadInfoImpl: Sized {
    fn Buttons(&self) -> ::windows::core::Result<super::super::super::super::Gaming::Input::GamepadButtons>;
    fn SetButtons(&self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::Result<()>;
    fn LeftThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickX(&self, value: f64) -> ::windows::core::Result<()>;
    fn LeftThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn SetLeftThumbstickY(&self, value: f64) -> ::windows::core::Result<()>;
    fn LeftTrigger(&self) -> ::windows::core::Result<f64>;
    fn SetLeftTrigger(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn SetRightThumbstickY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RightTrigger(&self) -> ::windows::core::Result<f64>;
    fn SetRightTrigger(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputGamepadInfoFactoryImpl: Sized {
    fn CreateInstanceFromGamepadReading(&self, reading: &super::super::super::super::Gaming::Input::GamepadReading) -> ::windows::core::Result<InjectedInputGamepadInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputKeyboardInfoImpl: Sized {
    fn KeyOptions(&self) -> ::windows::core::Result<InjectedInputKeyOptions>;
    fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> ::windows::core::Result<()>;
    fn ScanCode(&self) -> ::windows::core::Result<u16>;
    fn SetScanCode(&self, value: u16) -> ::windows::core::Result<()>;
    fn VirtualKey(&self) -> ::windows::core::Result<u16>;
    fn SetVirtualKey(&self, value: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputMouseInfoImpl: Sized {
    fn MouseOptions(&self) -> ::windows::core::Result<InjectedInputMouseOptions>;
    fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> ::windows::core::Result<()>;
    fn MouseData(&self) -> ::windows::core::Result<u32>;
    fn SetMouseData(&self, value: u32) -> ::windows::core::Result<()>;
    fn DeltaY(&self) -> ::windows::core::Result<i32>;
    fn SetDeltaY(&self, value: i32) -> ::windows::core::Result<()>;
    fn DeltaX(&self) -> ::windows::core::Result<i32>;
    fn SetDeltaX(&self, value: i32) -> ::windows::core::Result<()>;
    fn TimeOffsetInMilliseconds(&self) -> ::windows::core::Result<u32>;
    fn SetTimeOffsetInMilliseconds(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputPenInfoImpl: Sized {
    fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn PenButtons(&self) -> ::windows::core::Result<InjectedInputPenButtons>;
    fn SetPenButtons(&self, value: InjectedInputPenButtons) -> ::windows::core::Result<()>;
    fn PenParameters(&self) -> ::windows::core::Result<InjectedInputPenParameters>;
    fn SetPenParameters(&self, value: InjectedInputPenParameters) -> ::windows::core::Result<()>;
    fn Pressure(&self) -> ::windows::core::Result<f64>;
    fn SetPressure(&self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f64>;
    fn SetRotation(&self, value: f64) -> ::windows::core::Result<()>;
    fn TiltX(&self) -> ::windows::core::Result<i32>;
    fn SetTiltX(&self, value: i32) -> ::windows::core::Result<()>;
    fn TiltY(&self) -> ::windows::core::Result<i32>;
    fn SetTiltY(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInjectedInputTouchInfoImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<InjectedInputRectangle>;
    fn SetContact(&self, value: &InjectedInputRectangle) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<i32>;
    fn SetOrientation(&self, value: i32) -> ::windows::core::Result<()>;
    fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo>;
    fn SetPointerInfo(&self, value: &InjectedInputPointerInfo) -> ::windows::core::Result<()>;
    fn Pressure(&self) -> ::windows::core::Result<f64>;
    fn SetPressure(&self, value: f64) -> ::windows::core::Result<()>;
    fn TouchParameters(&self) -> ::windows::core::Result<InjectedInputTouchParameters>;
    fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorImpl: Sized {
    fn InjectKeyboardInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>) -> ::windows::core::Result<()>;
    fn InjectMouseInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>) -> ::windows::core::Result<()>;
    fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectTouchInput(&self, input: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>) -> ::windows::core::Result<()>;
    fn UninitializeTouchInjection(&self) -> ::windows::core::Result<()>;
    fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()>;
    fn InjectPenInput(&self, input: &::core::option::Option<InjectedInputPenInfo>) -> ::windows::core::Result<()>;
    fn UninitializePenInjection(&self) -> ::windows::core::Result<()>;
    fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjector2Impl: Sized + IInputInjectorImpl {
    fn InitializeGamepadInjection(&self) -> ::windows::core::Result<()>;
    fn InjectGamepadInput(&self, input: &::core::option::Option<InjectedInputGamepadInfo>) -> ::windows::core::Result<()>;
    fn UninitializeGamepadInjection(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStaticsImpl: Sized {
    fn TryCreate(&self) -> ::windows::core::Result<InputInjector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputInjectorStatics2Impl: Sized + IInputInjectorStaticsImpl {
    fn TryCreateForAppBroadcastOnly(&self) -> ::windows::core::Result<InputInjector>;
}
