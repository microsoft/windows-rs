use super::*;

#[derive(Clone)]
pub struct Writer<'a> {
    pub reader: &'a Reader<'a>,
    pub filter: &'a metadata::Filter<'a>,
    pub output: &'a str,
    pub namespace: &'a str,
    pub implement: bool, // TODO: ideally we can use this to generate implementation traits on the fly and
    // and have a single interface definition macro for consumption that expands to include
    // impl traits when the `implement` cfg flag is set and then this writer option would be
    // unecessary.
    //
    // Maybe this macro is the embedable version of the IDL format?! like a more intelligient
    // version of the existing interface macro...
    pub std: bool,     // tweaks for internal std library support
    pub sys: bool,     // writer sys-style bindings
    pub flatten: bool, // strips out namespaces - implies !package
    pub package: bool, // default is single file with no cfg - implies !flatten
    pub minimal: bool, // strips out enumerators - in future possibly other helpers as well
}

impl<'a> Writer<'a> {
    pub fn new(reader: &'a Reader, filter: &'a metadata::Filter, output: &'a str) -> Self {
        Self { reader, filter, output, namespace: "", implement: false, std: false, sys: false, flatten: false, package: false, minimal: false }
    }

    //
    // TypeDef
    //

    pub fn type_def_name(&self, def: TypeDef, generics: &[Type]) -> TokenStream {
        self.type_def_name_imp(def, generics, "")
    }
    pub fn type_def_vtbl_name(&self, def: TypeDef, generics: &[Type]) -> TokenStream {
        self.type_def_name_imp(def, generics, "_Vtbl")
    }
    pub fn type_def_name_imp(&self, def: TypeDef, generics: &[Type], suffix: &str) -> TokenStream {
        let type_name = self.reader.type_def_type_name(def);

        if type_name.namespace.is_empty() {
            to_ident(&self.scoped_name(def))
        } else {
            let mut namespace = self.namespace(type_name.namespace);
            let mut name = to_ident(type_name.name);
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

    //
    // Type
    //

    pub fn type_default_name(&self, ty: &Type) -> TokenStream {
        if let Type::WinrtArray(ty) = ty {
            self.type_default_name(ty)
        } else {
            let kind = self.type_name(ty);

            if ty.is_generic() {
                quote! { <#kind as ::windows_core::Type<#kind>>::Default }
            } else if type_is_nullable(self.reader, ty) && !self.sys {
                quote! { ::core::option::Option<#kind> }
            } else {
                kind
            }
        }
    }

    pub(crate) fn type_name(&self, ty: &Type) -> TokenStream {
        match ty {
            Type::Void => quote! { ::core::ffi::c_void },
            Type::Bool => quote! { bool },
            Type::Char => quote! { u16 },
            Type::I8 => quote! { i8 },
            Type::U8 => quote! { u8 },
            Type::I16 => quote! { i16 },
            Type::U16 => quote! { u16 },
            Type::I32 => quote! { i32 },
            Type::U32 => quote! { u32 },
            Type::I64 => quote! { i64 },
            Type::U64 => quote! { u64 },
            Type::F32 => quote! { f32 },
            Type::F64 => quote! { f64 },
            Type::ISize => quote! { isize },
            Type::USize => quote! { usize },
            Type::String => {
                let crate_name = self.crate_name();
                quote! { #crate_name HSTRING }
            }
            Type::BSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name BSTR }
            }
            Type::IInspectable => {
                let crate_name = self.crate_name();
                quote! { #crate_name IInspectable }
            }
            Type::GUID => {
                let crate_name = self.crate_name();
                quote! { #crate_name GUID }
            }
            Type::IUnknown => {
                let crate_name = self.crate_name();
                quote! { #crate_name IUnknown }
            }
            Type::HRESULT => {
                let crate_name = self.crate_name();
                quote! { #crate_name HRESULT }
            }
            Type::PSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PSTR }
            }
            Type::PWSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PWSTR }
            }
            Type::PCSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PCSTR }
            }
            Type::PCWSTR => {
                let crate_name = self.crate_name();
                quote! { #crate_name PCWSTR }
            }
            Type::Win32Array(ty, len) => {
                let name = self.type_default_name(ty);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Type::GenericParam(generic) => self.reader.generic_param_name(*generic).into(),
            Type::TypeDef(def, generics) => self.type_def_name(*def, generics),
            Type::MutPtr(ty, pointers) => {
                let pointers = mut_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            Type::ConstPtr(ty, pointers) => {
                let pointers = const_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            Type::WinrtArray(ty) => self.type_name(ty),
            Type::WinrtArrayRef(ty) => self.type_name(ty),
            Type::ConstRef(ty) => self.type_name(ty),
            Type::PrimitiveOrEnum(_, ty) => self.type_name(ty),
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_vtbl_name(&self, ty: &Type) -> TokenStream {
        match ty {
            Type::TypeDef(def, generics) => self.type_def_vtbl_name(*def, generics),
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_abi_name(&self, ty: &Type) -> TokenStream {
        if self.sys {
            return match ty {
                Type::PrimitiveOrEnum(ty, _) => self.type_default_name(ty),
                _ => self.type_default_name(ty),
            };
        }

        match ty {
            Type::IUnknown | Type::IInspectable => {
                quote! { *mut ::core::ffi::c_void }
            }
            Type::String => {
                quote! { ::std::mem::MaybeUninit<::windows_core::HSTRING> }
            }
            Type::BSTR => {
                quote! { ::std::mem::MaybeUninit<::windows_core::BSTR> }
            }
            Type::Win32Array(kind, len) => {
                let name = self.type_abi_name(kind);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Type::GenericParam(generic) => {
                let name = to_ident(self.reader.generic_param_name(*generic));
                quote! { ::windows_core::AbiType<#name> }
            }
            Type::TypeDef(def, _) => match self.reader.type_def_kind(*def) {
                TypeKind::Enum => self.type_def_name(*def, &[]),
                TypeKind::Struct => {
                    let tokens = self.type_def_name(*def, &[]);
                    if type_def_is_blittable(self.reader, *def) {
                        tokens
                    } else {
                        quote! { ::std::mem::MaybeUninit<#tokens> }
                    }
                }
                TypeKind::Delegate => {
                    if self.reader.type_def_flags(*def).contains(TypeAttributes::WindowsRuntime) {
                        quote! { *mut ::core::ffi::c_void }
                    } else {
                        self.type_def_name(*def, &[])
                    }
                }
                _ => quote! { *mut ::core::ffi::c_void },
            },
            Type::MutPtr(kind, pointers) => {
                let pointers_tokens = gen_mut_ptrs(*pointers);
                let kind = if *pointers > 1 { self.type_name(kind) } else { self.type_abi_name(kind) };
                quote! { #pointers_tokens #kind }
            }
            Type::ConstPtr(kind, pointers) => {
                let pointers_tokens = gen_const_ptrs(*pointers);
                let kind = if *pointers > 1 { self.type_name(kind) } else { self.type_abi_name(kind) };
                quote! { #pointers_tokens #kind }
            }
            Type::WinrtArray(kind) => self.type_abi_name(kind),
            Type::WinrtArrayRef(kind) => self.type_abi_name(kind),
            Type::PrimitiveOrEnum(ty, _) => self.type_name(ty),
            _ => self.type_name(ty),
        }
    }

    //
    // Constraints
    //

    pub fn generic_phantoms(&self, generics: &[Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { ::core::marker::PhantomData::<#generic>, });
        }
        tokens
    }
    pub fn generic_named_phantoms(&self, generics: &[Type]) -> Vec<TokenStream> {
        generics
            .iter()
            .map(|generic| {
                let generic = self.type_name(generic);
                quote! { #generic: ::core::marker::PhantomData::<#generic>, }
            })
            .collect()
    }
    pub fn generic_constraints(&self, generics: &[Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { #generic: ::windows_core::RuntimeType + 'static, });
        }
        tokens
    }
    pub fn generic_names(&self, generics: &[Type]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            let generic = self.type_name(generic);
            tokens.combine(&quote! { #generic, });
        }
        tokens
    }
    /// The signature params which are generic (along with their relative index)
    pub fn generic_params<'b>(&'b self, params: &'b [SignatureParam]) -> impl Iterator<Item = (usize, &SignatureParam)> + 'b {
        params.iter().filter(move |param| signature_param_is_convertible(self.reader, param)).enumerate()
    }
    /// The generic param names (i.e., `T` in `fn foo<T>()`)
    pub fn constraint_generics(&self, params: &[SignatureParam]) -> TokenStream {
        let mut generics = self.generic_params(params).map(|(position, _)| -> TokenStream { format!("P{position}").into() }).peekable();

        if generics.peek().is_some() {
            quote!(#(#generics),*)
        } else {
            TokenStream::new()
        }
    }
    /// A `where` clause for some constrained generic params
    pub fn where_clause(&self, params: &[SignatureParam]) -> TokenStream {
        let constraints = self.param_constraints(params);

        if !constraints.is_empty() {
            quote!(where #constraints)
        } else {
            quote!()
        }
    }
    fn param_constraints(&self, params: &[SignatureParam]) -> TokenStream {
        let mut tokens = TokenStream::new();
        let gen_name = |position| {
            let name: TokenStream = format!("P{position}").into();
            name
        };
        for (position, param) in self.generic_params(params) {
            match param.kind {
                SignatureParamKind::TryInto => {
                    let name: TokenStream = gen_name(position);
                    let into = self.type_name(&param.ty);
                    tokens.combine(&quote! { #name: ::windows_core::TryIntoParam<#into>, });
                }
                SignatureParamKind::IntoParam => {
                    let name: TokenStream = gen_name(position);
                    let into = self.type_name(&param.ty);
                    tokens.combine(&quote! { #name: ::windows_core::IntoParam<#into>, });
                }
                _ => {}
            }
        }
        tokens
    }

    //
    // Cfg
    //

    /// Generates doc comments for types, free functions, and constants.
    pub(crate) fn cfg_doc(&self, cfg: &Cfg) -> TokenStream {
        if !self.package {
            quote! {}
        } else {
            let mut tokens = format!(r#"`\"{}\"`"#, to_feature(self.namespace));
            let features = self.cfg_features_imp(cfg, self.namespace);

            for features in features {
                write!(tokens, r#", `\"{}\"`"#, to_feature(features)).unwrap();
            }

            if cfg.implement {
                tokens.push_str(r#", `\"implement\"`"#)
            }

            format!(r#" #[doc = "*Required features: {tokens}*"]"#).into()
        }
    }

    /// Generates doc comments for member functions (methods) and avoids redundantly declaring the
    /// enclosing module feature required by the method's type.
    pub(crate) fn cfg_method_doc(&self, cfg: &Cfg) -> TokenStream {
        if !self.package {
            quote! {}
        } else {
            let features = self.cfg_features_imp(cfg, self.namespace);
            if features.is_empty() {
                quote! {}
            } else {
                let mut tokens = String::new();
                for features in features {
                    write!(tokens, r#"`\"{}\"`, "#, to_feature(features)).unwrap();
                }
                tokens.truncate(tokens.len() - 2);
                format!(r#"#[doc = "*Required features: {tokens}*"]"#).into()
            }
        }
    }

    pub(crate) fn cfg_features(&self, cfg: &Cfg) -> TokenStream {
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

    fn cfg_features_imp(&self, cfg: &'a Cfg, namespace: &'a str) -> Vec<&'a str> {
        let mut compact = Vec::<&'static str>::new();
        if self.package {
            for feature in cfg.types.keys() {
                if !feature.is_empty() && !starts_with(namespace, feature) {
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

    fn cfg_not_features(&self, cfg: &Cfg) -> TokenStream {
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
                tokens.push_str("::windows::");
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
                "::windows_sys::core::".into()
            }
        } else {
            "::windows_core::".into()
        }
    }
    fn scoped_name(&self, def: TypeDef) -> String {
        if let Some(enclosing_type) = self.reader.type_def_enclosing_type(def) {
            for (index, nested_type) in self.reader.nested_types(enclosing_type).enumerate() {
                if self.reader.type_def_name(nested_type) == self.reader.type_def_name(def) {
                    return format!("{}_{index}", &self.scoped_name(enclosing_type));
                }
            }
        }
        self.reader.type_def_name(def).to_string()
    }
    pub fn value(&self, value: &Value) -> TokenStream {
        match value {
            Value::Bool(value) => quote! { #value },
            Value::U8(value) => quote! { #value },
            Value::I8(value) => quote! { #value },
            Value::U16(value) => quote! { #value },
            Value::I16(value) => quote! { #value },
            Value::U32(value) => quote! { #value },
            Value::I32(value) => quote! { #value },
            Value::U64(value) => quote! { #value },
            Value::I64(value) => quote! { #value },
            Value::F32(value) => quote! { #value },
            Value::F64(value) => quote! { #value },
            Value::String(value) => {
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
    pub fn typed_value(&self, value: &Value) -> TokenStream {
        let literal = self.value(value);
        match value {
            Value::Bool(_) => quote! { bool = #literal },
            Value::U8(_) => quote! { u8 = #literal },
            Value::I8(_) => quote! { i8 = #literal },
            Value::U16(_) => quote! { u16 = #literal },
            Value::I16(_) => quote! { i16 = #literal },
            Value::U32(_) => quote! { u32 = #literal },
            Value::I32(_) => quote! { i32 = #literal },
            Value::U64(_) => quote! { u64 = #literal },
            Value::I64(_) => quote! { i64 = #literal },
            Value::F32(_) => quote! { f32 = #literal },
            Value::F64(_) => quote! { f64 = #literal },
            Value::String(_) => {
                quote! { &str = #literal }
            }
            rest => unimplemented!("{rest:?}"),
        }
    }

    pub fn guid(&self, value: &GUID) -> TokenStream {
        let guid = self.type_name(&Type::GUID);
        format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
    }
    pub fn interface_core_traits(&self, def: TypeDef, _generics: &[Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        let name = crate::trim_tick(self.reader.type_def_name(def));
        quote! {
            #features
            impl<#constraints> ::core::cmp::PartialEq for #ident {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            #features
            impl<#constraints> ::core::cmp::Eq for #ident {}
            #features
            impl<#constraints> ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name).field(&self.0).finish()
                }
            }
        }
    }
    pub fn agile(&self, def: TypeDef, ident: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        if type_def_is_agile(self.reader, def) {
            quote! {
                #features
                unsafe impl<#constraints> ::core::marker::Send for #ident {}
                #features
                unsafe impl<#constraints> ::core::marker::Sync for #ident {}
            }
        } else {
            quote! {}
        }
    }
    pub fn async_get(&self, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        let mut kind = type_def_async_kind(self.reader, def);
        let mut async_generics = generics.to_vec();

        if kind == AsyncKind::None {
            for interface in type_def_interfaces(self.reader, def, generics) {
                if let Type::TypeDef(interface_def, interface_generics) = &interface {
                    kind = type_def_async_kind(self.reader, *interface_def);
                    if kind != AsyncKind::None {
                        async_generics = interface_generics.to_vec();
                        break;
                    }
                }
            }
        }

        if kind == AsyncKind::None {
            quote! {}
        } else {
            let return_type = match kind {
                AsyncKind::Operation | AsyncKind::OperationWithProgress => self.type_name(&async_generics[0]),
                _ => quote! { () },
            };

            let handler = match kind {
                AsyncKind::Action => quote! { AsyncActionCompletedHandler },
                AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
                AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
                AsyncKind::OperationWithProgress => {
                    quote! { AsyncOperationWithProgressCompletedHandler }
                }
                rest => unimplemented!("{rest:?}"),
            };

            let namespace = self.namespace("Windows.Foundation");

            quote! {
                #features
                impl<#constraints> #ident {
                    pub fn get(&self) -> ::windows_core::Result<#return_type> {
                        if self.Status()? == #namespace AsyncStatus::Started {
                            let (_waiter, signaler) = ::windows_core::imp::Waiter::new()?;
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
                impl<#constraints> ::std::future::Future for #ident {
                    type Output = ::windows_core::Result<#return_type>;

                    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
                        if self.Status()? == #namespace AsyncStatus::Started {
                            let waker = context.waker().clone();

                            let _ = self.SetCompleted(&#namespace #handler::new(move |_sender, _args| {
                                waker.wake_by_ref();
                                Ok(())
                            }));

                            ::std::task::Poll::Pending
                        } else {
                            ::std::task::Poll::Ready(self.GetResults())
                        }
                    }
                }
            }
        }
    }
    pub fn interface_winrt_trait(&self, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        if self.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            let type_signature = if self.reader.type_def_kind(def) == TypeKind::Class {
                let type_signature = Literal::byte_string(type_def_signature(self.reader, def, generics).as_bytes());
                quote! { ::windows_core::imp::ConstBuffer::from_slice(#type_signature) }
            } else {
                let signature = Literal::byte_string(
                    // TODO: workaround for riddle winmd generation (no attribute support)
                    if let Some(guid) = type_def_guid(self.reader, def) { format!("{{{:#?}}}", guid) } else { "TODO".to_string() }.as_bytes(),
                );

                if generics.is_empty() {
                    quote! { ::windows_core::imp::ConstBuffer::from_slice(#signature) }
                } else {
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
                            .push_other(<#g as ::windows_core::RuntimeType>::SIGNATURE)
                            #semi
                        }
                    });

                    quote! {
                        {
                            ::windows_core::imp::ConstBuffer::new()
                            .push_slice(b"pinterface(")
                            .push_slice(#signature)
                            .push_slice(b";")
                            #(#generics)*
                            .push_slice(b")")
                        }
                    }
                }
            };

            quote! {
                #features
                impl<#constraints> ::windows_core::RuntimeType for #ident {
                    const SIGNATURE: ::windows_core::imp::ConstBuffer = #type_signature;
                }
            }
        } else {
            quote! {}
        }
    }
    pub fn runtime_name_trait(&self, def: TypeDef, _generics: &[Type], name: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        if self.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            // TODO: this needs to use a ConstBuffer-like facility to accomodate generics
            let runtime_name = format!("{}", self.reader.type_def_type_name(def));

            quote! {
                #features
                impl<#constraints> ::windows_core::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
            }
        } else {
            quote! {
                #features
                impl ::windows_core::RuntimeName for #name {}
            }
        }
    }

    pub fn interface_trait(&self, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, features: &TokenStream, has_unknown_base: bool) -> TokenStream {
        if let Some(default) = type_def_default_interface(self.reader, def) {
            let default_name = self.type_name(&default);
            let vtbl = self.type_vtbl_name(&default);
            quote! {
                #features
                impl<#constraints> ::core::clone::Clone for #ident {
                    fn clone(&self) -> Self {
                        Self(self.0.clone())
                    }
                }
                #features
                unsafe impl ::windows_core::Interface for #ident {
                    type Vtable = #vtbl;
                }
                #features
                unsafe impl ::windows_core::ComInterface for #ident {
                    const IID: ::windows_core::GUID = <#default_name as ::windows_core::ComInterface>::IID;
                }
            }
        } else {
            let vtbl = self.type_def_vtbl_name(def, generics);
            let guid = if generics.is_empty() {
                match type_def_guid(self.reader, def) {
                    Some(guid) => self.guid(&guid),
                    None => {
                        quote! {
                            ::windows_core::GUID::zeroed()
                        }
                    }
                }
            } else {
                quote! {
                    ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE)
                }
            };

            let phantoms = self.generic_phantoms(generics);

            let mut tokens = quote! {
                #features
                unsafe impl<#constraints> ::windows_core::Interface for #ident {
                    type Vtable = #vtbl;
                }
                #features
                impl<#constraints> ::core::clone::Clone for #ident {
                    fn clone(&self) -> Self {
                        Self(self.0.clone(), #phantoms)
                    }
                }
            };

            if has_unknown_base {
                tokens.combine(&quote! {
                    #features
                    unsafe impl<#constraints> ::windows_core::ComInterface for #ident {
                        const IID: ::windows_core::GUID = #guid;
                    }
                });
            }

            tokens
        }
    }
    pub fn interface_vtbl(&self, def: TypeDef, generics: &[Type], _ident: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        let vtbl = self.type_def_vtbl_name(def, generics);
        let mut methods = quote! {};
        let mut method_names = MethodNames::new();
        method_names.add_vtable_types(self, def);
        let phantoms = self.generic_named_phantoms(generics);

        match type_def_vtables(self.reader, def).last() {
            Some(Type::IUnknown) => methods.combine(&quote! { pub base__: ::windows_core::IUnknown_Vtbl, }),
            Some(Type::IInspectable) => methods.combine(&quote! { pub base__: ::windows_core::IInspectable_Vtbl, }),
            Some(Type::TypeDef(def, _)) => {
                let vtbl = self.type_def_vtbl_name(*def, &[]);
                methods.combine(&quote! { pub base__: #vtbl, });
            }
            _ => {}
        }

        for method in self.reader.type_def_methods(def) {
            if self.reader.method_def_name(method) == ".ctor" {
                continue;
            }
            let name = method_names.add(self, method);
            let signature = method_def_signature(self.reader, self.reader.type_def_namespace(def), method, generics);
            let mut cfg = signature_cfg(self.reader, method);
            let signature = self.vtbl_signature(def, generics, &signature);
            cfg.add_feature(self.reader.type_def_namespace(def));
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
            #[repr(C)] #[doc(hidden)] pub struct #vtbl where #constraints {
                #methods
                #(pub #phantoms)*
            }
        }
    }
    pub fn vtbl_signature(&self, def: TypeDef, _generics: &[Type], signature: &Signature) -> TokenStream {
        let is_winrt = self.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime);

        let crate_name = self.crate_name();

        let (trailing_return_type, return_type, udt_return_type) = match &signature.return_type {
            Type::Void if is_winrt => (quote! {}, quote! { -> #crate_name HRESULT }, quote! {}),
            Type::Void => (quote! {}, quote! {}, quote! {}),
            Type::WinrtArray(kind) => {
                let tokens = self.type_abi_name(kind);
                (quote! { result_size__: *mut u32, result__: *mut *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
            }
            _ if is_winrt => {
                let tokens = self.type_abi_name(&signature.return_type);
                (quote! { result__: *mut #tokens }, quote! { -> #crate_name HRESULT }, quote! {})
            }
            _ if type_is_struct(self.reader, &signature.return_type) => {
                let tokens = self.type_abi_name(&signature.return_type);
                (quote! {}, quote! {}, quote! { result__: *mut #tokens, })
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
                let abi_size_name: TokenStream = format!("{}_array_size", self.reader.param_name(p.def)).into();

                if self.reader.param_flags(p.def).contains(ParamAttributes::In) {
                    if p.ty.is_winrt_array() {
                        quote! { #abi_size_name: u32, #name: *const #abi, }
                    } else if p.ty.is_const_ref() {
                        quote! { #name: &#abi, }
                    } else {
                        quote! { #name: #abi, }
                    }
                } else if p.ty.is_winrt_array() {
                    quote! { #abi_size_name: u32, #name: *mut #abi, }
                } else if p.ty.is_winrt_array_ref() {
                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi, }
                } else {
                    quote! { #name: *mut #abi, }
                }
            } else {
                match p.kind {
                    SignatureParamKind::ValueType => {
                        let abi = self.type_default_name(&p.ty);
                        quote! { #name: #abi, }
                    }
                    _ => {
                        let abi = self.type_abi_name(&p.ty);
                        quote! { #name: #abi, }
                    }
                }
            }
        });

        quote! { (this: *mut ::core::ffi::c_void, #udt_return_type #(#params)* #trailing_return_type) #return_type }
    }
    pub fn param_name(&self, param: Param) -> TokenStream {
        // In Rust, function parameters cannot be named the same as structs. This avoids some collisions that occur in the win32 metadata.
        // See Icmp6SendEcho2 for an example.
        to_ident(&self.reader.param_name(param).to_lowercase())
    }
    pub fn return_sig(&self, signature: &Signature) -> TokenStream {
        match &signature.return_type {
            Type::Void if self.reader.has_attribute(signature.def, "DoesNotReturnAttribute") => " -> !".into(),
            Type::Void => " -> ()".into(),
            _ => {
                let tokens = self.type_default_name(&signature.return_type);
                format!(" -> {}", tokens.as_str()).into()
            }
        }
    }
    pub fn win32_args(&self, params: &[SignatureParam], kind: SignatureKind) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in params.iter().enumerate() {
            let new = match kind {
                SignatureKind::Query(query) if query.object == position => {
                    quote! { &mut result__, }
                }
                SignatureKind::ReturnValue | SignatureKind::ResultValue if params.len() - 1 == position => {
                    quote! { &mut result__, }
                }
                SignatureKind::QueryOptional(query) if query.object == position => {
                    quote! { result__ as *mut _ as *mut _, }
                }
                SignatureKind::Query(query) | SignatureKind::QueryOptional(query) if query.guid == position => {
                    quote! { &<T as ::windows_core::ComInterface>::IID, }
                }
                _ => {
                    let name = self.param_name(param.def);
                    let flags = self.reader.param_flags(param.def);
                    match param.kind {
                        SignatureParamKind::ArrayFixed(_) | SignatureParamKind::ArrayRelativeLen(_) | SignatureParamKind::ArrayRelativeByteLen(_) => {
                            let map = if flags.contains(ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(::core::ptr::null(), |slice|slice.as_ptr()) }
                            } else {
                                quote! { #name.as_ptr() }
                            };
                            quote! { ::core::mem::transmute(#map), }
                        }
                        SignatureParamKind::ArrayRelativePtr(relative) => {
                            let name = self.param_name(params[relative].def);
                            let flags = self.reader.param_flags(params[relative].def);
                            if flags.contains(ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(0, |slice|slice.len() as _), }
                            } else {
                                quote! { #name.len() as _, }
                            }
                        }
                        SignatureParamKind::TryInto => {
                            quote! { #name.try_into_param()?.abi(), }
                        }
                        SignatureParamKind::IntoParam => {
                            quote! { #name.into_param().abi(), }
                        }
                        SignatureParamKind::OptionalPointer => {
                            if flags.contains(ParamAttributes::Out) {
                                quote! { ::core::mem::transmute(#name.unwrap_or(::std::ptr::null_mut())), }
                            } else {
                                quote! { ::core::mem::transmute(#name.unwrap_or(::std::ptr::null())), }
                            }
                        }
                        SignatureParamKind::ValueType => {
                            quote! { #name, }
                        }
                        SignatureParamKind::Blittable => {
                            if matches!(param.ty, Type::PrimitiveOrEnum(_, _)) {
                                quote! { #name.0 as _, }
                            } else {
                                quote! { ::core::mem::transmute(#name), }
                            }
                        }
                        SignatureParamKind::Other => {
                            quote! { ::core::mem::transmute_copy(#name), }
                        }
                    }
                }
            };
            tokens.combine(&new)
        }

        tokens
    }
    pub fn win32_params(&self, params: &[SignatureParam], kind: SignatureKind) -> TokenStream {
        let mut tokens = quote! {};

        let mut generic_params = self.generic_params(params);
        for (position, param) in params.iter().enumerate() {
            match kind {
                SignatureKind::Query(query) | SignatureKind::QueryOptional(query) => {
                    if query.object == position || query.guid == position {
                        continue;
                    }
                }
                SignatureKind::ReturnValue | SignatureKind::ResultValue if params.len() - 1 == position => {
                    continue;
                }
                _ => {}
            }

            let name = self.param_name(param.def);

            match param.kind {
                SignatureParamKind::ArrayFixed(fixed) => {
                    let ty = param.ty.deref();
                    let ty = self.type_default_name(&ty);
                    let len = Literal::u32_unsuffixed(fixed as u32);
                    let ty = if self.reader.param_flags(param.def).contains(ParamAttributes::Out) {
                        quote! { &mut [#ty; #len] }
                    } else {
                        quote! { &[#ty; #len] }
                    };
                    if self.reader.param_flags(param.def).contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: ::core::option::Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                SignatureParamKind::ArrayRelativeLen(_) => {
                    let ty = param.ty.deref();
                    let ty = self.type_default_name(&ty);
                    let ty = if self.reader.param_flags(param.def).contains(ParamAttributes::Out) {
                        quote! { &mut [#ty] }
                    } else {
                        quote! { &[#ty] }
                    };
                    if self.reader.param_flags(param.def).contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: ::core::option::Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                SignatureParamKind::ArrayRelativeByteLen(_) => {
                    let ty = if self.reader.param_flags(param.def).contains(ParamAttributes::Out) {
                        quote! { &mut [u8] }
                    } else {
                        quote! { &[u8] }
                    };
                    if self.reader.param_flags(param.def).contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: ::core::option::Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                SignatureParamKind::ArrayRelativePtr(_) => {}
                SignatureParamKind::TryInto | SignatureParamKind::IntoParam => {
                    let (position, _) = generic_params.next().unwrap();
                    let kind: TokenStream = format!("P{position}").into();
                    tokens.combine(&quote! { #name: #kind, });
                }
                SignatureParamKind::OptionalPointer => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: ::core::option::Option<#kind>, });
                }
                SignatureParamKind::ValueType | SignatureParamKind::Blittable => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: #kind, });
                }
                SignatureParamKind::Other => {
                    let kind = self.type_default_name(&param.ty);
                    tokens.combine(&quote! { #name: &#kind, });
                }
            }
        }

        tokens
    }

    pub fn impl_signature(&self, def: TypeDef, signature: &Signature) -> TokenStream {
        if self.reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            let is_delegate = self.reader.type_def_kind(def) == TypeKind::Delegate;
            let params = signature.params.iter().map(|p| self.winrt_produce_type(p, !is_delegate));

            let return_type = match &signature.return_type {
                Type::Void => quote! { () },
                _ => {
                    let tokens = self.type_name(&signature.return_type);

                    if signature.return_type.is_winrt_array() {
                        quote! { ::windows_core::Array<#tokens> }
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

            quote! { (#this #(#params),*) -> ::windows_core::Result<#return_type> }
        } else {
            let signature_kind = signature_kind(self.reader, signature);
            let mut params = quote! {};

            if signature_kind == SignatureKind::ResultValue {
                for param in &signature.params[..signature.params.len() - 1] {
                    params.combine(&self.win32_produce_type(param));
                }
            } else {
                for param in &signature.params {
                    params.combine(&self.win32_produce_type(param));
                }
            }

            let return_type = match signature_kind {
                SignatureKind::ReturnVoid => quote! {},
                SignatureKind::Query(_) | SignatureKind::QueryOptional(_) | SignatureKind::ResultVoid => quote! { -> ::windows_core::Result<()> },
                SignatureKind::ResultValue => {
                    let return_type = signature.params[signature.params.len() - 1].ty.deref();
                    let return_type = self.type_name(&return_type);

                    quote! { -> ::windows_core::Result<#return_type> }
                }
                _ => self.return_sig(signature),
            };

            quote! { (&self, #params) #return_type }
        }
    }
    fn winrt_produce_type(&self, param: &SignatureParam, include_param_names: bool) -> TokenStream {
        let default_type = self.type_default_name(&param.ty);

        let sig = if self.reader.param_flags(param.def).contains(ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                quote! { &[#default_type] }
            } else if type_is_primitive(self.reader, &param.ty) {
                quote! { #default_type }
            } else if type_is_nullable(self.reader, &param.ty) {
                let type_name = self.type_name(&param.ty);
                quote! { ::core::option::Option<&#type_name> }
            } else {
                quote! { &#default_type }
            }
        } else if param.ty.is_winrt_array() {
            quote! { &mut [#default_type] }
        } else if param.ty.is_winrt_array_ref() {
            let kind = self.type_name(&param.ty);
            quote! { &mut ::windows_core::Array<#kind> }
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
    fn win32_produce_type(&self, param: &SignatureParam) -> TokenStream {
        let name = self.param_name(param.def);
        let kind = self.type_default_name(&param.ty);

        if self.reader.param_flags(param.def).contains(ParamAttributes::In) {
            if type_is_primitive(self.reader, &param.ty) {
                quote! { #name: #kind, }
            } else if type_is_nullable(self.reader, &param.ty) {
                let kind = self.type_name(&param.ty);
                quote! { #name: ::core::option::Option<&#kind>, }
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

fn gen_mut_ptrs(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn gen_const_ptrs(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
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
