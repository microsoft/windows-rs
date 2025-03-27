use super::*;

pub fn merge<I, SO, SI>(output: SO, input: I)
where
    I: IntoIterator<Item = SI>,
    SO: AsRef<std::path::Path>,
    SI: AsRef<std::path::Path>,
{
    let input: Vec<_> = input
        .into_iter()
        .map(|input| {
            reader::File::read(&input).unwrap_or_else(|| {
                panic!(
                    "failed to read .winmd format from `{}`",
                    input.as_ref().display()
                )
            })
        })
        .collect();

    let mut types: BTreeMap<&str, BTreeMap<&str, Vec<reader::TypeDef>>> = BTreeMap::new();
    let mut nested: HashMap<reader::TypeDef, Vec<reader::TypeDef>> = HashMap::new();

    for input in &input {
        for def in input.TypeDef() {
            let namespace = def.namespace();

            if namespace.is_empty() {
                // Skips `<Module>` as well as nested types.
                continue;
            }

            types
                .entry(namespace)
                .or_default()
                .entry(def.name())
                .or_default()
                .push(def);
        }

        for map in input.NestedClass() {
            let outer = map.outer();
            let inner = map.inner();
            nested.entry(outer).or_default().push(inner);
        }
    }

    let output = output.as_ref();
    let name = output.with_extension("");

    let name = name
        .file_name()
        .expect("`--out` file name is required")
        .to_string_lossy();

    let mut writer = writer::File::new(&name);

    for ty in types.values().flat_map(|types| types.values()).flatten() {
        write_type(&mut writer, &nested, *ty, None);
    }

    let bytes = writer.into_stream();
    std::fs::write(output, bytes)
        .unwrap_or_else(|_| panic!("failed to write .winmd format to `{}`", output.display()));
}

fn write_type(
    writer: &mut writer::File,
    nested: &HashMap<reader::TypeDef, Vec<reader::TypeDef>>,
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

    // TODO: only do this for fast_merge?
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

    for inner_def in nested
        .get(&def)
        .map(|nested| nested.iter())
        .unwrap_or_else(|| [].iter())
    {
        debug_assert!(inner_def.namespace().is_empty());
        debug_assert!(inner_def.flags().is_nested());
        write_type(writer, nested, *inner_def, Some(type_def));
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
