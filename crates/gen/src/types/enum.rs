use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Enum(pub tables::TypeDef);

impl Enum {
    pub fn definition(&self) -> Option<tables::TypeDef> {
        Some(self.0)
    }

    pub fn type_signature(&self) -> String {
        let underlying_type = match self.underlying_type() {
            ElementType::I32 => "i4",
            ElementType::U32 => "u4",
            _ => unexpected!(),
        };

        format!(
            "enum({}.{};{})",
            self.0.namespace(),
            self.0.name(),
            underlying_type
        )
    }

    pub fn underlying_type(&self) -> ElementType {
        for field in self.0.fields() {
            if let Some(constant) = field.constant() {
                return constant.value_type();
            } else {
                let blob = &mut field.blob();
                blob.read_unsigned();
                blob.read_modifiers();

                blob.read_expected(0x1D);
                blob.read_modifiers();

                return ElementType::from_blob(blob, &[]);
            }
        }

        unexpected!();
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let underlying_type = self.underlying_type();

        // TODO: add bitwise operators if its u32 *or* if its got the Win32 flags attribute
        let bitwise = match underlying_type {
            ElementType::I32 => TokenStream::new(),
            ElementType::U32 => quote! {
                // TODO: add BitOrAssign and BitAndAssign
                impl ::std::ops::BitOr for #name {
                    type Output = Self;

                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for #name {
                    type Output = Self;

                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
            },
            _ => unexpected!(),
        };

        let underlying_type = match underlying_type {
            ElementType::I32 => format_ident!("i32"),
            ElementType::U32 => format_ident!("u32"),
            _ => unexpected!(),
        };

        let mut last: Option<ConstantValue> = None;

        let fields = self.0.fields().filter_map(|field| {
            if field.flags().literal() {
                let name = to_ident(field.name());

                if let Some(constant) = field.constant() {
                    let value = constant.value().gen_value();

                    Some(quote! {
                        pub const #name: Self = Self(#value);
                    })
                } else if let Some(last_value) = &last {
                    let next = last_value.next();
                    let value = next.gen_value();
                    last = Some(next);

                    Some(quote! {
                        pub const #name: Self = Self(#value);
                    })
                } else {
                    // TODO: need test for implicit value enums (and create win32metadata bug)
                    last = Some(ConstantValue::I32(0));

                    Some(quote! {
                        pub const #name: Self = Self(0);
                    })
                }
            } else {
                None
            }
        });

        let runtime_type = if self.0.is_winrt() {
            let signature = Literal::byte_string(&self.type_signature().as_bytes());

            quote! {
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #[allow(non_camel_case_types)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::marker::Copy, ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
            #[repr(transparent)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #![allow(non_upper_case_globals)]
                #(#fields)*
            }
            impl ::std::convert::From<#underlying_type> for #name {
                fn from(value: #underlying_type) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for #name {
                type Abi = Self;
            }
            #runtime_type
            #bitwise
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get_enum("Windows.Foundation", "AsyncStatus");
        assert_eq!(
            t.type_signature(),
            "enum(Windows.Foundation.AsyncStatus;i4)"
        );
    }
}
