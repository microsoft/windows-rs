#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageStatusEventImpl: Sized + IDispatchImpl {
    fn PackageStatusUpdated();
}
pub trait IPrintDocumentPackageTargetImpl: Sized {
    fn GetPackageTargetTypes();
    fn GetPackageTarget();
    fn Cancel();
}
pub trait IPrintDocumentPackageTargetFactoryImpl: Sized {
    fn CreateDocumentPackageTargetForPrintJob();
}
pub trait IXpsPrintJobImpl: Sized {
    fn Cancel();
    fn GetJobStatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStreamImpl: Sized + ISequentialStreamImpl {
    fn Close();
}
