use super::*;

#[derive(Debug)]
pub struct Callback {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    /// Non-default source calling convention; `None` is the platform default.
    pub calling_convention: Option<CallingConvention>,
}

impl Callback {
    /// Parse a non-variadic function-pointer typedef as a callback.
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        if name.is_empty() {
            return Ok(None);
        }
        // Per-header mode keeps underscore callbacks because functions may reference them.
        if parser.header_root.is_none() && name.starts_with('_') {
            return Ok(None);
        }

        let underlying = cursor.typedef_underlying_type();

        let underlying = if underlying.kind() == CXType_Elaborated {
            underlying.underlying_type()
        } else {
            underlying
        };

        let Some(fn_type) = underlying.function_pointee() else {
            return Ok(None);
        };

        // Variadic callbacks are emitted later as opaque typedef aliases.
        if fn_type.is_variadic() {
            return Ok(None);
        }

        let return_type = fn_type.fn_result_type().to_type(parser);

        // Two-level typedefs carry parameter declarations on the referenced base typedef.
        let param_source = if cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_ParmDecl)
        {
            cursor
        } else {
            cursor
                .children()
                .into_iter()
                .find(|c| c.kind() == CXCursor_TypeRef)
                .map(|c| c.referenced())
                .filter(|r| r.children().iter().any(|c| c.kind() == CXCursor_ParmDecl))
                .unwrap_or(cursor)
        };

        let source_name = param_source.name();
        let tokens = parser
            .tu
            .tokenize(parser.tu.to_expansion_range(param_source.extent()));

        // Use the shared SAL/MIDL path so callbacks match functions and COM methods.
        let midl_annotations =
            scan_method_param_annotations(&tokens, &source_name, parser.macro_defs);
        let params = parse_params(&param_source, &midl_annotations, parser);

        // clang erases x64 conventions from the type, so recover non-default ones from tokens.
        let calling_convention =
            detect_callback_calling_convention(&tokens, &source_name, parser.macro_defs);

        Ok(Some(Self {
            name,
            params,
            return_type,
            calling_convention,
        }))
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let params = self.params.iter().map(|param| {
            let name = write_ident(&param.name);
            let ty = write_type(namespace, &param.ty);
            let attrs = param_attrs_for_annotation(&param.annotation, &param.ty);
            quote! { #(#attrs)* #name: #ty }
        });

        let return_type = match &self.return_type {
            metadata::Type::Void => quote! {},
            ty => {
                let ty = write_type(namespace, ty);
                quote! { -> #ty }
            }
        };

        // Bare `extern fn` encodes the platform default in the reader.
        let abi = match self.calling_convention {
            Some(CallingConvention::Cdecl) => quote! { "C" },
            Some(CallingConvention::Fastcall) => quote! { "fastcall" },
            Some(CallingConvention::Stdcall) | None => quote! {},
        };

        Ok(quote! {
            extern #abi fn #name(#(#params),*) #return_type;
        })
    }
}
