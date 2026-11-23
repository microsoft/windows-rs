use super::*;

#[derive(Debug)]
pub struct Implements {
    all: bool,
    rules: NameRules,
}

impl Implements {
    pub fn new(reader: &Reader, patterns: &[&str]) -> Self {
        let parsed: Vec<_> = patterns
            .iter()
            .flat_map(|pattern| filter_parser::parse_filter_entry(pattern))
            .collect();
        let resolved = filter_parser::resolve_entries(reader, &parsed);
        for entry in &resolved {
            assert!(
                !entry.exclude,
                "implementation selectors cannot exclude interfaces"
            );
            match &entry.kind {
                filter_parser::ResolvedKind::Namespace(_) => {}
                filter_parser::ResolvedKind::Type { namespace, name } => {
                    assert!(
                        reader
                            .with_full_name(namespace, name)
                            .any(|ty| matches!(ty, Type::Interface(_) | Type::CppInterface(_))),
                        "implementation selector `{namespace}.{name}` is not an interface"
                    );
                }
                _ => panic!("implementation selectors must name interfaces or namespaces"),
            }
        }
        Self {
            all: patterns.is_empty(),
            rules: NameRules::new(&resolved),
        }
    }

    pub fn matches(&self, name: TypeName) -> bool {
        self.all || self.rules.includes_type_name(name).is_some()
    }
}
