use crate::*;

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub kind: TypeKind, // TODO: might have to be a full Type to ensure we can write out nested structs for ABI layout
}

impl Struct {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut fields = Vec::new();

        for field in def.fields(reader) {
            let name = field.name(reader).to_string();
            let kind = TypeKind::from_field(reader, field);
            fields.push(StructField { name, kind });
        }

        Self { name, fields }
    }

    pub fn to_stream(&self) -> TokenStream {
        panic!();
    }

    // pub fn add_dependencies(&self, reader: &Reader, map: &mut BTreeMap::<TypeDef, Type>) {
    //     // self.fields.iter().map(|f|&f.kind).for_each(|k|k.dependencies(f));
    // }
}
