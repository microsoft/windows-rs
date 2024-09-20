use super::*;
use std::collections::hash_map::Entry::*;

// TODO: this is the index for general type queries e.g. when there's a TypeRef or Blob that refers to a type by name
// then there's transmformations to flatten or filter this but the reader remains immutable to support the queries.
// This filtering process can then maybe remove actual duplicates?
pub struct Reader(HashMap<&'static str, HashMap<&'static str, Item>>);

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Item>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> &'static Self {
        let reader: &'static mut Reader = Box::leak(Box::new(Self(Default::default())));

        for mut file in files {
            file.reader = reader as *mut Reader;
            let file = Box::leak(Box::new(file));

            let mut nested = HashMap::<TypeDef, Vec<TypeDef>>::new();

            for key in file.table::<NestedClass>() {
                let inner = key.inner();
                nested.entry(key.outer()).or_default().push(inner);
            }

            for def in file.table::<TypeDef>() {
                let namespace = def.namespace();

                if namespace.is_empty() {
                    // This skips the nested types as we've already retrieved them.
                    continue;
                }

                let items = reader.0.entry(namespace).or_default();
                let name = def.name();
                let kind = def.kind();

                fn push(
                    items: &mut HashMap<&'static str, Item>,
                    name: &'static str,
                    item: Item,
                ) {
                    match items.entry(name) {
                        Occupied(mut existing) => {
                            match existing.get_mut() {
                                Item::Overload(existing) => existing.push(item),
                            }
                        }
                        Vacant(entry) => {
                            entry.insert(Item::Cpp(vec![item]));
                        }
                    }
                }

                if def.flags().contains(TypeAttributes::WindowsRuntime) {
                    match kind {
                        TypeKind::Interface => {
                            items.insert(name, Item::Interface(Interface { def }))
                        }
                        TypeKind::Class => items.insert(name, Item::Class(Class { def })),
                        TypeKind::Enum => items.insert(name, Item::Enum(Enum { def })),
                        TypeKind::Struct => items.insert(name, Item::Struct(Struct { def })),
                        TypeKind::Delegate => items.insert(name, Item::Delegate(Delegate { def })),
                    };
                } else {


                    match kind {
                        TypeKind::Interface => {
                            let item = CppItem::Interface(CppInterface { def });
                            push(items, name, item);
                        }
                        TypeKind::Class => {
                            if name == "Apis" {
                                for method in def.methods() {
                                    let name = method.name();
                                    let item = CppItem::Fn(CppFn { def, method });
                                    push(items, name, item);
                                }

                                for field in def.fields() {
                                    let name = field.name();
                                    let item = CppItem::Const(CppConst { def, field });
                                    push(items, name, item);
                                }
                            }
                        }
                        TypeKind::Enum => {
                            let item = CppItem::Enum(CppEnum { def });
                            push(items, name, item);

                            if !def.has_attribute("ScopedEnumAttribute") {
                                for field in def.fields() {
                                    if field.flags().contains(FieldAttributes::Literal) {
                                        let name = field.name();
                                        let item = CppItem::Const(CppConst { def, field });
                                        push(items, name, item);
                                    }
                                }
                            }
                        }
                        TypeKind::Struct => {
                            fn make(
                                def: TypeDef,
                                nested: &HashMap<TypeDef, Vec<TypeDef>>,
                            ) -> CppStruct {
                                let mut item = CppStruct {
                                    def,
                                    nested: HashMap::new(),
                                };

                                for def in nested.get(&def).into_iter().flatten() {
                                    item.nested.insert(def.name(), make(*def, nested));
                                }

                                item
                            }

                            let item = CppItem::Struct(make(def, &nested));
                            push(items, name, item);
                        }
                        TypeKind::Delegate => {
                            let item = CppItem::Delegate(CppDelegate { def });
                            push(items, name, item);
                        }
                    };
                }
            }
        }

        reader
    }

    pub fn get(&self, namespace: &str, name: &str) -> Option<&Item> {
        self.0.get(namespace).and_then(|items| items.get(name))
    }
}
