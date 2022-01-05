#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntryImpl: Sized {
    fn DisplayInfo(&self) -> ::windows::core::Result<super::AppDisplayInfo>;
    fn LaunchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry2Impl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry3Impl: Sized {
    fn LaunchForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry4Impl: Sized {
    fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suspending(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::SuspendingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn GetCurrentView(&self) -> ::windows::core::Result<CoreApplicationView>;
    fn Run(&self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<()>;
    fn RunWithActivationFactories(&self, activationfactorycallback: &::core::option::Option<super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplication2Impl: Sized {
    fn BackgroundActivated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LeavingBackground(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplication3Impl: Sized {
    fn RequestRestartAsync(&self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationExitImpl: Sized {
    fn Exit(&self) -> ::windows::core::Result<()>;
    fn Exiting(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveExiting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ICoreApplicationUnhandledErrorImpl: Sized {
    fn UnhandledErrorDetected(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationUseCountImpl: Sized {
    fn IncrementApplicationUseCount(&self) -> ::windows::core::Result<()>;
    fn DecrementApplicationUseCount(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationViewImpl: Sized {
    fn CoreWindow(&self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsMain(&self) -> ::windows::core::Result<bool>;
    fn IsHosted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationView2Impl: Sized {
    fn Dispatcher(&self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationView3Impl: Sized {
    fn IsComponent(&self) -> ::windows::core::Result<bool>;
    fn TitleBar(&self) -> ::windows::core::Result<CoreApplicationViewTitleBar>;
    fn HostedViewClosing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHostedViewClosing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationView5Impl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationView6Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationViewTitleBarImpl: Sized {
    fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExtendViewIntoTitleBar(&self) -> ::windows::core::Result<bool>;
    fn SystemOverlayLeftInset(&self) -> ::windows::core::Result<f64>;
    fn SystemOverlayRightInset(&self) -> ::windows::core::Result<f64>;
    fn Height(&self) -> ::windows::core::Result<f64>;
    fn LayoutMetricsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutMetricsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn IsVisibleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsVisibleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplicationImpl: Sized {
    fn Views(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>;
    fn CreateNewView(&self, runtimetype: &::windows::core::HSTRING, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<CoreApplicationView>;
    fn MainView(&self) -> ::windows::core::Result<CoreApplicationView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication2Impl: Sized {
    fn CreateNewViewFromMainView(&self) -> ::windows::core::Result<CoreApplicationView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication3Impl: Sized {
    fn CreateNewViewWithViewSource(&self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<CoreApplicationView>;
}
pub trait IFrameworkViewImpl: Sized {
    fn Initialize(&self, applicationview: &::core::option::Option<CoreApplicationView>) -> ::windows::core::Result<()>;
    fn SetWindow(&self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<()>;
    fn Load(&self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Run(&self) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
}
pub trait IFrameworkViewSourceImpl: Sized {
    fn CreateView(&self) -> ::windows::core::Result<IFrameworkView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostedViewClosingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn Propagate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorDetectedEventArgsImpl: Sized {
    fn UnhandledError(&self) -> ::windows::core::Result<UnhandledError>;
}
