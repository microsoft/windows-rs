/// Emits RDL text from the intermediate representation.
use crate::ir::*;

/// Emit all `items` wrapped in a `#[win32]` module hierarchy derived from `namespace`.
///
/// `namespace` should be a dot-separated path such as `Windows.Win32.System.Threading`.
/// An empty namespace places items at the root level (no module wrapping).
pub fn emit_rdl(namespace: &str, items: &[IrItem]) -> String {
    let body = emit_items(items, 1);
    if namespace.is_empty() {
        return body;
    }
    wrap_in_namespace(namespace, &body)
}

/// Wrap `body` (already indented at `depth = 1`) inside the module hierarchy for
/// `namespace`.
fn wrap_in_namespace(namespace: &str, body: &str) -> String {
    let parts: Vec<&str> = namespace.split('.').collect();
    let inner_indent = "    ".repeat(parts.len());

    // Re-indent the body lines to account for the module nesting depth.
    let re_indented: String = body
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                // The body was already indented for depth=1; re-indent for the actual depth.
                let stripped = line.strip_prefix("    ").unwrap_or(line);
                format!("{inner_indent}{stripped}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Build the nested module open/close pairs.
    let mut out = String::new();
    out.push_str("#[win32]\n");
    for (i, part) in parts.iter().enumerate() {
        let indent = "    ".repeat(i);
        out.push_str(&format!("{indent}mod {part} {{\n"));
    }
    out.push_str(&re_indented);
    if !re_indented.ends_with('\n') {
        out.push('\n');
    }
    for i in (0..parts.len()).rev() {
        let indent = "    ".repeat(i);
        out.push_str(&format!("{indent}}}\n"));
    }
    out
}

fn emit_items(items: &[IrItem], depth: usize) -> String {
    let mut out = String::new();
    for item in items {
        let text = match item {
            IrItem::Struct(s) => emit_struct(s, depth),
            IrItem::Enum(e) => emit_enum(e, depth),
            IrItem::Function(f) => emit_function(f, depth),
            IrItem::Callback(c) => emit_callback(c, depth),
            IrItem::Const(c) => emit_const(c, depth),
            IrItem::Interface(i) => emit_interface(i, depth),
        };
        if !text.is_empty() {
            out.push_str(&text);
            out.push('\n');
        }
    }
    out
}

fn indent(depth: usize) -> String {
    "    ".repeat(depth)
}

fn emit_struct(s: &IrStruct, depth: usize) -> String {
    let ind = indent(depth);
    let field_ind = indent(depth + 1);
    let keyword = if s.is_union { "union" } else { "struct" };

    if let Some(pack) = s.pack {
        format!(
            "{ind}#[packed({pack})]\n{ind}{keyword} {} {{\n{fields}{ind}}}\n",
            s.name,
            fields = s
                .fields
                .iter()
                .map(|f| format!("{field_ind}{}: {},\n", f.name, emit_type(&f.ty)))
                .collect::<String>(),
        )
    } else {
        format!(
            "{ind}{keyword} {} {{\n{fields}{ind}}}\n",
            s.name,
            fields = s
                .fields
                .iter()
                .map(|f| format!("{field_ind}{}: {},\n", f.name, emit_type(&f.ty)))
                .collect::<String>(),
        )
    }
}

fn emit_enum(e: &IrEnum, depth: usize) -> String {
    let ind = indent(depth);
    let variant_ind = indent(depth + 1);
    let flags_attr = if e.is_flags {
        format!("{ind}#[flags]\n")
    } else {
        String::new()
    };

    let variants: String = e
        .variants
        .iter()
        .map(|v| {
            if e.underlying.is_signed() {
                format!("{variant_ind}{} = {},\n", v.name, v.value)
            } else {
                format!("{variant_ind}{} = {},\n", v.name, v.value as u64)
            }
        })
        .collect();

    format!(
        "{ind}#[repr({})]\n{flags_attr}{ind}enum {} {{\n{variants}{ind}}}\n",
        e.underlying.rdl_name(),
        e.name,
    )
}

fn emit_function(f: &IrFunction, depth: usize) -> String {
    let ind = indent(depth);

    let library_attr = if f.last_error {
        format!("{ind}#[library(\"{}\", last_error = true)]\n", f.library)
    } else {
        format!("{ind}#[library(\"{}\")]\n", f.library)
    };

    let abi_str = match f.abi {
        IrAbi::System => String::new(),
        IrAbi::C => " \"C\"".to_string(),
        IrAbi::Fastcall => " \"fastcall\"".to_string(),
    };

    let params = emit_params(&f.params, depth + 1);
    let ret = emit_return_type(&f.ret);

    if params.contains('\n') {
        format!(
            "{library_attr}{ind}extern{abi_str} fn {}(\n{params}{ind}){ret};\n",
            f.name,
        )
    } else {
        let params_inline = params.trim().trim_end_matches(',').to_string();
        format!(
            "{library_attr}{ind}extern{abi_str} fn {}({params_inline}){ret};\n",
            f.name,
        )
    }
}

fn emit_callback(c: &IrCallback, depth: usize) -> String {
    let ind = indent(depth);
    let abi_str = match c.abi {
        IrAbi::System => String::new(),
        IrAbi::C => " \"C\"".to_string(),
        IrAbi::Fastcall => " \"fastcall\"".to_string(),
    };

    let params = emit_params(&c.params, depth + 1);
    let ret = emit_return_type(&c.ret);

    if params.contains('\n') {
        format!(
            "{ind}extern{abi_str} fn {}(\n{params}{ind}){ret};\n",
            c.name,
        )
    } else {
        let params_inline = params.trim().trim_end_matches(',').to_string();
        format!(
            "{ind}extern{abi_str} fn {}({params_inline}){ret};\n",
            c.name,
        )
    }
}

fn emit_interface(i: &IrInterface, depth: usize) -> String {
    let ind = indent(depth);
    let method_ind = indent(depth + 1);

    let guid_attr = if let Some(guid) = &i.guid {
        format!("{ind}#[\"{guid}\"]\n")
    } else {
        String::new()
    };

    let base_str = if let Some(base) = &i.base {
        format!(": {base} ")
    } else {
        String::new()
    };

    let methods: String = i
        .methods
        .iter()
        .map(|m| {
            let params = emit_params(&m.params, depth + 2);
            let ret = emit_return_type(&m.ret);
            if params.contains('\n') {
                format!(
                    "{method_ind}fn {}(&self,\n{params}{method_ind}){ret};\n",
                    m.name,
                )
            } else {
                let params_inline = params.trim().trim_end_matches(',').to_string();
                let sep = if params_inline.is_empty() { "" } else { ", " };
                format!(
                    "{method_ind}fn {}(&self{sep}{params_inline}){ret};\n",
                    m.name,
                )
            }
        })
        .collect();

    format!(
        "{guid_attr}{ind}interface {}{base_str}{{\n{methods}{ind}}}\n",
        i.name,
    )
}

fn emit_const(c: &IrConst, depth: usize) -> String {
    let ind = indent(depth);
    let value_str = match &c.value {
        ConstValue::Int(v) => v.to_string(),
        ConstValue::Uint(v) => v.to_string(),
        ConstValue::Float(v) => format!("{v}"),
    };
    format!(
        "{ind}const {}: {} = {value_str};\n",
        c.name,
        c.ty.rdl_name()
    )
}

fn emit_params(params: &[IrParam], depth: usize) -> String {
    if params.is_empty() {
        return String::new();
    }

    // If all params are simple (no modifier attributes) and there are ≤ 4, inline them.
    let all_simple = params.iter().all(|p| p.modifiers.is_empty());
    if all_simple && params.len() <= 4 {
        let inner: Vec<String> = params
            .iter()
            .map(|p| format!("{}: {}", p.name, emit_type(&p.ty)))
            .collect();
        return inner.join(", ");
    }

    let ind = indent(depth);
    let mut out = String::new();
    for p in params {
        let attr_prefix: String = p
            .modifiers
            .iter()
            .map(|m| match m {
                ParamModifier::In => "#[in] ".to_string(),
                ParamModifier::Out => "#[out] ".to_string(),
                ParamModifier::Opt => "#[opt] ".to_string(),
            })
            .collect();
        out.push_str(&format!(
            "{ind}{attr_prefix}{}: {},\n",
            p.name,
            emit_type(&p.ty)
        ));
    }
    out
}

fn emit_return_type(ty: &IrType) -> String {
    match ty {
        IrType::Void => String::new(),
        other => format!(" -> {}", emit_type(other)),
    }
}

pub fn emit_type(ty: &IrType) -> String {
    match ty {
        IrType::Void => "*mut void".to_string(),
        IrType::Primitive(p) => p.rdl_name().to_string(),
        IrType::Named(n) => n.clone(),
        IrType::Ptr { is_const, pointee } => {
            let modifier = if *is_const { "const" } else { "mut" };
            match pointee.as_ref() {
                IrType::Void => format!("*{modifier} void"),
                inner => format!("*{modifier} {}", emit_type(inner)),
            }
        }
        IrType::Array { elem, size } => {
            format!("[{}; {size}]", emit_type(elem))
        }
    }
}
