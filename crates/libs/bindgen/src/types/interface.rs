use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceKind {
    None,
    Default,
    Static,
    Composable,
    Base,
}

#[derive(Clone, Debug)]
pub enum MethodOrName {
    /// Method is fully described and emitted into the public surface.
    Method(Method),
    /// Method was demoted to an opaque `Slot: usize` because one or more
    /// dependent types are not part of the binding set. The `_Impl` trait
    /// and `Vtbl::new` are not emitted because we cannot describe the slot.
    Name(MethodDef),
    /// Method was explicitly excluded via the user's `--filter` rules. The
    /// slot is omitted from the callable surface, but the original ABI
    /// signature is still known (all dependencies remain in scope), so we
    /// emit a real vtable slot and a synthesised stub thunk that returns
    /// `E_NOTIMPL`. This keeps `_Impl` and `Vtbl::new` available so the
    /// interface can still be implemented from Rust.
    Filtered(Method),
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub def: TypeDef,
    pub generics: Vec<Type>,
    pub kind: InterfaceKind,
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def
    }
}

impl Eq for Interface {}

impl std::hash::Hash for Interface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.def.hash(state);
    }
}

impl Ord for Interface {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for Interface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Interface {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn get_methods(&self, config: &Config) -> Vec<MethodOrName> {
        let type_name = self.def.type_name();
        self.def
            .methods()
            .map(|def| {
                let method = Method::new(def, &self.generics, config.reader);
                if !config.filter.includes_method(type_name, def.name()) {
                    // Method-level `--filter` excluded this slot. The slot is
                    // omitted from the callable surface; `Vtbl::new` installs
                    // an `E_NOTIMPL` stub thunk in its place. If the method's
                    // dependent types are not in the binding set we still
                    // classify it as `Filtered`, but the emitter falls back
                    // to an opaque `usize` slot (no stub thunk) so the
                    // interface's `_Impl` trait can still be emitted.
                    MethodOrName::Filtered(method)
                } else if !method.dependencies.included(config) {
                    config
                        .warnings
                        .skip_method(method.def, &method.dependencies, config);
                    MethodOrName::Name(method.def)
                } else {
                    MethodOrName::Method(method)
                }
            })
            .collect()
    }

    // Returns `true` if any of this interface's own methods would be skipped due to
    // missing dependencies. Used (transitively across required interfaces) to decide
    // whether to emit the `_Impl` trait, since a derived `_Impl` cannot reference a
    // base `_Impl` that wasn't emitted. Method-level `--filter` exclusions do NOT
    // poison `_Impl` emission: filtered methods are stubbed with `E_NOTIMPL` thunks
    // in the generated `Vtbl::new` instead (or, if their dependencies are also out
    // of scope, demoted to an opaque `usize` slot with no stub thunk).
    pub fn has_skipped_methods(&self, config: &Config) -> bool {
        let type_name = self.def.type_name();
        self.def.methods().any(|def| {
            if !config.filter.includes_method(type_name, def.name()) {
                return false;
            }
            let method = Method::new(def, &self.generics, config.reader);
            !method.dependencies.included(config)
        })
    }

