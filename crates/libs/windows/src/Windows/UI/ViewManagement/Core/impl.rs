#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewImpl: Sized {
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn FrameworkAnimationRecommended(&self) -> ::windows::core::Result<bool>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreFrameworkInputViewStaticsImpl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UIContext>) -> ::windows::core::Result<CoreFrameworkInputView>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreFrameworkInputView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewImpl: Sized {
    fn OcclusionsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOcclusionsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCoreInputViewOcclusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn TryShowPrimaryView(&self) -> ::windows::core::Result<bool>;
    fn TryHidePrimaryView(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView2Impl: Sized {
    fn XYFocusTransferringFromPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferringFromPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XYFocusTransferredToPrimaryView(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXYFocusTransferredToPrimaryView(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryTransferXYFocusToPrimaryView(&self, origin: &super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView3Impl: Sized {
    fn TryShow(&self) -> ::windows::core::Result<bool>;
    fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool>;
    fn TryHide(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView4Impl: Sized {
    fn PrimaryViewShowing(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewShowing(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewHiding(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewHiding(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputView5Impl: Sized {
    fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows::core::Result<bool>;
    fn SupportedKindsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedKindsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrimaryViewAnimationStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrimaryViewAnimationStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewAnimationStartingEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AnimationDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewHidingEventArgsImpl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewOcclusionImpl: Sized {
    fn OccludingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn OcclusionKind(&self) -> ::windows::core::Result<CoreInputViewOcclusionKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewOcclusionsChangedEventArgsImpl: Sized {
    fn Occlusions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewShowingEventArgsImpl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreInputView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewStatics2Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UIContext>) -> ::windows::core::Result<CoreInputView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInputViewTransferringXYFocusEventArgsImpl: Sized {
    fn Origin(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Direction(&self) -> ::windows::core::Result<CoreInputViewXYFocusTransferDirection>;
    fn SetTransferHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransferHandled(&self) -> ::windows::core::Result<bool>;
    fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepPrimaryViewVisible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsControllerImpl: Sized {
    fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetAutoHideScrollBars(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetMessageDuration(&self, value: u32) -> ::windows::core::Result<()>;
    fn SetTextScaleFactor(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsControllerStaticsImpl: Sized {
    fn RequestDefaultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>>;
}
