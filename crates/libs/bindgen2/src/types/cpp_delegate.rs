use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppDelegate {
    pub def: TypeDef,
}

impl Ord for CppDelegate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for CppDelegate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppDelegate {
    pub fn type_name(&self) -> TypeName<'static> {
        self.def.type_name()
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        self.type_name().write(writer, &[])
    }

    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let type_name = self.def.type_name();
        let name = to_ident(type_name.name());
        let method = self.method();
        let signature = method.signature(type_name.namespace(), &[]);

        let params = signature.params.iter().map(|(ty, param)| {
            let name = to_ident(&param.name().to_lowercase());
            let ty = ty.write_default(writer);
            quote! { #name: #ty }
        });

        let return_sig = writer.write_return_sig(method, &signature, false);

        // TODO: maybe create Dependencies with config as arg and use a trait to capture depdnencies if "package"
        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, type_name.namespace(), &dependencies, false);

        // TODO: are all callback "system" ABI?

        quote! {
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
        }
    }

    pub fn dependencies2(&self, dependencies: &mut Dependencies2) {
        self.method()
            .signature(self.def.namespace(), &[])
            .dependencies2(dependencies);
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        let type_name = self.type_name();
        if dependencies.insert(type_name) {
            self.method()
                .signature(type_name.namespace(), &[])
                .dependencies(dependencies);
        }
    }
}
