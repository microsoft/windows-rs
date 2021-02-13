use super::*;

pub struct TypeTree {
    pub namespace: &'static str,
    pub types: Vec<ElementType>,
    pub namespaces: BTreeMap<&'static str, TypeTree>,
    pub include_foundation: bool,
}

impl TypeTree {
    fn from_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            types: Vec::new(),
            namespaces: BTreeMap::new(),
            include_foundation: false,
        }
    }

    pub fn from_limits(reader: &'static TypeReader, limits: &TypeLimits) -> Self {
        let mut root = Self::from_namespace("");

        let mut set = BTreeSet::new();

        for limit in limits.limits() {
            match &limit.limit {
                TypeLimit::All => {
                    for def in reader.namespace_types(&limit.namespace) {
                        root.insert_if(reader, &limit.namespace, &mut set, &def);
                    }
                }
                TypeLimit::Some(types) => {
                    for name in types {
                        root.insert_if(
                            reader,
                            &limit.namespace,
                            &mut set,
                            &reader.resolve_type(&limit.namespace, name),
                        );
                    }
                }
            }
        }

        root
    }

    fn insert_if(
        &mut self,
        reader: &TypeReader,
        namespace: &'static str,
        set: &mut BTreeSet<ElementType>,
        t: &ElementType,
    ) {
        if set.insert(t.clone()) {
            for def in t.dependencies() {
                self.insert_if(reader, def.namespace(), set, &ElementType::from_type_def(def, Vec::new()));
            }
            self.insert(namespace, t);
        }
    }

    fn insert(&mut self, namespace: &'static str, t: &ElementType) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .entry(&namespace[..pos])
                .or_insert_with(||Self::from_namespace(namespace))
                .insert(&namespace[pos + 1..], t);
        } else {
            self.namespaces
                .entry(namespace)
                .or_insert_with(||Self::from_namespace(namespace))
                .types
                .push(t.clone());
        }
    }

    pub fn remove(&mut self, namespace: &str) {
        if let Some(pos) = namespace.find('.') {
            if let Some(tree) = self.namespaces.get_mut(&namespace[..pos]) {
                tree.remove(&namespace[pos + 1..])
            }
        } else {
            self.namespaces.remove(namespace);
        }
    }

    pub fn reexport(&mut self) {
        self.namespaces
            .entry("Windows")
            .or_insert_with(||Self::from_namespace("Windows"))
            .include_foundation = true;
    }

    /// Turn the tree into a token stream for code generation
    pub fn gen<'a>(&'a self, gen: Gen) -> impl Iterator<Item = TokenStream> + 'a {
        self.types
            .iter()
            .map(move |t| t.gen(gen))
            .chain(gen_namespaces(&self.namespaces))
    }
}

fn gen_namespaces<'a>(namespaces: &'a BTreeMap<&'static str, TypeTree>) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(|(name, tree)| {
        let name = to_snake(name);
        let name = to_ident(&name);

        // TODO: why doesn't gen just retun a TokenStream?
        let mut tokens = tree.gen(Gen::Relative(tree.namespace)).collect::<Vec<_>>();

        if tree.include_foundation {
            tokens.push(quote! { pub use ::windows::*; });
        }

        quote! {
            // TODO: remove this allowance when https://github.com/microsoft/windows-rs/issues/212 is fixed
            #[allow(unused_variables)]
            #[allow(non_upper_case_globals)]
            pub mod #name {
                #(#tokens)*
            }
        }
    })
}
