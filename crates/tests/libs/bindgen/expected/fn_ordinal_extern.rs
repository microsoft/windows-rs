pub type Ordinal = unsafe extern "system" fn(value: i32) -> i32;
unsafe extern "system" {
    pub fn Ordinal(value: i32) -> i32;
}
