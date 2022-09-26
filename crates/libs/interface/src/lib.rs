use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

/// A COM interface definition
///
/// # Example
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
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
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
/// //^ parses this   
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Interface {
    visibility: syn::Visibility,
    name: syn::Ident,
    parent: Option<syn::Path>,
    methods: Vec<InterfaceMethod>,
    docs: Vec<syn::Attribute>,
}

impl Interface {
    /// Generates all the code needed for a COM interface
    fn gen_tokens(&self, guid: &Guid) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.visibility;
        let name = &self.name;
        let docs = &self.docs;
        let parent = self.parent_type();
        let vtable_name = quote::format_ident!("{}_Vtbl", name);
        let guid = guid.to_tokens()?;
        let implementation = self.gen_implementation();
        let com_trait = self.get_com_trait();
        let vtable = self.gen_vtable(&vtable_name);
        let conversions = self.gen_conversions();

        Ok(quote! {
            #[repr(transparent)]
            #(#docs)*
            #vis struct #name(#parent);
            #implementation
            unsafe impl ::windows::core::Vtable for #name {
                type Vtable = #vtable_name;
            }
            unsafe impl ::windows::core::Interface for #name {
                const IID: ::windows::core::GUID = #guid;
            }
            impl ::windows::core::RuntimeName for #name {}

            #com_trait
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

                let args = m.gen_args();
                let params = &m
                    .args
                    .iter()
                    .map(|a| {
                        let pat = &a.pat;
                        quote! { #pat }
                    })
                    .collect::<Vec<_>>();
                let ret = &m.ret;
                quote! {
                    #vis unsafe fn #name(&self, #(#args),*) #ret {
                        (::windows::core::Vtable::vtable(self).#name)(::windows::core::Vtable::as_raw(self), #(#params),*)
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

    fn get_com_trait(&self) -> proc_macro2::TokenStream {
        let name = quote::format_ident!("{}_Impl", self.name);
        let vis = &self.visibility;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let docs = &m.docs;
                let args = m.gen_args();
                let ret = &m.ret;
                quote! {
                    #(#docs)*
                    unsafe fn #name(&self, #(#args),*) #ret;
                }
            })
            .collect::<Vec<_>>();
        let parent = self.parent_trait_constraint();

        quote! {
            #[allow(non_camel_case_types)]
            #vis trait #name: Sized + #parent {
                #(#methods)*
            }
        }
    }

