use super::*;

pub struct Gen<'a> {
    pub reader: &'a Reader<'a>,
    pub namespace: &'a str,
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
    pub doc: bool,
    pub min_enum: bool,
    pub min_inherit: bool,
    pub min_xaml: bool,
    pub windows_extern: bool,
    pub component: bool,
}

impl<'a> Gen<'a> {
    pub fn new(reader: &'a Reader) -> Self {
        Self {
            reader,
            namespace: "",
            sys: false,
            flatten: false,
            cfg: false,
            doc: false,
            min_enum: false,
            min_inherit: false,
            min_xaml: false,
            windows_extern: false,
            component: false,
        }
    }

    //
    // Definitions
    //

    pub(crate) fn define(&self, def: TypeDef) -> TokenStream {
        match self.reader.type_def_kind(def) {
            TypeKind::Class => self.define_class(def),
            TypeKind::Interface => interfaces::gen(self, def),
            TypeKind::Enum => enums::gen(self, def),
            TypeKind::Struct => structs::gen(self, def),
            TypeKind::Delegate => self.define_delegate(def),
        }
    }
    pub(crate) fn define_function(&self, _def: MethodDef) -> TokenStream {
        " ".into()
    }
    pub(crate) fn define_constant(&self, _def: Field) -> TokenStream {
        " ".into()
    }
    fn define_class(&self, _def: TypeDef) -> TokenStream {
        " ".into()
    }
    fn define_delegate(&self, _def: TypeDef) -> TokenStream {
        " ".into()
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

            if self.reader.type_is_generic(ty) {
                quote! { <#kind as ::windows::core::RuntimeType>::DefaultType }
            } else if self.reader.type_is_nullable(ty) && !self.sys {
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
                quote! { ::#crate_name::core::HSTRING }
            }
            Type::IInspectable => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::IInspectable }
            }
            Type::GUID => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::GUID }
            }
            Type::IUnknown => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::IUnknown }
            }
            Type::HRESULT => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::HRESULT }
            }
            Type::PSTR => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::PSTR }
            }
            Type::PWSTR => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::PWSTR }
            }
            Type::PCSTR => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::PCSTR }
            }
            Type::PCWSTR => {
                let crate_name = self.crate_name();
                quote! { ::#crate_name::core::PCWSTR }
            }
            Type::Win32Array((ty, len)) => {
                let name = self.type_default_name(ty);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Type::GenericParam(generic) => self.reader.generic_param_name(*generic).into(),
            //Type::MethodDef(def) => self.reader.method_def_name(def).into(),
            //Type::Field(field) => field.name().into(),
            Type::TypeDef((def, generics)) => self.type_def_name(*def, generics),
            Type::MutPtr((ty, pointers)) => {
                let pointers = mut_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            Type::ConstPtr((ty, pointers)) => {
                let pointers = const_ptrs(*pointers);
                let ty = self.type_default_name(ty);
                quote! { #pointers #ty }
            }
            Type::WinrtArray(ty) => self.type_name(ty),
            Type::WinrtArrayRef(ty) => self.type_name(ty),
            Type::WinrtConstRef(ty) => self.type_name(ty),
            _ => unimplemented!(),
        }
    }
    pub fn type_vtbl_name(&self, ty: &Type) -> TokenStream {
        match ty {
            Type::TypeDef((def, generics)) => self.type_def_vtbl_name(*def, generics),
            _ => unimplemented!(),
        }
    }
    pub fn type_abi_name(&self, ty: &Type) -> TokenStream {
        self.type_abi_name_imp(ty, false)
    }
    // TODO: this is only because we're trying to avoid the ManuallyDrop below - I don't think that matters so may want to scrap this once we have parity.
    fn type_abi_name_imp(&self, ty: &Type, ptr: bool) -> TokenStream {
        match ty {
            Type::String => {
                quote! { ::core::mem::ManuallyDrop<::windows::core::HSTRING> }
            }
            Type::IUnknown | Type::IInspectable => {
                quote! { *mut ::core::ffi::c_void }
            }
            Type::Win32Array((kind, len)) => {
                let name = self.type_abi_name_imp(kind, ptr);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Type::GenericParam(generic) => {
                let name = to_ident(self.reader.generic_param_name(*generic));
                quote! { <#name as ::windows::core::Abi>::Abi }
            }
            Type::TypeDef((def, _)) => match self.reader.type_def_kind(*def) {
                TypeKind::Enum => self.type_def_name(*def, &[]),
                TypeKind::Struct => {
                    let tokens = self.type_def_name(*def, &[]);
                    if self.reader.type_def_is_blittable(*def) || ptr {
                        tokens
                    } else {
                        quote! { ::core::mem::ManuallyDrop<#tokens> }
                    }
                }
                _ => quote! { ::windows::core::RawPtr },
            },
            Type::MutPtr((kind, pointers)) => {
                let pointers = gen_mut_ptrs(*pointers);
                let kind = self.type_abi_name_imp(kind, true);
                quote! { #pointers #kind }
            }
            Type::ConstPtr((kind, pointers)) => {
                let pointers = gen_const_ptrs(*pointers);
                let kind = self.type_abi_name_imp(kind, true);
                quote! { #pointers #kind }
            }
            Type::WinrtArray(kind) => self.type_abi_name_imp(kind, ptr),
            Type::WinrtArrayRef(kind) => self.type_abi_name_imp(kind, ptr),
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
            tokens.combine(&quote! { #generic: ::windows::core::RuntimeType + 'static, });
        }
        tokens
    }
    pub fn param_constraints(&self, params: &[SignatureParam]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for (position, param) in params.iter().enumerate() {
            if self.reader.signature_param_is_convertible(param) {
                let name: TokenStream = format!("Param{}", position).into();
                let into = self.type_name(&param.ty);
                tokens.combine(&quote! { #name: ::windows::core::IntoParam<'a, #into>, });
            }
        }
        if !tokens.is_empty() {
            quote! { 'a, #tokens }
        } else {
            tokens
        }
    }

    //
    // Cfg
    //

    pub(crate) fn cfg_doc(&self, cfg: &Cfg) -> TokenStream {
        if !self.doc {
            quote! {}
        } else {
            let mut tokens = format!(r#"`\"{}\"`"#, to_feature(self.namespace));

            let mut features = cfg_features(cfg, self.namespace);
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }
            for features in features {
                tokens.push_str(&format!(r#", `\"{}\"`"#, to_feature(features)));
            }

            format!(r#"#[doc = "*Required features: {}*"]"#, tokens).into()
        }
    }

    pub(crate) fn cfg_features(&self, cfg: &Cfg) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
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

            let mut features = cfg_features(cfg, self.namespace);
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }

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
    }

    fn cfg_not_features(&self, cfg: &Cfg) -> TokenStream {
        let mut features = cfg_features(cfg, self.namespace);
        if !self.cfg || features.is_empty() {
            quote! {}
        } else {
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }
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
    fn crate_name(&self) -> TokenStream {
        if self.sys {
            "windows_sys".into()
        } else {
            "windows".into()
        }
    }
    fn scoped_name(&self, def: TypeDef) -> String {
        if let Some(enclosing_type) = self.reader.type_def_enclosing_type(def) {
            for (index, nested_type) in self.reader.nested_types(enclosing_type).iter().enumerate() {
                if self.reader.type_def_name(*nested_type) == self.reader.type_def_name(def) {
                    return format!("{}_{}", &self.scoped_name(enclosing_type), index);
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
                    tokens.push_str(&format!("{}", u.escape_default()));
                }

                tokens.push('\"');
                tokens.into()
            }
            _ => unimplemented!(),
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
            _ => unimplemented!(),
        }
    }

    pub fn guid(&self, value: &GUID) -> TokenStream {
        let guid = self.type_name(&Type::GUID);

        if self.sys {
            let a = Literal::u32_unsuffixed(value.0);
            let b = Literal::u16_unsuffixed(value.1);
            let c = Literal::u16_unsuffixed(value.2);
            let d = Literal::u8_unsuffixed(value.3);
            let e = Literal::u8_unsuffixed(value.4);
            let f = Literal::u8_unsuffixed(value.5);
            let g = Literal::u8_unsuffixed(value.6);
            let h = Literal::u8_unsuffixed(value.7);
            let i = Literal::u8_unsuffixed(value.8);
            let j = Literal::u8_unsuffixed(value.9);
            let k = Literal::u8_unsuffixed(value.10);

            quote! {
                #guid { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
            }
        } else {
            format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
        }
    }
    pub fn interface_core_traits(&self, def: TypeDef, _generics: &[Type], ident: &TokenStream, constraints: &TokenStream, phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        let name = trim_tick(self.reader.type_def_name(def));
        quote! {
            #features
            impl<#constraints> ::core::clone::Clone for #ident {
                fn clone(&self) -> Self {
                    Self(self.0.clone(), #phantoms)
                }
            }
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
    pub fn interface_winrt_trait(&self, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, features: &TokenStream) -> TokenStream {
        if self.reader.type_def_flags(def).winrt() {
            let type_signature = if self.reader.type_def_kind(def) == TypeKind::Class {
                let type_signature = Literal::byte_string(self.reader.type_def_signature(def, generics).as_bytes());
                quote! { ::windows::core::ConstBuffer::from_slice(#type_signature) }
            } else {
                let signature = Literal::byte_string(format!("{{{:#?}}}", self.reader.type_def_guid(def).unwrap()).as_bytes());

                if generics.is_empty() {
                    quote! { ::windows::core::ConstBuffer::from_slice(#signature) }
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
                            .push_other(<#g as ::windows::core::RuntimeType>::SIGNATURE)
                            #semi
                        }
                    });

                    quote! {
                        {
                            ::windows::core::ConstBuffer::new()
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
                unsafe impl<#constraints> ::windows::core::RuntimeType for #ident {
                    const SIGNATURE: ::windows::core::ConstBuffer = #type_signature;
                    type DefaultType = ::core::option::Option<Self>;
                    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
                        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
                    }
                }
            }
        } else {
            quote! {}
        }
    }
    pub fn interface_trait(&self, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        if let Some(default) = self.reader.type_def_default_interface(def) {
            let default_name = self.type_name(&default);
            let vtbl = self.type_vtbl_name(&default);
            quote! {
                #features
                unsafe impl ::windows::core::Interface for #ident {
                    type Vtable = #vtbl;
                    const IID: ::windows::core::GUID = <#default_name as ::windows::core::Interface>::IID;
                }
            }
        } else {
            let vtbl = self.type_def_vtbl_name(def, generics);
            let guid = if generics.is_empty() {
                match self.reader.type_def_guid(def) {
                    Some(guid) => self.guid(&guid),
                    None => {
                        quote! {
                            ::windows::core::GUID::zeroed()
                        }
                    }
                }
            } else {
                quote! {
                    ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE)
                }
            };
            quote! {
                #features
                unsafe impl<#constraints> ::windows::core::Interface for #ident {
                    type Vtable = #vtbl;
                    const IID: ::windows::core::GUID = #guid;
                }
            }
        }
    }
    pub fn interface_vtbl(&self, def: TypeDef, generics: &[Type], _ident: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
        let vtbl = self.type_def_vtbl_name(def, generics);
        let mut methods = quote! {};
        let mut method_names = MethodNames::new();
        method_names.add_vtable_types(self, def);
        let phantoms = self.generic_named_phantoms(generics);

        match self.reader.type_def_vtables(def).last() {
            Some(Type::IUnknown) => methods.combine(&quote! { pub base__: ::windows::core::IUnknownVtbl, }),
            Some(Type::IInspectable) => methods.combine(&quote! { pub base__: ::windows::core::IInspectableVtbl, }),
            Some(Type::TypeDef((def, _))) => {
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
            let signature = self.vtbl_signature(def, generics, method);
            let mut cfg = self.reader.method_def_cfg(method);
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
    pub fn vtbl_signature(&self, def: TypeDef, generics: &[Type], method: MethodDef) -> TokenStream {
        let is_winrt = self.reader.type_def_flags(def).winrt();
        let signature = self.reader.method_def_signature(method, generics);
        let hresult = self.type_name(&Type::HRESULT);

        let (trailing_return_type, return_type, udt_return_type) = if is_winrt {
            if let Some(return_type) = &signature.return_type {
                if let Type::WinrtArray(kind) = return_type {
                    let tokens = self.type_abi_name(kind);
                    (quote! { result_size__: *mut u32, result__: *mut *mut #tokens }, quote! { -> #hresult }, quote! {})
                } else {
                    let tokens = self.type_abi_name(return_type);
                    (quote! { result__: *mut #tokens }, quote! { -> #hresult }, quote! {})
                }
            } else {
                (quote! {}, quote! { -> #hresult }, quote! {})
            }
        } else if let Some(return_type) = &signature.return_type {
            if self.reader.type_is_udt(return_type) {
                let tokens = self.type_abi_name(return_type);
                (quote! {}, quote! {}, quote! { result__: *mut #tokens, })
            } else {
                let tokens = self.type_default_name(return_type);
                (quote! {}, quote! { -> #tokens }, quote! {})
            }
        } else {
            (quote! {}, quote! {}, quote! {})
        };

        let params = signature.params.iter().map(|p| {
            let name = self.param_name(p.def);
            if is_winrt {
                let abi = self.type_abi_name(&p.ty);
                let abi_size_name: TokenStream = format!("{}_array_size", self.reader.param_name(p.def)).into();

                if self.reader.param_flags(p.def).input() {
                    if self.reader.type_is_winrt_array(&p.ty) {
                        quote! { #abi_size_name: u32, #name: *const #abi, }
                    } else if self.reader.type_is_winrt_const_ref(&p.ty) {
                        quote! { #name: &#abi, }
                    } else {
                        quote! { #name: #abi, }
                    }
                } else if self.reader.type_is_winrt_array(&p.ty) {
                    quote! { #abi_size_name: u32, #name: *mut #abi, }
                } else if self.reader.type_is_winrt_array_ref(&p.ty) {
                    quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi, }
                } else {
                    quote! { #name: *mut #abi, }
                }
            } else {
                let abi = self.type_abi_name(&p.ty);
                quote! { #name: #abi, }
            }
        });

        quote! { (this: *mut ::core::ffi::c_void, #udt_return_type #(#params)* #trailing_return_type) #return_type }
    }
    pub fn param_name(&self, param: Param) -> TokenStream {
        self.reader.param_name(param).to_lowercase().into()
    }
}

pub fn to_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => format!("r#{}", name).into(),
        "Self" | "self" => format!("{}_", name).into(),
        "_" => "unused".into(),
        _ => trim_tick(name).into(),
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

pub fn cfg_features<'a>(cfg: &'a Cfg, namespace: &'a str) -> Vec<&'a str> {
    let mut compact = Vec::<&'static str>::new();
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
    compact
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
