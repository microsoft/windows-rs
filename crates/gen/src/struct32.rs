use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Struct32 {
    pub name: TypeName,
    pub fields: Vec<(&'static str, TypeKind, u32)>,
}

impl Struct32 {
    pub fn from_type_name(name: TypeName) -> Self {
        let mut fields = Vec::new();

        for field in name.def.fields() {
            let (kind, pointers) = TypeKind::from_field(&field, &name.namespace);
            fields.push((field.name(), kind, pointers));
        }

        Self { name, fields }
    }

    pub fn gen(&self) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        Vec::new()
    }
}
