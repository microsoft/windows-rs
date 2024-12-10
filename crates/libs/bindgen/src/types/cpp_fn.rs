use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppFn {
    pub namespace: &'static str,
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
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace, self.method.name())
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        self.type_name().write(writer, &[])
    }

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
            panic!()
        };

        let signature = self.method.signature(self.namespace, &[]);

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
        let signature = self.method.signature(self.namespace, &[]);
        let mut dependencies = TypeMap::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let link = self.write_link(writer, false);
        let cfg = writer.write_cfg(self.method, self.namespace, &dependencies, false);
        let window_long = self.write_window_long();
        if writer.config.sys {
            return quote! {
                #cfg
                #link
                #window_long
            };
        }

        let method = CppMethod::new(self.method, self.namespace);
        let args = method.write_args();
        let params = method.write_params(writer);
        let generics = method.write_generics();
        let abi_return_type = method.write_return(writer);

        let wrapper = match method.return_hint {
            ReturnHint::Query(..) => {
                let where_clause = method.write_where(writer, true);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params) -> windows_core::Result<T> #where_clause {
                        #link
                        let mut result__ = core::ptr::null_mut();
                        #name(#args).and_then(||windows_core::Type::from_abi(result__))
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                let where_clause = method.write_where(writer, true);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params result__: *mut Option<T>) -> windows_core::Result<()> #where_clause {
                        #link
                        #name(#args).ok()
                    }
                }
            }
            ReturnHint::ResultValue => {
                let where_clause = method.write_where(writer, false);
                let return_type = signature.params[signature.params.len() - 1].0.deref();

                let map = if !return_type.is_nullable() {
                    quote! { map(||core::mem::transmute(result__)) }
                } else {
                    quote! { and_then(||windows_core::Type::from_abi(result__)) }
                };

                let return_type = return_type.write_name(writer);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                        #link
                        let mut result__ = core::mem::zeroed();
                        #name(#args).#map
                    }
                }
            }
            ReturnHint::ResultVoid => {
                let where_clause = method.write_where(writer, false);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<()> #where_clause {
                        #link
                        #name(#args).ok()
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let where_clause = method.write_where(writer, false);

                let return_type = method.signature.params[method.signature.params.len() - 1]
                    .0
                    .deref();

                if return_type.is_nullable() {
                    let return_type = return_type.write_name(writer);

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
                    let map = if return_type.is_copyable() {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let where_clause = method.write_where(writer, false);
                    let return_type = return_type.write_name(writer);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> #return_type #where_clause {
                            #link
                            let mut result__ = core::mem::zeroed();
                            #name(#args);
                            #map
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct | ReturnHint::None => {
                let where_clause = method.write_where(writer, false);

                if method.handle_last_error() {
                    let return_type = signature.return_type.0.write_name(writer);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                            #link
                            let result__ = #name(#args);
                            (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
                        }
                    }
                } else {
                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) #abi_return_type #where_clause {
                            #link
                            #name(#args)
                        }
                    }
                }
            }
        };

        quote! {
            #wrapper
            #window_long
        }
    }

    fn write_window_long(&self) -> TokenStream {
        match self.method.name() {
            "GetWindowLongPtrA" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use GetWindowLongA as GetWindowLongPtrA;
            },
            "GetWindowLongPtrW" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use GetWindowLongW as GetWindowLongPtrW;
            },
            "SetWindowLongPtrA" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use SetWindowLongA as SetWindowLongPtrA;
            },
            "SetWindowLongPtrW" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use SetWindowLongW as SetWindowLongPtrW;
            },
            _ => quote! {},
        }
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        self.method
            .signature(self.namespace, &[])
            .dependencies(dependencies);

        match self.method.name() {
            "GetWindowLongPtrA" => self
                .method
                .reader()
                .unwrap_full_name("Windows.Win32.UI.WindowsAndMessaging", "GetWindowLongA")
                .dependencies(dependencies),
            "GetWindowLongPtrW" => self
                .method
                .reader()
                .unwrap_full_name("Windows.Win32.UI.WindowsAndMessaging", "GetWindowLongW")
                .dependencies(dependencies),
            "SetWindowLongPtrA" => self
                .method
                .reader()
                .unwrap_full_name("Windows.Win32.UI.WindowsAndMessaging", "SetWindowLongA")
                .dependencies(dependencies),
            "SetWindowLongPtrW" => self
                .method
                .reader()
                .unwrap_full_name("Windows.Win32.UI.WindowsAndMessaging", "SetWindowLongW")
                .dependencies(dependencies),
            _ => {}
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

fn link_fmt(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.0.replacen(" ! (  ", "!(", 1);
    tokens = tokens.replacen(" ( ", "(", 1);
    tokens = tokens.replace(" , ", ", ");
    tokens = tokens.replace(" )", ")");
    tokens.into()
}
