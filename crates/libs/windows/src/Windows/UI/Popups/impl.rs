#[cfg(feature = "implement_exclusive")]
pub trait IMessageDialogImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>>;
    fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn CancelCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContent(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>>;
    fn Options(&self) -> ::windows::core::Result<MessageDialogOptions>;
    fn SetOptions(&self, value: MessageDialogOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageDialogFactoryImpl: Sized {
    fn Create(&self, content: &::windows::core::HSTRING) -> ::windows::core::Result<MessageDialog>;
    fn CreateWithTitle(&self, content: &::windows::core::HSTRING, title: &::windows::core::HSTRING) -> ::windows::core::Result<MessageDialog>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupMenuImpl: Sized {
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>>;
    fn ShowAsync(&self, invocationpoint: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>>;
    fn ShowAsyncWithRect(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>>;
    fn ShowAsyncWithRectAndPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>>;
}
pub trait IUICommandImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler>;
    fn SetInvoked(&self, value: &::core::option::Option<UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetId(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUICommandFactoryImpl: Sized {
    fn Create(&self, label: &::windows::core::HSTRING) -> ::windows::core::Result<UICommand>;
    fn CreateWithHandler(&self, label: &::windows::core::HSTRING, action: &::core::option::Option<UICommandInvokedHandler>) -> ::windows::core::Result<UICommand>;
    fn CreateWithHandlerAndId(&self, label: &::windows::core::HSTRING, action: &::core::option::Option<UICommandInvokedHandler>, commandid: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<UICommand>;
}
