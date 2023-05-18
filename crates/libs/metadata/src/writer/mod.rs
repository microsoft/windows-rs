mod extend;
mod idl;
mod validate;
mod winmd;

// TODO: this "writer" is really both a reader and a writer that is more generalized than "reader". The "reader" should be the raw or low-level reader
// that is secondary while this should be the main metadata API.

use super::*;
pub use idl::format_idl;

impl Module {
    pub(crate) fn new(namespace: &str) -> Self {
        Self { namespace: namespace.to_string(), ..Default::default() }
    }

    pub(crate) fn insert(&mut self, namespace: &str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.modules.entry(namespace[pos..next].to_string()).or_insert_with(|| Self::new(&namespace[..next])).insert(namespace, next + 1)
        } else {
            self.modules.entry(namespace[pos..].to_string()).or_insert_with(|| Self::new(namespace))
        }
    }

    pub fn read(input: &[String], filter: &Filter) -> Result<Self> {
        let mut module = Module::default();
        winmd::read_winmd(&mut module, input, filter)?;
        idl::read_idl(&mut module, input, filter)?;
        Ok(module)
    }

    pub fn extend(&mut self, other: Module) {
        extend::extend(self, other)
    }

    pub fn validate(&self) -> Result<()> {
        validate::validate(self)
    }

    pub fn write(&self, path: &str) -> Result<()> {
        match extension(path) {
            (_, "idl") => idl::write_idl(self, path),
            (_, "winmd") => winmd::write_winmd(self, path),
            _ => Err(Error::new("output extension must be either .winmd or .idl")),
        }
    }

    pub fn contains_type(&self, namespace: &str, name: &str) -> bool {
        if let Some(next) = namespace.find('.') {
            if let Some(module) = self.modules.get(&namespace[..next]) {
                module.contains_type(&namespace[next + 1..], name)
            } else {
                false
            }
        } else if let Some(module) = self.modules.get(namespace) {
            module.types.contains_key(name)
        } else {
            false
        }
    }

    // TODO: add a `write_rust` method and get rid of the `bindgen` crate?
}

#[derive(Default, Debug)]
pub struct Module {
    pub namespace: String, // TODO: call this name for clarity
    pub modules: BTreeMap<String, Module>,

    // Note that a `Vec` is used since Win32 structs may have per-architecture definitions
    pub types: BTreeMap<String, Vec<TypeDef>>,
}

#[derive(Default, Debug)]
pub struct TypeDef {
    pub flags: TypeAttributes,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
    pub extends: Option<TypeRef>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub interfaces: Vec<TypeRef>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeRef {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
    // store an optional `assembly` for the name of the winmd/idl file where the type originates
}

#[derive(Debug, Clone)]
pub(crate) enum TypeRefIndex {
    None,
    TypeDef(usize),
    TypeRef(usize),
}

impl Default for TypeRefIndex {
    fn default() -> Self {
        Self::None
    }
}

// TODO: why not use 'syn' types directly?

#[derive(Debug, Clone, PartialEq)]
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
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    PSTR,
    PWSTR,
    PCSTR,
    PCWSTR,
    BSTR,
    TypeName,
    GenericParam(String), // TODO: need this in the writer?
    TypeRef(TypeRef),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    ConstRef(Box<Self>),
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}

impl Type {
    fn into_mut_ptr(self) -> Self {
        match self {
            Self::MutPtr((ty, count)) => Self::MutPtr((ty, count + 1)),
            Self::ConstPtr((ty, count)) => Self::MutPtr((ty, count + 1)),
            _ => Self::MutPtr((Box::new(self), 1)),
        }
    }

    fn into_const_ptr(self) -> Self {
        match self {
            Self::MutPtr((ty, count)) => Self::ConstPtr((ty, count + 1)),
            Self::ConstPtr((ty, count)) => Self::ConstPtr((ty, count + 1)),
            _ => Self::ConstPtr((Box::new(self), 1)),
        }
    }

    fn into_array(self, len: usize) -> Self {
        Self::Win32Array((Box::new(self), len))
    }
}

#[derive(Debug)]
pub struct Attribute {}

#[derive(Default, Debug)]
pub struct Field {
    flags: FieldAttributes,
    name: String,
    ty: Type,
    value: Option<Value>,
}

#[derive(Default, Debug)]
pub struct Param {
    flags: ParamAttributes,
    name: String,
    ty: Type,
}

#[derive(Default, Debug)]
pub struct Method {
    flags: MethodAttributes,
    name: String,
    params: Vec<Param>,
    return_type: Param,
    //attributes: Vec<Attribute>,
    //impl_flags: MethodImplAttributes,
    vararg: bool,
}

#[derive(Debug)]
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

impl Value {
    // TODO: should return u8
    fn to_code(&self) -> u16 {
        match self {
            Self::Bool(_) => ELEMENT_TYPE_BOOLEAN as _,
            Self::I8(_) => ELEMENT_TYPE_I1 as _,
            Self::U8(_) => ELEMENT_TYPE_U1 as _,
            Self::I16(_) => ELEMENT_TYPE_I2 as _,
            Self::U16(_) => ELEMENT_TYPE_U2 as _,
            Self::I32(_) => ELEMENT_TYPE_I4 as _,
            Self::U32(_) => ELEMENT_TYPE_U4 as _,
            Self::I64(_) => ELEMENT_TYPE_I8 as _,
            Self::U64(_) => ELEMENT_TYPE_U8 as _,
            Self::F32(_) => ELEMENT_TYPE_R4 as _,
            Self::F64(_) => ELEMENT_TYPE_R8 as _,
            Self::String(_) => ELEMENT_TYPE_STRING as _,
        }
    }
    fn to_expr(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::U8(value) => format!("{value}u8"),
            Self::I8(value) => format!("{value}i8"),
            Self::U16(value) => format!("{value}u16"),
            Self::I16(value) => format!("{value}i16"),
            Self::U32(value) => format!("{value}u32"),
            Self::I32(value) => format!("{value}i32"),
            Self::U64(value) => format!("{value}u64"),
            Self::I64(value) => format!("{value}i64"),
            Self::F32(value) => format!("{value}f32"),
            Self::F64(value) => format!("{value}f64"),
            Self::String(value) => value.clone(),
        }
    }
    fn neg(&self) -> Self {
        use std::ops::Neg;
        match self {
            Self::Bool(value) => Self::Bool(!value),
            Self::U8(value) => Self::U8(*value),
            Self::I8(value) => Self::I8(value.neg()),
            Self::U16(value) => Self::U16(*value),
            Self::I16(value) => Self::I16(value.neg()),
            Self::U32(value) => Self::U32(*value),
            Self::I32(value) => Self::I32(value.neg()),
            Self::U64(value) => Self::U64(*value),
            Self::I64(value) => Self::I64(value.neg()),
            Self::F32(value) => Self::F32(value.neg()),
            Self::F64(value) => Self::F64(value.neg()),
            Self::String(value) => Self::String(value.clone()),
        }
    }
}
