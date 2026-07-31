//! Define COM interfaces to call or implement.
//!
//! See [`interface`] for an example.

use syn::spanned::Spanned;

mod generation;
mod guid;
pub(crate) use guid::Guid;

#[cfg(test)]
mod tests;

/// Defines a COM interface to call or implement.
///
/// ```
/// use windows_core::*;
///
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value_Impl {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         unsafe { *value = self.0 };
///         HRESULT(0)
///     }
/// }
///
/// let _: IValue = Value(123).into();
/// ```
#[proc_macro_attribute]
pub fn interface(
    attributes: proc_macro::TokenStream,
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    interface_core(attributes.into(), original_type.into()).into()
}

fn interface_core(
    attributes: proc_macro2::TokenStream,
    item_tokens: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let guid = match syn::parse2::<Guid>(attributes) {
        Ok(g) => g,
        Err(e) => return e.into_compile_error(),
    };
    let interface = match syn::parse2::<Interface>(item_tokens) {
        Ok(i) => i,
        Err(e) => return e.into_compile_error(),
    };
    match interface.gen_tokens(&guid) {
        Ok(t) => t,
        Err(e) => e.into_compile_error(),
    }
}

/// A parsed `#[interface]` trait definition.
pub(crate) struct Interface {
    pub(crate) visibility: syn::Visibility,
    pub(crate) name: syn::Ident,
    pub(crate) parent: Option<syn::Path>,
    pub(crate) methods: Vec<InterfaceMethod>,
    pub(crate) docs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let mut docs = Vec::new();
        for attr in attributes.into_iter() {
            let path = attr.path();
            if path.is_ident("doc") {
                docs.push(attr);
            } else {
                return Err(syn::Error::new(path.span(), "Unrecognized attribute"));
            }
        }

        let visibility = input.parse::<syn::Visibility>()?;
        _ = input.parse::<syn::Token![unsafe]>()?;
        _ = input.parse::<syn::Token![trait]>()?;
        let name = input.parse::<syn::Ident>()?;
        _ = input.parse::<syn::Token![:]>();
        let parent = input.parse::<syn::Path>().ok();
        let content;
        syn::braced!(content in input);
        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse::<InterfaceMethod>()?);
        }
        Ok(Self {
            visibility,
            methods,
            name,
            parent,
            docs,
        })
    }
}

/// A method declaration inside an `#[interface]` trait.
pub(crate) struct InterfaceMethod {
    pub(crate) name: syn::Ident,
    pub(crate) visibility: syn::Visibility,
    pub(crate) args: Vec<InterfaceMethodArg>,
    pub(crate) ret: syn::ReturnType,
    pub(crate) docs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for InterfaceMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let docs = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.parse::<syn::Visibility>()?;
        let method = input.parse::<syn::TraitItemFn>()?;

        // Reject non-doc attributes.
        if let Some(i) = docs.iter().find(|a| !a.path().is_ident("doc")) {
            return Err(syn::Error::new(i.span(), "unexpected attribute"));
        }
        // Reject default method bodies.
        if let Some(i) = &method.default {
            return Err(syn::Error::new(
                i.span(),
                "unexpected default method implementation",
            ));
        }

        let sig = method.sig;

        // Reject unsupported function-signature features.
        if let Some(i) = &sig.abi {
            return Err(syn::Error::new(i.span(), "unexpected abi declaration"));
        }
        if let Some(i) = &sig.asyncness {
            return Err(syn::Error::new(i.span(), "unexpected async declaration"));
        }
        if let Some(i) = sig.generics.params.iter().next() {
            return Err(syn::Error::new(i.span(), "unexpected generics declaration"));
        }
        if let Some(i) = &sig.constness {
            return Err(syn::Error::new(i.span(), "unexpected const declaration"));
        }
        if sig.receiver().is_none() {
            return Err(syn::Error::new(
                sig.span(),
                "expected the method to have &self as its first argument",
            ));
        }
        if let Some(i) = &sig.variadic {
            return Err(syn::Error::new(i.span(), "unexpected variadic args"));
        }

        let args = sig
            .inputs
            .into_iter()
            .filter_map(|a| match a {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(p) => Some(p),
            })
            .map(|p| {
                Ok(InterfaceMethodArg {
                    ty: p.ty,
                    pat: p.pat,
                })
            })
            .collect::<Result<Vec<InterfaceMethodArg>, syn::Error>>()?;

        let ret = sig.output;
        Ok(Self {
            name: sig.ident,
            visibility,
            args,
            ret,
            docs,
        })
    }
}

/// A single argument in an [`InterfaceMethod`].
pub(crate) struct InterfaceMethodArg {
    /// The type of the argument.
    pub(crate) ty: Box<syn::Type>,
    /// The pattern (name) of the argument.
    pub(crate) pat: Box<syn::Pat>,
}
