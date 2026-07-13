pub type EXCEPTION_DISPOSITION = i32;
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = 3;
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = 0;
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = 1;
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = 2;
