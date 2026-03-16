use super::*;

pub fn write_enum(item: &metadata::reader::TypeDef) -> String {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let repr = item.fields().next().unwrap();
    let repr = if let Some(constant) = repr.constant() {
        constant.ty()
    } else {
        repr.ty()
    };

    let repr = write_type(namespace, &repr);

    let fields: String = item
        .fields()
        .filter_map(|field| {
            field.constant().map(|constant| {
                let name = write_ident(field.name());
                let value = write_value(namespace, &constant.value());
                format!("{name} = {value},\n")
            })
        })
        .collect();

    let is_flags_attr = |attr: metadata::reader::Attribute| {
        attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
    };

    let has_flags = item.attributes().any(is_flags_attr);

    let attrs = write_custom_attributes(
        item.attributes().filter(|attr| !is_flags_attr(*attr)),
        namespace,
        item.index(),
    );

    if has_flags {
        format!("#[repr({repr})]\n#[flags]\n{attrs}enum {name} {{\n{fields}}}\n")
    } else {
        format!("#[repr({repr})]\n{attrs}enum {name} {{\n{fields}}}\n")
    }
}
