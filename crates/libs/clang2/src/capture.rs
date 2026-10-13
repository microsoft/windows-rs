#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use native::*;

pub(super) fn read(unit: CXTranslationUnit, snapshot: &mut Snapshot) -> Result<(), Error> {
    let declarations = children(unsafe { clang_getTranslationUnitCursor(unit) });
    let mut capture = Capture {
        records: HashMap::new(),
        types: HashMap::new(),
        files: HashMap::new(),
        snapshot,
    };
    for &cursor in &declarations {
        if unsafe { clang_getCursorKind(cursor) } != CXCursor_StructDecl
            || text(unsafe { clang_getCursorSpelling(cursor) }).is_empty()
        {
            return Err(unsupported(cursor, "expected a named struct declaration"));
        }
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if !capture.records.contains_key(&key) {
            let id = RecordId(capture.snapshot.records.len());
            let usr = text(unsafe { clang_getCursorUSR(cursor) });
            if usr.is_empty() {
                return Err(unsupported(cursor, "record identity unavailable"));
            }
            capture.snapshot.records.push(Record {
                name: text(unsafe { clang_getCursorSpelling(cursor) }),
                usr,
                observations: Vec::new(),
            });
            capture.records.insert(key, id);
        }
    }
    for cursor in declarations {
        let record = capture.records[&Cursor(unsafe { clang_getCanonicalCursor(cursor) })];
        let definition = if unsafe { clang_isCursorDefinition(cursor) } != 0 {
            let mut fields = Vec::new();
            for field in children(cursor) {
                if unsafe { clang_getCursorKind(field) } != CXCursor_FieldDecl
                    || unsafe { clang_Cursor_isBitField(field) } != 0
                    || !children(field).iter().all(|child| {
                        matches!(
                            unsafe { clang_getCursorKind(*child) },
                            CXCursor_TypeRef | CXCursor_IntegerLiteral
                        )
                    })
                {
                    return Err(unsupported(field, "only plain named fields are supported"));
                }
                let name = text(unsafe { clang_getCursorSpelling(field) });
                if name.is_empty() {
                    return Err(unsupported(field, "unnamed field"));
                }
                let ty = unsafe { clang_getCursorType(field) };
                fields.push(Field {
                    name,
                    location: capture.location(field),
                    written: capture.ty(ty, field)?,
                    canonical: capture.ty(unsafe { clang_getCanonicalType(ty) }, field)?,
                    offset_bits: unsafe { clang_Cursor_getOffsetOfField(field) }
                        .try_into()
                        .map_err(|_| unsupported(field, "field offset unavailable"))?,
                });
            }
            Some(RecordDefinition {
                layout: layout(unsafe { clang_getCursorType(cursor) })
                    .map_err(|reason| unsupported(cursor, reason))?,
                fields,
            })
        } else {
            if !children(cursor).is_empty() {
                return Err(unsupported(cursor, "annotated forward declaration"));
            }
            None
        };
        let observation = RecordObservation {
            record,
            location: capture.location(cursor),
            definition,
        };
        let id = ObservationId(capture.snapshot.observations.len());
        capture.snapshot.observations.push(observation);
        capture.snapshot.records[record.0].observations.push(id);
    }
    Ok(())
}

struct Capture<'a> {
    records: HashMap<Cursor, RecordId>,
    types: HashMap<Type, TypeId>,
    files: HashMap<String, FileId>,
    snapshot: &'a mut Snapshot,
}

impl Capture<'_> {
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
                CXType_Record => {
                    let declaration =
                        Cursor(clang_getCanonicalCursor(clang_getTypeDeclaration(ty)));
                    TypeKind::Record(*self.records.get(&declaration).ok_or_else(|| {
                        unsupported(cursor, "record is outside captured declarations")
                    })?)
                }
                _ => {
                    return Err(unsupported(
                        cursor,
                        &format!("unsupported type `{}`", text(clang_getTypeSpelling(ty))),
                    ));
                }
            }
        };
        let incomplete = ty.kind == CXType_Record
            && unsafe { clang_Type_getSizeOf(ty) } == i64::from(CXTypeLayoutError_Incomplete);
        let value = Type {
            kind,
            qualifiers: Qualifiers {
                constant: unsafe { clang_isConstQualifiedType(ty) } != 0,
                volatile: unsafe { clang_isVolatileQualifiedType(ty) } != 0,
                restrict: unsafe { clang_isRestrictQualifiedType(ty) } != 0,
            },
            layout: if ty.kind == CXType_Void || incomplete {
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
