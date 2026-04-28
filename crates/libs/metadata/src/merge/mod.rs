use super::*;

pub struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Error(message.into())
    }
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nerror: {}", self.0)
    }
}

#[derive(Default)]
pub struct Merger {
    input: Vec<String>,
    /// Arch-tagged inputs: `(path, arch_bits)` where arch_bits is a bitmask
    /// (1=X86, 2=X64, 4=Arm64) indicating which architecture the file targets.
    arch_inputs: Vec<(String, i32)>,
    output: String,
}

impl Merger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input.push(input.as_ref().to_string());
        }
        self
    }

    /// Adds an architecture-tagged input winmd file.
    ///
    /// `arch` is a bitmask indicating which architecture this file was built for:
    ///   - `1` → X86
    ///   - `2` → X64
    ///   - `4` → Arm64
    ///
    /// When `merge()` is called, types present in **all** arch-tagged inputs get
    /// no `SupportedArchitectureAttribute`; types present only in a **subset** get
    /// `SupportedArchitectureAttribute(present_arch_mask)`.
    pub fn arch_input(&mut self, path: &str, arch: i32) -> &mut Self {
        self.arch_inputs.push((path.to_string(), arch));
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn merge(&self) -> Result<(), Error> {
        if self.output.is_empty() {
            return Err(Error::new("output is required"));
        }

        let output_path = std::path::Path::new(&self.output);
        let name = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::new(format!("invalid output path `{}`", self.output)))?;

        let files = read_inputs(&self.input)?;
        let index = reader::TypeIndex::new(files);

        let mut file = writer::File::new(name);

        // Write plain (untagged) inputs as-is.
        let mut types: Vec<reader::TypeDef<'_>> = index.types().collect();
        types.sort_by(|a, b| (a.namespace(), a.name()).cmp(&(b.namespace(), b.name())));

        for ty in types {
            write_type(&mut file, &index, ty, None, None);
        }

        // Write arch-tagged inputs with computed SupportedArchitecture annotations.
        if !self.arch_inputs.is_empty() {
            // Compute the bitmask for "all arches present in this merge run".
            let all_arches_mask: i32 = self.arch_inputs.iter().fold(0, |acc, (_, arch)| acc | arch);

            // Load each arch-tagged file group.
            let mut arch_groups: Vec<(reader::TypeIndex, i32)> =
                Vec::with_capacity(self.arch_inputs.len());
            for (path, arch_bits) in &self.arch_inputs {
                let files = read_inputs(std::slice::from_ref(path))?;
                arch_groups.push((reader::TypeIndex::new(files), *arch_bits));
            }

            // Compute the union of arch bits for each (namespace, name) pair.
            let mut arch_presence: HashMap<(String, String), i32> = HashMap::new();
            for (idx, arch_bits) in &arch_groups {
                for ty in idx.types() {
                    *arch_presence
                        .entry((ty.namespace().to_string(), ty.name().to_string()))
                        .or_default() |= arch_bits;
                }
            }

            // Build a flat list of (TypeIndex ref, TypeDef, arch_bits) sorted by (ns, name).
            let mut all_type_refs: Vec<(&reader::TypeIndex, reader::TypeDef<'_>, i32)> = Vec::new();
            for (idx, arch_bits) in &arch_groups {
                for ty in idx.types() {
                    all_type_refs.push((idx, ty, *arch_bits));
                }
            }
            all_type_refs
                .sort_by(|a, b| (a.1.namespace(), a.1.name()).cmp(&(b.1.namespace(), b.1.name())));

            // Write each type with the appropriate arch annotation.
            // For types present in all arches, deduplicate to a single arch-neutral TypeDef.
            // For types present in a subset, write each copy with SupportedArchitecture.
            let mut deduped: HashSet<(String, String)> = HashSet::new();
            for (idx, ty, _arch_bits) in &all_type_refs {
                let key = (ty.namespace().to_string(), ty.name().to_string());
                let present_mask = arch_presence.get(&key).copied().unwrap_or(0);

                let arch_override = if present_mask == all_arches_mask {
                    // Present on every arch: write once without SupportedArchitecture.
                    if !deduped.insert(key) {
                        continue; // Already written.
                    }
                    Some(0) // 0 = suppress arch attribute
                } else {
                    // Present on a subset: annotate with the union of arches that have it.
                    Some(present_mask)
                };

                write_type(&mut file, idx, *ty, None, arch_override);
            }
        }

        let bytes = file.into_stream();
        std::fs::write(&self.output, bytes)
            .map_err(|e| Error::new(format!("failed to write `{}`: {e}", self.output)))
    }
}

