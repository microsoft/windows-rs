use super::*;

/// Flags derived from SAL annotations or MIDL block-comment annotations on a parameter.
///
/// SAL annotations (`_In_`, `_Out_`, `_Inout_`, `_In_opt_`, etc.) are extracted from
/// `CXCursor_AnnotateAttr` or `CXCursor_UnexposedAttr` children of a `CXCursor_ParmDecl`
/// cursor.
///
/// MIDL comment annotations (`/* [in] */`, `/* [out] */`, `/* [retval][out] */`, etc.)
/// are extracted by tokenising the `ParmDecl` cursor extent and looking for
/// `CXToken_Comment` tokens that match the known MIDL attribute names.
///
/// When no annotation is detected, all fields remain `false` and direction defaults are
/// inferred from the parameter's type (mutable pointer/reference → Out, everything else → In),
/// matching the reader's `parse_param_attributes` logic.
#[derive(Debug, Default, Clone)]
pub struct ParamAnnotation {
    /// The parameter is annotated as an input (`_In_`, `_In_opt_`, `_Inout_`, `[in]`, …).
    pub in_param: bool,
    /// The parameter is annotated as an output (`_Out_`, `_Out_opt_`, `_Inout_`, `[out]`, …).
    pub out_param: bool,
    /// The parameter may be `NULL` / absent (`_In_opt_`, `_Out_opt_`, `[optional]`, …).
    pub optional: bool,
    /// The parameter is a retval (`[retval]` MIDL comment annotation).
    pub retval: bool,
}

impl ParamAnnotation {
    /// Returns `true` if any annotation was detected.
    pub fn is_annotated(&self) -> bool {
        self.in_param || self.out_param || self.optional || self.retval
    }
}

/// Method-level MIDL attributes extracted from block comments in a method declaration.
///
/// `is_propget` and `is_propput` indicate COM property accessor methods.  Both are
/// collapsed to `#[special]` in the RDL output because the get/put distinction is
/// already encoded in the `get_`/`put_` method name prefix.
#[derive(Debug, Default)]
pub struct MethodAnnotation {
    /// True when `[propget]` appears in a block comment before the method name.
    pub is_propget: bool,
    /// True when `[propput]` appears in a block comment before the method name.
    pub is_propput: bool,
}

/// Extract method-level MIDL annotations by scanning `tokens` up to (but not
/// including) the first identifier token whose spelling equals `method_name`.
///
/// Only `CXToken_Comment` tokens are inspected; all others are skipped.
/// Scanning stops at the function name to avoid reading into the parameter list.
pub fn extract_method_annotation(
    tokens: &[(CXTokenKind, String)],
    method_name: &str,
) -> MethodAnnotation {
    let mut annotation = MethodAnnotation::default();
    for (kind, spelling) in tokens {
        if *kind == CXToken_Identifier && spelling == method_name {
            break;
        }
        if *kind == CXToken_Comment {
            if spelling.contains("[propget]") {
                annotation.is_propget = true;
            }
            if spelling.contains("[propput]") {
                annotation.is_propput = true;
            }
        }
    }
    annotation
}

/// Extract parameter annotations from a `CXCursor_ParmDecl` cursor.
///
/// Two annotation sources are tried in priority order:
///
/// 1. **SAL annotations** — `CXCursor_AnnotateAttr` and `CXCursor_UnexposedAttr` children of
///    the cursor, produced by `__attribute__((annotate("_In_")))` portable stubs or by
///    `__declspec(SAL_*)` on Windows/MSVC targets.
///
/// 2. **MIDL block-comment annotations** — checked separately via
///    [`scan_method_param_annotations`] on the parent method/function's token stream,
///    because the ParmDecl extent does not include the leading `/* [in] */` comment.
///    Call sites should merge the result of this function with the MIDL scan:
///    if this returns `is_annotated() == false`, use the MIDL annotation instead.
pub fn extract_param_annotation(cursor: &Cursor, tu: &TranslationUnit) -> ParamAnnotation {
    let mut annotation = ParamAnnotation::default();

    for child in cursor.children() {
        match child.kind() {
            CXCursor_AnnotateAttr => {
                // __attribute__((annotate("_In_"))) and similar portable stubs.
                apply_sal_string(&child.name(), &mut annotation);
            }
            CXCursor_UnexposedAttr => {
                // __declspec(SAL_*) attributes produced by the real Windows SDK
                // sal.h when compiled with --target=x86_64-pc-windows-msvc.
                let tokens = tu.tokenize(tu.to_expansion_range(child.extent()));
                for (kind, spelling) in &tokens {
                    if *kind == CXToken_Identifier {
                        apply_sal_string(spelling, &mut annotation);
                    }
                }
            }
            _ => {}
        }
    }

    annotation
}

