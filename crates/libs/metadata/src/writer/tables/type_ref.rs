use super::*;

#[derive(Default, Clone)]
pub struct TypeRef {
    pub type_name: TypeName,
    pub assembly_ref: AssemblyRef,
    pub assembly_index: ResolutionScope,
}

impl TypeRef {
    pub fn system_value_type() -> Self {
        Self { assembly_ref: AssemblyRef::mscorlib(), type_name: TypeName::system_value_type(), ..Default::default() }
    }

    pub fn system_enum() -> Self {
        Self { assembly_ref: AssemblyRef::mscorlib(), type_name: TypeName::system_enum(), ..Default::default() }
    }

    pub fn system_delegate() -> Self {
        Self { assembly_ref: AssemblyRef::mscorlib(), type_name: TypeName::system_delegate(), ..Default::default() }
    }
}
