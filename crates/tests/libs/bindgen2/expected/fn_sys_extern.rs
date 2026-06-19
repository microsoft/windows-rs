pub type Function = unsafe extern "system" fn() -> u32;
unsafe extern "system" {
    pub fn Function() -> u32;
}
