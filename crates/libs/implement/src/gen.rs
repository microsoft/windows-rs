//! Generates output for the `implement` proc macro.
//!
//! Each function in this module focuses on generating one thing, or one kind of thing.
//! Each takes `ImplementInputs` as its input. `gen_all` calls all of the `gen_*` functions
//! and merges them into the final list of output items.
//!
//! `parse_quote` is used so that errors point at the specific generator rather than the
//! entire macro output.

use super::*;
use quote::{quote, quote_spanned};
use syn::{parse_quote, parse_quote_spanned};

/// Generates code for the `#[implements]` macro.
pub(crate) fn gen_all(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items: Vec<syn::Item> = Vec::with_capacity(64);

    items.push(gen_original_impl(inputs));
    items.push(gen_impl_struct(inputs));
    items.push(gen_impl_deref(inputs));
    items.push(gen_impl_impl(inputs));
    items.push(gen_iunknown_impl(inputs));
    items.push(gen_impl_com_object_inner(inputs));

    if inputs.compose {
        items.push(gen_impl_compose(inputs));
    }

    items.extend(gen_impl_from(inputs));
    items.extend(gen_impl_com_object_interfaces(inputs));

    for (i, interface_chain) in inputs.interface_chains.iter().enumerate() {
        items.push(gen_impl_as_impl(inputs, interface_chain, i));
    }

    items
}

/// The interface at index 0: the first interface chain, or `IInspectable` if there are none.
fn identity_type(inputs: &ImplementInputs) -> proc_macro2::TokenStream {
    if let Some(first) = inputs.interface_chains.first() {
        first.implement.to_ident()
    } else {
        quote! { ::windows_core::IInspectable }
    }
}

/// The `ComCell` type wrapped by `Foo_Impl`.
fn cell_type(inputs: &ImplementInputs) -> proc_macro2::TokenStream {
    let original_ident = &inputs.original_ident;
    let generics_idents = &inputs.generics_idents;
    let count = proc_macro2::Literal::usize_unsuffixed(inputs.interface_chains.len().max(1));

    let base = if inputs.compose {
        quote! { ::windows_core::ComposeBase }
    } else {
        quote! { () }
    };

    quote! { ::windows_core::imp::ComCell<#count, #base, #original_ident::#generics_idents> }
}

/// Generates an `impl` block for the original `Foo` type.
///
/// This `impl` block will contain `into_outer` and `into_static` (if applicable).
fn gen_original_impl(inputs: &ImplementInputs) -> syn::Item {
    let original_ident = &inputs.original_ident;
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;

    let mut output: syn::ItemImpl = parse_quote! {
        impl #generics #original_ident::#generics_idents where #constraints {}
    };

    output.items.push(gen_into_outer(inputs));

    // Static COM objects can't be generic: an open generic type has no known representation.
    if !inputs.is_generic {
        output.items.push(gen_into_static(inputs));
    }

    syn::Item::Impl(output)
}

/// Generates the structure definition for the `Foo_Impl` type.
fn gen_impl_struct(inputs: &ImplementInputs) -> syn::Item {
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let constraints = &inputs.constraints;
    let vis = &inputs.original_type.vis;
    let cell = cell_type(inputs);

    parse_quote! {
        #[repr(transparent)]
        #[allow(non_camel_case_types)]
        #vis struct #impl_ident #generics where #constraints { __com: #cell }
    }
}

/// Generates the implementation of `core::ops::Deref` for the generated `Foo_Impl` type.
fn gen_impl_deref(inputs: &ImplementInputs) -> syn::Item {
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let original_ident = &inputs.original_type.ident;
    let impl_ident = &inputs.impl_ident;

    parse_quote! {
        impl #generics ::core::ops::Deref for #impl_ident::#generics_idents where #constraints {
            type Target = #original_ident::#generics_idents;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.__com.get()
            }
        }
    }
}

