#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionCompletedEventArgsImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn CompositionSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionSegmentImpl: Sized {
    fn PreconversionString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextCompositionStartedEventArgsImpl: Sized {
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextEditContextImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InputScope(&self) -> ::windows::core::Result<CoreTextInputScope>;
    fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::core::Result<()>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputPaneDisplayPolicy(&self) -> ::windows::core::Result<CoreTextInputPaneDisplayPolicy>;
    fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::core::Result<()>;
    fn TextRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutRequested(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FormatUpdating(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatUpdating(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionStarted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompositionCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionCompleted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FocusRemoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusRemoved(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyFocusEnter(&self) -> ::windows::core::Result<()>;
    fn NotifyFocusLeave(&self) -> ::windows::core::Result<()>;
    fn NotifyTextChanged(&self, modifiedrange: &CoreTextRange, newlength: i32, newselection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifySelectionChanged(&self, selection: &CoreTextRange) -> ::windows::core::Result<()>;
    fn NotifyLayoutChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextEditContext2Impl: Sized {
    fn NotifyFocusLeaveCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotifyFocusLeaveCompleted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextFormatUpdatingEventArgsImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn TextColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>;
    fn UnderlineType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::UnderlineType>>;
    fn Reason(&self) -> ::windows::core::Result<CoreTextFormatUpdatingReason>;
    fn Result(&self) -> ::windows::core::Result<CoreTextFormatUpdatingResult>;
    fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutBoundsImpl: Sized {
    fn TextBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetTextBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ControlBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetControlBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequestImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn LayoutBounds(&self) -> ::windows::core::Result<CoreTextLayoutBounds>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequest2Impl: Sized {
    fn LayoutBoundsVisualPixels(&self) -> ::windows::core::Result<CoreTextLayoutBounds>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextLayoutRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextLayoutRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionRequestImpl: Sized {
    fn Selection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn SetSelection(&self, value: &CoreTextRange) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextSelectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextSelectionUpdatingEventArgsImpl: Sized {
    fn Selection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Result(&self) -> ::windows::core::Result<CoreTextSelectionUpdatingResult>;
    fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesManagerImpl: Sized {
    fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn InputLanguageChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputLanguageChanged(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEditContext(&self) -> ::windows::core::Result<CoreTextEditContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreTextServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextServicesStaticsImpl: Sized {
    fn HiddenCharacter(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextRequestImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<CoreTextTextRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTextTextUpdatingEventArgsImpl: Sized {
    fn Range(&self) -> ::windows::core::Result<CoreTextRange>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewSelection(&self) -> ::windows::core::Result<CoreTextRange>;
    fn InputLanguage(&self) -> ::windows::core::Result<super::super::super::Globalization::Language>;
    fn Result(&self) -> ::windows::core::Result<CoreTextTextUpdatingResult>;
    fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::core::Result<()>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