    fn write_cfg(&self, config: &Config) -> (Cfg, TokenStream) {
        write_full_cfg(self, config)
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let type_name = self.def.type_name();
        let methods = self.get_methods(config);

        let required_interfaces = self.required_interfaces(config.reader);
        let name = self.write_name(config);

        let vtbl_name = self.write_vtbl_name(config);
        let is_exclusive = self.is_exclusive();
        let constraints = config.write_generic_constraints(&self.generics);
        let phantoms = config.write_generic_phantoms(&self.generics);
        let named_phantoms = config.write_generic_named_phantoms(&self.generics);
        let (class_cfg, cfg) = self.write_cfg(config);

        let vtbl = {
            let virtual_names = &mut MethodNames::new();
            let result = config.write_result();

            let vtbl_methods = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = virtual_names.add(method.def);
                    let vtbl = method.write_abi(config, false);

                    let method_cfg = class_cfg.difference(&method.dependencies, config);
                    let yes = method_cfg.write(config, false);

                    if yes.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn(#vtbl) -> #result HRESULT,
                        }
                    } else {
                        let no = method_cfg.write(config, true);

                        quote! {
                            #yes
                            pub #name: unsafe extern "system" fn(#vtbl) -> #result HRESULT,
                            #no
                            #name: usize,
                        }
                    }
                }
                MethodOrName::Filtered(method) => {
                    // Filtered methods get a real (private) ABI slot. There is no
                    // public Rust caller, but the slot is targeted by the stub
                    // thunk that `Vtbl::new` installs so the layout is preserved
                    // and `E_NOTIMPL` is returned at runtime. If dependent types
                    // are not in the binding set, fall back to an opaque `usize`
                    // slot — `Vtbl::new` will initialize it to `0`.
                    let name = virtual_names.add(method.def);
                    if method.dependencies.included(config) {
                        let vtbl = method.write_abi(config, false);
                        quote! {
                            #name: unsafe extern "system" fn(#vtbl) -> #result HRESULT,
                        }
                    } else {
                        quote! { #name: usize, }
                    }
                }
                MethodOrName::Name(method) => {
                    let name = virtual_names.add(*method);
                    quote! { #name: usize, }
                }
            });

            let hide_vtbl = if config.sys {
                quote! {}
            } else {
                quote! { #[doc(hidden)] }
            };

            let core = config.write_core();

            quote! {
                #cfg
                #[repr(C)]
                #hide_vtbl
                pub struct #vtbl_name where #constraints {
                    pub base__: #core IInspectable_Vtbl,
                    #(#vtbl_methods)*
                    #named_phantoms
                }
            }
        };

        if config.sys {
            let mut result = quote! {};

            if !config.package {
                if let Some(guid) = self.def.guid_attribute() {
                    let name: TokenStream = format!("IID_{}", trim_tick(self.def.name())).into();
                    result.combine(config.write_cpp_const_guid(name, &guid));
                }

                result.combine(vtbl);
            }

            result
        } else {
            let mut result = if self.generics.is_empty() {
                let guid = config.write_guid_u128(&self.def.guid_attribute().unwrap());

                quote! {
                    #cfg
                    windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                    #cfg
                    impl windows_core::RuntimeType for #name {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                }
            } else {
                let guid = self.def.guid_attribute().unwrap();
                let pinterface = Literal::byte_string(&format!("pinterface({{{guid}}}"));

                let generics = self.generics.iter().map(|generic| {
                    let name = generic.write_name(config);

                    quote! {
                        .push_slice(b";").push_other(#name::SIGNATURE)
                    }
                });

                quote! {
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                    impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                    impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                    unsafe impl<#constraints> windows_core::Interface for #name {
                        type Vtable = #vtbl_name;
                        const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
                    }
                    impl<#constraints> windows_core::RuntimeType for #name {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#pinterface)#(#generics)*.push_slice(b")");
                    }
                }
            };

            if !is_exclusive && self.generics.is_empty() {
                result.combine(quote! {
                #cfg
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
            });
            }

            if !is_exclusive && !required_interfaces.is_empty() {
                if self.generics.is_empty() {
                    let interfaces = required_interfaces.iter().map(|ty| ty.write_name(config));

                    result.combine(quote! {
                        #cfg
                        windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                    });
                } else {
                    let interfaces = required_interfaces.iter().map(|ty| {
                    let ty = ty.write_name(config);
                    quote!{
                        impl<#constraints> windows_core::imp::CanInto<#ty> for #name { const QUERY: bool = true; }
                    }
                });

                    result.combine(quote! {
                        #(#interfaces)*
                    });
                }
            }

            // Even in `minimal` mode, exclusive instance interfaces still need their own-vtable
            // method block; otherwise WinRT class default interfaces would lose their callable
            // wrappers entirely. Exclusive factory interfaces (those referenced from the class
            // via Activatable/Static/Composable) are already exposed through the class, so we
            // can suppress their methods here to keep call sites concise.
            let is_factory = is_exclusive && config.minimal && self.is_factory(config.reader);
            if !is_exclusive || (config.minimal && !is_factory) {
                let method_names = &mut MethodNames::new();
                let virtual_names = &mut MethodNames::new();
                let mut method_tokens = TokenStream::new();

                for method in methods.iter().filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                }) {
                    let cfg = method.write_cfg(config, &class_cfg, false);

                    let method = method.write(
                        config,
                        Some(self),
                        InterfaceKind::Default,
                        method_names,
                        virtual_names,
                    );

                    method_tokens.combine(quote! {
                        #cfg
                        #method
                    });
                }

