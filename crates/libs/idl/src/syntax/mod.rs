mod parse;

pub struct File {
    pub items: Vec<Item>,
}

pub struct ItemEnum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

pub struct ItemInterface {
    pub name: String,
    pub methods: Vec<Method>,
}

pub struct ItemStruct {
    pub name: String,
}

pub enum Item {
    Enum(ItemEnum),
    Interface(ItemInterface),
    Struct(ItemStruct),
    Comment(String),
    Import(ItemImport),
}

pub struct Method {
    pub name: String,
    pub return_type: String,
    pub params: Vec<Param>,
}

pub struct Param {
    pub name: String,
    pub ty: String,
}

pub struct EnumVariant {
    pub name: String,
    pub value: Option<i64>,
}

pub struct ItemImport {
    pub name: String,
}
