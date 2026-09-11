#![allow(non_upper_case_globals)]

use clang_sys::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub source: String,
}

impl Input {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: normalize_name(&name.into()),
            source: source.into(),
        }
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
    Named { name: String, declaration: Location },
    Pointer { mutable: bool, target: Box<Self> },
    Array { target: Box<Self>, len: usize },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Field {
    pub name: String,
    pub ty: TypeRef,
    pub offset: i64,
    pub align: i64,
    pub size: i64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Variant {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FactData {
    Enum {
        repr: Scalar,
        variants: Vec<Variant>,
    },
    Macro {
        function_like: bool,
    },
    Record {
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
    Enum,
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
        let plan = self.plan()?;
        let mut items = BTreeMap::new();
        for planned in plan.types {
            let fact = planned.fact;
            let item = match &fact.data {
                FactData::Typedef { target } => {
                    format!(
                        "    type {} = {};\n",
                        planned.name,
                        planned_type_name(target, &plan.type_names)
                    )
                }
                FactData::Enum { repr, variants } if fact.definition => {
                    let mut item = format!(
                        "    #[repr({})]\n    enum {} {{\n",
                        scalar_name(*repr),
                        planned.name
                    );
                    for variant in variants {
                        item.push_str(&format!("        {} = {},\n", variant.name, variant.value));
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
                    let keyword = if *union { "union" } else { "struct" };
                    let mut item = String::new();
                    if let Some(packing) = packing {
                        item.push_str(&format!("    #[packed({packing})]\n"));
                    }
                    if let Some(alignment) = alignment {
                        item.push_str(&format!("    #[align({alignment})]\n"));
                    }
                    item.push_str(&format!("    {keyword} {} {{\n", planned.name));
                    for field in fields {
                        item.push_str(&format!(
                            "        {}: {},\n",
                            field.name,
                            planned_type_name(&field.ty, &plan.type_names)
                        ));
                    }
                    item.push_str("    }\n");
                    item
                }
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
        for constant in plan.constants {
            let item = format!(
                "    const {}: {} = {};\n",
                constant.name,
                constant_type_name(&constant.ty, &plan.type_names),
                value_name(&constant.value)
            );
            if items.insert(constant.name.clone(), item).is_some() {
                return Err(Error(format!("duplicate planned name `{}`", constant.name)));
            }
        }

        let namespaces: Vec<_> = namespace
            .split('.')
            .filter(|name| !name.is_empty())
            .collect();
        if namespaces.is_empty() {
            return Err(Error("namespace is empty".to_string()));
        }
        let mut result = String::from("#[win32]\n");
        for (depth, namespace) in namespaces.iter().enumerate() {
            result.push_str(&format!("{}mod {namespace} {{\n", "    ".repeat(depth)));
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
        Ok(result)
    }

    fn plan(&self) -> Result<Plan<'_>, Error> {
        #[derive(Default)]
        struct Roots<'a> {
            types: Vec<&'a Fact>,
            values: Vec<&'a Constant>,
        }

        let mut roots: BTreeMap<&str, Roots<'_>> = BTreeMap::new();
        for fact in self
            .facts
            .iter()
            .filter(|fact| fact.main_file && is_root_type_fact(fact))
        {
            roots.entry(&fact.name).or_default().types.push(fact);
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
        let mut type_roots = vec![];
        let mut constants = vec![];
        let mut root_names = BTreeSet::new();

        for (name, roots) in roots {
            if !roots.types.is_empty() {
                let root = choose_type_root(name, &roots.types)?;
                root_names.insert(name.to_string());
                type_roots.push(root);
            } else {
                let constant = choose_constant_root(name, &roots.values)?;
                constants.push(constant);
            }
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
                queue.push((constant.root.tu.as_str(), &constant.ty));
            }

            while let Some((tu, ty)) = queue.pop() {
                let (name, declaration) = match ty {
                    TypeRef::Pointer { target, .. } => {
                        queue.push((tu, target));
                        continue;
                    }
                    TypeRef::Array { target, .. } => {
                        queue.push((tu, target));
                        continue;
                    }
                    TypeRef::Named { name, declaration } => (name, declaration),
                    _ => continue,
                };
                let matches: Vec<_> = self
                    .facts
                    .iter()
                    .filter(|fact| {
                        fact.origin.tu == tu && fact.name == *name && fact.spelling == *declaration
                    })
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
                    let root = choose_type_root(name, choices)?;
                    if root_names.insert(name.to_string()) {
                        type_roots.push(root);
                    }
                }
                constants.retain(|constant| !root_names.contains(constant.name.as_str()));
                continue;
            }

            let mut facts_by_name = BTreeMap::new();
            for (name, choices) in grouped {
                facts_by_name.insert(name, choose_type_root(name, &choices)?);
            }
            break facts_by_name;
        };

        let mut required = root_names;
        for constant in &constants {
            insert_required_type_names(&constant.ty, &mut required);
        }
        let mut queue: Vec<_> = required.iter().rev().cloned().collect();
        while let Some(name) = queue.pop() {
            let Some(fact) = facts_by_name.get(name.as_str()) else {
                return Err(Error(format!("planned type `{name}` is not emittable")));
            };
            let mut dependencies = BTreeSet::new();
            match &fact.data {
                FactData::Typedef { target } => {
                    insert_required_type_names(target, &mut dependencies);
                }
                FactData::Record { fields, .. } => {
                    for field in fields {
                        insert_required_type_names(&field.ty, &mut dependencies);
                    }
                }
                _ => {}
            }
            for dependency in dependencies {
                if required.insert(dependency.clone()) {
                    queue.push(dependency);
                }
            }
        }

        let mut enum_alias_candidates: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for fact in &self.facts {
            if let FactData::Typedef {
                target: TypeRef::Named { name, declaration },
            } = &fact.data
                && name != &fact.name
                && let Some(selected) = facts_by_name.get(name.as_str())
                && self.facts.iter().any(|target| {
                    target.origin.tu == fact.origin.tu
                        && target.name == *name
                        && target.spelling == *declaration
                        && target.spelling.file == fact.spelling.file
                        && target.definition
                        && matches!(target.data, FactData::Enum { .. } | FactData::Record { .. })
                        && same_source_declaration(selected, target)
                })
            {
                enum_alias_candidates
                    .entry(selected.name.as_str())
                    .or_default()
                    .push(fact.name.as_str());
            }
        }
        let type_names: BTreeMap<_, _> = enum_alias_candidates
            .into_iter()
            .filter_map(|(target, aliases)| {
                let aliases: BTreeSet<_> = aliases.into_iter().collect();
                if aliases.len() != 1 {
                    return None;
                }
                Some((target.to_string(), (*aliases.first().unwrap()).to_string()))
            })
            .collect();
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
        constants.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(Plan {
            types,
            constants,
            type_names,
        })
    }
}

struct PlannedType<'a> {
    fact: &'a Fact,
    name: String,
}

struct Plan<'a> {
    types: Vec<PlannedType<'a>>,
    constants: Vec<&'a Constant>,
    type_names: BTreeMap<String, String>,
}

fn is_type_fact(fact: &Fact) -> bool {
    matches!(
        fact.data,
        FactData::Enum { .. } | FactData::Record { .. } | FactData::Typedef { .. }
    )
}

fn is_root_type_fact(fact: &Fact) -> bool {
    is_type_fact(fact) && (!matches!(fact.data, FactData::Record { .. }) || fact.definition)
}

fn choose_type_root<'a>(name: &str, roots: &[&'a Fact]) -> Result<&'a Fact, Error> {
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
    let definitions: Vec<_> = distinct
        .iter()
        .copied()
        .filter(|fact| {
            matches!(fact.data, FactData::Enum { .. } | FactData::Record { .. }) && fact.definition
        })
        .collect();
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
                    && fact.parent == root.parent
                    && matches!(
                        (&fact.data, &root.data),
                        (FactData::Enum { .. }, FactData::Enum { .. })
                            | (FactData::Record { .. }, FactData::Record { .. })
                    ))
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

