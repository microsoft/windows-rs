use super::*;

pub(super) fn read(
    capture: &mut Capture<'_>,
    cursor: CXCursor,
    owner: CXCursor,
) -> Result<AttributeKind, Error> {
    let payload = text(unsafe { clang_getCursorSpelling(cursor) });
    if let Some(members) = payload
        .strip_prefix("Win32Metadata.Enum(")
        .or_else(|| payload.strip_prefix("Win32Metadata.FlagsEnum("))
    {
        return group(
            capture,
            cursor,
            owner,
            members,
            payload.starts_with("Win32Metadata.FlagsEnum("),
        );
    }
    if payload == "Win32Metadata.AssociatedEnum" {
        return association(capture, cursor, owner);
    }
    if matches!(
        payload.as_str(),
        "Win32Metadata.SetLastError" | "Win32Metadata.ImportLibrary"
    ) {
        let name = payload.strip_prefix("Win32Metadata.").unwrap();
        if unsafe { clang_getCursorKind(owner) } != CXCursor_FunctionDecl {
            return Err(unsupported(
                cursor,
                &format!("{name} requires a free function"),
            ));
        }
        let arguments = children(cursor);
        if name == "SetLastError" {
            if !arguments.is_empty() {
                return Err(unsupported(cursor, "SetLastError takes no arguments"));
            }
            return Ok(AttributeKind::SetLastError);
        }
        if !(1..=2).contains(&arguments.len()) {
            return Err(unsupported(
                cursor,
                "ImportLibrary requires a DLL and optional entry point",
            ));
        }
        let library = string(arguments[0], "ImportLibrary")?;
        if library.is_empty()
            || library
                .chars()
                .any(|c| c.is_control() || matches!(c, '/' | '\\' | ':'))
        {
            return Err(unsupported(cursor, "invalid ImportLibrary DLL name"));
        }
        let name = arguments
            .get(1)
            .map(|&argument| entry_point(argument))
            .transpose()?;
        return Ok(AttributeKind::ImportLibrary(Box::new(ImportLibrary {
            library,
            name,
        })));
    }
    Ok(
        match sal::transport::read(&payload).map_err(|error| unsupported(cursor, &error.0))? {
            Some(value) => AttributeKind::Sal(Box::new(value)),
            None => AttributeKind::Unresolved(format!("Annotate({payload:?})")),
        },
    )
}

fn group(
    capture: &mut Capture<'_>,
    cursor: CXCursor,
    owner: CXCursor,
    names: &str,
    flags: bool,
) -> Result<AttributeKind, Error> {
    if owner.kind != CXCursor_TypedefDecl {
        return Err(unsupported(cursor, "enum group requires a typedef"));
    }
    let names = names
        .strip_suffix(')')
        .ok_or_else(|| unsupported(cursor, "invalid enum group payload"))?;
    let arguments = children(cursor);
    let names: Vec<_> = names.split(',').map(str::trim).collect();
    if arguments.len() != names.len() || names.is_empty() {
        return Err(unsupported(
            cursor,
            "enum group requires named constant arguments",
        ));
    }
    let mut seen = HashSet::new();
    let mut members = Vec::new();
    for (name, argument) in names.into_iter().zip(arguments) {
        if !identifier(name) || !seen.insert(name) {
            return Err(unsupported(
                cursor,
                "enum group requires distinct unqualified member names",
            ));
        }
        let ty = unsafe { clang_getCanonicalType(clang_getCursorType(argument)) };
        if ty.kind == CXType_Enum {
            return Err(unsupported(
                argument,
                "associate the existing native enum instead of regrouping its members",
            ));
        }
        let id = capture.ty(ty, argument)?;
        let TypeKind::Scalar(scalar) = &capture.snapshot.types[id.0].kind else {
            return Err(unsupported(
                argument,
                "enum member must be an integral constant",
            ));
        };
        if scalar.signed_integer().is_none() {
            return Err(unsupported(
                argument,
                "enum member must be an integral constant",
            ));
        }
        let scalar = scalar.clone();
        let result = unsafe { clang_Cursor_Evaluate(argument) };
        if result.is_null() {
            return Err(unsupported(
                argument,
                "enum member must be a constant integer",
            ));
        }
        let value = unsafe {
            let value = (clang_EvalResult_getKind(result) == CXEval_Int).then(|| {
                if clang_EvalResult_isUnsignedInt(result) != 0 {
                    EnumValue::Unsigned(clang_EvalResult_getAsUnsigned(result))
                } else {
                    EnumValue::Signed(clang_EvalResult_getAsLongLong(result))
                }
            });
            clang_EvalResult_dispose(result);
            value
        }
        .ok_or_else(|| unsupported(argument, "enum member must be a constant integer"))?;
        let mut native = None;
        let mut pending = vec![argument];
        while let Some(expression) = pending.pop() {
            if expression.kind == CXCursor_DeclRefExpr {
                let declaration = unsafe { clang_getCursorReferenced(expression) };
                if declaration.kind == CXCursor_VarDecl
                    && text(unsafe { clang_getCursorSpelling(declaration) }) == name
                {
                    let key = Cursor(unsafe { clang_getCanonicalCursor(declaration) });
                    if !capture.declarations.contains_key(&key)
                        && !capture.values.contains_key(&key)
                    {
                        return Err(unsupported(
                            argument,
                            "enum member declaration must be a captured namespace-scope constant",
                        ));
                    }
                    let id = capture.value(declaration)?;
                    let name = &capture.snapshot.values[id.0].name;
                    if native.as_ref().is_some_and(|previous| previous != name) {
                        return Err(unsupported(argument, "ambiguous enum member declaration"));
                    }
                    native = Some(name.clone());
                }
            }
            pending.extend(children(expression));
        }
        members.push(EnumGroupMember {
            name: name.into(),
            native,
            scalar,
            value,
        });
    }
    Ok(AttributeKind::EnumGroup(Box::new(EnumAnnotation {
        flags,
        members,
    })))
}

fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == b'_')
        && bytes.all(|c| c.is_ascii_alphanumeric() || c == b'_')
}

fn association(
    capture: &mut Capture<'_>,
    cursor: CXCursor,
    owner: CXCursor,
) -> Result<AttributeKind, Error> {
    if !matches!(
        owner.kind,
        CXCursor_FunctionDecl | CXCursor_CXXMethod | CXCursor_ParmDecl | CXCursor_FieldDecl
    ) {
        return Err(unsupported(
            cursor,
            "AssociatedEnum requires a parameter, return value, or field",
        ));
    }
    let arguments = children(cursor);
    let [argument] = arguments.as_slice() else {
        return Err(unsupported(cursor, "AssociatedEnum requires one enum name"));
    };
    let argument = string(*argument, "AssociatedEnum")?;
    let name = argument
        .strip_prefix("Win32Metadata.EnumName:")
        .ok_or_else(|| unsupported(cursor, "AssociatedEnum requires an enum-name reference"))?;
    let tag = name.starts_with("enum ");
    let name = name.strip_prefix("enum ").unwrap_or(name).trim();
    let absolute = name.starts_with("::");
    let name = name.strip_prefix("::").unwrap_or(name);
    if name.is_empty() || !name.split("::").all(identifier) {
        return Err(unsupported(cursor, "invalid AssociatedEnum name"));
    }
    let mut scopes = Vec::new();
    if !absolute {
        let mut parent = unsafe { clang_getCursorSemanticParent(owner) };
        while !matches!(parent.kind, CXCursor_InvalidFile | CXCursor_NoDeclFound) {
            let declarations = children(parent);
            if declarations.iter().any(|child| {
                child.kind == CXCursor_UsingDirective
                    || child.kind == CXCursor_UsingDeclaration
                        && text(unsafe { clang_getCursorSpelling(*child) })
                            == name.split("::").next().unwrap()
            }) {
                return Err(unsupported(
                    cursor,
                    "using-based AssociatedEnum lookup is unsupported; use a globally qualified name",
                ));
            }
            if parent.kind == CXCursor_TranslationUnit {
                break;
            }
            if matches!(
                parent.kind,
                CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
            ) && (text(unsafe { clang_getCursorSpelling(parent) })
                == name.split("::").next().unwrap()
                || declarations.iter().any(|child| {
                    text(unsafe { clang_getCursorSpelling(*child) })
                        == name.split("::").next().unwrap()
                }))
            {
                return Err(unsupported(
                    cursor,
                    "class-scoped AssociatedEnum lookup is unsupported; use a qualified namespace name",
                ));
            }
            if parent.kind == CXCursor_Namespace {
                scopes.push(declaration_name(parent));
            }
            parent = unsafe { clang_getCursorSemanticParent(parent) };
        }
    }
    scopes.push(String::new());
    for scope in scopes {
        let qualified = if scope.is_empty() {
            name.into()
        } else {
            format!("{scope}::{name}")
        };
        let Some(candidates) = capture.named_types.get(&qualified) else {
            continue;
        };
        if tag
            && candidates.iter().any(|target| {
                matches!(
                    target.0.kind,
                    CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
                )
            })
        {
            return Err(unsupported(cursor, "AssociatedEnum target is not an enum"));
        }
        let candidates: Vec<_> = candidates
            .iter()
            .filter(|target| !tag || target.0.kind == CXCursor_EnumDecl)
            .collect();
        let Some(target) = candidates.first() else {
            continue;
        };
        let target = target.0;
        if matches!(
            target.kind,
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        ) {
            return Err(unsupported(
                cursor,
                "AssociatedEnum target is not an enum or enum group",
            ));
        }
        let canonical = unsafe { clang_getCanonicalType(clang_getCursorType(target)) };
        let declaration = unsafe { clang_getTypeDeclaration(canonical) };
        if candidates.iter().skip(1).any(|other| {
            canonical.kind != CXType_Enum
                || unsafe {
                    clang_equalCursors(
                        declaration,
                        clang_getTypeDeclaration(clang_getCanonicalType(clang_getCursorType(
                            other.0,
                        ))),
                    )
                } == 0
        }) {
            return Err(unsupported(cursor, "ambiguous AssociatedEnum name"));
        }
        let typedef = canonical.kind != CXType_Enum;
        let name = if typedef {
            qualified
        } else {
            declaration_name(declaration)
        };
        let ty = capture.ty(unsafe { clang_getCursorType(target) }, target)?;
        return Ok(AttributeKind::AssociatedEnum(Box::new(AssociatedEnum {
            name,
            typedef,
            ty,
        })));
    }
    Err(unsupported(
        cursor,
        &format!("AssociatedEnum target `{name}` not found"),
    ))
}