/// Generates the vtable constants for `Foo_Impl`.
///
/// Associated constants support generic implementers where local constants cannot refer to
/// outer generic parameters.
fn gen_impl_impl(inputs: &ImplementInputs) -> syn::Item {
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;

    let mut output: syn::ItemImpl = parse_quote! {
        impl #generics #impl_ident::#generics_idents where #constraints {}
    };

    if inputs.interface_chains.is_empty() {
        output.items.push(parse_quote! {
            const VTABLE_IDENTITY: ::windows_core::IInspectable_Vtbl =
                ::windows_core::IInspectable_Vtbl::new::<
                    #impl_ident::#generics_idents,
                    0,
                >();
        });
    }

    for (interface_index, interface_chain) in inputs.interface_chains.iter().enumerate() {
        let vtbl_ty = interface_chain.implement.to_vtbl_ident();
        let vtable_const_ident = &interface_chain.vtable_const_ident;

        // Interface `i` is the `i`th pointer of the object.
        let offset: isize = -(interface_index as isize);
        output.items.push(parse_quote! {
            const #vtable_const_ident: #vtbl_ty = #vtbl_ty::new::<
                #impl_ident::#generics_idents,
                #offset,
            >();
        });
    }

    syn::Item::Impl(output)
}

/// Generates the `IUnknownImpl` implementation for the `Foo_Impl` type.
fn gen_iunknown_impl(inputs: &ImplementInputs) -> syn::Item {
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;
    let original_ident = &inputs.original_type.ident;
    let identity_type = identity_type(inputs);

    let trust_level = proc_macro2::Literal::usize_unsuffixed(inputs.trust_level);

    let mut output: syn::ItemImpl = parse_quote! {
        impl #generics ::windows_core::IUnknownImpl for #impl_ident::#generics_idents where #constraints {
            type Impl = #original_ident::#generics_idents;
            type Identity = #identity_type;

            #[inline(always)]
            fn get_impl(&self) -> &Self::Impl {
                self.__com.get()
            }

            #[inline(always)]
            fn get_impl_mut(&mut self) -> &mut Self::Impl {
                self.__com.get_mut()
            }

            #[inline(always)]
            fn into_inner(self) -> Self::Impl {
                self.__com.into_inner()
            }

            #[inline(always)]
            fn AddRef(&self) -> u32 {
                self.__com.count().add_ref()
            }

            #[inline(always)]
            unsafe fn Release(self_: *mut Self) -> u32 {
                let remaining = (*self_).__com.count().release();
                if remaining == 0 {
                    _ = ::windows_core::imp::object_from_raw(self_);
                }
                remaining
            }

            #[inline(always)]
            fn is_reference_count_one(&self) -> bool {
                self.__com.count().is_one()
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> ::windows_core::HRESULT {
                if value.is_null() {
                    return ::windows_core::imp::E_POINTER;
                }
                *value = #trust_level;
                ::windows_core::HRESULT(0)
            }

            fn to_object(&self) -> ::windows_core::ComObject<Self::Impl> {
                self.__com.count().add_ref();
                unsafe {
                    ::windows_core::ComObject::from_raw(::core::ptr::NonNull::from(self))
                }
            }
        }
    };

    let query_interface_fn = gen_query_interface(inputs);
    output.items.push(syn::ImplItem::Fn(query_interface_fn));

    syn::Item::Impl(output)
}

/// Generates the implementation of `ComObjectInner`.
fn gen_impl_com_object_inner(inputs: &ImplementInputs) -> syn::Item {
    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    parse_quote! {
        impl #generics ::windows_core::ComObjectInner for #original_ident::#generics_idents where #constraints {
            type Outer = #impl_ident::#generics_idents;

            // Safe code must get only a `ComObject`, never an owned `Foo_Impl` with direct
            // reference-count controls.
            fn into_object(self) -> ::windows_core::ComObject<Self> {
                let ptr = ::windows_core::imp::object_into_raw(unsafe { self.into_outer() });
                unsafe { ::windows_core::ComObject::from_raw(ptr) }
            }
        }
    }
}

