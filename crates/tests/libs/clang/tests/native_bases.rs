use std::path::Path;
use windows_clang::*;
use windows_metadata::reader::HasAttributes;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_base_metadata() {
    generate("native-base-metadata");
}

#[test]
fn native_base_bindgen_rejections() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\native-base-bindgen-rejections");
    std::fs::create_dir_all(&scratch).unwrap();
    let source = std::fs::read_to_string(test_clang::input_dir().join("native_bases.rdl")).unwrap();
    for (name, source, filter, diagnostic) in [
        (
            "mixed",
            source.replace("base1: IUnknown,", "base1: IUnknown,\nvalue: u32,"),
            "Combined",
            "requires only interface base storage",
        ),
        (
            "by-value",
            source.replace(
                "struct Combined {",
                "#[library(\"fixture\")] extern fn ByValue(value: Combined);\nstruct Combined {",
            ),
            "ByValue",
            "native objects cannot be passed by value",
        ),
    ] {
        let winmd = scratch.join(format!("{name}.winmd"));
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        let error = std::panic::catch_unwind(|| {
            windows_bindgen::builder()
                .input(&winmd)
                .filter(&format!("Windows.Win32.{filter}"))
                .output(scratch.join(format!("{name}.rs")))
                .write();
        })
        .unwrap_err();
        let message = error
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| error.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(message.contains(diagnostic), "{name}: {message}");
    }
}

#[test]
fn native_base_layout_rejections() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("native_bases", &["Combined"]);
    let mut snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in 0..snapshots.len() {
        let record = snapshots[snapshot]
            .records
            .iter()
            .find(|record| record.name == "Combined")
            .unwrap();
        let observation = record.observations[0].0;
        for case in 0..3 {
            let definition = snapshots[snapshot].observations[observation]
                .definition
                .as_mut()
                .unwrap();
            let offset = definition.bases[1].offset_bits;
            let layout = definition.layout;
            match case {
                0 => definition.bases[1].offset_bits = 0,
                1 => definition.layout.size += 1,
                _ => definition.layout.alignment = 1,
            }
            let error = projection::project(&merge(&snapshots).unwrap(), "Windows.Win32")
                .unwrap_err()
                .to_string();
            assert!(error.contains("layout"), "{error}");
            let definition = snapshots[snapshot].observations[observation]
                .definition
                .as_mut()
                .unwrap();
            definition.bases[1].offset_bits = offset;
            definition.layout = layout;
        }
    }
}

#[test]
fn native_base_rejections() {
    helpers::ensure_libclang();
    for (case, (name, diagnostic)) in [
        (
            "WithData",
            "requires only public nonvirtual interface bases",
        ),
        (
            "WithMethod",
            "requires only public nonvirtual interface bases",
        ),
        ("PrivateBase", "only public non-virtual bases"),
        ("VirtualBase", "only public non-virtual bases"),
        ("ContainsObject", "abstract"),
        ("ByValue", "cannot be used as value storage"),
        ("Further", "inheritance from native object"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut input = test_clang::selected_fixture("native_bases", &[name]);
        input
            .arguments
            .push(format!("-DNEGATIVE_BASES={}", case + 1));
        let error = projection::generate(&[input], "Windows.Win32")
            .unwrap_err()
            .to_string();
        assert!(error.contains(diagnostic), "{name}: {error}");
    }
}

fn generate(name: &str) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("native_bases", &["Combined", "PackedCombined"]);
    let mut snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "Combined")
            .unwrap();
        let definition = snapshot.observations[record.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        let pointer = if snapshot.target == Target::X86 { 4 } else { 8 };
        assert_eq!(
            definition.layout,
            Layout {
                size: pointer * 2,
                alignment: pointer
            }
        );
        assert_eq!(
            definition
                .bases
                .iter()
                .map(|base| base.offset_bits)
                .collect::<Vec<_>>(),
            [0, pointer * 8]
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Windows.Win32").unwrap();
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Windows.Win32").unwrap()
    );
    let text = &output.partitions["native_bases"].rdl;
    let golden = test_clang::input_dir().join("native_bases.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        *text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target").join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let rewritten = scratch.join("roundtrip.rdl");
    let mut source = text.clone();
    for _ in 0..2 {
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        let object = index.expect("Windows.Win32", "Combined");
        assert_eq!(object.fields().count(), 2);
        for field in object.fields() {
            assert!(field.has_attribute("NativeBaseAttribute"));
        }
        windows_rdl::writer()
            .input(&winmd)
            .filter("Windows.Win32")
            .filter("!Windows.Win32.Metadata")
            .output(&rewritten)
            .write()
            .unwrap();
        source = std::fs::read_to_string(&rewritten).unwrap();
        assert_eq!(&source, text);
    }
    (root, scratch, winmd)
}
