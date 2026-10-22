use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn mixed_pointers() {
    helpers::ensure_libclang();
    let output =
        projection::generate(&[selected_fixture("mixed_pointers", &[])], "Fixture").unwrap();
    let expected = std::fs::read_to_string(input_dir().join("mixed_pointers.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    let rdl = &output.partitions["mixed_pointers"].rdl;
    assert_eq!(*rdl, expected);

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-mixed-pointers.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang-mixed-pointers.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), expected);

    let bindings = root.join("target\\test-clang-mixed-pointers.rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(
        "\nfn check(value: Pointers, input: Inputs, callback: Callback) {\n\
        let _: *mut *const i32 = value.mutable_pointer;\n\
        let _: *const *mut i32 = value.constant_pointer;\n\
        let _: *mut *mut *const i32 = value.deep_mutable;\n\
        let _: *const *const *mut i32 = value.deep_constant;\n\
        let _: *mut *const *const i32 = value.alternating;\n\
        let _: [*mut *const i32; 3] = value.array;\n\
        let _: *const [*mut i32; 3] = value.array_pointer;\n\
        let _: *const *mut core::ffi::c_void = value.opaque;\n\
        let _: *mut Input = value.named_input;\n\
        let _: *const Output = value.named_output;\n\
        let _: *mut *const i32 = input;\n\
        let _: Option<unsafe extern \"C\" fn(*const *mut i32) -> *mut *const i32> = callback;\n\
        }\n",
    );
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join("target\\test-clang-mixed-pointers.rmeta");
    let result = std::process::Command::new("rustc")
        .args([
            "--crate-name",
            "test_clang_mixed_pointers",
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
        ])
        .arg(&bindings)
        .arg("-o")
        .arg(&compiled)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for path in [winmd, rewritten, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
}
