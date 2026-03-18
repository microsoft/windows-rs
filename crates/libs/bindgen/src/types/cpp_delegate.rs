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

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }

    pub fn write_cfg(&self, config: &Config) -> TokenStream {
        write_simple_cfg(self, config)
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let type_name = self.def.type_name();
        let name = to_ident(type_name.name());
        let method = self.method();
        let signature = method.method_signature(type_name.namespace(), &[], config.reader);

        let mut params = quote! {};

        for param in &signature.params {
            params.combine(write_param(config, param));
        }

        let return_sig = config.write_return_sig(method, &signature, false);
        let arches = write_arches(self.def);
        let cfg = self.write_cfg(config);

        let mut abi = None;

        if let Some(attribute) = self.def.find_attribute("UnmanagedFunctionPointerAttribute") {
            if let Some((_, Value::EnumValue(_, value))) = attribute.value().first() {
                match &**value {
                    Value::I32(1) => abi = Some("system"),
                    Value::I32(2) => abi = Some("C"),
                    Value::I32(5) => abi = Some("system"), // TODO: fastcall unsupported on non-x86 targets
                    rest => unreachable!("unexpected CallingConvention value in UnmanagedFunctionPointerAttribute: {rest:?}"),
                }
            }
        }

        quote! {
            #arches
            #cfg
            pub type #name = Option<unsafe extern #abi fn(#params) #return_sig>;
        }
    }
}

impl Dependencies for CppDelegate {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        self.method()
            .method_signature(self.def.namespace(), &[], reader)
            .combine(dependencies, reader);
    }
}

fn write_param(config: &Config, param: &Param) -> TokenStream {
    let name = param.write_ident();
    let type_name = param.write_name(config);

    if config.sys {
        return quote! { #name: #type_name, };
    }

    if param.is_input() {
        if param.is_copyable(config.reader) {
            return quote! { #name: #type_name, };
        } else {
            return quote! { #name: windows_core::Ref<#type_name>, };
        }
    }

    let deref = param.deref();

    if deref.is_interface() {
        let type_name = deref.write_name(config);
        quote! { #name: windows_core::OutRef<#type_name>, }
    } else {
        quote! { #name: #type_name, }
    }
}
