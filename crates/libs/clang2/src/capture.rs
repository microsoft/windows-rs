#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use std::ffi::{CStr, CString};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::Arc;

/// A header-owned declaration or macro, without projected types or selection policy.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DeclarationInfo {
    pub header: String,
    pub line: u32,
    pub name: String,
    pub kind: String,
    pub definition: bool,
    pub inline: bool,
    pub initializer: bool,
    pub function_macro: bool,
    pub empty_macro: bool,
    /// Type declarations scoped to a record rather than independently exported declarations.
    pub record_member: bool,
    /// Single-identifier macro chains ending at a captured native type or function declaration.
    pub macro_alias: Option<String>,
    /// Declaration-attribute macros, including single-identifier chains ending at attributes.
    pub macro_attribute: bool,
}

/// Inventories declarations whose expansion locations belong to the specified header files.
///
/// File identity, rather than a basename or path suffix, determines ownership. Discovery does
/// not validate native type contracts; pass the selected names to `capture` for that work.
pub fn discover(
    inputs: impl IntoIterator<Item = Input>,
    arguments: &[&str],
    headers: &[&str],
) -> Result<Vec<DeclarationInfo>, Error> {
    if headers.is_empty() {
        return Err(Error("discovery requires header files".into()));
    }
    let headers = headers
        .iter()
        .map(|header| c_string(header))
        .collect::<Result<Vec<_>, _>>()?;
    let _library = Library::new()?;
    let mut found = BTreeSet::new();
    let mut result = BTreeSet::new();
    for input in inputs {
        let unit = Unit::parse(&input, arguments)?;
        let files: Vec<_> = headers
            .iter()
            .enumerate()
            .filter_map(|(index, header)| {
                let file = unsafe { clang_getFile(unit.raw, header.as_ptr()) };
                if file.is_null() {
                    None
                } else {
                    found.insert(index);
                    Some(file)
                }
            })
            .collect();
        let root = unsafe { clang_getTranslationUnitCursor(unit.raw) };
        let declarations = declarations(root);
        let macro_definitions: Vec<_> = children(root)
            .into_iter()
            .filter(|cursor| unsafe { clang_getCursorKind(*cursor) == CXCursor_MacroDefinition })
            .collect();
        let names: BTreeSet<_> = declarations
            .iter()
            .filter(|cursor| {
                matches!(
                    unsafe { clang_getCursorKind(**cursor) },
                    CXCursor_FunctionDecl
                        | CXCursor_TypedefDecl
                        | CXCursor_TypeAliasDecl
                        | CXCursor_StructDecl
                        | CXCursor_UnionDecl
                        | CXCursor_ClassDecl
                        | CXCursor_EnumDecl
                )
            })
            .map(|cursor| qualified_name(*cursor))
            .collect();
        let macros: BTreeMap<_, _> = macro_definitions
            .iter()
            .map(|cursor| (qualified_name(*cursor), *cursor))
            .collect();
        let mut aliases = BTreeMap::new();
        for cursor in declarations.into_iter().chain(macro_definitions) {
            let position = unsafe { clang_getCursorLocation(cursor) };
            let mut file = std::ptr::null_mut();
            unsafe {
                clang_getExpansionLocation(
                    position,
                    &mut file,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            }
            if file.is_null()
                || !files
                    .iter()
                    .any(|expected| unsafe { clang_File_isEqual(file, *expected) != 0 })
            {
                continue;
            }
            let kind = unsafe { clang_getCursorKind(cursor) };
            let definition =
                (kind == CXCursor_MacroDefinition).then(|| macro_definition(unit.raw, cursor));
            let target = definition.as_ref().and_then(|_| {
                macro_target(
                    unit.raw,
                    &qualified_name(cursor),
                    &macros,
                    &names,
                    &mut aliases,
                )
            });
            let location = expansion_location(position);
            result.insert(DeclarationInfo {
                header: location.file,
                line: location.line,
                name: qualified_name(cursor),
                kind: string(unsafe { clang_getCursorKindSpelling(kind) }),
                definition: unsafe { clang_isCursorDefinition(cursor) != 0 },
                inline: kind == CXCursor_FunctionDecl
                    && unsafe { clang_Cursor_isFunctionInlined(cursor) != 0 },
                initializer: kind == CXCursor_VarDecl
                    && unsafe {
                        clang_Cursor_isNull(clang_Cursor_getVarDeclInitializer(cursor)) == 0
                    },
                function_macro: definition
                    .as_ref()
                    .is_some_and(|definition| definition.function),
                empty_macro: definition
                    .as_ref()
                    .is_some_and(|definition| definition.empty),
                macro_attribute: matches!(target, Some(MacroTarget::Attribute)),
                record_member: matches!(
                    unsafe { clang_getCursorKind(clang_getCursorSemanticParent(cursor)) },
                    CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl
                ),
                macro_alias: match target {
                    Some(MacroTarget::Declaration(name)) => Some(name),
                    _ => None,
                },
            });
        }
    }
    for (index, header) in headers.iter().enumerate() {
        if !found.contains(&index) {
            return Err(Error(format!(
                "discovery header was not included: {}",
                header.to_string_lossy()
            )));
        }
    }
    Ok(result.into_iter().collect())
}

#[derive(Clone)]
enum MacroTarget {
    Declaration(String),
    Attribute,
}

struct MacroDefinition {
    function: bool,
    empty: bool,
    alias: Option<String>,
    attribute: bool,
}

fn macro_target(
    unit: CXTranslationUnit,
    name: &str,
    macros: &BTreeMap<String, CXCursor>,
    declarations: &BTreeSet<String>,
    cache: &mut BTreeMap<String, Option<MacroTarget>>,
) -> Option<MacroTarget> {
    let mut target = name.to_string();
    let mut visited = BTreeSet::new();
    let result = loop {
        if let Some(result) = cache.get(&target) {
            break result.clone();
        }
        if !visited.insert(target.clone()) {
            break None;
        }
        let Some(cursor) = macros.get(&target) else {
            break declarations
                .contains(&target)
                .then_some(MacroTarget::Declaration(target));
        };
        let definition = macro_definition(unit, *cursor);
        if definition.attribute {
            break Some(MacroTarget::Attribute);
        }
        let Some(next) = definition.alias else {
            break None;
        };
        target = next;
    };
    for name in visited {
        cache.insert(name, result.clone());
    }
    result
}

fn macro_definition(unit: CXTranslationUnit, cursor: CXCursor) -> MacroDefinition {
    let mut tokens = std::ptr::null_mut();
    let mut count = 0;
    unsafe { clang_tokenize(unit, clang_getCursorExtent(cursor), &mut tokens, &mut count) };
    let significant: Vec<_> = (0..count)
        .map(|index| unsafe { *tokens.add(index as usize) })
        .filter(|token| unsafe { clang_getTokenKind(*token) } != CXToken_Comment)
        .collect();
    let spellings: Vec<_> = significant
        .iter()
        .map(|token| {
            string(unsafe { clang_getTokenSpelling(unit, *token) })
                .replace("\\\r\n", "")
                .replace("\\\n", "")
        })
        .collect();
    // The libclang function-like query uses current state, which can lose undefined helpers.
    let function = if spellings.get(1).is_some_and(|token| token == "(") {
        let (file, end) =
            position(unsafe { clang_getRangeEnd(clang_getTokenExtent(unit, significant[0])) });
        let (next_file, start) = position(unsafe { clang_getTokenLocation(unit, significant[1]) });
        let mut size = 0;
        let contents = unsafe { clang_getFileContents(unit, file, &mut size) };
        !contents.is_null()
            && file == next_file
            && end <= start
            && start as usize <= size
            && unsafe { std::slice::from_raw_parts(contents.cast::<u8>(), size) }
                [end as usize..start as usize]
                .split_inclusive(|byte| *byte == b'\n')
                .all(|part| part == b"\\\n" || part == b"\\\r\n")
    } else {
        false
    };
    let alias = if let [_, token] = significant.as_slice()
        && !function
        && unsafe { clang_getTokenKind(*token) } == CXToken_Identifier
    {
        Some(spellings[1].clone())
    } else {
        None
    };
    let attribute = !function
        && (matches!(
            spellings.get(1).map(String::as_str),
            Some("__declspec" | "__attribute__")
        ) || spellings
            .get(1..3)
            .is_some_and(|tokens| tokens == ["[", "["]));
    unsafe { clang_disposeTokens(unit, tokens, count) };
    MacroDefinition {
        function,
        empty: significant.len() <= 1,
        alias,
        attribute,
    }
}

/// Captures the native closure of explicitly named roots and their cross-TU observations.
///
/// Roots use qualified native names, for example `API::Packet`. References and output exclusions
/// do not participate in capture. Unknown roots and compiler errors are reported as errors.
pub fn capture(
    inputs: impl IntoIterator<Item = Input>,
    arguments: &[&str],
    roots: &[&str],
) -> Result<Snapshot, Error> {
    let report = capture_report(inputs, arguments, roots)?;
    if !report.rejected.is_empty() {
        return Err(Error(
            report.rejected.into_values().collect::<Vec<_>>().join("\n"),
        ));
    }
    Ok(report.snapshot.unwrap())
}

/// Captures valid roots while reporting failed macro probes by compiler diagnostic ownership.
///
/// All remaining native evidence still requires resolution. This does not suppress source errors,
/// native conflicts, or unavailable dependencies.
pub fn capture_report(
    inputs: impl IntoIterator<Item = Input>,
    arguments: &[&str],
    roots: &[&str],
) -> Result<CaptureReport, Error> {
    let mut inputs: Vec<_> = inputs.into_iter().collect();
    inputs.sort_by(|left, right| left.name.cmp(&right.name));
    if inputs.is_empty() || roots.is_empty() {
        return Err(Error("capture requires inputs and named roots".into()));
    }
    if roots.iter().any(|root| root.is_empty()) {
        return Err(Error("native root names cannot be empty".into()));
    }
    let roots: BTreeSet<_> = roots.iter().copied().collect();
    for pair in inputs.windows(2) {
        if pair[0].name == pair[1].name {
            return Err(Error(format!("duplicate input `{}`", pair[0].name)));
        }
    }
    let _library = Library::new()?;
    let mut units = inputs
        .iter()
        .map(|input| Unit::parse(input, arguments))
        .collect::<Result<Vec<_>, _>>()?;
    let mut macros = vec![];
    let mut rejected = BTreeMap::new();
    let mut parses = units.len();
    for (input, unit) in inputs.iter().zip(&mut units) {
        let mut selected = BTreeMap::new();
        let mut function_macros = BTreeSet::new();
        let mut sdk_sal = false;
        let mut sal_capture = false;
        for cursor in children(unsafe { clang_getTranslationUnitCursor(unit.raw) }) {
            if unsafe { clang_getCursorKind(cursor) } != CXCursor_MacroDefinition {
                continue;
            }
            let name = string(unsafe { clang_getCursorSpelling(cursor) });
            sdk_sal |= name == "_SAL_VERSION";
            sal_capture |= name == "__CLANG2_SAL_CAPTURE";
            if roots.contains(&name.as_str()) {
                if macro_definition(unit.raw, cursor).function {
                    function_macros.insert(name.clone());
                } else {
                    function_macros.remove(&name);
                }
                selected.insert(name, location(cursor));
            }
        }
        if sdk_sal && !sal_capture {
            return Err(Error(
                "SDK SAL requires the windows-clang2 src/sal.h capture header".into(),
            ));
        }
        for name in function_macros {
            rejected.insert(
                name.clone(),
                format!("function-like macro `{name}` cannot be a constant root"),
            );
        }
        if !selected.is_empty() {
            *unit = Unit::probe(
                input,
                arguments,
                &selected,
                &BTreeSet::new(),
                &mut rejected,
                &mut parses,
            )?;
            let mut pointers = BTreeSet::new();
            for cursor in children(unsafe { clang_getTranslationUnitCursor(unit.raw) }) {
                if unsafe { clang_getCursorKind(cursor) } != CXCursor_VarDecl
                    || unsafe {
                        clang_getCanonicalType(clang_getCursorType(
                            clang_Cursor_getVarDeclInitializer(cursor),
                        ))
                    }
                    .kind
                        != CXType_Pointer
                {
                    continue;
                }
                let name = string(unsafe { clang_getCursorSpelling(cursor) });
                if let Some(name) = name.strip_prefix("__clang2_value_")
                    && selected.contains_key(name)
                    && matches!(evaluate(cursor), Value::Unavailable(_))
                {
                    pointers.insert(name.to_string());
                }
            }
            if !pointers.is_empty() {
                *unit = Unit::probe(
                    input,
                    arguments,
                    &selected,
                    &pointers,
                    &mut rejected,
                    &mut parses,
                )?;
            }
        }
        macros.push(selected);
    }
    let roots: BTreeSet<_> = roots
        .into_iter()
        .filter(|root| !rejected.contains_key(*root))
        .collect();
    if roots.is_empty() {
        return Ok(CaptureReport {
            snapshot: None,
            rejected,
            parses,
        });
    }
    let target = units[0].target.clone();
    if units.iter().any(|unit| unit.target != target) {
        return Err(Error("capture inputs have different targets".into()));
    }
    let mut capture = Capture {
        units: &units,
        declarations: vec![],
        entities: vec![],
        entity_cursors: (0..units.len()).map(|_| HashMap::new()).collect(),
        contexts: (0..units.len()).map(|_| HashMap::new()).collect(),
        flag_enums: (0..units.len()).map(|_| BTreeSet::new()).collect(),
        macros,
        identities: BTreeMap::new(),
        interned: (0..units.len()).map(|_| HashMap::new()).collect(),
        pending: VecDeque::new(),
    };
    let mut selected = vec![];
    let mut found = BTreeSet::new();
    for (unit, parsed) in units.iter().enumerate() {
        let root = unsafe { clang_getTranslationUnitCursor(parsed.raw) };
        let flag_macros: BTreeSet<_> = children(root)
            .into_iter()
            .filter(|cursor| {
                (unsafe { clang_getCursorKind(*cursor) }) == CXCursor_MacroExpansion
                    && string(unsafe { clang_getCursorSpelling(*cursor) })
                        == "DEFINE_ENUM_FLAG_OPERATORS"
            })
            .map(|cursor| {
                let location = expansion_location(unsafe { clang_getCursorLocation(cursor) });
                (location.file, location.offset)
            })
            .collect();
        for cursor in declarations(root) {
            if unsafe { clang_getCursorKind(cursor) } == CXCursor_FunctionDecl
                && string(unsafe { clang_getCursorSpelling(cursor) }) == "operator|"
            {
                let location = expansion_location(unsafe { clang_getCursorLocation(cursor) });
                if flag_macros.contains(&(location.file, location.offset)) {
                    let ty = unsafe { clang_getCursorType(cursor) };
                    let result = unsafe { clang_getCanonicalType(clang_getResultType(ty)) };
                    if result.kind == CXType_Enum
                        && unsafe { clang_getNumArgTypes(ty) } == 2
                        && (0..2).all(|index| unsafe {
                            clang_equalTypes(
                                result,
                                clang_getCanonicalType(clang_getArgType(ty, index)),
                            ) != 0
                        })
                    {
                        capture.flag_enums[unit].insert(string(unsafe {
                            clang_getCursorUSR(clang_getTypeDeclaration(result))
                        }));
                    }
                }
            }
            if matches!(
                unsafe { clang_getCursorKind(cursor) },
                CXCursor_FunctionDecl | CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
            ) {
                capture.context(unit, cursor);
            }
            if matches!(
                unsafe { clang_getCursorKind(cursor) },
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl
            ) {
                for method in children(cursor)
                    .into_iter()
                    .filter(|child| unsafe { clang_getCursorKind(*child) == CXCursor_CXXMethod })
                {
                    capture.context(unit, method);
                }
            }
            let identity = string(unsafe { clang_getCursorUSR(cursor) });
            if !identity.is_empty() {
                let key = capture.candidate(unit, cursor, &identity)?;
                capture
                    .identities
                    .entry(key)
                    .or_default()
                    .push((unit, cursor));
            }
            let name = capture.name(unit, cursor);
            if roots.contains(&name.as_str())
                && (!capture.macros[unit].contains_key(&name)
                    || capture.macro_probe(unit, cursor).is_some())
            {
                found.insert(name);
                selected.push((unit, cursor));
            }
        }
    }
    for root in &roots {
        if !found.contains(*root) {
            return Err(Error(format!("native root `{root}` was not found")));
        }
    }
    let mut root_ids = BTreeSet::new();
    for (unit, cursor) in selected {
        root_ids.insert(capture.intern(unit, cursor)?);
    }
    let mut expanded = BTreeSet::new();
    while let Some((unit, cursor, id)) = capture.pending.pop_front() {
        let data = capture.declaration(unit, cursor)?;
        capture.declarations[id.0].data = data;
        let identity = &capture.declarations[id.0].candidate;
        if expanded.insert(identity.clone()) {
            let candidates = capture
                .identities
                .get(identity)
                .cloned()
                .unwrap_or_default();
            for (unit, cursor) in candidates {
                capture.intern(unit, cursor)?;
            }
        }
    }
    Ok(CaptureReport {
        snapshot: Some(Snapshot {
            entities: capture.entities,
            declarations: capture.declarations,
            roots: root_ids.into_iter().collect(),
            target,
            pointer_size: units[0].pointer_size,
            arguments: arguments.iter().map(|arg| (*arg).to_string()).collect(),
            diagnostics: units
                .iter()
                .flat_map(|unit| unit.diagnostics.clone())
                .collect(),
        }),
        rejected,
        parses,
    })
}

struct Library {
    owned: bool,
}

impl Library {
    fn new() -> Result<Self, Error> {
        let owned = !is_loaded();
        if owned {
            load().map_err(|error| Error(format!("failed to load libclang: {error}")))?;
        }
        let library = Self { owned };
        if !clang_Cursor_getVarDeclInitializer::is_loaded()
            || !clang_Cursor_isAnonymousRecordDecl::is_loaded()
            || !clang_getTranslationUnitTargetInfo::is_loaded()
            || !clang_Type_visitFields::is_loaded()
        {
            return Err(Error("loaded libclang lacks the native capture APIs; use the repository's pinned runtime".into()));
        }
        Ok(library)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        if self.owned {
            unload().unwrap();
        }
    }
}

struct Unit {
    raw: CXTranslationUnit,
    index: CXIndex,
    name: String,
    target: String,
    pointer_size: i64,
    diagnostics: Vec<String>,
    errors: Vec<CompilerError>,
}

struct CompilerError {
    location: Location,
    input: bool,
    fatal: bool,
    text: String,
}

impl Unit {
    fn parse(input: &Input, arguments: &[&str]) -> Result<Self, Error> {
        let unit = Self::parse_raw(input, arguments)?;
        if !unit.errors.is_empty() {
            return Err(Error(unit.diagnostics.join("\n")));
        }
        Ok(unit)
    }

    fn probe(
        input: &Input,
        arguments: &[&str],
        macros: &BTreeMap<String, Location>,
        pointers: &BTreeSet<String>,
        rejected: &mut BTreeMap<String, String>,
        parses: &mut usize,
    ) -> Result<Self, Error> {
        let mut arguments = arguments.to_vec();
        arguments.push("-ferror-limit=0");
        let arguments = arguments.as_slice();
        let source = |rejected: &BTreeMap<String, String>| {
            let mut source = input.source.clone();
            let mut ranges = BTreeMap::new();
            for name in macros.keys().filter(|name| !rejected.contains_key(*name)) {
                let start = source.len();
                writeln!(
                    source,
                    "\n#ifndef {name}\n#error selected macro is undefined: {name}\n#endif\n\
                     const auto& __clang2_value_{name} = ({name});"
                )
                .unwrap();
                if pointers.contains(name) {
                    writeln!(
                        source,
                        "const __INTPTR_TYPE__ __clang2_bits_{name} = (__INTPTR_TYPE__)({name});"
                    )
                    .unwrap();
                }
                ranges.insert(start, (source.len(), name));
            }
            (Input::new(&input.name, source), ranges)
        };
        let (probes, ranges) = source(rejected);
        *parses += 1;
        let unit = Self::parse_raw(&probes, arguments)?;
        if unit.errors.is_empty() {
            return Ok(unit);
        }
        for error in &unit.errors {
            let offset = error.location.offset as usize;
            let owner = ranges
                .range(..=offset)
                .next_back()
                .map(|(_, (end, name))| (end, name));
            let Some((_, name)) =
                owner.filter(|(end, _)| offset < **end && error.input && !error.fatal)
            else {
                return Err(Error(unit.diagnostics.join("\n")));
            };
            let origin = &macros[*name];
            let reason = format!(
                "macro `{name}` at {}:{}: {}",
                origin.file, origin.line, error.text
            );
            rejected
                .entry((*name).clone())
                .and_modify(|previous| {
                    previous.push('\n');
                    previous.push_str(&reason);
                })
                .or_insert(reason);
        }
        drop(unit);
        let (clean, _) = source(rejected);
        *parses += 1;
        Self::parse(&clean, arguments)
    }

    fn parse_raw(input: &Input, arguments: &[&str]) -> Result<Self, Error> {
        let name = c_string(&input.name)?;
        let source = c_string(&input.source)?;
        let args = arguments
            .iter()
            .map(|arg| c_string(arg))
            .collect::<Result<Vec<_>, _>>()?;
        let args: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
        let mut unsaved = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: source.as_ptr(),
            Length: input
                .source
                .len()
                .try_into()
                .map_err(|_| Error("input is too large".into()))?,
        };
        let count = args
            .len()
            .try_into()
            .map_err(|_| Error("too many arguments".into()))?;
        unsafe {
            let index = clang_createIndex(0, 0);
            if index.is_null() {
                return Err(Error("failed to create libclang index".into()));
            }
            let mut raw = std::ptr::null_mut();
            let code = clang_parseTranslationUnit2(
                index,
                name.as_ptr(),
                args.as_ptr(),
                count,
                &mut unsaved,
                1,
                CXTranslationUnit_SkipFunctionBodies
                    | CXTranslationUnit_DetailedPreprocessingRecord,
                &mut raw,
            );
            if code != CXError_Success || raw.is_null() {
                if !raw.is_null() {
                    clang_disposeTranslationUnit(raw);
                }
                clang_disposeIndex(index);
                return Err(Error(format!(
                    "failed to parse `{}`: libclang error {code}",
                    input.name
                )));
            }
            let mut unit = Self {
                raw,
                index,
                name: input.name.clone(),
                target: String::new(),
                pointer_size: 0,
                diagnostics: vec![],
                errors: vec![],
            };
            for i in 0..clang_getNumDiagnostics(raw) {
                let diagnostic = clang_getDiagnostic(raw, i);
                let severity = clang_getDiagnosticSeverity(diagnostic);
                let text = string(clang_formatDiagnostic(
                    diagnostic,
                    clang_defaultDiagnosticDisplayOptions(),
                ));
                if severity >= CXDiagnostic_Error {
                    let origin = clang_getDiagnosticLocation(diagnostic);
                    let (file, _) = position(origin);
                    let input_file = clang_getFile(raw, name.as_ptr());
                    unit.errors.push(CompilerError {
                        location: expansion_location(origin),
                        input: !file.is_null()
                            && !input_file.is_null()
                            && clang_File_isEqual(file, input_file) != 0,
                        fatal: severity >= CXDiagnostic_Fatal,
                        text: text.clone(),
                    });
                }
                unit.diagnostics.push(text);
                clang_disposeDiagnostic(diagnostic);
            }
            let target = clang_getTranslationUnitTargetInfo(raw);
            if target.is_null() {
                return Err(Error(format!(
                    "target information unavailable for `{}`",
                    input.name
                )));
            }
            unit.target = string(clang_TargetInfo_getTriple(target));
            unit.pointer_size = i64::from(clang_TargetInfo_getPointerWidth(target)) / 8;
            clang_TargetInfo_dispose(target);
            Ok(unit)
        }
    }
}

