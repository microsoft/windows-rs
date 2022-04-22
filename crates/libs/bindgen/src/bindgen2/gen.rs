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
            TypeKind::Enum => self.define_enum(def),
            TypeKind::Struct => self.define_struct(def),
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
    fn define_enum(&self, def: TypeDef) -> TokenStream {
        let type_name = self.reader.type_def_type_name(def);
        let ident = to_ident(type_name.name);
        let underlying_type = self.reader.type_def_underlying_type(def);
        let underlying_type = self.type_name(&underlying_type);
        let is_scoped = self.reader.type_def_is_scoped(def);
        let cfg = self.reader.type_def_cfg(def, &[]);
        let doc = self.cfg_doc(&cfg);
        let features = self.cfg_features(&cfg);

        let mut fields: Vec<(TokenStream, TokenStream)> = self.reader.type_def_fields(def)
        .filter_map(|field| {
            if self.reader.field_flags(field).literal() {
                let field_name = to_ident(self.reader.field_name(field));
                let constant = self.reader.field_constant(field).unwrap();
                let value = self.value(&self.reader.constant_value(constant));

                Some((field_name, value))
            } else {
                None
            }
        })
        .collect();

        if self.min_enum && fields.len() > 100 {
            fields.clear();
        }

        let eq = if self.sys {
            quote! {}
        } else {
            quote! {
                // Unfortunately, Rust requires these to be derived to allow constant patterns.
                #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
            }
        };

        let mut tokens = if is_scoped || !self.sys {
            quote! {
                #doc
                #features
                #[repr(transparent)]
                #eq
                pub struct #ident(pub #underlying_type);
            }
        } else {
            quote! {
                #doc
                #features
                pub type #ident = #underlying_type;
            }
        };
    
        tokens.combine(&if is_scoped {
            let fields = fields.iter().map(|(field_name, value)| {
                quote! {
                    pub const #field_name: Self = Self(#value);
                }
            });
    
            quote! {
                #features
                impl #ident {
                    #(#fields)*
                }
            }
        } else if !self.sys {
            let fields = fields.iter().map(|(field_name, value)| {
                quote! {
                    #doc
                    #features
                    pub const #field_name: #ident = #ident(#value);
                }
            });
    
            quote! {
                #(#fields)*
            }
        } else {
            let fields = fields.iter().map(|(field_name, value)| {
                quote! {
                    #doc
                    #features
                    pub const #field_name: #ident = #value;
                }
            });
    
            quote! {
                #(#fields)*
            }
        });
    
        if is_scoped || !self.sys {
            tokens.combine(&quote! {
                #features
                impl ::core::marker::Copy for #ident {}
                #features
                impl ::core::clone::Clone for #ident {
                    fn clone(&self) -> Self {
                        *self
                    }
                }
            });
        }
    
        if !self.sys {
            tokens.combine(&quote! {
                #features
                impl ::core::default::Default for #ident {
                    fn default() -> Self {
                        Self(0)
                    }
                }
            });
        }
    
        if !self.sys {
            let name = type_name.name;
            tokens.combine(&quote! {
                #features
                unsafe impl ::windows::core::Abi for #ident {
                    type Abi = Self;
                }
                #features
                impl ::core::fmt::Debug for #ident {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_tuple(#name).field(&self.0).finish()
                    }
                }
            });
    
            if self.reader.type_def_is_flags(def) {
                tokens.combine(&quote! {
                    #features
                    impl ::core::ops::BitOr for #ident {
                        type Output = Self;
    
                        fn bitor(self, other: Self) -> Self {
                            Self(self.0 | other.0)
                        }
                    }
                    #features
                    impl ::core::ops::BitAnd for #ident {
                        type Output = Self;
    
                        fn bitand(self, other: Self) -> Self {
                            Self(self.0 & other.0)
                        }
                    }
                    #features
                    impl ::core::ops::BitOrAssign for #ident {
                        fn bitor_assign(&mut self, other: Self) {
                            self.0.bitor_assign(other.0)
                        }
                    }
                    #features
                    impl ::core::ops::BitAndAssign for #ident {
                        fn bitand_assign(&mut self, other: Self) {
                            self.0.bitand_assign(other.0)
                        }
                    }
                    #features
                    impl ::core::ops::Not for #ident {
                        type Output = Self;
    
                        fn not(self) -> Self {
                            Self(self.0.not())
                        }
                    }
                });
            }
    
            if self.reader.type_def_flags(def).winrt() {
                let signature = Literal::byte_string(self.reader.type_def_signature(def, &[]).as_bytes());
    
                tokens.combine(&quote! {
                    #features
                    unsafe impl ::windows::core::RuntimeType for #ident {
                        const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
                        type DefaultType = Self;
                        fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
                            Ok(*from)
                        }
                    }
                });
            }
    
            tokens.combine(&extensions(type_name));
        }
    
        tokens
    }
    fn define_struct(&self, _def: TypeDef) -> TokenStream {
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

    fn type_name(&self, ty:&Type) -> TokenStream {
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

    fn cfg_features(&self, cfg: &Cfg) -> TokenStream {
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

    fn namespace(&self, namespace: &str) -> TokenStream {
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
            for (index, nested_type) in self.reader.nested_types(enclosing_type).enumerate() {
                if self.reader.type_def_name(nested_type) == self.reader.type_def_name(def) {
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
