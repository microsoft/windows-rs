#![allow(non_upper_case_globals)]

use clang_sys::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub source: String,
    pub roots: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeReference {
    pub namespace: String,
    pub name: String,
    pub kind: TypeReferenceKind,
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
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeReferenceKind {
    Interface,
    Type,
}

pub struct EmitOptions<'a> {
    pub namespace: &'a str,
    pub library: Option<&'a str>,
    pub libraries: Option<&'a BTreeMap<String, String>>,
    pub references: &'a BTreeMap<String, TypeReference>,
    pub functions: Option<&'a BTreeSet<String>>,
}

impl<'a> EmitOptions<'a> {
    pub fn new(namespace: &'a str, references: &'a BTreeMap<String, TypeReference>) -> Self {
        Self {
            namespace,
            library: None,
            libraries: None,
            references,
            functions: None,
        }
    }
}

impl Input {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        let name = normalize_name(&name.into());
        Self {
            roots: BTreeSet::from([name.clone()]),
            name,
            source: source.into(),
        }
    }

    pub fn with_roots(mut self, roots: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.roots
            .extend(roots.into_iter().map(|root| normalize_name(&root.into())));
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Origin {
    pub tu: String,
    pub local: u32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
        params: Vec<TypeRef>,
        result: TypeRef,
    },
    Class {
        guid: String,
    },
    Enum {
        repr: Scalar,
        variants: Vec<Variant>,
    },
    EnumFlag {
        target: String,
    },
    Macro {
        function_like: bool,
    },
    Function {
        link_name: String,
        convention: CallingConvention,
        params: Vec<Parameter>,
        result: TypeRef,
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
    Signed(i64),
    Unsigned(u64),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Constant {
    pub root: Origin,
    pub definition: Origin,
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
                "{} {:?} {} [{}:{} -> {}:{}]{}{}{}{}\n",
                origin(&fact.origin),
                fact.kind,
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
        let timing = std::env::var_os("WINDOWS_CLANG2").is_some();
        let plan_time = std::time::Instant::now();
        let plan = self.plan(options.references, options.functions)?;
        if timing {
            eprintln!("clang2 planning: {:.2}s", plan_time.elapsed().as_secs_f32());
        }
        let emission_time = std::time::Instant::now();
        let mut items = BTreeMap::new();
        for planned in plan.types {
            let fact = planned.fact;
            let item = match &fact.data {
                FactData::Callback {
                    convention,
                    params,
                    result,
                } => write_callable(
                    &planned.name,
                    *convention,
                    params,
                    result,
                    &plan.type_names,
                    &plan.interface_names,
                    &fact.origin.tu,
                ),
                FactData::Class { guid } => {
                    format!(
                        "    const {}: GUID = {};\n",
                        rdl_ident(&planned.name),
                        rdl_uuid(guid)
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
                FactData::Enum { repr, variants } if fact.definition => {
                    let flags = plan
                        .flag_enums
                        .contains(&(fact.origin.tu.clone(), planned.name.clone()));
                    let repr = if flags { unsigned_scalar(*repr) } else { *repr };
                    let mut item = format!(
                        "    #[repr({})]\n{}    enum {} {{\n",
                        scalar_name(repr),
                        if flags { "    #[flags]\n" } else { "" },
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
                } => write_interface(
                    &rdl_ident(&planned.name),
                    base.as_ref(),
                    guid.as_deref(),
                    methods,
                    &plan.type_names,
                    &plan.interface_names,
                    &fact.origin.tu,
                )?,
                _ => {
                    return Err(Error(format!(
                        "planned type `{}` is not emittable",
                        fact.name
                    )));
                }
            };
            if items.insert(planned.name.clone(), item).is_some() {
                return Err(Error(format!("duplicate planned name `{}`", planned.name)));
            }
        }
        for function in plan.functions {
            let FactData::Function {
                link_name,
                convention,
                params,
                result,
            } = &function.data
            else {
                return Err(Error(format!(
                    "planned function `{}` has no signature",
                    function.name
                )));
            };
            let params = params
                .iter()
                .map(|param| -> Result<_, Error> {
                    let ty = planned_param_type_name(
                        param,
                        &plan.type_names,
                        &plan.interface_names,
                        &function.origin.tu,
                    );
                    Ok(format!(
                        "{}{}: {}",
                        param_attributes(param, params, ty.starts_with("*mut "))?,
                        rdl_ident(&param.name),
                        ty
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
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
            let item = format!(
                "    #[library({library:?})]\n    extern{abi} fn {}({params}){result};\n",
                rdl_ident(&function.name),
            );
            if items.insert(function.name.clone(), item).is_some() {
                return Err(Error(format!("duplicate planned name `{}`", function.name)));
            }
        }
        for constant in plan.constants {
            let item = format!(
                "    const {}: {} = {};\n",
                rdl_ident(&constant.name),
                constant_type_name(&constant.ty, &plan.type_names),
                value_name(&constant.value)
            );
            if items.insert(constant.name.clone(), item).is_some() {
                return Err(Error(format!("duplicate planned name `{}`", constant.name)));
            }
        }

        let namespaces: Vec<_> = options
            .namespace
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
        for item in items.values() {
            for line in item.lines() {
                result.push_str(&indent);
                result.push_str(line);
                result.push('\n');
            }
        }
        for depth in (0..namespaces.len()).rev() {
            result.push_str(&format!("{}}}\n", "    ".repeat(depth)));
        }
        if timing {
            eprintln!(
                "clang2 emission: {:.2}s",
                emission_time.elapsed().as_secs_f32()
            );
        }
        Ok(result)
    }

    fn plan(
        &self,
        references: &BTreeMap<String, TypeReference>,
        selected_functions: Option<&BTreeSet<String>>,
    ) -> Result<Plan<'_>, Error> {
        let timing = std::env::var_os("WINDOWS_CLANG2").is_some();
        let mut phase_time = std::time::Instant::now();
        #[derive(Default)]
        struct Roots<'a> {
            types: Vec<&'a Fact>,
            functions: Vec<&'a Fact>,
            values: Vec<&'a Constant>,
        }

        let mut roots: BTreeMap<&str, Roots<'_>> = BTreeMap::new();
        for fact in self
            .facts
            .iter()
            .filter(|fact| fact.root && is_root_type_fact(fact))
        {
            roots.entry(&fact.name).or_default().types.push(fact);
        }
        for fact in self
            .facts
            .iter()
            .filter(|fact| fact.root && matches!(fact.data, FactData::Function { .. }))
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
            roots
                .entry(&constant.name)
                .or_default()
                .values
                .push(constant);
        }

        let facts_by_origin: HashMap<_, _> =
            self.facts.iter().map(|fact| (&fact.origin, fact)).collect();
        let mut facts_index: HashMap<&str, Vec<&Fact>> = HashMap::new();
        for fact in &self.facts {
            facts_index.entry(&fact.name).or_default().push(fact);
        }
        let mut type_roots = vec![];
        let mut functions = vec![];
        let mut constants = vec![];
        let mut root_names = BTreeSet::new();

        for (name, roots) in roots {
            if !roots.types.is_empty() && !roots.functions.is_empty() {
                return Err(Error(format!(
                    "type and function roots collide on `{name}`"
                )));
            }
            if !roots.types.is_empty() {
                if references.contains_key(name)
                    && !roots
                        .types
                        .iter()
                        .any(|fact| defines_local_type(name, fact))
                {
                    continue;
                }
                let root = choose_type_root(name, &roots.types, &facts_index)?;
                root_names.insert(name.to_string());
                type_roots.push(root);
            } else if !roots.functions.is_empty() {
                functions.push(choose_function_root(name, &roots.functions)?);
            } else {
                let constant = choose_constant_root(name, &roots.values)?;
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
                "clang2 plan roots: {:.2}s",
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
                        let fact = choose_type_root(name, &matches, &facts_index)?;
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
                if canonical_named_type(name).is_some() {
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
                        .map(|choices| (constant.name.as_str(), choices))
                })
                .collect();
            if !collisions.is_empty() {
                for (name, choices) in collisions {
                    let root = choose_type_root(name, choices, &facts_index)?;
                    if root_names.insert(name.to_string()) {
                        type_roots.push(root);
                    }
                }
                constants.retain(|constant| !root_names.contains(constant.name.as_str()));
                continue;
            }

            let mut facts_by_name = BTreeMap::new();
            for (name, choices) in grouped {
                facts_by_name.insert(name, choose_type_root(name, &choices, &facts_index)?);
            }
            break facts_by_name;
        };
        if timing {
            eprintln!(
                "clang2 plan closure: {:.2}s",
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
                "clang2 plan required: {:.2}s",
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
                            && matches!(
                                target.data,
                                FactData::Enum { .. }
                                    | FactData::Record { .. }
                                    | FactData::Interface { .. }
                            )
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
        if timing {
            eprintln!(
                "clang2 plan aliases: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
            phase_time = std::time::Instant::now();
        }
        for (name, reference) in references {
            if !local_roots.contains(name) {
                type_names.entry(name.clone()).or_insert_with(|| {
                    format!(
                        "{}::{}",
                        reference.namespace.replace('.', "::"),
                        reference.name
                    )
                });
            }
        }
        let alias_names: BTreeSet<_> = type_names.values().map(String::as_str).collect();
        let types: Vec<_> = facts_by_name
            .into_iter()
            .filter(|(name, _)| required.contains(*name))
            .filter(|(name, _)| !alias_names.contains(*name))
            .map(|(_, fact)| PlannedType {
                name: type_names
                    .get(fact.name.as_str())
                    .cloned()
                    .unwrap_or_else(|| fact.name.clone()),
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
            if reference.kind == TypeReferenceKind::Interface && !local_roots.contains(name) {
                interface_names.extend(
                    translation_units
                        .iter()
                        .map(|tu| ((*tu).to_string(), name.clone())),
                );
            }
        }
        let mut interface_aliases: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        for fact in &self.facts {
            if let FactData::Typedef {
                target: TypeRef::Named { name, declaration },
            } = &fact.data
                && facts_index
                    .get(name.as_str())
                    .into_iter()
                    .flatten()
                    .any(|target| {
                        target.origin.tu == fact.origin.tu && target.spelling == *declaration
                    })
            {
                interface_aliases
                    .entry((fact.origin.tu.clone(), name.clone()))
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
        if timing {
            eprintln!(
                "clang2 plan interfaces: {:.2}s",
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
        let mut output_names = BTreeSet::new();
        for planned in &types {
            if !output_names.insert(planned.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", planned.name)));
            }
        }
        for constant in &constants {
            if !output_names.insert(constant.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", constant.name)));
            }
        }
        for function in &functions {
            if !output_names.insert(function.name.as_str()) {
                return Err(Error(format!("duplicate planned name `{}`", function.name)));
            }
        }
        constants.sort_by(|left, right| left.name.cmp(&right.name));
        functions.sort_by(|left, right| left.name.cmp(&right.name));
        if timing {
            eprintln!(
                "clang2 plan finalize: {:.2}s",
                phase_time.elapsed().as_secs_f32()
            );
        }
        Ok(Plan {
            types,
            functions,
            constants,
            type_names,
            interface_names,
            flag_enums,
        })
    }
}

struct PlannedType<'a> {
    fact: &'a Fact,
    name: String,
}

struct Plan<'a> {
    types: Vec<PlannedType<'a>>,
    functions: Vec<&'a Fact>,
    constants: Vec<&'a Constant>,
    type_names: BTreeMap<String, String>,
    interface_names: BTreeSet<(String, String)>,
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
            | FactData::Class { .. }
            | FactData::Enum { .. }
            | FactData::Interface { .. }
            | FactData::Record { .. }
            | FactData::Typedef { .. }
    )
}

fn defines_local_type(name: &str, fact: &Fact) -> bool {
    match &fact.data {
        FactData::Class { .. } | FactData::Callback { .. } => true,
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

fn is_root_type_fact(fact: &Fact) -> bool {
    is_type_fact(fact)
        && canonical_named_type(&fact.name).is_none()
        && (!matches!(
            fact.data,
            FactData::Record { .. } | FactData::Interface { .. }
        ) || fact.definition)
}

fn choose_type_root<'a>(
    name: &str,
    roots: &[&'a Fact],
    facts_index: &HashMap<&str, Vec<&'a Fact>>,
) -> Result<&'a Fact, Error> {
    let mut distinct: Vec<&Fact> = vec![];
    for &root in roots {
        if !distinct
            .iter()
            .any(|existing| same_source_declaration(existing, root))
        {
            distinct.push(root);
        }
    }
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
        return emittable_type(
            name,
            distinct
                .iter()
                .min_by_key(|fact| &fact.spelling)
                .copied()
                .unwrap(),
        );
    }
    if let Some(class) = distinct
        .iter()
        .copied()
        .find(|fact| matches!(fact.data, FactData::Class { .. }))
        && distinct.iter().all(|fact| {
            fact.origin == class.origin
                || (fact.origin.tu == class.origin.tu
                    && fact.parent == class.parent
                    && matches!(
                        &fact.data,
                        FactData::Typedef {
                            target: TypeRef::Named { name: target, .. }
                        } if target == name
                    ))
        })
    {
        return emittable_type(name, class);
    }
    if let Some(first) = distinct.first()
        && let FactData::Typedef {
            target: first_target,
        } = &first.data
        && distinct.iter().all(|fact| {
            fact.origin.tu == first.origin.tu
                && fact.parent == first.parent
                && matches!(
                    &fact.data,
                    FactData::Typedef { target }
                        if resolved_type_shape(first_target, &first.origin.tu, facts_index)
                            == resolved_type_shape(target, &fact.origin.tu, facts_index)
                )
        })
    {
        return emittable_type(
            name,
            distinct
                .iter()
                .min_by_key(|fact| &fact.spelling)
                .copied()
                .unwrap(),
        );
    }

    fn resolved_type_shape(
        ty: &TypeRef,
        tu: &str,
        facts_index: &HashMap<&str, Vec<&Fact>>,
    ) -> String {
        fn write(
            ty: &TypeRef,
            tu: &str,
            facts_index: &HashMap<&str, Vec<&Fact>>,
            seen: &mut BTreeSet<Location>,
        ) -> String {
            match ty {
                TypeRef::Named { name, declaration } => {
                    if seen.insert(declaration.clone())
                        && let Some(target) = facts_index
                            .get(name.as_str())
                            .into_iter()
                            .flatten()
                            .find(|fact| {
                                fact.origin.tu == tu
                                    && fact.spelling == *declaration
                                    && matches!(fact.data, FactData::Typedef { .. })
                            })
                            .and_then(|fact| match &fact.data {
                                FactData::Typedef { target } => Some(target),
                                _ => None,
                            })
                    {
                        let result = write(target, tu, facts_index, seen);
                        seen.remove(declaration);
                        return result;
                    }
                    format!("named:{}", named_type_shape(name).unwrap_or(name))
                }
                TypeRef::Pointer { mutable, target } => {
                    format!("pointer:{mutable}:{}", write(target, tu, facts_index, seen))
                }
                TypeRef::Reference { mutable, target } => {
                    format!(
                        "reference:{mutable}:{}",
                        write(target, tu, facts_index, seen)
                    )
                }
                TypeRef::FunctionPointer {
                    convention,
                    params,
                    result,
                } => format!(
                    "function:{convention:?}:({}):{}",
                    params
                        .iter()
                        .map(|param| write(param, tu, facts_index, seen))
                        .collect::<Vec<_>>()
                        .join(","),
                    write(result, tu, facts_index, seen)
                ),
                TypeRef::OpaquePointer { mutable, tag } => format!("opaque:{mutable}:{tag}"),
                TypeRef::Array { target, len } => {
                    format!("array:{len}:{}", write(target, tu, facts_index, seen))
                }
                TypeRef::InlineRecord(record) => format!("record:{record:?}"),
                other => format!("{other:?}"),
            }
        }

        write(ty, tu, facts_index, &mut BTreeSet::new())
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
        let root = definitions
            .iter()
            .min_by_key(|fact| &fact.spelling)
            .copied()
            .unwrap();
        let equivalent_definitions = definitions.iter().all(|fact| {
            fact.origin.tu == root.origin.tu && fact.kind == root.kind && fact.data == root.data
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
        let same_tu_declarations = distinct.iter().all(|fact| {
            fact.origin == root.origin
                || (!fact.definition
                    && fact.origin.tu == root.origin.tu
                    && matches!(
                        (&fact.data, &root.data),
                        (FactData::Enum { .. }, FactData::Enum { .. })
                            | (FactData::Record { .. }, FactData::Record { .. })
                            | (FactData::Interface { .. }, FactData::Interface { .. })
                    )
                    && (fact.parent == root.parent
                        || (root.parent.is_some()
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
                            }))))
        });
        if same_tu_declarations {
            return Ok(root);
        }
    }
    Err(Error(format!("ambiguous type root `{name}`")))
}

fn choose_constant_root<'a>(name: &str, roots: &[&'a Constant]) -> Result<&'a Constant, Error> {
    let Some(first) = roots.first() else {
        return Err(Error(format!("missing constant root `{name}`")));
    };
    if roots
        .iter()
        .all(|constant| constant.ty == first.ty && constant.value == first.value)
    {
        Ok(first)
    } else {
        Err(Error(format!("ambiguous constant root `{name}`")))
    }
}

fn choose_function_root<'a>(name: &str, roots: &[&'a Fact]) -> Result<&'a Fact, Error> {
    let mut distinct: Vec<&Fact> = vec![];
    for &root in roots {
        if !distinct
            .iter()
            .any(|existing| same_source_declaration(existing, root))
        {
            distinct.push(root);
        }
    }
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
    Err(Error(format!("ambiguous function root `{name}`")))
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
        FactData::Callback { .. } | FactData::Typedef { .. } | FactData::Enum { .. }
            if fact.definition =>
        {
            Ok(fact)
        }
        FactData::Class { .. } | FactData::Record { .. } | FactData::Interface { .. } => Ok(fact),
        _ => Err(Error(format!("type root `{name}` is not emittable"))),
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
            queue.push((fact.origin.tu.as_str(), TypeEdge::Type(param)));
        }
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
                if let Some(name) = projected_string_name(param) {
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
            if let Some(name) = projected_string_name(param) {
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

pub fn extract(inputs: impl IntoIterator<Item = Input>, args: &[&str]) -> Result<Snapshot, Error> {
    let inputs: Vec<_> = inputs.into_iter().collect();
    let mut names = HashSet::new();
    for input in &inputs {
        if !names.insert(input.name.as_str()) {
            return Err(Error(format!("duplicate input name `{}`", input.name)));
        }
    }

    let _library = Library::new()?;
    let index = Index::new()?;
    let timing = std::env::var_os("WINDOWS_CLANG2").is_some();
    let parse_time = std::time::Instant::now();
    let mut translation_units = Vec::with_capacity(inputs.len());
    for input in &inputs {
        translation_units.push((
            input.name.clone(),
            TranslationUnit::parse(&index, input, args)?,
        ));
    }
    if timing {
        eprintln!("clang2 parse: {:.2}s", parse_time.elapsed().as_secs_f32());
    }

    let traversal_time = std::time::Instant::now();
    let mut facts = vec![];
    let mut constants = vec![];
    for (name, translation_unit) in &translation_units {
        let input = inputs.iter().find(|input| input.name == *name).unwrap();
        translation_unit.extract(input, &mut facts, &mut constants);
    }
    if timing {
        eprintln!(
            "clang2 traversal: {:.2}s",
            traversal_time.elapsed().as_secs_f32()
        );
    }
    facts.sort();
    for pair in facts.windows(2) {
        if pair[0].origin == pair[1].origin {
            return Err(Error(format!(
                "duplicate fact origin `{}`",
                origin(&pair[0].origin)
            )));
        }
    }
    let constant_time = std::time::Instant::now();
    for input in &inputs {
        constants.extend(evaluate_constants(&index, input, args, &facts)?);
    }
    if timing {
        eprintln!(
            "clang2 constants: {:.2}s",
            constant_time.elapsed().as_secs_f32()
        );
    }
    constants.sort();
    Ok(Snapshot { facts, constants })
}

struct Library;

impl Library {
    fn new() -> Result<Self, Error> {
        load().map_err(|error| Error(format!("failed to load libclang: {error}")))?;
        Ok(Self)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        _ = unload();
    }
}

struct Index(CXIndex);

impl Index {
    fn new() -> Result<Self, Error> {
        let value = unsafe { clang_createIndex(0, 0) };
        if value.is_null() {
            Err(Error("failed to create libclang index".into()))
        } else {
            Ok(Self(value))
        }
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { clang_disposeIndex(self.0) };
    }
}

struct TranslationUnit(CXTranslationUnit);

struct Evaluated {
    name: String,
    ty: TypeRef,
    value: Value,
}

impl TranslationUnit {
    fn parse(index: &Index, input: &Input, args: &[&str]) -> Result<Self, Error> {
        let name = CString::new(input.name.as_str())
            .map_err(|_| Error(format!("invalid input name `{}`", input.name)))?;
        let source = CString::new(input.source.as_str())
            .map_err(|_| Error(format!("input `{}` contains a null byte", input.name)))?;
        let args: Result<Vec<_>, _> = args
            .iter()
            .map(|arg| CString::new(*arg).map_err(|_| Error(format!("invalid argument `{arg}`"))))
            .collect();
        let args = args?;
        let arg_pointers: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
        let mut unsaved = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: source.as_ptr(),
            Length: input.source.len().try_into().unwrap(),
        };
        let value = unsafe {
            clang_parseTranslationUnit(
                index.0,
                name.as_ptr(),
                arg_pointers.as_ptr(),
                arg_pointers.len().try_into().unwrap(),
                &mut unsaved,
                1,
                CXTranslationUnit_DetailedPreprocessingRecord
                    | CXTranslationUnit_SkipFunctionBodies,
            )
        };
        if value.is_null() {
            return Err(Error(format!("failed to parse `{}`", input.name)));
        }

        let result = Self(value);
        let errors = result.errors();
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(Error(errors.join("\n")))
        }
    }

    fn parse_probe(
        index: &Index,
        input: &Input,
        probe: &str,
        args: &[&str],
    ) -> Result<Self, Error> {
        let synthetic_name = format!("{}.__clang2_eval.cpp", input.name);
        let synthetic_source = format!("{}\n{probe}", input.source);
        let name = CString::new(synthetic_name.as_str()).unwrap();
        let source = CString::new(synthetic_source).unwrap();
        let mut unsaved = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: source.as_ptr(),
            Length: source.as_bytes().len().try_into().unwrap(),
        };
        let mut args: Vec<_> = args.iter().map(|arg| CString::new(*arg).unwrap()).collect();
        args.push(CString::new("-ferror-limit=0").unwrap());
        let arg_pointers: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
        let value = unsafe {
            clang_parseTranslationUnit(
                index.0,
                name.as_ptr(),
                arg_pointers.as_ptr(),
                arg_pointers.len().try_into().unwrap(),
                &mut unsaved,
                1,
                CXTranslationUnit_KeepGoing | CXTranslationUnit_SkipFunctionBodies,
            )
        };
        if value.is_null() {
            Err(Error(format!(
                "failed to evaluate macro in `{}`",
                input.name
            )))
        } else {
            Ok(Self(value))
        }
    }

    fn errors(&self) -> Vec<String> {
        let mut result = vec![];
        let count = unsafe { clang_getNumDiagnostics(self.0) };
        for index in 0..count {
            let diagnostic = unsafe { clang_getDiagnostic(self.0, index) };
            let severity = unsafe { clang_getDiagnosticSeverity(diagnostic) };
            if severity >= CXDiagnostic_Error {
                result.push(cx_string(unsafe {
                    clang_formatDiagnostic(diagnostic, clang_defaultDiagnosticDisplayOptions())
                }));
            }
            unsafe { clang_disposeDiagnostic(diagnostic) };
        }
        result
    }

    fn extract(&self, input: &Input, facts: &mut Vec<Fact>, constants: &mut Vec<Constant>) {
        let macros = macro_definitions(unsafe { clang_getTranslationUnitCursor(self.0) });
        let mut traversal = Traversal {
            tu: &input.name,
            roots: &input.roots,
            next: 0,
            seen: HashMap::new(),
            macros: &macros,
            facts,
            constants,
        };
        extract_children(
            unsafe { clang_getTranslationUnitCursor(self.0) },
            None,
            &mut traversal,
        );
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe { clang_disposeTranslationUnit(self.0) };
    }
}

struct Traversal<'a> {
    tu: &'a str,
    roots: &'a BTreeSet<String>,
    next: u32,
    seen: HashMap<u32, Vec<(CXCursor, Origin)>>,
    macros: &'a HashMap<String, Vec<String>>,
    facts: &'a mut Vec<Fact>,
    constants: &'a mut Vec<Constant>,
}

fn extract_children(cursor: CXCursor, parent: Option<Origin>, traversal: &mut Traversal<'_>) {
    for child in cursor_children(cursor) {
        let local = traversal.next;
        traversal.next += 1;
        let kind = unsafe { clang_getCursorKind(child) };
        let mut child_parent = parent.clone();
        let mut repeated = false;

        let name = cx_string(unsafe { clang_getCursorSpelling(child) });
        let anonymous_enum =
            kind == CXCursor_EnumDecl && unsafe { clang_Cursor_isAnonymous(child) } != 0;
        if anonymous_enum
            && let Some((spelling, _, _, _)) = cursor_locations(child)
            && traversal.roots.contains(&spelling.file)
        {
            let ty = unsafe { clang_getEnumDeclIntegerType(child) };
            if let Some(repr) = scalar(ty) {
                let origin = Origin {
                    tu: traversal.tu.to_string(),
                    local,
                };
                for constant in cursor_children(child).into_iter().filter(|cursor| unsafe {
                    clang_getCursorKind(*cursor) == CXCursor_EnumConstantDecl
                }) {
                    let value =
                        if matches!(repr, Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::U64) {
                            Value::Unsigned(unsafe {
                                clang_getEnumConstantDeclUnsignedValue(constant)
                            })
                        } else {
                            Value::Signed(unsafe { clang_getEnumConstantDeclValue(constant) })
                        };
                    traversal.constants.push(Constant {
                        root: origin.clone(),
                        definition: origin.clone(),
                        name: cx_string(unsafe { clang_getCursorSpelling(constant) }),
                        ty: TypeRef::Scalar(repr),
                        value,
                    });
                }
            }
        }
        let fact_kind = if kind == CXCursor_MacroExpansion && name == "DEFINE_ENUM_FLAG_OPERATORS" {
            Some(FactKind::EnumFlag)
        } else if anonymous_enum {
            None
        } else {
            fact_kind(kind)
        };
        if let Some(fact_kind) = fact_kind {
            let anonymous_record = matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
                && unsafe { clang_Cursor_isAnonymous(child) } != 0;
            if !name.is_empty() && !anonymous_record {
                let hash = unsafe { clang_hashCursor(child) };
                let seen = traversal.seen.entry(hash).or_default();
                if let Some((_, origin)) = seen
                    .iter()
                    .find(|(cursor, _)| unsafe { clang_equalCursors(*cursor, child) } != 0)
                {
                    child_parent = Some(origin.clone());
                    repeated = true;
                } else if let Some((spelling, expansion, _, system)) = cursor_locations(child) {
                    let main_file = spelling.file == traversal.tu;
                    let root = traversal.roots.contains(&spelling.file);
                    let origin = Origin {
                        tu: traversal.tu.to_string(),
                        local,
                    };
                    seen.push((child, origin.clone()));
                    child_parent = Some(origin.clone());
                    traversal.facts.push(Fact {
                        origin,
                        parent: parent.clone(),
                        kind: fact_kind,
                        name,
                        spelling,
                        expansion,
                        definition: unsafe { clang_isCursorDefinition(child) } != 0,
                        main_file,
                        root,
                        system,
                        data: fact_data(child, fact_kind, traversal.macros),
                    });
                }
            }
        }

        if !repeated {
            extract_children(child, child_parent, traversal);
        }
    }
}

fn cursor_children(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(
        cursor: CXCursor,
        _parent: CXCursor,
        data: CXClientData,
    ) -> CXChildVisitResult {
        let children = unsafe { &mut *(data as *mut Vec<CXCursor>) };
        children.push(cursor);
        CXChildVisit_Continue
    }

    let mut children = vec![];
    unsafe {
        clang_visitChildren(cursor, visit, &mut children as *mut _ as CXClientData);
    }
    children
}

fn macro_definitions(cursor: CXCursor) -> HashMap<String, Vec<String>> {
    fn collect(cursor: CXCursor, result: &mut HashMap<String, Vec<String>>) {
        for child in cursor_children(cursor) {
            if unsafe { clang_getCursorKind(child) } == CXCursor_MacroDefinition {
                let name = cx_string(unsafe { clang_getCursorSpelling(child) });
                let tokens = cursor_tokens(child)
                    .into_iter()
                    .map(|(_, token)| token)
                    .skip(1)
                    .collect();
                result.insert(name, tokens);
            }
            collect(child, result);
        }
    }

    let mut result = HashMap::new();
    collect(cursor, &mut result);
    result
}

fn evaluate_constants(
    _index: &Index,
    input: &Input,
    args: &[&str],
    facts: &[Fact],
) -> Result<Vec<Constant>, Error> {
    let mut roots = BTreeMap::new();
    for fact in facts
        .iter()
        .filter(|fact| fact.origin.tu == input.name && fact.root)
    {
        if let FactData::Macro {
            function_like: false,
        } = fact.data
        {
            roots
                .entry(fact.name.clone())
                .or_insert_with(|| fact.origin.clone());
        }
    }
    let mut candidates = BTreeMap::new();
    for fact in facts.iter().filter(|fact| fact.origin.tu == input.name) {
        if let Some(root) = roots.get(&fact.name) {
            match fact.data {
                FactData::Macro {
                    function_like: false,
                } => {
                    candidates.insert(fact.name.clone(), (root.clone(), fact.origin.clone()));
                }
                FactData::Macro {
                    function_like: true,
                } => {
                    candidates.remove(&fact.name);
                }
                _ => {}
            }
        }
    }

    let names: Vec<_> = candidates.keys().cloned().collect();
    if names.is_empty() {
        return Ok(vec![]);
    }
    let probe_time = std::time::Instant::now();
    let mut evaluated = vec![];
    let mut reached = HashSet::new();
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(4);
    let batches: Vec<_> = names.chunks(4096).collect();
    let worker_count = workers.min(batches.len());
    let batch_results = std::thread::scope(|scope| {
        (0..worker_count)
            .map(|worker| {
                let batches = &batches;
                scope.spawn(move || {
                    let _library = Library::new()?;
                    let index = Index::new()?;
                    let mut evaluated = vec![];
                    let mut reached = HashSet::new();
                    for batch_index in (worker..batches.len()).step_by(worker_count) {
                        let batch = batches[batch_index];
                        let (batch_evaluated, batch_reached) =
                            evaluate_probe(&index, input, args, batch)?;
                        evaluated.extend(batch_evaluated);
                        reached.extend(batch_reached);
                    }
                    Ok((evaluated, reached))
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| {
                thread
                    .join()
                    .map_err(|_| Error("macro probe worker panicked".to_string()))?
            })
            .collect::<Result<Vec<_>, Error>>()
    })?;
    for (batch_evaluated, batch_reached) in batch_results {
        evaluated.extend(batch_evaluated);
        reached.extend(batch_reached);
    }
    let missing: Vec<_> = names
        .iter()
        .filter(|name| !reached.contains(name.as_str()))
        .cloned()
        .collect();
    let recovery_batches: Vec<_> = missing.chunks(128).collect();
    let recovery_batch_count = recovery_batches.len();
    let recovery_worker_count = workers.min(recovery_batches.len());
    let recovery_results = std::thread::scope(|scope| {
        (0..recovery_worker_count)
            .map(|worker| {
                let recovery_batches = &recovery_batches;
                scope.spawn(move || {
                    let _library = Library::new()?;
                    let index = Index::new()?;
                    let mut evaluated = vec![];
                    let mut reached = HashSet::new();
                    for batch in recovery_batches
                        .iter()
                        .skip(worker)
                        .step_by(recovery_worker_count)
                    {
                        let (batch_evaluated, batch_reached) =
                            evaluate_probe(&index, input, args, batch)?;
                        evaluated.extend(batch_evaluated);
                        reached.extend(batch_reached);
                    }
                    Ok((evaluated, reached))
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| {
                thread
                    .join()
                    .map_err(|_| Error("macro probe worker panicked".to_string()))?
            })
            .collect::<Result<Vec<_>, Error>>()
    })?;
    for (recovery_evaluated, recovery_reached) in recovery_results {
        evaluated.extend(recovery_evaluated);
        reached.extend(recovery_reached);
    }
    let fallback: Vec<_> = missing
        .into_iter()
        .filter(|name| !reached.contains(name.as_str()))
        .collect();
    let fallback_count = fallback.len();
    let fallback_size = fallback.len().div_ceil(workers).max(1);
    let fallback_batches: Vec<_> = fallback.chunks(fallback_size).collect();
    let fallback_results = std::thread::scope(|scope| {
        fallback_batches
            .iter()
            .map(|batch| {
                scope.spawn(move || {
                    let _library = Library::new()?;
                    let index = Index::new()?;
                    let mut evaluated = vec![];
                    for name in *batch {
                        evaluated.extend(
                            evaluate_probe(&index, input, args, std::slice::from_ref(name))?.0,
                        );
                    }
                    Ok(evaluated)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| {
                thread
                    .join()
                    .map_err(|_| Error("macro probe worker panicked".to_string()))?
            })
            .collect::<Result<Vec<_>, Error>>()
    })?;
    for fallback in fallback_results {
        evaluated.extend(fallback);
    }
    if std::env::var_os("WINDOWS_CLANG2").is_some() {
        eprintln!(
            "clang2 macro probes: {} candidates, {} bulk batches, {} recovery batches, {} fallback probes, {:.2}s",
            names.len(),
            batches.len(),
            recovery_batch_count,
            fallback_count,
            probe_time.elapsed().as_secs_f32()
        );
    }

    let mut constants = vec![];
    for evaluated in evaluated {
        let Some((root, definition)) = candidates.get(&evaluated.name) else {
            continue;
        };
        constants.push(Constant {
            root: root.clone(),
            definition: definition.clone(),
            name: evaluated.name,
            ty: evaluated.ty,
            value: evaluated.value,
        });
    }
    Ok(constants)
}

fn evaluate_probe(
    index: &Index,
    input: &Input,
    args: &[&str],
    names: &[String],
) -> Result<(Vec<Evaluated>, HashSet<String>), Error> {
    let mut probe = String::from(
        "#define __CLANG2_NARG(...) __CLANG2_NARG_(__VA_ARGS__,2,1,0)\n\
         #define __CLANG2_NARG_(_1,_2,N,...) N\n",
    );
    for name in names {
        probe.push_str(&format!(
            "#ifdef {name}\n\
             constexpr auto __clang2_eval_{name} = ({name});\n\
             enum {{ __clang2_count_{name} = __CLANG2_NARG({name}) }};\n\
             #endif\n"
        ));
    }

    let tu = TranslationUnit::parse_probe(index, input, &probe, args)?;
    let cursors = cursor_children(unsafe { clang_getTranslationUnitCursor(tu.0) });
    let mut counts = HashMap::new();
    for cursor in &cursors {
        if unsafe { clang_getCursorKind(*cursor) } == CXCursor_EnumDecl {
            for constant in cursor_children(*cursor) {
                let name = cx_string(unsafe { clang_getCursorSpelling(constant) });
                if let Some(name) = name.strip_prefix("__clang2_count_") {
                    counts.insert(name.to_string(), unsafe {
                        clang_getEnumConstantDeclValue(constant)
                    });
                }
            }
        }
    }

    let mut result = vec![];
    for cursor in cursors {
        let cursor_name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
        let Some(name) = cursor_name.strip_prefix("__clang2_eval_") else {
            continue;
        };
        if counts.get(name) != Some(&1) {
            continue;
        }
        let Some(value) = evaluate_integer(cursor) else {
            continue;
        };
        let ty = unsafe { clang_getCursorType(cursor) };
        let Some(mut ty_ref) = type_ref(ty) else {
            continue;
        };
        if let TypeRef::Named { declaration, .. } = &mut ty_ref
            && declaration.file == format!("{}.__clang2_eval.cpp", input.name)
        {
            declaration.file.clone_from(&input.name);
        }
        let Some(value_scalar) = scalar(ty) else {
            continue;
        };
        let value = if matches!(
            value_scalar,
            Scalar::Bool | Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::U64
        ) {
            Value::Unsigned(value.0)
        } else {
            Value::Signed(value.1)
        };
        result.push(Evaluated {
            name: name.to_string(),
            ty: ty_ref,
            value,
        });
    }
    let reached = counts.into_keys().collect();
    Ok((result, reached))
}

fn evaluate_integer(cursor: CXCursor) -> Option<(u64, i64)> {
    unsafe {
        let result = clang_Cursor_Evaluate(cursor);
        if result.is_null() {
            return None;
        }
        let value = (clang_EvalResult_getKind(result) == CXEval_Int).then(|| {
            (
                clang_EvalResult_getAsUnsigned(result),
                clang_EvalResult_getAsLongLong(result),
            )
        });
        clang_EvalResult_dispose(result);
        value
    }
}

fn cursor_locations(cursor: CXCursor) -> Option<(Location, Location, bool, bool)> {
    unsafe {
        let location = clang_getCursorLocation(cursor);
        let spelling = source_location(location, clang_getSpellingLocation)?;
        let expansion = source_location(location, clang_getExpansionLocation)?;
        Some((
            spelling,
            expansion,
            clang_Location_isFromMainFile(location) != 0,
            clang_Location_isInSystemHeader(location) != 0,
        ))
    }
}

type LocationFn = unsafe fn(CXSourceLocation, *mut CXFile, *mut u32, *mut u32, *mut u32);

fn source_location(location: CXSourceLocation, get: LocationFn) -> Option<Location> {
    unsafe {
        let mut file: CXFile = std::ptr::null_mut();
        let mut offset = 0;
        get(
            location,
            &mut file,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut offset,
        );
        (!file.is_null()).then(|| Location {
            file: normalize_name(&cx_string(clang_getFileName(file))),
            offset,
        })
    }
}

fn fact_kind(kind: CXCursorKind) -> Option<FactKind> {
    Some(match kind {
        CXCursor_ClassDecl => FactKind::Class,
        CXCursor_EnumDecl => FactKind::Enum,
        CXCursor_FunctionDecl => FactKind::Function,
        CXCursor_MacroDefinition => FactKind::Macro,
        CXCursor_Namespace => FactKind::Namespace,
        CXCursor_StructDecl => FactKind::Struct,
        CXCursor_TypedefDecl => FactKind::Typedef,
        CXCursor_UnionDecl => FactKind::Union,
        _ => return None,
    })
}

fn fact_data(cursor: CXCursor, kind: FactKind, macros: &HashMap<String, Vec<String>>) -> FactData {
    match kind {
        FactKind::Class => cursor_uuid(cursor).map_or_else(
            || FactData::Unsupported {
                reason: "class has no UUID".to_string(),
            },
            |guid| FactData::Class { guid },
        ),
        FactKind::Enum => {
            let ty = unsafe { clang_getEnumDeclIntegerType(cursor) };
            let Some(repr) = scalar(ty) else {
                return FactData::None;
            };
            let variants = cursor_children(cursor)
                .into_iter()
                .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_EnumConstantDecl)
                .map(|child| Variant {
                    name: cx_string(unsafe { clang_getCursorSpelling(child) }),
                    value: unsafe { clang_getEnumConstantDeclValue(child) },
                })
                .collect();
            FactData::Enum { repr, variants }
        }
        FactKind::Function => {
            if unsafe { clang_getCursorLinkage(cursor) } != CXLinkage_External
                || unsafe { clang_isCursorDefinition(cursor) } != 0
            {
                return FactData::Unsupported {
                    reason: "function is not an external declaration".to_string(),
                };
            }
            if unsafe { clang_Cursor_isVariadic(cursor) } != 0 {
                return FactData::Unsupported {
                    reason: "variadic function".to_string(),
                };
            }
            let result_ty = unsafe { clang_getCursorResultType(cursor) };
            let Some(result) = type_ref(result_ty) else {
                return FactData::Unsupported {
                    reason: format!(
                        "function has unsupported result type `{}`",
                        cx_string(unsafe { clang_getTypeSpelling(result_ty) })
                    ),
                };
            };
            let params = match callable_params(cursor, macros) {
                Ok(params) => params,
                Err(reason) => return FactData::Unsupported { reason },
            };
            let function_ty = unsafe { clang_getCursorType(cursor) };
            let Some(convention) = source_calling_convention(cursor, macros)
                .or_else(|| calling_convention_fact(function_ty))
            else {
                return FactData::Unsupported {
                    reason: "function has an unsupported calling convention".to_string(),
                };
            };
            FactData::Function {
                link_name: cx_string(unsafe { clang_Cursor_getMangling(cursor) }),
                convention,
                params,
                result,
            }
        }
        FactKind::Macro => FactData::Macro {
            function_like: unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0,
        },
        FactKind::EnumFlag => {
            let macro_name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
            let target = cursor_tokens(cursor).into_iter().find_map(|(kind, token)| {
                (kind == CXToken_Identifier && token != macro_name).then_some(token)
            });
            if let Some(target) = target {
                FactData::EnumFlag { target }
            } else {
                FactData::Unsupported {
                    reason: "enum flag macro has no type argument".to_string(),
                }
            }
        }
        FactKind::Struct | FactKind::Union => {
            if kind == FactKind::Struct && is_interface(cursor) {
                return interface_fact(cursor, macros);
            }
            let definition = unsafe { clang_isCursorDefinition(cursor) } != 0;
            let mut record = if definition {
                match inline_record(cursor, kind == FactKind::Union) {
                    Ok(record) => record,
                    Err(reason) => return FactData::Unsupported { reason },
                }
            } else {
                InlineRecord {
                    name: None,
                    base: None,
                    fields: vec![],
                    size: unsafe { clang_Type_getSizeOf(clang_getCursorType(cursor)) },
                    align: unsafe { clang_Type_getAlignOf(clang_getCursorType(cursor)) },
                    packing: None,
                    alignment: None,
                    union: kind == FactKind::Union,
                }
            };
            name_indirect_inline_records(
                &mut record,
                cx_string(unsafe { clang_getCursorSpelling(cursor) }).trim_start_matches('_'),
                &mut 0,
            );
            FactData::Record {
                base: record.base,
                fields: record.fields,
                size: record.size,
                align: record.align,
                packing: record.packing,
                alignment: record.alignment,
                union: record.union,
            }
        }
        FactKind::Typedef => {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
            if ty.kind == CXType_Pointer {
                let function = unsafe { clang_getPointeeType(ty) };
                if function.kind == CXType_FunctionProto {
                    return function_signature(function).map_or_else(
                        || FactData::Unsupported {
                            reason: "callback has an unsupported signature".to_string(),
                        },
                        |(convention, params, result)| FactData::Callback {
                            convention,
                            params,
                            result,
                        },
                    );
                }
            }
            type_ref(ty).map_or_else(
                || FactData::Unsupported {
                    reason: format!(
                        "typedef has unsupported type `{}`",
                        cx_string(unsafe { clang_getTypeSpelling(ty) })
                    ),
                },
                |target| FactData::Typedef { target },
            )
        }
        _ => FactData::None,
    }
}

fn is_interface(cursor: CXCursor) -> bool {
    let definition = unsafe { clang_getCursorDefinition(cursor) };
    let cursor = if unsafe { clang_Cursor_isNull(definition) } == 0 {
        definition
    } else {
        cursor
    };
    if unsafe { clang_isCursorDefinition(cursor) } == 0 {
        return false;
    }
    let children = cursor_children(cursor);
    if children
        .iter()
        .any(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_FieldDecl)
    {
        return false;
    }
    let methods: Vec<_> = children
        .iter()
        .filter(|child| unsafe {
            clang_getCursorKind(**child) == CXCursor_CXXMethod
                && clang_CXXMethod_isVirtual(**child) != 0
        })
        .collect();
    (!methods.is_empty()
        && methods
            .iter()
            .all(|method| unsafe { clang_CXXMethod_isPureVirtual(**method) } != 0))
        || children.iter().any(|child| {
            if unsafe { clang_getCursorKind(*child) } != CXCursor_CXXBaseSpecifier {
                return false;
            }
            let declaration = unsafe { clang_getTypeDeclaration(clang_getCursorType(*child)) };
            (unsafe { clang_Cursor_isNull(declaration) }) == 0 && is_interface(declaration)
        })
}

fn interface_fact(cursor: CXCursor, macros: &HashMap<String, Vec<String>>) -> FactData {
    let mut base = None;
    let guid = cursor_uuid(cursor);
    let mut methods = vec![];
    for child in cursor_children(cursor) {
        match unsafe { clang_getCursorKind(child) } {
            CXCursor_CXXBaseSpecifier => {
                if base.is_some() {
                    return FactData::Unsupported {
                        reason: "interface has multiple bases".to_string(),
                    };
                }
                let declaration = unsafe { clang_getTypeDeclaration(clang_getCursorType(child)) };
                if unsafe { clang_Cursor_isNull(declaration) } != 0 || !is_interface(declaration) {
                    return FactData::Unsupported {
                        reason: "interface base is not an interface".to_string(),
                    };
                }
                let Some(ty) = type_ref(unsafe { clang_getCursorType(child) }) else {
                    return FactData::Unsupported {
                        reason: "interface has an unsupported base".to_string(),
                    };
                };
                base = Some(ty);
            }
            CXCursor_CXXMethod if unsafe { clang_CXXMethod_isPureVirtual(child) } != 0 => {
                if method_overrides_base(child) {
                    continue;
                }
                let result_ty = unsafe { clang_getCursorResultType(child) };
                let Some(result) = type_ref(result_ty) else {
                    return FactData::Unsupported {
                        reason: "interface method has an unsupported result".to_string(),
                    };
                };
                let mut params = match callable_params(child, macros) {
                    Ok(params) => params,
                    Err(reason) => return FactData::Unsupported { reason },
                };
                let tokens = cursor_tokens(child);
                apply_midl_annotations(&tokens, &mut params);
                let special =
                    tokens_before_method_name(&tokens, child)
                        .iter()
                        .any(|(kind, token)| {
                            *kind == CXToken_Comment
                                && (token.contains("[propget]") || token.contains("[propput]"))
                        });
                methods.push(Method {
                    name: cx_string(unsafe { clang_getCursorSpelling(child) }),
                    params,
                    result,
                    special,
                });
            }
            CXCursor_CXXMethod if unsafe { clang_CXXMethod_isVirtual(child) } != 0 => {
                return FactData::Unsupported {
                    reason: "interface has a non-pure virtual method".to_string(),
                };
            }
            CXCursor_CXXMethod => {}
            CXCursor_Constructor | CXCursor_Destructor => {
                return FactData::Unsupported {
                    reason: "interface has a constructor or destructor".to_string(),
                };
            }
            _ => {}
        }
    }
    FactData::Interface {
        base,
        guid,
        methods,
    }
}

fn cursor_uuid(cursor: CXCursor) -> Option<String> {
    let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
    for child in cursor_children(cursor) {
        if unsafe { clang_getCursorKind(child) } != CXCursor_UnexposedAttr {
            continue;
        }
        let range = expansion_range(tu, unsafe { clang_getCursorExtent(child) });
        let mut tokens = std::ptr::null_mut();
        let mut count = 0;
        unsafe { clang_tokenize(tu, range, &mut tokens, &mut count) };
        for index in 0..count {
            let token = unsafe { *tokens.add(index as usize) };
            if unsafe { clang_getTokenKind(token) } == CXToken_Literal {
                let spelling = cx_string(unsafe { clang_getTokenSpelling(tu, token) });
                let value = spelling.trim_matches('"');
                if is_uuid(value) {
                    unsafe { clang_disposeTokens(tu, tokens, count) };
                    return Some(value.to_ascii_lowercase());
                }
            }
        }
        unsafe { clang_disposeTokens(tu, tokens, count) };
    }
    None
}

fn expansion_range(tu: CXTranslationUnit, range: CXSourceRange) -> CXSourceRange {
    unsafe {
        let mut start_file = std::ptr::null_mut();
        let mut start_line = 0;
        let mut start_column = 0;
        let mut start_offset = 0;
        clang_getExpansionLocation(
            clang_getRangeStart(range),
            &mut start_file,
            &mut start_line,
            &mut start_column,
            &mut start_offset,
        );
        let mut end_file = std::ptr::null_mut();
        let mut end_line = 0;
        let mut end_column = 0;
        let mut end_offset = 0;
        clang_getExpansionLocation(
            clang_getRangeEnd(range),
            &mut end_file,
            &mut end_line,
            &mut end_column,
            &mut end_offset,
        );
        clang_getRange(
            clang_getLocation(tu, start_file, start_line, start_column),
            clang_getLocation(tu, end_file, end_line, end_column),
        )
    }
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value
            .chars()
            .enumerate()
            .all(|(index, character)| match index {
                8 | 13 | 18 | 23 => character == '-',
                _ => character.is_ascii_hexdigit(),
            })
}

fn method_overrides_base(cursor: CXCursor) -> bool {
    let mut cursors = std::ptr::null_mut();
    let mut count = 0;
    unsafe {
        clang_getOverriddenCursors(cursor, &mut cursors, &mut count);
        clang_disposeOverriddenCursors(cursors);
    }
    count != 0
}

fn callable_params(
    cursor: CXCursor,
    macros: &HashMap<String, Vec<String>>,
) -> Result<Vec<Parameter>, String> {
    let mut params = vec![];
    for child in cursor_children(cursor)
        .into_iter()
        .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
    {
        let param_ty = unsafe { clang_getCursorType(child) };
        let Some(ty) = function_param_type(param_ty) else {
            return Err(format!(
                "parameter has unsupported type `{}`",
                cx_string(unsafe { clang_getTypeSpelling(param_ty) })
            ));
        };
        let mut name = cx_string(unsafe { clang_getCursorSpelling(child) });
        if name.is_empty() {
            name = format!("param{}", params.len());
        }
        let annotation = parameter_annotation(child);
        if let Some(reason) = &annotation.unsupported {
            return Err(reason.clone());
        }
        params.push(Parameter {
            name,
            ty,
            annotation,
        });
    }
    let names: BTreeSet<_> = params.iter().map(|param| param.name.clone()).collect();
    for param in &mut params {
        if let Some(size) = &mut param.annotation.size {
            match &mut size.value {
                SalSizeValue::Parameter(name) if !names.contains(name) => {
                    if macros.contains_key(name) {
                        size.value = SalSizeValue::Expression(name.clone());
                    } else {
                        return Err(format!("unresolved SAL size parameter `{name}`"));
                    }
                }
                SalSizeValue::IndirectParameter(name) if !names.contains(name) => {
                    return Err(format!("unresolved SAL size parameter `{name}`"));
                }
                SalSizeValue::Constant(_) if size.bytes => {
                    return Err("constant byte-size SAL annotations are unsupported".to_string());
                }
                SalSizeValue::Expression(_) => {}
                _ => {}
            }
        }
    }
    Ok(params)
}

fn cursor_tokens(cursor: CXCursor) -> Vec<(CXTokenKind, String)> {
    let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
    let range = expansion_range(tu, unsafe { clang_getCursorExtent(cursor) });
    let mut tokens = std::ptr::null_mut();
    let mut count = 0;
    unsafe { clang_tokenize(tu, range, &mut tokens, &mut count) };
    let result = (0..count)
        .map(|index| {
            let token = unsafe { *tokens.add(index as usize) };
            (
                unsafe { clang_getTokenKind(token) },
                cx_string(unsafe { clang_getTokenSpelling(tu, token) }),
            )
        })
        .collect();
    unsafe { clang_disposeTokens(tu, tokens, count) };
    result
}

fn tokens_before_method_name(
    tokens: &[(CXTokenKind, String)],
    cursor: CXCursor,
) -> &[(CXTokenKind, String)] {
    let name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
    let end = tokens
        .iter()
        .position(|(kind, token)| *kind == CXToken_Identifier && token == &name)
        .unwrap_or(0);
    &tokens[..end]
}

fn apply_midl_annotations(tokens: &[(CXTokenKind, String)], params: &mut [Parameter]) {
    let Some(open) = tokens
        .iter()
        .position(|(kind, token)| *kind == CXToken_Punctuation && token == "(")
    else {
        return;
    };
    let mut index = 0;
    let mut depth = 1;
    for (kind, token) in &tokens[open + 1..] {
        match (*kind, token.as_str()) {
            (CXToken_Punctuation, "(") => depth += 1,
            (CXToken_Punctuation, ")") => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            (CXToken_Punctuation, ",") if depth == 1 => index += 1,
            (CXToken_Comment, comment) if depth == 1 && index < params.len() => {
                let annotation = &mut params[index].annotation;
                annotation.input |= comment.contains("[in]");
                annotation.output |= comment.contains("[out]");
                annotation.optional |= comment.contains("[optional]");
                annotation.retval |= comment.contains("[retval]");
                annotation.com_out_ptr |= comment.contains("[iid_is]") && annotation.output;
            }
            _ => {}
        }
    }
}

fn parameter_annotation(cursor: CXCursor) -> ParamAnnotation {
    let mut result = ParamAnnotation::default();
    for child in cursor_children(cursor) {
        if unsafe { clang_getCursorKind(child) } != CXCursor_AnnotateAttr {
            continue;
        }
        let annotation = cx_string(unsafe { clang_getCursorSpelling(child) });
        if annotation.starts_with("_In_") || annotation.starts_with("_Inout_") {
            result.input = true;
        }
        if annotation.starts_with("_Out_")
            || annotation.starts_with("_Outptr_")
            || annotation.starts_with("_COM_Outptr_")
            || annotation.starts_with("_Inout_")
        {
            result.output = true;
        }
        if annotation.contains("_opt_")
            || (annotation.starts_with("_Outptr_") && annotation.contains("_result_maybenull_"))
        {
            result.optional = true;
        }
        if annotation == "_Reserved_" {
            result.reserved = true;
        }
        if annotation.starts_with("_COM_Outptr_") {
            result.com_out_ptr = true;
        }
        let sal_name = annotation
            .split_once('(')
            .map_or(annotation.as_str(), |value| value.0);
        if matches!(
            sal_name,
            "_In_z_" | "_In_opt_z_" | "_Out_z_" | "_Inout_z_" | "_Inout_opt_z_"
        ) {
            result.null_terminated = true;
        }
        if result.size.is_none()
            && (annotation.contains("_reads_")
                || annotation.contains("_writes_")
                || annotation.contains("_updates_"))
            && let Some(argument) = annotation
                .split_once('(')
                .and_then(|(_, rest)| rest.strip_suffix(')'))
                .and_then(|arguments| arguments.split(',').next())
        {
            let argument = argument.trim();
            let value = if let Some(value) = parse_sal_integer(argument) {
                SalSizeValue::Constant(value)
            } else if is_c_identifier(argument) {
                SalSizeValue::Parameter(argument.to_string())
            } else if let Some(argument) = argument.strip_prefix('*').map(str::trim)
                && is_c_identifier(argument)
            {
                SalSizeValue::IndirectParameter(argument.to_string())
            } else {
                SalSizeValue::Expression(argument.to_string())
            };
            result.size = Some(SalSize {
                bytes: annotation.contains("_bytes"),
                value,
            });
        }
    }
    result
}

fn parse_sal_integer(value: &str) -> Option<i32> {
    let value = value.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(value) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        i32::from_str_radix(value, 16).ok()
    } else {
        value.parse().ok()
    }
}

fn is_c_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(first) if first == '_' || first.is_ascii_alphabetic())
        && chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn function_signature(ty: CXType) -> Option<(CallingConvention, Vec<TypeRef>, TypeRef)> {
    let convention = calling_convention_fact(ty)?;
    let result = type_ref(unsafe { clang_getResultType(ty) })?;
    let count = unsafe { clang_getNumArgTypes(ty) };
    if count < 0 {
        return None;
    }
    let params = (0..count)
        .map(|index| {
            function_param_type(unsafe { clang_getArgType(ty, index.try_into().unwrap()) })
        })
        .collect::<Option<Vec<_>>>()?;
    Some((convention, params, result))
}

fn function_param_type(ty: CXType) -> Option<TypeRef> {
    if matches!(ty.kind, CXType_IncompleteArray | CXType_ConstantArray) {
        let element = unsafe { clang_getArrayElementType(ty) };
        return Some(TypeRef::Pointer {
            mutable: unsafe { clang_isConstQualifiedType(element) } == 0,
            target: Box::new(type_ref(element)?),
        });
    }
    if matches!(ty.kind, CXType_FunctionProto | CXType_FunctionNoProto) {
        let (convention, params, result) = function_signature(ty)?;
        return Some(TypeRef::FunctionPointer {
            convention,
            params,
            result: Box::new(result),
        });
    }
    type_ref(ty)
}

fn calling_convention_fact(ty: CXType) -> Option<CallingConvention> {
    Some(match unsafe { clang_getFunctionTypeCallingConv(ty) } {
        CXCallingConv_C => CallingConvention::C,
        CXCallingConv_Default | CXCallingConv_X86StdCall | CXCallingConv_Win64 => {
            CallingConvention::Platform
        }
        _ => return None,
    })
}

fn source_calling_convention(
    cursor: CXCursor,
    macros: &HashMap<String, Vec<String>>,
) -> Option<CallingConvention> {
    fn resolve(
        token: &str,
        macros: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
    ) -> Option<CallingConvention> {
        match token {
            "__stdcall" | "_stdcall" => return Some(CallingConvention::Platform),
            "__cdecl" | "_cdecl" => return Some(CallingConvention::C),
            _ => {}
        }
        if !visited.insert(token.to_string()) {
            return None;
        }
        macros
            .get(token)?
            .iter()
            .find_map(|token| resolve(token, macros, visited))
    }

    let tokens = cursor_tokens(cursor);
    tokens_before_method_name(&tokens, cursor)
        .iter()
        .find_map(|(_, token)| resolve(token, macros, &mut HashSet::new()))
}

fn inline_record(cursor: CXCursor, union: bool) -> Result<InlineRecord, String> {
    let mut base = None;
    let mut base_count = 0;
    let mut fields: Vec<Field> = vec![];
    let mut anonymous = 0;
    for child in cursor_children(cursor) {
        let kind = unsafe { clang_getCursorKind(child) };
        if kind == CXCursor_CXXBaseSpecifier {
            if union || unsafe { clang_isVirtualBase(child) } != 0 {
                return Err("record has unsupported inheritance".to_string());
            }
            let base_ty = unsafe { clang_getCursorType(child) };
            let base_ref = type_ref(base_ty)
                .ok_or_else(|| "record base has an unsupported type".to_string())?;
            let align = unsafe { clang_Type_getAlignOf(base_ty) };
            let size = unsafe { clang_Type_getSizeOf(base_ty) };
            if align <= 0 || size < 0 {
                return Err("record base has an invalid layout".to_string());
            }
            let offset = if let Some(previous) = fields.last() {
                align_up((previous.offset + previous.size * 8) / 8, align) * 8
            } else {
                0
            };
            base_count += 1;
            fields.push(Field {
                name: if base_count == 1 {
                    "Base".to_string()
                } else {
                    format!("Base{base_count}")
                },
                ty: base_ref.clone(),
                offset,
                align,
                size,
                bit_width: None,
            });
            base.get_or_insert(base_ref);
        } else if kind == CXCursor_FieldDecl {
            let name = cx_string(unsafe { clang_getCursorSpelling(child) });
            let field_ty = unsafe { clang_getCursorType(child) };
            let ty = type_ref(field_ty).ok_or_else(|| {
                format!(
                    "field `{name}` has unsupported type `{}`",
                    cx_string(unsafe { clang_getTypeSpelling(field_ty) })
                )
            })?;
            let bit_width = if unsafe { clang_Cursor_isBitField(child) } != 0 {
                let width = unsafe { clang_getFieldDeclBitWidth(child) };
                if width < 0 {
                    return Err(format!("bitfield `{name}` width is unavailable"));
                }
                Some(
                    width
                        .try_into()
                        .map_err(|_| format!("bitfield `{name}` width is out of range"))?,
                )
            } else {
                None
            };
            fields.push(Field {
                name,
                ty,
                offset: unsafe { clang_Cursor_getOffsetOfField(child) },
                align: if field_ty.kind == CXType_IncompleteArray {
                    unsafe { clang_Type_getAlignOf(clang_getArrayElementType(field_ty)) }
                } else {
                    unsafe { clang_Type_getAlignOf(field_ty) }
                },
                size: if field_ty.kind == CXType_IncompleteArray {
                    0
                } else {
                    unsafe { clang_Type_getSizeOf(field_ty) }
                },
                bit_width,
            });
        } else if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
            && unsafe { clang_Cursor_isAnonymousRecordDecl(child) } != 0
        {
            let nested = inline_record(child, kind == CXCursor_UnionDecl)?;
            let (promoted, relative) = promoted_members(&nested)
                .into_iter()
                .find_map(|(member, relative)| {
                    let member = CString::new(member).unwrap();
                    let promoted = unsafe {
                        clang_Type_getOffsetOf(clang_getCursorType(cursor), member.as_ptr())
                    };
                    (promoted >= 0).then_some((promoted, relative))
                })
                .ok_or_else(|| "anonymous aggregate offset is unavailable".to_string())?;
            anonymous += 1;
            fields.push(Field {
                name: if anonymous == 1 {
                    "Anonymous".to_string()
                } else {
                    format!("Anonymous{anonymous}")
                },
                offset: promoted - relative,
                align: nested.align,
                size: nested.size,
                bit_width: None,
                ty: TypeRef::InlineRecord(Box::new(nested)),
            });
        }
    }
    let mut names = BTreeSet::new();
    if let Some(duplicate) = fields
        .iter()
        .map(|field| field.name.as_str())
        .filter(|name| !name.is_empty())
        .find(|name| !names.insert(*name))
    {
        return Err(format!("record inheritance duplicates field `{duplicate}`"));
    }

    let ty = unsafe { clang_getCursorType(cursor) };
    let size = unsafe { clang_Type_getSizeOf(ty) };
    let align = unsafe { clang_Type_getAlignOf(ty) };
    let (packing, alignment) = record_layout(&fields, size, align, union)
        .map_err(|reason| format!("{reason}: fields {fields:?}, size {size}, alignment {align}"))?;
    Ok(InlineRecord {
        name: None,
        base,
        fields,
        size,
        align,
        packing,
        alignment,
        union,
    })
}

fn name_indirect_inline_records(record: &mut InlineRecord, owner: &str, next: &mut usize) {
    for field in &mut record.fields {
        name_indirect_inline_type(&mut field.ty, owner, next, false);
    }
}

fn name_indirect_inline_type(ty: &mut TypeRef, owner: &str, next: &mut usize, indirect: bool) {
    match ty {
        TypeRef::Array { target, .. }
        | TypeRef::Pointer { target, .. }
        | TypeRef::Reference { target, .. } => {
            name_indirect_inline_type(target, owner, next, true);
        }
        TypeRef::InlineRecord(record) => {
            if indirect && record.name.is_none() {
                record.name = Some(format!("{owner}_{}", *next));
                *next += 1;
            }
            name_indirect_inline_records(record, owner, next);
        }
        _ => {}
    }
}

fn promoted_members(record: &InlineRecord) -> Vec<(&str, i64)> {
    let mut result = vec![];
    for field in &record.fields {
        if !field.name.is_empty() {
            result.push((field.name.as_str(), field.offset));
        }
        if let TypeRef::InlineRecord(nested) = &field.ty {
            for (name, offset) in promoted_members(nested) {
                result.push((name, field.offset + offset));
            }
        }
    }
    result
}

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

fn write_callable(
    name: &str,
    convention: CallingConvention,
    params: &[TypeRef],
    result: &TypeRef,
    type_names: &BTreeMap<String, String>,
    interface_names: &BTreeSet<(String, String)>,
    tu: &str,
) -> String {
    let params = params
        .iter()
        .enumerate()
        .map(|(index, param)| {
            format!(
                "param{index}: {}",
                planned_emitted_type_name(param, type_names, interface_names, tu)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let result = if *result == TypeRef::Void {
        String::new()
    } else {
        format!(
            " -> {}",
            planned_emitted_type_name(result, type_names, interface_names, tu)
        )
    };
    format!(
        "    extern{} fn {}({params}){result};\n",
        calling_convention(convention),
        rdl_ident(name)
    )
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
    type_names: &BTreeMap<String, String>,
    interface_names: &BTreeSet<(String, String)>,
    tu: &str,
) -> Result<String, Error> {
    let base = base.map_or_else(String::new, |base| {
        format!(
            ": {}",
            planned_emitted_type_name(base, type_names, interface_names, tu)
        )
    });
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
            let params = method
                .params
                .iter()
                .map(|param| -> Result<_, Error> {
                    let ty = planned_param_type_name(param, type_names, interface_names, tu);
                    Ok(format!(
                        "{}{}: {}",
                        param_attributes(param, &method.params, ty.starts_with("*mut "))?,
                        rdl_ident(&param.name),
                        ty
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
            let return_type = if method.result == TypeRef::Void {
                String::new()
            } else {
                format!(
                    " -> {}",
                    planned_emitted_type_name(&method.result, type_names, interface_names, tu)
                )
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
    if let Some(name) = projected_string_name(param) {
        return type_names
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string());
    }
    planned_emitted_type_name(&param.ty, type_names, interface_names, tu)
}

fn projected_string_name(param: &Parameter) -> Option<&'static str> {
    if !param.annotation.null_terminated || param.annotation.size.is_some() {
        return None;
    }
    match &param.ty {
        TypeRef::Pointer { mutable, target } => match (mutable, target.as_ref()) {
            (false, TypeRef::Scalar(Scalar::I8 | Scalar::U8)) => Some("PCSTR"),
            (true, TypeRef::Scalar(Scalar::I8 | Scalar::U8)) => Some("PSTR"),
            (false, TypeRef::Scalar(Scalar::U16)) => Some("PCWSTR"),
            (true, TypeRef::Scalar(Scalar::U16)) => Some("PWSTR"),
            _ => None,
        },
        TypeRef::Named { name, .. } => match name.as_str() {
            "LPCSTR" => Some("PCSTR"),
            "LPSTR" => Some("PSTR"),
            "LPCWSTR" => Some("PCWSTR"),
            "LPWSTR" => Some("PWSTR"),
            _ => None,
        },
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
        && let Some(name) = canonical_named_type(name)
    {
        return name.to_string();
    }
    if let TypeRef::Reference { mutable, target } = ty {
        if let TypeRef::Named { name, .. } = target.as_ref()
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
        && let TypeRef::Named { name, .. } = target
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
    Some(match name {
        "boolean" | "BYTE" | "UCHAR" | "UINT8" | "uint8_t" => "u8",
        "WORD" | "USHORT" | "WCHAR" | "UINT16" | "uint16_t" => "u16",
        "DWORD" | "UINT" | "ULONG" | "DWORD32" | "UINT32" | "ULONG32" | "uint32_t" => "u32",
        "QWORD" | "ULONGLONG" | "DWORD64" | "UINT64" | "ULONG64" | "uint64_t" => "u64",
        "CHAR" | "INT8" | "int8_t" => "i8",
        "SHORT" | "INT16" | "int16_t" => "i16",
        "INT" | "LONG" | "INT32" | "LONG32" | "int32_t" => "i32",
        "LONGLONG" | "INT64" | "LONG64" | "int64_t" => "i64",
        "IID" | "CLSID" | "FMTID" | "UUID" => "GUID",
        "HRESULT" => "HRESULT",
        _ => return None,
    })
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

    for packing in [None, Some(1), Some(2), Some(4), Some(8), Some(16)] {
        let mut cursor = 0;
        let mut natural_align = 1;
        let mut matches = true;
        let groups = bitfield_groups(fields).map_err(|error| error.0)?;
        let mut group_index = 0;
        let mut index = 0;
        while index < fields.len() {
            let field = &fields[index];
            let field_align = packing.map_or(field.align, |packing| packing.min(field.align));
            natural_align = natural_align.max(field_align);
            let offset = if union {
                0
            } else {
                align_up(cursor, field_align)
            };
            let expected = offset * 8;
            if expected != field.offset {
                matches = false;
                break;
            }
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
        if align_up(content_size, align) == size {
            return Ok((packing, alignment));
        }
    }
    Err("record fields cannot reproduce Clang's layout".to_string())
}

fn align_up(value: i64, align: i64) -> i64 {
    (value + align - 1) / align * align
}

fn type_ref(ty: CXType) -> Option<TypeRef> {
    if ty.kind == CXType_Elaborated {
        return type_ref(unsafe { clang_Type_getNamedType(ty) });
    }
    if ty.kind == CXType_Void {
        return Some(TypeRef::Void);
    }
    if ty.kind == CXType_Pointer {
        let pointee = unsafe { clang_getPointeeType(ty) };
        if matches!(pointee.kind, CXType_FunctionProto | CXType_FunctionNoProto) {
            let (convention, params, result) = function_signature(pointee)?;
            return Some(TypeRef::FunctionPointer {
                convention,
                params,
                result: Box::new(result),
            });
        }
        let Some(target) = type_ref(pointee) else {
            let spelling = cx_string(unsafe { clang_getTypeSpelling(pointee) });
            let tag = spelling
                .strip_prefix("struct ")
                .filter(|tag| tag.ends_with("__"))?;
            return Some(TypeRef::OpaquePointer {
                mutable: unsafe { clang_isConstQualifiedType(pointee) } == 0,
                tag: tag.to_string(),
            });
        };
        return Some(TypeRef::Pointer {
            mutable: unsafe { clang_isConstQualifiedType(pointee) } == 0,
            target: Box::new(target),
        });
    }
    if ty.kind == CXType_LValueReference || ty.kind == CXType_RValueReference {
        let referent = unsafe { clang_getPointeeType(ty) };
        let target = type_ref(referent)?;
        return Some(TypeRef::Reference {
            mutable: unsafe { clang_isConstQualifiedType(referent) } == 0,
            target: Box::new(target),
        });
    }
    if matches!(ty.kind, CXType_ConstantArray | CXType_IncompleteArray) {
        let target = unsafe { clang_getArrayElementType(ty) };
        let len = if ty.kind == CXType_IncompleteArray {
            0
        } else {
            let len = unsafe { clang_getArraySize(ty) };
            if len < 0 {
                return None;
            }
            len.try_into().unwrap()
        };
        let target = type_ref(target)?;
        return Some(TypeRef::Array {
            target: Box::new(target),
            len,
        });
    }
    let declaration = unsafe { clang_getTypeDeclaration(ty) };
    let definition = unsafe { clang_getCursorDefinition(declaration) };
    let declaration = if unsafe { clang_Cursor_isNull(definition) } == 0 {
        definition
    } else {
        declaration
    };
    if unsafe { clang_Cursor_isNull(declaration) } == 0 {
        let kind = unsafe { clang_getCursorKind(declaration) };
        if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
            && unsafe { clang_Cursor_isAnonymous(declaration) } != 0
        {
            return inline_record(declaration, kind == CXCursor_UnionDecl)
                .ok()
                .map(|record| TypeRef::InlineRecord(Box::new(record)));
        }
        if matches!(
            kind,
            CXCursor_ClassDecl
                | CXCursor_EnumDecl
                | CXCursor_StructDecl
                | CXCursor_TypedefDecl
                | CXCursor_UnionDecl
        ) {
            let name = cx_string(unsafe { clang_getCursorSpelling(declaration) });
            if !name.is_empty()
                && let Some((location, _, _, _)) = cursor_locations(declaration)
            {
                return Some(TypeRef::Named {
                    name,
                    declaration: location,
                });
            }
        }
    }
    let canonical = unsafe { clang_getCanonicalType(ty) };
    if canonical.kind != ty.kind {
        return type_ref(canonical);
    }
    scalar(ty).map(TypeRef::Scalar)
}

fn scalar(ty: CXType) -> Option<Scalar> {
    let ty = unsafe { clang_getCanonicalType(ty) };
    Some(match ty.kind {
        CXType_Bool => Scalar::Bool,
        CXType_Float => Scalar::F32,
        CXType_Double | CXType_LongDouble => Scalar::F64,
        CXType_WChar | CXType_Char16 => Scalar::U16,
        CXType_Char32 => Scalar::U32,
        CXType_Char_S | CXType_SChar => Scalar::I8,
        CXType_Char_U | CXType_UChar => Scalar::U8,
        CXType_Short => Scalar::I16,
        CXType_UShort => Scalar::U16,
        CXType_Int | CXType_Long => Scalar::I32,
        CXType_UInt | CXType_ULong => Scalar::U32,
        CXType_LongLong => Scalar::I64,
        CXType_ULongLong => Scalar::U64,
        CXType_Enum => {
            let declaration = unsafe { clang_getTypeDeclaration(ty) };
            return scalar(unsafe { clang_getEnumDeclIntegerType(declaration) });
        }
        _ => return None,
    })
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

fn type_name(ty: &TypeRef) -> String {
    match ty {
        TypeRef::Void => "void".to_string(),
        TypeRef::Scalar(scalar) => scalar_name(*scalar).to_string(),
        TypeRef::Named { name, .. } => rdl_ident(name),
        TypeRef::Pointer { .. } => {
            let (mutable, depth, target) = pointer_run(ty);
            format!(
                "{}{}",
                format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth),
                type_name(target)
            )
        }
        TypeRef::Reference { mutable, target } => format!(
            "*{} {}",
            if *mutable { "mut" } else { "const" },
            type_name(target)
        ),
        TypeRef::FunctionPointer { .. } => "*mut u8".to_string(),
        TypeRef::OpaquePointer { mutable, .. } => {
            format!("*{} void", if *mutable { "mut" } else { "const" })
        }
        TypeRef::Array { target, len } => format!("[{}; {len}]", type_name(target)),
        TypeRef::InlineRecord(record) => record
            .name
            .clone()
            .unwrap_or_else(|| "<inline record>".to_string()),
    }
}

fn planned_type_name(ty: &TypeRef, type_names: &BTreeMap<String, String>) -> String {
    match ty {
        TypeRef::Named { name, .. } => rdl_ident(type_names.get(name).unwrap_or(name)),
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
        _ => type_name(ty),
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
        mutable = *level_mutable;
        depth += 1;
        ty = target;
    }
    (mutable, depth, ty)
}

fn constant_type_name(ty: &TypeRef, type_names: &BTreeMap<String, String>) -> String {
    match ty {
        TypeRef::Scalar(Scalar::Bool) => "u32".to_string(),
        TypeRef::Named { name, .. } => canonical_named_type(name)
            .map_or_else(|| planned_type_name(ty, type_names), str::to_string),
        _ => planned_type_name(ty, type_names),
    }
}

fn value_name(value: &Value) -> String {
    match value {
        Value::Signed(value) => value.to_string(),
        Value::Unsigned(value) => value.to_string(),
    }
}

fn rdl_ident(name: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
        "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if",
        "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv",
        "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "try",
        "type", "typeof", "union", "unsafe", "unsized", "use", "virtual", "where", "while",
        "yield",
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

fn cx_string(value: CXString) -> String {
    unsafe {
        let pointer = clang_getCString(value);
        let result = if pointer.is_null() {
            String::new()
        } else {
            CStr::from_ptr(pointer).to_string_lossy().into_owned()
        };
        clang_disposeString(value);
        result
    }
}

fn normalize_name(name: &str) -> String {
    name.replace('\\', "/")
}

fn origin(origin: &Origin) -> String {
    format!("{}#{}", origin.tu, origin.local)
}
