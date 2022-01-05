#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarTemplateSettingsImpl: Sized {
    fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CompactVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn CompactRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn MinimalVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn MinimalRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn HiddenVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn HiddenRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarTemplateSettings2Impl: Sized {
    fn NegativeCompactVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn NegativeMinimalVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn NegativeHiddenVerticalDelta(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseImpl: Sized {
    fn ClickMode(&self) -> ::windows::core::Result<super::ClickMode>;
    fn SetClickMode(&self, value: super::ClickMode) -> ::windows::core::Result<()>;
    fn IsPointerOver(&self) -> ::windows::core::Result<bool>;
    fn IsPressed(&self) -> ::windows::core::Result<bool>;
    fn Command(&self) -> ::windows::core::Result<super::super::Input::ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<super::super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseStaticsImpl: Sized {
    fn ClickModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPointerOverProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPressedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandParameterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewTemplateSettingsImpl: Sized {
    fn MinViewWidth(&self) -> ::windows::core::Result<f64>;
    fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay6(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasMoreContentAfter(&self) -> ::windows::core::Result<bool>;
    fn HasMoreContentBefore(&self) -> ::windows::core::Result<bool>;
    fn HasMoreViews(&self) -> ::windows::core::Result<bool>;
    fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICarouselPanelImpl: Sized {
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
    fn MakeVisible(&self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICarouselPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CarouselPanel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderImpl: Sized {
    fn ColorChannel(&self) -> ::windows::core::Result<super::ColorPickerHsvChannel>;
    fn SetColorChannel(&self, value: super::ColorPickerHsvChannel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSlider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderStaticsImpl: Sized {
    fn ColorChannelProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn HsvColor(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector4>;
    fn SetHsvColor(&self, value: &super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
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
    fn Shape(&self) -> ::windows::core::Result<super::ColorSpectrumShape>;
    fn SetShape(&self, value: super::ColorSpectrumShape) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<super::ColorSpectrumComponents>;
    fn SetComponents(&self, value: super::ColorSpectrumComponents) -> ::windows::core::Result<()>;
    fn ColorChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrum>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HsvColorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinHueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxHueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinSaturationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxSaturationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShapeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ComponentsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxTemplateSettingsImpl: Sized {
    fn DropDownOpenedHeight(&self) -> ::windows::core::Result<f64>;
    fn DropDownClosedHeight(&self) -> ::windows::core::Result<f64>;
    fn DropDownOffset(&self) -> ::windows::core::Result<f64>;
    fn SelectedItemDirection(&self) -> ::windows::core::Result<AnimationDirection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxTemplateSettings2Impl: Sized {
    fn DropDownContentMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarImpl: Sized {
    fn FlyoutTemplateSettings(&self) -> ::windows::core::Result<CommandBarFlyoutCommandBarTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBarFlyoutCommandBar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarTemplateSettingsImpl: Sized {
    fn OpenAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn OpenAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn CloseAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn CurrentWidth(&self) -> ::windows::core::Result<f64>;
    fn ExpandedWidth(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionDelta(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpOverflowVerticalPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownOverflowVerticalPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationHoldPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationHoldPosition(&self) -> ::windows::core::Result<f64>;
    fn ContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettingsImpl: Sized {
    fn ContentHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentMinWidth(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentMaxHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHeight(&self) -> ::windows::core::Result<f64>;
    fn NegativeOverflowContentHeight(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings2Impl: Sized {
    fn OverflowContentMaxWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings3Impl: Sized {
    fn EffectiveOverflowButtonVisibility(&self) -> ::windows::core::Result<super::super::Visibility>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings4Impl: Sized {
    fn OverflowContentCompactYTranslation(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentMinimalYTranslation(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHiddenYTranslation(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragCompletedEventArgsImpl: Sized {
    fn HorizontalChange(&self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&self) -> ::windows::core::Result<f64>;
    fn Canceled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragCompletedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(&self, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragCompletedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragDeltaEventArgsImpl: Sized {
    fn HorizontalChange(&self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragDeltaEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalChangeAndVerticalChange(&self, horizontalchange: f64, verticalchange: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragDeltaEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartedEventArgsImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalOffsetAndVerticalOffset(&self, horizontaloffset: f64, verticaloffset: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragStartedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseImpl: Sized {
    fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opening(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpening(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAt(&self, placementtarget: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase2Impl: Sized {
    fn Target(&self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::super::ElementSoundMode) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase3Impl: Sized {
    fn OverlayInputPassThroughElement(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOverlayInputPassThroughElement(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase4Impl: Sized {
    fn TryInvokeKeyboardAccelerator(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase5Impl: Sized {
    fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn InputDevicePrefersPrimaryCommands(&self) -> ::windows::core::Result<bool>;
    fn AreOpenCloseAnimationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreOpenCloseAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn ShowAt(&self, placementtarget: &::core::option::Option<super::super::DependencyObject>, showoptions: &::core::option::Option<FlyoutShowOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase6Impl: Sized {
    fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn XamlRoot(&self) -> ::windows::core::Result<super::super::XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<super::super::XamlRoot>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseClosingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseOverridesImpl: Sized {
    fn CreatePresenter(&self) -> ::windows::core::Result<super::Control>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseOverrides4Impl: Sized {
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStaticsImpl: Sized {
    fn PlacementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AttachedFlyoutProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetAttachedFlyout(&self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<FlyoutBase>;
    fn SetAttachedFlyout(&self, element: &::core::option::Option<super::super::FrameworkElement>, value: &::core::option::Option<FlyoutBase>) -> ::windows::core::Result<()>;
    fn ShowAttachedFlyout(&self, flyoutowner: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics2Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics3Impl: Sized {
    fn OverlayInputPassThroughElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics5Impl: Sized {
    fn TargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShowModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn InputDevicePrefersPrimaryCommandsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AreOpenCloseAnimationsEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics6Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutShowOptionsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetPosition(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>;
    fn SetExclusionRect(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutShowOptionsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutShowOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneratorPositionHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneratorPositionHelperStaticsImpl: Sized {
    fn FromIndexAndOffset(&self, index: i32, offset: i32) -> ::windows::core::Result<GeneratorPosition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterImpl: Sized {
    fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetGridViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetGridViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetGridViewItemPresenterPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterStaticsImpl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterHorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterVerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemTemplateSettingsImpl: Sized {
    fn DragItemsCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsChangedEventArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<i32>;
    fn Position(&self) -> ::windows::core::Result<GeneratorPosition>;
    fn OldPosition(&self) -> ::windows::core::Result<GeneratorPosition>;
    fn ItemCount(&self) -> ::windows::core::Result<i32>;
    fn ItemUICount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemBackgroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemBackgroundConverterStaticsImpl: Sized {
    fn EnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemForegroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemForegroundConverterStaticsImpl: Sized {
    fn EnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationStaticsImpl: Sized {
    fn GetLayoutExceptionElement(&self, dispatcher: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::UIElement>;
    fn GetLayoutSlot(&self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationStatics2Impl: Sized {
    fn GetAvailableSize(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterImpl: Sized {
    fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetListViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetListViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetListViewItemPresenterPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter2Impl: Sized {
    fn SelectedPressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPressedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusSecondaryBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusSecondaryBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckMode(&self) -> ::windows::core::Result<ListViewItemPresenterCheckMode>;
    fn SetCheckMode(&self, value: ListViewItemPresenterCheckMode) -> ::windows::core::Result<()>;
    fn PointerOverForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter3Impl: Sized {
    fn RevealBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetRevealBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn RevealBackgroundShowsAboveContent(&self) -> ::windows::core::Result<bool>;
    fn SetRevealBackgroundShowsAboveContent(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter4Impl: Sized {
    fn SelectedDisabledBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetCheckBoxCornerRadius(&self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetSelectionIndicatorCornerRadius(&self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionIndicatorVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SelectionIndicatorMode(&self) -> ::windows::core::Result<ListViewItemPresenterSelectionIndicatorMode>;
    fn SetSelectionIndicatorMode(&self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::Result<()>;
    fn SelectionIndicatorBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedInnerBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedInnerBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStaticsImpl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterHorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterVerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics2Impl: Sized {
    fn SelectedPressedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PressedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusSecondaryBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics3Impl: Sized {
    fn RevealBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBackgroundShowsAboveContentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics4Impl: Sized {
    fn SelectedDisabledBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxCornerRadiusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorCornerRadiusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPressedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedDisabledBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedInnerBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemTemplateSettingsImpl: Sized {
    fn DragItemsCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorImpl: Sized {
    fn ShouldLoop(&self) -> ::windows::core::Result<bool>;
    fn SetShouldLoop(&self, value: bool) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn SetItems(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ItemWidth(&self) -> ::windows::core::Result<i32>;
    fn SetItemWidth(&self, value: i32) -> ::windows::core::Result<()>;
    fn ItemHeight(&self) -> ::windows::core::Result<i32>;
    fn SetItemHeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorStaticsImpl: Sized {
    fn ShouldLoopProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterTemplateSettingsImpl: Sized {
    fn FlyoutContentMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterImpl: Sized {
    fn Icon(&self) -> ::windows::core::Result<super::IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<super::IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterStaticsImpl: Sized {
    fn IconProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientedVirtualizingPanelImpl: Sized {
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
    fn MakeVisible(&self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientedVirtualizingPanelFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PickerFlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseOverridesImpl: Sized {
    fn OnConfirmed(&self) -> ::windows::core::Result<()>;
    fn ShouldShowConfirmationButtons(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTitle(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PivotHeaderItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ChildTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
    fn SetChildTransitions(&self, value: &::core::option::Option<super::super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn IsLightDismissEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLightDismissEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup3Impl: Sized {
    fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup4Impl: Sized {
    fn PlacementTarget(&self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn SetPlacementTarget(&self, value: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn DesiredPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn SetDesiredPlacement(&self, value: PopupPlacementMode) -> ::windows::core::Result<()>;
    fn ActualPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn ActualPlacementChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualPlacementChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStaticsImpl: Sized {
    fn ChildProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildTransitionsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsLightDismissEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics3Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics4Impl: Sized {
    fn PlacementTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPlacementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarTemplateSettingsImpl: Sized {
    fn EllipseDiameter(&self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationWellPosition(&self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn IndicatorLengthDelta(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingTemplateSettingsImpl: Sized {
    fn EllipseDiameter(&self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn MaxSideLength(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseImpl: Sized {
    fn Minimum(&self) -> ::windows::core::Result<f64>;
    fn SetMinimum(&self, value: f64) -> ::windows::core::Result<()>;
    fn Maximum(&self) -> ::windows::core::Result<f64>;
    fn SetMaximum(&self, value: f64) -> ::windows::core::Result<()>;
    fn SmallChange(&self) -> ::windows::core::Result<f64>;
    fn SetSmallChange(&self, value: f64) -> ::windows::core::Result<()>;
    fn LargeChange(&self) -> ::windows::core::Result<f64>;
    fn SetLargeChange(&self, value: f64) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn ValueChanged(&self, handler: &::core::option::Option<RangeBaseValueChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseOverridesImpl: Sized {
    fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> ::windows::core::Result<()>;
    fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> ::windows::core::Result<()>;
    fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseStaticsImpl: Sized {
    fn MinimumProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaximumProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SmallChangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LargeChangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseValueChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<f64>;
    fn NewValue(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonImpl: Sized {
    fn Delay(&self) -> ::windows::core::Result<i32>;
    fn SetDelay(&self, value: i32) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<i32>;
    fn SetInterval(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonStaticsImpl: Sized {
    fn DelayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IntervalProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::Orientation>;
    fn SetOrientation(&self, value: super::Orientation) -> ::windows::core::Result<()>;
    fn ViewportSize(&self) -> ::windows::core::Result<f64>;
    fn SetViewportSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn IndicatorMode(&self) -> ::windows::core::Result<ScrollingIndicatorMode>;
    fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> ::windows::core::Result<()>;
    fn Scroll(&self, handler: &::core::option::Option<ScrollEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveScroll(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarStaticsImpl: Sized {
    fn OrientationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewportSizeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IndicatorModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollEventArgsImpl: Sized {
    fn NewValue(&self) -> ::windows::core::Result<f64>;
    fn ScrollEventType(&self) -> ::windows::core::Result<ScrollEventType>;
}
pub trait IScrollSnapPointsInfoImpl: Sized {
    fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn HorizontalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorImpl: Sized {
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValuePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSelectedValuePath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSynchronizedWithCurrentItem(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsSynchronizedWithCurrentItem(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemStaticsImpl: Sized {
    fn IsSelectedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorStaticsImpl: Sized {
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValuePathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsSynchronizedWithCurrentItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsSelectionActive(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutTemplateSettingsImpl: Sized {
    fn HeaderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn HeaderForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn IconSource(&self) -> ::windows::core::Result<super::super::Media::ImageSource>;
    fn ContentTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewTemplateSettingsImpl: Sized {
    fn OpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn OpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64>;
    fn OpenPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength>;
    fn CompactPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbImpl: Sized {
    fn IsDragging(&self) -> ::windows::core::Result<bool>;
    fn DragStarted(&self, handler: &::core::option::Option<DragStartedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragDelta(&self, handler: &::core::option::Option<DragDeltaEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragDelta(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragCompleted(&self, handler: &::core::option::Option<DragCompletedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CancelDrag(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbStaticsImpl: Sized {
    fn IsDraggingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITickBarImpl: Sized {
    fn Fill(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITickBarStaticsImpl: Sized {
    fn FillProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonImpl: Sized {
    fn IsChecked(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsChecked(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsThreeState(&self) -> ::windows::core::Result<bool>;
    fn SetIsThreeState(&self, value: bool) -> ::windows::core::Result<()>;
    fn Checked(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChecked(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unchecked(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnchecked(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Indeterminate(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIndeterminate(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonOverridesImpl: Sized {
    fn OnToggle(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonStaticsImpl: Sized {
    fn IsCheckedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsThreeStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchTemplateSettingsImpl: Sized {
    fn KnobCurrentToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobCurrentToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobOnToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobOffToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainOnToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainOffToOnOffset(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipTemplateSettingsImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
}