/// Generates `Compose`, letting `Foo` back a composable WinRT runtime class.
///
/// The factory writes the inner non-delegating `IInspectable` into the returned `base` slot.
fn gen_impl_compose(inputs: &ImplementInputs) -> syn::Item {
    let original_ident = &inputs.original_type.ident;
    let impl_ident = &inputs.impl_ident;
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let identity_type = identity_type(inputs);
    let mut lifetime = syn::Lifetime::new("'a", proc_macro2::Span::call_site());
    while inputs
        .original_type
        .generics
        .lifetimes()
        .any(|param| param.lifetime.ident == lifetime.ident)
    {
        lifetime.ident = quote::format_ident!("{}_", lifetime.ident);
    }

    parse_quote! {
        impl #generics ::windows_core::Compose for #original_ident::#generics_idents where #constraints {
            unsafe fn compose<#lifetime>(
                implementation: Self,
            ) -> (::windows_core::IInspectable, &#lifetime mut ::core::option::Option<::windows_core::IInspectable>) {
                unsafe {
                    let identity: ::windows_core::IUnknown = ::windows_core::ComObject::new(implementation).into_interface();
                    let this = ::windows_core::Interface::as_raw(&identity) as *mut #impl_ident::#generics_idents;
                    ::windows_core::imp::ComCell::compose::<#identity_type>(&raw mut (*this).__com, identity)
                }
            }
        }
    }
}

