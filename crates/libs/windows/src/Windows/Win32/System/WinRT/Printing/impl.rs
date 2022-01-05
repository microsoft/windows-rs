pub trait IPrintManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPrintUIForWindowAsync();
}
pub trait IPrintWorkflowConfigurationNativeImpl: Sized {
    fn PrinterQueue();
    fn DriverProperties();
    fn UserProperties();
}
pub trait IPrintWorkflowObjectModelSourceFileContentNativeImpl: Sized {
    fn StartXpsOMGeneration();
    fn ObjectFactory();
}
pub trait IPrintWorkflowXpsObjectModelTargetPackageNativeImpl: Sized {
    fn DocumentPackageTarget();
}
pub trait IPrintWorkflowXpsReceiverImpl: Sized {
    fn SetDocumentSequencePrintTicket();
    fn SetDocumentSequenceUri();
    fn AddDocumentData();
    fn AddPage();
    fn Close();
}
pub trait IPrintWorkflowXpsReceiver2Impl: Sized + IPrintWorkflowXpsReceiverImpl {
    fn Failed();
}
pub trait IPrinting3DManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPrintUIForWindowAsync();
}
