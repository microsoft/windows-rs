use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

/// A COM interface definition
///
/// # Example
/// ```rust
/// #[interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
#[proc_macro_attribute]
pub fn interface(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let guid = syn::parse_macro_input!(attributes as Guid);
    let interface = syn::parse_macro_input!(original_type as Interface);
    let tokens = match interface.gen_tokens(&guid) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };
    tokens.into()
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

/// Parsed interface
///
/// ```rust
/// #[interface("0000010c-0000-0000-C000-000000000046")]
/// unsafe trait IFoo {}
/// //^ parses this   
/// ```
struct Interface {
    pub visibility: syn::Visibility,
    pub name: syn::Ident,
    pub parent: Option<syn::Path>,
    pub methods: Vec<InterfaceMethod>,
    docs: Vec<syn::Attribute>,
}

impl Interface {
    /// Generates all the code needed for a COM interface
    fn gen_tokens(&self, guid: &Guid) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.visibility;
        let name = &self.name;
        // TODO: support non-IUnknown parents
        let parent = quote!(::windows::core::IUnknown);
        let vtable_name = quote::format_ident!("{}_Vtbl", name);
        let guid = guid.to_tokens()?;
        let implementation = self.gen_implementation();
        let vtable = self.gen_vtable(&vtable_name);
        let conversions = self.gen_conversions();
        Ok(quote! {
            #[repr(transparent)]
            #vis struct #name(#parent);
            #implementation

            unsafe impl ::windows::core::Interface for #name {
                type Vtable = #vtable_name;
                const IID: ::windows::core::GUID = #guid;
            }

            #vtable

            #conversions
        })
    }

    /// Generates the methods users can call on the COM interface pointer
    fn gen_implementation(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let vis = &m.visibility;
                let name = &m.name;

                if m.args.iter().any(|a| !a.pass_through) {
                    panic!("TODO: handle methods with non-pass through arguments");
                }
                let args = m.gen_args();
                let params = &m
                    .args
                    .iter()
                    .map(|a| {
                        let pat = &a.pat;
                        quote! { #pat }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    #vis unsafe fn #name(&self, #(#args)*) -> ::windows::core::HRESULT {
                        (::windows::core::Interface::vtable(self).#name)(::core::mem::transmute_copy(self), #(#params)*)
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl #name {
                #(#methods)*
            }
        }
    }

    /// Generates the vtable for a COM interface
    fn gen_vtable(&self, vtable_name: &syn::Ident) -> proc_macro2::TokenStream {
        let vtable_entries = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                if m.args.iter().any(|a| !a.pass_through) {
                    panic!("TODO: handle methods with non-pass through arguments");
                }
                let args = m.gen_args();
                quote! {
                    pub #name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) -> ::windows::core::HRESULT,
                }
            })
            .collect::<Vec<_>>();
        quote! {
            #[repr(C)]
            #[doc(hidden)]
            pub struct #vtable_name {
                #(#vtable_entries)*
            }
        }
    }

    /// Generates various conversions such as from and to `IUnknown`
    fn gen_conversions(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let name_string = format!("{}", name);
        quote! {
            impl ::core::convert::From<#name> for ::windows::core::IUnknown {
                fn from(value: #name) -> Self {
                    // TODO: handle when direct parent is not IUnknown
                    value.0
                }
            }
            impl ::core::convert::From<&#name> for ::windows::core::IUnknown {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                    ::windows::core::Param::Owned(self.into())
                }
            }
            impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a #name {
                fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                    // TODO: handle when direct parent is not IUnknown
                    ::windows::core::Param::Borrowed(&self.0)
                }
            }
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::core::cmp::Eq for #name {}
            impl ::core::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name_string).field(&self.0).finish()
                }
            }
        }
    }
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

/// Parsed interface guid attribute
///
/// ```rust
/// #[interface("0000010c-0000-0000-C000-000000000046")]
///           //^ parses this   
/// unsafe trait IFoo {}
/// ```
struct Guid(syn::LitStr);

