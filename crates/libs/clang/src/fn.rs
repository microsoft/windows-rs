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
    /// The DLL export symbol to link against, when it differs from [`name`](Self::name).
    ///
    /// A handful of SDK exports are declared and documented under a source alias whose
    /// object-like macro (`#define RtlGenRandom SystemFunction036`,
    /// `#define EnumProcesses K32EnumProcesses`) textually rewrites the prototype to the
    /// raw export before clang parses it. The faithful projection keeps the documented
    /// [`name`](Self::name) and records the raw export here as the P/Invoke import name so
    /// the linker still resolves the real symbol. `None` for the common case where the
    /// function's name is its own export symbol.
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

/// True when `name` appears in `tokens` as the function-name identifier — the token
/// immediately preceding the parameter-list `(`. Used to confirm that an alias macro
/// actually rewrote *this* prototype (`RtlGenRandom(` → `SystemFunction036`) rather than
/// the export being its own real prototype with a separate back-compat alias declared
/// elsewhere (`#define VarBoolFromInt VarBoolFromI4`, where the literal prototype is
/// `VarBoolFromI4(`).
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

        // Tokenise the function extent once for MIDL comment scanning and source-name
        // recovery. SAL annotations (via cursor children) take priority; the MIDL scan is
        // used as a fallback when no SAL annotation is found on a parameter.
        let fn_tokens = parser
            .tu
            .tokenize(parser.tu.to_expansion_range(cursor.extent()));

        // Recover the source (pre-macro-expansion) spelling when this declaration's name is
        // produced by an object-like alias macro (`#define RtlGenRandom SystemFunction036`,
        // `#define EnumProcesses K32EnumProcesses`). clang reports the expanded export name,
        // but the SDK declares the prototype — and its calling-convention token — under the
        // alias. The rename is applied only when the source tokens confirm that *this*
        // prototype was written with the alias (`RtlGenRandom(`) and not the export: a
        // `#define VarBoolFromInt VarBoolFromI4` back-compat alias, whose real prototype is
        // written as `VarBoolFromI4(`, must keep the documented export name. See
        // `Parser::alias_map`.
        let source_name = parser
            .alias_map
            .get(&export_name)
            .filter(|alias| {
                token_names_function(&fn_tokens, alias)
                    && !token_names_function(&fn_tokens, &export_name)
            })
            .cloned();
        let anchor = source_name.as_deref().unwrap_or(&export_name);

        let midl_annotations = scan_method_param_annotations(&fn_tokens, anchor);
        let calling_convention = detect_calling_convention(&fn_tokens, anchor, parser.macro_defs);

        let mut params = parse_params(&cursor, &midl_annotations, parser);

        // Recover the `ComOutPtr` (`#[iid_is]`) marker on caller-chosen-type COM creators
        // that the SDK headers leave unannotated (no `_COM_Outptr_` SAL, no MIDL `[iid_is]`)
        // — `DCompositionCreateDevice`, the `OleCreate*`/`PS*` families, … — from the
        // signature shape. See [`infer_iid_is`] for the (deliberately narrow) gate.
        infer_iid_is(&mut params, &return_type);

        // Prefer the DLL recovered from the import libraries for this exact
        // symbol; fall back to the run-wide library when the function is not
        // present in any supplied import library. The lookup is by the raw export
        // symbol, which is what the import libraries record.
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
            // A variadic function is always `__cdecl` on Windows: MSVC ignores a
            // stated `__stdcall`/`WINAPI` for varargs and uses `__cdecl`. Emitting
            // any other convention is both wrong and rejected outright by rustc on
            // non-MSVC targets (`extern "system"` C-variadics are an error).
            quote! { "C" }
        } else {
            match self.calling_convention {
                // An explicitly-stated convention is authoritative.
                Some(CallingConvention::Stdcall) => quote! { "system" },
                Some(CallingConvention::Cdecl) => quote! { "C" },
                Some(CallingConvention::Fastcall) => quote! { "fastcall" },
                // No explicit convention: fall back to linkage. An `extern "C"`
                // function with no stated convention defaults to `__cdecl` under
                // MSVC, so emitting `extern "C"` here remains faithful.
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
