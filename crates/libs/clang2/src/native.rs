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
    let mut file = std::ptr::null_mut();
    let (mut line, mut column, mut offset) = (0, 0, 0);
    unsafe {
        clang_getExpansionLocation(
            clang_getCursorLocation(cursor),
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

pub(super) fn field_initializer(cursor: CXCursor) -> Result<bool, Error> {
    let mut value = 0;
    if unsafe { windows_clang_getFieldInitializer(cursor, &raw mut value) } != WindowsClangSuccess {
        return Err(unsupported(cursor, "field initializer query failed"));
    }
    Ok(value != 0)
}

pub(super) fn layout_expression(
    declaration: CXCursor,
    expression: CXCursor,
) -> Result<bool, Error> {
    let mut value = 0;
    if unsafe { windows_clang_getLayoutExpression(declaration, expression, &raw mut value) }
        != WindowsClangSuccess
    {
        return Err(unsupported(expression, "layout expression query failed"));
    }
    Ok(value != 0)
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

pub(super) fn tokens(cursor: CXCursor) -> Vec<String> {
    unsafe {
        let unit = clang_Cursor_getTranslationUnit(cursor);
        let mut tokens = std::ptr::null_mut();
        let mut count = 0;
        clang_tokenize(
            unit,
            clang_getCursorExtent(cursor),
            &raw mut tokens,
            &raw mut count,
        );
        let result = (0..count)
            .map(|index| text(clang_getTokenSpelling(unit, *tokens.add(index as usize))))
            .collect();
        clang_disposeTokens(unit, tokens, count);
        result
    }
}

pub(super) fn record_fields(cursor: CXCursor) -> Vec<CXCursor> {
    #[derive(Default)]
    struct Fields {
        values: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    extern "C" fn visit(cursor: CXCursor, data: CXClientData) -> CXVisitorResult {
        let fields = unsafe { &mut *data.cast::<Fields>() };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fields.values.push(cursor)))
        {
            Ok(()) => CXVisit_Continue,
            Err(panic) => {
                fields.panic = Some(panic);
                CXVisit_Break
            }
        }
    }
    let mut result = Fields::default();
    unsafe {
        clang_Type_visitFields(
            clang_getCursorType(cursor),
            Some(visit),
            (&raw mut result).cast(),
        );
    }
    if let Some(panic) = result.panic {
        std::panic::resume_unwind(panic);
    }
    result.values
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
    #[derive(Default)]
    struct Children {
        values: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        let children = unsafe { &mut *data.cast::<Children>() };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            children.values.push(cursor);
        })) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                children.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }
    let mut result = Children::default();
    unsafe { clang_visitChildren(cursor, Some(visit), (&raw mut result).cast()) };
    if let Some(panic) = result.panic {
        std::panic::resume_unwind(panic);
    }
    result.values
}
