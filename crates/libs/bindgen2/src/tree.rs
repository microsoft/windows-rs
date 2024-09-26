use super::*;

type Dependencies = HashMap<&'static str, HashSet<&'static str>>;

#[derive(Debug)]
pub struct Tree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Self>,
    pub items: HashSet<&'static str>,
}

impl Tree {
    pub fn new(reader: &winmd::Reader, filter: &Filter, include_dependencies: bool) -> Self {
        let mut tree = Self::with_namespace("");
        let mut dependencies = HashMap::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                let tree = tree.insert_namespace(namespace);

                for name in reader[namespace].keys() {
                    if filter.includes_type_name(namespace, name) {
                        tree.items.insert(name);

                        if include_dependencies {
                            for item in &reader[namespace][name] {
                                item_dependencies(item, &mut dependencies);
                            }
                        }
                    }
                }
            }
        }

        if include_dependencies {
            for (namespace, names) in &dependencies {
                for name in names {
                    tree.insert_namespace(namespace).items.insert(name);
                }
            }
        }

        tree
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: HashMap::new(),
            items: HashSet::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str) -> &mut Self {
        fn insert_namespace<'a>(
            parent: &'a mut Tree,
            namespace: &'static str,
            pos: usize,
        ) -> &'a mut Tree {
            if let Some(next) = namespace[pos..].find('.') {
                let next = pos + next;

                let parent = parent
                    .nested
                    .entry(&namespace[pos..next])
                    .or_insert_with(|| Tree::with_namespace(&namespace[..next]));

                insert_namespace(parent, namespace, next + 1)
            } else {
                parent
                    .nested
                    .entry(&namespace[pos..])
                    .or_insert_with(|| Tree::with_namespace(namespace))
            }
        }

        insert_namespace(self, namespace, 0)
    }
}

fn item_dependencies(item: &winmd::Item, dependencies: &mut Dependencies) {
    match item {
        winmd::Item::Class(item) => class_dependencies(item.def, dependencies),
        winmd::Item::Delegate(item) => delegate_dependencies(item.def, dependencies),
        winmd::Item::Enum(item) => enum_dependencies(item.def, dependencies),
        winmd::Item::Interface(item) => interface_dependencies(item.def, dependencies),
        winmd::Item::Struct(item) => struct_dependencies(item.def, dependencies),
        winmd::Item::CppConst(item) => cpp_const_dependencies(item, dependencies),
        winmd::Item::CppDelegate(item) => delegate_dependencies(item.def, dependencies),
        winmd::Item::CppEnum(item) => enum_dependencies(item.def, dependencies),
        winmd::Item::CppFn(item) => cpp_fn_dependencies(item, dependencies),
        winmd::Item::CppInterface(item) => interface_dependencies(item.def, dependencies),
        winmd::Item::CppStruct(item) => cpp_struct_dependencies(item, dependencies),
    }
}

fn class_dependencies(_def: winmd::TypeDef, _dependencies: &mut Dependencies) {

}

fn delegate_dependencies(_def: winmd::TypeDef, _dependencies: &mut Dependencies) {

}

fn enum_dependencies(_def: winmd::TypeDef, _dependencies: &mut Dependencies) {

}

fn interface_dependencies(_def: winmd::TypeDef, _dependencies: &mut Dependencies) {

}

fn struct_dependencies(_def: winmd::TypeDef, _dependencies: &mut Dependencies) {

}

fn cpp_struct_dependencies(_item: &winmd::CppStruct, _dependencies: &mut Dependencies) {
    // TODO: call struct_dependencies recursively for nested structs
}

fn cpp_fn_dependencies(_item: &winmd::CppFn, _dependencies: &mut Dependencies) {
    
}

fn cpp_const_dependencies(_item: &winmd::CppConst, _dependencies: &mut Dependencies) {

}



