use std::collections::*;
use syn::parse::*;
use syn::Ident;
use syn::*;
use tokens::*;

#[proc_macro_attribute]
pub fn implement(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as ImplementAttributes);
    let generics = attributes.generics();

    let constraints = quote! {
        #(#generics: ::windows::core::RuntimeType + 'static,)*
    };

    let identity_type = if let Some(first) = attributes.implement.get(0) {
        first.to_ident()
    } else {
        quote! { ::windows::core::IInspectable }
    };

    let original_type2 = original_type.clone();
    let original_ident = TokenStream(syn::parse_macro_input!(original_type2 as syn::ItemStruct).ident.to_string());
    let impl_ident = original_ident.join("_Impl");
    let vtbl_idents = attributes.implement.iter().map(|implement| implement.to_vtbl_ident());
    let vtbl_idents2 = vtbl_idents.clone();

    let vtable_news = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let offset: TokenStream = format!("{}", -2 - enumerate as isize).into();
        quote! { #vtbl_ident::new::<Self, #original_ident::<#(#generics,)*>, #offset>() }
    });

    let offset = attributes.implement.iter().enumerate().map(|(offset, _)| {
        let offset: TokenStream = format!("{}", offset).into();
        offset
    });

    let queries = attributes.implement.iter().enumerate().map(|(count, implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let offset: TokenStream = format!("{}", count).into();
        quote! {
            else if #vtbl_ident::matches(iid) {
                &self.vtables.#offset as *const _ as *const _
            }
        }
    });

    let conversions = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let interface_ident = implement.to_ident();
        let offset: TokenStream = enumerate.to_string().into();
        quote! {
            impl <#constraints> ::core::convert::From<#original_ident::<#(#generics,)*>> for #interface_ident {
                fn from(this: #original_ident::<#(#generics,)*>) -> Self {
                    let this = #impl_ident::<#(#generics,)*>::new(this);
                    let mut this = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                    let vtable_ptr = &this.vtables.#offset;
                    // SAFETY: interfaces are in-memory equivalent to pointers to their vtables.
                    unsafe { ::core::mem::transmute(vtable_ptr) }
                }
            }
            impl <#constraints> ::windows::core::AsImpl<#original_ident::<#(#generics,)*>> for #interface_ident {
                fn as_impl(&self) -> &#original_ident::<#(#generics,)*> {
                    let this = ::windows::core::Interface::as_raw(self);
                    // SAFETY: the offset is guranteed to be in bounds, and the implementation struct
                    // is guaranteed to live at least as long as `self`.
                    unsafe {
                        // Subtract away the vtable offset plus 2 (for the `base` and `identity` fields) to get
                        // to the impl struct which contains that original implementation type.
                        let this = (this as *mut ::windows::core::RawPtr).sub(2 + #offset) as *mut #impl_ident::<#(#generics,)*>;
                        &(*this).this
                    }
                }
            }
        }
    });

    let tokens = quote! {
        #[repr(C)]
        struct #impl_ident<#(#generics,)*> where #constraints {
            base: ::core::option::Option<::windows::core::IInspectable>,
            identity: *const ::windows::core::IInspectableVtbl,
            vtables: (#(*const #vtbl_idents,)*),
            this: #original_ident::<#(#generics,)*>,
            count: ::windows::core::WeakRefCount,
        }
        impl <#constraints> #impl_ident::<#(#generics,)*> {
            const VTABLES: (#(#vtbl_idents2,)*) = (#(#vtable_news,)*);
            const IDENTITY: ::windows::core::IInspectableVtbl = ::windows::core::IInspectableVtbl::new::<Self, #identity_type, -1>();
            fn new(this: #original_ident::<#(#generics,)*>) -> Self {
                Self {
                    base: ::core::option::Option::None,
                    identity: &Self::IDENTITY,
                    vtables:(#(&Self::VTABLES.#offset,)*),
                    this,
                    count: ::windows::core::WeakRefCount::new(),
                }
            }
        }
        impl <#constraints> ::windows::core::IUnknownImpl for #impl_ident::<#(#generics,)*> {
            type Impl = #original_ident::<#(#generics,)*>;
            fn get_impl(&self) -> &Self::Impl {
                &self.this
            }
            unsafe fn QueryInterface(&self, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
                unsafe {
                    *interface = if iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IInspectable as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
                            &self.identity as *const _ as *const _
                    } #(#queries)* else {
                        ::core::ptr::null_mut()
                    };

                    if !(*interface).is_null() {
                        self.count.add_ref();
                        return ::windows::core::HRESULT(0);
                    }

                    *interface = self.count.query(iid, &self.identity as *const _ as *mut _);

                    if (*interface).is_null() {
                        if let Some(base) = &self.base {
                            ::windows::core::Interface::query(base, iid, interface)
                        } else {
                            ::windows::core::HRESULT(0x8000_4002) // E_NOINTERFACE
                        }
                    } else {
                        ::windows::core::HRESULT(0)
                    }
                }
            }
            fn AddRef(&self) -> u32 {
                self.count.add_ref()
            }
            unsafe fn Release(&self) -> u32 {
                let remaining = self.count.release();
                if remaining == 0 {
                    unsafe {
                        ::std::boxed::Box::from_raw(self as *const Self as *mut Self);
                    }
                }
                remaining
            }
        }
        impl <#constraints> #original_ident::<#(#generics,)*> {
            fn alloc<I: ::windows::core::Interface>(self) -> ::windows::core::Result<I> {
                let this = #impl_ident::<#(#generics,)*>::new(self);
                let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                let mut result = None;
                let result = unsafe {
                    <#impl_ident::<#(#generics,)*> as ::windows::core::IUnknownImpl>::QueryInterface(&*boxed, &I::IID, &mut result as *mut _ as _).and_some(result)
                };
                // If querying for the supplied interface fails, we must free the underlying implementation, otherwise that memory will be leaked
                if result.is_err() {
                    let _ = ::core::mem::ManuallyDrop::into_inner(boxed);
                }
                result
            }
        }
        impl <#constraints> ::windows::core::Compose for #original_ident::<#(#generics,)*> {
            unsafe fn compose<'a>(implementation: Self) -> (::windows::core::IInspectable, &'a mut ::core::option::Option<::windows::core::IInspectable>) {
                let inspectable: ::windows::core::IInspectable = implementation.into();
                let this: ::windows::core::RawPtr = ::windows::core::Interface::as_raw(&inspectable);
                let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut #impl_ident::<#(#generics,)*>;
                (inspectable, &mut (*this).base)
            }
        }
        impl <#constraints> ::core::convert::From<#original_ident::<#(#generics,)*>> for ::windows::core::IUnknown {
            fn from(this: #original_ident::<#(#generics,)*>) -> Self {
                let this = #impl_ident::<#(#generics,)*>::new(this);
                let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                unsafe {
                    ::core::mem::transmute(&boxed.identity)
                }
            }
        }
        impl <#constraints> ::core::convert::From<#original_ident::<#(#generics,)*>> for ::windows::core::IInspectable {
            fn from(this: #original_ident::<#(#generics,)*>) -> Self {
                let this = #impl_ident::<#(#generics,)*>::new(this);
                let boxed = ::core::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
                unsafe {
                    ::core::mem::transmute(&boxed.identity)
                }
            }
        }
        #(#conversions)*
    };

    let mut tokens = tokens.parse::<proc_macro::TokenStream>().unwrap();
    tokens.extend(core::iter::once(original_type));
    tokens
}

