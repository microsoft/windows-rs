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
    attributes: Vec<String>,
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
        params: Vec<Parameter>,
        annotations: Vec<AnnotationEvidence>,
    },
    Interface {
        evidence: InterfaceEvidence,
        base: Option<metadata::TypeName>,
        methods: Vec<Method>,
    },
}

struct Method {
    name: String,
    slot: u32,
    result: metadata::Type,
    params: Vec<Parameter>,
    annotations: Vec<AnnotationEvidence>,
}

struct Parameter {
    name: String,
    ty: metadata::Type,
    contract: ParameterContract,
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
                DeclarationKind::Interface {
                    evidence,
                    base,
                    methods,
                    ..
                } => Kind::Interface {
                    evidence: evidence.clone(),
                    base: base
                        .map(|id| {
                            names[id].clone().ok_or_else(|| {
                                Error::new(
                                    Stage::Representation,
                                    "interface base has no publication identity",
                                )
                            })
                        })
                        .transpose()?,
                    methods: methods
                        .iter()
                        .map(|method| {
                            if (snapshot.target == Target::X86
                                && method.function.evidence.convention != CallingConvention::Stdcall)
                                || (snapshot.target != Target::X86
                                    && !matches!(
                                        method.function.evidence.convention,
                                        CallingConvention::C | CallingConvention::Win64
                                    ))
                            {
                                return Err(Error::new(
                                    Stage::Representation,
                                    "interface method does not use the target COM calling convention",
                                ));
                            }
                            Ok(Method {
                                name: method.evidence.name.clone(),
                                slot: method.evidence.slot,
                                annotations: method.function.evidence.annotations.clone(),
                                result: lower(&method.function.result, &names, snapshot.target)?,
                                params: function_parameters(
                                    std::slice::from_ref(&method.function),
                                    &names,
                                    snapshot.target,
                                )?,
                            })
                        })
                        .collect::<Result<_>>()?,
                },
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
                    let params = function_parameters(observations, &names, snapshot.target)?;
                    Kind::Function {
                        import: import.clone(),
                        convention: first.evidence.convention,
                        variadic: first.evidence.variadic,
                        result: lower(&first.result, &names, snapshot.target)?,
                        params,
                        annotations: observations.iter().flat_map(|function| function.evidence.annotations.clone()).collect(),
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
                        attributes: vec![annotation_attributes(&decl.evidence.annotations)],
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
            let value = match &evidence.kind {
                MacroKind::Integer { value, .. } => value,
                MacroKind::Alias {
                    qualified_name,
                    kind,
                    usr,
                    expansion,
                    spelling,
                } => {
                    let targets = snapshot
                        .declarations
                        .iter()
                        .enumerate()
                        .filter(|(_, decl)| {
                            decl.evidence.qualified_name == *qualified_name
                                && decl.evidence.kind == *kind
                                && decl.evidence.usr == *usr
                                && decl.evidence.expansion == *expansion
                                && decl.evidence.spelling == *spelling
                        })
                        .collect::<Vec<_>>();
                    let [(id, target)] = targets.as_slice() else {
                        return Err(Error::new(
                            Stage::Representation,
                            format!(
                                "{}: alias target must have one original closed native declaration",
                                evidence.name
                            ),
                        ));
                    };
                    let name = names[*id].as_ref().ok_or_else(|| {
                        Error::new(
                            Stage::Representation,
                            "macro alias target has no publication identity",
                        )
                    })?;
                    let key = (
                        name.namespace.clone(),
                        name.name.clone(),
                        matches!(target.kind, DeclarationKind::Function(_)),
                    );
                    let entry = declarations.get_mut(&key).ok_or_else(|| {
                        Error::new(
                            Stage::Representation,
                            "macro alias target was omitted from closed output",
                        )
                    })?;
                    entry.attributes.push(format!(
                        "#[Windows::Win32::Metadata::NativeMacroAlias({:?}, {:?}, {:?})]",
                        evidence.name,
                        kind,
                        format!(
                            "source-sha256={}\n{}",
                            snapshot.source_sha256,
                            snapshot.arguments.join("\n")
                        )
                    ));
                    continue;
                }
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
                        attributes: Vec::new(),
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
                "{}{owner}{}",
                decl.attributes.join(" "),
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
            Kind::Interface {
                evidence,
                base,
                methods,
            } => {
                let base = base
                    .as_ref()
                    .map(|base| {
                        format!(
                            ": {}",
                            windows_rdl::emit::write_type(
                                "",
                                &metadata::Type::ClassName(base.clone())
                            )
                        )
                    })
                    .unwrap_or_default();
                let methods = methods
                    .iter()
                    .map(|method| {
                        let name = windows_rdl::emit::write_ident(&method.name);
                        let params = method
                            .params
                            .iter()
                            .map(|parameter| {
                                format!(
                                    "{} {}: {}",
                                    parameter_attributes(&parameter.contract),
                                    windows_rdl::emit::write_ident(&parameter.name),
                                    windows_rdl::emit::write_type("", &parameter.ty)
                                )
                            })
                            .collect::<Vec<_>>();
                        let result = if method.result == metadata::Type::Void {
                            String::new()
                        } else {
                            format!(" -> {}", windows_rdl::emit::write_type("", &method.result))
                        };
                        format!(
                            "{} #[slot({})] fn {name}(&self{}){result};",
                            annotation_attributes(&method.annotations),
                            method.slot,
                            if params.is_empty() {
                                String::new()
                            } else {
                                format!(", {}", params.join(", "))
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "#[guid(0x{})] #[vtable(size = {}, inherited = {})] {}{}interface {name}{base} {{ {methods} }}",
                    evidence.guid.replace('-', ""),
                    evidence.vtable_size,
                    evidence.inherited,
                    if evidence.no_vtable {
                        "#[Windows::Win32::Metadata::NativeAnnotation(\"CXX\", 0, \"__declspec(novtable)\")] "
                    } else {
                        ""
                    },
                    evidence.packing.map(|packing| format!(
                        "#[Windows::Win32::Metadata::NativeAnnotation(\"Clang\", 0, \"max_field_alignment = {packing}\")] "
                    )).unwrap_or_default(),
                )
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
                annotations,
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
                    .map(|parameter| {
                        let name = windows_rdl::emit::write_ident(&parameter.name);
                        let ty = windows_rdl::emit::write_type("", &parameter.ty);
                        format!("{} {name}: {ty}", parameter_attributes(&parameter.contract))
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
                    "{} #[library({:?}{import_name})] extern{abi} fn {name}({}){result};",
                    annotation_attributes(annotations),
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
                if let Some(visibility) = contract.type_visibility {
                    let value = match visibility {
                        TypeVisibility::Default => "default",
                        TypeVisibility::Hidden => "hidden",
                        TypeVisibility::Protected => "protected",
                    };
                    attributes.push_str(&format!(
                        "#[Windows::Win32::Metadata::NativeAnnotation(\"Clang\", 0, \"type_visibility = {value}\")] "
                    ));
                }
                format!(
                    "{attributes}{} {name} {{ {fields} }}",
                    if *union { "union" } else { "struct" }
                )
            }
        }
    }
}

fn function_parameters(
    observations: &[NativeFunction],
    names: &[Option<metadata::TypeName>],
    target: Target,
) -> Result<Vec<Parameter>> {
    let first = &observations[0];
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
    chosen
        .into_iter()
        .zip(&first.params)
        .enumerate()
        .map(|(ordinal, (name, param))| {
            let mut contract = param.evidence.contract.clone();
            contract.annotations = observations
                .iter()
                .flat_map(|function| {
                    function.params[ordinal]
                        .evidence
                        .contract
                        .annotations
                        .clone()
                })
                .collect();
            Ok(Parameter {
                name: name.unwrap(),
                ty: lower(&param.written, names, target)?,
                contract,
            })
        })
        .collect()
}

fn annotation_attributes(annotations: &[AnnotationEvidence]) -> String {
    let mut ordinals = BTreeMap::new();
    let mut result = String::new();
    for annotation in annotations {
        let ordinal = ordinals.entry(annotation.source.as_str()).or_insert(0);
        result.push_str(&format!(
            "#[Windows::Win32::Metadata::NativeAnnotation({:?}, {}, {:?})] ",
            annotation.source, ordinal, annotation.payload
        ));
        for (identifier, parameter) in &annotation.bindings {
            result.push_str(&format!(
                "#[Windows::Win32::Metadata::NativeAnnotationBinding({:?}, {}, {:?}, {})] ",
                annotation.source, ordinal, identifier, parameter
            ));
        }
        if let Some(resolution) = &annotation.resolution {
            result.push_str(&format!(
                "#[Windows::Win32::Metadata::NativeAnnotation({:?}, {}, {:?})] ",
                format!("{}-expansion", annotation.source),
                ordinal,
                resolution.payload
            ));
            for (definition_ordinal, definition) in resolution.definitions.iter().enumerate() {
                result.push_str(&format!(
                    "#[Windows::Win32::Metadata::NativeAnnotation({:?}, {}, {:?})] ",
                    format!("{}-definition-{ordinal}", annotation.source), definition_ordinal,
                    format!("{}\n{}:{}:{}@{}\nfunction-like={} variadic={}\nparameters={:?}\nreplacement={:?}",
                        definition.name, definition.location.file, definition.location.line,
                        definition.location.column, definition.location.offset,
                        definition.function_like, definition.variadic,
                        definition.parameters, definition.tokens)
                ));
            }
        }
        *ordinal += 1;
    }
    result
}

fn parameter_attributes(contract: &ParameterContract) -> String {
    let mut result: String = match contract.direction {
        None => "#[unspecified]".into(),
        Some(Direction::In) => "#[r#in]".into(),
        Some(Direction::Out) => "#[out]".into(),
        Some(Direction::InOut) => "#[r#in] #[out]".into(),
    };
    if contract.optional {
        result.push_str(" #[opt]");
    }
    if contract.com_outptr {
        result.push_str(" #[iid_is]");
    }
    result.push(' ');
    result.push_str(&annotation_attributes(&contract.annotations));
    for extent in &contract.extents {
        result.push_str(&format!(
            "#[Windows::Win32::Metadata::NativeBufferExtent({:?}, {:?}, {}, {})] ",
            extent.phase, extent.units, extent.parameter, extent.dereference
        ));
    }
    result
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
