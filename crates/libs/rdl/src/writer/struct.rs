use super::*;
use std::collections::{HashMap, HashSet};
use windows_metadata::AsRow;

pub fn write_struct_items(
    item: &metadata::reader::TypeDef,
) -> Result<Vec<(String, TokenStream)>, Error> {
    let tokens = write_record(item.namespace(), item, false, None)?;
    Ok(vec![(item.name().to_string(), tokens)])
}

fn write_record(
    namespace: &str,
    item: &metadata::reader::TypeDef,
    inline: bool,
    parent_arches: Option<i32>,
) -> Result<TokenStream, Error> {
    let nested: Vec<metadata::reader::TypeDef> = item.index().nested(*item).collect();
    let record_name = item.full_name();

    let mut uses: HashMap<String, Vec<usize>> = HashMap::new();
    for (position, field) in item.fields().enumerate() {
        if let metadata::Type::ValueName(tn) = field.ty()
            && let Some(leaf) = nested_leaf(&tn, &record_name)
        {
            uses.entry(leaf.to_string()).or_default().push(position);
        }
    }

    // A nested record stored by value in exactly one field, and named after that field's
    // position, is written inline. Other nested types are declared inside the owner's body.
    let inline_names: HashSet<&str> = uses
        .iter()
        .filter_map(|(name, positions)| {
            if let [position] = positions.as_slice()
                && *name == format!("{}_{position}", item.name())
                && nested.iter().any(|child| {
                    child.name() == name
                        && child.category() == metadata::reader::TypeCategory::Struct
                })
            {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect();

    let scoped_names: HashMap<String, String> = nested
        .iter()
        .map(|child| {
            (
                child.name().to_string(),
                format!("{}/{}", record_name.name, child.name()),
            )
        })
        .collect();

    let declarations = nested
        .iter()
        .filter(|child| !inline_names.contains(child.name()))
        .map(|child| match child.category() {
            metadata::reader::TypeCategory::Enum => {
                write_enum_scoped(namespace, child, Some(item.arches()))
            }
            metadata::reader::TypeCategory::Struct => {
                write_record(namespace, child, false, Some(item.arches()))
            }
            _ => Err(writer_err!(
                "only nested records and enums are supported: {}",
                child.name()
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let inline_map: HashMap<&str, metadata::reader::TypeDef> = nested
        .iter()
        .filter(|child| inline_names.contains(child.name()))
        .map(|child| (child.name(), *child))
        .collect();

    let fields: Vec<TokenStream> = item
        .fields()
        .map(|field| -> Result<TokenStream, Error> {
            let flags = field.flags();
            if flags.contains(metadata::FieldAttributes::Static)
                || flags.contains(metadata::FieldAttributes::Literal)
            {
                if flags
                    != (metadata::FieldAttributes::Public
                        | metadata::FieldAttributes::Static
                        | metadata::FieldAttributes::Literal
                        | (flags & metadata::FieldAttributes::HasDefault))
                    || field.constant().is_none()
                {
                    return Err(writer_err!(
                        "unsupported record constant `{}`",
                        field.name()
                    ));
                }
                return write_const(namespace, &field);
            }
            if let metadata::Type::ValueName(tn) = field.ty()
                && let Some(leaf) = nested_leaf(&tn, &record_name)
                && let Some(child) = inline_map.get(leaf)
            {
                let inner = write_record(namespace, child, true, Some(item.arches()))?;
                let name = write_ident(field.name());
                let field_attrs =
                    write_custom_attributes(field.attributes(), namespace, field.index())?;
                return Ok(quote! { #(#field_attrs)* #name: #inner, });
            }
            write_field_scoped(namespace, &field, &record_name, &scoped_names)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let keyword = struct_keyword(item);
    let packed_attr = write_packed_attr(item);
    let align_attr = write_align_attr(item);
    // Nested records inherit the owner's architecture.
    let arch_attr = if inline || parent_arches == Some(item.arches()) {
        quote! {}
    } else {
        write_arch_attr(item.arches())
    };
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["SupportedArchitectureAttribute", "AlignmentAttribute"],
    )?;

    if inline {
        Ok(quote! {
            #packed_attr #align_attr #(#custom_attrs)*
            #keyword {
                #(#declarations)*
                #(#fields)*
            }
        })
    } else {
        let name_ident = write_ident(item.name());
        Ok(quote! {
            #packed_attr
            #align_attr
            #arch_attr
            #(#custom_attrs)*
            #keyword #name_ident {
                #(#declarations)*
                #(#fields)*
            }
        })
    }
}

fn write_field_scoped(
    namespace: &str,
    item: &metadata::reader::Field,
    record_name: &metadata::TypeName,
    scoped_names: &HashMap<String, String>,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let resolved_ty = resolve_nested(&item.ty(), namespace, record_name, scoped_names);
    let ty = write_type(namespace, &resolved_ty);

    // Bit-field backing units render as C-like blocks instead of raw attributes.
    let members = collect_bitfield_members(item);
    if !members.is_empty() {
        let block = write_bitfield_block(&members);
        let field_attrs = write_custom_attributes_except(
            item.attributes(),
            namespace,
            item.index(),
            &["NativeBitfieldAttribute"],
        )?;
        return Ok(quote! { #(#field_attrs)* #name: #ty { #(#block)* }, });
    }

    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;
    Ok(quote! { #(#field_attrs)* #name: #ty, })
}

fn collect_bitfield_members(item: &metadata::reader::Field) -> Vec<(String, u32, u32)> {
    let mut members: Vec<(String, u32, u32)> = item
        .attributes()
        .filter(|attr| {
            attr.namespace() == METADATA_NAMESPACE && attr.name() == "NativeBitfieldAttribute"
        })
        .filter_map(|attr| {
            let values = attr.value();
            let name = match values.first().map(|(_, v)| v) {
                Some(metadata::Value::Utf8(s)) => s.clone(),
                _ => return None,
            };
            let as_u32 = |v: Option<&(String, metadata::Value)>| match v.map(|(_, v)| v) {
                Some(metadata::Value::I64(n)) => Some(*n as u32),
                _ => None,
            };
            Some((name, as_u32(values.get(1))?, as_u32(values.get(2))?))
        })
        .collect();
    members.sort_by_key(|(_, offset, _)| *offset);
    members
}

fn write_bitfield_block(members: &[(String, u32, u32)]) -> Vec<TokenStream> {
    let mut out = vec![];
    let mut cursor = 0u32;
    for (name, offset, width) in members {
        if *offset > cursor {
            let pad = Literal::u32_unsuffixed(offset - cursor);
            out.push(quote! { _: #pad, });
        }
        let member = write_ident(name);
        let width_lit = Literal::u32_unsuffixed(*width);
        out.push(quote! { #member: #width_lit, });
        cursor = offset + width;
    }
    out
}

fn nested_leaf<'a>(name: &'a metadata::TypeName, parent: &metadata::TypeName) -> Option<&'a str> {
    // Accept the unscoped leaf references in existing metadata as well.
    if name.namespace.is_empty() && !name.name.contains('/') {
        return Some(&name.name);
    }
    let (outer, leaf) = name.name.rsplit_once('/')?;
    (name.namespace == parent.namespace && outer == parent.name).then_some(leaf)
}

fn resolve_nested(
    ty: &metadata::Type,
    namespace: &str,
    record_name: &metadata::TypeName,
    scoped_names: &HashMap<String, String>,
) -> metadata::Type {
    let resolve = |ty: &metadata::Type| resolve_nested(ty, namespace, record_name, scoped_names);
    match ty {
        metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
            let mut resolved = tn.clone();
            if let Some(leaf) = nested_leaf(tn, record_name)
                && let Some(scoped) = scoped_names.get(leaf)
            {
                resolved.namespace = namespace.to_string();
                resolved.name.clone_from(scoped);
            }
            resolved.generics = tn.generics.iter().map(resolve).collect();
            if matches!(ty, metadata::Type::ValueName(_)) {
                metadata::Type::ValueName(resolved)
            } else {
                metadata::Type::ClassName(resolved)
            }
        }
        metadata::Type::Array(inner) => metadata::Type::Array(Box::new(resolve(inner))),
        metadata::Type::RefMut(inner) => metadata::Type::RefMut(Box::new(resolve(inner))),
        metadata::Type::RefConst(inner) => metadata::Type::RefConst(Box::new(resolve(inner))),
        metadata::Type::ArrayFixed(inner, len) => {
            metadata::Type::ArrayFixed(Box::new(resolve(inner)), *len)
        }
        metadata::Type::PtrMut(inner, ptrs) => {
            metadata::Type::PtrMut(Box::new(resolve(inner)), *ptrs)
        }
        metadata::Type::PtrConst(inner, ptrs) => {
            metadata::Type::PtrConst(Box::new(resolve(inner)), *ptrs)
        }
        metadata::Type::FnPtr(signature) => metadata::Type::FnPtr(Box::new(metadata::Signature {
            flags: signature.flags,
            return_type: resolve(&signature.return_type),
            types: signature.types.iter().map(resolve).collect(),
        })),
        _ => ty.clone(),
    }
}

fn struct_keyword(item: &metadata::reader::TypeDef) -> TokenStream {
    if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    }
}

fn write_packed_attr(item: &metadata::reader::TypeDef) -> TokenStream {
    if let Some(size) = item
        .class_layout()
        .map(|layout| layout.packing_size())
        .filter(|&size| size > 0)
    {
        let size_literal = Literal::u16_unsuffixed(size);
        return quote! { #[packed(#size_literal)] };
    }
    quote! {}
}

fn write_align_attr(item: &metadata::reader::TypeDef) -> TokenStream {
    let Some(attribute) = item.find_attribute("AlignmentAttribute") else {
        return quote! {};
    };
    let Some((_, metadata::Value::I32(alignment))) = attribute.value().into_iter().next() else {
        return quote! {};
    };
    let size_literal = Literal::i32_unsuffixed(alignment);
    quote! { #[align(#size_literal)] }
}
