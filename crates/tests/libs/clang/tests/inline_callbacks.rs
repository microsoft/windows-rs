use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn inline_callbacks() {
    helpers::ensure_libclang();
    let input = selected_fixture("inline_callbacks", &[]);
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (2, 2));
    let expected_path = input_dir().join("inline_callbacks.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected_path, &output.partitions["inline_callbacks"].rdl).unwrap();
    }
    let expected = std::fs::read_to_string(expected_path)
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["inline_callbacks"].rdl, expected);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-inline-callbacks.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang-inline-callbacks.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), expected);

    let bindings = root.join("target\\test-clang-inline-callbacks.rs");
    let compiled = bindings.with_extension("rmeta");
    for sys in [false, true] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Fixture.InlineCallbacks")
            .flat();
        if sys {
            builder.sys();
        }
        builder.output(&bindings).write();
        let mut rust = std::fs::read_to_string(&bindings).unwrap();
        rust.push_str(
            "\nfn check(value: InlineCallbacks) {\n\
             let _: Named = value.named;\n\
             let _: Option<unsafe extern \"C\" fn(*const Payload, Count) -> Count> = value.count;\n\
             let _: Option<unsafe extern \"C\" fn()> = value.empty;\n\
             let _: *mut Option<unsafe extern \"C\" fn(i16) -> i32> = value.pointer;\n\
             let _: [Option<unsafe extern \"C\" fn(i8)>; 2] = value.array;\n\
             let _: Option<unsafe extern \"C\" fn(*const Count, *mut [i32; 3])> = value.arrays;\n\
             let _: Option<unsafe extern \"C\" fn(Option<unsafe extern \"C\" fn(Count) -> Count>)> = value.function;\n\
             let _: Option<unsafe extern \"C\" fn(*const *mut i32) -> *mut *const i32> = value.mixed;\n\
             let _: Option<unsafe extern \"C\" fn(*mut InlineCallbacks) -> *mut InlineCallbacks> = value.recursive;\n\
             }\n\
             const _: () = {\n\
             let pointer = core::mem::size_of::<usize>();\n\
             assert!(core::mem::size_of::<InlineCallbacks>() == 12 * pointer);\n\
             assert!(core::mem::align_of::<InlineCallbacks>() == pointer);\n\
             assert!(core::mem::offset_of!(InlineCallbacks, factory) == 7 * pointer);\n\
             assert!(core::mem::offset_of!(InlineCallbacks, recursive) == 11 * pointer);\n\
             };\n"
        );
        std::fs::write(&bindings, rust).unwrap();
        let result = std::process::Command::new("rustc")
            .args([
                "--crate-name",
                "test_clang_inline_callbacks",
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
    }
    for path in [winmd, rewritten, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
    let snapshots = capture(&[input], &[Target::Arm64, Target::X64, Target::X86]).unwrap();
    assert_eq!(
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap(),
        output
    );
}
