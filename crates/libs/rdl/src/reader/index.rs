use super::*;

#[derive(Default)]
pub struct Index<'a> {
    pub namespaces: BTreeMap<String, Namespace<'a>>,
}

#[derive(Default)]
pub struct Namespace<'a> {
    pub types: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
    nested: BTreeMap<String, bool>,
    pub functions: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
    pub constants: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
}

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, file: &'a File, namespace: &str, item: &'a Item) {
        if let Item::Module(module) = item {
            let name = module.to_string();
            for item in &module.items {
                let namespace = if namespace.is_empty() {
                    name.clone()
                } else {
                    format!("{namespace}.{name}")
                };
                self.insert(file, &namespace, item);
            }
        } else {
            match item {
                Item::Struct(record) => self.insert_nested(
                    namespace,
                    &record.name.unraw_to_string(),
                    &record.fields,
                    &record.records,
                    record.winrt,
                ),
                Item::Union(record) => self.insert_nested(
                    namespace,
                    &record.name.unraw_to_string(),
                    &record.fields,
                    &record.records,
                    false,
                ),
                _ => {}
            }
            let namespace = self.namespaces.entry(namespace.to_string()).or_default();

            match item {
                Item::Fn(..) => {
                    namespace
                        .functions
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
                Item::Const(..) => {
                    namespace
                        .constants
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
                _ => {
                    namespace
                        .types
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
            }
        }
    }

    fn insert_nested(
        &mut self,
        namespace: &str,
        parent: &str,
        fields: &[Field],
        records: &[NestedRecord],
        winrt: bool,
    ) {
        let inline = fields.iter().enumerate().filter_map(|(position, field)| {
            if let FieldType::Nested(record) = &field.ty {
                let name = parent.rsplit('/').next().unwrap();
                Some((format!("{name}_{position}"), record.as_ref()))
            } else {
                None
            }
        });
        for (name, record) in records
            .iter()
            .map(|record| (record.name.as_ref().unwrap().unraw_to_string(), record))
            .chain(inline)
        {
            let name = format!("{parent}/{name}");
            self.namespaces
                .entry(namespace.to_string())
                .or_default()
                .nested
                .entry(name.clone())
                .or_insert(winrt);
            self.insert_nested(namespace, &name, &record.fields, &record.records, winrt);
        }
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.namespaces.get(namespace).is_some_and(|namespace| {
            namespace.nested.contains_key(name)
                || namespace.types.get(name).is_some_and(|v| !v.is_empty())
        })
    }

    pub fn is_value_type(&self, namespace: &str, name: &str) -> bool {
        self.namespaces
            .get(namespace)
            .is_some_and(|namespace| namespace.nested.contains_key(name))
            || self
                .namespaces
                .get(namespace)
                .and_then(|namespace| namespace.types.get(name))
                .and_then(|v| v.first())
                .is_some_and(|(_, item)| {
                    matches!(
                        item,
                        Item::Struct(_) | Item::Enum(_) | Item::Union(_) | Item::Typedef(_)
                    )
                })
    }

    pub fn get<'b>(&'b self, namespace: &str, name: &str) -> impl Iterator<Item = &'b Item> {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .into_iter()
            .flatten()
            .map(|(_, item)| *item)
    }

    pub fn is_winrt(&self, namespace: &str, name: &str) -> Option<bool> {
        if let Some(winrt) = self
            .namespaces
            .get(namespace)
            .and_then(|namespace| namespace.nested.get(name))
        {
            return Some(*winrt);
        }
        self.namespaces
            .get(namespace)
            .and_then(|ns| ns.types.get(name))
            .and_then(|v| v.first())
            .map(|(_, item)| match item {
                Item::Struct(s) => s.winrt,
                Item::Enum(e) => e.winrt,
                Item::Interface(i) => i.winrt,
                Item::Attribute(a) => a.winrt,
                // Delegates and classes are always WinRT.
                Item::Delegate(_) | Item::Class(_) => true,
                // Unions, callbacks, and typedefs are always non-WinRT.
                Item::Union(_) | Item::Callback(_) | Item::Typedef(_) => false,
                Item::Fn(_) | Item::Const(_) | Item::Module(_) => false,
            })
    }
}
