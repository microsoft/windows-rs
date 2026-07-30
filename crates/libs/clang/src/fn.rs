use super::*;

/// Calling convention as spelled in source; clang erases it from x64 function types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    Cdecl,
    /// `__stdcall` / `WINAPI` -> RDL `extern "system"`.
    Stdcall,
    Fastcall,
}

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub library: String,
    /// Raw DLL export symbol when an alias macro rewrote the documented source name.
    pub import_name: Option<String>,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    pub extern_c: bool,
    pub is_variadic: bool,
    /// True when the function is `__declspec(noreturn)` / `_Analysis_noreturn_`.
    pub does_not_return: bool,
    /// The source-expressed calling convention, when stated explicitly.
    pub calling_convention: Option<CallingConvention>,
}

/// Map a compiler keyword to a winmd-representable calling convention.
fn convention_keyword(spelling: &str) -> Option<CallingConvention> {
    match spelling {
        "__stdcall" | "_stdcall" => Some(CallingConvention::Stdcall),
        "__cdecl" | "_cdecl" => Some(CallingConvention::Cdecl),
        "__fastcall" | "_fastcall" => Some(CallingConvention::Fastcall),
        _ => None,
    }
}

/// Resolve a source token through convention macros such as `WINAPI` and `APIENTRY`.
fn resolve_convention<'a>(
    spelling: &'a str,
    macro_defs: &'a HashMap<String, Vec<String>>,
    visited: &mut HashSet<&'a str>,
) -> Option<CallingConvention> {
    if let Some(convention) = convention_keyword(spelling) {
        return Some(convention);
    }

    if !visited.insert(spelling) {
        return None;
    }

    let body = macro_defs.get(spelling)?;
    body.iter()
        .find_map(|token| resolve_convention(token, macro_defs, visited))
}

