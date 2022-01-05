pub trait IPrintDialogCallbackImpl: Sized {
    fn InitDone();
    fn SelectionChange();
    fn HandleMessage();
}
pub trait IPrintDialogServicesImpl: Sized {
    fn GetCurrentDevMode();
    fn GetCurrentPrinterName();
    fn GetCurrentPortName();
}