/// Scan the token stream of a method or function for MIDL block-comment parameter
/// annotations, returning one [`ParamAnnotation`] per parameter in declaration order.
///
/// MIDL-generated headers place a block comment such as `/* [in] */`, `/* [out] */`, or
/// `/* [retval][out] */` immediately before each parameter's type in the source text.
/// Because this comment falls outside the `ParmDecl` cursor's extent, it cannot be
/// detected by tokenising individual `ParmDecl` cursors.  Instead, this function
/// tokenises the entire method or function extent — which is already done in
/// `interface.rs` for propget/propput detection — and maps each comment to its
/// corresponding parameter by position in the token stream.
///
/// The algorithm:
/// 1. Skip tokens until the function/method name identifier is found.
/// 2. After the opening `(` of the parameter list, accumulate MIDL comment annotations.
/// 3. Each `,` (at depth 1) commits the accumulated annotation for the current parameter
///    and resets for the next.
/// 4. The closing `)` (back to depth 0) commits the last parameter's annotation.
///
/// Parameters without a preceding MIDL comment receive a default (all-false) annotation.
pub fn scan_method_param_annotations(
    tokens: &[(CXTokenKind, String)],
    method_name: &str,
) -> Vec<ParamAnnotation> {
    let mut result = Vec::new();
    let mut current = ParamAnnotation::default();
    let mut past_name = false;
    let mut paren_depth: i32 = 0;
    let mut in_params = false;

    for (kind, spelling) in tokens {
        if !past_name {
            if *kind == CXToken_Identifier && spelling == method_name {
                past_name = true;
            }
            continue;
        }

        match (*kind, spelling.as_str()) {
            (CXToken_Punctuation, "(") => {
                paren_depth += 1;
                if paren_depth == 1 {
                    in_params = true;
                    current = ParamAnnotation::default();
                }
            }
            (CXToken_Punctuation, ")") => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
                if paren_depth == 0 && in_params {
                    // Commit annotation for the last parameter.
                    // Guard: only push when we've already committed at least one param
                    // (via a `,`) OR when the current annotation is non-empty.  This
                    // prevents pushing a spurious default entry for zero-parameter
                    // functions and for single-parameter functions without any MIDL comment.
                    // For multi-parameter functions the `,` arm already pushed an entry
                    // for every param except the last, so `!result.is_empty()` is always
                    // true when there are ≥ 2 params regardless of comments.
                    if !result.is_empty() || current.is_annotated() {
                        result.push(current.clone());
                    }
                    break;
                }
            }
            (CXToken_Punctuation, ",") if in_params && paren_depth == 1 => {
                result.push(current.clone());
                current = ParamAnnotation::default();
            }
            (CXToken_Comment, s) if in_params && paren_depth == 1 => {
                apply_midl_param_comment(s, &mut current);
            }
            _ => {}
        }
    }

    result
}

/// Parse a MIDL block-comment string and apply the recognised annotations.
///
/// MIDL generates comments of the form `/* [in] */`, `/* [retval][out] */`, etc.
/// before each parameter.  This function checks for the well-known attribute names
/// using simple substring matching; unknown names are silently ignored.
pub fn apply_midl_param_comment(comment: &str, annotation: &mut ParamAnnotation) {
    if comment.contains("[in]") {
        annotation.in_param = true;
    }
    if comment.contains("[out]") {
        annotation.out_param = true;
    }
    if comment.contains("[retval]") {
        annotation.retval = true;
    }
    if comment.contains("[optional]") {
        annotation.optional = true;
    }
}

