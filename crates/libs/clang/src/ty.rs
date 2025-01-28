use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Type {
    kind: TypeKind,
    data: [*const std::ffi::c_void; 2],
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        self.kind
    }

    pub fn is_const(&self) -> bool {
        link!("libclang.dll" "system" fn clang_isConstQualifiedType(_: Type) -> u32);
        unsafe { clang_isConstQualifiedType(*self) != 0 }
    }

    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(Cursor) -> VisitResult,
    {
        type CXFieldVisitor =
            extern "system" fn(cursor: Cursor, data: *const std::ffi::c_void) -> VisitResult;

        link!("libclang.dll" "system" fn clang_Type_visitFields(_: Type, _: CXFieldVisitor, _: *const std::ffi::c_void) -> u32);

        extern "system" fn callback<Visitor>(
            cursor: Cursor,
            data: *const std::ffi::c_void,
        ) -> VisitResult
        where
            Visitor: FnMut(Cursor) -> VisitResult,
        {
            let callback: &mut Visitor = unsafe { &mut *(data as *mut _) };
            let result = (*callback)(cursor);
            debug_assert!(matches!(result, VisitResult::Break | VisitResult::Continue));
            result
        }

        unsafe {
            clang_Type_visitFields(*self, callback::<Visitor>, &mut visitor as *mut _ as _);
        }
    }
}
