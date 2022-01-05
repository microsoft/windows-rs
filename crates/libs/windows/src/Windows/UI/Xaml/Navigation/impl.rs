#[cfg(feature = "implement_exclusive")]
pub trait IFrameNavigationOptionsImpl: Sized {
    fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitionInfoOverride(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
    fn SetTransitionInfoOverride(&self, value: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameNavigationOptionsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameNavigationOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigatingCancelEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigatingCancelEventArgs2Impl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationEventArgsImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationEventArgs2Impl: Sized {
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationFailedEventArgsImpl: Sized {
    fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryImpl: Sized {
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryFactoryImpl: Sized {
    fn CreateInstance(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, navigationtransitioninfo: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<PageStackEntry>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryStaticsImpl: Sized {
    fn SourcePageTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
