mod attribute;
mod callback;
mod class;
mod delegate;
mod r#enum;
mod r#fn;
mod interface;
mod layout;
mod r#struct;

use super::*;
use attribute::*;
use callback::*;
use class::*;
use delegate::*;
use interface::*;
use layout::*;
use metadata::AsRow;
use metadata::HasAttributes;
use r#enum::*;
use r#fn::*;
use r#struct::*;
use windows_metadata as metadata;

// The writer is primarily an internal tool as most developers will write their own
// definitions or just accept whatever a component author provides. This is thus mostly for
// generating rdl for backfilling definitions and for testing.

#[derive(Default)]
pub struct Writer {
    input: Vec<String>,
    filter: Vec<String>,
    output: String,
    split: bool,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    /// Filter namespaces to include in the output.  Each call appends one rule:
    /// * `"Windows.Win32"` — include all namespaces whose prefix matches `Windows.Win32`
    /// * `"!Windows.Win32"` — exclude all namespaces whose prefix matches `Windows.Win32`
    ///
    /// Exclusions take priority: if a namespace matches both an include rule and an
    /// exclude rule, it is excluded.  If no filter rules are provided all namespaces
    /// from the input are written.
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter.push(filter.to_string());
        self
    }

    pub fn split(&mut self) -> &mut Self {
        self.split = true;
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut files = vec![];

        for file_name in &expand_files(&self.input, "winmd")? {
            files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        let index = metadata::reader::TypeIndex::new(files);
        let index = metadata::reader::ItemIndex::new(&index);

        if self.split {
            // Remove any stale rdl files from a previous run before writing.
            if let Ok(entries) = std::fs::read_dir(&self.output) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("rdl"))
                    {
                        let _ = std::fs::remove_file(path);
                    }
                }
            }

            for namespace in index.keys() {
                if namespace.is_empty() || !namespace_included(namespace, &self.filter) {
                    continue;
                }

                let mut layout = Layout::new();

                for (_, item) in index.namespace_items(namespace) {
                    for (name, tokens) in write_items(namespace, item) {
                        layout.insert(namespace, &name, item_winrt(item), tokens);
                    }
                }

                let output = layout.to_string();

                if output.is_empty() {
                    continue;
                }

                let mut path = std::path::PathBuf::new();
                path.push(&self.output);
                path.push(format!("{namespace}.rdl"));

                write_to_file(path.to_str().unwrap(), output);
            }
        } else {
            let mut layout = Layout::new();

            for namespace in index.keys() {
                if !namespace_included(namespace, &self.filter) {
                    continue;
                }

                for (_, item) in index.namespace_items(namespace) {
                    for (name, tokens) in write_items(namespace, item) {
                        layout.insert(namespace, &name, item_winrt(item), tokens);
                    }
                }
            }

            let output = layout.to_string();
            write_to_file(&self.output, output);
        }

        Ok(())
    }
}

#[track_caller]
fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if std::fs::create_dir_all(parent).is_err() {
            panic!("failed to create directory `{path}`");
        }
    }

    if std::fs::write(path, contents).is_err() {
        panic!("failed to write file `{path}`");
    }
}

fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
    namespace.starts_with(starts_with)
        && (namespace.len() == starts_with.len()
            || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
}

/// Returns `true` if `namespace` should be written given the `filter` rules.
///
/// Each rule is either a plain namespace prefix (include) or a `!`-prefixed
/// namespace prefix (exclude).  Exclusions take priority: the first exclude
/// rule whose prefix matches causes the function to return `false` immediately.
/// If no filter rules are provided, every non-empty namespace is included.
fn namespace_included(namespace: &str, filter: &[String]) -> bool {
    if filter.is_empty() {
        return true;
    }

    let mut included = false;

    for rule in filter {
        if let Some(prefix) = rule.strip_prefix('!') {
            if namespace_starts_with(namespace, prefix) {
                return false;
            }
        } else if namespace_starts_with(namespace, rule) {
            included = true;
        }
    }

    included
}

