#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CloseGestureInfoHandle();
    fn CloseTouchInputHandle();
    fn GESTURECONFIG();
    fn GESTURECONFIG_ID();
    fn GESTUREINFO();
    fn GESTURENOTIFYSTRUCT();
    fn GetGestureConfig();
    fn GetGestureExtraArgs();
    fn GetGestureInfo();
    fn GetTouchInputInfo();
    fn HGESTUREINFO();
    fn HTOUCHINPUT();
    fn IInertiaProcessor();
    fn IManipulationProcessor();
    fn InertiaProcessor();
    fn IsTouchWindow();
    fn MANIPULATION_PROCESSOR_MANIPULATIONS();
    fn ManipulationProcessor();
    fn REGISTER_TOUCH_WINDOW_FLAGS();
    fn RegisterTouchWindow();
    fn SetGestureConfig();
    fn TOUCHEVENTF_FLAGS();
    fn TOUCHINPUT();
    fn TOUCHINPUTMASKF_MASK();
    fn UnregisterTouchWindow();
    fn _IManipulationEvents();
}
