#[derive(PartialEq)]
pub enum SignatureKind {
    QueryInterface,
    ResultValue,
    ResultVoid,
    StructFixup,
    PreserveSig,
}
