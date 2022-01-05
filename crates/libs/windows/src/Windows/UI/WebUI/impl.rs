#[cfg(feature = "implement_exclusive")]
pub trait IActivatedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
pub trait IActivatedEventArgsDeferralImpl: Sized {
    fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivatedOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ActivatedDeferral>;
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IHtmlPrintDocumentSourceImpl: Sized + IPrintDocumentSourceImpl {
    fn Content(&self) -> ::windows::core::Result<PrintContent>;
    fn SetContent(&self, value: PrintContent) -> ::windows::core::Result<()>;
    fn LeftMargin(&self) -> ::windows::core::Result<f32>;
    fn SetLeftMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopMargin(&self) -> ::windows::core::Result<f32>;
    fn SetTopMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightMargin(&self) -> ::windows::core::Result<f32>;
    fn SetRightMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomMargin(&self) -> ::windows::core::Result<f32>;
    fn SetBottomMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn EnableHeaderFooter(&self) -> ::windows::core::Result<bool>;
    fn SetEnableHeaderFooter(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShrinkToFit(&self) -> ::windows::core::Result<bool>;
    fn SetShrinkToFit(&self, value: bool) -> ::windows::core::Result<()>;
    fn PercentScale(&self) -> ::windows::core::Result<f32>;
    fn SetPercentScale(&self, scalepercent: f32) -> ::windows::core::Result<()>;
    fn PageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrySetPageRange(&self, strpagerange: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INewWebUIViewCreatedEventArgsImpl: Sized {
    fn WebUIView(&self) -> ::windows::core::Result<WebUIView>;
    fn ActivatedEventArgs(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs>;
    fn HasPendingNavigate(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStaticsImpl: Sized {
    fn Activated(&self, handler: &::core::option::Option<ActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<ResumingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Navigated(&self, handler: &::core::option::Option<NavigatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics2Impl: Sized {
    fn LeavingBackground(&self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics3Impl: Sized {
    fn RequestRestartAsync(&self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics4Impl: Sized {
    fn NewWebUIViewCreated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWebUIViewCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BackgroundActivated(&self, handler: &::core::option::Option<BackgroundActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IWebUIBackgroundTaskInstanceImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn SetSucceeded(&self, succeeded: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIBackgroundTaskInstanceStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<IWebUIBackgroundTaskInstance>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
pub trait IWebUINavigatedEventArgsImpl: Sized {
    fn NavigatedOperation(&self) -> ::windows::core::Result<WebUINavigatedOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<WebUINavigatedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIViewImpl: Sized {
    fn ApplicationViewId(&self) -> ::windows::core::Result<i32>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIViewStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
    fn CreateWithUriAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
}
