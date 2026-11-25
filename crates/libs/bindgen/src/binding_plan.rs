use super::*;

/// Resolved ownership, definitions, and callable method instances consumed by the writers.
pub struct BindingPlan {
    pub selection: Selection,
    pub types: TypeMap,
    pub external: HashMap<TypeName, String>,
    pub implemented: HashSet<TypeName>,
    methods: HashMap<MethodDef, HashSet<Vec<GenericType>>>,
}

// Interface equality identifies definitions. Method instances also depend on nested arguments.
#[derive(PartialEq, Eq, Hash)]
enum GenericType {
    Interface(TypeDef, Vec<Self>),
    Delegate(TypeDef, Vec<Self>),
    Type(Type),
}

impl GenericType {
    fn arguments(types: &[Type]) -> Vec<Self> {
        types
            .iter()
            .map(|ty| match ty {
                Type::Interface(ty) => Self::Interface(ty.def, Self::arguments(&ty.generics)),
                Type::Delegate(ty) => Self::Delegate(ty.def, Self::arguments(&ty.generics)),
                _ => Self::Type(ty.clone()),
            })
            .collect()
    }
}

impl BindingPlan {
    pub fn new(
        reader: &Reader,
        entries: &[filter_parser::ResolvedFilter],
        references: References,
        implements: Option<&Implements>,
        style: Style,
    ) -> Self {
        let selection = Selection::resolve(reader, entries, &references);
        let (types, implemented) =
            TypeClosure::build(reader, &selection, &references, implements, style);
        if let Some(implements) = implements.filter(|implements| !implements.is_empty()) {
            for entry in entries {
                if let filter_parser::ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } = &entry.kind
                    && !members.is_empty()
                {
                    let ty = reader.with_full_name(namespace, name).next().unwrap();
                    let name = ty.type_name();
                    if matches!(ty, Type::Interface(_) | Type::CppInterface(_)) {
                        assert!(
                            !implements.matches(name) && !implemented.contains(&name),
                            "method-level filter on `{name}` conflicts with `--implement`: \
                             methods on implemented interfaces are always emitted"
                        );
                    }
                }
            }
        }
        let external = references.into_external(&types);
        let mut plan = Self {
            selection,
            types,
            external,
            implemented,
            methods: HashMap::new(),
        };

        let mut interfaces = HashMap::new();
        let mut include = |interface: Interface| {
            interfaces
                .entry((interface.def, GenericType::arguments(&interface.generics)))
                .or_insert(interface);
        };
        let mut native = HashSet::new();
        for ty in plan.types.values().flatten() {
            let required = match ty {
                Type::Interface(interface) => {
                    include(interface.clone());
                    interface.required_interfaces(reader)
                }
                Type::Class(class) => class.required_interfaces(reader),
                Type::CppInterface(interface) => {
                    native.insert(interface.clone());
                    continue;
                }
                _ => continue,
            };
            for interface in required {
                include(interface);
            }
        }
        for interface in interfaces.into_values() {
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
                    .insert(GenericType::arguments(generics));
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
            .is_some_and(|instances| instances.contains(&GenericType::arguments(generics)))
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
