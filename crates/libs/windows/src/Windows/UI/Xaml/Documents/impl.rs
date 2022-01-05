#[cfg(feature = "implement_exclusive")]
pub trait IBlockImpl: Sized {
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<f64>;
    fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy>;
    fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()>;
    fn Margin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlock2Impl: Sized {
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Block>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockStaticsImpl: Sized {
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineStackingStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBlockStatics2Impl: Sized {
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBoldImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContactContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkImpl: Sized {
    fn Info(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo>;
    fn SetInfo(&self, value: &::core::option::Option<super::super::Text::ContentLinkInfo>) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Cursor(&self) -> ::windows::core::Result<super::super::Core::CoreCursorType>;
    fn SetCursor(&self, value: super::super::Core::CoreCursorType) -> ::windows::core::Result<()>;
    fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusLeft(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusRight(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusUp(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusDown(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn IsTabStop(&self) -> ::windows::core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()>;
    fn TabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkInvokedEventArgsImpl: Sized {
    fn ContentLinkInfo(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkProviderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ContentLinkProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkStaticsImpl: Sized {
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CursorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FocusStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTabStopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsImpl: Sized {
    fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnicodeString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIndices(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetFontUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations>;
    fn SetStyleSimulations(&self, value: super::Media::StyleSimulations) -> ::windows::core::Result<()>;
    fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginX(&self) -> ::windows::core::Result<f64>;
    fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginY(&self) -> ::windows::core::Result<f64>;
    fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Fill(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphs2Impl: Sized {
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32>;
    fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsStaticsImpl: Sized {
    fn UnicodeStringProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IndicesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StyleSimulationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontRenderingEmSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OriginXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OriginYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FillProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlyphsStatics2Impl: Sized {
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorFontPaletteIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkImpl: Sized {
    fn NavigateUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetNavigateUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink2Impl: Sized {
    fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle>;
    fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink3Impl: Sized {
    fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusLeft(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusRight(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusUp(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusDown(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink4Impl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlink5Impl: Sized {
    fn IsTabStop(&self) -> ::windows::core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()>;
    fn TabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkClickEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStaticsImpl: Sized {
    fn NavigateUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics2Impl: Sized {
    fn UnderlineStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics3Impl: Sized {
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics4Impl: Sized {
    fn FocusStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkStatics5Impl: Sized {
    fn IsTabStopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Inline>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInlineUIContainerImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItalicImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILineBreakImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IParagraphImpl: Sized {
    fn Inlines(&self) -> ::windows::core::Result<InlineCollection>;
    fn TextIndent(&self) -> ::windows::core::Result<f64>;
    fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IParagraphStaticsImpl: Sized {
    fn TextIndentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceContentLinkProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRunImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection>;
    fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRunStaticsImpl: Sized {
    fn FlowDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpanImpl: Sized {
    fn Inlines(&self) -> ::windows::core::Result<InlineCollection>;
    fn SetInlines(&self, value: &::core::option::Option<InlineCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpanFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Span>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn FontStretch(&self) -> ::windows::core::Result<super::super::Text::FontStretch>;
    fn SetFontStretch(&self, value: super::super::Text::FontStretch) -> ::windows::core::Result<()>;
    fn CharacterSpacing(&self) -> ::windows::core::Result<i32>;
    fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()>;
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentStart(&self) -> ::windows::core::Result<TextPointer>;
    fn ContentEnd(&self) -> ::windows::core::Result<TextPointer>;
    fn ElementStart(&self) -> ::windows::core::Result<TextPointer>;
    fn ElementEnd(&self) -> ::windows::core::Result<TextPointer>;
    fn FindName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement2Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement3Impl: Sized {
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement4Impl: Sized {
    fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations>;
    fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()>;
    fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool>;
    fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetAccessKeyScopeOwner(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyTipPlacementMode(&self) -> ::windows::core::Result<super::Input::KeyTipPlacementMode>;
    fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> ::windows::core::Result<()>;
    fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayDismissed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayDismissed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElement5Impl: Sized {
    fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementOverridesImpl: Sized {
    fn OnDisconnectVisualChildren(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStaticsImpl: Sized {
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LanguageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics2Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics3Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExitDisplayModeOnAccessKeyInvokedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextElementStatics4Impl: Sized {
    fn TextDecorationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAccessKeyScopeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyScopeOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipPlacementModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyTipVerticalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterImpl: Sized {
    fn Ranges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TextRange>>;
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextHighlighter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextHighlighterStaticsImpl: Sized {
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPointerImpl: Sized {
    fn Parent(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement>;
    fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection>;
    fn Offset(&self) -> ::windows::core::Result<i32>;
    fn GetCharacterRect(&self, direction: LogicalDirection) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetPositionAtOffset(&self, offset: i32, direction: LogicalDirection) -> ::windows::core::Result<TextPointer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITypographyImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITypographyStaticsImpl: Sized {
    fn AnnotationAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAnnotationAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetAnnotationAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn EastAsianExpertFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianExpertForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetEastAsianExpertForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn EastAsianLanguageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianLanguage(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontEastAsianLanguage>;
    fn SetEastAsianLanguage(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontEastAsianLanguage) -> ::windows::core::Result<()>;
    fn EastAsianWidthsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetEastAsianWidths(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontEastAsianWidths>;
    fn SetEastAsianWidths(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontEastAsianWidths) -> ::windows::core::Result<()>;
    fn StandardLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStandardLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStandardLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn ContextualLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetContextualLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn DiscretionaryLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDiscretionaryLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetDiscretionaryLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn HistoricalLigaturesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHistoricalLigatures(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetHistoricalLigatures(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StandardSwashesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStandardSwashes(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetStandardSwashes(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn ContextualSwashesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualSwashes(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetContextualSwashes(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn ContextualAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetContextualAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetContextualAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticAlternatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticAlternates(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetStylisticAlternates(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn StylisticSet1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet1(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet1(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet2(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet2(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet3Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet3(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet3(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet4Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet4(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet4(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet5Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet5(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet5(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet6Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet6(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet6(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet7Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet7(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet7(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet8Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet8(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet8(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet9Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet9(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet9(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet10Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet10(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet10(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet11Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet11(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet11(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet12Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet12(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet12(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet13Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet13(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet13(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet14Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet14(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet14(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet15Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet15(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet15(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet16Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet16(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet16(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet17Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet17(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet17(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet18Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet18(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet18(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet19Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet19(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet19(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn StylisticSet20Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetStylisticSet20(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetStylisticSet20(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn CapitalsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCapitals(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontCapitals>;
    fn SetCapitals(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontCapitals) -> ::windows::core::Result<()>;
    fn CapitalSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCapitalSpacing(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetCapitalSpacing(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn KerningProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetKerning(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetKerning(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn CaseSensitiveFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCaseSensitiveForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetCaseSensitiveForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn HistoricalFormsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHistoricalForms(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetHistoricalForms(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn FractionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFraction(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontFraction>;
    fn SetFraction(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontFraction) -> ::windows::core::Result<()>;
    fn NumeralStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetNumeralStyle(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontNumeralStyle>;
    fn SetNumeralStyle(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontNumeralStyle) -> ::windows::core::Result<()>;
    fn NumeralAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetNumeralAlignment(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontNumeralAlignment>;
    fn SetNumeralAlignment(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontNumeralAlignment) -> ::windows::core::Result<()>;
    fn SlashedZeroProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSlashedZero(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetSlashedZero(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn MathematicalGreekProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetMathematicalGreek(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetMathematicalGreek(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn VariantsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetVariants(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::FontVariants>;
    fn SetVariants(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FontVariants) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnderlineImpl: Sized {}
