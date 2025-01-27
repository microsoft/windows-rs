use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CXTranslationUnit(pub(crate) *const std::ffi::c_void);

impl Free for CXTranslationUnit {
    fn free(&mut self) {
        link!("libclang.dll" "system" fn clang_disposeTranslationUnit(_: CXTranslationUnit));
        unsafe { clang_disposeTranslationUnit(*self) }
    }
}

impl CXTranslationUnit {
    pub fn cursor(&self) -> CXCursor {
        link!("libclang.dll" "system" fn clang_getTranslationUnitCursor(_: CXTranslationUnit) -> CXCursor);
        unsafe { clang_getTranslationUnitCursor(*self) }
    }
}
