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

        let mut params = quote! {};

        for param in &signature.params {
            params.combine(write_param(writer, param));
        }

        let return_sig = writer.write_return_sig(method, &signature, false);
        let arches = write_arches(self.def);
        let cfg = self.write_cfg(writer);

        quote! {
            #arches
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#params) #return_sig>;
        }
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        self.method()
            .signature(self.def.namespace(), &[])
            .dependencies(dependencies);
    }
}

fn write_param(writer: &Writer, param: &Param) -> TokenStream {
    let name = param.write_ident();
    let type_name = param.write_name(writer);

    if writer.config.sys {
        return quote! { #name: #type_name, };
    }

    if param.is_input() {
        if param.is_copyable() {
            return quote! { #name: #type_name, };
        } else {
            return quote! { #name: windows_core::Ref<'_, #type_name>, };
        }
    }

    let deref = param.deref();

    if deref.is_interface() {
        let type_name = deref.write_name(writer);
        quote! { #name: windows_core::OutRef<'_, #type_name>, }
    } else {
        quote! { #name: #type_name, }
    }
}
