use crate::*;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    // pub guid: Guid,
    pub methods: Vec<Method>,
    // pub default: bool,
    // pub exclusive: bool,
    // pub constructors: bool,
    // pub statics: bool,
    // pub overrides: bool,
    pub interfaces: Vec<Interface>,
}

impl Interface {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.methods.iter().flat_map(|m| m.dependencies()))
            .collect()
    }

    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let methods = def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();
        let interfaces = Vec::new();
        Self {
            name,
            methods,
            interfaces,
        }
    }

    fn from_type_ref(reader: &Reader, type_ref: TypeRef) -> Self {
        Self::from_type_def(reader, type_ref.resolve(reader))
    }

    fn from_type_spec(reader: &Reader, spec: TypeSpec) -> Self {
        let name = TypeName::from_type_spec(reader, spec);
        let methods = Vec::new();
        let interfaces = Vec::new();
        Self {
            name,
            methods,
            interfaces,
        }
    }

    fn from_type_def_or_ref(reader: &Reader, code: TypeDefOrRef) -> Self {
        match code {
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value),
        }
    }

    pub fn from_interface_impl(reader: &Reader, key: InterfaceImpl) -> Self {
        let mut interface = Self::from_type_def_or_ref(reader, key.interface(reader));
        // TODO: flip default/exclusive/overridable bits as needed
        interface
    }

    pub fn into_stream(&self) -> TokenStream {
        let name = &self.name.name;

        quote! {
            pub struct #name {

            }
        }
    }
}
