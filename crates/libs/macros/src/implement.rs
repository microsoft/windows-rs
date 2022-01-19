use syn::parse::*;
use syn::Ident;
use syn::*;
use quote::*;

// New traits-based implement macro that doesn't rely on metadata
// Also no support for overrides (Xaml) but developers can still implement overrides directly by implementning the necessary override interface
// Maybe gen up override traits to have a default implementation to make this simpler { Ok() }
// And don't make the overrides required constraints.
pub fn gen(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as ImplementAttributes);

    let original_type2 = original_type.clone();
    let original_ident = TokenStream(syn::parse_macro_input!(original_type2 as syn::ItemStruct).ident.to_string());
    let impl_ident = original_ident.join("_Impl");

    let vtbl_idents = attributes.implement.iter().map(|implement| {
        implement.to_vtbl_ident()
    });

    let vtbl_idents2 = vtbl_idents.clone();

    let vtable_news = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let base_offset: TokenStream = format!("{}", -2 - enumerate as isize).into();
        let impl_offset: TokenStream = format!("{}", 1 + attributes.implement.len() - enumerate).into();
        quote! { #vtbl_ident::new::<Self, #original_ident, #base_offset, #impl_offset>() }
    });

    let vtbl_count = attributes.implement.iter().enumerate().map(|(count,_)| {
        let offset: TokenStream = format!("{}", count).into();
        offset
    });

    let queries = attributes.implement.iter().enumerate().map(|(count,implement)| {
        let vtbl_ident = implement.to_vtbl_ident();
        let offset: TokenStream = format!("{}", count).into();
        quote! {
            else if #vtbl_ident::matches(iid) {
                &mut self.vtables.#offset as *mut _ as _
            }
        }
    });

    let froms = attributes.implement.iter().enumerate().map(|(enumerate, implement)| {
        let interface_ident = implement.to_ident();
        let offset: TokenStream = format!("{}", enumerate).into();
        quote! {
            impl From<#original_ident> for #interface_ident {
                fn from(this: #original_ident) -> Self {
                    unsafe {
                        let this = #impl_ident::new(this);
                        let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
                        ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                            &mut (*ptr).vtables.#offset as *mut _ as _,
                        ))
                    }
                }
            }
        }
    });

    let tokens = quote! {
        struct #impl_ident {
            base: ::core::option::Option<::windows::core::IInspectable>,
            identity: *const ::windows::core::IInspectableVtbl,
            vtables: (#(*const #vtbl_idents,)*),
            _this: #original_ident,
            count: ::windows::core::WeakRefCount,
        }
        impl #impl_ident {
            const VTABLES: (#(#vtbl_idents2,)*) = (#(#vtable_news,)*);
            const IDENTITY: ::windows::core::IInspectableVtbl = ::windows::core::IInspectableVtbl::new::<Self, ::windows::core::IInspectable, 0>();
            fn new(this: #original_ident) -> Self {
                Self {
                    base: ::core::option::Option::None,
                    identity: &Self::IDENTITY,
                    vtables:(#(&Self::VTABLES.#vtbl_count,)*),
                    _this: this,
                    count: ::windows::core::WeakRefCount::new(),
                }
            }
        }
        impl ::windows::core::IUnknownImpl for #impl_ident {
            fn QueryInterface(&mut self, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
                unsafe {
                    *interface = if iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IInspectable as ::windows::core::Interface>::IID
                        || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
                            &mut self.identity as *mut _ as _
                    } #(#queries)* else {
                        ::core::ptr::null_mut()
                    };

                    if !(*interface).is_null() {
                        self.count.add_ref();
                        return ::windows::core::HRESULT(0);
                    }

                    *interface = self.count.query(iid, &mut self.identity as *mut _ as _);

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
            fn AddRef(&mut self) -> u32 {
                self.count.add_ref()
            }
            fn Release(&mut self) -> u32 {
                let remaining = self.count.release();
                if remaining == 0 {
                    unsafe {
                        ::std::boxed::Box::from_raw(self);
                    }
                }
                remaining
            }
        }
        impl ::windows::core::Compose for #original_ident {
            unsafe fn compose<'a>(implementation: Self) -> (::windows::core::IInspectable, &'a mut ::core::option::Option<::windows::core::IInspectable>) {
                let inspectable: ::windows::core::IInspectable = implementation.into();
                let this = (&inspectable as *const _ as *mut ::windows::core::RawPtr).sub(1) as *mut #impl_ident;
                (inspectable, &mut (*this).base)
            }
        }
        impl From<#original_ident> for ::windows::core::IUnknown {
            fn from(this: #original_ident) -> Self {
                unsafe {
                    let this = #impl_ident::new(this);
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
                    ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                        &mut (*ptr).identity as *mut _ as _
                    ))
                }
            }
        }
        impl From<#original_ident> for ::windows::core::IInspectable {
            fn from(this: #original_ident) -> Self {
                unsafe {
                    let this = #impl_ident::new(this);
                    let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
                    ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                        &mut (*ptr).identity as *mut _ as _
                    ))
                }
            }
        }
        #(#froms)*
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
        self.type_name.clone().into()
    }
    fn to_vtbl_ident(&self) -> TokenStream {
        self.to_ident().join("_Vtbl")
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
                let mut def = ImplementType::default();
                def.type_name = input.ident.to_string();

                if !namespace.is_empty() {
                    def.type_name = format!("{}::{}", namespace, def.type_name);
                }

                for g in &input.generics {
                    def.generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(def)
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