fn item_winrt(item: &metadata::reader::Item) -> bool {
    match item {
        metadata::reader::Item::Type(item) => item
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

fn write_items(namespace: &str, item: &metadata::reader::Item) -> Vec<(String, String)> {
    match item {
        metadata::reader::Item::Type(ty) => write_type_def_items(namespace, ty),
        metadata::reader::Item::Fn(ty) => vec![(ty.name().to_string(), write_fn(namespace, ty))],
        metadata::reader::Item::Const(ty) => {
            vec![(ty.name().to_string(), write_const(namespace, ty))]
        }
    }
}

fn write_type_def_items(
    _namespace: &str,
    item: &metadata::reader::TypeDef,
) -> Vec<(String, String)> {
    match item.category() {
        metadata::reader::TypeCategory::Struct => write_struct_items(item),
        _ => {
            let tokens = write_type_def(item);
            if tokens.is_empty() {
                vec![]
            } else {
                vec![(item.name().to_string(), tokens)]
            }
        }
    }
}

fn write_const(namespace: &str, item: &metadata::reader::Field) -> String {
    match item.ty() {
        metadata::Type::Name(tn) if &tn == ("System", "Guid") => write_const_guid(namespace, item),
        _ => write_const_value(namespace, item),
    }
}

fn write_const_value(namespace: &str, item: &metadata::reader::Field) -> String {
    let name = write_ident(item.name());
    let constant = item.constant();
    let ty = write_type(namespace, &item.ty());
    let attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    if let Some(constant) = constant {
        let value = write_value(namespace, &constant.value());
        format!("{attrs}const {name}: {ty} = {value};\n")
    } else {
        format!("{attrs}const {name}: {ty};\n")
    }
}

fn write_const_guid(_namespace: &str, item: &metadata::reader::Field) -> String {
    let name = write_ident(item.name());
    let attribute = item
        .find_attribute("GuidAttribute")
        .expect("missing guid attribute");

    let value: u128 = attribute
        .value()
        .iter()
        .fold(0u128, |acc, (_, val)| match val {
            metadata::Value::U8(x) => (acc << 8) | *x as u128,
            metadata::Value::U16(x) => (acc << 16) | *x as u128,
            metadata::Value::U32(x) => (acc << 32) | *x as u128,
            metadata::Value::U64(x) => (acc << 64) | *x as u128,
            _ => panic!("unexpected guid attribute value"),
        });

    let value = format!(
        "0x{:08x}_{:04x}_{:04x}_{:04x}_{:012x}",
        (value >> 96) as u32,
        (value >> 80) as u16,
        (value >> 64) as u16,
        (value >> 48) as u16,
        value as u64 & 0xffffffffffff,
    );

    format!("const {name}: GUID = {value};\n")
}

fn write_params(
    namespace: &str,
    method: &metadata::reader::MethodDef,
    signature_types: Vec<metadata::Type>,
) -> Vec<String> {
    method
        .params()
        .filter(|param| param.sequence() != 0)
        .zip(signature_types)
        .map(|(param, ty)| {
            let in_attr = if !param.flags().contains(metadata::ParamAttributes::Out)
                && matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..))
            {
                "#[input] ".to_string()
            } else {
                String::new()
            };
            let out_attr = if param.flags().contains(metadata::ParamAttributes::Out)
                && !matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..))
            {
                "#[out] ".to_string()
            } else {
                String::new()
            };
            let opt_attr = if param.flags().contains(metadata::ParamAttributes::Optional) {
                "#[opt] ".to_string()
            } else {
                String::new()
            };
            let name = write_ident(param.name());
            let param_attrs = write_param_attributes(param.attributes(), namespace, method.index());
            let ty = write_type(namespace, &ty);
            format!("{param_attrs}{in_attr}{out_attr}{opt_attr}{name}: {ty}")
        })
        .collect()
}

