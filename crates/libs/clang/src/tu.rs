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

impl TranslationUnit {
    pub fn cursor(&self) -> Cursor {
        link!("libclang.dll" "system" fn clang_getTranslationUnitCursor(_: TranslationUnit) -> Cursor);
        unsafe { clang_getTranslationUnitCursor(*self) }
    }
}
