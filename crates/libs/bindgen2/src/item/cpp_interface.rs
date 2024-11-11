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
        let vtbl_name = self.write_vtbl_name(writer);

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
                    let name = item.write_vtbl_name(writer);
                    quote! { pub base__: #name, }
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

            if !writer.config.package {
                if self.has_unknown_base() {
                    if let Some(guid) = self.def.guid_attribute() {
                        let name: TokenStream = format!("IID_{}", self.def.name()).into();
                        result.combine(writer.write_cpp_const_guid(name, &guid));
                    }
                }

                result.combine(vtbl);
            }

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

            if let Some(Type::Item(Item::CppInterface(base))) = self.base_interfaces.last() {
                let base = base.write_name(writer);

                result.combine(quote! {
                    #cfg
                    impl core::ops::Deref for #name {
                        type Target = #base;
                        fn deref(&self) -> &Self::Target {
                            unsafe { core::mem::transmute(self) }
                        }
                    }
                });
            }

            if !self.base_interfaces.is_empty() {
                let bases = self.base_interfaces.iter().map(|ty| ty.write(writer));
                result.combine(quote! {
                    #cfg
                    windows_core::imp::interface_hierarchy!(#name, #(#bases),*);
                })
            }

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
                    #cfg
                    impl #name {
                        #methods
                    }
                });
            }

            result.combine(vtbl);

            let impl_name: TokenStream = format!("{}_Impl", self.def.name()).into();

            // TODO: need to test code gen each time this split happens
            if writer.config.package {
                fn collect(interface: &CppInterface, dependencies: &mut Dependencies) {
                    for method in interface.methods.iter() {
                        if let CppMethodOrName::Method(method) = method {
                            dependencies.combine(&method.dependencies);
                        }
                    }
                }

                collect(self, &mut dependencies);
                self.base_interfaces.iter().for_each(|interface| {
                    if let Type::Item(Item::CppInterface(item)) = interface {
                        collect(item, &mut dependencies);
                    }
                });
            }

            let cfg = writer.write_cfg(self.def, self.def.namespace(), &dependencies, false);

            let mut names = MethodNames::new();

            let field_methods: Vec<_> = self
                .methods
                .iter()
                .map(|method| match method {
                    CppMethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        if self.has_unknown_base() {
                        quote! { #name: #name::<Identity, OFFSET>, }
                        } else {
                            quote! { #name: #name::<Identity>, }
                        }
                    }
                    CppMethodOrName::Name(name) => {
                        // TODO: test this condition - should cause an AV when method is called
                        // TODO: does this need to use `MethodNames` for overloading?
                        let name = to_ident(name);
                        quote! { #name: 0, }
                    }
                })
                .collect();

            let mut names = MethodNames::new();

            let impl_methods: Vec<_> = self.methods.iter().map(|method| match method {
                CppMethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(writer, true);
                    let upcall = method.write_upcall(&impl_name, &name);

                    if self.has_unknown_base() {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name, const OFFSET: isize> #signature {
                            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            #upcall
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name> #signature {
                            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                            let this = &*((*this).this as *const Identity);
                            #upcall
                        }
                    }
                }
                }
                _ => quote! {},
            }).collect();

            let mut names = MethodNames::new();

            let trait_methods: Vec<_> = self
                .methods
                .iter()
                .map(|method| match method {
                    CppMethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        let signature = method.write_impl_signature(writer, true);
                        quote! { fn #name #signature; }
                    }
                    _ => quote! {},
                })
                .collect();

            let impl_base = self
                .base_interfaces
                .last()
                .map(|ty| ty.write_impl_name(writer));

            let field_base = self.base_interfaces.last().map(|ty|{
                match ty {
                    Type::IUnknown => quote! { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), },
                    Type::Object => quote! { base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(), },
                    Type::Item(Item::CppInterface(item)) => {
                        let ty = item.write_vtbl_name(writer);
                        if self.has_unknown_base () {
                            quote! { base__: #ty::new::<Identity, OFFSET>(), }
                        } else {
                            quote! { base__: #ty::new::<Identity>(), }
                        }
                    }
                    rest => unimplemented!("{rest:?}"),
                }
            });

            result.combine( if self.has_unknown_base() {
                quote! {
                    #cfg
                    pub trait #impl_name: #impl_base {
                        #(#trait_methods)*
                    }
                    #cfg
                    impl #vtbl_name {
                        pub const fn new<Identity: #impl_name, const OFFSET: isize>() -> Self {
                            #(#impl_methods)*
                            Self {
                                #field_base
                                #(#field_methods)*
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<#name as windows_core::Interface>::IID
                        }
                    }
                    #cfg
                    impl windows_core::RuntimeName for #name {}
                } 
            } else {
                let implvtbl_ident = impl_name.join("Vtbl");

                quote! {
                    #cfg
                    pub trait #impl_name : #impl_base {
                        #(#trait_methods)*
                    }
                    #cfg
                    impl #vtbl_name {
                        pub const fn new<Identity: #impl_name>() -> Self {
                            #(#impl_methods)*
                            Self{
                                #field_base
                                #(#field_methods)*
                            }
                        }
                    }
                    #[cfg(feature = "std")]
                    #cfg
                    struct #implvtbl_ident<T: #impl_name> (core::marker::PhantomData<T>);
                    #[cfg(feature = "std")]
                    #cfg
                    impl<T: #impl_name> #implvtbl_ident<T> {
                        const VTABLE: #vtbl_name = #vtbl_name::new::<T>();
                    }
                    #[cfg(feature = "std")]
                    #cfg
                    impl #name {
                        pub fn new<'a, T: #impl_name>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
                            let this = windows_core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                            let this = core::mem::ManuallyDrop::new(Box::new(this));
                            unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
                        }
                    }
                }
            });

            if self.def.is_agile() {
                result.combine(quote! {
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
                });
            }

            result
        }
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let namespace = writer.write_namespace(self.def.namespace());
        quote! { #namespace #name }
    }

    fn write_vtbl_name(&self, writer: &Writer) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).into();
        let namespace = writer.write_namespace(self.def.namespace());
        quote! { #namespace #name }
    }

    pub fn write_impl_name(&self, writer: &Writer) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", self.def.name()).into();
        let namespace = writer.write_namespace(self.def.namespace());
        quote! { #namespace #name }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            let base_interfaces = self.base_interfaces();

            if matches!(base_interfaces.first(), Some(Type::IUnknown)) {
                Type::IUnknown.dependencies(dependencies);
                Type::GUID.dependencies(dependencies);
            }

            for interface in &base_interfaces {
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
