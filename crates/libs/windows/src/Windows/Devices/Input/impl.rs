#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardCapabilitiesImpl: Sized {
    fn KeyboardPresent(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseCapabilitiesImpl: Sized {
    fn MousePresent(&self) -> ::windows::core::Result<i32>;
    fn VerticalWheelPresent(&self) -> ::windows::core::Result<i32>;
    fn HorizontalWheelPresent(&self) -> ::windows::core::Result<i32>;
    fn SwapButtons(&self) -> ::windows::core::Result<i32>;
    fn NumberOfButtons(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseDeviceImpl: Sized {
    fn MouseMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMouseMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseDeviceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<MouseDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseEventArgsImpl: Sized {
    fn MouseDelta(&self) -> ::windows::core::Result<MouseDelta>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenButtonListenerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonDoubleClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonDoubleClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TailButtonLongPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTailButtonLongPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenButtonListenerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenButtonListener>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDeviceImpl: Sized {
    fn PenId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDevice2Impl: Sized {
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDeviceStaticsImpl: Sized {
    fn GetFromPointerId(&self, pointerid: u32) -> ::windows::core::Result<PenDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockListenerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Docked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDocked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Undocked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUndocked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockListenerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenDockListener>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenDockedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonClickedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonDoubleClickedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPenTailButtonLongPressedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPenUndockedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDeviceImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType>;
    fn IsIntegrated(&self) -> ::windows::core::Result<bool>;
    fn MaxContacts(&self) -> ::windows::core::Result<u32>;
    fn PhysicalDeviceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ScreenRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SupportedUsages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDevice2Impl: Sized {
    fn MaxPointersWithZDistance(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDeviceStaticsImpl: Sized {
    fn GetPointerDevice(&self, pointerid: u32) -> ::windows::core::Result<PointerDevice>;
    fn GetPointerDevices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITouchCapabilitiesImpl: Sized {
    fn TouchPresent(&self) -> ::windows::core::Result<i32>;
    fn Contacts(&self) -> ::windows::core::Result<u32>;
}
