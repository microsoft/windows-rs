use super::*;
use metadata::HasAttributes;

#[derive(Clone)]
pub struct Writer {
    pub reader: &'static metadata::Reader,
    pub output: String,
    pub namespace: &'static str,
    pub implement: bool, // TODO: ideally we can use this to generate implementation traits on the fly and
    // and have a single interface definition macro for consumption that expands to include
    // impl traits when the `implement` cfg flag is set and then this writer option would be
    // unecessary.
    //
    // Maybe this macro is the embedable version of the IDL format?! like a more intelligient
    // version of the existing interface macro...
    pub std: bool,                 // tweaks for internal std library support
    pub sys: bool,                 // writer sys-style bindings
    pub flatten: bool,             // strips out namespaces - implies !package
    pub package: bool,             // default is single file with no cfg - implies !flatten
    pub minimal: bool,             // strips out enumerators - in future possibly other helpers as well
    pub no_inner_attributes: bool, // skips the inner attributes at the start of the file
    pub no_bindgen_comment: bool,  // skips the bindgen comment at the start of the file
    pub vtbl: bool,                // include minimal vtbl layout support for interfaces
    pub prepend: std::collections::HashMap<metadata::TypeDef, String>,
}

impl Writer {
    pub fn new(reader: &'static metadata::Reader, output: &str) -> Self {
        Self {
            reader,
            output: output.to_string(),
            namespace: "",
            implement: false,
            std: false,
            sys: false,
            flatten: false,
            package: false,
            minimal: false,
            no_inner_attributes: false,
            no_bindgen_comment: false,
            vtbl: false,
            prepend: Default::default(),
        }
    }

    //
    // metadata::TypeDef
    //

