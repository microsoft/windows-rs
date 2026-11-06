#![allow(non_upper_case_globals)]

use super::*;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use windows_clang_sys::*;

#[derive(Clone, Copy)]
pub(super) struct Cursor(pub CXCursor);

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        unsafe { clang_equalCursors(self.0, other.0) != 0 }
    }
}

impl Eq for Cursor {}

impl Hash for Cursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clang_hashCursor(self.0) }.hash(state);
    }
}

pub(super) fn layout(ty: CXType) -> Result<Layout, &'static str> {
    Ok(Layout {
        size: unsafe { clang_Type_getSizeOf(ty) }
            .try_into()
            .map_err(|_| "size unavailable")?,
        alignment: unsafe { clang_Type_getAlignOf(ty) }
            .try_into()
            .map_err(|_| "alignment unavailable")?,
    })
}

pub(super) fn source(cursor: CXCursor) -> (String, u32, u32, u32) {
    source_location(unsafe { clang_getCursorLocation(cursor) })
}

pub(super) fn source_location(location: CXSourceLocation) -> (String, u32, u32, u32) {
    let mut file = std::ptr::null_mut();
    let (mut line, mut column, mut offset) = (0, 0, 0);
    unsafe {
        clang_getExpansionLocation(
            location,
            &raw mut file,
            &raw mut line,
            &raw mut column,
            &raw mut offset,
        );
        (
            if file.is_null() {
                String::new()
            } else {
                text(clang_getFileName(file))
            },
            line,
            column,
            offset,
        )
    }
}

pub(super) fn unsupported(cursor: CXCursor, reason: &str) -> Error {
    let (file, line, column, _) = source(cursor);
    let kind = text(unsafe { clang_getCursorKindSpelling(clang_getCursorKind(cursor)) });
    let name = text(unsafe { clang_getCursorSpelling(cursor) });
    Error(format!("{file}:{line}:{column}: {kind} `{name}`: {reason}"))
}

/// Fails with `reason` and the status unless a companion query succeeded.
pub(super) fn check(
    status: WindowsClangStatus,
    cursor: CXCursor,
    reason: &str,
) -> Result<(), Error> {
    if status == WindowsClangSuccess {
        Ok(())
    } else {
        Err(unsupported(cursor, &format!("{reason}: {status}")))
    }
}

pub(super) fn field_initializer(cursor: CXCursor) -> Result<bool, Error> {
    let mut value = 0;
    check(
        unsafe { windows_clang_getFieldInitializer(cursor, &raw mut value) },
        cursor,
        "field initializer query failed",
    )?;
    Ok(value != 0)
}

pub(super) fn typedef_implicit(cursor: CXCursor) -> Result<bool, Error> {
    let mut value = 0;
    check(
        unsafe { windows_clang_getTypedefImplicit(cursor, &raw mut value) },
        cursor,
        "implicit typedef query failed",
    )?;
    Ok(value != 0)
}

pub(super) fn layout_expression(
    declaration: CXCursor,
    expression: CXCursor,
) -> Result<bool, Error> {
    let mut value = 0;
    check(
        unsafe { windows_clang_getLayoutExpression(declaration, expression, &raw mut value) },
        expression,
        "layout expression query failed",
    )?;
    Ok(value != 0)
}

/// Member and parameter names have no linkage, so object-like identifier aliases such as the
/// SDK's `DrawText -> DrawTextA` do not change their identity; the written name is retained.
pub(super) fn member_name(cursor: CXCursor) -> Result<String, Error> {
    let mut written = CXString::default();
    check(
        unsafe { windows_clang_getWrittenName(cursor, &raw mut written) },
        cursor,
        "written name query failed",
    )?;
    let written = text(written);
    if written.is_empty() {
        Ok(text(unsafe { clang_getCursorSpelling(cursor) }))
    } else {
        Ok(written)
    }
}

pub(super) fn text(value: CXString) -> String {
    unsafe {
        let pointer = clang_getCString(value);
        let result = if pointer.is_null() {
            String::new()
        } else {
            CStr::from_ptr(pointer).to_string_lossy().into_owned()
        };
        clang_disposeString(value);
        result
    }
}

pub(super) fn record_fields(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(cursor: CXCursor, data: CXClientData) -> CXVisitorResult {
        unsafe { &mut *data.cast::<Vec<CXCursor>>() }.push(cursor);
        CXVisit_Continue
    }
    let mut result = Vec::new();
    unsafe {
        clang_Type_visitFields(
            clang_getCursorType(cursor),
            Some(visit),
            (&raw mut result).cast(),
        );
    }
    result
}

pub(super) fn contains_record(ty: CXType, record: Cursor) -> bool {
    unsafe {
        let ty = clang_getCanonicalType(ty);
        match ty.kind {
            CXType_Pointer => contains_record(clang_getPointeeType(ty), record),
            CXType_ConstantArray | CXType_IncompleteArray => {
                contains_record(clang_getArrayElementType(ty), record)
            }
            CXType_FunctionProto => {
                contains_record(clang_getResultType(ty), record)
                    || (0..clang_getNumArgTypes(ty))
                        .any(|index| contains_record(clang_getArgType(ty, index as u32), record))
            }
            CXType_Record | CXType_Enum => {
                Cursor(clang_getCanonicalCursor(clang_getTypeDeclaration(ty))) == record
            }
            _ => false,
        }
    }
}
pub(super) fn owner_reference(cursor: CXCursor, reference: CXCursor) -> bool {
    unsafe {
        let referenced = Cursor(clang_getCanonicalCursor(clang_getCursorReferenced(
            reference,
        )));
        let mut parent = clang_getCursorSemanticParent(cursor);
        while clang_Cursor_isNull(parent) == 0 {
            if Cursor(clang_getCanonicalCursor(parent)) == referenced {
                return true;
            }
            parent = clang_getCursorSemanticParent(parent);
        }
    }
    false
}

pub(super) fn children(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        unsafe { &mut *data.cast::<Vec<CXCursor>>() }.push(cursor);
        CXChildVisit_Continue
    }
    let mut result = Vec::new();
    unsafe { clang_visitChildren(cursor, Some(visit), (&raw mut result).cast()) };
    result
}
