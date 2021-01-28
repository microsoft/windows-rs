use crate::*;
use squote::{format_ident, quote, Literal, TokenStream};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, Type)>,
    pub signature: String,
    pub is_typedef: bool,
    pub guid: TypeGuid,
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
        let mut unique = BTreeSet::new();

        for field in name.def.fields() {
            let mut t = Type::from_field(&field, &name.namespace);

            // TODO: workaround for https://github.com/microsoft/win32metadata/issues/132
            if let TypeKind::Delegate(_) = &t.kind {
                t.pointers = 0;
            }

            let mut field_name = to_snake(field.name());

            // A handful of Win32 structs, like `CIECHROMA` and `GenTspecParms`, have fields whose snake case
            // names are identical, so we handle this edge case by ensuring they get unique names.
            if !unique.insert(field_name.clone()) {
                let mut unique_count = 1;

                loop {
                    unique_count += 1;

                    let unique_field_name = format!("{}{}", field_name, unique_count);

                    if unique.insert(unique_field_name.clone()) {
                        field_name = unique_field_name;
                        break;
                    }
                }
            }

            fields.push((field_name, t));
        }

        let guid = TypeGuid::from_type_def(&name.def);

        // The C/C++ ABI assumes an empty struct occupies a single byte in memory.
        if fields.is_empty() && guid == TypeGuid::default() {
            let t = Type {
                kind: TypeKind::U8,
                pointers: 0,
                array: None,
                by_ref: false,
                modifiers: Vec::new(),
                param: None,
                name: "".to_string(),
                is_const: false,
                is_array: false,
                is_input: false,
            };

            fields.push(("reserved".to_string(), t));
        }

        let is_typedef = name
            .def
            .has_attribute(("Windows.Win32.Interop", "NativeTypedefAttribute"));

        Self {
            name,
            fields,
            signature,
            is_typedef,
            guid,
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

        if self.guid != TypeGuid::default() {
            let guid = self.name.gen_guid(&self.guid);

            return quote! {
                pub const #name: ::windows::Guid = #guid;
            };
        }

        // TODO: if the struct is blittable then don't generate a separate abi type.
        let abi_ident = format_ident!("{}_abi", self.name.name);

        let body = if self.is_typedef {
            let fields = self.fields.iter().map(|(_, kind)| {
                let kind = kind.gen_field();
                quote! {
                    pub #kind
                }
            });

            quote! {
                ( #(#fields),* );
            }
        } else {
            let fields = self.fields.iter().map(|(name, kind)| {
                let name = format_ident(&name);
                let kind = kind.gen_field();
                quote! {
                    pub #name: #kind
                }
            });

            quote! {
                { #(#fields),* }
            }
        };

        let defaults = if self.is_typedef {
            let defaults = self.fields.iter().map(|(_, kind)| {
                let value = kind.gen_default();
                quote! {
                    #value
                }
            });

            quote! {
                Self( #(#defaults),* )
            }
        } else {
            let defaults = self.fields.iter().map(|(name, kind)| {
                let name = format_ident(&name);
                let value = kind.gen_default();
                quote! {
                    #name: #value
                }
            });

            quote! {
                Self{ #(#defaults),* }
            }
        };

        let clones = if self.is_typedef {
            let clones = self.fields.iter().enumerate().map(|(index, (_, kind))| {
                let index = Literal::u32_unsuffixed(index as u32);
                let clone = kind.gen_clone(&quote! { #index });
                quote! {
                    #clone
                }
            });

            quote! {
                Self( #(#clones),* )
            }
        } else {
            let clones = self.fields.iter().map(|(name, kind)| {
                let name = format_ident(&name);
                let clone = kind.gen_clone(&quote! { #name });
                quote! {
                    #name: #clone
                }
            });

            quote! {
                Self{ #(#clones),* }
            }
        };

        let debug_fields = self
            .fields
            .iter()
            .enumerate()
            .filter_map(|(index, (name, t))| {
                if let TypeKind::Delegate(name) = &t.kind {
                    if !name.def.is_winrt() {
                        return None;
                    }
                }

                if self.is_typedef {
                    let index = Literal::u32_unsuffixed(index as u32);

                    Some(quote! {
                        .field(#name, &format_args!("{:?}", self.#index))
                    })
                } else {
                    let name_ident = format_ident(&name);

                    Some(quote! {
                        .field(#name, &format_args!("{:?}", self.#name_ident))
                    })
                }
            });

        let compare_fields = if self.fields.is_empty() {
            quote! { true }
        } else {
            let fields = self.fields.iter().enumerate().map(|(index, (name, t))| {
                let name_ident = format_ident(&name);

                if let TypeKind::Delegate(name) = &t.kind {
                    if !name.def.is_winrt() {
                        return quote! {
                            self.#name_ident.map(|f| f as usize) == other.#name_ident.map(|f| f as usize)
                        };
                    }
                }

                if self.is_typedef {
                    let index = Literal::u32_unsuffixed(index as u32);

                    quote! {
                        self.#index == other.#index
                    }
                } else {
                    quote! {
                        self.#name_ident == other.#name_ident
                    }
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
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
                }
            }
        };

        let copy = if self.is_typedef {
            quote! {
                impl ::std::marker::Copy for #name {}
            }
        } else {
            quote! {}
        };

        let debug_name = self.name.name;

        quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            pub struct #name #body
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_ident(#(#abi),*);
            unsafe impl ::windows::Abi for #name {
                type Abi = #abi_ident;
            }
            impl ::std::default::Default for #name {
                fn default() -> Self {
                    #defaults
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
                    #clones
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #compare_fields
                }
            }
            impl ::std::cmp::Eq for #name {}
            #copy
            #runtime_type
        }
    }
}
