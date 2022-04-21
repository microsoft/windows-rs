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
        quote!{function}
    }
    pub(crate) fn define_constant(&self, _def: Field) -> TokenStream {
        quote!{constant}
    }
    fn define_class(&self, _def: TypeDef) -> TokenStream {
        quote!{class}
    }
    fn define_interface(&self, _def: TypeDef) -> TokenStream {
        quote!{interface}
    }
    fn define_enum(&self, def: TypeDef) -> TokenStream {
        let name = self.reader.type_def_name(def);
        let ident = to_ident(name);


        quote!{enum}
    }
    fn define_struct(&self, _def: TypeDef) -> TokenStream {
        quote!{struct}
    }
    fn define_delegate(&self, _def: TypeDef) -> TokenStream {
        quote!{delegate}
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

// fn to_feature(name: &str) -> String {
//     let mut feature = String::new();

//     for name in name.split('.').skip(1) {
//         feature.push_str(name);
//         feature.push('_');
//     }

//     if feature.is_empty() {
//         feature = name.to_string();
//     } else {
//         feature.truncate(feature.len() - 1);
//     }

//     feature
// }

// pub fn cfg_features<'a>(cfg:&'a Cfg, namespace: &'a str) -> Vec<&'a str> {
//     let mut compact = Vec::<&'static str>::new();
//     for feature in cfg.types.keys() {
//         if !feature.is_empty() && !starts_with(namespace, feature) {
//             for pos in 0..compact.len() {
//                 if starts_with(feature, unsafe { compact.get_unchecked(pos) }) {
//                     compact.remove(pos);
//                     break;
//                 }
//             }
//             compact.push(feature);
//         }
//     }
//     compact
// }

// fn starts_with(namespace: &str, feature: &str) -> bool {
//     if namespace == feature {
//         return true;
//     }

//     if namespace.len() > feature.len() && namespace.as_bytes().get(feature.len()) == Some(&b'.') {
//         return namespace.starts_with(feature);
//     }

//     false
// }