/// Recover the function's own convention, ignoring callback-parameter conventions.
fn detect_calling_convention(
    tokens: &[(CXTokenKind, String)],
    name: &str,
    macro_defs: &HashMap<String, Vec<String>>,
) -> Option<CallingConvention> {
    let name_idx = tokens
        .iter()
        .position(|(_, s)| s == name)
        .filter(|&i| tokens.get(i + 1).is_some_and(|(_, s)| s == "("))?;

    let candidate = &tokens.get(name_idx.checked_sub(1)?)?.1;

    // `STDAPI_(type) Name(...)` leaves its closing `)` before the name.
    if candidate == ")" {
        let mut depth = 0usize;
        let mut i = name_idx - 1;
        loop {
            match tokens[i].1.as_str() {
                ")" => depth += 1,
                "(" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            i = i.checked_sub(1)?;
        }
        let macro_name = &tokens.get(i.checked_sub(1)?)?.1;
        return resolve_convention(macro_name, macro_defs, &mut HashSet::new());
    }

    // Preprocessor tokens can sit between the convention and the name; stop at a
    // declaration boundary so the scan cannot borrow a neighbour's convention.
    let mut i = name_idx.checked_sub(1)?;
    loop {
        let token = tokens[i].1.as_str();
        if matches!(token, ";" | "{" | "}") {
            return None;
        }
        if let Some(convention) = resolve_convention(token, macro_defs, &mut HashSet::new()) {
            return Some(convention);
        }
        i = i.checked_sub(1)?;
    }
}

/// Recover a callback typedef's non-default calling convention from source tokens.
pub(super) fn detect_callback_calling_convention(
    tokens: &[(CXTokenKind, String)],
    name: &str,
    macro_defs: &HashMap<String, Vec<String>>,
) -> Option<CallingConvention> {
    let name_idx = tokens.iter().rposition(|(_, s)| s == name)?;

    let mut i = name_idx.checked_sub(1)?;
    loop {
        let token = tokens[i].1.as_str();
        if matches!(token, ";" | "{" | "}") || token == "typedef" {
            return None;
        }
        if let Some(convention) = resolve_convention(token, macro_defs, &mut HashSet::new()) {
            return Some(convention);
        }
        i = i.checked_sub(1)?;
    }
}

/// True when `name` is this declaration's function-name token, not a separate alias.
fn token_names_function(tokens: &[(CXTokenKind, String)], name: &str) -> bool {
    tokens
        .iter()
        .position(|(_, s)| s == name)
        .is_some_and(|i| tokens.get(i + 1).is_some_and(|(_, s)| s == "("))
}

impl Fn {
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, extern_c: bool) -> Result<Self, Error> {
        let export_name = cursor.name();
        let return_type = cursor.result_type().to_type(parser);

        let is_variadic = cursor.ty().is_variadic();
        let does_not_return = detect_does_not_return(&cursor);

        // SAL annotations take priority; MIDL comments are a fallback.
        let fn_tokens = parser
            .tu
            .tokenize(parser.tu.to_expansion_range(cursor.extent()));

        // Restore source aliases only when this prototype was written with the alias; some
        // back-compat aliases point at a real export prototype that must keep its name.
        let source_name = parser
            .alias_map
            .get(&export_name)
            .filter(|alias| {
                token_names_function(&fn_tokens, alias)
                    && !token_names_function(&fn_tokens, &export_name)
            })
            .cloned();
        let anchor = source_name.as_deref().unwrap_or(&export_name);

        let midl_annotations = scan_method_param_annotations(&fn_tokens, anchor, parser.macro_defs);
        let calling_convention = detect_calling_convention(&fn_tokens, anchor, parser.macro_defs);

        let mut params = parse_params(&cursor, &midl_annotations, parser);

        // Recover missing caller-chosen-type COM annotations from signature shape.
        infer_iid_is(&mut params, &return_type);

        // Import libraries record the raw export symbol, not a source alias.
        let library = parser
            .libraries
            .get(&export_name)
            .cloned()
            .unwrap_or_else(|| parser.library.to_string());

        let (name, import_name) = match source_name {
            Some(source) => (source, Some(export_name)),
            None => (export_name, None),
        };

        Ok(Self {
            name,
            library,
            import_name,
            params,
            return_type,
            extern_c,
            is_variadic,
            does_not_return,
            calling_convention,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let library = &self.library;

        let mut params: Vec<TokenStream> = self
            .params
            .iter()
            .map(|param| {
                let name = write_ident(&param.name);
                let ty = write_type(namespace, &param.ty);
                let attrs = param_attrs_for_annotation(&param.annotation, &param.ty);
                quote! { #(#attrs)* #name: #ty }
            })
            .collect();

        if self.is_variadic {
            params.push(quote! { ... });
        }

        let return_type = match &self.return_type {
            metadata::Type::Void => quote! {},
            ty => {
                let ty = write_type(namespace, ty);
                quote! { -> #ty }
            }
        };

        let abi = if self.is_variadic {
            // Windows varargs are always `__cdecl`; rustc rejects `extern "system"` C varargs.
            quote! { "C" }
        } else {
            match self.calling_convention {
                Some(CallingConvention::Stdcall) => quote! { "system" },
                Some(CallingConvention::Cdecl) => quote! { "C" },
                Some(CallingConvention::Fastcall) => quote! { "fastcall" },
                // No explicit convention: `extern "C"` linkage implies MSVC `__cdecl`.
                None if self.extern_c => quote! { "C" },
                None => quote! {},
            }
        };

        let does_not_return = if self.does_not_return {
            does_not_return_attr()
        } else {
            quote! {}
        };

        let library_attr = if let Some(import) = &self.import_name {
            quote! { #[library(#library, import = #import)] }
        } else {
            quote! { #[library(#library)] }
        };

        Ok(quote! {
            #does_not_return
            #library_attr
            extern #abi fn #name(#(#params),*) #return_type;
        })
    }
}
