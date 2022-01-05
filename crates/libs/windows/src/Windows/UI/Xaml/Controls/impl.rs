#[cfg(feature = "implement_exclusive")]
pub trait IAnchorRequestedEventArgsImpl: Sized {
    fn Anchor(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetAnchor(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AnchorCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarImpl: Sized {
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSticky(&self) -> ::windows::core::Result<bool>;
    fn SetIsSticky(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBar2Impl: Sized {
    fn ClosedDisplayMode(&self) -> ::windows::core::Result<AppBarClosedDisplayMode>;
    fn SetClosedDisplayMode(&self, value: AppBarClosedDisplayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBar3Impl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::AppBarTemplateSettings>;
    fn Opening(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBar4Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButton3Impl: Sized {
    fn LabelPosition(&self) -> ::windows::core::Result<CommandBarLabelPosition>;
    fn SetLabelPosition(&self, value: CommandBarLabelPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButton4Impl: Sized {
    fn KeyboardAcceleratorTextOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyboardAcceleratorTextOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButton5Impl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::AppBarButtonTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonStaticsImpl: Sized {
    fn LabelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCompactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonStatics3Impl: Sized {
    fn LabelPositionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsInOverflowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DynamicOverflowOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonStatics4Impl: Sized {
    fn KeyboardAcceleratorTextOverrideProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarElementContainerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarElementContainerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarElementContainer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarElementContainerStaticsImpl: Sized {
    fn IsCompactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsInOverflowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DynamicOverflowOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarOverridesImpl: Sized {
    fn OnClosed(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOpened(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarOverrides3Impl: Sized {
    fn OnClosing(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOpening(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarSeparatorImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarSeparatorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarSeparator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarSeparatorStaticsImpl: Sized {
    fn IsCompactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarSeparatorStatics3Impl: Sized {
    fn IsInOverflowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DynamicOverflowOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarStaticsImpl: Sized {
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsStickyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarStatics2Impl: Sized {
    fn ClosedDisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarStatics4Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButton3Impl: Sized {
    fn LabelPosition(&self) -> ::windows::core::Result<CommandBarLabelPosition>;
    fn SetLabelPosition(&self, value: CommandBarLabelPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButton4Impl: Sized {
    fn KeyboardAcceleratorTextOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyboardAcceleratorTextOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButton5Impl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::AppBarToggleButtonTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonStaticsImpl: Sized {
    fn LabelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCompactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonStatics3Impl: Sized {
    fn LabelPositionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsInOverflowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DynamicOverflowOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonStatics4Impl: Sized {
    fn KeyboardAcceleratorTextOverrideProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxImpl: Sized {
    fn MaxSuggestionListHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMaxSuggestionListHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsSuggestionListOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSuggestionListOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextMemberPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextMemberPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateTextOnSelect(&self) -> ::windows::core::Result<bool>;
    fn SetUpdateTextOnSelect(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AutoMaximizeSuggestionArea(&self) -> ::windows::core::Result<bool>;
    fn SetAutoMaximizeSuggestionArea(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextBoxStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetTextBoxStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn SuggestionChosen(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AutoSuggestBox, AutoSuggestBoxSuggestionChosenEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuggestionChosen(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AutoSuggestBox, AutoSuggestBoxTextChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBox2Impl: Sized {
    fn QueryIcon(&self) -> ::windows::core::Result<IconElement>;
    fn SetQueryIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
    fn QuerySubmitted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AutoSuggestBox, AutoSuggestBoxQuerySubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuerySubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBox3Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBox4Impl: Sized {
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxQuerySubmittedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChosenSuggestion(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxStaticsImpl: Sized {
    fn MaxSuggestionListHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSuggestionListOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextMemberPathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn UpdateTextOnSelectProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AutoMaximizeSuggestionAreaProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextBoxStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxStatics2Impl: Sized {
    fn QueryIconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxStatics3Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxStatics4Impl: Sized {
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxSuggestionChosenEventArgsImpl: Sized {
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxTextChangedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AutoSuggestionBoxTextChangeReason>;
    fn SetReason(&self, value: AutoSuggestionBoxTextChangeReason) -> ::windows::core::Result<()>;
    fn CheckCurrent(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxTextChangedEventArgsStaticsImpl: Sized {
    fn ReasonProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackClickEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconImpl: Sized {
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIcon2Impl: Sized {
    fn ShowAsMonochrome(&self) -> ::windows::core::Result<bool>;
    fn SetShowAsMonochrome(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapIcon>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconSourceImpl: Sized {
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ShowAsMonochrome(&self) -> ::windows::core::Result<bool>;
    fn SetShowAsMonochrome(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapIconSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconSourceStaticsImpl: Sized {
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ShowAsMonochromeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconStaticsImpl: Sized {
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapIconStatics2Impl: Sized {
    fn ShowAsMonochromeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBorderImpl: Sized {
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn Child(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ChildTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetChildTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBorder2Impl: Sized {
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
    fn BackgroundTransition(&self) -> ::windows::core::Result<super::BrushTransition>;
    fn SetBackgroundTransition(&self, value: &::core::option::Option<super::BrushTransition>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBorderStaticsImpl: Sized {
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBorderStatics2Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Button>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonStaticsWithFlyoutImpl: Sized {
    fn FlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonWithFlyoutImpl: Sized {
    fn Flyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerImpl: Sized {
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IsCalendarOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsCalendarOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn DateFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDateFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn CalendarViewStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetCalendarViewStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn MinDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMinDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MaxDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMaxDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsTodayHighlighted(&self) -> ::windows::core::Result<bool>;
    fn SetIsTodayHighlighted(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayMode(&self) -> ::windows::core::Result<CalendarViewDisplayMode>;
    fn SetDisplayMode(&self, value: CalendarViewDisplayMode) -> ::windows::core::Result<()>;
    fn FirstDayOfWeek(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek>;
    fn SetFirstDayOfWeek(&self, value: super::super::super::Globalization::DayOfWeek) -> ::windows::core::Result<()>;
    fn DayOfWeekFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDayOfWeekFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCalendarIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsOutOfScopeEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutOfScopeEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsGroupLabelVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGroupLabelVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn CalendarViewDayItemChanging(&self, handler: &::core::option::Option<CalendarViewDayItemChangingEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCalendarViewDayItemChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CalendarDatePicker, CalendarDatePickerDateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDisplayDate(&self, date: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetYearDecadeDisplayDimensions(&self, columns: i32, rows: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePicker2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePicker3Impl: Sized {
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerDateChangedEventArgsImpl: Sized {
    fn NewDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn OldDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarDatePicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerStaticsImpl: Sized {
    fn DateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCalendarOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DateFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarViewStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinDateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxDateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTodayHighlightedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstDayOfWeekProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayOfWeekFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsOutOfScopeEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsGroupLabelVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerStatics3Impl: Sized {
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewImpl: Sized {
    fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCalendarIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DayOfWeekFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDayOfWeekFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsGroupLabelVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGroupLabelVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayMode(&self) -> ::windows::core::Result<CalendarViewDisplayMode>;
    fn SetDisplayMode(&self, value: CalendarViewDisplayMode) -> ::windows::core::Result<()>;
    fn FirstDayOfWeek(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek>;
    fn SetFirstDayOfWeek(&self, value: super::super::super::Globalization::DayOfWeek) -> ::windows::core::Result<()>;
    fn IsOutOfScopeEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutOfScopeEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTodayHighlighted(&self) -> ::windows::core::Result<bool>;
    fn SetIsTodayHighlighted(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMaxDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MinDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMinDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn NumberOfWeeksInView(&self) -> ::windows::core::Result<i32>;
    fn SetNumberOfWeeksInView(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedDates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::DateTime>>;
    fn SelectionMode(&self) -> ::windows::core::Result<CalendarViewSelectionMode>;
    fn SetSelectionMode(&self, value: CalendarViewSelectionMode) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::CalendarViewTemplateSettings>;
    fn FocusBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetFocusBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedHoverBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedHoverBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPressedBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedPressedBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn HoverBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetHoverBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PressedBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetPressedBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn OutOfScopeBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetOutOfScopeBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PressedForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetPressedForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BlackoutForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBlackoutForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn OutOfScopeForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetOutOfScopeForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DayItemFontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetDayItemFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn DayItemFontSize(&self) -> ::windows::core::Result<f64>;
    fn SetDayItemFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn DayItemFontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetDayItemFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn DayItemFontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetDayItemFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn TodayFontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetTodayFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FirstOfMonthLabelFontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFirstOfMonthLabelFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FirstOfMonthLabelFontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFirstOfMonthLabelFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FirstOfMonthLabelFontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFirstOfMonthLabelFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn FirstOfMonthLabelFontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFirstOfMonthLabelFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn MonthYearItemFontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetMonthYearItemFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn MonthYearItemFontSize(&self) -> ::windows::core::Result<f64>;
    fn SetMonthYearItemFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn MonthYearItemFontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetMonthYearItemFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn MonthYearItemFontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetMonthYearItemFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FirstOfYearDecadeLabelFontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFirstOfYearDecadeLabelFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FirstOfYearDecadeLabelFontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFirstOfYearDecadeLabelFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FirstOfYearDecadeLabelFontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFirstOfYearDecadeLabelFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn FirstOfYearDecadeLabelFontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFirstOfYearDecadeLabelFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn HorizontalDayItemAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalDayItemAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalDayItemAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalDayItemAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn HorizontalFirstOfMonthLabelAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalFirstOfMonthLabelAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalFirstOfMonthLabelAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalFirstOfMonthLabelAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn CalendarItemBorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetCalendarItemBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CalendarViewDayItemStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetCalendarViewDayItemStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn CalendarViewDayItemChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CalendarView, CalendarViewDayItemChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCalendarViewDayItemChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectedDatesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CalendarView, CalendarViewSelectedDatesChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedDatesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDisplayDate(&self, date: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetYearDecadeDisplayDimensions(&self, columns: i32, rows: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarView2Impl: Sized {
    fn SelectedDisabledBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedDisabledBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodaySelectedInnerBorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodaySelectedInnerBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BlackoutStrikethroughBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBlackoutStrikethroughBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BlackoutBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBlackoutBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemHoverBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemHoverBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemPressedBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemPressedBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CalendarItemDisabledBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetCalendarItemDisabledBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayBlackoutBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayBlackoutBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayHoverBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayHoverBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayPressedBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayPressedBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayDisabledBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayDisabledBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TodayBlackoutForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetTodayBlackoutForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedHoverForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedHoverForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPressedForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedPressedForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedDisabledForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetSelectedDisabledForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn OutOfScopeHoverForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetOutOfScopeHoverForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn OutOfScopePressedForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetOutOfScopePressedForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DisabledForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetDisabledForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DayItemMargin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetDayItemMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn MonthYearItemMargin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetMonthYearItemMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn FirstOfMonthLabelMargin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetFirstOfMonthLabelMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn FirstOfYearDecadeLabelMargin(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetFirstOfYearDecadeLabelMargin(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CalendarItemCornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCalendarItemCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewDayItemImpl: Sized {
    fn IsBlackout(&self) -> ::windows::core::Result<bool>;
    fn SetIsBlackout(&self, value: bool) -> ::windows::core::Result<()>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetDensityColors(&self, colors: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::Color>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewDayItemChangingEventArgsImpl: Sized {
    fn InRecycleQueue(&self) -> ::windows::core::Result<bool>;
    fn Item(&self) -> ::windows::core::Result<CalendarViewDayItem>;
    fn Phase(&self) -> ::windows::core::Result<u32>;
    fn RegisterUpdateCallback(&self, callback: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CalendarView, CalendarViewDayItemChangingEventArgs>>) -> ::windows::core::Result<()>;
    fn RegisterUpdateCallbackWithPhase(&self, callbackphase: u32, callback: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CalendarView, CalendarViewDayItemChangingEventArgs>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewDayItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarViewDayItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewDayItemStaticsImpl: Sized {
    fn IsBlackoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewSelectedDatesChangedEventArgsImpl: Sized {
    fn AddedDates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::DateTime>>;
    fn RemovedDates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewStaticsImpl: Sized {
    fn CalendarIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayOfWeekFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsGroupLabelVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstDayOfWeekProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsOutOfScopeEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTodayHighlightedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxDateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinDateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn NumberOfWeeksInViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedDatesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TemplateSettingsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FocusBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedHoverBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedPressedBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HoverBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PressedBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OutOfScopeBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PressedForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BlackoutForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OutOfScopeForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayItemFontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayItemFontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayItemFontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayItemFontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayFontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfMonthLabelFontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfMonthLabelFontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfMonthLabelFontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfMonthLabelFontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthYearItemFontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthYearItemFontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthYearItemFontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthYearItemFontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfYearDecadeLabelFontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfYearDecadeLabelFontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfYearDecadeLabelFontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfYearDecadeLabelFontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalDayItemAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalDayItemAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalFirstOfMonthLabelAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalFirstOfMonthLabelAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemBorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarViewDayItemStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewStatics2Impl: Sized {
    fn SelectedDisabledBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodaySelectedInnerBorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BlackoutStrikethroughBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BlackoutBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemHoverBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemPressedBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemDisabledBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayBlackoutBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayHoverBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayPressedBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayDisabledBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TodayBlackoutForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedHoverForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedPressedForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedDisabledForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OutOfScopeHoverForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OutOfScopePressedForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisabledForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayItemMarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthYearItemMarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfMonthLabelMarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FirstOfYearDecadeLabelMarginProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarItemCornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICandidateWindowBoundsChangedEventArgsImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICanvasImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICanvasFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Canvas>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICanvasStaticsImpl: Sized {
    fn LeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLeft(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<f64>;
    fn SetLeft(&self, element: &::core::option::Option<super::UIElement>, length: f64) -> ::windows::core::Result<()>;
    fn TopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetTop(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<f64>;
    fn SetTop(&self, element: &::core::option::Option<super::UIElement>, length: f64) -> ::windows::core::Result<()>;
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetZIndex(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, element: &::core::option::Option<super::UIElement>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::super::Media::Capture::MediaCapture>;
    fn SetSource(&self, value: &::core::option::Option<super::super::super::Media::Capture::MediaCapture>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementStaticsImpl: Sized {
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CheckBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChoosingGroupHeaderContainerEventArgsImpl: Sized {
    fn GroupHeaderContainer(&self) -> ::windows::core::Result<ListViewBaseHeaderItem>;
    fn SetGroupHeaderContainer(&self, value: &::core::option::Option<ListViewBaseHeaderItem>) -> ::windows::core::Result<()>;
    fn GroupIndex(&self) -> ::windows::core::Result<i32>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChoosingItemContainerEventArgsImpl: Sized {
    fn ItemIndex(&self) -> ::windows::core::Result<i32>;
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ItemContainer(&self) -> ::windows::core::Result<Primitives::SelectorItem>;
    fn SetItemContainer(&self, value: &::core::option::Option<Primitives::SelectorItem>) -> ::windows::core::Result<()>;
    fn IsContainerPrepared(&self) -> ::windows::core::Result<bool>;
    fn SetIsContainerPrepared(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICleanUpVirtualizedItemEventArgsImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn UIElement(&self) -> ::windows::core::Result<super::UIElement>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorChangedEventArgsImpl: Sized {
    fn OldColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn NewColor(&self) -> ::windows::core::Result<super::super::Color>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn PreviousColor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::Color>>;
    fn SetPreviousColor(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::Color>>) -> ::windows::core::Result<()>;
    fn IsAlphaEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAlphaEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorSpectrumVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorSpectrumVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorPreviewVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorPreviewVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorSliderVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorSliderVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAlphaSliderVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsAlphaSliderVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMoreButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsMoreButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorChannelTextInputVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorChannelTextInputVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAlphaTextInputVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsAlphaTextInputVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHexInputVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsHexInputVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinHue(&self) -> ::windows::core::Result<i32>;
    fn SetMinHue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxHue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxHue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinSaturation(&self) -> ::windows::core::Result<i32>;
    fn SetMinSaturation(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxSaturation(&self) -> ::windows::core::Result<i32>;
    fn SetMaxSaturation(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn SetMinValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn ColorSpectrumShape(&self) -> ::windows::core::Result<ColorSpectrumShape>;
    fn SetColorSpectrumShape(&self, value: ColorSpectrumShape) -> ::windows::core::Result<()>;
    fn ColorSpectrumComponents(&self) -> ::windows::core::Result<ColorSpectrumComponents>;
    fn SetColorSpectrumComponents(&self, value: ColorSpectrumComponents) -> ::windows::core::Result<()>;
    fn ColorChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ColorPicker, ColorChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PreviousColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAlphaEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorSpectrumVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorPreviewVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorSliderVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAlphaSliderVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsMoreButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorChannelTextInputVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAlphaTextInputVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHexInputVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinHueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxHueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinSaturationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxSaturationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorSpectrumShapeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorSpectrumComponentsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColumnDefinitionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<super::GridLength>;
    fn SetWidth(&self, value: &super::GridLength) -> ::windows::core::Result<()>;
    fn MaxWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn ActualWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColumnDefinitionStaticsImpl: Sized {
    fn WidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxImpl: Sized {
    fn IsDropDownOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsDropDownOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEditable(&self) -> ::windows::core::Result<bool>;
    fn IsSelectionBoxHighlighted(&self) -> ::windows::core::Result<bool>;
    fn MaxDropDownHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMaxDropDownHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn SelectionBoxItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SelectionBoxItemTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ComboBoxTemplateSettings>;
    fn DropDownClosed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDropDownClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DropDownOpened(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDropDownOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBox2Impl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBox3Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
    fn IsTextSearchEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextSearchEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBox4Impl: Sized {
    fn SelectionChangedTrigger(&self) -> ::windows::core::Result<ComboBoxSelectionChangedTrigger>;
    fn SetSelectionChangedTrigger(&self, value: ComboBoxSelectionChangedTrigger) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBox5Impl: Sized {
    fn PlaceholderForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetPlaceholderForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBox6Impl: Sized {
    fn SetIsEditable(&self, value: bool) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextBoxStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetTextBoxStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TextSubmitted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ComboBox, ComboBoxTextSubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextSubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxOverridesImpl: Sized {
    fn OnDropDownClosed(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnDropDownOpened(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStaticsImpl: Sized {
    fn IsDropDownOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxDropDownHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStatics2Impl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStatics3Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextSearchEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStatics4Impl: Sized {
    fn SelectionChangedTriggerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStatics5Impl: Sized {
    fn PlaceholderForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxStatics6Impl: Sized {
    fn IsEditableProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextBoxStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxTextSubmittedEventArgsImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarImpl: Sized {
    fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<ICommandBarElement>>;
    fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<ICommandBarElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBar2Impl: Sized {
    fn CommandBarOverflowPresenterStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetCommandBarOverflowPresenterStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn CommandBarTemplateSettings(&self) -> ::windows::core::Result<Primitives::CommandBarTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBar3Impl: Sized {
    fn DefaultLabelPosition(&self) -> ::windows::core::Result<CommandBarDefaultLabelPosition>;
    fn SetDefaultLabelPosition(&self, value: CommandBarDefaultLabelPosition) -> ::windows::core::Result<()>;
    fn OverflowButtonVisibility(&self) -> ::windows::core::Result<CommandBarOverflowButtonVisibility>;
    fn SetOverflowButtonVisibility(&self, value: CommandBarOverflowButtonVisibility) -> ::windows::core::Result<()>;
    fn IsDynamicOverflowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDynamicOverflowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn DynamicOverflowItemsChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CommandBar, DynamicOverflowItemsChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDynamicOverflowItemsChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ICommandBarElementImpl: Sized {
    fn IsCompact(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompact(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait ICommandBarElement2Impl: Sized {
    fn IsInOverflow(&self) -> ::windows::core::Result<bool>;
    fn DynamicOverflowOrder(&self) -> ::windows::core::Result<i32>;
    fn SetDynamicOverflowOrder(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutImpl: Sized {
    fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<ICommandBarElement>>;
    fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<ICommandBarElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBarFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarOverflowPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarOverflowPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBarOverflowPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarStaticsImpl: Sized {
    fn PrimaryCommandsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryCommandsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarStatics2Impl: Sized {
    fn CommandBarOverflowPresenterStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarStatics3Impl: Sized {
    fn DefaultLabelPositionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OverflowButtonVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsDynamicOverflowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerContentChangingEventArgsImpl: Sized {
    fn ItemContainer(&self) -> ::windows::core::Result<Primitives::SelectorItem>;
    fn InRecycleQueue(&self) -> ::windows::core::Result<bool>;
    fn ItemIndex(&self) -> ::windows::core::Result<i32>;
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Phase(&self) -> ::windows::core::Result<u32>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RegisterUpdateCallback(&self, callback: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, ContainerContentChangingEventArgs>>) -> ::windows::core::Result<()>;
    fn RegisterUpdateCallbackWithPhase(&self, callbackphase: u32, callback: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, ContainerContentChangingEventArgs>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentControlImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetContent(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ContentTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetContentTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ContentTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetContentTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn ContentTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetContentTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentControl2Impl: Sized {
    fn ContentTemplateRoot(&self) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ContentControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentControlOverridesImpl: Sized {
    fn OnContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnContentTemplateChanged(&self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnContentTemplateSelectorChanged(&self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentControlStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTemplateSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTitle(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TitleTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetTitleTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn FullSizeDesired(&self) -> ::windows::core::Result<bool>;
    fn SetFullSizeDesired(&self, value: bool) -> ::windows::core::Result<()>;
    fn PrimaryButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPrimaryButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SecondaryButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSecondaryButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrimaryButtonCommand(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetPrimaryButtonCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn SecondaryButtonCommand(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetSecondaryButtonCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn PrimaryButtonCommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetPrimaryButtonCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SecondaryButtonCommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSecondaryButtonCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn IsPrimaryButtonEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryButtonEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSecondaryButtonEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSecondaryButtonEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogClosingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogClosedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogOpenedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryButtonClick(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogButtonClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryButtonClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SecondaryButtonClick(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogButtonClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSecondaryButtonClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ContentDialogResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialog2Impl: Sized {
    fn CloseButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCloseButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CloseButtonCommand(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetCloseButtonCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CloseButtonCommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCloseButtonCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PrimaryButtonStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetPrimaryButtonStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn SecondaryButtonStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetSecondaryButtonStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn CloseButtonStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetCloseButtonStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn DefaultButton(&self) -> ::windows::core::Result<ContentDialogButton>;
    fn SetDefaultButton(&self, value: ContentDialogButton) -> ::windows::core::Result<()>;
    fn CloseButtonClick(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContentDialog, ContentDialogButtonClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseButtonClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialog3Impl: Sized {
    fn ShowAsyncWithPlacement(&self, placement: ContentDialogPlacement) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ContentDialogResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogButtonClickDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogButtonClickEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<ContentDialogButtonClickDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogClosedEventArgsImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<ContentDialogResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogClosingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogClosingEventArgsImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<ContentDialogResult>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<ContentDialogClosingDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ContentDialog>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogOpenedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TitleTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FullSizeDesiredProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PrimaryButtonTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryButtonTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PrimaryButtonCommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryButtonCommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PrimaryButtonCommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryButtonCommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPrimaryButtonEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSecondaryButtonEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentDialogStatics2Impl: Sized {
    fn CloseButtonTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CloseButtonCommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CloseButtonCommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PrimaryButtonStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryButtonStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CloseButtonStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DefaultButtonProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkChangedEventArgsImpl: Sized {
    fn ChangeKind(&self) -> ::windows::core::Result<ContentLinkChangeKind>;
    fn ContentLinkInfo(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo>;
    fn TextRange(&self) -> ::windows::core::Result<super::Documents::TextRange>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetContent(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ContentTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetContentTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ContentTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetContentTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn ContentTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetContentTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
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
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenter2Impl: Sized {
    fn OpticalMarginAlignment(&self) -> ::windows::core::Result<super::OpticalMarginAlignment>;
    fn SetOpticalMarginAlignment(&self, value: super::OpticalMarginAlignment) -> ::windows::core::Result<()>;
    fn TextLineBounds(&self) -> ::windows::core::Result<super::TextLineBounds>;
    fn SetTextLineBounds(&self, value: super::TextLineBounds) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenter3Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenter4Impl: Sized {
    fn TextWrapping(&self) -> ::windows::core::Result<super::TextWrapping>;
    fn SetTextWrapping(&self, value: super::TextWrapping) -> ::windows::core::Result<()>;
    fn MaxLines(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLines(&self, value: i32) -> ::windows::core::Result<()>;
    fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy>;
    fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<f64>;
    fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn HorizontalContentAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalContentAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalContentAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenter5Impl: Sized {
    fn BackgroundTransition(&self) -> ::windows::core::Result<super::BrushTransition>;
    fn SetBackgroundTransition(&self, value: &::core::option::Option<super::BrushTransition>) -> ::windows::core::Result<()>;
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ContentPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterOverridesImpl: Sized {
    fn OnContentTemplateChanged(&self, oldcontenttemplate: &::core::option::Option<super::DataTemplate>, newcontenttemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnContentTemplateSelectorChanged(&self, oldcontenttemplateselector: &::core::option::Option<DataTemplateSelector>, newcontenttemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTemplateSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterStatics2Impl: Sized {
    fn OpticalMarginAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextLineBoundsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterStatics3Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterStatics4Impl: Sized {
    fn TextWrappingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxLinesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineStackingStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPresenterStatics5Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContextMenuEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CursorLeft(&self) -> ::windows::core::Result<f64>;
    fn CursorTop(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlImpl: Sized {
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
    fn IsTabStop(&self) -> ::windows::core::Result<bool>;
    fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn TabNavigation(&self) -> ::windows::core::Result<super::Input::KeyboardNavigationMode>;
    fn SetTabNavigation(&self, value: super::Input::KeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn Template(&self) -> ::windows::core::Result<ControlTemplate>;
    fn SetTemplate(&self, value: &::core::option::Option<ControlTemplate>) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn HorizontalContentAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalContentAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalContentAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn IsEnabledChanged(&self, handler: &::core::option::Option<super::DependencyPropertyChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsEnabledChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ApplyTemplate(&self) -> ::windows::core::Result<bool>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControl2Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControl3Impl: Sized {
    fn UseSystemFocusVisuals(&self) -> ::windows::core::Result<bool>;
    fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControl4Impl: Sized {
    fn IsFocusEngagementEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFocusEngagementEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFocusEngaged(&self) -> ::windows::core::Result<bool>;
    fn SetIsFocusEngaged(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequiresPointer(&self) -> ::windows::core::Result<RequiresPointer>;
    fn SetRequiresPointer(&self, value: RequiresPointer) -> ::windows::core::Result<()>;
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
    fn FocusEngaged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Control, FocusEngagedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusEngaged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FocusDisengaged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Control, FocusDisengagedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusDisengaged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoveFocusEngagement(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControl5Impl: Sized {
    fn DefaultStyleResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetDefaultStyleResourceUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControl7Impl: Sized {
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Control>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlOverridesImpl: Sized {
    fn OnPointerEntered(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerPressed(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerMoved(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerReleased(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerExited(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerCaptureLost(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerCanceled(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPointerWheelChanged(&self, e: &::core::option::Option<super::Input::PointerRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnTapped(&self, e: &::core::option::Option<super::Input::TappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnDoubleTapped(&self, e: &::core::option::Option<super::Input::DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnHolding(&self, e: &::core::option::Option<super::Input::HoldingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnRightTapped(&self, e: &::core::option::Option<super::Input::RightTappedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationStarting(&self, e: &::core::option::Option<super::Input::ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationInertiaStarting(&self, e: &::core::option::Option<super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationStarted(&self, e: &::core::option::Option<super::Input::ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationDelta(&self, e: &::core::option::Option<super::Input::ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnManipulationCompleted(&self, e: &::core::option::Option<super::Input::ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnKeyUp(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnKeyDown(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnGotFocus(&self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnLostFocus(&self, e: &::core::option::Option<super::RoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragEnter(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragLeave(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDragOver(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
    fn OnDrop(&self, e: &::core::option::Option<super::DragEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlOverrides6Impl: Sized {
    fn OnPreviewKeyDown(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnPreviewKeyUp(&self, e: &::core::option::Option<super::Input::KeyRoutedEventArgs>) -> ::windows::core::Result<()>;
    fn OnCharacterReceived(&self, e: &::core::option::Option<super::Input::CharacterReceivedRoutedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlProtectedImpl: Sized {
    fn DefaultStyleKey(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDefaultStyleKey(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetTemplateChild(&self, childname: &::windows::core::HSTRING) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStaticsImpl: Sized {
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTabStopProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TabNavigationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DefaultStyleKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FocusStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStatics2Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStatics3Impl: Sized {
    fn UseSystemFocusVisualsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTemplateFocusTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsTemplateFocusTarget(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<bool>;
    fn SetIsTemplateFocusTarget(&self, element: &::core::option::Option<super::FrameworkElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStatics4Impl: Sized {
    fn IsFocusEngagementEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFocusEngagedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RequiresPointerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStatics5Impl: Sized {
    fn DefaultStyleResourceUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTemplateKeyTipTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsTemplateKeyTipTarget(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsTemplateKeyTipTarget(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlStatics7Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlTemplateImpl: Sized {
    fn TargetType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn SetTargetType(&self, value: &super::Interop::TypeName) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateSelectorImpl: Sized {
    fn SelectTemplate(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateSelector2Impl: Sized {
    fn SelectTemplateForItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateSelectorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateSelector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateSelectorOverridesImpl: Sized {
    fn SelectTemplateCore(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateSelectorOverrides2Impl: Sized {
    fn SelectTemplateForItemCore(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickedEventArgsImpl: Sized {
    fn OldDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NewDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCalendarIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn DayVisible(&self) -> ::windows::core::Result<bool>;
    fn SetDayVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn MonthVisible(&self) -> ::windows::core::Result<bool>;
    fn SetMonthVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn YearVisible(&self) -> ::windows::core::Result<bool>;
    fn SetYearVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn DayFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDayFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MonthFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMonthFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YearFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYearFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MinYear(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMinYear(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MaxYear(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMaxYear(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn DateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<DatePickerValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePicker2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePicker3Impl: Sized {
    fn SelectedDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetSelectedDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn SelectedDateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DatePicker, DatePickerSelectedValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedDateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DatePicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutImpl: Sized {
    fn CalendarIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCalendarIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetDate(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn DayVisible(&self) -> ::windows::core::Result<bool>;
    fn SetDayVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn MonthVisible(&self) -> ::windows::core::Result<bool>;
    fn SetMonthVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn YearVisible(&self) -> ::windows::core::Result<bool>;
    fn SetYearVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinYear(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMinYear(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MaxYear(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetMaxYear(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn DatePicked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DatePickerFlyout, DatePickedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDatePicked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAtAsync(&self, target: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyout2Impl: Sized {
    fn DayFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDayFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MonthFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMonthFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YearFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYearFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutItemImpl: Sized {
    fn PrimaryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPrimaryText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SecondaryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSecondaryText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutItemStaticsImpl: Sized {
    fn PrimaryTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SecondaryTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenter2Impl: Sized {
    fn IsDefaultShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDefaultShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenterStatics2Impl: Sized {
    fn IsDefaultShadowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutStaticsImpl: Sized {
    fn CalendarIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YearVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinYearProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxYearProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutStatics2Impl: Sized {
    fn DayFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YearFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerSelectedValueChangedEventArgsImpl: Sized {
    fn OldDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn NewDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CalendarIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YearVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DayFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MonthFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YearFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinYearProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxYearProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerStatics3Impl: Sized {
    fn SelectedDateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerValueChangedEventArgsImpl: Sized {
    fn OldDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NewDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemsCompletedEventArgsImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>;
    fn DropResult(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemsStartingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::DataTransfer::DataPackage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropDownButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDropDownButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDropDownButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<DropDownButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DropDownButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropDownButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DropDownButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDynamicOverflowItemsChangingEventArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<CommandBarDynamicOverflowAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipView2Impl: Sized {
    fn UseTouchAnimationsForAllNavigation(&self) -> ::windows::core::Result<bool>;
    fn SetUseTouchAnimationsForAllNavigation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewStatics2Impl: Sized {
    fn UseTouchAnimationsForAllNavigationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn FlyoutPresenterStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetFlyoutPresenterStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Flyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenter2Impl: Sized {
    fn IsDefaultShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDefaultShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterStatics2Impl: Sized {
    fn IsDefaultShadowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FlyoutPresenterStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusDisengagedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusEngagedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusEngagedEventArgs2Impl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconImpl: Sized {
    fn Glyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIcon2Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIcon3Impl: Sized {
    fn MirroredWhenRightToLeft(&self) -> ::windows::core::Result<bool>;
    fn SetMirroredWhenRightToLeft(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FontIcon>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconSourceImpl: Sized {
    fn Glyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontSize(&self) -> ::windows::core::Result<f64>;
    fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily>;
    fn SetFontFamily(&self, value: &::core::option::Option<super::Media::FontFamily>) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight>;
    fn SetFontWeight(&self, value: &super::super::Text::FontWeight) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle>;
    fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()>;
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MirroredWhenRightToLeft(&self) -> ::windows::core::Result<bool>;
    fn SetMirroredWhenRightToLeft(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FontIconSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconSourceStaticsImpl: Sized {
    fn GlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MirroredWhenRightToLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconStaticsImpl: Sized {
    fn GlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconStatics2Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontIconStatics3Impl: Sized {
    fn MirroredWhenRightToLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameImpl: Sized {
    fn CacheSize(&self) -> ::windows::core::Result<i32>;
    fn SetCacheSize(&self, value: i32) -> ::windows::core::Result<()>;
    fn CanGoBack(&self) -> ::windows::core::Result<bool>;
    fn CanGoForward(&self) -> ::windows::core::Result<bool>;
    fn CurrentSourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn SetSourcePageType(&self, value: &super::Interop::TypeName) -> ::windows::core::Result<()>;
    fn BackStackDepth(&self) -> ::windows::core::Result<i32>;
    fn Navigated(&self, handler: &::core::option::Option<super::Navigation::NavigatedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Navigating(&self, handler: &::core::option::Option<super::Navigation::NavigatingCancelEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigating(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationFailed(&self, handler: &::core::option::Option<super::Navigation::NavigationFailedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationStopped(&self, handler: &::core::option::Option<super::Navigation::NavigationStoppedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GoBack(&self) -> ::windows::core::Result<()>;
    fn GoForward(&self) -> ::windows::core::Result<()>;
    fn Navigate(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn GetNavigationState(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationState(&self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrame2Impl: Sized {
    fn BackStack(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::Navigation::PageStackEntry>>;
    fn ForwardStack(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::Navigation::PageStackEntry>>;
    fn Navigate(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, infooverride: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrame3Impl: Sized {
    fn GoBack(&self, transitioninfooverride: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrame4Impl: Sized {
    fn SetNavigationStateWithNavigationControl(&self, navigationstate: &::windows::core::HSTRING, suppressnavigate: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrame5Impl: Sized {
    fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn NavigateToType(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, navigationoptions: &::core::option::Option<super::Navigation::FrameNavigationOptions>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Frame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameStaticsImpl: Sized {
    fn CacheSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanGoBackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanGoForwardProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CurrentSourcePageTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SourcePageTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackStackDepthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameStatics2Impl: Sized {
    fn BackStackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForwardStackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameStatics5Impl: Sized {
    fn IsNavigationStackEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridImpl: Sized {
    fn RowDefinitions(&self) -> ::windows::core::Result<RowDefinitionCollection>;
    fn ColumnDefinitions(&self) -> ::windows::core::Result<ColumnDefinitionCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGrid2Impl: Sized {
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGrid3Impl: Sized {
    fn RowSpacing(&self) -> ::windows::core::Result<f64>;
    fn SetRowSpacing(&self, value: f64) -> ::windows::core::Result<()>;
    fn ColumnSpacing(&self) -> ::windows::core::Result<f64>;
    fn SetColumnSpacing(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGrid4Impl: Sized {
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Grid>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridStaticsImpl: Sized {
    fn RowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetRow(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<i32>;
    fn SetRow(&self, element: &::core::option::Option<super::FrameworkElement>, value: i32) -> ::windows::core::Result<()>;
    fn ColumnProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetColumn(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<i32>;
    fn SetColumn(&self, element: &::core::option::Option<super::FrameworkElement>, value: i32) -> ::windows::core::Result<()>;
    fn RowSpanProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetRowSpan(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<i32>;
    fn SetRowSpan(&self, element: &::core::option::Option<super::FrameworkElement>, value: i32) -> ::windows::core::Result<()>;
    fn ColumnSpanProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetColumnSpan(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<i32>;
    fn SetColumnSpan(&self, element: &::core::option::Option<super::FrameworkElement>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridStatics2Impl: Sized {
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridStatics3Impl: Sized {
    fn RowSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColumnSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridStatics4Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewHeaderItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemImpl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::GridViewItemTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyleImpl: Sized {
    fn Panel(&self) -> ::windows::core::Result<ItemsPanelTemplate>;
    fn SetPanel(&self, value: &::core::option::Option<ItemsPanelTemplate>) -> ::windows::core::Result<()>;
    fn ContainerStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetContainerStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn ContainerStyleSelector(&self) -> ::windows::core::Result<StyleSelector>;
    fn SetContainerStyleSelector(&self, value: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn HeaderTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetHeaderTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn HidesIfEmpty(&self) -> ::windows::core::Result<bool>;
    fn SetHidesIfEmpty(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyle2Impl: Sized {
    fn HeaderContainerStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetHeaderContainerStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyleFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupStyle>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyleSelectorImpl: Sized {
    fn SelectGroupStyle(&self, group: &::core::option::Option<::windows::core::IInspectable>, level: u32) -> ::windows::core::Result<GroupStyle>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyleSelectorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupStyleSelector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupStyleSelectorOverridesImpl: Sized {
    fn SelectGroupStyleCore(&self, group: &::core::option::Option<::windows::core::IInspectable>, level: u32) -> ::windows::core::Result<GroupStyle>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingPanelClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingPanelOpenedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewImpl: Sized {
    fn PlacementTarget(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPlacementTarget(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn PlacementAlignment(&self) -> ::windows::core::Result<HandwritingPanelPlacementAlignment>;
    fn SetPlacementAlignment(&self, value: HandwritingPanelPlacementAlignment) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn AreCandidatesEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreCandidatesEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HandwritingView, HandwritingPanelOpenedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HandwritingView, HandwritingPanelClosedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryClose(&self) -> ::windows::core::Result<bool>;
    fn TryOpen(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingView2Impl: Sized {
    fn IsSwitchToKeyboardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSwitchToKeyboardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCommandBarOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsCommandBarOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes>;
    fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn CandidatesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HandwritingView, HandwritingViewCandidatesChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCandidatesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextSubmitted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HandwritingView, HandwritingViewTextSubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextSubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCandidates(&self, candidatessessionid: u32) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SelectCandidate(&self, candidatessessionid: u32, selectedcandidateindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewCandidatesChangedEventArgsImpl: Sized {
    fn CandidatesSessionId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HandwritingView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewStaticsImpl: Sized {
    fn PlacementTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlacementAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AreCandidatesEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewStatics2Impl: Sized {
    fn IsSwitchToKeyboardEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCommandBarOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandwritingViewTextSubmittedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHubImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn DefaultSectionIndex(&self) -> ::windows::core::Result<i32>;
    fn SetDefaultSectionIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Sections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HubSection>>;
    fn SectionsInView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HubSection>>;
    fn SectionHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
    fn SectionHeaderClick(&self, handler: &::core::option::Option<HubSectionHeaderClickEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSectionHeaderClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SectionsInViewChanged(&self, handler: &::core::option::Option<SectionsInViewChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSectionsInViewChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScrollToSection(&self, section: &::core::option::Option<HubSection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Hub>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ContentTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetContentTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn IsHeaderInteractive(&self) -> ::windows::core::Result<bool>;
    fn SetIsHeaderInteractive(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubSection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionHeaderClickEventArgsImpl: Sized {
    fn Section(&self) -> ::windows::core::Result<HubSection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHeaderInteractiveProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DefaultSectionIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SemanticZoomOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsActiveViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomedInViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonImpl: Sized {
    fn NavigateUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetNavigateUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HyperlinkButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonStaticsImpl: Sized {
    fn NavigateUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconElementImpl: Sized {
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconElementFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IIconElementStaticsImpl: Sized {
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceImpl: Sized {
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceElementImpl: Sized {
    fn IconSource(&self) -> ::windows::core::Result<IconSource>;
    fn SetIconSource(&self, value: &::core::option::Option<IconSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IconSourceElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceElementStaticsImpl: Sized {
    fn IconSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IIconSourceStaticsImpl: Sized {
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetSource(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn NineGrid(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetNineGrid(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn PlayToSource(&self) -> ::windows::core::Result<super::super::super::Media::PlayTo::PlayToSource>;
    fn ImageFailed(&self, handler: &::core::option::Option<super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImage2Impl: Sized {
    fn GetAsCastingSource(&self) -> ::windows::core::Result<super::super::super::Media::Casting::CastingSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImage3Impl: Sized {
    fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageStaticsImpl: Sized {
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn NineGridProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlayToSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkCanvasImpl: Sized {
    fn InkPresenter(&self) -> ::windows::core::Result<super::super::Input::Inking::InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkCanvasFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkCanvas>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarImpl: Sized {
    fn InitialControls(&self) -> ::windows::core::Result<InkToolbarInitialControls>;
    fn SetInitialControls(&self, value: InkToolbarInitialControls) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<super::DependencyObjectCollection>;
    fn ActiveTool(&self) -> ::windows::core::Result<InkToolbarToolButton>;
    fn SetActiveTool(&self, value: &::core::option::Option<InkToolbarToolButton>) -> ::windows::core::Result<()>;
    fn InkDrawingAttributes(&self) -> ::windows::core::Result<super::super::Input::Inking::InkDrawingAttributes>;
    fn IsRulerButtonChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsRulerButtonChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetInkCanvas(&self) -> ::windows::core::Result<InkCanvas>;
    fn SetTargetInkCanvas(&self, value: &::core::option::Option<InkCanvas>) -> ::windows::core::Result<()>;
    fn ActiveToolChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveToolChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkDrawingAttributesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInkDrawingAttributesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EraseAllClicked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEraseAllClicked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsRulerButtonCheckedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsRulerButtonCheckedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetToolButton(&self, tool: InkToolbarTool) -> ::windows::core::Result<InkToolbarToolButton>;
    fn GetToggleButton(&self, tool: InkToolbarToggle) -> ::windows::core::Result<InkToolbarToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbar2Impl: Sized {
    fn IsStencilButtonChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsStencilButtonChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn ButtonFlyoutPlacement(&self) -> ::windows::core::Result<InkToolbarButtonFlyoutPlacement>;
    fn SetButtonFlyoutPlacement(&self, value: InkToolbarButtonFlyoutPlacement) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn IsStencilButtonCheckedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbar, InkToolbarIsStencilButtonCheckedChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsStencilButtonCheckedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetMenuButton(&self, menu: InkToolbarMenuKind) -> ::windows::core::Result<InkToolbarMenuButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbar3Impl: Sized {
    fn TargetInkPresenter(&self) -> ::windows::core::Result<super::super::Input::Inking::InkPresenter>;
    fn SetTargetInkPresenter(&self, value: &::core::option::Option<super::super::Input::Inking::InkPresenter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarBallpointPenButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarBallpointPenButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarBallpointPenButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenImpl: Sized {
    fn CreateInkDrawingAttributes(&self, brush: &::core::option::Option<super::Media::Brush>, strokewidth: f64) -> ::windows::core::Result<super::super::Input::Inking::InkDrawingAttributes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenButtonImpl: Sized {
    fn CustomPen(&self) -> ::windows::core::Result<InkToolbarCustomPen>;
    fn SetCustomPen(&self, value: &::core::option::Option<InkToolbarCustomPen>) -> ::windows::core::Result<()>;
    fn ConfigurationContent(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetConfigurationContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarCustomPenButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenButtonStaticsImpl: Sized {
    fn CustomPenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ConfigurationContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarCustomPen>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomPenOverridesImpl: Sized {
    fn CreateInkDrawingAttributesCore(&self, brush: &::core::option::Option<super::Media::Brush>, strokewidth: f64) -> ::windows::core::Result<super::super::Input::Inking::InkDrawingAttributes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomToggleButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomToggleButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarCustomToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomToolButtonImpl: Sized {
    fn ConfigurationContent(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetConfigurationContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomToolButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarCustomToolButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarCustomToolButtonStaticsImpl: Sized {
    fn ConfigurationContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarEraserButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarEraserButton2Impl: Sized {
    fn IsClearAllVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsClearAllVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarEraserButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarEraserButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarEraserButtonStatics2Impl: Sized {
    fn IsClearAllVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarFlyoutItemImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<InkToolbarFlyoutItemKind>;
    fn SetKind(&self, value: InkToolbarFlyoutItemKind) -> ::windows::core::Result<()>;
    fn IsChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn Checked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbarFlyoutItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChecked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unchecked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkToolbarFlyoutItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnchecked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarFlyoutItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarFlyoutItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarFlyoutItemStaticsImpl: Sized {
    fn KindProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCheckedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarHighlighterButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarHighlighterButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarHighlighterButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarIsStencilButtonCheckedChangedEventArgsImpl: Sized {
    fn StencilButton(&self) -> ::windows::core::Result<InkToolbarStencilButton>;
    fn StencilKind(&self) -> ::windows::core::Result<InkToolbarStencilKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarMenuButtonImpl: Sized {
    fn MenuKind(&self) -> ::windows::core::Result<InkToolbarMenuKind>;
    fn IsExtensionGlyphShown(&self) -> ::windows::core::Result<bool>;
    fn SetIsExtensionGlyphShown(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarMenuButtonFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarMenuButtonStaticsImpl: Sized {
    fn IsExtensionGlyphShownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenButtonImpl: Sized {
    fn Palette(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::Media::Brush>>;
    fn SetPalette(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::Media::Brush>>) -> ::windows::core::Result<()>;
    fn MinStrokeWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinStrokeWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxStrokeWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMaxStrokeWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn SelectedBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SelectedBrushIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedBrushIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedStrokeWidth(&self) -> ::windows::core::Result<f64>;
    fn SetSelectedStrokeWidth(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenButtonFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenButtonStaticsImpl: Sized {
    fn PaletteProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinStrokeWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxStrokeWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedBrushIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedStrokeWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenConfigurationControlImpl: Sized {
    fn PenButton(&self) -> ::windows::core::Result<InkToolbarPenButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenConfigurationControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarPenConfigurationControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPenConfigurationControlStaticsImpl: Sized {
    fn PenButtonProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPencilButtonImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarPencilButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarPencilButton>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IInkToolbarRulerButtonImpl: Sized {
    fn Ruler(&self) -> ::windows::core::Result<super::super::Input::Inking::InkPresenterRuler>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IInkToolbarRulerButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarRulerButton>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IInkToolbarRulerButtonStaticsImpl: Sized {
    fn RulerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStaticsImpl: Sized {
    fn InitialControlsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ActiveToolProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InkDrawingAttributesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsRulerButtonCheckedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TargetInkCanvasProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStatics2Impl: Sized {
    fn IsStencilButtonCheckedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ButtonFlyoutPlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStatics3Impl: Sized {
    fn TargetInkPresenterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStencilButtonImpl: Sized {
    fn Ruler(&self) -> ::windows::core::Result<super::super::Input::Inking::InkPresenterRuler>;
    fn Protractor(&self) -> ::windows::core::Result<super::super::Input::Inking::InkPresenterProtractor>;
    fn SelectedStencil(&self) -> ::windows::core::Result<InkToolbarStencilKind>;
    fn SetSelectedStencil(&self, value: InkToolbarStencilKind) -> ::windows::core::Result<()>;
    fn IsRulerItemVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsRulerItemVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsProtractorItemVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsProtractorItemVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStencilButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<InkToolbarStencilButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarStencilButtonStaticsImpl: Sized {
    fn RulerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProtractorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedStencilProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsRulerItemVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsProtractorItemVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarToggleButtonImpl: Sized {
    fn ToggleKind(&self) -> ::windows::core::Result<InkToolbarToggle>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarToggleButtonFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarToolButtonImpl: Sized {
    fn ToolKind(&self) -> ::windows::core::Result<InkToolbarTool>;
    fn IsExtensionGlyphShown(&self) -> ::windows::core::Result<bool>;
    fn SetIsExtensionGlyphShown(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarToolButtonFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarToolButtonStaticsImpl: Sized {
    fn IsExtensionGlyphShownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
pub trait IInsertionPanelImpl: Sized {
    fn GetInsertionIndexes(&self, position: &super::super::super::Foundation::Point, first: &mut i32, second: &mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsTextTrimmedChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IItemClickEventArgsImpl: Sized {
    fn ClickedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemContainerGeneratorImpl: Sized {
    fn ItemsChanged(&self, handler: &::core::option::Option<Primitives::ItemsChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn IndexFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn ContainerFromIndex(&self, index: i32) -> ::windows::core::Result<super::DependencyObject>;
    fn GetItemContainerGeneratorForPanel(&self, panel: &::core::option::Option<Panel>) -> ::windows::core::Result<ItemContainerGenerator>;
    fn StartAt(&self, position: &Primitives::GeneratorPosition, direction: Primitives::GeneratorDirection, allowstartatrealizeditem: bool) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn GenerateNext(&self, isnewlyrealized: &mut bool) -> ::windows::core::Result<super::DependencyObject>;
    fn PrepareItemContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
    fn Remove(&self, position: &Primitives::GeneratorPosition, count: i32) -> ::windows::core::Result<()>;
    fn GeneratorPositionFromIndex(&self, itemindex: i32) -> ::windows::core::Result<Primitives::GeneratorPosition>;
    fn IndexFromGeneratorPosition(&self, position: &Primitives::GeneratorPosition) -> ::windows::core::Result<i32>;
    fn Recycle(&self, position: &Primitives::GeneratorPosition, count: i32) -> ::windows::core::Result<()>;
}
pub trait IItemContainerMappingImpl: Sized {
    fn ItemFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn IndexFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn ContainerFromIndex(&self, index: i32) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlImpl: Sized {
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<ItemCollection>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ItemTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetItemTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn ItemsPanel(&self) -> ::windows::core::Result<ItemsPanelTemplate>;
    fn SetItemsPanel(&self, value: &::core::option::Option<ItemsPanelTemplate>) -> ::windows::core::Result<()>;
    fn DisplayMemberPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMemberPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemContainerStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetItemContainerStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn ItemContainerStyleSelector(&self) -> ::windows::core::Result<StyleSelector>;
    fn SetItemContainerStyleSelector(&self, value: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn ItemContainerGenerator(&self) -> ::windows::core::Result<ItemContainerGenerator>;
    fn ItemContainerTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetItemContainerTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn GroupStyle(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<GroupStyle>>;
    fn GroupStyleSelector(&self) -> ::windows::core::Result<GroupStyleSelector>;
    fn SetGroupStyleSelector(&self, value: &::core::option::Option<GroupStyleSelector>) -> ::windows::core::Result<()>;
    fn IsGrouping(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControl2Impl: Sized {
    fn ItemsPanelRoot(&self) -> ::windows::core::Result<Panel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControl3Impl: Sized {
    fn GroupHeaderContainerFromItemContainer(&self, itemcontainer: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemsControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlOverridesImpl: Sized {
    fn IsItemItsOwnContainerOverride(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn GetContainerForItemOverride(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn ClearContainerForItemOverride(&self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PrepareContainerForItemOverride(&self, element: &::core::option::Option<super::DependencyObject>, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&self, e: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnItemContainerStyleChanged(&self, olditemcontainerstyle: &::core::option::Option<super::Style>, newitemcontainerstyle: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn OnItemContainerStyleSelectorChanged(&self, olditemcontainerstyleselector: &::core::option::Option<StyleSelector>, newitemcontainerstyleselector: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn OnItemTemplateChanged(&self, olditemtemplate: &::core::option::Option<super::DataTemplate>, newitemtemplate: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnItemTemplateSelectorChanged(&self, olditemtemplateselector: &::core::option::Option<DataTemplateSelector>, newitemtemplateselector: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn OnGroupStyleSelectorChanged(&self, oldgroupstyleselector: &::core::option::Option<GroupStyleSelector>, newgroupstyleselector: &::core::option::Option<GroupStyleSelector>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlStaticsImpl: Sized {
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemTemplateSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayMemberPathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerStyleSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GroupStyleSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsGroupingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemsOwner(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ItemsControl>;
    fn ItemsControlFromItemContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ItemsControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPanelTemplateImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPickedEventArgsImpl: Sized {
    fn AddedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn RemovedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPresenterImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn HeaderTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetHeaderTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPresenter2Impl: Sized {
    fn Footer(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetFooter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn FooterTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetFooterTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn FooterTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetFooterTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPresenterStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsPresenterStatics2Impl: Sized {
    fn FooterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FooterTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FooterTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsStackPanelImpl: Sized {
    fn GroupPadding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetGroupPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn FirstCacheIndex(&self) -> ::windows::core::Result<i32>;
    fn FirstVisibleIndex(&self) -> ::windows::core::Result<i32>;
    fn LastVisibleIndex(&self) -> ::windows::core::Result<i32>;
    fn LastCacheIndex(&self) -> ::windows::core::Result<i32>;
    fn ScrollingDirection(&self) -> ::windows::core::Result<PanelScrollingDirection>;
    fn GroupHeaderPlacement(&self) -> ::windows::core::Result<Primitives::GroupHeaderPlacement>;
    fn SetGroupHeaderPlacement(&self, value: Primitives::GroupHeaderPlacement) -> ::windows::core::Result<()>;
    fn ItemsUpdatingScrollMode(&self) -> ::windows::core::Result<ItemsUpdatingScrollMode>;
    fn SetItemsUpdatingScrollMode(&self, value: ItemsUpdatingScrollMode) -> ::windows::core::Result<()>;
    fn CacheLength(&self) -> ::windows::core::Result<f64>;
    fn SetCacheLength(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsStackPanel2Impl: Sized {
    fn AreStickyGroupHeadersEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreStickyGroupHeadersEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsStackPanelStaticsImpl: Sized {
    fn GroupPaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GroupHeaderPlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CacheLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsStackPanelStatics2Impl: Sized {
    fn AreStickyGroupHeadersEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsWrapGridImpl: Sized {
    fn GroupPadding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetGroupPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn MaximumRowsOrColumns(&self) -> ::windows::core::Result<i32>;
    fn SetMaximumRowsOrColumns(&self, value: i32) -> ::windows::core::Result<()>;
    fn ItemWidth(&self) -> ::windows::core::Result<f64>;
    fn SetItemWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn ItemHeight(&self) -> ::windows::core::Result<f64>;
    fn SetItemHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn FirstCacheIndex(&self) -> ::windows::core::Result<i32>;
    fn FirstVisibleIndex(&self) -> ::windows::core::Result<i32>;
    fn LastVisibleIndex(&self) -> ::windows::core::Result<i32>;
    fn LastCacheIndex(&self) -> ::windows::core::Result<i32>;
    fn ScrollingDirection(&self) -> ::windows::core::Result<PanelScrollingDirection>;
    fn GroupHeaderPlacement(&self) -> ::windows::core::Result<Primitives::GroupHeaderPlacement>;
    fn SetGroupHeaderPlacement(&self, value: Primitives::GroupHeaderPlacement) -> ::windows::core::Result<()>;
    fn CacheLength(&self) -> ::windows::core::Result<f64>;
    fn SetCacheLength(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsWrapGrid2Impl: Sized {
    fn AreStickyGroupHeadersEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreStickyGroupHeadersEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsWrapGridStaticsImpl: Sized {
    fn GroupPaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaximumRowsOrColumnsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GroupHeaderPlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CacheLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsWrapGridStatics2Impl: Sized {
    fn AreStickyGroupHeadersEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxImpl: Sized {
    fn SelectedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn SelectionMode(&self) -> ::windows::core::Result<SelectionMode>;
    fn SetSelectionMode(&self, value: SelectionMode) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBox2Impl: Sized {
    fn SingleSelectionFollowsFocus(&self) -> ::windows::core::Result<bool>;
    fn SetSingleSelectionFollowsFocus(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxStaticsImpl: Sized {
    fn SelectionModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxStatics2Impl: Sized {
    fn SingleSelectionFollowsFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutImpl: Sized {
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn DisplayMemberPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMemberPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionMode(&self) -> ::windows::core::Result<ListPickerFlyoutSelectionMode>;
    fn SetSelectionMode(&self, value: ListPickerFlyoutSelectionMode) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValuePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSelectedValuePath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn ItemsPicked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListPickerFlyout, ItemsPickedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsPicked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAtAsync(&self, target: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutStaticsImpl: Sized {
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayMemberPathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedValuePathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseImpl: Sized {
    fn SelectedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn SelectionMode(&self) -> ::windows::core::Result<ListViewSelectionMode>;
    fn SetSelectionMode(&self, value: ListViewSelectionMode) -> ::windows::core::Result<()>;
    fn IsSwipeEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSwipeEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanDragItems(&self) -> ::windows::core::Result<bool>;
    fn SetCanDragItems(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReorderItems(&self) -> ::windows::core::Result<bool>;
    fn SetCanReorderItems(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsItemClickEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsItemClickEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn DataFetchSize(&self) -> ::windows::core::Result<f64>;
    fn SetDataFetchSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn IncrementalLoadingThreshold(&self) -> ::windows::core::Result<f64>;
    fn SetIncrementalLoadingThreshold(&self, value: f64) -> ::windows::core::Result<()>;
    fn IncrementalLoadingTrigger(&self) -> ::windows::core::Result<IncrementalLoadingTrigger>;
    fn SetIncrementalLoadingTrigger(&self, value: IncrementalLoadingTrigger) -> ::windows::core::Result<()>;
    fn ItemClick(&self, handler: &::core::option::Option<ItemClickEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragItemsStarting(&self, handler: &::core::option::Option<DragItemsStartingEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragItemsStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn LoadMoreItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::Data::LoadMoreItemsResult>>;
    fn ScrollIntoViewWithAlignment(&self, item: &::core::option::Option<::windows::core::IInspectable>, alignment: ScrollIntoViewAlignment) -> ::windows::core::Result<()>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn HeaderTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetHeaderTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBase2Impl: Sized {
    fn ShowsScrollingPlaceholders(&self) -> ::windows::core::Result<bool>;
    fn SetShowsScrollingPlaceholders(&self, value: bool) -> ::windows::core::Result<()>;
    fn ContainerContentChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, ContainerContentChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainerContentChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDesiredContainerUpdateDuration(&self, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Footer(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetFooter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn FooterTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetFooterTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn FooterTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetFooterTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBase3Impl: Sized {
    fn ReorderMode(&self) -> ::windows::core::Result<ListViewReorderMode>;
    fn SetReorderMode(&self, value: ListViewReorderMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBase4Impl: Sized {
    fn SelectedRanges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Data::ItemIndexRange>>;
    fn IsMultiSelectCheckBoxEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsMultiSelectCheckBoxEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn DragItemsCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, DragItemsCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragItemsCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChoosingItemContainer(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, ChoosingItemContainerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChoosingItemContainer(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChoosingGroupHeaderContainer(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ListViewBase, ChoosingGroupHeaderContainerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChoosingGroupHeaderContainer(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectRange(&self, itemindexrange: &::core::option::Option<super::Data::ItemIndexRange>) -> ::windows::core::Result<()>;
    fn DeselectRange(&self, itemindexrange: &::core::option::Option<super::Data::ItemIndexRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBase5Impl: Sized {
    fn SingleSelectionFollowsFocus(&self) -> ::windows::core::Result<bool>;
    fn SetSingleSelectionFollowsFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDragSource(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBase6Impl: Sized {
    fn TryStartConnectedAnimationAsync(&self, animation: &::core::option::Option<super::Media::Animation::ConnectedAnimation>, item: &::core::option::Option<::windows::core::IInspectable>, elementname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn PrepareConnectedAnimation(&self, key: &::windows::core::HSTRING, item: &::core::option::Option<::windows::core::IInspectable>, elementname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Media::Animation::ConnectedAnimation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseStaticsImpl: Sized {
    fn SelectionModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSwipeEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanDragItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanReorderItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsItemClickEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DataFetchSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IncrementalLoadingThresholdProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IncrementalLoadingTriggerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SemanticZoomOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsActiveViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomedInViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseStatics2Impl: Sized {
    fn ShowsScrollingPlaceholdersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FooterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FooterTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FooterTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseStatics3Impl: Sized {
    fn ReorderModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseStatics4Impl: Sized {
    fn IsMultiSelectCheckBoxEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseStatics5Impl: Sized {
    fn SingleSelectionFollowsFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewHeaderItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemImpl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ListViewItemTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewPersistenceHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewPersistenceHelperStaticsImpl: Sized {
    fn GetRelativeScrollPosition(&self, listviewbase: &::core::option::Option<ListViewBase>, itemtokeyhandler: &::core::option::Option<ListViewItemToKeyHandler>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRelativeScrollPositionAsync(&self, listviewbase: &::core::option::Option<ListViewBase>, relativescrollposition: &::windows::core::HSTRING, keytoitemhandler: &::core::option::Option<ListViewKeyToItemHandler>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementImpl: Sized {
    fn PosterSource(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetPosterSource(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetSource(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioOnly(&self) -> ::windows::core::Result<bool>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn Balance(&self) -> ::windows::core::Result<f64>;
    fn SetBalance(&self, value: f64) -> ::windows::core::Result<()>;
    fn NaturalVideoHeight(&self) -> ::windows::core::Result<i32>;
    fn NaturalVideoWidth(&self) -> ::windows::core::Result<i32>;
    fn NaturalDuration(&self) -> ::windows::core::Result<super::Duration>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn BufferingProgress(&self) -> ::windows::core::Result<f64>;
    fn DownloadProgressOffset(&self) -> ::windows::core::Result<f64>;
    fn CurrentState(&self) -> ::windows::core::Result<super::Media::MediaElementState>;
    fn Markers(&self) -> ::windows::core::Result<super::Media::TimelineMarkerCollection>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn CanPause(&self) -> ::windows::core::Result<bool>;
    fn AudioStreamCount(&self) -> ::windows::core::Result<i32>;
    fn AudioStreamIndex(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetAudioStreamIndex(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsLooping(&self) -> ::windows::core::Result<bool>;
    fn SetIsLooping(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlayToSource(&self) -> ::windows::core::Result<super::super::super::Media::PlayTo::PlayToSource>;
    fn DefaultPlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetDefaultPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn AspectRatioWidth(&self) -> ::windows::core::Result<i32>;
    fn AspectRatioHeight(&self) -> ::windows::core::Result<i32>;
    fn RealTimePlayback(&self) -> ::windows::core::Result<bool>;
    fn SetRealTimePlayback(&self, value: bool) -> ::windows::core::Result<()>;
    fn AudioCategory(&self) -> ::windows::core::Result<super::Media::AudioCategory>;
    fn SetAudioCategory(&self, value: super::Media::AudioCategory) -> ::windows::core::Result<()>;
    fn AudioDeviceType(&self) -> ::windows::core::Result<super::Media::AudioDeviceType>;
    fn SetAudioDeviceType(&self, value: super::Media::AudioDeviceType) -> ::windows::core::Result<()>;
    fn ProtectionManager(&self) -> ::windows::core::Result<super::super::super::Media::Protection::MediaProtectionManager>;
    fn SetProtectionManager(&self, value: &::core::option::Option<super::super::super::Media::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn Stereo3DVideoPackingMode(&self) -> ::windows::core::Result<super::Media::Stereo3DVideoPackingMode>;
    fn SetStereo3DVideoPackingMode(&self, value: super::Media::Stereo3DVideoPackingMode) -> ::windows::core::Result<()>;
    fn Stereo3DVideoRenderMode(&self) -> ::windows::core::Result<super::Media::Stereo3DVideoRenderMode>;
    fn SetStereo3DVideoRenderMode(&self, value: super::Media::Stereo3DVideoRenderMode) -> ::windows::core::Result<()>;
    fn IsStereo3DVideo(&self) -> ::windows::core::Result<bool>;
    fn MediaOpened(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaEnded(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaEnded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaFailed(&self, handler: &::core::option::Option<super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgressChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgressChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingProgressChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingProgressChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MarkerReached(&self, handler: &::core::option::Option<super::Media::TimelineMarkerRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMarkerReached(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RateChanged(&self, handler: &::core::option::Option<super::Media::RateChangedRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn CanPlayType(&self, r#type: &::windows::core::HSTRING) -> ::windows::core::Result<super::Media::MediaCanPlayResponse>;
    fn SetSource(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAudioStreamLanguage(&self, index: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddAudioEffect(&self, effectid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn AddVideoEffect(&self, effectid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RemoveAllEffects(&self) -> ::windows::core::Result<()>;
    fn ActualStereo3DVideoPackingMode(&self) -> ::windows::core::Result<super::Media::Stereo3DVideoPackingMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElement2Impl: Sized {
    fn AreTransportControlsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreTransportControlsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn IsFullWindow(&self) -> ::windows::core::Result<bool>;
    fn SetIsFullWindow(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetMediaStreamSource(&self, source: &::core::option::Option<super::super::super::Media::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn PlayToPreferredSourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetPlayToPreferredSourceUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElement3Impl: Sized {
    fn TransportControls(&self) -> ::windows::core::Result<MediaTransportControls>;
    fn SetTransportControls(&self, value: &::core::option::Option<MediaTransportControls>) -> ::windows::core::Result<()>;
    fn PartialMediaFailureDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaElement, super::Media::PartialMediaFailureDetectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePartialMediaFailureDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetPlaybackSource(&self, source: &::core::option::Option<super::super::super::Media::Playback::IMediaPlaybackSource>) -> ::windows::core::Result<()>;
    fn GetAsCastingSource(&self) -> ::windows::core::Result<super::super::super::Media::Casting::CastingSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementStaticsImpl: Sized {
    fn PosterSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsMutedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsAudioOnlyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VolumeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BalanceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn NaturalVideoHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn NaturalVideoWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn NaturalDurationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PositionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DownloadProgressProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BufferingProgressProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DownloadProgressOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CurrentStateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanSeekProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanPauseProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AudioStreamCountProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AudioStreamIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaybackRateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsLoopingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlayToSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DefaultPlaybackRateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AspectRatioWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AspectRatioHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RealTimePlaybackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AudioCategoryProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AudioDeviceTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProtectionManagerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Stereo3DVideoPackingModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Stereo3DVideoRenderModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsStereo3DVideoProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ActualStereo3DVideoPackingModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementStatics2Impl: Sized {
    fn AreTransportControlsEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFullWindowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlayToPreferredSourceUriProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::super::Media::Playback::IMediaPlaybackSource>;
    fn SetSource(&self, value: &::core::option::Option<super::super::super::Media::Playback::IMediaPlaybackSource>) -> ::windows::core::Result<()>;
    fn TransportControls(&self) -> ::windows::core::Result<MediaTransportControls>;
    fn SetTransportControls(&self, value: &::core::option::Option<MediaTransportControls>) -> ::windows::core::Result<()>;
    fn AreTransportControlsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreTransportControlsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PosterSource(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetPosterSource(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFullWindow(&self) -> ::windows::core::Result<bool>;
    fn SetIsFullWindow(&self, value: bool) -> ::windows::core::Result<()>;
    fn MediaPlayer(&self) -> ::windows::core::Result<super::super::super::Media::Playback::MediaPlayer>;
    fn SetMediaPlayer(&self, mediaplayer: &::core::option::Option<super::super::super::Media::Playback::MediaPlayer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementStaticsImpl: Sized {
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AreTransportControlsEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PosterSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFullWindowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MediaPlayerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerPresenterImpl: Sized {
    fn MediaPlayer(&self) -> ::windows::core::Result<super::super::super::Media::Playback::MediaPlayer>;
    fn SetMediaPlayer(&self, value: &::core::option::Option<super::super::super::Media::Playback::MediaPlayer>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn IsFullWindow(&self) -> ::windows::core::Result<bool>;
    fn SetIsFullWindow(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerPresenterStaticsImpl: Sized {
    fn MediaPlayerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFullWindowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsImpl: Sized {
    fn IsFullWindowButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsFullWindowButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFullWindowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFullWindowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastForwardButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastForwardButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastRewindButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastRewindButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastRewindEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastRewindEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsStopButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsStopButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsStopEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStopEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVolumeButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsVolumeButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVolumeEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVolumeEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlaybackRateButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsPlaybackRateButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlaybackRateEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPlaybackRateEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeekBarVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeekBarVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeekEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeekEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCompact(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompact(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControls2Impl: Sized {
    fn IsSkipForwardButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsSkipForwardButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSkipForwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSkipForwardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSkipBackwardButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsSkipBackwardButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSkipBackwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSkipBackwardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsNextTrackButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsNextTrackButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviousTrackButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsPreviousTrackButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn FastPlayFallbackBehaviour(&self) -> ::windows::core::Result<super::Media::FastPlayFallbackBehaviour>;
    fn SetFastPlayFallbackBehaviour(&self, value: super::Media::FastPlayFallbackBehaviour) -> ::windows::core::Result<()>;
    fn ThumbnailRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaTransportControls, super::Media::MediaTransportControlsThumbnailRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControls3Impl: Sized {
    fn ShowAndHideAutomatically(&self) -> ::windows::core::Result<bool>;
    fn SetShowAndHideAutomatically(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRepeatEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRepeatEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRepeatButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsRepeatButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControls4Impl: Sized {
    fn IsCompactOverlayButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompactOverlayButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCompactOverlayEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompactOverlayEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaTransportControls>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsHelperStaticsImpl: Sized {
    fn DropoutOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDropoutOrder(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetDropoutOrder(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsStaticsImpl: Sized {
    fn IsFullWindowButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFullWindowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFastForwardButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFastForwardEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFastRewindButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFastRewindEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsStopButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsStopEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsVolumeButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsVolumeEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPlaybackRateButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPlaybackRateEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSeekBarVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSeekEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCompactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsStatics2Impl: Sized {
    fn IsSkipForwardButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSkipForwardEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSkipBackwardButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSkipBackwardEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsNextTrackButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPreviousTrackButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FastPlayFallbackBehaviourProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsStatics3Impl: Sized {
    fn ShowAndHideAutomaticallyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsRepeatEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsRepeatButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsStatics4Impl: Sized {
    fn IsCompactOverlayButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsCompactOverlayEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<MenuBarItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<MenuFlyoutItemBase>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemFlyoutImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItemFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarStaticsImpl: Sized {
    fn ItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<MenuFlyoutItemBase>>;
    fn MenuFlyoutPresenterStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetMenuFlyoutPresenterStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyout2Impl: Sized {
    fn ShowAt(&self, targetelement: &::core::option::Option<super::UIElement>, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Command(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItem2Impl: Sized {
    fn Icon(&self) -> ::windows::core::Result<IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItem3Impl: Sized {
    fn KeyboardAcceleratorTextOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyboardAcceleratorTextOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::MenuFlyoutItemTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemStaticsImpl: Sized {
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemStatics2Impl: Sized {
    fn IconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemStatics3Impl: Sized {
    fn KeyboardAcceleratorTextOverrideProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenter2Impl: Sized {
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::MenuFlyoutPresenterTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenter3Impl: Sized {
    fn IsDefaultShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDefaultShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterStatics3Impl: Sized {
    fn IsDefaultShadowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSeparatorImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSeparatorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutSeparator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutStaticsImpl: Sized {
    fn MenuFlyoutPresenterStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSubItemImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<MenuFlyoutItemBase>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSubItem2Impl: Sized {
    fn Icon(&self) -> ::windows::core::Result<IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSubItemStaticsImpl: Sized {
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutSubItemStatics2Impl: Sized {
    fn IconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
pub trait INavigateImpl: Sized {
    fn Navigate(&self, sourcepagetype: &super::Interop::TypeName) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewImpl: Sized {
    fn IsPaneOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaneOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompactModeThresholdWidth(&self) -> ::windows::core::Result<f64>;
    fn SetCompactModeThresholdWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn ExpandedModeThresholdWidth(&self) -> ::windows::core::Result<f64>;
    fn SetExpandedModeThresholdWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn PaneFooter(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPaneFooter(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn DisplayMode(&self) -> ::windows::core::Result<NavigationViewDisplayMode>;
    fn IsSettingsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsSettingsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPaneToggleButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaneToggleButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysShowHeader(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysShowHeader(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompactPaneLength(&self) -> ::windows::core::Result<f64>;
    fn SetCompactPaneLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn OpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenPaneLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn PaneToggleButtonStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetPaneToggleButtonStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn MenuItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn MenuItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetMenuItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SettingsItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn AutoSuggestBox(&self) -> ::windows::core::Result<AutoSuggestBox>;
    fn SetAutoSuggestBox(&self, value: &::core::option::Option<AutoSuggestBox>) -> ::windows::core::Result<()>;
    fn MenuItemTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetMenuItemTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn MenuItemTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetMenuItemTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn MenuItemContainerStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetMenuItemContainerStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn MenuItemContainerStyleSelector(&self) -> ::windows::core::Result<StyleSelector>;
    fn SetMenuItemContainerStyleSelector(&self, value: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn MenuItemFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromMenuItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, NavigationViewSelectionChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, NavigationViewItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayModeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, NavigationViewDisplayModeChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayModeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationView2Impl: Sized {
    fn IsBackButtonVisible(&self) -> ::windows::core::Result<NavigationViewBackButtonVisible>;
    fn SetIsBackButtonVisible(&self, value: NavigationViewBackButtonVisible) -> ::windows::core::Result<()>;
    fn IsBackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PaneTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPaneTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BackRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, NavigationViewBackRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneClosed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneClosing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, NavigationViewPaneClosingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneClosing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneOpened(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneOpening(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NavigationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationView3Impl: Sized {
    fn PaneDisplayMode(&self) -> ::windows::core::Result<NavigationViewPaneDisplayMode>;
    fn SetPaneDisplayMode(&self, value: NavigationViewPaneDisplayMode) -> ::windows::core::Result<()>;
    fn PaneHeader(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPaneHeader(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn PaneCustomContent(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPaneCustomContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ContentOverlay(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContentOverlay(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn IsPaneVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaneVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn SelectionFollowsFocus(&self) -> ::windows::core::Result<NavigationViewSelectionFollowsFocus>;
    fn SetSelectionFollowsFocus(&self, value: NavigationViewSelectionFollowsFocus) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<NavigationViewTemplateSettings>;
    fn ShoulderNavigationEnabled(&self) -> ::windows::core::Result<NavigationViewShoulderNavigationEnabled>;
    fn SetShoulderNavigationEnabled(&self, value: NavigationViewShoulderNavigationEnabled) -> ::windows::core::Result<()>;
    fn OverflowLabelMode(&self) -> ::windows::core::Result<NavigationViewOverflowLabelMode>;
    fn SetOverflowLabelMode(&self, value: NavigationViewOverflowLabelMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewBackRequestedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewDisplayModeChangedEventArgsImpl: Sized {
    fn DisplayMode(&self) -> ::windows::core::Result<NavigationViewDisplayMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemImpl: Sized {
    fn Icon(&self) -> ::windows::core::Result<IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<IconElement>) -> ::windows::core::Result<()>;
    fn CompactPaneLength(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItem2Impl: Sized {
    fn SelectsOnInvoked(&self) -> ::windows::core::Result<bool>;
    fn SetSelectsOnInvoked(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemHeaderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemHeaderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemHeader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemInvokedEventArgsImpl: Sized {
    fn InvokedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn IsSettingsInvoked(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemInvokedEventArgs2Impl: Sized {
    fn InvokedItemContainer(&self) -> ::windows::core::Result<NavigationViewItemBase>;
    fn RecommendedNavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemSeparatorImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemSeparatorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemSeparator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemStaticsImpl: Sized {
    fn IconProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CompactPaneLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemStatics2Impl: Sized {
    fn SelectsOnInvokedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewListImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewListFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewList>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewPaneClosingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewSelectionChangedEventArgsImpl: Sized {
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn IsSettingsSelected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewSelectionChangedEventArgs2Impl: Sized {
    fn SelectedItemContainer(&self) -> ::windows::core::Result<NavigationViewItemBase>;
    fn RecommendedNavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewStaticsImpl: Sized {
    fn IsPaneOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CompactModeThresholdWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExpandedModeThresholdWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneFooterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSettingsVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPaneToggleButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysShowHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CompactPaneLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OpenPaneLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneToggleButtonStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemsSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SettingsItemProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AutoSuggestBoxProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemTemplateSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemContainerStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MenuItemContainerStyleSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewStatics2Impl: Sized {
    fn IsBackButtonVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsBackEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneTitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewStatics3Impl: Sized {
    fn PaneDisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneCustomContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentOverlayProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPaneVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionFollowsFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TemplateSettingsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ShoulderNavigationEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OverflowLabelModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewTemplateSettingsImpl: Sized {
    fn TopPadding(&self) -> ::windows::core::Result<f64>;
    fn OverflowButtonVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn PaneToggleButtonVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn BackButtonVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn TopPaneVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn LeftPaneVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn SingleSelectionFollowsFocus(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewTemplateSettingsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewTemplateSettingsStaticsImpl: Sized {
    fn TopPaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OverflowButtonVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneToggleButtonVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackButtonVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TopPaneVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LeftPaneVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SingleSelectionFollowsFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyEventArgsImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyEventArgs2Impl: Sized {
    fn CallingUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<Frame>;
    fn NavigationCacheMode(&self) -> ::windows::core::Result<super::Navigation::NavigationCacheMode>;
    fn SetNavigationCacheMode(&self, value: super::Navigation::NavigationCacheMode) -> ::windows::core::Result<()>;
    fn TopAppBar(&self) -> ::windows::core::Result<AppBar>;
    fn SetTopAppBar(&self, value: &::core::option::Option<AppBar>) -> ::windows::core::Result<()>;
    fn BottomAppBar(&self) -> ::windows::core::Result<AppBar>;
    fn SetBottomAppBar(&self, value: &::core::option::Option<AppBar>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Page>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageOverridesImpl: Sized {
    fn OnNavigatedFrom(&self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()>;
    fn OnNavigatedTo(&self, e: &::core::option::Option<super::Navigation::NavigationEventArgs>) -> ::windows::core::Result<()>;
    fn OnNavigatingFrom(&self, e: &::core::option::Option<super::Navigation::NavigatingCancelEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStaticsImpl: Sized {
    fn FrameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TopAppBarProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BottomAppBarProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanelImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<UIElementCollection>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn IsItemsHost(&self) -> ::windows::core::Result<bool>;
    fn ChildrenTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetChildrenTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanel2Impl: Sized {
    fn BackgroundTransition(&self) -> ::windows::core::Result<super::BrushTransition>;
    fn SetBackgroundTransition(&self, value: &::core::option::Option<super::BrushTransition>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Panel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanelStaticsImpl: Sized {
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsItemsHostProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildrenTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IParallaxViewImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn HorizontalShift(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalShift(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalSourceEndOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalSourceEndOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalSourceOffsetKind(&self) -> ::windows::core::Result<ParallaxSourceOffsetKind>;
    fn SetHorizontalSourceOffsetKind(&self, value: ParallaxSourceOffsetKind) -> ::windows::core::Result<()>;
    fn HorizontalSourceStartOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalSourceStartOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsHorizontalShiftClamped(&self) -> ::windows::core::Result<bool>;
    fn SetIsHorizontalShiftClamped(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVerticalShiftClamped(&self) -> ::windows::core::Result<bool>;
    fn SetIsVerticalShiftClamped(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxHorizontalShiftRatio(&self) -> ::windows::core::Result<f64>;
    fn SetMaxHorizontalShiftRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxVerticalShiftRatio(&self) -> ::windows::core::Result<f64>;
    fn SetMaxVerticalShiftRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetSource(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn VerticalShift(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalShift(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalSourceEndOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalSourceEndOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalSourceOffsetKind(&self) -> ::windows::core::Result<ParallaxSourceOffsetKind>;
    fn SetVerticalSourceOffsetKind(&self, value: ParallaxSourceOffsetKind) -> ::windows::core::Result<()>;
    fn VerticalSourceStartOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalSourceStartOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn RefreshAutomaticHorizontalOffsets(&self) -> ::windows::core::Result<()>;
    fn RefreshAutomaticVerticalOffsets(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IParallaxViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ParallaxView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IParallaxViewStaticsImpl: Sized {
    fn ChildProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalSourceEndOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalSourceOffsetKindProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalSourceStartOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxHorizontalShiftRatioProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalShiftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHorizontalShiftClampedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsVerticalShiftClampedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalSourceEndOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalSourceOffsetKindProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalSourceStartOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxVerticalShiftRatioProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalShiftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxImpl: Sized {
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PasswordChar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPasswordChar(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsPasswordRevealButtonEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPasswordRevealButtonEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn PasswordChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePasswordChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextMenuOpening(&self, handler: &::core::option::Option<ContextMenuOpeningEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextMenuOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBox2Impl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionHighlightColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn PreventKeyboardDisplayOnProgrammaticFocus(&self) -> ::windows::core::Result<bool>;
    fn SetPreventKeyboardDisplayOnProgrammaticFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn Paste(&self, handler: &::core::option::Option<TextControlPasteEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaste(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBox3Impl: Sized {
    fn PasswordRevealMode(&self) -> ::windows::core::Result<PasswordRevealMode>;
    fn SetPasswordRevealMode(&self, value: PasswordRevealMode) -> ::windows::core::Result<()>;
    fn TextReadingOrder(&self) -> ::windows::core::Result<super::TextReadingOrder>;
    fn SetTextReadingOrder(&self, value: super::TextReadingOrder) -> ::windows::core::Result<()>;
    fn InputScope(&self) -> ::windows::core::Result<super::Input::InputScope>;
    fn SetInputScope(&self, value: &::core::option::Option<super::Input::InputScope>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBox4Impl: Sized {
    fn PasswordChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PasswordBox, PasswordBoxPasswordChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePasswordChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBox5Impl: Sized {
    fn CanPasteClipboardContent(&self) -> ::windows::core::Result<bool>;
    fn SelectionFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetSelectionFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PasteFromClipboard(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxPasswordChangingEventArgsImpl: Sized {
    fn IsContentChanging(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxStaticsImpl: Sized {
    fn PasswordProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PasswordCharProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPasswordRevealButtonEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxStatics2Impl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionHighlightColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PreventKeyboardDisplayOnProgrammaticFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxStatics3Impl: Sized {
    fn PasswordRevealModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextReadingOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InputScopeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxStatics5Impl: Sized {
    fn CanPasteClipboardContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::Media::Geometry>;
    fn SetData(&self, value: &::core::option::Option<super::Media::Geometry>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PathIcon>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconSourceImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::Media::Geometry>;
    fn SetData(&self, value: &::core::option::Option<super::Media::Geometry>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PathIconSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconSourceStaticsImpl: Sized {
    fn DataProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIconStaticsImpl: Sized {
    fn DataProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureImpl: Sized {
    fn BadgeNumber(&self) -> ::windows::core::Result<i32>;
    fn SetBadgeNumber(&self, value: i32) -> ::windows::core::Result<()>;
    fn BadgeGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBadgeGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BadgeImageSource(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetBadgeImageSource(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn BadgeText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBadgeText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsGroup(&self) -> ::windows::core::Result<bool>;
    fn SetIsGroup(&self, value: bool) -> ::windows::core::Result<()>;
    fn Contact(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Contacts::Contact>;
    fn SetContact(&self, value: &::core::option::Option<super::super::super::ApplicationModel::Contacts::Contact>) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Initials(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetInitials(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PreferSmallImage(&self) -> ::windows::core::Result<bool>;
    fn SetPreferSmallImage(&self, value: bool) -> ::windows::core::Result<()>;
    fn ProfilePicture(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetProfilePicture(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PersonPicture>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureStaticsImpl: Sized {
    fn BadgeNumberProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BadgeGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BadgeImageSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BadgeTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsGroupProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContactProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayNameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InitialsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PreferSmallImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProfilePictureProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerConfirmedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ConfirmationButtonsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetConfirmationButtonsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Confirmed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PickerFlyout, PickerConfirmedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveConfirmed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAtAsync(&self, target: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ConfirmationButtonsVisibleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTitle(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TitleTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetTitleTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn IsLocked(&self) -> ::windows::core::Result<bool>;
    fn SetIsLocked(&self, value: bool) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PivotItemLoading(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Pivot, PivotItemEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePivotItemLoading(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PivotItemLoaded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Pivot, PivotItemEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePivotItemLoaded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PivotItemUnloading(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Pivot, PivotItemEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePivotItemUnloading(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PivotItemUnloaded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Pivot, PivotItemEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePivotItemUnloaded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivot2Impl: Sized {
    fn LeftHeader(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetLeftHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LeftHeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetLeftHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn RightHeader(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetRightHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RightHeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetRightHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivot3Impl: Sized {
    fn HeaderFocusVisualPlacement(&self) -> ::windows::core::Result<PivotHeaderFocusVisualPlacement>;
    fn SetHeaderFocusVisualPlacement(&self, value: PivotHeaderFocusVisualPlacement) -> ::windows::core::Result<()>;
    fn IsHeaderItemsCarouselEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHeaderItemsCarouselEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Pivot>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<PivotItem>;
    fn SetItem(&self, value: &::core::option::Option<PivotItem>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PivotItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TitleTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsLockedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SlideInAnimationGroupProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSlideInAnimationGroup(&self, element: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<PivotSlideInAnimationGroup>;
    fn SetSlideInAnimationGroup(&self, element: &::core::option::Option<super::FrameworkElement>, value: PivotSlideInAnimationGroup) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotStatics2Impl: Sized {
    fn LeftHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LeftHeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RightHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RightHeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotStatics3Impl: Sized {
    fn HeaderFocusVisualPlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHeaderItemsCarouselEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarImpl: Sized {
    fn IsIndeterminate(&self) -> ::windows::core::Result<bool>;
    fn SetIsIndeterminate(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowError(&self) -> ::windows::core::Result<bool>;
    fn SetShowError(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowPaused(&self) -> ::windows::core::Result<bool>;
    fn SetShowPaused(&self, value: bool) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ProgressBarTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarStaticsImpl: Sized {
    fn IsIndeterminateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ShowErrorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ShowPausedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ProgressRingTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingStaticsImpl: Sized {
    fn IsActiveProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonImpl: Sized {
    fn GroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroupName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RadioButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonStaticsImpl: Sized {
    fn GroupNameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlImpl: Sized {
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InitialSetValue(&self) -> ::windows::core::Result<i32>;
    fn SetInitialSetValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsClearEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsClearEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxRating(&self) -> ::windows::core::Result<i32>;
    fn SetMaxRating(&self, value: i32) -> ::windows::core::Result<()>;
    fn PlaceholderValue(&self) -> ::windows::core::Result<f64>;
    fn SetPlaceholderValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn ItemInfo(&self) -> ::windows::core::Result<RatingItemInfo>;
    fn SetItemInfo(&self, value: &::core::option::Option<RatingItemInfo>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn ValueChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RatingControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlStaticsImpl: Sized {
    fn CaptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InitialSetValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsClearEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxRatingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemInfoProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemFontInfoImpl: Sized {
    fn DisabledGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisabledGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Glyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PointerOverGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPointerOverGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PointerOverPlaceholderGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPointerOverPlaceholderGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UnsetGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnsetGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemFontInfoFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingItemFontInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemFontInfoStaticsImpl: Sized {
    fn DisabledGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointerOverGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointerOverPlaceholderGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn UnsetGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemImageInfoImpl: Sized {
    fn DisabledImage(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetDisabledImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn PlaceholderImage(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetPlaceholderImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn PointerOverImage(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetPointerOverImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn PointerOverPlaceholderImage(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetPointerOverPlaceholderImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn UnsetImage(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetUnsetImage(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemImageInfoFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingItemImageInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemImageInfoStaticsImpl: Sized {
    fn DisabledImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointerOverImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointerOverPlaceholderImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn UnsetImageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingItemInfoFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingItemInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshContainerImpl: Sized {
    fn Visualizer(&self) -> ::windows::core::Result<RefreshVisualizer>;
    fn SetVisualizer(&self, value: &::core::option::Option<RefreshVisualizer>) -> ::windows::core::Result<()>;
    fn PullDirection(&self) -> ::windows::core::Result<RefreshPullDirection>;
    fn SetPullDirection(&self, value: RefreshPullDirection) -> ::windows::core::Result<()>;
    fn RefreshRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RefreshContainer, RefreshRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRefreshRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestRefresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshContainerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RefreshContainer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshContainerStaticsImpl: Sized {
    fn VisualizerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PullDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshInteractionRatioChangedEventArgsImpl: Sized {
    fn InteractionRatio(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshRequestedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<RefreshVisualizerState>;
    fn NewState(&self) -> ::windows::core::Result<RefreshVisualizerState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshVisualizerImpl: Sized {
    fn RequestRefresh(&self) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<RefreshVisualizerOrientation>;
    fn SetOrientation(&self, value: RefreshVisualizerOrientation) -> ::windows::core::Result<()>;
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<RefreshVisualizerState>;
    fn RefreshRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RefreshVisualizer, RefreshRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRefreshRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RefreshStateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RefreshVisualizer, RefreshStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRefreshStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshVisualizerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RefreshVisualizer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRefreshVisualizerStaticsImpl: Sized {
    fn InfoProviderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativePanelImpl: Sized {
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativePanel2Impl: Sized {
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativePanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RelativePanel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativePanelStaticsImpl: Sized {
    fn LeftOfProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLeftOf(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetLeftOf(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AboveProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAbove(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAbove(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RightOfProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetRightOf(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetRightOf(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BelowProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetBelow(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetBelow(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignHorizontalCenterWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignHorizontalCenterWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignHorizontalCenterWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignVerticalCenterWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignVerticalCenterWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignVerticalCenterWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignLeftWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignLeftWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignLeftWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignTopWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignTopWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignTopWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignRightWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignRightWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignRightWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignBottomWithProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignBottomWith(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAlignBottomWith(&self, element: &::core::option::Option<super::UIElement>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AlignLeftWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignLeftWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignLeftWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn AlignTopWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignTopWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignTopWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn AlignRightWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignRightWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignRightWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn AlignBottomWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignBottomWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignBottomWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn AlignHorizontalCenterWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignHorizontalCenterWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignHorizontalCenterWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn AlignVerticalCenterWithPanelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAlignVerticalCenterWithPanel(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetAlignVerticalCenterWithPanel(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativePanelStatics2Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn AcceptsReturn(&self) -> ::windows::core::Result<bool>;
    fn SetAcceptsReturn(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn TextWrapping(&self) -> ::windows::core::Result<super::TextWrapping>;
    fn SetTextWrapping(&self, value: super::TextWrapping) -> ::windows::core::Result<()>;
    fn IsSpellCheckEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSpellCheckEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTextPredictionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextPredictionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Document(&self) -> ::windows::core::Result<super::super::Text::ITextDocument>;
    fn InputScope(&self) -> ::windows::core::Result<super::Input::InputScope>;
    fn SetInputScope(&self, value: &::core::option::Option<super::Input::InputScope>) -> ::windows::core::Result<()>;
    fn TextChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextMenuOpening(&self, handler: &::core::option::Option<ContextMenuOpeningEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextMenuOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox2Impl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionHighlightColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn PreventKeyboardDisplayOnProgrammaticFocus(&self) -> ::windows::core::Result<bool>;
    fn SetPreventKeyboardDisplayOnProgrammaticFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Paste(&self, handler: &::core::option::Option<TextControlPasteEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaste(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox3Impl: Sized {
    fn TextCompositionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, TextCompositionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextCompositionChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, TextCompositionChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextCompositionEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, TextCompositionEndedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionEnded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextReadingOrder(&self) -> ::windows::core::Result<super::TextReadingOrder>;
    fn SetTextReadingOrder(&self, value: super::TextReadingOrder) -> ::windows::core::Result<()>;
    fn DesiredCandidateWindowAlignment(&self) -> ::windows::core::Result<CandidateWindowAlignment>;
    fn SetDesiredCandidateWindowAlignment(&self, value: CandidateWindowAlignment) -> ::windows::core::Result<()>;
    fn CandidateWindowBoundsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, CandidateWindowBoundsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCandidateWindowBoundsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, RichEditBoxTextChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox4Impl: Sized {
    fn GetLinguisticAlternativesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn ClipboardCopyFormat(&self) -> ::windows::core::Result<RichEditClipboardFormat>;
    fn SetClipboardCopyFormat(&self, value: RichEditClipboardFormat) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox5Impl: Sized {
    fn SelectionHighlightColorWhenNotFocused(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColorWhenNotFocused(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLength(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox6Impl: Sized {
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn CharacterCasing(&self) -> ::windows::core::Result<CharacterCasing>;
    fn SetCharacterCasing(&self, value: CharacterCasing) -> ::windows::core::Result<()>;
    fn DisabledFormattingAccelerators(&self) -> ::windows::core::Result<DisabledFormattingAccelerators>;
    fn SetDisabledFormattingAccelerators(&self, value: DisabledFormattingAccelerators) -> ::windows::core::Result<()>;
    fn CopyingToClipboard(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, TextControlCopyingToClipboardEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCopyingToClipboard(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CuttingToClipboard(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, TextControlCuttingToClipboardEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCuttingToClipboard(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox7Impl: Sized {
    fn ContentLinkForegroundColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetContentLinkForegroundColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn ContentLinkBackgroundColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetContentLinkBackgroundColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn ContentLinkProviders(&self) -> ::windows::core::Result<super::Documents::ContentLinkProviderCollection>;
    fn SetContentLinkProviders(&self, value: &::core::option::Option<super::Documents::ContentLinkProviderCollection>) -> ::windows::core::Result<()>;
    fn HandwritingView(&self) -> ::windows::core::Result<HandwritingView>;
    fn SetHandwritingView(&self, value: &::core::option::Option<HandwritingView>) -> ::windows::core::Result<()>;
    fn IsHandwritingViewEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHandwritingViewEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ContentLinkChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, ContentLinkChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLinkChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContentLinkInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, super::Documents::ContentLinkInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLinkInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBox8Impl: Sized {
    fn TextDocument(&self) -> ::windows::core::Result<super::super::Text::RichEditTextDocument>;
    fn SelectionFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetSelectionFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn ProofingMenuFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectionChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichEditBox, RichEditBoxSelectionChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichEditBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxSelectionChangingEventArgsImpl: Sized {
    fn SelectionStart(&self) -> ::windows::core::Result<i32>;
    fn SelectionLength(&self) -> ::windows::core::Result<i32>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStaticsImpl: Sized {
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AcceptsReturnProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextWrappingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSpellCheckEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextPredictionEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InputScopeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics2Impl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionHighlightColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PreventKeyboardDisplayOnProgrammaticFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics3Impl: Sized {
    fn DesiredCandidateWindowAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextReadingOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics4Impl: Sized {
    fn ClipboardCopyFormatProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics5Impl: Sized {
    fn SelectionHighlightColorWhenNotFocusedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics6Impl: Sized {
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterCasingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisabledFormattingAcceleratorsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics7Impl: Sized {
    fn ContentLinkForegroundColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentLinkBackgroundColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ContentLinkProvidersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HandwritingViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHandwritingViewEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxStatics8Impl: Sized {
    fn SelectionFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProofingMenuFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxTextChangingEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxTextChangingEventArgs2Impl: Sized {
    fn IsContentChanging(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockImpl: Sized {
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
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn TextWrapping(&self) -> ::windows::core::Result<super::TextWrapping>;
    fn SetTextWrapping(&self, value: super::TextWrapping) -> ::windows::core::Result<()>;
    fn TextTrimming(&self) -> ::windows::core::Result<super::TextTrimming>;
    fn SetTextTrimming(&self, value: super::TextTrimming) -> ::windows::core::Result<()>;
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn Blocks(&self) -> ::windows::core::Result<super::Documents::BlockCollection>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<f64>;
    fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy>;
    fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()>;
    fn CharacterSpacing(&self) -> ::windows::core::Result<i32>;
    fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()>;
    fn OverflowContentTarget(&self) -> ::windows::core::Result<RichTextBlockOverflow>;
    fn SetOverflowContentTarget(&self, value: &::core::option::Option<RichTextBlockOverflow>) -> ::windows::core::Result<()>;
    fn IsTextSelectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextSelectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn HasOverflowContent(&self) -> ::windows::core::Result<bool>;
    fn SelectedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentStart(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn ContentEnd(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn SelectionStart(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn SelectionEnd(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn BaselineOffset(&self) -> ::windows::core::Result<f64>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextMenuOpening(&self, handler: &::core::option::Option<ContextMenuOpeningEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextMenuOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn Select(&self, start: &::core::option::Option<super::Documents::TextPointer>, end: &::core::option::Option<super::Documents::TextPointer>) -> ::windows::core::Result<()>;
    fn GetPositionFromPoint(&self, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
    fn TextIndent(&self) -> ::windows::core::Result<f64>;
    fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlock2Impl: Sized {
    fn MaxLines(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLines(&self, value: i32) -> ::windows::core::Result<()>;
    fn TextLineBounds(&self) -> ::windows::core::Result<super::TextLineBounds>;
    fn SetTextLineBounds(&self, value: super::TextLineBounds) -> ::windows::core::Result<()>;
    fn SelectionHighlightColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn OpticalMarginAlignment(&self) -> ::windows::core::Result<super::OpticalMarginAlignment>;
    fn SetOpticalMarginAlignment(&self, value: super::OpticalMarginAlignment) -> ::windows::core::Result<()>;
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextReadingOrder(&self) -> ::windows::core::Result<super::TextReadingOrder>;
    fn SetTextReadingOrder(&self, value: super::TextReadingOrder) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlock3Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlock4Impl: Sized {
    fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations>;
    fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlock5Impl: Sized {
    fn IsTextTrimmed(&self) -> ::windows::core::Result<bool>;
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn TextHighlighters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::Documents::TextHighlighter>>;
    fn IsTextTrimmedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichTextBlock, IsTextTrimmedChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsTextTrimmedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlock6Impl: Sized {
    fn SelectionFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetSelectionFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn CopySelectionToClipboard(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowImpl: Sized {
    fn OverflowContentTarget(&self) -> ::windows::core::Result<RichTextBlockOverflow>;
    fn SetOverflowContentTarget(&self, value: &::core::option::Option<RichTextBlockOverflow>) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn ContentSource(&self) -> ::windows::core::Result<RichTextBlock>;
    fn HasOverflowContent(&self) -> ::windows::core::Result<bool>;
    fn ContentStart(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn ContentEnd(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn BaselineOffset(&self) -> ::windows::core::Result<f64>;
    fn GetPositionFromPoint(&self, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflow2Impl: Sized {
    fn MaxLines(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLines(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflow3Impl: Sized {
    fn IsTextTrimmed(&self) -> ::windows::core::Result<bool>;
    fn IsTextTrimmedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RichTextBlockOverflow, IsTextTrimmedChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsTextTrimmedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowStaticsImpl: Sized {
    fn OverflowContentTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HasOverflowContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowStatics2Impl: Sized {
    fn MaxLinesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowStatics3Impl: Sized {
    fn IsTextTrimmedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStaticsImpl: Sized {
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextWrappingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextTrimmingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineStackingStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OverflowContentTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextSelectionEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HasOverflowContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextIndentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStatics2Impl: Sized {
    fn MaxLinesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextLineBoundsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionHighlightColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OpticalMarginAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextReadingOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStatics3Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStatics4Impl: Sized {
    fn TextDecorationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStatics5Impl: Sized {
    fn IsTextTrimmedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockStatics6Impl: Sized {
    fn SelectionFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRowDefinitionImpl: Sized {
    fn Height(&self) -> ::windows::core::Result<super::GridLength>;
    fn SetHeight(&self, value: &super::GridLength) -> ::windows::core::Result<()>;
    fn MaxHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn ActualHeight(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRowDefinitionStaticsImpl: Sized {
    fn HeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
pub trait IScrollAnchorProviderImpl: Sized {
    fn CurrentAnchor(&self) -> ::windows::core::Result<super::UIElement>;
    fn RegisterAnchorCandidate(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn UnregisterAnchorCandidate(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollContentPresenterImpl: Sized {
    fn CanVerticallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanVerticallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanHorizontallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExtentWidth(&self) -> ::windows::core::Result<f64>;
    fn ExtentHeight(&self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn ScrollOwner(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetScrollOwner(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LineUp(&self) -> ::windows::core::Result<()>;
    fn LineDown(&self) -> ::windows::core::Result<()>;
    fn LineLeft(&self) -> ::windows::core::Result<()>;
    fn LineRight(&self) -> ::windows::core::Result<()>;
    fn PageUp(&self) -> ::windows::core::Result<()>;
    fn PageDown(&self) -> ::windows::core::Result<()>;
    fn PageLeft(&self) -> ::windows::core::Result<()>;
    fn PageRight(&self) -> ::windows::core::Result<()>;
    fn MouseWheelUp(&self) -> ::windows::core::Result<()>;
    fn MouseWheelDown(&self) -> ::windows::core::Result<()>;
    fn MouseWheelLeft(&self) -> ::windows::core::Result<()>;
    fn MouseWheelRight(&self) -> ::windows::core::Result<()>;
    fn SetHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn SetVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn MakeVisible(&self, visual: &::core::option::Option<super::UIElement>, rectangle: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollContentPresenter2Impl: Sized {
    fn CanContentRenderOutsideBounds(&self) -> ::windows::core::Result<bool>;
    fn SetCanContentRenderOutsideBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn SizesContentToTemplatedParent(&self) -> ::windows::core::Result<bool>;
    fn SetSizesContentToTemplatedParent(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollContentPresenterStatics2Impl: Sized {
    fn CanContentRenderOutsideBoundsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SizesContentToTemplatedParentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerImpl: Sized {
    fn HorizontalScrollBarVisibility(&self) -> ::windows::core::Result<ScrollBarVisibility>;
    fn SetHorizontalScrollBarVisibility(&self, value: ScrollBarVisibility) -> ::windows::core::Result<()>;
    fn VerticalScrollBarVisibility(&self) -> ::windows::core::Result<ScrollBarVisibility>;
    fn SetVerticalScrollBarVisibility(&self, value: ScrollBarVisibility) -> ::windows::core::Result<()>;
    fn IsHorizontalRailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHorizontalRailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVerticalRailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVerticalRailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHorizontalScrollChainingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHorizontalScrollChainingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVerticalScrollChainingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVerticalScrollChainingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomChainingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomChainingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsScrollInertiaEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScrollInertiaEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomInertiaEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomInertiaEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn HorizontalScrollMode(&self) -> ::windows::core::Result<ScrollMode>;
    fn SetHorizontalScrollMode(&self, value: ScrollMode) -> ::windows::core::Result<()>;
    fn VerticalScrollMode(&self) -> ::windows::core::Result<ScrollMode>;
    fn SetVerticalScrollMode(&self, value: ScrollMode) -> ::windows::core::Result<()>;
    fn ZoomMode(&self) -> ::windows::core::Result<ZoomMode>;
    fn SetZoomMode(&self, value: ZoomMode) -> ::windows::core::Result<()>;
    fn HorizontalSnapPointsAlignment(&self) -> ::windows::core::Result<Primitives::SnapPointsAlignment>;
    fn SetHorizontalSnapPointsAlignment(&self, value: Primitives::SnapPointsAlignment) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsAlignment(&self) -> ::windows::core::Result<Primitives::SnapPointsAlignment>;
    fn SetVerticalSnapPointsAlignment(&self, value: Primitives::SnapPointsAlignment) -> ::windows::core::Result<()>;
    fn HorizontalSnapPointsType(&self) -> ::windows::core::Result<SnapPointsType>;
    fn SetHorizontalSnapPointsType(&self, value: SnapPointsType) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsType(&self) -> ::windows::core::Result<SnapPointsType>;
    fn SetVerticalSnapPointsType(&self, value: SnapPointsType) -> ::windows::core::Result<()>;
    fn ZoomSnapPointsType(&self) -> ::windows::core::Result<SnapPointsType>;
    fn SetZoomSnapPointsType(&self, value: SnapPointsType) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&self) -> ::windows::core::Result<f64>;
    fn ScrollableWidth(&self) -> ::windows::core::Result<f64>;
    fn ComputedHorizontalScrollBarVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn ExtentWidth(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&self) -> ::windows::core::Result<f64>;
    fn ScrollableHeight(&self) -> ::windows::core::Result<f64>;
    fn ComputedVerticalScrollBarVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn ExtentHeight(&self) -> ::windows::core::Result<f64>;
    fn MinZoomFactor(&self) -> ::windows::core::Result<f32>;
    fn SetMinZoomFactor(&self, value: f32) -> ::windows::core::Result<()>;
    fn MaxZoomFactor(&self) -> ::windows::core::Result<f32>;
    fn SetMaxZoomFactor(&self, value: f32) -> ::windows::core::Result<()>;
    fn ZoomFactor(&self) -> ::windows::core::Result<f32>;
    fn ZoomSnapPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<f32>>;
    fn ViewChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<ScrollViewerViewChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScrollToHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn ScrollToVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn ZoomToFactor(&self, factor: f32) -> ::windows::core::Result<()>;
    fn InvalidateScrollInfo(&self) -> ::windows::core::Result<()>;
    fn IsDeferredScrollingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDeferredScrollingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn BringIntoViewOnFocusChange(&self) -> ::windows::core::Result<bool>;
    fn SetBringIntoViewOnFocusChange(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewer2Impl: Sized {
    fn TopLeftHeader(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetTopLeftHeader(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn LeftHeader(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetLeftHeader(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn TopHeader(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetTopHeader(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ViewChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<ScrollViewerViewChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChangeView(&self, horizontaloffset: &::core::option::Option<super::super::super::Foundation::IReference<f64>>, verticaloffset: &::core::option::Option<super::super::super::Foundation::IReference<f64>>, zoomfactor: &::core::option::Option<super::super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<bool>;
    fn ChangeViewWithOptionalAnimation(&self, horizontaloffset: &::core::option::Option<super::super::super::Foundation::IReference<f64>>, verticaloffset: &::core::option::Option<super::super::super::Foundation::IReference<f64>>, zoomfactor: &::core::option::Option<super::super::super::Foundation::IReference<f32>>, disableanimation: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewer3Impl: Sized {
    fn DirectManipulationStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDirectManipulationStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DirectManipulationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDirectManipulationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewer4Impl: Sized {
    fn ReduceViewportForCoreInputViewOcclusions(&self) -> ::windows::core::Result<bool>;
    fn SetReduceViewportForCoreInputViewOcclusions(&self, value: bool) -> ::windows::core::Result<()>;
    fn HorizontalAnchorRatio(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalAnchorRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalAnchorRatio(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalAnchorRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn CanContentRenderOutsideBounds(&self) -> ::windows::core::Result<bool>;
    fn SetCanContentRenderOutsideBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn AnchorRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ScrollViewer, AnchorRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnchorRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerStaticsImpl: Sized {
    fn HorizontalSnapPointsAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalSnapPointsAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalSnapPointsTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalSnapPointsTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ZoomSnapPointsTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ViewportWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScrollableWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ComputedHorizontalScrollBarVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExtentWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ViewportHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScrollableHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ComputedVerticalScrollBarVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExtentHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinZoomFactorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxZoomFactorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ZoomFactorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ZoomSnapPointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalScrollBarVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHorizontalScrollBarVisibility(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ScrollBarVisibility>;
    fn SetHorizontalScrollBarVisibility(&self, element: &::core::option::Option<super::DependencyObject>, horizontalscrollbarvisibility: ScrollBarVisibility) -> ::windows::core::Result<()>;
    fn VerticalScrollBarVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetVerticalScrollBarVisibility(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ScrollBarVisibility>;
    fn SetVerticalScrollBarVisibility(&self, element: &::core::option::Option<super::DependencyObject>, verticalscrollbarvisibility: ScrollBarVisibility) -> ::windows::core::Result<()>;
    fn IsHorizontalRailEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsHorizontalRailEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsHorizontalRailEnabled(&self, element: &::core::option::Option<super::DependencyObject>, ishorizontalrailenabled: bool) -> ::windows::core::Result<()>;
    fn IsVerticalRailEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsVerticalRailEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsVerticalRailEnabled(&self, element: &::core::option::Option<super::DependencyObject>, isverticalrailenabled: bool) -> ::windows::core::Result<()>;
    fn IsHorizontalScrollChainingEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsHorizontalScrollChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsHorizontalScrollChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>, ishorizontalscrollchainingenabled: bool) -> ::windows::core::Result<()>;
    fn IsVerticalScrollChainingEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsVerticalScrollChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsVerticalScrollChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>, isverticalscrollchainingenabled: bool) -> ::windows::core::Result<()>;
    fn IsZoomChainingEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsZoomChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsZoomChainingEnabled(&self, element: &::core::option::Option<super::DependencyObject>, iszoomchainingenabled: bool) -> ::windows::core::Result<()>;
    fn IsScrollInertiaEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsScrollInertiaEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsScrollInertiaEnabled(&self, element: &::core::option::Option<super::DependencyObject>, isscrollinertiaenabled: bool) -> ::windows::core::Result<()>;
    fn IsZoomInertiaEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsZoomInertiaEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsZoomInertiaEnabled(&self, element: &::core::option::Option<super::DependencyObject>, iszoominertiaenabled: bool) -> ::windows::core::Result<()>;
    fn HorizontalScrollModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHorizontalScrollMode(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ScrollMode>;
    fn SetHorizontalScrollMode(&self, element: &::core::option::Option<super::DependencyObject>, horizontalscrollmode: ScrollMode) -> ::windows::core::Result<()>;
    fn VerticalScrollModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetVerticalScrollMode(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ScrollMode>;
    fn SetVerticalScrollMode(&self, element: &::core::option::Option<super::DependencyObject>, verticalscrollmode: ScrollMode) -> ::windows::core::Result<()>;
    fn ZoomModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetZoomMode(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<ZoomMode>;
    fn SetZoomMode(&self, element: &::core::option::Option<super::DependencyObject>, zoommode: ZoomMode) -> ::windows::core::Result<()>;
    fn IsDeferredScrollingEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDeferredScrollingEnabled(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDeferredScrollingEnabled(&self, element: &::core::option::Option<super::DependencyObject>, isdeferredscrollingenabled: bool) -> ::windows::core::Result<()>;
    fn BringIntoViewOnFocusChangeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetBringIntoViewOnFocusChange(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetBringIntoViewOnFocusChange(&self, element: &::core::option::Option<super::DependencyObject>, bringintoviewonfocuschange: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerStatics2Impl: Sized {
    fn TopLeftHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LeftHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TopHeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerStatics4Impl: Sized {
    fn ReduceViewportForCoreInputViewOcclusionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalAnchorRatioProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalAnchorRatioProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanContentRenderOutsideBoundsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCanContentRenderOutsideBounds(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetCanContentRenderOutsideBounds(&self, element: &::core::option::Option<super::DependencyObject>, cancontentrenderoutsidebounds: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerViewImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn ZoomFactor(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerViewChangedEventArgsImpl: Sized {
    fn IsIntermediate(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerViewChangingEventArgsImpl: Sized {
    fn NextView(&self) -> ::windows::core::Result<ScrollViewerView>;
    fn FinalView(&self) -> ::windows::core::Result<ScrollViewerView>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxImpl: Sized {
    fn SearchHistoryEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SearchHistoryContext(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSearchHistoryContext(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetQueryText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FocusOnKeyboardInput(&self) -> ::windows::core::Result<bool>;
    fn SetFocusOnKeyboardInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn ChooseSuggestionOnEnter(&self) -> ::windows::core::Result<bool>;
    fn SetChooseSuggestionOnEnter(&self, value: bool) -> ::windows::core::Result<()>;
    fn QueryChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchBox, SearchBoxQueryChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveQueryChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuggestionsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchBox, SearchBoxSuggestionsRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuggestionsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuerySubmitted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchBox, SearchBoxQuerySubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuerySubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResultSuggestionChosen(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchBox, SearchBoxResultSuggestionChosenEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveResultSuggestionChosen(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrepareForFocusOnKeyboardInput(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SearchBox, super::RoutedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrepareForFocusOnKeyboardInput(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetLocalContentSuggestionSettings(&self, settings: &::core::option::Option<super::super::super::ApplicationModel::Search::LocalContentSuggestionSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SearchBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxQueryChangedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Search::SearchQueryLinguisticDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxQuerySubmittedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Search::SearchQueryLinguisticDetails>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxResultSuggestionChosenEventArgsImpl: Sized {
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxStaticsImpl: Sized {
    fn SearchHistoryEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SearchHistoryContextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn QueryTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FocusOnKeyboardInputProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChooseSuggestionOnEnterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxSuggestionsRequestedEventArgsImpl: Sized {
    fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Search::SearchQueryLinguisticDetails>;
    fn Request(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Search::SearchSuggestionsRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISectionsInViewChangedEventArgsImpl: Sized {
    fn AddedSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HubSection>>;
    fn RemovedSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HubSection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISectionsInViewChangedEventArgsFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionChangedEventArgsImpl: Sized {
    fn AddedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn RemovedItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionChangedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithRemovedItemsAndAddedItems(&self, removeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>, addeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectionChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomImpl: Sized {
    fn ZoomedInView(&self) -> ::windows::core::Result<ISemanticZoomInformation>;
    fn SetZoomedInView(&self, value: &::core::option::Option<ISemanticZoomInformation>) -> ::windows::core::Result<()>;
    fn ZoomedOutView(&self) -> ::windows::core::Result<ISemanticZoomInformation>;
    fn SetZoomedOutView(&self, value: &::core::option::Option<ISemanticZoomInformation>) -> ::windows::core::Result<()>;
    fn IsZoomedInViewActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomedInViewActive(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanChangeViews(&self) -> ::windows::core::Result<bool>;
    fn SetCanChangeViews(&self, value: bool) -> ::windows::core::Result<()>;
    fn ViewChangeStarted(&self, handler: &::core::option::Option<SemanticZoomViewChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewChangeStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ViewChangeCompleted(&self, handler: &::core::option::Option<SemanticZoomViewChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewChangeCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ToggleActiveView(&self) -> ::windows::core::Result<()>;
    fn IsZoomOutButtonEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomOutButtonEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait ISemanticZoomInformationImpl: Sized {
    fn SemanticZoomOwner(&self) -> ::windows::core::Result<SemanticZoom>;
    fn SetSemanticZoomOwner(&self, value: &::core::option::Option<SemanticZoom>) -> ::windows::core::Result<()>;
    fn IsActiveView(&self) -> ::windows::core::Result<bool>;
    fn SetIsActiveView(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsZoomedInView(&self) -> ::windows::core::Result<bool>;
    fn SetIsZoomedInView(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitializeViewChange(&self) -> ::windows::core::Result<()>;
    fn CompleteViewChange(&self) -> ::windows::core::Result<()>;
    fn MakeVisible(&self, item: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeFrom(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn StartViewChangeTo(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeFrom(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn CompleteViewChangeTo(&self, source: &::core::option::Option<SemanticZoomLocation>, destination: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomLocationImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomStaticsImpl: Sized {
    fn ZoomedInViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ZoomedOutViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomedInViewActiveProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanChangeViewsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsZoomOutButtonEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomViewChangedEventArgsImpl: Sized {
    fn IsSourceZoomedInView(&self) -> ::windows::core::Result<bool>;
    fn SetIsSourceZoomedInView(&self, value: bool) -> ::windows::core::Result<()>;
    fn SourceItem(&self) -> ::windows::core::Result<SemanticZoomLocation>;
    fn SetSourceItem(&self, value: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
    fn DestinationItem(&self) -> ::windows::core::Result<SemanticZoomLocation>;
    fn SetDestinationItem(&self, value: &::core::option::Option<SemanticZoomLocation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HeaderBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetHeaderBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn HeaderForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetHeaderForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn IconSource(&self) -> ::windows::core::Result<super::Media::ImageSource>;
    fn SetIconSource(&self, value: &::core::option::Option<super::Media::ImageSource>) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::SettingsFlyoutTemplateSettings>;
    fn BackClick(&self, handler: &::core::option::Option<BackClickEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn ShowIndependent(&self) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SettingsFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderImpl: Sized {
    fn IntermediateValue(&self) -> ::windows::core::Result<f64>;
    fn SetIntermediateValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn StepFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetStepFrequency(&self, value: f64) -> ::windows::core::Result<()>;
    fn SnapsTo(&self) -> ::windows::core::Result<Primitives::SliderSnapsTo>;
    fn SetSnapsTo(&self, value: Primitives::SliderSnapsTo) -> ::windows::core::Result<()>;
    fn TickFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetTickFrequency(&self, value: f64) -> ::windows::core::Result<()>;
    fn TickPlacement(&self) -> ::windows::core::Result<Primitives::TickPlacement>;
    fn SetTickPlacement(&self, value: Primitives::TickPlacement) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn IsDirectionReversed(&self) -> ::windows::core::Result<bool>;
    fn SetIsDirectionReversed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsThumbToolTipEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsThumbToolTipEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbToolTipValueConverter(&self) -> ::windows::core::Result<super::Data::IValueConverter>;
    fn SetThumbToolTipValueConverter(&self, value: &::core::option::Option<super::Data::IValueConverter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlider2Impl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Slider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderStaticsImpl: Sized {
    fn IntermediateValueProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StepFrequencyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SnapsToProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TickFrequencyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TickPlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsDirectionReversedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsThumbToolTipEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ThumbToolTipValueConverterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderStatics2Impl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonImpl: Sized {
    fn Flyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn Command(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SplitButton, SplitButtonClickEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<SplitButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SplitButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonClickEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SplitButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitButtonStaticsImpl: Sized {
    fn FlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pane(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPane(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn IsPaneOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaneOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn OpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenPaneLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn CompactPaneLength(&self) -> ::windows::core::Result<f64>;
    fn SetCompactPaneLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn PanePlacement(&self) -> ::windows::core::Result<SplitViewPanePlacement>;
    fn SetPanePlacement(&self, value: SplitViewPanePlacement) -> ::windows::core::Result<()>;
    fn DisplayMode(&self) -> ::windows::core::Result<SplitViewDisplayMode>;
    fn SetDisplayMode(&self, value: SplitViewDisplayMode) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::SplitViewTemplateSettings>;
    fn PaneBackground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetPaneBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PaneClosing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SplitView, SplitViewPaneClosingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneClosing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneClosed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SplitView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitView2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitView3Impl: Sized {
    fn PaneOpening(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SplitView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PaneOpened(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SplitView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaneOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SplitView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewPaneClosingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsPaneOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OpenPaneLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CompactPaneLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PanePlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DisplayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TemplateSettingsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaneBackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelImpl: Sized {
    fn AreScrollSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn SetAreScrollSnapPointsRegular(&self, value: bool) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanel2Impl: Sized {
    fn BorderBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBorderBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetBorderThickness(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn CornerRadius(&self) -> ::windows::core::Result<super::CornerRadius>;
    fn SetCornerRadius(&self, value: &super::CornerRadius) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanel4Impl: Sized {
    fn Spacing(&self) -> ::windows::core::Result<f64>;
    fn SetSpacing(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanel5Impl: Sized {
    fn BackgroundSizing(&self) -> ::windows::core::Result<BackgroundSizing>;
    fn SetBackgroundSizing(&self, value: BackgroundSizing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StackPanel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelStaticsImpl: Sized {
    fn AreScrollSnapPointsRegularProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelStatics2Impl: Sized {
    fn BorderBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BorderThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CornerRadiusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelStatics4Impl: Sized {
    fn SpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStackPanelStatics5Impl: Sized {
    fn BackgroundSizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleSelectorImpl: Sized {
    fn SelectStyle(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::Style>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleSelectorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StyleSelector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleSelectorOverridesImpl: Sized {
    fn SelectStyleCore(&self, item: &::core::option::Option<::windows::core::IInspectable>, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::Style>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainBackgroundPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainBackgroundPanel2Impl: Sized {
    fn CreateCoreIndependentInputSource(&self, devicetypes: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<super::super::Core::CoreIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainBackgroundPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SwapChainBackgroundPanel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainPanelImpl: Sized {
    fn CompositionScaleX(&self) -> ::windows::core::Result<f32>;
    fn CompositionScaleY(&self) -> ::windows::core::Result<f32>;
    fn CompositionScaleChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SwapChainPanel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompositionScaleChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateCoreIndependentInputSource(&self, devicetypes: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<super::super::Core::CoreIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SwapChainPanel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwapChainPanelStaticsImpl: Sized {
    fn CompositionScaleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CompositionScaleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeControlImpl: Sized {
    fn LeftItems(&self) -> ::windows::core::Result<SwipeItems>;
    fn SetLeftItems(&self, value: &::core::option::Option<SwipeItems>) -> ::windows::core::Result<()>;
    fn RightItems(&self) -> ::windows::core::Result<SwipeItems>;
    fn SetRightItems(&self, value: &::core::option::Option<SwipeItems>) -> ::windows::core::Result<()>;
    fn TopItems(&self) -> ::windows::core::Result<SwipeItems>;
    fn SetTopItems(&self, value: &::core::option::Option<SwipeItems>) -> ::windows::core::Result<()>;
    fn BottomItems(&self) -> ::windows::core::Result<SwipeItems>;
    fn SetBottomItems(&self, value: &::core::option::Option<SwipeItems>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SwipeControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeControlStaticsImpl: Sized {
    fn LeftItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RightItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TopItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BottomItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconSource(&self) -> ::windows::core::Result<IconSource>;
    fn SetIconSource(&self, value: &::core::option::Option<IconSource>) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Command(&self) -> ::windows::core::Result<super::Input::ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BehaviorOnInvoked(&self) -> ::windows::core::Result<SwipeBehaviorOnInvoked>;
    fn SetBehaviorOnInvoked(&self, value: SwipeBehaviorOnInvoked) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SwipeItem, SwipeItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SwipeItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemInvokedEventArgsImpl: Sized {
    fn SwipeControl(&self) -> ::windows::core::Result<SwipeControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemStaticsImpl: Sized {
    fn IconSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BackgroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandParameterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn BehaviorOnInvokedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<SwipeMode>;
    fn SetMode(&self, value: SwipeMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SwipeItems>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeItemsStaticsImpl: Sized {
    fn ModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconImpl: Sized {
    fn Symbol(&self) -> ::windows::core::Result<Symbol>;
    fn SetSymbol(&self, value: Symbol) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconFactoryImpl: Sized {
    fn CreateInstanceWithSymbol(&self, symbol: Symbol) -> ::windows::core::Result<SymbolIcon>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconSourceImpl: Sized {
    fn Symbol(&self) -> ::windows::core::Result<Symbol>;
    fn SetSymbol(&self, value: Symbol) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SymbolIconSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconSourceStaticsImpl: Sized {
    fn SymbolProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymbolIconStaticsImpl: Sized {
    fn SymbolProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockImpl: Sized {
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
    fn TextWrapping(&self) -> ::windows::core::Result<super::TextWrapping>;
    fn SetTextWrapping(&self, value: super::TextWrapping) -> ::windows::core::Result<()>;
    fn TextTrimming(&self) -> ::windows::core::Result<super::TextTrimming>;
    fn SetTextTrimming(&self, value: super::TextTrimming) -> ::windows::core::Result<()>;
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Inlines(&self) -> ::windows::core::Result<super::Documents::InlineCollection>;
    fn Padding(&self) -> ::windows::core::Result<super::Thickness>;
    fn SetPadding(&self, value: &super::Thickness) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<f64>;
    fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy>;
    fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()>;
    fn IsTextSelectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextSelectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SelectedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentStart(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn ContentEnd(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn SelectionStart(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn SelectionEnd(&self) -> ::windows::core::Result<super::Documents::TextPointer>;
    fn BaselineOffset(&self) -> ::windows::core::Result<f64>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextMenuOpening(&self, handler: &::core::option::Option<ContextMenuOpeningEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextMenuOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn Select(&self, start: &::core::option::Option<super::Documents::TextPointer>, end: &::core::option::Option<super::Documents::TextPointer>) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock2Impl: Sized {
    fn SelectionHighlightColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn MaxLines(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLines(&self, value: i32) -> ::windows::core::Result<()>;
    fn TextLineBounds(&self) -> ::windows::core::Result<super::TextLineBounds>;
    fn SetTextLineBounds(&self, value: super::TextLineBounds) -> ::windows::core::Result<()>;
    fn OpticalMarginAlignment(&self) -> ::windows::core::Result<super::OpticalMarginAlignment>;
    fn SetOpticalMarginAlignment(&self, value: super::OpticalMarginAlignment) -> ::windows::core::Result<()>;
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextReadingOrder(&self) -> ::windows::core::Result<super::TextReadingOrder>;
    fn SetTextReadingOrder(&self, value: super::TextReadingOrder) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock3Impl: Sized {
    fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock4Impl: Sized {
    fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock5Impl: Sized {
    fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations>;
    fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock6Impl: Sized {
    fn IsTextTrimmed(&self) -> ::windows::core::Result<bool>;
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn TextHighlighters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::Documents::TextHighlighter>>;
    fn IsTextTrimmedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBlock, IsTextTrimmedChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsTextTrimmedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlock7Impl: Sized {
    fn SelectionFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetSelectionFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn CopySelectionToClipboard(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStaticsImpl: Sized {
    fn FontSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontFamilyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontWeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FontStretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterSpacingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextWrappingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextTrimmingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PaddingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LineStackingStrategyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextSelectionEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectedTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStatics2Impl: Sized {
    fn SelectionHighlightColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxLinesProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextLineBoundsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OpticalMarginAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextReadingOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStatics3Impl: Sized {
    fn IsTextScaleFactorEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStatics5Impl: Sized {
    fn TextDecorationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStatics6Impl: Sized {
    fn IsTextTrimmedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockStatics7Impl: Sized {
    fn SelectionFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSelectedText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionLength(&self) -> ::windows::core::Result<i32>;
    fn SetSelectionLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectionStart(&self) -> ::windows::core::Result<i32>;
    fn SetSelectionStart(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn AcceptsReturn(&self) -> ::windows::core::Result<bool>;
    fn SetAcceptsReturn(&self, value: bool) -> ::windows::core::Result<()>;
    fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn TextWrapping(&self) -> ::windows::core::Result<super::TextWrapping>;
    fn SetTextWrapping(&self, value: super::TextWrapping) -> ::windows::core::Result<()>;
    fn IsSpellCheckEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSpellCheckEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTextPredictionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextPredictionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputScope(&self) -> ::windows::core::Result<super::Input::InputScope>;
    fn SetInputScope(&self, value: &::core::option::Option<super::Input::InputScope>) -> ::windows::core::Result<()>;
    fn TextChanged(&self, handler: &::core::option::Option<TextChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextMenuOpening(&self, handler: &::core::option::Option<ContextMenuOpeningEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextMenuOpening(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Select(&self, start: i32, length: i32) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn GetRectFromCharacterIndex(&self, charindex: i32, trailingedge: bool) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox2Impl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionHighlightColor(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColor(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
    fn PreventKeyboardDisplayOnProgrammaticFocus(&self) -> ::windows::core::Result<bool>;
    fn SetPreventKeyboardDisplayOnProgrammaticFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Paste(&self, handler: &::core::option::Option<TextControlPasteEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaste(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox3Impl: Sized {
    fn TextCompositionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextCompositionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextCompositionChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextCompositionChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextCompositionEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextCompositionEndedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextCompositionEnded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextReadingOrder(&self) -> ::windows::core::Result<super::TextReadingOrder>;
    fn SetTextReadingOrder(&self, value: super::TextReadingOrder) -> ::windows::core::Result<()>;
    fn DesiredCandidateWindowAlignment(&self) -> ::windows::core::Result<CandidateWindowAlignment>;
    fn SetDesiredCandidateWindowAlignment(&self, value: CandidateWindowAlignment) -> ::windows::core::Result<()>;
    fn CandidateWindowBoundsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, CandidateWindowBoundsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCandidateWindowBoundsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TextChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextBoxTextChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox4Impl: Sized {
    fn GetLinguisticAlternativesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox5Impl: Sized {
    fn SelectionHighlightColorWhenNotFocused(&self) -> ::windows::core::Result<super::Media::SolidColorBrush>;
    fn SetSelectionHighlightColorWhenNotFocused(&self, value: &::core::option::Option<super::Media::SolidColorBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox6Impl: Sized {
    fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment>;
    fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()>;
    fn CharacterCasing(&self) -> ::windows::core::Result<CharacterCasing>;
    fn SetCharacterCasing(&self, value: CharacterCasing) -> ::windows::core::Result<()>;
    fn PlaceholderForeground(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetPlaceholderForeground(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CopyingToClipboard(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextControlCopyingToClipboardEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCopyingToClipboard(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CuttingToClipboard(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextControlCuttingToClipboardEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCuttingToClipboard(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BeforeTextChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextBoxBeforeTextChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBeforeTextChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox7Impl: Sized {
    fn HandwritingView(&self) -> ::windows::core::Result<HandwritingView>;
    fn SetHandwritingView(&self, value: &::core::option::Option<HandwritingView>) -> ::windows::core::Result<()>;
    fn IsHandwritingViewEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHandwritingViewEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBox8Impl: Sized {
    fn CanPasteClipboardContent(&self) -> ::windows::core::Result<bool>;
    fn CanUndo(&self) -> ::windows::core::Result<bool>;
    fn CanRedo(&self) -> ::windows::core::Result<bool>;
    fn SelectionFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn SetSelectionFlyout(&self, value: &::core::option::Option<Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn ProofingMenuFlyout(&self) -> ::windows::core::Result<Primitives::FlyoutBase>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDescription(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectionChanging(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TextBox, TextBoxSelectionChangingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Undo(&self) -> ::windows::core::Result<()>;
    fn Redo(&self) -> ::windows::core::Result<()>;
    fn PasteFromClipboard(&self) -> ::windows::core::Result<()>;
    fn CopySelectionToClipboard(&self) -> ::windows::core::Result<()>;
    fn CutSelectionToClipboard(&self) -> ::windows::core::Result<()>;
    fn ClearUndoRedoHistory(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxBeforeTextChangingEventArgsImpl: Sized {
    fn NewText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxSelectionChangingEventArgsImpl: Sized {
    fn SelectionStart(&self) -> ::windows::core::Result<i32>;
    fn SelectionLength(&self) -> ::windows::core::Result<i32>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStaticsImpl: Sized {
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaxLengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AcceptsReturnProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextWrappingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSpellCheckEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsTextPredictionEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn InputScopeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics2Impl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionHighlightColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PreventKeyboardDisplayOnProgrammaticFocusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsColorFontEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics3Impl: Sized {
    fn DesiredCandidateWindowAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextReadingOrderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics5Impl: Sized {
    fn SelectionHighlightColorWhenNotFocusedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics6Impl: Sized {
    fn HorizontalTextAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CharacterCasingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlaceholderForegroundProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics7Impl: Sized {
    fn HandwritingViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsHandwritingViewEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxStatics8Impl: Sized {
    fn CanPasteClipboardContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanUndoProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanRedoProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SelectionFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProofingMenuFlyoutProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxTextChangingEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxTextChangingEventArgs2Impl: Sized {
    fn IsContentChanging(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextCommandBarFlyoutImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextCommandBarFlyoutFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextCommandBarFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextCompositionChangedEventArgsImpl: Sized {
    fn StartIndex(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextCompositionEndedEventArgsImpl: Sized {
    fn StartIndex(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextCompositionStartedEventArgsImpl: Sized {
    fn StartIndex(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextControlCopyingToClipboardEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextControlCuttingToClipboardEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextControlPasteEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickedEventArgsImpl: Sized {
    fn OldTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn NewTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerImpl: Sized {
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ClockIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClockIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MinuteIncrement(&self) -> ::windows::core::Result<i32>;
    fn SetMinuteIncrement(&self, value: i32) -> ::windows::core::Result<()>;
    fn Time(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TimeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<TimePickerValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePicker2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePicker3Impl: Sized {
    fn SelectedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetSelectedTime(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SelectedTimeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TimePicker, TimePickerSelectedValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedTimeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TimePicker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutImpl: Sized {
    fn ClockIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClockIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Time(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MinuteIncrement(&self) -> ::windows::core::Result<i32>;
    fn SetMinuteIncrement(&self, value: i32) -> ::windows::core::Result<()>;
    fn TimePicked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TimePickerFlyout, TimePickedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimePicked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAtAsync(&self, target: &::core::option::Option<super::FrameworkElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenter2Impl: Sized {
    fn IsDefaultShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDefaultShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenterStatics2Impl: Sized {
    fn IsDefaultShadowEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutStaticsImpl: Sized {
    fn ClockIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TimeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinuteIncrementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerSelectedValueChangedEventArgsImpl: Sized {
    fn OldTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn NewTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerStaticsImpl: Sized {
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ClockIdentifierProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinuteIncrementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TimeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerStatics3Impl: Sized {
    fn SelectedTimeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerValueChangedEventArgsImpl: Sized {
    fn OldTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn NewTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemImpl: Sized {
    fn IsChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleMenuFlyoutItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemStaticsImpl: Sized {
    fn IsCheckedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSplitButtonImpl: Sized {
    fn IsChecked(&self) -> ::windows::core::Result<bool>;
    fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckedChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ToggleSplitButton, ToggleSplitButtonIsCheckedChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsCheckedChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSplitButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSplitButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<ToggleSplitButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSplitButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSplitButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSplitButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSplitButtonIsCheckedChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchImpl: Sized {
    fn IsOn(&self) -> ::windows::core::Result<bool>;
    fn SetIsOn(&self, value: bool) -> ::windows::core::Result<()>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHeader(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn HeaderTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetHeaderTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OnContent(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetOnContent(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnContentTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetOnContentTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn OffContent(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetOffContent(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OffContentTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetOffContentTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ToggleSwitchTemplateSettings>;
    fn Toggled(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveToggled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchOverridesImpl: Sized {
    fn OnToggled(&self) -> ::windows::core::Result<()>;
    fn OnOnContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnOffContentChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn OnHeaderChanged(&self, oldcontent: &::core::option::Option<::windows::core::IInspectable>, newcontent: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchStaticsImpl: Sized {
    fn IsOnProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HeaderTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OnContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OnContentTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OffContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OffContentTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn Placement(&self) -> ::windows::core::Result<Primitives::PlacementMode>;
    fn SetPlacement(&self, value: Primitives::PlacementMode) -> ::windows::core::Result<()>;
    fn PlacementTarget(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPlacementTarget(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn TemplateSettings(&self) -> ::windows::core::Result<Primitives::ToolTipTemplateSettings>;
    fn Closed(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTip2Impl: Sized {
    fn PlacementRect(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Rect>>;
    fn SetPlacementRect(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToolTip>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipServiceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipServiceStaticsImpl: Sized {
    fn PlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetPlacement(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Primitives::PlacementMode>;
    fn SetPlacement(&self, element: &::core::option::Option<super::DependencyObject>, value: Primitives::PlacementMode) -> ::windows::core::Result<()>;
    fn PlacementTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetPlacementTarget(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::UIElement>;
    fn SetPlacementTarget(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ToolTipProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetToolTip(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetToolTip(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipStaticsImpl: Sized {
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlacementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PlacementTargetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipStatics2Impl: Sized {
    fn PlacementRectProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewImpl: Sized {
    fn RootNodes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TreeViewNode>>;
    fn SelectionMode(&self) -> ::windows::core::Result<TreeViewSelectionMode>;
    fn SetSelectionMode(&self, value: TreeViewSelectionMode) -> ::windows::core::Result<()>;
    fn SelectedNodes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TreeViewNode>>;
    fn Expand(&self, value: &::core::option::Option<TreeViewNode>) -> ::windows::core::Result<()>;
    fn Collapse(&self, value: &::core::option::Option<TreeViewNode>) -> ::windows::core::Result<()>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn ItemInvoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TreeView, TreeViewItemInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Expanding(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TreeView, TreeViewExpandingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveExpanding(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Collapsed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TreeView, TreeViewCollapsedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollapsed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeView2Impl: Sized {
    fn NodeFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<TreeViewNode>;
    fn ContainerFromNode(&self, node: &::core::option::Option<TreeViewNode>) -> ::windows::core::Result<super::DependencyObject>;
    fn ItemFromContainer(&self, container: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ContainerFromItem(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::DependencyObject>;
    fn CanDragItems(&self) -> ::windows::core::Result<bool>;
    fn SetCanDragItems(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReorderItems(&self) -> ::windows::core::Result<bool>;
    fn SetCanReorderItems(&self, value: bool) -> ::windows::core::Result<()>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::DataTemplate>) -> ::windows::core::Result<()>;
    fn ItemTemplateSelector(&self) -> ::windows::core::Result<DataTemplateSelector>;
    fn SetItemTemplateSelector(&self, value: &::core::option::Option<DataTemplateSelector>) -> ::windows::core::Result<()>;
    fn ItemContainerStyle(&self) -> ::windows::core::Result<super::Style>;
    fn SetItemContainerStyle(&self, value: &::core::option::Option<super::Style>) -> ::windows::core::Result<()>;
    fn ItemContainerStyleSelector(&self) -> ::windows::core::Result<StyleSelector>;
    fn SetItemContainerStyleSelector(&self, value: &::core::option::Option<StyleSelector>) -> ::windows::core::Result<()>;
    fn ItemContainerTransitions(&self) -> ::windows::core::Result<super::Media::Animation::TransitionCollection>;
    fn SetItemContainerTransitions(&self, value: &::core::option::Option<super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn DragItemsStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TreeView, TreeViewDragItemsStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragItemsStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragItemsCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TreeView, TreeViewDragItemsCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragItemsCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewCollapsedEventArgsImpl: Sized {
    fn Node(&self) -> ::windows::core::Result<TreeViewNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewCollapsedEventArgs2Impl: Sized {
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewDragItemsCompletedEventArgsImpl: Sized {
    fn DropResult(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewDragItemsStartingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewExpandingEventArgsImpl: Sized {
    fn Node(&self) -> ::windows::core::Result<TreeViewNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewExpandingEventArgs2Impl: Sized {
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemImpl: Sized {
    fn GlyphOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetGlyphOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlyphBrush(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetGlyphBrush(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn ExpandedGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExpandedGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CollapsedGlyph(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCollapsedGlyph(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GlyphSize(&self) -> ::windows::core::Result<f64>;
    fn SetGlyphSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsExpanded(&self) -> ::windows::core::Result<bool>;
    fn SetIsExpanded(&self, value: bool) -> ::windows::core::Result<()>;
    fn TreeViewItemTemplateSettings(&self) -> ::windows::core::Result<TreeViewItemTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItem2Impl: Sized {
    fn HasUnrealizedChildren(&self) -> ::windows::core::Result<bool>;
    fn SetHasUnrealizedChildren(&self, value: bool) -> ::windows::core::Result<()>;
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemInvokedEventArgsImpl: Sized {
    fn InvokedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemStaticsImpl: Sized {
    fn GlyphOpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlyphBrushProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ExpandedGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CollapsedGlyphProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlyphSizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsExpandedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TreeViewItemTemplateSettingsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemStatics2Impl: Sized {
    fn HasUnrealizedChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemTemplateSettingsImpl: Sized {
    fn ExpandedGlyphVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn CollapsedGlyphVisibility(&self) -> ::windows::core::Result<super::Visibility>;
    fn Indentation(&self) -> ::windows::core::Result<super::Thickness>;
    fn DragItemsCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemTemplateSettingsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItemTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemTemplateSettingsStaticsImpl: Sized {
    fn ExpandedGlyphVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CollapsedGlyphVisibilityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IndentationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DragItemsCountProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewList>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewNodeImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetContent(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<TreeViewNode>;
    fn IsExpanded(&self) -> ::windows::core::Result<bool>;
    fn SetIsExpanded(&self, value: bool) -> ::windows::core::Result<()>;
    fn HasChildren(&self) -> ::windows::core::Result<bool>;
    fn Depth(&self) -> ::windows::core::Result<i32>;
    fn HasUnrealizedChildren(&self) -> ::windows::core::Result<bool>;
    fn SetHasUnrealizedChildren(&self, value: bool) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TreeViewNode>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewNodeFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewNodeStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DepthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsExpandedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HasChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewStaticsImpl: Sized {
    fn SelectionModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewStatics2Impl: Sized {
    fn CanDragItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanReorderItemsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemTemplateSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerStyleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerStyleSelectorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemContainerTransitionsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITwoPaneViewImpl: Sized {
    fn Pane1(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPane1(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pane2(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetPane2(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pane1Length(&self) -> ::windows::core::Result<super::GridLength>;
    fn SetPane1Length(&self, value: &super::GridLength) -> ::windows::core::Result<()>;
    fn Pane2Length(&self) -> ::windows::core::Result<super::GridLength>;
    fn SetPane2Length(&self, value: &super::GridLength) -> ::windows::core::Result<()>;
    fn PanePriority(&self) -> ::windows::core::Result<TwoPaneViewPriority>;
    fn SetPanePriority(&self, value: TwoPaneViewPriority) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<TwoPaneViewMode>;
    fn WideModeConfiguration(&self) -> ::windows::core::Result<TwoPaneViewWideModeConfiguration>;
    fn SetWideModeConfiguration(&self, value: TwoPaneViewWideModeConfiguration) -> ::windows::core::Result<()>;
    fn TallModeConfiguration(&self) -> ::windows::core::Result<TwoPaneViewTallModeConfiguration>;
    fn SetTallModeConfiguration(&self, value: TwoPaneViewTallModeConfiguration) -> ::windows::core::Result<()>;
    fn MinWideModeWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWideModeWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinTallModeHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinTallModeHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn ModeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<TwoPaneView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveModeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITwoPaneViewFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TwoPaneView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITwoPaneViewStaticsImpl: Sized {
    fn Pane1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Pane2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Pane1LengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Pane2LengthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PanePriorityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn WideModeConfigurationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TallModeConfigurationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinWideModeWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MinTallModeHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementCollectionImpl: Sized {
    fn Move(&self, oldindex: u32, newindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserControlImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserControlFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<UserControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserControlStaticsImpl: Sized {
    fn ContentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariableSizedWrapGridImpl: Sized {
    fn ItemHeight(&self) -> ::windows::core::Result<f64>;
    fn SetItemHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn ItemWidth(&self) -> ::windows::core::Result<f64>;
    fn SetItemWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn HorizontalChildrenAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalChildrenAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalChildrenAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalChildrenAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn MaximumRowsOrColumns(&self) -> ::windows::core::Result<i32>;
    fn SetMaximumRowsOrColumns(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariableSizedWrapGridStaticsImpl: Sized {
    fn ItemHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalChildrenAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalChildrenAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaximumRowsOrColumnsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RowSpanProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetRowSpan(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<i32>;
    fn SetRowSpan(&self, element: &::core::option::Option<super::UIElement>, value: i32) -> ::windows::core::Result<()>;
    fn ColumnSpanProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetColumnSpan(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<i32>;
    fn SetColumnSpan(&self, element: &::core::option::Option<super::UIElement>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IViewboxImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn StretchDirection(&self) -> ::windows::core::Result<StretchDirection>;
    fn SetStretchDirection(&self, value: StretchDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IViewboxStaticsImpl: Sized {
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingPanelImpl: Sized {
    fn ItemContainerGenerator(&self) -> ::windows::core::Result<ItemContainerGenerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingPanelFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingPanelOverridesImpl: Sized {
    fn OnItemsChanged(&self, sender: &::core::option::Option<::windows::core::IInspectable>, args: &::core::option::Option<Primitives::ItemsChangedEventArgs>) -> ::windows::core::Result<()>;
    fn OnClearChildren(&self) -> ::windows::core::Result<()>;
    fn BringIndexIntoView(&self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingPanelProtectedImpl: Sized {
    fn AddInternalChild(&self, child: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn InsertInternalChild(&self, index: i32, child: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn RemoveInternalChildRange(&self, index: i32, range: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingStackPanelImpl: Sized {
    fn AreScrollSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn SetAreScrollSnapPointsRegular(&self, value: bool) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn CleanUpVirtualizedItemEvent(&self, handler: &::core::option::Option<CleanUpVirtualizedItemEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCleanUpVirtualizedItemEvent(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingStackPanelOverridesImpl: Sized {
    fn OnCleanUpVirtualizedItem(&self, e: &::core::option::Option<CleanUpVirtualizedItemEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualizingStackPanelStaticsImpl: Sized {
    fn AreScrollSnapPointsRegularProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VirtualizationModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetVirtualizationMode(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<VirtualizationMode>;
    fn SetVirtualizationMode(&self, element: &::core::option::Option<super::DependencyObject>, value: VirtualizationMode) -> ::windows::core::Result<()>;
    fn IsVirtualizingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsVirtualizing(&self, o: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetSource(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AllowedScriptNotifyUris(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Uri>>;
    fn SetAllowedScriptNotifyUris(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Uri>>) -> ::windows::core::Result<()>;
    fn DataTransferPackage(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn LoadCompleted(&self, handler: &::core::option::Option<super::Navigation::LoadCompletedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScriptNotify(&self, handler: &::core::option::Option<NotifyEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationFailed(&self, handler: &::core::option::Option<WebViewNavigationFailedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InvokeScript(&self, scriptname: &::windows::core::HSTRING, arguments: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Navigate(&self, source: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView2Impl: Sized {
    fn CanGoBack(&self) -> ::windows::core::Result<bool>;
    fn CanGoForward(&self) -> ::windows::core::Result<bool>;
    fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NavigationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContentLoading(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DOMContentLoaded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GoForward(&self) -> ::windows::core::Result<()>;
    fn GoBack(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn CapturePreviewToStreamAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn InvokeScriptAsync(&self, scriptname: &::windows::core::HSTRING, arguments: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn NavigateToLocalStreamUri(&self, source: &::core::option::Option<super::super::super::Foundation::Uri>, streamresolver: &::core::option::Option<super::super::super::Web::IUriToStreamResolver>) -> ::windows::core::Result<()>;
    fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameContentLoading(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewLongRunningScriptDetectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewUnviewableContentIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestmessage: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<()>;
    fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView3Impl: Sized {
    fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool>;
    fn ContainsFullScreenElementChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView4Impl: Sized {
    fn ExecutionMode(&self) -> ::windows::core::Result<WebViewExecutionMode>;
    fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WebViewDeferredPermissionRequest>>;
    fn Settings(&self) -> ::windows::core::Result<WebViewSettings>;
    fn UnsupportedUriSchemeIdentified(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NewWindowRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewNewWindowRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PermissionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewPermissionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddWebAllowedObject(&self, name: &::windows::core::HSTRING, pobject: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn DeferredPermissionRequestById(&self, id: u32) -> ::windows::core::Result<WebViewDeferredPermissionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView5Impl: Sized {
    fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusLeft(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusRight(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusUp(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetXYFocusDown(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView6Impl: Sized {
    fn SeparateProcessLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewSeparateProcessLostEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeparateProcessLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebView7Impl: Sized {
    fn WebResourceRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebView, WebViewWebResourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewBrushImpl: Sized {
    fn SourceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSourceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Redraw(&self) -> ::windows::core::Result<()>;
    fn SetSource(&self, source: &::core::option::Option<WebView>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewBrushStaticsImpl: Sized {
    fn SourceNameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewContentLoadingEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewDOMContentLoadedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewDeferredPermissionRequestImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn PermissionType(&self) -> ::windows::core::Result<WebViewPermissionType>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Allow(&self) -> ::windows::core::Result<()>;
    fn Deny(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewFactory4Impl: Sized {
    fn CreateInstanceWithExecutionMode(&self, executionmode: WebViewExecutionMode) -> ::windows::core::Result<WebView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewLongRunningScriptDetectedEventArgsImpl: Sized {
    fn ExecutionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn StopPageScriptExecution(&self) -> ::windows::core::Result<bool>;
    fn SetStopPageScriptExecution(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewNavigationCompletedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn IsSuccess(&self) -> ::windows::core::Result<bool>;
    fn WebErrorStatus(&self) -> ::windows::core::Result<super::super::super::Web::WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewNavigationFailedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn WebErrorStatus(&self) -> ::windows::core::Result<super::super::super::Web::WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewNavigationStartingEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewNewWindowRequestedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Referrer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewPermissionRequestImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn PermissionType(&self) -> ::windows::core::Result<WebViewPermissionType>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn State(&self) -> ::windows::core::Result<WebViewPermissionState>;
    fn Defer(&self) -> ::windows::core::Result<()>;
    fn Allow(&self) -> ::windows::core::Result<()>;
    fn Deny(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewPermissionRequestedEventArgsImpl: Sized {
    fn PermissionRequest(&self) -> ::windows::core::Result<WebViewPermissionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewSeparateProcessLostEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewSettingsImpl: Sized {
    fn IsJavaScriptEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsJavaScriptEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsIndexedDBEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsIndexedDBEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewStaticsImpl: Sized {
    fn AnyScriptNotifyUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Foundation::Uri>>;
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AllowedScriptNotifyUrisProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DataTransferPackageProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewStatics2Impl: Sized {
    fn CanGoBackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CanGoForwardProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DocumentTitleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DefaultBackgroundColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewStatics3Impl: Sized {
    fn ContainsFullScreenElementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewStatics4Impl: Sized {
    fn DefaultExecutionMode(&self) -> ::windows::core::Result<WebViewExecutionMode>;
    fn ClearTemporaryWebDataAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewStatics5Impl: Sized {
    fn XYFocusLeftProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusRightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusUpProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn XYFocusDownProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewUnsupportedUriSchemeIdentifiedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewUnviewableContentIdentifiedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Referrer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewUnviewableContentIdentifiedEventArgs2Impl: Sized {
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewWebResourceRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpRequestMessage>;
    fn Response(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
    fn SetResponse(&self, value: &::core::option::Option<super::super::super::Web::Http::HttpResponseMessage>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWrapGridImpl: Sized {
    fn ItemWidth(&self) -> ::windows::core::Result<f64>;
    fn SetItemWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn ItemHeight(&self) -> ::windows::core::Result<f64>;
    fn SetItemHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<Orientation>;
    fn SetOrientation(&self, value: Orientation) -> ::windows::core::Result<()>;
    fn HorizontalChildrenAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment>;
    fn SetHorizontalChildrenAlignment(&self, value: super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalChildrenAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment>;
    fn SetVerticalChildrenAlignment(&self, value: super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn MaximumRowsOrColumns(&self) -> ::windows::core::Result<i32>;
    fn SetMaximumRowsOrColumns(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWrapGridStaticsImpl: Sized {
    fn ItemWidthProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemHeightProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn HorizontalChildrenAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn VerticalChildrenAlignmentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MaximumRowsOrColumnsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
