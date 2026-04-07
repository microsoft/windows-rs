//! Define COM interfaces to call or implement.
//!
//! Take a look at [macro@interface] for an example.
//!
//! Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
//!
//! # Architecture
//!
//! ## What this macro generates
//!
//! For an interface declared as:
//!
//! ```rust,ignore
//! #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
//! unsafe trait IFoo: IUnknown {
//!     fn GetValue(&self, value: *mut i32) -> HRESULT;
//! }
//! ```
//!
//! The macro emits:
//!
//! - `struct IFoo(IUnknown)` — a `#[repr(transparent)]` struct wrapping the parent.
//! - `unsafe impl Interface for IFoo` — with the IID constant.
//! - `impl Deref for IFoo` — to reach parent-interface methods.
//! - `impl IFoo { fn GetValue(...) }` — safe wrapper that calls through the vtable.
//! - `trait IFoo_Impl: Sized` — the trait that `#[implement]` users must implement on
//!   their `Foo_Impl` type.
//! - `struct IFoo_Vtbl` — the vtable layout, with a `new::<Identity, OFFSET>()` constructor
//!   and a `matches(iid)` helper used by `QueryInterface`.
//! - `Clone`, `PartialEq`, `Eq`, `Debug`, and `From<IFoo> for IUnknown` implementations.
//!
//! ## Naming conventions
//!
//! - `IFoo` — the interface struct, usable as a COM pointer.
//! - `IFoo_Vtbl` — the vtable struct (hidden from docs).
//! - `IFoo_Impl` — the implementation trait; see `windows-implement` for who implements it.
//!
//! ## Relationship to `windows-implement`
//!
//! `#[interface]` and `#[implement]` (in `windows-implement`) must agree on:
//! - The `_Impl` naming convention.
//! - The vtable `new::<Identity, OFFSET>()` constructor signature.
//! - The `matches(iid)` helper used during `QueryInterface`.

use syn::spanned::Spanned;

mod gen;
mod guid;
pub(crate) use guid::Guid;

#[cfg(test)]
mod tests;

/// Defines a COM interface to call or implement.
///
/// # Example
/// ```rust,no_run
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
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// let object: IValue = Value(123).into();
/// // Call interface methods...
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
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
/// //^ this is parsed as an Interface
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
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

/// A single method declaration inside an `#[interface]` trait body.
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
///   //^ this is parsed as an InterfaceMethod
/// }
/// ```
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
