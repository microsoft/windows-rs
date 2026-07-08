use super::*;

/// The calling convention as written in the C/C++ source.
///
/// Win32 headers state the convention explicitly via the `__stdcall` / `__cdecl`
/// / `__fastcall` keywords or a macro that expands to one (`WINAPI`, `CALLBACK`,
/// `STDMETHODCALLTYPE`, ...). On a 64-bit target clang erases the convention from
/// both the function type and its spelling — there is a single x64 convention —
/// so it is recovered from the *source tokens* of the declaration instead, which
/// keeps it faithful and architecture-independent (the winmd is arch-neutral; a
/// `__stdcall` function is `CallConvPlatformapi`, correct on every architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    /// `__cdecl` → RDL `extern "C"` (`CallConvCdecl`).
    Cdecl,
    /// `__stdcall` / `WINAPI` → RDL `extern "system"` (`CallConvPlatformapi`,
    /// the Win32 default convention).
    Stdcall,
    /// `__fastcall` → RDL `extern "fastcall"` (`CallConvFastcall`).
    Fastcall,
}

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub library: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    pub extern_c: bool,
    pub is_variadic: bool,
    /// True when the function is `__declspec(noreturn)` / `_Analysis_noreturn_`.
    pub does_not_return: bool,
    /// The source-expressed calling convention, when stated explicitly.
    pub calling_convention: Option<CallingConvention>,
}

/// Map a compiler calling-convention keyword to its [`CallingConvention`].
///
/// `__thiscall` / `__vectorcall` have no winmd representation and are reported as
/// `None`, so they are faithfully omitted rather than approximated.
fn convention_keyword(spelling: &str) -> Option<CallingConvention> {
    match spelling {
        "__stdcall" | "_stdcall" => Some(CallingConvention::Stdcall),
        "__cdecl" | "_cdecl" => Some(CallingConvention::Cdecl),
        "__fastcall" | "_fastcall" => Some(CallingConvention::Fastcall),
        _ => None,
    }
}

/// Resolve a single source token to a calling convention, expanding macros
/// (`WINAPI` → `__stdcall`, `APIENTRY` → `WINAPI` → `__stdcall`, ...) transitively
/// via the translation unit's macro-definition map. A visited set guards against
/// self-referential macros.
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

/// Recover the explicitly-written calling convention from a function declaration.
///
/// The convention token, when present, precedes the function name
/// (`<ret> WINAPI Name(...)`); a calling convention on a function-pointer
/// *parameter* appears later, inside the parameter list, so anchoring on the name
/// token avoids mistaking a callback parameter's convention for the function's. The
/// convention is not always *adjacent* to the name — see the backward scan below.
fn detect_calling_convention(
    tokens: &[(CXTokenKind, String)],
    name: &str,
    macro_defs: &HashMap<String, Vec<String>>,
) -> Option<CallingConvention> {
    // The function name is the token directly preceding the parameter list `(`.
    let name_idx = tokens
        .iter()
        .position(|(_, s)| s == name)
        .filter(|&i| tokens.get(i + 1).is_some_and(|(_, s)| s == "("))?;

    let candidate = &tokens.get(name_idx.checked_sub(1)?)?.1;

    // A function-like convention macro (`STDAPI_(type) Name(...)`,
    // `STDMETHODIMP_(type) Name(...)`) leaves its closing `)` immediately before the
    // name. Walk back to the matching `(` and resolve the macro name preceding it.
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

    // Walk backward across the declaration's return-type / specifier tokens to the
    // first identifier that names a calling convention. Inspecting only the token
    // adjacent to the name is not enough when the convention hides behind
    // preprocessor noise: `clang_tokenize` is purely lexical, so a declaration
    // whose `WINAPI` is written inside a `#ifndef _MAC` / `#else` / `#endif` block
    // (e.g. `DefWindowProc` in `winuser.h`) tokenises as
    // `... WINAPI #else <ret> CALLBACK #endif Name (` — the directive tokens and the
    // inactive branch sit between the convention and the name, so the adjacent token
    // is `endif` and detection would wrongly fall back to the `extern "C"` linkage.
    // Scanning back to the nearest statement boundary recovers it (both `_MAC`
    // branches spell the same platform convention: `CALLBACK` = `WINAPI` =
    // `__stdcall`). The `;`/`{`/`}` boundary stops the scan from borrowing a
    // neighbouring declaration's convention.
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

/// Recover the calling convention of a function-pointer typedef (callback).
///
/// The convention macro sits in the declarator *before* the callback name —
/// `typedef RET (CALLBACK *NAME)...` (pointer form, name preceded by `*`) or
/// `typedef RET CALLBACK NAME(...)` (bare function-type form). The declarator name
/// is the last identifier of the declarator, so anchoring on its final occurrence
/// and scanning backward to the declaration start recovers the convention while
/// ignoring any convention on a function-pointer *parameter* (which appears after
/// the name). Returns `None` for the platform default (`CALLBACK`/`WINAPI`/
/// `__stdcall`), which the reader already encodes as `CallingConvention.Winapi`.
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

impl Fn {
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, extern_c: bool) -> Result<Self, Error> {
        let name = cursor.name();
        let return_type = cursor.result_type().to_type(parser);

        let is_variadic = cursor.ty().is_variadic();
        let does_not_return = detect_does_not_return(&cursor);

        // Tokenise the function extent once for MIDL comment scanning.
        // SAL annotations (via cursor children) take priority; the MIDL scan is
        // used as a fallback when no SAL annotation is found on a parameter.
        let fn_tokens = parser
            .tu
            .tokenize(parser.tu.to_expansion_range(cursor.extent()));
        let midl_annotations = scan_method_param_annotations(&fn_tokens, &name);
        let calling_convention = detect_calling_convention(&fn_tokens, &name, parser.macro_defs);

        let params = parse_params(&cursor, &midl_annotations, parser);

        // Prefer the DLL recovered from the import libraries for this exact
        // symbol; fall back to the run-wide library when the function is not
        // present in any supplied import library.
        let library = parser
            .libraries
            .get(&name)
            .cloned()
            .unwrap_or_else(|| parser.library.to_string());

        Ok(Self {
            name,
            library,
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

        let abi = match self.calling_convention {
            // An explicitly-stated convention is authoritative.
            Some(CallingConvention::Stdcall) => quote! { "system" },
            Some(CallingConvention::Cdecl) => quote! { "C" },
            Some(CallingConvention::Fastcall) => quote! { "fastcall" },
            // No explicit convention: fall back to linkage. An `extern "C"`
            // function with no stated convention defaults to `__cdecl` under
            // MSVC, so emitting `extern "C"` here remains faithful.
            None if self.extern_c => quote! { "C" },
            None => quote! {},
        };

        let does_not_return = if self.does_not_return {
            does_not_return_attr()
        } else {
            quote! {}
        };

        Ok(quote! {
            #does_not_return
            #[library(#library)]
            extern #abi fn #name(#(#params),*) #return_type;
        })
    }
}
