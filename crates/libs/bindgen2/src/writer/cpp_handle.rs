use super::*;

impl Writer {
    pub fn write_cpp_handle(&self, item: TypeDef) -> TokenStream {
        let name = to_ident(item.name());
        let ty = item.underlying_type();
        let ty_name = ty.write(self);

        if self.config.sys {
            quote! {
                pub type #name = #ty_name;
            }
        } else {
            let mut derive = quote! { Clone, Copy, Debug, PartialEq, Eq, };

            let default = if ty.is_pointer() {
                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            } else {
                derive.combine(quote! { Default, });
                quote! {}
            };

            let invalid = item.invalid_values();

            let is_invalid = if ty.is_pointer() && (invalid.is_empty() || invalid == [0]) {
                quote! {
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            self.0.is_null()
                        }
                    }
                }
            } else if invalid.is_empty() {
                quote! {}
            } else {
                let invalid = invalid.iter().map(|value| {
                    let literal = Literal::i64_unsuffixed(*value);

                    if ty.is_pointer() || (*value < 0 && ty.is_unsigned()) {
                        quote! { self.0 == #literal as _ }
                    } else {
                        quote! { self.0 == #literal }
                    }
                });
                quote! {
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            #(#invalid)||*
                        }
                    }
                }
            };

            let free = if let Some(function) = item.free_function() {
                if is_invalid.is_empty() {
                    // TODO: https://github.com/microsoft/win32metadata/issues/1891
                    quote! {}
                } else {
                    let free = to_ident(function.method.name());
                    let signature = function.method.signature(&[]);

                    // BCryptCloseAlgorithmProvider has an unused trailing parameter.
                    let tail = if signature.params.len() > 1 {
                        quote! { , 0 }
                    } else {
                        quote! {}
                    };

                    let result = if signature.return_type.0 == Type::Void {
                        quote! {}
                    } else {
                        quote! { _ = }
                    };

                    quote! {
                        impl windows_core::Free for #name {
                            #[inline]
                            unsafe fn free(&mut self) {
                                if !self.is_invalid() {
                                    #result #free(*self #tail);
                                }
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #[repr(transparent)]
                #[derive(#derive)]
                pub struct #name(pub #ty_name);
                impl windows_core::TypeKind for #name { // TODO: get rid of TypeKind on Win32 types
                    type TypeKind = windows_core::CopyType;
                }
                #is_invalid
                #free
                #default

            }
        }
    }
}
