#[derive(PartialEq)]
pub enum SignatureKind {
    QueryInterface,
    ReturnValue,
    ReturnUdt,
    PreserveSig,
    ReturnVoid,
}
