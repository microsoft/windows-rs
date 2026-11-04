use super::*;

pub(super) fn read(cursor: CXCursor, owner: CXCursor) -> Result<AttributeKind, Error> {
    let payload = text(unsafe { clang_getCursorSpelling(cursor) });
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
        let library = string(arguments[0])?;
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

fn string(cursor: CXCursor) -> Result<String, Error> {
    let mut value = CXString::default();
    if unsafe { windows_clang_getStringLiteral(cursor, &raw mut value) } != WindowsClangSuccess {
        return Err(unsupported(
            cursor,
            "ImportLibrary requires ordinary narrow string literals without embedded nulls",
        ));
    }
    let result = unsafe { std::ffi::CStr::from_ptr(clang_getCString(value)) }
        .to_str()
        .map(str::to_owned);
    unsafe { clang_disposeString(value) };
    result.map_err(|_| unsupported(cursor, "ImportLibrary string is not UTF-8"))
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
    let name = string(cursor)?;
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
