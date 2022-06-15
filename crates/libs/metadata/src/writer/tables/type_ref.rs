use super::*;

#[derive(Default)]
pub struct TypeRef {
    pub assembly_ref: String,
    pub type_name: TypeName,
}

impl TypeRef {
    pub fn new(assembly_ref: &str, type_name: TypeName) -> Self {
        Self { assembly_ref: assembly_ref.to_string(), type_name }
    }
    pub fn system_value_type() -> Self {
        Self::new("mscorlib", TypeName::new("System", "ValueType"))
    }
    pub fn system_enum() -> Self {
        Self::new("mscorlib", TypeName::new("System", "Enum"))
    }
    pub fn system_delegate() -> Self {
        Self::new("mscorlib", TypeName::new("System", "MulticastDelegate"))
    }
}
