#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};

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
        clang_getSpellingLocation(
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
    Error(format!("{file}:{line}:{column}: {reason}"))
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
    unsafe { clang_visitChildren(cursor, visit, (&raw mut result).cast()) };
    if let Some(panic) = result.panic {
        std::panic::resume_unwind(panic);
    }
    result.values
}
