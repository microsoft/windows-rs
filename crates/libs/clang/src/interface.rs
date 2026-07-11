use super::*;

/// A single method on a COM interface, parsed from a pure-virtual `CXCursor_CXXMethod`.
#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    /// True if `[propget]` was found in the block comment preceding the method name.
    pub is_propget: bool,
    /// True if `[propput]` was found in the block comment preceding the method name.
    pub is_propput: bool,
}

/// A COM-style abstract interface parsed from a C++ `struct`/`class` that has at least one
/// pure-virtual method (and optionally a `__declspec(uuid(...))` attribute).
#[derive(Debug)]
pub struct Interface {
    pub name: String,
    /// The UUID string (without braces or quotes), e.g.
    /// `"00000000-0000-0000-c000-000000000046"`, if a `__declspec(uuid(...))` attribute was found.
    pub guid: Option<String>,
    /// The base interface type (with its namespace from `ref_map` when available), if any.
    pub base: Option<metadata::Type>,
    pub methods: Vec<InterfaceMethod>,
}

impl Interface {
    /// Parse a `CXCursor_StructDecl` or `CXCursor_ClassDecl` cursor as a COM interface.
    ///
    /// Extracts:
    /// - The type name from the cursor spelling.
    /// - An optional UUID from any `__declspec(uuid("..."))` attribute (`CXCursor_UnexposedAttr`).
    /// - An optional single base interface from the first `CXCursor_CXXBaseSpecifier` child.
    /// - All pure-virtual `CXCursor_CXXMethod` children as interface methods.
    ///
    /// Overloaded (same-name) methods are reordered to match MSVC's vtable layout:
    /// a run of consecutive same-name virtual functions is emitted in reverse
    /// declaration order (see the reversal note near the end of this function).
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists (e.g. `_IFoo` → `IFoo`).
        let name = parser
            .tag_rename
            .get(&tag_name)
            .cloned()
            .unwrap_or(tag_name);
        let guid = cursor.extract_uuid(parser.tu);

        // Find the first base class (COM interfaces only inherit from one base).
        let base = cursor.children().iter().find_map(|c| {
            if c.kind() == CXCursor_CXXBaseSpecifier {
                // Use the type declaration's spelling to get the unqualified base name.
                let base_name = c.ty().ty().name();
                if !base_name.is_empty() {
                    // Check if the base interface exists in the reference metadata; if so,
                    // use its reference namespace so the emitted path is fully qualified.
                    // In flat per-header mode every interface lives in the single root
                    // namespace, so the base resolves there by name.
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

        // Collect pure-virtual methods as interface methods.
        let mut methods = vec![];
        for child in cursor.children() {
            if child.kind() != CXCursor_CXXMethod || !child.is_pure_virtual() {
                continue;
            }

            // Old-style `DECLARE_INTERFACE_` headers redeclare the entire inherited method
            // chain in each derived interface. Those redeclarations override base-class
            // virtuals and reuse their vtable slots, so skip them — the emitted `base__`
            // vtable already reconstructs the full inherited chain. Emitting them again
            // would double the inherited slots and corrupt the vtable layout.
            if child.overrides_base_method() {
                continue;
            }

            let method_name = demacro_member_name(child.name(), parser);
            let tokens = parser
                .tu
                .tokenize(parser.tu.to_expansion_range(child.extent()));
            let method_annotation = extract_method_annotation(&tokens, &method_name);
            // Pre-scan the method tokens for MIDL parameter comment annotations.
            // SAL annotations on each ParmDecl cursor take priority; MIDL comments
            // are used as a fallback.
            let midl_param_annotations = scan_method_param_annotations(&tokens, &method_name);
            let return_type = child.result_type().to_type(parser);

            let params = parse_params(&child, &midl_param_annotations, parser);

            methods.push(InterfaceMethod {
                name: method_name,
                params,
                return_type,
                is_propget: method_annotation.is_propget,
                is_propput: method_annotation.is_propput,
            });
        }

        // MSVC lays out a run of overloaded (same-name) virtual functions into the
        // vtable in reverse declaration order. libclang yields cursors in source
        // order, so reverse each maximal run of consecutive same-name methods to
        // match the real COM vtable ABI. Runs of length one are unaffected.
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

    /// Emit this interface as an RDL token stream.
    ///
    /// Produces:
    /// ```text
    /// #[guid(0x…)] | #[no_guid]
    /// interface Name [: Base] {
    ///     fn Method(&self, params…) [-> RetType];
    ///     …
    /// }
    /// ```
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

/// Restores a COM method's true member name when it was rewritten by a Win32
/// A/W name macro during preprocessing.
///
/// The scrape runs without `UNICODE` defined, so headers like `winuser.h`
/// leave object-like aliases such as `#define DrawText DrawTextA` active. When
/// a COM interface declares a method whose identifier collides with one of
/// those macros (e.g. `ID2D1RenderTarget::DrawText`), libclang reports the
/// expanded spelling (`DrawTextA`). If `macro_defs` holds an alias whose sole
/// replacement token is the expanded name, the base identifier is the faithful
/// member name and is restored. Free functions are unaffected: their real
/// exported symbols already are the `A`/`W` names.
fn demacro_member_name(name: String, parser: &Parser<'_>) -> String {
    if let Some(base) = name.strip_suffix('A').or_else(|| name.strip_suffix('W'))
        && parser
            .macro_defs
            .get(base)
            .is_some_and(|body| body.len() == 1 && body[0] == name)
    {
        return base.to_string();
    }
    name
}
