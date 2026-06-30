use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CppInterface {
    pub def: TypeDef,
}

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
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn get_methods(&self, config: &Config) -> Vec<MethodOrName<CppMethod>> {
        let namespace = self.def.namespace();
        let type_name = self.def.type_name();

        self.def
            .methods()
            .map(|def| {
                let method = CppMethod::new(def, namespace, config.reader);
                if method_is_skipped(def, type_name, &method.dependencies, config) {
                    MethodOrName::Name(method.def)
                } else {
                    MethodOrName::Method(method)
                }
            })
            .collect()
    }

    // Returns `true` if any of this interface's own methods would be skipped due to
    // missing dependencies. Used (transitively across base interfaces) to decide
    // whether to emit the `_Impl` trait, since a derived `_Impl` cannot reference a
    // base `_Impl` that wasn't emitted.
    pub fn has_skipped_methods(&self, config: &Config) -> bool {
        let namespace = self.def.namespace();
        let type_name = self.def.type_name();
        self.def.methods().any(|def| {
            let method = CppMethod::new(def, namespace, config.reader);
            method_is_skipped(def, type_name, &method.dependencies, config)
        })
    }

    fn write_cfg(&self, config: &Config) -> (Cfg, TokenStream) {
        write_full_cfg(self, config)
    }

    // A "delegate-shaped" handler interface: `IUnknown`-derived with exactly one
    // method, named `Invoke`, that maps to `ReturnHint::ResultVoid`
    // (`HRESULT Invoke(..)` with no out-param). COM event/completion handlers
    // have this shape, and bindgen can give them the same closure-accepting
    // constructor it generates for WinRT delegates. Gated on `--minimal` so the
    // published `windows`/`windows-sys` crates are unaffected and the closure
    // matches the WinRT minimal soundness model (`Fn + 'static`, returns `S_OK`).
    fn delegate_method(&self, config: &Config) -> Option<CppMethod> {
        if !config.bindgen.style.is_minimal() {
            return None;
        }

        if self.base_interfaces(config.reader) != [Type::IUnknown] {
            return None;
        }

        let mut methods = self.def.methods();
        let only = methods.next()?;
        if methods.next().is_some() || only.name() != "Invoke" {
            return None;
        }

        let method = CppMethod::new(only, self.def.namespace(), config.reader);

        if method.return_hint != ReturnHint::ResultVoid {
            return None;
        }

        if method_is_skipped(only, self.def.type_name(), &method.dependencies, config) {
            return None;
        }

        Some(method)
    }

    // Generates a closure-accepting constructor for a delegate-shaped handler
    // interface, mirroring `Delegate::write`'s minimal-mode path but reusing the
    // COM `_Vtbl` already emitted by `write()`. Reuses `windows_core::imp::DelegateBox`
    // (its `QueryInterface` claims `IAgileObject`/`IMarshal` exactly as
    // `implement_decl!` does, so this is a faithful drop-in for hand-written
    // handler adapters). Lets consumers write `IXHandler::new(|sender, args| ..)`
    // instead of an `implement`-based adapter struct.
    fn write_closure_ctor(&self, config: &Config, method: &CppMethod) -> TokenStream {
        let name = to_ident(self.def.name());
        let vtbl_name = self.write_vtbl_name(config);
        let boxed: TokenStream = format!("{}Box", self.def.name()).parse().unwrap();
        let (_, cfg) = self.write_cfg(config);
        let vis = config.item_vis();

        let fn_signature = method.write_closure_fn_signature(config);
        let invoke_vtbl = method.write_abi(config, true);
        let invoke_upcall = method.write_closure_upcall(config);

        quote! {
            #cfg
            impl #name {
                #vis fn new<F: Fn #fn_signature + 'static>(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(&#boxed::<F>::VTABLE, invoke);
                    unsafe {
                        core::mem::transmute(windows_core::imp::box_new(com))
                    }
                }
            }
            #cfg
            struct #boxed<F: Fn #fn_signature + 'static>(core::marker::PhantomData<fn() -> F>);
            #cfg
            impl<F: Fn #fn_signature + 'static> #boxed<F> {
                const VTABLE: #vtbl_name = #vtbl_name {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: windows_core::imp::DelegateBox::<#name, F>::QueryInterface,
                        AddRef: windows_core::imp::DelegateBox::<#name, F>::AddRef,
                        Release: windows_core::imp::DelegateBox::<#name, F>::Release,
                    },
                    Invoke: Self::Invoke,
                };
                unsafe extern "system" fn Invoke #invoke_vtbl {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox::<#name, F>);
                        #invoke_upcall
                    }
                }
            }
        }
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let methods = self.get_methods(config);

        let base_interfaces = self.base_interfaces(config.reader);
        let has_unknown_base = matches!(base_interfaces.first(), Some(Type::IUnknown));
        let (class_cfg, cfg) = self.write_cfg(config);
        let vtbl_name = self.write_vtbl_name(config);

        let vtbl = {
            let core = config.write_core();

            let base = match base_interfaces.last() {
                Some(Type::IUnknown) => {
                    quote! { pub base__: #core IUnknown_Vtbl, }
                }
                Some(Type::Object) => {
                    quote! { pub base__: #core IInspectable_Vtbl, }
                }
                Some(Type::CppInterface(ty)) => {
                    let name = ty.write_vtbl_name(config);
                    quote! { pub base__: #name, }
                }
                _ => quote! {},
            };

            let mut names = MethodNames::for_style(&config.bindgen.style);

            let methods = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let method_cfg = class_cfg.difference(&method.dependencies, config);
                    let yes = method_cfg.write(config, false);

                    let name = names.add(method.def);
                    let abi = method.write_abi(config, false);

                    if yes.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn #abi,
                        }
                    } else {
                        let no = method_cfg.write(config, true);

                        quote! {
                            #yes
                            pub #name: unsafe extern "system" fn #abi,
                            #no
                            #name: usize,
                        }
                    }
                }
                MethodOrName::Name(method) => {
                    let name = names.add(*method);
                    quote! { #name: usize, }
                }
            });

            let hide_vtbl = config.doc_hidden_in_package();

            quote! {
                #cfg
                #[repr(C)]
                #hide_vtbl
                pub struct #vtbl_name {
                    #base
                    #(#methods)*
                }
            }
        };

        if config.bindgen.style.is_sys() {
            let mut result = quote! {};

            if !config.bindgen.layout.is_package() {
                if has_unknown_base {
                    if let Some(guid) = self.def.guid_attribute() {
                        let name: TokenStream = format!("IID_{}", self.def.name()).parse().unwrap();
                        result.combine(config.write_cpp_const_guid(name, &guid));
                    }
                }

                result.combine(vtbl);
            }

            result
        } else {
            let name = to_ident(self.def.name());

            // Delegate-shaped handler interfaces get a closure constructor
            // (`new`) instead of the `_Impl` producer trait — the closure box
            // is the better producer. They must still be listed in `--implement`
            // so the `Invoke` method survives dead-code elimination.
            let delegate_method = self.delegate_method(config);

            let mut result = if has_unknown_base {
                if let Some(guid) = self.def.guid_attribute() {
                    let guid = config.write_guid_u128(&guid);

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

            if let Some(Type::CppInterface(base)) = base_interfaces.last() {
                let base = base.write_name(config);

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

            if !base_interfaces.is_empty() {
                let bases = base_interfaces.iter().map(|ty| ty.write_name(config));
                result.combine(quote! {
                    #cfg
                    windows_core::imp::interface_hierarchy!(#name, #(#bases),*);
                });
            }

            // In minimal mode, interfaces listed in `--implement` are meant to be
            // implemented via their `_Impl` trait rather than called, so suppress the
            // caller-side method wrappers to avoid emitting dead code. This mirrors the
            // WinRT interface path (see `interface.rs`).
            let suppress_methods = config.bindgen.style.is_minimal()
                && config.should_implement(self.def.type_name(), false);

            let mut methods_tokens = quote! {};

            if !suppress_methods {
                let method_names = &mut MethodNames::for_style(&config.bindgen.style);
                let virtual_names = &mut MethodNames::for_style(&config.bindgen.style);

                // These methods live in `impl #name`, so references to this
                // interface are emitted as `Self` (clippy::use_self).
                let self_config = config.with_self_ty(self.type_name(), &[]);

                for method in methods.iter().filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                }) {
                    let cfg = method.write_cfg(config, &class_cfg, false);
                    let method = method.write(&self_config, method_names, virtual_names);

                    methods_tokens.combine(quote! {
                        #cfg
                        #method
                    });
                }
            }

            if !methods_tokens.is_empty() {
                result.combine(quote! {
                    #cfg
                    impl #name {
                        #methods_tokens
                    }
                });
            }

            result.combine(vtbl);

            if self.def.is_agile() {
                result.combine(quote! {
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
                });
            }

            if config.should_implement(self.def.type_name(), true) && delegate_method.is_none() {
                let impl_name: TokenStream = format!("{}_Impl", self.def.name()).parse().unwrap();

                let cfg = if config.bindgen.layout.is_package() {
                    fn combine(
                        interface: &CppInterface,
                        dependencies: &mut TypeMap,
                        config: &Config,
                    ) {
                        for method in &interface.get_methods(config) {
                            if let MethodOrName::Method(method) = method {
                                dependencies.combine(&method.dependencies);
                            }
                        }
                    }

                    let mut dependencies = self.dependencies(config.reader);
                    combine(self, &mut dependencies, config);

                    base_interfaces.iter().for_each(|interface| {
                        if let Type::CppInterface(ty) = interface {
                            combine(ty, &mut dependencies, config);
                        }
                    });

                    Cfg::new(&dependencies, config).write(config, false)
                } else {
                    quote! {}
                };

                let mut names = MethodNames::for_style(&config.bindgen.style);

                let field_methods: Vec<_> = methods
                    .iter()
                    .map(|method| match method {
                        MethodOrName::Method(method) => {
                            let name = names.add(method.def);
                            if has_unknown_base {
                                quote! { #name: #name::<Identity, OFFSET>, }
                            } else {
                                quote! { #name: #name::<Identity>, }
                            }
                        }
                        MethodOrName::Name(method) => {
                            let name = names.add(*method);
                            quote! { #name: 0, }
                        }
                    })
                    .collect();

                let mut names = MethodNames::for_style(&config.bindgen.style);

                let impl_methods: Vec<_> = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(config, true);
                    let upcall = method.write_upcall(&impl_name, &name, config.reader);

                    if has_unknown_base {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name, const OFFSET: isize> #signature {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                #upcall
                            }
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name> #signature {
                            unsafe {
                                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                                let this = &*((*this).this as *const Identity);
                                #upcall
                            }
                        }
                    }
                }
                }
                _ => quote! {},
            }).collect();

                let mut names = MethodNames::for_style(&config.bindgen.style);

                let trait_methods: Vec<_> = methods
                    .iter()
                    .map(|method| match method {
                        MethodOrName::Method(method) => {
                            let name = names.add(method.def);
                            let signature = method.write_impl_signature(config, true);
                            quote! { fn #name #signature; }
                        }
                        _ => quote! {},
                    })
                    .collect();

                let impl_base = base_interfaces.last().map(|ty| ty.write_impl_name(config));

                let field_base = base_interfaces.last().map(|ty|{
                match ty {
                    Type::IUnknown => quote! { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), },
                    Type::Object => quote! { base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(), },
                    Type::CppInterface(ty) => {
                        let ty = ty.write_vtbl_name(config);
                        if has_unknown_base {
                            quote! { base__: #ty::new::<Identity, OFFSET>(), }
                        } else {
                            quote! { base__: #ty::new::<Identity>(), }
                        }
                    }
                    rest => panic!("{rest:?}"),
                }
            });

                // If any methods were skipped due to missing dependencies, omit the
                // `_Impl` trait rather than emitting a partial vtable. Propagate the
                // omission across base interfaces.
                let has_skipped_methods = methods
                    .iter()
                    .any(|method| matches!(method, MethodOrName::Name(_)))
                    || base_interfaces.iter().any(|ty| match ty {
                        Type::CppInterface(ty) => ty.has_skipped_methods(config),
                        _ => false,
                    });

                if has_skipped_methods {
                    if has_unknown_base {
                        result.combine(quote! {
                            #cfg
                            impl windows_core::RuntimeName for #name {}
                        });
                    }
                } else {
                    result.combine( if has_unknown_base {
                let matches = base_interfaces.iter().filter_map(|ty|{
                    match ty {
                        Type::CppInterface(ty) => {
                            let name = ty.write_name(config);
                            Some(quote! { || iid == &<#name as windows_core::Interface>::IID })
                        }
                        _ => None,
                    }
                });

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
                            iid == &<#name as windows_core::Interface>::IID #(#matches)*
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
                    #cfg
                    struct #implvtbl_ident<T: #impl_name> (core::marker::PhantomData<T>);
                    #cfg
                    impl<T: #impl_name> #implvtbl_ident<T> {
                        const VTABLE: #vtbl_name = #vtbl_name::new::<T>();
                    }
                    #cfg
                    impl #name {
                        pub fn new<'a, T: #impl_name>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
                            let this = windows_core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                            let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
                            unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
                        }
                    }
                }
            });
                }
            }

            if let Some(method) = &delegate_method {
                result.combine(self.write_closure_ctor(config, method));
            }

            result
        }
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    fn write_vtbl_name(&self, config: &Config) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).parse().unwrap();
        let namespace = config.write_namespace(self.def.type_name());
        quote! { #namespace #name }
    }

    pub fn write_impl_name(&self, config: &Config) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", self.def.name()).parse().unwrap();
        let namespace = config.write_namespace(self.def.type_name());
        quote! { #namespace #name }
    }

    pub fn base_interfaces(&self, reader: &Reader) -> Vec<Type> {
        let mut bases = vec![];
        let mut def = self.def;

        while let Some(base) = def
            .interface_impls()
            .map(move |imp| imp.ty(&[], reader))
            .next()
        {
            match base {
                Type::CppInterface(ref ty) => {
                    def = ty.def;
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
                rest => panic!("{rest:?}"),
            }
        }

        bases
    }
}

impl Dependencies for CppInterface {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        let base_interfaces = self.base_interfaces(reader);

        for interface in &base_interfaces {
            interface.combine(dependencies, reader);
        }

        for method in self.def.methods() {
            for ty in method
                .method_signature(self.def.namespace(), &[], reader)
                .types()
            {
                if ty.is_core() {
                    ty.combine(dependencies, reader);
                }
            }
        }
    }
}
