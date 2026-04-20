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
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input.push(input.as_ref().to_string());
        }

        self
    }

    pub fn filters<I, S>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for filter in filters {
            self.filter.push(filter.as_ref().to_string());
        }

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

    pub fn split(&mut self, split: bool) -> &mut Self {
        self.split = split;
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut files = vec![];

        for file_name in &expand_input_paths(&self.input, "winmd", ".")?.0 {
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
                    for (item_name, tokens) in write_items(namespace, item)? {
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

                let path_str = path
                    .to_str()
                    .ok_or_else(|| writer_err!("output path contains non-UTF-8 characters"))?;
                write_to_file(
                    path_str,
                    formatter::format(&output).replace("#[r#in]", "#[in]"),
                )?;
            }
        } else {
            let mut layout = Layout::new();

            for namespace in index.keys() {
                for (name, item) in index.namespace_items(namespace) {
                    if !item_included(&rules, namespace, name) {
                        continue;
                    }
                    for (item_name, tokens) in write_items(namespace, item)? {
                        layout.insert(namespace, &item_name, item_winrt(item), tokens.to_string());
                    }
                }
            }

            let output = layout.to_string();
            write_to_file(
                &self.output,
                formatter::format(&output).replace("#[r#in]", "#[in]"),
            )?;
        }

        Ok(())
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

fn write_items(
    namespace: &str,
    item: &metadata::reader::Item,
) -> Result<Vec<(String, TokenStream)>, Error> {
    match item {
        metadata::reader::Item::Type(ty) => write_type_def_items(namespace, ty),
        metadata::reader::Item::Fn(ty) => {
            Ok(vec![(ty.name().to_string(), write_fn(namespace, ty)?)])
        }
        metadata::reader::Item::Const(ty) => {
            Ok(vec![(ty.name().to_string(), write_const(namespace, ty)?)])
        }
    }
}

fn write_type_def_items(
    namespace: &str,
    item: &metadata::reader::TypeDef,
) -> Result<Vec<(String, TokenStream)>, Error> {
    match item.category() {
        metadata::reader::TypeCategory::Struct => {
            // A struct with NativeTypedefAttribute is written as `type NAME = TYPE;`.
            if item.attributes().any(|attr| {
                attr.namespace() == "Windows.Win32.Foundation.Metadata"
                    && attr.name() == "NativeTypedefAttribute"
            }) {
                let name = write_ident(item.name());
                let field = item
                    .fields()
                    .next()
                    .ok_or_else(|| writer_err!("typedef `{}` has no field", item.name()))?;
                let ty = write_type(namespace, &field.ty());
                let tokens = quote! { type #name = #ty; };
                return Ok(vec![(item.name().to_string(), tokens)]);
            }
            write_struct_items(item)
        }
        _ => {
            let tokens = write_type_def(item)?;
            if tokens.is_empty() {
                Ok(vec![])
            } else {
                Ok(vec![(item.name().to_string(), tokens)])
            }
        }
    }
}

fn write_const(namespace: &str, item: &metadata::reader::Field) -> Result<TokenStream, Error> {
    match item.ty() {
        metadata::Type::ValueName(tn) if &tn == ("System", "Guid") => {
            write_const_guid(namespace, item)
        }
        _ => write_const_value(namespace, item),
    }
}

fn write_const_value(
    namespace: &str,
    item: &metadata::reader::Field,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let constant = item.constant();
    let ty = write_type(namespace, &item.ty());
    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;

    Ok(if let Some(constant) = constant {
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
    })
}

fn write_const_guid(
    _namespace: &str,
    item: &metadata::reader::Field,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let attribute = item
        .find_attribute("GuidAttribute")
        .ok_or_else(|| writer_err!("GUID constant `{}` has no `GuidAttribute`", item.name()))?;

    let value: u128 = attribute
        .value()
        .into_iter()
        .try_fold(0u128, |acc, (_, val)| match val {
            metadata::Value::U8(x) => Ok((acc << 8) | x as u128),
            metadata::Value::U16(x) => Ok((acc << 16) | x as u128),
            metadata::Value::U32(x) => Ok((acc << 32) | x as u128),
            metadata::Value::U64(x) => Ok((acc << 64) | x as u128),
            _ => Err(writer_err!(
                "unexpected value type in `GuidAttribute` for `{}`",
                item.name()
            )),
        })?;

    let value = format!(
        "0x{:08x}_{:04x}_{:04x}_{:04x}_{:012x}",
        (value >> 96) as u32,
        (value >> 80) as u16,
        (value >> 64) as u16,
        (value >> 48) as u16,
        value as u64 & 0xffffffffffff,
    );

    let literal = syn::LitInt::new(&value, Span::call_site());
    Ok(quote! { const #name: GUID = #literal; })
}

fn write_params(
    namespace: &str,
    method: &metadata::reader::MethodDef,
    signature_types: Vec<metadata::Type>,
) -> Result<Vec<TokenStream>, Error> {
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
                write_custom_attributes(param.attributes(), namespace, method.index())?;
            let ty = write_type(namespace, &ty);
            Ok(quote! { #(#param_attrs)* #in_attr #out_attr #opt_attr #name: #ty })
        })
        .collect()
}

fn write_return_type(
    namespace: &str,
    method: &metadata::reader::MethodDef,
    signature: &metadata::Signature,
) -> Result<TokenStream, Error> {
    let return_attrs: Vec<TokenStream> = method
        .params()
        .find(|p| p.sequence() == 0)
        .map(|p| write_custom_attributes(p.attributes(), namespace, method.index()))
        .transpose()?
        .unwrap_or_default();

    Ok(match &signature.return_type {
        metadata::Type::Void => quote! {},
        ty => {
            let ty = write_type(namespace, ty);
            quote! { -> #(#return_attrs)* #ty }
        }
    })
}

fn write_custom_attributes<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
) -> Result<Vec<TokenStream>, Error> {
    write_custom_attributes_except(attributes, item_namespace, index, &[])
}

fn write_custom_attributes_except<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::TypeIndex,
    exclude: &[&str],
) -> Result<Vec<TokenStream>, Error> {
    attributes
        .filter(|attr| {
            !(namespace_starts_with(attr.namespace(), "System")
                || exclude.contains(&attr.name())
                // `NativeTypedefAttribute` is handled by the typedef writer; skip it here.
                || (attr.namespace() == "Windows.Win32.Foundation.Metadata"
                    && attr.name() == "NativeTypedefAttribute"))
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
            let args: Vec<TokenStream> = attr
                .value()
                .into_iter()
                .map(|(name, v)| {
                    let value_ts = match &v {
                        metadata::Value::EnumValue(tn, inner) => {
                            write_enum_value(item_namespace, tn, inner, index)?
                        }
                        _ => write_value(item_namespace, &v),
                    };
                    let ts = if name.is_empty() {
                        value_ts
                    } else {
                        let name_ident = write_ident(&name);
                        quote! { #name_ident = #value_ts }
                    };
                    Ok(ts)
                })
                .collect::<Result<Vec<_>, Error>>()?;

            Ok(if args.is_empty() {
                quote! { #[#name_ts] }
            } else {
                quote! { #[#name_ts(#(#args),*)] }
            })
        })
        .collect()
}

/// Writes an enum attribute argument as its variant name by looking up the integer
/// value in the TypeIndex.  For flag enums (those with `System.FlagsAttribute`),
/// falls back to decomposing the value into a `Flag1 | Flag2` combination when no
/// exact variant match is found.  Returns an error when the enum type cannot be
/// found in the index (e.g. the winmd that defines it was not provided).
fn write_enum_value(
    namespace: &str,
    tn: &metadata::TypeName,
    inner: &metadata::Value,
    index: &metadata::reader::TypeIndex,
) -> Result<TokenStream, Error> {
    let inner_i32 = match inner {
        metadata::Value::I32(n) => *n,
        _ => return Ok(write_value(namespace, inner)),
    };

    let mut found_in_index = false;
    for typedef in index.get(&tn.namespace, &tn.name) {
        found_in_index = true;
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
                            return Ok(quote! { #variant });
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
                    return Ok(flags_ts);
                }
            }
        }
    }

    if !found_in_index {
        return Err(writer_err!(
            "enum type `{}::{}` not found in the metadata index; ensure the winmd file that defines it is included",
            tn.namespace,
            tn.name
        ));
    }

    Ok(write_value(namespace, inner))
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

fn write_type_def(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    match item.category() {
        // Structs/unions are handled by write_struct_items (which may return
        // multiple flat items) so this branch is never reached from the main
        // write loop.  It is kept for completeness and internal callers.
        metadata::reader::TypeCategory::Struct => Ok(quote! {}),
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

/// Extracts the raw GUID tuple `(data1, data2, data3, data4)` from a `GuidAttribute`.
fn extract_guid_from_attribute(
    attr: metadata::reader::Attribute,
) -> Result<(u32, u16, u16, [u8; 8]), Error> {
    let values: Vec<_> = attr.value().into_iter().map(|(_, v)| v).collect();
    if values.len() != 11 {
        return Err(writer_err!(
            "GuidAttribute must have exactly 11 arguments, got {}",
            values.len()
        ));
    }
    let d1 = match values[0] {
        metadata::Value::U32(v) => v,
        ref v => return Err(writer_err!("GuidAttribute d1: expected U32, got {v:?}")),
    };
    let d2 = match values[1] {
        metadata::Value::U16(v) => v,
        ref v => return Err(writer_err!("GuidAttribute d2: expected U16, got {v:?}")),
    };
    let d3 = match values[2] {
        metadata::Value::U16(v) => v,
        ref v => return Err(writer_err!("GuidAttribute d3: expected U16, got {v:?}")),
    };
    let mut d4 = [0u8; 8];
    for i in 0..8 {
        d4[i] = match values[3 + i] {
            metadata::Value::U8(v) => v,
            ref v => return Err(writer_err!("GuidAttribute d4[{i}]: expected U8, got {v:?}")),
        };
    }
    Ok((d1, d2, d3, d4))
}

/// Describes how a GUID should appear in the RDL output for an interface or delegate.
enum GuidOutput {
    /// The GUID matches what would be automatically derived — omit it from the output.
    Omit,
    /// An explicit GUID is stored that differs from the derived value — emit `#[guid(0x…)]`.
    Explicit(u32, u16, u16, [u8; 8]),
    /// No `GuidAttribute` is present — emit `#[no_guid]` to prevent re-derivation on read-back.
    None,
}

/// Formats GUID components as a UUID-style hex u128 literal, e.g.
/// `0x005023ca_72b1_11d3_9fc4_00c04f79a0a3`.
fn format_guid_u128(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> String {
    let d4_word = u16::from_be_bytes([d4[0], d4[1]]);
    let d4_node = u64::from_be_bytes([0, 0, d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]);
    format!("0x{d1:08x}_{d2:04x}_{d3:04x}_{d4_word:04x}_{d4_node:012x}")
}

/// Core GUID-output logic shared by interfaces and delegates.
///
/// Compares the stored `GuidAttribute` against the value that would be derived from
/// `methods` and returns the appropriate [`GuidOutput`] variant.
fn guid_output(
    item: &metadata::reader::TypeDef,
    methods: &[(&str, &[metadata::Type], &metadata::Type)],
) -> Result<GuidOutput, Error> {
    let Some(attr) = item.find_attribute("GuidAttribute") else {
        return Ok(GuidOutput::None);
    };
    let stored = extract_guid_from_attribute(attr)?;
    let s = crate::reader::guid::build_interface_string(
        item.namespace(),
        metadata::trim_tick(item.name()),
        methods,
    );
    let derived = crate::reader::guid::guid_from_interface_string(&s);
    if stored == derived {
        Ok(GuidOutput::Omit)
    } else {
        Ok(GuidOutput::Explicit(stored.0, stored.1, stored.2, stored.3))
    }
}

/// Determines the GUID output mode for an interface `TypeDef`.
fn interface_guid_output(
    item: &metadata::reader::TypeDef,
    generics: &[metadata::Type],
) -> Result<GuidOutput, Error> {
    let sigs: Vec<(String, Vec<metadata::Type>, metadata::Type)> = item
        .methods()
        .map(|m| {
            let sig = m.signature(generics);
            (m.name().to_string(), sig.types, sig.return_type)
        })
        .collect();
    let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = sigs
        .iter()
        .map(|(n, t, r)| (n.as_str(), t.as_slice(), r))
        .collect();
    guid_output(item, &methods)
}

/// Determines the GUID output mode for a delegate `TypeDef`.
fn delegate_guid_output(
    item: &metadata::reader::TypeDef,
    generics: &[metadata::Type],
) -> Result<GuidOutput, Error> {
    let (types, return_type) = item
        .methods()
        .find(|m| m.name() == "Invoke")
        .map(|invoke| {
            let sig = invoke.signature(generics);
            (sig.types, sig.return_type)
        })
        .unwrap_or_else(|| (vec![], metadata::Type::Void));
    guid_output(item, &[("Invoke", types.as_slice(), &return_type)])
}

/// Reads the calling convention from an `UnmanagedFunctionPointerAttribute`, returning the
/// ABI string (`"C"`, `"fastcall"`, or `"system"` which maps to `None` for callbacks where
/// it is the default, and `Some("system")` for delegates where it must be explicit).
///
/// Returns `None` when no `UnmanagedFunctionPointerAttribute` is present.
fn read_unmanaged_abi(item: &metadata::reader::TypeDef) -> Option<i32> {
    item.find_attribute("UnmanagedFunctionPointerAttribute")
        .and_then(|attribute| attribute.value().into_iter().next())
        .and_then(|(_, v)| {
            if let metadata::Value::EnumValue(_, value) = v {
                if let metadata::Value::I32(n) = *value {
                    return Some(n);
                }
            }
            None
        })
}

/// Collects the generic type parameters of `item` as a `(types, tokens)` pair.
///
/// `types` is the `Vec<Type::Generic>` needed for signature computation; `tokens` is the
/// `<T, U, …>` token stream for the RDL output (empty when there are no generic params).
fn write_generic_params(item: &metadata::reader::TypeDef) -> (Vec<metadata::Type>, TokenStream) {
    let types: Vec<_> = item
        .generic_params()
        .map(|param| metadata::Type::Generic(param.name().to_string(), param.sequence()))
        .collect();
    let tokens = if types.is_empty() {
        quote! {}
    } else {
        let names = item.generic_params().map(|param| write_ident(param.name()));
        quote! { <#(#names),*> }
    };
    (types, tokens)
}
