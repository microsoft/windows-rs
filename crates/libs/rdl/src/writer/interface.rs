use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> String {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let methods: String = item
        .methods()
        .map(|method| write_method(namespace, &method, &generics))
        .collect();

    let requires: Vec<_> = item.interface_impls().collect();

    let requires_str = if requires.is_empty() {
        String::new()
    } else {
        let ifaces: Vec<String> = requires
            .iter()
            .map(|imp| write_type(namespace, &imp.interface(&generics)))
            .collect();
        format!(": {}", ifaces.join(" + "))
    };

    let generics_str = if generics.is_empty() {
        String::new()
    } else {
        let names: Vec<String> = item
            .generic_params()
            .map(|param| write_ident(param.name()))
            .collect();
        format!("<{}>", names.join(", "))
    };

    let guid_exclude: &[&str] = if interface_guid_is_derived(item) {
        &["GuidAttribute"]
    } else {
        &[]
    };
    let attrs =
        write_custom_attributes_except(item.attributes(), namespace, item.index(), guid_exclude);

    let header = format!("{attrs}interface {name}{generics_str}{requires_str} ");
    write_block(header, methods)
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> String {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, &signature);
    let params = std::iter::once("&self".to_string())
        .chain(write_params(namespace, item, signature.types))
        .collect::<Vec<_>>();

    let method_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    // Emit the built-in `#[special]` pseudo-attribute when SpecialName is set,
    // preserving properties and events on round-trip.
    let special_attr = if item
        .flags()
        .contains(metadata::MethodAttributes::SpecialName)
    {
        "#[special]\n".to_string()
    } else {
        String::new()
    };

    let ret_str = if return_type.is_empty() {
        String::new()
    } else {
        format!(" {return_type}")
    };

    format!(
        "{special_attr}{method_attrs}fn {name}({}){ret_str};\n",
        params.join(", ")
    )
}
