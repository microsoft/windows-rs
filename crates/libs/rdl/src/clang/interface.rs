use super::*;

/// MIDL attributes extracted from the leading block comment of a pure-virtual
/// method declaration. `propget` indicates a property accessor function.
#[derive(Debug, Default)]
struct MethodModifiers {
    /// True when `[propget]` appears in a block comment before the method name.
    is_propget: bool,
}

impl MethodModifiers {
    /// Extract method modifiers by scanning `tokens` up to (but not including)
    /// the first identifier token whose spelling equals `method_name`.
    ///
    /// Only `CXToken_Comment` tokens are inspected; all others are skipped.
    /// Scanning stops at the function name to avoid reading into the parameter list.
    fn from_tokens(tokens: &[(CXTokenKind, String)], method_name: &str) -> Self {
        let mut modifiers = Self::default();
        for (kind, spelling) in tokens {
            if *kind == CXToken_Identifier && spelling == method_name {
                break;
            }
            if *kind == CXToken_Comment && spelling.contains("[propget]") {
                modifiers.is_propget = true;
            }
        }
        modifiers
    }
}

/// A single method on a COM interface, parsed from a pure-virtual `CXCursor_CXXMethod`.
#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    /// True if `[propget]` was found in the block comment preceding the method name.
    pub is_propget: bool,
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
    pub fn parse(
        cursor: Cursor,
        namespace: &str,
        tu: &TranslationUnit,
        ref_map: &HashMap<String, String>,
        tag_rename: &HashMap<String, String>,
        pending: &mut Vec<Cursor>,
    ) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists (e.g. `_IFoo` → `IFoo`).
        let name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
        let guid = cursor.extract_uuid(tu);

        // Find the first base class (COM interfaces only inherit from one base).
        let base = cursor.children().iter().find_map(|c| {
            if c.kind() == CXCursor_CXXBaseSpecifier {
                // Use the type declaration's spelling to get the unqualified base name.
                let base_name = c.ty().ty().name();
                if !base_name.is_empty() {
                    // Check if the base interface exists in the reference metadata; if so,
                    // use its reference namespace so the emitted path is fully qualified.
                    let base_ns = ref_map
                        .get(&base_name)
                        .map(|s| s.as_str())
                        .unwrap_or(namespace);
                    return Some(metadata::Type::value_named(base_ns, &base_name));
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

            let method_name = child.name();
            let tokens = tu.tokenize(tu.to_expansion_range(child.extent()));
            let modifiers = MethodModifiers::from_tokens(&tokens, &method_name);
            let return_type = child
                .result_type()
                .to_type(namespace, ref_map, tag_rename, pending);

            let mut params = vec![];
            for param in child.children() {
                if param.kind() != CXCursor_ParmDecl {
                    continue;
                }
                let param_name = param.name();
                let param_ty = param.ty().to_type(namespace, ref_map, tag_rename, pending);
                params.push(Param {
                    name: param_name,
                    ty: param_ty,
                });
            }

            methods.push(InterfaceMethod {
                name: method_name,
                params,
                return_type,
                is_propget: modifiers.is_propget,
            });
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
                    quote! { #pname: #pty }
                });
                let return_type = match &m.return_type {
                    metadata::Type::Void => quote! {},
                    ty => {
                        let ty = write_type(namespace, ty);
                        quote! { -> #ty }
                    }
                };
                let special_attr = if m.is_propget {
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
