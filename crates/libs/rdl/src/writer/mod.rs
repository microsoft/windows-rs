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
use proc_macro2::*;
use quote::*;
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

    /// Filter what to include in the output.  Each call appends one rule.
    ///
    /// Rules are resolved against the input metadata and may be:
    /// * `"Windows.Win32"` — a namespace prefix: includes all items in matching namespaces
    /// * `"Windows.Win32.Foundation.POINT"` — a qualified type name: includes only that specific type
    /// * `"POINT"` — an unqualified type name: includes every type named `POINT` across all namespaces
    ///
    /// Prefix a rule with `!` to exclude instead of include.  Exclusions win over inclusions.
    /// If no filter rules are provided, everything from the input is written.
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
        let rules = resolve_filter(&self.filter, &index);

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
                if namespace.is_empty() {
                    continue;
                }

                let mut layout = Layout::new();

                for (name, item) in index.namespace_items(namespace) {
                    if !item_included(&rules, namespace, name) {
                        continue;
                    }
                    for (item_name, tokens) in write_items(namespace, item) {
                        layout.insert(namespace, &item_name, item_winrt(item), tokens.to_string());
                    }
                }

                let output = layout.to_string();

                if output.is_empty() {
                    continue;
                }

                let mut path = std::path::PathBuf::new();
                path.push(&self.output);
                path.push(format!("{namespace}.rdl"));

                write_to_file(
                    path.to_str().unwrap(),
                    formatter::format(&output).replace("#[r#in]", "#[in]"),
                );
            }
        } else {
            let mut layout = Layout::new();

            for namespace in index.keys() {
                for (name, item) in index.namespace_items(namespace) {
                    if !item_included(&rules, namespace, name) {
                        continue;
                    }
                    for (item_name, tokens) in write_items(namespace, item) {
                        layout.insert(namespace, &item_name, item_winrt(item), tokens.to_string());
                    }
                }
            }

            let output = layout.to_string();
            write_to_file(
                &self.output,
                formatter::format(&output).replace("#[r#in]", "#[in]"),
            );
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

enum FilterRule {
    /// All items whose namespace has this prefix.
    Namespace(String),
    /// A single specific (namespace, name) item.
    Type(String, String),
}

/// Resolves raw filter strings against the available metadata, producing typed rules.
///
/// The resolution order for each rule string `r` (after stripping `!`) is:
/// 1. Namespace prefix — any known namespace starts with `r` → [`FilterRule::Namespace`]
/// 2. Qualified type — `r` is `"Namespace.TypeName"` and that type exists → [`FilterRule::Type`]
/// 3. Unqualified type — `r` contains no dot and exists as a type name in one or more
///    namespaces → one [`FilterRule::Type`] per matching namespace
/// 4. Fallback — treat as a namespace prefix (may match nothing)
fn resolve_filter<'a>(
    filter: &[String],
    index: &metadata::reader::ItemIndex<'a>,
) -> Vec<(FilterRule, bool)> {
    let mut rules = vec![];

    for f in filter {
        let (rule_str, include) = if let Some(r) = f.strip_prefix('!') {
            (r, false)
        } else {
            (f.as_str(), true)
        };

        // 1. Namespace prefix match.
        if index.keys().any(|ns| namespace_starts_with(ns, rule_str)) {
            rules.push((FilterRule::Namespace(rule_str.to_string()), include));
            continue;
        }

        // 2. Qualified type name: "Namespace.TypeName".
        if let Some((namespace, name)) = rule_str.rsplit_once('.') {
            if index.get(namespace, name).next().is_some() {
                rules.push((
                    FilterRule::Type(namespace.to_string(), name.to_string()),
                    include,
                ));
                continue;
            }
        }

        // 3. Unqualified type name: search every namespace.
        let mut found = false;
        for ns in index.keys() {
            if index.get(ns, rule_str).next().is_some() {
                rules.push((
                    FilterRule::Type(ns.to_string(), rule_str.to_string()),
                    include,
                ));
                found = true;
            }
        }
        if found {
            continue;
        }

        // 4. Fallback: treat as a namespace prefix (may match nothing).
        rules.push((FilterRule::Namespace(rule_str.to_string()), include));
    }

    rules
}

/// Returns `true` if the item identified by `(namespace, name)` should be written.
///
/// When `rules` is empty every item is included.  Otherwise, an item is included when
/// at least one include rule matches it and no exclude rule matches it.  Exclude rules
/// are checked first and short-circuit immediately.
fn item_included(rules: &[(FilterRule, bool)], namespace: &str, name: &str) -> bool {
    if rules.is_empty() {
        return true;
    }

    let mut matched_include = false;

    for (rule, include) in rules {
        let matches = match rule {
            FilterRule::Namespace(prefix) => namespace_starts_with(namespace, prefix),
            FilterRule::Type(ns, n) => ns == namespace && n == name,
        };

        if matches {
            if !include {
                return false;
            }
            matched_include = true;
        }
    }

    matched_include
}

