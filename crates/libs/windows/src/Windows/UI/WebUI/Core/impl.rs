#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode>;
    fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>;
    fn MenuOpened(&self, handler: &::core::option::Option<MenuOpenedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuOpened(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MenuClosed(&self, handler: &::core::option::Option<MenuClosedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMenuClosed(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, value: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarBitmapIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarBitmapIconFactoryImpl: Sized {
    fn Create(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebUICommandBarBitmapIcon>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarConfirmationButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IWebUICommandBarElementImpl: Sized {}
pub trait IWebUICommandBarIconImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarIconButtonImpl: Sized + IWebUICommandBarElementImpl {
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsToggleButton(&self) -> ::windows::core::Result<bool>;
    fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon>;
    fn SetIcon(&self, value: &::core::option::Option<IWebUICommandBarIcon>) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarItemInvokedEventArgsImpl: Sized {
    fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSizeChangedEventArgsImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<WebUICommandBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconImpl: Sized + IWebUICommandBarIconImpl {
    fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSymbol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUICommandBarSymbolIconFactoryImpl: Sized {
    fn Create(&self, symbol: &::windows::core::HSTRING) -> ::windows::core::Result<WebUICommandBarSymbolIcon>;
}
