pub type Fixed = unsafe extern "system" fn(count: u32) -> u32;
unsafe extern "system" {
    pub fn Fixed(count: u32) -> u32;
}
pub type VariadicC = unsafe extern "C" fn(count: u32, ...) -> u32;
unsafe extern "C" {
    pub fn VariadicC(count: u32, ...) -> u32;
}
pub type VariadicSystem = unsafe extern "system" fn(count: u32, ...) -> u32;
unsafe extern "system" {
    pub fn VariadicSystem(count: u32, ...) -> u32;
}
