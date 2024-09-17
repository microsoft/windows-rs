use super::*;

pub struct Reader {
    items: HashMap<&'static str, HashMap<&'static str, Item>>,
}

impl Reader {
    pub fn new(files: Vec<File>) -> &'static Self {
        let reader: &'static mut Reader = Box::leak(Box::new(Self {
            items: Default::default(),
        }));

        for mut file in files {
            file.reader = reader as *mut Reader;
            let file = Box::leak(Box::new(file));

            let mut nested = HashMap::<TypeDef, HashMap<&'static str, TypeDef>>::new();

            for key in file.table::<NestedClass>() {
                let inner = key.inner();
                nested
                    .entry(key.outer())
                    .or_default()
                    .insert(inner.name(), inner);
            }

            for def in file.table::<TypeDef>() {
                let namespace = def.namespace();

                if namespace.is_empty() {
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
                    match kind {
                        TypeKind::Interface => {
                            items
                                .entry(name)
                                .and_modify(|existing| {
                                    if let Item::CppInterface(existing) = existing {
                                        existing.push(CppInterface { def });
                                    } else {
                                        panic!("Type mismatch");
                                    }
                                })
                                .or_insert_with(|| Item::CppInterface(vec![CppInterface { def }]));
                        }
                        TypeKind::Class => {
                            if name != "Apis" {
                                unimplemented!("Non-WinRT class not called `Apis`");
                            }

                            for def in def.methods() {
                                let name = def.name();

                                items
                                    .entry(name)
                                    .and_modify(|existing| {
                                        if let Item::CppFn(existing) = existing {
                                            existing.push(CppFn { def, namespace });
                                        } else {
                                            panic!("Type mismatch");
                                        }
                                    })
                                    .or_insert_with(|| Item::CppFn(vec![CppFn { def, namespace }]));
                            }

                            for def in def.fields() {
                                let name = def.name();

                                items
                                    .entry(name)
                                    .and_modify(|existing| {
                                        if let Item::CppConst(existing) = existing {
                                            existing.push(CppConst { def });
                                        } else {
                                            panic!("Type mismatch");
                                        }
                                    })
                                    .or_insert_with(|| Item::CppConst(vec![CppConst { def }]));
                            }
                        }
                        TypeKind::Enum => {
                            items
                                .entry(name)
                                .and_modify(|existing| {
                                    if let Item::CppEnum(existing) = existing {
                                        existing.push(CppEnum { def });
                                    } else {
                                        panic!("Type mismatch");
                                    }
                                })
                                .or_insert_with(|| Item::CppEnum(vec![CppEnum { def }]));
                        }
                        TypeKind::Struct => {
                            
                        }
                        TypeKind::Delegate => {
                            items
                                .entry(name)
                                .and_modify(|existing| {
                                    if let Item::CppCallback(existing) = existing {
                                        existing.push(CppCallback { def });
                                    } else {
                                        panic!("Type mismatch");
                                    }
                                })
                                .or_insert_with(|| Item::CppCallback(vec![CppCallback { def }]));
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