fn read_inputs(inputs: &[String]) -> Result<Vec<reader::File>, Error> {
    let mut result = vec![];

    for input in inputs {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            let entries = std::fs::read_dir(path)
                .map_err(|e| Error::new(format!("failed to read directory `{input}`: {e}")))?;

            for entry in entries.flatten() {
                let entry_path = entry.path();

                if entry_path.is_file()
                    && entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
                {
                    let path_str = entry_path.to_string_lossy().to_string();
                    let file = reader::File::read(&entry_path)
                        .ok_or_else(|| Error::new(format!("failed to read `{path_str}`")))?;
                    result.push(file);
                }
            }

            if result.len() == prev_len {
                return Err(Error::new(format!(
                    "no .winmd files found in directory `{input}`"
                )));
            }
        } else {
            let file = reader::File::read(path)
                .ok_or_else(|| Error::new(format!("failed to read `{input}`")))?;
            result.push(file);
        }
    }

    Ok(result)
}

/// Writes a `TypeDef` (and its nested types) from `index` into `file`.
///
/// `arch_override` controls the `SupportedArchitectureAttribute` on the TypeDef:
///   - `None`     → copy attributes as-is (plain merge, no arch logic)
///   - `Some(0)`  → drop any existing `SupportedArchitectureAttribute`
///   - `Some(n)`  → drop existing and write `SupportedArchitecture(n)`
fn write_type(
    file: &mut writer::File,
    index: &reader::TypeIndex,
    def: reader::TypeDef,
    outer: Option<writer::TypeDef>,
    arch_override: Option<i32>,
) {
    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(file.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    debug_assert!(
        !def.flags().is_nested() || def.namespace().is_empty(),
        "nested type should have empty namespace"
    );
    debug_assert!(
        def.flags().is_nested() || !def.namespace().is_empty(),
        "non-nested type should have non-empty namespace"
    );

    let type_def = file.TypeDef(def.namespace(), def.name(), extends, def.flags());

    if let Some(outer) = outer {
        file.NestedClass(type_def, outer);
    }

    for field in def.fields() {
        let field_def = file.Field(field.name(), &field.ty(), field.flags());

        if let Some(constant) = field.constant() {
            file.Constant(writer::HasConstant::Field(field_def), &constant.value());
        }

        write_attributes(file, writer::HasAttribute::Field(field_def), field);
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    write_attributes_with_arch(
        file,
        writer::HasAttribute::TypeDef(type_def),
        def,
        arch_override,
    );

    for map in def.interface_impls() {
        let interface_impl = file.InterfaceImpl(type_def, &map.interface(&generics));
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
            let method_def = file.MethodDef(
                method.name(),
                &method.signature(&generics),
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
    }

    if let Some(class_layout) = def.class_layout() {
        file.ClassLayout(
            type_def,
            class_layout.packing_size(),
            class_layout.class_size(),
        );
    }

    for inner_def in index.nested(def) {
        debug_assert!(inner_def.namespace().is_empty());
        debug_assert!(inner_def.flags().is_nested());
        write_type(file, index, inner_def, Some(type_def), arch_override);
    }
}

fn write_attributes<'a, R: reader::HasAttributes<'a>>(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
) {
    write_attributes_with_arch(file, parent, row, None);
}

/// Like [`write_attributes`] but with optional architecture annotation overriding.
///
/// When `arch_override` is `Some`:
///   - Any existing `SupportedArchitectureAttribute` on `row` is **dropped**.
///   - If `arch_override` is `Some(bits)` with `bits != 0`, a new
///     `SupportedArchitectureAttribute(bits)` is written.
///   - If `arch_override` is `Some(0)`, no arch attribute is written (arch-neutral).
fn write_attributes_with_arch<'a, R: reader::HasAttributes<'a>>(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
    arch_override: Option<i32>,
) {
    for attribute in row.attributes() {
        let ctor = attribute.ctor();
        let ty = ctor.parent();

        // Skip the existing SupportedArchitectureAttribute when we're overriding arch.
        if arch_override.is_some()
            && ty.namespace() == "Windows.Win32.Foundation.Metadata"
            && ty.name() == "SupportedArchitectureAttribute"
        {
            continue;
        }

        let attribute_ref =
            writer::MemberRefParent::TypeRef(file.TypeRef(ty.namespace(), ty.name()));

        let ctor_ref = file.MemberRef(".ctor", &ctor.signature(&[]), attribute_ref);

        file.Attribute(
            parent,
            writer::AttributeType::MemberRef(ctor_ref),
            &attribute.value(),
        );
    }

    // Emit the overriding arch attribute when requested (non-zero bits).
    if let Some(arch_bits) = arch_override {
        if arch_bits != 0 {
            write_supported_architecture_attr(file, parent, arch_bits);
        }
    }
}

/// Writes a `SupportedArchitectureAttribute` with the given `arch_bits` bitmask.
fn write_supported_architecture_attr(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    arch_bits: i32,
) {
    let ns = "Windows.Win32.Foundation.Metadata";
    let name = "SupportedArchitectureAttribute";

    let type_ref = writer::MemberRefParent::TypeRef(file.TypeRef(ns, name));

    let sig = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![Type::I32],
    };

    let ctor_ref = file.MemberRef(".ctor", &sig, type_ref);

    file.Attribute(
        parent,
        writer::AttributeType::MemberRef(ctor_ref),
        &[("".to_string(), Value::I32(arch_bits))],
    );
}
