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

    pub fn find<P>(&self, mut predicate: P) -> Option<Cursor>
    where
        P: FnMut(Cursor) -> bool,
    {
        let mut result = None;

        self.visit(|next| {
            if predicate(next) {
                result = Some(next);
                Visit::Break
            } else {
                Visit::Continue
            }
        });

        result
    }

    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(Cursor) -> Visit,
    {
        type CXFieldVisitor =
            extern "system" fn(field: Cursor, data: *const std::ffi::c_void) -> Visit;

        link!("libclang.dll" "system" fn clang_Type_visitFields(_: Type, _: CXFieldVisitor, _: *const std::ffi::c_void) -> u32);

        extern "system" fn callback<Visitor>(field: Cursor, data: *const std::ffi::c_void) -> Visit
        where
            Visitor: FnMut(Cursor) -> Visit,
        {
            let callback: &mut Visitor = unsafe { &mut *(data as *mut _) };
            let result = (*callback)(field);
            debug_assert!(matches!(result, Visit::Break | Visit::Continue));
            result
        }

        unsafe {
            clang_Type_visitFields(*self, callback::<Visitor>, &mut visitor as *mut _ as _);
        }
    }
}
