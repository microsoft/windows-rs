use super::*;

/// Collects definitions without changing the requested callable surface.
pub struct TypeClosure<'a> {
    reader: &'a Reader,
    filter: &'a Filter,
    references: &'a References,
    implements: Option<&'a Implements>,
    style: Style,
    types: TypeMap,
    pending: Vec<(Type, i32)>,
}

impl<'a> TypeClosure<'a> {
    pub fn build(
        reader: &'a Reader,
        filter: &'a Filter,
        references: &'a References,
        implements: Option<&'a Implements>,
        style: Style,
    ) -> TypeMap {
        let mut result = Self {
            reader,
            filter,
            references,
            implements,
            style,
            types: TypeMap::new(),
            pending: Vec::new(),
        };
        for name in filter.selected_types() {
            for ty in reader.types(name).iter() {
                result.reach(ty, ALL_ARCHES);
            }
        }
        while let Some((ty, arches)) = result.pending.pop() {
            result.definition(&ty, arches);
        }
        result.types
    }

    fn reach(&mut self, ty: &Type, arches: i32) {
        ty.visit_names(self.reader, Style::Full, &mut |ty| {
            self.reach_definition(ty, arches);
            true
        });
    }

    fn reach_definition(&mut self, ty: &Type, arches: i32) {
        let name = ty.type_name();
        if self.references.owns(name, &self.filter.rules) || self.filter.excludes_type_name(name) {
            return;
        }
        let variants = if matches!(
            ty,
            Type::CppStruct(_)
                | Type::CppDelegate(_)
                | Type::CppEnum(_)
                | Type::CppFn(_)
                | Type::CppInterface(_)
        ) {
            self.reader
                .with_full_name(name.namespace(), name.name())
                .collect()
        } else {
            vec![ty.clone()]
        };
        for definition in variants {
            // Only direct requests may retain definitions with excluded dependencies.
            if self.filter.rules.has_exclusions()
                && !self.filter.is_explicit(name)
                && !Eligibility::check(
                    &definition,
                    self.reader,
                    self.filter,
                    self.references,
                    self.style,
                )
            {
                continue;
            }
            let reached = arches & arch_mask(definition.arches());
            if reached != 0 && self.types.insert_reached(definition.clone(), reached) {
                self.pending.push((definition, reached));
            }
        }
    }

    fn dependency(&mut self, ty: &Type, arches: i32) {
        // Sys signatures erase interface values, not explicitly requested definitions or bases.
        ty.visit_names(self.reader, self.style, &mut |ty| {
            self.reach_definition(ty, arches);
            true
        });
    }

    fn abi(&mut self, ty: &Type, arches: i32) {
        if ty.is_interface_with_reader(self.reader) {
            self.reach(&ty.projection_type(self.reader), arches);
        } else {
            match ty {
                Type::PtrMut(inner, _) | Type::PtrConst(inner, _) | Type::ArrayFixed(inner, _) => {
                    self.abi(inner, arches);
                }
                _ => self.reach(ty, arches),
            }
        }
    }

    fn method(&mut self, method: MethodDef, generics: &[Type], arches: i32) {
        for ty in method.associated_enums(self.reader) {
            self.reach(&ty, arches);
        }
        let signature = method.method_signature(generics, self.reader);
        for ty in signature.types() {
            self.dependency(ty, arches);
        }
    }

    fn interface(&mut self, def: TypeDef, generics: &[Type], arches: i32) {
        let name = def.type_name();
        let implemented = self
            .implements
            .is_some_and(|implements| implements.matches(name));
        for method in def.methods() {
            if method
                .find_attribute("NativeOverrideSlotAttribute")
                .is_none()
                && (implemented || self.filter.includes_method(name, method))
            {
                self.method(method, generics, arches);
            }
        }
    }

