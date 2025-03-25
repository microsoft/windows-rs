use std::collections::HashMap;
use windows_ecma335::*;

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

    if input.is_empty() {
        panic!("at least one `--in` is required");
    };

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

    for path in input {
        let Some(reader) = reader::File::read(&path) else {
            panic!("failed to read .winmd format `{path}`");
        };

        let mut nested = HashMap::<usize, Vec<usize>>::new();

        for map in reader.NestedClass() {
            let outer = map.outer();
            let inner = map.inner();
            nested.entry(outer.index()).or_default().push(inner.index());
        }

        for def in reader.TypeDef().skip(1) {
            // TODO: does this need to be sorted (relative to other input files) to ensure stable output?

            if !def.flags().is_nested() {
                write_type(&reader, &nested, &mut writer, def, None);
            }
        }
    }

    let bytes = writer.into_stream();
    std::fs::write(output, bytes).unwrap();
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn expand_input(input: Vec<String>) -> Vec<String> {
    let mut result = vec![];

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
                    result.push(path.to_string_lossy().to_string());
                }
            }

            if result.len() == prev_len {
                panic!("failed to find .winmd files in directory `{input}`");
            }
        } else {
            result.push(input);
        }
    }

    result
}

fn write_type(
    reader: &reader::File,
    nested: &HashMap<usize, Vec<usize>>,
    writer: &mut writer::File,
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

        write_attributes(writer, writer::HasAttribute::Field(parent), &field);
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.sequence()))
        .collect();

    write_attributes(writer, writer::HasAttribute::TypeDef(type_def), &def);

    for map in def.interface_impls() {
        let interface_impl = writer.InterfaceImpl(type_def, &map.interface(&generics));

        write_attributes(
            writer,
            writer::HasAttribute::InterfaceImpl(interface_impl),
            &map,
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

                // TODO: this is broken?!
                write_attributes(writer, writer::HasAttribute::Param(param), &param_def);
            }

            write_attributes(writer, writer::HasAttribute::MethodDef(method_def), &method);

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

    if let Some(inner) = nested.get(&def.index()) {
        for inner in inner {
            let inner_def: reader::TypeDef = reader.row(*inner);
            debug_assert!(inner_def.namespace().is_empty());
            debug_assert!(inner_def.flags().is_nested());
            write_type(reader, nested, writer, inner_def, Some(type_def));
        }
    }
}

fn write_attributes<'a, R: reader::HasAttributes<'a>>(
    writer: &mut writer::File,
    parent: writer::HasAttribute,
    row: &'a R,
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
