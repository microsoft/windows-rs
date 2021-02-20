use super::*;

pub struct MethodGen {
    pub gen: Gen,
    pub name: &'static str,
    pub offset: u32,
    pub overload: u32,
    pub kind: InterfaceKind,
}
