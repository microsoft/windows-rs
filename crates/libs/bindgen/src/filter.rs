use super::*;

#[derive(Debug, Default)]
pub struct NameRules(Vec<(String, bool)>);

impl NameRules {
    pub fn has_exclusions(&self) -> bool {
        self.0.iter().any(|(_, include)| !include)
    }

    pub fn new(entries: &[filter_parser::ResolvedFilter]) -> Self {
        use filter_parser::ResolvedKind;
        let mut rules = Vec::new();
        for entry in entries {
            let path = match &entry.kind {
                ResolvedKind::Namespace(namespace) => namespace.clone(),
                ResolvedKind::Type { namespace, name } => format!("{namespace}.{name}"),
                ResolvedKind::Members {
                    namespace, name, ..
                } if !entry.exclude => {
                    format!("{namespace}.{name}")
                }
                _ => continue,
            };
            rules.push((path, !entry.exclude));
        }
        rules.sort_unstable_by(|a, b| (b.0.len(), !b.1, &b.0).cmp(&(a.0.len(), !a.1, &a.0)));
        rules.dedup();
        Self(rules)
    }

    fn matching(&self, name: TypeName) -> Option<(&str, bool)> {
        self.0.iter().find_map(|(path, include)| {
            let matches = if path.len() <= name.namespace().len() {
                namespace_starts_with(name.namespace(), path)
            } else {
                path.strip_prefix(name.namespace())
                    .and_then(|tail| tail.strip_prefix('.'))
                    == Some(name.name())
            };
            matches.then_some((path.as_str(), *include))
        })
    }

    pub fn includes_type_name(&self, name: TypeName) -> Option<&str> {
        self.matching(name)
            .and_then(|(path, include)| include.then_some(path))
    }

