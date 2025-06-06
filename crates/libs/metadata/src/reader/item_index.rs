use super::*;

pub enum Item<'a> {
    Type(TypeDef<'a>),
    Fn(MethodDef<'a>),
    Const(Field<'a>),
}

type HashType<'a> = HashMap<&'a str, HashMap<&'a str, Vec<Item<'a>>>>;

pub struct ItemIndex<'a>(HashType<'a>);

impl<'a> ItemIndex<'a> {
    pub fn new(index: &'a TypeIndex) -> Self {
        let mut members: HashType = HashMap::new();

        for (namespace, name, ty) in index.iter() {
            insert(&mut members, namespace, name, Item::Type(ty));

            if !ty.flags().contains(TypeAttributes::WindowsRuntime) {
                match ty.category() {
                    TypeCategory::Class if name == "Apis" => {
                        for method in ty.methods() {
                            insert(&mut members, namespace, method.name(), Item::Fn(method));
                        }
                        for field in ty.fields() {
                            insert(&mut members, namespace, field.name(), Item::Const(field));
                        }
                    }
                    TypeCategory::Enum if !ty.has_attribute("ScopedEnumAttribute") => {
                        for field in ty.fields() {
                            if field.flags().contains(FieldAttributes::Literal) {
                                insert(&mut members, namespace, field.name(), Item::Const(field));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Self(members)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &Item<'_>)> + '_ {
        self.0
            .iter()
            .flat_map(|(namespace, items)| {
                items
                    .iter()
                    .map(move |(name, items)| (namespace, name, items))
            })
            .flat_map(|(namespace, name, items)| {
                items.iter().map(move |item| (*namespace, *name, item))
            })
    }

    pub fn items(&self) -> impl Iterator<Item = &Item<'_>> + '_ {
        self.0.values().flat_map(|items| items.values()).flatten()
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = &Item<'_>> + '_ {
        self.0
            .get(namespace)
            .and_then(|items| items.get(name))
            .into_iter()
            .flatten()
    }

    #[track_caller]
    pub fn expect(&self, namespace: &str, name: &str) -> &Item<'_> {
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

fn insert<'a>(members: &mut HashType<'a>, namespace: &'a str, name: &'a str, member: Item<'a>) {
    members
        .entry(namespace)
        .or_default()
        .entry(name)
        .or_default()
        .push(member);
}
