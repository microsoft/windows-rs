use super::*;

/// Rewrites the namespaces of a flat winmd into a header-based partition.
///
/// The canonical Win32/WDK winmds live in a single flat namespace (`Windows.Win32`) because
/// the Windows SDK is a flat C namespace with globally-unique symbol names. That is the right
/// shape for every flat/minimal consumer, but `windows-bindgen --package` derives both the
/// file layout and the per-namespace Cargo features from namespaces. This remapper is the
/// "optional downstream map over the flat namespace": given a `name -> namespace` routing map
/// (built upstream from the per-header `.rdl` partition), it reads the flat winmd(s) and emits
/// a namespaced winmd suitable for `--package`.
///
/// The single flat `Apis` container (which holds every function and constant) is **split** into
/// one `Apis` container per target namespace, routing each member by name. Every embedded type
/// reference (`extends`, field types, method signatures, and interface impls) is rewritten
/// through the same map. References whose name is not in the map (external types, the
/// `Windows.Win32.Metadata` attribute types) pass through unchanged.
#[derive(Default)]
pub struct Remapper {
    input: Vec<String>,
    output: String,
    /// Type/function/constant name -> target namespace.
    routes: HashMap<String, String>,
    /// Namespaces whose members are subject to remapping (e.g. `Windows.Win32`). References in
    /// any other namespace are copied verbatim.
    sources: Vec<String>,
    /// Namespace used for a source-namespace member whose name is absent from `routes`.
    fallback: String,
}