#[derive(Default)]
struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,
}

impl ImplementType {
    fn to_ident(&self) -> TokenStream {
        let mut tokens: TokenStream = self.type_name.clone().into();
        tokens.push_str("<");

        for g in &self.generics {
            tokens.combine(&g.to_ident());
            tokens.push_str(",");
        }

        tokens.push_str(">");
        tokens
    }
    fn to_vtbl_ident(&self) -> TokenStream {
        let ident = self.to_ident();
        quote! {
            <#ident as ::windows::core::Interface>::Vtable
        }
    }
    fn generics(&self) -> BTreeSet<TokenStream> {
        let mut set = BTreeSet::new();

        if self.type_name.len() == 1 && self.type_name == self.type_name.to_uppercase() {
            set.insert(self.type_name.clone().into());
        }

        for def in &self.generics {
            set.append(&mut def.generics());
        }

        set
    }
}

#[derive(Default)]
struct ImplementAttributes {
    pub implement: Vec<ImplementType>,
}

impl Parse for ImplementAttributes {
    fn parse(cursor: ParseStream) -> Result<Self> {
        let mut input = Self::default();

        while !cursor.is_empty() {
            input.parse_implement(cursor)?;
        }

        Ok(input)
    }
}

impl ImplementAttributes {
    fn generics(&self) -> BTreeSet<TokenStream> {
        let mut set = BTreeSet::new();

        for def in &self.implement {
            set.append(&mut def.generics());
        }

        set
    }

    fn parse_implement(&mut self, cursor: ParseStream) -> Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(&tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(&mut self, tree: &UseTree2, namespace: &mut String) -> Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(&*input.tree, namespace)?;
            }
            UseTree2::Name(_) => {
                self.implement.push(tree.to_element_type(namespace)?);
            }
            UseTree2::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(tree, namespace)?;
                }
            }
        }

        Ok(())
    }
}

enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
}

impl UseTree2 {
    fn to_element_type(&self, namespace: &mut String) -> Result<ImplementType> {
        match self {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(namespace)
            }
            UseTree2::Name(input) => {
                let mut type_name = input.ident.to_string();

                if !namespace.is_empty() {
                    type_name = format!("{}::{}", namespace, type_name);
                }

                let mut generics = vec![];

                for g in &input.generics {
                    generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(ImplementType { type_name, generics })
            }
            UseTree2::Group(input) => Err(Error::new(input.brace_token.span, "Syntax not supported")),
        }
    }
}

struct UsePath2 {
    pub ident: Ident,
    pub tree: Box<UseTree2>,
}

struct UseName2 {
    pub ident: Ident,
    pub generics: Vec<UseTree2>,
}

struct UseGroup2 {
    pub brace_token: token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, Token![,]>,
}

impl Parse for UseTree2 {
    fn parse(input: ParseStream) -> Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(Ident::parse_any)?;
            if input.peek(Token![::]) {
                input.parse::<Token![::]>()?;
                Ok(UseTree2::Path(UsePath2 { ident, tree: Box::new(input.parse()?) }))
            } else {
                let generics = if input.peek(Token![<]) {
                    input.parse::<Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse)?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}
