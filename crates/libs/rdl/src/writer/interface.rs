use super::*;

pub fn write_interface(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = write_ident(item.name());

    let generics: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();

    let methods = item
        .methods()
        .map(|method| write_method(namespace, &method, &generics));

    let requires: Vec<_> = item.interface_impls().collect();

    let requires_tokens = if requires.is_empty() {
        quote! {}
    } else {
        let ifaces = requires
            .iter()
            .map(|imp| write_type(namespace, &imp.interface(&generics)));
        quote! { : #(#ifaces)+* }
    };

    let generics = if generics.is_empty() {
        quote! {}
    } else {
        let generics = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#generics),*> }
    };

    // Decide whether to emit GuidAttribute:
    // * WinRT interfaces: the reader derives the GUID via UUID v5 from the interface shape;
    //   no need to emit GuidAttribute.
    // * Win32 COM interfaces: the GUID may be a historical non-derivable value that must be
    //   preserved.  We only emit GuidAttribute when the stored value differs from the UUID v5
    //   derived value (i.e. it is genuinely non-derivable).  If it matches, the reader will
    //   re-derive it correctly and no emission is needed.
    let excluded: &[&str] = if !item
        .flags()
        .contains(metadata::TypeAttributes::WindowsRuntime)
        && win32_guid_is_non_derivable(item)
    {
        &[]
    } else {
        &["GuidAttribute"]
    };
    let custom_attrs =
        write_custom_attributes_except(item.attributes(), namespace, item.index(), excluded);

    quote! {
        #(#custom_attrs)*
        interface #name #generics #requires_tokens {
            #(#methods)*
        }
    }
}

fn write_method(
    namespace: &str,
    item: &metadata::reader::MethodDef,
    generics: &[metadata::Type],
) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(generics);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params =
        std::iter::once(quote! { &self }).chain(params.zip(signature.types).map(|(param, ty)| {
            let name = write_ident(param.name());
            let is_out = param.flags().contains(metadata::ParamAttributes::Out);
            let out_attr = param_out_attr(is_out, &ty);
            let ty = write_type(namespace, &ty);
            let param_attrs = write_custom_attributes(param.attributes(), namespace, item.index());
            quote! { #(#param_attrs)* #out_attr #name: #ty }
        }));

    let method_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    // Emit the built-in `#[special]` pseudo-attribute when SpecialName is set,
    // preserving properties and events on round-trip.
    let special_attr = if item
        .flags()
        .contains(metadata::MethodAttributes::SpecialName)
    {
        quote! { #[special] }
    } else {
        quote! {}
    };

    quote! {
        #special_attr
        #(#method_attrs)*
        fn #name(#(#params),*) #return_type;
    }
}

/// Returns `true` when a Win32 COM interface has a GUID that was NOT derived from its
/// interface shape using the midlrt UUID v5 algorithm.  Such GUIDs are historical values
/// that cannot be reconstructed from the interface definition alone and must be preserved
/// in the rdl output.
///
/// If the stored `GuidAttribute` matches the UUID v5 computed from the current method
/// signatures, the GUID is derivable and need not be emitted (the reader will reconstruct
/// the same value on the next roundtrip).
fn win32_guid_is_non_derivable(item: &metadata::reader::TypeDef) -> bool {
    let Some(attr) = item.find_attribute("GuidAttribute") else {
        return false;
    };

    // Decode the stored GUID from the attribute's positional args.
    let vals: Vec<_> = attr.value().into_iter().map(|(_, v)| v).collect();
    if vals.len() < 11 {
        return false;
    }
    let stored = (
        match &vals[0] {
            metadata::Value::U32(v) => *v,
            _ => return false,
        },
        match &vals[1] {
            metadata::Value::U16(v) => *v,
            _ => return false,
        },
        match &vals[2] {
            metadata::Value::U16(v) => *v,
            _ => return false,
        },
        [3, 4, 5, 6, 7, 8, 9, 10].map(|i| match &vals[i] {
            metadata::Value::U8(v) => *v,
            _ => 0,
        }),
    );

    // Compute the UUID v5 derived GUID from the interface's method signatures.
    let method_sigs: Vec<(String, Vec<metadata::Type>, metadata::Type)> = item
        .methods()
        .map(|m| {
            let sig = m.signature(&[]);
            (m.name().to_string(), sig.types, sig.return_type)
        })
        .collect();
    let method_refs: Vec<(&str, &[metadata::Type], &metadata::Type)> = method_sigs
        .iter()
        .map(|(name, types, ret)| (name.as_str(), types.as_slice(), ret))
        .collect();

    let iface_str =
        crate::reader::guid::build_interface_string(item.namespace(), item.name(), &method_refs);
    let (d1, d2, d3, d4) = crate::reader::guid::guid_from_interface_string(&iface_str);

    stored != (d1, d2, d3, d4)
}
