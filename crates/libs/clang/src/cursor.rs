use super::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cursor {
    kind: CursorKind,
    xdata: i32,
    data: [*const std::ffi::c_void; 3],
}

impl std::fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", (self.kind, self.name()))
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        link!("libclang.dll" "system" fn clang_equalCursors(_: Cursor, _: Cursor) -> u32);
        unsafe { clang_equalCursors(*self, *other) != 0 }
    }
}

impl Cursor {
    pub fn name(&self) -> Owned<CXString> {
        link!("libclang.dll" "system" fn clang_getCursorDisplayName(_: Cursor) -> CXString);
        unsafe { Owned::new(clang_getCursorDisplayName(*self)) }
    }

    pub fn kind(&self) -> CursorKind {
        self.kind
    }

    pub fn ty(&self) -> Type {
        link!("libclang.dll" "system" fn clang_getCursorType(_: Cursor) -> Type);
        unsafe { clang_getCursorType(*self) }
    }

    pub fn find<P>(&self, mut predicate: P) -> Option<Cursor>
    where
        P: FnMut(Self) -> bool,
    {
        let mut result = None;

        self.visit(|next, _| {
            if predicate(next) {
                result = Some(next);
                VisitResult::Break
            } else {
                VisitResult::Continue
            }
        });

        result
    }

    pub fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(Self, Self) -> VisitResult,
    {
        type CXCursorVisitor = extern "system" fn(
            child: Cursor,
            parent: Cursor,
            data: *const std::ffi::c_void,
        ) -> VisitResult;

        link!("libclang.dll" "system" fn clang_visitChildren(_: Cursor, _: CXCursorVisitor, _: *const std::ffi::c_void) -> u32);

        extern "system" fn callback<Visitor>(
            child: Cursor,
            parent: Cursor,
            data: *const std::ffi::c_void,
        ) -> VisitResult
        where
            Visitor: FnMut(Cursor, Cursor) -> VisitResult,
        {
            let callback: &mut Visitor = unsafe { &mut *(data as *mut _) };
            let result = (*callback)(child, parent);

            debug_assert!(matches!(
                result,
                VisitResult::Break | VisitResult::Continue | VisitResult::Recurse
            ));

            result
        }

        unsafe {
            clang_visitChildren(*self, callback::<Visitor>, &mut visitor as *mut _ as _);
        }
    }
}