fn item_winrt(item: &metadata::reader::Item) -> bool {
    match item {
        metadata::reader::Item::Type(item) => item
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

fn write_items(namespace: &str, item: &metadata::reader::Item) -> Vec<(String, TokenStream)> {
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
) -> Vec<(String, TokenStream)> {
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

fn write_const(namespace: &str, item: &metadata::reader::Field) -> TokenStream {
    match item.ty() {
        metadata::Type::ValueName(tn) if &tn == ("System", "Guid") => {
            write_const_guid(namespace, item)
        }
        _ => write_const_value(namespace, item),
    }
}

fn write_const_value(namespace: &str, item: &metadata::reader::Field) -> TokenStream {
    let name = write_ident(item.name());
    let constant = item.constant();
    let ty = write_type(namespace, &item.ty());
    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    if let Some(constant) = constant {
        let value = write_value(namespace, &constant.value());
        quote! {
            #(#custom_attrs)*
            const #name: #ty = #value;
        }
    } else {
        quote! {
            #(#custom_attrs)*
            const #name: #ty;
        }
    }
}

fn write_const_guid(_namespace: &str, item: &metadata::reader::Field) -> TokenStream {
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

    let literal = syn::LitInt::new(&value, Span::call_site());
    quote! { const #name: GUID = #literal; }
}

fn write_params(
    namespace: &str,
    method: &metadata::reader::MethodDef,
    signature_types: Vec<metadata::Type>,
) -> Vec<TokenStream> {
    method
        .params()
        .filter(|param| param.sequence() != 0)
        .zip(signature_types)
        .map(|(param, ty)| {
            let has_in = param.flags().contains(metadata::ParamAttributes::In);
            let has_out = param.flags().contains(metadata::ParamAttributes::Out);
            let is_mutable = matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));
            // When neither `In` nor `Out` is set (e.g., metadata from external sources),
            // assume `In` as the default.
            let effective_in = has_in || !has_out;
            let in_attr = if effective_in && (has_out || is_mutable) {
                quote! { #[r#in] }
            } else {
                quote! {}
            };
            let out_attr = if has_out && (effective_in || !is_mutable) {
                quote! { #[out] }
            } else {
                quote! {}
            };
            let opt_attr = if param.flags().contains(metadata::ParamAttributes::Optional) {
                quote! { #[opt] }
            } else {
                quote! {}
            };
            let name = write_ident(param.name());
            let param_attrs =
                write_custom_attributes(param.attributes(), namespace, method.index());
            let ty = write_type(namespace, &ty);
            quote! { #(#param_attrs)* #in_attr #out_attr #opt_attr #name: #ty }
        })
        .collect()
}

fn write_return_type(namespace: &str, signature: &metadata::Signature) -> TokenStream {
    match &signature.return_type {
        metadata::Type::Void => quote! {},
        ty => {
            let ty = write_type(namespace, ty);
            quote! { -> #ty }
        }
    }
}

fn write_custom_attributes<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
) -> Vec<TokenStream> {
    write_custom_attributes_except(attributes, item_namespace, index, &[])
}

fn write_custom_attributes_except<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
    exclude: &[&str],
) -> Vec<TokenStream> {
    attributes
        .filter(|attr| {
            !namespace_starts_with(attr.namespace(), "System") && !exclude.contains(&attr.name())
        })
        .map(|attr| {
            let attr_ns = attr.namespace();
            let attr_short = attr
                .name()
                .strip_suffix("Attribute")
                .unwrap_or_else(|| attr.name());

            // Build the (possibly qualified) attribute path token stream.
            let name_ts = if attr_ns.is_empty() || attr_ns == item_namespace {
                write_ident(attr_short)
            } else {
                let mut tokens = TokenStream::new();
                for part in attr_ns.split('.') {
                    let ident = write_ident(part);
                    tokens = quote! { #tokens #ident :: };
                }
                let short = write_ident(attr_short);
                quote! { #tokens #short }
            };

            // Build the args token stream.  Positional args are emitted as plain values;
            // named args (non-empty name) are emitted as `name = value`.
            let args: Vec<_> = attr
                .value()
                .into_iter()
                .map(|(name, v)| {
                    let value_ts = match &v {
                        metadata::Value::EnumValue(tn, inner) => {
                            write_enum_value(item_namespace, tn, inner, index)
                        }
                        _ => write_value(item_namespace, &v),
                    };
                    if name.is_empty() {
                        value_ts
                    } else {
                        let name_ident = write_ident(&name);
                        quote! { #name_ident = #value_ts }
                    }
                })
                .collect();

            if args.is_empty() {
                quote! { #[#name_ts] }
            } else {
                quote! { #[#name_ts(#(#args),*)] }
            }
        })
        .collect()
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
) -> TokenStream {
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
                            let variant = write_ident(field.name());
                            return quote! { #variant };
                        }
                    }
                }
            }

            // For flag enums, try to decompose into a `Flag1 | Flag2` combination.
            let has_flags = typedef.attributes().any(|attr| {
                attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
            });

            if has_flags {
                if let Some(flags_ts) = write_flags_combination(namespace, &typedef, inner_i32) {
                    return flags_ts;
                }
            }
        }
    }

    write_value(namespace, inner)
}

/// Attempts to express `value` as a bitwise OR of known enum variants for a flags
/// enum.  Returns `None` if the value cannot be fully covered by the available
/// variants (i.e. there are leftover bits with no matching name).
fn write_flags_combination(
    _namespace: &str,
    typedef: &metadata::reader::TypeDef,
    value: i32,
) -> Option<TokenStream> {
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

    let mut iter = components.iter();
    let first = write_ident(iter.next().unwrap());
    let result = iter.fold(first, |acc, name| {
        let variant = write_ident(name);
        quote! { #acc | #variant }
    });

    Some(result)
}

fn write_type_def(item: &metadata::reader::TypeDef) -> TokenStream {
    match item.category() {
        // Structs/unions are handled by write_struct_items (which may return
        // multiple flat items) so this branch is never reached from the main
        // write loop.  It is kept for completeness and internal callers.
        metadata::reader::TypeCategory::Struct => quote! {},
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

fn write_value(namespace: &str, value: &metadata::Value) -> TokenStream {
    match value {
        metadata::Value::Bool(value) => quote! { #value },
        metadata::Value::U8(value) => {
            let literal = Literal::u8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I8(value) => {
            let literal = Literal::i8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U16(value) => {
            let literal = Literal::u16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I16(value) => {
            let literal = Literal::i16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U32(value) => {
            let literal = Literal::u32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I32(value) => {
            let literal = Literal::i32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U64(value) => {
            let literal = Literal::u64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I64(value) => {
            let literal = Literal::i64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::F32(value) => {
            let literal = Literal::f32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::F64(value) => {
            let literal = Literal::f64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::Utf8(value) => quote! { #value },
        metadata::Value::Utf16(value) => quote! { #value },
        metadata::Value::TypeName(tn) => {
            write_type(namespace, &metadata::Type::ClassName(tn.clone()))
        }
        metadata::Value::EnumValue(_, inner) => write_value(namespace, inner),
    }
}

fn write_type_ref(namespace: &str, item: &metadata::reader::TypeDefOrRef) -> TokenStream {
    write_type(
        namespace,
        &metadata::Type::class_named(item.namespace(), item.name()),
    )
}

fn write_type(namespace: &str, item: &metadata::Type) -> TokenStream {
    use metadata::Type::*;
    match item {
        Bool => quote! { bool },
        Char => quote! { u16 },
        I8 => quote! { i8 },
        U8 => quote! { u8 },
        I16 => quote! { i16 },
        U16 => quote! { u16 },
        I32 => quote! { i32 },
        U32 => quote! { u32 },
        I64 => quote! { i64 },
        U64 => quote! { u64 },
        F32 => quote! { f32 },
        F64 => quote! { f64 },
        ISize => quote! { isize },
        USize => quote! { usize },

        Void => quote! { void },
        String => quote! { String },
        Object => quote! { Object },
        ClassName(tn) if tn == ("System", "Type") => quote! { Type },
        ValueName(tn) if tn == ("System", "Guid") => quote! { GUID },
        ValueName(tn) if tn == ("Windows.Foundation", "HResult") => quote! { HRESULT },

        Array(ty) => {
            let ty = write_type(namespace, ty);
            quote! { [#ty] }
        }
        ArrayFixed(ty, len) => {
            let ty = write_type(namespace, ty);
            let len = Literal::usize_unsuffixed(*len);
            quote! { [#ty; #len] }
        }
        RefMut(ty) => {
            let ty = write_type(namespace, ty);
            quote! { &mut #ty }
        }
        RefConst(ty) => {
            let ty = write_type(namespace, ty);
            quote! { & #ty }
        }
        PtrMut(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *mut #ty };
            }

            ty
        }
        PtrConst(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *const #ty };
            }

            ty
        }
        ClassName(type_name) | ValueName(type_name) => {
            let name = write_ident(&type_name.name);

            let name = if type_name.generics.is_empty() {
                name
            } else {
                let generics = type_name
                    .generics
                    .iter()
                    .map(|ty| write_type(namespace, ty));
                quote! { #name <#(#generics),*> }
            };

            // The empty namespace test is for nested types.
            if namespace == type_name.namespace || type_name.namespace.is_empty() {
                name
            } else {
                let mut relative = namespace.split('.').peekable();
                let mut namespace = type_name.namespace.split('.').peekable();
                let shares_root = relative.peek() == namespace.peek();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    namespace.next();
                }

                let mut tokens = TokenStream::new();

                if shares_root {
                    for _ in 0..relative.count() {
                        tokens = quote! { #tokens super:: };
                    }
                }

                for namespace in namespace {
                    let namespace = write_ident(namespace);
                    tokens = quote! { #tokens #namespace ::};
                }

                quote! { #tokens #name }
            }
        }
        Generic(name, _) => {
            let name = write_ident(name);
            quote! { #name }
        }
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

fn write_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    let name = match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => format_ident!("r#{name}"),
        "Self" | "self" => format_ident!("{name}_"),
        "_" => format_ident!("unused"),
        _ => format_ident!("{}", windows_metadata::trim_tick(name)),
    };

    quote! { #name }
}
