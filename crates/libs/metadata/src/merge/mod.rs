use super::*;

/// Error produced by [`Merger::merge`].
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

/// Builder for merging multiple `.winmd` files into one.
///
/// # Example
///
/// ```no_run
/// windows_metadata::merge()
///     .input("a.winmd")
///     .input("b.winmd")
///     .output("merged.winmd")
///     .merge()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct Merger {
    input: Vec<String>,
    output: String,
}

impl Merger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a `.winmd` input file path or directory of `.winmd` files.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    /// Adds multiple `.winmd` input file paths or directories.
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

    /// Sets the output `.winmd` file path.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    /// Merges the input files and writes the result to the output path.
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

        // Sort types by (namespace, name) for deterministic output order.
        let mut types: Vec<reader::TypeDef<'_>> = index.types().collect();
        types.sort_by(|a, b| (a.namespace(), a.name()).cmp(&(b.namespace(), b.name())));

        for ty in types {
            write_type(&mut file, &index, ty, None);
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

fn write_type(
    file: &mut writer::File,
    index: &reader::TypeIndex,
    def: reader::TypeDef,
    outer: Option<writer::TypeDef>,
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
        let field_id = file.Field(field.name(), &field.ty(), field.flags());

        if let Some(constant) = field.constant() {
            file.Constant(writer::HasConstant::Field(field_id), &constant.value());
        }

        write_attributes(file, writer::HasAttribute::Field(field_id), field);
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    write_attributes(file, writer::HasAttribute::TypeDef(type_def), def);

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
                let param =
                    file.Param(param_def.name(), param_def.sequence(), param_def.flags());
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
        write_type(file, index, inner_def, Some(type_def));
    }
}

fn write_attributes<'a, R: reader::HasAttributes<'a>>(
    file: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
) {
    for attribute in row.attributes() {
        let ctor = attribute.ctor();
        let ty = ctor.parent();

        let attribute_ref =
            writer::MemberRefParent::TypeRef(file.TypeRef(ty.namespace(), ty.name()));

        let member_ref = file.MemberRef(".ctor", &ctor.signature(&[]), attribute_ref);

        file.Attribute(
            parent,
            writer::AttributeType::MemberRef(member_ref),
            &attribute.value(),
        );
    }
}
