use windows_metadata::*;

/// Compiles inline RDL into `dir/name.winmd` and returns its path.
fn winmd(dir: &std::path::Path, name: &str, rdl: &str) -> String {
    let rdl_path = dir.join(format!("{name}.rdl"));
    std::fs::write(&rdl_path, rdl).unwrap();
    let out = dir.join(format!("{name}.winmd"));
    windows_rdl::reader()
        .input(rdl_path.to_string_lossy().as_ref())
        .output(out.to_string_lossy().as_ref())
        .write()
        .unwrap();
    out.to_string_lossy().into_owned()
}

/// `Outer` must decompose into a top-level type with one anonymous nested struct,
/// which in turn contains one anonymous nested union — expressed as real
/// `NestedClass` rows (nested types living in the empty namespace, `NestedPublic`).
fn assert_nested(index: &reader::Index) {
    let outer = index
        .types()
        .find(|t| t.name() == "Outer")
        .expect("Outer type present");
    assert!(
        !outer.flags().is_nested(),
        "top-level Outer must not be nested"
    );

    let children: Vec<_> = index.nested(outer).collect();
    assert_eq!(
        children.len(),
        1,
        "Outer should have exactly one nested type"
    );
    let child = children[0];
    assert!(
        child.flags().is_nested(),
        "nested struct must be NestedPublic"
    );
    assert!(
        child.namespace().is_empty(),
        "nested type must live in the empty namespace"
    );
    assert!(
        !child.flags().contains(TypeAttributes::ExplicitLayout),
        "the inner anonymous aggregate should be a struct (sequential layout)"
    );

    let grandchildren: Vec<_> = index.nested(child).collect();
    assert_eq!(
        grandchildren.len(),
        1,
        "the nested struct should itself contain one nested union"
    );
    assert!(
        grandchildren[0]
            .flags()
            .contains(TypeAttributes::ExplicitLayout),
        "the deepest anonymous aggregate should be a union (explicit layout)"
    );
}

/// Inline anonymous nested struct/union syntax must survive the whole pipeline:
/// the reader emits real `NestedClass` rows, `merge` preserves them, and the
/// writer decompiles them back to inline syntax that the reader re-encodes
/// identically.
#[test]
fn nested_types_survive_rdl_merge_and_writer() {
    let dir = std::env::temp_dir().join("win_nested_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();

    let src = "#[win32] mod Test { \
        struct Outer { \
            header: u32, \
            Anonymous: struct { x: i32, Anonymous: union { a: i32, b: f32 } }, \
            tail: u16, \
        } \
    }";
    let winmd_path = winmd(&dir, "nested", src);

    // 1. The reader produced real NestedClass rows.
    let index = reader::Index::read(&winmd_path).unwrap();
    assert_nested(&index);

    // 2. merge() preserves the nested structure.
    let merged = dir.join("merged.winmd");
    merge()
        .input(&winmd_path)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let merged_index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    assert_nested(&merged_index);

    // 3. writer -> RDL emits inline nested syntax (not hoisted flat siblings).
    let rdl_dir = dir.join("rdl");
    std::fs::create_dir_all(&rdl_dir).unwrap();
    windows_rdl::writer()
        .input(&winmd_path)
        .output(rdl_dir.to_string_lossy().as_ref())
        .split(true)
        .write()
        .unwrap();
    let rdl_text = std::fs::read_to_string(rdl_dir.join("Test.rdl")).unwrap();
    assert!(
        rdl_text.contains("Anonymous: struct {"),
        "inline nested struct syntax missing:\n{rdl_text}"
    );
    assert!(
        rdl_text.contains("Anonymous: union {"),
        "inline nested union syntax missing:\n{rdl_text}"
    );

    // 4. Reading that RDL back reproduces the nested structure.
    let roundtrip = dir.join("roundtrip.winmd");
    windows_rdl::reader()
        .input(rdl_dir.to_string_lossy().as_ref())
        .output(roundtrip.to_string_lossy().as_ref())
        .write()
        .unwrap();
    let roundtrip_index = reader::Index::read(roundtrip.to_string_lossy().as_ref()).unwrap();
    assert_nested(&roundtrip_index);
}

/// The architecture of a nested type is always that of its enclosing type, so the
/// RDL omits the redundant `#[arch]` on inline nested records — but the winmd must
/// still carry it (bindgen hoists nested types to arch-gated flat helpers). The
/// reader therefore inherits the parent's architecture onto a nested type that has
/// no explicit `#[arch]`, and the writer omits it again on the way out.
#[test]
fn nested_type_inherits_parent_arch() {
    let dir = std::env::temp_dir().join("win_nested_arch");
    std::fs::create_dir_all(&dir).unwrap();

    // `Foo` is x64-only; its inline nested struct carries no `#[arch]` of its own.
    let src = "#[win32] mod Test { \
        #[arch(X64)] struct Foo { \
            flags: u32, \
            Anonymous: struct { lo: u32, hi: u32 }, \
        } \
    }";
    let winmd_path = winmd(&dir, "arch", src);
    let index = reader::Index::read(&winmd_path).unwrap();

    let arch_bits = |t: reader::TypeDef| -> Option<i32> {
        let attr = t.find_attribute("SupportedArchitectureAttribute")?;
        match attr.value().first() {
            Some((_, Value::I32(v))) => Some(*v),
            Some((_, Value::EnumValue(_, inner))) => match inner.as_ref() {
                Value::I32(v) => Some(*v),
                _ => None,
            },
            _ => None,
        }
    };

    let foo = index
        .types()
        .find(|t| t.name() == "Foo")
        .expect("Foo present");
    assert_eq!(
        arch_bits(foo),
        Some(2),
        "top-level Foo must be tagged x64 (2)"
    );

    let child = index.nested(foo).next().expect("Foo has a nested type");
    assert_eq!(
        arch_bits(child),
        Some(2),
        "the nested type must inherit its parent's x64 architecture in the winmd"
    );

    // The writer omits the redundant `#[arch]` on the inline nested record but keeps
    // it on the top-level type.
    let rdl_dir = dir.join("rdl");
    std::fs::create_dir_all(&rdl_dir).unwrap();
    windows_rdl::writer()
        .input(&winmd_path)
        .output(rdl_dir.to_string_lossy().as_ref())
        .split(true)
        .write()
        .unwrap();
    let rdl_text = std::fs::read_to_string(rdl_dir.join("Test.rdl")).unwrap();
    assert_eq!(
        rdl_text.matches("#[arch(").count(),
        1,
        "exactly one #[arch] (on the top-level type, not the nested one):\n{rdl_text}"
    );
}
