use super::*;
use clang_sys::*;
use std::cell::OnceCell;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

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
    let timing = std::env::var_os("WINDOWS_CLANG_TIMING").is_some();
    let parse_time = std::time::Instant::now();
    let mut translation_units = Vec::with_capacity(inputs.len());
    for input in &inputs {
        translation_units.push((
            input.name.clone(),
            TranslationUnit::parse(&index, input, args)?,
        ));
    }
    if timing {
        eprintln!("clang parse: {:.2}s", parse_time.elapsed().as_secs_f32());
    }

    let traversal_time = std::time::Instant::now();
    let mut facts = vec![];
    let mut constants = vec![];
    let mut extracted = vec![];
    for (name, translation_unit) in &translation_units {
        let input = inputs.iter().find(|input| input.name == *name).unwrap();
        extracted.push(translation_unit.extract(input, &mut facts, &mut constants));
    }
    decode_root_macro_definitions(&mut facts, &extracted);
    if timing {
        eprintln!(
            "clang traversal: {:.2}s",
            traversal_time.elapsed().as_secs_f32()
        );
    }
    let constant_time = std::time::Instant::now();
    let phase_time = std::time::Instant::now();
    for (input, extracted) in inputs.iter().zip(&extracted) {
        constants.extend(evaluate_constants(
            &index,
            input,
            args,
            &facts,
            &extracted.macros,
        )?);
    }
    if timing {
        eprintln!(
            "clang constant evaluation: {:.2}s",
            phase_time.elapsed().as_secs_f32()
        );
    }
    let phase_time = std::time::Instant::now();
    decode_reachable_structs(&mut facts, &constants, &extracted);
    if timing {
        eprintln!(
            "clang deferred records: {:.2}s",
            phase_time.elapsed().as_secs_f32()
        );
    }
    let phase_time = std::time::Instant::now();
    apply_macro_enum_overrides(&mut facts, &mut constants);
    if timing {
        eprintln!(
            "clang macro overrides: {:.2}s",
            phase_time.elapsed().as_secs_f32()
        );
    }
    let phase_time = std::time::Instant::now();
    recover_midl_artifacts(&inputs, &mut facts, &mut constants);
    if timing {
        eprintln!(
            "clang MIDL recovery: {:.2}s",
            phase_time.elapsed().as_secs_f32()
        );
    }
    if timing {
        eprintln!(
            "clang constants: {:.2}s",
            constant_time.elapsed().as_secs_f32()
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
    constants.sort();
    Ok(Snapshot { facts, constants })
}

fn apply_macro_enum_overrides(facts: &mut [Fact], constants: &mut Vec<Constant>) {
    let macro_origins: HashSet<_> = facts
        .iter()
        .filter_map(|fact| {
            matches!(fact.data, FactData::Macro { .. })
                .then_some((fact.origin.tu.as_str(), fact.origin.local))
        })
        .collect();
    let mut enum_members: HashMap<_, Vec<_>> = HashMap::new();
    for (fact_index, fact) in facts.iter().enumerate() {
        let FactData::Enum { repr, variants, .. } = &fact.data else {
            continue;
        };
        for (variant_index, variant) in variants.iter().enumerate() {
            enum_members
                .entry((
                    fact.origin.tu.as_str(),
                    fact.spelling.file.as_str(),
                    variant.name.as_str(),
                ))
                .or_default()
                .push((fact_index, variant_index, *repr));
        }
    }

    let mut overrides = vec![];
    for constant in constants.iter() {
        if !macro_origins.contains(&(constant.definition.tu.as_str(), constant.definition.local)) {
            continue;
        }
        let candidates: Vec<_> = enum_members
            .get(&(
                constant.root.tu.as_str(),
                constant.spelling.file.as_str(),
                constant.name.as_str(),
            ))
            .into_iter()
            .flatten()
            .filter_map(|(fact_index, variant_index, repr)| {
                enum_override_value(&constant.value, *repr)
                    .map(|value| (*fact_index, *variant_index, value))
            })
            .collect();
        if let [(fact_index, variant_index, value)] = candidates.as_slice() {
            overrides.push((
                constant.definition.clone(),
                constant.name.clone(),
                *fact_index,
                *variant_index,
                *value,
            ));
        }
    }
    drop(enum_members);
    drop(macro_origins);
    for (_, _, fact_index, variant_index, value) in &overrides {
        let FactData::Enum { variants, .. } = &mut facts[*fact_index].data else {
            unreachable!()
        };
        variants[*variant_index].value = *value;
    }
    let overridden: HashSet<_> = overrides
        .iter()
        .map(|(definition, name, ..)| (definition, name.as_str()))
        .collect();
    constants
        .retain(|constant| !overridden.contains(&(&constant.definition, constant.name.as_str())));
}

fn enum_override_value(value: &Value, repr: Scalar) -> Option<i64> {
    match (value, repr) {
        (Value::Signed(value), Scalar::I8) => i8::try_from(*value).ok().map(i64::from),
        (Value::Signed(value), Scalar::I16) => i16::try_from(*value).ok().map(i64::from),
        (Value::Signed(value), Scalar::I32) => i32::try_from(*value).ok().map(i64::from),
        (Value::Signed(value), Scalar::I64) => Some(*value),
        (Value::Unsigned(value), Scalar::U8) => u8::try_from(*value).ok().map(i64::from),
        (Value::Unsigned(value), Scalar::U16) => u16::try_from(*value).ok().map(i64::from),
        (Value::Unsigned(value), Scalar::U32) => u32::try_from(*value).ok().map(i64::from),
        (Value::Unsigned(value), Scalar::U64) => Some(*value as i64),
        _ => None,
    }
}

fn recover_midl_artifacts(inputs: &[Input], facts: &mut [Fact], constants: &mut Vec<Constant>) {
    let mut sources = HashMap::new();
    let reference_counts = reference_counts(facts, constants);
    let mut named_typedefs = HashSet::new();
    let mut typedefs_by_file: HashMap<(String, String), Vec<usize>> = HashMap::new();
    for (index, fact) in facts.iter().enumerate() {
        let FactData::Typedef { target } = &fact.data else {
            continue;
        };
        if let TypeRef::Named { name, declaration } = target {
            named_typedefs.insert((fact.origin.tu.clone(), name.clone(), declaration.clone()));
        }
        if fact.root {
            typedefs_by_file
                .entry((fact.origin.tu.clone(), fact.spelling.file.clone()))
                .or_default()
                .push(index);
        }
    }
    for typedefs in typedefs_by_file.values_mut() {
        typedefs.sort_by_key(|index| facts[*index].spelling.offset);
    }

    let mut enum_recoveries = vec![];
    for (enum_index, enum_fact) in facts.iter().enumerate() {
        let FactData::Enum { repr, variants, .. } = &enum_fact.data else {
            continue;
        };
        let generated_name = midl_generated_name(&enum_fact.name);
        let private_alias_name = midl_private_alias_name(&enum_fact.name);
        if !enum_fact.root || (!generated_name && private_alias_name.is_none()) {
            continue;
        }
        if named_typedefs.contains(&(
            enum_fact.origin.tu.clone(),
            enum_fact.name.clone(),
            enum_fact.spelling.clone(),
        )) {
            continue;
        }
        if reference_counts
            .get(&(enum_fact.origin.tu.clone(), enum_fact.name.clone()))
            .copied()
            .unwrap_or_default()
            != 0
        {
            continue;
        }
        let adjacent_alias = typedefs_by_file
            .get(&(enum_fact.origin.tu.clone(), enum_fact.spelling.file.clone()))
            .and_then(|typedefs| {
                typedefs.iter().find_map(|index| {
                    if facts[*index].spelling.offset <= enum_fact.spelling.offset {
                        return None;
                    }
                    scalar_type(&facts[*index].data, &facts[*index].origin.tu, facts)
                        .map(|scalar| (*index, scalar))
                })
            })
            .and_then(|(index, scalar)| {
                let fact = &facts[index];
                (adjacent_midl_enum_alias(inputs, &mut sources, enum_fact, fact)
                    && (generated_name
                        || private_alias_name.is_some_and(|name| {
                            name == fact.name
                                && midl_generated_header(inputs, &sources, &enum_fact.spelling.file)
                        }))
                    && variants.iter().all(|variant| {
                        value_for_midl_scalar(variant.value, *repr, scalar).is_some()
                    }))
                .then_some(index)
            });
        if !generated_name && adjacent_alias.is_none() {
            continue;
        }
        enum_recoveries.push((
            enum_index,
            adjacent_alias,
            *repr,
            variants.clone(),
            enum_fact.origin.clone(),
            enum_fact.spelling.clone(),
        ));
    }
    for (enum_index, alias_index, enum_repr, variants, origin, spelling) in enum_recoveries {
        let (ty, scalar) = if let Some(alias_index) = alias_index {
            (
                TypeRef::Named {
                    name: facts[alias_index].name.clone(),
                    declaration: facts[alias_index].spelling.clone(),
                },
                scalar_type(
                    &facts[alias_index].data,
                    &facts[alias_index].origin.tu,
                    facts,
                )
                .unwrap(),
            )
        } else {
            (TypeRef::Scalar(enum_repr), enum_repr)
        };
        facts[enum_index].root = false;
        constants.extend(variants.into_iter().map(|variant| Constant {
            root: origin.clone(),
            definition: origin.clone(),
            spelling: spelling.clone(),
            name: variant.name,
            ty: ty.clone(),
            value: value_for_midl_scalar(variant.value, enum_repr, scalar).unwrap(),
        }));
    }

    let mut opaque_recoveries = vec![];
    for (alias_index, alias_fact) in facts.iter().enumerate() {
        let FactData::Typedef {
            target:
                TypeRef::Pointer {
                    mutable,
                    target: pointee,
                },
        } = &alias_fact.data
        else {
            continue;
        };
        let TypeRef::Named { name, declaration } = pointee.as_ref() else {
            continue;
        };
        if !alias_fact.root || !midl_generated_name(name) {
            continue;
        }
        let Some((record_index, record_fact)) = facts.iter().enumerate().find(|(_, candidate)| {
            candidate.origin.tu == alias_fact.origin.tu
                && candidate.spelling == *declaration
                && matches!(
                    &candidate.data,
                    FactData::Record { fields, union: false, .. }
                        if matches!(fields.as_slice(), [Field { name, .. }] if name == "_")
                )
        }) else {
            continue;
        };
        let references = reference_counts
            .get(&(alias_fact.origin.tu.clone(), name.clone()))
            .copied()
            .unwrap_or_default();
        if references != 1
            || !midl_record_pointer_alias(inputs, &mut sources, record_fact, alias_fact)
        {
            continue;
        }
        opaque_recoveries.push((alias_index, record_index, *mutable, alias_fact.name.clone()));
    }
    for (alias_index, record_index, mutable, alias) in opaque_recoveries {
        facts[record_index].root = false;
        let FactData::Typedef { target } = &mut facts[alias_index].data else {
            unreachable!()
        };
        *target = TypeRef::OpaquePointer {
            mutable,
            tag: alias,
        };
    }
}

fn midl_generated_name(name: &str) -> bool {
    name.starts_with("__MIDL")
}

fn midl_private_alias_name(name: &str) -> Option<&str> {
    name.strip_prefix('_')
        .filter(|name| !name.is_empty() && !name.starts_with('_'))
}

fn midl_generated_header(inputs: &[Input], sources: &HashMap<String, String>, file: &str) -> bool {
    inputs
        .iter()
        .find(|input| input.name == file)
        .map(|input| input.source.as_str())
        .or_else(|| sources.get(file).map(String::as_str))
        .is_some_and(|source| {
            source
                .lines()
                .take(10)
                .any(|line| line.contains("File created by MIDL compiler version"))
        })
}

fn input_segment(
    inputs: &[Input],
    sources: &mut HashMap<String, String>,
    start: &Location,
    end: &Location,
) -> Option<String> {
    if start.file != end.file || start.offset >= end.offset {
        return None;
    }
    if let Some(input) = inputs.iter().find(|input| input.name == start.file) {
        return input
            .source
            .get(start.offset as usize..end.offset as usize)
            .map(str::to_string);
    }
    if !sources.contains_key(&start.file) {
        sources.insert(
            start.file.clone(),
            std::fs::read_to_string(&start.file).ok()?,
        );
    }
    sources
        .get(&start.file)?
        .get(start.offset as usize..end.offset as usize)
        .map(str::to_string)
}

fn adjacent_midl_enum_alias(
    inputs: &[Input],
    sources: &mut HashMap<String, String>,
    enum_fact: &Fact,
    alias_fact: &Fact,
) -> bool {
    let Some(source) = input_segment(inputs, sources, &enum_fact.spelling, &alias_fact.spelling)
    else {
        return false;
    };
    let compact: String = source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    let declaration = compact
        .strip_prefix(&enum_fact.name)
        .or_else(|| compact.strip_prefix(&format!("enum{}", enum_fact.name)));
    let Some(declaration) = declaration else {
        return false;
    };
    declaration.starts_with('{')
        && declaration
            .rfind('}')
            .is_some_and(|end| declaration[end..].starts_with("};typedef"))
        && declaration.matches('{').count() == 1
        && declaration.matches('}').count() == 1
}

fn midl_record_pointer_alias(
    inputs: &[Input],
    sources: &mut HashMap<String, String>,
    record: &Fact,
    alias: &Fact,
) -> bool {
    let Some(source) = input_segment(inputs, sources, &record.spelling, &alias.spelling) else {
        return false;
    };
    let compact: String = source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    let declaration = compact
        .strip_prefix(&record.name)
        .or_else(|| compact.strip_prefix(&format!("struct{}", record.name)));
    declaration.is_some_and(|declaration| {
        declaration.starts_with('{')
            && declaration.ends_with("}*")
            && declaration.matches('{').count() == 1
            && declaration.matches('}').count() == 1
    })
}

fn scalar_type(data: &FactData, tu: &str, facts: &[Fact]) -> Option<Scalar> {
    fn resolve(
        ty: &TypeRef,
        tu: &str,
        facts: &[Fact],
        seen: &mut HashSet<Location>,
    ) -> Option<Scalar> {
        match ty {
            TypeRef::Scalar(scalar) => Some(*scalar),
            TypeRef::Named { declaration, .. } if seen.insert(declaration.clone()) => {
                let fact = facts
                    .iter()
                    .find(|fact| fact.origin.tu == tu && fact.spelling == *declaration)?;
                match &fact.data {
                    FactData::Typedef { target } => resolve(target, tu, facts, seen),
                    FactData::Enum { repr, .. } => Some(*repr),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    let FactData::Typedef { target } = data else {
        return None;
    };
    match resolve(target, tu, facts, &mut HashSet::new())? {
        scalar @ (Scalar::I8
        | Scalar::U8
        | Scalar::I16
        | Scalar::U16
        | Scalar::I32
        | Scalar::U32
        | Scalar::I64
        | Scalar::U64) => Some(scalar),
        Scalar::Bool | Scalar::F32 | Scalar::F64 => None,
    }
}

fn value_for_scalar(value: i64, scalar: Scalar) -> Option<Value> {
    match scalar {
        Scalar::U8 => u8::try_from(value)
            .ok()
            .map(|value| Value::Unsigned(u64::from(value))),
        Scalar::U16 => u16::try_from(value)
            .ok()
            .map(|value| Value::Unsigned(u64::from(value))),
        Scalar::U32 => u32::try_from(value)
            .ok()
            .map(|value| Value::Unsigned(u64::from(value))),
        Scalar::U64 => u64::try_from(value).ok().map(Value::Unsigned),
        Scalar::I8 => i8::try_from(value)
            .ok()
            .map(|value| Value::Signed(i64::from(value))),
        Scalar::I16 => i16::try_from(value)
            .ok()
            .map(|value| Value::Signed(i64::from(value))),
        Scalar::I32 => i32::try_from(value)
            .ok()
            .map(|value| Value::Signed(i64::from(value))),
        Scalar::I64 => Some(Value::Signed(value)),
        Scalar::Bool | Scalar::F32 | Scalar::F64 => None,
    }
}

fn value_for_midl_scalar(value: i64, source: Scalar, target: Scalar) -> Option<Value> {
    value_for_scalar(value, target).or_else(|| match (source, target) {
        (Scalar::I8, Scalar::U8) => Some(Value::Unsigned(u64::from(value as u8))),
        (Scalar::I16, Scalar::U16) => Some(Value::Unsigned(u64::from(value as u16))),
        (Scalar::I32, Scalar::U32) => Some(Value::Unsigned(u64::from(value as u32))),
        (Scalar::I64, Scalar::U64) => Some(Value::Unsigned(value as u64)),
        (Scalar::U8, Scalar::I8) => Some(Value::Signed(i64::from(value as i8))),
        (Scalar::U16, Scalar::I16) => Some(Value::Signed(i64::from(value as i16))),
        (Scalar::U32, Scalar::I32) => Some(Value::Signed(i64::from(value as i32))),
        (Scalar::U64, Scalar::I64) => Some(Value::Signed(value)),
        _ => None,
    })
}

fn reference_counts(facts: &[Fact], constants: &[Constant]) -> HashMap<(String, String), usize> {
    let mut result = HashMap::new();
    for fact in facts {
        let mut names = HashSet::new();
        fact_type_names(&fact.data, &mut names);
        for name in names {
            *result.entry((fact.origin.tu.clone(), name)).or_default() += 1;
        }
    }
    for constant in constants {
        let mut names = HashSet::new();
        type_names(&constant.ty, &mut names);
        for name in names {
            *result.entry((constant.root.tu.clone(), name)).or_default() += 1;
        }
    }
    result
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
        let synthetic_name = format!("{}.__clang_eval.cpp", input.name);
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

    fn extract<'tu>(
        &'tu self,
        input: &Input,
        facts: &mut Vec<Fact>,
        constants: &mut Vec<Constant>,
    ) -> Extracted<'tu> {
        let timing = std::env::var_os("WINDOWS_CLANG_TIMING").is_some();
        let phase_time = std::time::Instant::now();
        let macros = macro_definitions(self, unsafe { clang_getTranslationUnitCursor(self.0) });
        if timing {
            eprintln!(
                "clang macro index: {} definitions, {} expansion files, {:.2}s",
                macros.definitions.len(),
                macros.expansion_orders.len(),
                phase_time.elapsed().as_secs_f32()
            );
        }
        let phase_time = std::time::Instant::now();
        let initial_facts = facts.len();
        let mut traversal = Traversal {
            tu: &input.name,
            roots: &input.roots,
            root_dirs: &input.root_dirs,
            root_suffixes: &input.root_suffixes,
            excluded_roots: &input.excluded_roots,
            next: 0,
            seen: HashMap::new(),
            macros: &macros,
            pending_structs: vec![],
            pending_macros: vec![],
            facts,
            constants,
        };
        extract_children(
            unsafe { clang_getTranslationUnitCursor(self.0) },
            None,
            &mut traversal,
        );
        if timing {
            eprintln!(
                "clang declarations: {} cursors, {} facts, {:.2}s",
                traversal.next,
                traversal.facts.len() - initial_facts,
                phase_time.elapsed().as_secs_f32()
            );
        }
        let pending_structs = std::mem::take(&mut traversal.pending_structs);
        let pending_macros = std::mem::take(&mut traversal.pending_macros);
        drop(traversal);
        Extracted {
            macros,
            pending_structs,
            pending_macros,
        }
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
    root_dirs: &'a BTreeSet<String>,
    root_suffixes: &'a BTreeSet<String>,
    excluded_roots: &'a BTreeSet<String>,
    next: u32,
    seen: HashMap<u32, Vec<(CXCursor, Origin)>>,
    macros: &'a MacroDefinitions<'a>,
    pending_structs: Vec<(usize, CXCursor)>,
    pending_macros: Vec<(usize, CXCursor)>,
    facts: &'a mut Vec<Fact>,
    constants: &'a mut Vec<Constant>,
}

struct Extracted<'tu> {
    macros: MacroDefinitions<'tu>,
    pending_structs: Vec<(usize, CXCursor)>,
    pending_macros: Vec<(usize, CXCursor)>,
}

impl Traversal<'_> {
    fn is_root(&self, file: &str) -> bool {
        !self.excluded_roots.contains(file)
            && (self.roots.contains(file)
                || self.root_dirs.iter().any(|root| file.starts_with(root))
                || self.root_suffixes.iter().any(|root| {
                    file == root
                        || file
                            .strip_suffix(root)
                            .is_some_and(|prefix| prefix.ends_with('/'))
                }))
    }
}

fn extract_children(cursor: CXCursor, parent: Option<&Origin>, traversal: &mut Traversal<'_>) {
    struct Visit<'parent, 'traversal, 'facts> {
        parent: Option<&'parent Origin>,
        traversal: &'traversal mut Traversal<'facts>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }

    extern "C" fn visit(
        cursor: CXCursor,
        _parent: CXCursor,
        data: CXClientData,
    ) -> CXChildVisitResult {
        let visit = unsafe { &mut *(data as *mut Visit<'_, '_, '_>) };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            extract_child(cursor, visit.parent, visit.traversal);
        })) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                visit.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }

    let mut state = Visit {
        parent,
        traversal,
        panic: None,
    };
    unsafe {
        clang_visitChildren(cursor, visit, &mut state as *mut _ as CXClientData);
    }
    if let Some(panic) = state.panic {
        std::panic::resume_unwind(panic);
    }
}

