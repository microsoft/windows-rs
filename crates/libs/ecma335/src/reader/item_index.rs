use super::*;

type HashType<'a> = HashMap<&'a str, HashMap<&'a str, Vec<Item<'a>>>>;

pub struct ItemIndex<'a>(HashType<'a>);

impl<'a> std::ops::Deref for ItemIndex<'a> {
    type Target = HashType<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ItemIndex<'a> {
    pub fn new(index: &'a Index) -> Self {
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
}

fn insert<'a>(members: &mut HashType<'a>, namespace: &'a str, name: &'a str, member: Item<'a>) {
    members
        .entry(namespace)
        .or_default()
        .entry(name)
        .or_default()
        .push(member);
}
