use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitByteStr, LitStr, Result, Token, parenthesized};

/// Defines a static TraceLogging provider.
///
/// The provider name and identifier become compile-time provider metadata.
///
/// ```ignore
/// define_provider!(
///     SAMPLE_PROVIDER,
///     "WindowsTracingSample",
///     id(GUID::from_u128(0x4bd2826e_54a1_4ba9_bf63_92b73ea1ac4a))
/// );
/// ```
#[proc_macro]
pub fn define_provider(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<ProviderInput>(input) {
        Ok(input) => match input.expand() {
            Ok(tokens) => tokens.into(),
            Err(error) => error.into_compile_error().into(),
        },
        Err(error) => error.into_compile_error().into(),
    }
}

/// Writes a statically defined TraceLogging event.
///
/// `level`, `keyword`, and `id_version` configure the event descriptor. Supported fields are
/// `bool`, signed and unsigned integers through 64 bits, `f32`, `f64`, `guid`, `hresult`,
/// `win32_error`, `str`, `utf16`, and `binary`.
///
/// ```ignore
/// write_event!(
///     SAMPLE_PROVIDER,
///     "Started",
///     level(Level::INFORMATIONAL),
///     keyword(0x1),
///     str("Message", "Hello from Rust"),
///     u32("ProcessId", std::process::id()),
/// );
/// ```
///
/// Field expressions are evaluated only when the event passes the provider's level and keyword
/// filters.
#[proc_macro]
pub fn write_event(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<EventInput>(input) {
        Ok(input) => match input.expand() {
            Ok(tokens) => tokens.into(),
            Err(error) => error.into_compile_error().into(),
        },
        Err(error) => error.into_compile_error().into(),
    }
}

struct ProviderInput {
    symbol: Ident,
    name: LitStr,
    id: Expr,
}

impl Parse for ProviderInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let symbol = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let option = input.call(Ident::parse_any)?;
        if option != "id" {
            return Err(syn::Error::new(option.span(), "expected id(...)"));
        }

        let content;
        parenthesized!(content in input);
        let id = content.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        if !input.is_empty() {
            return Err(input.error("unexpected provider option"));
        }

        Ok(Self { symbol, name, id })
    }
}

impl ProviderInput {
    fn expand(self) -> Result<TokenStream> {
        let Self { symbol, name, id } = self;
        let metadata = provider_metadata(&name)?;
        let metadata = LitByteStr::new(&metadata, name.span());

        Ok(quote! {
            static #symbol: ::windows_tracing::Provider =
                ::windows_tracing::Provider::__new(#id, #name, #metadata);
        })
    }
}

struct EventInput {
    provider: Ident,
    name: LitStr,
    id: Expr,
    version: Expr,
    level: Expr,
    keyword: Expr,
    fields: Vec<Field>,
}

impl Parse for EventInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let provider = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;

        let mut id = syn::parse_quote!(0u16);
        let mut version = syn::parse_quote!(0u8);
        let mut level = syn::parse_quote!(::windows_tracing::Level::VERBOSE);
        let mut keyword = syn::parse_quote!(0u64);
        let mut fields = Vec::new();
        let mut has_id_version = false;
        let mut has_level = false;
        let mut has_keyword = false;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            let option = input.call(Ident::parse_any)?;
            let content;
            parenthesized!(content in input);

            match option.to_string().as_str() {
                "id_version" => {
                    if has_id_version {
                        return Err(syn::Error::new(option.span(), "duplicate id_version"));
                    }
                    has_id_version = true;
                    id = content.parse()?;
                    content.parse::<Token![,]>()?;
                    version = content.parse()?;
                }
                "level" => {
                    if has_level {
                        return Err(syn::Error::new(option.span(), "duplicate level"));
                    }
                    has_level = true;
                    level = content.parse()?;
                }
                "keyword" => {
                    if has_keyword {
                        return Err(syn::Error::new(option.span(), "duplicate keyword"));
                    }
                    has_keyword = true;
                    keyword = content.parse()?;
                }
                _ => fields.push(Field::parse(option, &content)?),
            }

            if !content.is_empty() {
                return Err(content.error("unexpected option argument"));
            }
        }

        Ok(Self {
            provider,
            name,
            id,
            version,
            level,
            keyword,
            fields,
        })
    }
}

