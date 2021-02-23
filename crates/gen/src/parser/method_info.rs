use super::*;

pub struct MethodInfo {
    pub name: &'static str,
    pub vtable_offset: u32,
    pub overload: u32,
}
