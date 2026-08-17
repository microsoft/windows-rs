use super::*;

impl Struct {
    pub(super) fn write(
        &self,
        architectures: &TokenStream,
        cfg: &TokenStream,
        layout: Layout,
        projection: Projection,
        custom_derives: &[String],
    ) -> TokenStream {
        let name = tokens::ident(&self.name);
        if self.fields.is_empty() {
            let repr = self.repr();
            if self.union {
                let nested = self
                    .nested
                    .iter()
                    .map(|nested| nested.write_context_with_cfg(layout, projection, cfg));
                return quote! {
                    #repr
                    #architectures
                    #cfg
                    #[derive(Clone, Copy)]
                    pub union #name {
                        pub value: u8,
                    }
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #(#nested)*
                };
            }
            let nested = self
                .nested
                .iter()
                .map(|nested| nested.write_context_with_cfg(layout, projection, cfg));
            let (derive, default) =
                self.default_tokens(&name, architectures, cfg, projection, custom_derives);
            return quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub struct #name(pub u8);
                #default
                #(#nested)*
            };
        }
        let fields =
            self.fields
                .iter()
                .zip(&self.field_copy)
                .map(|((field_name, ty), copyable)| {
                    let field_name = tokens::ident(field_name);
                    let projected = ty.write_field_projection_owner(
                        &self.namespace,
                        &self.name,
                        layout,
                        projection,
                    );
                    let projected = if self.union
                        && !projection.is_sys()
                        && !copyable
                        && !ty.is_interface()
                        && !ty.is_bstr()
                        && !ty.is_hstring()
                    {
                        quote! { core::mem::ManuallyDrop<#projected> }
                    } else {
                        projected
                    };
                    quote! { pub #field_name: #projected, }
                });
        let repr = self.repr();
        let nested = self
            .nested
            .iter()
            .map(|nested| nested.write_context_with_cfg(layout, projection, cfg));
        if self.union {
            let derive = if projection.is_sys() || self.copyable {
                quote! { #[derive(Clone, Copy)] }
            } else {
                quote! {}
            };
            let manual_clone = (!projection.is_sys() && !self.copyable).then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Clone for #name {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                }
            });
            quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub union #name {
                    #(#fields)*
                }
                #manual_clone
                #architectures
                #cfg
                impl Default for #name {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #(#nested)*
            }
        } else {
            let (derive, default) =
                self.default_tokens(&name, architectures, cfg, projection, custom_derives);
            let bitfields = self.write_bitfields(&name, architectures, cfg, projection);
            quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub struct #name {
                    #(#fields)*
                }
                #bitfields
                #default
                #(#nested)*
            }
        }
    }

    fn write_bitfields(
        &self,
        name: &TokenStream,
        architectures: &TokenStream,
        cfg: &TokenStream,
        projection: Projection,
    ) -> TokenStream {
        if projection.is_sys() || self.bitfields.is_empty() {
            return quote! {};
        }
        let accessors = self
            .bitfields
            .iter()
            .filter_map(Bitfield::write)
            .collect::<Vec<_>>();
        if accessors.is_empty() {
            return quote! {};
        }
        quote! {
            #architectures
            #cfg
            impl #name {
                #(#accessors)*
            }
        }
    }

    fn repr(&self) -> TokenStream {
        if let Some(align) = self.align {
            let align = Literal::u32_unsuffixed(align);
            quote! { #[repr(C, align(#align))] }
        } else if let Some(packing) = self.packing {
            let packing = Literal::u16_unsuffixed(packing);
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        }
    }

    fn default_tokens(
        &self,
        name: &TokenStream,
        architectures: &TokenStream,
        cfg: &TokenStream,
        projection: Projection,
        custom_derives: &[String],
    ) -> (TokenStream, TokenStream) {
        let custom_derives = custom_derives
            .iter()
            .map(|derive| tokens::ident(derive))
            .collect::<Vec<_>>();
        if !projection.is_sys() && self.manual_clone {
            let default = (self.default != native_default::Policy::Derive).then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            });
            return (
                quote! {},
                quote! {
                    #architectures
                    #cfg
                    impl Clone for #name {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                    #default
                },
            );
        }
        if !projection.is_sys() && self.packing.is_none() {
            let copy = self.traits.copy.then(|| quote! { , Copy });
            let debug = self.traits.debug.then(|| quote! { , Debug });
            let partial_eq = self.traits.partial_eq.then(|| quote! { , PartialEq });
            let eq = self.traits.eq.then(|| quote! { , Eq });
            let derive_default = matches!(
                self.default,
                native_default::Policy::Derive | native_default::Policy::ScopedEnum
            )
            .then(|| quote! { , Default });
            let default = (!matches!(
                self.default,
                native_default::Policy::Derive | native_default::Policy::ScopedEnum
            ))
            .then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            });
            return (
                quote! {
                    #[derive(Clone #copy #debug #(, #custom_derives)* #derive_default #eq #partial_eq)]
                },
                default.unwrap_or_default(),
            );
        }
        if !projection.is_sys() && !self.copyable {
            if self.default != native_default::Policy::Derive {
                let derive = if custom_derives.is_empty() {
                    TokenStream::new()
                } else {
                    quote! { #[derive(#(#custom_derives),*)] }
                };
                return (
                    derive,
                    quote! {
                        #architectures
                        #cfg
                        impl Default for #name {
                            fn default() -> Self {
                                unsafe { core::mem::zeroed() }
                            }
                        }
                    },
                );
            }
            return (
                quote! { #[derive(#(#custom_derives,)* Default)] },
                quote! {},
            );
        }
        if self.default != native_default::Policy::Derive {
            (
                quote! { #[derive(Clone, Copy #(, #custom_derives)*)] },
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                },
            )
        } else {
            (
                quote! { #[derive(Clone, Copy #(, #custom_derives)*, Default)] },
                quote! {},
            )
        }
    }
}