fn write_return_type(namespace: &str, signature: &metadata::Signature) -> String {
    match &signature.return_type {
        metadata::Type::Void => String::new(),
        ty => format!("-> {}", write_type(namespace, ty)),
    }
}

fn write_custom_attributes<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
) -> String {
    write_custom_attributes_impl(attributes, item_namespace, index, &[], false)
}

fn write_custom_attributes_except<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
    exclude: &[&str],
) -> String {
    write_custom_attributes_impl(attributes, item_namespace, index, exclude, false)
}

/// Attributes on method parameters are emitted inline (space-terminated rather
/// than newline-terminated) so they appear on the same line as the parameter.
fn write_param_attributes<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
) -> String {
    write_custom_attributes_impl(attributes, item_namespace, index, &[], true)
}

fn write_custom_attributes_impl<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
    exclude: &[&str],
    inline: bool,
) -> String {
    let sep = if inline { " " } else { "\n" };
    let mut output = String::new();
    for attr in attributes {
        if namespace_starts_with(attr.namespace(), "System") || exclude.contains(&attr.name()) {
            continue;
        }

        let attr_ns = attr.ctor().parent().namespace();
        let attr_short = attr
            .name()
            .strip_suffix("Attribute")
            .unwrap_or_else(|| attr.name());

        // Build the (possibly qualified) attribute path.
        let name_str = if attr_ns.is_empty() || attr_ns == item_namespace {
            write_ident(attr_short)
        } else {
            let mut s = String::new();
            for part in attr_ns.split('.') {
                s.push_str(&write_ident(part));
                s.push_str("::");
            }
            s.push_str(&write_ident(attr_short));
            s
        };

        // Build the args list.  Positional args are emitted as plain values;
        // named args (non-empty name) are emitted as `name = value`.
        let args: Vec<String> = attr
            .value()
            .into_iter()
            .map(|(name, v)| {
                let value_str = match &v {
                    metadata::Value::EnumValue(tn, inner) => {
                        write_enum_value(item_namespace, tn, inner, index)
                    }
                    _ => write_value(item_namespace, &v),
                };
                if name.is_empty() {
                    value_str
                } else {
                    format!("{} = {}", write_ident(&name), value_str)
                }
            })
            .collect();

        if args.is_empty() {
            output.push_str(&format!("#[{name_str}]{sep}"));
        } else {
            output.push_str(&format!("#[{name_str}({})]{sep}", args.join(", ")));
        }
    }
    output
}

/// Writes an enum attribute argument as its variant name by looking up the integer
/// value in the TypeIndex.  For flag enums (those with `System.FlagsAttribute`),
/// falls back to decomposing the value into a `Flag1 | Flag2` combination when no
/// exact variant match is found.  Falls back to the raw inner value when no match
/// or decomposition can be found.
fn write_enum_value(
    namespace: &str,
    tn: &metadata::TypeName,
    inner: &metadata::Value,
    index: &metadata::reader::TypeIndex,
) -> String {
    let inner_i32 = match inner {
        metadata::Value::I32(n) => *n,
        _ => return write_value(namespace, inner),
    };

    for typedef in index.get(&tn.namespace, &tn.name) {
        if typedef.category() == metadata::reader::TypeCategory::Enum {
            // First try an exact variant match.
            for field in typedef.fields() {
                if field.flags().contains(metadata::FieldAttributes::Literal) {
                    if let Some(constant) = field.constant() {
                        let matches = match constant.value() {
                            metadata::Value::I32(v) => v == inner_i32,
                            // Use bit-equal comparison so that `U32(0xFFFFFFFF)` matches
                            // the `I32(-1)` that attribute blobs carry for that value.
                            metadata::Value::U32(v) => v == inner_i32 as u32,
                            _ => false,
                        };
                        if matches {
                            return write_ident(field.name());
                        }
                    }
                }
            }

            // For flag enums, try to decompose into a `Flag1 | Flag2` combination.
            let has_flags = typedef.attributes().any(|attr| {
                attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
            });

            if has_flags {
                if let Some(flags_str) = write_flags_combination(&typedef, inner_i32) {
                    return flags_str;
                }
            }
        }
    }

    write_value(namespace, inner)
}