impl Remapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    /// Registers a namespace whose members are remapped. Members of any namespace not listed
    /// here are copied without change.
    pub fn source(&mut self, namespace: &str) -> &mut Self {
        self.sources.push(namespace.to_string());
        self
    }

    /// Sets the target namespace for a member whose name is missing from the route map.
    pub fn fallback(&mut self, namespace: &str) -> &mut Self {
        self.fallback = namespace.to_string();
        self
    }

    /// Bulk-registers `name -> namespace` routes.
    pub fn routes<I, K, V>(&mut self, routes: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (name, namespace) in routes {
            self.routes.insert(name.into(), namespace.into());
        }
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn remap(&self) -> Result<(), Error> {
        if self.output.is_empty() {
            return Err(Error::new("output is required"));
        }

        let output_path = std::path::Path::new(&self.output);
        let name = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::new(format!("invalid output path `{}`", self.output)))?;

        let files = read_inputs(&self.input)?;
        let index = reader::Index::new(files);
        let mut file = writer::File::new(name);

        // Collect the flat `Apis` container(s) to split after the regular types. Every other
        // type is written directly with its namespace and references remapped.
        let mut apis: Vec<reader::TypeDef> = Vec::new();
        let mut types: Vec<reader::TypeDef> = index.types().collect();
        types.sort_by(|a, b| (a.namespace(), a.name()).cmp(&(b.namespace(), b.name())));

        for ty in types {
            if self.is_source_apis(ty) {
                apis.push(ty);
            } else {
                self.write_type(&mut file, &index, ty, None);
            }
        }

        self.split_apis(&mut file, &apis);

        let bytes = file.into_stream();
        std::fs::write(&self.output, bytes)
            .map_err(|e| Error::new(format!("failed to write `{}`: {e}", self.output)))
    }

    fn is_source_apis(&self, ty: reader::TypeDef) -> bool {
        !ty.flags().contains(TypeAttributes::WindowsRuntime)
            && ty.category() == reader::TypeCategory::Class
            && ty.name() == "Apis"
            && self.sources.iter().any(|s| s == ty.namespace())
    }

    /// Target namespace for a `(namespace, name)` reference. Only names in a source namespace are
    /// remapped; anything else keeps its namespace.
    fn target(&self, namespace: &str, name: &str) -> String {
        if self.sources.iter().any(|s| s == namespace) {
            self.routes
                .get(trim_tick(name))
                .cloned()
                .unwrap_or_else(|| self.fallback.clone())
        } else {
            namespace.to_string()
        }
    }

    fn remap_type(&self, ty: &Type) -> Type {
        match ty {
            Type::ClassName(tn) => Type::ClassName(self.remap_type_name(tn)),
            Type::ValueName(tn) => Type::ValueName(self.remap_type_name(tn)),
            Type::Array(inner) => Type::Array(Box::new(self.remap_type(inner))),
            Type::RefMut(inner) => Type::RefMut(Box::new(self.remap_type(inner))),
            Type::RefConst(inner) => Type::RefConst(Box::new(self.remap_type(inner))),
            Type::PtrMut(inner, n) => Type::PtrMut(Box::new(self.remap_type(inner)), *n),
            Type::PtrConst(inner, n) => Type::PtrConst(Box::new(self.remap_type(inner)), *n),
            Type::ArrayFixed(inner, n) => Type::ArrayFixed(Box::new(self.remap_type(inner)), *n),
            other => other.clone(),
        }
    }

    fn remap_type_name(&self, tn: &TypeName) -> TypeName {
        TypeName {
            namespace: self.target(&tn.namespace, &tn.name),
            name: tn.name.clone(),
            generics: tn.generics.iter().map(|g| self.remap_type(g)).collect(),
        }
    }

    fn remap_signature(&self, sig: &Signature) -> Signature {
        Signature {
            flags: sig.flags,
            return_type: self.remap_type(&sig.return_type),
            types: sig.types.iter().map(|t| self.remap_type(t)).collect(),
        }
    }

    fn remap_extends(&self, file: &mut writer::File, def: reader::TypeDef) -> writer::TypeDefOrRef {
        def.extends()
            .map(|extends| {
                let namespace = self.target(extends.namespace(), extends.name());
                writer::TypeDefOrRef::TypeRef(file.TypeRef(&namespace, extends.name()))
            })
            .unwrap_or_default()
    }

    /// Writes a single (non-`Apis`) type into `namespace`-remapped form, mirroring the ordering
    /// invariants of `merge::write_type` (a `TypeDef`'s fields and methods must be emitted
    /// contiguously right after it).
    fn write_type(
        &self,
        file: &mut writer::File,
        index: &reader::Index,
        def: reader::TypeDef,
        outer: Option<writer::TypeDef>,
    ) {
        let extends = self.remap_extends(file, def);
        let namespace = if def.flags().is_nested() {
            String::new()
        } else {
            self.target(def.namespace(), def.name())
        };

        let type_def = file.TypeDef(&namespace, def.name(), extends, def.flags());

        if let Some(outer) = outer {
            file.NestedClass(type_def, outer);
        }

        for field in def.fields() {
            let field_def = file.Field(field.name(), &self.remap_type(&field.ty()), field.flags());
            if let Some(constant) = field.constant() {
                file.Constant(writer::HasConstant::Field(field_def), &constant.value());
            }
            write_attributes(file, writer::HasAttribute::Field(field_def), field);
        }

        let generics: Vec<_> = def
            .generic_params()
            .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
            .collect();

        write_attributes(file, writer::HasAttribute::TypeDef(type_def), def);

        for map in def.interface_impls() {
            let interface = self.remap_type(&map.interface(&generics));
            let interface_impl = file.InterfaceImpl(type_def, &interface);
            write_attributes(
                file,
                writer::HasAttribute::InterfaceImpl(interface_impl),
                map,
            );
        }

        for generic in def.generic_params() {
            file.GenericParam(
                generic.name(),
                writer::TypeOrMethodDef::TypeDef(type_def),
                generic.sequence(),
                generic.flags(),
            );
        }

        let is_winrt_class = def.category() == reader::TypeCategory::Class
            && def.flags().contains(TypeAttributes::WindowsRuntime);

        if !is_winrt_class {
            for method in def.methods() {
                self.write_method(file, method, &generics);
            }
        }

        if let Some(class_layout) = def.class_layout() {
            file.ClassLayout(
                type_def,
                class_layout.packing_size(),
                class_layout.class_size(),
            );
        }

        for inner_def in index.nested(def) {
            self.write_type(file, index, inner_def, Some(type_def));
        }
    }

    fn write_method(&self, file: &mut writer::File, method: reader::MethodDef, generics: &[Type]) {
        let signature = self.remap_signature(&method.signature(generics));
        let method_def = file.MethodDef(
            method.name(),
            &signature,
            method.flags(),
            method.impl_flags(),
        );
        for param_def in method.params() {
            let param = file.Param(param_def.name(), param_def.sequence(), param_def.flags());
            write_attributes(file, writer::HasAttribute::Param(param), param_def);
        }
        write_attributes(file, writer::HasAttribute::MethodDef(method_def), method);
        if let Some(impl_map) = method.impl_map() {
            file.ImplMap(
                method_def,
                impl_map.flags(),
                impl_map.import_name(),
                impl_map.import_scope().name(),
            );
        }
    }

    /// Splits the flat `Apis` container(s) into one `Apis` per target namespace. Each member
    /// (constant field or function method) is routed by name; the per-namespace containers are
    /// emitted so that each `TypeDef`'s fields and methods stay contiguous.
    fn split_apis(&self, file: &mut writer::File, apis: &[reader::TypeDef]) {
        // Route every member to a target namespace, preserving encounter order per namespace.
        let mut namespaces: Vec<String> = Vec::new();
        let mut fields: HashMap<String, Vec<reader::Field>> = HashMap::new();
        let mut methods: HashMap<String, Vec<reader::MethodDef>> = HashMap::new();

        let record = |namespaces: &mut Vec<String>, namespace: String| {
            if !namespaces.contains(&namespace) {
                namespaces.push(namespace);
            }
        };

        // A representative `Apis` type per namespace to copy flags/extends from.
        let template = apis.first().copied();

        for &container in apis {
            for field in container.fields() {
                let namespace = self.target(container.namespace(), field.name());
                record(&mut namespaces, namespace.clone());
                fields.entry(namespace).or_default().push(field);
            }
            for method in container.methods() {
                let namespace = self.target(container.namespace(), method.name());
                record(&mut namespaces, namespace.clone());
                methods.entry(namespace).or_default().push(method);
            }
        }

        namespaces.sort();

        let Some(template) = template else { return };

        for namespace in &namespaces {
            let extends = self.remap_extends(file, template);
            let type_def = file.TypeDef(namespace, "Apis", extends, template.flags());

            for field in fields.get(namespace).into_iter().flatten() {
                let field_def =
                    file.Field(field.name(), &self.remap_type(&field.ty()), field.flags());
                if let Some(constant) = field.constant() {
                    file.Constant(writer::HasConstant::Field(field_def), &constant.value());
                }
                write_attributes(file, writer::HasAttribute::Field(field_def), *field);
            }

            for method in methods.get(namespace).into_iter().flatten() {
                self.write_method(file, *method, &[]);
            }

            write_attributes(file, writer::HasAttribute::TypeDef(type_def), template);
        }
    }
}
