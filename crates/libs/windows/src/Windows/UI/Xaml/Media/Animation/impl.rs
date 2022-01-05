#[cfg(feature = "implement_exclusive")]
pub trait IAddDeleteThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEaseImpl: Sized {
    fn Amplitude(&self) -> ::windows::core::Result<f64>;
    fn SetAmplitude(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEaseStaticsImpl: Sized {
    fn AmplitudeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BasicConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboardImpl: Sized {
    fn Storyboard(&self) -> ::windows::core::Result<Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboardStaticsImpl: Sized {
    fn StoryboardProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEaseImpl: Sized {
    fn Bounces(&self) -> ::windows::core::Result<i32>;
    fn SetBounces(&self, value: i32) -> ::windows::core::Result<()>;
    fn Bounciness(&self) -> ::windows::core::Result<f64>;
    fn SetBounciness(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEaseStaticsImpl: Sized {
    fn BouncesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BouncinessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICircleEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ColorKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetValue(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfoImpl: Sized {
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfoStaticsImpl: Sized {
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggerElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsStaggerElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsStaggerElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationImpl: Sized {
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryStart(&self, destination: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimation2Impl: Sized {
    fn IsScaleAnimationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryStartWithCoordinatedElements(&self, destination: &::core::option::Option<super::super::UIElement>, coordinatedelements: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>) -> ::windows::core::Result<bool>;
    fn SetAnimationComponent(&self, component: ConnectedAnimationComponent, animation: &::core::option::Option<super::super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimation3Impl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<ConnectedAnimationConfiguration>;
    fn SetConfiguration(&self, value: &::core::option::Option<ConnectedAnimationConfiguration>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfigurationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationServiceImpl: Sized {
    fn DefaultDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetDefaultDuration(&self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DefaultEasingFunction(&self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction>;
    fn SetDefaultEasingFunction(&self, value: &::core::option::Option<super::super::super::Composition::CompositionEasingFunction>) -> ::windows::core::Result<()>;
    fn PrepareToAnimate(&self, key: &::windows::core::HSTRING, source: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<ConnectedAnimation>;
    fn GetAnimation(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<ConnectedAnimation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationServiceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ConnectedAnimationService>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransitionImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransitionStaticsImpl: Sized {
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContinuumNavigationTransitionInfoImpl: Sized {
    fn ExitElement(&self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetExitElement(&self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContinuumNavigationTransitionInfoStaticsImpl: Sized {
    fn ExitElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsEntranceElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsEntranceElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsEntranceElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn IsExitElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsExitElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsExitElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn ExitElementContainerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetExitElementContainer(&self, element: &::core::option::Option<super::super::Controls::ListViewBase>) -> ::windows::core::Result<bool>;
    fn SetExitElementContainer(&self, element: &::core::option::Option<super::super::Controls::ListViewBase>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICubicEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DirectConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteColorKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteDoubleKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteObjectKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscretePointKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<DoubleKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DoubleKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOverThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOverThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimationImpl: Sized {
    fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimationStaticsImpl: Sized {
    fn EntranceTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimationImpl: Sized {
    fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimationStaticsImpl: Sized {
    fn EntranceTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseImpl: Sized {
    fn EasingMode(&self) -> ::windows::core::Result<EasingMode>;
    fn SetEasingMode(&self, value: EasingMode) -> ::windows::core::Result<()>;
    fn Ease(&self, normalizedtime: f64) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseStaticsImpl: Sized {
    fn EasingModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeUIThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeUIThemeTransitionStaticsImpl: Sized {
    fn EdgeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEaseImpl: Sized {
    fn Oscillations(&self) -> ::windows::core::Result<i32>;
    fn SetOscillations(&self, value: i32) -> ::windows::core::Result<()>;
    fn Springiness(&self) -> ::windows::core::Result<f64>;
    fn SetSpringiness(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEaseStaticsImpl: Sized {
    fn OscillationsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpringinessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfoStaticsImpl: Sized {
    fn IsTargetElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsTargetElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsTargetElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransitionImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransitionStaticsImpl: Sized {
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEaseImpl: Sized {
    fn Exponent(&self) -> ::windows::core::Result<f64>;
    fn SetExponent(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEaseStaticsImpl: Sized {
    fn ExponentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfiguration2Impl: Sized {
    fn IsShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GravityConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeySplineImpl: Sized {
    fn ControlPoint1(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint1(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn ControlPoint2(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint2(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyTimeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyTimeHelperStaticsImpl: Sized {
    fn FromTimeSpan(&self, timespan: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<KeyTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearColorKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearDoubleKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearPointKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransitionImpl: Sized {
    fn DefaultNavigationTransitionInfo(&self) -> ::windows::core::Result<NavigationTransitionInfo>;
    fn SetDefaultNavigationTransitionInfo(&self, value: &::core::option::Option<NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransitionStaticsImpl: Sized {
    fn DefaultNavigationTransitionInfoProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoOverridesImpl: Sized {
    fn GetNavigationStateCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationStateCore(&self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ObjectKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ObjectKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaneThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaneThemeTransitionStaticsImpl: Sized {
    fn EdgeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<PointKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetValue(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PointKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransitionImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransitionStaticsImpl: Sized {
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEaseImpl: Sized {
    fn Power(&self) -> ::windows::core::Result<f64>;
    fn SetPower(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEaseStaticsImpl: Sized {
    fn PowerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IQuarticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IQuinticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IReorderThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatBehaviorHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatBehaviorHelperStaticsImpl: Sized {
    fn Forever(&self) -> ::windows::core::Result<RepeatBehavior>;
    fn FromCount(&self, count: f64) -> ::windows::core::Result<RepeatBehavior>;
    fn FromDuration(&self, duration: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<RepeatBehavior>;
    fn GetHasCount(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn GetHasDuration(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &RepeatBehavior, value: &RepeatBehavior) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransition2Impl: Sized {
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransitionStatics2Impl: Sized {
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISineEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfo2Impl: Sized {
    fn Effect(&self) -> ::windows::core::Result<SlideNavigationTransitionEffect>;
    fn SetEffect(&self, value: SlideNavigationTransitionEffect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfoStatics2Impl: Sized {
    fn EffectProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitCloseThemeAnimationImpl: Sized {
    fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitCloseThemeAnimationStaticsImpl: Sized {
    fn OpenedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitOpenThemeAnimationImpl: Sized {
    fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitOpenThemeAnimationStaticsImpl: Sized {
    fn OpenedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoryboardImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<TimelineCollection>;
    fn Seek(&self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Begin(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn GetCurrentState(&self) -> ::windows::core::Result<ClockState>;
    fn GetCurrentTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SeekAlignedToLastTick(&self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SkipToFill(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoryboardStaticsImpl: Sized {
    fn TargetPropertyProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetProperty(&self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetProperty(&self, element: &::core::option::Option<Timeline>, path: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetName(&self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, element: &::core::option::Option<Timeline>, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTarget(&self, timeline: &::core::option::Option<Timeline>, target: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISuppressNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ToVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineImpl: Sized {
    fn AutoReverse(&self) -> ::windows::core::Result<bool>;
    fn SetAutoReverse(&self, value: bool) -> ::windows::core::Result<()>;
    fn BeginTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>;
    fn SetBeginTime(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Duration>;
    fn SetDuration(&self, value: &super::super::Duration) -> ::windows::core::Result<()>;
    fn SpeedRatio(&self) -> ::windows::core::Result<f64>;
    fn SetSpeedRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn FillBehavior(&self) -> ::windows::core::Result<FillBehavior>;
    fn SetFillBehavior(&self, value: FillBehavior) -> ::windows::core::Result<()>;
    fn RepeatBehavior(&self) -> ::windows::core::Result<RepeatBehavior>;
    fn SetRepeatBehavior(&self, value: &RepeatBehavior) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Timeline>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineStaticsImpl: Sized {
    fn AllowDependentAnimations(&self) -> ::windows::core::Result<bool>;
    fn SetAllowDependentAnimations(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoReverseProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BeginTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DurationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpeedRatioProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FillBehaviorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RepeatBehaviorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransitionFactoryImpl: Sized {}
