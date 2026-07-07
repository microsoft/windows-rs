use super::*;

#[derive(Debug)]
pub struct Callback {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    /// The source-expressed calling convention, when stated explicitly. `None`
    /// is the platform default (`CALLBACK`/`WINAPI`/`__stdcall`), which the
    /// reader encodes as `CallingConvention.Winapi`.
    pub calling_convention: Option<CallingConvention>,
}

impl Callback {
    /// Try to parse a `CXCursor_TypedefDecl` cursor as a callback (function-pointer typedef).
    ///
    /// Returns `Ok(Some(Callback))` when the typedef's underlying type is a pointer to a
    /// function prototype, `Ok(None)` otherwise.
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        if name.is_empty() {
            return Ok(None);
        }
        // Legacy (editorial-namespace) mode excludes internal, underscore-prefixed
        // function-pointer typedefs (CRT/system callbacks that win32metadata omits).
        // Faithful per-header mode emits them: they are part of the source surface and
        // are referenced by the functions that take them, so dropping them would leave
        // a dangling reference.
        if parser.header_root.is_none() && name.starts_with('_') {
            return Ok(None);
        }

        let underlying = cursor.typedef_underlying_type();

        // Resolve elaborated types (e.g. `typedef struct Foo Foo`) first.
        let underlying = if underlying.kind() == CXType_Elaborated {
            underlying.underlying_type()
        } else {
            underlying
        };

        // We only handle pointer-to-function typedefs.
        let Some(fn_type) = underlying.function_pointee() else {
            return Ok(None);
        };

        // Variadic function-pointer typedefs (e.g. `typedef int (*printf_t)(const char*, ...)`)
        // cannot be represented as Windows metadata callbacks.  Skip them; the caller will
        // fall through to Typedef::parse which emits a `type` alias instead.
        if fn_type.is_variadic() {
            return Ok(None);
        }

        let return_type = fn_type.fn_result_type().to_type(parser);

        // The parameter declarations — names, SAL annotations, and (for two-level
        // typedefs) the calling convention — live on the cursor's `ParmDecl` children.
        // Single-level pointer/function typedefs carry them directly; a two-level typedef
        // (`typedef RET (CONV *NAME)(ARGS); typedef NAME PNAME;`, as with winnt.h's
        // `TP_SIMPLE_CALLBACK` / `PTP_SIMPLE_CALLBACK`) carries them on the base
        // function-type declaration reached via the `TypeRef` child.
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

        // Route callback params through the shared SAL/MIDL machinery so callbacks match
        // functions and COM methods: `In`/`Out`/array-size annotations, string const-ness
        // and `ComOutPtr` promotion, and consistent `param{idx}` naming for unnamed params.
        let midl_annotations = scan_method_param_annotations(&tokens, &source_name);
        let params = parse_params(&param_source, &midl_annotations, parser);

        // Recover an explicitly-stated `__cdecl` / `__fastcall` convention from the
        // declaration's source tokens (clang erases it from the type on 64-bit targets).
        // The platform default (`CALLBACK` / `WINAPI` / `__stdcall`) is left as `None`: the
        // reader encodes it as `CallingConvention.Winapi`, which is faithful on every
        // architecture. The `param_source` tokens carry the convention in both the
        // single-level and two-level cases.
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

        // Only a non-default convention is spelled out; the bare `extern fn` form
        // encodes the platform default (`CallingConvention.Winapi`) in the reader.
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
