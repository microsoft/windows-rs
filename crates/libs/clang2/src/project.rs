use super::*;

mod record;
use record::*;

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

/// A caller-supplied integer typedef contract, checked against the native target layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerSized {
    Signed,
    Unsigned,
}

impl PointerSized {
    fn name(self) -> &'static str {
        match self {
            Self::Signed => "isize",
            Self::Unsigned => "usize",
        }
    }

    fn accepts(self, ty: &Type, pointer_size: i64) -> bool {
        let TypeKind::Builtin {
            kind,
            layout: Some(layout),
        } = &ty.kind
        else {
            return false;
        };
        let integer = match self {
            Self::Signed => matches!(kind.as_str(), "Int" | "Long" | "LongLong"),
            Self::Unsigned => matches!(kind.as_str(), "UInt" | "ULong" | "ULongLong"),
        };
        integer && layout.size == pointer_size && layout.align == pointer_size
    }
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
    pub target: ImportTarget,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportTarget {
    Name(String),
    Ordinal(u16),
}

pub struct ProjectionOptions {
    pub namespace: String,
    pub library: Option<String>,
    /// Exact DLL imports. Unmapped functions require the explicit `library` fallback.
    pub imports: BTreeMap<String, FunctionImport>,
    /// External record/enum bindings and trusted scalar or pointer typedef contracts.
    pub references: BTreeMap<String, TypeReference>,
    /// Explicit native integer typedefs that use `isize` or `usize` in RDL.
    pub pointer_sized: BTreeMap<String, PointerSized>,
    /// Omit inline function roots, including header-only overloads of imported functions.
    pub exclude_inline_functions: bool,
    /// Trusted pointer-sized value types for SAL-annotated null-terminated strings.
    pub string_references: BTreeMap<StringKind, TypeReference>,
    /// Publish UUID-bearing opaque native classes as GUID constants instead of class declarations.
    pub class_guids: Option<TypeReference>,
}

impl ProjectionOptions {
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            library: None,
            imports: BTreeMap::new(),
            references: BTreeMap::new(),
            pointer_sized: BTreeMap::new(),
            exclude_inline_functions: false,
            string_references: BTreeMap::new(),
            class_guids: None,
        }
    }
}

/// Closed projected declarations. Rendering does not access the native graph.
#[derive(Debug)]
pub struct Plan {
    namespace: String,
    items: BTreeMap<String, Item>,
    omitted: BTreeMap<String, String>,
    owners: BTreeMap<String, String>,
    annotations: BTreeMap<String, BTreeMap<String, String>>,
}

impl Plan {
    /// Selected roots omitted by an explicit projection policy, with their reasons.
    pub fn omitted(&self) -> &BTreeMap<String, String> {
        &self.omitted
    }

    pub fn rdl(&self) -> String {
        self.render(self.items.iter())
    }

    /// Renders each planned declaration once, in its owning source header partition.
    ///
    /// Complete definitions outrank forward declarations; equal evidence uses the first
    /// lexicographic source path. A selected alias that names a record also owns its output.
    pub fn rdl_by_header(&self) -> Result<BTreeMap<String, String>, Error> {
        let mut partitions: BTreeMap<&str, Vec<_>> = BTreeMap::new();
        for (name, item) in &self.items {
            if self.owners[name].is_empty() {
                return Err(Error(format!(
                    "source header ownership is unavailable for `{name}`"
                )));
            }
            partitions
                .entry(&self.owners[name])
                .or_default()
                .push((name, item));
        }
        Ok(partitions
            .into_iter()
            .map(|(header, items)| (header.to_string(), self.render(items.into_iter())))
            .collect())
    }

