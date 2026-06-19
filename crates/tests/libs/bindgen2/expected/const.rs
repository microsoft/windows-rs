pub const CONSTANT: u32 = 260;
pub const OK: TestHRESULT = TestHRESULT(0x0_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TestHRESULT(pub i32);
