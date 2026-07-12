use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceKind {
    None,
    Default,
    Static,
    Composable,
    Base,
}

// Shared by the WinRT (`Interface`/`Method`) and Win32/COM (`CppInterface`/`CppMethod`)
// generators: a vtable slot is either a fully-projected method or an opaque,
// name-only placeholder (filtered out or missing dependencies).
#[derive(Clone, Debug)]
pub enum MethodOrName<M> {
    Method(M),
    Name(MethodDef),
}

// A method collapses to an opaque, name-only vtable slot when it is demoted by a
// method-level filter, or (outside minimal mode) when its dependencies are not all
// included. Shared by both the WinRT and COM interface generators so the policy that
// decides `Method` vs `Name` lives in one place.
pub fn method_is_skipped(
    def: MethodDef,
    type_name: TypeName,
    dependencies: &TypeMap,
    config: &Config,
) -> bool {
    (!config.bindgen.style.is_minimal() && !dependencies.included(config))
        || !config.includes_method(type_name, def)
}

// The two pieces a vtable/`_Impl` writer needs from a projected method, regardless of
// whether it is a WinRT `Method` or a Win32/COM `CppMethod`. Lets the shared emitters
// below iterate `MethodOrName<M>` without caring which generator produced it.
pub trait MethodItem {
    fn def(&self) -> MethodDef;
    fn dependencies(&self) -> &TypeMap;
}

impl MethodItem for Method {
    fn def(&self) -> MethodDef {
        self.def
    }
    fn dependencies(&self) -> &TypeMap {
        &self.dependencies
    }
}

impl MethodItem for CppMethod {
    fn def(&self) -> MethodDef {
        self.def
    }
    fn dependencies(&self) -> &TypeMap {
        &self.dependencies
    }
}

