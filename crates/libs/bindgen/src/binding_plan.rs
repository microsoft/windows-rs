use super::*;

/// Resolved ownership, definitions, and callable method instances consumed by the writers.
pub struct BindingPlan {
    pub selection: Selection,
    pub types: TypeMap,
    pub external: HashMap<TypeName, String>,
    pub implemented: HashSet<TypeName>,
    methods: HashMap<MethodDef, HashSet<Vec<Type>>>,
}

impl BindingPlan {
    pub fn new(
        reader: &Reader,
        entries: &[filter_parser::ResolvedFilter],
        references: References,
        implements: Option<&Implements>,
        style: Style,
    ) -> Self {
        let selection = Selection::resolve(reader, entries, &references, implements);
        let types = TypeClosure::build(reader, &selection, &references, implements, style);
        let implemented = types
            .keys()
            .copied()
            .filter(|name| implements.is_some_and(|implements| implements.matches(*name)))
            .collect();
        let external = references.into_external(&types);
        let mut plan = Self {
            selection,
            types,
            external,
            implemented,
            methods: HashMap::new(),
        };

        let mut interfaces = HashSet::new();
        let mut native = HashSet::new();
        for ty in plan.types.values().flatten() {
            let required = match ty {
                Type::Interface(interface) => {
                    interfaces.insert(interface.clone());
                    interface.required_interfaces(reader)
                }
                Type::Class(class) => class.required_interfaces(reader),
                Type::CppInterface(interface) => {
                    native.insert(interface.clone());
                    continue;
                }
                _ => continue,
            };
            interfaces.extend(required);
        }
        for interface in interfaces {
            if plan.provides(&Type::Interface(interface.clone())) {
                plan.select_methods(reader, interface.def, &interface.generics);
            }
        }
        for interface in native {
            plan.select_methods(reader, interface.def, &[]);
        }
        plan
    }

    fn select_methods(&mut self, reader: &Reader, owner: TypeDef, generics: &[Type]) {
        let name = owner.type_name();
        for method in owner.methods() {
            if !self.external.contains_key(&name)
                && !self.implemented.contains(&name)
                && !self.selection.includes_method(name, method)
            {
                continue;
            }
            let signature = method.method_signature(generics, reader);
            if signature.types().all(|ty| self.provides(ty))
                && method
                    .associated_enums(reader)
                    .into_iter()
                    .all(|ty| self.provides(&ty))
            {
                self.methods
                    .entry(method)
                    .or_default()
                    .insert(generics.to_vec());
            }
        }
    }

    pub fn provides(&self, ty: &Type) -> bool {
        let name = ty.type_name();
        (name.namespace().is_empty()
            || self.types.contains_key(&name)
            || self.external.contains_key(&name))
            && ty.generics().iter().all(|ty| self.provides(ty))
    }

    pub fn includes_method(&self, method: MethodDef, generics: &[Type]) -> bool {
        self.methods
            .get(&method)
            .is_some_and(|instances| instances.contains(generics))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_generic_method_availability() {
        let reader = default_reader(false);
        let entries: Vec<_> = [
            "Windows.Foundation.Collections.StringMap",
            "Windows.UI.Composition.ImplicitAnimationCollection::{}",
        ]
        .into_iter()
        .flat_map(filter_parser::parse_filter_entry)
        .collect();
        let entries = filter_parser::resolve_entries(reader, &entries);
        let references = References::new(
            reader,
            vec![ReferenceStage::new(
                "windows_collections",
                "Windows.Foundation.Collections",
            )],
        );
        let plan = BindingPlan::new(reader, &entries, references, None, Style::Full);
        for (namespace, name, available) in [
            ("Windows.Foundation.Collections", "StringMap", true),
            (
                "Windows.UI.Composition",
                "ImplicitAnimationCollection",
                false,
            ),
        ] {
            let Type::Class(class) = reader.with_full_name(namespace, name).next().unwrap() else {
                panic!();
            };
            let interface = class
                .required_interfaces(reader)
                .into_iter()
                .find(|interface| interface.def.name() == "IIterable`1")
                .unwrap();
            let first = interface
                .def
                .methods()
                .find(|method| method.name() == "First")
                .unwrap();
            assert_eq!(
                plan.includes_method(first, &interface.generics),
                available,
                "{name}::First"
            );
        }
    }
}
