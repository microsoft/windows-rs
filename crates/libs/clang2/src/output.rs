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
    owner: Option<metadata::TypeName>,
}

enum Kind {
    Constant(metadata::Value),
    Alias(metadata::Type),
    Opaque {
        union: bool,
    },
    Record {
        union: bool,
        fields: Vec<Field>,
        contract: RecordLayoutContract,
    },
    Enum {
        underlying: metadata::Type,
        scoped: bool,
        members: Vec<(String, metadata::Value)>,
    },
    Function {
        import: Import,
        convention: CallingConvention,
        variadic: bool,
        result: metadata::Type,
        params: Vec<(String, metadata::Type)>,
    },
}

struct Field {
    name: String,
    anonymous: bool,
    kind: FieldKind,
}

enum FieldKind {
    Type(metadata::Type),
    Nested(Box<Kind>),
}

impl Closed {
    pub fn new(
        snapshot: &Snapshot,
        namespace: &str,
        imports: &BTreeMap<String, Import>,
    ) -> Result<Self> {
        snapshot.check_records()?;
        if namespace.split('.').count() > 64 || namespace.split('.').any(|name| !identifier(name)) {
            return Err(Error::new(
                Stage::Representation,
                "invalid output namespace",
            ));
        }
        let mut names = snapshot
            .declarations
            .iter()
            .map(|decl| {
                if decl.anonymous {
                    return Ok(None);
                }
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
                Ok(Some(name))
            })
            .collect::<Result<Vec<_>>>()?;
        for (id, decl) in snapshot.declarations.iter().enumerate() {
            if decl.anonymous && matches!(decl.kind, DeclarationKind::Enum { .. }) {
                let aliases = snapshot
                    .declarations
                    .iter()
                    .enumerate()
                    .filter_map(|(alias, decl)| {
                        if let DeclarationKind::Alias { written, .. } = &decl.kind
                            && written.kind == NativeKind::Enum(id)
                            && written.qualifiers == Qualifiers::default()
                        {
                            Some(alias)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let [alias] = aliases.as_slice() else {
                    return Err(Error::new(
                        Stage::Representation,
                        "unnamed enum requires one direct selected naming typedef",
                    ));
                };
                let name = names[*alias].clone();
                names[id] = name;
            }
        }
        let mut declarations = BTreeMap::new();
        let mut inline = BTreeSet::new();
        for (owner, decl) in snapshot.declarations.iter().enumerate() {
            if let DeclarationKind::Record { fields, .. } = &decl.kind {
                for field in fields {
                    if let NativeKind::Record(child) = field.written.kind
                        && snapshot.declarations[child].anonymous
                        && (snapshot.declarations[child].owner != Some(owner)
                            || field.written.qualifiers != Qualifiers::default()
                            || !inline.insert(child))
                    {
                        return Err(Error::new(
                            Stage::Representation,
                            "unnamed record must have one unqualified direct owning field",
                        ));
                    }
                }
            }
        }
        for (id, decl) in snapshot.declarations.iter().enumerate() {
            if decl.anonymous
                && !matches!(decl.kind, DeclarationKind::Enum { .. })
                && !inline.contains(&id)
            {
                return Err(Error::new(
                    Stage::Representation,
                    "unnamed record arrays, pointers, and aliases require a separate identity contract",
                ));
            }
        }
        for (id, decl) in snapshot.declarations.iter().enumerate() {
            let Some(name) = &names[id] else { continue };
            if decl.anonymous && matches!(decl.kind, DeclarationKind::Enum { .. }) {
                continue;
            }
            let kind = match &decl.kind {
                DeclarationKind::Alias { written, .. } => {
                    if let NativeKind::Enum(target) = written.kind
                        && written.qualifiers == Qualifiers::default()
                        && snapshot.declarations[target].anonymous
                    {
                        Self::enumeration(snapshot, target, &names)?
                    } else {
                        if let NativeKind::Record(target) | NativeKind::Enum(target) = written.kind
                            && written.qualifiers == Qualifiers::default()
                            && names[target].as_ref() == Some(name)
                        {
                            continue;
                        }
                        Kind::Alias(lower(written, &names, snapshot.target)?)
                    }
                }
                DeclarationKind::Opaque { union } => Kind::Opaque { union: *union },
                DeclarationKind::Record { .. } => Self::record(snapshot, id, &names)?,
                DeclarationKind::Enum { .. } => Self::enumeration(snapshot, id, &names)?,
                DeclarationKind::Function(observations) => {
                    let first = &observations[0];
                    let import = imports.get(&first.evidence.linker_name).ok_or_else(|| {
                        Error::new(
                            Stage::Representation,
                            format!(
                                "{}: missing import contract for native linker symbol `{}`",
                                decl.evidence.qualified_name, first.evidence.linker_name
                            ),
                        )
                    })?;
                    if import.dll.is_empty()
                        || import.export.is_empty()
                        || import.dll.contains('\0')
                        || import.export.contains('\0')
                    {
                        return Err(Error::new(
                            Stage::Representation,
                            "invalid named-export import contract",
                        ));
                    }
                    let mut chosen = (0..first.params.len())
                        .map(|ordinal| {
                            observations
                                .iter()
                                .map(|function| function.params[ordinal].evidence.name.as_str())
                                .filter(|name| identifier(name))
                                .min()
                                .map(str::to_string)
                        })
                        .collect::<Vec<_>>();
                    let mut used = chosen.iter().flatten().cloned().collect::<BTreeSet<_>>();
                    let mut assigned = BTreeSet::new();
                    for (ordinal, name) in chosen.iter_mut().enumerate() {
                        if name
                            .as_ref()
                            .is_none_or(|name| !assigned.insert(name.clone()))
                        {
                            let mut fallback = format!("param{ordinal}");
                            while !used.insert(fallback.clone()) {
                                fallback.push('_');
                            }
                            *name = Some(fallback);
                        }
                    }
                    let params = chosen
                        .into_iter()
                        .zip(&first.params)
                        .map(|(name, param)| {
                            Ok((
                                name.unwrap(),
                                lower(&param.written, &names, snapshot.target)?,
                            ))
                        })
                        .collect::<Result<_>>()?;
                    Kind::Function {
                        import: import.clone(),
                        convention: first.evidence.convention,
                        variadic: first.evidence.variadic,
                        result: lower(&first.result, &names, snapshot.target)?,
                        params,
                    }
                }
            };
            let owner = decl
                .owner
                .map(|owner| {
                    names[owner].clone().ok_or_else(|| {
                        Error::new(
                            Stage::Representation,
                            "named types inside unnamed records require a separate publication contract",
                        )
                    })
                })
                .transpose()?;
            let key = (
                name.namespace.clone(),
                name.name.clone(),
                matches!(kind, Kind::Function { .. }),
            );
            if declarations
                .insert(
                    key,
                    Entry {
                        namespace: name.namespace.clone(),
                        name: name.name.clone(),
                        kind,
                        owner,
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
        for evidence in &snapshot.macros {
            if !identifier(&evidence.name) {
                return Err(Error::new(
                    Stage::Representation,
                    "unsupported macro publication name",
                ));
            }
            let MacroKind::Integer { value, .. } = &evidence.kind else {
                return Err(Error::new(
                    Stage::Representation,
                    format!(
                        "{}: macro alias publication requires a profile-specific contract",
                        evidence.name
                    ),
                ));
            };
            let key = (namespace.to_string(), evidence.name.clone(), true);
            if declarations
                .insert(
                    key,
                    Entry {
                        namespace: namespace.to_string(),
                        name: evidence.name.clone(),
                        kind: Kind::Constant(value.clone()),
                        owner: None,
                    },
                )
                .is_some()
            {
                return Err(Error::new(
                    Stage::Representation,
                    format!(
                        "macro publication collision for {namespace}.{}",
                        evidence.name
                    ),
                ));
            }
        }
        Ok(Self {
            declarations: declarations.into_values().collect(),
        })
    }

    fn enumeration(
        snapshot: &Snapshot,
        id: Id,
        names: &[Option<metadata::TypeName>],
    ) -> Result<Kind> {
        let DeclarationKind::Enum {
            underlying,
            scoped,
            members,
        } = &snapshot.declarations[id].kind
        else {
            return Err(Error::new(
                Stage::Representation,
                "enum edge targets a non-enum",
            ));
        };
        if members.iter().any(|member| !identifier(&member.name)) {
            return Err(Error::new(
                Stage::Representation,
                "unsupported native enum member name",
            ));
        }
        Ok(Kind::Enum {
            underlying: lower(underlying, names, snapshot.target)?,
            scoped: *scoped,
            members: members
                .iter()
                .map(|member| (member.name.clone(), member.value.clone()))
                .collect(),
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
            let owner = decl.owner.as_ref().map_or(String::new(), |owner| {
                let owner =
                    windows_rdl::emit::write_type("", &metadata::Type::ValueName(owner.clone()));
                format!("#[Windows::Win32::Metadata::NativeTypeOwner({owner})] ")
            });
            node.items.push(format!(
                "{owner}{}",
                Self::render(&decl.kind, &name.to_string())
            ));
        }
        Ok(windows_rdl::formatter::format(&root.render(true)))
    }

    fn record(snapshot: &Snapshot, id: Id, names: &[Option<metadata::TypeName>]) -> Result<Kind> {
        let DeclarationKind::Record {
            union,
            fields,
            contract,
        } = &snapshot.declarations[id].kind
        else {
            return Err(Error::new(
                Stage::Representation,
                "unnamed record has no storage definition",
            ));
        };
        let mut used: BTreeSet<_> = fields
            .iter()
            .filter(|field| !field.evidence.anonymous)
            .map(|field| field.evidence.name.clone())
            .collect();
        let fields = fields
            .iter()
            .map(|field| {
                let name = if field.evidence.anonymous {
                    let mut name = format!("Anonymous_{}", field.evidence.ordinal);
                    while !used.insert(name.clone()) {
                        name.push('_');
                    }
                    name
                } else {
                    if !identifier(&field.evidence.name) {
                        return Err(Error::new(
                            Stage::Representation,
                            "unsupported native field publication name",
                        ));
                    }
                    field.evidence.name.clone()
                };
                let kind = if let NativeKind::Record(child) = field.written.kind
                    && snapshot.declarations[child].anonymous
                {
                    FieldKind::Nested(Box::new(Self::record(snapshot, child, names)?))
                } else {
                    FieldKind::Type(lower(&field.written, names, snapshot.target)?)
                };
                Ok(Field {
                    name,
                    anonymous: field.evidence.anonymous,
                    kind,
                })
            })
            .collect::<Result<_>>()?;
        Ok(Kind::Record {
            union: *union,
            fields,
            contract: contract.unwrap_or_default(),
        })
    }

    fn render(kind: &Kind, name: &str) -> String {
        match kind {
            Kind::Constant(value) => format!(
                "const {name}: {} = {};",
                windows_rdl::emit::write_type("", &value.ty()),
                windows_rdl::emit::write_value("", value)
            ),
            Kind::Alias(ty) => {
                let ty = windows_rdl::emit::write_type("", ty);
                format!("type {name} = {ty};")
            }
            Kind::Opaque { union } => format!(
                "#[opaque] {} {name} {{}}",
                if *union { "union" } else { "struct" }
            ),
            Kind::Enum {
                underlying,
                scoped,
                members,
            } => {
                let underlying = windows_rdl::emit::write_type("", underlying);
                let members = members
                    .iter()
                    .map(|(name, value)| {
                        let name = windows_rdl::emit::write_ident(name);
                        let value = windows_rdl::emit::write_value("", value);
                        format!("{name} = {value},")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "#[repr({underlying})] {}enum {name} {{ {members} }}",
                    if *scoped { "#[scoped] " } else { "" }
                )
            }
            Kind::Function {
                import,
                convention,
                variadic,
                result,
                params,
            } => {
                let import_name = if import.export == name.trim_start_matches("r#") {
                    String::new()
                } else {
                    format!(", import = {:?}", import.export)
                };
                let abi = match convention {
                    CallingConvention::C => " \"C\"",
                    CallingConvention::Stdcall | CallingConvention::Win64 => "",
                    CallingConvention::Fastcall => " \"fastcall\"",
                };
                let mut params = params
                    .iter()
                    .map(|(name, ty)| {
                        let name = windows_rdl::emit::write_ident(name);
                        let ty = windows_rdl::emit::write_type("", ty);
                        format!("{name}: {ty}")
                    })
                    .collect::<Vec<_>>();
                if *variadic {
                    params.push("...".into());
                }
                let result = if *result == metadata::Type::Void {
                    String::new()
                } else {
                    format!(" -> {}", windows_rdl::emit::write_type("", result))
                };
                format!(
                    "#[library({:?}{import_name})] extern{abi} fn {name}({}){result};",
                    import.dll,
                    params.join(", ")
                )
            }
            Kind::Record {
                union,
                fields,
                contract,
            } => {
                let fields = fields
                    .iter()
                    .map(|field| {
                        let name = windows_rdl::emit::write_ident(&field.name);
                        let ty = match &field.kind {
                            FieldKind::Type(ty) => {
                                windows_rdl::emit::write_type("", ty).to_string()
                            }
                            FieldKind::Nested(kind) => Self::render(kind, ""),
                        };
                        let attribute = if field.anonymous {
                            "#[Windows::Win32::Metadata::NativeAnonymousField] "
                        } else {
                            ""
                        };
                        format!("{attribute}{name}: {ty},")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let mut attributes = String::new();
                if let Some(packing) = contract.packing {
                    attributes.push_str(&format!("#[packed({packing})] "));
                }
                if let Some(alignment) = contract.alignment {
                    attributes.push_str(&format!("#[align({alignment})] "));
                }
                format!(
                    "{attributes}{} {name} {{ {fields} }}",
                    if *union { "union" } else { "struct" }
                )
            }
        }
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

fn lower(
    ty: &NativeType,
    names: &[Option<metadata::TypeName>],
    target: Target,
) -> Result<metadata::Type> {
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
        NativeKind::Alias(id) | NativeKind::Record(id) | NativeKind::Enum(id) => {
            Type::ValueName(names[*id].clone().ok_or_else(|| {
                Error::new(
                    Stage::Representation,
                    "unnamed type cannot supply a named reference",
                )
            })?)
        }
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
