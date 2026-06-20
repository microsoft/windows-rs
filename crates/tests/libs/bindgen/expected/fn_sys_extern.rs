pub type ExternFunction = unsafe extern "system" fn() -> u32;
unsafe extern "system" {
    pub fn ExternFunction() -> u32;
}
