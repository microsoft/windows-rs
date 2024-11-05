use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CppFn {
    pub def: TypeDef,
    pub method: MethodDef,
}

impl Ord for CppFn {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.method.name(), self.method).cmp(&(other.method.name(), other.method))
    }
}

impl PartialOrd for CppFn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppFn {
    pub fn write_link(&self, writer: &Writer, underlying_types: bool) -> TokenStream {
        let name = self.method.name();
        let library = self.method.module_name().to_lowercase();
        let impl_map = self.method.impl_map().expect("ImplMap not found");
        let mut symbol = Some(impl_map.import_name());

        if symbol == Some(name) {
            symbol = None;
        }

        let name = to_ident(self.method.name());
        let impl_flags = impl_map.flags();

        let abi = if impl_flags.contains(PInvokeAttributes::CallConvPlatformapi) {
            "system"
        } else if impl_flags.contains(PInvokeAttributes::CallConvCdecl) {
            "cdecl"
        } else {
            unimplemented!()
        };

        let signature = self.method.signature(&[]);

        let params = signature.params.iter().map(|(ty, param)| {
            let name = to_ident(&param.name().to_lowercase());
            let ty = if underlying_types {
                ty.underlying_type().write_abi(writer)
            } else {
                ty.write_abi(writer)
            };
            quote! { #name: #ty }
        });

        let return_sig = writer.write_return_sig(self.method, &signature, underlying_types);

        let vararg =
            if writer.config.sys && signature.call_flags.contains(MethodCallAttributes::VARARG) {
                quote! { , ... }
            } else {
                quote! {}
            };

        link_fmt(quote! {
            windows_targets::link!(#library #abi #symbol fn #name(#(#params),* #vararg) #return_sig);
        })
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.method.name());
        let signature = self.method.signature(&[]);
        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let link = self.write_link(writer, false);
        let cfg = writer.write_cfg(self.method, self.def.namespace(), &dependencies, false);

        if writer.config.sys {
            return quote! {
                #cfg
                #link
            };
        }

        let method = CppMethod::new(self.method, self.def.namespace());
        // dbg!(&method);
        let args = method.write_args();
        let params = method.write_params(writer);
        let generics = method.write_generics();
        let where_clause = method.write_where(writer);
        let abi_return_type = method.write_return(writer);

        match method.return_hint {
            ReturnHint::Query(..) => {
                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params) -> windows_core::Result<T> where #where_clause T: windows_core::Interface {
                        #link
                        let mut result__ = core::ptr::null_mut();
                        #name(#args).and_then(||windows_core::Type::from_abi(result__))
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params result__: *mut Option<T>) -> windows_core::Result<()> where #where_clause  T: windows_core::Interface {
                        #link
                        #name(#args).ok()
                    }
                }
            }
            ReturnHint::ResultValue => {
                let return_type = signature.params[signature.params.len() - 1].0.deref();

                let map = if return_type.is_blittable() {
                    quote! { map(||result__) }
                } else {
                    quote! { and_then(||windows_core::Type::from_abi(result__)) }
                };

                let return_type = return_type.write(writer);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> where #where_clause {
                        #link
                        let mut result__ = core::mem::zeroed();
                        #name(#args).#map
                    }
                }
            }
            ReturnHint::ResultVoid => {
                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<()> where #where_clause {
                        #link
                        #name(#args).ok()
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let return_type = method.signature.params[method.signature.params.len() - 1]
                    .0
                    .deref();

                if return_type.is_nullable() {
                    let return_type = return_type.write(writer);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                            #link
                            let mut result__ = core::mem::zeroed();
                            #name(#args);
                            windows_core::Type::from_abi(result__)
                        }
                    }
                } else {
                    let map = if return_type.is_blittable() {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let return_type = return_type.write(writer);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> #return_type where #where_clause {
                            #link
                            let mut result__ = core::mem::zeroed();
                            #name(#args);
                            #map
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct | ReturnHint::None => {
                if method.handle_last_error() {
                    let return_type = signature.return_type.0.write(writer);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> where #where_clause {
                            #link
                            let result__ = #name(#args);
                            (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
                        }
                    }
                } else {
                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) #abi_return_type where #where_clause {
                            #link
                            #name(#args)
                        }
                    }
                }
            }
            ReturnHint::ReturnVoid => {
                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) where #where_clause {
                        #link
                        #name(#args)
                    }
                }
            }
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.method.name()) {
            self.method.signature(&[]).dependencies(dependencies);
        }
    }
}

impl Writer {
    pub fn write_return_sig(
        &self,
        method: MethodDef,
        signature: &Signature,
        underlying_types: bool,
    ) -> TokenStream {
        match &signature.return_type.0 {
            Type::Void => {
                if method.has_attribute("DoesNotReturnAttribute") {
                    quote! { -> ! }
                } else {
                    quote! {}
                }
            }
            ty => {
                let ty = if underlying_types {
                    ty.underlying_type().write_default(self)
                } else {
                    ty.write_default(self)
                };

                quote! { -> #ty }
            }
        }
    }
}

// TODO: https://github.com/microsoft/windows-rs/issues/3314
fn link_fmt(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.0.replacen(" ! (  ", "!(", 1);
    tokens = tokens.replacen(" ( ", "(", 1);
    tokens = tokens.replace(" , ", ", ");
    tokens = tokens.replace(" )", ")");
    tokens.into()
}
