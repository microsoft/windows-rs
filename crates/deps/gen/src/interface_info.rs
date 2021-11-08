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
        let mut tokens = TokenStream::with_capacity();

        for interface in interfaces {
            for (vtable_offset, method) in interface.def.methods().enumerate() {
                let name = method.rust_name();
                let overload = method_names.entry(name.clone()).or_insert(0);
                *overload += 1;

                let info = MethodInfo {
                    name,
                    vtable_offset: vtable_offset as u32 + 6,
                    overload: *overload,
                    is_deprecated: method.is_deprecated(),
                };

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
                    impl<#constraints> ::std::convert::From<#from> for #into {
                        fn from(value: #from) -> Self {
                            unsafe { ::core::mem::transmute(value) }
                        }
                    }
                    #cfg
                    impl<#constraints> ::std::convert::From<&#from> for #into {
                        fn from(value: &#from) -> Self {
                            ::std::convert::From::from(::std::clone::Clone::clone(value))
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::runtime::IntoParam<'a, #into> for #from {
                        fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                            ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::runtime::IntoParam<'a, #into> for &#from {
                        fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                            ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                        }
                    }
                }
            }
            InterfaceKind::NonDefault => {
                // Note: these implement `TryFrom` instead of `From` as they are fallible since new non-default interfaces
                // may be added in subsequent versions of a class.
                quote! {
                    #cfg
                    impl<#constraints> ::std::convert::TryFrom<#from> for #into {
                        type Error = ::windows::runtime::Error;
                        fn try_from(value: #from) -> ::windows::runtime::Result<Self> {
                            ::std::convert::TryFrom::try_from(&value)
                        }
                    }
                    #cfg
                    impl<#constraints> ::std::convert::TryFrom<&#from> for #into {
                        type Error = ::windows::runtime::Error;
                        fn try_from(value: &#from) -> ::windows::runtime::Result<Self> {
                            ::windows::runtime::Interface::cast(value)
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::runtime::IntoParam<'a, #into> for #from {
                        fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                            ::windows::runtime::IntoParam::into_param(&self)
                        }
                    }
                    #cfg
                    impl<'a, #constraints> ::windows::runtime::IntoParam<'a, #into> for &#from {
                        fn into_param(self) -> ::windows::runtime::Param<'a, #into> {
                            ::std::convert::TryInto::<#into>::try_into(self)
                                .map(::windows::runtime::Param::Owned)
                                .unwrap_or(::windows::runtime::Param::None)
                        }
                    }
                }
            }
            _ => TokenStream::new(),
        }
    }
}
