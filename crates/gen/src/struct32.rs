use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Struct32 {
    pub name: TypeName,
    pub fields: Vec<(&'static str, Type)>,
}

impl Struct32 {
    pub fn from_type_name(name: TypeName) -> Self {
        let mut fields = Vec::new();

        for field in name.def.fields() {
            let t = Type::from_field(&field, &name.namespace);
            fields.push((field.name(), t));
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
