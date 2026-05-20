pub type GetTickCount = unsafe extern "system" fn() -> u32;
unsafe extern "system" {
    pub fn GetTickCount() -> u32;
}
