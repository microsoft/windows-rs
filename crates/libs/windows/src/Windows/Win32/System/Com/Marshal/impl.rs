pub trait IMarshalImpl: Sized {
    fn GetUnmarshalClass();
    fn GetMarshalSizeMax();
    fn MarshalInterface();
    fn UnmarshalInterface();
    fn ReleaseMarshalData();
    fn DisconnectObject();
}
pub trait IMarshal2Impl: Sized + IMarshalImpl {}
pub trait IMarshalingStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetMarshalingContextAttribute();
}
