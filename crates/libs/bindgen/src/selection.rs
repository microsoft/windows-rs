use super::*;
use filter_parser::{ResolvedFilter, ResolvedKind};

/// Resolved selections. Method identities belong to the type exposing them, which may be a
/// class rather than their declaring interface.
#[derive(Debug, Default)]
pub struct Selection {
    pub roots: BTreeSet<TypeName>,
    pub methods: HashMap<TypeName, HashSet<MethodDef>>,
    explicit: HashSet<TypeName>,
    excluded: HashSet<TypeName>,
    variants: HashMap<TypeName, VariantSelection>,
    constructors: HashSet<TypeName>,
    full_hierarchy: HashSet<TypeName>,
    hierarchy: HashSet<(TypeName, TypeName)>,
}

#[derive(Debug, Default)]
pub struct VariantSelection(HashSet<&'static str>);

impl VariantSelection {
    pub fn includes(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Member {
    Method(MethodDef),
    Variant(&'static str),
    Constructor,
}

/// Authored requests exist only while resolving selections.
#[derive(Default)]
struct Request {
    whole: bool,
    keep: HashSet<Member>,
    drop: HashSet<Member>,
}

impl Request {
    fn includes(&self, member: Member) -> bool {
        !self.drop.contains(&member)
            && if self.keep.is_empty() {
                self.whole || !self.drop.is_empty()
            } else {
                self.keep.contains(&member)
            }
    }

    fn add(&mut self, members: impl IntoIterator<Item = Member>, include: bool) {
        if include {
            self.keep.extend(members);
        } else {
            self.drop.extend(members);
        }
    }
}

/// Name matching is confined to request and reference resolution.
#[derive(Debug)]
pub struct NameRules(Vec<(String, bool)>);

impl NameRules {
    pub fn new(entries: &[ResolvedFilter]) -> Self {
        let mut rules = Self(
            entries
                .iter()
                .map(|entry| match &entry.kind {
                    ResolvedKind::Namespace(name) => (name.clone(), !entry.exclude),
                    ResolvedKind::Type { namespace, name } => {
                        (format!("{namespace}.{name}"), !entry.exclude)
                    }
                    ResolvedKind::Members {
                        namespace, name, ..
                    } => (format!("{namespace}.{name}"), true),
                })
                .collect(),
        );
        rules.sort();
        rules
    }

    fn sort(&mut self) {
        self.0.sort_unstable_by(|left, right| {
            (right.0.len(), !right.1).cmp(&(left.0.len(), !left.1))
        });
    }

    fn matching(&self, name: TypeName) -> Option<&(String, bool)> {
        self.0
            .iter()
            .find(|rule| match_type_name(&rule.0, name.namespace(), name.name()))
    }

    pub fn applies_to_namespace(&self, namespace: &str) -> bool {
        self.0.iter().any(|(rule, _)| {
            namespace_starts_with(namespace, rule) || namespace_starts_with(rule, namespace)
        })
    }

    pub fn includes(&self, name: TypeName) -> Option<&str> {
        self.matching(name)
            .filter(|rule| rule.1)
            .map(|rule| rule.0.as_str())
    }

    fn excludes(&self, name: TypeName) -> bool {
        self.matching(name).is_some_and(|rule| !rule.1)
    }
}

impl Selection {
    #[track_caller]
    pub fn resolve(reader: &Reader, entries: &[ResolvedFilter], references: &References) -> Self {
        let mut rules = NameRules::new(entries);
        let mut requests = HashMap::<TypeName, Request>::new();
        let mut selection = Self::default();

        for entry in entries {
            let (namespace, name, members) = match &entry.kind {
                ResolvedKind::Namespace(_) => continue,
                ResolvedKind::Type { namespace, name } => {
                    if entry.exclude {
                        continue;
                    }
                    (namespace, name, None)
                }
                ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } => (namespace, name, Some(members)),
            };
            let types: Vec<_> = reader.with_full_name(namespace, name).collect();
            let ty = types.first().unwrap();
            let name = ty.type_name();
            selection.explicit.insert(name);
            let request = requests.entry(name).or_default();
            if let Some(members) = members {
                for member in members {
                    let resolved: Vec<_> = types
                        .iter()
                        .flat_map(|ty| resolve_member(reader, ty, member, !entry.exclude))
                        .collect();
                    assert!(
                        !resolved.is_empty(),
                        "{} `{member}` not found on `{name}`",
                        if matches!(ty, Type::Enum(_) | Type::CppEnum(_)) {
                            "variant"
                        } else {
                            "method"
                        }
                    );
                    request.add(resolved, !entry.exclude);
                }
            } else {
                request.whole = true;
            }
        }

        // An unscoped enum's selected variants are independent metadata roots. Their explicit
        // rules also preserve a named enum selection inside an excluded namespace.
        for (name, request) in &requests {
            for ty in reader.with_full_name(name.namespace(), name.name()) {
                if let Type::CppEnum(enumeration) = ty
                    && !enumeration.def.has_attribute("ScopedEnumAttribute")
                {
                    for field in enumeration.def.fields().filter(|field| {
                        field.flags().contains(FieldAttributes::Literal)
                            && request.includes(Member::Variant(field.name()))
                    }) {
                        rules
                            .0
                            .push((format!("{}.{}", name.namespace(), field.name()), true));
                    }
                }
            }
        }
        rules.sort();

        for namespace in reader.namespaces() {
            if !rules.applies_to_namespace(namespace) {
                continue;
            }
            for name in reader.type_names(namespace) {
                if rules.excludes(name) {
                    selection.excluded.insert(name);
                }
                if let Some(rule) = rules.includes(name)
                    && rule.len() <= namespace.len()
                    && !references
                        .matching_rule(name)
                        .is_some_and(|reference| reference.len() > rule.len())
                {
                    requests.entry(name).or_default().whole = true;
                }
            }
        }

        for (name, request) in &requests {
            if selection.excluded.contains(name) {
                continue;
            }
            selection.roots.insert(*name);
            for ty in reader.with_full_name(name.namespace(), name.name()) {
                if request.whole {
                    selection.select_hierarchy(reader, &ty);
                }
                match &ty {
                    Type::Class(class) => {
                        if request.includes(Member::Constructor) {
                            selection.constructors.insert(*name);
                        }
                        let methods = class
                            .required_interfaces(reader)
                            .into_iter()
                            .flat_map(|interface| interface.def.methods())
                            .filter(|method| request.includes(Member::Method(*method)))
                            .collect::<HashSet<_>>();
                        for method in &methods {
                            if request.keep.contains(&Member::Method(*method)) {
                                selection.hierarchy.insert((*name, method_owner(*method)));
                            }
                        }
                        if request.whole || !request.keep.is_empty() || !request.drop.is_empty() {
                            selection.methods.entry(*name).or_default().extend(methods);
                        }
                    }
                    Type::Interface(_) | Type::CppInterface(_) | Type::Delegate(_) => {
                        if request.whole || !request.keep.is_empty() || !request.drop.is_empty() {
                            selection.methods.entry(*name).or_default().extend(
                                type_methods(&ty)
                                    .into_iter()
                                    .filter(|method| request.includes(Member::Method(*method))),
                            );
                        }
                    }
                    Type::Enum(enumeration) => {
                        selection.select_variants(*name, enumeration.def, request);
                    }
                    Type::CppEnum(enumeration) => {
                        selection.select_variants(*name, enumeration.def, request);
                        if !enumeration.def.has_attribute("ScopedEnumAttribute") {
                            for field in enumeration.def.fields().filter(|field| {
                                field.flags().contains(FieldAttributes::Literal)
                                    && request.includes(Member::Variant(field.name()))
                            }) {
                                selection
                                    .roots
                                    .insert(TypeName(name.namespace(), field.name()));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Each class view keeps its own method selection. Local interfaces receive the union
        // needed to implement those views; external interfaces retain their existing ownership.
        let routed: Vec<_> = selection
            .methods
            .iter()
            .filter(|(name, _)| {
                matches!(
                    reader.with_full_name(name.namespace(), name.name()).next(),
                    Some(Type::Class(_))
                )
            })
            .flat_map(|(_, methods)| methods.iter().copied())
            .collect();
        for method in routed {
            let owner = method_owner(method);
            if selection.excludes_type_name(owner)
                || (references.contains(owner).is_some() && !selection.roots.contains(&owner))
                || requests
                    .get(&owner)
                    .is_some_and(|request| request.drop.contains(&Member::Method(method)))
            {
                continue;
            }
            selection.roots.insert(owner);
            selection.methods.entry(owner).or_default().insert(method);
        }

        selection
    }

    fn select_variants(&mut self, name: TypeName, def: TypeDef, request: &Request) {
        self.variants.entry(name).or_default().0.extend(
            def.fields()
                .filter(|field| {
                    field.flags().contains(FieldAttributes::Literal)
                        && request.includes(Member::Variant(field.name()))
                })
                .map(|field| field.name()),
        );
    }

    fn select_hierarchy(&mut self, reader: &Reader, ty: &Type) {
        self.full_hierarchy.insert(ty.type_name());
        if let Type::Class(class) = ty {
            for base in class.bases(reader) {
                self.full_hierarchy.insert(base.type_name());
            }
        }
    }

    pub fn excludes_type_name(&self, name: TypeName) -> bool {
        self.excluded.contains(&name)
    }

    pub fn selects_type(&self, name: TypeName) -> bool {
        self.roots.contains(&name)
    }

    pub fn names_type(&self, namespace: &'static str, name: &'static str) -> bool {
        self.explicit.contains(&TypeName(namespace, name))
    }

    pub fn is_shell(&self, name: TypeName) -> bool {
        !self.methods.contains_key(&name)
    }

    pub fn includes_method(&self, name: TypeName, method: MethodDef) -> bool {
        self.methods
            .get(&name)
            .is_some_and(|methods| methods.contains(&method))
    }

    pub fn enum_variant_filter(
        &self,
        namespace: &'static str,
        name: &'static str,
    ) -> Option<&VariantSelection> {
        self.variants.get(&TypeName(namespace, name))
    }

    pub fn is_activatable(&self, namespace: &'static str, name: &'static str) -> bool {
        self.constructors.contains(&TypeName(namespace, name))
    }

    pub fn includes_full_hierarchy(&self, namespace: &'static str, name: &'static str) -> bool {
        self.full_hierarchy.contains(&TypeName(namespace, name))
    }

    pub fn includes_hierarchy(
        &self,
        namespace: &'static str,
        name: &'static str,
        interface: &Interface,
    ) -> bool {
        self.includes_full_hierarchy(namespace, name)
            || self
                .hierarchy
                .contains(&(TypeName(namespace, name), interface.type_name()))
    }
}

fn type_methods(ty: &Type) -> Vec<MethodDef> {
    let def = match ty {
        Type::Interface(ty) => ty.def,
        Type::CppInterface(ty) => ty.def,
        Type::Delegate(ty) => ty.def,
        _ => unreachable!(),
    };
    def.methods().collect()
}

fn method_owner(method: MethodDef) -> TypeName {
    let MemberRefParent::TypeDef(owner) = method.parent() else {
        unreachable!();
    };
    owner.type_name()
}

#[track_caller]
fn resolve_member(reader: &Reader, ty: &Type, member: &str, include: bool) -> Vec<Member> {
    match ty {
        Type::Enum(enumeration) => resolve_variant(enumeration.def, member),
        Type::CppEnum(enumeration) => resolve_variant(enumeration.def, member),
        Type::Class(class) => {
            let interfaces = class.required_interfaces(reader);
            if member == "CreateInstance" {
                let mut members = vec![Member::Constructor];
                for interface in interfaces
                    .iter()
                    .filter(|interface| interface.kind == InterfaceKind::Composable)
                {
                    members.extend(
                        resolve_methods(
                            &interface.def.methods().collect::<Vec<_>>(),
                            member,
                            include,
                        )
                        .into_iter()
                        .map(Member::Method),
                    );
                }
                return members;
            }
            for interface in interfaces {
                let methods = resolve_methods(
                    &interface.def.methods().collect::<Vec<_>>(),
                    member,
                    include,
                );
                if !methods.is_empty() {
                    return methods.into_iter().map(Member::Method).collect();
                }
            }
            Vec::new()
        }
        Type::Interface(_) | Type::CppInterface(_) | Type::Delegate(_) => {
            let methods = resolve_methods(&type_methods(ty), member, include);
            methods.into_iter().map(Member::Method).collect()
        }
        _ => panic!(
            "type `{}` does not support member-level filtering",
            ty.type_name()
        ),
    }
}

fn resolve_variant(def: TypeDef, name: &str) -> Vec<Member> {
    let field = def
        .fields()
        .find(|field| field.flags().contains(FieldAttributes::Literal) && field.name() == name)
        .map(|field| Member::Variant(field.name()));
    field.into_iter().collect()
}

fn resolve_methods(defs: &[MethodDef], member: &str, include: bool) -> Vec<MethodDef> {
    let mut names = expand_method_part(member, defs);
    if include {
        names.extend(names.clone().iter().filter_map(|name| {
            name.strip_prefix("add_")
                .map(|name| format!("remove_{name}"))
                .filter(|name| defs.iter().any(|def| def.name() == name))
        }));
    }
    defs.iter()
        .copied()
        .filter(|method| {
            let name = method_overload_name(*method);
            names
                .iter()
                .any(|selected| selected == name.as_deref().unwrap_or(method.name()))
        })
        .collect()
}

fn expand_method_part(member: &str, defs: &[MethodDef]) -> Vec<String> {
    if defs.iter().any(|method| method.name() == member) {
        return vec![member.to_string()];
    }
    for prefixes in [["get_", "put_"], ["add_", "remove_"]] {
        let names: Vec<_> = prefixes
            .into_iter()
            .map(|prefix| format!("{prefix}{member}"))
            .filter(|name| defs.iter().any(|method| method.name() == name))
            .collect();
        if !names.is_empty() {
            return names;
        }
    }
    if defs
        .iter()
        .any(|method| method_overload_name(*method).as_deref() == Some(member))
    {
        return vec![member.to_string()];
    }
    Vec::new()
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace_starts_with(namespace, rule);
    }
    rule.starts_with(namespace)
        && rule.as_bytes()[namespace.len()] == b'.'
        && name == &rule[namespace.len() + 1..]
}
