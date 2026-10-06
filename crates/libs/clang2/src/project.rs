use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    Value,
    Interface,
}

/// A trusted metadata binding, not evidence that the external ABI matches the native declaration.
pub struct TypeReference {
    pub namespace: String,
    pub name: String,
    pub kind: ReferenceKind,
}

pub struct ProjectionOptions {
    pub namespace: String,
    pub library: Option<String>,
    pub references: BTreeMap<String, TypeReference>,
}

impl ProjectionOptions {
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            library: None,
            references: BTreeMap::new(),
        }
    }
}

/// Closed projected declarations. Rendering does not access the native graph.
#[derive(Debug)]
pub struct Plan {
    namespace: String,
    items: BTreeMap<String, Item>,
    omitted: BTreeMap<String, String>,
}

impl Plan {
    /// Selected roots omitted by an explicit projection policy, with their reasons.
    pub fn omitted(&self) -> &BTreeMap<String, String> {
        &self.omitted
    }

    pub fn rdl(&self) -> String {
        let mut output = format!("#[win32]\nmod {} {{\n", self.namespace);
        for (name, item) in &self.items {
            match item {
                Item::Record(fields) => {
                    writeln!(output, "    struct {name} {{").unwrap();
                    for (field, ty) in fields {
                        writeln!(output, "        {field}: {},", ty.text()).unwrap();
                    }
                    output.push_str("    }\n");
                }
                Item::Alias(ty) => writeln!(output, "    type {name} = {};", ty.text()).unwrap(),
                Item::Function {
                    abi,
                    library,
                    link_name,
                    parameters,
                    result,
                } => {
                    writeln!(
                        output,
                        "    #[library({library:?}, import = {link_name:?})]"
                    )
                    .unwrap();
                    write!(output, "    extern {abi:?} fn {name}(").unwrap();
                    for (index, (attributes, ty)) in parameters.iter().enumerate() {
                        if index != 0 {
                            output.push_str(", ");
                        }
                        write!(output, "{attributes}p{index}: {}", ty.text()).unwrap();
                    }
                    output.push(')');
                    if !matches!(result, ProjectedType::Void) {
                        write!(output, " -> {}", result.text()).unwrap();
                    }
                    output.push_str(";\n");
                }
                Item::Constant { ty, value } => {
                    writeln!(output, "    const {name}: {} = {value};", ty.text()).unwrap();
                }
                Item::Interface {
                    base,
                    guid,
                    methods,
                } => {
                    write!(
                        output,
                        "    #[guid({})]\n    interface {name}",
                        rdl_guid(guid)
                    )
                    .unwrap();
                    if let Some(base) = base {
                        write!(output, ": {base}").unwrap();
                    }
                    output.push_str(" {\n");
                    for (method, parameters, result) in methods {
                        write!(output, "        fn {method}(&self").unwrap();
                        for (attributes, parameter, ty) in parameters {
                            write!(output, ", {attributes}{parameter}: {}", ty.text()).unwrap();
                        }
                        output.push(')');
                        if !matches!(result, ProjectedType::Void) {
                            write!(output, " -> {}", result.text()).unwrap();
                        }
                        output.push_str(";\n");
                    }
                    output.push_str("    }\n");
                }
            }
        }
        output.push_str("}\n");
        output
    }
}

