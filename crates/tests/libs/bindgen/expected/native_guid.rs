pub const ID: Identifier = Identifier {
    first: 4275878552,
    second: 43981,
    third: 34661,
    bytes: [128, 255, 0, 1, 2, 3, 4, 254],
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub first: u32,
    pub second: u16,
    pub third: u16,
    pub bytes: [u8; 8],
}
impl Default for Identifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KEY: Key = Key {
    format: Identifier {
        first: 4275878552,
        second: 43981,
        third: 34661,
        bytes: [128, 255, 0, 1, 2, 3, 4, 254],
    },
    property: 65543,
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Key {
    pub format: Identifier,
    pub property: u32,
}
