#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use native::*;

pub(super) fn read(
    unit: CXTranslationUnit,
    roots: &[String],
    snapshot: &mut Snapshot,
) -> Result<(), Error> {
    let declarations = declarations(unsafe { clang_getTranslationUnitCursor(unit) });
    let mut capture = Capture {
        declarations: HashMap::new(),
        pending: Vec::new(),
        records: HashMap::new(),
        aliases: HashMap::new(),
        types: HashMap::new(),
        files: HashMap::new(),
        snapshot,
    };
    let mut names: HashMap<String, Vec<Cursor>> = HashMap::new();
    let mut all = Vec::new();
    for &cursor in &declarations {
        let name = text(unsafe { clang_getCursorSpelling(cursor) });
        if !matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_StructDecl | CXCursor_TypedefDecl
        ) || name.is_empty()
        {
            if roots.is_empty() {
                return Err(unsupported(
                    cursor,
                    "expected a named struct or typedef declaration",
                ));
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
        for name in roots {
            let Some(candidates) = names.get(name) else {
                return Err(Error(format!("requested type `{name}` not found")));
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
        let declaration = if unsafe { clang_getCursorKind(cursor.0) } == CXCursor_TypedefDecl {
            DeclarationId::Alias(capture.alias(cursor.0)?)
        } else {
            DeclarationId::Record(capture.record(cursor.0)?)
        };
        capture.snapshot.roots.push(declaration);
    }
    while let Some(key) = capture.pending.pop() {
        for cursor in capture.declarations.remove(&key).unwrap() {
            if let Some(&record) = capture.records.get(&key) {
                capture.observation(record, cursor)?;
            } else {
                let alias = capture.aliases[&key];
                capture.alias_observation(alias, cursor)?;
            }
        }
    }
    Ok(())
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
    records: HashMap<Cursor, RecordId>,
    aliases: HashMap<Cursor, AliasId>,
    types: HashMap<Type, TypeId>,
    files: HashMap<String, FileId>,
    snapshot: &'a mut Snapshot,
}

impl Capture<'_> {
    fn alias(&mut self, cursor: CXCursor) -> Result<AliasId, Error> {
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.aliases.get(&key) {
            return Ok(id);
        }
        if !self.declarations.contains_key(&key) {
            return Err(unsupported(cursor, "dependency is not a global typedef"));
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
        let observation = AliasObservation {
            location: self.location(cursor),
            underlying: self.ty(underlying, cursor)?,
            canonical: self.ty(unsafe { clang_getCanonicalType(underlying) }, cursor)?,
            attributes,
        };
        self.snapshot.aliases[alias.0]
            .observations
            .push(observation);
        Ok(())
    }

    fn record(&mut self, cursor: CXCursor) -> Result<RecordId, Error> {
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.records.get(&key) {
            return Ok(id);
        }
        if !self.declarations.contains_key(&key) {
            return Err(unsupported(
                cursor,
                "dependency is not a global named struct",
            ));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(unsupported(cursor, "record identity unavailable"));
        }
        let id = RecordId(self.snapshot.records.len());
        self.snapshot.records.push(Record {
            name: text(unsafe { clang_getCursorSpelling(cursor) }),
            usr,
            observations: Vec::new(),
        });
        self.records.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    fn observation(&mut self, record: RecordId, cursor: CXCursor) -> Result<(), Error> {
        let mut attributes = Vec::new();
        let definition = if unsafe { clang_isCursorDefinition(cursor) } != 0 {
            let mut fields = Vec::new();
            for field in children(cursor) {
                if unsafe { clang_isAttribute(clang_getCursorKind(field)) } != 0 {
                    attributes.push(self.attribute(field));
                    continue;
                }
                if unsafe { clang_getCursorKind(field) } != CXCursor_FieldDecl
                    || unsafe { clang_Cursor_isBitField(field) } != 0
                {
                    return Err(unsupported(
                        cursor,
                        &format!(
                            "only plain named fields are supported in `{}`: {} `{}`",
                            text(unsafe { clang_getCursorSpelling(cursor) }),
                            text(unsafe {
                                clang_getCursorKindSpelling(clang_getCursorKind(field))
                            }),
                            text(unsafe { clang_getCursorSpelling(field) }),
                        ),
                    ));
                }
                let name = text(unsafe { clang_getCursorSpelling(field) });
                if name.is_empty() {
                    return Err(unsupported(field, "unnamed field"));
                }
                let ty = unsafe { clang_getCursorType(field) };
                fields.push(Field {
                    name,
                    location: self.location(field),
                    written: self.ty(ty, field)?,
                    canonical: self.ty(unsafe { clang_getCanonicalType(ty) }, field)?,
                    offset_bits: unsafe { clang_Cursor_getOffsetOfField(field) }
                        .try_into()
                        .map_err(|_| unsupported(field, "field offset unavailable"))?,
                    attributes: self.attributes(field)?,
                });
            }
            Some(RecordDefinition {
                layout: layout(unsafe { clang_getCursorType(cursor) })
                    .map_err(|reason| unsupported(cursor, reason))?,
                fields,
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

    fn attribute(&mut self, cursor: CXCursor) -> Attribute {
        let (file, _, _, _) = source(cursor);
        let kind = unsafe { clang_getCursorKind(cursor) };
        Attribute {
            kind: if kind == CXCursor_PackedAttr {
                AttributeKind::Packed
            } else {
                AttributeKind::Unresolved(text(unsafe { clang_getCursorKindSpelling(kind) }))
            },
            spelling: text(unsafe { clang_getCursorSpelling(cursor) }),
            tokens: tokens(cursor),
            location: (!file.is_empty()).then(|| self.location(cursor)),
        }
    }

    fn attributes(&mut self, cursor: CXCursor) -> Result<Vec<Attribute>, Error> {
        let mut result = Vec::new();
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0 {
                result.push(self.attribute(child));
            } else if !matches!(kind, CXCursor_TypeRef | CXCursor_IntegerLiteral) {
                let is_underlying_record = unsafe {
                    clang_getCursorKind(cursor) == CXCursor_TypedefDecl
                        && kind == CXCursor_StructDecl
                        && Cursor(clang_getCanonicalCursor(child))
                            == Cursor(clang_getCanonicalCursor(clang_getTypeDeclaration(
                                clang_getCanonicalType(clang_getTypedefDeclUnderlyingType(cursor)),
                            )))
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
                CXType_Typedef => TypeKind::Alias(self.alias(clang_getTypeDeclaration(ty))?),
                _ => {
                    return Err(unsupported(
                        cursor,
                        &format!("unsupported type `{}`", text(clang_getTypeSpelling(ty))),
                    ));
                }
            }
        };
        let canonical = unsafe { clang_getCanonicalType(ty) };
        let incomplete = canonical.kind == CXType_Record
            && unsafe { clang_Type_getSizeOf(ty) } == i64::from(CXTypeLayoutError_Incomplete);
        let value = Type {
            kind,
            qualifiers: Qualifiers {
                constant: unsafe { clang_isConstQualifiedType(ty) } != 0,
                volatile: unsafe { clang_isVolatileQualifiedType(ty) } != 0,
                restrict: unsafe { clang_isRestrictQualifiedType(ty) } != 0,
            },
            layout: if canonical.kind == CXType_Void || incomplete {
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