fn rdl_guid(guid: &str) -> String {
    let hex = guid.replace('-', "");
    format!(
        "0x{}_{}_{}_{}_{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

#[derive(Debug)]
enum Item {
    Record(Vec<(String, ProjectedType)>),
    Alias(ProjectedType),
    Interface {
        base: Option<String>,
        guid: String,
        methods: Vec<(String, Vec<(String, String, ProjectedType)>, ProjectedType)>,
    },
    Function {
        abi: &'static str,
        library: String,
        link_name: String,
        parameters: Vec<(String, ProjectedType)>,
        result: ProjectedType,
    },
    Constant {
        ty: ProjectedType,
        value: String,
    },
}

#[derive(Debug)]
enum ProjectedType {
    Void,
    Scalar(&'static str, Layout),
    Named(String, Option<Layout>),
    Class(String),
    Pointer {
        mutable: bool,
        depth: usize,
        target: Box<Self>,
    },
}

impl ProjectedType {
    fn text(&self) -> String {
        match self {
            Self::Void => "void".into(),
            Self::Scalar(name, _) => (*name).into(),
            Self::Named(name, _) | Self::Class(name) => name.clone(),
            Self::Pointer {
                mutable,
                depth,
                target,
            } => {
                format!(
                    "{}{}",
                    if *mutable { "*mut " } else { "*const " }.repeat(*depth),
                    target.text()
                )
            }
        }
    }

    fn layout(&self, pointer_size: i64) -> Option<Layout> {
        match self {
            Self::Void => None,
            Self::Scalar(_, layout) => Some(layout.clone()),
            Self::Named(_, layout) => layout.clone(),
            Self::Class(_) | Self::Pointer { .. } => Some(Layout {
                size: pointer_size,
                align: pointer_size,
            }),
        }
    }
}

impl Resolved<'_> {
    pub fn project(&self, options: &ProjectionOptions) -> Result<Plan, Error> {
        if !self.snapshot.target.contains("-windows-") {
            return Err(Error(
                "projection currently supports Windows targets only".into(),
            ));
        }
        let namespace = namespace(&options.namespace)?;
        for group in &self.groups {
            let declaration = &self.snapshot.declarations[group[0].0];
            if let Some(reference) = options.references.get(&declaration.name) {
                let mut binding = declaration;
                if let DeclarationData::Alias { canonical, .. } = &binding.data
                    && let TypeKind::Named(target) = canonical.kind
                    && self.snapshot.declarations[target.0].name == declaration.name
                {
                    binding = &self.snapshot.declarations[target.0];
                }
                if !matches!(
                    binding.data,
                    DeclarationData::Record { .. } | DeclarationData::Enum { .. }
                ) {
                    return Err(Error(format!(
                        "external bindings currently require a native record or enum: {}",
                        declaration.name
                    )));
                }
                namespace_name(reference)?;
            }
        }
        let mut builder = Builder {
            resolved: self,
            options,
            pending: VecDeque::new(),
            scheduled: BTreeSet::new(),
            plan: Plan {
                namespace,
                items: BTreeMap::new(),
                omitted: BTreeMap::new(),
            },
        };
        for root in &self.snapshot.roots {
            builder.schedule(*root);
        }
        while let Some(id) = builder.pending.pop_front() {
            builder.item(id)?;
        }
        Ok(builder.plan)
    }
}

struct Builder<'a, 's> {
    resolved: &'a Resolved<'s>,
    options: &'a ProjectionOptions,
    pending: VecDeque<Id>,
    scheduled: BTreeSet<Id>,
    plan: Plan,
}

impl Builder<'_, '_> {
    fn schedule(&mut self, id: Id) {
        let id = self.resolved.representatives[id.0];
        if self.scheduled.insert(id) {
            self.pending.push_back(id);
        }
    }

    fn item(&mut self, id: Id) -> Result<(), Error> {
        let declaration = &self.resolved.snapshot.declarations[id.0];
        if self.options.references.contains_key(&declaration.name) {
            self.plan.omitted.insert(
                declaration.name.clone(),
                "provided by external metadata".into(),
            );
            return Ok(());
        }
        let name = ident(&declaration.name)?;
        let item = match &declaration.data {
            DeclarationData::Record {
                complete: true,
                fields,
                bases,
                methods,
                guid: Some(guid),
                ..
            } if fields.is_empty() && !methods.is_empty() => {
                self.interface(id, &name, guid, bases, methods)?
            }
            DeclarationData::Record {
                kind,
                complete: true,
                layout: Some(layout),
                fields,
                bases,
                methods,
                ..
            } if kind == "StructDecl" && bases.is_empty() && methods.is_empty() => {
                let mut projected = vec![];
                if fields.is_empty() {
                    return Err(Error(format!(
                        "empty record projection is not implemented for `{name}`"
                    )));
                }
                let mut size = 0;
                let mut align = 1;
                for field in fields {
                    if field.bit_width.is_some() {
                        return Err(Error(format!(
                            "bitfield projection is not implemented for `{name}`"
                        )));
                    }
                    let (ty, object) = self.lower(&field.ty, &mut BTreeSet::new())?;
                    if object {
                        return Err(Error("native interface objects require a pointer".into()));
                    }
                    let field_layout =
                        ty.layout(self.resolved.snapshot.pointer_size)
                            .ok_or_else(|| {
                                Error(format!(
                                    "projected layout unavailable for `{name}::{}`",
                                    field.name
                                ))
                            })?;
                    size = align_up(size, field_layout.align);
                    if size * 8 != field.offset {
                        return Err(Error(format!(
                            "`{name}` requires unsupported packing or field alignment"
                        )));
                    }
                    size += field_layout.size;
                    align = align.max(field_layout.align);
                    projected.push((ident(&field.name)?, ty));
                }
                let expected = Layout {
                    size: align_up(size, align),
                    align,
                };
                if expected != *layout {
                    return Err(Error(format!("projected layout differs for `{name}`")));
                }
                Item::Record(projected)
            }
            DeclarationData::Alias { target, .. } => {
                let (ty, object) = self.lower(target, &mut BTreeSet::new())?;
                if object || matches!(ty, ProjectedType::Class(_)) {
                    return Err(Error(
                        "explicit interface alias emission is not implemented".into(),
                    ));
                }
                Item::Alias(ty)
            }
            DeclarationData::Function {
                canonical,
                ty: written,
                link_name,
                ..
            } => {
                let TypeKind::Function {
                    result,
                    parameters,
                    prototype: true,
                    variadic: false,
                    ..
                } = &canonical.kind
                else {
                    return Err(Error(
                        "only fixed-prototype functions can be projected".into(),
                    ));
                };
                let TypeKind::Function { convention, .. } = &written.kind else {
                    unreachable!()
                };
                // These are the stable CXCallingConv C, X86StdCall, and Win64 values.
                let abi = match convention {
                    1 => "C",
                    2 | 10 => "system",
                    _ => {
                        return Err(Error(format!(
                            "calling convention {convention} is not supported by this projection"
                        )));
                    }
                };
                let library = self
                    .options
                    .library
                    .clone()
                    .ok_or_else(|| Error(format!("`{name}` requires an import library")))?;
                let annotations = &self.resolved.annotations[&id][&0];
                let mut params = vec![];
                for (index, parameter) in parameters.iter().enumerate() {
                    let (ty, object) = self.lower(parameter, &mut BTreeSet::new())?;
                    if object {
                        return Err(Error("native interface objects require a pointer".into()));
                    }
                    if ty.layout(self.resolved.snapshot.pointer_size).is_none() {
                        return Err(Error(
                            "by-value external parameter layout is not available".into(),
                        ));
                    }
                    let attributes = parameter_attributes(&annotations[index], &ty)?;
                    params.push((attributes, ty));
                }
                let (result, object) = self.lower(result, &mut BTreeSet::new())?;
                if object {
                    return Err(Error("native interface objects require a pointer".into()));
                }
                if !matches!(result, ProjectedType::Void)
                    && result.layout(self.resolved.snapshot.pointer_size).is_none()
                {
                    return Err(Error(
                        "by-value external result layout is not available".into(),
                    ));
                }
                Item::Function {
                    abi,
                    library,
                    link_name: link_name.clone(),
                    parameters: params,
                    result,
                }
            }
            DeclarationData::Variable { ty, value } => {
                let (ty, object) = self.lower(ty, &mut BTreeSet::new())?;
                if object || matches!(ty, ProjectedType::Class(_)) {
                    self.plan.omitted.insert(
                        declaration.name.clone(),
                        "interface values are not metadata constants".into(),
                    );
                    return Ok(());
                }
                let value = match (value, &ty) {
                    (Value::Integer(value), ProjectedType::Scalar(name, _)) => {
                        if *name == "bool" {
                            (*value != 0).to_string()
                        } else if name.starts_with('i') {
                            (*value as i64).to_string()
                        } else {
                            value.to_string()
                        }
                    }
                    (Value::Integer(value), ProjectedType::Pointer { .. }) => {
                        (*value as i64).to_string()
                    }
                    (Value::Float(value), ProjectedType::Scalar("f32" | "f64", _))
                        if f64::from_bits(*value).is_finite() =>
                    {
                        // Keep a floating literal, including the sign of negative zero.
                        format!("{:?}", f64::from_bits(*value))
                    }
                    _ => {
                        return Err(Error(format!(
                            "constant representation is not supported for `{name}`"
                        )));
                    }
                };
                Item::Constant { ty, value }
            }
            _ => return Err(Error(format!("projection is not implemented for `{name}`"))),
        };
        if self.plan.items.insert(name.clone(), item).is_some() {
            return Err(Error(format!(
                "multiple native entities project to `{name}`"
            )));
        }
        Ok(())
    }

    fn interface(
        &mut self,
        id: Id,
        name: &str,
        guid: &str,
        bases: &[Type],
        methods: &[Method],
    ) -> Result<Item, Error> {
        let base = match bases {
            [] => None,
            [base_ty] => {
                let TypeKind::Named(base_id) = &base_ty.kind else {
                    return Err(Error(format!(
                        "interface base projection is not implemented for `{name}`"
                    )));
                };
                let base_id = self.resolved.representatives[base_id.0];
                let base_declaration = &self.resolved.snapshot.declarations[base_id.0];
                if let Some(reference) = self.options.references.get(&base_declaration.name) {
                    if reference.kind != ReferenceKind::Interface {
                        return Err(Error(format!(
                            "interface base `{}` must bind to an external interface",
                            base_declaration.name
                        )));
                    }
                    Some(namespace_name(reference)?)
                } else if let DeclarationData::Record {
                    fields: base_fields,
                    methods: base_methods,
                    ..
                } = &base_declaration.data
                    && base_fields.is_empty()
                    && !base_methods.is_empty()
                {
                    self.schedule(base_id);
                    Some(ident(&base_declaration.name)?)
                } else {
                    return Err(Error(format!(
                        "interface base `{}` is not a projectable interface",
                        base_declaration.name
                    )));
                }
            }
            _ => {
                return Err(Error(format!(
                    "multiple interface bases are not implemented for `{name}`"
                )));
            }
        };
        let mut projected = vec![];
        for (index, method) in methods.iter().enumerate() {
            if !method.virtual_method || !method.pure || method.static_method {
                return Err(Error(format!(
                    "`{name}::{}` must be a pure virtual COM method",
                    method.name
                )));
            }
            if method.const_method || method.ref_qualifier != 0 {
                return Err(Error(format!(
                    "`{name}::{}` has an unsupported method qualifier",
                    method.name
                )));
            }
            let TypeKind::Function {
                result,
                parameters,
                prototype: true,
                variadic: false,
                ..
            } = &method.canonical.kind
            else {
                return Err(Error(format!(
                    "only fixed-prototype methods can be projected for `{name}::{}`",
                    method.name
                )));
            };
            let TypeKind::Function { convention, .. } = &method.ty.kind else {
                unreachable!()
            };
            // RDL COM methods use the system ABI: stdcall on x86, the platform ABI on 64-bit.
            if !matches!(
                (self.resolved.snapshot.pointer_size, convention),
                (4, 2) | (8, 1 | 10)
            ) {
                return Err(Error(format!(
                    "`{name}::{}` has a calling convention incompatible with COM: {convention}",
                    method.name
                )));
            }
            let annotations = &self.resolved.annotations[&id][&(index + 1)];
            let mut params = vec![];
            for (parameter_index, (parameter, ty)) in
                method.parameters.iter().zip(parameters).enumerate()
            {
                let (ty, object) = self.lower(ty, &mut BTreeSet::new())?;
                if object {
                    return Err(Error("native interface objects require a pointer".into()));
                }
                if ty.layout(self.resolved.snapshot.pointer_size).is_none() {
                    return Err(Error(format!(
                        "`{name}::{}` has an unsupported by-value parameter layout",
                        method.name
                    )));
                }
                let attributes = parameter_attributes(&annotations[parameter_index], &ty)?;
                params.push((attributes, ident(&parameter.name)?, ty));
            }
            let (result, object) = self.lower(result, &mut BTreeSet::new())?;
            if object {
                return Err(Error("native interface objects require a pointer".into()));
            }
            if !matches!(result, ProjectedType::Void)
                && result.layout(self.resolved.snapshot.pointer_size).is_none()
            {
                return Err(Error(format!(
                    "`{name}::{}` has an unsupported result layout",
                    method.name
                )));
            }
            projected.push((ident(&method.name)?, params, result));
        }
        Ok(Item::Interface {
            base,
            guid: guid.to_string(),
            methods: projected,
        })
    }

    fn lower(
        &mut self,
        ty: &Type,
        aliases: &mut BTreeSet<Id>,
    ) -> Result<(ProjectedType, bool), Error> {
        Ok(match &ty.kind {
            TypeKind::Builtin { kind, layout } => {
                let name = match kind.as_str() {
                    "Void" => return Ok((ProjectedType::Void, false)),
                    "Bool" => "bool",
                    "Char_S" | "SChar" => "i8",
                    "Char_U" | "UChar" => "u8",
                    "Short" => "i16",
                    "UShort" | "WChar" | "Char16" => "u16",
                    "Int" | "Long" => "i32",
                    "UInt" | "ULong" | "Char32" => "u32",
                    "LongLong" => "i64",
                    "ULongLong" => "u64",
                    "Float" => "f32",
                    "Double" | "LongDouble" => "f64",
                    _ => {
                        return Err(Error(format!(
                            "builtin projection is not implemented: {kind}"
                        )));
                    }
                };
                let size = match name {
                    "bool" | "i8" | "u8" => 1,
                    "i16" | "u16" => 2,
                    "i32" | "u32" | "f32" => 4,
                    _ => 8,
                };
                let expected = Layout { size, align: size };
                if layout.as_ref() != Some(&expected) {
                    return Err(Error(format!(
                        "builtin `{kind}` has an unsupported projected layout"
                    )));
                }
                (ProjectedType::Scalar(name, expected), false)
            }
            TypeKind::Named(id) => {
                let id = self.resolved.representatives[id.0];
                let declaration = &self.resolved.snapshot.declarations[id.0];
                if let DeclarationData::Alias { target, .. } = &declaration.data {
                    if !aliases.insert(id) {
                        return Err(Error("cyclic native alias".into()));
                    }
                    let result = self.lower(target, aliases)?;
                    aliases.remove(&id);
                    result
                } else if let Some(reference) = self.options.references.get(&declaration.name) {
                    let name = namespace_name(reference)?;
                    if reference.kind == ReferenceKind::Interface {
                        (ProjectedType::Class(name), true)
                    } else {
                        (ProjectedType::Named(name, None), false)
                    }
                } else if let DeclarationData::Record {
                    layout,
                    fields,
                    bases,
                    methods,
                    ..
                } = &declaration.data
                {
                    if !fields.is_empty() && (!bases.is_empty() || !methods.is_empty()) {
                        return Err(Error(format!(
                            "local C++ interface projection is not implemented for `{}`; an external binding is required",
                            declaration.name
                        )));
                    }
                    let name = ident(&declaration.name)?;
                    self.schedule(id);
                    if fields.is_empty() && !methods.is_empty() {
                        (ProjectedType::Class(name), true)
                    } else {
                        (ProjectedType::Named(name, layout.clone()), false)
                    }
                } else {
                    return Err(Error(format!(
                        "named type projection is not implemented for `{}`",
                        declaration.name
                    )));
                }
            }
            TypeKind::Pointer(target) | TypeKind::LValueReference(target) => {
                let (projected, object) = self.lower(target, aliases)?;
                if object {
                    (projected, false)
                } else {
                    let mutable = !target.qualifiers.constant;
                    let pointer = match projected {
                        ProjectedType::Pointer {
                            mutable: inner,
                            depth,
                            target,
                        } => ProjectedType::Pointer {
                            mutable: mutable && inner,
                            depth: depth + 1,
                            target,
                        },
                        target => ProjectedType::Pointer {
                            mutable,
                            depth: 1,
                            target: Box::new(target),
                        },
                    };
                    (pointer, false)
                }
            }
            _ => return Err(Error("this native type has no prototype projection".into())),
        })
    }
}

fn parameter_attributes(annotations: &[String], ty: &ProjectedType) -> Result<String, Error> {
    let mut attributes = String::new();
    for annotation in annotations {
        attributes.push_str(match annotation.as_str() {
            "_In_" => "#[in] ",
            "_Out_" => "#[out] ",
            "_Inout_" => "#[in] #[out] ",
            "_In_opt_" if matches!(ty, ProjectedType::Pointer { .. } | ProjectedType::Class(_)) => {
                "#[in] #[opt] "
            }
            "_COM_Outptr_" => match ty {
                ProjectedType::Pointer {
                    mutable: true,
                    depth: 2,
                    target,
                } if matches!(**target, ProjectedType::Void) => "#[out] #[iid_is] ",
                ProjectedType::Pointer {
                    mutable: true,
                    depth: 1,
                    target,
                } if matches!(**target, ProjectedType::Class(_)) => "#[out] ",
                _ => {
                    return Err(Error(
                        "_COM_Outptr_ requires a writable void** or interface output pointer"
                            .into(),
                    ));
                }
            },
            _ => {
                return Err(Error(format!(
                    "annotation projection is not implemented: {annotation}"
                )));
            }
        });
    }
    Ok(attributes)
}

fn align_up(value: i64, alignment: i64) -> i64 {
    (value + alignment - 1) / alignment * alignment
}

fn namespace_name(reference: &TypeReference) -> Result<String, Error> {
    Ok(format!(
        "{}::{}",
        namespace(&reference.namespace)?.replace('.', "::"),
        ident(&reference.name)?
    ))
}

fn ident(value: &str) -> Result<String, Error> {
    let mut chars = value.chars();
    if !chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(Error(format!("unsupported output identifier `{value}`")));
    }
    Ok(
        if matches!(
            value,
            "type"
                | "mod"
                | "struct"
                | "enum"
                | "fn"
                | "const"
                | "mut"
                | "ref"
                | "self"
                | "Self"
                | "super"
                | "crate"
                | "use"
                | "pub"
                | "extern"
                | "return"
                | "impl"
                | "trait"
                | "where"
                | "in"
                | "as"
                | "match"
                | "loop"
                | "for"
                | "while"
                | "if"
                | "else"
                | "break"
                | "continue"
                | "let"
                | "static"
                | "move"
                | "unsafe"
                | "async"
                | "await"
                | "dyn"
                | "true"
                | "false"
        ) {
            format!("r#{value}")
        } else {
            value.into()
        },
    )
}

fn namespace(value: &str) -> Result<String, Error> {
    value
        .split('.')
        .map(ident)
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| parts.join("."))
}
