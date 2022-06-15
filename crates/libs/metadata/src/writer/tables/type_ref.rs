use super::*;

#[derive(Default, Clone)]
pub struct TypeRef {
    pub type_name: TypeName,
    pub assembly_ref: String,
    pub assembly_index: ResolutionScope,
}

impl TypeRef {
    pub fn new(assembly_ref: &str, type_name: TypeName) -> Self {
        Self { assembly_ref: assembly_ref.to_string(), type_name, ..Default::default() }
    }

    pub fn system_value_type() -> Self {
        Self::new("mscorlib", TypeName::system_value_type())
    }

    pub fn system_enum() -> Self {
        Self::new("mscorlib", TypeName::system_enum())
    }

    pub fn system_delegate() -> Self {
        Self::new("mscorlib", TypeName::system_delegate())
    }
}
