use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXType {
    kind: CXTypeKind,
    data: [*const std::ffi::c_void; 2],
}

impl CXType {
    pub fn kind(&self) -> CXTypeKind {
        self.kind
    }

    pub fn is_const(&self) -> bool {
        link!("libclang.dll" "system" fn clang_isConstQualifiedType(_: CXType) -> u32);
        unsafe { clang_isConstQualifiedType(*self) != 0 }
    }

    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(CXCursor) -> CXVisitorResult,
    {
        type CXFieldVisitor =
            extern "system" fn(cursor: CXCursor, data: *const std::ffi::c_void) -> CXVisitorResult;

        link!("libclang.dll" "system" fn clang_Type_visitFields(_: CXType, _: CXFieldVisitor, _: *const std::ffi::c_void) -> u32);

        extern "system" fn callback<Visitor>(
            cursor: CXCursor,
            data: *const std::ffi::c_void,
        ) -> CXVisitorResult
        where
            Visitor: FnMut(CXCursor) -> CXVisitorResult,
        {
            let callback: &mut Visitor = unsafe { &mut *(data as *mut _) };
            (*callback)(cursor)
        }

        unsafe {
            clang_Type_visitFields(*self, callback::<Visitor>, &mut visitor as *mut _ as _);
        }
    }
}
