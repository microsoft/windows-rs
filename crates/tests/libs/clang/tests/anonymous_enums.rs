use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn anonymous_enums() {
    helpers::ensure_libclang();
    roundtrip("anonymous-enums", &[""]);
    shared_constants();
    rejections();
}

#[test]
#[ignore = "requires Rust standard libraries for all three MSVC targets"]
fn anonymous_enum_architectures() {
    helpers::ensure_libclang();
    roundtrip(
        "anonymous-enums-cross-targets",
        &[
            "i686-pc-windows-msvc",
            "x86_64-pc-windows-msvc",
            "aarch64-pc-windows-msvc",
        ],
    );
}

fn roundtrip(name: &str, targets: &[&str]) {
    let snapshots = capture(&[selected_fixture("anonymous_enums", &[])], &TARGETS).unwrap();
    assert!(snapshots.iter().all(|snapshot| {
        snapshot
            .enums
            .iter()
            .flat_map(|enumeration| &enumeration.observations)
            .any(|observation| observation.anonymous)
    }));
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(
        (
            output.records,
            output.enums,
            output.aliases,
            output.constants
        ),
        (1, 1, 0, 9)
    );
    let rdl = &output.partitions["anonymous_enums"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("anonymous_enums.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join(format!("target\\test-clang-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    let rebuilt = root.join(format!("target\\test-clang-{name}-rebuilt.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&rebuilt)
        .write()
        .unwrap();
    let bindings = root.join(format!("target\\test-clang-{name}.rs"));
    windows_bindgen::builder()
        .input(&rebuilt)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(
        "\nconst _: () = {\n\
        assert!(Zero == 0 && Expression == 16 && Negative == -4);\n\
        assert!(High == u64::MAX && Low == i64::MIN && Small == u16::MAX && value__ == 1);\n\
        assert!(SharedName == 7 && core::mem::size_of::<SharedName>() == 4);\n\
        assert!(Width == if cfg!(target_arch = \"x86\") { 32 } else { 64 });\n\
        };\n\
        fn types(_: i32, _: u64, _: u16) {}\n\
        fn constants() {\n\
        types(Negative, High, Small);\n\
        #[cfg(target_arch = \"x86\")] let _: i32 = Width;\n\
        #[cfg(not(target_arch = \"x86\"))] let _: u64 = Width;\n\
        }\n",
    );
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join(format!("target\\test-clang-{name}.rmeta"));
    for target in targets {
        let mut command = std::process::Command::new("rustc");
        command.args([
            "--crate-name",
            "test_clang_anonymous_enums",
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
        ]);
        if !target.is_empty() {
            command.args(["--target", target]);
        }
        let result = command
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
    for path in [winmd, rewritten, rebuilt, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
}

fn shared_constants() {
    let mut inputs = [
        selected_fixture("anonymous_enum_a", &[]),
        selected_fixture("anonymous_enum_b", &[]),
    ];
    for input in &mut inputs {
        input.source = format!("#include \"{}\"", input.name);
    }
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!(output.constants, 3);
    assert_eq!(
        output.partitions["foundation"].rdl,
        "#[win32]\nmod Fixture {\n    const Common: i32 = 4;\n}\n"
    );
    assert_eq!(output.partitions["foundation"].headers.len(), 2);
    assert!(
        output.partitions["anonymous_enum_a"]
            .rdl
            .contains("const OnlyA: i32 = 10;")
    );
    assert!(
        output.partitions["anonymous_enum_b"]
            .rdl
            .contains("const OnlyB: i32 = 20;")
    );
    inputs.reverse();
    inputs[0].name = "renamed.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    for prefix in ["#define VALUE 5\n", "#define BACKING long\n"] {
        let mut inputs = [
            selected_fixture("anonymous_enum_a", &[]),
            selected_fixture("anonymous_enum_b", &[]),
        ];
        inputs[1].source.insert_str(0, prefix);
        let error = projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting native definitions for constant `Common`"),
            "{error}"
        );
    }
}

fn rejections() {
    let output = projection::generate_partial(
        &[selected_fixture("anonymous_enum_rejections", &[])],
        "Fixture",
    )
    .unwrap();
    assert_eq!(
        (
            output.output.records,
            output.output.enums,
            output.output.aliases
        ),
        (0, 1, 1)
    );
    let rdl = &output.output.partitions["anonymous_enum_rejections"].rdl;
    assert!(rdl.contains("const Survives: i32 = 5;"), "{rdl}");
    for name in [
        "Aligned",
        "Attributed",
        "Neighbor",
        "EnumPointer",
        "PointerUser",
    ] {
        assert!(!rdl.contains(name), "{rdl}");
    }
    assert!(rdl.contains("const PointerValue: i32 = 4;"), "{rdl}");
    assert!(
        rdl.contains("enum AliasA {\n        AmbiguousValue = 6,"),
        "{rdl}"
    );
    assert!(rdl.contains("type AliasB = AliasA;"), "{rdl}");
    assert_eq!(output.output.constants, 2);
    assert!(
        output
            .omissions
            .iter()
            .any(|message| message.contains("anonymous enum has no public type name"))
    );
}
