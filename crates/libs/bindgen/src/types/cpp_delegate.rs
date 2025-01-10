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
    pub fn type_name(&self) -> TypeName {
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

    pub fn write_cfg(&self, writer: &Writer) -> TokenStream {
        if !writer.config.package {
            return quote! {};
        }

        let mut dependencies = TypeMap::new();
        self.dependencies(&mut dependencies);
        Cfg::new(self.def, &dependencies).write(writer, false)
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
        let arches = write_arches(self.def);
        let cfg = self.write_cfg(writer);

        quote! {
            #arches
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
        }
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        self.method()
            .signature(self.def.namespace(), &[])
            .dependencies(dependencies);
    }
}
