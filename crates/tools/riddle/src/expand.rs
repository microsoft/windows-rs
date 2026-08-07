use metadata::{AsRow, HasAttributes};
use windows_metadata as metadata;

pub fn render(bytes: Vec<u8>) -> String {
    let file = metadata::reader::File::new(bytes).unwrap();
    let index = metadata::reader::Index::new(vec![file]);
    let mut types: Vec<_> = index.types().collect();
    for ty in index.types() {
        types.extend(index.nested_recursive(ty));
    }
    types.sort_by(|left, right| {
        (left.namespace(), left.name(), left.row_id()).cmp(&(
            right.namespace(),
            right.name(),
            right.row_id(),
        ))
    });

    let mut output = String::new();
    for ty in types {
        render_type(&mut output, ty);
    }
    output
}

fn render_type(output: &mut String, ty: metadata::reader::TypeDef<'_>) {
    let qualified_name = if ty.namespace().is_empty() {
        ty.name().to_string()
    } else {
        format!("{}.{}", ty.namespace(), ty.name())
    };
    output.push_str(&format!(
        "type {qualified_name} category={:?} flags={:#010x}\n",
        ty.category(),
        ty.flags().0
    ));

    if let Some(extends) = ty.extends() {
        output.push_str(&format!(
            "  extends {}.{}\n",
            extends.namespace(),
            extends.name()
        ));
    }
    for implementation in ty.interface_impls() {
        output.push_str(&format!(
            "  implements {}\n",
            render_type_name(&implementation.interface(&generics(ty)))
        ));
    }
    if let Some(layout) = ty.class_layout() {
        output.push_str(&format!(
            "  layout packing={} size={}\n",
            layout.packing_size(),
            layout.class_size()
        ));
    }
    render_attributes(output, "  ", ty.attributes());

    for field in ty.fields() {
        output.push_str(&format!(
            "  field {}: {} flags={:#06x}",
            field.name(),
            render_type_name(&field.ty()),
            field.flags().0
        ));
        if let Some(layout) = field.layout() {
            output.push_str(&format!(" offset={}", layout.offset()));
        }
        if let Some(constant) = field.constant() {
            output.push_str(&format!(" value={:?}", constant.value()));
        }
        output.push('\n');
        render_attributes(output, "    ", field.attributes());
    }

    let generics = generics(ty);
    for method in ty.methods() {
        let signature = method.signature(&generics);
        output.push_str(&format!("  method {}(", method.name()));
        let params = method.params_by_sequence(signature.types.len()).ok();
        for (position, parameter_type) in signature.types.iter().enumerate() {
            if position != 0 {
                output.push_str(", ");
            }
            let name = params
                .as_ref()
                .and_then(|params| params.params()[position])
                .map_or_else(
                    || format!("p{}", position + 1),
                    |param| param.name().to_string(),
                );
            output.push_str(&format!("{name}: {}", render_type_name(parameter_type)));
        }
        output.push_str(&format!(
            ") -> {} flags={:#06x} impl={:#06x} call={:#04x}",
            render_type_name(&signature.return_type),
            method.flags().0,
            method.impl_flags().0,
            signature.flags.0
        ));
        if let Some(map) = method.impl_map() {
            output.push_str(&format!(
                " pinvoke={}!{} map={:#06x}",
                map.import_scope().name(),
                map.import_name(),
                map.flags().0
            ));
        }
        if let Some(overload) = method
            .attributes()
            .find(|attribute| {
                attribute.namespace() == "Windows.Foundation.Metadata"
                    && attribute.name() == "OverloadAttribute"
            })
            .and_then(|attribute| {
                attribute
                    .value()
                    .into_iter()
                    .find_map(|(_, value)| match value {
                        metadata::Value::Utf8(value) => Some(value),
                        _ => None,
                    })
            })
        {
            output.push_str(&format!(" projected={overload}"));
        }
        if method.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Foundation.Metadata"
                && attribute.name() == "DefaultOverloadAttribute"
        }) {
            output.push_str(" default");
        }
        output.push('\n');
        render_attributes(output, "    ", method.attributes());
    }

    for property in ty.properties() {
        let signature = property.signature(&generics);
        output.push_str(&format!(
            "  property {}: {}",
            property.name(),
            render_type_name(&signature.return_type)
        ));
        if !signature.types.is_empty() {
            output.push_str(" indexes=(");
            for (position, parameter) in signature.types.iter().enumerate() {
                if position != 0 {
                    output.push_str(", ");
                }
                output.push_str(&render_type_name(parameter));
            }
            output.push(')');
        }
        render_semantics(output, property.semantics());
        output.push('\n');
    }

    for event in ty.events() {
        output.push_str(&format!(
            "  event {}: {}",
            event.name(),
            render_type_name(&event.ty(&generics))
        ));
        render_semantics(output, event.semantics());
        output.push('\n');
    }

    output.push('\n');
}

