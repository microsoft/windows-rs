use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TranslationUnit(pub(crate) *const std::ffi::c_void);

impl Free for TranslationUnit {
    fn free(&mut self) {
        link!("libclang.dll" "system" fn clang_disposeTranslationUnit(_: TranslationUnit));
        unsafe { clang_disposeTranslationUnit(*self) }
    }
}

impl std::fmt::Debug for TranslationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spelling())
    }
}

impl std::fmt::Display for TranslationUnit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.spelling())
    }
}

impl TranslationUnit {
    pub fn spelling(&self) -> Owned<CXString> {
        link!("libclang.dll" "system" fn clang_getTranslationUnitSpelling(_: TranslationUnit) -> CXString);
        unsafe { Owned::new(clang_getTranslationUnitSpelling(*self)) }
    }

    pub fn cursor(&self) -> Cursor {
        link!("libclang.dll" "system" fn clang_getTranslationUnitCursor(_: TranslationUnit) -> Cursor);
        unsafe { clang_getTranslationUnitCursor(*self) }
    }
}
