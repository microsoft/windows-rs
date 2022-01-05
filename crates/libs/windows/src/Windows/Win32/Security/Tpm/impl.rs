pub trait ITpmVirtualSmartCardManagerImpl: Sized {
    fn CreateVirtualSmartCard();
    fn DestroyVirtualSmartCard();
}
pub trait ITpmVirtualSmartCardManager2Impl: Sized + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithPinPolicy();
}
pub trait ITpmVirtualSmartCardManager3Impl: Sized + ITpmVirtualSmartCardManager2Impl + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithAttestation();
}
pub trait ITpmVirtualSmartCardManagerStatusCallbackImpl: Sized {
    fn ReportProgress();
    fn ReportError();
}