fn string(cursor: CXCursor, annotation: &str) -> Result<String, Error> {
    let mut value = CXString::default();
    if unsafe { windows_clang_getStringLiteral(cursor, &raw mut value) } != WindowsClangSuccess {
        return Err(unsupported(
            cursor,
            &format!(
                "{annotation} requires ordinary narrow string literals without embedded nulls"
            ),
        ));
    }
    let result = unsafe { std::ffi::CStr::from_ptr(clang_getCString(value)) }
        .to_str()
        .map(str::to_owned);
    unsafe { clang_disposeString(value) };
    result.map_err(|_| unsupported(cursor, &format!("{annotation} string is not UTF-8")))
}

fn entry_point(cursor: CXCursor) -> Result<String, Error> {
    let mut ty = unsafe { clang_getCanonicalType(clang_getCursorType(cursor)) };
    if ty.kind == CXType_Enum {
        ty = unsafe {
            clang_getCanonicalType(clang_getEnumDeclIntegerType(clang_getTypeDeclaration(ty)))
        };
    }
    if matches!(
        ty.kind,
        CXType_Char_S
            | CXType_SChar
            | CXType_Char_U
            | CXType_UChar
            | CXType_Char16
            | CXType_Char32
            | CXType_Short
            | CXType_UShort
            | CXType_WChar
            | CXType_Int
            | CXType_UInt
            | CXType_Long
            | CXType_ULong
            | CXType_LongLong
            | CXType_ULongLong
    ) {
        let result = unsafe { clang_Cursor_Evaluate(cursor) };
        if result.is_null() {
            return Err(unsupported(
                cursor,
                "ImportLibrary ordinal is not a constant integer",
            ));
        }
        let ordinal = unsafe {
            let ordinal = (clang_EvalResult_getKind(result) == CXEval_Int)
                .then(|| clang_EvalResult_getAsUnsigned(result));
            clang_EvalResult_dispose(result);
            ordinal
        };
        let Some(ordinal @ 1..=65535) = ordinal else {
            return Err(unsupported(
                cursor,
                "ImportLibrary ordinal must be between 1 and 65535",
            ));
        };
        return Ok(format!("#{ordinal}"));
    }
    let name = string(cursor, "ImportLibrary")?;
    if name.is_empty() || name.chars().any(char::is_control) {
        return Err(unsupported(cursor, "invalid ImportLibrary entry point"));
    }
    if let Some(ordinal) = name.strip_prefix('#')
        && (ordinal.is_empty()
            || !ordinal.bytes().all(|c| c.is_ascii_digit())
            || !ordinal.parse::<u16>().is_ok_and(|value| value != 0))
    {
        return Err(unsupported(
            cursor,
            "ImportLibrary ordinal must be between 1 and 65535",
        ));
    }
    Ok(name)
}
