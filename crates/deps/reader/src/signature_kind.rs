#[derive(PartialEq)]
pub enum SignatureKind {
    Query,
    QueryOptional,
    ResultValue,
    ResultVoid,
    ReturnStruct,
    PreserveSig,
}
