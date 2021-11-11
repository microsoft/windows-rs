#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EnableMouseInPointer();
    fn GetPointerCursorId();
    fn GetPointerDevice();
    fn GetPointerDeviceCursors();
    fn GetPointerDeviceProperties();
    fn GetPointerDeviceRects();
    fn GetPointerDevices();
    fn GetPointerFrameInfo();
    fn GetPointerFrameInfoHistory();
    fn GetPointerFramePenInfo();
    fn GetPointerFramePenInfoHistory();
    fn GetPointerFrameTouchInfo();
    fn GetPointerFrameTouchInfoHistory();
    fn GetPointerInfo();
    fn GetPointerInfoHistory();
    fn GetPointerInputTransform();
    fn GetPointerPenInfo();
    fn GetPointerPenInfoHistory();
    fn GetPointerTouchInfo();
    fn GetPointerTouchInfoHistory();
    fn GetPointerType();
    fn GetRawPointerDeviceData();
    fn GetUnpredictedMessagePos();
    fn InitializeTouchInjection();
    fn InjectSyntheticPointerInput();
    fn InjectTouchInput();
    fn IsMouseInPointerEnabled();
    fn SkipPointerFrameMessages();
}
