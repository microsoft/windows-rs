use super::*;

pub fn gen_enum(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    let name = gen_type_name(def, gen);
    let underlying_type = def.underlying_type();

    // WinRT enums don't have the flags attribute but are paritioned merely based
    // on whether they are signed.
    let bitwise = matches!(underlying_type, ElementType::U32);

    // Win32 enums sadly don't use unsigned values uniformly so we need to rely
    // on the flags attribute.
    let bitwise = if bitwise || def.has_flags() {
        quote! {
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
            impl ::std::ops::BitOrAssign for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0)
                }
            }
            impl ::std::ops::BitAndAssign for #name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0)
                }
            }
            impl ::std::ops::Not for #name {
                type Output = Self;

                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
        }
    } else {
        quote! {}
    };

    let underlying_type = gen_name(&underlying_type, gen);
    let mut last: Option<ConstantValue> = None;

    let fields: Vec<Field> = def.fields().collect();

    // A minimal enum definition  still include all fields unless there are too many fields.
    // In such cases, the build script simply needs to import the type directly to generate all fields.
    let fields = if include == TypeInclude::Full || fields.len() < 100 {
        let fields = fields.iter().filter_map(|field| {
            if field.is_literal() {
                let field_name = to_ident(field.name());

                if let Some(constant) = field.constant() {
                    let value = gen_constant_value(&constant.value());

                    Some(quote! {
                        pub const #field_name: #name = #name(#value);
                    })
                } else if let Some(last_value) = &last {
                    let next = last_value.next();
                    let value = gen_constant_value(&next);
                    last = Some(next);

                    Some(quote! {
                        pub const #field_name: #name = #name(#value);
                    })
                } else {
                    last = Some(ConstantValue::I32(0));

                    Some(quote! {
                        pub const #field_name: #name = #name(0);
                    })
                }
            } else {
                None
            }
        });

        if def.is_scoped() {
            quote! {
                impl #name {
                    #(#fields)*
                }
            }
        } else {
            quote! {
                #(#fields)*
            }
        }
    } else {
        TokenStream::new()
    };

    let runtime_type = if def.is_winrt() {
        let signature = Literal::byte_string(def.type_signature().as_bytes());

        quote! {
            unsafe impl ::windows::RuntimeType for #name {
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::marker::Copy, ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
        #[repr(transparent)]
        pub struct #name(pub #underlying_type);
        #fields
        impl ::std::convert::From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::Abi for #name {
            type Abi = Self;
            type DefaultType = Self;
        }
        #runtime_type
        #bitwise
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t =
            TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "AsyncStatus"));
        assert_eq!(
            t.type_signature(),
            "enum(Windows.Foundation.AsyncStatus;i4)"
        );
    }
}
