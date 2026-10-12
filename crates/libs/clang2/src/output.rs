#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;

pub(super) struct Closed {
    declarations: Vec<Entry>,
}

struct Entry {
    namespace: String,
    name: String,
    kind: Kind,
}

enum Kind {
    Alias(metadata::Type),
    Opaque { union: bool },
}

impl Closed {
    pub fn new(snapshot: &Snapshot, namespace: &str) -> Result<Self> {
        if namespace.split('.').count() > 64 || namespace.split('.').any(|name| !identifier(name)) {
            return Err(Error::new(
                Stage::Representation,
                "invalid output namespace",
            ));
        }
        let names = snapshot
            .declarations
            .iter()
            .map(|decl| {
                if decl.names.iter().any(|name| !identifier(name)) {
                    return Err(Error::new(
                        Stage::Representation,
                        "unsupported native publication name",
                    ));
                }
                let mut ns = namespace.to_string();
                for name in &decl.names[..decl.names.len() - 1] {
                    ns.push('.');
                    ns.push_str(name);
                }
                let name = metadata::TypeName::named(&ns, decl.names.last().unwrap());
                if &name == ("System", "Guid") || &name == ("Windows.Foundation", "HResult") {
                    return Err(Error::new(
                        Stage::Representation,
                        "publication collides with a predefined metadata identity",
                    ));
                }
                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;
        let mut declarations = BTreeMap::new();
        for (id, decl) in snapshot.declarations.iter().enumerate() {
            let name = &names[id];
            let kind = match &decl.kind {
                DeclarationKind::Alias { written, .. } => {
                    if let NativeKind::Record(target) = written.kind
                        && written.qualifiers == Qualifiers::default()
                        && &names[target] == name
                    {
                        continue;
                    }
                    Kind::Alias(lower(written, &names, snapshot.target)?)
                }
                DeclarationKind::Opaque { union } => Kind::Opaque { union: *union },
            };
            let key = (name.namespace.clone(), name.name.clone());
            if declarations
                .insert(
                    key,
                    Entry {
                        namespace: name.namespace.clone(),
                        name: name.name.clone(),
                        kind,
                    },
                )
                .is_some()
            {
                return Err(Error::new(
                    Stage::Representation,
                    format!("publication collision for {}.{}", name.namespace, name.name),
                ));
            }
        }
        Ok(Self {
            declarations: declarations.into_values().collect(),
        })
    }

    pub fn rdl(&self) -> Result<String> {
        #[derive(Default)]
        struct Namespace {
            items: Vec<String>,
            children: BTreeMap<String, Self>,
        }
        impl Namespace {
            fn render(&self, top: bool) -> String {
                let mut result = self.items.join("\n");
                for (name, child) in &self.children {
                    let name = windows_rdl::emit::write_ident(name);
                    result.push_str(&format!(
                        "{}mod {name} {{ {} }}\n",
                        if top { "#[win32] " } else { "" },
                        child.render(false)
                    ));
                }
                result
            }
        }
        let mut root = Namespace::default();
        for decl in &self.declarations {
            let mut node = &mut root;
            for part in decl.namespace.split('.') {
                node = node.children.entry(part.to_string()).or_default();
            }
            let name = windows_rdl::emit::write_ident(&decl.name);
            node.items.push(match &decl.kind {
                Kind::Alias(ty) => {
                    let ty = windows_rdl::emit::write_type("", ty);
                    format!("type {name} = {ty};")
                }
                Kind::Opaque { union } => format!(
                    "#[opaque] {} {name} {{}}",
                    if *union { "union" } else { "struct" }
                ),
            });
        }
        Ok(windows_rdl::formatter::format(&root.render(true)))
    }
}

fn identifier(name: &str) -> bool {
    if matches!(name, "self" | "Self" | "_") {
        return false;
    }
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn lower(ty: &NativeType, names: &[metadata::TypeName], target: Target) -> Result<metadata::Type> {
    use metadata::Type;
    if ty.qualifiers.restrict {
        return Err(Error::new(
            Stage::Representation,
            "restrict qualification requires a C7 contract",
        ));
    }
    let mut result = match &ty.kind {
        NativeKind::Void => Type::Void,
        NativeKind::Scalar(kind) => {
            let (result, size) = match *kind {
                CXType_Bool => (Type::Bool, 1),
                CXType_Char_S | CXType_SChar => (Type::I8, 1),
                CXType_Char_U | CXType_UChar => (Type::U8, 1),
                CXType_Short => (Type::I16, 2),
                CXType_UShort | CXType_WChar | CXType_Char16 => (Type::U16, 2),
                CXType_Int | CXType_Long => (Type::I32, 4),
                CXType_UInt | CXType_ULong | CXType_Char32 => (Type::U32, 4),
                CXType_LongLong => (Type::I64, 8),
                CXType_ULongLong => (Type::U64, 8),
                CXType_Float => (Type::F32, 4),
                CXType_Double | CXType_LongDouble => (Type::F64, 8),
                _ => {
                    return Err(Error::new(
                        Stage::Representation,
                        "unsupported native scalar",
                    ));
                }
            };
            if ty
                .layout
                .is_none_or(|layout| layout.size != size || layout.alignment != size)
            {
                return Err(Error::new(
                    Stage::Representation,
                    "native scalar disagrees with Windows target contract",
                ));
            }
            result
        }
        NativeKind::Alias(id) | NativeKind::Record(id) => Type::ValueName(names[*id].clone()),
        NativeKind::Pointer(element) | NativeKind::Reference(element) => {
            if matches!(ty.kind, NativeKind::Pointer(_))
                && ty.layout.is_none_or(|layout| {
                    layout.size != u64::from(target.pointer_bits() / 8)
                        || layout.alignment != layout.size
                })
            {
                return Err(Error::new(
                    Stage::Representation,
                    "native pointer size disagrees with target",
                ));
            }
            let mut element = element.as_ref().clone();
            let constant = element.qualifiers.constant;
            element.qualifiers.constant = false;
            let element = Box::new(lower(&element, names, target)?);
            match (&ty.kind, constant) {
                (NativeKind::Pointer(_), false) => Type::PtrMut(element, 1),
                (NativeKind::Pointer(_), true) => Type::PtrConst(element, 1),
                (_, false) => Type::RefMut(element),
                (_, true) => Type::RefConst(element),
            }
        }
        NativeKind::Array(element, count) => {
            Type::ArrayFixed(Box::new(lower(element, names, target)?), *count)
        }
    };
    if ty.qualifiers.volatile {
        result = Type::Volatile(Box::new(result));
    }
    if ty.qualifiers.constant {
        result = Type::Const(Box::new(result));
    }
    Ok(result)
}