impl Guid {
    /// The various chunks of a COM interface GUID separated by "-"
    fn chunks(&self) -> syn::Result<[String; 5]> {
        fn ensure_length<'a>(part: Option<&'a str>, index: usize, length: usize, span: proc_macro2::Span) -> syn::Result<String> {
            let part = match part {
                Some(p) => p,
                None => return Err(syn::Error::new(span, format!("The IID missing part at index {}", index,))),
            };

            if part.len() != length {
                return Err(syn::Error::new(span, format!("The IID part at index {} must be {} characters long but was {} characters", index, length, part.len())));
            }

            Ok(part.to_owned())
        }

        let guid_value = self.0.value();
        let mut delimited = guid_value.split('-').fuse();
        let chunks = [ensure_length(delimited.next(), 0, 8, self.0.span())?, ensure_length(delimited.next(), 1, 4, self.0.span())?, ensure_length(delimited.next(), 2, 4, self.0.span())?, ensure_length(delimited.next(), 3, 4, self.0.span())?, ensure_length(delimited.next(), 4, 12, self.0.span())?];

        Ok(chunks)
    }

    fn to_tokens(&self) -> syn::Result<proc_macro2::TokenStream> {
        fn hex_lit(num: &str) -> syn::LitInt {
            syn::LitInt::new(&format!("0x{}", num), proc_macro2::Span::call_site())
        }

        let chunks = self.chunks()?;
        let data1 = hex_lit(&chunks[0]);
        let data2 = hex_lit(&chunks[1]);
        let data3 = hex_lit(&chunks[2]);
        let (data4_1, data4_2) = chunks[3].split_at(2);
        let data4_1 = hex_lit(data4_1);
        let data4_2 = hex_lit(data4_2);
        let (data4_3, rest) = chunks[4].split_at(2);
        let data4_3 = hex_lit(data4_3);

        let (data4_4, rest) = rest.split_at(2);
        let data4_4 = hex_lit(data4_4);

        let (data4_5, rest) = rest.split_at(2);
        let data4_5 = hex_lit(data4_5);

        let (data4_6, rest) = rest.split_at(2);
        let data4_6 = hex_lit(data4_6);

        let (data4_7, data4_8) = rest.split_at(2);
        let data4_7 = hex_lit(data4_7);
        let data4_8 = hex_lit(data4_8);
        Ok(quote! {
            ::windows::core::GUID {
                data1: #data1,
                data2: #data2,
                data3: #data3,
                data4: [#data4_1, #data4_2, #data4_3, #data4_4, #data4_5, #data4_6, #data4_7, #data4_8]
            }
        })
    }
}

impl Parse for Guid {
    fn parse(cursor: ParseStream) -> syn::Result<Self> {
        let string: syn::LitStr = cursor.parse()?;

        Ok(Self(string))
    }
}

/// A parsed interface method
///
/// ```rust
/// #[interface("0000010c-0000-0000-C000-000000000046")]
/// unsafe trait IFoo {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
///     //^ parses this   
/// }
/// ```
struct InterfaceMethod {
    pub name: syn::Ident,
    pub visibility: syn::Visibility,
    pub args: Vec<InterfaceMethodArg>,
    pub ret: syn::ReturnType,
    pub docs: Vec<syn::Attribute>,
}

impl InterfaceMethod {
    /// Generates arguments (of the form `$pat: $type`)
    fn gen_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .map(|a| {
                let pat = &a.pat;
                let ty = &a.ty;
                quote! { #pat: #ty }
            })
            .collect::<Vec<_>>()
    }
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
                let pass_through = matches!(&*p.ty, syn::Type::Ptr(_));

                Ok(InterfaceMethodArg { ty: p.ty, pat: p.pat, pass_through })
            })
            .collect::<Result<Vec<InterfaceMethodArg>, syn::Error>>()?;

        let ret = sig.output;
        Ok(InterfaceMethod { name: sig.ident, visibility, args, ret, docs })
    }
}

/// An argument to an interface method
struct InterfaceMethodArg {
    /// The type of the argument
    pub ty: Box<syn::Type>,
    /// The name of the argument
    pub pat: Box<syn::Pat>,
    /// Whether the argument needs transformation before crossing an FFI boundary
    pub pass_through: bool,
}
