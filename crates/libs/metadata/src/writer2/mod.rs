mod imp;

// Generates an in-memory .winmd file.
pub fn write(name: &str, winrt: bool, items: &[Item], assemblies: &[&str]) -> Vec<u8> {
    imp::write(name, winrt, items, assemblies)
}

pub enum Item {
    Struct(Struct),
    Enum(Enum),
}

impl Item {
    pub(crate) fn type_name(&self) -> (&str, &str) {
        match self {
            Self::Struct(ty) => (ty.namespace.as_str(), ty.name.as_str()),
            Self::Enum(ty) => (ty.namespace.as_str(), ty.name.as_str()),
        }
    }

    pub(crate) fn is_value_type(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            Self::Enum(_) => true,
        }
    }
}

pub struct Struct {
    pub namespace: String,
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Enum {
    pub namespace: String,
    pub name: String,
    pub constants: Vec<Constant>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(name: &str, ty: Type) -> Self {
        Self { name: name.to_string(), ty }
    }
}

pub struct Constant {
    pub name: String,
    pub value: Value,
}

impl Constant {
    pub fn new(name: &str, value: Value) -> Self {
        Self { name: name.to_string(), value }
    }
}

pub enum Type {
    Void,
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
    ISize,
    USize,
    String,
    Named((String, String)),
}

impl Type {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self::Named((namespace.to_string(), name.to_string()))
    }
}

pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
}
