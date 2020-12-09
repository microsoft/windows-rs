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

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.1.kind.dependencies())
            .collect()
    }

    pub fn gen(&self) -> TokenStream {
        // TODO: finish structs first
        // Make sure there's an ABI reprsentation
        // So that we can still have safe wrappers for things like structs of BSTRs

        let name = self.name.gen();

        let mut fields = TokenStream::new();

        for (name, t) in &self.fields {
            let name = format_ident(&name);
            fields.combine(&quote! { pub #name: });

            for _ in 0..t.pointers {
                fields.combine(&quote! { *mut });
            }

            let kind = t.kind.gen_field();
            fields.combine(&quote! { #kind, });
        }

        quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            pub struct #name {
                #fields
            }
        }
    }
}
