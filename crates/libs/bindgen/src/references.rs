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
    pub name: String, // crate name like "windows_future"
    pub rules: NameRules,
}

#[derive(Debug, Default)]
pub struct References {
    entries: Vec<Reference>,
    provided: HashMap<TypeName, usize>,
}

impl References {
    pub fn new(reader: &Reader, stage: Vec<ReferenceStage>) -> Self {
        let entries: Vec<_> = stage
            .into_iter()
            .map(|stage| {
                let entries = filter_parser::parse_filter_entry(&stage.path);
                let resolved = filter_parser::resolve_entries(reader, &entries);
                let rules = NameRules::new(&resolved);

                Reference {
                    name: stage.name,
                    rules,
                }
            })
            .collect();
        let mut provided = HashMap::new();
        for namespace in reader.namespaces() {
            let candidates: Vec<_> = entries
                .iter()
                .enumerate()
                .filter(|(_, reference)| reference.rules.applies_to_namespace(namespace))
                .collect();
            if candidates.is_empty() {
                continue;
            }
            for name in reader.type_names(namespace) {
                if let Some((index, _)) = candidates
                    .iter()
                    .find(|(_, reference)| reference.rules.includes(name).is_some())
                {
                    provided.insert(name, *index);
                }
            }
        }
        Self { entries, provided }
    }

    pub fn contains(&self, name: TypeName) -> Option<&Reference> {
        self.provided.get(&name).map(|index| &self.entries[*index])
    }

    pub fn matching_rule(&self, name: TypeName) -> Option<&str> {
        self.contains(name)
            .and_then(|reference| reference.rules.includes(name))
    }

    pub fn into_external(self, types: &TypeMap) -> HashMap<TypeName, String> {
        self.provided
            .into_iter()
            .filter(|(name, _)| !types.contains_key(name))
            .map(|(name, index)| (name, self.entries[index].name.clone()))
            .collect()
    }
}