impl EventInput {
    fn expand(self) -> Result<TokenStream> {
        let metadata = event_metadata(&self.name.value(), &self.fields)?;
        if metadata.len() > u16::MAX as usize {
            return Err(syn::Error::new(
                self.name.span(),
                "event metadata exceeds the TraceLogging limit",
            ));
        }

        let descriptor_count = 2 + self
            .fields
            .iter()
            .map(|field| field.kind.descriptor_count())
            .sum::<usize>();
        if descriptor_count > 128 {
            return Err(syn::Error::new(
                self.name.span(),
                "event requires more than 128 data descriptors",
            ));
        }

        let metadata = LitByteStr::new(&metadata, self.name.span());
        let provider = self.provider;
        let id = self.id;
        let version = self.version;
        let level = self.level;
        let keyword = self.keyword;

        let mut bindings = Vec::new();
        let mut descriptors = vec![
            quote!(::windows_tracing::EventDataDescriptor::__provider(
                #provider.__metadata()
            )),
            quote!(::windows_tracing::EventDataDescriptor::__event(#metadata)),
        ];

        for (index, field) in self.fields.into_iter().enumerate() {
            let generated = field.expand(index);
            bindings.extend(generated.bindings);
            descriptors.extend(generated.descriptors);
        }

        Ok(quote! {
            loop {
                let __windows_tracing_level: ::windows_tracing::Level = #level;
                let __windows_tracing_keyword: u64 = #keyword;
                if !#provider.enabled(__windows_tracing_level, __windows_tracing_keyword) {
                    break ::windows_tracing::WIN32_ERROR(0);
                }

                #(#bindings)*

                let __windows_tracing_descriptor =
                    ::windows_tracing::EventDescriptor::__new(
                        #id,
                        #version,
                        __windows_tracing_level,
                        __windows_tracing_keyword,
                    );
                let __windows_tracing_data = [#(#descriptors),*];
                break #provider.__write(
                    &__windows_tracing_descriptor,
                    &__windows_tracing_data,
                );
            }
        })
    }
}

struct Field {
    name: LitStr,
    value: Expr,
    kind: FieldKind,
}

impl Field {
    fn parse(kind: Ident, input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;
        let value = input.parse()?;
        let kind = FieldKind::parse(&kind)?;
        Ok(Self { name, value, kind })
    }

