use super::*;

pub fn write_delegate(item: &metadata::reader::TypeDef) -> String {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let method = item
        .methods()
        .find(|method| method.name() == "Invoke")
        .expect("delegates are expected to have this named method");

    let signature = method.signature(&generics);
    let return_type = write_return_type(namespace, &signature);
    let params = method.params().filter(|param| param.sequence() != 0);

    let params: Vec<String> = params
        .zip(signature.types)
        .map(|(param, ty)| {
            let name = write_ident(param.name());
            let ty = write_type(namespace, &ty);
            format!("{name}: {ty}")
        })
        .collect();

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
        // GuidAttribute is derived from the delegate shape; skip it so round-trips stay clean
        &["GuidAttribute", "UnmanagedFunctionPointerAttribute"],
    );

    let mut abi = None;

    if let Some(attribute) = item.find_attribute("UnmanagedFunctionPointerAttribute") {
        if let Some((_, metadata::Value::EnumValue(_, value))) = attribute.value().first() {
            match &**value {
                metadata::Value::I32(1) => abi = Some("system"),
                metadata::Value::I32(2) => abi = Some("C"),
                metadata::Value::I32(5) => abi = Some("fastcall"),
                rest => unreachable!("unexpected CallingConvention value in UnmanagedFunctionPointerAttribute: {rest:?}"),
            }
        }
    }

    let abi_str = match abi {
        Some(abi) => format!(" \"{abi}\""),
        None => String::new(),
    };
    let ret_str = if return_type.is_empty() {
        String::new()
    } else {
        format!(" {return_type}")
    };

    format!(
        "{attrs}delegate{abi_str} fn {name}{generics_str}({}){ret_str};\n",
        params.join(", ")
    )
}
