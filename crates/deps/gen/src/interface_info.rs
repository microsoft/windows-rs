use super::*;

pub struct InterfaceInfo {
    pub def: TypeDef,
    pub kind: InterfaceKind,
    pub is_base: bool,
    pub version: (u16, u16),
}

impl InterfaceInfo {
    pub fn sort(result: &mut Vec<Self>) {
        result.sort_by(|a, b| {
            if a.is_base != b.is_base {
                return a.is_base.cmp(&b.is_base);
            }

            if a.version != b.version {
                return a.version.cmp(&b.version);
            }

            if a.kind != b.kind {
                return a.kind.cmp(&b.kind);
            }

            a.def.type_name().cmp(&b.def.type_name())
        });
    }

    pub fn gen_methods(interfaces: &[Self], gen: &Gen) -> TokenStream {
        let mut method_names = BTreeMap::<String, u32>::new();
        let mut tokens = TokenStream::new();

        for interface in interfaces {
            if interface.is_base && gen.relative.starts_with("Windows.UI.Xaml") && !interface.def.namespace().starts_with("Windows.Foundation") {
                continue;
            }

            for (vtable_offset, method) in interface.def.methods().enumerate() {
                let name = method.rust_name();
                let overload = method_names.entry(name.clone()).or_insert(0);
                *overload += 1;

                let info = MethodInfo { name, vtable_offset: vtable_offset as u32 + 6, overload: *overload, is_deprecated: method.is_deprecated() };

                let signature = method.signature(&interface.def.generics);
                tokens.combine(&gen_winrt_method(&signature, &info, interface, gen));
            }
        }

        tokens
    }

    pub fn gen_conversion(&self, from: &TokenStream, constraints: &TokenStream, features: &BTreeSet<&'static str>, gen: &Gen) -> TokenStream {
        if self.def.is_exclusive() {
            return TokenStream::new();
        }

        let into = gen_type_name(&self.def, gen);
        let mut features = features.clone();
        features.insert(self.def.namespace());
        let cfg = gen.gen_cfg(&features);

        match self.kind {
            InterfaceKind::Default => {
                quote! {
                    #cfg
                    impl<#constraints> ::core::convert::From<#from> for #into {
                        fn from(value: #from) -> Self {
                            unsafe { ::core::mem::transmute(value) }
                        }
                    }
                    #cfg
                    impl<#constraints> ::core::convert::From<&#from> for #into {
                        fn from(value: &#from) -> Self {
                            ::core::convert::From::from(::core::clone::Clone::clone(value))
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::core::IntoParam<'a, #into> for #from {
                        fn into_param(self) -> ::windows::core::Param<'a, #into> {
                            ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::core::IntoParam<'a, #into> for &#from {
                        fn into_param(self) -> ::windows::core::Param<'a, #into> {
                            ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                        }
                    }
                }
            }
            InterfaceKind::NonDefault => {
                // Note: these implement `TryFrom` instead of `From` as they are fallible since new non-default interfaces
                // may be added in subsequent versions of a class.
                quote! {
                    #cfg
                    impl<#constraints> ::core::convert::TryFrom<#from> for #into {
                        type Error = ::windows::core::Error;
                        fn try_from(value: #from) -> ::windows::core::Result<Self> {
                            ::core::convert::TryFrom::try_from(&value)
                        }
                    }
                    #cfg
                    impl<#constraints> ::core::convert::TryFrom<&#from> for #into {
                        type Error = ::windows::core::Error;
                        fn try_from(value: &#from) -> ::windows::core::Result<Self> {
                            ::windows::core::Interface::cast(value)
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::core::IntoParam<'a, #into> for #from {
                        fn into_param(self) -> ::windows::core::Param<'a, #into> {
                            ::windows::core::IntoParam::into_param(&self)
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::core::IntoParam<'a, #into> for &#from {
                        fn into_param(self) -> ::windows::core::Param<'a, #into> {
                            ::core::convert::TryInto::<#into>::try_into(self)
                                .map(::windows::core::Param::Owned)
                                .unwrap_or(::windows::core::Param::None)
                        }
                    }
                }
            }
            _ => TokenStream::new(),
        }
    }
}
