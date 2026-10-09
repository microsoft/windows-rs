use super::*;

pub(super) fn immutable(ty: CXType) -> bool {
    let ty = unsafe { clang_getCanonicalType(ty) };
    if unsafe { clang_isVolatileQualifiedType(ty) } != 0 {
        return false;
    }
    match ty.kind {
        CXType_Record => record_fields(unsafe { clang_getTypeDeclaration(ty) })
            .into_iter()
            .all(|field| {
                (unsafe { clang_CXXField_isMutable(field) }) == 0
                    && immutable(unsafe { clang_getCursorType(field) })
            }),
        CXType_ConstantArray => immutable(unsafe { clang_getArrayElementType(ty) }),
        _ => true,
    }
}

pub(super) enum AggregateProbe {
    Scalar(String),
    Aggregate(Vec<Self>),
}

impl AggregateProbe {
    pub fn eligible(cursor: CXCursor) -> bool {
        // Probes must not change call evaluation or fill incomplete source initializers.
        if matches!(
            unsafe { clang_getCursorKind(cursor) },
            CXCursor_CallExpr | CXCursor_InitListExpr
        ) {
            return false;
        }
        if unsafe { clang_getCursorKind(cursor) } == CXCursor_DeclRefExpr
            && matches!(
                unsafe { clang_getCanonicalType(clang_getCursorType(cursor)) }.kind,
                CXType_Record | CXType_ConstantArray
            )
        {
            return false;
        }
        children(cursor)
            .into_iter()
            .filter(|child| unsafe { clang_isExpression(clang_getCursorKind(*child)) } != 0)
            .all(Self::eligible)
    }

    pub fn new(ty: CXType, expression: String) -> Option<Self> {
        let ty = unsafe { clang_getCanonicalType(ty) };
        match ty.kind {
            CXType_Record => {
                let declaration = unsafe { clang_getTypeDeclaration(ty) };
                if unsafe { clang_getCursorKind(declaration) } != CXCursor_StructDecl
                    || unsafe { clang_isPODType(ty) } == 0
                {
                    return None;
                }
                let fields = record_fields(declaration);
                if fields.is_empty() {
                    return None;
                }
                fields
                    .into_iter()
                    .map(|field| {
                        let name = string(unsafe { clang_getCursorSpelling(field) });
                        if name.is_empty() {
                            return None;
                        }
                        Self::new(
                            unsafe { clang_getCursorType(field) },
                            format!("{expression}.{name}"),
                        )
                    })
                    .collect::<Option<Vec<_>>>()
                    .map(Self::Aggregate)
            }
            CXType_ConstantArray => {
                let size = unsafe { clang_getArraySize(ty) };
                let element = unsafe { clang_getArrayElementType(ty) };
                (0..size)
                    .map(|index| Self::new(element, format!("{expression}[{index}]")))
                    .collect::<Option<Vec<_>>>()
                    .map(Self::Aggregate)
            }
            _ if unsafe { clang_Type_getSizeOf(ty) } <= 8
                && matches!(
                    ty.kind,
                    CXType_Bool
                        | CXType_Char_U
                        | CXType_UChar
                        | CXType_Char_S
                        | CXType_SChar
                        | CXType_UShort
                        | CXType_Short
                        | CXType_UInt
                        | CXType_Int
                        | CXType_ULong
                        | CXType_Long
                        | CXType_ULongLong
                        | CXType_LongLong
                        | CXType_Float
                        | CXType_Double
                        | CXType_Enum
                ) =>
            {
                Some(Self::Scalar(expression))
            }
            _ => None,
        }
    }

    pub fn write(&self, source: &mut String, name: &str) {
        let mut index = 0;
        self.write_inner(source, name, &mut index);
    }

    fn write_inner(&self, source: &mut String, name: &str, index: &mut usize) {
        match self {
            Self::Scalar(expression) => {
                writeln!(
                    source,
                    "constexpr auto __clang2_aggregate_{name}_{index} = {expression};"
                )
                .unwrap();
                *index += 1;
            }
            Self::Aggregate(fields) => {
                for field in fields {
                    field.write_inner(source, name, index);
                }
            }
        }
    }

    pub fn read(&self, cursors: &BTreeMap<String, CXCursor>, name: &str) -> Result<Value, Error> {
        let mut index = 0;
        self.read_inner(cursors, name, &mut index)
    }

    fn read_inner(
        &self,
        cursors: &BTreeMap<String, CXCursor>,
        name: &str,
        index: &mut usize,
    ) -> Result<Value, Error> {
        match self {
            Self::Scalar(_) => {
                let probe = format!("__clang2_aggregate_{name}_{index}");
                let cursor = cursors
                    .get(&probe)
                    .ok_or_else(|| Error(format!("missing aggregate constant probe `{probe}`")))?;
                *index += 1;
                Ok(evaluate(*cursor))
            }
            Self::Aggregate(fields) => {
                let mut values = vec![];
                for field in fields {
                    let value = field.read_inner(cursors, name, index)?;
                    if matches!(value, Value::Unavailable(_)) {
                        return Ok(value);
                    }
                    values.push(value);
                }
                Ok(Value::Aggregate(values))
            }
        }
    }
}
