use super::*;

pub struct MethodGen {
    pub gen: Gen,
    pub name: &'static str,
    pub vtable_offset: u32,
    pub overload: u32,
    pub interface: GenericType,
    pub kind: InterfaceKind,
}
