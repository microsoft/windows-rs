use super::*;

fn snapshot(source: &str) -> BTreeMap<String, (Vec<ParameterAnnotations>, Vec<Parameter>)> {
    helpers::ensure_libclang();
    let _library = Library::new().unwrap();
    let index = Index::new().unwrap();
    let input = Input::new("annotations.h", source);
    let tu = TranslationUnit::parse(&index, &input, &["-x", "c++"]).unwrap();
    let root = unsafe { clang_getTranslationUnitCursor(tu.0) };
    let macros = macro_definitions(&tu, root);
    cursor_children(root)
        .into_iter()
        .filter(|cursor| unsafe { clang_getCursorKind(*cursor) } == CXCursor_FunctionDecl)
        .map(|cursor| {
            let name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
            let cursors: Vec<_> = cursor_children(cursor)
                .into_iter()
                .filter(|cursor| unsafe { clang_getCursorKind(*cursor) } == CXCursor_ParmDecl)
                .collect();
            let captured = ParameterAnnotations::capture(cursor, &cursors, &macros);
            let before = captured.clone();
            let projected = project_params(&cursors, &captured, &macros, false).unwrap();
            assert_eq!(captured, before);
            (name, (captured, projected))
        })
        .collect()
}

fn parameter<'a>(
    snapshot: &'a BTreeMap<String, (Vec<ParameterAnnotations>, Vec<Parameter>)>,
    name: &str,
) -> (&'a ParameterAnnotations, &'a Parameter) {
    let (captured, projected) = &snapshot[name];
    (captured.last().unwrap(), projected.last().unwrap())
}

#[test]
fn projection_summary_does_not_replace_individual_contracts() {
    let snapshot = snapshot(
        "void First(int n, int m,
             __attribute__((annotate(\"_In_reads_(n)\"), annotate(\"_Out_writes_(m)\"))) int *value);
         void Second(int n, int m,
             __attribute__((annotate(\"_In_reads_(n)\"), annotate(\"_Out_writes_(n)\"))) int *value);
         void Third(int n, int m,
             __attribute__((annotate(\"_Out_writes_to_(n,m)\"))) int *value);",
    );
    let (first, first_projection) = parameter(&snapshot, "First");
    let (second, second_projection) = parameter(&snapshot, "Second");
    let (third, third_projection) = parameter(&snapshot, "Third");
    assert_eq!(first_projection.annotation, second_projection.annotation);
    assert_ne!(first, second);
    assert_eq!(first.attributes.len(), 2);
    assert_eq!(first.attributes[1], "_Out_writes_(m)");
    assert_eq!(second.attributes[1], "_Out_writes_(n)");
    assert_eq!(third.attributes[0], "_Out_writes_to_(n,m)");
    for (captured, projected) in [
        (first, first_projection),
        (second, second_projection),
        (third, third_projection),
    ] {
        assert_eq!(captured.project_attributes(false), projected.annotation);
    }
}

#[test]
fn byte_count_normalization_changes_only_the_projection() {
    let snapshot =
        snapshot("void Use(__attribute__((annotate(\"_Out_writes_bytes_(16)\"))) int *value);");
    let (captured, projected) = parameter(&snapshot, "Use");
    let before = captured.clone();
    assert_eq!(
        captured.project_attributes(false).size,
        Some(SalSize {
            bytes: true,
            value: SalSizeValue::Constant(16)
        })
    );
    assert_eq!(
        projected.annotation.size,
        Some(SalSize {
            bytes: false,
            value: SalSizeValue::Constant(4)
        })
    );
    assert_eq!(*captured, before);
    assert_eq!(captured.attributes[0], "_Out_writes_bytes_(16)");
}

#[test]
fn source_markers_are_applied_after_attribute_direction() {
    let snapshot = snapshot(
        "#define OUT_ATTRIBUTE __attribute__((annotate(\"_Out_\")))
         void Use(OUT_ATTRIBUTE /* [iid_is] */ void **value);",
    );
    let (captured, projected) = parameter(&snapshot, "Use");
    assert_eq!(captured.attributes[0], "_Out_");
    assert_eq!(
        captured.source,
        [SourceAnnotation::Comment("/* [iid_is] */".to_string())]
    );
    let mut summary = captured.project_attributes(true);
    assert!(summary.output);
    assert!(!summary.com_out_ptr);
    captured.project_source(&mut summary, true);
    assert!(summary.com_out_ptr);
    assert_eq!(summary, projected.annotation);
}

#[test]
fn source_comment_order_is_retained() {
    let snapshot = snapshot(
        "#define OUT
         void First(/* [iid_is] */ OUT void **value);
         void Second(OUT /* [iid_is] */ void **value);",
    );
    let (first, first_projection) = parameter(&snapshot, "First");
    let (second, second_projection) = parameter(&snapshot, "Second");
    assert_ne!(first.source, second.source);
    assert!(!first_projection.annotation.com_out_ptr);
    assert!(second_projection.annotation.com_out_ptr);
    for (captured, projected) in [(first, first_projection), (second, second_projection)] {
        let mut summary = captured.project_attributes(true);
        captured.project_source(&mut summary, true);
        assert_eq!(summary, projected.annotation);
    }
}

#[test]
fn unknown_attributes_are_not_erased_by_projection() {
    let snapshot =
        snapshot("void Use(__attribute__((annotate(\"custom_contract(a,b)\"))) int *value);");
    let (captured, projected) = parameter(&snapshot, "Use");
    assert_eq!(projected.annotation, ParamAnnotation::default());
    assert_eq!(captured.attributes.len(), 1);
    assert_eq!(captured.attributes[0], "custom_contract(a,b)");
}
