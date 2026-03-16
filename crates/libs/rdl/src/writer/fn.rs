use super::*;

pub fn write_fn(namespace: &str, item: &metadata::reader::MethodDef) -> String {
    let name = write_ident(item.name());
    let signature = item.signature(&[]);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params: Vec<String> = params
        .zip(signature.types)
        .map(|(param, ty)| {
            let name = write_ident(param.name());
            let ty = write_type(namespace, &ty);
            format!("{name}: {ty}")
        })
        .collect();

    let Some(impl_map) = item.impl_map() else {
        unreachable!("fn item must have an ImplMap to be written as an `fn` item")
    };

    let scope = impl_map.import_scope();
    let library = scope.name();
    let flags = impl_map.flags();

    let abi = if flags.contains(metadata::PInvokeAttributes::CallConvFastcall) {
        Some("fastcall")
    } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
        Some("C")
    } else if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
        None
    } else {
        unreachable!(
            "unexpected calling convention in ImplMap flags: {:?}",
            flags
        )
    };

    let attrs = write_custom_attributes(item.attributes(), namespace, item.index());

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
        "{attrs}#[library(\"{library}\")]\nextern{abi_str} fn {name}({}){ret_str};\n",
        params.join(", ")
    )
}
