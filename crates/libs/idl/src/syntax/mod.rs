mod parse;

pub struct File {
    pub items: Vec<Item>,
}

pub struct ItemEnum {
    pub ident: String,
}

pub struct ItemInterface {
    pub name: String,
    pub methods: Vec<Method>,
}

pub struct ItemStruct {
    pub ident: String,
}

pub enum Item {
    Enum(ItemEnum),
    Interface(ItemInterface),
    Struct(ItemStruct),
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
