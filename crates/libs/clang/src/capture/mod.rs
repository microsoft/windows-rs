#![allow(non_upper_case_globals)]

use super::*;
use native::*;
use windows_clang_sys::*;

mod annotations;
mod comments;
mod values;

pub(super) struct MacroProbe {
    pub name: String,
    pub start: u32,
    pub declaration: u32,
    pub expansion: u32,
    pub function: bool,
}

pub(super) fn macro_definition(
    unit: CXTranslationUnit,
    file: CXFile,
    probe: &MacroProbe,
) -> Result<CXCursor, Error> {
    let expansion = unsafe {
        clang_getCursor(
            unit,
            clang_getLocationForOffset(unit, file, probe.expansion),
        )
    };
    if expansion.kind != CXCursor_MacroExpansion
        || text(unsafe { clang_getCursorSpelling(expansion) }) != probe.name
    {
        return Err(Error(format!(
            "macro `{}` has no expansion at its evaluation probe",
            probe.name
        )));
    }
    let definition = unsafe { clang_getCursorReferenced(expansion) };
    if definition.kind != CXCursor_MacroDefinition
        || unsafe { clang_Cursor_isMacroFunctionLike(definition) } != 0
    {
        return Err(Error(format!(
            "macro `{}` is not an object-like definition",
            probe.name
        )));
    }
    Ok(definition)
}

