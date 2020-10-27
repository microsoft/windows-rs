use crate::*;
use squote::{format_ident, quote, Literal, TokenStream};

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<(String, EnumConstant)>,
    pub signature: String,
    pub underlying_type: winmd::ElementType,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EnumConstant {
    U32(u32),
    I32(i32),
}

impl Enum {
    pub fn from_type_name(reader: &winmd::TypeReader, name: TypeName) -> Self {
        let signature = name.enum_signature(reader);
        let mut fields = Vec::new();

        for field in name.def.fields(reader) {
            for constant in field.constants(reader) {
                let name = field.name(reader).to_string();
                let mut value = constant.value(reader);

                let value = match constant.value_type(reader) {
                    winmd::ElementType::I32 => EnumConstant::I32(value.read_i32()),
                    winmd::ElementType::U32 => EnumConstant::U32(value.read_u32()),
                    _ => panic!("Enum::from_type_def"),
                };

                fields.push((name, value));
            }
        }

        let underlying_type = match fields[0].1 {
            EnumConstant::U32(_) => winmd::ElementType::U32,
            EnumConstant::I32(_) => winmd::ElementType::I32,
        };

        Self {
            name,
            fields,
            signature,
            underlying_type,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let signature = Literal::byte_string(&self.signature.as_bytes());

        let repr = match self.fields[0].1 {
            EnumConstant::U32(_) => format_ident!("u32"),
            EnumConstant::I32(_) => format_ident!("i32"),
        };

        let fields = self.fields.iter().map(|(name, value)| {
            let name = format_ident(&name);
            let value = match value {
                EnumConstant::U32(value) => quote! { #value },
                EnumConstant::I32(value) => quote! { #value },
            };

            quote! {
                pub const #name: Self = Self { value: #value };
            }
        });
        let bitwise = bitwise_operators(&name, self.fields[0].1);

        quote! {
            #[repr(transparent)]
            #[derive(::std::marker::Copy, ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::Eq, ::std::cmp::PartialEq)]
            pub struct #name {
                value: #repr
            }
            impl #name {
                #![allow(non_upper_case_globals)]
                #(#fields)*
            }
            unsafe impl ::winrt::RuntimeType for #name {
                const SIGNATURE: ::winrt::ConstBuffer = ::winrt::ConstBuffer::from_slice(#signature);
            }
            unsafe impl ::winrt::AbiTransferable for #name {
                type Abi = #repr;
                fn get_abi(&self) -> Self::Abi {
                    self.value
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    &mut self.value
                }
            }
            #bitwise
        }
    }
}

fn bitwise_operators(name: &TokenStream, value_type: EnumConstant) -> TokenStream {
    if let EnumConstant::I32(_) = value_type {
        return TokenStream::new();
    }

    quote! {
        impl ::std::ops::BitOr for #name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self { value: self.value | rhs.value }
            }
        }
        impl ::std::ops::BitAnd for #name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self { value: self.value & rhs.value }
            }
        }
    }
}
