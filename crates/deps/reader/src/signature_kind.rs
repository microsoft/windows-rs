#[derive(PartialEq)]
pub enum SignatureKind {
    Query,
    QueryOptional,
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnVoid,
    PreserveSig,
}
