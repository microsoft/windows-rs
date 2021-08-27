#[derive(PartialEq)]
pub enum SignatureKind {
    QueryInterface,
    ResultValue,
    ResultVoid,
    ReturnStruct,
    PreserveSig,
}