/// Attempts to express `value` as a bitwise OR of known enum variants for a flags
/// enum.  Returns `None` if the value cannot be fully covered by the available
/// variants (i.e. there are leftover bits with no matching name).
fn write_flags_combination(typedef: &metadata::reader::TypeDef, value: i32) -> Option<String> {
    // Collect all non-zero literal fields together with their i32 values.
    let mut fields: Vec<(String, i32)> = typedef
        .fields()
        .filter_map(|field| {
            if !field.flags().contains(metadata::FieldAttributes::Literal) {
                return None;
            }
            let constant = field.constant()?;
            let v = match constant.value() {
                metadata::Value::I32(v) => v,
                // Reinterpret-cast U32 to i32 so that values like 0xFFFFFFFF
                // (`All`) are included and compared bit-identically below.
                metadata::Value::U32(v) => v as i32,
                _ => return None,
            };
            if v == 0 {
                None
            } else {
                Some((field.name().to_string(), v))
            }
        })
        .collect();

    // Sort descending by the unsigned interpretation so that composite flags
    // like `All = 0xFFFFFFFF` are tried before individual bits, giving more
    // compact results.
    fields.sort_by_key(|b| std::cmp::Reverse(b.1 as u32));

    let mut remaining = value;
    let mut components: Vec<String> = Vec::new();

    for (name, v) in &fields {
        if remaining == 0 {
            break;
        }
        if (remaining & v) == *v {
            remaining &= !v;
            components.push(name.clone());
        }
    }

    if remaining != 0 || components.is_empty() {
        // `remaining != 0` means there are bits with no matching variant.
        // `components.is_empty()` means `value` was 0 and no zero-valued variant
        // was found — the caller's exact-match pass handles the named-zero case
        // (e.g. `None = 0`), so falling back to the raw numeric value is correct.
        return None;
    }

    let result = components
        .iter()
        .map(|name| write_ident(name))
        .collect::<Vec<_>>()
        .join(" | ");

    Some(result)
}

fn write_type_def(item: &metadata::reader::TypeDef) -> String {
    match item.category() {
        // Structs/unions are handled by write_struct_items (which may return
        // multiple flat items) so this branch is never reached from the main
        // write loop.  It is kept for completeness and internal callers.
        metadata::reader::TypeCategory::Struct => String::new(),
        metadata::reader::TypeCategory::Enum => write_enum(item),
        metadata::reader::TypeCategory::Interface => write_interface(item),
        metadata::reader::TypeCategory::Class => write_class(item),
        metadata::reader::TypeCategory::Delegate => {
            if item
                .flags()
                .contains(metadata::TypeAttributes::WindowsRuntime)
            {
                write_delegate(item)
            } else {
                write_callback(item)
            }
        }
        metadata::reader::TypeCategory::Attribute => write_attribute(item),
    }
}

fn write_value(namespace: &str, value: &metadata::Value) -> String {
    match value {
        metadata::Value::Bool(value) => value.to_string(),
        metadata::Value::U8(value) => value.to_string(),
        metadata::Value::I8(value) => value.to_string(),
        metadata::Value::U16(value) => value.to_string(),
        metadata::Value::I16(value) => value.to_string(),
        metadata::Value::U32(value) => value.to_string(),
        metadata::Value::I32(value) => value.to_string(),
        metadata::Value::U64(value) => value.to_string(),
        metadata::Value::I64(value) => value.to_string(),
        metadata::Value::F32(value) => format_float_f32(*value),
        metadata::Value::F64(value) => format_float_f64(*value),
        metadata::Value::Utf8(value) => format!("{value:?}"),
        metadata::Value::Utf16(value) => format!("{value:?}"),
        metadata::Value::TypeName(tn) => write_type(namespace, &metadata::Type::Name(tn.clone())),
        metadata::Value::EnumValue(_, inner) => write_value(namespace, inner),
    }
}