                for interface in &required_interfaces {
                    // In `minimal` mode callers `cast` to the owning interface explicitly.
                    if config.minimal {
                        continue;
                    }
                    let virtual_names = &mut MethodNames::new();

                    for method in
                        interface
                            .get_methods(config)
                            .iter()
                            .filter_map(|method| match &method {
                                MethodOrName::Method(method) => Some(method),
                                _ => None,
                            })
                    {
                        let cfg = method.write_cfg(config, &class_cfg, false);

                        let method = method.write(
                            config,
                            Some(interface),
                            interface.kind,
                            method_names,
                            virtual_names,
                        );

                        method_tokens.combine(quote! {
                            #cfg
                            #method
                        });
                    }
                }

                if !method_tokens.is_empty() {
                    result.combine(quote! {
                        #cfg
                        impl<#constraints> #name {
                            #method_tokens
                        }
                    });
                }

                if self.def.is_agile() {
                    result.combine(quote! {
                        #cfg
                        unsafe impl<#constraints> Send for #name {}
                        #cfg
                        unsafe impl<#constraints> Sync for #name {}
                    });
                }

                let into_iterator = if config.minimal {
                    None
                } else {
                    required_interfaces
                        .iter()
                        .find(|interface| interface.type_name() == TypeName::IIterable)
                        .map(|interface| {
                            let ty = interface.generics[0].write_name(config);
                            let namespace = config.write_namespace(TypeName::IIterator);

                            quote! {
                                #cfg
                                impl<#constraints> IntoIterator for #name {
                                    type Item = #ty;
                                    type IntoIter = #namespace IIterator<Self::Item>;

                                    fn into_iter(self) -> Self::IntoIter {
                                        IntoIterator::into_iter(&self)
                                    }
                                }
                                #cfg
                                impl<#constraints> IntoIterator for &#name {
                                    type Item = #ty;
                                    type IntoIter = #namespace IIterator<Self::Item>;

                                    fn into_iter(self) -> Self::IntoIter {
                                        self.First().unwrap()
                                    }
                                }

                            }
                        })
                };

                if let Some(into_iterator) = into_iterator {
                    result.combine(into_iterator);
                }
            }

            if config.should_implement(type_name, !is_exclusive) {
                let impl_name: TokenStream = format!("{}_Impl", trim_tick(self.def.name())).into();

                let generics: Vec<_> = self
                    .generics
                    .iter()
                    .map(|ty| ty.write_name(config))
                    .collect();

                let runtime_name = format!("{type_name}");

                let cfg = if config.package {
                    fn combine(interface: &Interface, dependencies: &mut TypeMap, config: &Config) {
                        for method in interface.get_methods(config).iter() {
                            if let MethodOrName::Method(method) = method {
                                dependencies.combine(&method.dependencies);
                            }
                        }
                    }

                    let mut dependencies = self.dependencies(config.reader);
                    combine(self, &mut dependencies, config);

                    required_interfaces
                        .iter()
                        .for_each(|interface| combine(interface, &mut dependencies, config));

                    Cfg::new(&dependencies, config).write(config, false)
                } else {
                    quote! {}
                };

                result.combine(quote! {
                    #cfg
                    impl<#constraints> windows_core::RuntimeName for #name {
                        const NAME: &'static str = #runtime_name;
                    }
                });

                // If any methods were skipped due to missing dependencies, the interface cannot be
                // fully described, so omit the ability to implement it rather than emitting a
                // partial vtable with null function pointer slots. Also propagate the omission
                // when any required (base) interface had its `_Impl` trait omitted, since a
                // derived `_Impl` cannot reference a base `_Impl` that wasn't emitted.
                //
                // Method-level `--filter` exclusions do NOT poison `_Impl` emission: the
                // corresponding slots get synthesised `E_NOTIMPL` stub thunks below so the
                // vtable layout stays correct without forcing the user to implement them.
                let has_skipped_methods = methods
                    .iter()
                    .any(|method| matches!(method, MethodOrName::Name(_)))
                    || required_interfaces
                        .iter()
                        .any(|ty| ty.has_skipped_methods(config));

                if has_skipped_methods {
                    config.warnings.skip_implement(self.def);
                } else {
                    let mut names = MethodNames::new();

                    let field_methods: Vec<_> = methods
                        .iter()
                        .map(|method| match method {
                            MethodOrName::Method(method) => {
                                let name = names.add(method.def);
                                quote! { #name: #name::<#(#generics,)* Identity, OFFSET>, }
                            }
                            MethodOrName::Filtered(method) => {
                                let name = names.add(method.def);
                                if method.dependencies.included(config) {
                                    quote! { #name: #name::<#(#generics,)* Identity, OFFSET>, }
                                } else {
                                    quote! { #name: 0, }
                                }
                            }
                            MethodOrName::Name(method) => {
                                let name = names.add(*method);
                                quote! { #name: 0, }
                            }
                        })
                        .collect();

                    let mut names = MethodNames::new();

                    let impl_methods: Vec<_> = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(config, true);
                    let call = quote! { #impl_name::#name };
                    let upcall = method.write_upcall(call, true, config);

                    quote! {
                        unsafe extern "system" fn #name<#constraints Identity: #impl_name <#(#generics,)*>, const OFFSET: isize> (#signature) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                #upcall
                            }
                        }
                    }
                }
                MethodOrName::Filtered(method) => {
                    // Synthesised stub for a `--filter`-excluded slot. The ABI
                    // signature matches the original method (all dependencies
                    // are in scope), but the body simply returns `E_NOTIMPL`
                    // without invoking any user code. Parameters are unused.
                    // If dependencies are missing, no thunk is emitted; the
                    // slot is left as an opaque `0`.
                    let name = names.add(method.def);
                    if method.dependencies.included(config) {
                        let signature = method.write_abi(config, true);
                        quote! {
                            #[allow(unused_variables)]
                            unsafe extern "system" fn #name<#constraints Identity: #impl_name <#(#generics,)*>, const OFFSET: isize> (#signature) -> windows_core::HRESULT {
                                windows_core::HRESULT(0x80004001_u32 as i32)
                            }
                        }
                    } else {
                        quote! {}
                    }
                }
                _ => quote! {},
            }).collect();

                    let mut names = MethodNames::new();

                    let trait_methods: Vec<_> = methods
                        .iter()
                        .map(|method| match method {
                            MethodOrName::Method(method) => {
                                let name = names.add(method.def);
                                let signature = method.write_impl_signature(config, true, true);
                                quote! { fn #name #signature; }
                            }
                            MethodOrName::Filtered(method) => {
                                // Reserve the name so subsequent overload numbering matches
                                // the vtable order, but emit nothing in the trait.
                                let _ = names.add(method.def);
                                quote! {}
                            }
                            _ => quote! {},
                        })
                        .collect();

                    let requires = if required_interfaces.is_empty() {
                        quote! { windows_core::IUnknownImpl }
                    } else {
                        let interfaces = required_interfaces
                            .iter()
                            .map(|ty| ty.write_impl_name(config));

                        quote! {  #(#interfaces)+* }
                    };

                    result.combine(quote! {
                #cfg
                pub trait #impl_name <#(#generics),*> : #requires where #constraints {
                    #(#trait_methods)*
                }
                #cfg
                impl<#constraints> #vtbl_name {
                    pub const fn new<Identity: #impl_name <#(#generics,)*>, const OFFSET: isize>() -> Self {
                        #(#impl_methods)*
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(),
                            #(#field_methods)*
                            #named_phantoms
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<#name as windows_core::Interface>::IID
                    }
                }
            });
                }
            }

            result.combine(vtbl);
            result.combine(self.write_extensions());
            result
        }
    }

    fn write_extensions(&self) -> TokenStream {
        match self.type_name() {
            TypeName::IIterator => {
                quote! {
                    impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
                        type Item = T;

                        fn next(&mut self) -> Option<Self::Item> {
                            let result = if self.HasCurrent().unwrap_or(false) {
                                self.Current().ok()
                            } else {
                                None
                            };

                            if result.is_some() {
                                // Ignore MoveNext errors; treat as end-of-stream on the next
                                // iteration but still yield the value already fetched.
                                let _ = self.MoveNext();
                            }

                            result
                        }
                    }
                }
            }
            TypeName::IIterable => {
                quote! {
                    impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
                        type Item = T;
                        type IntoIter = IIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            IntoIterator::into_iter(&self)
                        }
                    }
                    impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
                        type Item = T;
                        type IntoIter = IIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            self.First().unwrap()
                        }
                    }

                }
            }
            _ => quote! {},
        }
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &self.generics)
    }

    fn write_vtbl_name(&self, config: &Config) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", trim_tick(self.def.name())).into();

        if self.generics.is_empty() {
            name
        } else {
            let generics = self.generics.iter().map(|ty| ty.write_name(config));
            quote! { #name < #(#generics,)* > }
        }
    }

    pub fn write_impl_name(&self, config: &Config) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", trim_tick(self.def.name())).into();
        let namespace = config.write_namespace(self.def.type_name());

        if self.generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = self.generics.iter().map(|ty| ty.write_name(config));
            quote! { #namespace #name < #(#generics),* > }
        }
    }

    pub fn is_exclusive(&self) -> bool {
        self.def.has_attribute("ExclusiveToAttribute")
    }

    // An exclusive interface is a "factory" interface when its owning class references it via
    // `Activatable`/`Static`/`Composable` rather than implementing it. Methods on such interfaces
    // are reachable through the class, so in `minimal` mode we can avoid emitting them on the
    // interface itself to keep call sites concise.
    pub fn is_factory(&self, reader: &Reader) -> bool {
        let Some(attribute) = self.def.find_attribute("ExclusiveToAttribute") else {
            return false;
        };
        let value = attribute.value();
        let Some((_, Value::TypeName(class_tn))) = value.first() else {
            return false;
        };
        let Some(Type::Class(class)) = reader
            .with_full_name(class_tn.namespace.as_str(), class_tn.name.as_str())
            .next()
        else {
            return false;
        };

        let our_namespace = self.def.namespace();
        let our_name = self.def.name();

        for attribute in class.def.attributes() {
            match attribute.name() {
                "StaticAttribute" | "ActivatableAttribute" | "ComposableAttribute" => {
                    for (_, arg) in attribute.value() {
                        if let Value::TypeName(tn) = arg {
                            if tn.namespace.as_str() == our_namespace
                                && tn.name.as_str() == our_name
                            {
                                return true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn runtime_signature(&self, reader: &Reader) -> String {
        interface_signature(self.def, &self.generics, reader)
    }

    pub fn required_interfaces(&self, reader: &Reader) -> Vec<Self> {
        fn walk(interface: &Interface, set: &mut Vec<Interface>, reader: &Reader) {
            for ty in interface
                .def
                .interface_impls()
                .map(|imp| imp.ty(&interface.generics, reader))
            {
                let Type::Interface(interface) = ty else {
                    panic!();
                };

                if !set.iter().any(|existing| existing.def == interface.def) {
                    walk(&interface, set, reader);
                    set.push(interface);
                }
            }
        }
        let mut set = vec![];
        walk(self, &mut set, reader);

        set.sort();
        set.dedup();
        set
    }
}

impl Dependencies for Interface {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        Type::Object.combine(dependencies, reader);

        for interface in self.required_interfaces(reader) {
            Type::Interface(interface).combine(dependencies, reader);
        }

        // Different specializations of Interface may have different generics...
        for ty in &self.generics {
            ty.combine(dependencies, reader);
        }

        let is_iterable = self.type_name() == TypeName::IIterable;

        for method in self.def.methods() {
            for ty in method
                .method_signature(self.def.namespace(), &self.generics, reader)
                .types()
            {
                if is_iterable || ty.is_core() {
                    ty.combine(dependencies, reader);
                }
            }
        }
    }
}
