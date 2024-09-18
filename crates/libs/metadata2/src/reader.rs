use super::*;
use std::collections::hash_map::Entry::*;

pub struct Reader {
    pub items: HashMap<&'static str, HashMap<&'static str, Item>>,
}

impl Reader {
    pub fn new(files: Vec<File>) -> &'static Self {
        let reader: &'static mut Reader = Box::leak(Box::new(Self {
            items: Default::default(),
        }));

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
                    // This skips over nested TypeDefs.
                    continue;
                }

                let items = reader.items.entry(namespace).or_default();
                let name = def.name();
                let kind = def.kind();

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
                    fn push(
                        items: &mut HashMap<&'static str, Item>,
                        name: &'static str,
                        item: CppItem,
                    ) {
                        match items.entry(name) {
                            Occupied(mut existing) => {
                                if let Item::Cpp(existing) = existing.get_mut() {
                                    existing.push(item);
                                } else {
                                    panic!("Type mismatch");
                                }
                            }
                            Vacant(entry) => {
                                entry.insert(Item::Cpp(vec![item]));
                            }
                        }
                    }

                    match kind {
                        TypeKind::Interface => {
                            let item = CppItem::Interface(CppInterface { def });
                            push(items, name, item);
                        }
                        TypeKind::Class => {
                            if name == "Apis" {
                                for def in def.methods() {
                                    let name = def.name();
                                    let item = CppItem::Fn(CppFn { def, namespace });
                                    push(items, name, item);
                                }

                                for def in def.fields() {
                                    let name = def.name();
                                    let item = CppItem::Const(CppConst { def });
                                    push(items, name, item);
                                }
                            }
                        }
                        TypeKind::Enum => {
                            let item = CppItem::Enum(CppEnum { def });
                            push(items, name, item);

                            if !def.has_attribute("ScopedEnumAttribute") {
                                for def in def.fields() {
                                    if def.flags().contains(FieldAttributes::Literal) {
                                        let name = def.name();
                                        let item = CppItem::Const(CppConst { def });
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

    //         match kind {
    //             TypeKind::Interface => {
    //                 push(items, CppItem::Interface(CppInterface { def }));
    //             }
    //             TypeKind::Class => {
    //                 if name != "Apis" {
    //                     unimplemented!("Non-WinRT class not called `Apis`");
    //                 }

    //                 for method in def.methods() {

    //                 }

    //                 for field in def.fields() {

    //                 }
    //             }
    //             TypeKind::Enum => {
    //                 push(items, CppItem::Enum(CppEnum { def }));
    //             }
    //             TypeKind::Struct => {
    //                 push(items, CppItem::Interface(CppInterface { def }));
    //             }
    //             TypeKind::Delegate => {

    //             }
    //         }
    //     }

    //     if name == "Apis" {
    //         for method in def.methods() {
    //             namespace_items
    //                 .entry(method.name())
    //                 .or_default()
    //                 .push(Item::Fn(method, namespace));
    //         }

    //         for field in def.fields() {
    //             namespace_items
    //                 .entry(field.name())
    //                 .or_default()
    //                 .push(Item::Const(field));
    //         }
    //     } else {
    //         namespace_items
    //             .entry(name)
    //             .or_default()
    //             .push(Item::Type(def));

    //         // TODO: these should all be fields on the Apis class so we don't have to go looking for all of these as well.
    //         if def.extends() == Some(TypeName::Enum)
    //             && !def.flags().contains(TypeAttributes::WindowsRuntime)
    //             && !def.has_attribute("ScopedEnumAttribute")
    //         {
    //             for field in def
    //                 .fields()
    //                 .filter(|field| field.flags().contains(FieldAttributes::Literal))
    //             {
    //                 namespace_items
    //                     .entry(field.name())
    //                     .or_default()
    //                     .push(Item::Const(field));
    //             }
    //         }
    //     }
    // }
    //     }

    //     reader
    // }

    pub fn type_from_blob(
        &self,
        _blob: &mut Blob,
        _enclosing: Option<TypeDef>,
        _generics: &[Type],
    ) -> Type {
        todo!()
    }

    pub fn get_type_def(
        &self,
        _namespace: &str,
        _name: &str,
    ) -> impl Iterator<Item = TypeDef> + '_ {
        todo!();
        None.into_iter()
    }

    pub fn type_from_ref(
        &self,
        _code: TypeDefOrRef,
        _enclosing: Option<TypeDef>,
        _generics: &[Type],
    ) -> Type {
        todo!();
    }
}