fn format_float_f32(value: f32) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn format_float_f64(value: f64) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn write_type_ref(namespace: &str, item: &metadata::reader::TypeDefOrRef) -> String {
    write_type(
        namespace,
        &metadata::Type::named(item.namespace(), item.name()),
    )
}

/// Emit `header {}` for an empty body or `header {\n{body}}\n` otherwise.
fn write_block(header: String, body: String) -> String {
    if body.is_empty() {
        format!("{header}{{}}\n")
    } else {
        format!("{header}{{\n{body}}}\n")
    }
}

fn write_type(namespace: &str, item: &metadata::Type) -> std::string::String {
    use metadata::Type::*;
    match item {
        Bool => "bool".to_string(),
        Char => "u16".to_string(),
        I8 => "i8".to_string(),
        U8 => "u8".to_string(),
        I16 => "i16".to_string(),
        U16 => "u16".to_string(),
        I32 => "i32".to_string(),
        U32 => "u32".to_string(),
        I64 => "i64".to_string(),
        U64 => "u64".to_string(),
        F32 => "f32".to_string(),
        F64 => "f64".to_string(),
        ISize => "isize".to_string(),
        USize => "usize".to_string(),

        Void => "void".to_string(),
        String => "String".to_string(),
        Object => "Object".to_string(),
        Name(tn) if tn == ("System", "Type") => "Type".to_string(),
        Name(tn) if tn == ("System", "Guid") => "GUID".to_string(),
        Name(tn) if tn == ("Windows.Foundation", "HResult") => "HRESULT".to_string(),

        Array(ty) => format!("[{}]", write_type(namespace, ty)),
        ArrayFixed(ty, len) => format!("[{}; {}]", write_type(namespace, ty), len),
        RefMut(ty) => format!("&mut {}", write_type(namespace, ty)),
        RefConst(ty) => format!("&{}", write_type(namespace, ty)),
        PtrMut(ty, pointers) => {
            let mut s = write_type(namespace, ty);
            for _ in 0..*pointers {
                s = format!("*mut {s}");
            }
            s
        }
        PtrConst(ty, pointers) => {
            let mut s = write_type(namespace, ty);
            for _ in 0..*pointers {
                s = format!("*const {s}");
            }
            s
        }
        Name(type_name) => {
            let name = write_ident(&type_name.name);

            let name = if type_name.generics.is_empty() {
                name
            } else {
                let generics: Vec<std::string::String> = type_name
                    .generics
                    .iter()
                    .map(|ty| write_type(namespace, ty))
                    .collect();
                format!("{name}<{}>", generics.join(", "))
            };

            // The empty namespace test is for nested types.
            if namespace == type_name.namespace || type_name.namespace.is_empty() {
                name
            } else {
                let mut relative = namespace.split('.').peekable();
                let mut type_ns = type_name.namespace.split('.').peekable();
                let shares_root = relative.peek() == type_ns.peek();

                while relative.peek() == type_ns.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    type_ns.next();
                }

                let mut prefix = std::string::String::new();

                if shares_root {
                    for _ in 0..relative.count() {
                        prefix.push_str("super::");
                    }
                }

                for ns_part in type_ns {
                    prefix.push_str(&write_ident(ns_part));
                    prefix.push_str("::");
                }

                format!("{prefix}{name}")
            }
        }
        Generic(name, _) => write_ident(name),
    }
}

