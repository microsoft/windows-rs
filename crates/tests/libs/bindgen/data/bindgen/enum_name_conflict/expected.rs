pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct NameConflict(pub i32);
    pub const NameConflict_: NameConflict = NameConflict(123i32);
}
