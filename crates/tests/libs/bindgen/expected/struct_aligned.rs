#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverAligned {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmmFrame {
    pub Xmm0: M128A,
    pub Flags: u32,
}
