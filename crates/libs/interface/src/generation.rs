//! Code generation for the `#[interface]` macro.
//!
//! The `gen_*` methods live here, separate from the parsing code in `lib.rs`.

use super::{Guid, Interface, InterfaceMethod, InterfaceMethodArg};
use quote::quote;

impl Interface {
    /// Generates all the code needed for a COM interface.
    pub(crate) fn gen_tokens(&self, guid: &Guid) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.visibility;
        let name = &self.name;
        let docs = &self.docs;
        let parent = self.parent_type();
        let vtable_name = quote::format_ident!("{}_Vtbl", name);
        let guid = guid.to_tokens()?;
        let implementation = self.gen_implementation();
        let com_trait = self.gen_com_trait();
        let vtable = self.gen_vtable(&vtable_name);
        let conversions = self.gen_conversions();

        Ok(quote! {
            #[repr(transparent)]
            #(#docs)*
            #vis struct #name(#parent);
            #implementation
            unsafe impl ::windows_core::Interface for #name {
                type Vtable = #vtable_name;
                const IID: ::windows_core::GUID = #guid;
            }
            impl ::windows_core::RuntimeName for #name {}
            impl ::core::ops::Deref for #name {
                type Target = #parent;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            #com_trait
            #vtable
            #conversions
        })
    }

    /// Generates the safe caller-side methods users call on an interface pointer.
    fn gen_implementation(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let vis = &m.visibility;
                let method_name = &m.name;

                let generics = m.gen_consume_generics();
                let params = m.gen_consume_params();
                let args = m.gen_consume_args();
                let ret = &m.ret;

                if m.is_result() {
                    quote! {
                        #[inline(always)]
                        #vis unsafe fn #method_name<#(#generics),*>(&self, #(#params),*) #ret {
                            (::windows_core::Interface::vtable(self).#method_name)(::windows_core::Interface::as_raw(self), #(#args),*).ok()
                        }
                    }
                } else {
                    quote! {
                        #[inline(always)]
                        #vis unsafe fn #method_name<#(#generics),*>(&self, #(#params),*) #ret {
                            (::windows_core::Interface::vtable(self).#method_name)(::windows_core::Interface::as_raw(self), #(#args),*)
                        }
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

    /// Generates the `IFoo_Impl` trait that implementors must satisfy.
    fn gen_com_trait(&self) -> proc_macro2::TokenStream {
        let name = quote::format_ident!("{}_Impl", self.name);
        let vis = &self.visibility;
        let methods = self
            .methods
            .iter()
            .map(|m| {
                let method_name = &m.name;
                let docs = &m.docs;
                let args = m.gen_args();
                let ret = &m.ret;
                quote! {
                    #(#docs)*
                    unsafe fn #method_name(&self, #(#args),*) #ret;
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

    /// Generates the vtable struct and constructor.
    ///
    /// Parented COM interfaces need `base__` and pointer adjustment; scoped interfaces get a
    /// hidden `IFoo_ImplVtbl<T>` plus an `IFoo::new` wrapper.
    fn gen_vtable(&self, vtable_name: &syn::Ident) -> proc_macro2::TokenStream {
        let vis = &self.visibility;
        let name = &self.name;
        let trait_name = quote::format_ident!("{}_Impl", name);
        let implvtbl_name = quote::format_ident!("{}_ImplVtbl", name);

        let vtable_entries = self
            .methods
            .iter()
            .map(|m| {
                let method_name = &m.name;
                let ret = &m.ret;
                let args = m.gen_args();

                if m.is_result() {
                    quote! {
                        pub #method_name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) -> ::windows_core::HRESULT,
                    }
                } else {
                    quote! {
                        pub #method_name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, #(#args),*) #ret,
                    }
                }
            })
            .collect::<Vec<_>>();

        let parent_vtable_generics = quote!(Identity, OFFSET);
        let parent_vtable = self.parent_vtable();

        // `or_parent_matches` is `|| <ParentVtable>::matches(iid)` for non-IUnknown parents.
        // This lets QueryInterface traverse the full interface inheritance chain.
        let or_parent_matches = match parent_vtable.as_ref() {
            Some(parent) if !self.parent_is_iunknown() => quote! (|| <#parent>::matches(iid)),
            _ => quote!(),
        };

        let functions = self
            .methods
            .iter()
            .map(|m| {
                let method_name = &m.name;
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

                let ret = if m.is_result() {
                    quote! { -> ::windows_core::HRESULT }
                } else {
                    quote! { #ret }
                };

                if parent_vtable.is_some() {
                    quote! {
                        unsafe extern "system" fn #method_name<
                            Identity: ::windows_core::IUnknownImpl,
                            const OFFSET: isize
                        >(
                            this: *mut ::core::ffi::c_void, // <-- This is the COM "this" pointer, which is not the same as &T or &T_Impl.
                            #(#args),*
                        ) #ret
                        where
                            Identity : #trait_name
                        {
                            // Adjust COM's interface-specific `this` pointer back to the root
                            // implementer; `OFFSET` is counted in pointer-sized slots.
                            let this_outer: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);

                            // Use the explicit trait to disambiguate inherited same-name methods.
                            <Identity as #trait_name>::#method_name(this_outer, #(#params),*).into()
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #method_name<Impl: #trait_name>(this: *mut ::core::ffi::c_void, #(#args),*) #ret {
                            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
                            let this = (*this).this as *const Impl;
                            (*this).#method_name(#(#params),*).into()
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        if let Some(parent_vtable) = parent_vtable {
            let entries = self
                .methods
                .iter()
                .map(|m| {
                    let method_name = &m.name;
                    quote!(#method_name: #method_name::<Identity, OFFSET>)
                })
                .collect::<Vec<_>>();

            quote! {
                #[repr(C)]
                #[doc(hidden)]
                #vis struct #vtable_name {
                    pub base__: #parent_vtable,
                    #(#vtable_entries)*
                }
                impl #vtable_name {
                    pub const fn new<
                        Identity: ::windows_core::IUnknownImpl,
                        const OFFSET: isize,
                    >() -> Self
                    where
                        Identity : #trait_name
                    {
                        #(#functions)*
                        Self { base__: #parent_vtable::new::<#parent_vtable_generics>(), #(#entries),* }
                    }

                    #[inline(always)]
                    pub fn matches(iid: &::windows_core::GUID) -> bool {
                        *iid == <#name as ::windows_core::Interface>::IID
                        #or_parent_matches
                    }
                }
            }
        } else {
            let entries = self
                .methods
                .iter()
                .map(|m| {
                    let method_name = &m.name;
                    quote!(#method_name: #method_name::<Impl>)
                })
                .collect::<Vec<_>>();

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
                struct #implvtbl_name<T: #trait_name> (::core::marker::PhantomData<T>);
                impl<T: #trait_name> #implvtbl_name<T> {
                    const VTABLE: #vtable_name = #vtable_name::new::<T>();
                }
                impl #name {
                    fn new<'a, T: #trait_name>(this: &'a T) -> ::windows_core::ScopedInterface<'a, #name> {
                        let this = ::windows_core::ScopedHeap { vtable: &#implvtbl_name::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                        let this = ::core::mem::ManuallyDrop::new(::windows_core::imp::box_new(this));
                        unsafe { ::windows_core::ScopedInterface::new(::core::mem::transmute(&this.vtable)) }
                    }
                }
            }
        }
    }

    /// Generates `Clone`, `PartialEq`, `Eq`, `Debug`, and `From` conversions.
    fn gen_conversions(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let name_string = format!("{name}");

        // Parented interfaces delegate through the inheritance chain; scoped interfaces have a
        // raw inner pointer, so they fall back to transmute.
        let into_iunknown_impl = if self.parent.is_some() {
            quote! {
                impl ::core::convert::From<#name> for ::windows_core::IUnknown {
                    fn from(value: #name) -> Self {
                        ::windows_core::IUnknown::from(value.0)
                    }
                }
            }
        } else {
            quote! {
                impl ::core::convert::From<#name> for ::windows_core::IUnknown {
                    fn from(value: #name) -> Self {
                        unsafe { ::core::mem::transmute(value) }
                    }
                }
            }
        };

        quote! {
            #into_iunknown_impl
            impl ::core::convert::From<&#name> for ::windows_core::IUnknown {
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
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    f.debug_tuple(#name_string).field(&::windows_core::Interface::as_raw(self)).finish()
                }
            }
        }
    }

    /// The type token used as the inner field of the `#[repr(transparent)]` struct.
    pub(crate) fn parent_type(&self) -> proc_macro2::TokenStream {
        if let Some(parent) = &self.parent {
            quote!(#parent)
        } else {
            quote!(::core::ptr::NonNull<::core::ffi::c_void>)
        }
    }

    fn parent_vtable(&self) -> Option<proc_macro2::TokenStream> {
        self.parent
            .as_ref()
            .map(|parent| quote! { <#parent as ::windows_core::Interface>::Vtable })
    }

    fn parent_is_iunknown(&self) -> bool {
        if let Some(ident) = self.parent_path().last() {
            ident == "IUnknown"
        } else {
            false
        }
    }

    fn parent_path(&self) -> Vec<syn::Ident> {
        if let Some(parent) = &self.parent {
            parent
                .segments
                .iter()
                .map(|segment| segment.ident.clone())
                .collect()
        } else {
            vec![]
        }
    }

    /// Returns the `ParentName_Impl` supertrait constraint, except for `IUnknown`.
    fn parent_trait_constraint(&self) -> proc_macro2::TokenStream {
        if let Some((ident, path)) = self.parent_path().split_last()
            && ident != "IUnknown"
        {
            let ident = quote::format_ident!("{}_Impl", ident);
            return quote! { #(#path::)* #ident };
        }

        quote! {}
    }
}

impl InterfaceMethod {
    /// Returns `true` when the method's return type is `Result<T>`.
    pub(crate) fn is_result(&self) -> bool {
        if let syn::ReturnType::Type(_, ty) = &self.ret
            && let syn::Type::Path(path) = &**ty
            && let Some(segment) = path.path.segments.last()
        {
            let ident = segment.ident.to_string();
            if ident == "Result"
                && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                && args.args.len() == 1
            {
                return true;
            }
        }

        false
    }

    /// Generates `name: Type` argument pairs for the raw vtable function signature.
    pub(crate) fn gen_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .map(|a| {
                let pat = &a.pat;
                let ty = &a.ty;
                quote! { #pat: #ty }
            })
            .collect::<Vec<_>>()
    }

    /// Generates generic parameter declarations for `Ref<T>` / `OutRef<T>` caller-side wrappers.
    pub(crate) fn gen_consume_generics(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .enumerate()
            .filter_map(|(generic_index, a)| {
                if let Some((ty, ident)) = a.borrow_type() {
                    let generic_ident = quote::format_ident!("P{generic_index}");
                    if ident == "Ref" {
                        Some(quote! { #generic_ident: ::windows_core::Param<#ty> })
                    } else {
                        Some(quote! { #generic_ident: ::windows_core::OutParam<#ty> })
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    /// Generates parameter declarations for the caller-side method, replacing `Ref<T>` /
    /// `OutRef<T>` arguments with their corresponding generic parameter types.
    pub(crate) fn gen_consume_params(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .enumerate()
            .map(|(generic_index, a)| {
                let pat = &a.pat;

                if a.borrow_type().is_some() {
                    let generic_ident = quote::format_ident!("P{generic_index}");
                    quote! { #pat: #generic_ident }
                } else {
                    let ty = &a.ty;
                    quote! { #pat: #ty }
                }
            })
            .collect::<Vec<_>>()
    }

    /// Generates the argument expressions forwarded to the raw vtable function from the
    /// caller-side method.
    pub(crate) fn gen_consume_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.args
            .iter()
            .map(|a| {
                let pat = &a.pat;

                if let Some((_, ident)) = a.borrow_type() {
                    if ident == "Ref" {
                        quote! { #pat.param().borrow() }
                    } else {
                        quote! { #pat.borrow_mut() }
                    }
                } else {
                    quote! { #pat }
                }
            })
            .collect::<Vec<_>>()
    }
}

impl InterfaceMethodArg {
    /// If this argument is `Ref<T>` or `OutRef<T>`, returns `(T, "Ref")` or `(T, "OutRef")`.
    pub(crate) fn borrow_type(&self) -> Option<(syn::Type, String)> {
        if let syn::Type::Path(path) = &*self.ty
            && let Some(segment) = path.path.segments.last()
        {
            let ident = segment.ident.to_string();
            if matches!(ident.as_str(), "Ref" | "OutRef")
                && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                && args.args.len() == 1
                && let Some(syn::GenericArgument::Type(ty)) = args.args.first()
            {
                return Some((ty.clone(), ident));
            }
        }

        None
    }
}
