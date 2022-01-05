#[cfg(feature = "implement_exclusive")]
pub trait ICoreIncrementalInkStrokeImpl: Sized {
    fn AppendInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CreateInkStroke(&self) -> ::windows::core::Result<super::InkStroke>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<super::InkDrawingAttributes>;
    fn PointTransform(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreIncrementalInkStrokeFactoryImpl: Sized {
    fn Create(&self, drawingattributes: &::core::option::Option<super::InkDrawingAttributes>, pointtransform: &super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CoreIncrementalInkStroke>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSourceImpl: Sized {
    fn PointerEntering(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntering(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovering(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovering(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExiting(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExiting(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoving(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoving(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleasing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleasing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSource2Impl: Sized {
    fn PointerCursor(&self) -> ::windows::core::Result<super::super::super::Core::CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<super::super::super::Core::CoreCursor>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkIndependentInputSourceStaticsImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreInkIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreInkPresenterHostImpl: Sized {
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
    fn RootVisual(&self) -> ::windows::core::Result<super::super::super::Composition::ContainerVisual>;
    fn SetRootVisual(&self, value: &::core::option::Option<super::super::super::Composition::ContainerVisual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateEventArgsImpl: Sized {
    fn NewInkPoints(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>;
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn Disposition(&self) -> ::windows::core::Result<CoreWetStrokeDisposition>;
    fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateSourceImpl: Sized {
    fn WetStrokeStarting(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStarting(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeContinuing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeContinuing(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeStopping(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeStopping(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCompleted(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCompleted(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WetStrokeCanceled(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWetStrokeCanceled(&self, cookie: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWetStrokeUpdateSourceStaticsImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<super::InkPresenter>) -> ::windows::core::Result<CoreWetStrokeUpdateSource>;
}
