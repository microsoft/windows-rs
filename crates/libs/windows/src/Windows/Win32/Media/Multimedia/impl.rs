pub trait IAVIEditStreamImpl: Sized {
    fn Cut();
    fn Copy();
    fn Paste();
    fn Clone();
    fn SetInfo();
}
pub trait IAVIFileImpl: Sized {
    fn Info();
    fn GetStream();
    fn CreateStream();
    fn WriteData();
    fn ReadData();
    fn EndRecord();
    fn DeleteStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAVIPersistFileImpl: Sized + IPersistFileImpl + IPersistImpl {
    fn Reserved1();
}
pub trait IAVIStreamImpl: Sized {
    fn Create();
    fn Info();
    fn FindSample();
    fn ReadFormat();
    fn SetFormat();
    fn Read();
    fn Write();
    fn Delete();
    fn ReadData();
    fn WriteData();
    fn SetInfo();
}
pub trait IAVIStreamingImpl: Sized {
    fn Begin();
    fn End();
}
pub trait IGetFrameImpl: Sized {
    fn GetFrame();
    fn Begin();
    fn End();
    fn SetFormat();
}
