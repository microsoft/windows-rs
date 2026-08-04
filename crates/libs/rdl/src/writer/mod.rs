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
use r#enum::*;
use r#fn::*;
use interface::*;
use layout::*;
use metadata::AsRow;
use metadata::HasAttributes;
use r#struct::*;

#[derive(Default)]
/// Builder that converts `.winmd` metadata into RDL.
pub struct Writer {
    input: Vec<String>,
    input_default: bool,
    input_bytes: Vec<Vec<u8>>,
    filter: Vec<String>,
    output: String,
    split: bool,
    partition: Option<HashMap<String, String>>,
}

impl Writer {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an input `.winmd` file or directory.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    /// Adds a `.winmd` file from memory.
    pub fn input_bytes(&mut self, input: &[u8]) -> &mut Self {
        self.input_bytes.push(input.to_vec());
        self
    }

    /// Adds the default Windows metadata inputs.
    pub fn input_default(&mut self) -> &mut Self {
        self.input_default = true;
        self
    }

    /// Sets the output `.rdl` file or directory path.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    /// Adds multiple input `.winmd` files.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input(input.as_ref());
        }
        self
    }

    /// Adds multiple filter rules. See [`filter`][Self::filter].
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

    /// Adds an include/exclude filter rule. Prefix with `!` to exclude.
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter.push(filter.to_string());
        self
    }

    /// Writes each namespace to a separate file.
    pub fn split(&mut self) -> &mut Self {
        self.split = true;
        self
    }

    /// Partitions output into one `<stem>.rdl` per defining header.
    pub fn partition(&mut self, map: HashMap<String, String>) -> &mut Self {
        self.partition = Some(map);
        self
    }

    /// Converts the inputs and writes the RDL to the configured output.
    pub fn write(&self) -> Result<(), Error> {
        let mut files = vec![];

        for file_name in &expand_input_paths(&self.input, "winmd", ".")?.0 {
            files.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        if self.input_default {
            files.extend(
                [windows_default::WINRT, windows_default::WIN32]
                    .into_iter()
                    .map(|bytes| metadata::reader::File::new(bytes.to_vec()).unwrap()),
            );
        }

        for bytes in &self.input_bytes {
            files.push(
                metadata::reader::File::new(bytes.clone())
                    .ok_or_else(|| Error::new("invalid input", "<memory>", 0, 0))?,
            );
        }

        let index = metadata::reader::Index::new(files);
        let rules = resolve_filter(&self.filter, &index);

        if let Some(map) = &self.partition {
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

            let mut layouts: BTreeMap<String, Layout> = BTreeMap::new();
            for namespace in index.namespaces() {
                if namespace.is_empty() {
                    continue;
                }
                for (name, item) in index.namespace_items(namespace) {
                    if !item_included(&rules, namespace, name) {
                        continue;
                    }
                    let Some(stem) = map.get(name) else {
                        continue;
                    };
                    let layout = layouts.entry(stem.clone()).or_default();
                    for (item_name, tokens) in write_items(namespace, item)? {
                        layout.insert(namespace, &item_name, item_winrt(item), tokens.to_string());
                    }
                }
            }

            for (stem, layout) in &layouts {
                let output = layout.to_string();
                if output.is_empty() {
                    continue;
                }

                let mut path = std::path::PathBuf::new();
                path.push(&self.output);
                path.push(format!("{stem}.rdl"));

                let path_str = path
                    .to_str()
                    .ok_or_else(|| writer_err!("output path contains non-UTF-8 characters"))?;
                write_to_file(path_str, formatter::format(&output))?;
            }

            return Ok(());
        }

        if self.split {
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

            for namespace in index.namespaces() {
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
                write_to_file(path_str, formatter::format(&output))?;
            }
        } else {
            let mut layout = Layout::new();

            for namespace in index.namespaces() {
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
            write_to_file(&self.output, formatter::format(&output))?;
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
    Namespace(String),
    Type(String, String),
}

/// Resolves filter strings as namespace prefixes, qualified names, or unqualified names.
fn resolve_filter(filter: &[String], index: &metadata::reader::Index) -> Vec<(FilterRule, bool)> {
    let mut rules = vec![];

    for f in filter {
        let (rule_str, include) = if let Some(r) = f.strip_prefix('!') {
            (r, false)
        } else {
            (f.as_str(), true)
        };

        if index
            .namespaces()
            .any(|ns| namespace_starts_with(ns, rule_str))
        {
            rules.push((FilterRule::Namespace(rule_str.to_string()), include));
            continue;
        }

        if let Some((namespace, name)) = rule_str.rsplit_once('.')
            && index.get_item(namespace, name).next().is_some()
        {
            rules.push((
                FilterRule::Type(namespace.to_string(), name.to_string()),
                include,
            ));
            continue;
        }

        let mut found = false;
        for ns in index.namespaces() {
            if index.get_item(ns, rule_str).next().is_some() {
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

        rules.push((FilterRule::Namespace(rule_str.to_string()), include));
    }

    rules
}

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

fn item_winrt(item: metadata::reader::Item) -> bool {
    match item {
        metadata::reader::Item::Type(item) => item
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

fn write_items(
    namespace: &str,
    item: metadata::reader::Item,
) -> Result<Vec<(String, TokenStream)>, Error> {
    match item {
        metadata::reader::Item::Type(ty) => write_type_def_items(namespace, &ty),
        metadata::reader::Item::Fn(ty) => {
            Ok(vec![(ty.name().to_string(), write_fn(namespace, &ty)?)])
        }
        metadata::reader::Item::Const(ty) => {
            Ok(vec![(ty.name().to_string(), write_const(namespace, &ty)?)])
        }
    }
}

fn write_type_def_items(
    namespace: &str,
    item: &metadata::reader::TypeDef,
) -> Result<Vec<(String, TokenStream)>, Error> {
    if item.category() == metadata::reader::TypeCategory::Struct {
        // Native typedefs round-trip as `type NAME = TYPE;`.
        if item.attributes().any(|attr| {
            attr.namespace() == METADATA_NAMESPACE && attr.name() == "NativeTypedefAttribute"
        }) {
            let name = write_ident(item.name());
            let field = item
                .fields()
                .next()
                .ok_or_else(|| writer_err!("typedef `{}` has no field", item.name()))?;
            let ty = write_type(namespace, &field.ty());
            let arch_attr = write_arch_attr(item.arches());
            let tokens = quote! { #arch_attr type #name = #ty; };
            return Ok(vec![(item.name().to_string(), tokens)]);
        }
        write_struct_items(item)
    } else {
        let tokens = write_type_def(item)?;
        if tokens.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![(item.name().to_string(), tokens)])
        }
    }
}

fn write_const(namespace: &str, item: &metadata::reader::Field) -> Result<TokenStream, Error> {
    // GUID constants render as inline `0x...` literals, not raw attributes.
    let is_guid = match item.ty() {
        metadata::Type::ValueName(tn) => &tn == ("System", "Guid") || tn.name == "GUID",
        _ => false,
    };
    if is_guid && item.find_attribute("GuidAttribute").is_some() {
        write_const_guid(namespace, item)
    } else if item.find_attribute("GuidAttribute").is_some() {
        // Property-key constants split `fmtid` into GuidAttribute and `pid` into Constant.
        write_const_property_key(namespace, item)
    } else {
        write_const_value(namespace, item)
    }
}

fn write_const_value(
    namespace: &str,
    item: &metadata::reader::Field,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let constant = item.constant();
    let ty = write_type(namespace, &item.ty());
    let arch_attr = write_arch_attr(item.arches());
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["SupportedArchitectureAttribute"],
    )?;

    Ok(if let Some(constant) = constant {
        let value = write_typed_value(namespace, &item.ty(), &constant.value());
        quote! {
            #arch_attr
            #(#custom_attrs)*
            const #name: #ty = #value;
        }
    } else {
        quote! {
            #arch_attr
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
    let arch_attr = write_arch_attr(item.arches());
    let literal = guid_attribute_literal(item)?;
    Ok(quote! { #arch_attr const #name: GUID = #literal; })
}

/// Recombines a property-key constant's `fmtid` and `pid` into RDL form.
fn write_const_property_key(
    namespace: &str,
    item: &metadata::reader::Field,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let ty = write_type(namespace, &item.ty());
    let arch_attr = write_arch_attr(item.arches());
    let guid = guid_attribute_literal(item)?;
    let constant = item
        .constant()
        .ok_or_else(|| writer_err!("property key constant `{}` has no `pid` value", item.name()))?;
    let pid = write_value(namespace, &constant.value());
    Ok(quote! { #arch_attr #[guid(#guid)] const #name: #ty = #pid; })
}

/// Folds the 11-argument `GuidAttribute` into RDL's u128 GUID literal.
fn guid_attribute_literal(item: &metadata::reader::Field) -> Result<syn::LitInt, Error> {
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

    Ok(syn::LitInt::new(&value, Span::call_site()))
}

fn write_params(
    namespace: &str,
    method: &metadata::reader::MethodDef,
    signature_types: Vec<metadata::Type>,
) -> Result<Vec<TokenStream>, Error> {
    let params = method
        .params_by_sequence(signature_types.len())
        .map_err(|error| {
            writer_err!(
                "method `{}` has invalid parameter metadata: {error}",
                method.name()
            )
        })?;

    signature_types
        .into_iter()
        .enumerate()
        .map(|(position, ty)| {
            let param = params.params()[position];
            let is_mutable = matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));
            let direction = param.map_or_else(
                || {
                    if is_mutable {
                        metadata::reader::ParamDirection::Output
                    } else {
                        metadata::reader::ParamDirection::Input
                    }
                },
                |param| param.direction(),
            );
            let (effective_in, has_out) = match direction {
                metadata::reader::ParamDirection::Unspecified
                | metadata::reader::ParamDirection::Input => (true, false),
                metadata::reader::ParamDirection::Output => (false, true),
                metadata::reader::ParamDirection::InputOutput => (true, true),
            };
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
            let opt_attr = if param.is_some_and(|param| param.is_optional()) {
                quote! { #[opt] }
            } else {
                quote! {}
            };
            let name = param.map_or_else(
                || write_ident(&format!("p{position}")),
                |param| write_ident(param.name()),
            );
            let param_attrs = match param {
                Some(param) => write_custom_attributes_except(
                    param.attributes(),
                    namespace,
                    method.index(),
                    &[],
                )?,
                None => Vec::new(),
            };
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
    let params = method
        .params_by_sequence(signature.types.len())
        .map_err(|error| {
            writer_err!(
                "method `{}` has invalid parameter metadata: {error}",
                method.name()
            )
        })?;
    let return_attrs: Vec<TokenStream> = params
        .return_param()
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
    index: &windows_metadata::reader::Index,
) -> Result<Vec<TokenStream>, Error> {
    write_custom_attributes_except(attributes, item_namespace, index, &[])
}

fn write_custom_attributes_except<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    item_namespace: &str,
    index: &windows_metadata::reader::Index,
    exclude: &[&str],
) -> Result<Vec<TokenStream>, Error> {
    let mut rendered = attributes
        .filter(|attr| {
            !(namespace_starts_with(attr.namespace(), "System")
                || exclude.contains(&attr.name())
                || (attr.namespace() == METADATA_NAMESPACE
                    && attr.name() == "NativeTypedefAttribute"))
        })
        .map(|attr| {
            let attr_ns = attr.namespace();
            let values = attr.value();

            // Naturalized metadata attributes render as short RDL spellings when possible.
            let pseudo = if attr_ns == METADATA_NAMESPACE {
                let arg_names: Vec<String> = values.iter().map(|(n, _)| n.clone()).collect();
                pseudo_for_metadata(attr.name(), &arg_names)
            } else {
                None
            };

            let name_ts = if let Some(pseudo) = pseudo {
                write_ident(pseudo.short)
            } else {
                let attr_short = attr
                    .name()
                    .strip_suffix("Attribute")
                    .unwrap_or_else(|| attr.name());

                if attr_ns.is_empty() || attr_ns == item_namespace {
                    write_ident(attr_short)
                } else {
                    let mut tokens = TokenStream::new();
                    for part in attr_ns.split('.') {
                        let ident = write_ident(part);
                        tokens = quote! { #tokens #ident :: };
                    }
                    let short = write_ident(attr_short);
                    quote! { #tokens #short }
                }
            };

            // Property-bound pseudos emit their single property value positionally.
            let drop_names = pseudo.and_then(|p| p.prop).is_some();
            let args: Vec<TokenStream> = values
                .into_iter()
                .map(|(name, v)| {
                    let value_ts = match &v {
                        metadata::Value::EnumValue(tn, inner) => {
                            write_enum_value(item_namespace, tn, inner, index)?
                        }
                        _ => write_value(item_namespace, &v),
                    };
                    let ts = if name.is_empty() || drop_names {
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
        .collect::<Result<Vec<TokenStream>, Error>>()?;

    // Attribute table order varies between metadata builds; sort rendered text for stable output.
    rendered.sort_by_key(|ts| ts.to_string());
    Ok(rendered)
}

/// Renders enum-valued attribute arguments as variants, decomposing flags when needed.
fn write_enum_value(
    namespace: &str,
    tn: &metadata::TypeName,
    inner: &metadata::Value,
    index: &metadata::reader::Index,
) -> Result<TokenStream, Error> {
    let inner_i32 = match inner {
        metadata::Value::I32(n) => *n,
        _ => return Ok(write_value(namespace, inner)),
    };

    let mut found_in_index = false;
    for typedef in index.get(&tn.namespace, &tn.name) {
        found_in_index = true;
        if typedef.category() == metadata::reader::TypeCategory::Enum {
            for field in typedef.fields() {
                if field.flags().contains(metadata::FieldAttributes::Literal)
                    && let Some(constant) = field.constant()
                {
                    let matches = match constant.value() {
                        metadata::Value::I32(v) => v == inner_i32,
                        // Attribute blobs carry enum values as signed integers.
                        metadata::Value::U32(v) => v == inner_i32 as u32,
                        _ => false,
                    };
                    if matches {
                        let variant = write_ident(field.name());
                        return Ok(quote! { #variant });
                    }
                }
            }

            let has_flags = typedef.attributes().any(|attr| {
                attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
            });

            if has_flags
                && let Some(flags_ts) = write_flags_combination(namespace, &typedef, inner_i32)
            {
                return Ok(flags_ts);
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

fn write_flags_combination(
    _namespace: &str,
    typedef: &metadata::reader::TypeDef,
    value: i32,
) -> Option<TokenStream> {
    let mut fields: Vec<(String, i32)> = typedef
        .fields()
        .filter_map(|field| {
            if !field.flags().contains(metadata::FieldAttributes::Literal) {
                return None;
            }
            let constant = field.constant()?;
            let v = match constant.value() {
                metadata::Value::I32(v) => v,
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

    // Prefer composite flags such as `All = 0xFFFFFFFF` over individual bits.
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

/// Emits `#[arch(...)]` for a non-zero X86/X64/Arm64 bitmask.
pub(super) fn write_arch_attr(arches: i32) -> TokenStream {
    if arches == 0 {
        return quote! {};
    }

    let mut parts: Vec<TokenStream> = vec![];
    if arches & 1 != 0 {
        parts.push(quote! { X86 });
    }
    if arches & 2 != 0 {
        parts.push(quote! { X64 });
    }
    if arches & 4 != 0 {
        parts.push(quote! { Arm64 });
    }

    if parts.is_empty() {
        return quote! {};
    }

    let value = parts
        .iter()
        .skip(1)
        .fold(parts[0].clone(), |acc, p| quote! { #acc | #p });

    quote! { #[arch(#value)] }
}

fn write_type_def(item: &metadata::reader::TypeDef) -> Result<TokenStream, Error> {
    match item.category() {
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

fn write_type_ref(namespace: &str, item: &metadata::reader::TypeDefOrRef) -> TokenStream {
    write_type(
        namespace,
        &metadata::Type::class_named(item.namespace(), item.name()),
    )
}

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

enum GuidOutput {
    Omit,
    Explicit(u32, u16, u16, [u8; 8]),
    None,
}

/// Omits derived GUIDs while preserving explicit or absent GUID state.
fn guid_output(
    item: &metadata::reader::TypeDef,
    methods: &[(&str, &[metadata::Type], &metadata::Type)],
) -> Result<GuidOutput, Error> {
    let Some(attr) = item.find_attribute("GuidAttribute") else {
        return Ok(GuidOutput::None);
    };
    let stored = extract_guid_from_attribute(attr)?;
    let s = reader::guid::build_interface_string(
        item.namespace(),
        metadata::trim_tick(item.name()),
        methods,
    );
    let derived = reader::guid::guid_from_interface_string(&s);
    if stored == derived {
        Ok(GuidOutput::Omit)
    } else {
        Ok(GuidOutput::Explicit(stored.0, stored.1, stored.2, stored.3))
    }
}

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

fn delegate_guid_output(
    item: &metadata::reader::TypeDef,
    generics: &[metadata::Type],
) -> Result<GuidOutput, Error> {
    let (types, return_type) = item.methods().find(|m| m.name() == "Invoke").map_or_else(
        || (vec![], metadata::Type::Void),
        |invoke| {
            let sig = invoke.signature(generics);
            (sig.types, sig.return_type)
        },
    );
    guid_output(item, &[("Invoke", types.as_slice(), &return_type)])
}

/// Reads the raw Win32 callback ABI value.
fn read_unmanaged_abi(item: &metadata::reader::TypeDef) -> Option<i32> {
    item.find_attribute("UnmanagedFunctionPointerAttribute")
        .and_then(|attribute| attribute.value().into_iter().next())
        .and_then(|(_, v)| {
            if let metadata::Value::EnumValue(_, value) = v
                && let metadata::Value::I32(n) = *value
            {
                return Some(n);
            }
            None
        })
}

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