pub(super) fn read(
    unit: CXTranslationUnit,
    input: &Input,
    macros: &[MacroProbe],
    snapshot: &mut Snapshot,
    partial: bool,
) -> Result<partial::Issues, Error> {
    let roots = &input.roots;
    let root_paths = input
        .root_paths
        .iter()
        .flatten()
        .map(|path| {
            let canonical = std::fs::canonicalize(path)
                .map_err(|error| Error(format!("root path {}: {error}", path.display())))?;
            if !canonical.is_dir() && !canonical.is_file() {
                return Err(Error(format!(
                    "root path {} is not a file or directory",
                    path.display()
                )));
            }
            Ok(canonical)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let mut root_files = HashMap::new();
    let mut issues = partial::Issues::default();
    let declarations = declarations(unsafe { clang_getTranslationUnitCursor(unit) });
    let mut sites = HashMap::new();
    let mut main_file = std::ptr::null_mut();
    if !macros.is_empty() {
        let name = std::ffi::CString::new(snapshot.input.as_str()).unwrap();
        main_file = unsafe { clang_getFile(unit, name.as_ptr()) };
        for &cursor in &declarations {
            let kind = unsafe { clang_getCursorKind(cursor) };
            if kind == CXCursor_VarDecl {
                let (file, offset) = expansion_site(cursor);
                if !file.is_null() && file == main_file {
                    sites.insert((kind, offset), cursor);
                }
            }
        }
    }
    let mut function_probes = HashSet::new();
    let probes = macros
        .iter()
        .map(|probe| {
            let name = &probe.name;
            let definition = macro_definition(unit, main_file, probe)?;
            let declaration = sites
                .get(&(CXCursor_VarDecl, probe.declaration))
                .ok_or_else(|| Error(format!("macro `{name}` has no evaluation declaration")))?;
            let key = Cursor(unsafe { clang_getCanonicalCursor(*declaration) });
            if probe.function {
                function_probes.insert(key);
            }
            Ok((key, (name.clone(), definition)))
        })
        .collect::<Result<HashMap<_, _>, Error>>()?;
    let mut capture = Capture {
        declarations: HashMap::new(),
        pending: Vec::new(),
        records: HashMap::new(),
        aliases: HashMap::new(),
        enums: HashMap::new(),
        bitwise_enums: HashSet::new(),
        values: HashMap::new(),
        probes,
        function_probes,
        types: HashMap::new(),
        files: HashMap::new(),
        partial,
        external_interfaces: &input.external_interfaces,
        snapshot,
    };
    let mut names: HashMap<String, Vec<Cursor>> = HashMap::new();
    let mut all = Vec::new();
    let mut all_seen = HashSet::new();
    for &cursor in &declarations {
        let name = declaration_name(cursor);
        let name = capture
            .probes
            .get(&Cursor(unsafe { clang_getCanonicalCursor(cursor) }))
            .map_or(name, |(name, _)| name.clone());
        let kind = unsafe { clang_getCursorKind(cursor) };
        if unsafe { clang_isPreprocessing(kind) } != 0 {
            continue;
        }
        if let Some(enumeration) = bitwise_enum(cursor) {
            capture.bitwise_enums.insert(enumeration);
        }
        if matches!(
            unsafe { clang_getCursorKind(clang_getCursorSemanticParent(cursor)) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            if matches!(
                kind,
                CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl | CXCursor_EnumDecl
            ) {
                let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
                capture.declarations.entry(key).or_default().push(cursor);
            }
            continue;
        }
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        let synthetic = !capture.probes.contains_key(&key)
            && macros.first().is_some_and(|probe| {
                let (file, offset) = expansion_site(cursor);
                file == main_file && offset >= probe.start
            });
        let mut included = true;
        if input.root_paths.is_some() && !capture.probes.contains_key(&key) && !synthetic {
            let file = capture.location(cursor).file;
            if let Some(allowed) = root_files.get(&file) {
                included = *allowed;
            } else {
                let name = &capture.snapshot.files[file.0];
                if !name.is_empty() && name != &capture.snapshot.input {
                    let path = std::fs::canonicalize(name)
                        .map_err(|error| Error(format!("root header {name}: {error}")))?;
                    included = root_paths.iter().any(|root| path.starts_with(root));
                }
                root_files.insert(file, included);
            }
        }
        let selected_function = input.function_symbols.as_ref().is_none_or(|targets| {
            let function = if kind == CXCursor_FunctionDecl {
                Some(cursor)
            } else if capture.function_probes.contains(&key) {
                // Invalid probes stay selected so capture reports their errors.
                values::function_alias_target(cursor).ok()
            } else {
                None
            };
            function.is_none_or(|function| {
                let symbol = text(unsafe { clang_Cursor_getMangling(function) });
                targets
                    .get(&capture.snapshot.target)
                    .is_some_and(|symbols| symbols.contains(&symbol))
            })
        });
        let automatic_root = !synthetic
            && included
            && selected_function
            && (input.namespace_roots == NamespaceRoots::All
                || !in_namespace(cursor)
                || binary_identity(cursor, kind)?);
        if !matches!(
            kind,
            CXCursor_StructDecl
                | CXCursor_ClassDecl
                | CXCursor_UnionDecl
                | CXCursor_EnumDecl
                | CXCursor_TypedefDecl
                | CXCursor_TypeAliasDecl
                | CXCursor_FunctionDecl
                | CXCursor_VarDecl
        ) || name.is_empty()
        {
            if roots.is_empty() && automatic_root {
                let error = unsupported(cursor, "expected a supported global declaration");
                if !partial {
                    return Err(error);
                }
                let reason = format!("{:?}: {error}", capture.snapshot.target);
                let key = if name.is_empty() {
                    None
                } else {
                    match kind {
                        CXCursor_ClassDecl
                        | CXCursor_ClassTemplate
                        | CXCursor_ClassTemplatePartialSpecialization => {
                            Some(partial::Key::Tag(name))
                        }
                        CXCursor_TypeAliasTemplateDecl => Some(partial::Key::Alias(name)),
                        _ => None,
                    }
                };
                if let Some(key) = key {
                    issues.reject(key, reason);
                } else {
                    issues.messages.insert(reason);
                }
            }
            continue;
        }
        let observations = capture.declarations.entry(key).or_insert_with(|| {
            names.entry(name).or_default().push(key);
            Vec::new()
        });
        observations.push(cursor);
        if automatic_root && all_seen.insert(key) {
            all.push(key);
        }
    }
    let mut selected = HashSet::new();
    let selected = if roots.is_empty() {
        all
    } else {
        let mut result = Vec::new();
        for name in roots.iter().chain(macros.iter().map(|probe| &probe.name)) {
            let Some(candidates) = names.get(name) else {
                return Err(Error(format!("requested declaration `{name}` not found")));
            };
            for &candidate in candidates {
                if selected.insert(candidate) {
                    result.push(candidate);
                }
            }
        }
        result
    };
    let mut c_functions = HashSet::new();
    for cursor in &selected {
        if unsafe { clang_getCursorKind(cursor.0) } == CXCursor_FunctionDecl
            && unsafe { clang_getCursorLinkage(cursor.0) } == CXLinkage_External
            && values::language_linkage(cursor.0)? == LanguageLinkage::C
        {
            c_functions.insert(declaration_name(cursor.0));
        }
    }
    for cursor in selected {
        if unsafe { clang_getCursorKind(cursor.0) } == CXCursor_FunctionDecl
            && unsafe { clang_getCursorLinkage(cursor.0) } == CXLinkage_External
            && c_functions.contains(&declaration_name(cursor.0))
            && values::language_linkage(cursor.0)? == LanguageLinkage::Cxx
        {
            let observations = &capture.declarations[&cursor];
            if observations
                .iter()
                .all(|cursor| unsafe { clang_Cursor_isFunctionInlined(*cursor) } != 0)
                && observations
                    .iter()
                    .any(|cursor| unsafe { clang_isCursorDefinition(*cursor) } != 0)
            {
                for observation in observations.clone() {
                    let usr = text(unsafe { clang_getCursorUSR(observation) });
                    if usr.is_empty() {
                        return Err(unsupported(
                            observation,
                            "inline function identity unavailable",
                        ));
                    }
                    let function = InlineFunction {
                        name: declaration_name(observation),
                        usr,
                        location: capture.location(observation),
                    };
                    capture.snapshot.inline_functions.push(function);
                }
                continue;
            }
        }
        let declaration = if matches!(
            unsafe { clang_getCursorKind(cursor.0) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            DeclarationId::Record(capture.record(cursor.0)?)
        } else if unsafe { clang_getCursorKind(cursor.0) } == CXCursor_EnumDecl {
            DeclarationId::Enum(capture.enumeration(cursor.0)?)
        } else if matches!(
            unsafe { clang_getCursorKind(cursor.0) },
            CXCursor_FunctionDecl | CXCursor_VarDecl
        ) {
            DeclarationId::Value(capture.value(cursor.0)?)
        } else {
            DeclarationId::Alias(capture.alias(cursor.0)?)
        };
        capture.snapshot.roots.push(declaration);
    }
    while let Some(key) = capture.pending.pop() {
        let observations = capture.declarations.remove(&key).unwrap_or_else(|| {
            panic!(
                "{}",
                unsupported(
                    key.0,
                    "queued declaration has no observation catalogue entry"
                )
            )
        });
        for cursor in observations {
            let (id, result) = if let Some(&record) = capture.records.get(&key) {
                (
                    DeclarationId::Record(record),
                    capture.observation(record, cursor),
                )
            } else if let Some(&enumeration) = capture.enums.get(&key) {
                (
                    DeclarationId::Enum(enumeration),
                    capture.enum_observation(enumeration, cursor),
                )
            } else if let Some(&value) = capture.values.get(&key) {
                (
                    DeclarationId::Value(value),
                    capture.value_observation(value, cursor),
                )
            } else {
                let alias = capture.aliases[&key];
                (
                    DeclarationId::Alias(alias),
                    capture.alias_observation(alias, cursor),
                )
            };
            if let Err(error) = result {
                if !partial {
                    return Err(error);
                }
                issues.reject(
                    if let Some((name, _)) = capture.probes.get(&key) {
                        partial::Key::Macro(name.clone())
                    } else {
                        partial::Key::of(capture.snapshot, id)
                    },
                    format!("{:?}: {error}", capture.snapshot.target),
                );
            }
        }
    }
    Ok(issues)
}

fn expansion_site(cursor: CXCursor) -> (CXFile, u32) {
    let mut file = std::ptr::null_mut();
    let mut offset = 0;
    unsafe {
        clang_getExpansionLocation(
            clang_getCursorLocation(cursor),
            &raw mut file,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &raw mut offset,
        );
    }
    (file, offset)
}

fn declarations(cursor: CXCursor) -> Vec<CXCursor> {
    let mut result = Vec::new();
    for child in children(cursor) {
        let kind = unsafe { clang_getCursorKind(child) };
        let namespace = kind == CXCursor_Namespace
            && !text(unsafe { clang_getCursorSpelling(child) }).is_empty()
            && !children(child)
                .iter()
                .any(|child| unsafe { clang_isAttribute(clang_getCursorKind(*child)) } != 0);
        if kind == CXCursor_LinkageSpec || namespace {
            result.extend(declarations(child));
        } else {
            result.push(child);
            if matches!(
                unsafe { clang_getCursorKind(child) },
                CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
            ) {
                // Elaborated tags can be lexically inside a record but semantically global.
                result.extend(declarations(child).into_iter().filter(|nested| {
                    matches!(
                        unsafe { clang_getCursorKind(*nested) },
                        CXCursor_StructDecl
                            | CXCursor_ClassDecl
                            | CXCursor_UnionDecl
                            | CXCursor_EnumDecl
                    )
                }));
            }
        }
    }
    result
}

fn declaration_name(cursor: CXCursor) -> String {
    let name = text(unsafe { clang_getCursorSpelling(cursor) });
    let mut scope = Vec::new();
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    while unsafe { clang_Cursor_isNull(parent) } == 0 {
        match unsafe { clang_getCursorKind(parent) } {
            CXCursor_Namespace => scope.push(text(unsafe { clang_getCursorSpelling(parent) })),
            CXCursor_LinkageSpec => {}
            _ => break,
        }
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    scope.reverse();
    scope.push(name);
    scope.join("::")
}

fn in_namespace(cursor: CXCursor) -> bool {
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    while unsafe { clang_getCursorKind(parent) } == CXCursor_LinkageSpec {
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    let kind = unsafe { clang_getCursorKind(parent) };
    kind == CXCursor_Namespace
}

// C linkage ignores C++ scope, and a UUID identifies a record independently of its spelling.
fn binary_identity(cursor: CXCursor, kind: CXCursorKind) -> Result<bool, Error> {
    Ok(match kind {
        CXCursor_FunctionDecl => {
            let mut language = WindowsClangNoLanguageLinkage;
            let status = unsafe { windows_clang_getLanguageLinkage(cursor, &raw mut language) };
            if status != WindowsClangSuccess {
                return Err(unsupported(
                    cursor,
                    "declaration language linkage unavailable",
                ));
            }
            language == WindowsClangCLanguageLinkage
        }
        CXCursor_StructDecl | CXCursor_ClassDecl => children(cursor).into_iter().any(|child| {
            if unsafe { clang_isAttribute(clang_getCursorKind(child)) } == 0 {
                return false;
            }
            let name = unsafe { windows_clang_getAttributeKindName(child) };
            !name.is_null() && unsafe { std::ffi::CStr::from_ptr(name) }.to_bytes() == b"Uuid"
        }),
        _ => false,
    })
}

/// The enum of a free `E operator|(E, E)` declaration, as `DEFINE_ENUM_FLAG_OPERATORS` declares.
fn bitwise_enum(cursor: CXCursor) -> Option<Cursor> {
    unsafe {
        if clang_getCursorKind(cursor) != CXCursor_FunctionDecl
            || text(clang_getCursorSpelling(cursor)) != "operator|"
            || clang_Cursor_getNumArguments(cursor) != 2
        {
            return None;
        }
        let result = clang_getCanonicalType(clang_getCursorResultType(cursor));
        if (0..2).any(|index| {
            clang_equalTypes(
                clang_getCanonicalType(clang_getCursorType(clang_Cursor_getArgument(
                    cursor, index,
                ))),
                result,
            ) == 0
        }) {
            return None;
        }
        let declaration = clang_getTypeDeclaration(result);
        (clang_getCursorKind(declaration) == CXCursor_EnumDecl)
            .then(|| Cursor(clang_getCanonicalCursor(declaration)))
    }
}

struct Capture<'a> {
    declarations: HashMap<Cursor, Vec<CXCursor>>,
    pending: Vec<Cursor>,
    values: HashMap<Cursor, ValueId>,
    probes: HashMap<Cursor, (String, CXCursor)>,
    function_probes: HashSet<Cursor>,
    records: HashMap<Cursor, RecordId>,
    aliases: HashMap<Cursor, AliasId>,
    enums: HashMap<Cursor, EnumId>,
    /// Enums for which the header declares `E operator|(E, E)`.
    bitwise_enums: HashSet<Cursor>,
    types: HashMap<Type, TypeId>,
    files: HashMap<String, FileId>,
    partial: bool,
    external_interfaces: &'a BTreeSet<String>,
    snapshot: &'a mut Snapshot,
}

impl Capture<'_> {
    fn enumeration(&mut self, cursor: CXCursor) -> Result<EnumId, Error> {
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.enums.get(&key) {
            return Ok(id);
        }
        if !self.declarations.contains_key(&key) {
            return Err(unsupported(cursor, "dependency is not a global named enum"));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(unsupported(cursor, "enum identity unavailable"));
        }
        let parent = unsafe { clang_getCursorSemanticParent(cursor) };
        let owner = if matches!(
            unsafe { clang_getCursorKind(parent) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            Some(self.record(parent)?)
        } else {
            None
        };
        let name = if let Some(owner) = owner {
            let leaf = if unsafe { clang_Cursor_isAnonymous(cursor) } != 0 {
                let member = children(cursor)
                    .into_iter()
                    .find(|child| child.kind == CXCursor_EnumConstantDecl)
                    .ok_or_else(|| {
                        unsupported(cursor, "anonymous member enum has no member identity")
                    })?;
                format!(
                    "<anonymous {}>",
                    text(unsafe { clang_getCursorSpelling(member) })
                )
            } else {
                text(unsafe { clang_getCursorSpelling(cursor) })
            };
            format!("{}::{leaf}", self.snapshot.record_label(owner))
        } else {
            declaration_name(cursor)
        };
        let access = match unsafe { clang_getCXXAccessSpecifier(cursor) } {
            CX_CXXPrivate => FieldAccess::Private,
            CX_CXXProtected => FieldAccess::Protected,
            _ => FieldAccess::Public,
        };
        let id = EnumId(self.snapshot.enums.len());
        self.snapshot.enums.push(Enum {
            name,
            usr,
            owner,
            access,
            observations: Vec::new(),
        });
        self.enums.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    fn enum_observation(&mut self, id: EnumId, cursor: CXCursor) -> Result<(), Error> {
        let underlying = unsafe { clang_getEnumDeclIntegerType(cursor) };
        let canonical = self.ty(unsafe { clang_getCanonicalType(underlying) }, cursor)?;
        let signed = match &self.snapshot.types[canonical.0].kind {
            TypeKind::Scalar(scalar) => scalar.signed_integer(),
            _ => None,
        }
        .ok_or_else(|| unsupported(cursor, "unsupported enum backing type"))?;
        let mut attributes = Vec::new();
        let mut members = Vec::new();
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0 {
                attributes.push(self.attribute(child)?);
            } else if kind == CXCursor_EnumConstantDecl {
                let mut attributes = Vec::new();
                for attribute in children(child) {
                    if unsafe { clang_isAttribute(clang_getCursorKind(attribute)) } != 0 {
                        attributes.push(self.attribute(attribute)?);
                    }
                }
                members.push(EnumMember {
                    name: text(unsafe { clang_getCursorSpelling(child) }),
                    location: self.location(child),
                    value: if signed {
                        EnumValue::Signed(unsafe { clang_getEnumConstantDeclValue(child) })
                    } else {
                        EnumValue::Unsigned(unsafe {
                            clang_getEnumConstantDeclUnsignedValue(child)
                        })
                    },
                    attributes,
                });
            } else if !matches!(kind, CXCursor_TypeRef | CXCursor_NamespaceRef) {
                return Err(unsupported(child, "unsupported enum declaration child"));
            }
        }
        let observation = EnumObservation {
            location: self.location(cursor),
            underlying: self.ty(underlying, cursor)?,
            canonical,
            layout: layout(unsafe { clang_getCursorType(cursor) })
                .map_err(|reason| unsupported(cursor, reason))?,
            scoped: unsafe { clang_EnumDecl_isScoped(cursor) } != 0,
            anonymous: unsafe { clang_Cursor_isAnonymous(cursor) } != 0,
            bitwise: self
                .bitwise_enums
                .contains(&Cursor(unsafe { clang_getCanonicalCursor(cursor) })),
            attributes,
            members: (unsafe { clang_isCursorDefinition(cursor) } != 0).then_some(members),
        };
        self.snapshot.enums[id.0].observations.push(observation);
        Ok(())
    }

    fn alias(&mut self, cursor: CXCursor) -> Result<AliasId, Error> {
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.aliases.get(&key) {
            return Ok(id);
        }
        if typedef_implicit(key.0)?
            && unsafe { clang_getCursorKind(clang_getCursorSemanticParent(key.0)) }
                == CXCursor_TranslationUnit
        {
            let observations = self.declarations.entry(key).or_default();
            if !observations.iter().any(|cursor| Cursor(*cursor) == key) {
                observations.insert(0, key.0);
            }
        } else if !self.declarations.contains_key(&key) {
            return Err(unsupported(cursor, "dependency is not a global type alias"));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(unsupported(cursor, "typedef identity unavailable"));
        }
        let id = AliasId(self.snapshot.aliases.len());
        self.snapshot.aliases.push(Alias {
            name: declaration_name(cursor),
            usr,
            observations: Vec::new(),
        });
        self.aliases.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    fn alias_observation(&mut self, alias: AliasId, cursor: CXCursor) -> Result<(), Error> {
        let underlying = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
        let parameters = self.callback_parameters(cursor, underlying)?;
        let attributes = self.declaration_attributes(cursor, parameters.is_some(), None)?;
        let mut defines_tag = 0;
        let status = unsafe { windows_clang_getTypedefDefinition(cursor, &raw mut defines_tag) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("typedef definition query failed: {status}"),
            ));
        }
        let observation = AliasObservation {
            location: self.location(cursor),
            underlying: self.ty(underlying, cursor)?,
            canonical: self.ty(unsafe { clang_getCanonicalType(underlying) }, cursor)?,
            implicit: typedef_implicit(cursor)?,
            defines_tag: defines_tag != 0,
            attributes,
            parameters,
        };
        self.snapshot.aliases[alias.0]
            .observations
            .push(observation);
        Ok(())
    }

    fn record(&mut self, cursor: CXCursor) -> Result<RecordId, Error> {
        if !matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            return Err(unsupported(
                cursor,
                "expected a struct, class, or union declaration",
            ));
        }
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.records.get(&key) {
            return Ok(id);
        }
        let parent = unsafe { clang_getCursorSemanticParent(cursor) };
        let owner = if matches!(
            unsafe { clang_getCursorKind(parent) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            let record = self.record(parent)?;
            let field = if unsafe { clang_Cursor_isAnonymous(cursor) } != 0 {
                let definition = unsafe { clang_getCursorDefinition(parent) };
                Some(
                    record_fields(definition)
                        .iter()
                        .position(|field| {
                            contains_record(unsafe { clang_getCursorType(*field) }, key)
                        })
                        .ok_or_else(|| {
                            unsupported(cursor, "unnamed nested record has no owning field")
                        })?,
                )
            } else {
                None
            };
            let declarations = self.declarations.entry(key).or_default();
            for cursor in [cursor, unsafe { clang_getCursorDefinition(cursor) }] {
                if unsafe { clang_Cursor_isNull(cursor) } == 0
                    && !declarations
                        .iter()
                        .any(|existing| Cursor(*existing) == Cursor(cursor))
                {
                    declarations.push(cursor);
                }
            }
            Some(RecordOwner { record, field })
        } else {
            None
        };
        if !self.declarations.contains_key(&key) {
            return Err(unsupported(
                cursor,
                "dependency is not a global named record",
            ));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(unsupported(cursor, "record identity unavailable"));
        }
        let id = RecordId(self.snapshot.records.len());
        self.snapshot.records.push(Record {
            external: self.external_interfaces.contains(&declaration_name(cursor)),
            name: if owner.is_some_and(|owner| owner.field.is_some()) {
                String::new()
            } else {
                declaration_name(cursor)
            },
            usr,
            owner,
            kind: match unsafe { clang_getCursorKind(cursor) } {
                CXCursor_UnionDecl => RecordKind::Union,
                CXCursor_ClassDecl => RecordKind::Class,
                _ => RecordKind::Struct,
            },
            observations: Vec::new(),
        });
        self.records.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    fn observation(&mut self, record: RecordId, cursor: CXCursor) -> Result<(), Error> {
        let mut attributes = Vec::new();
        let definition = if unsafe { clang_isCursorDefinition(cursor) } != 0 {
            let native_fields = record_fields(cursor);
            let external = self.snapshot.records[record.0].external;
            if external && !native_fields.is_empty() {
                return Err(unsupported(
                    cursor,
                    "external COM definitions cannot contain data fields",
                ));
            }
            let mut fields = Vec::new();
            let mut enums = Vec::new();
            let mut bases = Vec::new();
            let mut methods = Vec::new();
            let mut non_vtable_members = Vec::new();
            for child in children(cursor) {
                let kind = unsafe { clang_getCursorKind(child) };
                if unsafe { clang_isAttribute(kind) } != 0 {
                    attributes.push(self.attribute(child)?);
                    continue;
                }
                if external && kind != CXCursor_CXXBaseSpecifier {
                    continue;
                }
                if kind == CXCursor_EnumDecl {
                    enums.push(self.enumeration(child)?);
                    continue;
                }
                if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl) {
                    self.record(child)?;
                    continue;
                }
                if kind == CXCursor_NamespaceRef
                    || (kind == CXCursor_TypeRef && owner_reference(cursor, child))
                {
                    continue;
                }
                if kind == CXCursor_CXXAccessSpecifier {
                    continue;
                }
                if kind == CXCursor_CXXBaseSpecifier {
                    if unsafe { clang_isVirtualBase(child) } != 0
                        || unsafe { clang_getCXXAccessSpecifier(child) } != CX_CXXPublic
                    {
                        return Err(unsupported(
                            child,
                            "only public non-virtual bases are supported",
                        ));
                    }
                    bases.push(Base {
                        record: self.record(unsafe {
                            clang_getTypeDeclaration(clang_getCanonicalType(clang_getCursorType(
                                child,
                            )))
                        })?,
                        offset_bits: unsafe { clang_getOffsetOfBase(cursor, child) }
                            .try_into()
                            .map_err(|_| unsupported(child, "base offset unavailable"))?,
                    });
                    continue;
                }
                let non_vtable = if kind == CXCursor_FunctionTemplate
                    && matches!(
                        unsafe { clang_getTemplateCursorKind(child) },
                        CXCursor_CXXMethod | CXCursor_Constructor
                    ) {
                    Some(NonVTableMemberKind::Template)
                } else if kind == CXCursor_CXXMethod
                    && unsafe { clang_CXXMethod_isVirtual(child) } == 0
                {
                    Some(NonVTableMemberKind::Method)
                } else if kind == CXCursor_Constructor {
                    Some(NonVTableMemberKind::Constructor)
                } else if kind == CXCursor_Destructor
                    && unsafe { clang_CXXMethod_isVirtual(child) } == 0
                {
                    Some(NonVTableMemberKind::Destructor)
                } else if kind == CXCursor_ConversionFunction
                    && unsafe { clang_CXXMethod_isVirtual(child) } == 0
                {
                    Some(NonVTableMemberKind::Conversion)
                } else if kind == CXCursor_FriendDecl {
                    Some(NonVTableMemberKind::Friend)
                } else if kind == CXCursor_UsingDeclaration {
                    let references: Vec<_> = children(child)
                        .into_iter()
                        .filter(|cursor| unsafe { clang_getCursorKind(*cursor) } == CXCursor_OverloadedDeclRef)
                        .collect();
                    let mut methods = 0;
                    for reference in references {
                        for index in 0..unsafe { clang_getNumOverloadedDecls(reference) } {
                            let target = unsafe { clang_getOverloadedDecl(reference, index) };
                            if unsafe { clang_getCursorKind(target) } != CXCursor_CXXMethod {
                                return Err(unsupported(
                                    child,
                                    "only method using declarations are supported",
                                ));
                            }
                            methods += 1;
                        }
                    }
                    if methods == 0 {
                        return Err(unsupported(
                            child,
                            "method using declaration targets unavailable",
                        ));
                    }
                    Some(NonVTableMemberKind::Using)
                } else {
                    None
                };
                if let Some(kind) = non_vtable {
                    non_vtable_members.push(NonVTableMember {
                        name: member_name(child)?,
                        usr: text(unsafe { clang_getCursorUSR(child) }),
                        location: self.location(child),
                        kind,
                    });
                    continue;
                }
                if kind == CXCursor_CXXMethod {
                    if unsafe { clang_getCXXAccessSpecifier(child) } != CX_CXXPublic {
                        return Err(unsupported(
                            child,
                            "only public virtual methods are supported",
                        ));
                    }
                    if unsafe { clang_isCursorDefinition(child) } != 0 {
                        return Err(unsupported(
                            child,
                            "virtual method definitions are unsupported",
                        ));
                    }
                    let mut overridden = std::ptr::null_mut();
                    let mut count = 0;
                    let inherited = unsafe {
                        clang_getOverriddenCursors(child, &raw mut overridden, &raw mut count);
                        let inherited = (count == 1).then(|| *overridden);
                        clang_disposeOverriddenCursors(overridden);
                        inherited
                    };
                    if count > 1 {
                        return Err(unsupported(
                            child,
                            "multiple overridden virtual methods are unsupported",
                        ));
                    }
                    let mut slot = 0;
                    if unsafe { windows_clang_getVirtualMethodSlot(child, &raw mut slot) }
                        != WindowsClangSuccess
                    {
                        return Err(unsupported(
                            child,
                            "primary virtual method slot unavailable",
                        ));
                    }
                    if let Some(inherited) = inherited {
                        let mut inherited_slot = 0;
                        if unsafe {
                            windows_clang_getVirtualMethodSlot(inherited, &raw mut inherited_slot)
                        } != WindowsClangSuccess
                            || inherited_slot != slot
                            || unsafe {
                                clang_equalTypes(
                                    clang_getCanonicalType(clang_getCursorResultType(child)),
                                    clang_getCanonicalType(clang_getCursorResultType(inherited)),
                                )
                            } == 0
                        {
                            return Err(unsupported(
                                child,
                                "adjusted or covariant virtual overrides are unsupported",
                            ));
                        }
                    }
                    methods.push(Method {
                        name: member_name(child)?,
                        slot,
                        overrides: inherited.is_some(),
                        pure: unsafe { clang_CXXMethod_isPureVirtual(child) } != 0,
                        observation: self.function_observation(child)?,
                        comments: self.method_comments(child),
                    });
                    continue;
                }
                if kind != CXCursor_FieldDecl {
                    return Err(unsupported(
                        cursor,
                        &format!(
                            "only fields are supported in `{}`: {} `{}`",
                            text(unsafe { clang_getCursorSpelling(cursor) }),
                            text(unsafe { clang_getCursorKindSpelling(kind) }),
                            text(unsafe { clang_getCursorSpelling(child) }),
                        ),
                    ));
                }
            }
            for field in native_fields {
                let access = match unsafe { clang_getCXXAccessSpecifier(field) } {
                    CX_CXXPrivate => FieldAccess::Private,
                    CX_CXXProtected => FieldAccess::Protected,
                    _ => FieldAccess::Public,
                };
                let ty = unsafe { clang_getCursorType(field) };
                if unsafe { clang_getCanonicalType(ty) }.kind == CXType_LValueReference {
                    return Err(unsupported(
                        field,
                        &format!(
                            "unsupported type `{}` in a record field",
                            text(unsafe { clang_getTypeSpelling(ty) })
                        ),
                    ));
                }
                let anonymous = unsafe {
                    clang_Cursor_isAnonymousRecordDecl(clang_getTypeDeclaration(ty)) != 0
                };
                let name = if anonymous {
                    String::new()
                } else {
                    member_name(field)?
                };
                if field_initializer(field)? {
                    return Err(unsupported(field, "field initializers are unsupported"));
                }
                let bit_width = if unsafe { clang_Cursor_isBitField(field) } != 0 {
                    Some(
                        unsafe { clang_getFieldDeclBitWidth(field) }
                            .try_into()
                            .map_err(|_| unsupported(field, "bitfield width unavailable"))?,
                    )
                } else {
                    None
                };
                if name.is_empty() && bit_width.is_none() && !anonymous {
                    return Err(unsupported(field, "unnamed field"));
                }
                fields.push(Field {
                    name,
                    access,
                    location: self.location(field),
                    written: self.ty(ty, field)?,
                    canonical: self.ty(unsafe { clang_getCanonicalType(ty) }, field)?,
                    offset_bits: unsafe { clang_Cursor_getOffsetOfField(field) }
                        .try_into()
                        .map_err(|_| unsupported(field, "field offset unavailable"))?,
                    bit_width,
                    attributes: self.attributes(field)?,
                });
            }
            methods.sort_by_key(|method| method.slot);
            let mut info = WindowsClangRecordInfo::default();
            if unsafe { windows_clang_getRecordInfo(cursor, &raw mut info) } != WindowsClangSuccess
            {
                return Err(unsupported(
                    cursor,
                    "record special-member semantics unavailable",
                ));
            }
            Some(RecordDefinition {
                layout: layout(unsafe { clang_getCursorType(cursor) })
                    .map_err(|reason| unsupported(cursor, reason))?,
                semantics: RecordSemantics {
                    can_pass_in_registers: info.can_pass_in_registers != 0,
                    empty: info.empty != 0,
                    non_trivial_copy_assignment: info.non_trivial_copy_assignment != 0,
                    deleted_copy_assignment: info.deleted_copy_assignment != 0,
                    non_trivial_destructor: info.non_trivial_destructor != 0,
                    user_provided_constructor: info.user_provided_constructor != 0,
                    constructor_template: info.constructor_template != 0,
                },
                fields,
                enums,
                bases,
                methods,
                non_vtable_members,
            })
        } else {
            attributes = self.attributes(cursor)?;
            None
        };
        let observation = RecordObservation {
            record,
            location: self.location(cursor),
            definition,
            attributes,
        };
        let id = ObservationId(self.snapshot.observations.len());
        self.snapshot.observations.push(observation);
        self.snapshot.records[record.0].observations.push(id);
        Ok(())
    }

    fn attribute(&mut self, cursor: CXCursor) -> Result<Attribute, Error> {
        let (file, _, _, _) = source(cursor);
        let name = unsafe { windows_clang_getAttributeKindName(cursor) };
        if name.is_null() {
            return Err(unsupported(cursor, "native attribute identity unavailable"));
        }
        let name = unsafe { std::ffi::CStr::from_ptr(name) }.to_string_lossy();
        let kind = match name.as_ref() {
            "Packed" => AttributeKind::Packed,
            "FlagEnum" => AttributeKind::FlagEnum,
            "NoTrivialAutoVarInit" => AttributeKind::NoTrivialAutoVarInit,
            "MSAllocator" => AttributeKind::MSAllocator,
            "MSNoVTable" => AttributeKind::NoVTable,
            "DLLImport" => AttributeKind::DllImport,
            "DLLExport" => AttributeKind::DllExport,
            "NoThrow" => AttributeKind::NoThrow,
            "CXX11NoReturn" => AttributeKind::Cxx11NoReturn,
            "SelectAny" => AttributeKind::SelectAny,
            "Annotate" => annotations::read(cursor)?,
            "Deprecated" => {
                let mut message = CXString::default();
                let mut replacement = CXString::default();
                let status = unsafe {
                    windows_clang_getDeprecation(cursor, &raw mut message, &raw mut replacement)
                };
                if status != WindowsClangSuccess {
                    return Err(unsupported(
                        cursor,
                        &format!("deprecation query failed: {status}"),
                    ));
                }
                AttributeKind::Deprecated(Box::new(Deprecation {
                    message: text(message),
                    replacement: text(replacement),
                }))
            }
            "Uuid" => {
                let mut uuid = CXString::default();
                let status = unsafe { windows_clang_getUuid(cursor, &raw mut uuid) };
                if status != WindowsClangSuccess {
                    return Err(unsupported(cursor, &format!("UUID query failed: {status}")));
                }
                let uuid = text(uuid);
                let digits = uuid.trim_matches(['{', '}']).replace('-', "");
                let value = u128::from_str_radix(&digits, 16)
                    .map_err(|_| unsupported(cursor, "invalid native UUID payload"))?;
                if digits.len() != 32 {
                    return Err(unsupported(cursor, "invalid native UUID width"));
                }
                AttributeKind::Uuid(value)
            }
            "TypeVisibility" => {
                let mut visibility = CXVisibility_Invalid;
                let status =
                    unsafe { windows_clang_getTypeVisibility(cursor, &raw mut visibility) };
                if status != WindowsClangSuccess {
                    return Err(unsupported(
                        cursor,
                        &format!("type visibility query failed: {status}"),
                    ));
                }
                if visibility != CXVisibility_Default {
                    return Err(unsupported(
                        cursor,
                        &format!("unsupported type visibility {visibility}"),
                    ));
                }
                AttributeKind::DefaultTypeVisibility
            }
            "Aligned" => {
                let mut info = WindowsClangAlignmentInfo::default();
                let status = unsafe { windows_clang_getAlignment(cursor, &raw mut info) };
                if status != WindowsClangSuccess {
                    return Err(unsupported(
                        cursor,
                        &format!("alignment query failed: {status}"),
                    ));
                }
                let kind = match info.kind {
                    WindowsClangAlignmentGNU => AlignmentKind::Gnu,
                    WindowsClangAlignmentStandard => AlignmentKind::Standard,
                    WindowsClangAlignmentDeclspec => AlignmentKind::Declspec,
                    value => {
                        return Err(unsupported(
                            cursor,
                            &format!("unsupported alignment kind {value}"),
                        ));
                    }
                };
                AttributeKind::Aligned {
                    bits: info.bits,
                    kind,
                }
            }
            "MaxFieldAlignment" => {
                let mut bits = 0;
                let status = unsafe { windows_clang_getMaxFieldAlignment(cursor, &raw mut bits) };
                if status != WindowsClangSuccess {
                    return Err(unsupported(
                        cursor,
                        &format!("packing query failed: {status}"),
                    ));
                }
                AttributeKind::MaxFieldAlignment { bits }
            }
            _ => AttributeKind::Unresolved(name.into_owned()),
        };
        if self.partial && matches!(kind, AttributeKind::Unresolved(_)) {
            return Err(unsupported(
                cursor,
                &format!("unresolved attribute {kind:?}"),
            ));
        }
        Ok(Attribute {
            kind,
            location: (!file.is_empty()).then(|| self.location(cursor)),
        })
    }

    fn attributes(&mut self, cursor: CXCursor) -> Result<Vec<Attribute>, Error> {
        self.declaration_attributes(cursor, false, None)
    }

    fn declaration_attributes(
        &mut self,
        cursor: CXCursor,
        parameters: bool,
        default: Option<CXCursor>,
    ) -> Result<Vec<Attribute>, Error> {
        let mut result = Vec::new();
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0 {
                result.push(self.attribute(child)?);
            } else if kind == CXCursor_ParmDecl {
                if !parameters
                    && !matches!(
                        unsafe { clang_getCursorKind(cursor) },
                        CXCursor_FunctionDecl | CXCursor_CXXMethod
                    )
                    && !self.attributes(child)?.is_empty()
                {
                    return Err(unsupported(
                        child,
                        "attributed callback parameter is unsupported",
                    ));
                }
            } else if default.is_some_and(|default| Cursor(default) == Cursor(child)) {
                continue;
            } else if !matches!(kind, CXCursor_TypeRef | CXCursor_NamespaceRef)
                && !(unsafe { clang_isExpression(kind) } != 0 && layout_expression(cursor, child)?)
            {
                let is_underlying_record = unsafe {
                    if matches!(
                        clang_getCursorKind(cursor),
                        CXCursor_TypedefDecl | CXCursor_TypeAliasDecl | CXCursor_FieldDecl
                    ) && matches!(
                        kind,
                        CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_EnumDecl
                    ) {
                        let ty = if clang_getCursorKind(cursor) == CXCursor_FieldDecl {
                            clang_getCursorType(cursor)
                        } else {
                            clang_getTypedefDeclUnderlyingType(cursor)
                        };
                        contains_record(ty, Cursor(clang_getCanonicalCursor(child)))
                    } else {
                        false
                    }
                };
                if !is_underlying_record {
                    return Err(unsupported(child, "unsupported declaration child"));
                }
            }
        }
        Ok(result)
    }

    fn location(&mut self, cursor: CXCursor) -> Location {
        self.location_at(unsafe { clang_getCursorLocation(cursor) })
    }

    fn location_at(&mut self, location: CXSourceLocation) -> Location {
        let (name, line, column, offset) = source_location(location);
        let file = if let Some(&file) = self.files.get(&name) {
            file
        } else {
            let id = FileId(self.snapshot.files.len());
            self.snapshot.files.push(name.clone());
            self.files.insert(name, id);
            id
        };
        Location {
            file,
            line,
            column,
            offset,
        }
    }

    fn pointer(&mut self, ty: CXType, cursor: CXCursor) -> Result<TypeKind, Error> {
        let mut info = WindowsClangPointerInfo::default();
        let status = unsafe { windows_clang_getPointerInfo(ty, &raw mut info) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("pointer representation query failed: {status}"),
            ));
        }
        let fixed = match info.representation {
            WindowsClangNativePointer => false,
            WindowsClangPointer64 => true,
            representation => {
                return Err(unsupported(
                    cursor,
                    &format!("unsupported pointer representation: {representation}"),
                ));
            }
        };
        if fixed
            && matches!(
                unsafe { clang_getCanonicalType(info.pointee) }.kind,
                CXType_FunctionProto | CXType_FunctionNoProto
            )
        {
            return Err(unsupported(
                cursor,
                "fixed-width function pointers are unsupported",
            ));
        }
        let pointee = self.ty(info.pointee, cursor)?;
        Ok(if fixed {
            TypeKind::Pointer64(pointee)
        } else {
            TypeKind::Pointer(pointee)
        })
    }

    fn ty(&mut self, mut ty: CXType, cursor: CXCursor) -> Result<TypeId, Error> {
        let mut qualifiers = 0;
        let status = unsafe { windows_clang_getTypeQualifiers(ty, &raw mut qualifiers) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("type qualifier query failed: {status}"),
            ));
        }
        let mut supported =
            (WindowsClangConst | WindowsClangVolatile | WindowsClangRestrict) as u32;
        if qualifiers & WindowsClangUnaligned as u32 != 0 {
            let unqualified = unsafe { clang_getUnqualifiedType(clang_getCanonicalType(ty)) };
            if !matches!(
                unqualified.kind,
                CXType_Void
                    | CXType_FunctionProto
                    | CXType_FunctionNoProto
                    | CXType_LValueReference
                    | CXType_RValueReference
                    | CXType_IncompleteArray
            ) && layout(unqualified).is_ok_and(|layout| layout.alignment == 1)
            {
                supported |= WindowsClangUnaligned as u32;
            }
        }
        if qualifiers & !supported != 0 {
            return Err(unsupported(
                cursor,
                &format!("unsupported type qualifiers: {qualifiers:#x}"),
            ));
        }
        if ty.kind == CXType_Elaborated {
            ty = unsafe { clang_Type_getNamedType(ty) };
        }
        if ty.kind == CXType_Unexposed {
            let status = unsafe { windows_clang_getPredefinedType(ty, &raw mut ty) };
            if status != WindowsClangSuccess && status != WindowsClangWrongKind {
                return Err(unsupported(
                    cursor,
                    &format!("predefined type query failed: {status}"),
                ));
            }
        }
        let kind = unsafe {
            match ty.kind {
                CXType_Void => TypeKind::Void,
                CXType_Bool => TypeKind::Scalar(Scalar::Bool),
                CXType_Char_S | CXType_Char_U => TypeKind::Scalar(Scalar::Char {
                    signed: ty.kind == CXType_Char_S,
                }),
                CXType_SChar => TypeKind::Scalar(Scalar::SignedChar),
                CXType_UChar => TypeKind::Scalar(Scalar::UnsignedChar),
                CXType_WChar => TypeKind::Scalar(Scalar::WChar),
                CXType_Char16 => TypeKind::Scalar(Scalar::Char16),
                CXType_Char32 => TypeKind::Scalar(Scalar::Char32),
                CXType_Short => TypeKind::Scalar(Scalar::Short),
                CXType_UShort => TypeKind::Scalar(Scalar::UnsignedShort),
                CXType_Int => TypeKind::Scalar(Scalar::Int),
                CXType_UInt => TypeKind::Scalar(Scalar::UnsignedInt),
                CXType_Long => TypeKind::Scalar(Scalar::Long),
                CXType_ULong => TypeKind::Scalar(Scalar::UnsignedLong),
                CXType_LongLong => TypeKind::Scalar(Scalar::LongLong),
                CXType_ULongLong => TypeKind::Scalar(Scalar::UnsignedLongLong),
                CXType_Float => TypeKind::Scalar(Scalar::Float),
                CXType_Double => TypeKind::Scalar(Scalar::Double),
                CXType_LongDouble => TypeKind::Scalar(Scalar::LongDouble),
                CXType_Pointer => self.pointer(ty, cursor)?,
                CXType_ConstantArray => TypeKind::Array {
                    element: self.ty(clang_getArrayElementType(ty), cursor)?,
                    length: Some(
                        clang_getArraySize(ty)
                            .try_into()
                            .map_err(|_| unsupported(cursor, "array extent unavailable"))?,
                    ),
                },
                CXType_IncompleteArray => TypeKind::Array {
                    element: self.ty(clang_getArrayElementType(ty), cursor)?,
                    length: None,
                },
                CXType_Record => TypeKind::Record(self.record(clang_getTypeDeclaration(ty))?),
                CXType_Enum => TypeKind::Enum(self.enumeration(clang_getTypeDeclaration(ty))?),
                CXType_Typedef => TypeKind::Alias(self.alias(clang_getTypeDeclaration(ty))?),
                CXType_Attributed => {
                    let name = windows_clang_getTypeAttributeKindName(ty);
                    if name.is_null() {
                        return Err(unsupported(cursor, "type attribute identity unavailable"));
                    }
                    let name = std::ffi::CStr::from_ptr(name).to_string_lossy();
                    if name == "Ptr64" {
                        self.pointer(ty, cursor)?
                    } else {
                        let convention = match name.as_ref() {
                            "CDecl" => CallingConvention::C,
                            "StdCall" => CallingConvention::Stdcall,
                            _ => {
                                return Err(unsupported(
                                    cursor,
                                    &format!("unsupported type attribute {name}"),
                                ));
                            }
                        };
                        TypeKind::Attributed {
                            convention,
                            modified: self.ty(clang_Type_getModifiedType(ty), cursor)?,
                            equivalent: self.ty(clang_getCanonicalType(ty), cursor)?,
                        }
                    }
                }
                CXType_LValueReference => {
                    TypeKind::LValueReference(self.ty(clang_getPointeeType(ty), cursor)?)
                }
                CXType_FunctionProto => {
                    let mut info = WindowsClangFunctionInfo::default();
                    let status = windows_clang_getFunctionInfo(ty, &raw mut info);
                    if status != WindowsClangSuccess {
                        return Err(unsupported(
                            cursor,
                            &format!("function property query failed: {status}"),
                        ));
                    }
                    if info.has_regparm != 0
                        || info.produces_result != 0
                        || info.no_caller_saved_registers != 0
                        || info.no_cf_check != 0
                        || info.cmse_nonsecure_call != 0
                        || info.additional_properties & !(WindowsClangMethodQualifiers as u32) != 0
                    {
                        return Err(unsupported(
                            cursor,
                            &format!(
                                "unsupported function properties: regparm={:?}, noreturn={}, produces_result={}, no_caller_saved_registers={}, no_cf_check={}, cmse_nonsecure_call={}, additional={:#x}",
                                (info.has_regparm != 0).then_some(info.regparm),
                                info.no_return,
                                info.produces_result,
                                info.no_caller_saved_registers,
                                info.no_cf_check,
                                info.cmse_nonsecure_call,
                                info.additional_properties,
                            ),
                        ));
                    }
                    let exceptions = match clang_getExceptionSpecificationType(ty) {
                        CXCursor_ExceptionSpecificationKind_None => ExceptionSpecification::None,
                        CXCursor_ExceptionSpecificationKind_DynamicNone => {
                            ExceptionSpecification::DynamicNone
                        }
                        CXCursor_ExceptionSpecificationKind_BasicNoexcept => {
                            ExceptionSpecification::Noexcept
                        }
                        CXCursor_ExceptionSpecificationKind_NoThrow => {
                            ExceptionSpecification::NoThrow
                        }
                        value => {
                            return Err(unsupported(
                                cursor,
                                &format!("unsupported function exception specification: {value}"),
                            ));
                        }
                    };
                    let mut method_qualifiers = 0;
                    let status = windows_clang_getMethodQualifiers(ty, &raw mut method_qualifiers);
                    if status != WindowsClangSuccess || method_qualifiers & !supported != 0 {
                        return Err(unsupported(cursor, "unsupported method qualifiers"));
                    }
                    let convention = match clang_getFunctionTypeCallingConv(ty) {
                        CXCallingConv_C => CallingConvention::C,
                        CXCallingConv_X86StdCall => CallingConvention::Stdcall,
                        value => {
                            return Err(unsupported(
                                cursor,
                                &format!("unsupported calling convention {value}"),
                            ));
                        }
                    };
                    let count: u32 = clang_getNumArgTypes(ty)
                        .try_into()
                        .map_err(|_| unsupported(cursor, "callback parameter types unavailable"))?;
                    let parameters = (0..count)
                        .map(|index| self.ty(clang_getArgType(ty, index), cursor))
                        .collect::<Result<_, _>>()?;
                    TypeKind::Function(Box::new(Function {
                        convention,
                        exceptions,
                        noreturn: info.no_return != 0,
                        variadic: clang_isFunctionTypeVariadic(ty) != 0,
                        method_qualifiers: Qualifiers {
                            constant: method_qualifiers & WindowsClangConst as u32 != 0,
                            volatile: method_qualifiers & WindowsClangVolatile as u32 != 0,
                            restrict: method_qualifiers & WindowsClangRestrict as u32 != 0,
                            unaligned: false,
                        },
                        result: self.ty(clang_getResultType(ty), cursor)?,
                        parameters,
                    }))
                }
                _ => {
                    return Err(unsupported(
                        cursor,
                        &format!("unsupported type `{}`", text(clang_getTypeSpelling(ty))),
                    ));
                }
            }
        };
        let canonical = unsafe { clang_getCanonicalType(ty) };
        let incomplete = matches!(canonical.kind, CXType_Record | CXType_Enum)
            && unsafe { clang_Type_getSizeOf(ty) } == i64::from(CXTypeLayoutError_Incomplete);
        let value = Type {
            kind,
            qualifiers: Qualifiers {
                constant: qualifiers & WindowsClangConst as u32 != 0,
                volatile: qualifiers & WindowsClangVolatile as u32 != 0,
                restrict: qualifiers & WindowsClangRestrict as u32 != 0,
                unaligned: qualifiers & WindowsClangUnaligned as u32 != 0,
            },
            layout: if matches!(
                canonical.kind,
                CXType_Void
                    | CXType_FunctionProto
                    | CXType_LValueReference
                    | CXType_IncompleteArray
            ) || incomplete
            {
                None
            } else {
                Some(layout(ty).map_err(|reason| unsupported(cursor, reason))?)
            },
        };
        if let Some(&id) = self.types.get(&value) {
            return Ok(id);
        }
        let id = TypeId(self.snapshot.types.len());
        self.snapshot.types.push(value.clone());
        self.types.insert(value, id);
        Ok(id)
    }
}
