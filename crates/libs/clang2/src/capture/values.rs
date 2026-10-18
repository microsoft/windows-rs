use super::*;

pub(super) fn language_linkage(cursor: CXCursor) -> Result<LanguageLinkage, Error> {
    let mut language = WindowsClangNoLanguageLinkage;
    let status = unsafe { windows_clang_getLanguageLinkage(cursor, &raw mut language) };
    match (status, language) {
        (WindowsClangSuccess, WindowsClangCLanguageLinkage) => Ok(LanguageLinkage::C),
        (WindowsClangSuccess, WindowsClangCXXLanguageLinkage) => Ok(LanguageLinkage::Cxx),
        _ => Err(unsupported(
            cursor,
            "declaration language linkage unavailable",
        )),
    }
}

impl Capture<'_> {
    pub(super) fn value(&mut self, cursor: CXCursor) -> Result<ValueId, Error> {
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        if let Some(&id) = self.values.get(&key) {
            return Ok(id);
        }
        let name = text(unsafe { clang_getCursorSpelling(cursor) });
        let (name, origin) = self.probes.get(&key).cloned().unwrap_or((name, cursor));
        let usr = text(unsafe { clang_getCursorUSR(origin) });
        if usr.is_empty() {
            return Err(unsupported(
                origin,
                "value declaration identity unavailable",
            ));
        }
        let id = ValueId(self.snapshot.values.len());
        self.snapshot.values.push(ValueDeclaration {
            name,
            usr,
            observations: Vec::new(),
        });
        self.values.insert(key, id);
        self.pending.push(key);
        Ok(id)
    }

    pub(super) fn value_observation(&mut self, id: ValueId, cursor: CXCursor) -> Result<(), Error> {
        let observation = if unsafe { clang_getCursorKind(cursor) } == CXCursor_FunctionDecl {
            if unsafe { clang_getCursorLinkage(cursor) } != CXLinkage_External
                || unsafe { clang_isCursorDefinition(cursor) } != 0
            {
                return Err(unsupported(
                    cursor,
                    "only external function declarations are supported",
                ));
            }
            let observation = self.function_observation(cursor)?;
            if !matches!(
                observation.kind,
                ValueKind::Function {
                    language: LanguageLinkage::C,
                    ..
                }
            ) {
                return Err(unsupported(
                    cursor,
                    "only C language linkage functions are supported",
                ));
            }
            observation
        } else {
            self.constant_observation(cursor)?
        };
        self.snapshot.values[id.0].observations.push(observation);
        Ok(())
    }

    pub(super) fn function_observation(
        &mut self,
        cursor: CXCursor,
    ) -> Result<ValueObservation, Error> {
        let ty = unsafe { clang_getCursorType(cursor) };
        let canonical = self.ty(unsafe { clang_getCanonicalType(ty) }, cursor)?;
        let written = self.ty(ty, cursor)?;
        let language = language_linkage(cursor)?;
        let count: u32 = unsafe { clang_Cursor_getNumArguments(cursor) }
            .try_into()
            .map_err(|_| unsupported(cursor, "function parameters unavailable"))?;
        let mut parameters = Vec::new();
        for index in 0..count {
            let parameter = unsafe { clang_Cursor_getArgument(cursor, index) };
            parameters.push(self.parameter(parameter)?);
        }
        self.parameter_comments(cursor, &mut parameters);
        let (convention_origin, stdcall_candidate) = if language == LanguageLinkage::C {
            self.import_convention(cursor, canonical)?
        } else {
            (CallingConventionOrigin::Unknown, None)
        };

        Ok(ValueObservation {
            location: self.location(cursor),
            written,
            canonical,
            attributes: self.attributes(cursor)?,
            kind: ValueKind::Function {
                parameters,
                mangling: text(unsafe { clang_Cursor_getMangling(cursor) }),
                language,
                convention_origin,
                stdcall_candidate,
            },
        })
    }

    fn import_convention(
        &mut self,
        cursor: CXCursor,
        canonical: TypeId,
    ) -> Result<(CallingConventionOrigin, Option<Box<FunctionCandidate>>), Error> {
        let mut info = WindowsClangImportConvention::default();
        let status = unsafe { windows_clang_getImportConvention(cursor, &raw mut info) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("import convention query failed: {status}"),
            ));
        }
        let mangling = text(info.stdcall_symbol);
        let origin = match info.origin {
            WindowsClangConventionUnknown => CallingConventionOrigin::Unknown,
            WindowsClangConventionDefault => CallingConventionOrigin::Default,
            WindowsClangConventionExplicit => CallingConventionOrigin::Explicit,
            value => {
                return Err(unsupported(
                    cursor,
                    &format!("unknown convention origin: {value}"),
                ));
            }
        };
        if info.stdcall_type.kind == CXType_Invalid {
            return Ok((origin, None));
        }
        let written = self.ty(info.stdcall_type, cursor)?;
        let alternative = self.ty(unsafe { clang_getCanonicalType(info.stdcall_type) }, cursor)?;
        let TypeKind::Function(original) = &self.snapshot.types[canonical.0].kind else {
            return Err(unsupported(cursor, "expected canonical function prototype"));
        };
        let TypeKind::Function(candidate) = &self.snapshot.types[alternative.0].kind else {
            return Err(unsupported(
                cursor,
                "expected alternative function prototype",
            ));
        };
        let mut expected = original.as_ref().clone();
        expected.convention = CallingConvention::Stdcall;
        if self.snapshot.target != Target::X86
            || origin != CallingConventionOrigin::Default
            || original.convention != CallingConvention::C
            || expected != **candidate
            || mangling.is_empty()
        {
            return Err(unsupported(cursor, "invalid compiler stdcall candidate"));
        }
        Ok((
            origin,
            Some(Box::new(FunctionCandidate {
                written,
                canonical: alternative,
                mangling,
            })),
        ))
    }

    fn parameter(&mut self, cursor: CXCursor) -> Result<Parameter, Error> {
        let mut expression = CXCursor::default();
        let mut spelling = CXString::default();
        let status = unsafe {
            windows_clang_getParameterDefaultArgument(
                cursor,
                &raw mut expression,
                &raw mut spelling,
            )
        };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("parameter default query failed: {status}"),
            ));
        }
        let spelling = text(spelling);
        let default = (unsafe { clang_Cursor_isNull(expression) } == 0).then_some(expression);
        Ok(Parameter {
            name: text(unsafe { clang_getCursorSpelling(cursor) }),
            written: self.ty(unsafe { clang_getCursorType(cursor) }, cursor)?,
            location: self.location(cursor),
            attributes: self.declaration_attributes(cursor, false, default)?,
            default: default.map(|expression| DefaultArgument {
                expression: spelling,
                location: self.location(expression),
            }),
            comments: Vec::new(),
        })
    }

    pub(super) fn callback_parameters(
        &mut self,
        cursor: CXCursor,
        mut ty: CXType,
    ) -> Result<Option<Vec<Parameter>>, Error> {
        loop {
            ty = unsafe {
                match ty.kind {
                    CXType_Elaborated => clang_Type_getNamedType(ty),
                    CXType_Attributed => clang_Type_getModifiedType(ty),
                    CXType_Pointer => clang_getPointeeType(ty),
                    CXType_FunctionProto => break,
                    _ => return Ok(None),
                }
            };
        }
        let parameters = children(cursor)
            .into_iter()
            .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_ParmDecl)
            .collect::<Vec<_>>();
        if parameters.len() != unsafe { clang_getNumArgTypes(ty) } as usize {
            return Err(unsupported(
                cursor,
                "callback parameter ownership is ambiguous",
            ));
        }
        let mut parameters = parameters
            .into_iter()
            .map(|parameter| self.parameter(parameter))
            .collect::<Result<Vec<_>, _>>()?;
        self.parameter_comments(cursor, &mut parameters);
        Ok(Some(parameters))
    }

    fn constant_observation(&mut self, cursor: CXCursor) -> Result<ValueObservation, Error> {
        let linkage = match unsafe { clang_getCursorLinkage(cursor) } {
            CXLinkage_Internal => ConstantLinkage::Internal,
            CXLinkage_External => ConstantLinkage::External {
                language: language_linkage(cursor)?,
                mangling: text(unsafe { clang_Cursor_getMangling(cursor) }),
            },
            _ => return Err(unsupported(cursor, "unsupported constant linkage")),
        };
        let ty = unsafe { clang_getCursorType(cursor) };
        let canonical = self.ty(unsafe { clang_getCanonicalType(ty) }, cursor)?;
        if !self.snapshot.types[canonical.0].qualifiers.constant {
            return Err(unsupported(
                cursor,
                "constant declaration requires a const-qualified type",
            ));
        }
        if unsafe { clang_getCursorTLSKind(cursor) } != CXTLS_None {
            return Err(unsupported(
                cursor,
                "thread-local constants are unsupported",
            ));
        }
        let declaration = unsafe {
            clang_Cursor_isNull(clang_Cursor_getVarDeclInitializer(cursor)) != 0
                && clang_isCursorDefinition(cursor) == 0
        };
        let value = if declaration && matches!(linkage, ConstantLinkage::External { .. }) {
            None
        } else {
            Some(self.constant_value(cursor, ty, canonical)?)
        };
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        let definition = self.probes.get(&key).map(|(_, cursor)| *cursor);
        if definition.is_some() && value.is_none() {
            return Err(unsupported(cursor, "macro probe requires an initializer"));
        }
        let mut attributes = Vec::new();
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0 {
                attributes.push(self.attribute(child)?);
            } else if kind != CXCursor_TypeRef && unsafe { clang_isExpression(kind) } == 0 {
                return Err(unsupported(child, "unsupported constant declaration child"));
            }
        }
        Ok(ValueObservation {
            location: self.location(definition.unwrap_or(cursor)),
            written: if definition.is_some() {
                let mut expression = unsafe { clang_getNullCursor() };
                let status = unsafe { windows_clang_getTypeOfOperand(cursor, &raw mut expression) };
                if status != WindowsClangSuccess {
                    return Err(unsupported(
                        cursor,
                        &format!("macro expression type unavailable: {status}"),
                    ));
                }
                self.ty(unsafe { clang_getCursorType(expression) }, expression)?
            } else {
                self.ty(ty, cursor)?
            },
            canonical,
            attributes,
            kind: ValueKind::Constant {
                value,
                origin: if definition.is_some() {
                    ConstantOrigin::Macro
                } else {
                    ConstantOrigin::Declaration
                },
                linkage,
            },
        })
    }

    fn constant_value(
        &mut self,
        cursor: CXCursor,
        ty: CXType,
        canonical: TypeId,
    ) -> Result<ConstantValue, Error> {
        let pointer = if unsafe { clang_getCanonicalType(ty) }.kind == CXType_Pointer {
            let mut value = 0;
            let status =
                unsafe { windows_clang_getConstantPointerInitializer(cursor, &raw mut value) };
            if status != WindowsClangSuccess {
                return Err(unsupported(
                    cursor,
                    &format!("numeric pointer initializer unavailable: {status}"),
                ));
            }
            Some(value)
        } else {
            let mut constant = 0;
            let status = unsafe { windows_clang_getConstantInitializer(cursor, &raw mut constant) };
            if status != WindowsClangSuccess {
                return Err(unsupported(
                    cursor,
                    &format!("constant initializer query failed: {status}"),
                ));
            }
            if constant == 0 {
                return Err(unsupported(
                    cursor,
                    "initializer is not a constant expression",
                ));
            }
            None
        };
        let native = &self.snapshot.types[canonical.0];
        let value_type = if matches!(native.kind, TypeKind::Enum(_)) {
            self.ty(
                unsafe {
                    clang_getEnumDeclIntegerType(clang_getTypeDeclaration(clang_getCanonicalType(
                        ty,
                    )))
                },
                cursor,
            )?
        } else {
            canonical
        };
        let value = if let Some(value) = pointer {
            ConstantValue::Pointer(value)
        } else if let TypeKind::Scalar(scalar) = &self.snapshot.types[value_type.0].kind {
            let result = unsafe { clang_Cursor_Evaluate(cursor) };
            if result.is_null() {
                return Err(unsupported(cursor, "constant initializer is not evaluable"));
            }
            unsafe {
                let value = match (scalar, clang_EvalResult_getKind(result)) {
                    (Scalar::Bool, CXEval_Int) => Some(ConstantValue::Boolean(
                        clang_EvalResult_getAsInt(result) != 0,
                    )),
                    (Scalar::Float, CXEval_Float) => {
                        let value = clang_EvalResult_getAsDouble(result) as f32;
                        value
                            .is_finite()
                            .then(|| ConstantValue::Float32(value.to_bits()))
                    }
                    (Scalar::Double, CXEval_Float) => {
                        let value = clang_EvalResult_getAsDouble(result);
                        value
                            .is_finite()
                            .then(|| ConstantValue::Float64(value.to_bits()))
                    }
                    (scalar, CXEval_Int) => scalar.signed_integer().map(|signed| {
                        ConstantValue::Integer(if signed {
                            EnumValue::Signed(clang_EvalResult_getAsLongLong(result))
                        } else {
                            EnumValue::Unsigned(clang_EvalResult_getAsUnsigned(result))
                        })
                    }),
                    _ => None,
                };
                clang_EvalResult_dispose(result);
                value
            }
            .ok_or_else(|| {
                unsupported(
                    cursor,
                    "initializer is not a supported finite scalar constant",
                )
            })?
        } else {
            aggregate_constant(cursor, ty)?
        };
        Ok(value)
    }
}

