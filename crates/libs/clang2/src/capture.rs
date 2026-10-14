#![allow(non_upper_case_globals)]

use super::*;
use native::*;
use windows_clang_sys::*;

mod values;

pub(super) fn read(
    unit: CXTranslationUnit,
    roots: &[String],
    macros: &[String],
    snapshot: &mut Snapshot,
    partial: bool,
) -> Result<partial::Issues, Error> {
    let mut issues = partial::Issues::default();
    let declarations = declarations(unsafe { clang_getTranslationUnitCursor(unit) });
    let mut expanded = HashMap::new();
    for &cursor in &declarations {
        if unsafe { clang_getCursorKind(cursor) } == CXCursor_MacroExpansion {
            let name = text(unsafe { clang_getCursorSpelling(cursor) });
            if macros.contains(&name) {
                expanded.insert(name, unsafe { clang_getCursorReferenced(cursor) });
            }
        }
    }
    let probes = macros
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let definition = expanded
                .get(name)
                .copied()
                .ok_or_else(|| Error(format!("macro `{name}` has no native expansion evidence")))?;
            if unsafe { clang_getCursorKind(definition) } != CXCursor_MacroDefinition
                || unsafe { clang_Cursor_isMacroFunctionLike(definition) } != 0
            {
                return Err(Error(format!(
                    "macro `{name}` is not an object-like definition"
                )));
            }
            Ok((
                format!("__windows_clang_macro_{index}"),
                (name.clone(), definition),
            ))
        })
        .collect::<Result<HashMap<_, _>, Error>>()?;
    let mut capture = Capture {
        declarations: HashMap::new(),
        pending: Vec::new(),
        records: HashMap::new(),
        aliases: HashMap::new(),
        enums: HashMap::new(),
        values: HashMap::new(),
        probes,
        types: HashMap::new(),
        files: HashMap::new(),
        partial,
        snapshot,
    };
    let mut names: HashMap<String, Vec<Cursor>> = HashMap::new();
    let mut all = Vec::new();
    for &cursor in &declarations {
        let name = text(unsafe { clang_getCursorSpelling(cursor) });
        let name = capture
            .probes
            .get(&name)
            .map_or(name.clone(), |(name, _)| name.clone());
        let kind = unsafe { clang_getCursorKind(cursor) };
        if unsafe { clang_isPreprocessing(kind) } != 0 {
            continue;
        }
        if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl)
            && matches!(
                unsafe { clang_getCursorKind(clang_getCursorSemanticParent(cursor)) },
                CXCursor_StructDecl | CXCursor_UnionDecl
            )
        {
            let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
            capture.declarations.entry(key).or_default().push(cursor);
            continue;
        }
        if !matches!(
            kind,
            CXCursor_StructDecl
                | CXCursor_UnionDecl
                | CXCursor_EnumDecl
                | CXCursor_TypedefDecl
                | CXCursor_TypeAliasDecl
                | CXCursor_FunctionDecl
                | CXCursor_VarDecl
        ) || name.is_empty()
        {
            if roots.is_empty() {
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
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        let observations = capture.declarations.entry(key).or_insert_with(|| {
            names.entry(name).or_default().push(key);
            all.push(key);
            Vec::new()
        });
        observations.push(cursor);
    }
    let mut selected = HashSet::new();
    let selected = if roots.is_empty() {
        all
    } else {
        let mut result = Vec::new();
        for name in roots.iter().chain(macros) {
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
    for cursor in selected {
        let declaration = if matches!(
            unsafe { clang_getCursorKind(cursor.0) },
            CXCursor_StructDecl | CXCursor_UnionDecl
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
                    partial::Key::of(capture.snapshot, id),
                    format!("{:?}: {error}", capture.snapshot.target),
                );
            }
        }
    }
    Ok(issues)
}

fn declarations(cursor: CXCursor) -> Vec<CXCursor> {
    let mut result = Vec::new();
    for child in children(cursor) {
        if unsafe { clang_getCursorKind(child) } == CXCursor_LinkageSpec {
            result.extend(declarations(child));
        } else {
            result.push(child);
        }
    }
    result
}

