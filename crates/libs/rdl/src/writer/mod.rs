mod attribute;
mod class;
mod delegate;
mod r#enum;
mod interface;
mod layout;
mod r#struct;

use super::*;
use attribute::*;
use class::*;
use delegate::*;
use interface::*;
use layout::*;
use metadata::AsRow;
use metadata::HasAttributes;
use proc_macro2::*;
use quote::*;
use r#enum::*;
use r#struct::*;
use windows_metadata as metadata;

// TODO: the writer is primarily an internal tool as most developers will write their own
// definitions or just accept whatever a component author provides. This is thus mostly for
// generating rdl for backfilling definitions and for testing.

#[derive(Default)]
pub struct Writer {
    input: Vec<String>,
    reference: Vec<String>,
    output: String,
    namespace: String,
    recursive: bool,
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

    pub fn reference(&mut self, reference: &str) -> &mut Self {
        self.reference.push(reference.to_string());
        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn recursive(&mut self) -> &mut Self {
        self.recursive = true;
        self
    }

    pub fn split(&mut self) -> &mut Self {
        self.split = true;
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut input = vec![];

        for file_name in &expand_files(&self.input, "winmd")? {
            input.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid input", file_name, 0, 0))?,
            );
        }

        for file_name in &expand_files(&self.reference, "winmd")? {
            input.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid reference", file_name, 0, 0))?,
            );
        }

        let index = metadata::reader::TypeIndex::new(input);
        let index = metadata::reader::ItemIndex::new(&index);

        if self.split {
            for namespace in index.keys() {
                if !namespace.is_empty() && !namespace_starts_with(namespace, &self.namespace) {
                    continue;
                }

                let mut layout = Layout::new();

                for (name, item) in index.namespace_items(namespace) {
                    layout.insert(
                        namespace,
                        name,
                        item_arches(item),
                        item_winrt(item),
                        write(namespace, item).to_string(),
                    );
                }

                let output = layout.to_string();

                let mut path = std::path::PathBuf::new();
                path.push(&self.output);
                path.push(format!("{namespace}.rdl"));

                write_to_file(path.to_str().unwrap(), formatter::format(&output));
            }
        } else {
            let mut layout = Layout::new();

            for namespace in index.keys() {
                if !self.namespace.is_empty() {
                    if self.recursive {
                        if !namespace_starts_with(namespace, &self.namespace) {
                            continue;
                        }
                    } else if *namespace != self.namespace {
                        continue;
                    }
                }

                for (name, item) in index.namespace_items(namespace) {
                    layout.insert(
                        namespace,
                        name,
                        item_arches(item),
                        item_winrt(item),
                        write(namespace, item).to_string(),
                    );
                }
            }

            let output = layout.to_string();
            write_to_file(&self.output, formatter::format(&output));
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

fn item_arches(item: &metadata::reader::Item) -> i32 {
    match item {
        metadata::reader::Item::Type(ty) => ty.arches(),
        metadata::reader::Item::Fn(ty) => ty.arches(),
        metadata::reader::Item::Const(ty) => ty.arches(),
    }
}

fn item_winrt(item: &metadata::reader::Item) -> bool {
    match item {
        metadata::reader::Item::Type(item) => item
            .flags()
            .contains(metadata::TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

fn write(namespace: &str, item: &metadata::reader::Item) -> TokenStream {
    match item {
        metadata::reader::Item::Type(ty) => write_type_def(ty),
        metadata::reader::Item::Fn(ty) => write_fn(namespace, ty),
        metadata::reader::Item::Const(ty) => write_const(namespace, ty),
    }
}

fn write_const(namespace: &str, item: &metadata::reader::Field) -> TokenStream {
    match item.ty() {
        metadata::Type::Name(tn) if &tn == ("System", "Guid") => write_const_guid(namespace, item),
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

fn write_return_type(namespace: &str, signature: &metadata::Signature) -> TokenStream {
    match &signature.return_type {
        metadata::Type::Void => quote! {},
        ty => {
            let ty = write_type(namespace, ty);
            quote! { -> #ty }
        }
    }
}

fn write_fn(namespace: &str, item: &metadata::reader::MethodDef) -> TokenStream {
    let name = write_ident(item.name());
    let signature = item.signature(&[]);

    let return_type = write_return_type(namespace, &signature);
    let params = item.params().filter(|param| param.sequence() != 0);

    let params = params.zip(signature.types).map(|(param, ty)| {
        let name = write_ident(param.name());
        let ty = write_type(namespace, &ty);
        quote! { #name: #ty }
    });

    let Some(impl_map) = item.impl_map() else {
        todo!()
    };

    let scope = impl_map.import_scope();
    let library = scope.name();
    let flags = impl_map.flags();

    let abi = if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
        "system"
    } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
        "C"
    } else {
        todo!()
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! {
        #(#custom_attrs)*
        #[link(name = #library, abi = #abi)]
        fn #name(#(#params),*) #return_type;
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
        .filter(|attr| !exclude.contains(&attr.name()))
        .map(|attr| {
            let attr_ns = attr.ctor().parent().namespace();
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
        metadata::reader::TypeCategory::Struct => write_struct(item),
        metadata::reader::TypeCategory::Enum => write_enum(item),
        metadata::reader::TypeCategory::Interface => write_interface(item),
        metadata::reader::TypeCategory::Class => write_class(item),
        metadata::reader::TypeCategory::Delegate => write_delegate(item),
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
        metadata::Value::TypeName(tn) => write_type(namespace, &metadata::Type::Name(tn.clone())),
        metadata::Value::EnumValue(_, inner) => write_value(namespace, inner),
    }
}

fn write_type_ref(namespace: &str, item: &metadata::reader::TypeDefOrRef) -> TokenStream {
    write_type(
        namespace,
        &metadata::Type::named(item.namespace(), item.name()),
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
        Object => quote! { IInspectable },
        Name(tn) if tn == ("System", "Type") => quote! { Type },
        Name(tn) if tn == ("System", "Guid") => quote! { GUID },
        Name(tn) if tn == ("Windows.Foundation", "HResult") => quote! { HRESULT },

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
        Name(type_name) => {
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