fn same_source_declaration(left: &Fact, right: &Fact) -> bool {
    left.kind == right.kind
        && left.name == right.name
        && left.spelling == right.spelling
        && left.definition == right.definition
        && left.data == right.data
}

fn emittable_type<'a>(name: &str, fact: &'a Fact) -> Result<&'a Fact, Error> {
    match fact.data {
        FactData::Typedef { .. } | FactData::Enum { .. } if fact.definition => Ok(fact),
        FactData::Record { .. } => Ok(fact),
        _ => Err(Error(format!("type root `{name}` is not emittable"))),
    }
}

fn queue_type_edges<'a>(fact: &'a Fact, queue: &mut Vec<(&'a str, &'a TypeRef)>) {
    if let FactData::Typedef { target } = &fact.data {
        queue.push((fact.origin.tu.as_str(), target));
    } else if let FactData::Record { fields, .. } = &fact.data {
        for field in fields {
            queue.push((fact.origin.tu.as_str(), &field.ty));
        }
    }
}

fn insert_required_type_names(ty: &TypeRef, required: &mut BTreeSet<String>) {
    match ty {
        TypeRef::Named { name, .. } => {
            required.insert(name.clone());
        }
        TypeRef::Pointer { target, .. } => insert_required_type_names(target, required),
        TypeRef::Array { target, .. } => insert_required_type_names(target, required),
        _ => {}
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
    let mut translation_units = Vec::with_capacity(inputs.len());
    for input in &inputs {
        translation_units.push((
            input.name.clone(),
            TranslationUnit::parse(&index, input, args)?,
        ));
    }

    let mut facts = vec![];
    for (name, translation_unit) in &translation_units {
        translation_unit.extract(name, &mut facts);
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
    let mut constants = vec![];
    for input in &inputs {
        constants.extend(evaluate_constants(&index, input, args, &facts)?);
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

    fn extract(&self, tu: &str, facts: &mut Vec<Fact>) {
        let mut traversal = Traversal {
            tu,
            next: 0,
            seen: HashMap::new(),
            facts,
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
    next: u32,
    seen: HashMap<u32, Vec<(CXCursor, Origin)>>,
    facts: &'a mut Vec<Fact>,
}

fn extract_children(cursor: CXCursor, parent: Option<Origin>, traversal: &mut Traversal<'_>) {
    for child in cursor_children(cursor) {
        let local = traversal.next;
        traversal.next += 1;
        let kind = unsafe { clang_getCursorKind(child) };
        let mut child_parent = parent.clone();
        let mut repeated = false;

        if let Some(fact_kind) = fact_kind(kind) {
            let name = cx_string(unsafe { clang_getCursorSpelling(child) });
            if !name.is_empty() {
                let hash = unsafe { clang_hashCursor(child) };
                let seen = traversal.seen.entry(hash).or_default();
                if let Some((_, origin)) = seen
                    .iter()
                    .find(|(cursor, _)| unsafe { clang_equalCursors(*cursor, child) } != 0)
                {
                    child_parent = Some(origin.clone());
                    repeated = true;
                } else if let Some((spelling, expansion, main_file, system)) =
                    cursor_locations(child)
                {
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
                        system,
                        data: fact_data(child, fact_kind),
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

fn evaluate_constants(
    index: &Index,
    input: &Input,
    args: &[&str],
    facts: &[Fact],
) -> Result<Vec<Constant>, Error> {
    let mut roots = BTreeMap::new();
    for fact in facts
        .iter()
        .filter(|fact| fact.origin.tu == input.name && fact.main_file)
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
    let (mut evaluated, reached) = evaluate_probe(index, input, args, &names)?;
    let missing: Vec<_> = names
        .iter()
        .filter(|name| !reached.contains(name.as_str()))
        .cloned()
        .collect();
    for name in missing {
        evaluated.extend(evaluate_probe(index, input, args, &[name])?.0);
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

fn fact_data(cursor: CXCursor, kind: FactKind) -> FactData {
    match kind {
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
        FactKind::Macro => FactData::Macro {
            function_like: unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0,
        },
        FactKind::Struct | FactKind::Union => {
            let definition = unsafe { clang_isCursorDefinition(cursor) } != 0;
            let fields = if definition {
                let mut fields = vec![];
                for child in cursor_children(cursor)
                    .into_iter()
                    .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_FieldDecl)
                {
                    let name = cx_string(unsafe { clang_getCursorSpelling(child) });
                    let field_ty = unsafe { clang_getCursorType(child) };
                    let Some(ty) = type_ref(field_ty) else {
                        return FactData::Unsupported {
                            reason: format!(
                                "field `{name}` has unsupported type `{}`",
                                cx_string(unsafe { clang_getTypeSpelling(field_ty) })
                            ),
                        };
                    };
                    fields.push(Field {
                        name,
                        ty,
                        offset: unsafe { clang_Cursor_getOffsetOfField(child) },
                        align: unsafe { clang_Type_getAlignOf(field_ty) },
                        size: unsafe { clang_Type_getSizeOf(field_ty) },
                    });
                }
                fields
            } else {
                vec![]
            };
            let ty = unsafe { clang_getCursorType(cursor) };
            let size = unsafe { clang_Type_getSizeOf(ty) };
            let align = unsafe { clang_Type_getAlignOf(ty) };
            let union = kind == FactKind::Union;
            let (packing, alignment) = if definition {
                match record_layout(&fields, size, align, union) {
                    Ok(layout) => layout,
                    Err(reason) => return FactData::Unsupported { reason },
                }
            } else {
                (None, None)
            };
            FactData::Record {
                fields,
                size,
                align,
                packing,
                alignment,
                union,
            }
        }
        FactKind::Typedef => {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
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
        for field in fields {
            let field_align = packing.map_or(field.align, |packing| packing.min(field.align));
            natural_align = natural_align.max(field_align);
            let offset = if union {
                0
            } else {
                align_up(cursor, field_align)
            };
            if offset * 8 != field.offset {
                matches = false;
                break;
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
        let content_size = if fields.is_empty() { size } else { cursor };
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
    if ty.kind == CXType_Void {
        return Some(TypeRef::Void);
    }
    if ty.kind == CXType_Pointer {
        let pointee = unsafe { clang_getPointeeType(ty) };
        return Some(TypeRef::Pointer {
            mutable: unsafe { clang_isConstQualifiedType(pointee) } == 0,
            target: Box::new(type_ref(pointee)?),
        });
    }
    if ty.kind == CXType_ConstantArray {
        let target = unsafe { clang_getArrayElementType(ty) };
        let len = unsafe { clang_getArraySize(ty) };
        if len < 0 {
            return None;
        }
        return Some(TypeRef::Array {
            target: Box::new(type_ref(target)?),
            len: len.try_into().unwrap(),
        });
    }
    let declaration = unsafe { clang_getTypeDeclaration(ty) };
    if unsafe { clang_Cursor_isNull(declaration) } == 0 {
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

fn type_name(ty: &TypeRef) -> String {
    match ty {
        TypeRef::Void => "void".to_string(),
        TypeRef::Scalar(scalar) => scalar_name(*scalar).to_string(),
        TypeRef::Named { name, .. } => name.clone(),
        TypeRef::Pointer { .. } => {
            let (mutable, depth, target) = pointer_run(ty);
            format!(
                "{}{}",
                format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth),
                type_name(target)
            )
        }
        TypeRef::Array { target, len } => format!("[{}; {len}]", type_name(target)),
    }
}

fn planned_type_name(ty: &TypeRef, type_names: &BTreeMap<String, String>) -> String {
    match ty {
        TypeRef::Named { name, .. } => type_names.get(name).unwrap_or(name).clone(),
        TypeRef::Pointer { .. } => {
            let (mutable, depth, target) = pointer_run(ty);
            format!(
                "{}{}",
                format!("*{} ", if mutable { "mut" } else { "const" }).repeat(depth),
                planned_type_name(target, type_names)
            )
        }
        TypeRef::Array { target, len } => {
            format!("[{}; {len}]", planned_type_name(target, type_names))
        }
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
        _ => planned_type_name(ty, type_names),
    }
}

fn value_name(value: &Value) -> String {
    match value {
        Value::Signed(value) => value.to_string(),
        Value::Unsigned(value) => value.to_string(),
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