// Emits the method-pointer fields of a `_Vtbl` struct. Shared by both interface
// generators: the slot-name allocation, the per-method `#[cfg]` gate, the
// `#[cfg]`-out `usize` fallback, and the opaque name-only slot are identical; only the
// post-`fn` signature differs (WinRT appends `-> HRESULT`, COM's `write_abi` already
// carries the native return), so the caller supplies it via `signature`.
pub fn write_vtbl_methods<M: MethodItem>(
    methods: &[MethodOrName<M>],
    class_cfg: &Cfg,
    config: &Config,
    mut signature: impl FnMut(&M) -> TokenStream,
) -> Vec<TokenStream> {
    let mut names = MethodNames::for_style(&config.bindgen.style);
    methods
        .iter()
        .map(|method| match method {
            MethodOrName::Method(method) => {
                let method_cfg = class_cfg.difference(method.dependencies(), config);
                let yes = method_cfg.write(config, false);
                let name = names.add(method.def());
                let sig = signature(method);

                if yes.is_empty() {
                    quote! {
                        pub #name: unsafe extern "system" fn #sig,
                    }
                } else {
                    let no = method_cfg.write(config, true);

                    quote! {
                        #yes
                        pub #name: unsafe extern "system" fn #sig,
                        #no
                        #name: usize,
                    }
                }
            }
            MethodOrName::Name(method) => {
                let name = names.add(*method);
                quote! { #name: usize, }
            }
        })
        .collect()
}

// Emits the `field_methods` of a `_Vtbl::new()` initializer (the `name: name::<..>,`
// entries). The opaque `name: 0,` fallback is identical across generators; the turbofish
// type arguments differ (WinRT carries generics, COM branches on `OFFSET`), so the caller
// supplies the `Method` arm via `method_field`.
pub fn write_impl_field_methods<M: MethodItem>(
    methods: &[MethodOrName<M>],
    config: &Config,
    mut method_field: impl FnMut(&TokenStream) -> TokenStream,
) -> Vec<TokenStream> {
    let mut names = MethodNames::for_style(&config.bindgen.style);
    methods
        .iter()
        .map(|method| match method {
            MethodOrName::Method(method) => {
                let name = names.add(method.def());
                method_field(&name)
            }
            MethodOrName::Name(method) => {
                let name = names.add(*method);
                quote! { #name: 0, }
            }
        })
        .collect()
}

// Emits the `fn name signature;` entries of an `_Impl` producer trait. Identical across
// generators apart from the `write_impl_signature` arity (WinRT takes an extra flag), so
// the caller supplies the signature via `signature`.
pub fn write_impl_trait_methods<M: MethodItem>(
    methods: &[MethodOrName<M>],
    config: &Config,
    mut signature: impl FnMut(&M) -> TokenStream,
) -> Vec<TokenStream> {
    let mut names = MethodNames::for_style(&config.bindgen.style);
    methods
        .iter()
        .map(|method| match method {
            MethodOrName::Method(method) => {
                let name = names.add(method.def());
                let sig = signature(method);
                quote! { fn #name #sig; }
            }
            _ => quote! {},
        })
        .collect()
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

    pub fn get_methods(&self, config: &Config) -> Vec<MethodOrName<Method>> {
        let type_name = self.def.type_name();
        self.def
            .methods()
            .map(|def| {
                let method = Method::new(def, &self.generics, config.reader);
                if method_is_skipped(def, type_name, &method.dependencies, config) {
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
    // base `_Impl` that wasn't emitted.
    pub fn has_skipped_methods(&self, config: &Config) -> bool {
        let type_name = self.def.type_name();
        self.def.methods().any(|def| {
            let method = Method::new(def, &self.generics, config.reader);
            method_is_skipped(def, type_name, &method.dependencies, config)
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
            let core = config.write_core();

            // Drop trailing usize slots — nothing indexes past the last real
            // method, so they waste space and compile time.
            let methods_for_vtbl: &[MethodOrName<Method>] = {
                let last_real = methods
                    .iter()
                    .rposition(|m| matches!(m, MethodOrName::Method(_)));
                match last_real {
                    Some(pos) => &methods[..=pos],
                    None => &[],
                }
            };

            let vtbl_methods = write_vtbl_methods(methods_for_vtbl, &class_cfg, config, |method| {
                let vtbl = method.write_abi(config, false);
                quote! { (#vtbl) -> #core HRESULT }
            });

            let hide_vtbl = config.doc_hidden_in_package();

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

        if config.bindgen.style.is_sys() {
            let mut result = quote! {};

            if !config.bindgen.layout.is_package() {
                if let Some(guid) = self.def.guid_attribute() {
                    let name: TokenStream = format!("IID_{}", trim_tick(self.def.name()))
                        .parse()
                        .unwrap();
                    result.combine(config.write_cpp_const_guid(name, &guid));
                }

                result.combine(vtbl);
            }

            result
        } else {
            let mut result = if self.generics.is_empty() {
                let guid = config.write_guid_u128(&self.def.guid_attribute().unwrap());

                // In minimal mode, NAME is only needed for interfaces that are
                // being implemented (for GetRuntimeClassName). The trait provides
                // an empty default, so omitting it is safe.
                let name_const = if config.emit_runtime_name(type_name) {
                    let type_name_bytes = Literal::byte_string(format!("{type_name}").as_bytes());
                    quote! {
                        const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#type_name_bytes);
                    }
                } else {
                    quote! {}
                };

                quote! {
                    #cfg
                    windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                    #cfg
                    impl windows_core::RuntimeType for #name {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                        #name_const
                    }
                }
            } else {
                let guid = self.def.guid_attribute().unwrap();
                let pinterface = Literal::byte_string(format!("pinterface({{{guid}}}").as_bytes());

                let generics = self.generics.iter().map(|generic| {
                    let name = generic.write_name(config);

                    quote! {
                        .push_slice(b";").push_other(#name::SIGNATURE)
                    }
                });

                // In minimal mode, NAME on parameterized interfaces is only needed
                // when the interface is implemented (for GetRuntimeClassName via
                // RUNTIME_CLASS_NAME). Skip it otherwise.
                let name_const = if config.emit_runtime_name(type_name) {
                    let arity = self.generics.len();
                    let name_prefix =
                        Literal::byte_string(format!("{type_name}`{arity}<").as_bytes());
                    let name_generics: Vec<_> = self
                        .generics
                        .iter()
                        .enumerate()
                        .map(|(i, generic)| {
                            let gen_name = generic.write_name(config);
                            if i == 0 {
                                quote! { .push_other(#gen_name::NAME) }
                            } else {
                                quote! { .push_slice(b", ").push_other(#gen_name::NAME) }
                            }
                        })
                        .collect();
                    quote! {
                        const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#name_prefix)#(#name_generics)*.push_slice(b">");
                    }
                } else {
                    quote! {}
                };

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
                        #name_const
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
                    let interfaces: Vec<_> = required_interfaces
                        .iter()
                        .filter(|ty| {
                            if config.bindgen.style.is_minimal() {
                                let tn = Type::Interface((*ty).clone()).type_name();
                                config.types.contains_key(&tn)
                            } else {
                                true
                            }
                        })
                        .map(|ty| ty.write_name(config))
                        .collect();

                    if !interfaces.is_empty() {
                        result.combine(quote! {
                            #cfg
                            windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                        });
                    }
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

            // Even in `minimal` mode (or when MinimalTypeMap is active), exclusive instance
            // interfaces still need their own-vtable method block; otherwise WinRT class
            // default interfaces would lose their callable wrappers entirely. Exclusive
            // factory interfaces (those referenced from the class via
            // Activatable/Static/Composable) are already exposed through the class, and
            // exclusive `--implement` interfaces (like overrides) are meant to be *implemented*
            // via the `_Impl` trait — not called. In both cases we suppress the caller-side
            // method wrapper to avoid dead code.
            let minimal = config.bindgen.style.is_minimal();
            let suppress_methods = is_exclusive
                && minimal
                && (self.is_factory(config.reader) || config.should_implement(type_name, false));
            if !is_exclusive || (minimal && !suppress_methods) {
                let method_names = &mut MethodNames::for_style(&config.bindgen.style);
                let virtual_names = &mut MethodNames::for_style(&config.bindgen.style);
                let mut method_tokens = TokenStream::new();
                // These methods live in `impl<..> #name`, so references to this
                // interface (with matching generics) are emitted as `Self`.
                let self_config = config.with_self_ty(self.type_name(), &self.generics);

                for method in methods.iter().filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                }) {
                    let cfg = method.write_cfg(config, &class_cfg, false);

                    let method = method.write(
                        &self_config,
                        Some(self),
                        InterfaceKind::Default,
                        method_names,
                        virtual_names,
                        true,
                    );

                    method_tokens.combine(quote! {
                        #cfg
                        #method
                    });
                }

                for interface in &required_interfaces {
                    // In `minimal` mode callers `cast` to the owning interface explicitly.
                    if !config.bindgen.style.emit_inherited_forwarders() {
                        continue;
                    }
                    let virtual_names = &mut MethodNames::for_style(&config.bindgen.style);

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
                            &self_config,
                            Some(interface),
                            interface.kind,
                            method_names,
                            virtual_names,
                            true,
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

                let into_iterator = if config.bindgen.style.emit_iterable_into_iterator() {
                    required_interfaces
                        .iter()
                        .find(|interface| interface.type_name() == TypeName::IIterable)
                        .map(|interface| {
                            let ty = interface.generics[0].write_name(config);

                            quote! {
                                #cfg
                                impl<#constraints> IntoIterator for #name {
                                    type Item = #ty;
                                    type IntoIter = windows_collections::BufferedIterator<Self::Item>;

                                    fn into_iter(self) -> Self::IntoIter {
                                        IntoIterator::into_iter(&self)
                                    }
                                }
                                #cfg
                                impl<#constraints> IntoIterator for &#name {
                                    type Item = #ty;
                                    type IntoIter = windows_collections::BufferedIterator<Self::Item>;

                                    fn into_iter(self) -> Self::IntoIter {
                                        windows_collections::BufferedIterator::new(self.First().unwrap())
                                    }
                                }

                            }
                        })
                } else {
                    None
                };

                if let Some(into_iterator) = into_iterator {
                    result.combine(into_iterator);
                }
            }

            if config.should_implement(type_name, !is_exclusive) {
                let impl_name: TokenStream = format!("{}_Impl", trim_tick(self.def.name()))
                    .parse()
                    .unwrap();

                let generics: Vec<_> = self
                    .generics
                    .iter()
                    .map(|ty| ty.write_name(config))
                    .collect();

                let runtime_name = format!("{type_name}");

                let cfg = if config.bindgen.layout.is_package() {
                    fn combine(interface: &Interface, dependencies: &mut TypeMap, config: &Config) {
                        for method in &interface.get_methods(config) {
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

                result.combine(if self.generics.is_empty() {
                    quote! {
                        #cfg
                        impl<#constraints> windows_core::RuntimeName for #name {
                            const NAME: &'static str = #runtime_name;
                        }
                    }
                } else {
                    quote! {
                        #cfg
                        impl<#constraints> windows_core::RuntimeName for #name {
                            const NAME: &'static str = #runtime_name;
                            const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer = <Self as windows_core::RuntimeType>::NAME;
                        }
                    }
                });

                // If any methods were skipped due to missing dependencies, the interface cannot be
                // fully described, so omit the ability to implement it rather than emitting a
                // partial vtable with null function pointer slots. Also propagate the omission
                // when any required (base) interface had its `_Impl` trait omitted, since a
                // derived `_Impl` cannot reference a base `_Impl` that wasn't emitted.
                let has_skipped_methods = methods
                    .iter()
                    .any(|method| matches!(method, MethodOrName::Name(_)))
                    || required_interfaces
                        .iter()
                        .any(|ty| ty.has_skipped_methods(config));

                if has_skipped_methods {
                } else {
                    let field_methods = write_impl_field_methods(&methods, config, |name| {
                        quote! { #name: #name::<#(#generics,)* Identity, OFFSET>, }
                    });

                    let mut names = MethodNames::for_style(&config.bindgen.style);

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
                _ => quote! {},
            }).collect();

                    let trait_methods = write_impl_trait_methods(&methods, config, |method| {
                        method.write_impl_signature(config, true, true)
                    });

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
                        type IntoIter = windows_collections::BufferedIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            IntoIterator::into_iter(&self)
                        }
                    }
                    impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
                        type Item = T;
                        type IntoIter = windows_collections::BufferedIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            windows_collections::BufferedIterator::new(self.First().unwrap())
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
        let name: TokenStream = format!("{}_Vtbl", trim_tick(self.def.name()))
            .parse()
            .unwrap();

        if self.generics.is_empty() {
            name
        } else {
            let generics = self.generics.iter().map(|ty| ty.write_name(config));
            quote! { #name < #(#generics,)* > }
        }
    }

    pub fn write_impl_name(&self, config: &Config) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", trim_tick(self.def.name()))
            .parse()
            .unwrap();
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
                .method_signature(&self.generics, reader)
                .types()
            {
                if is_iterable || ty.is_core() {
                    ty.combine(dependencies, reader);
                }
            }
        }
    }
}