    /// Generates the vtable for a COM interface
    fn gen_vtable(&self, vtable_name: &syn::Ident) -> proc_macro2::TokenStream {
        let vis = &self.visibility;
        let name = &self.name;
        let trait_name = quote::format_ident!("{}_Impl", name);
        let implvtbl_name = quote::format_ident!("{}_ImplVtbl", name);

        let vtable_entries = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let ret = &m.ret;
                let args = m.gen_args();
                quote! {
                    pub #name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) #ret,
                }
            })
            .collect::<Vec<_>>();

        let parent_vtable_generics = if self.parent_is_iunknown() { quote!(Identity, OFFSET) } else { quote!(Identity, Impl, OFFSET) };
        let parent_vtable = self.parent_vtable();

        let functions = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                let args = m.gen_args();
                let params = &m
                    .args
                    .iter()
                    .map(|a| {
                        let pat = &a.pat;
                        quote! { #pat }
                    })
                    .collect::<Vec<_>>();
                let ret = &m.ret;
                if parent_vtable.is_some() {
                    quote! {
                        unsafe extern "system" fn #name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #trait_name, const OFFSET: isize>(this: *mut ::core::ffi::c_void, #(#args),*) #ret {
                            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
                            let this = (*this).get_impl();
                            this.#name(#(#params),*).into()
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #name<Impl: #trait_name>(this: *mut ::core::ffi::c_void, #(#args),*) #ret {
                            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
                            let this = (*this).this as *const Impl;
                            (*this).#name(#(#params),*).into()
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let entries = self
            .methods
            .iter()
            .map(|m| {
                let name = &m.name;
                if parent_vtable.is_some() {
                    quote! {
                            #name: #name::<Identity, Impl, OFFSET>
                    }
                } else {
                    quote! {
                            #name: #name::<Impl>
                    }
                }
            })
            .collect::<Vec<_>>();

        if let Some(parent_vtable) = parent_vtable {
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                #vis struct #vtable_name {
                    pub base__: #parent_vtable,
                    #(#vtable_entries)*
                }
                impl #vtable_name {
                    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #trait_name, const OFFSET: isize>() -> Self {
                        #(#functions)*
                        Self { base__: #parent_vtable::new::<#parent_vtable_generics>(), #(#entries),* }
                    }

                    pub fn matches(iid: &windows::core::GUID) -> bool {
                        iid == &<#name as ::windows::core::Interface>::IID
                    }
                }
            }
        } else {
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                #vis struct #vtable_name {
                    #(#vtable_entries)*
                }
                impl #vtable_name {
                    pub const fn new<Impl: #trait_name>() -> Self {
                        #(#functions)*
                        Self { #(#entries),* }
                    }
                }
                struct #implvtbl_name<T: #trait_name> (::std::marker::PhantomData<T>);
                impl<T: #trait_name> #implvtbl_name<T> {
                    const VTABLE: #vtable_name = #vtable_name::new::<T>();
                }
                impl #name {
                    fn new<'a, T: #trait_name>(this: &'a T) -> ::windows::core::ScopedInterface<'a, #name> {
                        let this = ::windows::core::ScopedHeap { vtable: &#implvtbl_name::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                        let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                        unsafe { ::windows::core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
                    }
                }
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
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            impl ::core::convert::From<&#name> for ::windows::core::IUnknown {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
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

    fn parent_type(&self) -> proc_macro2::TokenStream {
        if let Some(parent) = &self.parent {
            quote!(#parent)
        } else {
            quote!(::std::ptr::NonNull<::std::ffi::c_void>)
        }
    }

    fn parent_vtable(&self) -> Option<proc_macro2::TokenStream> {
        if let Some(i) = self.parent_ident() {
            let i = quote::format_ident!("{}_Vtbl", i);
            Some(quote!(#i))
        } else {
            None
        }
    }

    fn parent_is_iunknown(&self) -> bool {
        if let Some(ident) = self.parent_ident() {
            ident == "IUnknown"
        } else {
            false
        }
    }

    fn parent_ident(&self) -> Option<&syn::Ident> {
        if let Some(parent) = &self.parent {
            Some(&parent.segments.last().as_ref().expect("segements should never be empty").ident)
        } else {
            None
        }
    }

    /// Gets the parent trait constrait which is nothing if the parent is IUnknown
    fn parent_trait_constraint(&self) -> proc_macro2::TokenStream {
        if let Some(i) = self.parent_ident() {
            if i == "IUnknown" {
                return quote!();
            }
            let i = quote::format_ident!("{}_Impl", i);
            quote!(#i)
        } else {
            quote!()
        }
    }
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let mut docs = Vec::new();
        for attr in attributes.into_iter() {
            let path = &attr.path;
            if path.is_ident("doc") {
                docs.push(attr);
            } else {
                return Err(syn::Error::new(path.span(), "Unrecognized attribute "));
            }
        }

        let visibility = input.parse::<syn::Visibility>()?;
        let _ = input.parse::<syn::Token![unsafe]>()?;
        let _ = input.parse::<syn::Token![trait]>()?;
        let name = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![:]>();
        let parent = input.parse::<syn::Path>().ok();
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
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
///                              //^ parses this   
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Guid(Option<syn::LitStr>);

impl Guid {
    fn to_tokens(&self) -> syn::Result<proc_macro2::TokenStream> {
        fn hex_lit(num: &str) -> syn::LitInt {
            syn::LitInt::new(&format!("0x{}", num), proc_macro2::Span::call_site())
        }

        fn ensure_length(part: Option<&str>, index: usize, length: usize, span: proc_macro2::Span) -> syn::Result<String> {
            let part = match part {
                Some(p) => p,
                None => return Err(syn::Error::new(span, format!("The IID missing part at index {}", index,))),
            };

            if part.len() != length {
                return Err(syn::Error::new(span, format!("The IID part at index {} must be {} characters long but was {} characters", index, length, part.len())));
            }

            Ok(part.to_owned())
        }

        if let Some(value) = &self.0 {
            let guid_value = value.value();
            let mut delimited = guid_value.split('-').fuse();
            let chunks = [ensure_length(delimited.next(), 0, 8, value.span())?, ensure_length(delimited.next(), 1, 4, value.span())?, ensure_length(delimited.next(), 2, 4, value.span())?, ensure_length(delimited.next(), 3, 4, value.span())?, ensure_length(delimited.next(), 4, 12, value.span())?];

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
        } else {
            Ok(quote! {
                ::windows::core::GUID::zeroed()
            })
        }
    }
}

impl Parse for Guid {
    fn parse(cursor: ParseStream) -> syn::Result<Self> {
        let string: Option<syn::LitStr> = cursor.parse().ok();

        Ok(Self(string))
    }
}

/// A parsed interface method
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
///   //^ parses this   
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
            .map(|p| Ok(InterfaceMethodArg { ty: p.ty, pat: p.pat }))
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
}
