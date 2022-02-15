use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use tokens::quote;

#[proc_macro_attribute]
pub fn interface(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as Guid);
    let typ = syn::parse_macro_input!(original_type as Interface);
    let tokens = quote! {
        compile_error!("`interface` macro is not yet implemented");
    };
    tokens.parse::<proc_macro::TokenStream>().unwrap()
}
macro_rules! bail {
    ($item:expr, $($msg:tt),*) => {
        return Err(syn::Error::new($item.span(), std::fmt::format(format_args!($($msg),*))));
    };

}

macro_rules! unexpected_token {
    ($item:expr, $msg:expr) => {
        if let Some(i) = $item {
            bail!(i, "unexpected {}", $msg);
        }
    };
}
macro_rules! expected_token {
    ($sig:tt.$item:tt(), $msg:expr) => {
        if let None = $sig.$item() {
            bail!($sig, "expected {}", $msg);
        }
    };
}

/// Parsed interface guid attribute
///
/// ```rust
/// #[interface("0000010c-0000-0000-C000-000000000046")]
///           //^ this function parses this   
/// unsafe trait IFoo {}
/// ```
struct Guid(syn::LitStr);

impl Parse for Guid {
    fn parse(cursor: ParseStream) -> syn::Result<Self> {
        let string: syn::LitStr = cursor.parse()?;

        Ok(Self(string))
    }
}

/// Parsed interface
///
/// ```rust
/// #[interface("0000010c-0000-0000-C000-000000000046")]
/// unsafe trait IFoo {}
/// //^ this function parses this   
/// ```
struct Interface {
    pub visibility: syn::Visibility,
    pub name: syn::Ident,
    pub parent: Option<syn::Path>,
    pub methods: Vec<InterfaceMethod>,
    docs: Vec<syn::Attribute>,
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let mut docs = Vec::new();
        for attr in attributes.into_iter() {
            let path = &attr.path;
            let tokens = &attr.tokens;
            if path.is_ident("doc") {
                docs.push(attr);
            } else {
                return Err(syn::Error::new(path.span(), "Unrecognized attribute "));
            }
        }

        let visibility = input.parse::<syn::Visibility>()?;
        let _ = input.parse::<syn::Token![unsafe]>()?;
        let interface = input.parse::<syn::Token![trait]>()?;
        let name = input.parse::<syn::Ident>()?;
        let mut parent = None;
        if name != "IUnknown" {
            let _ = input.parse::<syn::Token![:]>().map_err(|_| syn::Error::new(name.span(), format!("Interfaces must inherit from another interface like so: `interface {}: IParentInterface`", name)))?;
            parent = Some(input.parse::<syn::Path>()?);
        }
        let content;
        syn::braced!(content in input);
        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse::<InterfaceMethod>()?);
        }
        Ok(Self { visibility, methods, name, parent, docs })
    }
}

struct InterfaceMethod {
    pub name: syn::Ident,
    pub visibility: syn::Visibility,
    pub args: Vec<InterfaceMethodArg>,
    pub ret: syn::ReturnType,
    pub docs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for InterfaceMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let docs = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.parse::<syn::Visibility>()?;
        let method = input.parse::<syn::TraitItemMethod>()?;
        unexpected_token!(docs.iter().find(|a| !a.path.is_ident("doc")), "attribute");
        unexpected_token!(method.default, "default method implementation");
        let sig = method.sig;
        unexpected_token!(sig.abi, "abi declaration");
        unexpected_token!(sig.asyncness, "async declaration");
        unexpected_token!(sig.generics.params.iter().next(), "generics declaration");
        unexpected_token!(sig.constness, "const declaration");
        expected_token!(sig.receiver(), "the method to have &self as its first argument");
        unexpected_token!(sig.variadic, "variadic args");
        let args = sig
            .inputs
            .into_iter()
            .filter_map(|a| match a {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(p) => Some(p),
            })
            .map(|p| {
                let mut filter = p.attrs.iter().filter(|a| a.path.is_ident("pass_through")).fuse();
                let pass_through = filter.next().is_some();

                unexpected_token!(filter.next(), "function attribute");
                Ok(InterfaceMethodArg { ty: p.ty, pat: p.pat, pass_through })
            })
            .collect::<Result<Vec<InterfaceMethodArg>, syn::Error>>()?;

        let ret = sig.output;
        Ok(InterfaceMethod { name: sig.ident, visibility, args, ret, docs })
    }
}

struct InterfaceMethodArg {
    pub ty: Box<syn::Type>,
    pub pat: Box<syn::Pat>,
    pub pass_through: bool,
}
