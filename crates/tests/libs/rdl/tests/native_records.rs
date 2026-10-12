use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_records_{name}.winmd"))
}

fn vocabulary(name: &str) -> PathBuf {
    let path = output(&format!("vocabulary_{name}"));
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    path
}

fn check(path: &Path) {
    let index = Index::read(path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let outer = index.expect("Test", "Outer");
    let fields = outer.fields().collect::<Vec<_>>();
    assert!(fields[0].is_native_anonymous().unwrap());
    assert!(!fields[1].is_native_anonymous().unwrap());
    assert!(!fields[2].is_native_anonymous().unwrap());
    assert_eq!(fields[0].name(), "Anonymous");
    assert_eq!(index.nested(outer).count(), 2);
    for name in ["Inner", "Alias"] {
        assert_eq!(
            index.expect("Test.Outer", name).native_owner().unwrap(),
            Some(TypeName::named("Test", "Outer"))
        );
    }
    assert_eq!(outer.native_owner().unwrap(), None);
    assert_eq!(fields[2].ty(), Type::value_named("Test.Outer", "Inner"));
}

#[test]
fn source_ownership_survives_roundtrip_and_merge() {
    let reference = vocabulary("roundtrip");
    let source = include_str!("../native_records.rdl");
    let original = output("original");
    windows_rdl::reader()
        .reference(&reference)
        .input_text(source)
        .output(&original)
        .write()
        .unwrap();
    check(&original);
    let text = original.with_extension("rdl");
    windows_rdl::writer()
        .input(&original)
        .output(&text)
        .write()
        .unwrap();
    let roundtrip = output("roundtrip");
    windows_rdl::reader()
        .reference(&reference)
        .input(&text)
        .output(&roundtrip)
        .write()
        .unwrap();
    check(&roundtrip);
    let merged = output("merged");
    merge().input(&original).output(&merged).merge().unwrap();
    check(&merged);
}

#[test]
fn invalid_descriptors_preserve_existing_output() {
    let reference = vocabulary("invalid");
    let path = output("invalid");
    let marker = "Windows::Win32::Metadata::NativeAnonymousField";
    let owner = "Windows::Win32::Metadata::NativeTypeOwner";
    for (body, diagnostic) in [
        (
            format!("#[{marker}] struct Outer {{ value: i32, }}"),
            "invalid owner",
        ),
        (
            format!("struct Outer {{ #[{marker}] value: i32, }}"),
            "direct nested record",
        ),
        (
            format!("struct Inner {{ value: i32, }} struct Outer {{ #[{marker}] value: Inner, }}"),
            "owned by its enclosing record",
        ),
        (
            format!("struct Outer {{ #[{marker}] #[{marker}] value: struct {{ value: i32, }}, }}"),
            "duplicate",
        ),
        (
            format!("struct Outer {{ #[{owner}(Outer)] value: i32, }}"),
            "invalid owner",
        ),
        (
            format!("#[{owner}(Outer)] struct Outer {{ value: i32, }}"),
            "cyclic",
        ),
        (
            format!(
                "#[{owner}(B)] struct A {{ value: i32, }} #[{owner}(A)] struct B {{ value: i32, }}"
            ),
            "cyclic",
        ),
        (
            format!(
                "struct Outer {{ value: i32, }} type Alias = Outer; #[{owner}(Alias)] struct Inner {{ value: i32, }}"
            ),
            "not a typedef",
        ),
        (
            format!(
                "struct Outer {{ value: i32, }} #[{owner}(Outer)] #[{owner}(Outer)] struct Inner {{ value: i32, }}"
            ),
            "duplicate",
        ),
    ] {
        std::fs::write(&path, b"existing publication").unwrap();
        let source = format!("#[win32] mod Test {{ {body} }}");
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&source)
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{body}: {error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"existing publication");
    }
}

#[test]
fn architecture_merge_keeps_distinct_ownership_and_unnamed_members() {
    let reference = vocabulary("architectures");
    let x86 = output("x86");
    let x64 = output("x64");
    let source = "#[win32] mod Test {
            struct Left { value: i32, }
            struct Right { value: i32, }
            #[Windows::Win32::Metadata::NativeTypeOwner(Left)]
            struct Inner { value: i32, }
            struct Outer {
                #[Windows::Win32::Metadata::NativeAnonymousField]
                Anonymous: union { value: i32, other: f32, },
            }
        }";
    for (source, path) in [
        (source.to_string(), &x86),
        (
            source
                .replace("NativeTypeOwner(Left)", "NativeTypeOwner(Right)")
                .replace("#[Windows::Win32::Metadata::NativeAnonymousField]", ""),
            &x64,
        ),
    ] {
        windows_rdl::reader()
            .reference(&reference)
            .input_text(&source)
            .output(path)
            .write()
            .unwrap();
    }
    let merged = output("architectures");
    merge()
        .arch_input(&x86, 1)
        .arch_input(&x64, 2)
        .output(&merged)
        .merge()
        .unwrap();
    let index = Index::read(&merged).unwrap();
    index.validate_native_contracts(None).unwrap();
    assert_eq!(index.get("Test", "Inner").count(), 2);
    assert_eq!(index.get("Test", "Outer").count(), 2);
    for (arch, owner, anonymous) in [(1, "Left", true), (2, "Right", false)] {
        let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], arch);
        assert_eq!(
            index.expect("Test", "Inner").native_owner().unwrap(),
            Some(TypeName::named("Test", owner))
        );
        assert_eq!(
            index
                .expect("Test", "Outer")
                .fields()
                .next()
                .unwrap()
                .is_native_anonymous()
                .unwrap(),
            anonymous
        );
    }
}

#[test]
fn ownership_cycles_are_checked_within_each_architecture() {
    let reference = vocabulary("parallel");
    let path = output("parallel");
    let source = "#[win32] mod Test {
            #[arch(X86)] #[Windows::Win32::Metadata::NativeTypeOwner(B)]
            struct A { value: i32, }
            #[arch(X64)] struct A { value: i32, }
            #[arch(X86)] struct B { value: i32, }
            #[arch(X64)] #[Windows::Win32::Metadata::NativeTypeOwner(A)]
            struct B { value: i32, }
        }";
    windows_rdl::reader()
        .reference(&reference)
        .input_text(source)
        .output(&path)
        .write()
        .unwrap();
    Index::read(path)
        .unwrap()
        .validate_native_contracts(None)
        .unwrap();
}
