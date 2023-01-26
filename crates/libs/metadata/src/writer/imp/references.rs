use super::*;

#[derive(Default)]
pub struct References<'a> {
    map: BTreeMap<(&'a str, &'a str), Reference>,
}

pub struct Reference {
    pub index: u32,
    pub value_type: bool,
}

pub struct StagedReferences<'a>(References<'a>);

impl<'a> References<'a> {
    pub fn insert(&mut self, namespace: &'a str, name: &'a str, assemblies: &reader::Reader) {
        self.map.entry((namespace, name)).or_insert_with(|| {
            let type_def = assemblies.get(reader::TypeName::new(namespace, name)).next().expect("Type not found");
            let value_type = matches!(assemblies.type_def_kind(type_def), reader::TypeKind::Struct | reader::TypeKind::Enum);
            Reference { value_type, index: 0 }
        });
    }

    pub fn stage(mut self) -> StagedReferences<'a> {
        for (index, value) in self.map.values_mut().enumerate() {
            value.index = index as _;
        }
        StagedReferences(self)
    }
}

impl<'a> StagedReferences<'a> {
    pub fn get(&'a self, namespace: &'a str, name: &'a str) -> Option<&'a Reference> {
        self.0.map.get(&(namespace, name))
    }
}
