use super::*;

pub struct ReferenceStage {
    name: String,
    path: String,
}

impl ReferenceStage {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub name: String,   // crate name like "windows_future"
    pub filter: Filter, // what this reference provides
}

#[derive(Debug, Default)]
pub struct References(Vec<Reference>);

impl References {
    pub fn new(reader: &Reader, stage: Vec<ReferenceStage>) -> Self {
        Self(
            stage
                .into_iter()
                .map(|stage| {
                    let entries = filter_parser::parse_filter_entry(&stage.path);
                    let resolved = filter_parser::resolve_entries(reader, &entries);
                    let filter = Filter::from_resolved(reader, &resolved);

                    Reference {
                        name: stage.name,
                        filter,
                    }
                })
                .collect(),
        )
    }

    pub fn contains(&self, name: TypeName) -> Option<&Reference> {
        self.0
            .iter()
            .find(|reference| reference.filter.includes_type_name(name).is_some())
    }

    pub fn matching_rule(&self, name: TypeName) -> Option<&str> {
        self.0
            .iter()
            .find_map(|reference| reference.filter.includes_type_name(name))
    }
}
