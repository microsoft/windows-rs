pub type PCWSTR = *const u16;
pub mod Test {
    pub const GREETING: super::PCWSTR = [104, 101, 108, 108, 111, 0].as_ptr();
}