    fn expand(self, index: usize) -> GeneratedField {
        let value = format_ident!("__windows_tracing_field_{index}");
        let length = format_ident!("__windows_tracing_length_{index}");
        let expression = self.value;
        let ty = self.kind.rust_type();

        match self.kind {
            FieldKind::Str8 => GeneratedField {
                bindings: vec![
                    quote!(let #value: &str = #expression;),
                    checked_length(&length, quote!(#value.len())),
                ],
                descriptors: vec![
                    quote!(::windows_tracing::EventDataDescriptor::__value(&#length)),
                    quote!(::windows_tracing::EventDataDescriptor::__data(#value.as_bytes())),
                ],
            },
            FieldKind::Str16 | FieldKind::Binary => {
                let bytes = if self.kind == FieldKind::Str16 {
                    quote!(::core::mem::size_of_val(#value))
                } else {
                    quote!(#value.len())
                };
                GeneratedField {
                    bindings: vec![
                        quote!(let #value: #ty = #expression;),
                        checked_length(&length, bytes),
                    ],
                    descriptors: vec![
                        quote!(::windows_tracing::EventDataDescriptor::__value(&#length)),
                        quote!(::windows_tracing::EventDataDescriptor::__data(#value)),
                    ],
                }
            }
            FieldKind::Bool32 => {
                let encoded = format_ident!("__windows_tracing_encoded_{index}");
                GeneratedField {
                    bindings: vec![
                        quote!(let #value: bool = #expression;),
                        quote!(let #encoded: i32 = #value.into();),
                    ],
                    descriptors: vec![
                        quote!(::windows_tracing::EventDataDescriptor::__value(&#encoded)),
                    ],
                }
            }
            _ => GeneratedField {
                bindings: vec![quote!(let #value: #ty = #expression;)],
                descriptors: vec![quote!(::windows_tracing::EventDataDescriptor::__value(&#value))],
            },
        }
    }
}

struct GeneratedField {
    bindings: Vec<TokenStream>,
    descriptors: Vec<TokenStream>,
}

fn checked_length(name: &Ident, length: TokenStream) -> TokenStream {
    quote! {
        let #name = match u16::try_from(#length) {
            Ok(value) => value,
            Err(_) => break ::windows_tracing::WIN32_ERROR(234),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    Bool32,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Guid,
    HResult,
    Win32Error,
    Str8,
    Str16,
    Binary,
}

impl FieldKind {
    fn parse(name: &Ident) -> Result<Self> {
        match name.to_string().as_str() {
            "bool" => Ok(Self::Bool32),
            "i8" => Ok(Self::I8),
            "u8" => Ok(Self::U8),
            "i16" => Ok(Self::I16),
            "u16" => Ok(Self::U16),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" => Ok(Self::I64),
            "u64" => Ok(Self::U64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "guid" => Ok(Self::Guid),
            "hresult" => Ok(Self::HResult),
            "win32_error" => Ok(Self::Win32Error),
            "str" => Ok(Self::Str8),
            "utf16" => Ok(Self::Str16),
            "binary" => Ok(Self::Binary),
            _ => Err(syn::Error::new(name.span(), "unsupported tracing option")),
        }
    }

    fn metadata(self) -> (u8, Option<u8>) {
        match self {
            Self::Bool32 => (13, None),
            Self::I8 => (3, None),
            Self::U8 => (4, None),
            Self::I16 => (5, None),
            Self::U16 => (6, None),
            Self::I32 => (7, None),
            Self::U32 => (8, None),
            Self::I64 => (9, None),
            Self::U64 => (10, None),
            Self::F32 => (11, None),
            Self::F64 => (12, None),
            Self::Guid => (15, None),
            Self::HResult => (7, Some(15)),
            Self::Win32Error => (8, Some(13)),
            Self::Str8 => (23, Some(35)),
            Self::Str16 => (22, None),
            Self::Binary => (14, None),
        }
    }

    fn rust_type(self) -> TokenStream {
        match self {
            Self::Bool32 => quote!(bool),
            Self::I8 => quote!(i8),
            Self::U8 => quote!(u8),
            Self::I16 => quote!(i16),
            Self::U16 => quote!(u16),
            Self::I32 => quote!(i32),
            Self::U32 => quote!(u32),
            Self::I64 => quote!(i64),
            Self::U64 => quote!(u64),
            Self::F32 => quote!(f32),
            Self::F64 => quote!(f64),
            Self::Guid => quote!(::windows_tracing::GUID),
            Self::HResult => quote!(::windows_tracing::HRESULT),
            Self::Win32Error => quote!(::windows_tracing::WIN32_ERROR),
            Self::Str8 => quote!(&str),
            Self::Str16 => quote!(&[u16]),
            Self::Binary => quote!(&[u8]),
        }
    }

    fn descriptor_count(self) -> usize {
        match self {
            Self::Str8 | Self::Str16 | Self::Binary => 2,
            _ => 1,
        }
    }
}

fn provider_metadata(name: &LitStr) -> Result<Vec<u8>> {
    let value = name.value();
    if value.is_empty() {
        return Err(syn::Error::new(
            name.span(),
            "provider name must not be empty",
        ));
    }
    if value.contains('\0') {
        return Err(syn::Error::new(
            name.span(),
            "provider name must not contain a null character",
        ));
    }

    let mut metadata = vec![0, 0];
    metadata.extend_from_slice(value.as_bytes());
    metadata.push(0);
    if metadata.len() > u16::MAX as usize {
        return Err(syn::Error::new(
            name.span(),
            "provider metadata exceeds the TraceLogging limit",
        ));
    }
    let length = metadata.len() as u16;
    metadata[..2].copy_from_slice(&length.to_le_bytes());
    Ok(metadata)
}

fn event_metadata(name: &str, fields: &[Field]) -> Result<Vec<u8>> {
    if name.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "event name must not be empty",
        ));
    }
    if name.contains('\0') {
        return Err(syn::Error::new(
            Span::call_site(),
            "event name must not contain a null character",
        ));
    }

    let mut metadata = vec![0, 0, 0];
    metadata.extend_from_slice(name.as_bytes());
    metadata.push(0);

    for field in fields {
        let name = field.name.value();
        if name.is_empty() {
            return Err(syn::Error::new(
                field.name.span(),
                "field name must not be empty",
            ));
        }
        if name.contains('\0') {
            return Err(syn::Error::new(
                field.name.span(),
                "field name must not contain a null character",
            ));
        }
        metadata.extend_from_slice(name.as_bytes());
        metadata.push(0);
        let (input, output) = field.kind.metadata();
        metadata.push(if output.is_some() {
            input | 0x80
        } else {
            input
        });
        if let Some(output) = output {
            metadata.push(output);
        }
    }

    let length = metadata.len() as u16;
    metadata[..2].copy_from_slice(&length.to_le_bytes());
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_encoding() {
        assert_eq!(
            provider_metadata(&LitStr::new("Provider", Span::call_site())).unwrap(),
            b"\x0b\0Provider\0".as_slice()
        );
    }

    #[test]
    fn event_encoding() {
        let fields = vec![
            Field {
                name: LitStr::new("Name", Span::call_site()),
                value: syn::parse_quote!("value"),
                kind: FieldKind::Str8,
            },
            Field {
                name: LitStr::new("Count", Span::call_site()),
                value: syn::parse_quote!(1),
                kind: FieldKind::U32,
            },
        ];
        assert_eq!(
            event_metadata("Event", &fields).unwrap(),
            b"\x17\0\0Event\0Name\0\x97\x23Count\0\x08".as_slice()
        );
    }
}
