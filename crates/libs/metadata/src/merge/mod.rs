use super::*;

mod remap;
pub use remap::Remapper;

pub struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
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
    ///   - `1` -> X86
    ///   - `2` -> X64
    ///   - `4` -> Arm64
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
        let index = reader::Index::new(files);

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
            let mut arch_groups: Vec<(reader::Index, i32)> =
                Vec::with_capacity(self.arch_inputs.len());
            for (path, arch_bits) in &self.arch_inputs {
                let files = read_inputs(std::slice::from_ref(path))?;
                arch_groups.push((reader::Index::new(files), *arch_bits));
            }

            // Group every arch copy by (namespace, name).
            let mut groups: BTreeMap<
                (String, String),
                Vec<(&reader::Index, reader::TypeDef<'_>, i32)>,
            > = BTreeMap::new();
            for (idx, arch_bits) in &arch_groups {
                for ty in idx.types() {
                    groups
                        .entry((ty.namespace().to_string(), ty.name().to_string()))
                        .or_default()
                        .push((idx, ty, *arch_bits));
                }
            }

            for copies in groups.values() {
                let (idx, ty, _) = copies[0];
                if ty.category() == reader::TypeCategory::Class {
                    // Apis container (constants + functions): union the members so
                    // arch-divergent ones are each kept and tagged. `write_type_arch_merged`
                    // tags a member neutral only when it spans every arch in the run
                    // (`all_arches_mask`), else per-arch.
                    write_type_arch_merged(&mut file, idx, ty, copies, all_arches_mask);
                } else {
                    // A value type. Group the per-arch copies by structural signature and
                    // emit one copy per distinct shape, tagged with the union of the arch
                    // bits of the copies sharing that shape. Arches with an identical shape
                    // (e.g. `ARM64_NT_CONTEXT` is byte-identical on x64 and x86) collapse to
                    // one `SupportedArchitecture(x64|x86)` definition; a shape that diverges
                    // across arches (the CONTEXT class) is split per shape instead of
                    // collapsing to `copies[0]` and dropping the other layout. A group
                    // spanning every arch in the run is written arch-neutral (`Some(0)`);
                    // anything narrower keeps its subset tag.
                    let mut by_sig: Vec<(String, &reader::Index, reader::TypeDef, i32)> = vec![];
                    for (cidx, c, bits) in copies {
                        let sig = type_sig(cidx, *c);
                        if let Some(entry) = by_sig.iter_mut().find(|(s, ..)| *s == sig) {
                            entry.3 |= *bits;
                        } else {
                            by_sig.push((sig, cidx, *c, *bits));
                        }
                    }
                    for (_, cidx, c, bits) in &by_sig {
                        let arch = if *bits == all_arches_mask { 0 } else { *bits };
                        write_type(&mut file, cidx, *c, None, Some(arch));
                    }
                }
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
///   - `None`     -> copy attributes as-is (plain merge, no arch logic)
///   - `Some(0)`  -> drop any existing `SupportedArchitectureAttribute`
///   - `Some(n)`  -> drop existing and write `SupportedArchitecture(n)`
fn write_type(
    file: &mut writer::File,
    index: &reader::Index,
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
        write_field(file, field, None);
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
            write_method(file, method, &generics, None);
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

/// Writes a field (and its constant) with an optional `SupportedArchitecture` override.
fn write_field(file: &mut writer::File, field: reader::Field, arch_override: Option<i32>) {
    let field_def = file.Field(field.name(), &field.ty(), field.flags());
    if let Some(constant) = field.constant() {
        file.Constant(writer::HasConstant::Field(field_def), &constant.value());
    }
    write_attributes_with_arch(
        file,
        writer::HasAttribute::Field(field_def),
        field,
        arch_override,
    );
}

/// Writes a method (params, impl map) with an optional `SupportedArchitecture` override.
fn write_method(
    file: &mut writer::File,
    method: reader::MethodDef,
    generics: &[Type],
    arch_override: Option<i32>,
) {
    let method_def = file.MethodDef(
        method.name(),
        &method.signature(generics),
        method.flags(),
        method.impl_flags(),
    );
    for param_def in method.params() {
        let param = file.Param(param_def.name(), param_def.sequence(), param_def.flags());
        write_attributes(file, writer::HasAttribute::Param(param), param_def);
    }
    write_attributes_with_arch(
        file,
        writer::HasAttribute::MethodDef(method_def),
        method,
        arch_override,
    );
    if let Some(impl_map) = method.impl_map() {
        file.ImplMap(
            method_def,
            impl_map.flags(),
            impl_map.import_name(),
            impl_map.import_scope().name(),
        );
    }
}

/// Writes a TypeDef present on every architecture, unioning its fields and methods across
/// arch copies. Members shared by all arches stay arch-neutral; members present on only a
/// subset (arch-divergent constants like `CONTEXT_ALL`, arch-only functions) are emitted
/// once per distinct definition with a `SupportedArchitecture` tag. This keeps the apis
/// container's per-arch constants/functions instead of dropping all but x64.
fn write_type_arch_merged(
    file: &mut writer::File,
    index: &reader::Index,
    def: reader::TypeDef,
    copies: &[(&reader::Index, reader::TypeDef, i32)],
    all_mask: i32,
) {
    let extends = def
        .extends()
        .map(|e| writer::TypeDefOrRef::TypeRef(file.TypeRef(e.namespace(), e.name())))
        .unwrap_or_default();
    let type_def = file.TypeDef(def.namespace(), def.name(), extends, def.flags());

    let generics: Vec<_> = def
        .generic_params()
        .map(|p| Type::Generic(p.name().to_string(), p.sequence()))
        .collect();

    write_attributes_with_arch(file, writer::HasAttribute::TypeDef(type_def), def, Some(0));
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

    // Union fields by (name, type, value); accumulate the arch bits each distinct field
    // appears on, keeping a representative to re-emit.
    let mut fields: BTreeMap<String, (reader::Field, i32)> = BTreeMap::new();
    for (_, ty, bits) in copies {
        for field in ty.fields() {
            let val = field
                .constant()
                .map(|c| format!("{:?}", c.value()))
                .unwrap_or_default();
            let key = format!("{}|{:?}|{val}", field.name(), field.ty());
            fields.entry(key).or_insert((field, 0)).1 |= bits;
        }
    }
    for (field, bits) in fields.into_values() {
        write_field(file, field, Some(if bits == all_mask { 0 } else { bits }));
    }

    let is_winrt_class = def.category() == reader::TypeCategory::Class
        && def.flags().contains(TypeAttributes::WindowsRuntime);
    if !is_winrt_class {
        let mut methods: BTreeMap<String, (reader::MethodDef, i32)> = BTreeMap::new();
        for (_, ty, bits) in copies {
            for method in ty.methods() {
                let key = format!("{}|{:?}", method.name(), method.signature(&generics));
                methods.entry(key).or_insert((method, 0)).1 |= bits;
            }
        }
        for (method, bits) in methods.into_values() {
            write_method(
                file,
                method,
                &generics,
                Some(if bits == all_mask { 0 } else { bits }),
            );
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
        write_type(file, index, inner_def, Some(type_def), Some(0));
    }
}

/// A structural signature for a value type: its fields (name + type + constant value),
/// packing/size, and any forced over-alignment. Used to detect arch copies that diverge
/// in shape (the CONTEXT class) so they are emitted per-arch instead of collapsed to one.
///
/// Methods are included so that reference TypeDefs whose divergence lives entirely in a
/// method signature rather than in fields (Win32 callbacks and WinRT delegates, whose
/// shape is the `Invoke` signature) are correctly detected as arch-divergent.
///
/// Field constant values are included so an enum whose members hold different per-arch
/// values does not collapse (which would silently drop the divergent values), matching
/// the apis-container member key which also folds in the constant value.
///
/// The `AlignmentAttribute` value is included because it is the *sole* winmd encoding of
/// forced over-alignment (`__declspec(align(N))` / `alignas(N)`): the `ClassLayout` can
/// only *lower* alignment via its packing size, so two copies identical in fields/layout
/// but differing only in raised alignment would otherwise collapse to one arch-neutral
/// copy and silently drop the other arch's alignment.
fn type_sig(index: &reader::Index, def: reader::TypeDef) -> String {
    let fields: Vec<String> = def
        .fields()
        .map(|f| {
            let val = f
                .constant()
                .map(|c| format!("{:?}", c.value()))
                .unwrap_or_default();
            format!("{}:{:?}={val}", f.name(), f.ty())
        })
        .collect();
    let methods: Vec<String> = def
        .methods()
        .map(|m| format!("{}:{:?}", m.name(), m.signature(&[])))
        .collect();
    let layout = def
        .class_layout()
        .map(|l| (l.packing_size(), l.class_size()));
    let align = def
        .find_attribute("AlignmentAttribute")
        .map(|a| format!("{:?}", a.value()));
    // Nested types are referenced by their (arch-invariant) leaf name, so a field
    // referencing a nested type looks identical across arches even when the nested
    // type's own shape diverges (e.g. `HSTRING_HEADER`'s inline union is `[24]` on
    // 64-bit and `[20]` on x86). Recurse into the nested subtree so such divergence
    // surfaces in the enclosing type's signature, so the arch merge then splits the
    // enclosing type per arch (hoisting `#[arch]` up to it) instead of collapsing to
    // one neutral copy and silently dropping the other arch's nested shape.
    let nested: Vec<String> = index
        .nested(def)
        .map(|inner| format!("{}={}", inner.name(), type_sig(index, inner)))
        .collect();
    format!(
        "{fields:?}|{methods:?}|{layout:?}|{align:?}|{:?}|{nested:?}",
        def.flags()
    )
}

fn write_attributes<'a, R: HasAttributes<'a>>(
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
fn write_attributes_with_arch<'a, R: HasAttributes<'a>>(
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
            && ty.namespace() == "Windows.Win32.Metadata"
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
    let ns = "Windows.Win32.Metadata";
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
        &[(String::new(), Value::I32(arch_bits))],
    );
}
