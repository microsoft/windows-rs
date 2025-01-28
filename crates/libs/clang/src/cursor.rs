use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXCursor {
    kind: CXCursorKind,
    xdata: i32,
    data: [*const std::ffi::c_void; 3],
}

impl std::fmt::Debug for CXCursor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", (self.kind, self.name()))
    }
}

impl CXCursor {
    pub fn name(&self) -> Owned<CXString> {
        link!("libclang.dll" "system" fn clang_getCursorDisplayName(_: CXCursor) -> CXString);
        unsafe { Owned::new(clang_getCursorDisplayName(*self)) }
    }

    pub fn kind(&self) -> CXCursorKind {
        self.kind
    }

    pub fn ty(&self) -> CXType {
        link!("libclang.dll" "system" fn clang_getCursorType(_: CXCursor) -> CXType);
        unsafe { clang_getCursorType(*self) }
    }

    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(CXCursor) -> CXChildVisitResult,
    {
        type CXCursorVisitor = extern "system" fn(
            cursor: CXCursor,
            parent: CXCursor,
            data: *const std::ffi::c_void,
        ) -> CXChildVisitResult;

        link!("libclang.dll" "system" fn clang_visitChildren(_: CXCursor, _: CXCursorVisitor, _: *const std::ffi::c_void) -> u32);

        extern "system" fn callback<Visitor>(
            child: CXCursor,
            _: CXCursor,
            data: *const std::ffi::c_void,
        ) -> CXChildVisitResult
        where
            Visitor: FnMut(CXCursor) -> CXChildVisitResult,
        {
            let callback: &mut Visitor = unsafe { &mut *(data as *mut _) };
            (*callback)(child)
        }

        unsafe {
            clang_visitChildren(*self, callback::<Visitor>, &mut visitor as *mut _ as _);
        }
    }
}
