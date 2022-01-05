#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBarButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBarToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarToggleButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AutoSuggestBox>) -> ::windows::core::Result<AutoSuggestBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerImpl: Sized {
    fn EventsSource(&self) -> ::windows::core::Result<AutomationPeer>;
    fn SetEventsSource(&self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn GetPattern(&self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn RaiseAutomationEvent(&self, eventid: AutomationEvents) -> ::windows::core::Result<()>;
    fn RaisePropertyChangedEvent(&self, automationproperty: &::core::option::Option<super::AutomationProperty>, oldvalue: &::core::option::Option<::windows::core::IInspectable>, newvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlType(&self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangle(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildren(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledBy(&self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientation(&self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocus(&self) -> ::windows::core::Result<bool>;
    fn IsContentElement(&self) -> ::windows::core::Result<bool>;
    fn IsControlElement(&self) -> ::windows::core::Result<bool>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusable(&self) -> ::windows::core::Result<bool>;
    fn IsOffscreen(&self) -> ::windows::core::Result<bool>;
    fn IsPassword(&self) -> ::windows::core::Result<bool>;
    fn IsRequiredForForm(&self) -> ::windows::core::Result<bool>;
    fn SetFocus(&self) -> ::windows::core::Result<()>;
    fn GetParent(&self) -> ::windows::core::Result<AutomationPeer>;
    fn InvalidatePeer(&self) -> ::windows::core::Result<()>;
    fn GetPeerFromPoint(&self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSetting(&self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer2Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer3Impl: Sized {
    fn Navigate(&self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPoint(&self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElement(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
    fn GetControlledPeers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
    fn GetAnnotations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn SetParent(&self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn RaiseTextEditTextChangedEvent(&self, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetPositionInSet(&self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSet(&self) -> ::windows::core::Result<i32>;
    fn GetLevel(&self) -> ::windows::core::Result<i32>;
    fn RaiseStructureChangedEvent(&self, structurechangetype: AutomationStructureChangeType, child: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer4Impl: Sized {
    fn GetLandmarkType(&self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer5Impl: Sized {
    fn IsPeripheral(&self) -> ::windows::core::Result<bool>;
    fn IsDataValidForForm(&self) -> ::windows::core::Result<bool>;
    fn GetFullDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer6Impl: Sized {
    fn GetCulture(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer7Impl: Sized {
    fn RaiseNotificationEvent(&self, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: &::windows::core::HSTRING, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer8Impl: Sized {
    fn GetHeadingLevel(&self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer9Impl: Sized {
    fn IsDialog(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::AnnotationType>;
    fn SetType(&self, value: super::AnnotationType) -> ::windows::core::Result<()>;
    fn Peer(&self) -> ::windows::core::Result<AutomationPeer>;
    fn SetPeer(&self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationFactoryImpl: Sized {
    fn CreateInstance(&self, r#type: super::AnnotationType) -> ::windows::core::Result<AutomationPeerAnnotation>;
    fn CreateWithPeerParameter(&self, r#type: super::AnnotationType, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<AutomationPeerAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationStaticsImpl: Sized {
    fn TypeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PeerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverridesImpl: Sized {
    fn GetPatternCore(&self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAcceleratorKeyCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKeyCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlTypeCore(&self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationIdCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangleCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildrenCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassNameCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePointCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpTextCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatusCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledByCore(&self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNameCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientationCore(&self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocusCore(&self) -> ::windows::core::Result<bool>;
    fn IsContentElementCore(&self) -> ::windows::core::Result<bool>;
    fn IsControlElementCore(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledCore(&self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusableCore(&self) -> ::windows::core::Result<bool>;
    fn IsOffscreenCore(&self) -> ::windows::core::Result<bool>;
    fn IsPasswordCore(&self) -> ::windows::core::Result<bool>;
    fn IsRequiredForFormCore(&self) -> ::windows::core::Result<bool>;
    fn SetFocusCore(&self) -> ::windows::core::Result<()>;
    fn GetPeerFromPointCore(&self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSettingCore(&self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides2Impl: Sized {
    fn ShowContextMenuCore(&self) -> ::windows::core::Result<()>;
    fn GetControlledPeersCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides3Impl: Sized {
    fn NavigateCore(&self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPointCore(&self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElementCore(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnnotationsCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn GetPositionInSetCore(&self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSetCore(&self) -> ::windows::core::Result<i32>;
    fn GetLevelCore(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides4Impl: Sized {
    fn GetLandmarkTypeCore(&self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides5Impl: Sized {
    fn IsPeripheralCore(&self) -> ::windows::core::Result<bool>;
    fn IsDataValidForFormCore(&self) -> ::windows::core::Result<bool>;
    fn GetFullDescriptionCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDescribedByCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsToCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsFromCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides6Impl: Sized {
    fn GetCultureCore(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides8Impl: Sized {
    fn GetHeadingLevelCore(&self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides9Impl: Sized {
    fn IsDialogCore(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerProtectedImpl: Sized {
    fn PeerFromProvider(&self, provider: &::core::option::Option<super::Provider::IRawElementProviderSimple>) -> ::windows::core::Result<AutomationPeer>;
    fn ProviderFromPeer(&self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<super::Provider::IRawElementProviderSimple>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStaticsImpl: Sized {
    fn ListenerExists(&self, eventid: AutomationEvents) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStatics3Impl: Sized {
    fn GenerateRawElementProviderRuntimeId(&self) -> ::windows::core::Result<RawElementProviderRuntimeId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Button>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ButtonBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CalendarDatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarDatePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CaptureElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CaptureElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CheckBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CheckBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorPickerSlider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSliderAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorSpectrum>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrumAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ComboBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ComboBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ComboBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::DatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DatePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlipView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlipViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<FlipViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutPresenterAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerImpl: Sized {
    fn Owner(&self) -> ::windows::core::Result<super::super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::FrameworkElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerStaticsImpl: Sized {
    fn FromElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
    fn CreatePeerForElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<GridViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GroupItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHubAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Hub>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::HubSection>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubSectionAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::HyperlinkButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HyperlinkButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IImageAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Image>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ImageAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ItemsControlAutomationPeer(&self) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ItemsControlAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeer2Impl: Sized {
    fn CreateItemAutomationPeer(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ItemsControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerOverrides2Impl: Sized {
    fn OnCreateItemAutomationPeer(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewBaseHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListViewBaseAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaPlayerElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaTransportControls>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaTransportControlsAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<super::super::Controls::MenuBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<super::super::Controls::MenuBarItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutPresenterAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::NavigationViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PasswordBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PasswordBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PersonPicture>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PersonPictureAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Pivot>) -> ::windows::core::Result<PivotAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PivotItem>) -> ::windows::core::Result<PivotItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<PivotAutomationPeer>) -> ::windows::core::Result<PivotItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ProgressBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ProgressRing>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressRingAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RadioButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RadioButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::RangeBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RatingControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::RepeatButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RepeatButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichEditBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichEditBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichTextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichTextBlockOverflow>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockOverflowAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ScrollBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ScrollViewer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollViewerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SearchBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SearchBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::Selector>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<SelectorAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SemanticZoom>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SemanticZoomAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SettingsFlyout>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SettingsFlyoutAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Slider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SliderAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBlockAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TextBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::Thumb>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThumbAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TimePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TimePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ToggleMenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleMenuFlyoutItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ToggleSwitch>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSwitchAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TreeViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TreeViewList>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewListAutomationPeer>;
}