struct Capture<'a> {
    declarations: HashMap<Cursor, Vec<CXCursor>>,
    pending: Vec<Cursor>,
    values: HashMap<Cursor, ValueId>,
    probes: HashMap<String, (String, CXCursor)>,
    records: HashMap<Cursor, RecordId>,
    aliases: HashMap<Cursor, AliasId>,
    enums: HashMap<Cursor, EnumId>,
    types: HashMap<Type, TypeId>,
    files: HashMap<String, FileId>,
    partial: bool,
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
        let id = EnumId(self.snapshot.enums.len());
        self.snapshot.enums.push(Enum {
            name: text(unsafe { clang_getCursorSpelling(cursor) }),
            usr,
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
            } else if kind != CXCursor_TypeRef {
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
        if !self.declarations.contains_key(&key) {
            return Err(unsupported(cursor, "dependency is not a global type alias"));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(unsupported(cursor, "typedef identity unavailable"));
        }
        let id = AliasId(self.snapshot.aliases.len());
        self.snapshot.aliases.push(Alias {
            name: text(unsafe { clang_getCursorSpelling(cursor) }),
            usr,
            observations: Vec::new(),
        });
        self.aliases.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    fn alias_observation(&mut self, alias: AliasId, cursor: CXCursor) -> Result<(), Error> {
        let attributes = self.attributes(cursor)?;
        let underlying = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
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
            defines_tag: defines_tag != 0,
            attributes,
        };
        self.snapshot.aliases[alias.0]
            .observations
            .push(observation);
        Ok(())
    }

    fn record(&mut self, cursor: CXCursor) -> Result<RecordId, Error> {
        if !matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_StructDecl | CXCursor_UnionDecl
        ) {
            return Err(unsupported(
                cursor,
                "expected a struct or union declaration",
            ));
        }
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.records.get(&key) {
            return Ok(id);
        }
        let parent = unsafe { clang_getCursorSemanticParent(cursor) };
        let owner = if matches!(
            unsafe { clang_getCursorKind(parent) },
            CXCursor_StructDecl | CXCursor_UnionDecl
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
            name: if owner.is_some_and(|owner| owner.field.is_some()) {
                String::new()
            } else {
                text(unsafe { clang_getCursorSpelling(cursor) })
            },
            usr,
            owner,
            kind: if unsafe { clang_getCursorKind(cursor) } == CXCursor_UnionDecl {
                RecordKind::Union
            } else {
                RecordKind::Struct
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
            let mut fields = Vec::new();
            let mut bases = Vec::new();
            let mut methods = Vec::new();
            let mut member_templates = Vec::new();
            for child in children(cursor) {
                let kind = unsafe { clang_getCursorKind(child) };
                if unsafe { clang_isAttribute(kind) } != 0 {
                    attributes.push(self.attribute(child)?);
                    continue;
                }
                if matches!(kind, CXCursor_StructDecl | CXCursor_UnionDecl) {
                    self.record(child)?;
                    continue;
                }
                if kind == CXCursor_TypeRef && owner_reference(cursor, child) {
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
                    bases.push(self.record(unsafe {
                        clang_getTypeDeclaration(clang_getCanonicalType(clang_getCursorType(child)))
                    })?);
                    continue;
                }
                if kind == CXCursor_FunctionTemplate
                    && unsafe { clang_getTemplateCursorKind(child) } == CXCursor_CXXMethod
                {
                    member_templates.push(MemberTemplate {
                        name: text(unsafe { clang_getCursorSpelling(child) }),
                        usr: text(unsafe { clang_getCursorUSR(child) }),
                        location: self.location(child),
                    });
                    continue;
                }
                if kind == CXCursor_CXXMethod {
                    if unsafe { clang_CXXMethod_isPureVirtual(child) } == 0
                        || unsafe { clang_getCXXAccessSpecifier(child) } != CX_CXXPublic
                    {
                        return Err(unsupported(
                            child,
                            "only public pure virtual methods are supported",
                        ));
                    }
                    let mut overridden = std::ptr::null_mut();
                    let mut count = 0;
                    unsafe {
                        clang_getOverriddenCursors(child, &raw mut overridden, &raw mut count);
                        clang_disposeOverriddenCursors(overridden);
                    }
                    if count != 0 {
                        return Err(unsupported(
                            child,
                            "overridden virtual methods are unsupported",
                        ));
                    }
                    methods.push(Method {
                        name: text(unsafe { clang_getCursorSpelling(child) }),
                        observation: self.function_observation(child)?,
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
                if matches!(
                    unsafe { clang_getCXXAccessSpecifier(field) },
                    CX_CXXPrivate | CX_CXXProtected
                ) {
                    return Err(unsupported(field, "non-public fields are unsupported"));
                }
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
                    text(unsafe { clang_getCursorSpelling(field) })
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
            Some(RecordDefinition {
                layout: layout(unsafe { clang_getCursorType(cursor) })
                    .map_err(|reason| unsupported(cursor, reason))?,
                fields,
                bases,
                methods,
                member_templates,
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
            "MSNoVTable" => AttributeKind::NoVTable,
            "DLLImport" => AttributeKind::DllImport,
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
                &format!("unresolved attribute {kind:?}: {:?}", tokens(cursor)),
            ));
        }
        Ok(Attribute {
            kind,
            spelling: text(unsafe { clang_getCursorSpelling(cursor) }),
            tokens: tokens(cursor),
            location: (!file.is_empty()).then(|| self.location(cursor)),
        })
    }

    fn attributes(&mut self, cursor: CXCursor) -> Result<Vec<Attribute>, Error> {
        let mut result = Vec::new();
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0 {
                result.push(self.attribute(child)?);
            } else if kind == CXCursor_ParmDecl {
                if !self.attributes(child)?.is_empty() {
                    return Err(unsupported(
                        child,
                        "attributed callback parameter is unsupported",
                    ));
                }
            } else if kind != CXCursor_TypeRef
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
        let (name, line, column, offset) = source(cursor);
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

    fn ty(&mut self, mut ty: CXType, cursor: CXCursor) -> Result<TypeId, Error> {
        let mut qualifiers = 0;
        let status = unsafe { windows_clang_getTypeQualifiers(ty, &raw mut qualifiers) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("type qualifier query failed: {status}"),
            ));
        }
        let supported = (WindowsClangConst | WindowsClangVolatile | WindowsClangRestrict) as u32;
        if qualifiers & !supported != 0 {
            return Err(unsupported(
                cursor,
                &format!("unsupported type qualifiers: {qualifiers:#x}"),
            ));
        }
        if ty.kind == CXType_Elaborated {
            ty = unsafe { clang_Type_getNamedType(ty) };
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
                CXType_Pointer => TypeKind::Pointer(self.ty(clang_getPointeeType(ty), cursor)?),
                CXType_ConstantArray => TypeKind::Array {
                    element: self.ty(clang_getArrayElementType(ty), cursor)?,
                    length: clang_getArraySize(ty)
                        .try_into()
                        .map_err(|_| unsupported(cursor, "array extent unavailable"))?,
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
                        || info.no_return != 0
                        || info.produces_result != 0
                        || info.no_caller_saved_registers != 0
                        || info.no_cf_check != 0
                        || info.cmse_nonsecure_call != 0
                        || info.additional_properties != 0
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
                    if clang_isFunctionTypeVariadic(ty) != 0
                        || clang_getExceptionSpecificationType(ty)
                            != CXCursor_ExceptionSpecificationKind_None
                    {
                        return Err(unsupported(
                            cursor,
                            "variadic or exception-qualified callback is unsupported",
                        ));
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
            },
            layout: if matches!(
                canonical.kind,
                CXType_Void | CXType_FunctionProto | CXType_LValueReference
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
