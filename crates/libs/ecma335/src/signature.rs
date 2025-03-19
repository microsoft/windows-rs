use super::*;

pub struct Signature {
    pub flags: MethodCallAttributes,
    pub return_type: Type,
    pub types: Vec<Type>,
}
