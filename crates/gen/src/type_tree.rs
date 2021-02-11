use crate::*;
use squote::TokenStream;

/// A namespaced tree of types
#[derive(Default)]
pub struct TypeTree {
    pub types: Vec<TypeDefinition>,
    pub namespaces: TypeNamespaces,
    pub include_foundation: bool,
}

impl TypeTree {
    pub fn from_limits(reader: &'static winmd::TypeReader, limits: &TypeLimits) -> Self {
        let mut tree = TypeTree::default();
        let mut set = std::collections::BTreeSet::new();

        for limit in limits.limits() {
            match &limit.limit {
                TypeLimit::All => {
                    for def in reader.namespace_types(&limit.namespace) {
                        tree.insert_if(reader, &limit.namespace, &mut set, &def);
                    }
                }
                TypeLimit::Some(types) => {
                    for name in types {
                        tree.insert_if(
                            reader,
                            &limit.namespace,
                            &mut set,
                            &reader.resolve_type(&limit.namespace, name),
                        );
                    }
                }
            }
        }

        tree
    }

    fn insert_if(
        &mut self,
        reader: &winmd::TypeReader,
        namespace: &'static str,
        set: &mut std::collections::BTreeSet<winmd::TypeDef>,
        def: &winmd::ElementType,
    ) {
        match def {
            winmd::ElementType::TypeDef(def) => match def.def.category() {
                winmd::TypeCategory::Contract | winmd::TypeCategory::Attribute => {}
                _ => {
                    if set.insert(def.def) {
                        let t = TypeDefinition::from_type_def(&def.def);

                        for def in t.dependencies() {
                            self.insert_if(
                                reader,
                                def.namespace(),
                                set,
                                &winmd::ElementType::TypeDef(winmd::GenericTypeDef::from_type_def(
                                    def,
                                    Vec::new(),
                                )),
                            );
                        }

                        self.insert(namespace, t);
                    }
                }
            },
            winmd::ElementType::MethodDef(method) => {
                let t = TypeDefinition::from_method_def(method, namespace);

                // TODO: need universal dependencies (this is duplicated here and there)
                for def in t.dependencies() {
                    self.insert_if(
                        reader,
                        def.namespace(),
                        set,
                        &winmd::ElementType::TypeDef(winmd::GenericTypeDef::from_type_def(
                            def,
                            Vec::new(),
                        )),
                    );
                }

                self.insert(namespace, t);
            }
            winmd::ElementType::Constant(field) => {
                let t = TypeDefinition::from_field(field);
                self.insert(namespace, t);
            }
            _ => {}
        }
    }

    /// Insert a [`TypeDefinition`] into [`TypeTree`]
    ///
    /// This recursively searchs the tree for an entry corresponding to the namespace
    pub fn insert(&mut self, namespace: &'static str, t: TypeDefinition) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .0
                .entry(&namespace[..pos])
                .or_default()
                .insert(&namespace[pos + 1..], t);
        } else {
            self.namespaces
                .0
                .entry(namespace)
                .or_default()
                .types
                .push(t);
        }
    }

    pub fn remove(&mut self, namespace: &str) {
        if let Some(pos) = namespace.find('.') {
            if let Some(tree) = self.namespaces.0.get_mut(&namespace[..pos]) {
                tree.remove(&namespace[pos + 1..])
            }
        } else {
            self.namespaces.0.remove(namespace);
        }
    }

    pub fn reexport(&mut self) {
        self.namespaces
            .0
            .entry("Windows")
            .or_default()
            .include_foundation = true;
    }

    /// Turn the tree into a token stream for code generation
    pub fn gen<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.types
            .iter()
            .map(|t| t.gen())
            .chain(self.namespaces.gen())
    }
}