    pub fn excludes_type_name(&self, name: TypeName) -> bool {
        self.matching(name).is_some_and(|(_, include)| !include)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Member {
    Method(MethodDef),
    Variant(&'static str),
    Constructor,
}

#[derive(Debug, Default)]
enum Members {
    #[default]
    None,
    Some(HashSet<Member>),
    All,
}

impl Members {
    fn insert(&mut self, member: Member) {
        if matches!(self, Self::None) {
            *self = Self::Some(HashSet::new());
        }
        if let Self::Some(members) = self {
            members.insert(member);
        }
    }

    fn contains(&self, member: Member) -> bool {
        match self {
            Self::None => false,
            Self::Some(members) => members.contains(&member),
            Self::All => true,
        }
    }
}

#[derive(Debug, Default)]
struct Selection {
    keep: Option<Members>,
    drop: HashSet<Member>,
}

impl Selection {
    fn includes(&self, member: Member, default: bool) -> bool {
        !self.drop.contains(&member)
            && self
                .keep
                .as_ref()
                .map_or(default, |keep| keep.contains(member))
    }
}

#[derive(Debug, Default)]
pub struct Filter {
    pub rules: NameRules,
    selections: HashMap<TypeName, Selection>,
}

impl std::ops::Deref for Filter {
    type Target = NameRules;

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl Filter {
    pub fn selected_types(&self) -> impl Iterator<Item = TypeName> + '_ {
        self.selections
            .iter()
            .filter_map(|(name, selection)| selection.keep.is_some().then_some(*name))
    }

    pub fn is_selected(&self, name: TypeName) -> bool {
        self.selections
            .get(&name)
            .is_some_and(|selection| selection.keep.is_some())
    }

    pub fn is_explicit(&self, name: TypeName) -> bool {
        self.includes_type_name(name)
            .is_some_and(|rule| rule.len() > name.namespace().len())
    }

    pub fn includes_method(&self, name: TypeName, method: MethodDef) -> bool {
        self.selections
            .get(&name)
            .is_some_and(|selection| selection.includes(Member::Method(method), false))
    }

    pub fn includes_variant(&self, name: TypeName, variant: &'static str) -> bool {
        self.selections
            .get(&name)
            .is_none_or(|selection| selection.includes(Member::Variant(variant), true))
    }

    pub fn is_activatable(&self, namespace: &'static str, name: &'static str) -> bool {
        self.selections
            .get(&TypeName(namespace, name))
            .is_some_and(|selection| selection.includes(Member::Constructor, false))
    }

    pub fn from_resolved(reader: &Reader, entries: &[filter_parser::ResolvedFilter]) -> Self {
        use filter_parser::ResolvedKind;
        let mut result = Self {
            rules: NameRules::new(entries),
            selections: HashMap::new(),
        };
        for entry in entries {
            match &entry.kind {
                ResolvedKind::Namespace(namespace) if !entry.exclude => {
                    for namespace in reader
                        .namespaces()
                        .filter(|ns| namespace_starts_with(ns, namespace))
                    {
                        for name in reader.type_names(namespace) {
                            result.select_type(reader, name, true);
                        }
                    }
                }
                ResolvedKind::Type { namespace, name } if !entry.exclude => {
                    let name = reader.find_name(namespace, name).unwrap();
                    result.select_type(reader, name, true);
                }
                ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } => {
                    let name = reader.find_name(namespace, name).unwrap();
                    if !entry.exclude {
                        result.select_type(reader, name, false);
                    }
                    for member in members {
                        result.select_member(reader, name, member, !entry.exclude);
                    }
                }
                _ => {}
            }
        }
        result
    }

    fn select_type(&mut self, reader: &Reader, name: TypeName, all: bool) {
        if self.excludes_type_name(name) {
            return;
        }
        let keep = self
            .selections
            .entry(name)
            .or_default()
            .keep
            .get_or_insert_default();
        if !all || matches!(keep, Members::All) {
            return;
        }
        *keep = Members::All;
        for ty in reader.types(name).iter() {
            match ty {
                Type::CppConst(constant) if constant.is_enum_member => {
                    let owner = constant.field.field_type(None, reader).type_name();
                    self.select(owner, Member::Variant(constant.field.name()), true);
                }
                Type::Class(class) => {
                    for interface in class.required_interfaces(reader) {
                        self.select_type(reader, interface.type_name(), true);
                    }
                }
                Type::Interface(interface) => {
                    for interface in interface.required_interfaces(reader) {
                        self.select_type(reader, interface.type_name(), true);
                    }
                }
                Type::CppInterface(interface) => {
                    for base in interface.base_interfaces(reader) {
                        if let Type::CppInterface(base) = base {
                            self.select_type(reader, base.type_name(), true);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn select(&mut self, name: TypeName, member: Member, include: bool) {
        let selection = self.selections.entry(name).or_default();
        if include {
            selection.keep.get_or_insert_default().insert(member);
        } else {
            selection.drop.insert(member);
        }
    }

    #[track_caller]
    fn select_member(&mut self, reader: &Reader, name: TypeName, member: &str, include: bool) {
        let mut found = false;
        for ty in reader.types(name).iter() {
            let mut candidates = Vec::new();
            match ty {
                Type::Enum(e) => {
                    if let Some(field) = e.def.fields().find(|f| {
                        f.name() == member && f.flags().contains(FieldAttributes::Literal)
                    }) {
                        self.select(name, Member::Variant(field.name()), include);
                        found = true;
                    }
                    continue;
                }
                Type::CppEnum(e) => {
                    if let Some(field) = e.def.fields().find(|f| {
                        f.name() == member && f.flags().contains(FieldAttributes::Literal)
                    }) {
                        self.select(name, Member::Variant(field.name()), include);
                        found = true;
                    }
                    continue;
                }
                Type::Class(class) => {
                    if member == "CreateInstance" {
                        self.select(name, Member::Constructor, include);
                        found = true;
                    }
                    candidates.extend(class.required_interfaces(reader).iter().map(|i| i.def));
                }
                Type::Interface(interface) => {
                    candidates.push(interface.def);
                    candidates.extend(interface.required_interfaces(reader).iter().map(|i| i.def));
                }
                Type::CppInterface(interface) => {
                    candidates.push(interface.def);
                    candidates.extend(
                        interface
                            .base_interfaces(reader)
                            .into_iter()
                            .rev()
                            .filter_map(|base| {
                                if let Type::CppInterface(base) = base {
                                    Some(base.def)
                                } else {
                                    None
                                }
                            }),
                    );
                }
                Type::Delegate(delegate) => candidates.push(delegate.def),
                _ => panic!("type `{name}` does not support member-level filtering"),
            }
            for def in candidates {
                let methods = resolve_methods(def, member, include);
                if methods.is_empty() {
                    continue;
                }
                for method in methods {
                    let method = CppInterface::method_owner(method, reader);
                    let MemberRefParent::TypeDef(owner) = method.parent() else {
                        unreachable!()
                    };
                    let owner = owner.type_name();
                    if !include || !self.excludes_type_name(owner) {
                        self.select(owner, Member::Method(method), include);
                    }
                }
                found = true;
                break;
            }
        }
        assert!(found, "member `{member}` not found on `{name}`");
    }
}

fn resolve_methods(def: TypeDef, name: &str, include: bool) -> Vec<MethodDef> {
    let methods: Vec<_> = def.methods().collect();
    let named = |method: &MethodDef| {
        method_overload_name(*method)
            .as_deref()
            .unwrap_or(method.name())
            == name
    };
    let mut result: Vec<_> = methods.iter().copied().filter(named).collect();
    if result.is_empty() {
        for prefixes in [["get_", "put_"], ["add_", "remove_"]] {
            result.extend(methods.iter().copied().filter(|method| {
                prefixes
                    .iter()
                    .any(|prefix| method.name().strip_prefix(prefix) == Some(name))
            }));
            if !result.is_empty() {
                break;
            }
        }
    }
    if include {
        let removers: Vec<_> = result
            .iter()
            .filter_map(|method| {
                let event = method.name().strip_prefix("add_")?;
                methods
                    .iter()
                    .copied()
                    .find(|method| method.name().strip_prefix("remove_") == Some(event))
            })
            .collect();
        result.extend(removers);
    }
    result
}