fn render_semantics<'a>(
    output: &mut String,
    semantics: impl Iterator<Item = metadata::reader::MethodSemantics<'a>>,
) {
    let mut first = true;
    for semantics in semantics {
        if first {
            output.push_str(" semantics=[");
            first = false;
        } else {
            output.push_str(", ");
        }
        output.push_str(&format!(
            "{:#06x}:{}",
            semantics.semantics(),
            semantics.method().name()
        ));
    }
    if !first {
        output.push(']');
    }
}

fn render_attributes<'a>(
    output: &mut String,
    indent: &str,
    attributes: impl Iterator<Item = metadata::reader::Attribute<'a>>,
) {
    for attribute in attributes {
        output.push_str(&format!(
            "{indent}attribute {}.{}\n",
            attribute.namespace(),
            attribute.name()
        ));
    }
}

fn generics(ty: metadata::reader::TypeDef<'_>) -> Vec<metadata::Type> {
    ty.generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect()
}

fn render_type_name(ty: &metadata::Type) -> String {
    match ty {
        metadata::Type::Void => "void".to_string(),
        metadata::Type::Bool => "bool".to_string(),
        metadata::Type::Char => "char".to_string(),
        metadata::Type::I8 => "i8".to_string(),
        metadata::Type::U8 => "u8".to_string(),
        metadata::Type::I16 => "i16".to_string(),
        metadata::Type::U16 => "u16".to_string(),
        metadata::Type::I32 => "i32".to_string(),
        metadata::Type::U32 => "u32".to_string(),
        metadata::Type::I64 => "i64".to_string(),
        metadata::Type::U64 => "u64".to_string(),
        metadata::Type::F32 => "f32".to_string(),
        metadata::Type::F64 => "f64".to_string(),
        metadata::Type::ISize => "isize".to_string(),
        metadata::Type::USize => "usize".to_string(),
        metadata::Type::String => "String".to_string(),
        metadata::Type::Object => "Object".to_string(),
        metadata::Type::ClassName(name) | metadata::Type::ValueName(name) => {
            let mut output = if name.namespace.is_empty() {
                name.name.clone()
            } else {
                format!("{}.{}", name.namespace, name.name)
            };
            if !name.generics.is_empty() {
                output.push('<');
                for (position, generic) in name.generics.iter().enumerate() {
                    if position != 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&render_type_name(generic));
                }
                output.push('>');
            }
            output
        }
        metadata::Type::Array(element) => format!("[{}]", render_type_name(element)),
        metadata::Type::Generic(name, _) => name.clone(),
        metadata::Type::RefMut(element) => format!("&mut {}", render_type_name(element)),
        metadata::Type::RefConst(element) => format!("&{}", render_type_name(element)),
        metadata::Type::PtrMut(element, depth) => {
            format!("{}{}", "*mut ".repeat(*depth), render_type_name(element))
        }
        metadata::Type::PtrConst(element, depth) => {
            format!("{}{}", "*const ".repeat(*depth), render_type_name(element))
        }
        metadata::Type::ArrayFixed(element, len) => {
            format!("[{}; {len}]", render_type_name(element))
        }
    }
}
