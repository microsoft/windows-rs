use windows_metadata::*;

enum ArgKind {
    None,
    Input,
    Output,
}

fn main() {
    let time = std::time::Instant::now();
    let mut output = None;
    let mut input = vec![];
    let mut kind = ArgKind::None;

    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.to_string());
                } else {
                    panic!("exactly one `--out` is required");
                }
            }
            ArgKind::Input => input.push(arg.to_string()),
        }
    }

    let input = expand_input(input);

    let Some(output) = output else {
        panic!("exactly one `--out` is required");
    };

    let output = std::path::Path::new(&output);
    let name = output.with_extension("");

    let name = name
        .file_name()
        .expect("`--out` file name is required")
        .to_string_lossy();

    let mut writer = writer::File::new(&name);

    let index = reader::TypeIndex::new(input);

    for ty in index.types() {
        write_type(&mut writer, &index, ty, None);
    }

    let bytes = writer.into_stream();
    std::fs::write(output, bytes).unwrap();
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn expand_input(input: Vec<String>) -> Vec<reader::File> {
    let mut result = vec![];

    let read_file = |path| {
        reader::File::read(&path).unwrap_or_else(|| panic!("failed to read .winmd format `{path}`"))
    };

    for input in input {
        let path = std::path::Path::new(&input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("failed to read directory `{input}`"))
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
                    result.push(read_file(path.to_string_lossy().to_string()));
                }
            }

            if result.len() == prev_len {
                panic!("failed to find .winmd files in directory `{input}`");
            }
        } else {
            result.push(read_file(input));
        }
    }

    result
}

fn write_type(
    writer: &mut writer::File,
    index: &reader::TypeIndex,
    def: reader::TypeDef,
    outer: Option<writer::TypeDef>,
) {
    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(writer.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    if cfg!(debug_assertions) {
        if def.flags().is_nested() {
            assert!(def.namespace().is_empty());
        } else {
            assert!(!def.namespace().is_empty());
        }
    }

    let type_def = writer.TypeDef(def.namespace(), def.name(), extends, def.flags());

    if let Some(outer) = outer {
        writer.NestedClass(type_def, outer);
    }

    for field in def.fields() {
        let parent = writer.Field(field.name(), &field.ty(), field.flags());

        if let Some(constant) = field.constant() {
            writer.Constant(writer::HasConstant::Field(parent), &constant.value());
        }

        write_attributes(writer, writer::HasAttribute::Field(parent), field);
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.sequence()))
        .collect();

    write_attributes(writer, writer::HasAttribute::TypeDef(type_def), def);

    for map in def.interface_impls() {
        let interface_impl = writer.InterfaceImpl(type_def, &map.interface(&generics));

        write_attributes(
            writer,
            writer::HasAttribute::InterfaceImpl(interface_impl),
            map,
        );
    }

    for generic in def.generic_params() {
        writer.GenericParam(
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
            let method_def = writer.MethodDef(
                method.name(),
                &method.signature(&generics),
                method.flags(),
                method.impl_flags(),
            );

            for param_def in method.params() {
                let param = writer.Param(param_def.name(), param_def.sequence(), param_def.flags());
                write_attributes(writer, writer::HasAttribute::Param(param), param_def);
            }

            write_attributes(writer, writer::HasAttribute::MethodDef(method_def), method);

            if let Some(impl_map) = method.impl_map() {
                writer.ImplMap(
                    method_def,
                    impl_map.flags(),
                    impl_map.import_name(),
                    impl_map.import_scope().name(),
                );
            }
        }
    }

    if let Some(class_layout) = def.class_layout() {
        writer.ClassLayout(
            type_def,
            class_layout.packing_size(),
            class_layout.class_size(),
        );
    }

    for inner_def in index.nested(def) {
        debug_assert!(inner_def.namespace().is_empty());
        debug_assert!(inner_def.flags().is_nested());
        write_type(writer, index, inner_def, Some(type_def));
    }
}

fn write_attributes<'a, R: reader::HasAttributes<'a>>(
    writer: &mut writer::File,
    parent: writer::HasAttribute,
    row: R,
) {
    for attribute in row.attributes() {
        let ctor = attribute.ctor();
        let ty = ctor.parent();

        let attribute_ref =
            writer::MemberRefParent::TypeRef(writer.TypeRef(ty.namespace(), ty.name()));

        let ctor = writer.MemberRef(".ctor", &ctor.signature(&[]), attribute_ref);

        writer.Attribute(
            parent,
            writer::AttributeType::MemberRef(ctor),
            &attribute.value(),
        );
    }
}
