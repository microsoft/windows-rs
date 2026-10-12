use super::*;

pub(super) struct Definition {
    name: String,
    tokens: Vec<String>,
    expansion: Location,
    spelling: Location,
    type_alias: bool,
}

pub(super) fn definition(
    unit: CXTranslationUnit,
    cursor: CXCursor,
    type_names: &BTreeSet<String>,
) -> Result<Definition> {
    if unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0 {
        return Err(error(
            cursor,
            "function-like macros require a separate contract",
        ));
    }
    let mut tokens = std::ptr::null_mut();
    let mut count = 0;
    unsafe {
        clang_tokenize(
            unit,
            clang_getCursorExtent(cursor),
            &raw mut tokens,
            &raw mut count,
        );
    };
    let values = if count == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(tokens, count as usize) }
            .iter()
            .filter(|token| unsafe { clang_getTokenKind(**token) } != CXToken_Comment)
            .map(|token| text(unsafe { clang_getTokenSpelling(unit, *token) }))
            .collect::<Vec<_>>()
    };
    unsafe { clang_disposeTokens(unit, tokens, count) };
    let name = text(unsafe { clang_getCursorSpelling(cursor) });
    if values.first() != Some(&name) || values.len() < 2 {
        return Err(error(cursor, "macro has no captured replacement tokens"));
    }
    let tokens = values[1..].to_vec();
    let type_alias = tokens.len() == 1 && type_names.contains(&tokens[0]);
    let (expansion, spelling) = locations(cursor);
    Ok(Definition {
        name,
        tokens,
        expansion,
        spelling,
        type_alias,
    })
}

