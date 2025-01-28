use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CXIndex(*const std::ffi::c_void);

impl Free for CXIndex {
    fn free(&mut self) {
        link!("libclang.dll" "system" fn clang_disposeIndex(_: CXIndex));
        unsafe { clang_disposeIndex(*self) };
    }
}

impl CXIndex {
    pub fn new() -> Owned<Self> {
        link!("libclang.dll" "system" fn clang_createIndex(_: i32, _: i32) -> CXIndex);
        unsafe { Owned::new(clang_createIndex(0, 0)) }
    }

    pub fn tu(&self, filename: &std::ffi::CStr, options: u32) -> Option<Owned<CXTranslationUnit>> {
        link!("libclang.dll" "system" fn clang_parseTranslationUnit(_: CXIndex, _: *const i8, _: *const *const u8, _: i32, _: *const std::ffi::c_void, _: u32, _: u32) -> CXTranslationUnit);

        let tu = unsafe {
            Owned::new(clang_parseTranslationUnit(
                *self,
                filename.as_ptr(),
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                options,
            ))
        };

        (!tu.0.is_null()).then_some(tu)
    }
}
