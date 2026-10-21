use super::*;
use std::collections::{HashMap, HashSet};
use windows_metadata::AsRow;

pub fn write_struct_items(
    item: &metadata::reader::TypeDef,
) -> Result<Vec<(String, TokenStream)>, Error> {
    if item
        .flags()
        .contains(metadata::TypeAttributes::NestedPublic)
    {
        return Ok(vec![]);
    }

    let namespace = item.namespace();
    let tokens = write_record(namespace, item, false, None)?;
    Ok(vec![(item.name().to_string(), tokens)])
}

fn write_record(
    namespace: &str,
    item: &metadata::reader::TypeDef,
    inline: bool,
    parent_arches: Option<i32>,
) -> Result<TokenStream, Error> {
    if item.category() != metadata::reader::TypeCategory::Struct {
        return Err(writer_err!(
            "only nested records are supported: {}",
            item.name()
        ));
    }
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
    let inline_names: HashSet<_> = uses
        .iter()
        .filter_map(|(name, positions)| {
            if let [position] = positions.as_slice()
                && *name == format!("{}_{position}", item.name())
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
        .map(|child| write_record(namespace, child, false, Some(item.arches())))
        .collect::<Result<Vec<_>, _>>()?;

    let inline_map: HashMap<String, metadata::reader::TypeDef> = nested
        .iter()
        .filter(|child| inline_names.contains(child.name()))
        .map(|child| (child.name().to_string(), *child))
        .collect();

    let fields: Vec<TokenStream> = item
        .fields()
        .map(|field| -> Result<TokenStream, Error> {
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
    // Inline nested records inherit the parent's architecture.
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

fn packing_of(item: &metadata::reader::TypeDef) -> Option<u16> {
    item.class_layout()
        .map(|l| l.packing_size())
        .filter(|&s| s > 0)
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
        let block = write_bitfield_block(namespace, &members);
        let field_attrs = write_custom_attributes_except(
            item.attributes(),
            namespace,
            item.index(),
            &["NativeBitfieldAttribute"],
        )?;
        return Ok(quote! { #(#field_attrs)* #name: #ty { #(#block)* }, });
    }

    let field_attrs = write_custom_attributes_with_names(
        item.attributes(),
        namespace,
        item.index(),
        &[],
        Some(ExtentOwner::Field),
    )?;
    Ok(quote! { #(#field_attrs)* #name: #ty, })
}

struct BitfieldMember {
    name: String,
    offset: u32,
    width: u32,
    ty: Option<metadata::Type>,
}

fn collect_bitfield_members(item: &metadata::reader::Field) -> Vec<BitfieldMember> {
    let members: Option<Vec<_>> = item
        .attributes()
        .filter(|attr| {
            attr.namespace() == METADATA_NAMESPACE && attr.name() == "NativeBitfieldAttribute"
        })
        .map(|attr| {
            let values = attr.value();
            let [
                (name_key, metadata::Value::Utf8(name)),
                (offset_key, metadata::Value::I64(offset)),
                (width_key, metadata::Value::I64(width)),
                rest @ ..,
            ] = values.as_slice()
            else {
                return None;
            };
            if !name_key.is_empty() || !offset_key.is_empty() || !width_key.is_empty() {
                return None;
            }
            let ty = match rest {
                [] => None,
                [(key, metadata::Value::TypeName(ty))] if key == "MemberType" => Some(
                    ty.primitive()
                        .unwrap_or_else(|| metadata::Type::ValueName(ty.clone())),
                ),
                _ => return None,
            };
            Some(BitfieldMember {
                name: name.clone(),
                offset: (*offset).try_into().ok()?,
                width: (*width).try_into().ok()?,
                ty,
            })
        })
        .collect();
    let Some(mut members) = members else {
        return Vec::new();
    };
    members.sort_by_key(|member| member.offset);
    let mut cursor = 0;
    for member in &members {
        if member.offset < cursor {
            return Vec::new();
        }
        let Some(end) = member.offset.checked_add(member.width) else {
            return Vec::new();
        };
        cursor = end;
    }
    members
}

fn write_bitfield_block(namespace: &str, members: &[BitfieldMember]) -> Vec<TokenStream> {
    let mut out = vec![];
    let mut cursor = 0u32;
    for member in members {
        if member.offset > cursor {
            out.push(write_bitfield_member(
                namespace,
                None,
                member.offset - cursor,
                None,
            ));
        }
        out.push(write_bitfield_member(
            namespace,
            Some(&member.name),
            member.width,
            member.ty.as_ref(),
        ));
        cursor = member.offset + member.width;
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
    match ty {
        metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
            let mut resolved = tn.clone();
            if let Some(leaf) = nested_leaf(tn, record_name)
                && let Some(scoped) = scoped_names.get(leaf)
            {
                resolved.namespace = namespace.to_string();
                resolved.name.clone_from(scoped);
            }
            resolved.generics = tn
                .generics
                .iter()
                .map(|ty| resolve_nested(ty, namespace, record_name, scoped_names))
                .collect();
            if matches!(ty, metadata::Type::ValueName(_)) {
                metadata::Type::ValueName(resolved)
            } else {
                metadata::Type::ClassName(resolved)
            }
        }
        metadata::Type::Array(inner) => metadata::Type::Array(Box::new(resolve_nested(
            inner,
            namespace,
            record_name,
            scoped_names,
        ))),
        metadata::Type::RefMut(inner) => metadata::Type::RefMut(Box::new(resolve_nested(
            inner,
            namespace,
            record_name,
            scoped_names,
        ))),
        metadata::Type::RefConst(inner) => metadata::Type::RefConst(Box::new(resolve_nested(
            inner,
            namespace,
            record_name,
            scoped_names,
        ))),
        metadata::Type::ArrayFixed(inner, len) => metadata::Type::ArrayFixed(
            Box::new(resolve_nested(inner, namespace, record_name, scoped_names)),
            *len,
        ),
        metadata::Type::PtrMut(inner, ptrs) => metadata::Type::PtrMut(
            Box::new(resolve_nested(inner, namespace, record_name, scoped_names)),
            *ptrs,
        ),
        metadata::Type::PtrConst(inner, ptrs) => metadata::Type::PtrConst(
            Box::new(resolve_nested(inner, namespace, record_name, scoped_names)),
            *ptrs,
        ),
        metadata::Type::Ptr64(inner, constant) => metadata::Type::Ptr64(
            Box::new(resolve_nested(inner, namespace, record_name, scoped_names)),
            *constant,
        ),
        metadata::Type::FnPtr(signature) => metadata::Type::FnPtr(Box::new(metadata::Signature {
            flags: signature.flags,
            return_type: resolve_nested(
                &signature.return_type,
                namespace,
                record_name,
                scoped_names,
            ),
            types: signature
                .types
                .iter()
                .map(|ty| resolve_nested(ty, namespace, record_name, scoped_names))
                .collect(),
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
    write_packed_attr_value(packing_of(item))
}

fn write_packed_attr_value(packing: Option<u16>) -> TokenStream {
    if let Some(size) = packing {
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
