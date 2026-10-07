#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

#[test]
fn sdk_wrappers_do_not_erase_unsupported_output_contracts() {
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
        let error = snapshot.resolve().unwrap().project(&options).unwrap_err();
        assert!(
            error.to_string().contains(annotation),
            "{root}: unsupported source annotation was lost: {error}"
        );
    }
}
