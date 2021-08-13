use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeInclude {
    Full,
    Minimal,
    None,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TypeEntry {
    pub def: ElementType,
    pub include: TypeInclude,
}

impl TypeEntry {
    pub fn gen(&self, gen: &Gen) -> TokenStream {
        if self.include == TypeInclude::None {
            return TokenStream::new();
        }

        match &self.def {
            ElementType::TypeDef(def) => def.clone().with_generics().gen(gen, self.include),
            ElementType::MethodDef(def) => def.gen(gen),
            ElementType::Field(def) => gen_field(def, gen),
            _ => unimplemented!(),
        }
    }
}

pub struct TypeTree {
    pub namespace: &'static str,
    pub types: BTreeMap<&'static str, TypeEntry>,
    pub namespaces: BTreeMap<&'static str, TypeTree>,
    pub include: bool,
}

impl TypeTree {
    pub fn from_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            types: BTreeMap::new(),
            namespaces: BTreeMap::new(),
            include: false,
        }
    }

    pub fn insert_namespace(&mut self, namespace: &'static str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::from_namespace(&namespace[..next]))
                .insert_namespace(namespace, next + 1)
        } else {
            self.namespaces
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::from_namespace(namespace))
        }
    }

    pub fn insert_type(&mut self, name: &'static str, def: ElementType) {
        self.types.entry(name).or_insert_with(|| TypeEntry {
            def,
            include: TypeInclude::None,
        });
    }

    // TODO: slow method - remove or make this an iterator somehow?
    pub fn namespaces(&self) -> Vec<&'static str> {
        let mut namespaces = Vec::new();

        for tree in self.namespaces.values() {
            if !tree.types.is_empty() {
                namespaces.push(tree.namespace)
            }

            namespaces.append(&mut tree.namespaces());
        }

        namespaces
    }

    pub fn get_type(&self, name: &str) -> Option<&TypeEntry> {
        self.types.get(name)
    }

    pub fn get_type_mut(&mut self, name: &str) -> Option<&mut TypeEntry> {
        self.types.get_mut(name)
    }

    pub fn get_namespace(&self, namespace: &str) -> Option<&Self> {
        self.get_namespace_pos(namespace, 0)
    }

    fn get_namespace_pos(&self, namespace: &str, pos: usize) -> Option<&Self> {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .get(&namespace[pos..next])
                .and_then(|child| child.get_namespace_pos(namespace, next + 1))
        } else {
            self.namespaces.get(&namespace[pos..])
        }
    }

    pub fn get_namespace_mut(&mut self, namespace: &str) -> Option<&mut Self> {
        self.get_namespace_mut_pos(namespace, 0)
    }

    fn get_namespace_mut_pos(&mut self, namespace: &str, pos: usize) -> Option<&mut Self> {
        self.include = true;
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .get_mut(&namespace[pos..next])
                .and_then(|child| child.get_namespace_mut_pos(namespace, next + 1))
        } else {
            self.namespaces.get_mut(&namespace[pos..]).map(|ns| {
                ns.include = true;
                ns
            })
        }
    }

    pub fn gen(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let gen = Gen::Relative(self.namespace);

        self.types
            .iter()
            .map(move |t| t.1.gen(&gen))
            .chain(gen_namespaces(&self.namespaces))
    }
}

fn gen_namespaces<'a>(
    namespaces: &'a BTreeMap<&'static str, TypeTree>,
) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include {
            let name = to_ident(name);

            let tokens = tree.gen();

            quote! {
                // TODO: https://github.com/microsoft/windows-rs/issues/212
                // TODO: https://github.com/microsoft/win32metadata/issues/380
                #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
                pub mod #name {
                    #(#tokens)*
                }
            }
         } else {
             TokenStream::new()
         }
    })
}
