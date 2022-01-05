pub trait IOplockBreakingHandlerImpl: Sized {
    fn OplockBreaking();
}
pub trait IRandomAccessStreamFileAccessModeImpl: Sized {
    fn GetMode();
}
pub trait IStorageFolderHandleAccessImpl: Sized {
    fn Create();
}
pub trait IStorageItemHandleAccessImpl: Sized {
    fn Create();
}
pub trait IUnbufferedFileHandleOplockCallbackImpl: Sized {
    fn OnBrokenCallback();
}
pub trait IUnbufferedFileHandleProviderImpl: Sized {
    fn OpenUnbufferedFileHandle();
    fn CloseUnbufferedFileHandle();
}
