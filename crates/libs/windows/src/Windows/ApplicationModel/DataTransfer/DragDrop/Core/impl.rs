#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragDropManagerImpl: Sized {
    fn TargetRequested(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetRequested(&self, value: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AreConcurrentOperationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragDropManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreDragDropManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragInfoImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::DataPackageView>;
    fn Modifiers(&self) -> ::windows::core::Result<super::DragDropModifiers>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragInfo2Impl: Sized + ICoreDragInfoImpl {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::DataPackage>;
    fn SetPointerId(&self, pointerid: u32) -> ::windows::core::Result<()>;
    fn SetDragUIContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn DragUIContentMode(&self) -> ::windows::core::Result<CoreDragUIContentMode>;
    fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragOperation2Impl: Sized + ICoreDragOperationImpl {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation>;
    fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragUIOverrideImpl: Sized {
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsContentVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCaptionVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCaptionVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsGlyphVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGlyphVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
pub trait ICoreDropOperationTargetImpl: Sized {
    fn EnterAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDropOperationTargetRequestedEventArgsImpl: Sized {
    fn SetTarget(&self, target: &::core::option::Option<ICoreDropOperationTarget>) -> ::windows::core::Result<()>;
}
