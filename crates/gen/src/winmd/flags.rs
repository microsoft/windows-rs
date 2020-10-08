pub struct MethodFlags(pub u32);
pub struct TypeFlags(pub u32);

#[derive(Default)]
pub struct ParamFlags(pub u32);

#[derive(Default)]
pub struct FieldFlags(pub u32);

impl MethodFlags {
    pub fn special(&self) -> bool {
        self.0 & 0b1000_0000_0000 != 0
    }
}

impl TypeFlags {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100_0000_0000_0000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b10_0000 != 0
    }
}

impl ParamFlags {
    pub fn input(&self) -> bool {
        self.0 & 0b1 != 0
    }
}

impl FieldFlags {
    pub fn literal(&self) -> bool {
        self.0 & 0b100_0000 != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}

#[derive(PartialEq)]
pub enum TypeCategory {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}

#[allow(dead_code)]
pub enum ParamCategory {
    Array,
    Enum,
    Generic,
    Object,
    Primitive,
    String,
    Struct,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MethodCategory {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}
