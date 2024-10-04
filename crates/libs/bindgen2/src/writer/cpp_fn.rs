use super::*;

impl Writer {
    pub fn write_cpp_fn(&self, item: &'static CppFn) -> TokenStream {
        let name = to_ident(item.method.name());
        let library = item.method.module_name().to_lowercase();
        let impl_map = item.method.impl_map().expect("ImplMap not found");
        let symbol = impl_map.import_name();
        let impl_flags = impl_map.flags();

        let abi = if impl_flags.contains(PInvokeAttributes::CallConvPlatformapi) {
            "system"
        } else if impl_flags.contains(PInvokeAttributes::CallConvCdecl) {
            "cdecl"
        } else {
            unimplemented!()
        };

        let signature = item.method.signature(&[]);

        let params = signature.params.iter().map(|(ty, param)| {
            let name = to_ident(param.name());
            let ty = self.write_default_name(&ty);
            quote! { #name: #ty }
        });

        let return_sig = self.write_return_sig(item.method, &signature);

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.method, item.def.namespace(), dependencies, false);

        let link = quote! {
            #cfg
            windows_targets::link!(#library #abi #symbol fn #name(#(#params)*) #return_sig);
        };

        if self.sys {
            return link;
        }

        // TODO: build wrapper

        quote! {}
    }

    pub fn write_return_sig(&self, method: MethodDef, signature: &Signature) -> TokenStream {
        match &signature.return_type.0 {
            Type::Void => {
                if method.has_attribute("DoesNotReturnAttribute") {
                    quote! { -> ! }
                } else {
                    quote! {}
                }
            }
            rest => {
                let ty = self.write_default_name(&rest);
                quote! { -> #ty }
            }
        }
    }
}
