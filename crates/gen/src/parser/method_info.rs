pub struct MethodInfo {
    pub name: String,
    pub vtable_offset: u32,
    pub overload: u32,
    pub is_deprecated: bool,
}
