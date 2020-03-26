pub struct MethodFlags(pub u32);
pub struct TypeFlags(pub u32);

#[derive(Default)]
pub struct ParamFlags(pub u32);

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

pub enum TypeCategory {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}

pub enum ParamCategory {
    Array,
    Enum,
    Generic,
    Object,
    Primitive,
    String,
    Struct,
}

pub enum ElementType {
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Object,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MethodCategory {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}