impl Drop for Unit {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.raw);
            clang_disposeIndex(self.index);
        }
    }
}

struct Capture<'a> {
    units: &'a [Unit],
    declarations: Vec<Declaration>,
    entities: Vec<Entity>,
    entity_cursors: Vec<HashMap<u32, Vec<(CXCursor, EntityId)>>>,
    contexts: Vec<HashMap<(CXFile, u32), AnnotationContext>>,
    flag_enums: Vec<BTreeSet<String>>,
    macros: Vec<BTreeMap<String, Location>>,
    identities: BTreeMap<String, Vec<(usize, CXCursor)>>,
    interned: Vec<HashMap<u32, Vec<(CXCursor, Id)>>>,
    pending: VecDeque<(usize, CXCursor, Id)>,
}

struct AnnotationContext {
    distance: u32,
    parameters: Result<Arc<[String]>, String>,
    prefix_owners: Vec<CXCursor>,
}

fn position(location: CXSourceLocation) -> (CXFile, u32) {
    let mut file = std::ptr::null_mut();
    let mut offset = 0;
    unsafe {
        clang_getExpansionLocation(
            location,
            &mut file,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut offset,
        );
    }
    (file, offset)
}

impl Capture<'_> {
    fn context(&mut self, unit: usize, cursor: CXCursor) {
        let alias = matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
        );
        let range = unsafe { clang_getCursorExtent(cursor) };
        let (file, start) = position(unsafe { clang_getRangeStart(range) });
        let (end_file, end) = position(unsafe { clang_getRangeEnd(range) });
        let parameters: Vec<_> = children(cursor)
            .into_iter()
            .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
            .collect();
        let mut context = None;
        for owner in std::iter::once(cursor).chain(parameters.iter().copied()) {
            for attr in children(owner)
                .into_iter()
                .filter(|child| unsafe { clang_getCursorKind(*child) == CXCursor_AnnotateAttr })
            {
                let origin = position(unsafe { clang_getCursorLocation(attr) });
                if file.is_null() || file != end_file {
                    self.contexts[unit].insert(
                        origin,
                        AnnotationContext {
                            distance: 0,
                            parameters: Err("callable source range is unavailable".into()),
                            prefix_owners: vec![],
                        },
                    );
                    continue;
                }
                if !alias && (origin.0 != file || origin.1 >= end) {
                    continue;
                }
                // Prefix attributes can precede a macro-started declaration extent. Inherited
                // attributes keep the nearest attached declaration's original parameter context.
                let distance = if alias {
                    0
                } else {
                    start.saturating_sub(origin.1)
                };
                let context = context.get_or_insert_with(|| {
                    parameters
                        .iter()
                        .map(|parameter| string(unsafe { clang_getCursorSpelling(*parameter) }))
                        .collect::<Arc<[String]>>()
                });
                if let Some(previous) = self.contexts[unit].get_mut(&origin)
                    && previous.distance <= distance
                {
                    if previous.distance == distance {
                        if previous
                            .parameters
                            .as_ref()
                            .is_ok_and(|previous| previous != context)
                        {
                            previous.parameters = Err(format!(
                                "ambiguous annotation context for `{}`",
                                qualified_name(cursor)
                            ));
                        }
                        if distance > 0 {
                            previous.prefix_owners.push(owner);
                        }
                    }
                } else {
                    self.contexts[unit].insert(
                        origin,
                        AnnotationContext {
                            distance,
                            parameters: Ok(context.clone()),
                            prefix_owners: if distance > 0 { vec![owner] } else { vec![] },
                        },
                    );
                }
            }
        }
    }

    fn macro_probe(&self, unit: usize, cursor: CXCursor) -> Option<&str> {
        if unsafe { clang_getCursorKind(cursor) } != CXCursor_VarDecl {
            return None;
        }
        let name = qualified_name(cursor);
        self.macros[unit]
            .get_key_value(name.strip_prefix("__clang2_value_")?)
            .map(|(name, _)| name.as_str())
    }

    fn name(&self, unit: usize, cursor: CXCursor) -> String {
        self.macro_probe(unit, cursor)
            .map_or_else(|| qualified_name(cursor), str::to_string)
    }

    fn candidate(&self, unit: usize, cursor: CXCursor, identity: &str) -> Result<String, Error> {
        let parent = unsafe { clang_getCursorSemanticParent(cursor) };
        if unsafe { clang_Cursor_isAnonymous(cursor) } != 0
            && matches!(
                unsafe { clang_getCursorKind(cursor) },
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl
            )
            && matches!(
                unsafe { clang_getCursorKind(parent) },
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl
            )
        {
            // Sibling anonymous records can share a USR; their owner and field slot distinguish them.
            let slot = record_fields(parent)
                .iter()
                .position(|field| unsafe {
                    clang_equalCursors(
                        clang_getCanonicalCursor(field_declaration(*field)),
                        clang_getCanonicalCursor(cursor),
                    ) != 0
                })
                .ok_or_else(|| {
                    Error(format!(
                        "anonymous member ownership unavailable for `{}`",
                        qualified_name(cursor)
                    ))
                })?;
            let parent_identity = string(unsafe { clang_getCursorUSR(parent) });
            let parent = self.candidate(unit, parent, &parent_identity)?;
            return Ok(format!("anonymous:{parent}:{slot}"));
        }
        Ok(if let Some(name) = self.macro_probe(unit, cursor) {
            format!("macro:{name}")
        } else {
            candidate(&self.units[unit].name, cursor, identity)
        })
    }

    fn intern(&mut self, unit: usize, cursor: CXCursor) -> Result<Id, Error> {
        let canonical = unsafe { clang_getCanonicalCursor(cursor) };
        let hash = unsafe { clang_hashCursor(cursor) };
        let bucket = self.interned[unit].entry(hash).or_default();
        if let Some((_, id)) = bucket
            .iter()
            .find(|(existing, _)| unsafe { clang_equalCursors(*existing, cursor) } != 0)
        {
            return Ok(*id);
        }
        let canonical_bucket = self.entity_cursors[unit]
            .entry(unsafe { clang_hashCursor(canonical) })
            .or_default();
        let entity = if let Some((_, entity)) = canonical_bucket
            .iter()
            .find(|(existing, _)| unsafe { clang_equalCursors(*existing, canonical) } != 0)
        {
            *entity
        } else {
            let entity = EntityId(self.entities.len());
            self.entities.push(Entity {
                observations: vec![],
            });
            canonical_bucket.push((canonical, entity));
            entity
        };
        let identity = string(unsafe { clang_getCursorUSR(canonical) });
        if identity.is_empty() {
            return Err(Error(format!(
                "native identity unavailable for `{}` in `{}`",
                qualified_name(cursor),
                self.units[unit].name
            )));
        }
        let id = Id(self.declarations.len());
        bucket.push((cursor, id));
        self.entities[entity.0].observations.push(id);
        let name = self.name(unit, cursor);
        let origin = self
            .macro_probe(unit, cursor)
            .map(|name| &self.macros[unit][name]);
        let owner = origin.map_or_else(
            || expansion_location(unsafe { clang_getCursorLocation(cursor) }).file,
            |location| location.file.clone(),
        );
        let location = origin.cloned().unwrap_or_else(|| location(cursor));
        self.declarations.push(Declaration {
            entity,
            owner,
            location,
            candidate: self.candidate(unit, canonical, &identity)?,
            name,
            identity,
            unit: self.units[unit].name.clone(),
            annotations: self.annotations(
                unit,
                cursor,
                matches!(
                    unsafe { clang_getCursorKind(cursor) },
                    CXCursor_FunctionDecl | CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
                ),
            )?,
            data: DeclarationData::Pending,
        });
        self.pending.push_back((unit, cursor, id));
        Ok(id)
    }

    fn declaration(&mut self, unit: usize, cursor: CXCursor) -> Result<DeclarationData, Error> {
        let kind = unsafe { clang_getCursorKind(cursor) };
        Ok(match kind {
            CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl => {
                let complete = unsafe { clang_isCursorDefinition(cursor) } != 0;
                let mut fields = vec![];
                let mut bases = vec![];
                let mut methods = vec![];
                let mut unavailable = vec![];
                if unsafe { clang_Cursor_isNull(clang_getSpecializedCursorTemplate(cursor)) } == 0 {
                    unavailable.push("template specialization capture is not implemented".into());
                }
                if complete {
                    for field in record_fields(cursor) {
                        let offset = unsafe { clang_Cursor_getOffsetOfField(field) };
                        let bit_width = (unsafe { clang_Cursor_isBitField(field) } != 0)
                            .then(|| unsafe { clang_getFieldDeclBitWidth(field) });
                        if offset < 0 || bit_width.is_some_and(|width| width < 0) {
                            unavailable.push("field layout unavailable".into());
                        }
                        let ty = unsafe { clang_getCursorType(field) };
                        // Implicit anonymous members have synthesized spellings containing file paths.
                        let anonymous = unsafe {
                            clang_Cursor_isAnonymousRecordDecl(clang_getTypeDeclaration(ty)) != 0
                        };
                        fields.push(Field {
                            name: if anonymous {
                                String::new()
                            } else {
                                string(unsafe { clang_getCursorSpelling(field) })
                            },
                            ty: self.written_type(unit, field, ty)?,
                            offset,
                            bit_width,
                            annotations: self.annotations(unit, field, false)?,
                        });
                    }
                }
                for child in children(cursor) {
                    match unsafe { clang_getCursorKind(child) } {
                        CXCursor_CXXBaseSpecifier => {
                            bases.push(self.ty(unit, unsafe { clang_getCursorType(child) })?);
                            if unsafe { clang_isVirtualBase(child) } != 0 {
                                unavailable.push("virtual base layout is not implemented".into());
                            }
                        }
                        CXCursor_CXXMethod => {
                            let annotations = self.annotations(unit, child, true)?;
                            methods.push(Method {
                                property: method_property(child, &annotations)?,
                                name: string(unsafe { clang_getCursorSpelling(child) }),
                                ty: self.written_type(unit, child, unsafe {
                                    clang_getCursorType(child)
                                })?,
                                canonical: self.ty(unit, unsafe {
                                    clang_getCanonicalType(clang_getCursorType(child))
                                })?,
                                parameters: self.parameters(unit, child)?,
                                virtual_method: unsafe { clang_CXXMethod_isVirtual(child) } != 0,
                                static_method: unsafe { clang_CXXMethod_isStatic(child) } != 0,
                                const_method: unsafe { clang_CXXMethod_isConst(child) } != 0,
                                ref_qualifier: unsafe {
                                    clang_Type_getCXXRefQualifier(clang_getCursorType(child))
                                },
                                pure: unsafe { clang_CXXMethod_isPureVirtual(child) } != 0,
                                overrides: overridden_methods(child),
                                annotations,
                            });
                        }
                        CXCursor_Constructor | CXCursor_Destructor => unavailable.push(format!(
                            "{} capture is not implemented",
                            string(unsafe {
                                clang_getCursorKindSpelling(clang_getCursorKind(child))
                            })
                        )),
                        _ => {}
                    }
                }
                let layout = complete
                    .then(|| layout(unsafe { clang_getCursorType(cursor) }))
                    .flatten();
                if complete && layout.is_none() {
                    unavailable.push("record layout unavailable".into());
                }
                if bases.len() > 1 || (!bases.is_empty() && !fields.is_empty()) {
                    unavailable
                        .push("data-bearing or multiple inheritance is not implemented".into());
                }
                DeclarationData::Record {
                    kind: match kind {
                        CXCursor_StructDecl => RecordKind::Struct,
                        CXCursor_UnionDecl => RecordKind::Union,
                        CXCursor_ClassDecl => RecordKind::Class,
                        _ => unreachable!(),
                    },
                    unnamed: unsafe { clang_Cursor_isAnonymous(cursor) } != 0,
                    complete,
                    layout,
                    fields,
                    bases,
                    methods,
                    guid: uuid(cursor)?,
                    unavailable,
                }
            }
            CXCursor_TypedefDecl | CXCursor_TypeAliasDecl => DeclarationData::Alias {
                target: self.written_type(unit, cursor, unsafe {
                    clang_getTypedefDeclUnderlyingType(cursor)
                })?,
                canonical: self.ty(unit, unsafe {
                    clang_getCanonicalType(clang_getTypedefDeclUnderlyingType(cursor))
                })?,
                parameters: self.parameters(unit, cursor)?,
            },
            CXCursor_EnumDecl => DeclarationData::Enum {
                annotations: children(cursor)
                    .into_iter()
                    .filter(
                        |child| unsafe { clang_getCursorKind(*child) } == CXCursor_EnumConstantDecl,
                    )
                    .map(|child| self.annotations(unit, child, false))
                    .collect::<Result<_, _>>()?,
                complete: unsafe { clang_isCursorDefinition(cursor) } != 0,
                scoped: unsafe { clang_EnumDecl_isScoped(cursor) } != 0,
                flags: children(cursor)
                    .iter()
                    .any(|child| unsafe { clang_getCursorKind(*child) == CXCursor_FlagEnum })
                    || self.flag_enums[unit]
                        .contains(&string(unsafe { clang_getCursorUSR(cursor) })),
                repr: self.ty(unit, unsafe { clang_getEnumDeclIntegerType(cursor) })?,
                variants: children(cursor)
                    .into_iter()
                    .filter(
                        |child| unsafe { clang_getCursorKind(*child) } == CXCursor_EnumConstantDecl,
                    )
                    .map(|child| {
                        let size =
                            unsafe { clang_Type_getSizeOf(clang_getEnumDeclIntegerType(cursor)) };
                        let value = if (1..=8).contains(&size) {
                            Value::Integer(unsafe { clang_getEnumConstantDeclUnsignedValue(child) })
                        } else {
                            Value::Unavailable(
                                "enum values require a known width of at most 64 bits".into(),
                            )
                        };
                        (string(unsafe { clang_getCursorSpelling(child) }), value)
                    })
                    .collect(),
            },
            CXCursor_FunctionDecl => DeclarationData::Function {
                inline: unsafe { clang_Cursor_isFunctionInlined(cursor) } != 0,
                ty: self.written_type(unit, cursor, unsafe { clang_getCursorType(cursor) })?,
                canonical: self.ty(unit, unsafe {
                    clang_getCanonicalType(clang_getCursorType(cursor))
                })?,
                link_name: string(unsafe { clang_Cursor_getMangling(cursor) }),
                parameters: self.parameters(unit, cursor)?,
            },
            CXCursor_VarDecl => {
                let source = if let Some(name) = self.macro_probe(unit, cursor) {
                    let source = unsafe { clang_Cursor_getVarDeclInitializer(cursor) };
                    if unsafe { clang_Cursor_isNull(source) } != 0 {
                        return Err(Error(format!(
                            "selected macro probe `{name}` has no initializer in `{}`",
                            self.units[unit].name
                        )));
                    }
                    source
                } else {
                    cursor
                };
                let ty = unsafe { clang_getCursorType(source) };
                DeclarationData::Variable {
                    ty: self.written_type(unit, source, ty)?,
                    canonical: self.ty(unit, unsafe { clang_getCanonicalType(ty) })?,
                    value: {
                        let value = evaluate(cursor);
                        let name = self.name(unit, cursor);
                        if matches!(value, Value::Unavailable(_))
                            && self.macro_probe(unit, cursor).is_some()
                            && unsafe {
                                clang_getCanonicalType(clang_getCursorType(
                                    clang_Cursor_getVarDeclInitializer(cursor),
                                ))
                            }
                            .kind
                                == CXType_Pointer
                        {
                            let bits = format!("__clang2_bits_{name}");
                            let bits = children(unsafe {
                                clang_getTranslationUnitCursor(self.units[unit].raw)
                            })
                            .into_iter()
                            .find(|cursor| {
                                string(unsafe { clang_getCursorSpelling(*cursor) }) == bits
                            })
                            .ok_or_else(|| {
                                Error(format!("missing pointer constant probe for `{name}`"))
                            })?;
                            evaluate(bits)
                        } else {
                            value
                        }
                    },
                }
            }
            _ => DeclarationData::Unavailable(format!(
                "{} capture is not implemented",
                string(unsafe { clang_getCursorKindSpelling(kind) })
            )),
        })
    }

    fn parameters(&self, unit: usize, cursor: CXCursor) -> Result<Vec<Parameter>, Error> {
        let params: Vec<_> = children(cursor)
            .into_iter()
            .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
            .collect();
        if params.is_empty() {
            let mut ty = unsafe {
                if matches!(
                    clang_getCursorKind(cursor),
                    CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
                ) {
                    clang_getTypedefDeclUnderlyingType(cursor)
                } else {
                    clang_getCursorType(cursor)
                }
            };
            while matches!(ty.kind, CXType_Elaborated | CXType_Pointer) {
                ty = unsafe {
                    if ty.kind == CXType_Elaborated {
                        clang_Type_getNamedType(ty)
                    } else {
                        clang_getPointeeType(ty)
                    }
                };
            }
            let mut canonical = unsafe { clang_getCanonicalType(ty) };
            if canonical.kind == CXType_Pointer {
                canonical = unsafe { clang_getPointeeType(canonical) };
            }
            if ty.kind == CXType_Typedef && unsafe { clang_getNumArgTypes(canonical) } >= 0 {
                let declaration = unsafe { clang_getTypeDeclaration(ty) };
                if unsafe { clang_equalCursors(declaration, cursor) } == 0 {
                    return self.parameters(unit, declaration);
                }
            }
            if let Some(declaration) = callable_reference(cursor, canonical)? {
                return self.parameters(unit, declaration);
            }
        }
        let comments = parameter_comments(cursor, &params);
        params
            .into_iter()
            .zip(comments)
            .map(|(parameter, comments)| {
                let mut annotations = self.annotations(unit, parameter, true)?;
                annotations.extend(comments);
                Ok(Parameter {
                    name: string(unsafe { clang_getCursorSpelling(parameter) }),
                    annotations,
                })
            })
            .collect()
    }

    fn written_type(&mut self, unit: usize, cursor: CXCursor, ty: CXType) -> Result<Type, Error> {
        let mut result = self.ty(unit, ty)?;
        let mut written = &mut result;
        let mut native = ty;
        while matches!(
            written.kind,
            TypeKind::Pointer(_) | TypeKind::LValueReference(_) | TypeKind::RValueReference(_)
        ) {
            written = match &mut written.kind {
                TypeKind::Pointer(target)
                | TypeKind::LValueReference(target)
                | TypeKind::RValueReference(target) => target,
                _ => unreachable!(),
            };
            native = unsafe { clang_getPointeeType(native) };
        }
        if matches!(written.kind, TypeKind::Function { .. }) {
            if let Some(declaration) =
                callable_reference(cursor, unsafe { clang_getCanonicalType(native) })?
            {
                written.kind = TypeKind::Named(self.intern(unit, declaration)?);
            } else if let TypeKind::Function { parameters, .. } = &mut written.kind {
                let declarations: Vec<_> = children(cursor)
                    .into_iter()
                    .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
                    .collect();
                if declarations.len() == parameters.len() {
                    for (parameter, declaration) in parameters.iter_mut().zip(declarations) {
                        *parameter = self.written_type(unit, declaration, unsafe {
                            clang_getCursorType(declaration)
                        })?;
                    }
                }
            }
        }
        Ok(result)
    }

    fn annotations(
        &self,
        unit: usize,
        cursor: CXCursor,
        callable: bool,
    ) -> Result<Vec<Annotation>, Error> {
        let attrs: Vec<_> = children(cursor)
            .into_iter()
            .filter(|attr| unsafe { clang_getCursorKind(*attr) } == CXCursor_AnnotateAttr)
            .collect();
        let range = unsafe { clang_getCursorExtent(cursor) };
        let start = position(unsafe { clang_getRangeStart(range) });
        let end = position(unsafe { clang_getRangeEnd(range) });
        let owned = |attr: CXCursor| {
            let origin = position(unsafe { clang_getCursorLocation(attr) });
            (origin.0 == start.0 && origin.0 == end.0 && start.1 <= origin.1 && origin.1 < end.1)
                || self.contexts[unit].get(&origin).is_some_and(|context| {
                    context
                        .prefix_owners
                        .iter()
                        .any(|owner| unsafe { clang_equalCursors(*owner, cursor) } != 0)
                })
        };
        let has_owned = attrs.iter().any(|attr| owned(*attr));
        let mut result = attrs
            .into_iter()
            .filter(|attr| !has_owned || owned(*attr))
            .map(|attr| {
                let origin = unsafe { clang_getCursorLocation(attr) };
                let location = expansion_location(origin);
                let context = if callable {
                    self.contexts[unit]
                        .get(&position(origin))
                        .ok_or_else(|| {
                            Error(format!(
                                "annotation context unavailable at {}:{}",
                                location.file, location.line
                            ))
                        })?
                        .parameters
                        .as_ref()
                        .map_err(|reason| Error(reason.clone()))?
                        .clone()
                } else {
                    Arc::default()
                };
                let text = string(unsafe { clang_getCursorSpelling(attr) });
                let text = text.strip_suffix("()").unwrap_or(&text).to_string();
                Ok(Annotation {
                    source: AnnotationSource::Sal,
                    text,
                    context,
                    location,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        if matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_FunctionDecl | CXCursor_CXXMethod
        ) {
            result.extend(declaration_comments(cursor));
            let mut ty = unsafe { clang_getCursorType(cursor) };
            while ty.kind == CXType_Typedef {
                let alias = unsafe { clang_getTypeDeclaration(ty) };
                result.extend(self.annotations(unit, alias, true)?);
                ty = unsafe { clang_getTypedefDeclUnderlyingType(alias) };
                if ty.kind == CXType_Elaborated {
                    ty = unsafe { clang_Type_getNamedType(ty) };
                }
            }
        }
        Ok(result)
    }

    fn ty(&mut self, unit: usize, ty: CXType) -> Result<Type, Error> {
        let qualifiers = Qualifiers {
            constant: unsafe { clang_isConstQualifiedType(ty) } != 0,
            volatile: unsafe { clang_isVolatileQualifiedType(ty) } != 0,
            restrict: unsafe { clang_isRestrictQualifiedType(ty) } != 0,
        };
        let kind = match ty.kind {
            CXType_Auto if unsafe { clang_getCanonicalType(ty) }.kind != CXType_Auto => {
                return self.ty(unit, unsafe { clang_getCanonicalType(ty) });
            }
            CXType_Unexposed
                if matches!(
                    unsafe { clang_getCanonicalType(ty) }.kind,
                    CXType_Void..=CXType_LongDouble
                ) =>
            {
                let mut canonical = self.ty(unit, unsafe { clang_getCanonicalType(ty) })?;
                canonical.qualifiers.constant |= qualifiers.constant;
                canonical.qualifiers.volatile |= qualifiers.volatile;
                canonical.qualifiers.restrict |= qualifiers.restrict;
                return Ok(canonical);
            }
            CXType_Unexposed
                if unsafe { clang_getCanonicalType(ty) }.kind == CXType_Unexposed
                    && string(unsafe {
                        clang_getTypeSpelling(clang_getUnqualifiedType(clang_getCanonicalType(ty)))
                    }) == "char8_t" =>
            {
                // libclang has no CXType kind for the C++20 builtin char8_t.
                TypeKind::Builtin {
                    kind: "Char8".into(),
                    layout: layout(ty),
                }
            }
            CXType_Elaborated => {
                let mut named = self.ty(unit, unsafe { clang_Type_getNamedType(ty) })?;
                named.qualifiers.constant |= qualifiers.constant;
                named.qualifiers.volatile |= qualifiers.volatile;
                named.qualifiers.restrict |= qualifiers.restrict;
                return Ok(named);
            }
            CXType_Void | CXType_Bool | CXType_Char_U | CXType_UChar | CXType_Char16
            | CXType_Char32 | CXType_UShort | CXType_UInt | CXType_ULong | CXType_ULongLong
            | CXType_UInt128 | CXType_Char_S | CXType_SChar | CXType_WChar | CXType_Short
            | CXType_Int | CXType_Long | CXType_LongLong | CXType_Int128 | CXType_Float
            | CXType_Double | CXType_LongDouble => TypeKind::Builtin {
                kind: string(unsafe { clang_getTypeKindSpelling(ty.kind) }),
                layout: layout(ty),
            },
            CXType_Record | CXType_Enum | CXType_Typedef => {
                TypeKind::Named(self.intern(unit, unsafe { clang_getTypeDeclaration(ty) })?)
            }
            CXType_Unexposed if unsafe { clang_getCanonicalType(ty) }.kind == CXType_Record => {
                TypeKind::Named(self.intern(unit, unsafe {
                    clang_getTypeDeclaration(clang_getCanonicalType(ty))
                })?)
            }
            CXType_Pointer | CXType_LValueReference | CXType_RValueReference => {
                let target = Box::new(self.ty(unit, unsafe { clang_getPointeeType(ty) })?);
                match ty.kind {
                    CXType_Pointer => TypeKind::Pointer(target),
                    CXType_LValueReference => TypeKind::LValueReference(target),
                    _ => TypeKind::RValueReference(target),
                }
            }
            CXType_ConstantArray | CXType_IncompleteArray => {
                let length = if ty.kind == CXType_ConstantArray {
                    Some(
                        unsafe { clang_getArraySize(ty) }
                            .try_into()
                            .map_err(|_| Error("constant array extent is unavailable".into()))?,
                    )
                } else {
                    None
                };
                TypeKind::Array {
                    length,
                    element: Box::new(self.ty(unit, unsafe { clang_getArrayElementType(ty) })?),
                }
            }
            CXType_FunctionProto | CXType_FunctionNoProto => TypeKind::Function {
                prototype: ty.kind == CXType_FunctionProto,
                convention: unsafe { clang_getFunctionTypeCallingConv(ty) },
                exception_specification: unsafe { clang_getExceptionSpecificationType(ty) },
                variadic: unsafe { clang_isFunctionTypeVariadic(ty) } != 0,
                result: Box::new(self.ty(unit, unsafe { clang_getResultType(ty) })?),
                parameters: (0..unsafe { clang_getNumArgTypes(ty) })
                    .map(|index| self.ty(unit, unsafe { clang_getArgType(ty, index as u32) }))
                    .collect::<Result<_, _>>()?,
            },
            _ => TypeKind::Unavailable(format!(
                "{} type `{}` capture is not implemented",
                string(unsafe { clang_getTypeKindSpelling(ty.kind) }),
                string(unsafe { clang_getTypeSpelling(ty) }),
            )),
        };
        Ok(Type { qualifiers, kind })
    }
}

unsafe fn field_declaration(field: CXCursor) -> CXCursor {
    let mut ty = unsafe { clang_getCanonicalType(clang_getCursorType(field)) };
    loop {
        ty = match ty.kind {
            CXType_ConstantArray | CXType_IncompleteArray => unsafe {
                clang_getArrayElementType(ty)
            },
            CXType_Pointer | CXType_LValueReference | CXType_RValueReference => unsafe {
                clang_getPointeeType(ty)
            },
            _ => return unsafe { clang_getTypeDeclaration(ty) },
        };
    }
}

fn overridden_methods(cursor: CXCursor) -> Vec<String> {
    let mut raw = std::ptr::null_mut();
    let mut count = 0;
    unsafe { clang_getOverriddenCursors(cursor, &mut raw, &mut count) };
    let mut methods: Vec<_> = (0..count)
        .map(|index| string(unsafe { clang_getCursorUSR(*raw.add(index as usize)) }))
        .collect();
    unsafe { clang_disposeOverriddenCursors(raw) };
    methods.sort();
    methods
}

fn callable_reference(cursor: CXCursor, ty: CXType) -> Result<Option<CXCursor>, Error> {
    if !matches!(ty.kind, CXType_FunctionProto | CXType_FunctionNoProto) {
        return Ok(None);
    }
    let mut result = None;
    for reference in children(cursor)
        .into_iter()
        .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_TypeRef)
    {
        let declaration = unsafe { clang_getCursorReferenced(reference) };
        if !matches!(
            unsafe { clang_getCursorKind(declaration) },
            CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
        ) || unsafe {
            clang_equalTypes(
                ty,
                clang_getCanonicalType(clang_getTypedefDeclUnderlyingType(declaration)),
            )
        } == 0
        {
            continue;
        }
        if let Some(previous) = result
            && unsafe { clang_equalCursors(previous, declaration) } == 0
        {
            return Err(Error("ambiguous written callable typedef reference".into()));
        }
        result = Some(declaration);
    }
    Ok(result)
}

fn parameter_comments(cursor: CXCursor, parameters: &[CXCursor]) -> Vec<Vec<Annotation>> {
    let mut result: Vec<Vec<Annotation>> = parameters.iter().map(|_| vec![]).collect();
    let Some(last) = parameters.last() else {
        return result;
    };
    let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
    let range = expansion_range(tu, unsafe {
        clang_getRange(
            clang_getRangeStart(clang_getCursorExtent(cursor)),
            clang_getRangeEnd(clang_getCursorExtent(*last)),
        )
    });
    let mut raw = std::ptr::null_mut();
    let mut count = 0;
    unsafe { clang_tokenize(tu, range, &mut raw, &mut count) };
    let tokens: Vec<_> = (0..count)
        .map(|index| {
            let token = unsafe { *raw.add(index as usize) };
            (
                unsafe { clang_getTokenKind(token) },
                string(unsafe { clang_getTokenSpelling(tu, token) }),
                expansion_location(unsafe { clang_getTokenLocation(tu, token) }),
            )
        })
        .collect();
    unsafe { clang_disposeTokens(tu, raw, count) };
    let context: Arc<[String]> = parameters
        .iter()
        .map(|parameter| string(unsafe { clang_getCursorSpelling(*parameter) }))
        .collect();
    for (index, parameter) in parameters.iter().enumerate() {
        let start =
            expansion_location(unsafe { clang_getRangeStart(clang_getCursorExtent(*parameter)) });
        let end = tokens.partition_point(|(_, _, location)| location.offset < start.offset);
        let mut begin = end;
        while begin > 0 && tokens[begin - 1].0 == CXToken_Comment {
            begin -= 1;
        }
        if begin == end || begin == 0 || tokens[begin - 1].1 != if index == 0 { "(" } else { "," } {
            continue;
        }
        for (_, comment, origin) in &tokens[begin..end] {
            if origin.file == start.file && Annotation::midl_attributes(comment).is_some() {
                result[index].push(Annotation {
                    source: AnnotationSource::Midl,
                    text: comment
                        .trim_start_matches("/*")
                        .trim_end_matches("*/")
                        .trim()
                        .into(),
                    context: context.clone(),
                    location: origin.clone(),
                });
            }
        }
    }
    result
}

fn method_property(cursor: CXCursor, annotations: &[Annotation]) -> Result<Option<String>, Error> {
    let mut properties = BTreeSet::new();
    for annotation in annotations
        .iter()
        .filter(|annotation| annotation.source == AnnotationSource::Midl)
    {
        properties.extend(
            Annotation::midl_attributes(&annotation.text)
                .unwrap()
                .into_iter()
                .filter(|attribute| matches!(*attribute, "propget" | "propput" | "propputref"))
                .map(str::to_string),
        );
    }
    if properties.len() > 1 {
        return Err(Error(format!(
            "conflicting MIDL property markers: {}",
            qualified_name(cursor)
        )));
    }
    Ok(properties.pop_first())
}

fn declaration_comments(cursor: CXCursor) -> Vec<Annotation> {
    let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
    let range = expansion_range(tu, unsafe {
        clang_getRange(
            clang_getRangeStart(clang_getCursorExtent(cursor)),
            clang_getCursorLocation(cursor),
        )
    });
    let mut raw = std::ptr::null_mut();
    let mut count = 0;
    unsafe { clang_tokenize(tu, range, &mut raw, &mut count) };
    let context: Arc<[String]> = children(cursor)
        .into_iter()
        .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
        .map(|child| string(unsafe { clang_getCursorSpelling(child) }))
        .collect();
    let mut result = vec![];
    for index in 0..count {
        let token = unsafe { *raw.add(index as usize) };
        if unsafe { clang_getTokenKind(token) } == CXToken_Comment {
            let text = string(unsafe { clang_getTokenSpelling(tu, token) });
            if Annotation::midl_attributes(&text).is_some() {
                result.push(Annotation {
                    source: AnnotationSource::Midl,
                    text: text
                        .trim_start_matches("/*")
                        .trim_end_matches("*/")
                        .trim()
                        .into(),
                    context: context.clone(),
                    location: expansion_location(unsafe { clang_getTokenLocation(tu, token) }),
                });
            }
        }
    }
    unsafe { clang_disposeTokens(tu, raw, count) };
    result
}

fn declarations(root: CXCursor) -> Vec<CXCursor> {
    let mut result = vec![];
    let mut pending = children(root);
    while let Some(cursor) = pending.pop() {
        let kind = unsafe { clang_getCursorKind(cursor) };
        if matches!(
            kind,
            CXCursor_Namespace | CXCursor_LinkageSpec | CXCursor_UnexposedDecl
        ) {
            pending.extend(children(cursor));
        } else if unsafe { clang_isDeclaration(kind) } != 0 {
            result.push(cursor);
            if matches!(
                kind,
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_ClassDecl
            ) {
                pending.extend(children(cursor).into_iter().filter(|child| {
                    matches!(
                        unsafe { clang_getCursorKind(*child) },
                        CXCursor_StructDecl
                            | CXCursor_UnionDecl
                            | CXCursor_ClassDecl
                            | CXCursor_EnumDecl
                            | CXCursor_TypedefDecl
                            | CXCursor_TypeAliasDecl
                    )
                }));
            }
        }
    }
    result
}

fn qualified_name(cursor: CXCursor) -> String {
    let mut names = vec![string(unsafe { clang_getCursorSpelling(cursor) })];
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    while unsafe { clang_Cursor_isNull(parent) } == 0
        && unsafe { clang_getCursorKind(parent) } != CXCursor_TranslationUnit
    {
        let name = string(unsafe { clang_getCursorSpelling(parent) });
        if !name.is_empty() {
            names.push(name);
        } else if unsafe { clang_getCursorKind(parent) } == CXCursor_Namespace {
            names.push("(anonymous namespace)".into());
        }
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    names.reverse();
    names.join("::")
}

fn candidate(unit: &str, cursor: CXCursor, identity: &str) -> String {
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    let mut local = matches!(
        unsafe { clang_getCursorLinkage(cursor) },
        CXLinkage_Internal | CXLinkage_UniqueExternal
    );
    while unsafe { clang_Cursor_isNull(parent) } == 0 {
        let kind = unsafe { clang_getCursorKind(parent) };
        local |= kind == CXCursor_FunctionDecl
            || (kind == CXCursor_Namespace
                && string(unsafe { clang_getCursorSpelling(parent) }).is_empty());
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    if local {
        format!("local:{unit}:{identity}")
    } else if matches!(
        unsafe { clang_getCursorKind(cursor) },
        CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
    ) {
        // Typedef USRs include their source file even in a named namespace.
        format!("alias:{}", qualified_name(cursor))
    } else {
        identity.to_string()
    }
}

fn location(cursor: CXCursor) -> Location {
    let mut file = std::ptr::null_mut();
    let mut line = 0;
    let mut column = 0;
    let mut offset = 0;
    unsafe {
        clang_getSpellingLocation(
            clang_getCursorLocation(cursor),
            &mut file,
            &mut line,
            &mut column,
            &mut offset,
        );
    }
    Location {
        file: if file.is_null() {
            String::new()
        } else {
            string(unsafe { clang_getFileName(file) })
        },
        line,
        column,
        offset,
    }
}

fn expansion_location(location: CXSourceLocation) -> Location {
    let mut file = std::ptr::null_mut();
    let mut line = 0;
    let mut column = 0;
    let mut offset = 0;
    unsafe { clang_getExpansionLocation(location, &mut file, &mut line, &mut column, &mut offset) };
    Location {
        file: if file.is_null() {
            String::new()
        } else {
            string(unsafe { clang_getFileName(file) })
        },
        line,
        column,
        offset,
    }
}

/// Reads a `__declspec(uuid(...))` attribute directly on `cursor`, lowercase and without braces.
///
/// Libclang does not expose UuidAttr's value. Its terse declaration printer preserves the
/// compiler-expanded value without printing the body or guessing macro argument substitutions.
fn uuid(cursor: CXCursor) -> Result<Option<String>, Error> {
    if !children(cursor)
        .iter()
        .any(|attr| unsafe { clang_getCursorKind(*attr) == CXCursor_UnexposedAttr })
    {
        return Ok(None);
    }
    let policy = unsafe { clang_getCursorPrintingPolicy(cursor) };
    unsafe { clang_PrintingPolicy_setProperty(policy, CXPrintingPolicy_TerseOutput, 1) };
    let printed = string(unsafe { clang_getCursorPrettyPrinted(cursor, policy) });
    unsafe { clang_PrintingPolicy_dispose(policy) };
    let mut text = printed.as_str();
    while !text.is_empty() {
        if let Some(value) = text.strip_prefix("__declspec(uuid(\"") {
            let value = value
                .split_once("\"))")
                .map(|(value, _)| value.trim_start_matches('{').trim_end_matches('}'))
                .filter(|value| is_uuid(value))
                .ok_or_else(|| {
                    Error(format!(
                        "UUID value unavailable for `{}`",
                        qualified_name(cursor)
                    ))
                })?;
            return Ok(Some(value.to_ascii_lowercase()));
        }
        let character = text.chars().next().unwrap();
        text = &text[character.len_utf8()..];
        if matches!(character, '"' | '\'') {
            while let Some(next) = text.chars().next() {
                text = &text[next.len_utf8()..];
                if next == '\\' {
                    if let Some(escaped) = text.chars().next() {
                        text = &text[escaped.len_utf8()..];
                    }
                } else if next == character {
                    break;
                }
            }
        }
    }
    Ok(None)
}

// `clang_getCursorExtent` for a macro-backed `__declspec(uuid(...))` attribute reports a
// spelling-location range, which can resolve to the macro's definition site rather than its
// expansion site. Converting both endpoints to their expansion location recovers the range of
// the actual usage, e.g. `MIDL_INTERFACE("...")`, so tokenizing it finds the right literal.
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

fn layout(ty: CXType) -> Option<Layout> {
    let size = unsafe { clang_Type_getSizeOf(ty) };
    let align = unsafe { clang_Type_getAlignOf(ty) };
    (size >= 0 && align > 0).then_some(Layout { size, align })
}

fn evaluate(cursor: CXCursor) -> Value {
    let initializer = unsafe { clang_Cursor_getVarDeclInitializer(cursor) };
    if unsafe { clang_Cursor_isNull(initializer) } != 0 {
        return Value::None;
    }
    evaluate_initializer(initializer)
}

fn evaluate_initializer(cursor: CXCursor) -> Value {
    unsafe {
        let ty = clang_getCanonicalType(clang_getCursorType(cursor));
        if matches!(
            ty.kind,
            CXType_Record | CXType_ConstantArray | CXType_Pointer
        ) && matches!(
            clang_getCursorKind(cursor),
            CXCursor_ParenExpr | CXCursor_UnexposedExpr | CXCursor_CXXFunctionalCastExpr
        ) {
            let expressions: Vec<_> = children(cursor)
                .into_iter()
                .filter(|child| clang_isExpression(clang_getCursorKind(*child)) != 0)
                .collect();
            if let [child] = expressions.as_slice()
                && (clang_equalTypes(ty, clang_getCanonicalType(clang_getCursorType(*child))) != 0
                    || (ty.kind == CXType_Record
                        && clang_equalCursors(
                            clang_getTypeDeclaration(ty),
                            clang_getTypeDeclaration(clang_getCanonicalType(clang_getCursorType(
                                *child,
                            ))),
                        ) != 0)
                    || (ty.kind == CXType_Pointer
                        && clang_getCanonicalType(clang_getCursorType(*child)).kind
                            == CXType_ConstantArray
                        && clang_equalTypes(
                            clang_getUnqualifiedType(clang_getPointeeType(ty)),
                            clang_getUnqualifiedType(clang_getArrayElementType(
                                clang_getCanonicalType(clang_getCursorType(*child)),
                            )),
                        ) != 0))
            {
                return evaluate_initializer(*child);
            }
        }
        if clang_getCursorKind(cursor) == CXCursor_StringLiteral {
            return string_literal(cursor).unwrap_or_else(|error| Value::Unavailable(error.0));
        }
        if matches!(ty.kind, CXType_Record | CXType_ConstantArray) {
            if clang_getCursorKind(cursor) != CXCursor_InitListExpr {
                return Value::Unavailable(
                    "aggregate constants require explicit initializer lists".into(),
                );
            }
            let count = if ty.kind == CXType_ConstantArray {
                clang_getArraySize(ty) as usize
            } else {
                let declaration = clang_getTypeDeclaration(ty);
                if clang_getCursorKind(declaration) != CXCursor_StructDecl {
                    return Value::Unavailable(
                        "aggregate constants require struct or array types".into(),
                    );
                }
                record_fields(declaration).len()
            };
            let elements: Vec<_> = children(cursor)
                .into_iter()
                .filter(|child| clang_isExpression(clang_getCursorKind(*child)) != 0)
                .collect();
            if elements.len() != count {
                return Value::Unavailable(
                    "aggregate constants require every element to be explicit".into(),
                );
            }
            let mut values = vec![];
            for element in elements {
                let value = evaluate_initializer(element);
                if matches!(value, Value::Unavailable(_)) {
                    return value;
                }
                values.push(value);
            }
            return Value::Aggregate(values);
        }
        if clang_Type_getSizeOf(ty) > 8 {
            return Value::Unavailable(
                "constant values wider than 64 bits are not implemented".into(),
            );
        }
        let result = clang_Cursor_Evaluate(cursor);
        if result.is_null() {
            return Value::Unavailable("initializer is not a supported constant expression".into());
        }
        let value = match clang_EvalResult_getKind(result) {
            CXEval_Int => Value::Integer(clang_EvalResult_getAsUnsigned(result)),
            CXEval_Float => Value::Float(clang_EvalResult_getAsDouble(result).to_bits()),
            _ => Value::Unavailable("initializer value kind is not implemented".into()),
        };
        clang_EvalResult_dispose(result);
        value
    }
}

fn string_literal(cursor: CXCursor) -> Result<Value, Error> {
    // The evaluated-string API uses a NUL-terminated copy. The compiler's escaped spelling
    // retains every code unit, including embedded NULs and UTF-16 surrogates.
    let spelling = string(unsafe { clang_getCursorSpelling(cursor) });
    let (prefix, text) = spelling
        .split_once('"')
        .and_then(|(prefix, text)| text.strip_suffix('"').map(|text| (prefix, text)))
        .ok_or_else(|| Error("compiler string literal spelling is unavailable".into()))?;
    let ty = unsafe { clang_getCanonicalType(clang_getCursorType(cursor)) };
    let width = unsafe { clang_Type_getSizeOf(clang_getArrayElementType(ty)) };
    let encoding = match (prefix, width) {
        ("", 1) => StringEncoding::Narrow,
        ("u8", 1) => StringEncoding::Utf8,
        ("L" | "u", 2) => StringEncoding::Utf16,
        ("L" | "U", 4) => StringEncoding::Utf32,
        _ => return Err(Error("unsupported compiler string literal encoding".into())),
    };
    let mut units = vec![];
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '"' {
            if chars.next() != Some('"') {
                return Err(Error("invalid compiler string literal boundary".into()));
            }
            continue;
        }
        let mut unicode = false;
        let value = if character != '\\' {
            u32::from(character)
        } else {
            match chars.next() {
                Some('\\') => u32::from('\\'),
                Some('"') => u32::from('"'),
                Some('\'') => u32::from('\''),
                Some('?') => u32::from('?'),
                Some('a') => 7,
                Some('b') => 8,
                Some('t') => 9,
                Some('n') => 10,
                Some('v') => 11,
                Some('f') => 12,
                Some('r') => 13,
                Some(prefix @ ('x' | 'u' | 'U' | '0'..='7')) => {
                    let (radix, maximum, mut value, mut count) = match prefix {
                        'x' => (16, usize::MAX, 0u32, 0),
                        'u' => (16, 4, 0, 0),
                        'U' => (16, 8, 0, 0),
                        digit => (8, 3, digit.to_digit(8).unwrap(), 1),
                    };
                    while count < maximum {
                        let Some(digit) = chars.peek().and_then(|next| next.to_digit(radix)) else {
                            break;
                        };
                        chars.next();
                        value = value
                            .checked_mul(radix)
                            .and_then(|value| value.checked_add(digit))
                            .ok_or_else(|| {
                                Error("compiler string escape exceeds a code unit".into())
                            })?;
                        count += 1;
                    }
                    unicode = matches!(prefix, 'u' | 'U');
                    if count == 0 || (unicode && count != maximum) {
                        return Err(Error("invalid compiler string escape".into()));
                    }
                    value
                }
                _ => return Err(Error("unsupported compiler string escape".into())),
            }
        };
        if unicode && width == 2 {
            let character = char::from_u32(value)
                .ok_or_else(|| Error("invalid compiler Unicode escape".into()))?;
            units.extend(
                character
                    .encode_utf16(&mut [0; 2])
                    .iter()
                    .map(|unit| u32::from(*unit)),
            );
        } else {
            if (width == 1 && value > u32::from(u8::MAX))
                || (width == 2 && value > u32::from(u16::MAX))
            {
                return Err(Error(
                    "compiler string escape exceeds its native code unit".into(),
                ));
            }
            units.push(value);
        }
    }
    if unsafe { clang_getArraySize(ty) } != units.len() as i64 + 1 {
        return Err(Error(
            "compiler string spelling disagrees with its native extent".into(),
        ));
    }
    Ok(Value::String { encoding, units })
}

fn c_string(value: &str) -> Result<CString, Error> {
    CString::new(value).map_err(|_| Error("input or argument contains a null byte".into()))
}

fn string(value: CXString) -> String {
    unsafe {
        let ptr = clang_getCString(value);
        let result = if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        clang_disposeString(value);
        result
    }
}

#[derive(Default)]
struct CursorCollector {
    cursors: Vec<CXCursor>,
    panic: Option<Box<dyn std::any::Any + Send>>,
}

impl CursorCollector {
    fn push(&mut self, cursor: CXCursor) -> bool {
        match catch_unwind(AssertUnwindSafe(|| self.cursors.push(cursor))) {
            Ok(()) => true,
            Err(panic) => {
                self.panic = Some(panic);
                false
            }
        }
    }

    fn finish(self) -> Vec<CXCursor> {
        if let Some(panic) = self.panic {
            resume_unwind(panic);
        }
        self.cursors
    }
}

fn children(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        if unsafe { &mut *data.cast::<CursorCollector>() }.push(cursor) {
            CXChildVisit_Continue
        } else {
            CXChildVisit_Break
        }
    }
    let mut state = CursorCollector::default();
    unsafe { clang_visitChildren(cursor, visit, (&raw mut state).cast()) };
    state.finish()
}

fn record_fields(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(cursor: CXCursor, data: CXClientData) -> CXVisitorResult {
        if unsafe { &mut *data.cast::<CursorCollector>() }.push(cursor) {
            CXVisit_Continue
        } else {
            CXVisit_Break
        }
    }
    let mut state = CursorCollector::default();
    unsafe { clang_Type_visitFields(clang_getCursorType(cursor), visit, (&raw mut state).cast()) };
    state.finish()
}
