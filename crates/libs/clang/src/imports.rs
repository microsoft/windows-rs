use super::*;

extern "C" {
    pub fn strlen(s: *const u8) -> usize;
}

link!("libclang.dll" "system" fn clang_getClangVersion() -> CXString);
link!("libclang.dll" "system" fn clang_disposeString(_: CXString));
link!("libclang.dll" "system" fn clang_getCString(_: CXString) -> *const u8);
