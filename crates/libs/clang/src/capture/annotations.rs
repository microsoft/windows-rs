use super::*;

pub(super) fn read(cursor: CXCursor) -> Result<AttributeKind, Error> {
    let payload = text(unsafe { clang_getCursorSpelling(cursor) });
    Ok(
        match sal::transport::read(&payload).map_err(|error| unsupported(cursor, &error.0))? {
            Some(value) => AttributeKind::Sal(Box::new(value)),
            None => AttributeKind::Unresolved("Annotate".into()),
        },
    )
}
