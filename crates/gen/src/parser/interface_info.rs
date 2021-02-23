use super::*;

pub struct InterfaceInfo {
    pub interface: types::Interface,
    pub kind: InterfaceKind,
    pub is_base: bool,
    pub is_exclusive: bool,
    pub version: (u16, u16),
}
