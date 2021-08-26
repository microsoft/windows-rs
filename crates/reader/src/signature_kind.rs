#[derive(PartialEq)]
pub enum SignatureKind {
    QueryInterface,
    ResultValue, // -> Result<T>
    ResultVoid, // -> Result<()>
    StructFixup,
    PreserveSig,
}
