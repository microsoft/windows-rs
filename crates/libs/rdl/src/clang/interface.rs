use super::*;

/// A single method on a COM interface, parsed from a pure-virtual `CXCursor_CXXMethod`.
#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
}

/// A COM-style abstract interface parsed from a C++ `struct`/`class` that has at least one
/// pure-virtual method (and optionally a `__declspec(uuid(...))` attribute).
#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub namespace: String,
    /// The UUID string (without braces or quotes), e.g.
    /// `"00000000-0000-0000-c000-000000000046"`, if a `__declspec(uuid(...))` attribute was found.
    pub guid: Option<String>,
    /// The unqualified name of the single base interface (e.g. `"IUnknown"`), if any.
    pub base: Option<String>,
    pub methods: Vec<InterfaceMethod>,
}

impl Interface {
    /// Returns `true` if `cursor` is a `struct`/`class` that looks like a COM interface —
    /// i.e. it has at least one pure-virtual method child.
    pub fn is_com_interface(cursor: Cursor) -> bool {
        cursor.has_pure_virtual_methods()
    }

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
    ) -> Result<Self, Error> {
        let name = cursor.name();
        let guid = cursor.extract_uuid(tu);

        // Find the first base class (COM interfaces only inherit from one base).
        let base = cursor.children().iter().find_map(|c| {
            if c.kind() == CXCursor_CXXBaseSpecifier {
                // Use the type declaration's spelling to get the unqualified base name.
                let base_name = c.ty().ty().name();
                if !base_name.is_empty() {
                    return Some(base_name);
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
            let return_type = child.result_type().to_type(namespace, ref_map);

            let mut params = vec![];
            for param in child.children() {
                if param.kind() != CXCursor_ParmDecl {
                    continue;
                }
                let param_name = param.name();
                let param_ty = param.ty().to_type(namespace, ref_map);
                params.push(Param {
                    name: param_name,
                    ty: param_ty,
                });
            }

            methods.push(InterfaceMethod {
                name: method_name,
                params,
                return_type,
            });
        }

        Ok(Self {
            name,
            namespace: namespace.to_string(),
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
    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let guid_token = if let Some(uuid) = &self.guid {
            let lit_str = uuid_to_u128_literal(uuid);
            let lit = syn::LitInt::new(&lit_str, Span::call_site());
            quote! { #[guid(#lit)] }
        } else {
            quote! { #[no_guid] }
        };

        let requires_token = if let Some(base) = &self.base {
            let base_ident = write_ident(base);
            quote! { : #base_ident }
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
                    let pty = write_type(&self.namespace, &p.ty);
                    quote! { #pname: #pty }
                });
                let return_type = match &m.return_type {
                    metadata::Type::Void => quote! {},
                    ty => {
                        let ty = write_type(&self.namespace, ty);
                        quote! { -> #ty }
                    }
                };
                quote! { fn #mname(&self, #(#params),*) #return_type; }
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

/// Converts a UUID string (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`) into the u128 hex literal
/// format used in RDL GUID attributes, e.g. `"0x00000000_0000_0000_c000_000000000046"`.
///
/// The format matches `format_guid_u128` in `writer/mod.rs`:
/// `0x{d1:08x}_{d2:04x}_{d3:04x}_{d4_word:04x}_{d4_node:012x}`.
///
/// # Panics
///
/// Panics if `uuid` does not match the format validated by [`is_uuid_format`] (i.e. five
/// hyphen-separated groups of 8-4-4-4-12 hex digits).  Callers must ensure `is_uuid_format`
/// returns `true` for `uuid` before calling this function.
fn uuid_to_u128_literal(uuid: &str) -> String {
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(
        parts.len(),
        5,
        "uuid_to_u128_literal: expected 5 hyphen-separated groups in `{uuid}`"
    );
    let d1 = u32::from_str_radix(parts[0], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d1 in `{uuid}`"));
    let d2 = u16::from_str_radix(parts[1], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d2 in `{uuid}`"));
    let d3 = u16::from_str_radix(parts[2], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d3 in `{uuid}`"));
    let d4_str = format!("{}{}", parts[3], parts[4]);
    let mut d4 = [0u8; 8];
    for i in 0..8 {
        d4[i] = u8::from_str_radix(&d4_str[i * 2..i * 2 + 2], 16)
            .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d4[{i}] in `{uuid}`"));
    }
    let d4_word = u16::from_be_bytes([d4[0], d4[1]]);
    let d4_node = u64::from_be_bytes([0, 0, d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]);
    format!("0x{d1:08x}_{d2:04x}_{d3:04x}_{d4_word:04x}_{d4_node:012x}")
}
