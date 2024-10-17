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
    pub fn write(&self, writer: &Writer) -> TokenStream {
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
            let ty = ty.write_default(writer);
            quote! { #name: #ty }
        });

        let return_sig = writer.write_return_sig(self.method, &signature);

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies, &writer.config);
        }

        let vararg =
            if writer.config.sys && signature.call_flags.contains(MethodCallAttributes::VARARG) {
                quote! { , ... }
            } else {
                quote! {}
            };

        let link = link_fmt(quote! {
            windows_targets::link!(#library #abi #symbol fn #name(#(#params),* #vararg) #return_sig);
        });

        let cfg = writer.write_cfg(self.method, self.def.namespace(), dependencies, false);

        if writer.config.sys {
            return quote! {
                #cfg
                #link
            };
        }

        // TODO: build wrapper

        quote! {}
    }
}

impl Writer {
    pub fn write_return_sig(&self, method: MethodDef, signature: &Signature) -> TokenStream {
        match &signature.return_type.0 {
            Type::Void => {
                if method.has_attribute("DoesNotReturnAttribute") {
                    quote! { -> ! }
                } else {
                    quote! {}
                }
            }
            ty => {
                let ty = ty.write_default(self);
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
