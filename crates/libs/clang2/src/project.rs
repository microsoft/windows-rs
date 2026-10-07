use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    Value,
    Interface,
}

/// Null-terminated pointer contracts supplied by external metadata.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StringKind {
    Ansi,
    AnsiConst,
    Wide,
    WideConst,
}

/// A trusted metadata binding, not evidence that the external ABI matches the native declaration.
pub struct TypeReference {
    pub namespace: String,
    pub name: String,
    pub kind: ReferenceKind,
}

/// A caller-supplied DLL import keyed by the compiler's native linker symbol.
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionImport {
    pub library: String,
    pub name: String,
}

pub struct ProjectionOptions {
    pub namespace: String,
    pub library: Option<String>,
    /// Exact DLL imports. Unmapped functions require the explicit `library` fallback.
    pub imports: BTreeMap<String, FunctionImport>,
    /// External record/enum bindings and trusted scalar or pointer typedef contracts.
    pub references: BTreeMap<String, TypeReference>,
    /// Trusted pointer-sized value types for SAL-annotated null-terminated strings.
    pub string_references: BTreeMap<StringKind, TypeReference>,
}

impl ProjectionOptions {
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            library: None,
            imports: BTreeMap::new(),
            references: BTreeMap::new(),
            string_references: BTreeMap::new(),
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
        let mut namespaces = self.namespace.rsplit('.');
        let mut output = format!("#[win32]\nmod {} {{\n", namespaces.next().unwrap());
        for (name, item) in &self.items {
            match item {
                Item::Record { fields, alignment } => {
                    if let Some(alignment) = alignment {
                        writeln!(output, "    #[align({alignment})]").unwrap();
                    }
                    writeln!(output, "    struct {name} {{").unwrap();
                    for (field, ty) in fields {
                        writeln!(output, "        {field}: {},", ty.text()).unwrap();
                    }
                    output.push_str("    }\n");
                }
                Item::Alias(ty) => writeln!(output, "    type {name} = {};", ty.text()).unwrap(),
                Item::Enum {
                    repr,
                    flags,
                    variants,
                } => {
                    if *flags {
                        output.push_str("    #[flags]\n");
                    }
                    writeln!(output, "    #[repr({repr})]\n    enum {name} {{").unwrap();
                    for (variant, value) in variants {
                        writeln!(output, "        {variant} = {value},").unwrap();
                    }
                    output.push_str("    }\n");
                }
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
                    for (method, special, parameters, result) in methods {
                        if *special {
                            output.push_str("        #[special]\n");
                        }
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
        for namespace in namespaces {
            let body: String = output.lines().map(|line| format!("    {line}\n")).collect();
            output = format!("mod {namespace} {{\n{body}}}\n");
        }
        output
    }

    fn adjusted_record(&self, ty: &ProjectedType, checked: &mut BTreeMap<String, bool>) -> bool {
        if matches!(ty, ProjectedType::Padding(_)) {
            return true;
        }
        let ProjectedType::Named(name, _) = ty else {
            return false;
        };
        if let Some(adjusted) = checked.get(name) {
            return *adjusted;
        }
        let adjusted = match self.items.get(name) {
            Some(Item::Record { fields, alignment }) => {
                alignment.is_some()
                    || fields
                        .iter()
                        .any(|(_, ty)| self.adjusted_record(ty, checked))
            }
            _ => false,
        };
        checked.insert(name.clone(), adjusted);
        adjusted
    }

    fn validate_calls(&self) -> Result<(), Error> {
        let mut checked = BTreeMap::new();
        for (name, item) in &self.items {
            let adjusted = match item {
                Item::Function {
                    parameters, result, ..
                } => {
                    parameters
                        .iter()
                        .any(|(_, ty)| self.adjusted_record(ty, &mut checked))
                        || self.adjusted_record(result, &mut checked)
                }
                Item::Interface { methods, .. } => {
                    methods.iter().any(|(_, _, parameters, result)| {
                        parameters
                            .iter()
                            .any(|(_, _, ty)| self.adjusted_record(ty, &mut checked))
                            || self.adjusted_record(result, &mut checked)
                    })
                }
                _ => false,
            };
            if adjusted {
                return Err(Error(format!(
                    "`{name}`: by-value calls with adjusted record layouts require native ABI coverage"
                )));
            }
        }
        Ok(())
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
    Record {
        fields: Vec<(String, ProjectedType)>,
        alignment: Option<i64>,
    },
    Alias(ProjectedType),
    Enum {
        repr: &'static str,
        flags: bool,
        variants: Vec<(String, String)>,
    },
    Interface {
        base: Option<String>,
        guid: String,
        methods: Vec<(
            String,
            bool,
            Vec<(String, String, ProjectedType)>,
            ProjectedType,
        )>,
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

#[derive(Clone, Debug, PartialEq)]
enum ProjectedType {
    Padding(i64),
    Void,
    Scalar(&'static str, Layout),
    ScalarReference(String, &'static str, Layout),
    PointerReference {
        name: String,
        mutable: bool,
        string: Option<StringKind>,
    },
    Named(String, Option<Layout>),
    Class(String),
    Pointer {
        mutable: bool,
        depth: usize,
        target: Box<Self>,
    },
}

impl ProjectedType {
    fn scalar_kind(&self) -> Option<&'static str> {
        match self {
            Self::Scalar(kind, _) | Self::ScalarReference(_, kind, _) => Some(kind),
            _ => None,
        }
    }

    fn text(&self) -> String {
        match self {
            // Padding can remain uninitialized when native code copies a record.
            Self::Padding(size) => format!("union {{ bytes: [u8; {size}], uninit: [u8; 0], }}"),
            Self::Void => "void".into(),
            Self::Scalar(name, _) => (*name).into(),
            Self::Named(name, _) | Self::Class(name) | Self::ScalarReference(name, ..) => {
                name.clone()
            }
            Self::PointerReference { name, .. } => name.clone(),
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
            Self::Padding(size) => Some(Layout {
                size: *size,
                align: 1,
            }),
            Self::Void => None,
            Self::Scalar(_, layout) | Self::ScalarReference(_, _, layout) => Some(layout.clone()),
            Self::Named(_, layout) => layout.clone(),
            Self::Class(_) | Self::Pointer { .. } | Self::PointerReference { .. } => Some(Layout {
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
        for reference in options.string_references.values() {
            if reference.kind != ReferenceKind::Value {
                return Err(Error(
                    "string bindings must reference metadata value types".into(),
                ));
            }
            namespace_name(reference)?;
        }
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
                ) && !matches!(
                    &binding.data,
                    DeclarationData::Alias { canonical, .. }
                        if matches!(canonical.kind, TypeKind::Builtin { .. } | TypeKind::Pointer(_))
                            && reference.kind == ReferenceKind::Value
                ) {
                    return Err(Error(format!(
                        "external bindings require a native record, enum, or scalar/pointer value typedef: {}",
                        declaration.name
                    )));
                }
                namespace_name(reference)?;
            }
        }
        let mut builder = Builder {
            resolved: self,
            options,
            aliases: BTreeMap::new(),
            names: BTreeMap::new(),
            groups: self
                .groups
                .iter()
                .map(|group| (self.representatives[group[0].0], group.as_slice()))
                .collect(),
            pending: VecDeque::new(),
            scheduled: BTreeSet::new(),
            plan: Plan {
                namespace,
                items: BTreeMap::new(),
                omitted: BTreeMap::new(),
            },
        };
        for root in &self.snapshot.roots {
            let alias = &self.snapshot.declarations[root.0];
            if let DeclarationData::Alias { canonical, .. } = &alias.data
                && let TypeKind::Named(target) = canonical.kind
            {
                let target = self.representatives[target.0];
                if matches!(
                    self.snapshot.declarations[target.0].data,
                    DeclarationData::Record { .. }
                ) && !self.snapshot.roots.iter().any(|root| {
                    self.representatives[root.0] == target
                        && !matches!(
                            self.snapshot.declarations[root.0].data,
                            DeclarationData::Record { unnamed: true, .. }
                        )
                }) {
                    let name = ident(&alias.name)?;
                    if let Some(previous) = builder.names.insert(target, name.clone())
                        && previous != name
                    {
                        return Err(Error(
                            "multiple selected aliases name the same record".into(),
                        ));
                    }
                }
            }
        }
        for root in &self.snapshot.roots {
            builder.schedule(*root);
        }
        while let Some(id) = builder.pending.pop_front() {
            builder.item(id)?;
        }
        builder.plan.validate_calls()?;
        Ok(builder.plan)
    }
}

struct Builder<'a, 's> {
    resolved: &'a Resolved<'s>,
    options: &'a ProjectionOptions,
    groups: BTreeMap<Id, &'a [Id]>,
    aliases: BTreeMap<Id, (ProjectedType, bool)>,
    names: BTreeMap<Id, String>,
    pending: VecDeque<Id>,
    scheduled: BTreeSet<Id>,
    plan: Plan,
}

impl Builder<'_, '_> {
    fn name(&self, id: Id) -> Result<String, Error> {
        self.names.get(&id).cloned().map_or_else(
            || ident(&self.resolved.snapshot.declarations[id.0].name),
            Ok,
        )
    }
    fn schedule(&mut self, id: Id) {
        let id = self.resolved.representatives[id.0];
        if self.scheduled.insert(id) {
            self.pending.push_back(id);
        }
    }

    fn item(&mut self, id: Id) -> Result<(), Error> {
        let declaration = &self.resolved.snapshot.declarations[id.0];
        if self.options.references.contains_key(&declaration.name) {
            if let DeclarationData::Alias { canonical, .. } = &declaration.data
                && matches!(
                    canonical.kind,
                    TypeKind::Builtin { .. } | TypeKind::Pointer(_)
                )
            {
                self.typedef_reference(canonical, &self.options.references[&declaration.name])?;
            }
            self.plan.omitted.insert(
                declaration.name.clone(),
                "provided by external metadata".into(),
            );
            return Ok(());
        }
        if let DeclarationData::Alias { canonical, .. } = &declaration.data
            && let TypeKind::Named(target) = canonical.kind
            && (self.resolved.snapshot.declarations[target.0].name == declaration.name
                || self.names.get(&self.resolved.representatives[target.0])
                    == Some(&declaration.name))
        {
            self.schedule(target);
            return Ok(());
        }
        let name = self.name(id)?;
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
                let mut names: BTreeSet<_> =
                    fields.iter().map(|field| field.name.clone()).collect();
                for (index, field) in fields.iter().enumerate() {
                    if field.bit_width.is_some() {
                        return Err(Error(format!(
                            "bitfield projection is not implemented for `{name}`"
                        )));
                    }
                    if field.name.is_empty() {
                        return Err(Error(format!(
                            "anonymous aggregate projection is not implemented for `{name}`"
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
                    let natural = align_up(size, field_layout.align);
                    if field.offset < natural * 8 || field.offset % (field_layout.align * 8) != 0 {
                        return Err(Error(format!(
                            "`{name}` requires unsupported packing or field alignment"
                        )));
                    }
                    let offset = field.offset / 8;
                    if offset > natural {
                        let mut padding = format!("__padding{index}");
                        while !names.insert(padding.clone()) {
                            padding.push('_');
                        }
                        projected.push((padding, ProjectedType::Padding(offset - size)));
                    }
                    size = offset;
                    size += field_layout.size;
                    align = align.max(field_layout.align);
                    projected.push((ident(&field.name)?, ty));
                }
                if layout.align < align
                    || layout.align > 32768
                    || !layout.align.is_positive()
                    || !(layout.align as u64).is_power_of_two()
                {
                    return Err(Error(format!("unsupported record alignment for `{name}`")));
                }
                let expected = Layout {
                    size: align_up(size, layout.align),
                    align: layout.align,
                };
                if expected != *layout {
                    return Err(Error(format!("projected layout differs for `{name}`")));
                }
                Item::Record {
                    fields: projected,
                    alignment: (layout.align > align).then_some(layout.align),
                }
            }
            DeclarationData::Alias { .. } => {
                let (ty, object) = self.alias(id, &mut BTreeSet::new())?;
                if object || matches!(ty, ProjectedType::Class(_)) {
                    return Err(Error(
                        "explicit interface alias emission is not implemented".into(),
                    ));
                }
                Item::Alias(ty)
            }
            DeclarationData::Enum {
                complete: true,
                repr,
                flags,
                variants,
                ..
            } => {
                let (repr, layout) = self.enum_repr(repr)?;
                let variants = variants
                    .iter()
                    .map(|(name, value)| {
                        let Value::Integer(value) = value else {
                            return Err(Error(format!("enum value unavailable for `{name}`")));
                        };
                        let value = if repr.starts_with('i') {
                            let shift = 64 - layout.size * 8;
                            (((*value << shift) as i64) >> shift).to_string()
                        } else {
                            value.to_string()
                        };
                        Ok((ident(name)?, value))
                    })
                    .collect::<Result<_, _>>()?;
                Item::Enum {
                    repr,
                    flags: *flags,
                    variants,
                }
            }
            DeclarationData::Function {
                canonical,
                ty: written,
                link_name,
                ..
            } => {
                let TypeKind::Function {
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
                let (library, import_name) =
                    if let Some(import) = self.options.imports.get(link_name) {
                        (import.library.clone(), import.name.clone())
                    } else {
                        (
                            self.options.library.clone().ok_or_else(|| {
                                Error(format!(
                                    "`{name}` requires an import library for `{link_name}`"
                                ))
                            })?,
                            link_name.clone(),
                        )
                    };
                let (params, result) = self.signature(id, 0)?;
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
                    link_name: import_name,
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
                    (
                        Value::Integer(value),
                        ProjectedType::Pointer { .. } | ProjectedType::PointerReference { .. },
                    ) => (*value as i64).to_string(),
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
        let mut names = BTreeSet::new();
        for (index, method) in methods.iter().enumerate() {
            if let Some(property) = &method.property
                && !match property.as_str() {
                    "propget" => method.name.starts_with("get_"),
                    "propput" => method.name.starts_with("put_"),
                    _ => false,
                }
            {
                return Err(Error(format!(
                    "`{name}::{}` has an unsupported MIDL property contract: {property}",
                    method.name
                )));
            }
            if !method.overrides.is_empty() {
                return Err(Error(format!(
                    "`{name}::{}` reuses an inherited virtual slot; override projection is not implemented",
                    method.name
                )));
            }
            if !names.insert(&method.name) {
                return Err(Error(format!(
                    "`{name}`: overloaded COM methods require native vtable ordering"
                )));
            }
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
            let (parameters, result) = self.signature(id, index + 1)?;
            let mut params = vec![];
            for (parameter, (attributes, ty)) in method.parameters.iter().zip(parameters) {
                params.push((attributes, ident(&parameter.name)?, ty));
            }
            if matches!(result, ProjectedType::Named(..)) {
                return Err(Error(format!(
                    "`{name}::{}`: by-value record results are not supported for COM methods",
                    method.name
                )));
            }
            if !matches!(result, ProjectedType::Void)
                && result.layout(self.resolved.snapshot.pointer_size).is_none()
            {
                return Err(Error(format!(
                    "`{name}::{}` has an unsupported result layout",
                    method.name
                )));
            }
            projected.push((
                ident(&method.name)?,
                method.property.is_some(),
                params,
                result,
            ));
        }
        Ok(Item::Interface {
            base,
            guid: guid.to_string(),
            methods: projected,
        })
    }

    fn parameters(
        &mut self,
        parameters: &[Type],
        annotations: &[Vec<String>],
    ) -> Result<Vec<(String, ProjectedType)>, Error> {
        assert_eq!(parameters.len(), annotations.len());
        let mut types = vec![];
        for parameter in parameters {
            let (ty, object) = self.lower(parameter, &mut BTreeSet::new())?;
            if object {
                return Err(Error("native interface objects require a pointer".into()));
            }
            if ty.layout(self.resolved.snapshot.pointer_size).is_none() {
                return Err(Error(
                    "by-value external parameter layout is not available".into(),
                ));
            }
            types.push(ty);
        }
        let attributes = annotations
            .iter()
            .enumerate()
            .map(|(index, _)| parameter_attributes(annotations, index, &types))
            .collect::<Result<Vec<_>, _>>()?;
        attributes
            .into_iter()
            .zip(types)
            .zip(annotations)
            .map(|((attributes, ty), annotations)| {
                let Some(kind) = string_kind(annotations, &ty)? else {
                    return Ok((attributes, ty));
                };
                let reference = self.options.string_references.get(&kind).ok_or_else(|| {
                    Error(format!(
                        "null-terminated {kind:?} parameter requires an explicit string binding"
                    ))
                })?;
                if let ProjectedType::PointerReference { name, .. } = &ty {
                    if *name != namespace_name(reference)? {
                        return Err(Error("conflicting typedef and string bindings".into()));
                    }
                    return Ok((attributes, ty));
                }
                let layout = ty.layout(self.resolved.snapshot.pointer_size);
                Ok((
                    attributes,
                    ProjectedType::Named(namespace_name(reference)?, layout),
                ))
            })
            .collect()
    }

    fn signature(
        &mut self,
        id: Id,
        slot: usize,
    ) -> Result<(Vec<(String, ProjectedType)>, ProjectedType), Error> {
        let group = self.groups[&id];
        let mut signature = None;
        for observation in group {
            let data = &self.resolved.snapshot.declarations[observation.0].data;
            let ty = match data {
                DeclarationData::Function { ty, .. } => ty,
                DeclarationData::Record { methods, .. } if !methods.is_empty() => {
                    &methods[slot - 1].ty
                }
                DeclarationData::Record { .. } => continue,
                _ => unreachable!(),
            };
            let TypeKind::Function {
                result, parameters, ..
            } = &ty.kind
            else {
                unreachable!()
            };
            let parameters = self.parameters(parameters, &self.resolved.annotations[&id][&slot])?;
            let (result, object) = self.lower(result, &mut BTreeSet::new())?;
            if object {
                return Err(Error("native interface objects require a pointer".into()));
            }
            let projected = (parameters, result);
            if let Some(previous) = &signature
                && previous != &projected
            {
                return Err(Error(format!(
                    "conflicting projected typedef contracts for `{}`",
                    self.resolved.snapshot.declarations[id.0].name
                )));
            }
            signature = Some(projected);
        }
        Ok(signature.unwrap())
    }

    fn typedef_reference(
        &mut self,
        canonical: &Type,
        reference: &TypeReference,
    ) -> Result<ProjectedType, Error> {
        if let TypeKind::Pointer(target) = &canonical.kind {
            let mutable = !target.qualifiers.constant;
            let string = match &target.kind {
                TypeKind::Builtin { kind, layout } => match (
                    kind.as_str(),
                    layout.as_ref().map(|layout| layout.size),
                    mutable,
                ) {
                    ("Char_S" | "SChar" | "Char_U" | "UChar", Some(1), true) => {
                        Some(StringKind::Ansi)
                    }
                    ("Char_S" | "SChar" | "Char_U" | "UChar", Some(1), false) => {
                        Some(StringKind::AnsiConst)
                    }
                    ("UShort" | "WChar" | "Char16", Some(2), true) => Some(StringKind::Wide),
                    ("UShort" | "WChar" | "Char16", Some(2), false) => Some(StringKind::WideConst),
                    _ => None,
                },
                _ => None,
            };
            return Ok(ProjectedType::PointerReference {
                name: namespace_name(reference)?,
                mutable,
                string,
            });
        }
        let (ProjectedType::Scalar(kind, layout), false) =
            self.lower(canonical, &mut BTreeSet::new())?
        else {
            return Err(Error(
                "external scalar typedef requires a supported non-void scalar".into(),
            ));
        };
        Ok(ProjectedType::ScalarReference(
            namespace_name(reference)?,
            kind,
            layout,
        ))
    }

    fn enum_repr(&mut self, repr: &Type) -> Result<(&'static str, Layout), Error> {
        let (ProjectedType::Scalar(kind, layout), false) =
            self.lower(repr, &mut BTreeSet::new())?
        else {
            return Err(Error(
                "enum representation requires an integer scalar".into(),
            ));
        };
        if !matches!(
            kind,
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64"
        ) {
            return Err(Error(
                "enum representation requires an integer scalar".into(),
            ));
        }
        Ok((kind, layout))
    }

    fn alias(
        &mut self,
        id: Id,
        visiting: &mut BTreeSet<Id>,
    ) -> Result<(ProjectedType, bool), Error> {
        if let Some(result) = self.aliases.get(&id) {
            return Ok(result.clone());
        }
        if !visiting.insert(id) {
            return Err(Error("cyclic native alias".into()));
        }
        let mut result = None;
        for observation in self.groups[&id] {
            let DeclarationData::Alias { target, .. } =
                &self.resolved.snapshot.declarations[observation.0].data
            else {
                unreachable!()
            };
            let projected = self.lower(target, visiting)?;
            if let Some(previous) = &result
                && previous != &projected
            {
                return Err(Error(format!(
                    "conflicting projected typedef contracts for `{}`",
                    self.resolved.snapshot.declarations[id.0].name
                )));
            }
            result = Some(projected);
        }
        visiting.remove(&id);
        let result = result.unwrap();
        self.aliases.insert(id, result.clone());
        Ok(result)
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
                if let DeclarationData::Alias { canonical, .. } = &declaration.data {
                    if let Some(reference) = self.options.references.get(&declaration.name)
                        && matches!(
                            canonical.kind,
                            TypeKind::Builtin { .. } | TypeKind::Pointer(_)
                        )
                    {
                        return Ok((self.typedef_reference(canonical, reference)?, false));
                    }
                    self.alias(id, aliases)?
                } else if let Some(reference) = self.options.references.get(&declaration.name) {
                    let name = namespace_name(reference)?;
                    if reference.kind == ReferenceKind::Interface {
                        (ProjectedType::Class(name), true)
                    } else {
                        (ProjectedType::Named(name, None), false)
                    }
                } else if let DeclarationData::Enum {
                    complete: true,
                    repr,
                    ..
                } = &declaration.data
                {
                    let (kind, layout) = self.enum_repr(repr)?;
                    self.schedule(id);
                    (
                        ProjectedType::ScalarReference(ident(&declaration.name)?, kind, layout),
                        false,
                    )
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
                    let name = self.name(id)?;
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

fn parameter_attributes(
    all_annotations: &[Vec<String>],
    index: usize,
    parameters: &[ProjectedType],
) -> Result<String, Error> {
    let ty = &parameters[index];
    let annotations = &all_annotations[index];
    let mut attributes = String::new();
    let mut sized = false;
    for annotation in annotations {
        if let Some((name, argument)) = annotation
            .split_once('(')
            .and_then(|(name, rest)| rest.strip_suffix(')').map(|argument| (name, argument)))
        {
            let (name, capacity, written) = match name {
                "_Out_writes_bytes_all_" => ("_Out_writes_bytes_", argument, Some(argument)),
                "_Out_writes_bytes_all_opt_" => {
                    ("_Out_writes_bytes_opt_", argument, Some(argument))
                }
                "_Out_writes_bytes_to_" | "_Out_writes_bytes_to_opt_" => {
                    let (capacity, written) = argument.split_once(',').ok_or_else(|| {
                        Error(format!("invalid output byte relationship: {annotation}"))
                    })?;
                    (
                        if name.ends_with("_opt_") {
                            "_Out_writes_bytes_opt_"
                        } else {
                            "_Out_writes_bytes_"
                        },
                        capacity,
                        Some(written),
                    )
                }
                _ => (name, argument, None),
            };
            let (direction, bytes, optional, output) = match name {
                "_In_reads_" => ("#[in] ", false, false, false),
                "_Out_writes_" => ("#[out] ", false, false, true),
                "_Inout_updates_" => ("#[in] #[out] ", false, false, true),
                "_In_reads_bytes_" => ("#[in] ", true, false, false),
                "_Out_writes_bytes_" => ("#[out] ", true, false, true),
                "_Inout_updates_bytes_" => ("#[in] #[out] ", true, false, true),
                "_In_reads_opt_" => ("#[in] ", false, true, false),
                "_Out_writes_opt_" => ("#[out] ", false, true, true),
                "_Inout_updates_opt_" => ("#[in] #[out] ", false, true, true),
                "_In_reads_bytes_opt_" => ("#[in] ", true, true, false),
                "_Out_writes_bytes_opt_" => ("#[out] ", true, true, true),
                "_Inout_updates_bytes_opt_" => ("#[in] #[out] ", true, true, true),
                _ => {
                    return Err(Error(format!(
                        "annotation projection is not implemented: {annotation}"
                    )));
                }
            };
            if sized {
                return Err(Error(
                    "multiple buffer-length annotations are not supported".into(),
                ));
            }
            let ProjectedType::Pointer {
                mutable,
                depth,
                target,
            } = ty
            else {
                return Err(Error(format!("{name} requires a buffer pointer")));
            };
            if output && !mutable {
                return Err(Error(format!("{name} requires a writable buffer")));
            }
            if !bytes && *depth == 1 && matches!(**target, ProjectedType::Void) {
                return Err(Error(format!("{name} requires a non-void element type")));
            }
            attributes.push_str(direction);
            if optional {
                attributes.push_str("#[opt] ");
            }
            attributes.push_str(&buffer_length(capacity.trim(), bytes, parameters)?);
            if let Some(written) = written {
                attributes.push_str(&written_bytes(written.trim(), parameters, all_annotations)?);
            }
            sized = true;
            continue;
        }
        attributes.push_str(match annotation.as_str() {
            "_In_" => "#[in] ",
            // A consumed interface pointer borrows an object, not a writable pointer slot.
            "_Out_" | "_Inout_" if matches!(ty, ProjectedType::Class(_)) => "#[in] ",
            "_Out_" => "#[out] ",
            "_Inout_" => "#[in] #[out] ",
            "_In_opt_"
                if matches!(
                    ty,
                    ProjectedType::Pointer { .. }
                        | ProjectedType::PointerReference { .. }
                        | ProjectedType::Class(_)
                ) =>
            {
                "#[in] #[opt] "
            }
            "_In_z_" => "#[in] ",
            "_In_opt_z_" => "#[in] #[opt] ",
            "_Out_z_" | "_Inout_z_" => {
                if !matches!(
                    ty,
                    ProjectedType::Pointer { mutable: true, .. }
                        | ProjectedType::PointerReference { mutable: true, .. }
                ) {
                    return Err(Error(format!(
                        "{annotation} requires a writable string pointer"
                    )));
                }
                if annotation == "_Out_z_" {
                    "#[out] "
                } else {
                    "#[in] #[out] "
                }
            }
            "_Out_opt_" | "_Inout_opt_" => {
                if matches!(ty, ProjectedType::Class(_)) {
                    attributes.push_str("#[in] #[opt] ");
                    continue;
                }
                if !matches!(
                    ty,
                    ProjectedType::Pointer { mutable: true, .. }
                        | ProjectedType::PointerReference { mutable: true, .. }
                ) {
                    return Err(Error(format!("{annotation} requires a writable pointer")));
                }
                if annotation == "_Out_opt_" {
                    "#[out] #[opt] "
                } else {
                    "#[in] #[out] #[opt] "
                }
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

fn written_bytes(
    argument: &str,
    parameters: &[ProjectedType],
    annotations: &[Vec<String>],
) -> Result<String, Error> {
    let (argument, dereference) = argument
        .strip_prefix('*')
        .map_or((argument, false), |value| (value.trim(), true));
    let index = argument
        .strip_prefix('$')
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
        .ok_or_else(|| Error(format!("unsupported written byte count `{argument}`")))?;
    let ty = parameters
        .get(index as usize)
        .ok_or_else(|| Error("written byte count parameter is out of range".into()))?;
    let scalar = if dereference {
        let ProjectedType::Pointer {
            mutable: true,
            depth: 1,
            target,
        } = ty
        else {
            return Err(Error(
                "written byte count requires a writable integer pointer".into(),
            ));
        };
        let count_annotations = &annotations[index as usize];
        if !count_annotations
            .iter()
            .any(|value| matches!(value.as_str(), "_Out_" | "_Inout_"))
            || count_annotations
                .iter()
                .any(|value| value.contains("_opt_"))
        {
            return Err(Error(
                "written byte count requires a nonoptional output parameter".into(),
            ));
        }
        target.scalar_kind()
    } else {
        ty.scalar_kind()
    };
    if !matches!(
        scalar,
        Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64")
    ) {
        return Err(Error("written byte count must be an integer".into()));
    }
    Ok(format!(
        "#[written_bytes(BytesParamIndex = {index}, Dereference = {dereference})] "
    ))
}

fn string_kind(annotations: &[String], ty: &ProjectedType) -> Result<Option<StringKind>, Error> {
    if !annotations.iter().any(|annotation| {
        matches!(
            annotation.as_str(),
            "_In_z_" | "_In_opt_z_" | "_Out_z_" | "_Inout_z_"
        )
    }) {
        return Ok(None);
    }
    if let ProjectedType::PointerReference { string, .. } = ty {
        return string.map(Some).ok_or_else(|| {
            Error("null-terminated strings require a single character pointer".into())
        });
    }
    let ProjectedType::Pointer {
        mutable,
        depth: 1,
        target,
    } = ty
    else {
        return Err(Error(
            "null-terminated strings require a single character pointer".into(),
        ));
    };
    Ok(Some(match (target.scalar_kind(), mutable) {
        (Some("i8" | "u8"), true) => StringKind::Ansi,
        (Some("i8" | "u8"), false) => StringKind::AnsiConst,
        (Some("u16"), true) => StringKind::Wide,
        (Some("u16"), false) => StringKind::WideConst,
        _ => {
            return Err(Error(
                "null-terminated strings require 8-bit or unsigned 16-bit characters".into(),
            ));
        }
    }))
}

fn buffer_length(
    argument: &str,
    bytes: bool,
    parameters: &[ProjectedType],
) -> Result<String, Error> {
    if let Some(index) = argument
        .strip_prefix('$')
        .filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
    {
        let Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64") =
            usize::try_from(index)
                .ok()
                .and_then(|index| parameters.get(index))
                .and_then(ProjectedType::scalar_kind)
        else {
            return Err(Error(format!(
                "buffer length `{argument}` must refer to an integer parameter"
            )));
        };
        let attribute = if bytes { "size_param" } else { "len_param" };
        return Ok(format!("#[{attribute}({index})] "));
    }
    if !bytes
        && argument.bytes().all(|byte| byte.is_ascii_digit())
        && (argument == "0" || !argument.starts_with('0'))
        && let Ok(value) = argument.parse::<i32>()
    {
        return Ok(format!("#[len_const({value})] "));
    }
    Err(Error(format!("unsupported buffer length `{argument}`")))
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
