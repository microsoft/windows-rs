pub trait IADesktopP2Impl: Sized {
    fn ReReadWallpaper();
    fn GetADObjectFlags();
    fn UpdateAllDesktopSubscriptions();
    fn MakeDynamicChanges();
}
pub trait IActiveDesktopPImpl: Sized {
    fn SetSafeMode();
    fn EnsureUpdateHTML();
    fn SetScheme();
    fn GetScheme();
}
pub trait IBriefcaseInitiatorImpl: Sized {
    fn IsMonikerInBriefcase();
}
pub trait IEmptyVolumeCacheImpl: Sized {
    fn Initialize();
    fn GetSpaceUsed();
    fn Purge();
    fn ShowProperties();
    fn Deactivate();
}
pub trait IEmptyVolumeCache2Impl: Sized + IEmptyVolumeCacheImpl {
    fn InitializeEx();
}
pub trait IEmptyVolumeCacheCallBackImpl: Sized {
    fn ScanProgress();
    fn PurgeProgress();
}
pub trait IReconcilableObjectImpl: Sized {
    fn Reconcile();
    fn GetProgressFeedbackMaxEstimate();
}
pub trait IReconcileInitiatorImpl: Sized {
    fn SetAbortCallback();
    fn SetProgressFeedback();
}
