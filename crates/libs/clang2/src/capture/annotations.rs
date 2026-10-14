use super::*;

pub(super) fn read(cursor: CXCursor) -> Result<AttributeKind, Error> {
    let payload = text(unsafe { clang_getCursorSpelling(cursor) });
    let Some(payload) = payload.strip_prefix("windows_clang:sal:") else {
        return Ok(AttributeKind::Unresolved("Annotate".into()));
    };
    let invalid = || unsupported(cursor, "invalid SAL annotation transport");
    let (version, payload) = payload.split_once(':').ok_or_else(invalid)?;
    if !matches!(version, "1" | "1.1" | "1.2" | "2" | "L") {
        return Err(invalid());
    }
    let parts: Vec<_> = payload.split('\n').collect();
    let [name, arguments, expanded] = parts.as_slice() else {
        return Err(invalid());
    };
    if name.is_empty()
        || !name.bytes().enumerate().all(|(index, byte)| {
            byte == b'_' || byte.is_ascii_alphabetic() || index != 0 && byte.is_ascii_digit()
        })
    {
        return Err(invalid());
    }
    let arguments = arguments
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(invalid)?;
    let expanded = expanded
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(invalid)?;
    Ok(AttributeKind::Sal(Box::new(SalAnnotation {
        version: version.into(),
        name: (*name).into(),
        arguments: arguments.into(),
        expanded: expanded.into(),
    })))
}