fn extract_child(child: CXCursor, parent: Option<&Origin>, traversal: &mut Traversal<'_>) {
    let local = traversal.next;
    traversal.next += 1;
    let kind = unsafe { clang_getCursorKind(child) };
    let mut child_parent = None;
    let mut repeated = false;

    let mut name = if matches!(
        kind,
        CXCursor_FunctionDecl
            | CXCursor_VarDecl
            | CXCursor_MacroExpansion
            | CXCursor_ClassDecl
            | CXCursor_EnumDecl
            | CXCursor_MacroDefinition
            | CXCursor_Namespace
            | CXCursor_StructDecl
            | CXCursor_TypedefDecl
            | CXCursor_UnionDecl
    ) {
        cx_string(unsafe { clang_getCursorSpelling(child) })
    } else {
        String::new()
    };
    if kind == CXCursor_FunctionDecl
        && let Some(source_name) = source_function_name(child, &name, traversal.macros)
    {
        name = source_name;
    }
    if kind == CXCursor_VarDecl
        && !name.is_empty()
        && let Some((spelling, _, _, _)) = cursor_locations(child)
        && traversal.is_root(&spelling.file)
    {
        let ty = unsafe { clang_getCursorType(child) };
        if unsafe { clang_isConstQualifiedType(ty) } != 0
            && let Some(scalar @ (Scalar::F32 | Scalar::F64)) = scalar(ty)
            && let Some(value) = evaluate_float(child, scalar)
        {
            let origin = Origin {
                tu: traversal.tu.to_string(),
                local,
            };
            traversal.constants.push(Constant {
                root: origin.clone(),
                definition: origin,
                spelling,
                name: name.clone(),
                ty: TypeRef::Scalar(scalar),
                value,
            });
        }
    }
    let anonymous_enum =
        kind == CXCursor_EnumDecl && unsafe { clang_Cursor_isAnonymous(child) } != 0;
    if anonymous_enum
        && let Some((spelling, _, _, _)) = cursor_locations(child)
        && traversal.is_root(&spelling.file)
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
                let value = if matches!(repr, Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::U64)
                {
                    Value::Unsigned(unsafe { clang_getEnumConstantDeclUnsignedValue(constant) })
                } else {
                    Value::Signed(unsafe { clang_getEnumConstantDeclValue(constant) })
                };
                traversal.constants.push(Constant {
                    root: origin.clone(),
                    definition: origin.clone(),
                    spelling: spelling.clone(),
                    name: cx_string(unsafe { clang_getCursorSpelling(constant) }),
                    ty: TypeRef::Scalar(repr),
                    value,
                });
            }
        }
    }
    let fact_kind = if kind == CXCursor_MacroExpansion && name == "DEFINE_ENUM_FLAG_OPERATORS" {
        Some(FactKind::EnumFlag)
    } else if kind == CXCursor_MacroExpansion
        && matches!(
            name.as_str(),
            "DEFINE_GUID" | "DEFINE_OLEGUID" | "DEFINE_KNOWN_FOLDER"
        )
    {
        let ole = name == "DEFINE_OLEGUID";
        parse_define_guid_tokens(&cursor_tokens(child), ole).map(|(guid_name, _)| {
            name = guid_name;
            FactKind::Guid
        })
    } else if kind == CXCursor_MacroExpansion
        && matches!(name.as_str(), "DEFINE_PROPERTYKEY" | "DEFINE_DEVPROPKEY")
    {
        parse_property_key_tokens(&cursor_tokens(child)).map(|(key_name, _, _)| {
            name = key_name;
            FactKind::Guid
        })
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
                let root = !traversal.excluded_roots.contains(&spelling.file)
                    && (traversal.roots.contains(&spelling.file)
                        || traversal
                            .root_dirs
                            .iter()
                            .any(|root| spelling.file.starts_with(root))
                        || traversal.root_suffixes.iter().any(|root| {
                            spelling.file == *root
                                || spelling
                                    .file
                                    .strip_suffix(root)
                                    .is_some_and(|prefix| prefix.ends_with('/'))
                        }));
                let origin = Origin {
                    tu: traversal.tu.to_string(),
                    local,
                };
                seen.push((child, origin.clone()));
                child_parent = Some(origin.clone());
                if root || !matches!(fact_kind, FactKind::Function | FactKind::Guid) {
                    if fact_kind == FactKind::Macro
                        && unsafe { clang_Cursor_isMacroFunctionLike(child) } != 0
                    {
                        return;
                    }
                    let deferred_struct = !root && fact_kind == FactKind::Struct;
                    let deferred_macro = !root && fact_kind == FactKind::Macro;
                    let data = if deferred_struct || deferred_macro {
                        FactData::None
                    } else {
                        fact_data(child, fact_kind, traversal.macros)
                    };
                    let index = traversal.facts.len();
                    traversal.facts.push(Fact {
                        origin,
                        parent: parent.cloned(),
                        kind: fact_kind,
                        name,
                        spelling,
                        expansion,
                        definition: unsafe { clang_isCursorDefinition(child) } != 0,
                        main_file,
                        root,
                        system,
                        data,
                    });
                    if deferred_struct {
                        traversal.pending_structs.push((index, child));
                    }
                    if deferred_macro {
                        traversal.pending_macros.push((index, child));
                    }
                }
            }
        }
    }

    if !repeated {
        extract_children(child, child_parent.as_ref().or(parent), traversal);
    }
}

