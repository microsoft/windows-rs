use super::*;

#[derive(Clone, Debug)]
pub enum CppMethodOrName {
    Method(CppMethod),
    Name(&'static str),
}

#[derive(Clone, Debug)]
pub struct CppInterface {
    pub def: TypeDef,
    pub methods: Vec<CppMethodOrName>,
    pub base_interfaces: Vec<Type>,
    // TODO: store dependencies here (in expand) to avoid repeated calls to self.base_interfaces()
}

impl PartialEq for CppInterface {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def
    }
}

impl Eq for CppInterface {}

impl Ord for CppInterface {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppInterface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppInterface {
    pub fn expand(&mut self, filter: &NameTree) {
        let namespace = self.def.namespace();

        self.methods = self
            .def
            .methods()
            .map(|def| {
                let method = CppMethod::new(def, namespace);
                if method.dependencies.included(filter) {
                    CppMethodOrName::Method(method)
                } else {
                    CppMethodOrName::Name(method.def.name())
                }
            })
            .collect();

        self.base_interfaces = self.base_interfaces();

        for interface in self.base_interfaces.iter_mut() {
            if let Type::Item(Item::CppInterface(item)) = interface {
                item.expand(filter);
            }
        }
    }

    fn has_unknown_base(&self) -> bool {
        matches!(self.base_interfaces.first(), Some(Type::IUnknown))
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies);
        }

        let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);
        let vtbl_name = self.write_vtbl_name();

        let vtbl = {
            let core = writer.write_core();

            let base = match self.base_interfaces.last() {
                Some(Type::IUnknown) => {
                    quote! { pub base__: #core IUnknown_Vtbl, }
                }
                Some(Type::Object) => {
                    quote! { pub base__: #core IInspectable_Vtbl, }
                }
                Some(Type::Item(Item::CppInterface(item))) => {
                    let namespace = writer.write_namespace(item.def.namespace());
                    let name = item.write_vtbl_name();
                    quote! { pub base__: #namespace #name, }
                }
                _ => quote! {},
            };

            let mut names = MethodNames::new();

            let methods = self.methods.iter().map(|method| match method {
                CppMethodOrName::Method(method) => {
                    let mut difference = Dependencies::new();

                    if writer.config.package {
                        difference = method.dependencies.difference(&dependencies);
                    }

                    let name = names.add(method.def);
                    let abi = method.write_abi(writer, false);
                    let cfg = writer.write_cfg(self.def, self.def.namespace(), &difference, false);

                    if cfg.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn #abi,
                        }
                    } else {
                        let cfg_not =
                            writer.write_cfg(self.def, self.def.namespace(), &difference, true);

                        quote! {
                            #cfg
                            pub #name: unsafe extern "system" fn #abi,
                            #cfg_not
                            #name: usize,
                        }
                    }
                }
                CppMethodOrName::Name(name) => {
                    let name = to_ident(name);
                    quote! { #name: usize, }
                }
            });

            quote! {
                #cfg
                #[repr(C)]
                pub struct #vtbl_name {
                    #base
                    #(#methods)*
                }
            }
        };

        if writer.config.sys {
            let mut result = quote! {};

            if self.has_unknown_base() {
                if let Some(guid) = self.def.guid_attribute() {
                    let name: TokenStream = format!("IID_{}", self.def.name()).into();
                    result.combine(writer.write_cpp_const_guid(name, &guid));
                }
            }

            result.combine(vtbl);
            result
        } else {
            let name = to_ident(self.def.name());

            let mut result = if self.has_unknown_base() {
                if let Some(guid) = self.def.guid_attribute() {
                    let guid = writer.write_guid_u128(&guid);

                    quote! {
                        #cfg
                        windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                    }
                } else {
                    quote! {
                        #cfg
                        windows_core::imp::define_interface!(#name, #vtbl_name, 0);
                    }
                }
            } else {
                quote! {
                    #cfg
                    windows_core::imp::define_interface!(#name, #vtbl_name);
                }
            };

            let method_names = &mut MethodNames::new();
            let virtual_names = &mut MethodNames::new();
            let mut methods = quote! {};

            for method in self.methods.iter().filter_map(|method| match &method {
                CppMethodOrName::Method(method) => Some(method),
                _ => None,
            }) {
                let mut difference = Dependencies::new();

                if writer.config.package {
                    difference = method.dependencies.difference(&dependencies);
                }

                let cfg = writer.write_cfg(self.def, self.def.namespace(), &difference, false);

                let method = method.write(writer, method_names, virtual_names);

                methods.combine(quote! {
                    #cfg
                    #method
                });
            }

            if !methods.is_empty() {
                result.combine(quote! {
                    impl #name {
                        #methods
                    }
                });
            }

            result.combine(vtbl);
            result
        }
    }

    fn write_vtbl(&self, _writer: &Writer) -> TokenStream {
        quote! {}
    }

    fn write_vtbl_name(&self) -> TokenStream {
        format!("{}_Vtbl", self.def.name()).into()
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for interface in self.base_interfaces() {
                if let Type::Item(Item::CppInterface(item)) = interface {
                    item.dependencies(dependencies);
                }
            }
        }
    }

    pub fn base_interfaces(&self) -> Vec<Type> {
        let mut bases = vec![];
        let mut def = self.def;

        while let Some(base) = def.interface_impls().map(move |imp| imp.ty(&[])).next() {
            match base {
                Type::Item(Item::CppInterface(ref item)) => {
                    def = item.def;
                    bases.insert(0, base);
                }
                Type::Object => {
                    bases.insert(0, Type::IUnknown);
                    bases.insert(1, Type::Object);
                    break;
                }
                Type::IUnknown => {
                    bases.insert(0, Type::IUnknown);
                    break;
                }
                rest => unimplemented!("{rest:?}"),
            }
        }

        bases
    }
}
