// TODO: sort these out

use crate::*;

pub fn gen_field(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature();

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = constant.value().gen();

            quote! {
                pub const #name: #value;
            }
        } else {
            let kind = signature.gen_win32(gen);
            let value = constant.value().gen_value();

            quote! {
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = Guid::from_attributes(def.attributes()) {
        let guid = guid.gen();
        quote! { pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = signature.gen_win32(gen);
        let fmtid = pkey.fmtid.gen();
        let pid = pkey.pid;
        quote! {
            pub const #name: #kind = #kind {
                fmtid: ::windows::Guid::from_values(#fmtid),
                pid: #pid,
            };
        }
    } else {
        quote! {}
    }
}

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_method_name(def, gen); // TODO: this probably doesn't need gen
    let signature = def.signature(&[]);

    let constraints = signature.gen_constraints(&signature.params);
    let params = signature.gen_win32_params(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = p.param.gen_name();
        let tokens = p.gen_win32_abi_param(gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = if let Some(t) = &signature.return_type {
        // TODO: This should be gen_win32_abi?
        let tokens = t.gen_win32(gen);
        quote! { -> #tokens }
    } else {
        TokenStream::new()
    };

    let args = signature.params.iter().map(|p| p.gen_win32_abi_arg());
    let mut link = def.impl_map().expect("Function").scope().name();

    let raw_dylib = cfg!(feature = "raw_dylib");

    // TODO: remove this whole block once raw-dylib has stabilized as the workarounds
    // will no longer be necessary.
    if !raw_dylib
        && (link.contains("-ms-win-") || link == "D3DCOMPILER_47" || link == "SspiCli")
    {
        link = "onecoreuap";
    }

    let static_lib = def
        .attributes()
        .filter_map(|attribute| match attribute.name() {
            "StaticLibraryAttribute" => Some(attribute.args()[0].1.gen_value()),
            _ => None,
        })
        .next();

    let link_attr = match static_lib {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            // TODO: switch to always using raw-dylib once it has stabilized
            if raw_dylib {
                quote! { #[link(name = #link, kind="raw-dylib")] }
            } else {
                quote! { #[link(name = #link)] }
            }
        }
    };

    if signature.has_query_interface() {
        let leading_params = &signature.params[..signature.params.len() - 2];
        let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
        let params = signature.gen_win32_params(leading_params, gen);

        quote! {
            pub unsafe fn #name<#constraints T: ::windows::Interface>(#params) -> ::windows::Result<T> {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*) #abi_return_type;
                    }
                    let mut result__ = ::std::option::Option::None;
                    #name(#(#args,)* &<T as ::windows::Interface>::IID, ::windows::Abi::set_abi(&mut result__)).and_some(result__)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    } else if signature.has_retval() {
        let leading_params = &signature.params[..signature.params.len() - 1];
        let args = leading_params.iter().map(|p| p.gen_win32_abi_arg());
        let params = signature.gen_win32_params(leading_params, gen);

        let return_type_tokens = signature
            .params
            .last()
            .unwrap()
            .signature
            .kind
            .gen_name(gen);

        quote! {
            pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<#return_type_tokens> {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*) #abi_return_type;
                    }
                    let mut result__: <#return_type_tokens as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    #name(#(#args,)* &mut result__).from_abi::<#return_type_tokens>(result__)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    } else if let Some(return_type) = &signature.return_type {
        match &return_type.kind {
            ElementType::HRESULT => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) -> ::windows::HRESULT;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            ElementType::TypeDef(def) if def.type_name() == TypeName::NTSTATUS => {
                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> ::windows::Result<()> {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            _ => {
                let return_type = return_type.gen_win32(gen);

                quote! {
                    pub unsafe fn #name<#constraints>(#params) -> #return_type {
                        #[cfg(windows)]
                        {
                            #link_attr
                            extern "system" {
                                fn #name(#(#abi_params),*) #abi_return_type;
                            }
                            #name(#(#args),*)
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
        }
    } else {
        quote! {
            pub unsafe fn #name<#constraints>(#params) {
                #[cfg(windows)]
                {
                    #link_attr
                    extern "system" {
                        fn #name(#(#abi_params),*);
                    }
                    #name(#(#args),*)
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
    }
}

pub fn gen_field_name(def:&Field) -> TokenStream {
    // TODO: This should simply return: def.name().to_string()
    let name = format_ident!("{}", def.name());
    quote! { #name }
}

pub fn gen_method_name(def:&MethodDef, gen: &Gen) -> TokenStream {
    let namespace = gen.namespace(def.parent().namespace());
    let name = format_ident!("{}", def.name());
    quote! { #namespace #name }
}