fn aggregate_constant(cursor: CXCursor, ty: CXType) -> Result<ConstantValue, Error> {
    let integer = |path: &[u32]| -> Result<u64, Error> {
        let mut value = 0;
        let status = unsafe {
            windows_clang_getConstantIntegerElement(
                cursor,
                path.as_ptr(),
                path.len() as u32,
                &raw mut value,
            )
        };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("aggregate constant element query failed: {status}"),
            ));
        }
        Ok(value)
    };
    let canonical = unsafe { clang_getCanonicalType(ty) };
    if canonical.kind == CXType_ConstantArray {
        let element = unsafe { clang_getCanonicalType(clang_getArrayElementType(canonical)) };
        let wide = matches!(element.kind, CXType_WChar | CXType_Char16);
        if wide || matches!(element.kind, CXType_Char_S | CXType_Char_U) {
            let count: u32 = unsafe { clang_getArraySize(canonical) }
                .try_into()
                .map_err(|_| unsupported(cursor, "string array size is unavailable"))?;
            let mut units = (0..count)
                .map(|index| {
                    integer(&[index])?
                        .try_into()
                        .map_err(|_| unsupported(cursor, "string code unit exceeds u16"))
                })
                .collect::<Result<Vec<u16>, Error>>()?;
            if units.pop() != Some(0) {
                return Err(unsupported(
                    cursor,
                    "string array requires a terminating zero",
                ));
            }
            if !wide && units.iter().any(|unit| *unit > 0x7f) {
                return Err(unsupported(
                    cursor,
                    "non-ASCII narrow string encoding is unsupported",
                ));
            }
            let value = String::from_utf16(&units)
                .map_err(|_| unsupported(cursor, "invalid UTF-16 string constant"))?;
            return Ok(ConstantValue::String { value, wide });
        }
    }
    let guid = |prefix: &[u32]| -> Result<u128, Error> {
        let leaf = |suffix: &[u32]| {
            let path: Vec<_> = prefix.iter().chain(suffix).copied().collect();
            integer(&path)
        };
        let mut value = u128::from(leaf(&[0])?) << 96
            | u128::from(leaf(&[1])?) << 80
            | u128::from(leaf(&[2])?) << 64;
        for index in 0..8 {
            value |= u128::from(leaf(&[3, index])?) << (56 - index * 8);
        }
        Ok(value)
    };
    if guid_type(ty) {
        return Ok(ConstantValue::Guid(guid(&[])?));
    }
    let ty = unsafe { clang_getCanonicalType(ty) };
    if ty.kind == CXType_Record {
        let fields = record_fields(unsafe { clang_getTypeDeclaration(ty) });
        if let [format, id] = fields.as_slice()
            && text(unsafe { clang_getCursorSpelling(*format) }) == "fmtid"
            && text(unsafe { clang_getCursorSpelling(*id) }) == "pid"
            && guid_type(unsafe { clang_getCursorType(*format) })
            && unsigned_field(*id, 4)
            && unsafe { clang_Cursor_getOffsetOfField(*format) } == 0
            && unsafe { clang_Cursor_getOffsetOfField(*id) } == 128
            && unsafe { clang_Type_getSizeOf(ty) } == 20
        {
            return Ok(ConstantValue::PropertyKey {
                format: guid(&[0])?,
                id: integer(&[1])?
                    .try_into()
                    .map_err(|_| unsupported(cursor, "property ID exceeds u32"))?,
            });
        }
    }
    Err(unsupported(cursor, "unsupported aggregate constant shape"))
}

