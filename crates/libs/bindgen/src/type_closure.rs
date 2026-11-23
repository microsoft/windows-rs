use super::*;

/// Collects definitions without changing the requested callable surface.
pub struct TypeClosure<'a> {
    reader: &'a Reader,
    filter: &'a Filter,
    references: &'a References,
    implements: Option<&'a Implements>,
    style: Style,
    publication: Option<TypeMap>,
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
            publication: filter
                .rules
                .has_exclusions()
                .then(|| TypeMap::with_references(reader, references)),
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
        let ty = ty.dependency_type();
        if let Type::Ptr64(inner, _) = ty {
            self.types.insert(Type::Pointer64Helper);
            self.dependency(inner, arches);
            return;
        }
        if let Type::FnPtr(signature) = ty {
            for ty in signature.types() {
                self.dependency(ty, arches);
            }
            return;
        }
        if ty.is_intrinsic() {
            return;
        }
        if let Some(owner) = ty.enclosing() {
            for ty in self.reader.with_full_name(owner.namespace(), owner.name()) {
                self.reach(&ty, arches);
            }
            return;
        }
        let name = ty.type_name();
        let (definition, arguments) = ty.split_generic(self.reader);
        for argument in &arguments {
            self.reach(argument, arches);
        }
        if self.references.owns(name, &self.filter.rules) || self.filter.excludes_type_name(name) {
            return;
        }
        let variants = if matches!(
            definition,
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
            vec![definition]
        };
        for definition in variants {
            // Only direct requests may retain definitions with excluded dependencies.
            if let Some(publication) = &self.publication
                && !self.filter.is_explicit(name)
            {
                let mut dependencies = if self.references.contains(name).is_some() {
                    TypeMap::new()
                } else {
                    publication.clone()
                };
                // Sys value/function signatures erase interface pointers, but interface
                // definitions still need their base vtables.
                match &definition {
                    Type::CppFn(function) => {
                        if self.style.is_sys() {
                            function.combine_sys(&mut dependencies, self.reader);
                        } else {
                            function.combine(&mut dependencies, self.reader);
                        }
                    }
                    Type::CppStruct(_) | Type::CppDelegate(_) | Type::Struct(_)
                        if self.style.is_sys() =>
                    {
                        definition.combine_sys(&mut dependencies, self.reader);
                    }
                    _ => definition.combine(&mut dependencies, self.reader),
                }
                if dependencies.excluded(self.filter, self.references) {
                    continue;
                }
            }
            let reached = arches & arch_mask(definition.arches());
            if reached != 0 && self.types.insert_reached(definition.clone(), reached) {
                self.pending.push((definition, reached));
            }
        }
    }

    fn dependency(&mut self, ty: &Type, arches: i32) {
        // Sys signatures erase interface values, not explicitly requested definitions or bases.
        if !self.style.is_sys() || !ty.dependency_type().is_interface() {
            self.reach(ty, arches);
        }
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
            Type::Struct(s) => {
                for field in s.def.fields() {
                    self.dependency(&field.field_type(None, self.reader), arches);
                }
            }
            Type::CppStruct(s) => {
                for ty in s.definition_types(self.reader) {
                    self.dependency(&ty, arches);
                }
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
            Type::CppDelegate(d) => {
                for method in d.def.methods().filter(|method| method.name() == "Invoke") {
                    self.method(method, &[], arches);
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
                for ty in function.method.associated_enums(self.reader) {
                    self.reach(&ty, arches);
                }
                let signature = function.method.method_signature(&[], self.reader);
                self.dependency(&signature.return_type, arches);
                for param in &signature.params {
                    if self.style.is_minimal() {
                        self.abi(&param.ty, arches);
                    } else {
                        self.dependency(&param.ty, arches);
                    }
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
