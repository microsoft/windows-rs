use super::*;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: metadata::Type,
    /// Reconstructed inline nested record for C anonymous aggregate members.
    pub nested: Option<Box<Struct>>,
    /// Logical bit-field members packed into this synthetic storage unit.
    pub bitfields: Vec<(String, u32, u32)>,
}

/// Function or method parameter with optional SAL metadata.
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    /// SAL-derived direction/optional flags.
    pub annotation: ParamAnnotation,
}