fn unsigned_field(field: CXCursor, size: i64) -> bool {
    let ty = unsafe { clang_getCanonicalType(clang_getCursorType(field)) };
    matches!(
        ty.kind,
        CXType_UChar | CXType_UShort | CXType_UInt | CXType_ULong
    ) && unsafe { clang_Type_getSizeOf(ty) } == size
        && unsafe { clang_Cursor_isBitField(field) } == 0
}

fn guid_type(ty: CXType) -> bool {
    let ty = unsafe { clang_getCanonicalType(ty) };
    if ty.kind != CXType_Record || unsafe { clang_Type_getSizeOf(ty) } != 16 {
        return false;
    }
    let fields = record_fields(unsafe { clang_getTypeDeclaration(ty) });
    let [a, b, c, d] = fields.as_slice() else {
        return false;
    };
    for ((field, name), offset) in fields
        .iter()
        .zip(["Data1", "Data2", "Data3", "Data4"])
        .zip([0, 32, 48, 64])
    {
        if text(unsafe { clang_getCursorSpelling(*field) }) != name
            || unsafe { clang_Cursor_getOffsetOfField(*field) } != offset
        {
            return false;
        }
    }
    let array = unsafe { clang_getCanonicalType(clang_getCursorType(*d)) };
    unsigned_field(*a, 4)
        && unsigned_field(*b, 2)
        && unsigned_field(*c, 2)
        && array.kind == CXType_ConstantArray
        && unsafe { clang_getArraySize(array) } == 8
        && unsafe { clang_getCanonicalType(clang_getArrayElementType(array)) }.kind == CXType_UChar
}
