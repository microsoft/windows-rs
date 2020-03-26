use crate::*;

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<EnumField>,
}

#[derive(Debug)]
pub struct EnumField {
    pub name: String,
    pub value: EnumConstant,
}

#[derive(Debug)]
pub enum EnumConstant {
    U32(u32),
    I32(i32),
}

impl Enum {
    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut fields = Vec::new();

        for field in def.fields(reader) {
            for constant in field.constants(reader) {
                let name = field.name(reader).to_string();
                let mut value = constant.value(reader);

                let value = match constant.value_type(reader) {
                    0x08 => EnumConstant::I32(value.read_i32()),
                    0x09 => EnumConstant::U32(value.read_u32()),
                    _ => panic!("Enum::from_type_def"),
                };

                fields.push(EnumField { name, value });
            }
        }

        Self { name, fields }
    }

    pub fn to_stream(&self) -> TokenStream {
        panic!();
    }
    
    pub fn dependencies<F: Fn(&TypeName)>(&self, f: &F) {
    }


}
