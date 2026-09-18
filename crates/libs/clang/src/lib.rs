#![allow(non_upper_case_globals)]
#![doc = include_str!("../readme.md")]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{Display, Formatter};

mod extract;
pub use extract::extract;

mod builder;
pub use builder::{Clang, clang};

mod references;
pub use references::MetadataReferences;

#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub source: String,
    pub roots: BTreeSet<String>,
    pub root_dirs: BTreeSet<String>,
    pub root_suffixes: BTreeSet<String>,
    pub excluded_roots: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeReference {
    pub namespace: String,
    pub name: String,
    pub kind: TypeReferenceKind,
    pub enum_members: BTreeSet<String>,
}

impl TypeReference {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        kind: TypeReferenceKind,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            kind,
            enum_members: BTreeSet::new(),
        }
    }

    pub fn with_enum_members(
        mut self,
        members: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.enum_members
            .extend(members.into_iter().map(Into::into));
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeReferenceKind {
    Enum,
    Interface,
    Type,
}

pub struct EmitOptions<'a> {
    pub namespace: &'a str,
    pub library: Option<&'a str>,
    pub libraries: Option<&'a BTreeMap<String, String>>,
    pub references: &'a BTreeMap<String, TypeReference>,
    pub excluded: Option<&'a BTreeSet<String>>,
    pub excluded_types: Option<&'a BTreeSet<String>>,
    pub excluded_functions: Option<&'a BTreeSet<String>>,
    pub excluded_constants: Option<&'a BTreeSet<String>>,
    pub functions: Option<&'a BTreeSet<String>>,
}

impl<'a> EmitOptions<'a> {
    pub fn new(namespace: &'a str, references: &'a BTreeMap<String, TypeReference>) -> Self {
        Self {
            namespace,
            library: None,
            libraries: None,
            references,
            excluded: None,
            excluded_types: None,
            excluded_functions: None,
            excluded_constants: None,
            functions: None,
        }
    }
}

