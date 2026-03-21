use super::*;

pub fn write_attribute(item: &metadata::reader::TypeDef) -> String {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let methods: String = item
        .methods()
        .map(|method| write_method(namespace, &method))
        .collect();

    // Named instance-field properties (e.g. `version: u32`).
    // Skip literals (enum variants), statics, and special-name fields (value__).
    let fields: String = item
        .fields()
        .filter_map(|f| {
            let flags = f.flags();
            if flags.contains(metadata::FieldAttributes::Public)
                && !flags.contains(metadata::FieldAttributes::Static)
                && !flags.contains(metadata::FieldAttributes::Literal)
                && !flags.contains(metadata::FieldAttributes::SpecialName)
            {
                let name = write_ident(f.name());
                let ty = write_type(namespace, &f.ty());
                Some(format!("{name}: {ty},\n"))
            } else {
                None
            }
        })
        .collect();

    let attrs = write_custom_attributes(item.attributes(), namespace, item.index());
    let header = format!("{attrs}attribute {name} ");
    write_block(header, format!("{methods}{fields}"))
}

fn write_method(namespace: &str, item: &metadata::reader::MethodDef) -> String {
    let signature = item.signature(&[]);
    let params = write_params(namespace, item, signature.types);
    format!("fn({});\n", params.join(", "))
}
