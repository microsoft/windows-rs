pub trait IWsbApplicationAsyncImpl: Sized {
    fn QueryStatus();
    fn Abort();
}
pub trait IWsbApplicationBackupSupportImpl: Sized {
    fn CheckConsistency();
}
pub trait IWsbApplicationRestoreSupportImpl: Sized {
    fn PreRestore();
    fn PostRestore();
    fn OrderComponents();
    fn IsRollForwardSupported();
}
