#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppExitedEventArgsImpl: Sized {
    fn ExitCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppManagerImpl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DesignerAppExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDesignerAppExited(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateNewViewAsync(&self, initialviewstate: DesignerAppViewState, initialviewsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DesignerAppView>>;
    fn LoadObjectIntoAppAsync(&self, dllname: &::windows::core::HSTRING, classid: &::windows::core::GUID, initializationdata: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppManagerFactoryImpl: Sized {
    fn Create(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<DesignerAppManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppViewImpl: Sized {
    fn ApplicationViewId(&self) -> ::windows::core::Result<i32>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ViewState(&self) -> ::windows::core::Result<DesignerAppViewState>;
    fn ViewSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn UpdateViewAsync(&self, viewstate: DesignerAppViewState, viewsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn HasFocus(&self) -> ::windows::core::Result<bool>;
    fn TakeFocusRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTakeFocusRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigateFocus(&self, request: &::core::option::Option<XamlSourceFocusNavigationRequest>) -> ::windows::core::Result<XamlSourceFocusNavigationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DesktopWindowXamlSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceGotFocusEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceTakeFocusRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementCompositionPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IElementCompositionPreviewStaticsImpl: Sized {
    fn GetElementVisual(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::Visual>;
    fn GetElementChildVisual(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::Visual>;
    fn SetElementChildVisual(&self, element: &::core::option::Option<super::UIElement>, visual: &::core::option::Option<super::super::Composition::Visual>) -> ::windows::core::Result<()>;
    fn GetScrollViewerManipulationPropertySet(&self, scrollviewer: &::core::option::Option<super::Controls::ScrollViewer>) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementCompositionPreviewStatics2Impl: Sized {
    fn SetImplicitShowAnimation(&self, element: &::core::option::Option<super::UIElement>, animation: &::core::option::Option<super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn SetImplicitHideAnimation(&self, element: &::core::option::Option<super::UIElement>, animation: &::core::option::Option<super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn SetIsTranslationEnabled(&self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn GetPointerPositionPropertySet(&self, targetelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementCompositionPreviewStatics3Impl: Sized {
    fn SetAppWindowContent(&self, appwindow: &::core::option::Option<super::super::WindowManagement::AppWindow>, xamlcontent: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn GetAppWindowContent(&self, appwindow: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsXamlManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsXamlManagerStaticsImpl: Sized {
    fn InitializeForCurrentThread(&self) -> ::windows::core::Result<WindowsXamlManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationRequestImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<XamlSourceFocusNavigationReason>;
    fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationRequestFactoryImpl: Sized {
    fn CreateInstance(&self, reason: XamlSourceFocusNavigationReason) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
    fn CreateInstanceWithHintRect(&self, reason: XamlSourceFocusNavigationReason, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
    fn CreateInstanceWithHintRectAndCorrelationId(&self, reason: XamlSourceFocusNavigationReason, hintrect: &super::super::super::Foundation::Rect, correlationid: &::windows::core::GUID) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationResultImpl: Sized {
    fn WasFocusMoved(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationResultFactoryImpl: Sized {
    fn CreateInstance(&self, focusmoved: bool) -> ::windows::core::Result<XamlSourceFocusNavigationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUIPresenterImpl: Sized {
    fn RootElement(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetRootElement(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ThemeKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetThemeKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ThemeResourcesXaml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetThemeResourcesXaml(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetSize(&self, width: i32, height: i32) -> ::windows::core::Result<()>;
    fn Render(&self) -> ::windows::core::Result<()>;
    fn Present(&self) -> ::windows::core::Result<()>;
}
pub trait IXamlUIPresenterHostImpl: Sized {
    fn ResolveFileResource(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IXamlUIPresenterHost2Impl: Sized {
    fn GetGenericXamlFilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IXamlUIPresenterHost3Impl: Sized {
    fn ResolveDictionaryResource(&self, dictionary: &::core::option::Option<super::ResourceDictionary>, dictionarykey: &::core::option::Option<::windows::core::IInspectable>, suggestedvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUIPresenterStaticsImpl: Sized {
    fn CompleteTimelinesAutomatically(&self) -> ::windows::core::Result<bool>;
    fn SetCompleteTimelinesAutomatically(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetHost(&self, host: &::core::option::Option<IXamlUIPresenterHost>) -> ::windows::core::Result<()>;
    fn NotifyWindowSizeChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUIPresenterStatics2Impl: Sized {
    fn GetFlyoutPlacementTargetInfo(&self, placementtarget: &::core::option::Option<super::FrameworkElement>, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetFlyoutPlacement(&self, placementtargetbounds: &super::super::super::Foundation::Rect, controlsize: &super::super::super::Foundation::Size, mincontrolsize: &super::super::super::Foundation::Size, containerrect: &super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
