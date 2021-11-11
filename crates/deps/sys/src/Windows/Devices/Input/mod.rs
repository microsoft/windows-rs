#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {
    fn IKeyboardCapabilities();
    fn IMouseCapabilities();
    fn IMouseDevice();
    fn IMouseDeviceStatics();
    fn IMouseEventArgs();
    fn IPenButtonListener();
    fn IPenButtonListenerStatics();
    fn IPenDevice();
    fn IPenDevice2();
    fn IPenDeviceStatics();
    fn IPenDockListener();
    fn IPenDockListenerStatics();
    fn IPenDockedEventArgs();
    fn IPenTailButtonClickedEventArgs();
    fn IPenTailButtonDoubleClickedEventArgs();
    fn IPenTailButtonLongPressedEventArgs();
    fn IPenUndockedEventArgs();
    fn IPointerDevice();
    fn IPointerDevice2();
    fn IPointerDeviceStatics();
    fn ITouchCapabilities();
    fn KeyboardCapabilities();
    fn MouseCapabilities();
    fn MouseDelta();
    fn MouseDevice();
    fn MouseEventArgs();
    fn PenButtonListener();
    fn PenDevice();
    fn PenDockListener();
    fn PenDockedEventArgs();
    fn PenTailButtonClickedEventArgs();
    fn PenTailButtonDoubleClickedEventArgs();
    fn PenTailButtonLongPressedEventArgs();
    fn PenUndockedEventArgs();
    fn PointerDevice();
    fn PointerDeviceType();
    fn PointerDeviceUsage();
    fn TouchCapabilities();
}
