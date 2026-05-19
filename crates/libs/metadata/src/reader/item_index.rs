use super::*;

#[derive(Debug, Clone)]
pub enum Item {
    Type(TypeDef),
    Fn(MethodDef),
    Const(Field),
}

type HashType = HashMap<String, HashMap<String, Vec<Item>>>;

pub struct ItemIndex(HashType);

impl core::ops::Deref for ItemIndex {
    type Target = HashType;

    fn deref(&self) -> &HashType {
        &self.0
    }
}

impl ItemIndex {
    pub fn new(index: &TypeIndex) -> Self {
        let mut members: HashType = HashMap::new();

        for (namespace, name, ty) in index.iter() {
            let apis = !ty.flags().contains(TypeAttributes::WindowsRuntime)
                && ty.category() == TypeCategory::Class
                && name == "Apis";

            if apis {
                for method in ty.methods() {
                    let name = method.name().to_string();
                    insert(&mut members, namespace, &name, Item::Fn(method));
                }
                for field in ty.fields() {
                    let name = field.name().to_string();
                    insert(&mut members, namespace, &name, Item::Const(field));
                }
            } else {
                insert(&mut members, namespace, name, Item::Type(ty));
            }
        }

        Self(members)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &Item)> + '_ {
        self.0
            .iter()
            .flat_map(|(namespace, items)| {
                items
                    .iter()
                    .map(move |(name, items)| (namespace, name, items))
            })
            .flat_map(|(namespace, name, items)| {
                items.iter().map(move |item| (namespace.as_str(), name.as_str(), item))
            })
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> + '_ {
        self.0.values().flat_map(|items| items.values()).flatten()
    }

    pub fn namespace_items(&self, namespace: &str) -> impl Iterator<Item = (&str, &Item)> + '_ {
        self.0
            .get(namespace)
            .into_iter()
            .flatten()
            .flat_map(|(name, items)| items.iter().map(move |item| (name.as_str(), item)))
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = &Item> + '_ {
        self.0
            .get(namespace)
            .and_then(|items| items.get(name))
            .into_iter()
            .flatten()
    }

    #[track_caller]
    pub fn expect(&self, namespace: &str, name: &str) -> &Item {
        let mut iter = self.get(namespace, name);

        if let Some(item) = iter.next() {
            if iter.next().is_none() {
                item
            } else {
                panic!("more than one type found: {namespace}.{name}");
            }
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }
}

fn insert(members: &mut HashType, namespace: &str, name: &str, member: Item) {
    members
        .entry(namespace.to_string())
        .or_default()
        .entry(name.to_string())
        .or_default()
        .push(member);
}