fn decode_reachable_structs(
    facts: &mut [Fact],
    constants: &[Constant],
    extracted: &[Extracted<'_>],
) {
    let mut reachable: HashSet<String> = facts
        .iter()
        .filter(|fact| fact.root)
        .map(|fact| fact.name.clone())
        .collect();
    for fact in facts.iter() {
        fact_type_names(&fact.data, &mut reachable);
    }
    for constant in constants {
        type_names(&constant.ty, &mut reachable);
    }
    let mut pending: HashMap<String, Vec<(usize, usize, CXCursor)>> = HashMap::new();
    for (extraction_index, extraction) in extracted.iter().enumerate() {
        for &(fact_index, cursor) in &extraction.pending_structs {
            pending
                .entry(facts[fact_index].name.clone())
                .or_default()
                .push((extraction_index, fact_index, cursor));
        }
    }
    let mut queue: Vec<_> = reachable.iter().cloned().collect();
    while let Some(name) = queue.pop() {
        let Some(candidates) = pending.remove(&name) else {
            continue;
        };
        for (extraction_index, fact_index, cursor) in candidates {
            let data = fact_data(
                cursor,
                FactKind::Struct,
                &extracted[extraction_index].macros,
            );
            let mut dependencies = HashSet::new();
            fact_type_names(&data, &mut dependencies);
            for dependency in dependencies {
                if reachable.insert(dependency.clone()) {
                    queue.push(dependency);
                }
            }
            facts[fact_index].data = data;
        }
    }
}

fn decode_root_macro_definitions(facts: &mut [Fact], extracted: &[Extracted<'_>]) {
    let roots: HashSet<_> = facts
        .iter()
        .filter(|fact| fact.root && fact.kind == FactKind::Macro)
        .map(|fact| fact.name.clone())
        .collect();
    for extraction in extracted {
        for &(index, cursor) in &extraction.pending_macros {
            if roots.contains(facts[index].name.as_str()) {
                facts[index].data = fact_data(cursor, FactKind::Macro, &extraction.macros);
            }
        }
    }
}

fn fact_type_names(data: &FactData, names: &mut HashSet<String>) {
    let mut visit = |ty| type_names(ty, names);
    match data {
        FactData::Typedef { target } => visit(target),
        FactData::Callback { params, result, .. } | FactData::Function { params, result, .. } => {
            visit(result);
            for param in params {
                visit(&param.ty);
            }
        }
        FactData::Record { base, fields, .. } => {
            if let Some(base) = base {
                visit(base);
            }
            for field in fields {
                visit(&field.ty);
            }
        }
        FactData::Interface { base, methods, .. } => {
            if let Some(base) = base {
                visit(base);
            }
            for method in methods {
                visit(&method.result);
                for param in &method.params {
                    visit(&param.ty);
                }
            }
        }
        FactData::PropertyKey { ty, .. } => {
            names.insert((*ty).to_string());
        }
        FactData::EnumFlag { target } => {
            names.insert(target.clone());
        }
        _ => {}
    }
}

fn type_names(ty: &TypeRef, names: &mut HashSet<String>) {
    match ty {
        TypeRef::Named { name, .. } => {
            names.insert(name.clone());
        }
        TypeRef::Generic { name, args, .. } => {
            names.insert(name.clone());
            for arg in args {
                type_names(arg, names);
            }
        }
        TypeRef::Array { target, .. }
        | TypeRef::Pointer { target, .. }
        | TypeRef::Reference { target, .. } => type_names(target, names),
        TypeRef::FunctionPointer { params, result, .. } => {
            type_names(result, names);
            for param in params {
                type_names(param, names);
            }
        }
        TypeRef::InlineRecord(record) => {
            if let Some(base) = &record.base {
                type_names(base, names);
            }
            for field in &record.fields {
                type_names(&field.ty, names);
            }
        }
        _ => {}
    }
}

fn cursor_children(cursor: CXCursor) -> Vec<CXCursor> {
    struct Visit {
        children: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }

    extern "C" fn visit(
        cursor: CXCursor,
        _parent: CXCursor,
        data: CXClientData,
    ) -> CXChildVisitResult {
        let visit = unsafe { &mut *(data as *mut Visit) };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            visit.children.push(cursor);
        })) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                visit.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }

    let mut state = Visit {
        children: vec![],
        panic: None,
    };
    unsafe {
        clang_visitChildren(cursor, visit, &mut state as *mut _ as CXClientData);
    }
    if let Some(panic) = state.panic {
        std::panic::resume_unwind(panic);
    }
    state.children
}

