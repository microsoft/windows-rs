use super::*;

#[derive(Debug)]
pub struct Signature {
    pub flags: MethodCallAttributes,
    pub return_type: Type,
    pub types: Vec<Type>,
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![],
        }
    }
}
