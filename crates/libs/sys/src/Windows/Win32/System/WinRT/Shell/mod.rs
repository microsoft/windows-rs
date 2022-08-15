pub type CreateProcessMethod = i32;
pub const CpCreateProcess: CreateProcessMethod = 0i32;
pub const CpCreateProcessAsUser: CreateProcessMethod = 1i32;
pub const CpAicLaunchAdminProcess: CreateProcessMethod = 2i32;
pub type IDDEInitializer = *mut ::core::ffi::c_void;