/// Generates the `query_interface` method.
fn gen_query_interface(inputs: &ImplementInputs) -> syn::ImplItemFn {
    let queries = inputs
        .interface_chains
        .iter()
        .enumerate()
        .map(|(index, interface_chain)| {
            let chain_ty = interface_chain.implement.to_vtbl_ident();
            let interface_ty = interface_chain.implement.to_ident();
            quote_spanned! {
                interface_chain.implement.span =>
                if #chain_ty::matches(iid)
                    || (<#interface_ty as ::windows_core::Interface>::INSPECTABLE
                        && *iid == <::windows_core::IInspectable as ::windows_core::Interface>::IID)
                {
                    return ::core::option::Option::Some(#index);
                }
            }
        });

    // Dynamic casting requires that the object not contain non-static lifetimes.
    let any = if inputs.original_type.generics.lifetimes().count() == 0 {
        quote! { ::core::option::Option::Some(self as &dyn ::core::any::Any) }
    } else {
        quote! { ::core::option::Option::None }
    };

    let agile = inputs.agile;

    let find = if inputs.interface_chains.is_empty() {
        quote! { |_| ::core::option::Option::None }
    } else {
        quote! {
            |iid| {
                #(#queries)*
                ::core::option::Option::None
            }
        }
    };

    parse_quote! {
        unsafe fn QueryInterface(
            &self,
            iid: *const ::windows_core::GUID,
            interface: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            unsafe {
                self.__com.query_interface::<<Self as ::windows_core::IUnknownImpl>::Identity>(
                    iid,
                    interface,
                    #agile,
                    #any,
                    #find,
                )
            }
        }
    }
}

/// Generates the `T::into_outer` function. This function is part of how we construct a
/// `ComObject<T>` from a `T`.
fn gen_into_outer(inputs: &ImplementInputs) -> syn::ImplItem {
    let generics_idents = &inputs.generics_idents;
    let impl_ident = &inputs.impl_ident;

    let vtables: Vec<_> = if inputs.interface_chains.is_empty() {
        vec![quote! {
            ::windows_core::imp::Vtable::new(&#impl_ident::#generics_idents::VTABLE_IDENTITY)
        }]
    } else {
        inputs
            .interface_chains
            .iter()
            .map(|interface_chain| {
                let vtable_const_ident = &interface_chain.vtable_const_ident;
                quote_spanned! {
                    interface_chain.implement.span =>
                    ::windows_core::imp::Vtable::new(&#impl_ident::#generics_idents::#vtable_const_ident)
                }
            })
            .collect()
    };

    let base = if inputs.compose {
        quote! { ::windows_core::ComposeBase::new() }
    } else {
        quote! { () }
    };

    // If the type is generic then into_outer() cannot be a const fn.
    let maybe_const = if inputs.is_generic {
        quote!()
    } else {
        quote!(const)
    };

    parse_quote! {
        // Builds the outer object. Internal only: app code must never own a `Foo_Impl`,
        // since mutable access to it would shear the refcount. Callers `into_static` and
        // `into_object` uphold this by only handing out `StaticComObject` / `ComObject`.
        #[inline(always)]
        #maybe_const unsafe fn into_outer(self) -> #impl_ident::#generics_idents {
            #impl_ident { __com: unsafe { ::windows_core::imp::ComCell::new([#(#vtables),*], #base, self) } }
        }
    }
}

/// Generates the `T::into_static` function. This function is part of how we construct a
/// `StaticComObject<T>` from a `T`.
fn gen_into_static(inputs: &ImplementInputs) -> syn::ImplItem {
    assert!(!inputs.is_generic);
    parse_quote! {
        /// This converts a partially-constructed COM object (containing application state but
        /// without vtable and reference count yet set up) into a `StaticComObject`. This allows
        /// the COM object to be stored in static (global) variables.
        pub const fn into_static(self) -> ::windows_core::StaticComObject<Self> {
            ::windows_core::StaticComObject::from_outer(unsafe { self.into_outer() })
        }
    }
}

/// Generates `From<Foo>` conversions to each declared interface (shorthand for
/// `ComObject::new(value).into_interface()`). `IUnknown` and `IInspectable` conversions are
/// provided by `windows-core`.
///
/// Only the declared interface chain roots get a conversion - the macro can't see an
/// interface's parents. For a base interface, go through the child:
/// `IFoo3::from(value).into()`.
fn gen_impl_from(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let original_ident = &inputs.original_type.ident;
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;

    inputs
        .interface_chains
        .iter()
        .map(|interface_chain| {
            let interface_ident = interface_chain.implement.to_ident();

            parse_quote_spanned! {
                interface_chain.implement.span =>
                impl #generics ::core::convert::From<#original_ident::#generics_idents> for #interface_ident where #constraints {
                    #[inline(always)]
                    fn from(this: #original_ident::#generics_idents) -> Self {
                        let com_object = ::windows_core::ComObject::new(this);
                        com_object.into_interface()
                    }
                }
            }
        })
        .collect()
}

/// Generates the `ComObjectInterface` implementation for `IUnknown` and each interface chain.
///
/// Each of these `impl` blocks says "this COM object implements this COM interface".
/// It allows the `ComObject` type to do conversions from the `ComObject` to `IFoo` instances,
/// _without_ doing a `QueryInterface` call. `IInspectable` is provided by `windows-core`.
fn gen_impl_com_object_interfaces(inputs: &ImplementInputs) -> Vec<syn::Item> {
    let mut items = Vec::new();

    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let impl_ident = &inputs.impl_ident;

    items.push(parse_quote! {
        impl #generics ::windows_core::ComObjectInterface<::windows_core::IUnknown> for #impl_ident::#generics_idents where #constraints {
            #[inline(always)]
            fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, ::windows_core::IUnknown> {
                unsafe { self.__com.interface_ref(0) }
            }
        }
    });

    for (index, interface_chain) in inputs.interface_chains.iter().enumerate() {
        let interface_ident = interface_chain.implement.to_ident();

        items.push(parse_quote_spanned! {
            interface_chain.implement.span =>
            #[allow(clippy::needless_lifetimes)]
            impl #generics ::windows_core::ComObjectInterface<#interface_ident> for #impl_ident::#generics_idents where #constraints {
                #[inline(always)]
                fn as_interface_ref(&self) -> ::windows_core::InterfaceRef<'_, #interface_ident> {
                    unsafe { self.__com.interface_ref(#index) }
                }
            }
        });
    }

    items
}

/// Generates the implementation of the `AsImpl` trait for a given interface chain.
fn gen_impl_as_impl(
    inputs: &ImplementInputs,
    interface_chain: &InterfaceChain,
    interface_chain_index: usize,
) -> syn::Item {
    let generics = &inputs.generics;
    let generics_idents = &inputs.generics_idents;
    let constraints = &inputs.constraints;
    let interface_ident = interface_chain.implement.to_ident();
    let original_ident = &inputs.original_type.ident;
    let impl_ident = &inputs.impl_ident;

    parse_quote_spanned! {
        interface_chain.implement.span =>
        impl #generics ::windows_core::AsImpl<#original_ident::#generics_idents> for #interface_ident where #constraints {
            // SAFETY: interface `i` is the `i`th pointer of the object, and the object lives at
            // least as long as `self`.
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<#original_ident::#generics_idents> {
                unsafe {
                    let this = ::windows_core::Interface::as_raw(self);
                    let this = (this as *mut *mut ::core::ffi::c_void).sub(#interface_chain_index) as *mut #impl_ident::#generics_idents;
                    ::core::ptr::NonNull::new_unchecked(::windows_core::imp::ComCell::this_ptr(&raw mut (*this).__com))
                }
            }
        }
    }
}
