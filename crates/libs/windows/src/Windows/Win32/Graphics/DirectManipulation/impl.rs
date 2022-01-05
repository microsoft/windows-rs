pub trait IDirectManipulationAutoScrollBehaviorImpl: Sized {
    fn SetConfiguration();
}
pub trait IDirectManipulationCompositorImpl: Sized {
    fn AddContent();
    fn RemoveContent();
    fn SetUpdateManager();
    fn Flush();
}
pub trait IDirectManipulationCompositor2Impl: Sized + IDirectManipulationCompositorImpl {
    fn AddContentWithCrossProcessChaining();
}
pub trait IDirectManipulationContentImpl: Sized {
    fn GetContentRect();
    fn SetContentRect();
    fn GetViewport();
    fn GetTag();
    fn SetTag();
    fn GetOutputTransform();
    fn GetContentTransform();
    fn SyncContentTransform();
}
pub trait IDirectManipulationDeferContactServiceImpl: Sized {
    fn DeferContact();
    fn CancelContact();
    fn CancelDeferral();
}
pub trait IDirectManipulationDragDropBehaviorImpl: Sized {
    fn SetConfiguration();
    fn GetStatus();
}
pub trait IDirectManipulationDragDropEventHandlerImpl: Sized {
    fn OnDragDropStatusChange();
}
pub trait IDirectManipulationFrameInfoProviderImpl: Sized {
    fn GetNextFrameInfo();
}
pub trait IDirectManipulationInteractionEventHandlerImpl: Sized {
    fn OnInteraction();
}
pub trait IDirectManipulationManagerImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn RegisterHitTestTarget();
    fn ProcessInput();
    fn GetUpdateManager();
    fn CreateViewport();
    fn CreateContent();
}
pub trait IDirectManipulationManager2Impl: Sized + IDirectManipulationManagerImpl {
    fn CreateBehavior();
}
pub trait IDirectManipulationManager3Impl: Sized + IDirectManipulationManager2Impl + IDirectManipulationManagerImpl {
    fn GetService();
}
pub trait IDirectManipulationPrimaryContentImpl: Sized {
    fn SetSnapInterval();
    fn SetSnapPoints();
    fn SetSnapType();
    fn SetSnapCoordinate();
    fn SetZoomBoundaries();
    fn SetHorizontalAlignment();
    fn SetVerticalAlignment();
    fn GetInertiaEndTransform();
    fn GetCenterPoint();
}
pub trait IDirectManipulationUpdateHandlerImpl: Sized {
    fn Update();
}
pub trait IDirectManipulationUpdateManagerImpl: Sized {
    fn RegisterWaitHandleCallback();
    fn UnregisterWaitHandleCallback();
    fn Update();
}
pub trait IDirectManipulationViewportImpl: Sized {
    fn Enable();
    fn Disable();
    fn SetContact();
    fn ReleaseContact();
    fn ReleaseAllContacts();
    fn GetStatus();
    fn GetTag();
    fn SetTag();
    fn GetViewportRect();
    fn SetViewportRect();
    fn ZoomToRect();
    fn SetViewportTransform();
    fn SyncDisplayTransform();
    fn GetPrimaryContent();
    fn AddContent();
    fn RemoveContent();
    fn SetViewportOptions();
    fn AddConfiguration();
    fn RemoveConfiguration();
    fn ActivateConfiguration();
    fn SetManualGesture();
    fn SetChaining();
    fn AddEventHandler();
    fn RemoveEventHandler();
    fn SetInputMode();
    fn SetUpdateMode();
    fn Stop();
    fn Abandon();
}
pub trait IDirectManipulationViewport2Impl: Sized + IDirectManipulationViewportImpl {
    fn AddBehavior();
    fn RemoveBehavior();
    fn RemoveAllBehaviors();
}
pub trait IDirectManipulationViewportEventHandlerImpl: Sized {
    fn OnViewportStatusChanged();
    fn OnViewportUpdated();
    fn OnContentUpdated();
}