/// Extracts the raw GUID tuple `(data1, data2, data3, data4)` from a `GuidAttribute`.
fn extract_guid_from_attribute(attr: metadata::reader::Attribute) -> (u32, u16, u16, [u8; 8]) {
    let values: Vec<_> = attr.value().into_iter().map(|(_, v)| v).collect();
    assert_eq!(
        values.len(),
        11,
        "GuidAttribute must have exactly 11 arguments"
    );
    let d1 = match values[0] {
        metadata::Value::U32(v) => v,
        ref v => panic!("GuidAttribute d1: expected U32, got {v:?}"),
    };
    let d2 = match values[1] {
        metadata::Value::U16(v) => v,
        ref v => panic!("GuidAttribute d2: expected U16, got {v:?}"),
    };
    let d3 = match values[2] {
        metadata::Value::U16(v) => v,
        ref v => panic!("GuidAttribute d3: expected U16, got {v:?}"),
    };
    let d4 = std::array::from_fn(|i| match values[3 + i] {
        metadata::Value::U8(v) => v,
        ref v => panic!("GuidAttribute d4[{i}]: expected U8, got {v:?}"),
    });
    (d1, d2, d3, d4)
}

/// Returns `true` when the `GuidAttribute` stored on an interface `TypeDef` matches what
/// would be automatically derived from the interface shape (name + method signatures).
/// When `true`, the attribute is redundant and may be omitted from the RDL output.
/// When `false`, the GUID was set explicitly and must be preserved.
fn interface_guid_is_derived(item: &metadata::reader::TypeDef) -> bool {
    let Some(attr) = item.find_attribute("GuidAttribute") else {
        return true;
    };
    let stored = extract_guid_from_attribute(attr);

    let generics: Vec<_> = item
        .generic_params()
        .map(|p| metadata::Type::Generic(p.name().to_string(), p.sequence()))
        .collect();

    let sigs: Vec<(String, Vec<metadata::Type>, metadata::Type)> = item
        .methods()
        .map(|m| {
            let sig = m.signature(&generics);
            (m.name().to_string(), sig.types, sig.return_type)
        })
        .collect();

    let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = sigs
        .iter()
        .map(|(n, t, r)| (n.as_str(), t.as_slice(), r))
        .collect();

    let s = crate::reader::guid::build_interface_string(
        item.namespace(),
        metadata::trim_tick(item.name()),
        &methods,
    );
    let derived = crate::reader::guid::guid_from_interface_string(&s);
    stored == derived
}

/// Returns `true` when the `GuidAttribute` stored on a delegate `TypeDef` matches what
/// would be automatically derived from the `Invoke` method signature.
/// When `true`, the attribute is redundant and may be omitted from the RDL output.
/// When `false`, the GUID was set explicitly and must be preserved.
fn delegate_guid_is_derived(item: &metadata::reader::TypeDef) -> bool {
    let Some(attr) = item.find_attribute("GuidAttribute") else {
        return true;
    };
    let stored = extract_guid_from_attribute(attr);

    let generics: Vec<_> = item
        .generic_params()
        .map(|p| metadata::Type::Generic(p.name().to_string(), p.sequence()))
        .collect();

    let (types, return_type) = item
        .methods()
        .find(|m| m.name() == "Invoke")
        .map(|invoke| {
            let sig = invoke.signature(&generics);
            (sig.types, sig.return_type)
        })
        .unwrap_or_else(|| (vec![], metadata::Type::Void));

    let s = crate::reader::guid::build_interface_string(
        item.namespace(),
        metadata::trim_tick(item.name()),
        &[("Invoke", types.as_slice(), &return_type)],
    );
    let derived = crate::reader::guid::guid_from_interface_string(&s);
    stored == derived
}

fn write_ident(name: &str) -> String {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    let trimmed = windows_metadata::trim_tick(name);
    match trimmed {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => format!("r#{trimmed}"),
        "Self" | "self" => format!("{trimmed}_"),
        "_" => "unused".to_string(),
        _ => trimmed.to_string(),
    }
}
