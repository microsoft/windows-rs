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
            namespace:"",
            sys: false,
            flatten: false,
            cfg:false,
            doc:false,
            min_enum:false,
            min_inherit:false,
            min_xaml: false,
            windows_extern:false,
            component:false,
        }
    }

    //
    // Definitions
    //

    pub(crate) fn define(&self, def: TypeDef) -> TokenStream {
        match self.reader.type_def_kind(def) {
            TypeKind::Class => self.define_class(def),
            TypeKind::Interface => self.define_interface(def),
            TypeKind::Enum => enums::gen(def,self),
            TypeKind::Struct => structs::gen(def,self),
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
    fn define_interface(&self, _def: TypeDef) -> TokenStream {
        " ".into()
    }
    fn define_delegate(&self, _def: TypeDef) -> TokenStream {
        " ".into()
    }

    //
    // TypeDef
    //

    pub fn type_def_name(&self, def: TypeDef, generics: &[Type]) -> TokenStream {
        let type_name = self.reader.type_def_type_name(def);

        if type_name.namespace.is_empty() {
            to_ident(&self.scoped_name(def))
        } else {
            let mut namespace = self.namespace(type_name.namespace);
            let name = to_ident(type_name.name);
    
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
    pub fn type_def_generics(&self, def: TypeDef) -> Vec<TokenStream> {
        self.reader.type_def_generic_params(def).map(|param|self.reader.generic_param_name(param)).map(|name|to_ident(name)).collect()
    }
    pub fn phantoms(&self, generics: &[TokenStream]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            tokens.combine(&quote! { ::core::marker::PhantomData::<#generic>, });
        }
        tokens
    }
    pub fn constraints(&self, generics: &[TokenStream]) -> TokenStream {
        let mut tokens = TokenStream::new();
        for generic in generics {
            tokens.combine(&quote! { #generic: ::windows::core::RuntimeType + 'static, });
        }
        tokens
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

    pub(crate)fn type_name(&self, ty:&Type) -> TokenStream {
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

pub fn cfg_features<'a>(cfg:&'a Cfg, namespace: &'a str) -> Vec<&'a str> {
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
