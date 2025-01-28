#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CXVisitorResult(i32);

pub const CXVisit_Break: CXVisitorResult = CXVisitorResult(0);
pub const CXVisit_Continue: CXVisitorResult = CXVisitorResult(1);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CXChildVisitResult(i32);

pub const CXChildVisit_Break: CXChildVisitResult = CXChildVisitResult(0);
pub const CXChildVisit_Continue: CXChildVisitResult = CXChildVisitResult(1);
pub const CXChildVisit_Recurse: CXChildVisitResult = CXChildVisitResult(2);
