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

        // TODO: if the struct is blittable then don't generate a separate abi type.
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

        let clones = self.fields.iter().map(|(name, kind)| {
            let name = format_ident(&name);
            let clone = kind.gen_clone(&name);
            quote! {
                #name: #clone
            }
        });

        let debug_fields = self.fields.iter().filter_map(|(name, t)| {
            if let TypeKind::Delegate(name) = &t.kind {
                if !name.def.is_winrt() {
                    return None;
                }
            }

            let name_ident = format_ident(&name);

            Some(quote! {
                .field(#name, &format_args!("{:?}", self.#name_ident))
            })
        });

        let compare_fields = if self.fields.is_empty() {
            quote! { true }
        } else {
            let fields = self.fields.iter().map(|(name, t)| {
                let name_ident = format_ident(&name);

                if let TypeKind::Delegate(name) = &t.kind {
                    if !name.def.is_winrt() {
                        return quote! {
                            self.#name_ident.map(|f| f as usize) == other.#name_ident.map(|f| f as usize)
                        };
                    }
                }

                quote! {
                    self.#name_ident == other.#name_ident
                }
            });

            quote! {
                #(#fields)&&*
            }
        };

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

        let debug_name = self.name.name;

        quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            pub struct #name {
                #(#fields),*
            }
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
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct(#debug_name)
                        #(#debug_fields)*
                        .finish()
                }
            }
            impl ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self{ #(#clones),* }
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #compare_fields
                }
            }
            impl ::std::cmp::Eq for #name {}
            #runtime_type
        }
    }
}
