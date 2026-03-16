use super::*;
use windows_metadata::AsRow;

pub fn write_struct(item: &metadata::reader::TypeDef) -> String {
    if is_nested_type(item) {
        return String::new();
    }

    let namespace = item.namespace();
    let name = write_ident(item.name());

    let fields: String = item
        .fields()
        .map(|field| write_field(namespace, item, &field))
        .collect();

    let keyword = if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        "union"
    } else {
        "struct"
    };

    let attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    format!("{attrs}{keyword} {name} {{\n{fields}}}\n")
}

fn write_field(
    namespace: &str,
    parent: &metadata::reader::TypeDef,
    item: &metadata::reader::Field,
) -> String {
    let name = write_ident(item.name());

    let ty_str = match item.ty() {
        metadata::Type::Name(ty_name) => {
            if let Some(_resolved) = item.index().get(namespace, &ty_name.name).next() {
                write_type(
                    namespace,
                    &metadata::Type::named(&ty_name.namespace, &ty_name.name),
                )
            } else if ty_name.namespace.is_empty() && !ty_name.name.contains('/') {
                let nested = item
                    .index()
                    .nested(*parent)
                    .find(|t| t.name() == ty_name.name)
                    .unwrap_or_else(|| panic!("Could not resolve nested type: {}", ty_name.name));

                let keyword = nested_keyword(&nested);
                let fields: String = nested
                    .fields()
                    .map(|f| write_field(namespace, &nested, &f))
                    .collect();
                format!("{keyword} {{\n{fields}}}")
            } else if ty_name.namespace.is_empty() {
                let mut segments = ty_name.name.split('/');
                let first = segments.next().unwrap();

                let mut resolved = item
                    .index()
                    .nested(*parent)
                    .find(|t| t.name() == first)
                    .unwrap_or_else(|| panic!("Could not resolve nested type: {}", first));

                for segment in segments {
                    resolved = item
                        .index()
                        .nested(resolved)
                        .find(|t| t.name() == segment)
                        .unwrap_or_else(|| panic!("Could not resolve nested type: {}", segment));
                }

                let keyword = nested_keyword(&resolved);
                let fields: String = resolved
                    .fields()
                    .map(|f| write_field(namespace, &resolved, &f))
                    .collect();
                format!("{keyword} {{\n{fields}}}")
            } else {
                write_type(namespace, &metadata::Type::Name(ty_name.clone()))
            }
        }
        _ => write_type(namespace, &item.ty()),
    };

    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    // A type string ending with `}` is a multi-line nested struct/union.
    // The closing brace and field comma must appear together on their own line
    // so that reindent() correctly handles the indentation.
    if ty_str.ends_with('}') {
        format!("{field_attrs}{name}: {},\n", ty_str)
    } else {
        format!("{field_attrs}{name}: {ty_str},\n")
    }
}

fn is_nested_type(item: &metadata::reader::TypeDef) -> bool {
    item.flags()
        .contains(metadata::TypeAttributes::NestedPublic)
}

fn nested_keyword(item: &metadata::reader::TypeDef) -> &'static str {
    if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        "union"
    } else {
        "struct"
    }
}
