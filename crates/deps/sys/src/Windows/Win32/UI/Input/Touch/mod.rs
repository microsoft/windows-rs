#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CloseGestureInfoHandle();
    fn CloseTouchInputHandle();
    fn GetGestureConfig();
    fn GetGestureExtraArgs();
    fn GetGestureInfo();
    fn GetTouchInputInfo();
    fn IsTouchWindow();
    fn RegisterTouchWindow();
    fn SetGestureConfig();
    fn UnregisterTouchWindow();
}