/// Map a SAL macro name to the corresponding [`ParamAnnotation`] flags.
///
/// Only the most common Win32 SAL macros are handled.  Unknown names are silently
/// ignored so that non-SAL `__attribute__((annotate(…)))` annotations on parameters
/// do not cause spurious errors.
fn apply_sal_string(sal: &str, annotation: &mut ParamAnnotation) {
    match sal {
        // --- Input-only ---
        "_In_" | "_In_z_" | "_In_nz_" | "_In_reads_" | "_In_reads_bytes_" | "_In_reads_z_"
        | "_In_reads_or_z_" | "_In_bytecount_" | "_In_count_" => {
            annotation.in_param = true;
        }

        // --- Input-only, optional ---
        "_In_opt_"
        | "_In_opt_z_"
        | "_In_reads_opt_"
        | "_In_reads_bytes_opt_"
        | "_In_reads_or_z_opt_" => {
            annotation.in_param = true;
            annotation.optional = true;
        }

        // --- Output-only ---
        "_Out_"
        | "_Out_z_"
        | "_Out_writes_"
        | "_Out_writes_bytes_"
        | "_Out_writes_z_"
        | "_Out_writes_bytes_all_"
        | "_Outptr_"
        | "_COM_Outptr_"
        | "_Outptr_result_z_"
        | "_Outptr_result_buffer_"
        | "_Outptr_result_bytebuffer_" => {
            annotation.out_param = true;
        }

        // --- Output-only, optional ---
        "_Out_opt_"
        | "_Out_writes_opt_"
        | "_Out_writes_bytes_opt_"
        | "_Outptr_opt_"
        | "_COM_Outptr_opt_"
        | "_Outptr_opt_result_z_"
        | "_Outptr_result_maybenull_"
        | "_Outptr_opt_result_maybenull_" => {
            annotation.out_param = true;
            annotation.optional = true;
        }

        // --- In + Out ---
        "_Inout_"
        | "_Inout_z_"
        | "_Inout_updates_"
        | "_Inout_updates_bytes_"
        | "_Inout_updates_z_" => {
            annotation.in_param = true;
            annotation.out_param = true;
        }

        // --- In + Out, optional ---
        "_Inout_opt_" | "_Inout_opt_z_" | "_Inout_updates_opt_" | "_Inout_updates_bytes_opt_" => {
            annotation.in_param = true;
            annotation.out_param = true;
            annotation.optional = true;
        }

        _ => {}
    }
}

/// Compute RDL attribute tokens for a parameter given its annotation and type.
///
/// Returns a `Vec<TokenStream>` containing zero or more of `#[r#in]`, `#[out]`,
/// `#[opt]`, and `#[retval]` that should be placed before the parameter name.
///
/// The emission rules mirror those of [`writer::write_params`] so that the
/// clang → RDL → winmd → RDL roundtrip is stable: attributes emitted here are
/// the same attributes that the writer would re-emit after encoding into winmd.
///
/// When `annotation` has no flags set (no SAL or MIDL annotation was detected),
/// no attributes are emitted (the reader will infer direction from the type).
pub fn param_attrs_for_annotation(
    annotation: &ParamAnnotation,
    ty: &metadata::Type,
) -> Vec<TokenStream> {
    if !annotation.is_annotated() {
        return vec![];
    }

    let in_param = annotation.in_param;
    let out_param = annotation.out_param;
    let optional = annotation.optional;
    let retval = annotation.retval;

    let is_mutable = matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));

    // effective_in: treat as In when explicitly in or when not explicitly out
    let effective_in = in_param || !out_param;

    // Emit #[r#in] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when In but the type is mutable (default would otherwise be Out)
    let mut attrs = vec![];
    if effective_in && (out_param || is_mutable) {
        attrs.push(quote! { #[r#in] });
    }

    // Emit #[out] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when Out but the type is not mutable (default would otherwise be In)
    if out_param && (effective_in || !is_mutable) {
        attrs.push(quote! { #[out] });
    }

    if optional {
        attrs.push(quote! { #[opt] });
    }

    if retval {
        attrs.push(quote! { #[retval] });
    }

    attrs
}
