pub trait IInkCommitRequestHandlerImpl: Sized {
    fn OnCommitRequested();
}
pub trait IInkD2DRendererImpl: Sized {
    fn Draw();
}
pub trait IInkD2DRenderer2Impl: Sized {
    fn Draw();
}
pub trait IInkDesktopHostImpl: Sized {
    fn QueueWorkItem();
    fn CreateInkPresenter();
    fn CreateAndInitializeInkPresenter();
}
pub trait IInkHostWorkItemImpl: Sized {
    fn Invoke();
}
pub trait IInkPresenterDesktopImpl: Sized {
    fn SetRootVisual();
    fn SetCommitRequestHandler();
    fn GetSize();
    fn SetSize();
    fn OnHighContrastChanged();
}