    fn render<'a>(&self, items: impl Iterator<Item = (&'a String, &'a Item)>) -> String {
        let mut namespaces = self.namespace.rsplit('.');
        let mut output = format!("#[win32]\nmod {} {{\n", namespaces.next().unwrap());
        for (name, item) in items {
            let annotations = self.annotations.get(name);
            if let Some(attributes) = annotations.and_then(|annotations| annotations.get("")) {
                writeln!(output, "    {}", attributes.trim_end()).unwrap();
            }
            match item {
                Item::Record(record) => {
                    output.push_str("    ");
                    record.write(&mut output, Some(name), 4);
                    output.push('\n');
                }
                Item::Opaque { union } => {
                    writeln!(
                        output,
                        "    {} {name} {{}}",
                        if *union { "union" } else { "struct" }
                    )
                    .unwrap();
                }
                Item::Alias(ty) => writeln!(output, "    type {name} = {};", ty.text()).unwrap(),
                Item::Class { guid } => {
                    writeln!(output, "    #[guid({})]\n    class {name};", rdl_guid(guid)).unwrap();
                }
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
                        if let Some(attributes) =
                            annotations.and_then(|annotations| annotations.get(variant))
                        {
                            writeln!(output, "        {}", attributes.trim_end()).unwrap();
                        }
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
                    match link_name {
                        ImportTarget::Name(name) => {
                            writeln!(output, "    #[library({library:?}, import = {name:?})]")
                                .unwrap();
                        }
                        ImportTarget::Ordinal(ordinal) => {
                            writeln!(output, "    #[library({library:?}, ordinal = {ordinal})]")
                                .unwrap();
                        }
                    }
                    write!(output, "    extern {abi:?} fn {name}(").unwrap();
                    for (index, (attributes, parameter, ty)) in parameters.iter().enumerate() {
                        if index != 0 {
                            output.push_str(", ");
                        }
                        write!(output, "{attributes}{parameter}: {}", ty.text()).unwrap();
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
                Item::StringConstant { encoding, value } => {
                    writeln!(
                        output,
                        "    #[encoding({encoding:?})]\n    const {name}: String = {value:?};"
                    )
                    .unwrap();
                }
                Item::GuidConstant { ty, guid, pid } => {
                    write!(
                        output,
                        "    #[guid(0x{guid:032x})]\n    const {name}: {}",
                        ty.text()
                    )
                    .unwrap();
                    if let Some(pid) = pid {
                        write!(output, " = {pid}").unwrap();
                    }
                    output.push_str(";\n");
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
                        if let Some(attributes) =
                            annotations.and_then(|annotations| annotations.get(method))
                        {
                            writeln!(output, "        {}", attributes.trim_end()).unwrap();
                        }
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
                Item::Callback {
                    abi,
                    parameters,
                    result,
                } => {
                    write!(output, "    extern {abi:?} fn {name}(").unwrap();
                    for (index, (attributes, parameter, ty)) in parameters.iter().enumerate() {
                        if index > 0 {
                            output.push_str(", ");
                        }
                        write!(output, "{attributes}{parameter}: {}", ty.text()).unwrap();
                    }
                    output.push(')');
                    if !matches!(result, ProjectedType::Void) {
                        write!(output, " -> {}", result.text()).unwrap();
                    }
                    output.push_str(";\n");
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

    fn unproven_record(
        &self,
        ty: &ProjectedType,
        checked: &mut BTreeMap<String, Option<&'static str>>,
    ) -> Option<&'static str> {
        const ADJUSTED: &str =
            "by-value calls with adjusted record layouts require native ABI coverage";
        let ty = ty.contract();
        match ty {
            ProjectedType::Padding(_) => return Some(ADJUSTED),
            ProjectedType::InlineRecord(_) => {
                return Some("by-value unions or anonymous records require native ABI coverage");
            }
            ProjectedType::RecordReference(..) => {
                return Some("by-value external record calls require native ABI coverage");
            }
            ProjectedType::Array { element, .. } => return self.unproven_record(element, checked),
            _ => {}
        }
        let ProjectedType::Named(name, layout) = ty else {
            return None;
        };
        if layout.is_none() {
            return Some("by-value incomplete record calls require a native layout");
        }
        if let Some(adjusted) = checked.get(name) {
            return *adjusted;
        }
        let reason = match self.items.get(name) {
            Some(Item::Record(record)) => {
                if record.kind == RecordKind::Union || record.anonymous_fields {
                    Some("by-value unions or anonymous records require native ABI coverage")
                } else if record.alignment.is_some() {
                    Some(ADJUSTED)
                } else {
                    record
                        .fields
                        .iter()
                        .find_map(|(_, _, ty)| self.unproven_record(ty, checked))
                }
            }
            _ => None,
        };
        checked.insert(name.clone(), reason);
        reason
    }

    fn validate_calls(&self) -> Result<(), Error> {
        let mut checked = BTreeMap::new();
        for (name, item) in &self.items {
            let reason = match item {
                Item::Function {
                    parameters, result, ..
                }
                | Item::Callback {
                    parameters, result, ..
                } => parameters
                    .iter()
                    .find_map(|(_, _, ty)| self.unproven_record(ty, &mut checked))
                    .or_else(|| self.unproven_record(result, &mut checked)),
                Item::Interface { methods, .. } => {
                    methods.iter().find_map(|(_, _, parameters, result)| {
                        parameters
                            .iter()
                            .find_map(|(_, _, ty)| self.unproven_record(ty, &mut checked))
                            .or_else(|| self.unproven_record(result, &mut checked))
                    })
                }
                _ => None,
            };
            if let Some(reason) = reason {
                return Err(Error(format!("`{name}`: {reason}")));
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
    Opaque {
        union: bool,
    },
    StringConstant {
        encoding: &'static str,
        value: String,
    },
    Class {
        guid: String,
    },
    Record(Record),
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
        link_name: ImportTarget,
        parameters: Vec<(String, String, ProjectedType)>,
        result: ProjectedType,
    },
    Callback {
        abi: &'static str,
        parameters: Vec<(String, String, ProjectedType)>,
        result: ProjectedType,
    },
    Constant {
        ty: ProjectedType,
        value: String,
    },
    GuidConstant {
        ty: ProjectedType,
        guid: u128,
        pid: Option<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum ProjectedType {
    Padding(i64),
    Void,
    Scalar(&'static str, Layout),
    ScalarReference(String, &'static str, Layout),
    Alias(String, Box<Self>),
    PointerReference {
        name: String,
        mutable: bool,
        string: Option<StringKind>,
    },
    Named(String, Option<Layout>),
    InlineRecord(Box<Record>),
    RecordReference(String, Option<Layout>),
    Array {
        element: Box<Self>,
        length: i64,
    },
    Class(String),
    Callback(String),
    Callable(String),
    Pointer {
        mutable: bool,
        depth: usize,
        target: Box<Self>,
    },
}

impl ProjectedType {
    fn contract(&self) -> &Self {
        match self {
            Self::Alias(_, target) => target.contract(),
            _ => self,
        }
    }

    fn scalar_kind(&self) -> Option<&'static str> {
        match self.contract() {
            Self::Scalar(kind, _) | Self::ScalarReference(_, kind, _) => Some(kind),
            _ => None,
        }
    }

    fn text(&self) -> String {
        match self {
            // Padding can remain uninitialized when native code copies a record.
            Self::Padding(size) => format!("union {{ bytes: [u8; {size}], uninit: [u8; 0], }}"),
            Self::InlineRecord(record) => {
                let mut output = String::new();
                record.write(&mut output, None, 0);
                output
            }
            Self::Void => "void".into(),
            Self::Scalar(name, _) => (*name).into(),
            Self::Named(name, _)
            | Self::Alias(name, _)
            | Self::RecordReference(name, _)
            | Self::Class(name)
            | Self::Callback(name)
            | Self::Callable(name)
            | Self::ScalarReference(name, ..) => name.clone(),
            Self::Array { element, length } => format!("[{}; {length}]", element.text()),
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
            Self::Alias(_, target) => target.layout(pointer_size),
            Self::Padding(size) => Some(Layout {
                size: *size,
                align: 1,
            }),
            Self::Void | Self::Callable(_) => None,
            Self::Scalar(_, layout) | Self::ScalarReference(_, _, layout) => Some(layout.clone()),
            Self::Named(_, layout) | Self::RecordReference(_, layout) => layout.clone(),
            Self::InlineRecord(record) => Some(record.layout.clone()),
            Self::Array { element, length } => {
                let layout = element.layout(pointer_size)?;
                Some(Layout {
                    size: layout.size.checked_mul(*length)?,
                    align: layout.align,
                })
            }
            Self::Class(_)
            | Self::Callback(_)
            | Self::Pointer { .. }
            | Self::PointerReference { .. } => Some(Layout {
                size: pointer_size,
                align: pointer_size,
            }),
        }
    }
}

/// Validated projection policy shared by independent plans. Root-specific names and caches
/// belong to each plan, so assessing one subset cannot affect another.
pub struct Projection<'a, 's> {
    resolved: &'a Resolved<'s>,
    options: &'a ProjectionOptions,
    namespace: String,
}

impl Projection<'_, '_> {
    pub fn project(&self) -> Result<Plan, Error> {
        let mut roots: Vec<_> = self.resolved.roots.values().flatten().copied().collect();
        roots.sort();
        self.resolved
            .project_ids(self.options, &self.namespace, &roots)
    }

    pub fn project_roots(&self, names: &[&str]) -> Result<Plan, Error> {
        if names.is_empty() {
            return Err(Error("projection requires selected roots".into()));
        }
        let mut roots = vec![];
        for name in names {
            let selected = self
                .resolved
                .roots
                .get(name)
                .ok_or_else(|| Error(format!("`{name}` is not a captured root")))?;
            roots.extend(selected);
        }
        roots.sort();
        roots.dedup();
        self.resolved
            .project_ids(self.options, &self.namespace, &roots)
    }
}

impl<'s> Resolved<'s> {
    pub fn project(&self, options: &ProjectionOptions) -> Result<Plan, Error> {
        self.projection(options)?.project()
    }

    /// Projects a subset of captured roots after the complete snapshot has passed resolution.
    ///
    /// Dependencies remain included. This cannot bypass native conflicts or select uncaptured
    /// names. Individually successful subsets can still conflict when projected together.
    pub fn project_roots(
        &self,
        options: &ProjectionOptions,
        names: &[&str],
    ) -> Result<Plan, Error> {
        self.projection(options)?.project_roots(names)
    }

    pub fn projection<'a>(
        &'a self,
        options: &'a ProjectionOptions,
    ) -> Result<Projection<'a, 's>, Error> {
        if !self.snapshot.target.contains("-windows-") {
            return Err(Error(
                "projection currently supports Windows targets only".into(),
            ));
        }
        let namespace = namespace(&options.namespace)?;
        if let Some(reference) = &options.class_guids {
            if reference.kind != ReferenceKind::Value {
                return Err(Error(
                    "class GUID constants require a metadata value type".into(),
                ));
            }
            namespace_name(reference)?;
        }
        for (name, contract) in &options.pointer_sized {
            if options.references.contains_key(name) {
                return Err(Error(format!(
                    "conflicting pointer-sized and external bindings for `{name}`"
                )));
            }
            for id in self.names.get(name.as_str()).into_iter().flatten() {
                for observation in &self.groups[id] {
                    let declaration = &self.snapshot.declarations[observation.0];
                    if !matches!(
                        &declaration.data,
                        DeclarationData::Alias { canonical, .. }
                            if contract.accepts(canonical, self.snapshot.pointer_size)
                    ) {
                        return Err(Error(format!(
                            "pointer-sized binding `{name}` requires a native {} integer typedef with pointer size and alignment",
                            contract.name()
                        )));
                    }
                }
            }
        }
        for reference in options.string_references.values() {
            if reference.kind != ReferenceKind::Value {
                return Err(Error(
                    "string bindings must reference metadata value types".into(),
                ));
            }
            namespace_name(reference)?;
        }
        for (name, reference) in &options.references {
            for id in self.names.get(name.as_str()).into_iter().flatten() {
                let declaration = &self.snapshot.declarations[id.0];
                let mut binding = declaration;
                if let DeclarationData::Alias { canonical, .. } = &binding.data
                    && let TypeKind::Named(target) = canonical.kind
                    && self.snapshot.declarations[target.0].name == declaration.name
                {
                    binding = &self.snapshot.declarations[target.0];
                }
                if matches!(binding.data, DeclarationData::Enum { .. })
                    && reference.kind != ReferenceKind::Value
                {
                    return Err(Error(format!(
                        "enum binding `{name}` requires a metadata value type"
                    )));
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
        Ok(Projection {
            resolved: self,
            options,
            namespace,
        })
    }

    fn project_ids(
        &self,
        options: &ProjectionOptions,
        namespace: &str,
        roots: &[Id],
    ) -> Result<Plan, Error> {
        let (excluded, roots): (Vec<_>, Vec<_>) = roots.iter().copied().partition(|root| {
            options.exclude_inline_functions
                && self.groups[&self.representatives[root.0]].iter().all(|id| {
                    matches!(
                        self.snapshot.declarations[id.0].data,
                        DeclarationData::Function { inline: true, .. }
                    )
                })
        });
        let omitted = if excluded.is_empty() {
            BTreeMap::new()
        } else {
            let included: BTreeSet<_> = roots
                .iter()
                .map(|id| self.snapshot.declarations[id.0].name.as_str())
                .collect();
            excluded
                .iter()
                .map(|id| &self.snapshot.declarations[id.0].name)
                .filter(|name| !included.contains(name.as_str()))
                .map(|name| (name.clone(), "inline function excluded by policy".into()))
                .collect()
        };
        let mut builder = Builder {
            resolved: self,
            options,
            aliases: BTreeMap::new(),
            names: BTreeMap::new(),
            owners: BTreeMap::new(),
            pending: VecDeque::new(),
            scheduled: BTreeSet::new(),
            plan: Plan {
                annotations: BTreeMap::new(),
                namespace: namespace.into(),
                items: BTreeMap::new(),
                omitted,
                owners: BTreeMap::new(),
            },
        };
        let selected: BTreeSet<_> = roots
            .iter()
            .filter(|root| {
                !matches!(
                    self.snapshot.declarations[root.0].data,
                    DeclarationData::Record { unnamed: true, .. }
                )
            })
            .map(|root| self.representatives[root.0])
            .collect();
        for root in &roots {
            let alias = &self.snapshot.declarations[root.0];
            if let DeclarationData::Alias { canonical, .. } = &alias.data
                && self.annotations[&self.representatives[root.0]]
                    .own
                    .sal
                    .is_empty()
                && self.annotations[&self.representatives[root.0]]
                    .own
                    .midl
                    .is_empty()
                && let TypeKind::Named(target) = canonical.kind
            {
                let target = self.representatives[target.0];
                if matches!(
                    self.snapshot.declarations[target.0].data,
                    DeclarationData::Record { .. }
                ) && !selected.contains(&target)
                {
                    let name = ident(&alias.name)?;
                    if let Some(previous) = builder.names.insert(target, name.clone())
                        && previous != name
                    {
                        return Err(Error(
                            "multiple selected aliases name the same record".into(),
                        ));
                    }
                    builder
                        .owners
                        .insert(target, builder.owner(self.representatives[root.0]));
                }
            }
        }
        for root in &roots {
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
    aliases: BTreeMap<Id, (ProjectedType, bool)>,
    names: BTreeMap<Id, String>,
    owners: BTreeMap<Id, String>,
    pending: VecDeque<Id>,
    scheduled: BTreeSet<Id>,
    plan: Plan,
}

impl<'s> Builder<'_, 's> {
    fn owner(&self, id: Id) -> String {
        if let Some(owner) = self.owners.get(&id) {
            return owner.clone();
        }
        self.resolved.groups[&id]
            .iter()
            .map(|id| &self.resolved.snapshot.declarations[id.0])
            .min_by_key(|declaration| {
                (
                    std::cmp::Reverse(declaration.data.evidence_rank()),
                    &declaration.owner,
                )
            })
            .unwrap()
            .owner
            .clone()
    }

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
            let annotations = &self.resolved.annotations[&id].own;
            if !annotations.sal.is_empty() || !annotations.midl.is_empty() {
                return Err(Error(format!(
                    "annotated record alias `{}` requires a separate declaration",
                    declaration.name
                )));
            }
            self.schedule(target);
            return Ok(());
        }
        let name = self.name(id)?;
        let native_annotations = &self.resolved.annotations[&id];
        let mut annotations = BTreeMap::new();
        let own = source_attributes(&native_annotations.own);
        if !own.is_empty() {
            annotations.insert(String::new(), own);
        }
        if let DeclarationData::Record { methods, .. } = &declaration.data {
            for (method, source) in methods.iter().zip(&native_annotations.methods) {
                let attributes = source_attributes(source);
                if !attributes.is_empty() {
                    annotations.insert(ident(&method.name)?, attributes);
                }
            }
        }
        if let DeclarationData::Enum { variants, .. } = &declaration.data {
            for ((name, _), source) in variants.iter().zip(&native_annotations.fields) {
                let attributes = source_attributes(source);
                if !attributes.is_empty() {
                    annotations.insert(ident(name)?, attributes);
                }
            }
        }
        if !annotations.is_empty() {
            self.plan.annotations.insert(name.clone(), annotations);
        }
        let guid = self.resolved.guids.get(&id).copied();
        let item = match &declaration.data {
            DeclarationData::Record {
                kind,
                complete: false,
                layout: None,
                fields,
                bases,
                methods,
                ..
            } if matches!(kind, RecordKind::Struct | RecordKind::Union)
                && fields.is_empty()
                && bases.is_empty()
                && methods.is_empty() =>
            {
                Item::Opaque {
                    union: *kind == RecordKind::Union,
                }
            }
            DeclarationData::Record {
                kind,
                complete: false,
                fields,
                bases,
                methods,
                ..
            } if *kind == RecordKind::Class
                && fields.is_empty()
                && bases.is_empty()
                && methods.is_empty()
                && guid.is_some() =>
            {
                if let Some(reference) = &self.options.class_guids {
                    Item::GuidConstant {
                        ty: ProjectedType::Named(namespace_name(reference)?, None),
                        guid: u128::from_str_radix(&guid.unwrap().replace('-', ""), 16).unwrap(),
                        pid: None,
                    }
                } else {
                    Item::Class {
                        guid: guid.unwrap().into(),
                    }
                }
            }
            DeclarationData::Record {
                complete: true,
                fields,
                bases,
                methods,
                ..
            } if fields.is_empty() && !methods.is_empty() && guid.is_some() => {
                self.interface(id, &name, guid.unwrap(), bases, methods)?
            }
            DeclarationData::Record {
                kind,
                complete: true,
                layout: Some(_),
                bases,
                methods,
                ..
            } if matches!(kind, RecordKind::Struct | RecordKind::Union)
                && bases.is_empty()
                && methods.is_empty() =>
            {
                Item::Record(self.record(id)?)
            }
            DeclarationData::Alias { target, .. }
                if matches!(
                    target.kind,
                    TypeKind::Function { .. } | TypeKind::Pointer(_)
                ) && self.function_type(target).is_ok() =>
            {
                let ty = self.function_type(target)?;
                let TypeKind::Function {
                    convention,
                    prototype: true,
                    variadic: false,
                    ..
                } = &ty.kind
                else {
                    return Err(Error(
                        "only fixed-prototype callbacks can be projected".into(),
                    ));
                };
                let abi = calling_convention(*convention)?;
                let (parameters, result) = self.signature(id, 0)?;
                Item::Callback {
                    abi,
                    parameters,
                    result,
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
                let TypeKind::Function { convention, .. } = &canonical.kind else {
                    unreachable!()
                };
                let abi = calling_convention(*convention)?;
                let (library, import_name) =
                    if let Some(import) = self.options.imports.get(link_name) {
                        (import.library.clone(), import.target.clone())
                    } else {
                        (
                            self.options.library.clone().ok_or_else(|| {
                                Error(format!(
                                    "`{name}` requires an import library for `{link_name}`"
                                ))
                            })?,
                            ImportTarget::Name(link_name.clone()),
                        )
                    };
                if let ImportTarget::Name(import_name) = &import_name
                    && (import_name.is_empty()
                        || import_name.contains('\0')
                        || import_name.starts_with('#'))
                {
                    return Err(Error(format!(
                        "DLL export name cannot be represented in metadata: {import_name:?}"
                    )));
                }
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
            DeclarationData::Variable {
                ty: native_ty,
                value,
                ..
            } => {
                let mut projected = None;
                for observation in &self.resolved.groups[&id] {
                    let DeclarationData::Variable { ty, .. } =
                        &self.resolved.snapshot.declarations[observation.0].data
                    else {
                        unreachable!()
                    };
                    let value = self.lower(ty, &mut BTreeSet::new())?;
                    if let Some(previous) = &projected
                        && previous != &value
                    {
                        return Err(Error(format!(
                            "conflicting projected typedef contracts for `{name}`"
                        )));
                    }
                    projected = Some(value);
                }
                let (ty, object) = projected.unwrap();
                if object || matches!(ty, ProjectedType::Class(_)) {
                    self.plan.omitted.insert(
                        declaration.name.clone(),
                        "interface values are not metadata constants".into(),
                    );
                    return Ok(());
                }
                if matches!(value, Value::None) {
                    return Err(Error(format!(
                        "initializer unavailable for `{name}`; declaration-only data cannot be emitted as a constant"
                    )));
                }
                if let Value::String { encoding, units } = value {
                    if let ProjectedType::Array { length, .. } = ty.contract()
                        && *length != units.len() as i64 + 1
                    {
                        return Err(Error(format!(
                            "string constant `{name}` has padded or truncated storage"
                        )));
                    }
                    let (encoding, value) = match encoding {
                        StringEncoding::Narrow | StringEncoding::Utf8 => {
                            let bytes = units
                                .iter()
                                .map(|unit| u8::try_from(*unit))
                                .collect::<Result<Vec<_>, _>>()
                                .map_err(|_| {
                                    Error(format!("string constant `{name}` exceeds byte storage"))
                                })?;
                            let value = String::from_utf8(bytes).map_err(|_| {
                                Error(format!("string constant `{name}` is not valid UTF-8"))
                            })?;
                            ("ansi", value)
                        }
                        StringEncoding::Utf16 => {
                            let units = units
                                .iter()
                                .map(|unit| u16::try_from(*unit))
                                .collect::<Result<Vec<_>, _>>()
                                .map_err(|_| {
                                    Error(format!(
                                        "string constant `{name}` exceeds UTF-16 storage"
                                    ))
                                })?;
                            let value = String::from_utf16(&units).map_err(|_| {
                                Error(format!(
                                    "string constant `{name}` contains unpaired UTF-16 surrogates"
                                ))
                            })?;
                            ("utf-16", value)
                        }
                        StringEncoding::Utf32 => {
                            return Err(Error(format!(
                                "UTF-32 string constant `{name}` has no supported RDL encoding"
                            )));
                        }
                    };
                    Item::StringConstant { encoding, value }
                } else if matches!(value, Value::Aggregate(_)) {
                    let (guid, pid) = self.guid_constant(native_ty, value).ok_or_else(|| {
                        Error(format!(
                            "aggregate constant representation is not supported for `{name}`"
                        ))
                    })?;
                    Item::GuidConstant { ty, guid, pid }
                } else {
                    let value = match (value, ty.contract()) {
                        (
                            Value::Integer(value),
                            ProjectedType::Scalar(name, _)
                            | ProjectedType::ScalarReference(_, name, _),
                        ) => {
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
            }
            _ => return Err(Error(format!("projection is not implemented for `{name}`"))),
        };
        if self.plan.items.insert(name.clone(), item).is_some() {
            return Err(Error(format!(
                "multiple native entities project to `{name}`"
            )));
        }
        self.plan.owners.insert(name, self.owner(id));
        Ok(())
    }

    fn canonical<'t>(&'t self, mut ty: &'t Type) -> &'t Type {
        while let TypeKind::Named(id) = ty.kind {
            let id = self.resolved.representatives[id.0];
            if let DeclarationData::Alias { canonical, .. } =
                &self.resolved.snapshot.declarations[id.0].data
            {
                ty = canonical;
            } else {
                break;
            }
        }
        ty
    }

    fn constant_fields<'t>(&'t self, ty: &'t Type, size: i64) -> Option<&'t [Field]> {
        let TypeKind::Named(id) = self.canonical(ty).kind else {
            return None;
        };
        let id = self.resolved.representatives[id.0];
        let DeclarationData::Record {
            kind,
            complete: true,
            layout: Some(layout),
            fields,
            bases,
            methods,
            ..
        } = &self.resolved.snapshot.declarations[id.0].data
        else {
            return None;
        };
        (*kind == RecordKind::Struct
            && bases.is_empty()
            && methods.is_empty()
            && *layout == Layout { size, align: 4 }
            && fields.iter().all(|field| field.bit_width.is_none()))
        .then_some(fields)
    }

    fn unsigned_constant_type(&self, ty: &Type, size: i64) -> bool {
        let TypeKind::Builtin {
            kind,
            layout: Some(layout),
        } = &self.canonical(ty).kind
        else {
            return false;
        };
        *layout == Layout { size, align: size }
            && matches!(
                (size, kind.as_str()),
                (1, "UChar") | (2, "UShort") | (4, "UInt" | "ULong")
            )
    }

    fn guid_value(&self, ty: &Type, value: &Value) -> Option<u128> {
        let [a, b, c, d] = self.constant_fields(ty, 16)? else {
            return None;
        };
        if [a.offset, b.offset, c.offset, d.offset] != [0, 32, 48, 64]
            || !self.unsigned_constant_type(&a.ty, 4)
            || !self.unsigned_constant_type(&b.ty, 2)
            || !self.unsigned_constant_type(&c.ty, 2)
        {
            return None;
        }
        let TypeKind::Array {
            length: Some(8),
            element,
        } = &self.canonical(&d.ty).kind
        else {
            return None;
        };
        if !self.unsigned_constant_type(element, 1) {
            return None;
        }
        let Value::Aggregate(values) = value else {
            return None;
        };
        let [
            Value::Integer(a),
            Value::Integer(b),
            Value::Integer(c),
            Value::Aggregate(d),
        ] = values.as_slice()
        else {
            return None;
        };
        if *a > u32::MAX as u64 || *b > u16::MAX as u64 || *c > u16::MAX as u64 || d.len() != 8 {
            return None;
        }
        let mut value = (*a as u128) << 96 | (*b as u128) << 80 | (*c as u128) << 64;
        for (index, byte) in d.iter().enumerate() {
            let Value::Integer(byte) = byte else {
                return None;
            };
            if *byte > u8::MAX as u64 {
                return None;
            }
            value |= (*byte as u128) << (56 - index * 8);
        }
        Some(value)
    }

    fn guid_constant(&self, ty: &Type, value: &Value) -> Option<(u128, Option<u32>)> {
        if let Some(guid) = self.guid_value(ty, value) {
            return Some((guid, None));
        }
        let [fmtid, pid] = self.constant_fields(ty, 20)? else {
            return None;
        };
        if fmtid.offset != 0 || pid.offset != 128 || !self.unsigned_constant_type(&pid.ty, 4) {
            return None;
        }
        let Value::Aggregate(values) = value else {
            return None;
        };
        let [guid, Value::Integer(pid)] = values.as_slice() else {
            return None;
        };
        Some((
            self.guid_value(&fmtid.ty, guid)?,
            Some(u32::try_from(*pid).ok()?),
        ))
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
            if matches!(result.contract(), ProjectedType::Named(..)) {
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
                parameters,
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
        source: &[SourceAnnotations],
    ) -> Result<Vec<(String, ProjectedType)>, Error> {
        let annotations: Vec<_> = source.iter().map(SourceAnnotations::lowered).collect();
        if parameters.len() != annotations.len() {
            return Err(Error(
                "callable parameter annotation evidence is incomplete".into(),
            ));
        }
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
            .map(|(index, _)| {
                let mut attributes = source_attributes(&source[index]);
                attributes.push_str(&parameter_attributes(&annotations, index, &types)?);
                Ok(attributes)
            })
            .collect::<Result<Vec<_>, _>>()?;
        attributes
            .into_iter()
            .zip(types)
            .zip(annotations)
            .map(|((attributes, ty), annotations)| {
                let Some(kind) = string_kind(&annotations, &ty)? else {
                    return Ok((attributes, ty));
                };
                let Some(reference) = self.options.string_references.get(&kind) else {
                    return Ok((attributes, ty));
                };
                if let ProjectedType::PointerReference { name, .. } = ty.contract() {
                    if *name != namespace_name(reference)? {
                        return Err(Error("conflicting typedef and string bindings".into()));
                    }
                    return Ok((attributes, ty));
                }
                if matches!(ty, ProjectedType::Alias(..)) {
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
    ) -> Result<(Vec<(String, String, ProjectedType)>, ProjectedType), Error> {
        let group = &self.resolved.groups[&id];
        let mut signature = None;
        for observation in group {
            let data = &self.resolved.snapshot.declarations[observation.0].data;
            let ty = match data {
                DeclarationData::Function { ty, .. }
                | DeclarationData::Alias { target: ty, .. } => ty,
                DeclarationData::Record { methods, .. } if !methods.is_empty() => {
                    &methods[slot - 1].ty
                }
                DeclarationData::Record { .. } => continue,
                _ => unreachable!(),
            };
            let ty = self.function_type(ty)?;
            let TypeKind::Function {
                result, parameters, ..
            } = &ty.kind
            else {
                unreachable!()
            };
            let parameters = self.parameters(
                parameters,
                &self.resolved.annotations[&id].parameters[&slot],
            )?;
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
        let (parameters, result) = signature.unwrap();
        let names = &self.resolved.annotations[&id].parameter_names[&slot];
        let parameters = parameters
            .into_iter()
            .zip(names)
            .map(|((attributes, ty), name)| Ok((attributes, ident(name)?, ty)))
            .collect::<Result<_, Error>>()?;
        Ok((parameters, result))
    }

    fn function_type<'t>(&self, mut ty: &'t Type) -> Result<&'t Type, Error>
    where
        's: 't,
    {
        let mut visiting = BTreeSet::new();
        let mut pointer = false;
        loop {
            if let TypeKind::Pointer(target) = &ty.kind {
                if pointer {
                    return Err(Error("callable typedef has multiple pointer levels".into()));
                }
                pointer = true;
                ty = target;
                continue;
            }
            let TypeKind::Named(id) = &ty.kind else {
                break;
            };
            let id = self.resolved.representatives[id.0];
            if !visiting.insert(id) {
                return Err(Error("cyclic callable typedef".into()));
            }
            let DeclarationData::Alias { target, .. } =
                &self.resolved.snapshot.declarations[id.0].data
            else {
                return Err(Error("callable typedef does not name a function".into()));
            };
            ty = target;
        }
        if !matches!(ty.kind, TypeKind::Function { .. }) {
            return Err(Error("written callable type is not a function".into()));
        }
        Ok(ty)
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
                    ("Char_S" | "SChar" | "Char_U" | "UChar" | "Char8", Some(1), true) => {
                        Some(StringKind::Ansi)
                    }
                    ("Char_S" | "SChar" | "Char_U" | "UChar" | "Char8", Some(1), false) => {
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
        let name = &self.resolved.snapshot.declarations[id.0].name;
        let contract = self.options.pointer_sized.get(name);
        if !visiting.insert(id) {
            return Err(Error("cyclic native alias".into()));
        }
        let mut result = None;
        for observation in &self.resolved.groups[&id] {
            let DeclarationData::Alias { target, .. } =
                &self.resolved.snapshot.declarations[observation.0].data
            else {
                unreachable!()
            };
            let (mut ty, object) = self.lower(target, visiting)?;
            if let Some(contract) = contract {
                match &mut ty {
                    ProjectedType::Scalar(kind, _) => *kind = contract.name(),
                    ProjectedType::Alias(_, target)
                        if target.scalar_kind() == Some(contract.name()) => {}
                    _ => {
                        return Err(Error(format!(
                            "pointer-sized binding `{name}` conflicts with its written typedef contract"
                        )));
                    }
                }
            }
            let projected = (ty, object);
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
                    "Char_U" | "UChar" | "Char8" => "u8",
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
                    let DeclarationData::Alias { target, .. } = &declaration.data else {
                        unreachable!()
                    };
                    if matches!(
                        target.kind,
                        TypeKind::Function { .. } | TypeKind::Pointer(_)
                    ) && self.function_type(target).is_ok()
                    {
                        let name = self.name(id)?;
                        self.schedule(id);
                        return Ok((
                            if matches!(canonical.kind, TypeKind::Function { .. }) {
                                ProjectedType::Callable(name)
                            } else {
                                ProjectedType::Callback(name)
                            },
                            false,
                        ));
                    }
                    let (projected, object) = self.alias(id, aliases)?;
                    let annotations = &self.resolved.annotations[&id].own;
                    if !annotations.sal.is_empty() || !annotations.midl.is_empty() {
                        self.schedule(id);
                        let name = self.name(id)?;
                        (ProjectedType::Alias(name, Box::new(projected)), object)
                    } else {
                        (projected, object)
                    }
                } else if let Some(reference) = self.options.references.get(&declaration.name) {
                    let name = namespace_name(reference)?;
                    if reference.kind == ReferenceKind::Interface {
                        (ProjectedType::Class(name), true)
                    } else {
                        match &declaration.data {
                            DeclarationData::Record { layout, .. } => {
                                (ProjectedType::RecordReference(name, layout.clone()), false)
                            }
                            DeclarationData::Enum { repr, .. } => {
                                let (kind, layout) = self.enum_repr(repr)?;
                                (ProjectedType::ScalarReference(name, kind, layout), false)
                            }
                            _ => (ProjectedType::Named(name, None), false),
                        }
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
                    kind,
                    complete,
                    layout,
                    fields,
                    bases,
                    methods,
                    ..
                } = &declaration.data
                {
                    if *kind == RecordKind::Class
                        && !complete
                        && self.resolved.guids.contains_key(&id)
                    {
                        return Err(Error(format!(
                            "native class `{}` has identity but no captured object definition",
                            declaration.name
                        )));
                    }
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
                } else if let ProjectedType::Callable(name) = projected {
                    (ProjectedType::Callback(name), false)
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
            TypeKind::Array {
                length: Some(length),
                element,
            } => {
                let length = i64::try_from(*length)
                    .map_err(|_| Error("array length exceeds supported layout size".into()))?;
                let (element, object) = self.lower(element, aliases)?;
                if object {
                    return Err(Error("native interface objects require a pointer".into()));
                }
                (
                    ProjectedType::Array {
                        element: Box::new(element),
                        length,
                    },
                    false,
                )
            }
            _ => return Err(Error("this native type has no prototype projection".into())),
        })
    }
}

fn calling_convention(convention: i32) -> Result<&'static str, Error> {
    // Stable CXCallingConv C, X86StdCall, and Win64 values.
    match convention {
        1 => Ok("C"),
        2 | 10 => Ok("system"),
        _ => Err(Error(format!(
            "calling convention {convention} is not supported by this projection"
        ))),
    }
}

fn source_attributes(annotations: &SourceAnnotations) -> String {
    let mut result = String::new();
    for (source, annotations) in [("midl", &annotations.midl), ("sal", &annotations.sal)] {
        if !annotations.is_empty() {
            let text = annotations.join(" ");
            write!(result, "#[annotation({source:?}, {text:?})] ").unwrap();
        }
    }
    result
}

fn parameter_attributes(
    all_annotations: &[Vec<String>],
    index: usize,
    parameters: &[ProjectedType],
) -> Result<String, Error> {
    let ty = parameters[index].contract();
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
                _ => continue,
            };
            if sized {
                return Err(Error(
                    "multiple buffer-length annotations are not supported".into(),
                ));
            }
            let (mutable, void) = match ty {
                ProjectedType::Pointer {
                    mutable,
                    depth,
                    target,
                } => (
                    *mutable,
                    *depth == 1 && matches!(target.contract(), ProjectedType::Void),
                ),
                ProjectedType::PointerReference {
                    mutable,
                    string: Some(_),
                    ..
                } => (*mutable, false),
                _ => return Err(Error(format!("{name} requires a buffer pointer"))),
            };
            if output && !mutable {
                return Err(Error(format!("{name} requires a writable buffer")));
            }
            if !bytes && void {
                return Err(Error(format!("{name} requires a non-void element type")));
            }
            attributes.push_str(direction);
            if optional {
                attributes.push_str("#[opt] ");
            }
            if let Some(length) = buffer_length(capacity.trim(), bytes, parameters)? {
                attributes.push_str(&length);
            }
            if let Some(written) = written
                && let Some(written) = written_bytes(written.trim(), parameters, all_annotations)?
            {
                attributes.push_str(&written);
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
                        | ProjectedType::Callback(_)
                ) =>
            {
                "#[in] #[opt] "
            }
            "_In_opt_" => return Err(Error("_In_opt_ requires a pointer".into())),
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
            _ => continue,
        });
    }
    Ok(attributes)
}

fn written_bytes(
    argument: &str,
    parameters: &[ProjectedType],
    annotations: &[Vec<String>],
) -> Result<Option<String>, Error> {
    let (argument, dereference) = argument
        .strip_prefix('*')
        .map_or((argument, false), |value| (value.trim(), true));
    let Some(index) = argument
        .strip_prefix('$')
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
    else {
        return Ok(None);
    };
    let ty = parameters
        .get(index as usize)
        .ok_or_else(|| Error("written byte count parameter is out of range".into()))?
        .contract();
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
        Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize")
    ) {
        return Err(Error("written byte count must be an integer".into()));
    }
    Ok(Some(format!(
        "#[written_bytes(BytesParamIndex = {index}, Dereference = {dereference})] "
    )))
}

fn string_kind(annotations: &[String], ty: &ProjectedType) -> Result<Option<StringKind>, Error> {
    let ty = ty.contract();
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
) -> Result<Option<String>, Error> {
    if let Some(index) = argument
        .strip_prefix('$')
        .filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|value| value.parse::<i16>().ok())
    {
        let Some("i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize") =
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
        return Ok(Some(format!("#[{attribute}({index})] ")));
    }
    if !bytes
        && argument.bytes().all(|byte| byte.is_ascii_digit())
        && (argument == "0" || !argument.starts_with('0'))
        && let Ok(value) = argument.parse::<i32>()
    {
        return Ok(Some(format!("#[len_const({value})] ")));
    }
    Ok(None)
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
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "gen"
                | "macro"
                | "override"
                | "priv"
                | "try"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
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
