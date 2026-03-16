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
        format!(": {} ", ifaces.join(" + "))
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

    let attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        // GuidAttribute is derived from the interface shape; skip it so round-trips stay clean
        &["GuidAttribute"],
    );

    if methods.is_empty() {
        format!("{attrs}interface {name}{generics_str} {requires_str}{{}}\n")
    } else {
        format!("{attrs}interface {name}{generics_str} {requires_str}{{\n{methods}}}\n")
    }
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> String {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let mut param_strs: Vec<String> = vec!["&self".to_string()];
    for (param, ty) in params.zip(signature.types) {
        let param_name = write_ident(param.name());
        let out_attr = if param.flags().contains(metadata::ParamAttributes::Out)
            && !matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..))
        {
            "#[out] ".to_string()
        } else {
            String::new()
        };
        let ty_str = write_type(namespace, &ty);
        let param_attrs = write_custom_attributes(param.attributes(), namespace, item.index());
        param_strs.push(format!("{param_attrs}{out_attr}{param_name}: {ty_str}"));
    }

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
        param_strs.join(", ")
    )
}
