pub trait ICompositionFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetContentTag();
    fn GetCompositionFrameId();
    fn GetDisplayInstanceArray();
}
pub trait IIndependentFlipFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetOutputAdapterLUID();
    fn GetOutputVidPnSourceId();
    fn GetContentTag();
    fn GetDisplayedTime();
    fn GetPresentDuration();
}
pub trait IPresentStatisticsImpl: Sized {
    fn GetPresentId();
    fn GetKind();
}
pub trait IPresentStatusPresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetCompositionFrameId();
    fn GetPresentStatus();
}
pub trait IPresentationBufferImpl: Sized {
    fn GetAvailableEvent();
    fn IsAvailable();
}
pub trait IPresentationContentImpl: Sized {
    fn SetTag();
}
pub trait IPresentationFactoryImpl: Sized {
    fn IsPresentationSupported();
    fn IsPresentationSupportedWithIndependentFlip();
    fn CreatePresentationManager();
}
pub trait IPresentationManagerImpl: Sized {
    fn AddBufferFromResource();
    fn CreatePresentationSurface();
    fn GetNextPresentId();
    fn SetTargetTime();
    fn SetPreferredPresentDuration();
    fn ForceVSyncInterrupt();
    fn Present();
    fn GetPresentRetiringFence();
    fn CancelPresentsFrom();
    fn GetLostEvent();
    fn GetPresentStatisticsAvailableEvent();
    fn EnablePresentStatisticsKind();
    fn GetNextPresentStatistics();
}
pub trait IPresentationSurfaceImpl: Sized + IPresentationContentImpl {
    fn SetBuffer();
    fn SetColorSpace();
    fn SetAlphaMode();
    fn SetSourceRect();
    fn SetTransform();
    fn RestrictToOutput();
    fn SetDisableReadback();
    fn SetLetterboxingMargins();
}