#[derive(Default)]
struct MacroDefinitions<'tu> {
    definitions: HashMap<String, Vec<MacroDefinition>>,
    expansion_orders: HashMap<String, Vec<(u32, usize)>>,
    translation_unit: PhantomData<&'tu TranslationUnit>,
}

struct MacroDefinition {
    order: usize,
    cursor: CXCursor,
    function_like: bool,
    tokens: OnceCell<Vec<String>>,
}

impl MacroDefinition {
    fn tokens(&self) -> &[String] {
        self.tokens.get_or_init(|| {
            cursor_tokens(self.cursor)
                .into_iter()
                .map(|(_, token)| token)
                .skip(1)
                .collect()
        })
    }
}

impl MacroDefinitions<'_> {
    fn contains_key(&self, name: &str) -> bool {
        self.definitions.contains_key(name)
    }

    fn final_value(&self, name: &str) -> Option<&[String]> {
        self.definitions
            .get(name)?
            .last()
            .map(MacroDefinition::tokens)
    }

    fn definition(&self, cursor: CXCursor, name: &str) -> Option<(&[String], bool)> {
        let definition = self
            .definitions
            .get(name)?
            .iter()
            .find(|definition| unsafe { clang_equalCursors(definition.cursor, cursor) } != 0)?;
        Some((definition.tokens(), definition.function_like))
    }

    fn get_before(&self, name: &str, order: usize) -> Option<&[String]> {
        self.definitions
            .get(name)?
            .iter()
            .filter(|definition| definition.order < order)
            .max_by_key(|definition| definition.order)
            .map(MacroDefinition::tokens)
    }

    fn expansion_order(&self, cursor: CXCursor) -> Option<usize> {
        let (file, start, end) = cursor_expansion_extent(cursor)?;
        let expansions = self.expansion_orders.get(&file)?;
        let index = expansions.partition_point(|(offset, _)| *offset < start);
        expansions
            .get(index)
            .filter(|(offset, _)| *offset <= end)
            .map(|(_, order)| *order)
    }

    fn final_definition(&self, name: &str) -> Option<(&[String], bool)> {
        let definition = self.definitions.get(name)?.last()?;
        Some((definition.tokens(), definition.function_like))
    }
}

fn macro_definitions<'tu>(
    _translation_unit: &'tu TranslationUnit,
    cursor: CXCursor,
) -> MacroDefinitions<'tu> {
    let mut result = MacroDefinitions::default();
    for (order, child) in cursor_children(cursor).into_iter().enumerate() {
        let kind = unsafe { clang_getCursorKind(child) };
        if kind == CXCursor_MacroDefinition {
            let name = cx_string(unsafe { clang_getCursorSpelling(child) });
            let function_like = unsafe { clang_Cursor_isMacroFunctionLike(child) } != 0;
            result
                .definitions
                .entry(name)
                .or_default()
                .push(MacroDefinition {
                    order,
                    cursor: child,
                    function_like,
                    tokens: OnceCell::new(),
                });
        } else if kind == CXCursor_MacroExpansion
            && let Some((_, expansion, _, _)) = cursor_locations(child)
        {
            result
                .expansion_orders
                .entry(expansion.file)
                .or_default()
                .push((expansion.offset, order));
        }
    }
    for expansions in result.expansion_orders.values_mut() {
        expansions.sort_unstable_by_key(|(offset, order)| (*offset, std::cmp::Reverse(*order)));
        expansions.dedup_by_key(|(offset, _)| *offset);
    }
    result
}

fn cursor_expansion_extent(cursor: CXCursor) -> Option<(String, u32, u32)> {
    let range = unsafe { clang_getCursorExtent(cursor) };
    let start = source_location(
        unsafe { clang_getRangeStart(range) },
        clang_getExpansionLocation,
    )?;
    let end = source_location(
        unsafe { clang_getRangeEnd(range) },
        clang_getExpansionLocation,
    )?;
    (start.file == end.file).then_some((start.file, start.offset, end.offset))
}

fn source_function_name(
    cursor: CXCursor,
    link_name: &str,
    macros: &MacroDefinitions,
) -> Option<String> {
    let tokens = cursor_tokens(cursor);
    let token_name = tokens
        .iter()
        .enumerate()
        .filter(|(index, _)| tokens.get(index + 1).is_some_and(|(_, token)| token == "("))
        .map(|(_, (_, token))| token)
        .filter(|token| token.as_str() != link_name)
        .filter(|token| {
            macros
                .final_value(token)
                .is_some_and(|replacement| replacement == [link_name])
        })
        .min()
        .cloned();
    token_name.or_else(|| {
        let source = cursor_source(cursor)?;
        source
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .filter(|token| *token != link_name)
            .filter(|token| {
                macros
                    .final_value(token)
                    .is_some_and(|replacement| replacement == [link_name])
            })
            .min()
            .map(str::to_string)
    })
}

fn cursor_source(cursor: CXCursor) -> Option<String> {
    let (_, expansion, _, _) = cursor_locations(cursor)?;
    let source = std::fs::read(expansion.file).ok()?;
    let source = source.get(expansion.offset as usize..)?;
    let len = source
        .iter()
        .take(4096)
        .position(|byte| *byte == b';')
        .map_or(source.len().min(4096), |index| index + 1);
    let source = &source[..len];
    std::str::from_utf8(source).ok().map(str::to_string)
}

