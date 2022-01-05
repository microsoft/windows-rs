pub trait ID3DBlobImpl: Sized {
    fn GetBufferPointer();
    fn GetBufferSize();
}
pub trait ID3DDestructionNotifierImpl: Sized {
    fn RegisterDestructionCallback();
    fn UnregisterDestructionCallback();
}
pub trait ID3DIncludeImpl: Sized {
    fn Open();
    fn Close();
}
