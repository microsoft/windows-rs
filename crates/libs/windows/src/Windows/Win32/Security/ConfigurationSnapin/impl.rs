pub trait ISceSvcAttachmentDataImpl: Sized {
    fn GetData();
    fn Initialize();
    fn FreeBuffer();
    fn CloseHandle();
}
pub trait ISceSvcAttachmentPersistInfoImpl: Sized {
    fn Save();
    fn IsDirty();
    fn FreeBuffer();
}
