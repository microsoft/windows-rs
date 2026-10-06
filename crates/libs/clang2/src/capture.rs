#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use std::ffi::{CStr, CString};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// Captures the native closure of explicitly named roots and their cross-TU observations.
///
/// Roots use qualified native names, for example `API::Packet`. References and output exclusions
/// do not participate in capture. Unknown roots and compiler errors are reported as errors.
pub fn capture(
    inputs: impl IntoIterator<Item = Input>,
    arguments: &[&str],
    roots: &[&str],
) -> Result<Snapshot, Error> {
    let mut inputs: Vec<_> = inputs.into_iter().collect();
    inputs.sort_by(|left, right| left.name.cmp(&right.name));
    if inputs.is_empty() || roots.is_empty() {
        return Err(Error("capture requires inputs and named roots".into()));
    }
    if roots.iter().any(|root| root.is_empty()) {
        return Err(Error("native root names cannot be empty".into()));
    }
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
    for (input, unit) in inputs.iter().zip(&mut units) {
        let mut selected = BTreeMap::new();
        for cursor in children(unsafe { clang_getTranslationUnitCursor(unit.raw) }) {
            if unsafe { clang_getCursorKind(cursor) } != CXCursor_MacroDefinition {
                continue;
            }
            let name = string(unsafe { clang_getCursorSpelling(cursor) });
            if roots.contains(&name.as_str()) {
                if unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0 {
                    return Err(Error(format!(
                        "function-like macro `{name}` cannot be a constant root"
                    )));
                }
                selected.insert(name, location(cursor));
            }
        }
        if !selected.is_empty() {
            let mut source = input.source.clone();
            for name in selected.keys() {
                writeln!(
                    source,
                    "\n#ifndef {name}\n#error selected macro is undefined: {name}\n#endif\n\
                    const auto __clang2_value_{name} = ({name});\n\
                    const __INTPTR_TYPE__ __clang2_bits_{name} = (__INTPTR_TYPE__)({name});"
                )
                .unwrap();
            }
            *unit = Unit::parse(&Input::new(&input.name, source), arguments)?;
        }
        macros.push(selected);
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
        contexts: (0..units.len()).map(|_| vec![]).collect(),
        macros,
        identities: BTreeMap::new(),
        interned: (0..units.len()).map(|_| HashMap::new()).collect(),
        pending: VecDeque::new(),
    };
    let mut selected = vec![];
    let mut found = BTreeSet::new();
    for (unit, parsed) in units.iter().enumerate() {
        for cursor in declarations(unsafe { clang_getTranslationUnitCursor(parsed.raw) }) {
            if matches!(
                unsafe { clang_getCursorKind(cursor) },
                CXCursor_FunctionDecl | CXCursor_TypedefDecl | CXCursor_TypeAliasDecl
            ) {
                capture.contexts[unit].push(cursor);
            }
            let identity = string(unsafe { clang_getCursorUSR(cursor) });
            if !identity.is_empty() {
                let key = capture.candidate(unit, cursor, &identity);
                capture
                    .identities
                    .entry(key)
                    .or_default()
                    .push((unit, cursor));
            }
            let name = capture.name(unit, cursor);
            if roots.contains(&name.as_str()) {
                found.insert(name);
                selected.push((unit, cursor));
            }
        }
    }
    for root in roots {
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
    Ok(Snapshot {
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
}

impl Unit {
    fn parse(input: &Input, arguments: &[&str]) -> Result<Self, Error> {
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
            };
            let mut errors = false;
            for i in 0..clang_getNumDiagnostics(raw) {
                let diagnostic = clang_getDiagnostic(raw, i);
                errors |= clang_getDiagnosticSeverity(diagnostic) >= CXDiagnostic_Error;
                unit.diagnostics.push(string(clang_formatDiagnostic(
                    diagnostic,
                    clang_defaultDiagnosticDisplayOptions(),
                )));
                clang_disposeDiagnostic(diagnostic);
            }
            if errors {
                return Err(Error(unit.diagnostics.join("\n")));
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
    contexts: Vec<Vec<CXCursor>>,
    macros: Vec<BTreeMap<String, Location>>,
    identities: BTreeMap<String, Vec<(usize, CXCursor)>>,
    interned: Vec<HashMap<u32, Vec<(CXCursor, Id)>>>,
    pending: VecDeque<(usize, CXCursor, Id)>,
}

impl Capture<'_> {
    fn name(&self, unit: usize, cursor: CXCursor) -> String {
        let name = qualified_name(cursor);
        if let Some(name) = name.strip_prefix("__clang2_value_")
            && self.macros[unit].contains_key(name)
        {
            return name.to_string();
        }
        name
    }

    fn candidate(&self, unit: usize, cursor: CXCursor, identity: &str) -> String {
        let name = self.name(unit, cursor);
        if self.macros[unit].contains_key(&name) {
            format!("macro:{name}")
        } else {
            candidate(&self.units[unit].name, cursor, identity)
        }
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
        self.declarations.push(Declaration {
            entity,
            location: self.macros[unit]
                .get(&name)
                .cloned()
                .unwrap_or_else(|| location(cursor)),
            candidate: self.candidate(unit, canonical, &identity),
            name,
            identity,
            unit: self.units[unit].name.clone(),
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
                for child in children(cursor) {
                    match unsafe { clang_getCursorKind(child) } {
                        CXCursor_FieldDecl => {
                            let offset = unsafe { clang_Cursor_getOffsetOfField(child) };
                            let bit_width = (unsafe { clang_Cursor_isBitField(child) } != 0)
                                .then(|| unsafe { clang_getFieldDeclBitWidth(child) });
                            if offset < 0 || bit_width.is_some_and(|width| width < 0) {
                                unavailable.push("field layout unavailable".into());
                            }
                            fields.push(Field {
                                name: string(unsafe { clang_getCursorSpelling(child) }),
                                ty: self.ty(unit, unsafe { clang_getCursorType(child) })?,
                                offset,
                                bit_width,
                            });
                        }
                        CXCursor_CXXBaseSpecifier => {
                            bases.push(self.ty(unit, unsafe { clang_getCursorType(child) })?);
                            if unsafe { clang_isVirtualBase(child) } != 0 {
                                unavailable.push("virtual base layout is not implemented".into());
                            }
                        }
                        CXCursor_CXXMethod => methods.push(Method {
                            name: string(unsafe { clang_getCursorSpelling(child) }),
                            ty: self.ty(unit, unsafe { clang_getCursorType(child) })?,
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
                        }),
                        CXCursor_Constructor | CXCursor_Destructor => unavailable.push(format!(
                            "{} capture is not implemented",
                            string(unsafe {
                                clang_getCursorKindSpelling(clang_getCursorKind(child))
                            })
                        )),
                        CXCursor_StructDecl | CXCursor_UnionDecl
                            if unsafe { clang_Cursor_isAnonymousRecordDecl(child) } != 0 =>
                        {
                            unavailable
                                .push("anonymous aggregate capture is not implemented".into());
                        }
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
                    kind: string(unsafe { clang_getCursorKindSpelling(kind) }),
                    complete,
                    layout,
                    fields,
                    bases,
                    methods,
                    guid: complete.then(|| uuid(cursor)).flatten(),
                    unavailable,
                }
            }
            CXCursor_TypedefDecl | CXCursor_TypeAliasDecl => DeclarationData::Alias {
                target: self.ty(unit, unsafe { clang_getTypedefDeclUnderlyingType(cursor) })?,
                canonical: self.ty(unit, unsafe {
                    clang_getCanonicalType(clang_getTypedefDeclUnderlyingType(cursor))
                })?,
                parameters: self.parameters(unit, cursor)?,
            },
            CXCursor_EnumDecl => DeclarationData::Enum {
                complete: unsafe { clang_isCursorDefinition(cursor) } != 0,
                scoped: unsafe { clang_EnumDecl_isScoped(cursor) } != 0,
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
                ty: self.ty(unit, unsafe { clang_getCursorType(cursor) })?,
                canonical: self.ty(unit, unsafe {
                    clang_getCanonicalType(clang_getCursorType(cursor))
                })?,
                link_name: string(unsafe { clang_Cursor_getMangling(cursor) }),
                parameters: self.parameters(unit, cursor)?,
            },
            CXCursor_VarDecl => DeclarationData::Variable {
                ty: {
                    let source = if self.macros[unit].contains_key(&self.name(unit, cursor)) {
                        children(cursor)
                            .into_iter()
                            .find(|child| unsafe {
                                clang_isExpression(clang_getCursorKind(*child)) != 0
                            })
                            .ok_or_else(|| {
                                Error("selected macro probe has no initializer".into())
                            })?
                    } else {
                        cursor
                    };
                    self.ty(unit, unsafe { clang_getCursorType(source) })?
                },
                value: {
                    let value = evaluate(cursor);
                    let name = self.name(unit, cursor);
                    if matches!(value, Value::Unavailable(_))
                        && self.macros[unit].contains_key(&name)
                        && unsafe { clang_getCanonicalType(clang_getCursorType(cursor)) }.kind
                            == CXType_Pointer
                    {
                        let bits = format!("__clang2_bits_{name}");
                        let bits = children(unsafe {
                            clang_getTranslationUnitCursor(self.units[unit].raw)
                        })
                        .into_iter()
                        .find(|cursor| string(unsafe { clang_getCursorSpelling(*cursor) }) == bits)
                        .ok_or_else(|| {
                            Error(format!("missing pointer constant probe for `{name}`"))
                        })?;
                        evaluate(bits)
                    } else {
                        value
                    }
                },
            },
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
        params.into_iter().map(|parameter| {
            let annotations = children(parameter).into_iter()
                .filter(|attr| unsafe { clang_getCursorKind(*attr) } == CXCursor_AnnotateAttr)
                .map(|attr| {
                    let location = expansion_location(unsafe { clang_getCursorLocation(attr) });
                    let owner = self.contexts[unit].iter().copied().chain([cursor]).find(|owner| {
                        let range = unsafe { clang_getCursorExtent(*owner) };
                        let start = expansion_location(unsafe { clang_getRangeStart(range) });
                        let end = expansion_location(unsafe { clang_getRangeEnd(range) });
                        start.file == location.file && end.file == location.file
                            && start.offset <= location.offset && location.offset <= end.offset
                    }).ok_or_else(|| Error(format!("annotation context unavailable at {}:{}", location.file, location.line)))?;
                    Ok(Annotation {
                        text: string(unsafe { clang_getCursorSpelling(attr) }),
                        context: children(owner).into_iter()
                            .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
                            .map(|child| string(unsafe { clang_getCursorSpelling(child) })).collect(),
                        location,
                    })
                }).collect::<Result<_, Error>>()?;
            Ok(Parameter { name: string(unsafe { clang_getCursorSpelling(parameter) }), annotations })
        }).collect()
    }

    fn ty(&mut self, unit: usize, ty: CXType) -> Result<Type, Error> {
        let qualifiers = Qualifiers {
            constant: unsafe { clang_isConstQualifiedType(ty) } != 0,
            volatile: unsafe { clang_isVolatileQualifiedType(ty) } != 0,
            restrict: unsafe { clang_isRestrictQualifiedType(ty) } != 0,
        };
        let kind = match ty.kind {
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
/// MIDL_INTERFACE expands to this attribute. The literal is read from tokens because libclang's
/// stable API does not expose attribute arguments directly.
fn uuid(cursor: CXCursor) -> Option<String> {
    let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
    for attr in children(cursor) {
        if unsafe { clang_getCursorKind(attr) } != CXCursor_UnexposedAttr {
            continue;
        }
        let range = expansion_range(tu, unsafe { clang_getCursorExtent(attr) });
        let mut tokens = std::ptr::null_mut();
        let mut count = 0;
        unsafe { clang_tokenize(tu, range, &mut tokens, &mut count) };
        let mut found = None;
        for index in 0..count {
            let token = unsafe { *tokens.add(index as usize) };
            if unsafe { clang_getTokenKind(token) } != CXToken_Literal {
                continue;
            }
            let spelling = string(unsafe { clang_getTokenSpelling(tu, token) });
            let value = spelling.trim_matches('"');
            if is_uuid(value) {
                found = Some(value.to_ascii_lowercase());
                break;
            }
        }
        unsafe { clang_disposeTokens(tu, tokens, count) };
        if found.is_some() {
            return found;
        }
    }
    None
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
    unsafe {
        if clang_Cursor_isNull(clang_Cursor_getVarDeclInitializer(cursor)) != 0 {
            return Value::None;
        }
        if clang_Type_getSizeOf(clang_getCursorType(cursor)) > 8 {
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

fn children(cursor: CXCursor) -> Vec<CXCursor> {
    struct State {
        cursors: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        let state = unsafe { &mut *data.cast::<State>() };
        match catch_unwind(AssertUnwindSafe(|| state.cursors.push(cursor))) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                state.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }
    let mut state = State {
        cursors: vec![],
        panic: None,
    };
    unsafe { clang_visitChildren(cursor, visit, (&mut state as *mut State).cast()) };
    if let Some(panic) = state.panic {
        resume_unwind(panic);
    }
    state.cursors
}