impl Input {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        let name = normalize_name(&name.into());
        Self {
            roots: BTreeSet::from([name.clone()]),
            root_dirs: BTreeSet::new(),
            root_suffixes: BTreeSet::new(),
            excluded_roots: BTreeSet::new(),
            name,
            source: source.into(),
        }
    }

    pub fn with_roots(mut self, roots: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.roots
            .extend(roots.into_iter().map(|root| normalize_name(&root.into())));
        self
    }

    pub fn with_root_suffixes(
        mut self,
        roots: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.root_suffixes
            .extend(roots.into_iter().map(|root| normalize_name(&root.into())));
        self
    }

    pub fn with_root_dirs(mut self, roots: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.root_dirs.extend(roots.into_iter().map(|root| {
            let mut root = normalize_name(&root.into());
            if !root.ends_with('/') {
                root.push('/');
            }
            root
        }));
        self
    }

    pub fn with_excluded_roots(
        mut self,
        roots: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.excluded_roots
            .extend(roots.into_iter().map(|root| normalize_name(&root.into())));
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Origin {
    pub tu: String,
    pub local: u32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location {
    pub file: String,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Scalar {
    Bool,
    F32,
    F64,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeRef {
    Void,
    String,
    Object,
    Scalar(Scalar),
    Named {
        name: String,
        declaration: Location,
    },
    Pointer {
        mutable: bool,
        target: Box<Self>,
    },
    Reference {
        mutable: bool,
        target: Box<Self>,
    },
    FunctionPointer {
        convention: CallingConvention,
        params: Vec<Self>,
        result: Box<Self>,
    },
    OpaquePointer {
        mutable: bool,
        tag: String,
    },
    Array {
        target: Box<Self>,
        len: usize,
    },
    Generic {
        name: String,
        declaration: Location,
        args: Vec<Self>,
    },
    InlineRecord(Box<InlineRecord>),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InlineRecord {
    pub name: Option<String>,
    pub base: Option<TypeRef>,
    pub fields: Vec<Field>,
    pub size: i64,
    pub align: i64,
    pub packing: Option<i64>,
    pub alignment: Option<i64>,
    pub union: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Field {
    pub name: String,
    pub ty: TypeRef,
    pub offset: i64,
    pub align: i64,
    pub size: i64,
    pub bit_width: Option<u32>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Variant {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Parameter {
    pub name: String,
    pub ty: TypeRef,
    pub annotation: ParamAnnotation,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Method {
    pub name: String,
    pub params: Vec<Parameter>,
    pub result: TypeRef,
    pub special: bool,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ParamAnnotation {
    pub input: bool,
    pub output: bool,
    pub optional: bool,
    pub reserved: bool,
    pub com_out_ptr: bool,
    pub retval: bool,
    pub null_terminated: bool,
    pub size: Option<SalSize>,
    pub unsupported: Option<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SalSize {
    pub bytes: bool,
    pub value: SalSizeValue,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SalSizeValue {
    Constant(i32),
    Parameter(String),
    IndirectParameter(String),
    Expression(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CallingConvention {
    Platform,
    C,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FactData {
    Callback {
        convention: CallingConvention,
        params: Vec<Parameter>,
        result: TypeRef,
    },
    Class {
        guid: String,
    },
    Enum {
        repr: Scalar,
        variants: Vec<Variant>,
        fixed: bool,
        scoped: bool,
    },
    EnumFlag {
        target: String,
    },
    Guid {
        value: String,
    },
    PropertyKey {
        ty: &'static str,
        guid: String,
        pid: u32,
    },
    Macro {
        function_like: bool,
        tokens: Vec<String>,
    },
    Function {
        link_name: String,
        convention: CallingConvention,
        params: Vec<Parameter>,
        result: TypeRef,
        variadic: bool,
        noreturn: bool,
    },
    Interface {
        base: Option<TypeRef>,
        guid: Option<String>,
        methods: Vec<Method>,
    },
    Record {
        base: Option<TypeRef>,
        fields: Vec<Field>,
        size: i64,
        align: i64,
        packing: Option<i64>,
        alignment: Option<i64>,
        union: bool,
    },
    Typedef {
        target: TypeRef,
    },
    Unsupported {
        reason: String,
    },
    None,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FactKind {
    Class,
    Enum,
    EnumFlag,
    Function,
    Guid,
    Macro,
    Namespace,
    Struct,
    Typedef,
    Union,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fact {
    pub origin: Origin,
    pub parent: Option<Origin>,
    pub kind: FactKind,
    pub name: String,
    pub spelling: Location,
    pub expansion: Location,
    pub definition: bool,
    pub main_file: bool,
    pub root: bool,
    pub system: bool,
    pub data: FactData,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    F32(u32),
    F64(u64),
    Signed(i64),
    Unsigned(u64),
    Utf8(String),
    Utf16(String),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Constant {
    pub root: Origin,
    pub definition: Origin,
    pub spelling: Location,
    pub name: String,
    pub ty: TypeRef,
    pub value: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    facts: Vec<Fact>,
    constants: Vec<Constant>,
}

impl Snapshot {
    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn unsupported(&self) -> impl Iterator<Item = (&Fact, &str)> {
        self.facts.iter().filter_map(|fact| {
            if let FactData::Unsupported { reason } = &fact.data {
                Some((fact, reason.as_str()))
            } else {
                None
            }
        })
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();
        for fact in &self.facts {
            let parent = fact
                .parent
                .as_ref()
                .map_or(String::new(), |parent| format!(" <- {}", origin(parent)));
            result.push_str(&format!(
                "{} {:?}/{} {} [{}:{} -> {}:{}]{}{}{}{}\n",
                origin(&fact.origin),
                fact.kind,
                fact_data_kind(&fact.data),
                fact.name,
                fact.spelling.file,
                fact.spelling.offset,
                fact.expansion.file,
                fact.expansion.offset,
                if fact.definition { " definition" } else { "" },
                if fact.main_file { " main" } else { "" },
                if fact.system { " system" } else { "" },
                parent,
            ));
        }
        for constant in &self.constants {
            let route = format!(
                "{} -> {}",
                origin(&constant.root),
                origin(&constant.definition)
            );
            result.push_str(&format!(
                "{} Constant {} {:?} = {:?}\n",
                route, constant.name, constant.ty, constant.value
            ));
        }
        result
    }

    pub fn emit(&self, namespace: &str) -> Result<String, Error> {
        let references = BTreeMap::new();
        self.emit_with_options(&EmitOptions::new(namespace, &references))
    }

    pub fn emit_with_library(&self, namespace: &str, library: &str) -> Result<String, Error> {
        let references = BTreeMap::new();
        let mut options = EmitOptions::new(namespace, &references);
        options.library = Some(library);
        self.emit_with_options(&options)
    }

    pub fn emit_with_options(&self, options: &EmitOptions<'_>) -> Result<String, Error> {
        let items = self.emit_items(options)?;
        write_rdl(
            options.namespace,
            items.values().map(|(_, item)| item.as_str()),
        )
    }

    pub fn emit_by_header_with_options(
        &self,
        options: &EmitOptions<'_>,
    ) -> Result<BTreeMap<String, String>, Error> {
        let items = self.emit_items(options)?;
        let mut partitions: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (_, (header, item)) in items {
            partitions.entry(header).or_default().push(item);
        }
        partitions
            .into_iter()
            .map(|(header, items)| {
                Ok((
                    header,
                    write_rdl(options.namespace, items.iter().map(String::as_str))?,
                ))
            })
            .collect()
    }

    fn emit_items(
        &self,
        options: &EmitOptions<'_>,
    ) -> Result<BTreeMap<(String, OutputKind), (String, String)>, Error> {
        let timing = std::env::var_os("WINDOWS_CLANG_TIMING").is_some();
        let plan_time = std::time::Instant::now();
        let plan = self.plan(
            options.references,
            options.excluded_types.or(options.excluded),
            options.excluded_functions.or(options.excluded),
            options.excluded_constants.or(options.excluded),
            options.functions,
        )?;
        if timing {
            eprintln!("clang planning: {:.2}s", plan_time.elapsed().as_secs_f32());
        }
        let emission_time = std::time::Instant::now();
        let mut items = BTreeMap::new();
        let planned = plan
            .values
            .into_iter()
            .map(|planned| (planned, OutputKind::Value))
            .chain(
                plan.types
                    .into_iter()
                    .map(|planned| (planned, OutputKind::Type)),
            );
        for (planned, kind) in planned {
            let fact = planned.fact;
            let item = match &fact.data {
                FactData::Callback {
                    convention,
                    params,
                    result,
                } => {
                    let projection = TypeProjection::new(
                        &plan.type_names,
                        &plan.interface_names,
                        &fact.origin.tu,
                    );
                    write_callback(&planned.name, *convention, params, result, &projection)?
                }
                FactData::Class { guid } => {
                    format!(
                        "    const {}: GUID = {};\n",
                        rdl_ident(&planned.name),
                        rdl_uuid(guid)
                    )
                }
                FactData::Guid { value } => {
                    format!(
                        "    const {}: GUID = {};\n",
                        rdl_ident(&planned.name),
                        rdl_uuid(value)
                    )
                }
                FactData::PropertyKey { ty, guid, pid } => {
                    format!(
                        "    #[guid({})]\n    const {}: {} = {pid};\n",
                        rdl_uuid(guid),
                        rdl_ident(&planned.name),
                        rdl_ident(ty)
                    )
                }
                FactData::Typedef {
                    target: TypeRef::InlineRecord(record),
                } => {
                    let projection = TypeProjection::new(
                        &plan.type_names,
                        &plan.interface_names,
                        &fact.origin.tu,
                    );
                    write_named_record(
                        &rdl_ident(&planned.name),
                        &record.fields,
                        record.packing,
                        record.alignment,
                        record.union,
                        &projection,
                    )?
                }
                FactData::Typedef { target } => {
                    format!(
                        "    type {} = {};\n",
                        rdl_ident(&planned.name),
                        planned_emitted_type_name(
                            target,
                            &plan.type_names,
                            &plan.interface_names,
                            &fact.origin.tu,
                        )
                    )
                }
                FactData::Enum {
                    repr,
                    variants,
                    scoped,
                    ..
                } => {
                    let flags = plan
                        .flag_enums
                        .contains(&(fact.origin.tu.clone(), planned.name.clone()));
                    let repr = if flags { unsigned_scalar(*repr) } else { *repr };
                    let mut item = format!(
                        "    #[repr({})]\n{}{}    enum {} {{\n",
                        scalar_name(repr),
                        if flags { "    #[flags]\n" } else { "" },
                        if *scoped { "    #[scoped]\n" } else { "" },
                        rdl_ident(&planned.name)
                    );
                    for variant in variants {
                        item.push_str(&format!(
                            "        {} = {},\n",
                            rdl_ident(&variant.name),
                            enum_value(variant.value, repr)
                        ));
                    }
                    item.push_str("    }\n");
                    item
                }
                FactData::Record {
                    fields,
                    packing,
                    alignment,
                    union,
                    ..
                } => {
                    let projection = TypeProjection::new(
                        &plan.type_names,
                        &plan.interface_names,
                        &fact.origin.tu,
                    );
                    write_named_record(
                        &rdl_ident(&planned.name),
                        fields,
                        *packing,
                        *alignment,
                        *union,
                        &projection,
                    )?
                }
                FactData::Interface {
                    base,
                    guid,
                    methods,
                } => {
                    let projection = TypeProjection::new(
                        &plan.type_names,
                        &plan.interface_names,
                        &fact.origin.tu,
                    );
                    write_interface(
                        &rdl_ident(&planned.name),
                        base.as_ref(),
                        guid.as_deref().or_else(|| {
                            plan.interface_guids.get(&planned.name).map(String::as_str)
                        }),
                        methods,
                        &projection,
                    )?
                }
                _ => {
                    return Err(Error(format!(
                        "planned type `{}` is not emittable",
                        fact.name
                    )));
                }
            };
            if items
                .insert(
                    (planned.name.clone(), kind),
                    (fact.spelling.file.clone(), item),
                )
                .is_some()
            {
                return Err(Error(format!("duplicate planned name `{}`", planned.name)));
            }
        }
        for function in plan.functions {
            let FactData::Function {
                link_name,
                convention,
                params,
                result,
                variadic,
                noreturn,
            } = &function.data
            else {
                return Err(Error(format!(
                    "planned function `{}` has no signature",
                    function.name
                )));
            };
            let projection =
                TypeProjection::new(&plan.type_names, &plan.interface_names, &function.origin.tu);
            let mut params = write_params(params, &projection)?;
            if *variadic {
                params.push("...".to_string());
            }
            let params = params.join(", ");
            let result = if *result == TypeRef::Void {
                String::new()
            } else {
                format!(
                    " -> {}",
                    planned_emitted_type_name(
                        result,
                        &plan.type_names,
                        &plan.interface_names,
                        &function.origin.tu,
                    )
                )
            };
            let library = options
                .libraries
                .and_then(|libraries| libraries.get(link_name).map(String::as_str))
                .or(options.library)
                .ok_or_else(|| {
                    Error(format!(
                        "function `{}` requires an import library",
                        function.name
                    ))
                })?;
            let abi = calling_convention(*convention);
            let library = if function.name == *link_name {
                format!("#[library({library:?})]")
            } else {
                format!("#[library({library:?}, import = {link_name:?})]")
            };
            let item = format!(
                "{}    {library}\n    extern{abi} fn {}({params}){result};\n",
                if *noreturn { "    #[noreturn]\n" } else { "" },
                rdl_ident(&function.name),
            );
            if items
                .insert(
                    (function.name.clone(), OutputKind::Value),
                    (function.spelling.file.clone(), item),
                )
                .is_some()
            {
                return Err(Error(format!("duplicate planned name `{}`", function.name)));
            }
        }
        for constant in plan.constants {
            let encoding = match &constant.value {
                Value::Utf8(_) => "    #[encoding(\"ansi\")]\n",
                Value::Utf16(_) => "    #[encoding(\"utf-16\")]\n",
                _ => "",
            };
            let ty = match &constant.value {
                Value::Utf8(_) | Value::Utf16(_) => "String".to_string(),
                _ => constant_type_name(&constant.ty, &plan.type_names),
            };
            let item = format!(
                "{encoding}    const {}: {ty} = {};\n",
                rdl_ident(&constant.name),
                value_name(&constant.value)
            );
            if items
                .insert(
                    (constant.name.clone(), OutputKind::Value),
                    (constant.spelling.file.clone(), item),
                )
                .is_some()
            {
                return Err(Error(format!("duplicate planned name `{}`", constant.name)));
            }
        }
        if timing {
            eprintln!(
                "clang emission: {:.2}s",
                emission_time.elapsed().as_secs_f32()
            );
        }
        Ok(items)
    }

    fn plan(
        &self,
        references: &BTreeMap<String, TypeReference>,
        excluded_types: Option<&BTreeSet<String>>,
        excluded_functions: Option<&BTreeSet<String>>,
        excluded_constants: Option<&BTreeSet<String>>,
        selected_functions: Option<&BTreeSet<String>>,
    ) -> Result<Plan<'_>, Error> {
        let timing = std::env::var_os("WINDOWS_CLANG_TIMING").is_some();
        let mut phase_time = std::time::Instant::now();
        #[derive(Default)]
        struct Roots<'a> {
            types: Vec<&'a Fact>,
            values: Vec<&'a Fact>,
            functions: Vec<&'a Fact>,
            constants: Vec<&'a Constant>,
        }

        let facts_by_origin: HashMap<_, _> =
            self.facts.iter().map(|fact| (&fact.origin, fact)).collect();
        let is_flat_root = |fact| is_flat_declaration(fact, &facts_by_origin, references, true);
        let is_flat_dependency =
            |fact| is_flat_declaration(fact, &facts_by_origin, references, false);
        let interfaces: BTreeSet<_> = self
            .facts
            .iter()
            .filter(|fact| {
                is_flat_dependency(fact) && matches!(fact.data, FactData::Interface { .. })
            })
            .map(|fact| fact.name.as_str())
            .collect();
        let mut declared_interface_guids = BTreeMap::new();
        for fact in self.facts.iter().filter(|fact| {
            fact.root && matches!(fact.data, FactData::Interface { .. }) && is_flat_root(fact)
        }) {
            let FactData::Interface {
                guid: Some(guid), ..
            } = &fact.data
            else {
                continue;
            };
            if let Some(previous) =
                declared_interface_guids.insert(fact.name.as_str(), guid.as_str())
                && previous != guid
            {
                return Err(Error(format!(
                    "interface `{}` has conflicting UUID attributes",
                    fact.name
                )));
            }
        }
        let mut interface_guids = BTreeMap::new();
        for fact in self.facts.iter().filter(|fact| {
            fact.root && matches!(fact.data, FactData::Guid { .. }) && is_flat_root(fact)
        }) {
            let Some(interface) = fact.name.strip_prefix("IID_") else {
                continue;
            };
            if !interfaces.contains(interface) || declared_interface_guids.contains_key(interface) {
                continue;
            }
            let FactData::Guid { value } = &fact.data else {
                unreachable!()
            };
            if let Some(previous) = interface_guids.insert(interface.to_string(), value.clone())
                && previous != *value
            {
                return Err(Error(format!(
                    "interface `{interface}` has conflicting IID declarations"
                )));
            }
        }

        let mut roots: BTreeMap<&str, Roots<'_>> = BTreeMap::new();
        for fact in self
            .facts
            .iter()
            .filter(|fact| fact.root && is_root_fact(fact) && is_flat_root(fact))
            .filter(|fact| {
                let Some(interface) = fact.name.strip_prefix("IID_") else {
                    return true;
                };
                let FactData::Guid { value } = &fact.data else {
                    return true;
                };
                if references
                    .get(interface)
                    .is_some_and(|reference| reference.kind == TypeReferenceKind::Interface)
                {
                    return false;
                }
                if !interfaces.contains(interface) {
                    return true;
                }
                declared_interface_guids
                    .get(interface)
                    .is_some_and(|declared| *declared != value)
            })
        {
            let roots = roots.entry(&fact.name).or_default();
            if is_value_fact(fact) {
                roots.values.push(fact);
            } else {
                roots.types.push(fact);
            }
        }
        for fact in self
            .facts
            .iter()
            .filter(|fact| {
                fact.root && matches!(fact.data, FactData::Function { .. }) && is_flat_root(fact)
            })
            .filter(|fact| excluded_functions.is_none_or(|excluded| !excluded.contains(&fact.name)))
            .filter(|fact| {
                selected_functions.is_none_or(|functions| {
                    matches!(
                        &fact.data,
                        FactData::Function { link_name, .. } if functions.contains(link_name)
                    )
                })
            })
        {
            roots.entry(&fact.name).or_default().functions.push(fact);
        }
        for constant in &self.constants {
            if excluded_constants.is_some_and(|excluded| excluded.contains(&constant.name)) {
                continue;
            }
            roots
                .entry(&constant.name)
                .or_default()
                .constants
                .push(constant);
        }

        let mut facts_index: HashMap<&str, Vec<&Fact>> = HashMap::new();
        for fact in self.facts.iter().filter(|fact| is_flat_dependency(fact)) {
            facts_index.entry(&fact.name).or_default().push(fact);
        }
        let facts_by_declaration: BTreeMap<_, _> = self
            .facts
            .iter()
            .map(|fact| ((fact.origin.tu.clone(), fact.spelling.clone()), fact))
            .collect();
        let extended_reference_enums: BTreeSet<_> = excluded_types
            .into_iter()
            .flatten()
            .filter_map(|name| {
                let reference = references.get(name)?;
                if reference.kind != TypeReferenceKind::Enum {
                    return None;
                }
                if reference.enum_members.is_empty()
                    || self
                        .facts
                        .iter()
                        .filter(|fact| fact.name == *name)
                        .filter_map(|fact| {
                            underlying_enum_fact(fact, &facts_by_declaration, &mut BTreeSet::new())
                        })
                        .filter_map(|fact| match &fact.data {
                            FactData::Enum { variants, .. } => Some(variants.as_slice()),
                            _ => None,
                        })
                        .flatten()
                        .any(|variant| !reference.enum_members.contains(&variant.name))
                {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();
        let excluded_declarations: BTreeSet<_> = excluded_types
            .into_iter()
            .flat_map(|excluded| {
                let extended_reference_enums = &extended_reference_enums;
                self.facts.iter().filter_map(move |fact| {
                    if !excluded.contains(&fact.name)
                        || extended_reference_enums.contains(&fact.name)
                    {
                        return None;
                    }
                    if let FactData::Typedef {
                        target: TypeRef::Named { declaration, .. },
                    } = &fact.data
                    {
                        Some((fact.origin.tu.clone(), declaration.clone()))
                    } else {
                        None
                    }
                })
            })
            .collect();
        let excluded_local_names: BTreeSet<_> = excluded_types
            .into_iter()
            .flat_map(|excluded| {
                let extended_reference_enums = &extended_reference_enums;
                self.facts.iter().filter_map(move |fact| {
                    if !excluded.contains(&fact.name)
                        || extended_reference_enums.contains(&fact.name)
                    {
                        return None;
                    }
                    if let FactData::Typedef {
                        target: TypeRef::Named { name, .. },
                    } = &fact.data
                    {
                        Some((fact.origin.tu.clone(), name.clone()))
                    } else {
                        None
                    }
                })
            })
            .collect();
        let mut type_roots = vec![];
        let mut value_roots = vec![];
        let mut functions = vec![];
        let mut constants = vec![];
        let mut root_names = BTreeSet::new();
        let mut shape_cache = ShapeCache::default();

        for (name, roots) in roots {
            if !roots.types.is_empty() && !roots.functions.is_empty() {
                return Err(Error(format!(
                    "type and function roots collide on `{name}`"
                )));
            }
            let value = if roots.values.is_empty()
                || excluded_constants.is_some_and(|excluded| excluded.contains(name))
            {
                None
            } else {
                Some(choose_value_root(name, &roots.values)?)
            };
            let constant = if roots.constants.is_empty() {
                None
            } else {
                Some(choose_constant_root(name, &roots.constants)?)
            };
            let types_alias_value_class =
                types_alias_value_class(name, &roots.types, &roots.values);
            if !roots.types.is_empty() && !types_alias_value_class {
                let excluded = roots.types.iter().any(|fact| {
                    excluded_declarations.contains(&(fact.origin.tu.clone(), fact.spelling.clone()))
                        || excluded_local_names
                            .contains(&(fact.origin.tu.clone(), fact.name.clone()))
                }) || (excluded_types
                    .is_some_and(|excluded| excluded.contains(name))
                    && !extended_reference_enums.contains(name))
                    || (references.contains_key(name)
                        && !roots
                            .types
                            .iter()
                            .any(|fact| defines_local_type(name, fact)));
                if !excluded {
                    let root = choose_type_root_cached(
                        name,
                        &roots.types,
                        &facts_index,
                        &mut shape_cache,
                    )?;
                    root_names.insert(name.to_string());
                    type_roots.push(root);
                }
            } else if !roots.functions.is_empty() {
                functions.push(choose_function_root(name, &roots.functions)?);
            }
            if let Some(value) = value {
                value_roots.push(value);
            }
            if let Some(constant) = constant {
                constants.push(constant);
            }
        }
        if let Some(selected) = selected_functions {
            let found: BTreeSet<_> = functions
                .iter()
                .filter_map(|function| match &function.data {
                    FactData::Function { link_name, .. } => Some(link_name.as_str()),
                    _ => None,
                })
                .collect();
            if let Some(missing) = selected.iter().find(|name| !found.contains(name.as_str())) {
                return Err(Error(format!(
                    "selected function `{missing}` was not found"
                )));
            }
        }
        if timing {
            eprintln!(
                "clang plan roots: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }

        let facts_by_name = loop {
            let mut facts = BTreeSet::new();
            let mut queue = vec![];
            for root in &type_roots {
                if facts.insert(root.origin.clone()) {
                    queue_type_edges(root, &mut queue);
                }
            }
            for root in &value_roots {
                queue_type_edges(root, &mut queue);
            }
            for constant in &constants {
                queue.push((constant.root.tu.as_str(), TypeEdge::Type(&constant.ty)));
            }
            for function in &functions {
                queue_function_edges(function, &mut queue);
            }

            while let Some((tu, edge)) = queue.pop() {
                let ty = match edge {
                    TypeEdge::Type(ty) => ty,
                    TypeEdge::Projected(name) => {
                        if references.contains_key(name) && !root_names.contains(name) {
                            continue;
                        }
                        let matches: Vec<_> = facts_index
                            .get(name)
                            .into_iter()
                            .flatten()
                            .copied()
                            .filter(|fact| fact.origin.tu == tu && is_type_fact(fact))
                            .collect();
                        let fact = choose_type_root_cached(
                            name,
                            &matches,
                            &facts_index,
                            &mut shape_cache,
                        )?;
                        if facts.insert(fact.origin.clone()) {
                            queue_type_edges(fact, &mut queue);
                        }
                        continue;
                    }
                };
                let (name, declaration) = match ty {
                    TypeRef::Pointer { target, .. } | TypeRef::Reference { target, .. } => {
                        queue.push((tu, TypeEdge::Type(target)));
                        continue;
                    }
                    TypeRef::FunctionPointer { .. } | TypeRef::OpaquePointer { .. } => continue,
                    TypeRef::Array { target, .. } => {
                        queue.push((tu, TypeEdge::Type(target)));
                        continue;
                    }
                    TypeRef::InlineRecord(record) => {
                        for field in &record.fields {
                            queue.push((tu, TypeEdge::Type(&field.ty)));
                        }
                        continue;
                    }
                    TypeRef::Named { name, declaration } => (name, declaration),
                    _ => continue,
                };
                if excluded_local_names.contains(&(tu.to_string(), name.clone())) {
                    continue;
                }
                if references.contains_key(name) && !root_names.contains(name) {
                    continue;
                }
                let matches: Vec<_> = facts_index
                    .get(name.as_str())
                    .into_iter()
                    .flatten()
                    .copied()
                    .filter(|fact| fact.origin.tu == tu && fact.spelling == *declaration)
                    .collect();
                let [fact] = matches.as_slice() else {
                    return Err(Error(format!(
                        "unresolved local type `{name}` in translation unit `{tu}`"
                    )));
                };
                if let FactData::Unsupported { reason } = &fact.data {
                    return Err(Error(format!(
                        "unsupported type `{name}` in translation unit `{tu}`: {reason}"
                    )));
                }
                if !is_type_fact(fact) {
                    return Err(Error(format!(
                        "unresolved local type `{name}` in translation unit `{tu}`"
                    )));
                };
                if canonical_named_type(name).is_some()
                    && matches!(fact.data, FactData::Typedef { .. })
                {
                    continue;
                }
                if facts.insert(fact.origin.clone()) {
                    queue_type_edges(fact, &mut queue);
                }
            }

            let mut grouped: BTreeMap<&str, Vec<&Fact>> = BTreeMap::new();
            for fact in facts.into_iter().map(|origin| facts_by_origin[&origin]) {
                grouped.entry(&fact.name).or_default().push(fact);
            }

            let collisions: Vec<_> = constants
                .iter()
                .filter_map(|constant| {
                    grouped
                        .get(constant.name.as_str())
                        .map(|choices| (constant, choices))
                })
                .collect();
            if !collisions.is_empty() {
                let mut added = false;
                for (constant, choices) in collisions {
                    let root = choose_type_root_cached(
                        &constant.name,
                        choices,
                        &facts_index,
                        &mut shape_cache,
                    )?;
                    if root_names.insert(constant.name.clone()) {
                        type_roots.push(root);
                        added = true;
                    }
                }
                if added {
                    continue;
                }
            }

            let mut facts_by_name = BTreeMap::new();
            for (name, choices) in grouped {
                let selected =
                    choose_type_root_cached(name, &choices, &facts_index, &mut shape_cache)?;
                facts_by_name.insert(name, selected);
            }
            break facts_by_name;
        };
        let mut validated_layouts: HashSet<_> = facts_by_name
            .values()
            .filter(|fact| {
                !matches!(fact.data, FactData::Typedef { .. })
                    && !matches!(fact.data, FactData::Record { .. } if !fact.definition)
            })
            .map(|fact| fact.origin.clone())
            .collect();
        let mut safe_layouts: HashMap<String, HashSet<Location>> = HashMap::new();
        for fact in facts_index.values().flatten().copied().filter(|fact| {
            !matches!(fact.data, FactData::Typedef { .. })
                && !matches!(fact.data, FactData::Record { .. } if !fact.definition)
        }) {
            safe_layouts
                .entry(fact.origin.tu.clone())
                .or_default()
                .insert(fact.spelling.clone());
        }
        loop {
            let additions: Vec<_> = facts_index
                .values()
                .flatten()
                .copied()
                .filter(|fact| {
                    !safe_layouts
                        .get(&fact.origin.tu)
                        .is_some_and(|safe| safe.contains(&fact.spelling))
                })
                .filter_map(|fact| match &fact.data {
                    FactData::Typedef { target }
                        if known_complete_layout(
                            target,
                            &fact.origin.tu,
                            &safe_layouts,
                            references,
                        ) =>
                    {
                        Some((fact.origin.tu.clone(), fact.spelling.clone()))
                    }
                    _ => None,
                })
                .collect();
            if additions.is_empty() {
                break;
            }
            for (tu, declaration) in additions {
                safe_layouts.entry(tu).or_default().insert(declaration);
            }
        }
        let layout = LayoutContext {
            facts_index: &facts_index,
            planned_types: &facts_by_name,
        };
        let validation_time = std::time::Instant::now();
        for fact in facts_by_name.values() {
            validate_fact_layouts(fact, &layout, &mut safe_layouts, &mut validated_layouts)?;
        }
        for fact in &value_roots {
            validate_fact_layouts(fact, &layout, &mut safe_layouts, &mut validated_layouts)?;
        }
        if timing {
            eprintln!(
                "clang validate types and values: {:.2}s",
                validation_time.elapsed().as_secs_f32()
            );
        }
        let validation_time = std::time::Instant::now();
        for function in &functions {
            validate_fact_layouts(function, &layout, &mut safe_layouts, &mut validated_layouts)?;
        }
        if timing {
            eprintln!(
                "clang validate functions: {:.2}s",
                validation_time.elapsed().as_secs_f32()
            );
        }
        let validation_time = std::time::Instant::now();
        for constant in &constants {
            validate_complete_layout(
                &constant.ty,
                &constant.root.tu,
                &layout,
                &mut safe_layouts,
                &mut BTreeSet::new(),
                &mut validated_layouts,
            )?;
        }
        if timing {
            eprintln!(
                "clang validate constants: {:.2}s",
                validation_time.elapsed().as_secs_f32()
            );
        }
        if timing {
            eprintln!(
                "clang plan closure: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }

        let local_roots = root_names.clone();
        let required: BTreeSet<String> = facts_by_name
            .keys()
            .map(|name| (*name).to_string())
            .collect();
        if timing {
            eprintln!(
                "clang plan required: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }

        let mut enum_alias_candidates: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for fact in &self.facts {
            if let FactData::Typedef {
                target: TypeRef::Named { name, declaration },
            } = &fact.data
                && name != &fact.name
                && !references.contains_key(name)
                && let Some(selected) = facts_by_name.get(name.as_str())
                && facts_index
                    .get(name.as_str())
                    .into_iter()
                    .flatten()
                    .any(|target| {
                        target.origin.tu == fact.origin.tu
                            && target.spelling == *declaration
                            && target.spelling.file == fact.spelling.file
                            && target.definition
                            && (matches!(
                                target.data,
                                FactData::Enum { .. } | FactData::Record { .. }
                            ) || (matches!(target.data, FactData::Interface { .. })
                                && selected.name.starts_with('_')))
                            && same_source_declaration(selected, target)
                    })
            {
                enum_alias_candidates
                    .entry(selected.name.as_str())
                    .or_default()
                    .push(fact.name.as_str());
            }
        }
        let mut type_names: BTreeMap<_, _> = enum_alias_candidates
            .into_iter()
            .filter_map(|(target, aliases)| {
                let aliases: BTreeSet<_> = aliases.into_iter().collect();
                if aliases.len() != 1 {
                    return None;
                }
                Some((target.to_string(), (*aliases.first().unwrap()).to_string()))
            })
            .collect();
        for name in &extended_reference_enums {
            let enum_names: BTreeSet<_> = self
                .facts
                .iter()
                .filter(|fact| fact.name == *name)
                .filter_map(|fact| {
                    underlying_enum_fact(fact, &facts_by_declaration, &mut BTreeSet::new())
                })
                .map(|fact| fact.name.as_str())
                .collect();
            if let [enum_name] = enum_names.into_iter().collect::<Vec<_>>().as_slice() {
                type_names.insert((*enum_name).to_string(), name.clone());
            }
        }
        for fact in &self.facts {
            if let FactData::Typedef {
                target: TypeRef::Pointer { target, .. },
            } = &fact.data
                && let TypeRef::Named { name, .. } = target.as_ref()
                && let Some(public_name) = name.strip_prefix('_')
                && fact.name == format!("P{public_name}")
                && !references.contains_key(name)
                && facts_index
                    .get(name.as_str())
                    .into_iter()
                    .flatten()
                    .any(|target| target.origin.tu == fact.origin.tu)
            {
                let public_aliases_private = facts_index
                    .get(public_name)
                    .into_iter()
                    .flatten()
                    .any(|candidate| {
                        matches!(
                            &candidate.data,
                            FactData::Typedef {
                                target: TypeRef::Named {
                                    name: target_name,
                                    ..
                                },
                            } if target_name == name
                        )
                    });
                if !public_aliases_private
                    && facts_index
                        .get(public_name)
                        .into_iter()
                        .flatten()
                        .any(|candidate| {
                            matches!(
                                candidate.data,
                                FactData::Class { .. }
                                    | FactData::Callback { .. }
                                    | FactData::Enum { .. }
                                    | FactData::Interface { .. }
                                    | FactData::Record { .. }
                            ) || defines_local_type(public_name, candidate)
                        })
                {
                    continue;
                }
                type_names
                    .entry(name.clone())
                    .or_insert_with(|| public_name.to_string());
            }
        }
        let mut external_alias_candidates: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
        if let Some(excluded) = excluded_types {
            for fact in &self.facts {
                if !excluded.contains(&fact.name) {
                    continue;
                }
                let Some(reference) = references.get(&fact.name) else {
                    continue;
                };
                if extended_reference_enums.contains(&fact.name) {
                    continue;
                }
                if let FactData::Typedef {
                    target: TypeRef::Named { name, .. },
                } = &fact.data
                {
                    if references.contains_key(name) || named_type_shape(name).is_some() {
                        continue;
                    }
                    external_alias_candidates
                        .entry(name)
                        .or_default()
                        .insert(format!(
                            "{}::{}",
                            reference.namespace.replace('.', "::"),
                            reference.name
                        ));
                }
            }
        }
        for (name, aliases) in external_alias_candidates {
            if let [alias] = aliases.into_iter().collect::<Vec<_>>().as_slice() {
                type_names
                    .entry(name.to_string())
                    .or_insert_with(|| alias.clone());
            }
        }
        if timing {
            eprintln!(
                "clang plan aliases: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }
        for (name, reference) in references {
            let excluded = excluded_types.is_some_and(|excluded| excluded.contains(name))
                && !extended_reference_enums.contains(name);
            if excluded {
                type_names.insert(
                    name.clone(),
                    format!(
                        "{}::{}",
                        reference.namespace.replace('.', "::"),
                        reference.name
                    ),
                );
            } else if !local_roots.contains(name) {
                type_names.entry(name.clone()).or_insert_with(|| {
                    format!(
                        "{}::{}",
                        reference.namespace.replace('.', "::"),
                        reference.name
                    )
                });
            }
        }
        for (name, fact) in &facts_by_name {
            if required.contains(*name)
                && canonical_named_type(name).is_some()
                && !matches!(fact.data, FactData::Typedef { .. })
            {
                type_names
                    .entry((*name).to_string())
                    .or_insert_with(|| (*name).to_string());
            }
        }
        let alias_names: BTreeSet<_> = type_names
            .iter()
            .filter_map(|(source, target)| (source != target).then_some(target.as_str()))
            .collect();
        let mut types: Vec<_> = facts_by_name
            .into_iter()
            .filter(|(name, _)| required.contains(*name))
            .filter(|(name, _)| !alias_names.contains(*name))
            .map(|(_, fact)| PlannedFact {
                name: type_names
                    .get(fact.name.as_str())
                    .cloned()
                    .unwrap_or_else(|| fact.name.clone()),
                fact,
            })
            .collect();
        let values: Vec<_> = value_roots
            .into_iter()
            .map(|fact| PlannedFact {
                name: fact.name.clone(),
                fact,
            })
            .collect();
        let mut interface_names = BTreeSet::new();
        for planned in &types {
            if matches!(planned.fact.data, FactData::Interface { .. }) {
                for fact in facts_index
                    .get(planned.fact.name.as_str())
                    .into_iter()
                    .flatten()
                {
                    if matches!(fact.data, FactData::Interface { .. })
                        && same_source_declaration(planned.fact, fact)
                    {
                        interface_names.insert((fact.origin.tu.clone(), fact.name.clone()));
                    }
                }
            }
        }
        let translation_units: BTreeSet<_> = self
            .facts
            .iter()
            .map(|fact| fact.origin.tu.as_str())
            .collect();
        for (name, reference) in references {
            if reference.kind == TypeReferenceKind::Interface
                && (excluded_types.is_some_and(|excluded| excluded.contains(name))
                    || !local_roots.contains(name))
            {
                interface_names.extend(
                    translation_units
                        .iter()
                        .map(|tu| ((*tu).to_string(), name.clone())),
                );
            }
        }
        let mut interface_aliases: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        for fact in &self.facts {
            let FactData::Typedef {
                target: TypeRef::Named { name: target, .. } | TypeRef::Generic { name: target, .. },
            } = &fact.data
            else {
                continue;
            };
            if interface_names.contains(&(fact.origin.tu.clone(), target.clone())) {
                interface_aliases
                    .entry((fact.origin.tu.clone(), target.clone()))
                    .or_default()
                    .push(fact.name.clone());
            }
        }
        let mut interface_queue: Vec<_> = interface_names.iter().cloned().collect();
        while let Some(key) = interface_queue.pop() {
            for alias in interface_aliases.get(&key).into_iter().flatten() {
                let alias = (key.0.clone(), alias.clone());
                if interface_names.insert(alias.clone()) {
                    interface_queue.push(alias);
                }
            }
        }
        let mut pointer_interface_aliases = BTreeMap::new();
        for fact in &self.facts {
            let FactData::Typedef {
                target: TypeRef::Pointer { target, .. },
            } = &fact.data
            else {
                continue;
            };
            let (TypeRef::Named { name: target, .. } | TypeRef::Generic { name: target, .. }) =
                target.as_ref()
            else {
                continue;
            };
            if !interface_names.contains(&(fact.origin.tu.clone(), target.clone())) {
                continue;
            }
            let projected = type_names
                .get(target)
                .cloned()
                .unwrap_or_else(|| target.clone());
            if let Some(previous) =
                pointer_interface_aliases.insert(fact.name.clone(), projected.clone())
                && previous != projected
            {
                return Err(Error(format!(
                    "interface pointer alias `{}` has conflicting targets",
                    fact.name
                )));
            }
        }
        for (alias, target) in &pointer_interface_aliases {
            type_names.insert(alias.clone(), target.clone());
        }
        types.retain(|planned| !pointer_interface_aliases.contains_key(&planned.fact.name));
        if timing {
            eprintln!(
                "clang plan interfaces: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }
        let flag_enums = self
            .facts
            .iter()
            .filter_map(|fact| {
                if let FactData::EnumFlag { target } = &fact.data {
                    Some((fact.origin.tu.clone(), target.clone()))
                } else {
                    None
                }
            })
            .collect();
        let mut type_output_names = BTreeSet::new();
        for planned in &types {
            if !type_output_names.insert(planned.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", planned.name)));
            }
        }
        let mut value_output_names = BTreeSet::new();
        for planned in &values {
            if !value_output_names.insert(planned.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", planned.name)));
            }
        }
        for constant in &constants {
            if !value_output_names.insert(constant.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", constant.name)));
            }
        }
        for function in &functions {
            if type_output_names.contains(function.name.as_str())
                || !value_output_names.insert(function.name.as_str())
            {
                return Err(Error(format!("duplicate planned name `{}`", function.name)));
            }
        }
        constants.sort_by(|left, right| left.name.cmp(&right.name));
        functions.sort_by(|left, right| left.name.cmp(&right.name));
        if timing {
            eprintln!(
                "clang plan finalize: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
        }
        Ok(Plan {
            types,
            values,
            functions,
            constants,
            type_names,
            interface_names,
            interface_guids,
            flag_enums,
        })
    }
}

fn write_rdl<'a>(
    namespace: &str,
    items: impl IntoIterator<Item = &'a str>,
) -> Result<String, Error> {
    let namespaces: Vec<_> = namespace
        .split('.')
        .filter(|name| !name.is_empty())
        .collect();
    if namespaces.is_empty() {
        return Err(Error("namespace is empty".to_string()));
    }
    let mut result = String::from("#[win32]\n");
    for (depth, namespace) in namespaces.iter().enumerate() {
        result.push_str(&format!(
            "{}mod {} {{\n",
            "    ".repeat(depth),
            rdl_ident(namespace)
        ));
    }
    let indent = "    ".repeat(namespaces.len() - 1);
    for item in items {
        for line in item.lines() {
            result.push_str(&indent);
            result.push_str(line);
            result.push('\n');
        }
    }
    for depth in (0..namespaces.len()).rev() {
        result.push_str(&format!("{}}}\n", "    ".repeat(depth)));
    }
    Ok(result)
}

struct PlannedFact<'a> {
    fact: &'a Fact,
    name: String,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum OutputKind {
    Value,
    Type,
}

struct Plan<'a> {
    types: Vec<PlannedFact<'a>>,
    values: Vec<PlannedFact<'a>>,
    functions: Vec<&'a Fact>,
    constants: Vec<&'a Constant>,
    type_names: BTreeMap<String, String>,
    interface_names: BTreeSet<(String, String)>,
    interface_guids: BTreeMap<String, String>,
    flag_enums: BTreeSet<(String, String)>,
}

struct TypeProjection<'a> {
    type_names: &'a BTreeMap<String, String>,
    interface_names: &'a BTreeSet<(String, String)>,
    tu: &'a str,
}

impl<'a> TypeProjection<'a> {
    fn new(
        type_names: &'a BTreeMap<String, String>,
        interface_names: &'a BTreeSet<(String, String)>,
        tu: &'a str,
    ) -> Self {
        Self {
            type_names,
            interface_names,
            tu,
        }
    }

    fn name(&self, ty: &TypeRef) -> String {
        planned_emitted_type_name(ty, self.type_names, self.interface_names, self.tu)
    }
}

fn is_type_fact(fact: &Fact) -> bool {
    matches!(
        fact.data,
        FactData::Callback { .. }
            | FactData::Enum { .. }
            | FactData::Interface { .. }
            | FactData::Record { .. }
            | FactData::Typedef { .. }
    )
}

fn fact_data_kind(data: &FactData) -> &'static str {
    match data {
        FactData::Callback { .. } => "Callback",
        FactData::Class { .. } => "Class",
        FactData::Enum { .. } => "Enum",
        FactData::EnumFlag { .. } => "EnumFlag",
        FactData::Function { .. } => "Function",
        FactData::Guid { .. } => "Guid",
        FactData::Interface { .. } => "Interface",
        FactData::Macro { .. } => "Macro",
        FactData::None => "None",
        FactData::PropertyKey { .. } => "PropertyKey",
        FactData::Record { .. } => "Record",
        FactData::Typedef { .. } => "Typedef",
        FactData::Unsupported { .. } => "Unsupported",
    }
}

fn is_value_fact(fact: &Fact) -> bool {
    matches!(
        fact.data,
        FactData::Class { .. } | FactData::Guid { .. } | FactData::PropertyKey { .. }
    )
}

fn is_flat_declaration(
    fact: &Fact,
    facts_by_origin: &HashMap<&Origin, &Fact>,
    references: &BTreeMap<String, TypeReference>,
    root: bool,
) -> bool {
    if references.is_empty() {
        return true;
    }
    let mut namespaces = vec![];
    let mut nested = false;
    let mut parent = fact.parent.as_ref();
    while let Some(origin) = parent {
        let Some(fact) = facts_by_origin.get(origin) else {
            break;
        };
        if fact.kind == FactKind::Namespace {
            namespaces.push(fact.name.as_str());
        } else {
            nested = true;
        }
        parent = fact.parent.as_ref();
    }
    if root && nested {
        return false;
    }
    namespaces.reverse();

    match namespaces.first().copied() {
        None | Some("Windows") => true,
        Some("ABI") => {
            let namespace = namespaces[1..].join(".");
            !references
                .get(&fact.name)
                .is_some_and(|reference| reference.namespace == namespace)
        }
        Some(_) => false,
    }
}

fn defines_local_type(name: &str, fact: &Fact) -> bool {
    match &fact.data {
        FactData::Callback { .. } => true,
        FactData::Enum { .. } | FactData::Interface { .. } | FactData::Record { .. } => {
            fact.definition
        }
        FactData::Typedef {
            target: TypeRef::Named { name: target, .. },
        } => target != name,
        FactData::Typedef { .. } => true,
        _ => false,
    }
}

fn is_root_fact(fact: &Fact) -> bool {
    (is_type_fact(fact) || is_value_fact(fact))
        && !(canonical_named_type(&fact.name).is_some()
            && matches!(fact.data, FactData::Typedef { .. }))
        && (!matches!(
            fact.data,
            FactData::Record { .. } | FactData::Interface { .. }
        ) || fact.definition)
}

fn declaration_kind(
    ty: &TypeRef,
    tu: &str,
    facts_index: &HashMap<&str, Vec<&Fact>>,
    seen: &mut BTreeSet<Location>,
) -> Option<FactKind> {
    let TypeRef::Named { name, declaration } = ty else {
        return None;
    };
    if !seen.insert(declaration.clone()) {
        return None;
    }
    let fact = facts_index
        .get(name.as_str())?
        .iter()
        .find(|fact| fact.origin.tu == tu && fact.spelling == *declaration)?;
    if let FactData::Typedef { target } = &fact.data {
        declaration_kind(target, tu, facts_index, seen)
    } else {
        Some(fact.kind)
    }
}

fn is_tag_declaration(fact: &Fact) -> bool {
    matches!(
        fact.data,
        FactData::Enum { .. } | FactData::Record { .. } | FactData::Interface { .. }
    )
}

fn incomplete_declaration_matches_definition(declaration: &Fact, definition: &Fact) -> bool {
    !declaration.definition
        && declaration.kind == definition.kind
        && is_tag_declaration(declaration)
        && is_tag_declaration(definition)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct TypeShape(u64, u64);

#[derive(Default)]
struct ShapeCache {
    declarations: HashMap<String, HashMap<Location, TypeShape>>,
    recursive: HashMap<String, HashMap<Location, HashMap<TypeShape, TypeShape>>>,
}

impl ShapeCache {
    fn declaration(&self, tu: &str, declaration: &Location) -> Option<TypeShape> {
        self.declarations.get(tu)?.get(declaration).copied()
    }

    fn insert_declaration(&mut self, tu: &str, declaration: &Location, shape: TypeShape) {
        self.declarations
            .entry(tu.to_string())
            .or_default()
            .insert(declaration.clone(), shape);
    }

    fn is_recursive(&self, tu: &str, declaration: &Location) -> bool {
        self.recursive
            .get(tu)
            .is_some_and(|declarations| declarations.contains_key(declaration))
    }

    fn recursive(&self, tu: &str, declaration: &Location, context: TypeShape) -> Option<TypeShape> {
        self.recursive
            .get(tu)?
            .get(declaration)?
            .get(&context)
            .copied()
    }

    fn insert_recursive(
        &mut self,
        tu: &str,
        declaration: &Location,
        context: TypeShape,
        shape: TypeShape,
    ) {
        self.recursive
            .entry(tu.to_string())
            .or_default()
            .entry(declaration.clone())
            .or_default()
            .insert(context, shape);
    }
}

fn type_shape(value: &str) -> TypeShape {
    use std::hash::{Hash, Hasher};

    // Recursive declarations share compact fingerprints instead of expanded shape strings.
    let mut first = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut first);

    let mut second = 0x9e3779b97f4a7c15u64;
    for byte in value.bytes() {
        second ^= u64::from(byte);
        second = second.wrapping_mul(0x100000001b3);
        second ^= second >> 32;
    }
    TypeShape(first.finish(), second)
}

fn recursion_shape(seen: &BTreeSet<Location>) -> TypeShape {
    use std::hash::{Hash, Hasher};

    let mut first = std::collections::hash_map::DefaultHasher::new();
    seen.hash(&mut first);

    let mut second = std::collections::hash_map::DefaultHasher::new();
    1u8.hash(&mut second);
    seen.hash(&mut second);

    TypeShape(first.finish(), second.finish())
}

fn preferred_fact<'a>(facts: &[&'a Fact]) -> &'a Fact {
    facts
        .iter()
        .copied()
        .min_by_key(|fact| (!fact.root, &fact.origin))
        .unwrap()
}

fn choose_value_root<'a>(name: &str, roots: &[&'a Fact]) -> Result<&'a Fact, Error> {
    let distinct = distinct_source_declarations(roots);
    let Some(first) = distinct.first() else {
        return Err(Error(format!("missing value root `{name}`")));
    };
    if distinct.iter().all(|fact| fact.data == first.data) {
        Ok(preferred_fact(&distinct))
    } else {
        Err(Error(format!("ambiguous value root `{name}`")))
    }
}

fn types_alias_value_class(name: &str, types: &[&Fact], values: &[&Fact]) -> bool {
    types.iter().all(|fact| {
        values
            .iter()
            .copied()
            .filter(|value| matches!(value.data, FactData::Class { .. }))
            .any(|value| {
                fact.origin == value.origin
                    || (fact.origin.tu == value.origin.tu
                        && fact.parent == value.parent
                        && matches!(
                            &fact.data,
                            FactData::Typedef {
                                target: TypeRef::Named { name: target, .. }
                            } if target == name
                        ))
            })
    })
}

fn choose_type_root<'a>(
    name: &str,
    roots: &[&'a Fact],
    facts_index: &HashMap<&str, Vec<&'a Fact>>,
) -> Result<&'a Fact, Error> {
    choose_type_root_cached(name, roots, facts_index, &mut ShapeCache::default())
}

fn choose_type_root_cached<'a>(
    name: &str,
    roots: &[&'a Fact],
    facts_index: &HashMap<&str, Vec<&'a Fact>>,
    shape_cache: &mut ShapeCache,
) -> Result<&'a Fact, Error> {
    let distinct = distinct_source_declarations(roots);
    if let [root] = distinct.as_slice() {
        return emittable_type(name, root);
    }
    if let Some(first) = distinct.first()
        && distinct.iter().all(|fact| {
            fact.origin.tu == first.origin.tu
                && fact.kind == first.kind
                && fact.definition == first.definition
                && fact.data == first.data
        })
    {
        return emittable_type(name, preferred_fact(&distinct));
    }
    if let Some(target_name) = distinct.first().and_then(|first| match &first.data {
        FactData::Typedef {
            target: TypeRef::Named { name, .. },
        } => Some(name),
        _ => None,
    }) && distinct.iter().all(|fact| {
        matches!(
            &fact.data,
            FactData::Typedef {
                target: TypeRef::Named { name, .. }
            } if name == target_name
        )
    }) {
        let complete: Vec<_> = distinct
            .iter()
            .copied()
            .filter(|fact| {
                let FactData::Typedef {
                    target: TypeRef::Named { name, declaration },
                } = &fact.data
                else {
                    return false;
                };
                facts_index
                    .get(name.as_str())
                    .into_iter()
                    .flatten()
                    .any(|target| {
                        target.origin.tu == fact.origin.tu
                            && target.spelling == *declaration
                            && target.definition
                            && matches!(
                                target.data,
                                FactData::Enum { .. }
                                    | FactData::Record { .. }
                                    | FactData::Interface { .. }
                            )
                    })
            })
            .collect();
        if let [complete] = complete.as_slice() {
            let FactData::Typedef {
                target: complete_target,
            } = &complete.data
            else {
                unreachable!()
            };
            let complete_kind = declaration_kind(
                complete_target,
                &complete.origin.tu,
                facts_index,
                &mut BTreeSet::new(),
            );
            if complete_kind.is_some()
                && distinct.iter().all(|fact| {
                    let FactData::Typedef { target } = &fact.data else {
                        return false;
                    };
                    declaration_kind(target, &fact.origin.tu, facts_index, &mut BTreeSet::new())
                        == complete_kind
                })
            {
                return emittable_type(name, complete);
            }
        }
    }
    if let Some(first) = distinct.first()
        && let FactData::Typedef {
            target: first_target,
        } = &first.data
        && distinct.iter().all(|fact| {
            ((fact.origin.tu == first.origin.tu && fact.parent == first.parent)
                || (fact.parent.is_none() && first.parent.is_none()))
                && matches!(
                    &fact.data,
                    FactData::Typedef { target }
                        if equivalent_type(
                            first_target,
                            &first.origin.tu,
                            target,
                            &fact.origin.tu,
                            facts_index,
                            shape_cache,
                        )
                )
        })
    {
        return emittable_type(name, preferred_fact(&distinct));
    }

    fn resolved_type_shape(
        ty: &TypeRef,
        tu: &str,
        facts_index: &HashMap<&str, Vec<&Fact>>,
        shape_cache: &mut ShapeCache,
    ) -> TypeShape {
        fn write(
            ty: &TypeRef,
            tu: &str,
            facts_index: &HashMap<&str, Vec<&Fact>>,
            seen: &mut BTreeSet<Location>,
            cache: &mut ShapeCache,
            cycle: &mut bool,
        ) -> TypeShape {
            match ty {
                TypeRef::Named { name, declaration } => {
                    if let Some(shape) = cache.declaration(tu, declaration) {
                        return shape;
                    }
                    if cache.is_recursive(tu, declaration) {
                        let context = recursion_shape(seen);
                        if let Some(shape) = cache.recursive(tu, declaration, context) {
                            *cycle = true;
                            return shape;
                        }
                    }
                    if !seen.insert(declaration.clone()) {
                        *cycle = true;
                        return type_shape(&format!(
                            "named:{}",
                            named_type_shape(name).unwrap_or(name)
                        ));
                    }
                    let mut nested_cycle = false;
                    if let Some(target) = facts_index
                        .get(name.as_str())
                        .into_iter()
                        .flatten()
                        .find(|fact| fact.origin.tu == tu && fact.spelling == *declaration)
                    {
                        let result = match &target.data {
                            FactData::Typedef { target } => Some(write(
                                target,
                                tu,
                                facts_index,
                                seen,
                                cache,
                                &mut nested_cycle,
                            )),
                            FactData::Record {
                                base,
                                fields,
                                size,
                                align,
                                packing,
                                alignment,
                                union,
                            } => {
                                let base = base.as_ref().map(|base| {
                                    write(base, tu, facts_index, seen, cache, &mut nested_cycle)
                                });
                                let fields = fields
                                    .iter()
                                    .map(|field| {
                                        format!(
                                            "{}:{}:{}:{}:{:?}:{:?}",
                                            field.name,
                                            field.offset,
                                            field.align,
                                            field.size,
                                            field.bit_width,
                                            write(
                                                &field.ty,
                                                tu,
                                                facts_index,
                                                seen,
                                                cache,
                                                &mut nested_cycle,
                                            )
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(",");
                                Some(type_shape(&format!(
                                    "record:{base:?}:{size}:{align}:{packing:?}:{alignment:?}:{union}:{fields}"
                                )))
                            }
                            FactData::Enum {
                                repr,
                                variants,
                                fixed,
                                scoped,
                            } => Some(type_shape(&format!(
                                "enum:{repr:?}:{variants:?}:{fixed}:{scoped}"
                            ))),
                            FactData::Callback {
                                convention,
                                params,
                                result,
                            } => {
                                let params = params
                                    .iter()
                                    .map(|param| {
                                        format!(
                                            "{}:{:?}:{:?}",
                                            param.name,
                                            write(
                                                &param.ty,
                                                tu,
                                                facts_index,
                                                seen,
                                                cache,
                                                &mut nested_cycle,
                                            ),
                                            param.annotation
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(",");
                                Some(type_shape(&format!(
                                    "callback:{convention:?}:({params}):{:?}",
                                    write(result, tu, facts_index, seen, cache, &mut nested_cycle,)
                                )))
                            }
                            FactData::Interface {
                                base,
                                guid,
                                methods,
                            } => {
                                let base = base.as_ref().map(|base| {
                                    write(base, tu, facts_index, seen, cache, &mut nested_cycle)
                                });
                                let methods = methods
                                    .iter()
                                    .map(|method| {
                                        let params = method
                                            .params
                                            .iter()
                                            .map(|param| {
                                                format!(
                                                    "{}:{:?}:{:?}",
                                                    param.name,
                                                    write(
                                                        &param.ty,
                                                        tu,
                                                        facts_index,
                                                        seen,
                                                        cache,
                                                        &mut nested_cycle,
                                                    ),
                                                    param.annotation
                                                )
                                            })
                                            .collect::<Vec<_>>()
                                            .join(",");
                                        format!(
                                            "{}:({params}):{:?}:{}",
                                            method.name,
                                            write(
                                                &method.result,
                                                tu,
                                                facts_index,
                                                seen,
                                                cache,
                                                &mut nested_cycle,
                                            ),
                                            method.special
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(",");
                                Some(type_shape(&format!(
                                    "interface:{base:?}:{guid:?}:{methods}"
                                )))
                            }
                            _ => None,
                        };
                        seen.remove(declaration);
                        *cycle |= nested_cycle;
                        if let Some(result) = result {
                            if !nested_cycle {
                                cache.insert_declaration(tu, declaration, result);
                            } else {
                                cache.insert_recursive(
                                    tu,
                                    declaration,
                                    recursion_shape(seen),
                                    result,
                                );
                            }
                            return result;
                        }
                    }
                    seen.remove(declaration);
                    type_shape(&format!("named:{}", named_type_shape(name).unwrap_or(name)))
                }
                TypeRef::Pointer { mutable, target } => type_shape(&format!(
                    "pointer:{mutable}:{:?}",
                    write(target, tu, facts_index, seen, cache, cycle)
                )),
                TypeRef::Reference { mutable, target } => type_shape(&format!(
                    "reference:{mutable}:{:?}",
                    write(target, tu, facts_index, seen, cache, cycle)
                )),
                TypeRef::FunctionPointer {
                    convention,
                    params,
                    result,
                } => type_shape(&format!(
                    "function:{convention:?}:({:?}):{:?}",
                    params
                        .iter()
                        .map(|param| { write(param, tu, facts_index, seen, cache, cycle) })
                        .collect::<Vec<_>>(),
                    write(result, tu, facts_index, seen, cache, cycle)
                )),
                TypeRef::OpaquePointer { mutable, tag } => {
                    type_shape(&format!("opaque:{mutable}:{tag}"))
                }
                TypeRef::Array { target, len } => type_shape(&format!(
                    "array:{len}:{:?}",
                    write(target, tu, facts_index, seen, cache, cycle)
                )),
                TypeRef::Generic { name, args, .. } => type_shape(&format!(
                    "generic:{name}:{:?}",
                    args.iter()
                        .map(|arg| { write(arg, tu, facts_index, seen, cache, cycle) })
                        .collect::<Vec<_>>()
                )),
                TypeRef::InlineRecord(record) => type_shape(&format!("record:{record:?}")),
                other => type_shape(&format!("{other:?}")),
            }
        }

        write(
            ty,
            tu,
            facts_index,
            &mut BTreeSet::new(),
            shape_cache,
            &mut false,
        )
    }

    fn equivalent_type(
        left: &TypeRef,
        left_tu: &str,
        right: &TypeRef,
        right_tu: &str,
        facts_index: &HashMap<&str, Vec<&Fact>>,
        shape_cache: &mut ShapeCache,
    ) -> bool {
        fn incomplete(
            ty: &TypeRef,
            tu: &str,
            facts_index: &HashMap<&str, Vec<&Fact>>,
            seen: &mut BTreeSet<Location>,
        ) -> bool {
            let TypeRef::Named { name, declaration } = ty else {
                return false;
            };
            if !seen.insert(declaration.clone()) {
                return true;
            }
            let Some(fact) = facts_index
                .get(name.as_str())
                .into_iter()
                .flatten()
                .find(|fact| fact.origin.tu == tu && fact.spelling == *declaration)
            else {
                return true;
            };
            match &fact.data {
                FactData::Typedef { target } => incomplete(target, tu, facts_index, seen),
                FactData::Record { .. } | FactData::Interface { .. } => !fact.definition,
                FactData::Enum { fixed, .. } => !fact.definition && !fixed,
                _ => false,
            }
        }

        fn matching_incomplete_declaration_kind(
            left: &TypeRef,
            left_tu: &str,
            right: &TypeRef,
            right_tu: &str,
            facts_index: &HashMap<&str, Vec<&Fact>>,
        ) -> bool {
            if !incomplete(left, left_tu, facts_index, &mut BTreeSet::new())
                && !incomplete(right, right_tu, facts_index, &mut BTreeSet::new())
            {
                return false;
            }
            let left_kind = declaration_kind(left, left_tu, facts_index, &mut BTreeSet::new());
            left_kind.is_some()
                && left_kind == declaration_kind(right, right_tu, facts_index, &mut BTreeSet::new())
        }

        match (left, right) {
            (
                TypeRef::Named {
                    name: left_name, ..
                },
                TypeRef::Named {
                    name: right_name, ..
                },
            ) if left_name == right_name
                && matching_incomplete_declaration_kind(
                    left,
                    left_tu,
                    right,
                    right_tu,
                    facts_index,
                ) =>
            {
                true
            }
            (
                TypeRef::Pointer {
                    mutable: left_mutable,
                    target: left_target,
                },
                TypeRef::Pointer {
                    mutable: right_mutable,
                    target: right_target,
                },
            )
            | (
                TypeRef::Reference {
                    mutable: left_mutable,
                    target: left_target,
                },
                TypeRef::Reference {
                    mutable: right_mutable,
                    target: right_target,
                },
            ) => {
                left_mutable == right_mutable
                    && equivalent_type(
                        left_target,
                        left_tu,
                        right_target,
                        right_tu,
                        facts_index,
                        shape_cache,
                    )
            }
            (
                TypeRef::Array {
                    target: left_target,
                    len: left_len,
                },
                TypeRef::Array {
                    target: right_target,
                    len: right_len,
                },
            ) => {
                left_len == right_len
                    && equivalent_type(
                        left_target,
                        left_tu,
                        right_target,
                        right_tu,
                        facts_index,
                        shape_cache,
                    )
            }
            _ => {
                resolved_type_shape(left, left_tu, facts_index, shape_cache)
                    == resolved_type_shape(right, right_tu, facts_index, shape_cache)
            }
        }
    }

    fn equivalent_record(
        left: &Fact,
        right: &Fact,
        facts_index: &HashMap<&str, Vec<&Fact>>,
        shape_cache: &mut ShapeCache,
    ) -> bool {
        let (
            FactData::Record {
                base: left_base,
                fields: left_fields,
                size: left_size,
                align: left_align,
                packing: left_packing,
                alignment: left_alignment,
                union: left_union,
            },
            FactData::Record {
                base: right_base,
                fields: right_fields,
                size: right_size,
                align: right_align,
                packing: right_packing,
                alignment: right_alignment,
                union: right_union,
            },
        ) = (&left.data, &right.data)
        else {
            return false;
        };
        left_size == right_size
            && left_align == right_align
            && left_packing == right_packing
            && left_alignment == right_alignment
            && left_union == right_union
            && match (left_base, right_base) {
                (None, None) => true,
                (Some(left_base), Some(right_base)) => equivalent_type(
                    left_base,
                    &left.origin.tu,
                    right_base,
                    &right.origin.tu,
                    facts_index,
                    shape_cache,
                ),
                _ => false,
            }
            && left_fields.len() == right_fields.len()
            && left_fields
                .iter()
                .zip(right_fields)
                .all(|(left_field, right_field)| {
                    left_field.name == right_field.name
                        && left_field.offset == right_field.offset
                        && left_field.align == right_field.align
                        && left_field.size == right_field.size
                        && left_field.bit_width == right_field.bit_width
                        && equivalent_type(
                            &left_field.ty,
                            &left.origin.tu,
                            &right_field.ty,
                            &right.origin.tu,
                            facts_index,
                            shape_cache,
                        )
                })
    }

    let declarations: Vec<_> = distinct
        .iter()
        .copied()
        .filter(|fact| {
            matches!(
                fact.data,
                FactData::Enum { .. } | FactData::Record { .. } | FactData::Interface { .. }
            )
        })
        .collect();
    if let Some(first) = declarations.first()
        && !first.definition
        && declarations.iter().all(|fact| {
            !fact.definition
                && fact.kind == first.kind
                && fact.data == first.data
                && (fact.origin.tu == first.origin.tu || fact.spelling == first.spelling)
        })
        && distinct.iter().all(|fact| {
            declarations.contains(fact)
                || matches!(
                    &fact.data,
                    FactData::Typedef {
                        target: TypeRef::Named { declaration, .. }
                    } if declarations.iter().any(|target| {
                        target.origin.tu == fact.origin.tu && target.spelling == *declaration
                    })
                )
        })
    {
        return emittable_type(name, preferred_fact(&declarations));
    }

    let definitions: Vec<_> = distinct
        .iter()
        .copied()
        .filter(|fact| {
            matches!(
                fact.data,
                FactData::Enum { .. } | FactData::Record { .. } | FactData::Interface { .. }
            ) && fact.definition
        })
        .collect();
    if definitions.len() > 1 {
        let root = preferred_fact(&definitions);
        let equivalent_definitions = definitions.iter().all(|fact| {
            fact.kind == root.kind
                && ((fact.origin.tu == root.origin.tu && fact.data == root.data)
                    || equivalent_record(root, fact, facts_index, shape_cache))
        });
        let definitions_and_aliases = distinct.iter().all(|fact| {
            definitions.contains(fact)
                || matches!(
                    &fact.data,
                    FactData::Typedef {
                        target: TypeRef::Named { declaration, .. }
                    } if definitions.iter().any(|definition| definition.spelling == *declaration)
                )
        });
        if equivalent_definitions && definitions_and_aliases {
            return Ok(root);
        }
    }
    if let [root] = definitions.as_slice() {
        let aliases_target_root = distinct.iter().all(|fact| {
            fact.origin == root.origin
                || matches!(
                    &fact.data,
                    FactData::Typedef {
                        target: TypeRef::Named { declaration, .. }
                    } if declaration == &root.spelling
                )
        });
        if aliases_target_root {
            return Ok(root);
        }
        let compatible_declarations_and_aliases = distinct.iter().all(|fact| {
            let linked_nested_declaration = root.parent.is_some()
                && fact.parent.is_none()
                && facts_index.values().flatten().any(|alias| {
                    alias.root
                        && alias.origin.tu == fact.origin.tu
                        && matches!(
                            &alias.data,
                            FactData::Typedef {
                                target: TypeRef::Named { declaration, .. }
                            } if declaration == &fact.spelling
                        )
                });
            fact.origin == root.origin
                || (incomplete_declaration_matches_definition(fact, root)
                    && ((fact.parent.is_none() && root.parent.is_none())
                        || (fact.origin.tu == root.origin.tu
                            && (fact.parent == root.parent || linked_nested_declaration))))
                || matches!(
                    &fact.data,
                    FactData::Typedef {
                        target: TypeRef::Named { declaration, .. }
                    } if facts_index
                        .get(name)
                        .into_iter()
                        .flatten()
                        .any(|target| {
                            target.origin.tu == fact.origin.tu
                                && target.spelling == *declaration
                                && target.kind == root.kind
                                && (target.origin == root.origin || !target.definition)
                                && target.parent.is_none()
                                && root.parent.is_none()
                        })
                )
        });
        if compatible_declarations_and_aliases {
            return Ok(root);
        }
    }
    let choices = distinct
        .iter()
        .map(|fact| {
            format!(
                "{}:{} {:?}/{} {}",
                fact.spelling.file,
                fact.spelling.offset,
                fact.kind,
                fact_data_kind(&fact.data),
                if fact.definition {
                    "definition"
                } else {
                    "declaration"
                }
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    Err(Error(format!("ambiguous type root `{name}`: {choices}")))
}

fn choose_constant_root<'a>(name: &str, roots: &[&'a Constant]) -> Result<&'a Constant, Error> {
    let Some(first) = roots.first() else {
        return Err(Error(format!("missing constant root `{name}`")));
    };
    if roots.iter().all(|constant| {
        constant_types_match(&constant.ty, &first.ty) && constant.value == first.value
    }) {
        Ok(roots
            .iter()
            .min_by_key(|constant| &constant.spelling)
            .copied()
            .unwrap())
    } else {
        Err(Error(format!("ambiguous constant root `{name}`")))
    }
}

fn constant_types_match(left: &TypeRef, right: &TypeRef) -> bool {
    left == right
        || matches!(
            (left, right),
            (
                TypeRef::Named {
                    name: left_name, ..
                },
                TypeRef::Named {
                    name: right_name, ..
                },
            ) if named_type_shape(left_name) == named_type_shape(right_name)
                && named_type_shape(left_name).is_some()
        )
}

fn choose_function_root<'a>(name: &str, roots: &[&'a Fact]) -> Result<&'a Fact, Error> {
    let distinct = distinct_source_declarations(roots);
    if let [root] = distinct.as_slice() {
        return Ok(root);
    }
    if let Some(first) = distinct.first()
        && let FactData::Function {
            link_name: first_link_name,
            ..
        } = &first.data
        && distinct.iter().all(|fact| {
            fact.origin.tu == first.origin.tu
                && fact.parent == first.parent
                && matches!(
                    &fact.data,
                    FactData::Function { link_name, .. } if link_name == first_link_name
                )
        })
    {
        return Ok(distinct
            .iter()
            .min_by_key(|fact| &fact.spelling)
            .copied()
            .unwrap());
    }
    let choices = distinct
        .iter()
        .map(|fact| {
            format!(
                "{}:{} {:?}",
                fact.spelling.file, fact.spelling.offset, fact.data
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    Err(Error(format!(
        "ambiguous function root `{name}`: {choices}"
    )))
}

fn distinct_source_declarations<'a>(roots: &[&'a Fact]) -> Vec<&'a Fact> {
    let mut distinct: Vec<&Fact> = vec![];
    for &root in roots {
        if !distinct
            .iter()
            .any(|existing| same_source_declaration(existing, root))
        {
            distinct.push(root);
        }
    }
    distinct
}

fn same_source_declaration(left: &Fact, right: &Fact) -> bool {
    left.kind == right.kind
        && left.name == right.name
        && left.spelling == right.spelling
        && left.definition == right.definition
        && left.data == right.data
}

fn emittable_type<'a>(name: &str, fact: &'a Fact) -> Result<&'a Fact, Error> {
    match fact.data {
        FactData::Callback { .. } | FactData::Typedef { .. } if fact.definition => Ok(fact),
        FactData::Enum { fixed, .. } if fact.definition || fixed => Ok(fact),
        FactData::Class { .. }
        | FactData::Guid { .. }
        | FactData::PropertyKey { .. }
        | FactData::Record { .. }
        | FactData::Interface { .. } => Ok(fact),
        _ => Err(Error(format!("type root `{name}` is not emittable"))),
    }
}

fn underlying_enum_fact<'a>(
    fact: &'a Fact,
    facts_by_declaration: &BTreeMap<(String, Location), &'a Fact>,
    seen: &mut BTreeSet<(String, Location)>,
) -> Option<&'a Fact> {
    match &fact.data {
        FactData::Enum { .. } => Some(fact),
        FactData::Typedef {
            target: TypeRef::Named { declaration, .. },
        } => {
            let key = (fact.origin.tu.clone(), declaration.clone());
            if !seen.insert(key.clone()) {
                return None;
            }
            underlying_enum_fact(*facts_by_declaration.get(&key)?, facts_by_declaration, seen)
        }
        _ => None,
    }
}

struct LayoutContext<'a, 'facts> {
    facts_index: &'a HashMap<&'facts str, Vec<&'facts Fact>>,
    planned_types: &'a BTreeMap<&'facts str, &'facts Fact>,
}

fn validate_fact_layouts(
    fact: &Fact,
    layout: &LayoutContext<'_, '_>,
    safe_layouts: &mut HashMap<String, HashSet<Location>>,
    validated: &mut HashSet<Origin>,
) -> Result<(), Error> {
    let mut validate = |ty| {
        validate_complete_layout(
            ty,
            &fact.origin.tu,
            layout,
            safe_layouts,
            &mut BTreeSet::new(),
            validated,
        )
    };
    match &fact.data {
        FactData::Callback { params, result, .. } | FactData::Function { params, result, .. } => {
            validate(result)?;
            for param in params {
                validate(&param.ty)?;
            }
        }
        FactData::Record { base, fields, .. } => {
            if let Some(base) = base {
                validate(base)?;
            }
            for field in fields {
                validate(&field.ty)?;
            }
        }
        FactData::Interface { base, methods, .. } => {
            if let Some(base) = base {
                validate(base)?;
            }
            for method in methods {
                validate(&method.result)?;
                for param in &method.params {
                    validate(&param.ty)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn known_complete_layout(
    ty: &TypeRef,
    tu: &str,
    safe_layouts: &HashMap<String, HashSet<Location>>,
    references: &BTreeMap<String, TypeReference>,
) -> bool {
    match ty {
        TypeRef::Pointer { .. }
        | TypeRef::Reference { .. }
        | TypeRef::FunctionPointer { .. }
        | TypeRef::OpaquePointer { .. }
        | TypeRef::Void
        | TypeRef::String
        | TypeRef::Object
        | TypeRef::Scalar(_)
        | TypeRef::Generic { .. } => true,
        TypeRef::Array { target, .. } => {
            known_complete_layout(target, tu, safe_layouts, references)
        }
        TypeRef::InlineRecord(record) => {
            record
                .base
                .as_ref()
                .is_none_or(|base| known_complete_layout(base, tu, safe_layouts, references))
                && record
                    .fields
                    .iter()
                    .all(|field| known_complete_layout(&field.ty, tu, safe_layouts, references))
        }
        TypeRef::Named { name, declaration } => {
            references.contains_key(name)
                || safe_layouts
                    .get(tu)
                    .is_some_and(|safe| safe.contains(declaration))
        }
    }
}

fn validate_complete_layout(
    ty: &TypeRef,
    tu: &str,
    layout: &LayoutContext<'_, '_>,
    safe_layouts: &mut HashMap<String, HashSet<Location>>,
    seen: &mut BTreeSet<(String, Location)>,
    validated: &mut HashSet<Origin>,
) -> Result<(), Error> {
    match ty {
        TypeRef::Pointer { .. }
        | TypeRef::Reference { .. }
        | TypeRef::FunctionPointer { .. }
        | TypeRef::OpaquePointer { .. }
        | TypeRef::Void
        | TypeRef::String
        | TypeRef::Object
        | TypeRef::Scalar(_)
        | TypeRef::Generic { .. } => Ok(()),
        TypeRef::Array { target, .. } => {
            validate_complete_layout(target, tu, layout, safe_layouts, seen, validated)
        }
        TypeRef::InlineRecord(record) => {
            if let Some(base) = &record.base {
                validate_complete_layout(base, tu, layout, safe_layouts, seen, validated)?;
            }
            for field in &record.fields {
                validate_complete_layout(&field.ty, tu, layout, safe_layouts, seen, validated)?;
            }
            Ok(())
        }
        TypeRef::Named { name, declaration } => {
            if safe_layouts
                .get(tu)
                .is_some_and(|safe| safe.contains(declaration))
            {
                return Ok(());
            }
            let matches: Vec<_> = layout
                .facts_index
                .get(name.as_str())
                .into_iter()
                .flatten()
                .copied()
                .filter(|fact| fact.origin.tu == tu && fact.spelling == *declaration)
                .collect();
            let fact = if matches
                .iter()
                .any(|fact| matches!(fact.data, FactData::Typedef { .. }))
            {
                choose_type_root(name, &matches, layout.facts_index)?
            } else if let Some(fact) = layout.planned_types.get(name.as_str()).copied() {
                fact
            } else if !matches.is_empty() {
                choose_type_root(name, &matches, layout.facts_index)?
            } else {
                return Ok(());
            };
            if validated.contains(&fact.origin) {
                return Ok(());
            }
            if !seen.insert((fact.origin.tu.clone(), fact.spelling.clone())) {
                return Ok(());
            }
            let result = match &fact.data {
                FactData::Record { .. } if !fact.definition => Err(Error(format!(
                    "incomplete record `{name}` is used by value in translation unit `{tu}`"
                ))),
                FactData::Typedef { target } => validate_complete_layout(
                    target,
                    &fact.origin.tu,
                    layout,
                    safe_layouts,
                    seen,
                    validated,
                ),
                _ => Ok(()),
            };
            if result.is_ok() {
                validated.insert(fact.origin.clone());
                safe_layouts
                    .entry(tu.to_string())
                    .or_default()
                    .insert(declaration.clone());
            }
            result
        }
    }
}

enum TypeEdge<'a> {
    Type(&'a TypeRef),
    Projected(&'static str),
}

fn queue_type_edges<'a>(fact: &'a Fact, queue: &mut Vec<(&'a str, TypeEdge<'a>)>) {
    if let FactData::Typedef { target } = &fact.data {
        queue.push((fact.origin.tu.as_str(), TypeEdge::Type(target)));
    } else if let FactData::Callback { params, result, .. } = &fact.data {
        queue.push((fact.origin.tu.as_str(), TypeEdge::Type(result)));
        for param in params {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(&param.ty)));
        }
    } else if let FactData::PropertyKey { ty, .. } = &fact.data {
        queue.push((fact.origin.tu.as_str(), TypeEdge::Projected(ty)));
    } else if let FactData::Record { base, fields, .. } = &fact.data {
        if let Some(base) = base {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(base)));
        }
        for field in fields {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(&field.ty)));
        }
    } else if let FactData::Interface { base, methods, .. } = &fact.data {
        if let Some(base) = base {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(base)));
        }
        for method in methods {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(&method.result)));
            for param in &method.params {
                queue.push((fact.origin.tu.as_str(), TypeEdge::Type(&param.ty)));
                if let Some(name) = parameter_string_name(param) {
                    queue.push((fact.origin.tu.as_str(), TypeEdge::Projected(name)));
                }
            }
        }
    }
}

fn queue_function_edges<'a>(fact: &'a Fact, queue: &mut Vec<(&'a str, TypeEdge<'a>)>) {
    if let FactData::Function { params, result, .. } = &fact.data {
        queue.push((fact.origin.tu.as_str(), TypeEdge::Type(result)));
        for param in params {
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(&param.ty)));
            if let Some(name) = parameter_string_name(param) {
                queue.push((fact.origin.tu.as_str(), TypeEdge::Projected(name)));
            }
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Error {}

struct BitfieldGroup {
    start: usize,
    end: usize,
    offset: i64,
}

fn bitfield_groups(fields: &[Field]) -> Result<Vec<BitfieldGroup>, Error> {
    let mut groups = vec![];
    let mut index = 0;
    while index < fields.len() {
        let field = &fields[index];
        let Some(width) = field.bit_width else {
            index += 1;
            continue;
        };
        if width == 0 {
            index += 1;
            continue;
        }
        let start = index;
        let offset = field.offset;
        let limit = offset + field.size * 8;
        index += 1;
        while index < fields.len() {
            let next = &fields[index];
            let Some(width) = next.bit_width else {
                break;
            };
            if width == 0 || next.size != field.size || next.offset + i64::from(width) > limit {
                break;
            }
            index += 1;
        }
        groups.push(BitfieldGroup {
            start,
            end: index,
            offset,
        });
    }
    Ok(groups)
}

fn write_callback(
    name: &str,
    convention: CallingConvention,
    params: &[Parameter],
    result: &TypeRef,
    projection: &TypeProjection,
) -> Result<String, Error> {
    let params = write_params(params, projection)?.join(", ");
    let result = if *result == TypeRef::Void {
        String::new()
    } else {
        format!(" -> {}", projection.name(result))
    };
    Ok(format!(
        "    extern{} fn {}({params}){result};\n",
        calling_convention(convention),
        rdl_ident(name)
    ))
}

fn write_params(
    params: &[Parameter],
    projection: &TypeProjection<'_>,
) -> Result<Vec<String>, Error> {
    params
        .iter()
        .map(|param| {
            Ok(format!(
                "{}{}: {}",
                param_attributes(
                    param,
                    params,
                    emitted_pointer_is_mutable(param, projection.interface_names, projection.tu),
                )?,
                rdl_ident(&param.name),
                planned_param_type_name(
                    param,
                    projection.type_names,
                    projection.interface_names,
                    projection.tu,
                )
            ))
        })
        .collect()
}

fn param_attributes(
    param: &Parameter,
    params: &[Parameter],
    emitted_mutable: bool,
) -> Result<String, Error> {
    let annotation = &param.annotation;
    if let Some(reason) = &annotation.unsupported {
        return Err(Error(reason.clone()));
    }
    let mut result = String::new();
    if let Some(size) = &annotation.size {
        match &size.value {
            SalSizeValue::Constant(value) if !size.bytes => {
                result.push_str(&format!("#[len_const({value})] "));
            }
            SalSizeValue::Parameter(name) | SalSizeValue::IndirectParameter(name) => {
                let index = params
                    .iter()
                    .position(|param| param.name == *name)
                    .ok_or_else(|| Error(format!("unresolved SAL size parameter `{name}`")))?;
                let attr = if size.bytes {
                    "size_param"
                } else {
                    "len_param"
                };
                result.push_str(&format!("#[{attr}({index})] "));
            }
            SalSizeValue::Constant(_) => {
                return Err(Error(
                    "constant byte-size SAL annotations are unsupported".to_string(),
                ));
            }
            SalSizeValue::Expression(_) => {}
        }
    }
    if annotation.reserved {
        result.push_str("#[reserved] ");
    }
    if annotation.com_out_ptr {
        result.push_str("#[iid_is] ");
    }
    if annotation.input && (annotation.output || emitted_mutable) {
        result.push_str("#[in] ");
    }
    if annotation.output && (annotation.input || !emitted_mutable) {
        result.push_str("#[out] ");
    }
    if annotation.optional {
        result.push_str("#[opt] ");
    }
    if annotation.retval {
        result.push_str("#[retval] ");
    }
    Ok(result)
}

fn write_interface(
    name: &str,
    base: Option<&TypeRef>,
    guid: Option<&str>,
    methods: &[Method],
    projection: &TypeProjection,
) -> Result<String, Error> {
    let base = base.map_or_else(String::new, |base| format!(": {}", projection.name(base)));
    let attribute = guid.map_or_else(
        || "#[no_guid]".to_string(),
        |guid| format!("#[guid({})]", rdl_uuid(guid)),
    );
    let mut result = format!("    {attribute}\n    interface {name}{base} {{\n");
    let mut start = 0;
    while start < methods.len() {
        let mut end = start + 1;
        while end < methods.len() && methods[end].name == methods[start].name {
            end += 1;
        }
        for method in methods[start..end].iter().rev() {
            let params = write_params(&method.params, projection)?.join(", ");
            let return_type = if method.result == TypeRef::Void {
                String::new()
            } else {
                format!(" -> {}", projection.name(&method.result))
            };
            result.push_str(&format!(
                "        {}fn {}(&self{}{}){return_type};\n",
                if method.special { "#[special] " } else { "" },
                rdl_ident(&method.name),
                if params.is_empty() { "" } else { ", " },
                params
            ));
        }
        start = end;
    }
    result.push_str("    }\n");
    Ok(result)
}

fn rdl_uuid(guid: &str) -> String {
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

fn planned_param_type_name(
    param: &Parameter,
    type_names: &BTreeMap<String, String>,
    interface_names: &BTreeSet<(String, String)>,
    tu: &str,
) -> String {
    if param.annotation.com_out_ptr {
        return "*mut *mut void".to_string();
    }
    if let Some(name) = parameter_string_name(param) {
        return type_names
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string());
    }
    planned_emitted_type_name(&param.ty, type_names, interface_names, tu)
}

fn emitted_pointer_is_mutable(
    param: &Parameter,
    interface_names: &BTreeSet<(String, String)>,
    tu: &str,
) -> bool {
    if param.annotation.com_out_ptr {
        return true;
    }
    if parameter_string_name(param).is_some() {
        return false;
    }
    match &param.ty {
        TypeRef::Pointer { .. } => {
            let (mutable, depth, target) = pointer_run(&param.ty);
            if depth == 1
                && matches!(
                    target,
                    TypeRef::Named { name, .. } | TypeRef::Generic { name, .. }
                        if interface_names.contains(&(tu.to_string(), name.clone()))
                )
            {
                false
            } else {
                mutable
            }
        }
        TypeRef::Reference { mutable, target } => {
            !matches!(
                target.as_ref(),
                TypeRef::Named { name, .. } | TypeRef::Generic { name, .. }
                    if interface_names.contains(&(tu.to_string(), name.clone()))
            ) && *mutable
        }
        TypeRef::FunctionPointer { .. } => true,
        TypeRef::OpaquePointer { mutable, .. } => *mutable,
        TypeRef::Named { name, .. } if matches!(name.as_str(), "PVOID" | "LPVOID") => true,
        _ => false,
    }
}

fn parameter_string_name(param: &Parameter) -> Option<&'static str> {
    match &param.ty {
        TypeRef::Pointer { mutable, target } if param.annotation.null_terminated => {
            match (mutable, target.as_ref()) {
                (false, TypeRef::Scalar(Scalar::I8 | Scalar::U8)) => Some("PCSTR"),
                (true, TypeRef::Scalar(Scalar::I8 | Scalar::U8)) => Some("PSTR"),
                (false, TypeRef::Scalar(Scalar::U16)) => Some("PCWSTR"),
                (true, TypeRef::Scalar(Scalar::U16)) => Some("PWSTR"),
                _ => None,
            }
        }
        TypeRef::Named { name, .. } => {
            if param.annotation.input && !param.annotation.output {
                match name.as_str() {
                    "LPSTR" => return Some("PCSTR"),
                    "LPWSTR" => return Some("PCWSTR"),
                    _ => {}
                }
            }
            canonical_string_name(name)
        }
        _ => None,
    }
}

fn planned_emitted_type_name(
    ty: &TypeRef,
    type_names: &BTreeMap<String, String>,
    interface_names: &BTreeSet<(String, String)>,
    tu: &str,
) -> String {
    if matches!(ty, TypeRef::FunctionPointer { .. }) {
        return "*mut u8".to_string();
    }
    if let TypeRef::OpaquePointer { mutable, .. } = ty {
        return format!("*{} void", if *mutable { "mut" } else { "const" });
    }
    if let TypeRef::Named { name, .. } = ty
        && let Some(name) = type_names.get(name)
    {
        return rdl_ident(name);
    }
    if let TypeRef::Named { name, .. } = ty
        && let Some(name) = canonical_named_type(name)
    {
        return type_names
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string());
    }
    if let TypeRef::Reference { mutable, target } = ty {
        if let TypeRef::Named { name, .. } | TypeRef::Generic { name, .. } = target.as_ref()
            && interface_names.contains(&(tu.to_string(), name.clone()))
        {
            return planned_type_name(target, type_names);
        }
        return format!(
            "*{} {}",
            if *mutable { "mut" } else { "const" },
            planned_emitted_type_name(target, type_names, interface_names, tu)
        );
    }
    if let TypeRef::Array { target, len } = ty {
        return format!(
            "[{}; {len}]",
            planned_emitted_type_name(target, type_names, interface_names, tu)
        );
    }
    let (mutable, depth, target) = pointer_run(ty);
    if depth != 0
        && let TypeRef::Named { name, .. } | TypeRef::Generic { name, .. } = target
        && interface_names.contains(&(tu.to_string(), name.clone()))
    {
        return format!(
            "{}{}",
            format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth - 1),
            planned_emitted_type_name(target, type_names, interface_names, tu)
        );
    }
    if depth != 0 {
        return format!(
            "{}{}",
            format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth),
            planned_emitted_type_name(target, type_names, interface_names, tu)
        );
    }
    planned_type_name(ty, type_names)
}

fn canonical_named_type(name: &str) -> Option<&'static str> {
    if let Some(name) = canonical_string_name(name) {
        return Some(name);
    }
    Some(match name {
        "boolean" | "BYTE" | "UCHAR" | "UINT8" | "uint8_t" => "u8",
        "WORD" | "USHORT" | "WCHAR" | "UINT16" | "uint16_t" => "u16",
        "DWORD" | "UINT" | "ULONG" | "DWORD32" | "UINT32" | "ULONG32" | "uint32_t" => "u32",
        "QWORD" | "ULONGLONG" | "DWORD64" | "UINT64" | "ULONG64" | "uint64_t" => "u64",
        "CHAR" | "INT8" | "int8_t" => "i8",
        "SHORT" | "INT16" | "int16_t" => "i16",
        "INT" | "LONG" | "INT32" | "LONG32" | "int32_t" => "i32",
        "LONGLONG" | "INT64" | "LONG64" | "int64_t" => "i64",
        "FLOAT" => "f32",
        "DOUBLE" => "f64",
        "UINT_PTR" | "ULONG_PTR" | "DWORD_PTR" | "SIZE_T" | "size_t" | "rsize_t" | "uintptr_t" => {
            "usize"
        }
        "INT_PTR" | "LONG_PTR" | "SSIZE_T" | "intptr_t" | "ptrdiff_t" => "isize",
        "LPUNKNOWN" => "IUnknown",
        "PVOID" | "LPVOID" => "*mut void",
        "IID" | "CLSID" | "FMTID" | "UUID" => "GUID",
        "HRESULT" => "HRESULT",
        _ => return None,
    })
}

fn canonical_string_name(name: &str) -> Option<&'static str> {
    match name {
        "PCSTR" | "LPCSTR" => Some("PCSTR"),
        "PSTR" | "LPSTR" => Some("PSTR"),
        "PCWSTR" | "LPCWSTR" => Some("PCWSTR"),
        "PWSTR" | "LPWSTR" => Some("PWSTR"),
        _ => None,
    }
}

fn named_type_shape(name: &str) -> Option<&'static str> {
    match name {
        "NTSTATUS" => Some("i32"),
        _ => canonical_named_type(name),
    }
}

fn calling_convention(convention: CallingConvention) -> &'static str {
    match convention {
        CallingConvention::Platform => "",
        CallingConvention::C => " \"C\"",
    }
}

fn write_named_record(
    name: &str,
    fields: &[Field],
    packing: Option<i64>,
    alignment: Option<i64>,
    union: bool,
    projection: &TypeProjection<'_>,
) -> Result<String, Error> {
    let keyword = if union { "union" } else { "struct" };
    let mut result = String::new();
    if let Some(packing) = packing {
        result.push_str(&format!("    #[packed({packing})]\n"));
    }
    if let Some(alignment) = alignment {
        result.push_str(&format!("    #[align({alignment})]\n"));
    }
    result.push_str(&format!("    {keyword} {name} {{\n"));
    result.push_str(&write_record_fields(fields, projection, 8)?);
    result.push_str("    }\n");
    result.push_str(&write_nested_records(fields, projection)?);
    Ok(result)
}

fn write_nested_records(
    fields: &[Field],
    projection: &TypeProjection<'_>,
) -> Result<String, Error> {
    let mut result = String::new();
    for field in fields {
        result.push_str(&write_nested_type(&field.ty, projection)?);
    }
    Ok(result)
}

fn write_nested_type(ty: &TypeRef, projection: &TypeProjection<'_>) -> Result<String, Error> {
    match ty {
        TypeRef::Array { target, .. }
        | TypeRef::Pointer { target, .. }
        | TypeRef::Reference { target, .. } => write_nested_type(target, projection),
        TypeRef::InlineRecord(record) => {
            if let Some(name) = &record.name {
                write_named_record(
                    &rdl_ident(name),
                    &record.fields,
                    record.packing,
                    record.alignment,
                    record.union,
                    projection,
                )
            } else {
                write_nested_records(&record.fields, projection)
            }
        }
        _ => Ok(String::new()),
    }
}

fn write_record_fields(
    fields: &[Field],
    projection: &TypeProjection<'_>,
    indent: usize,
) -> Result<String, Error> {
    let mut result = String::new();
    let spaces = " ".repeat(indent);
    let bitfield_groups = bitfield_groups(fields)?;
    let mut group_index = 0;
    let mut index = 0;
    while index < fields.len() {
        let field = &fields[index];
        if field.bit_width == Some(0) {
            index += 1;
            continue;
        }
        if let TypeRef::InlineRecord(record) = &field.ty {
            let keyword = if record.union { "union" } else { "struct" };
            result.push_str(&format!("{spaces}{}: ", rdl_ident(&field.name)));
            if let Some(packing) = record.packing {
                result.push_str(&format!("#[packed({packing})] "));
            }
            if let Some(alignment) = record.alignment {
                result.push_str(&format!("#[align({alignment})] "));
            }
            result.push_str(&format!("{keyword} {{\n"));
            result.push_str(&write_record_fields(
                &record.fields,
                projection,
                indent + 4,
            )?);
            result.push_str(&format!("{spaces}}},\n"));
            index += 1;
            continue;
        }
        if field.bit_width.is_none() {
            result.push_str(&format!(
                "{spaces}{}: {},\n",
                rdl_ident(&field.name),
                projection.name(&field.ty)
            ));
            index += 1;
            continue;
        }
        let group = &bitfield_groups[group_index];
        group_index += 1;
        let backing = if bitfield_groups.len() == 1 {
            "_bitfield".to_string()
        } else {
            format!("_bitfield{group_index}")
        };
        result.push_str(&format!(
            "{spaces}{backing}: {} {{\n",
            projection.name(&field.ty)
        ));
        let mut cursor = group.offset;
        for member in &fields[group.start..group.end] {
            if member.offset > cursor {
                result.push_str(&format!(
                    "{}_: {},\n",
                    " ".repeat(indent + 4),
                    member.offset - cursor
                ));
            }
            let width = member.bit_width.unwrap();
            let name = if member.name.is_empty() {
                "_".to_string()
            } else {
                rdl_ident(&member.name)
            };
            result.push_str(&format!("{}{name}: {width},\n", " ".repeat(indent + 4)));
            cursor = member.offset + i64::from(width);
        }
        result.push_str(&format!("{spaces}}},\n"));
        index = group.end;
    }
    Ok(result)
}

fn record_layout(
    fields: &[Field],
    size: i64,
    align: i64,
    union: bool,
) -> Result<(Option<i64>, Option<i64>), String> {
    if size < 0 || align <= 0 {
        return Err(format!(
            "invalid record layout: size {size}, alignment {align}"
        ));
    }
    if let Some(field) = fields
        .iter()
        .find(|field| field.offset < 0 || field.align <= 0 || field.size < 0)
    {
        return Err(format!(
            "invalid layout for field `{}`: offset {}, size {}, alignment {}",
            field.name, field.offset, field.size, field.align
        ));
    }

    let groups = bitfield_groups(fields).map_err(|error| error.0)?;
    let mut best = None;
    for packing in [None, Some(1), Some(2), Some(4), Some(8), Some(16)] {
        let mut cursor = 0;
        let mut natural_align = 1;
        let mut matches = true;
        let mut group_index = 0;
        let mut index = 0;
        while index < fields.len() {
            let field = &fields[index];
            let field_align = packing.map_or(field.align, |packing| packing.min(field.align));
            natural_align = natural_align.max(field_align);
            let mut offset = if union {
                0
            } else {
                align_up(cursor, field_align)
            };
            if offset * 8 != field.offset {
                let explicit_align = [2, 4, 8, 16]
                    .into_iter()
                    .filter(|candidate| *candidate > field_align && *candidate <= align)
                    .find(|candidate| align_up(cursor, *candidate) * 8 == field.offset);
                let Some(explicit_align) = explicit_align else {
                    matches = false;
                    break;
                };
                offset = align_up(cursor, explicit_align);
            }
            let expected = offset * 8;
            if field.bit_width == Some(0) {
                index += 1;
                continue;
            }
            if field.bit_width.is_some() {
                let group = &groups[group_index];
                group_index += 1;
                if fields[group.start..group.end].iter().any(|member| {
                    member.offset < expected
                        || member.offset + i64::from(member.bit_width.unwrap())
                            > expected + field.size * 8
                }) {
                    matches = false;
                    break;
                }
                index = group.end;
            } else {
                index += 1;
            }
            if union {
                cursor = cursor.max(field.size);
            } else {
                cursor = offset + field.size;
            }
        }
        if !matches || align < natural_align {
            continue;
        }
        let alignment = (align > natural_align).then_some(align);
        let content_size = if fields.is_empty() {
            size
        } else if cursor == 0 {
            1
        } else {
            cursor
        };
        if (cursor == 0 && size == 1) || align_up(content_size, align) == size {
            let score = (alignment.is_none(), natural_align);
            if best.is_none_or(|(_, _, best_score)| score > best_score) {
                best = Some((packing, alignment, score));
            }
        }
    }
    best.map(|(packing, alignment, _)| (packing, alignment))
        .ok_or_else(|| "record fields cannot reproduce Clang's layout".to_string())
}

fn align_up(value: i64, align: i64) -> i64 {
    (value + align - 1) / align * align
}

fn scalar_name(scalar: Scalar) -> &'static str {
    match scalar {
        Scalar::Bool => "bool",
        Scalar::F32 => "f32",
        Scalar::F64 => "f64",
        Scalar::I8 => "i8",
        Scalar::U8 => "u8",
        Scalar::I16 => "i16",
        Scalar::U16 => "u16",
        Scalar::I32 => "i32",
        Scalar::U32 => "u32",
        Scalar::I64 => "i64",
        Scalar::U64 => "u64",
    }
}

fn unsigned_scalar(scalar: Scalar) -> Scalar {
    match scalar {
        Scalar::I8 => Scalar::U8,
        Scalar::I16 => Scalar::U16,
        Scalar::I32 => Scalar::U32,
        Scalar::I64 => Scalar::U64,
        _ => scalar,
    }
}

fn enum_value(value: i64, repr: Scalar) -> String {
    match repr {
        Scalar::U8 => (value as u8).to_string(),
        Scalar::U16 => (value as u16).to_string(),
        Scalar::U32 => (value as u32).to_string(),
        Scalar::U64 => (value as u64).to_string(),
        _ => value.to_string(),
    }
}

fn planned_type_name(ty: &TypeRef, type_names: &BTreeMap<String, String>) -> String {
    match ty {
        TypeRef::Void => "void".to_string(),
        TypeRef::String => "String".to_string(),
        TypeRef::Object => "Object".to_string(),
        TypeRef::Scalar(scalar) => scalar_name(*scalar).to_string(),
        TypeRef::Named { name, .. } => rdl_ident(type_names.get(name).unwrap_or(name)),
        TypeRef::Generic { name, args, .. } => format!(
            "{}<{}>",
            rdl_ident(type_names.get(name).unwrap_or(name)),
            args.iter()
                .map(|arg| planned_type_name(arg, type_names))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeRef::Pointer { .. } => {
            let (mutable, depth, target) = pointer_run(ty);
            format!(
                "{}{}",
                format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth),
                planned_type_name(target, type_names)
            )
        }
        TypeRef::Reference { mutable, target } => format!(
            "*{} {}",
            if *mutable { "mut" } else { "const" },
            planned_type_name(target, type_names)
        ),
        TypeRef::FunctionPointer { .. } => "*mut u8".to_string(),
        TypeRef::OpaquePointer { mutable, .. } => {
            format!("*{} void", if *mutable { "mut" } else { "const" })
        }
        TypeRef::Array { target, len } => {
            format!("[{}; {len}]", planned_type_name(target, type_names))
        }
        TypeRef::InlineRecord(record) => record
            .name
            .clone()
            .unwrap_or_else(|| "<inline record>".to_string()),
    }
}

fn pointer_run(mut ty: &TypeRef) -> (bool, usize, &TypeRef) {
    let mut mutable = true;
    let mut depth = 0;
    while let TypeRef::Pointer {
        mutable: level_mutable,
        target,
    } = ty
    {
        mutable &= *level_mutable;
        depth += 1;
        ty = target;
    }
    (mutable, depth, ty)
}

fn constant_type_name(ty: &TypeRef, type_names: &BTreeMap<String, String>) -> String {
    match ty {
        TypeRef::Scalar(Scalar::Bool) => "u32".to_string(),
        TypeRef::Named { name, .. } if type_names.contains_key(name) => {
            planned_type_name(ty, type_names)
        }
        TypeRef::Named { name, .. } => canonical_named_type(name)
            .map_or_else(|| planned_type_name(ty, type_names), str::to_string),
        _ => planned_type_name(ty, type_names),
    }
}

fn value_name(value: &Value) -> String {
    match value {
        Value::F32(value) => float_name(f32::from_bits(*value) as f64),
        Value::F64(value) => float_name(f64::from_bits(*value)),
        Value::Signed(value) => value.to_string(),
        Value::Unsigned(value) => value.to_string(),
        Value::Utf8(value) | Value::Utf16(value) => format!("{value:?}"),
    }
}

fn float_name(value: f64) -> String {
    let value = value.to_string();
    if value.contains(['.', 'e', 'E']) {
        value
    } else {
        format!("{value}.0")
    }
}

fn rdl_ident(name: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do",
        "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in",
        "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
        "return", "static", "struct", "trait", "true", "try", "type", "typeof", "union", "unsafe",
        "unsized", "use", "virtual", "where", "while", "yield",
    ];
    if name == "_" {
        "__".to_string()
    } else if ["crate", "self", "Self", "super"].contains(&name) {
        format!("{name}_")
    } else if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_string()
    }
}

fn normalize_name(name: &str) -> String {
    name.replace('\\', "/")
}

fn origin(origin: &Origin) -> String {
    format!("{}#{}", origin.tu, origin.local)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_shape_cache_is_context_specific() {
        let declaration = Location {
            file: "recursive.hpp".to_string(),
            offset: 7,
        };
        let first_context = TypeShape(1, 2);
        let second_context = TypeShape(3, 4);
        let first_shape = TypeShape(5, 6);
        let second_shape = TypeShape(7, 8);
        let mut cache = ShapeCache::default();

        cache.insert_recursive("tu", &declaration, first_context, first_shape);
        cache.insert_recursive("tu", &declaration, second_context, second_shape);

        assert!(cache.is_recursive("tu", &declaration));
        assert_eq!(
            cache.recursive("tu", &declaration, first_context),
            Some(first_shape)
        );
        assert_eq!(
            cache.recursive("tu", &declaration, second_context),
            Some(second_shape)
        );
        assert_eq!(
            cache.recursive("other-tu", &declaration, first_context),
            None
        );
    }

    #[test]
    fn canonical_string_aliases_share_one_mapping() {
        for (alias, canonical) in [
            ("PCSTR", "PCSTR"),
            ("LPCSTR", "PCSTR"),
            ("PSTR", "PSTR"),
            ("LPSTR", "PSTR"),
            ("PCWSTR", "PCWSTR"),
            ("LPCWSTR", "PCWSTR"),
            ("PWSTR", "PWSTR"),
            ("LPWSTR", "PWSTR"),
        ] {
            assert_eq!(canonical_string_name(alias), Some(canonical));
            assert_eq!(canonical_named_type(alias), Some(canonical));
        }
    }

    #[test]
    fn canonical_floating_aliases_use_rdl_primitives() {
        assert_eq!(canonical_named_type("FLOAT"), Some("f32"));
        assert_eq!(canonical_named_type("DOUBLE"), Some("f64"));
    }
}
