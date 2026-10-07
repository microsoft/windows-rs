#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

#[test]
fn source_families_and_scopes_survive_projection() {
    let snapshot = sdk::capture_sdk(
        "--target=x86_64-pc-windows-msvc",
        include_str!("../input/annotation_evidence.h"),
        &[
            "Packet",
            "Query",
            "Annotated",
            "Invoke",
            "Status",
            "Choice",
            "AnnotatedPacket",
            "InspectAliases",
        ],
    );
    let mut options = windows_clang2::ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
    let expected =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/annotation_evidence.rdl");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn conflicting_uninterpreted_ranges_do_not_agree() {
    let arguments = sdk::arguments("--target=x86_64-pc-windows-msvc");
    let snapshot = windows_clang2::capture(
        [
            windows_clang2::Input::new(
                "a.h",
                "extern \"C\" int Range(_In_range_(0,10) int value);",
            ),
            windows_clang2::Input::new(
                "b.h",
                "extern \"C\" int Range(_In_range_(20,30) int value);",
            ),
        ],
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &["Range"],
    )
    .unwrap();
    assert!(
        snapshot
            .resolve()
            .err()
            .unwrap()
            .to_string()
            .contains("conflicting annotations")
    );
}

#[test]
fn raw_contracts_bind_parameters_but_not_literals_or_members() {
    let source = r#"
        struct Object { int value; };
        extern "C" void Inspect(int value, Object* object,
            int* __attribute__((annotate(R"sal(_Vendor_(value,object->value,"value",value > value,object.value,ns::value,value ? value : value,R"(value)",0x10,1u,1e3))sal"))) data,
            /* [out, size_is(data[value]), helpstring("]")] */ int* nested, int x10, int u, int e3);
    "#;
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new("binding.h", source)],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
        &["Inspect"],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
    assert!(    rdl.contains(r#"_Vendor_($0,$1->value,\"value\",$0 > $0,$1.value,ns::value,$0 ? $0 : $0,R\"(value)\",0x10,1u,1e3)"#), "{rdl}");
    assert!(
        rdl.contains(r#"[out, size_is($2[$0]), helpstring(\"]\")]"#),
        "{rdl}"
    );
}

#[test]
fn field_and_return_contract_conflicts_are_checked_before_projection() {
    let args = sdk::arguments("--target=x86_64-pc-windows-msvc");
    for (source, root) in [
        (
            "struct Packet { _Field_range_(0, 10) int value; };",
            "Packet",
        ),
        ("extern \"C\" _Ret_range_(0, 10) int Query();", "Query"),
        (
            "extern \"C\" int Query(_In_range_(0, 10) int value);",
            "Query",
        ),
    ] {
        let conflicting = source.replace("0, 10", "20, 30");
        let snapshot = windows_clang2::capture(
            [
                windows_clang2::Input::new("a.h", source),
                windows_clang2::Input::new("b.h", conflicting),
            ],
            &args.iter().map(String::as_str).collect::<Vec<_>>(),
            &[root],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err(), "{source}");
    }
}

#[test]
fn sdk_capture_without_the_capture_header_fails_explicitly() {
    let include = sdk::include().join("shared");
    let error = windows_clang2::capture(
        [windows_clang2::Input::new(
            "missing.h",
            "#include <sal.h>\nextern \"C\" int Query(_In_range_(0, 10) int value);",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-isystem",
            include.to_str().unwrap(),
        ],
        &["Query"],
    )
    .err()
    .unwrap();
    assert!(error.to_string().contains("capture header"), "{error}");
}

#[test]
fn annotation_clause_order_is_part_of_agreement() {
    let arguments = sdk::arguments("--target=x86_64-pc-windows-msvc");
    let snapshot = windows_clang2::capture(
        [
            windows_clang2::Input::new(
                "a.h",
                "extern \"C\" void Use(_Pre_ _Notnull_ _Post_ _Maybenull_ int* value);",
            ),
            windows_clang2::Input::new(
                "b.h",
                "extern \"C\" void Use(_Post_ _Notnull_ _Pre_ _Maybenull_ int* renamed);",
            ),
        ],
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &["Use"],
    )
    .unwrap();
    let error = snapshot.resolve().err().unwrap();
    assert!(
        error.to_string().contains("conflicting annotations"),
        "{error}"
    );
}

#[test]
fn sdk_wrappers_preserve_uninterpreted_output_contracts() {
    for (root, annotation) in [
        ("PointerOut", "_Outptr_"),
        ("NullableOut", "_Outptr_result_maybenull_"),
    ] {
        let snapshot = sdk::capture_sdk(
            "--target=x86_64-pc-windows-msvc",
            include_str!("../input/output_annotations.h"),
            &[root],
        );
        let mut options = windows_clang2::ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
        assert!(
            rdl.contains(&format!("#[annotation(\"sal\", \"{annotation}\")]")),
            "{root}: source annotation was lost: {rdl}"
        );
    }
}
