use super::*;

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
        let mut language = WindowsClangNoLanguageLinkage;
        let status = unsafe { windows_clang_getLanguageLinkage(cursor, &raw mut language) };
        let language = match (status, language) {
            (WindowsClangSuccess, WindowsClangCLanguageLinkage) => LanguageLinkage::C,
            (WindowsClangSuccess, WindowsClangCXXLanguageLinkage) => LanguageLinkage::Cxx,
            _ => return Err(unsupported(cursor, "function language linkage unavailable")),
        };
        let count: u32 = unsafe { clang_Cursor_getNumArguments(cursor) }
            .try_into()
            .map_err(|_| unsupported(cursor, "function parameters unavailable"))?;
        let mut parameters = Vec::new();
        for index in 0..count {
            let parameter = unsafe { clang_Cursor_getArgument(cursor, index) };
            parameters.push(Parameter {
                name: text(unsafe { clang_getCursorSpelling(parameter) }),
                written: self.ty(unsafe { clang_getCursorType(parameter) }, parameter)?,
                location: self.location(parameter),
                attributes: self.attributes(parameter)?,
            });
        }
        Ok(ValueObservation {
            location: self.location(cursor),
            written,
            canonical,
            attributes: self.attributes(cursor)?,
            kind: ValueKind::Function {
                parameters,
                mangling: text(unsafe { clang_Cursor_getMangling(cursor) }),
                language,
            },
        })
    }

    fn constant_observation(&mut self, cursor: CXCursor) -> Result<ValueObservation, Error> {
        let mut constant = 0;
        let status =
            unsafe { windows_clang_getIntegerConstantInitializer(cursor, &raw mut constant) };
        if status != WindowsClangSuccess {
            return Err(unsupported(
                cursor,
                &format!("integer initializer query failed: {status}"),
            ));
        }
        if constant == 0 {
            return Err(unsupported(
                cursor,
                "initializer is not an integer constant expression",
            ));
        }
        let key = Cursor(unsafe { clang_getCanonicalCursor(cursor) });
        let definition = self.probes.get(&key).map(|(_, cursor)| *cursor);
        let ty = unsafe { clang_getCursorType(cursor) };
        let canonical = self.ty(unsafe { clang_getCanonicalType(ty) }, cursor)?;
        let native = &self.snapshot.types[canonical.0];
        let signed = match &native.kind {
            TypeKind::Scalar(scalar) if native.qualifiers.constant => scalar.signed_integer(),
            _ => None,
        }
        .ok_or_else(|| unsupported(cursor, "only constant integer declarations are supported"))?;
        let result = unsafe { clang_Cursor_Evaluate(cursor) };
        if result.is_null() {
            return Err(unsupported(cursor, "constant initializer is not evaluable"));
        }
        let value = unsafe {
            let value = (clang_EvalResult_getKind(result) == CXEval_Int).then(|| {
                if signed {
                    EnumValue::Signed(clang_EvalResult_getAsLongLong(result))
                } else {
                    EnumValue::Unsigned(clang_EvalResult_getAsUnsigned(result))
                }
            });
            clang_EvalResult_dispose(result);
            value
        }
        .ok_or_else(|| unsupported(cursor, "initializer does not evaluate to an integer"))?;
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
                canonical
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
            },
        })
    }
}
