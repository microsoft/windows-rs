pub mod Test {
    pub const One: Status = Status(1i32);
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Status(pub i32);
    pub const Three: Status = Status(3i32);
    pub const Two: Status = Status(2i32);
}
