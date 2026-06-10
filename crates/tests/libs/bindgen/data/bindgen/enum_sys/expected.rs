#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2);
    pub const Completed: Self = Self(1);
    pub const Error: Self = Self(3);
    pub const Started: Self = Self(0);
}