    fn definition(&mut self, ty: &Type, arches: i32) {
        match ty {
            Type::Struct(_) | Type::CppStruct(_) | Type::CppDelegate(_) => {
                ty.visit_dependencies(self.reader, &mut |ty| {
                    self.dependency(ty, arches);
                    true
                });
            }
            Type::CppEnum(e) if !e.def.has_attribute("ScopedEnumAttribute") => {
                if self.filter.is_selected(e.def.type_name()) {
                    for field in e
                        .def
                        .fields()
                        .filter(|field| field.flags().contains(FieldAttributes::Literal))
                    {
                        if self
                            .filter
                            .includes_variant(e.def.type_name(), field.name())
                        {
                            for constant in
                                self.reader.with_full_name(e.def.namespace(), field.name())
                            {
                                self.reach(&constant, arches);
                            }
                        }
                    }
                }
            }
            Type::Delegate(d) => {
                for method in d.def.methods().filter(|method| method.name() == "Invoke") {
                    self.method(method, &d.generics, arches);
                }
            }
            Type::Interface(interface) => {
                self.reach(&Type::Object, arches);
                for required in interface.required_interfaces(self.reader) {
                    self.reach(&Type::Interface(required), arches);
                }
                self.interface(interface.def, &interface.generics, arches);
            }
            Type::CppInterface(interface) => {
                for base in interface.base_interfaces(self.reader) {
                    self.reach(&base, arches);
                }
                self.interface(interface.def, &[], arches);
            }
            Type::CppFn(function) => {
                if !self.style.is_minimal() {
                    function.visit_dependencies(self.reader, &mut |ty| {
                        self.dependency(ty, arches);
                        true
                    });
                    return;
                }
                for ty in function.method.associated_enums(self.reader) {
                    self.reach(&ty, arches);
                }
                let signature = function.method.method_signature(&[], self.reader);
                self.dependency(&signature.return_type, arches);
                for param in &signature.params {
                    self.abi(&param.ty, arches);
                }
                if let Some(name) = function.window_long_dependency(self.reader) {
                    self.reach(
                        &self.reader.unwrap_full_name(function.namespace, name),
                        arches,
                    );
                }
            }
            Type::CppConst(constant) => {
                self.dependency(&constant.dependency_type(self.reader), arches);
            }
            Type::Class(class) => {
                for interface in class.required_interfaces(self.reader) {
                    if interface.kind == InterfaceKind::Default
                        || (!interface.is_exclusive()
                            && matches!(interface.kind, InterfaceKind::None | InterfaceKind::Base))
                    {
                        self.reach(&Type::Interface(interface), arches);
                    }
                }
                for base in class.bases(self.reader) {
                    self.reach(&Type::Class(base), arches);
                }
            }
            Type::IUnknown => {
                self.reach(&Type::GUID, arches);
                self.reach(&Type::HRESULT, arches);
            }
            Type::Object => self.reach(&Type::IUnknown, arches),
            _ => {}
        }
    }
}

struct Eligibility<'a> {
    reader: &'a Reader,
    filter: &'a Filter,
    references: &'a References,
    style: Style,
    opaque_references: bool,
    seen: HashSet<Type>,
}

impl Eligibility<'_> {
    fn check(
        ty: &Type,
        reader: &Reader,
        filter: &Filter,
        references: &References,
        style: Style,
    ) -> bool {
        // Interface definitions require their bases even when their signatures erase interfaces.
        let style = if matches!(
            ty,
            Type::CppFn(_) | Type::CppStruct(_) | Type::CppDelegate(_) | Type::Struct(_)
        ) {
            style
        } else {
            Style::Full
        };
        let mut check = Eligibility {
            reader,
            filter,
            references,
            style,
            // Locally generated reference types must validate their definitions, not stop at them.
            opaque_references: references.contains(ty.type_name()).is_none(),
            seen: HashSet::new(),
        };
        if matches!(ty, Type::CppFn(_)) {
            // Check this function variant, not its architecture siblings.
            ty.visit_dependencies(reader, &mut |ty| check.signature(ty))
        } else {
            check.signature(ty)
        }
    }

    fn signature(&mut self, ty: &Type) -> bool {
        ty.visit_names(self.reader, self.style, &mut |ty| self.definition(ty))
    }

    fn definition(&mut self, ty: &Type) -> bool {
        let name = ty.type_name();
        let referenced = self.references.contains(name).is_some();
        if !referenced
            && self.filter.excludes_type_name(name)
            && !matches!(ty, Type::CppConst(constant) if constant.is_enum_member)
        {
            return false;
        }
        if (self.opaque_references && referenced) || !self.seen.insert(ty.clone()) {
            return true;
        }
        ty.visit_variants(self.reader, &mut |ty| self.definition(ty))
            && ty.visit_dependencies(self.reader, &mut |ty| self.signature(ty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn types(entries: &[&str]) -> TypeMap {
        let reader = default_reader(false);
        let entries: Vec<_> = entries
            .iter()
            .flat_map(|entry| filter_parser::parse_filter_entry(entry))
            .collect();
        let entries = filter_parser::resolve_entries(reader, &entries);
        let references = References::default();
        let filter = Filter::from_resolved(reader, &entries);
        TypeClosure::build(reader, &filter, &references, None, Style::Full)
    }

    #[test]
    fn precise_type_exclusions() {
        for name in [
            "GetTickCount64",
            "DateTime",
            "Uri",
            "IUriRuntimeClass",
            "IUriRuntimeClass::AbsoluteUri",
            "IUriRuntimeClass::{}",
        ] {
            let owner = name.split("::").next().unwrap();
            let exclusion = format!("!{owner}");
            for entries in [[name, exclusion.as_str()], [exclusion.as_str(), name]] {
                let types = types(&["GetTickCount", entries[0], entries[1]]);
                assert!(!types.keys().any(|name| name.name() == owner));
                assert!(types.contains_key(&TypeName("Windows.Win32", "GetTickCount")));
            }
        }
        let types = types(&[
            "Windows.Win32::{GetTickCount,GetTickCount64}",
            "!Windows.Win32::{GetTickCount64}",
        ]);
        assert!(types.contains_key(&TypeName("Windows.Win32", "GetTickCount")));
        assert!(!types.contains_key(&TypeName("Windows.Win32", "GetTickCount64")));
    }

    #[test]
    fn precise_type_selection_overrides_namespace_exclusion() {
        for entries in [
            ["Windows.Foundation.DateTime", "!Windows.Foundation"],
            ["!Windows.Foundation", "Windows.Foundation.DateTime"],
        ] {
            assert!(types(&entries).contains_key(&TypeName("Windows.Foundation", "DateTime")));
        }
    }
}