pub(super) fn capture(
    compiler: &Compiler,
    input: &Input,
    definitions: Vec<Definition>,
) -> Result<(Vec<MacroEvidence>, MacroProbe)> {
    let mut source = input.source.clone();
    source.push_str("\nnamespace __windows_clang2_macros {\n");
    for (index, definition) in definitions.iter().enumerate() {
        if definition.type_alias {
            source.push_str(&format!("using alias_{index} = {};\n", definition.name));
        } else {
            source.push_str(&format!(
                "constexpr auto value_{index} = ({});\n",
                definition.name
            ));
        }
    }
    source.push_str("}\n");
    let mut probe_input = Input::new(&input.name, &source, input.target);
    probe_input.arguments.clone_from(&input.arguments);
    probe_input.roots.clone_from(&input.roots);
    let session = Session::new(Arc::clone(&compiler.library), &probe_input)?;
    let cursors = children(unsafe { clang_getTranslationUnitCursor(session.unit) });
    let namespaces = cursors
        .iter()
        .filter(|cursor| {
            (unsafe { clang_getCursorKind(**cursor) }) == CXCursor_Namespace
                && text(unsafe { clang_getCursorSpelling(**cursor) }) == "__windows_clang2_macros"
        })
        .copied()
        .collect::<Vec<_>>();
    let [namespace] = namespaces.as_slice() else {
        return Err(Error::new(
            Stage::Capture,
            "macro probe namespace is missing or collides",
        ));
    };
    let declarations = children(*namespace);
    let mut macros = Vec::new();
    for (index, definition) in definitions.into_iter().enumerate() {
        let active = cursors
            .iter()
            .filter(|cursor| {
                (unsafe { clang_getCursorKind(**cursor) }) == CXCursor_MacroExpansion
                    && locations(**cursor).0.file == input.name
                    && locations(**cursor).0.offset as usize >= input.source.len()
                    && text(unsafe { clang_getCursorSpelling(**cursor) }) == definition.name
            })
            .copied()
            .collect::<Vec<_>>();
        let [active] = active.as_slice() else {
            return Err(Error::new(
                Stage::Scope,
                format!(
                    "{}: macro has no unique expansion in the derived probe",
                    definition.name
                ),
            ));
        };
        let referenced = unsafe { clang_getCursorReferenced(*active) };
        if unsafe { clang_getCursorKind(referenced) } != CXCursor_MacroDefinition
            || locations(referenced).0 != definition.expansion
        {
            let mut failure = error(
                *active,
                "active macro definition differs from the selected root",
            );
            failure.stage = Stage::Scope;
            return Err(failure);
        }
        let name = if definition.type_alias {
            format!("alias_{index}")
        } else {
            format!("value_{index}")
        };
        let values = declarations
            .iter()
            .filter(|cursor| text(unsafe { clang_getCursorSpelling(**cursor) }) == name)
            .copied()
            .collect::<Vec<_>>();
        let [cursor] = values.as_slice() else {
            return Err(Error::new(
                Stage::Capture,
                format!(
                    "{}: macro probe declaration is missing or collides",
                    definition.name
                ),
            ));
        };
        let kind = if definition.type_alias {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(*cursor) };
            let ty = if ty.kind == CXType_Elaborated {
                unsafe { clang_Type_getNamedType(ty) }
            } else {
                ty
            };
            let target = unsafe { clang_getTypeDeclaration(ty) };
            if !matches!(
                unsafe { clang_getCursorKind(target) },
                CXCursor_TypedefDecl
                    | CXCursor_TypeAliasDecl
                    | CXCursor_StructDecl
                    | CXCursor_ClassDecl
                    | CXCursor_UnionDecl
                    | CXCursor_EnumDecl
            ) {
                return Err(error(
                    *cursor,
                    "macro type alias has no named native target",
                ));
            }
            alias(target)?
        } else {
            let initializer = unsafe { clang_Cursor_getVarDeclInitializer(*cursor) };
            let ty = unsafe { clang_getCursorType(initializer) };
            let canonical = unsafe { clang_getCanonicalType(ty) };
            if canonical.kind == CXType_Pointer
                && unsafe { clang_getPointeeType(canonical) }.kind == CXType_FunctionProto
                && definition.tokens.len() == 1
            {
                let mut pending = vec![initializer];
                let mut targets = Vec::new();
                while let Some(child) = pending.pop() {
                    if unsafe { clang_getCursorKind(child) } == CXCursor_DeclRefExpr {
                        targets.push(unsafe { clang_getCursorReferenced(child) });
                    }
                    pending.extend(children(child));
                }
                let [target] = targets.as_slice() else {
                    return Err(error(
                        *cursor,
                        "macro function alias has no unique native target",
                    ));
                };
                if unsafe { clang_getCursorKind(*target) } != CXCursor_FunctionDecl {
                    return Err(error(
                        *target,
                        "macro function alias does not reference a function",
                    ));
                }
                alias(*target)?
            } else {
                let scalar = if ty.kind == CXType_Unexposed
                    && unsafe { clang_getCursorKind(clang_getTypeDeclaration(ty)) }
                        == CXCursor_NoDeclFound
                {
                    canonical.kind
                } else {
                    ty.kind
                };
                if !matches!(
                    scalar,
                    CXType_Bool
                        | CXType_Char_S
                        | CXType_Char_U
                        | CXType_SChar
                        | CXType_UChar
                        | CXType_Short
                        | CXType_UShort
                        | CXType_Int
                        | CXType_UInt
                        | CXType_Long
                        | CXType_ULong
                        | CXType_LongLong
                        | CXType_ULongLong
                ) {
                    return Err(error(
                        *cursor,
                        &format!(
                            "{}: macro value requires a scalar integer expression without erased type contracts: {} ({})",
                            definition.name,
                            text(unsafe { clang_getTypeSpelling(ty) }),
                            ty.kind
                        ),
                    ));
                }
                let evaluation = unsafe { clang_Cursor_Evaluate(*cursor) };
                if evaluation.is_null() {
                    return Err(error(*cursor, "compiler did not evaluate macro expression"));
                }
                let value = if unsafe { clang_EvalResult_getKind(evaluation) } == CXEval_Int {
                    let signed = unsafe { clang_EvalResult_getAsLongLong(evaluation) };
                    let unsigned = unsafe { clang_EvalResult_getAsUnsigned(evaluation) };
                    if canonical.kind == CXType_Bool {
                        Ok(metadata::Value::Bool(signed != 0))
                    } else {
                        integer_value(*cursor, canonical.kind, signed, unsigned)
                    }
                } else {
                    Err(error(
                        *cursor,
                        "compiler macro evaluation is not an integer",
                    ))
                };
                unsafe { clang_EvalResult_dispose(evaluation) };
                MacroKind::Integer {
                    written_type: text(unsafe { clang_getTypeSpelling(ty) }),
                    canonical_type: text(unsafe { clang_getTypeSpelling(canonical) }),
                    value: value?,
                }
            }
        };
        macros.push(MacroEvidence {
            name: definition.name,
            tokens: definition.tokens,
            expansion: definition.expansion,
            spelling: definition.spelling,
            kind,
        });
    }
    Ok((
        macros,
        MacroProbe {
            source,
            arguments: session.arguments.clone(),
            diagnostics: session.diagnostics.clone(),
        },
    ))
}

fn alias(cursor: CXCursor) -> Result<MacroKind> {
    let (expansion, spelling) = locations(cursor);
    let usr = text(unsafe { clang_getCursorUSR(cursor) });
    if usr.is_empty() {
        return Err(error(cursor, "macro alias target has no compiler identity"));
    }
    Ok(MacroKind::Alias {
        qualified_name: names(cursor)?.join("::"),
        kind: text(unsafe { clang_getCursorKindSpelling(clang_getCursorKind(cursor)) }),
        usr,
        expansion,
        spelling,
    })
}
