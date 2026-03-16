use super::*;

pub fn write_class(item: &metadata::reader::TypeDef) -> String {
    let namespace = item.namespace();
    let name = write_ident(item.name());
    let extends = item.extends().expect("class always extends");

    let extends_str = if extends == ("System", "Object") {
        String::new()
    } else {
        let ty = write_type_ref(namespace, &extends);
        format!(": {ty}")
    };

    let attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    let interfaces: String = item
        .interface_impls()
        .map(|imp| write_interface_impl(namespace, &imp))
        .collect();

    let header = format!("{attrs}class {name}{extends_str} ");
    write_block(header, interfaces)
}

fn write_interface_impl(namespace: &str, imp: &metadata::reader::InterfaceImpl) -> String {
    let interface = write_type(namespace, &imp.interface(&[]));

    let default_attr = if imp.has_attribute("DefaultAttribute") {
        "#[default]\n".to_string()
    } else {
        String::new()
    };

    format!("{default_attr}{interface},\n")
}
