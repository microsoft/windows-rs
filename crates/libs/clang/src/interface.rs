use super::*;

/// COM interface method.
#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    /// `[propget]` marker from the MIDL method comment.
    pub is_propget: bool,
    /// `[propput]` marker from the MIDL method comment.
    pub is_propput: bool,
}

/// COM-style abstract interface parsed from a C++ `struct`/`class`.
#[derive(Debug)]
pub struct Interface {
    pub name: String,
    /// UUID string without braces or quotes.
    pub guid: Option<String>,
    /// Base interface type, qualified from `ref_map` when needed.
    pub base: Option<metadata::Type>,
    pub methods: Vec<InterfaceMethod>,
}

impl Interface {
    /// Parse a C++ abstract record as a COM interface.
    ///
    /// Consecutive same-name overloads are reversed to match MSVC vtable layout.
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists.
        let name = parser
            .tag_rename
            .get(&tag_name)
            .cloned()
            .unwrap_or(tag_name);
        let guid = cursor.extract_uuid(parser.tu);

        // COM interfaces inherit from at most one base.
        let base = cursor.children().iter().find_map(|c| {
            if c.kind() == CXCursor_CXXBaseSpecifier {
                let base_name = c.ty().ty().name();
                if !base_name.is_empty() {
                    // Flat mode resolves in the root namespace; namespaced mode qualifies bases.
                    let base_ns = if parser.header_root.is_some() {
                        parser.namespace.to_string()
                    } else {
                        parser
                            .ref_map
                            .get(&base_name)
                            .map_or(parser.namespace, |s| s.as_str())
                            .to_string()
                    };
                    return Some(metadata::Type::value_named(&base_ns, &base_name));
                }
            }
            None
        });

        let mut methods = vec![];
        for child in cursor.children() {
            if child.kind() != CXCursor_CXXMethod || !child.is_pure_virtual() {
                continue;
            }

            // `DECLARE_INTERFACE_` redeclarations reuse base slots; emitting them doubles slots.
            if child.overrides_base_method() {
                continue;
            }

            let method_name = demacro_member_name(child.name(), parser.macro_defs);
            let tokens = parser
                .tu
                .tokenize(parser.tu.to_expansion_range(child.extent()));
            let method_annotation = extract_method_annotation(&tokens, &method_name);
            // SAL annotations take priority; MIDL comments are a fallback.
            let midl_param_annotations =
                scan_method_param_annotations(&tokens, &method_name, parser.macro_defs);
            let return_type = child.result_type().to_type(parser);

            let mut params = parse_params(&child, &midl_param_annotations, parser);

            // Recover missing caller-chosen-type COM annotations from signature shape.
            infer_iid_is(&mut params, &return_type);

            methods.push(InterfaceMethod {
                name: method_name,
                params,
                return_type,
                is_propget: method_annotation.is_propget,
                is_propput: method_annotation.is_propput,
            });
        }

        // MSVC vtables store consecutive same-name overloads in reverse source order.
        let mut start = 0;
        while start < methods.len() {
            let mut end = start + 1;
            while end < methods.len() && methods[end].name == methods[start].name {
                end += 1;
            }
            methods[start..end].reverse();
            start = end;
        }

        Ok(Self {
            name,
            guid,
            base,
            methods,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let guid_token = if let Some(uuid) = &self.guid {
            let lit_str = uuid_to_u128_literal(uuid);
            let lit = syn::LitInt::new(&lit_str, Span::call_site());
            quote! { #[guid(#lit)] }
        } else {
            quote! { #[no_guid] }
        };

        let requires_token = if let Some(base_type) = &self.base {
            let base_tokens = write_type(namespace, base_type);
            quote! { : #base_tokens }
        } else {
            quote! {}
        };

        let methods_tokens: Vec<TokenStream> = self
            .methods
            .iter()
            .map(|m| {
                let mname = write_ident(&m.name);
                let params = m.params.iter().map(|p| {
                    let pname = write_ident(&p.name);
                    let pty = write_type(namespace, &p.ty);
                    let attrs = param_attrs_for_annotation(&p.annotation, &p.ty);
                    quote! { #(#attrs)* #pname: #pty }
                });
                let return_type = match &m.return_type {
                    metadata::Type::Void => quote! {},
                    ty => {
                        let ty = write_type(namespace, ty);
                        quote! { -> #ty }
                    }
                };
                let special_attr = if m.is_propget || m.is_propput {
                    quote! { #[special] }
                } else {
                    quote! {}
                };
                quote! { #special_attr fn #mname(&self, #(#params),*) #return_type; }
            })
            .collect();

        Ok(quote! {
            #guid_token
            interface #name #requires_token {
                #(#methods_tokens)*
            }
        })
    }
}
