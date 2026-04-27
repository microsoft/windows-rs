use super::*;

/// Direction and optional flags derived from a SAL annotation on a parameter.
///
/// SAL annotations such as `_In_`, `_Out_`, `_Inout_`, `_In_opt_`, etc. are extracted
/// from `CXCursor_AnnotateAttr` or `CXCursor_UnexposedAttr` children of a
/// `CXCursor_ParmDecl` cursor and mapped to this struct.
///
/// When no SAL annotation is detected, all fields remain `false` and direction
/// defaults are inferred from the parameter's type (mutable pointer/reference → Out,
/// everything else → In), matching the reader's `parse_param_attributes` logic.
#[derive(Debug, Default, Clone)]
pub struct ParamAnnotation {
    /// The parameter is annotated as an input (`_In_`, `_In_opt_`, `_Inout_`, …).
    pub in_param: bool,
    /// The parameter is annotated as an output (`_Out_`, `_Out_opt_`, `_Inout_`, …).
    pub out_param: bool,
    /// The parameter may be `NULL` / absent (`_In_opt_`, `_Out_opt_`, `_Inout_opt_`, …).
    pub optional: bool,
}

impl ParamAnnotation {
    /// Returns `true` if any SAL annotation was detected.
    pub fn is_annotated(&self) -> bool {
        self.in_param || self.out_param || self.optional
    }
}

/// Extract SAL annotations from the children of a `CXCursor_ParmDecl` cursor.
///
/// Two child-cursor kinds are inspected:
///
/// - `CXCursor_AnnotateAttr`: produced by `__attribute__((annotate("…")))`.
///   The spelling is retrieved via `clang_getCursorSpelling` and matched directly
///   against the table of known SAL macro names.
///
/// - `CXCursor_UnexposedAttr`: produced on Windows/MSVC targets by `__declspec(…)`.
///   The attribute extent is tokenised and every identifier token is tested against
///   the same table.
///
/// Both approaches are combined so that the function works regardless of how the
/// SAL macros are defined (real Windows SDK headers vs. portable stub definitions).
pub fn extract_sal(cursor: &Cursor, tu: &TranslationUnit) -> ParamAnnotation {
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
