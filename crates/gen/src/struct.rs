use crate::*;
use squote::{format_ident, quote, Literal, TokenStream};

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, Type)>,
    pub signature: String,
}

impl Struct {
    pub fn from_type_name(name: TypeName) -> Self {
        let is_winrt = name.def.is_winrt();

        let signature = if is_winrt {
            name.struct_signature()
        } else {
            String::new()
        };

        let mut fields = Vec::new();

        for field in name.def.fields() {
            let field_name = if is_winrt {
                to_snake(field.name(), MethodKind::Normal)
            } else {
                field.name().to_string()
            };

            let t = Type::from_field(&field, &name.namespace);

            fields.push((field_name, t));
        }

        Self {
            name,
            fields,
            signature,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.1.kind.dependencies())
            .collect()
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let abi_ident = format_ident!("{}_abi", self.name.name);

        let fields = self.fields.iter().map(|(name, kind)| {
            let name = format_ident(&name);
            let kind = kind.gen_field();
            quote! {
                pub #name: #kind
            }
        });

        let defaults = self.fields.iter().map(|(name, kind)| {
            let name = format_ident(&name);
            let value = kind.gen_default();
            quote! {
                #name: #value
            }
        });

        let abi = self.fields.iter().map(|field| field.1.gen_abi());

        let runtime_type = if self.signature.is_empty() {
            TokenStream::new()
        } else {
            let signature = Literal::byte_string(&self.signature.as_bytes());

            quote! {
                unsafe impl ::winrt::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::winrt::ConstBuffer = ::winrt::ConstBuffer::from_slice(#signature);
                }
            }
        };

        // TODO: unroll these traits - it's too expensive to call derive macro.
        // https://github.com/microsoft/winrt-rs/issues/353

        quote! {
            #[repr(C)]
            #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
            pub struct #name {
                #(#fields),*
            }
            impl ::std::cmp::Eq for #name {}
            #[repr(C)]
            pub struct #abi_ident(#(#abi),*);
            unsafe impl ::winrt::Abi for #name {
                type Abi = #abi_ident;
            }
            impl ::std::default::Default for #name {
                fn default() -> Self {
                    Self{ #(#defaults),* }
                }
            }
            #runtime_type
        }
    }
}
