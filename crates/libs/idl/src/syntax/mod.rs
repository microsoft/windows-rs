mod parse;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Enum {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub struct Interface {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub implements: Vec<InterfaceImpl>,
    pub methods: Vec<Method>,
}

#[derive(Debug)]
pub struct InterfaceImpl {
    pub attributes: Vec<Attribute>,
    pub name: String,
}

#[derive(Debug)]
pub struct StructField {
    pub attributes: Vec<Attribute>,
    pub field_type: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Struct {
pub attributes: Vec<Attribute>,
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub enum Item {
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),
    Import(Import),
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub params: Vec<Param>,
}

#[derive(Debug)]
pub struct Param {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<i64>,
}

#[derive(Debug)]
pub struct Import {
    pub name: String,
}

// #[derive(Debug)]
// pub struct Library {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
// }

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub parameters: Vec<(String, String)>,
}
