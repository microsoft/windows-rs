use super::*;

pub struct InterfaceInfo {
    pub def: GenericType,
    pub kind: InterfaceKind,
    pub is_base: bool,
    pub version: (u16, u16),
}