    pub fn type_def_name(&self, def: metadata::TypeDef, generics: &[metadata::Type]) -> TokenStream {
        self.type_def_name_imp(def, generics, "")
    }
    pub fn type_def_vtbl_name(&self, def: metadata::TypeDef, generics: &[metadata::Type]) -> TokenStream {
        self.type_def_name_imp(def, generics, "_Vtbl")
    }
    pub fn type_def_name_imp(&self, def: metadata::TypeDef, generics: &[metadata::Type], suffix: &str) -> TokenStream {
        let type_name = def.type_name();

        if type_name.namespace().is_empty() {
            to_ident(&self.scoped_name(def))
        } else {
            let mut namespace = self.namespace(type_name.namespace());
            let mut name = to_ident(type_name.name());
            name.push_str(suffix);

            if generics.is_empty() || self.sys {
                namespace.combine(&name);
                namespace
            } else {
                let colon_separated = if !namespace.as_str().is_empty() {
                    quote! { :: }
                } else {
                    quote! {}
                };

                let generics = generics.iter().map(|ty| self.type_name(ty));
                quote! { #namespace #name #colon_separated<#(#generics),*> }
            }
        }
    }
    pub fn type_def(&self, def: metadata::TypeDef) -> TokenStream {
        let tokens = match def.kind() {
            metadata::TypeKind::Class => classes::writer(self, def),
            metadata::TypeKind::Interface => interfaces::writer(self, def),
            metadata::TypeKind::Enum => enums::writer(self, def),
            metadata::TypeKind::Struct => structs::writer(self, def),
            metadata::TypeKind::Delegate => delegates::writer(self, def),
        };

        if let Some(prepend) = self.prepend.get(&def) {
            let mut combined = TokenStream::from(prepend);
            combined.combine(&tokens);
            combined
        } else {
            tokens
        }
    }

    //
    // Type
    //

    pub fn type_default_name(&self, ty: &metadata::Type) -> TokenStream {
        if let metadata::Type::WinrtArray(ty) = ty {
            self.type_default_name(ty)
        } else {
            let kind = self.type_name(ty);

            if matches!(ty, metadata::Type::GenericParam(_)) {
                quote! { <#kind as windows_core::Type<#kind>>::Default }
            } else if metadata::type_is_nullable(ty) && !self.sys {
                quote! { Option<#kind> }
            } else {
                kind
            }
        }
    }

    pub(crate) fn type_name(&self, ty: &metadata::Type) -> TokenStream {
        match ty {
            metadata::Type::Void => quote! { core::ffi::c_void },
            metadata::Type::Bool => quote! { bool },
            metadata::Type::Char => quote! { u16 },
            metadata::Type::I8 => quote! { i8 },
            metadata::Type::U8 => quote! { u8 },
            metadata::Type::I16 => quote! { i16 },
            metadata::Type::U16 => quote! { u16 },
            metadata::Type::I32 => quote! { i32 },
            metadata::Type::U32 => quote! { u32 },
            metadata::Type::I64 => quote! { i64 },
            metadata::Type::U64 => quote! { u64 },
            metadata::Type::F32 => quote! { f32 },
            metadata::Type::F64 => quote! { f64 },
            metadata::Type::ISize => quote! { isize },
            metadata::Type::USize => quote! { usize },
            metadata::Type::String => {
                if self.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let crate_name = self.crate_name();
                    quote! { #crate_name HSTRING }
                }
            }
            metadata::Type::BSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name BSTR }
            }
            metadata::Type::VARIANT => {
                let crate_name = self.crate_name();
                quote! { #crate_name VARIANT }
            }
            metadata::Type::PROPVARIANT => {
                let crate_name = self.crate_name();
                quote! { #crate_name PROPVARIANT }
            }
            metadata::Type::IInspectable => {
                if self.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let crate_name = self.crate_name();
                    quote! { #crate_name IInspectable }
                }
            }
            metadata::Type::GUID => {
                let crate_name = self.crate_name();
                quote! { #crate_name GUID }
            }
            metadata::Type::IUnknown => {
                if self.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let crate_name = self.crate_name();
                    quote! { #crate_name IUnknown }
                }
            }
            metadata::Type::HRESULT => {
                let crate_name = self.crate_name();
                quote! { #crate_name HRESULT }
            }
            metadata::Type::PSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PSTR }
            }
            metadata::Type::PWSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PWSTR }
            }
            metadata::Type::PCSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PCSTR }
            }
            metadata::Type::PCWSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PCWSTR }
            }
            metadata::Type::Win32Array(ty, len) => {
                let name = self.type_default_name(ty);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            metadata::Type::GenericParam(generic) => generic.name().into(),
            metadata::Type::TypeDef(def, generics) => {
                if self.sys {
                    match def.kind() {
                        metadata::TypeKind::Interface => quote! { *mut core::ffi::c_void },
                        metadata::TypeKind::Delegate if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) => quote! { *mut core::ffi::c_void },
                        _ => self.type_def_name(*def, generics),
                    }
                } else {
                    self.type_def_name(*def, generics)
                }
            }
            metadata::Type::MutPtr(ty, pointers) => {
                let pointers = mut_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            metadata::Type::ConstPtr(ty, pointers) => {
                let pointers = const_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            metadata::Type::WinrtArray(ty) => self.type_name(ty),
            metadata::Type::WinrtArrayRef(ty) => self.type_name(ty),
            metadata::Type::ConstRef(ty) => self.type_name(ty),
            metadata::Type::PrimitiveOrEnum(_, ty) => self.type_name(ty),
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_vtbl_name(&self, ty: &metadata::Type) -> TokenStream {
        match ty {
            metadata::Type::TypeDef(def, generics) => self.type_def_vtbl_name(*def, generics),
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_abi_name(&self, ty: &metadata::Type) -> TokenStream {
        if self.sys {
            return match ty {
                metadata::Type::PrimitiveOrEnum(ty, _) => self.type_default_name(ty),
                _ => self.type_default_name(ty),
            };
        }

        match ty {
            metadata::Type::IUnknown | metadata::Type::IInspectable => {
                quote! { *mut core::ffi::c_void }
            }
            metadata::Type::String => {
                quote! { std::mem::MaybeUninit<windows_core::HSTRING> }
            }
            metadata::Type::BSTR => {
                quote! { std::mem::MaybeUninit<windows_core::BSTR> }
            }
            metadata::Type::VARIANT => {
                quote! { std::mem::MaybeUninit<windows_core::VARIANT> }
            }
            metadata::Type::PROPVARIANT => {
                quote! { std::mem::MaybeUninit<windows_core::PROPVARIANT> }
            }
            metadata::Type::Win32Array(kind, len) => {
                let name = self.type_abi_name(kind);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            metadata::Type::GenericParam(generic) => {
                let name = to_ident(generic.name());
                quote! { windows_core::AbiType<#name> }
            }
            metadata::Type::TypeDef(def, _) => match def.kind() {
                metadata::TypeKind::Enum => self.type_def_name(*def, &[]),
                metadata::TypeKind::Struct => {
                    let tokens = self.type_def_name(*def, &[]);
                    if metadata::type_def_is_blittable(*def) {
                        tokens
                    } else {
                        quote! { std::mem::MaybeUninit<#tokens> }
                    }
                }
                metadata::TypeKind::Delegate => {
                    if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
                        quote! { *mut core::ffi::c_void }
                    } else {
                        self.type_def_name(*def, &[])
                    }
                }
                _ => quote! { *mut core::ffi::c_void },
            },
            metadata::Type::MutPtr(kind, pointers) => {
                let pointers_tokens = gen_mut_ptrs(*pointers);
                let kind = if *pointers > 1 { self.type_name(kind) } else { self.type_abi_name(kind) };
                quote! { #pointers_tokens #kind }
            }
            metadata::Type::ConstPtr(kind, pointers) => {
                let pointers_tokens = gen_const_ptrs(*pointers);
                let kind = if *pointers > 1 { self.type_name(kind) } else { self.type_abi_name(kind) };
                quote! { #pointers_tokens #kind }
            }
            metadata::Type::WinrtArray(kind) => self.type_abi_name(kind),
            metadata::Type::WinrtArrayRef(kind) => self.type_abi_name(kind),
            metadata::Type::PrimitiveOrEnum(ty, _) => self.type_name(ty),
            _ => self.type_name(ty),
        }
    }

    //
    // Constraints
    //

    pub fn generic_phantoms(&self, generics: &[metadata::Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { core::marker::PhantomData::<#generic>, });
        }
        tokens
    }
    pub fn generic_named_phantoms(&self, generics: &[metadata::Type]) -> Vec<TokenStream> {
        generics
            .iter()
            .map(|generic| {
                let generic = self.type_name(generic);
                quote! { #generic: core::marker::PhantomData::<#generic>, }
            })
            .collect()
    }
    pub fn generic_constraints(&self, generics: &[metadata::Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { #generic: windows_core::RuntimeType + 'static, });
        }
        tokens
    }
    pub fn generic_names(&self, generics: &[metadata::Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { #generic, });
        }
        tokens
    }
    /// The signature params which are generic (along with their relative index)
    pub fn generic_params<'b>(&'b self, params: &'b [metadata::SignatureParam]) -> impl Iterator<Item = (usize, &metadata::SignatureParam)> + 'b {
        params.iter().filter(move |param| param.is_convertible()).enumerate()
    }
    /// The generic param names (i.e., `T` in `fn foo<T>()`)
    pub fn constraint_generics(&self, params: &[metadata::SignatureParam]) -> TokenStream {
        let mut generics = self.generic_params(params).map(|(position, _)| -> TokenStream { format!("P{position}").into() }).peekable();

        if generics.peek().is_some() {
            quote!(#(#generics),*)
        } else {
            TokenStream::new()
        }
    }
    /// A `where` clause for some constrained generic params
    pub fn where_clause(&self, params: &[metadata::SignatureParam]) -> TokenStream {
        let constraints = self.param_constraints(params);

        if !constraints.is_empty() {
            quote!(where #constraints)
        } else {
            quote!()
        }
    }
    fn param_constraints(&self, params: &[metadata::SignatureParam]) -> TokenStream {
        let mut tokens = TokenStream::new();
        let gen_name = |position| {
            let name: TokenStream = format!("P{position}").into();
            name
        };
        for (position, param) in self.generic_params(params) {
            if param.kind == metadata::SignatureParamKind::IntoParam {
                let name: TokenStream = gen_name(position);
                let into = self.type_name(&param.ty);
                tokens.combine(&quote! { #name: windows_core::Param<#into>, });
            }
        }
        tokens
    }

    //
    // Cfg
    //

    pub(crate) fn cfg_features(&self, cfg: &cfg::Cfg) -> TokenStream {
        let arches = &cfg.arches;
        let arch = match arches.len() {
            0 => quote! {},
            1 => {
                quote! { #[cfg(#(target_arch = #arches),*)] }
            }
            _ => {
                quote! { #[cfg(any(#(target_arch = #arches),*))] }
            }
        };

        let features = self.cfg_features_imp(cfg, self.namespace);

        let features = match features.len() {
            0 => quote! {},
            1 => {
                let features = features.iter().cloned().map(to_feature);
                quote! { #[cfg(#(feature = #features)*)] }
            }
            _ => {
                let features = features.iter().cloned().map(to_feature);
                quote! { #[cfg(all( #(feature = #features),* ))] }
            }
        };

        quote! { #arch #features }
    }

    fn cfg_features_imp(&self, cfg: &cfg::Cfg, namespace: &str) -> Vec<&'static str> {
        let mut compact = Vec::<&'static str>::new();
        if self.package {
            for feature in cfg.types.keys() {
                if !feature.is_empty() && !starts_with(namespace, feature) && !is_defaulted_foundation_feature(namespace, feature) {
                    for pos in 0..compact.len() {
                        if starts_with(feature, unsafe { compact.get_unchecked(pos) }) {
                            compact.remove(pos);
                            break;
                        }
                    }
                    compact.push(feature);
                }
            }
        }
        compact
    }

    fn cfg_not_features(&self, cfg: &cfg::Cfg) -> TokenStream {
        let features = self.cfg_features_imp(cfg, self.namespace);
        if features.is_empty() {
            quote! {}
        } else {
            match features.len() {
                0 => quote! {},
                1 => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(not(#(feature = #features)*))] }
                }
                _ => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(not(all( #(feature = #features),* )))] }
                }
            }
        }
    }

    //
    // Other helpers
    //

    pub(crate) fn namespace(&self, namespace: &str) -> TokenStream {
        if self.flatten || namespace == self.namespace {
            quote! {}
        } else {
            let is_external = namespace.starts_with("Windows.") && !self.namespace.starts_with("Windows");
            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::new();

            if is_external {
                tokens.push_str("windows::");
                namespace.next();
            } else {
                for _ in 0..relative.count() {
                    tokens.push_str("super::");
                }
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }
    pub fn crate_name(&self) -> TokenStream {
        if self.sys {
            if self.flatten {
                TokenStream::new()
            } else {
                "windows_sys::core::".into()
            }
        } else {
            "windows_core::".into()
        }
    }
    fn scoped_name(&self, def: metadata::TypeDef) -> String {
        if let Some(enclosing_type) = def.enclosing_type() {
            for (index, nested_type) in self.reader.nested_types(enclosing_type).enumerate() {
                if nested_type.name() == def.name() {
                    return format!("{}_{index}", &self.scoped_name(enclosing_type));
                }
            }
        }
        def.name().to_string()
    }
    pub fn value(&self, value: &metadata::Value) -> TokenStream {
        match value {
            metadata::Value::Bool(value) => quote! { #value },
            metadata::Value::U8(value) => quote! { #value },
            metadata::Value::I8(value) => quote! { #value },
            metadata::Value::U16(value) => quote! { #value },
            metadata::Value::I16(value) => quote! { #value },
            metadata::Value::U32(value) => quote! { #value },
            metadata::Value::I32(value) => quote! { #value },
            metadata::Value::U64(value) => quote! { #value },
            metadata::Value::I64(value) => quote! { #value },
            metadata::Value::F32(value) => quote! { #value },
            metadata::Value::F64(value) => quote! { #value },
            metadata::Value::String(value) => {
                let mut tokens = "\"".to_string();

                for u in value.chars() {
                    write!(tokens, "{}", u.escape_default()).unwrap();
                }

                tokens.push('\"');
                tokens.into()
            }
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn typed_value(&self, value: &metadata::Value) -> TokenStream {
        let literal = self.value(value);
        match value {
            metadata::Value::Bool(_) => quote! { bool = #literal },
            metadata::Value::U8(_) => quote! { u8 = #literal },
            metadata::Value::I8(_) => quote! { i8 = #literal },
            metadata::Value::U16(_) => quote! { u16 = #literal },
            metadata::Value::I16(_) => quote! { i16 = #literal },
            metadata::Value::U32(_) => quote! { u32 = #literal },
            metadata::Value::I32(_) => quote! { i32 = #literal },
            metadata::Value::U64(_) => quote! { u64 = #literal },
            metadata::Value::I64(_) => quote! { i64 = #literal },
            metadata::Value::F32(_) => quote! { f32 = #literal },
            metadata::Value::F64(_) => quote! { f64 = #literal },
            metadata::Value::String(_) => {
                quote! { &str = #literal }
            }
            rest => unimplemented!("{rest:?}"),
        }
    }

    pub fn guid(&self, value: &metadata::Guid) -> TokenStream {
        let guid = self.type_name(&metadata::Type::GUID);
        format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
    }

    pub fn guid_literal(&self, value: Option<metadata::Guid>) -> TokenStream {
        match value {
            Some(value) => format!("0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into(),
            None => quote! { 0 },
        }
    }

    pub fn agile(&self, def: metadata::TypeDef, ident: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        if type_def_is_agile(def) {
            quote! {
                #features
                unsafe impl<#constraints> Send for #ident {}
                #features
                unsafe impl<#constraints> Sync for #ident {}
            }
        } else {
            quote! {}
        }
    }
    pub fn async_get(&self, def: metadata::TypeDef, generics: &[metadata::Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        let mut kind = type_def_async_kind(def);
        let mut async_generics = generics.to_vec();

        if kind == metadata::AsyncKind::None {
            for interface in def.interface_impls().map(move |imp| imp.ty(generics)) {
                if let metadata::Type::TypeDef(interface_def, interface_generics) = &interface {
                    kind = type_def_async_kind(*interface_def);
                    if kind != metadata::AsyncKind::None {
                        async_generics = interface_generics.to_vec();
                        break;
                    }
                }
            }
        }

        if kind == metadata::AsyncKind::None {
            quote! {}
        } else {
            let return_type = match kind {
                metadata::AsyncKind::Operation | metadata::AsyncKind::OperationWithProgress => self.type_name(&async_generics[0]),
                _ => quote! { () },
            };

            let handler = match kind {
                metadata::AsyncKind::Action => quote! { AsyncActionCompletedHandler },
                metadata::AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
                metadata::AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
                metadata::AsyncKind::OperationWithProgress => {
                    quote! { AsyncOperationWithProgressCompletedHandler }
                }
                rest => unimplemented!("{rest:?}"),
            };

            let namespace = self.namespace("Windows.Foundation");

            quote! {
                #features
                impl<#constraints> #ident {
                    pub fn get(&self) -> windows_core::Result<#return_type> {
                        if self.Status()? == #namespace AsyncStatus::Started {
                            let (_waiter, signaler) = windows_core::imp::Waiter::new()?;
                            self.SetCompleted(&#namespace  #handler::new(move |_sender, _args| {
                                // Safe because the waiter will only be dropped after being signaled.
                                unsafe { signaler.signal(); }
                                Ok(())
                            }))?;
                        }
                        self.GetResults()
                    }
                }
                #features
                impl<#constraints> std::future::Future for #ident {
                    type Output = windows_core::Result<#return_type>;

                    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                        if self.Status()? == #namespace AsyncStatus::Started {
                            let waker = context.waker().clone();

                            let _ = self.SetCompleted(&#namespace #handler::new(move |_sender, _args| {
                                waker.wake_by_ref();
                                Ok(())
                            }));

                            std::task::Poll::Pending
                        } else {
                            std::task::Poll::Ready(self.GetResults())
                        }
                    }
                }
            }
        }
    }
    pub fn interface_winrt_trait(&self, def: metadata::TypeDef, generics: &[metadata::Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
            let type_signature = if def.kind() == metadata::TypeKind::Class {
                let default = metadata::type_def_default_interface(def).expect("missing default interface");
                let default_name = self.type_name(&default);
                quote! { windows_core::imp::ConstBuffer::for_class::<Self, #default_name>() }
            } else if generics.is_empty() {
                quote! { windows_core::imp::ConstBuffer::for_interface::<Self>() }
            } else {
                let signature = Literal::byte_string(
                    // TODO: workaround for riddle winmd generation (no attribute support)
                    if let Some(guid) = metadata::type_def_guid(def) { format!("{{{:#?}}}", guid) } else { "TODO".to_string() }.as_bytes(),
                );

                let generics = generics.iter().enumerate().map(|(index, g)| {
                    let g = self.type_name(g);
                    let semi = if index != generics.len() - 1 {
                        Some(quote! {
                            .push_slice(b";")
                        })
                    } else {
                        None
                    };

                    quote! {
                        .push_other(#g::SIGNATURE)
                        #semi
                    }
                });

                quote! {
                    {
                        windows_core::imp::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(#signature)
                        .push_slice(b";")
                        #(#generics)*
                        .push_slice(b")")
                    }
                }
            };

            quote! {
                #features
                impl<#constraints> windows_core::RuntimeType for #ident {
                    const SIGNATURE: windows_core::imp::ConstBuffer = #type_signature;
                }
            }
        } else {
            quote! {}
        }
    }
    pub fn runtime_name_trait(&self, def: metadata::TypeDef, _generics: &[metadata::Type], name: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
            // TODO: this needs to use a ConstBuffer-like facility to accomodate generics
            let runtime_name = format!("{}", def.type_name());

            quote! {
                #features
                impl<#constraints> windows_core::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
            }
        } else {
            quote! {
                #features
                impl windows_core::RuntimeName for #name {}
            }
        }
    }

    pub fn interface_trait(&self, def: metadata::TypeDef, generics: &[metadata::Type], ident: &TokenStream, constraints: &TokenStream, features: &TokenStream, has_unknown_base: bool) -> TokenStream {
        if let Some(default) = metadata::type_def_default_interface(def) {
            let default_name = self.type_name(&default);
            let vtbl = self.type_vtbl_name(&default);
            quote! {
                #features
                unsafe impl windows_core::Interface for #ident {
                    type Vtable = #vtbl;
                    const IID: windows_core::GUID = <#default_name as windows_core::Interface>::IID;
                }
            }
        } else {
            let vtbl = self.type_def_vtbl_name(def, generics);
            let guid = if generics.is_empty() {
                match metadata::type_def_guid(def) {
                    Some(guid) => self.guid(&guid),
                    None => {
                        quote! {
                            windows_core::GUID::zeroed()
                        }
                    }
                }
            } else {
                quote! {
                    windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE)
                }
            };

            if has_unknown_base {
                if generics.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        #features
                        unsafe impl<#constraints> windows_core::Interface for #ident {
                            type Vtable = #vtbl;
                            const IID: windows_core::GUID = #guid;
                        }
                    }
                }
            } else {
                quote! {}
            }
        }
    }
    pub fn interface_vtbl(&self, def: metadata::TypeDef, generics: &[metadata::Type], constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        let vtbl = self.type_def_vtbl_name(def, generics);
        let mut methods = quote! {};
        let mut method_names = MethodNames::new();
        let phantoms = self.generic_named_phantoms(generics);
        let crate_name = self.crate_name();

        match metadata::type_def_vtables(def).last() {
            Some(metadata::Type::IUnknown) => methods.combine(&quote! { pub base__: #crate_name IUnknown_Vtbl, }),
            Some(metadata::Type::IInspectable) => methods.combine(&quote! { pub base__: #crate_name IInspectable_Vtbl, }),
            Some(metadata::Type::TypeDef(def, _)) => {
                let vtbl = self.type_def_vtbl_name(*def, &[]);
                methods.combine(&quote! { pub base__: #vtbl, });
            }
            _ => {}
        }

        for method in def.methods() {
            if method.name() == ".ctor" {
                continue;
            }
            let name = method_names.add(method);
            let signature = metadata::method_def_signature(def.namespace(), method, generics);
            let mut cfg = cfg::signature_cfg(self, method);
            let signature = self.vtbl_signature(def, false, &signature);
            cfg.add_feature(def.namespace());
            let cfg_all = self.cfg_features(&cfg);
            let cfg_not = self.cfg_not_features(&cfg);

            let signature = quote! { pub #name: unsafe extern "system" fn #signature, };

            if cfg_all.is_empty() {
                methods.combine(&signature);
            } else {
                methods.combine(&quote! {
                    #cfg_all
                    #signature
                    #cfg_not
                    #name: usize,
                });
            }
        }

        quote! {
            #features
            #[repr(C)] pub struct #vtbl where #constraints {
                #methods
                #(pub #phantoms)*
            }
        }
    }
    pub fn vtbl_signature(&self, def: metadata::TypeDef, named_params: bool, signature: &metadata::Signature) -> TokenStream {
        let is_winrt = def.flags().contains(metadata::TypeAttributes::WindowsRuntime);

        let crate_name = self.crate_name();

        let (trailing_return_type, return_type, udt_return_type) = match &signature.return_type {
            metadata::Type::Void if is_winrt => (quote! {}, quote! { -> #crate_name HRESULT }, quote! {}),
            metadata::Type::Void => (quote! {}, quote! {}, quote! {}),
            metadata::Type::WinrtArray(kind) => {
                let tokens = self.type_abi_name(kind);
                if named_params {
                    (quote! { result_size__: *mut u32, result__: *mut *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
                } else {
                    (quote! { *mut u32, *mut *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
                }
            }
            _ if is_winrt => {
                let tokens = self.type_abi_name(&signature.return_type);
                if named_params {
                    (quote! { result__: *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
                } else {
                    (quote! { *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
                }
            }
            _ if metadata::type_is_struct(&signature.return_type) => {
                let tokens = self.type_abi_name(&signature.return_type);
                if named_params {
                    (quote! {}, quote! {}, quote! { result__: *mut #tokens, })
                } else {
                    (quote! {}, quote! {}, quote! { *mut #tokens, })
                }
            }
            _ => {
                let tokens = self.type_default_name(&signature.return_type);
                (quote! {}, quote! { -> #tokens }, quote! {})
            }
        };

        let params = signature.params.iter().map(|p| {
            let name = self.param_name(p.def);
            if is_winrt {
                let abi = self.type_abi_name(&p.ty);
                let abi_size_name: TokenStream = format!("{}_array_size", p.def.name()).into();

                if p.def.flags().contains(metadata::ParamAttributes::In) {
                    if p.ty.is_winrt_array() {
                        if named_params {
                            quote! { #abi_size_name: u32, #name: *const #abi, }
                        } else {
                            quote! { u32, *const #abi, }
                        }
                    } else if p.ty.is_const_ref() {
                        if named_params {
                            quote! { #name: &#abi, }
                        } else {
                            quote! { &#abi, }
                        }
                    } else if named_params {
                        quote! { #name: #abi, }
                    } else {
                        quote! { #abi, }
                    }
                } else if p.ty.is_winrt_array() {
                    if named_params {
                        quote! { #abi_size_name: u32, #name: *mut #abi, }
                    } else {
                        quote! { u32, *mut #abi, }
                    }
                } else if p.ty.is_winrt_array_ref() {
                    if named_params {
                        quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi, }
                    } else {
                        quote! { *mut u32, *mut *mut #abi, }
                    }
                } else if named_params {
                    quote! { #name: *mut #abi, }
                } else {
                    quote! { *mut #abi, }
                }
            } else {
                match p.kind {
                    metadata::SignatureParamKind::ValueType => {
                        let abi = self.type_default_name(&p.ty);
                        if named_params {
                            quote! { #name: #abi, }
                        } else {
                            quote! { #abi, }
                        }
                    }
                    _ => {
                        let abi = self.type_abi_name(&p.ty);
                        if named_params {
                            quote! { #name: #abi, }
                        } else {
                            quote! { #abi, }
                        }
                    }
                }
            }
        });

        if named_params {
            quote! { (this: *mut core::ffi::c_void, #udt_return_type #(#params)* #trailing_return_type) #return_type }
        } else {
            quote! { (*mut core::ffi::c_void, #udt_return_type #(#params)* #trailing_return_type) #return_type }
        }
    }
    pub fn param_name(&self, param: metadata::Param) -> TokenStream {
        // In Rust, function parameters cannot be named the same as structs. This avoids some collisions that occur in the win32 metadata.
        // See Icmp6SendEcho2 for an example.
        to_ident(&param.name().to_lowercase())
    }
    pub fn return_sig(&self, signature: &metadata::Signature) -> TokenStream {
        match &signature.return_type {
            metadata::Type::Void if signature.def.has_attribute("DoesNotReturnAttribute") => " -> !".into(),
            metadata::Type::Void => TokenStream::new(),
            _ => {
                let tokens = self.type_default_name(&signature.return_type);
                format!(" -> {}", tokens.as_str()).into()
            }
        }
    }
    pub fn win32_args(&self, params: &[metadata::SignatureParam], kind: metadata::SignatureKind) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in params.iter().enumerate() {
            let new = match kind {
                metadata::SignatureKind::Query(query) if query.object == position => {
                    quote! { &mut result__, }
                }
                metadata::SignatureKind::ReturnValue | metadata::SignatureKind::ResultValue if params.len() - 1 == position => {
                    quote! { &mut result__, }
                }
                metadata::SignatureKind::QueryOptional(query) if query.object == position => {
                    quote! { result__ as *mut _ as *mut _, }
                }
                metadata::SignatureKind::Query(query) | metadata::SignatureKind::QueryOptional(query) if query.guid == position => {
                    quote! { &T::IID, }
                }
                _ => {
                    let name = self.param_name(param.def);
                    let flags = param.def.flags();
                    match param.kind {
                        metadata::SignatureParamKind::ArrayFixed(_) | metadata::SignatureParamKind::ArrayRelativeLen(_) | metadata::SignatureParamKind::ArrayRelativeByteLen(_) => {
                            let map = if flags.contains(metadata::ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(core::ptr::null(), |slice|slice.as_ptr()) }
                            } else {
                                quote! { #name.as_ptr() }
                            };
                            quote! { core::mem::transmute(#map), }
                        }
                        metadata::SignatureParamKind::ArrayRelativePtr(relative) => {
                            let name = self.param_name(params[relative].def);
                            let flags = params[relative].def.flags();
                            if flags.contains(metadata::ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(0, |slice|slice.len().try_into().unwrap()), }
                            } else {
                                quote! { #name.len().try_into().unwrap(), }
                            }
                        }
                        metadata::SignatureParamKind::IntoParam => {
                            quote! { #name.param().abi(), }
                        }
                        metadata::SignatureParamKind::OptionalPointer => {
                            if flags.contains(metadata::ParamAttributes::Out) {
                                quote! { core::mem::transmute(#name.unwrap_or(std::ptr::null_mut())), }
                            } else {
                                quote! { core::mem::transmute(#name.unwrap_or(std::ptr::null())), }
                            }
                        }
                        metadata::SignatureParamKind::ValueType => {
                            quote! { #name, }
                        }
                        metadata::SignatureParamKind::Blittable => {
                            if matches!(param.ty, metadata::Type::PrimitiveOrEnum(_, _)) {
                                quote! { #name.0 as _, }
                            } else {
                                quote! { core::mem::transmute(#name), }
                            }
                        }
                        metadata::SignatureParamKind::Other => {
                            quote! { core::mem::transmute_copy(#name), }
                        }
                    }
                }
            };
            tokens.combine(&new)
        }

        tokens
    }
    pub fn win32_params(&self, params: &[metadata::SignatureParam], kind: metadata::SignatureKind) -> TokenStream {
        let mut tokens = quote! {};

        let mut generic_params = self.generic_params(params);
        for (position, param) in params.iter().enumerate() {
            match kind {
                metadata::SignatureKind::Query(query) | metadata::SignatureKind::QueryOptional(query) => {
                    if query.object == position || query.guid == position {
                        continue;
                    }
                }
                metadata::SignatureKind::ReturnValue | metadata::SignatureKind::ResultValue if params.len() - 1 == position => {
                    continue;
                }
                _ => {}
            }

            let name = self.param_name(param.def);

            match param.kind {
                metadata::SignatureParamKind::ArrayFixed(fixed) => {
                    let ty = param.ty.deref();
                    let ty = self.type_default_name(&ty);
                    let len = Literal::u32_unsuffixed(fixed as u32);
                    let ty = if param.def.flags().contains(metadata::ParamAttributes::Out) {
                        quote! { &mut [#ty; #len] }
                    } else {
                        quote! { &[#ty; #len] }
                    };
                    if param.def.flags().contains(metadata::ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                metadata::SignatureParamKind::ArrayRelativeLen(_) => {
                    let ty = param.ty.deref();
                    let ty = self.type_default_name(&ty);
                    let ty = if param.def.flags().contains(metadata::ParamAttributes::Out) {
                        quote! { &mut [#ty] }
                    } else {
                        quote! { &[#ty] }
                    };
                    if param.def.flags().contains(metadata::ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                metadata::SignatureParamKind::ArrayRelativeByteLen(_) => {
                    let ty = if param.def.flags().contains(metadata::ParamAttributes::Out) {
                        quote! { &mut [u8] }
                    } else {
                        quote! { &[u8] }
                    };
                    if param.def.flags().contains(metadata::ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                metadata::SignatureParamKind::ArrayRelativePtr(_) => {}
                metadata::SignatureParamKind::IntoParam => {
                    let (position, _) = generic_params.next().unwrap();
                    let kind: TokenStream = format!("P{position}").into();
                    tokens.combine(&quote! { #name: #kind, });
                }
                metadata::SignatureParamKind::OptionalPointer => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: Option<#kind>, });
                }
                metadata::SignatureParamKind::ValueType | metadata::SignatureParamKind::Blittable => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: #kind, });
                }
                metadata::SignatureParamKind::Other => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: &#kind, });
                }
            }
        }

        tokens
    }

    pub fn impl_signature(&self, def: metadata::TypeDef, signature: &metadata::Signature) -> TokenStream {
        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
            let is_delegate = def.kind() == metadata::TypeKind::Delegate;
            let params = signature.params.iter().map(|p| self.winrt_produce_type(p, !is_delegate));

            let return_type = match &signature.return_type {
                metadata::Type::Void => quote! { () },
                _ => {
                    let tokens = self.type_name(&signature.return_type);

                    if signature.return_type.is_winrt_array() {
                        quote! { windows_core::Array<#tokens> }
                    } else {
                        tokens
                    }
                }
            };

            let this = if is_delegate {
                quote! {}
            } else {
                quote! { &self, }
            };

            quote! { (#this #(#params),*) -> windows_core::Result<#return_type> }
        } else {
            let signature_kind = signature.kind();
            let mut params = quote! {};

            if signature_kind == metadata::SignatureKind::ResultValue {
                for param in &signature.params[..signature.params.len() - 1] {
                    params.combine(&self.win32_produce_type(param));
                }
            } else {
                for param in &signature.params {
                    params.combine(&self.win32_produce_type(param));
                }
            }

            let return_type = match signature_kind {
                metadata::SignatureKind::ReturnVoid => quote! {},
                metadata::SignatureKind::Query(_) | metadata::SignatureKind::QueryOptional(_) | metadata::SignatureKind::ResultVoid => quote! { -> windows_core::Result<()> },
                metadata::SignatureKind::ResultValue => {
                    let return_type = signature.params[signature.params.len() - 1].ty.deref();
                    let return_type = self.type_name(&return_type);

                    quote! { -> windows_core::Result<#return_type> }
                }
                _ => self.return_sig(signature),
            };

            quote! { (&self, #params) #return_type }
        }
    }
    fn winrt_produce_type(&self, param: &metadata::SignatureParam, include_param_names: bool) -> TokenStream {
        let default_type = self.type_default_name(&param.ty);

        let sig = if param.def.flags().contains(metadata::ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                quote! { &[#default_type] }
            } else if metadata::type_is_primitive(&param.ty) {
                quote! { #default_type }
            } else if metadata::type_is_nullable(&param.ty) {
                let type_name = self.type_name(&param.ty);
                quote! { Option<&#type_name> }
            } else {
                quote! { &#default_type }
            }
        } else if param.ty.is_winrt_array() {
            quote! { &mut [#default_type] }
        } else if param.ty.is_winrt_array_ref() {
            let kind = self.type_name(&param.ty);
            quote! { &mut windows_core::Array<#kind> }
        } else {
            quote! { &mut #default_type }
        };

        if include_param_names {
            let name = self.param_name(param.def);
            quote! { #name: #sig }
        } else {
            sig
        }
    }
    fn win32_produce_type(&self, param: &metadata::SignatureParam) -> TokenStream {
        let name = self.param_name(param.def);
        let kind = self.type_default_name(&param.ty);

        if param.def.flags().contains(metadata::ParamAttributes::In) {
            if metadata::type_is_primitive(&param.ty) {
                quote! { #name: #kind, }
            } else if metadata::type_is_nullable(&param.ty) {
                let kind = self.type_name(&param.ty);
                quote! { #name: Option<&#kind>, }
            } else {
                quote! { #name: &#kind, }
            }
        } else {
            quote! { #name: #kind, }
        }
    }
}

fn mut_ptrs(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn const_ptrs(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}

pub fn cfg_features(cfg: &cfg::Cfg) -> Vec<String> {
    let mut compact = Vec::<&'static str>::new();
    for feature in cfg.types.keys() {
        if !feature.is_empty() {
            for pos in 0..compact.len() {
                if starts_with(feature, unsafe { compact.get_unchecked(pos) }) {
                    compact.remove(pos);
                    break;
                }
            }
            compact.push(feature);
        }
    }
    compact.into_iter().map(to_feature).collect()
}

fn to_feature(name: &str) -> String {
    let mut feature = String::new();

    for name in name.split('.').skip(1) {
        feature.push_str(name);
        feature.push('_');
    }

    if feature.is_empty() {
        feature = name.to_string();
    } else {
        feature.truncate(feature.len() - 1);
    }

    feature
}

fn starts_with(namespace: &str, feature: &str) -> bool {
    if namespace == feature {
        return true;
    }

    if namespace.len() > feature.len() && namespace.as_bytes().get(feature.len()) == Some(&b'.') {
        return namespace.starts_with(feature);
    }

    false
}

fn is_defaulted_foundation_feature(namespace: &str, feature: &str) -> bool {
    let is_winrt = !starts_with(namespace, "Windows.Win32") && !starts_with(namespace, "Windows.Wdk");

    if !is_winrt && feature == "Windows.Win32.Foundation" {
        true
    } else {
        is_winrt && feature == "Windows.Foundation"
    }
}

fn gen_mut_ptrs(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn gen_const_ptrs(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}

fn type_def_async_kind(row: metadata::TypeDef) -> metadata::AsyncKind {
    match row.type_name() {
        metadata::TypeName::IAsyncAction => metadata::AsyncKind::Action,
        metadata::TypeName::IAsyncActionWithProgress => metadata::AsyncKind::ActionWithProgress,
        metadata::TypeName::IAsyncOperation => metadata::AsyncKind::Operation,
        metadata::TypeName::IAsyncOperationWithProgress => metadata::AsyncKind::OperationWithProgress,
        _ => metadata::AsyncKind::None,
    }
}

fn type_def_is_agile(row: metadata::TypeDef) -> bool {
    for attribute in row.attributes() {
        match attribute.name() {
            "AgileAttribute" => return true,
            "MarshalingBehaviorAttribute" => {
                if let Some((_, metadata::Value::EnumDef(_, value))) = attribute.args().first() {
                    if let metadata::Value::I32(2) = **value {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    matches!(row.type_name(), metadata::TypeName::IAsyncAction | metadata::TypeName::IAsyncActionWithProgress | metadata::TypeName::IAsyncOperation | metadata::TypeName::IAsyncOperationWithProgress)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        assert!(starts_with("Windows.Win32.Graphics.Direct3D11on12", "Windows.Win32.Graphics.Direct3D11on12"));
        assert!(starts_with("Windows.Win32.Graphics.Direct3D11on12", "Windows.Win32.Graphics"));
        assert!(!starts_with("Windows.Win32.Graphics.Direct3D11on12", "Windows.Win32.Graphics.Direct3D11"));
        assert!(!starts_with("Windows.Win32.Graphics.Direct3D", "Windows.Win32.Graphics.Direct3D11"));
    }
}