fn evaluate_constants(
    index: &Index,
    input: &Input,
    args: &[&str],
    facts: &[Fact],
    macros: &MacroDefinitions,
) -> Result<Vec<Constant>, Error> {
    let mut strings = BTreeMap::new();
    let mut roots = BTreeMap::new();
    for fact in facts
        .iter()
        .filter(|fact| fact.origin.tu == input.name && fact.root)
    {
        if let FactData::Macro {
            function_like: false,
            tokens,
        } = &fact.data
        {
            if let Some(value) = string_macro_value(&fact.name, macros, &mut HashSet::new()) {
                let ty = match value {
                    Value::Utf8(_) => TypeRef::Pointer {
                        mutable: false,
                        target: Box::new(TypeRef::Scalar(Scalar::I8)),
                    },
                    Value::Utf16(_) => TypeRef::Pointer {
                        mutable: false,
                        target: Box::new(TypeRef::Scalar(Scalar::U16)),
                    },
                    _ => unreachable!(),
                };
                strings
                    .entry(fact.name.clone())
                    .or_insert_with(|| Constant {
                        root: fact.origin.clone(),
                        definition: fact.origin.clone(),
                        spelling: fact.spelling.clone(),
                        name: fact.name.clone(),
                        ty,
                        value,
                    });
            }
            if macro_may_be_integer(tokens) {
                roots
                    .entry(fact.name.clone())
                    .or_insert_with(|| (fact.origin.clone(), fact.spelling.clone()));
            }
        }
    }
    let mut candidates = BTreeMap::new();
    for fact in facts.iter().filter(|fact| fact.origin.tu == input.name) {
        if let Some((root, spelling)) = roots.get(&fact.name)
            && matches!(
                fact.data,
                FactData::Macro {
                    function_like: false,
                    ..
                }
            )
        {
            candidates.insert(
                fact.name.clone(),
                (root.clone(), fact.origin.clone(), spelling.clone()),
            );
        }
    }
    candidates.retain(|name, _| {
        macros
            .final_definition(name)
            .is_some_and(|(_, function_like)| !function_like)
    });
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
    let batches: Vec<_> = names.chunks(16384).collect();
    let batch_results =
        run_probe_workers(workers, batches.len(), |index, worker, worker_count| {
            let mut evaluated = vec![];
            let mut reached = HashSet::new();
            for batch in batches.iter().skip(worker).step_by(worker_count) {
                let (batch_evaluated, batch_reached) = evaluate_probe(index, input, args, batch)?;
                evaluated.extend(batch_evaluated);
                reached.extend(batch_reached);
            }
            Ok((evaluated, reached))
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
    let recovery_batches: Vec<_> = missing.chunks(512).collect();
    let recovery_batch_count = recovery_batches.len();
    let recovery_results = run_probe_workers(
        workers,
        recovery_batches.len(),
        |index, worker, worker_count| {
            let mut evaluated = vec![];
            let mut reached = HashSet::new();
            for batch in recovery_batches.iter().skip(worker).step_by(worker_count) {
                let (batch_evaluated, batch_reached) = evaluate_probe(index, input, args, batch)?;
                evaluated.extend(batch_evaluated);
                reached.extend(batch_reached);
            }
            Ok((evaluated, reached))
        },
    )?;
    for (recovery_evaluated, recovery_reached) in recovery_results {
        evaluated.extend(recovery_evaluated);
        reached.extend(recovery_reached);
    }
    let unresolved: Vec<_> = missing
        .into_iter()
        .filter(|name| !reached.contains(name.as_str()))
        .collect();
    let isolation_batches: Vec<_> = unresolved.chunks(16).collect();
    let isolation_batch_count = isolation_batches.len();
    let isolation_results = run_probe_workers(
        workers,
        isolation_batches.len(),
        |index, worker, worker_count| {
            let mut evaluated = vec![];
            let mut reached = HashSet::new();
            for batch in isolation_batches.iter().skip(worker).step_by(worker_count) {
                let (batch_evaluated, batch_reached) = evaluate_probe(index, input, args, batch)?;
                evaluated.extend(batch_evaluated);
                reached.extend(batch_reached);
            }
            Ok((evaluated, reached))
        },
    )?;
    for (isolation_evaluated, isolation_reached) in isolation_results {
        evaluated.extend(isolation_evaluated);
        reached.extend(isolation_reached);
    }
    let defined = defined_macros(index, input, args, &unresolved)?;
    let fallback: Vec<_> = unresolved
        .iter()
        .filter(|name| !reached.contains(name.as_str()) && defined.contains(name.as_str()))
        .cloned()
        .collect();
    let fallback_count = fallback.len();
    let fallback_size = fallback.len().div_ceil(workers).max(1);
    let fallback_batches: Vec<_> = fallback.chunks(fallback_size).collect();
    let fallback_results =
        run_probe_workers(workers, fallback_batches.len(), |index, worker, _| {
            evaluate_singleton_probes(index, input, args, fallback_batches[worker], &reached)
        })?;
    for fallback in fallback_results {
        evaluated.extend(fallback);
    }
    if std::env::var_os("WINDOWS_CLANG_TIMING").is_some() {
        eprintln!(
            "clang macro probes: {} candidates, {} bulk batches, {} recovery batches, {} isolation batches, {} singleton probes, {:.2}s",
            names.len(),
            batches.len(),
            recovery_batch_count,
            isolation_batch_count,
            fallback_count,
            probe_time.elapsed().as_secs_f32()
        );
    }

    let mut constants: Vec<_> = strings.into_values().collect();
    for evaluated in evaluated {
        let Some((root, definition, spelling)) = candidates.get(&evaluated.name) else {
            continue;
        };
        constants.push(Constant {
            root: root.clone(),
            definition: definition.clone(),
            spelling: spelling.clone(),
            name: evaluated.name,
            ty: evaluated.ty,
            value: evaluated.value,
        });
    }
    Ok(constants)
}

fn run_probe_workers<T, F>(workers: usize, batch_count: usize, work: F) -> Result<Vec<T>, Error>
where
    T: Send,
    F: Fn(&Index, usize, usize) -> Result<T, Error> + Sync,
{
    let worker_count = workers.min(batch_count);
    std::thread::scope(|scope| {
        (0..worker_count)
            .map(|worker| {
                let work = &work;
                scope.spawn(move || {
                    let _library = Library::new()?;
                    let index = Index::new()?;
                    work(&index, worker, worker_count)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| {
                thread
                    .join()
                    .map_err(|_| Error("macro probe worker panicked".to_string()))?
            })
            .collect()
    })
}

fn macro_may_be_integer(tokens: &[String]) -> bool {
    !tokens.is_empty()
        && !tokens
            .iter()
            .any(|token| string_literal(token).is_some() || matches!(token.as_str(), "{" | "}"))
}

fn string_macro_value(
    name: &str,
    macros: &MacroDefinitions,
    visited: &mut HashSet<String>,
) -> Option<Value> {
    if !visited.insert(name.to_string()) {
        return None;
    }
    let (mut tokens, function_like) = macros.final_definition(name)?;
    if function_like {
        return None;
    }
    while tokens.len() >= 2 && tokens.first()? == "(" && tokens.last()? == ")" {
        tokens = &tokens[1..tokens.len() - 1];
    }
    if let [alias] = tokens
        && macros.contains_key(alias)
    {
        return string_macro_value(alias, macros, visited);
    }

    let mut value = String::new();
    let mut wide = None;
    for token in tokens {
        let (token_wide, inner) = string_literal(token)?;
        if wide.is_some_and(|wide| wide != token_wide) {
            return None;
        }
        wide = Some(token_wide);
        value.push_str(&decode_c_string(inner, token_wide)?);
    }
    match wide? {
        true => Some(Value::Utf16(value)),
        false => Some(Value::Utf8(value)),
    }
}

fn string_literal(token: &str) -> Option<(bool, &str)> {
    let (wide, quoted) = if let Some(value) = token.strip_prefix("u8") {
        (false, value)
    } else if let Some(value) = token
        .strip_prefix('L')
        .or_else(|| token.strip_prefix('u'))
        .or_else(|| token.strip_prefix('U'))
    {
        (true, value)
    } else {
        (false, token)
    };
    Some((wide, quoted.strip_prefix('"')?.strip_suffix('"')?))
}

fn decode_c_string(inner: &str, wide: bool) -> Option<String> {
    if !wide {
        return decode_c_bytes(inner);
    }
    let mut output = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        let character = match chars.next()? {
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            '?' => '?',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'a' => '\u{07}',
            'b' => '\u{08}',
            'f' => '\u{0c}',
            'v' => '\u{0b}',
            prefix @ ('x' | 'u' | 'U') => {
                let max = match prefix {
                    'u' => 4,
                    'U' => 8,
                    _ => usize::MAX,
                };
                let (value, count) = take_radix(&mut chars, 16, max);
                if count == 0 {
                    return None;
                }
                char::from_u32(value)?
            }
            digit @ '0'..='7' => {
                let mut value = digit.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                char::from_u32(value)?
            }
            other => {
                output.push('\\');
                other
            }
        };
        output.push(character);
    }
    Some(output)
}

fn decode_c_bytes(inner: &str) -> Option<String> {
    let mut output = vec![];
    let mut chars = inner.chars().peekable();
    while let Some(character) = chars.next() {
        if character != '\\' {
            let mut bytes = [0; 4];
            output.extend_from_slice(character.encode_utf8(&mut bytes).as_bytes());
            continue;
        }
        match chars.next()? {
            '\\' => output.push(b'\\'),
            '"' => output.push(b'"'),
            '\'' => output.push(b'\''),
            '?' => output.push(b'?'),
            'n' => output.push(b'\n'),
            'r' => output.push(b'\r'),
            't' => output.push(b'\t'),
            'a' => output.push(0x07),
            'b' => output.push(0x08),
            'f' => output.push(0x0c),
            'v' => output.push(0x0b),
            'x' => {
                let (value, count) = take_radix(&mut chars, 16, usize::MAX);
                if count == 0 {
                    return None;
                }
                output.push(u8::try_from(value).ok()?);
            }
            prefix @ ('u' | 'U') => {
                let max = if prefix == 'u' { 4 } else { 8 };
                let (value, count) = take_radix(&mut chars, 16, max);
                if count != max {
                    return None;
                }
                let character = char::from_u32(value)?;
                let mut bytes = [0; 4];
                output.extend_from_slice(character.encode_utf8(&mut bytes).as_bytes());
            }
            digit @ '0'..='7' => {
                let mut value = digit.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                output.push(u8::try_from(value).ok()?);
            }
            other => {
                output.push(b'\\');
                let mut bytes = [0; 4];
                output.extend_from_slice(other.encode_utf8(&mut bytes).as_bytes());
            }
        }
    }
    String::from_utf8(output).ok()
}

fn take_radix(
    chars: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    radix: u32,
    max: usize,
) -> (u32, usize) {
    let mut value = 0u32;
    let mut count = 0;
    while count < max {
        let Some(digit) = chars.peek().and_then(|character| character.to_digit(radix)) else {
            break;
        };
        chars.next();
        let Some(next) = value
            .checked_mul(radix)
            .and_then(|value| value.checked_add(digit))
        else {
            return (0, 0);
        };
        value = next;
        count += 1;
    }
    (value, count)
}

fn evaluate_probe(
    index: &Index,
    input: &Input,
    args: &[&str],
    names: &[String],
) -> Result<(Vec<Evaluated>, HashSet<String>), Error> {
    let tu = TranslationUnit::parse_probe(index, input, &probe_source(names), args)?;
    Ok(evaluate_parsed_probe(&tu, input))
}

fn evaluate_singleton_probes(
    index: &Index,
    input: &Input,
    args: &[&str],
    names: &[String],
    reached: &HashSet<String>,
) -> Result<Vec<Evaluated>, Error> {
    let mut evaluated = vec![];
    for name in names {
        if !reached.contains(name.as_str()) {
            evaluated.extend(evaluate_probe(index, input, args, std::slice::from_ref(name))?.0);
        }
    }
    Ok(evaluated)
}

fn defined_macros(
    index: &Index,
    input: &Input,
    args: &[&str],
    names: &[String],
) -> Result<HashSet<String>, Error> {
    if names.is_empty() {
        return Ok(HashSet::new());
    }
    let mut probe = String::new();
    for name in names {
        probe.push_str(&format!(
            "#ifdef {name}\n\
             enum {{ __clang_defined_{name} = 1 }};\n\
             #endif\n"
        ));
    }
    let tu = TranslationUnit::parse_probe(index, input, &probe, args)?;
    let mut result = HashSet::new();
    for cursor in cursor_children(unsafe { clang_getTranslationUnitCursor(tu.0) }) {
        if unsafe { clang_getCursorKind(cursor) } != CXCursor_EnumDecl {
            continue;
        }
        for constant in cursor_children(cursor) {
            let name = cx_string(unsafe { clang_getCursorSpelling(constant) });
            if let Some(name) = name.strip_prefix("__clang_defined_") {
                result.insert(name.to_string());
            }
        }
    }
    Ok(result)
}

fn probe_source(names: &[String]) -> String {
    let mut probe = String::from(
        "#define __WINDOWS_CLANG_NARG(...) __WINDOWS_CLANG_NARG_(__VA_ARGS__,2,1,0)\n\
         #define __WINDOWS_CLANG_NARG_(_1,_2,N,...) N\n",
    );
    for name in names {
        probe.push_str(&format!(
            "#ifdef {name}\n\
             constexpr auto __clang_eval_{name} = ({name});\n\
             constexpr __int64 __clang_bits_{name} = \
                 (__int64)((__INTPTR_TYPE__)({name}));\n\
             enum {{ __clang_count_{name} = __WINDOWS_CLANG_NARG({name}) }};\n\
             #endif\n"
        ));
    }
    probe
}

fn evaluate_parsed_probe(tu: &TranslationUnit, input: &Input) -> (Vec<Evaluated>, HashSet<String>) {
    let cursors = cursor_children(unsafe { clang_getTranslationUnitCursor(tu.0) });
    let mut counts = HashMap::new();
    for cursor in &cursors {
        if unsafe { clang_getCursorKind(*cursor) } == CXCursor_EnumDecl {
            for constant in cursor_children(*cursor) {
                let name = cx_string(unsafe { clang_getCursorSpelling(constant) });
                if let Some(name) = name.strip_prefix("__clang_count_") {
                    counts.insert(name.to_string(), unsafe {
                        clang_getEnumConstantDeclValue(constant)
                    });
                }
            }
        }
    }

    let mut result = vec![];
    let integer_values: HashMap<_, _> = cursors
        .iter()
        .filter_map(|cursor| {
            let cursor_name = cx_string(unsafe { clang_getCursorSpelling(*cursor) });
            let name = cursor_name.strip_prefix("__clang_bits_")?;
            Some((name.to_string(), evaluate_integer(*cursor)?))
        })
        .collect();
    for cursor in cursors {
        let cursor_name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
        let Some(name) = cursor_name.strip_prefix("__clang_eval_") else {
            continue;
        };
        if counts.get(name) != Some(&1) {
            continue;
        }
        let ty = unsafe { clang_getCursorType(cursor) };
        let Some(mut ty_ref) = type_ref(ty) else {
            continue;
        };
        let canonical = unsafe { clang_getCanonicalType(ty) };
        if canonical.kind == CXType_Pointer
            && matches!(
                unsafe { clang_getPointeeType(canonical) }.kind,
                CXType_FunctionProto | CXType_FunctionNoProto
            )
        {
            continue;
        }
        if let TypeRef::Named { declaration, .. } = &mut ty_ref
            && declaration.file == format!("{}.__clang_eval.cpp", input.name)
        {
            declaration.file.clone_from(&input.name);
        }
        let value_scalar = scalar(ty);
        let value = if let Some(value_scalar @ (Scalar::F32 | Scalar::F64)) = value_scalar {
            let Some(value) = evaluate_float(cursor, value_scalar) else {
                continue;
            };
            value
        } else {
            let Some(value) =
                evaluate_integer(cursor).or_else(|| integer_values.get(name).copied())
            else {
                continue;
            };
            if value_scalar.is_some_and(|value_scalar| {
                matches!(
                    value_scalar,
                    Scalar::Bool | Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::U64
                )
            }) {
                Value::Unsigned(value.0)
            } else {
                Value::Signed(value.1)
            }
        };
        result.push(Evaluated {
            name: name.to_string(),
            ty: ty_ref,
            value,
        });
    }
    let reached = counts.into_keys().collect();
    (result, reached)
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

fn evaluate_float(cursor: CXCursor, scalar: Scalar) -> Option<Value> {
    unsafe {
        let result = clang_Cursor_Evaluate(cursor);
        if result.is_null() {
            return None;
        }
        let value = (clang_EvalResult_getKind(result) == CXEval_Float).then(|| {
            let value = clang_EvalResult_getAsDouble(result);
            match scalar {
                Scalar::F32 => Value::F32((value as f32).to_bits()),
                Scalar::F64 => Value::F64(value.to_bits()),
                _ => unreachable!(),
            }
        });
        clang_EvalResult_dispose(result);
        value
    }
}

fn cursor_locations(cursor: CXCursor) -> Option<(Location, Location, bool, bool)> {
    unsafe {
        let location = clang_getCursorLocation(cursor);
        let expansion = source_location(location, clang_getExpansionLocation)?;
        let spelling = source_location(location, clang_getSpellingLocation)
            .unwrap_or_else(|| expansion.clone());
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

fn fact_data(cursor: CXCursor, kind: FactKind, macros: &MacroDefinitions) -> FactData {
    match kind {
        FactKind::Class => {
            if is_interface(cursor) {
                interface_fact(cursor, macros)
            } else {
                cursor_uuid(cursor).map_or_else(
                    || FactData::Unsupported {
                        reason: "class has no UUID".to_string(),
                    },
                    |guid| FactData::Class { guid },
                )
            }
        }
        FactKind::Enum => {
            let ty = unsafe { clang_getEnumDeclIntegerType(cursor) };
            let Some(repr) = scalar(ty) else {
                return FactData::None;
            };
            let fixed = cursor_tokens(cursor).iter().any(|(_, token)| token == ":");
            let variants = cursor_children(cursor)
                .into_iter()
                .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_EnumConstantDecl)
                .map(|child| Variant {
                    name: cx_string(unsafe { clang_getCursorSpelling(child) }),
                    value: unsafe { clang_getEnumConstantDeclValue(child) },
                })
                .collect();
            FactData::Enum {
                repr,
                variants,
                fixed,
                scoped: unsafe { clang_EnumDecl_isScoped(cursor) } != 0,
            }
        }
        FactKind::Function => {
            if unsafe { clang_getCursorLinkage(cursor) } != CXLinkage_External
                || unsafe { clang_isCursorDefinition(cursor) } != 0
            {
                return FactData::Unsupported {
                    reason: "function is not an external declaration".to_string(),
                };
            }
            let variadic = unsafe { clang_Cursor_isVariadic(cursor) } != 0;
            let result_ty = unsafe { clang_getCursorResultType(cursor) };
            let Some(result) = type_ref(result_ty) else {
                return FactData::Unsupported {
                    reason: format!(
                        "function has unsupported result type `{}`",
                        cx_string(unsafe { clang_getTypeSpelling(result_ty) })
                    ),
                };
            };
            let params = match callable_params(cursor, macros, false) {
                Ok(params) => params,
                Err(reason) => return FactData::Unsupported { reason },
            };
            let function_ty = unsafe { clang_getCursorType(cursor) };
            let Some(convention) = (if variadic {
                calling_convention_fact(function_ty)
            } else {
                source_calling_convention(cursor, macros)
                    .or_else(|| calling_convention_fact(function_ty))
            }) else {
                return FactData::Unsupported {
                    reason: "function has an unsupported calling convention".to_string(),
                };
            };
            FactData::Function {
                link_name: external_link_name(cursor),
                convention,
                params,
                result,
                variadic,
                noreturn: function_is_noreturn(cursor),
            }
        }
        FactKind::Macro => {
            let name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
            let (tokens, function_like) = macros.definition(cursor, &name).map_or_else(
                || {
                    (
                        cursor_tokens(cursor)
                            .into_iter()
                            .map(|(_, token)| token)
                            .skip(1)
                            .collect(),
                        unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0,
                    )
                },
                |(tokens, function_like)| (tokens.to_vec(), function_like),
            );
            FactData::Macro {
                function_like,
                tokens,
            }
        }
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
        FactKind::Guid => {
            let tokens = cursor_tokens(cursor);
            let macro_name = tokens
                .first()
                .map(|(_, token)| token.as_str())
                .unwrap_or_default();
            if matches!(macro_name, "DEFINE_PROPERTYKEY" | "DEFINE_DEVPROPKEY") {
                return parse_property_key_tokens(&tokens).map_or_else(
                    || FactData::Unsupported {
                        reason: "property-key macro has invalid arguments".to_string(),
                    },
                    |(_, guid, pid)| FactData::PropertyKey {
                        ty: if macro_name == "DEFINE_DEVPROPKEY" {
                            "DEVPROPKEY"
                        } else {
                            "PROPERTYKEY"
                        },
                        guid,
                        pid,
                    },
                );
            }
            let ole = macro_name == "DEFINE_OLEGUID";
            parse_define_guid_tokens(&tokens, ole).map_or_else(
                || FactData::Unsupported {
                    reason: "GUID macro has invalid arguments".to_string(),
                },
                |(_, value)| FactData::Guid { value },
            )
        }
        FactKind::Struct | FactKind::Union => {
            if kind == FactKind::Struct && is_interface(cursor) {
                return interface_fact(cursor, macros);
            }
            if kind == FactKind::Struct
                && let Some(guid) = cursor_uuid(cursor)
            {
                return FactData::Class { guid };
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
            let function = if ty.kind == CXType_Pointer {
                unsafe { clang_getPointeeType(ty) }
            } else {
                ty
            };
            if matches!(function.kind, CXType_FunctionProto | CXType_FunctionNoProto) {
                let inherited_convention = if ty.kind == CXType_Pointer {
                    cursor_children(cursor)
                        .into_iter()
                        .filter(|child| unsafe { clang_getCursorKind(*child) == CXCursor_TypeRef })
                        .find_map(|child| {
                            let declaration = unsafe { clang_getCursorReferenced(child) };
                            (unsafe { clang_Cursor_isNull(declaration) } == 0)
                                .then(|| source_calling_convention(declaration, macros))
                                .flatten()
                        })
                } else {
                    None
                };
                return function_signature(
                    function,
                    source_calling_convention(cursor, macros).or(inherited_convention),
                )
                .map_or_else(
                    || FactData::Unsupported {
                        reason: "callback has an unsupported signature".to_string(),
                    },
                    |(convention, params, result)| FactData::Callback {
                        convention,
                        params: callback_params(cursor, macros, params),
                        result,
                    },
                );
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

fn external_link_name(cursor: CXCursor) -> String {
    let name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
    let mangled = cx_string(unsafe { clang_Cursor_getMangling(cursor) });
    if mangled == name
        || mangled == format!("_{name}")
        || mangled
            .strip_prefix(&format!("_{name}@"))
            .is_some_and(|bytes| bytes.chars().all(|c| c.is_ascii_digit()))
        || mangled
            .strip_prefix(&format!("@{name}@"))
            .is_some_and(|bytes| bytes.chars().all(|c| c.is_ascii_digit()))
    {
        name
    } else {
        mangled
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
        && (cursor_uuid(cursor).is_some()
            || methods
                .iter()
                .all(|method| unsafe { clang_CXXMethod_isPureVirtual(**method) } != 0)))
        || children.iter().any(|child| {
            if unsafe { clang_getCursorKind(*child) } != CXCursor_CXXBaseSpecifier {
                return false;
            }
            let declaration = unsafe { clang_getTypeDeclaration(clang_getCursorType(*child)) };
            (unsafe { clang_Cursor_isNull(declaration) }) == 0 && is_interface(declaration)
        })
}

fn cursor_definition(cursor: CXCursor) -> CXCursor {
    let definition = unsafe { clang_getCursorDefinition(cursor) };
    if unsafe { clang_Cursor_isNull(definition) } == 0 {
        definition
    } else {
        cursor
    }
}

fn interface_inherits_from(cursor: CXCursor, ancestor: CXCursor) -> bool {
    let cursor = cursor_definition(cursor);
    let ancestor = cursor_definition(ancestor);
    for child in cursor_children(cursor) {
        if unsafe { clang_getCursorKind(child) } != CXCursor_CXXBaseSpecifier {
            continue;
        }
        let base = unsafe { clang_getTypeDeclaration(clang_getCursorType(child)) };
        if unsafe { clang_Cursor_isNull(base) } != 0 {
            continue;
        }
        let base = cursor_definition(base);
        if unsafe { clang_equalCursors(base, ancestor) } != 0
            || interface_inherits_from(base, ancestor)
        {
            return true;
        }
    }
    false
}

fn interface_fact(cursor: CXCursor, macros: &MacroDefinitions) -> FactData {
    let children = cursor_children(cursor);
    let async_interface =
        cx_string(unsafe { clang_getCursorSpelling(cursor) }).starts_with("Async");
    let mut bases = vec![];
    for child in &children {
        if unsafe { clang_getCursorKind(*child) } != CXCursor_CXXBaseSpecifier {
            continue;
        }
        let declaration = unsafe { clang_getTypeDeclaration(clang_getCursorType(*child)) };
        if unsafe { clang_Cursor_isNull(declaration) } != 0 || !is_interface(declaration) {
            return FactData::Unsupported {
                reason: "interface base is not an interface".to_string(),
            };
        }
        bases.push((*child, declaration));
    }
    let most_derived: Vec<_> = bases
        .iter()
        .filter(|(_, candidate)| {
            !bases.iter().any(|(_, other)| {
                (unsafe { clang_equalCursors(*candidate, *other) }) == 0
                    && interface_inherits_from(*other, *candidate)
            })
        })
        .collect();
    let base = match most_derived.as_slice() {
        [] => None,
        [(child, _)] => {
            let Some(ty) = type_ref(unsafe { clang_getCursorType(*child) }) else {
                return FactData::Unsupported {
                    reason: "interface has an unsupported base".to_string(),
                };
            };
            Some(ty)
        }
        _ if bases.len() == 2
            && cx_string(unsafe { clang_getCursorSpelling(bases[1].1) }) == "IUnknown" =>
        {
            let Some(ty) = type_ref(unsafe { clang_getCursorType(bases[0].0) }) else {
                return FactData::Unsupported {
                    reason: "interface has an unsupported base".to_string(),
                };
            };
            Some(ty)
        }
        _ => {
            return FactData::Unsupported {
                reason: "interface has unrelated multiple bases".to_string(),
            };
        }
    };
    let guid = cursor_uuid(cursor);
    let mut methods = vec![];
    for child in children {
        match unsafe { clang_getCursorKind(child) } {
            CXCursor_CXXBaseSpecifier => {}
            CXCursor_CXXMethod if unsafe { clang_CXXMethod_isVirtual(child) } != 0 => {
                if method_overrides_base(child) {
                    continue;
                }
                let result_ty = unsafe { clang_getCursorResultType(child) };
                let Some(result) = type_ref(result_ty) else {
                    return FactData::Unsupported {
                        reason: "interface method has an unsupported result".to_string(),
                    };
                };
                let method_name = cx_string(unsafe { clang_getCursorSpelling(child) });
                let method_name =
                    source_function_name(child, &method_name, macros).unwrap_or(method_name);
                let params = match callable_params(
                    child,
                    macros,
                    async_interface && method_name.starts_with("Finish_"),
                ) {
                    Ok(params) => params,
                    Err(reason) => return FactData::Unsupported { reason },
                };
                let tokens = cursor_tokens(child);
                let special =
                    tokens_before_method_name(&tokens, child)
                        .iter()
                        .any(|(kind, token)| {
                            *kind == CXToken_Comment
                                && (token.contains("[propget]") || token.contains("[propput]"))
                        });
                methods.push(Method {
                    name: method_name,
                    params,
                    result,
                    special,
                });
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

fn parse_define_guid_tokens(
    tokens: &[(CXTokenKind, String)],
    ole: bool,
) -> Option<(String, String)> {
    let lparen = tokens
        .iter()
        .position(|(kind, token)| *kind == CXToken_Punctuation && token == "(")?;
    let name = tokens[lparen + 1..]
        .iter()
        .find(|(kind, _)| *kind == CXToken_Identifier)?
        .1
        .clone();
    let mut values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(kind, _)| *kind == CXToken_Literal)
        .map(|(_, token)| parse_c_integer(token))
        .collect::<Option<_>>()?;
    if ole {
        if values.len() != 3 {
            return None;
        }

        values.extend_from_slice(&[0xc0, 0, 0, 0, 0, 0, 0, 0x46]);
    }
    Some((name, format_guid(&values)?))
}

fn parse_property_key_tokens(tokens: &[(CXTokenKind, String)]) -> Option<(String, String, u32)> {
    let lparen = tokens
        .iter()
        .position(|(kind, token)| *kind == CXToken_Punctuation && token == "(")?;
    let name = tokens[lparen + 1..]
        .iter()
        .find(|(kind, _)| *kind == CXToken_Identifier)?
        .1
        .clone();
    let values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(kind, _)| *kind == CXToken_Literal)
        .map(|(_, token)| parse_c_integer(token))
        .collect::<Option<_>>()?;
    if values.len() != 12 || values[11] > u32::MAX.into() {
        return None;
    }
    Some((name, format_guid(&values[..11])?, values[11] as u32))
}

fn parse_c_integer(value: &str) -> Option<u64> {
    let digits = value.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        digits.parse().ok()
    }
}

fn format_guid(values: &[u64]) -> Option<String> {
    if values.len() != 11
        || values[0] > u32::MAX.into()
        || values[1] > u16::MAX.into()
        || values[2] > u16::MAX.into()
        || values[3..].iter().any(|value| *value > u8::MAX.into())
    {
        return None;
    }
    Some(format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
        values[5],
        values[6],
        values[7],
        values[8],
        values[9],
        values[10],
    ))
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
    macros: &MacroDefinitions,
    allow_unresolved_size: bool,
) -> Result<Vec<Parameter>, String> {
    let mut params = vec![];
    for child in cursor_children(cursor)
        .into_iter()
        .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
    {
        let param_ty = unsafe { clang_getCursorType(child) };
        let Some(ty) = function_param_type_at_cursor(child, param_ty) else {
            return Err(format!(
                "parameter has unsupported type `{}`",
                cx_string(unsafe { clang_getTypeSpelling(param_ty) })
            ));
        };
        let mut name = cx_string(unsafe { clang_getCursorSpelling(child) });
        if name.is_empty() {
            name = format!("param{}", params.len());
        }
        let mut annotation = parameter_annotation(child);
        normalize_constant_byte_size(param_ty, &mut annotation);
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
        let mut discard_size = false;
        if let Some(size) = &mut param.annotation.size {
            match &mut size.value {
                SalSizeValue::Parameter(name) if !names.contains(name) => {
                    if macros.contains_key(name) {
                        size.value = SalSizeValue::Expression(name.clone());
                    } else if allow_unresolved_size {
                        discard_size = true;
                    } else {
                        size.value = SalSizeValue::Expression(name.clone());
                    }
                }
                SalSizeValue::IndirectParameter(name) if !names.contains(name) => {
                    if allow_unresolved_size {
                        discard_size = true;
                    } else {
                        size.value = SalSizeValue::Expression(format!("*{name}"));
                    }
                }
                SalSizeValue::Constant(_) if size.bytes => {
                    return Err("constant byte-size SAL annotations are unsupported".to_string());
                }
                SalSizeValue::Expression(_) => {}
                _ => {}
            }
        }
        if discard_size {
            param.annotation.size = None;
        }
    }
    apply_source_annotations(cursor, macros, &mut params);
    Ok(params)
}

fn callback_params(
    cursor: CXCursor,
    macros: &MacroDefinitions,
    fallback: Vec<TypeRef>,
) -> Vec<Parameter> {
    let mut candidates = vec![cursor];
    candidates.extend(
        cursor_children(cursor)
            .into_iter()
            .filter(|child| unsafe { clang_getCursorKind(*child) == CXCursor_TypeRef })
            .map(|child| unsafe { clang_getCursorReferenced(child) })
            .filter(|child| unsafe { clang_Cursor_isNull(*child) } == 0),
    );
    for candidate in candidates {
        if let Ok(params) = callable_params(candidate, macros, false)
            && params.len() == fallback.len()
        {
            return params;
        }
    }
    fallback
        .into_iter()
        .enumerate()
        .map(|(index, ty)| Parameter {
            name: format!("param{index}"),
            ty,
            annotation: ParamAnnotation::default(),
        })
        .collect()
}

fn normalize_constant_byte_size(ty: CXType, annotation: &mut ParamAnnotation) {
    let Some(SalSize {
        bytes: true,
        value: SalSizeValue::Constant(value),
    }) = &mut annotation.size
    else {
        return;
    };
    let canonical = unsafe { clang_getCanonicalType(ty) };
    if canonical.kind != CXType_Pointer {
        return;
    }
    let element_size = unsafe { clang_Type_getSizeOf(clang_getPointeeType(canonical)) };
    if element_size > 0
        && i64::from(*value) % element_size == 0
        && let Ok(element_count) = i32::try_from(i64::from(*value) / element_size)
    {
        *value = element_count;
        if let Some(size) = &mut annotation.size {
            size.bytes = false;
        }
    }
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

fn apply_source_annotations(cursor: CXCursor, macros: &MacroDefinitions, params: &mut [Parameter]) {
    let tokens = cursor_tokens(cursor);
    let parameter_cursors: Vec<_> = cursor_children(cursor)
        .into_iter()
        .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
        .collect();
    let cursor_name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
    let name_index = tokens
        .iter()
        .position(|(kind, token)| *kind == CXToken_Identifier && token == &cursor_name);
    let Some(open) = tokens
        .iter()
        .enumerate()
        .skip(name_index.map_or(0, |index| index + 1))
        .find(|(_, (kind, token))| *kind == CXToken_Punctuation && token == "(")
        .map(|(index, _)| index)
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
            (CXToken_Identifier, annotation)
                if depth == 1 && index < params.len() && annotation.starts_with("_COM_Outptr_") =>
            {
                let param = &mut params[index].annotation;
                param.output = true;
                param.com_out_ptr |= parameter_cursors.get(index).is_some_and(|cursor| {
                    is_void_double_pointer(unsafe { clang_getCursorType(*cursor) })
                });
                param.optional |= annotation.contains("_opt_");
            }
            (CXToken_Identifier, annotation)
                if depth == 1 && index < params.len() && macros.contains_key(annotation) =>
            {
                let param = &mut params[index].annotation;
                match annotation {
                    "IN" => param.input = true,
                    "OUT" => param.output = true,
                    "OPTIONAL" => param.optional = true,
                    _ => {}
                }
            }
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
            result.com_out_ptr = is_void_double_pointer(unsafe { clang_getCursorType(cursor) });
        }
        let sal_name = annotation
            .split_once('(')
            .map_or(annotation.as_str(), |value| value.0);
        if sal_name.contains("_z_") || sal_name.ends_with("_z") {
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

fn function_is_noreturn(cursor: CXCursor) -> bool {
    let ty = unsafe { clang_getCursorType(cursor) };
    if cx_string(unsafe { clang_getTypeSpelling(ty) }).contains("noreturn") {
        return true;
    }
    cursor_children(cursor).into_iter().any(|child| {
        (unsafe { clang_getCursorKind(child) }) == CXCursor_AnnotateAttr
            && cx_string(unsafe { clang_getCursorSpelling(child) }) == "_Analysis_noreturn_"
    })
}

fn is_void_double_pointer(ty: CXType) -> bool {
    let ty = unsafe { clang_getCanonicalType(ty) };
    if ty.kind != CXType_Pointer {
        return false;
    }
    let ty = unsafe { clang_getCanonicalType(clang_getPointeeType(ty)) };
    if ty.kind != CXType_Pointer {
        return false;
    }
    unsafe { clang_getCanonicalType(clang_getPointeeType(ty)) }.kind == CXType_Void
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

fn function_signature(
    ty: CXType,
    convention: Option<CallingConvention>,
) -> Option<(CallingConvention, Vec<TypeRef>, TypeRef)> {
    let convention = convention.or_else(|| calling_convention_fact(ty))?;
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
    let array = if matches!(ty.kind, CXType_IncompleteArray | CXType_ConstantArray) {
        Some(ty)
    } else {
        let declaration = unsafe { clang_getTypeDeclaration(ty) };
        if unsafe { clang_getCursorKind(declaration) } == CXCursor_TypedefDecl {
            let underlying = unsafe { clang_getTypedefDeclUnderlyingType(declaration) };
            matches!(
                underlying.kind,
                CXType_IncompleteArray | CXType_ConstantArray
            )
            .then_some(underlying)
        } else {
            None
        }
    };
    if let Some(array) = array {
        let canonical = unsafe { clang_getCanonicalType(ty) };
        if canonical.kind == CXType_Pointer {
            return type_ref(canonical);
        }
        let element = unsafe { clang_getArrayElementType(array) };
        return Some(TypeRef::Pointer {
            mutable: unsafe {
                clang_isConstQualifiedType(ty) == 0 && clang_isConstQualifiedType(element) == 0
            },
            target: Box::new(type_ref(element)?),
        });
    }
    if matches!(ty.kind, CXType_FunctionProto | CXType_FunctionNoProto) {
        let (convention, params, result) = function_signature(ty, None)?;
        return Some(TypeRef::FunctionPointer {
            convention,
            params,
            result: Box::new(result),
        });
    }
    type_ref(ty)
}

fn function_param_type_at_cursor(cursor: CXCursor, ty: CXType) -> Option<TypeRef> {
    let mut ty = function_param_type(ty)?;
    preserve_named_function_type(cursor, &mut ty);
    Some(ty)
}

fn preserve_named_function_type(cursor: CXCursor, ty: &mut TypeRef) {
    let Some((name, declaration)) = cursor_children(cursor).into_iter().find_map(|child| {
        if unsafe { clang_getCursorKind(child) } != CXCursor_TypeRef {
            return None;
        }
        let referenced = unsafe { clang_getCursorReferenced(child) };
        if unsafe { clang_getCursorKind(referenced) } != CXCursor_TypedefDecl {
            return None;
        }
        let underlying =
            unsafe { clang_getCanonicalType(clang_getTypedefDeclUnderlyingType(referenced)) };
        if !matches!(
            underlying.kind,
            CXType_FunctionProto | CXType_FunctionNoProto
        ) {
            return None;
        }
        let name = cx_string(unsafe { clang_getCursorSpelling(referenced) });
        let (declaration, _, _, _) = cursor_locations(referenced)?;
        Some((name, declaration))
    }) else {
        return;
    };
    replace_function_pointer(ty, &TypeRef::Named { name, declaration });
}

fn replace_function_pointer(ty: &mut TypeRef, replacement: &TypeRef) -> bool {
    match ty {
        TypeRef::FunctionPointer { .. } => {
            *ty = replacement.clone();
            true
        }
        TypeRef::Pointer { target, .. } | TypeRef::Reference { target, .. } => {
            replace_function_pointer(target, replacement)
        }
        _ => false,
    }
}

fn calling_convention_fact(ty: CXType) -> Option<CallingConvention> {
    Some(match unsafe { clang_getFunctionTypeCallingConv(ty) } {
        CXCallingConv_C => CallingConvention::C,
        CXCallingConv_Default
        | CXCallingConv_X86FastCall
        | CXCallingConv_X86StdCall
        | CXCallingConv_Win64 => CallingConvention::Platform,
        _ => return None,
    })
}

fn source_calling_convention(
    cursor: CXCursor,
    macros: &MacroDefinitions,
) -> Option<CallingConvention> {
    fn literal(token: &str) -> Option<CallingConvention> {
        match token {
            "__stdcall" | "_stdcall" => Some(CallingConvention::Platform),
            "__cdecl" | "_cdecl" => Some(CallingConvention::C),
            _ => None,
        }
    }

    fn resolve(
        token: &str,
        macros: &MacroDefinitions,
        order: usize,
        visited: &mut HashSet<String>,
    ) -> Option<CallingConvention> {
        if let Some(convention) = literal(token) {
            return Some(convention);
        }
        if !visited.insert(token.to_string()) {
            return None;
        }
        macros
            .get_before(token, order)?
            .iter()
            .find_map(|token| resolve(token, macros, order, visited))
    }

    let tokens = cursor_tokens(cursor);
    macros
        .expansion_order(cursor)
        .and_then(|order| {
            tokens_before_method_name(&tokens, cursor)
                .iter()
                .find_map(|(_, token)| resolve(token, macros, order, &mut HashSet::new()))
                .or_else(|| {
                    tokens
                        .iter()
                        .find_map(|(_, token)| resolve(token, macros, order, &mut HashSet::new()))
                })
        })
        .or_else(|| tokens.iter().find_map(|(_, token)| literal(token)))
        .or_else(|| {
            cursor_source(cursor)?
                .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
                .find_map(literal)
        })
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
            let mut ty = type_ref(field_ty).ok_or_else(|| {
                format!(
                    "field `{name}` has unsupported type `{}`",
                    cx_string(unsafe { clang_getTypeSpelling(field_ty) })
                )
            })?;
            preserve_named_function_type(child, &mut ty);
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

fn name_indirect_inline_records(record: &mut InlineRecord, owner: &str) {
    let mut next = 0;
    for field in &mut record.fields {
        name_indirect_inline_type(&mut field.ty, owner, &mut next, false);
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
            let nested_owner = record.name.clone().unwrap_or_else(|| {
                let name = format!("{owner}_{}", *next);
                *next += 1;
                if indirect {
                    record.name = Some(name.clone());
                }
                name
            });
            name_indirect_inline_records(record, &nested_owner);
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

fn type_ref(ty: CXType) -> Option<TypeRef> {
    let generic_count = unsafe { clang_Type_getNumTemplateArguments(ty) };
    if generic_count > 0 && ty.kind != CXType_Typedef {
        let declaration = unsafe { clang_getTypeDeclaration(ty) };
        let name = cx_string(unsafe { clang_getCursorSpelling(declaration) });
        let (declaration, _, _, _) = cursor_locations(declaration)?;
        let args = (0..generic_count)
            .map(|index| {
                winrt_generic_arg(unsafe {
                    clang_Type_getTemplateArgumentAsType(ty, index.try_into().unwrap())
                })
            })
            .collect::<Option<Vec<_>>>()?;
        return Some(TypeRef::Generic {
            name,
            declaration,
            args,
        });
    }
    if ty.kind == CXType_Elaborated {
        let declaration = unsafe { clang_getTypeDeclaration(ty) };
        if unsafe { clang_Cursor_isNull(declaration) } == 0 {
            let kind = unsafe { clang_getCursorKind(declaration) };
            if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
                && cx_string(unsafe { clang_getTypeSpelling(ty) }).contains("(unnamed at ")
            {
                return inline_record(declaration, kind == CXCursor_UnionDecl)
                    .ok()
                    .map(|record| TypeRef::InlineRecord(Box::new(record)));
            }
        }
        return type_ref(unsafe { clang_Type_getNamedType(ty) });
    }
    if ty.kind == CXType_Void {
        return Some(TypeRef::Void);
    }
    if ty.kind == CXType_Pointer {
        let pointee = unsafe { clang_getPointeeType(ty) };
        let canonical_pointee = unsafe { clang_getCanonicalType(pointee) };
        if matches!(
            canonical_pointee.kind,
            CXType_FunctionProto | CXType_FunctionNoProto
        ) {
            let (convention, params, result) = function_signature(canonical_pointee, None)?;
            return Some(TypeRef::FunctionPointer {
                convention,
                params,
                result: Box::new(result),
            });
        }
        let declaration = unsafe { clang_getTypeDeclaration(pointee) };
        if unsafe { clang_Cursor_isNull(declaration) } == 0
            && unsafe { clang_getCursorKind(declaration) } == CXCursor_ClassDecl
            && !is_interface(declaration)
            && cursor_uuid(declaration).is_none()
        {
            return Some(TypeRef::OpaquePointer {
                mutable: unsafe { clang_isConstQualifiedType(pointee) } == 0,
                tag: cx_string(unsafe { clang_getCursorSpelling(declaration) }),
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
        let spelling = cx_string(unsafe { clang_getTypeSpelling(ty) });
        if kind == CXCursor_EnumDecl && spelling.contains("(unnamed enum at ") {
            return scalar(ty).map(TypeRef::Scalar);
        }
        if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
            && (unsafe { clang_Cursor_isAnonymous(declaration) } != 0
                || cx_string(unsafe { clang_getCursorSpelling(declaration) }).is_empty()
                || spelling.contains("(unnamed at "))
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
            if kind == CXCursor_EnumDecl && name.contains("(unnamed enum at ") {
                return scalar(ty).map(TypeRef::Scalar);
            }
            if !name.is_empty()
                && let Some((location, _, _, _)) = cursor_locations(declaration)
            {
                return Some(TypeRef::Named {
                    name,
                    declaration: location,
                });
            }
        }
        if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
            && cx_string(unsafe { clang_getCursorSpelling(declaration) }).ends_with("__")
        {
            return inline_record(declaration, kind == CXCursor_UnionDecl)
                .ok()
                .map(|record| TypeRef::InlineRecord(Box::new(record)));
        }
    }
    let canonical = unsafe { clang_getCanonicalType(ty) };
    if canonical.kind != ty.kind {
        return type_ref(canonical);
    }
    scalar(ty).map(TypeRef::Scalar)
}

fn winrt_generic_arg(mut ty: CXType) -> Option<TypeRef> {
    ty = unsafe { clang_getCanonicalType(ty) };
    while ty.kind == CXType_Pointer {
        ty = unsafe { clang_getPointeeType(ty) };
    }
    let canonical = unsafe { clang_getCanonicalType(ty) };
    let declaration = unsafe { clang_getTypeDeclaration(canonical) };
    match cx_string(unsafe { clang_getCursorSpelling(declaration) }).as_str() {
        "HSTRING__" => Some(TypeRef::String),
        "IInspectable" => Some(TypeRef::Object),
        _ => type_ref(ty),
    }
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
