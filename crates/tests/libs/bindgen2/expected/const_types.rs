pub const A_U8: u8 = 255;
pub const B_I8: i8 = -1;
pub const C_U16: u16 = 1000;
pub const D_I16: i16 = -500;
pub const E_U32: u32 = 100000;
pub const F_I32: i32 = -100000;
pub const G_U64: u64 = 1000000;
pub const H_I64: i64 = -1000000;
pub const I_TYPED: MyI32 = MyI32(0x2A_u32 as _);
pub const J_TYPED: MyU64 = MyU64(999);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MyI32(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MyU64(pub u64);
