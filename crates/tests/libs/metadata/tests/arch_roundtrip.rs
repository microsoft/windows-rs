use windows_metadata::*;

fn winmd(dir: &std::path::Path, name: &str, rdl: &str) -> String {
    let rdl_path = dir.join(format!("{name}.rdl"));
    std::fs::write(&rdl_path, rdl).unwrap();
    let out = dir.join(format!("{name}.winmd"));
    windows_rdl::reader()
        .input(&rdl_path)
        .output(&out)
        .write()
        .unwrap();
    out.to_string_lossy().into_owned()
}

fn type_arch(t: &reader::TypeDef) -> Option<i32> {
    arch_value(t.attributes())
}

fn field_arch(f: &reader::Field) -> Option<i32> {
    arch_value(f.attributes())
}

fn arch_value<'a>(attrs: impl Iterator<Item = reader::Attribute<'a>>) -> Option<i32> {
    attrs.into_iter().find_map(|a| {
        (a.ctor().parent().name() == "SupportedArchitectureAttribute").then(|| {
            match a.value().first() {
                Some((_, Value::I32(v))) => *v,
                _ => 0,
            }
        })
    })
}

// The writer and reader must agree on `#[arch(...)]` for types and constants.
#[test]
fn arch_survives_winmd_rdl_roundtrip() {
    let dir = std::env::temp_dir().join("win_arch_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { struct CTX { a: i32, b: i32 } type AL = i32; extern \"C\" fn CB(a: i64) -> i32; const X64_ONLY: i32 = 1; const CTX_ALL: i32 = 100; }",
    );
    let arm = winmd(
        &dir,
        "arm",
        "#[win32] mod Test { struct CTX { x: i32 } type AL = i64; extern \"C\" fn CB(a: i32) -> i32; const ARM_ONLY: i32 = 2; const CTX_ALL: i32 = 200; }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();

    let rdl_dir = dir.join("rdl");
    std::fs::create_dir_all(&rdl_dir).unwrap();
    windows_rdl::writer()
        .input(&merged)
        .output(&rdl_dir)
        .split()
        .write()
        .unwrap();

    // Raw attributes on constants cannot be parsed by the RDL reader.
    let rdl = std::fs::read_to_string(rdl_dir.join("Test.rdl")).unwrap();
    assert!(
        rdl.contains("#[arch(X64)]"),
        "missing #[arch(X64)] sugar:\n{rdl}"
    );
    assert!(
        rdl.contains("#[arch(Arm64)]"),
        "missing #[arch(Arm64)] sugar:\n{rdl}"
    );
    assert!(
        !rdl.contains("SupportedArchitecture"),
        "raw SupportedArchitecture attribute leaked into RDL:\n{rdl}"
    );

    let out = dir.join("roundtrip.winmd");
    windows_rdl::reader()
        .input(&rdl_dir)
        .output(&out)
        .write()
        .unwrap();
    let index = reader::Index::read(out.to_string_lossy().as_ref()).unwrap();

    let mut ctx: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CTX")
        .filter_map(|t| type_arch(&t))
        .collect();
    ctx.sort();
    assert_eq!(ctx, vec![2, 4], "CTX struct arch tags lost on round-trip");

    // Callbacks are reference TypeDefs and use a separate arch writer/reader path.
    let mut cb: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CB")
        .filter_map(|t| type_arch(&t))
        .collect();
    cb.sort();
    assert_eq!(cb, vec![2, 4], "CB callback arch tags lost on round-trip");

    // NativeTypedef aliases decompile to bare `type` forms that must retain their arch tags.
    let mut al: Vec<_> = index
        .types()
        .filter(|t| t.name() == "AL")
        .filter_map(|t| type_arch(&t))
        .collect();
    al.sort();
    assert_eq!(
        al,
        vec![2, 4],
        "AL typedef alias arch tags lost on round-trip"
    );

    let apis = index.types().find(|t| t.name() == "Apis").unwrap();
    let consts: Vec<_> = apis.fields().filter(|f| f.constant().is_some()).collect();

    let mut ctx_all: Vec<_> = consts
        .iter()
        .filter(|f| f.name() == "CTX_ALL")
        .filter_map(|f| field_arch(f))
        .collect();
    ctx_all.sort();
    assert_eq!(
        ctx_all,
        vec![2, 4],
        "CTX_ALL const arch tags lost on round-trip"
    );

    let x64_only = consts.iter().find(|f| f.name() == "X64_ONLY").unwrap();
    assert_eq!(
        field_arch(x64_only),
        Some(2),
        "X64_ONLY const arch tag lost"
    );
    let arm_only = consts.iter().find(|f| f.name() == "ARM_ONLY").unwrap();
    assert_eq!(
        field_arch(arm_only),
        Some(4),
        "ARM_ONLY const arch tag lost"
    );
}

// Regression for `HSTRING_HEADER`: nested type shape must participate in the merge signature.
#[test]
fn arch_divergent_nested_type_hoists_arch_to_enclosing() {
    let dir = std::env::temp_dir().join("win_arch_nested");
    std::fs::create_dir_all(&dir).unwrap();

    let wide = "#[win32] mod Test { struct HDR { Reserved: union { Reserved1: i64, Reserved2: [i8; 24] } } }";
    let narrow = "#[win32] mod Test { struct HDR { Reserved: union { Reserved1: i64, Reserved2: [i8; 20] } } }";
    let x64 = winmd(&dir, "x64", wide);
    let arm = winmd(&dir, "arm", wide);
    let x86 = winmd(&dir, "x86", narrow);

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .arch_input(&x86, 1)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();

    let mut hdr: Vec<_> = index
        .types()
        .filter(|t| t.name() == "HDR" && !t.flags().is_nested())
        .filter_map(|t| type_arch(&t))
        .collect();
    hdr.sort();
    assert_eq!(
        hdr,
        vec![1, 6],
        "arch-divergent nested type must split enclosing struct into x86 (1) + x64|arm64 (6)"
    );

    for t in index
        .types()
        .filter(|t| t.name() == "HDR" && !t.flags().is_nested())
    {
        assert_eq!(
            index.nested(t).count(),
            1,
            "each arch copy of HDR must keep its inline nested union"
        );
    }
}

// `ClassLayout` cannot represent forced over-alignment, so the merge must compare its attribute.
#[test]
fn arch_divergent_forced_alignment_splits() {
    let dir = std::env::temp_dir().join("win_arch_align");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { #[align(16)] struct AS { a: i64 } }",
    );
    let arm = winmd(
        &dir,
        "arm",
        "#[win32] mod Test { #[align(32)] struct AS { a: i64 } }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();

    let mut aligns: Vec<_> = index
        .types()
        .filter(|t| t.name() == "AS")
        .filter_map(|t| type_arch(&t))
        .collect();
    aligns.sort();
    assert_eq!(
        aligns,
        vec![2, 4],
        "arch-divergent forced alignment must split into x64 (2) + arm64 (4)"
    );
}

// Divergent types present on only an arch subset must not inherit the subset's union tag.
#[test]
fn subset_present_divergent_type_splits() {
    let dir = std::env::temp_dir().join("win_arch_subset");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { struct CTX { a: i32, b: i32 } }",
    );
    let arm = winmd(&dir, "arm", "#[win32] mod Test { struct CTX { x: i64 } }");
    let x86 = winmd(&dir, "x86", "#[win32] mod Test { struct OTHER { z: i32 } }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .arch_input(&x86, 1)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();

    let mut ctx: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CTX")
        .filter_map(|t| type_arch(&t))
        .collect();
    ctx.sort();
    assert_eq!(
        ctx,
        vec![2, 4],
        "subset-present divergent type must split into x64 (2) + arm64 (4), not collapse"
    );
}

// Enum constant values must participate in the arch merge signature.
#[test]
fn arch_divergent_enum_constant_values_split() {
    let dir = std::env::temp_dir().join("win_arch_enum_vals");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 1, B = 2 } }",
    );
    let arm = winmd(
        &dir,
        "arm",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 10, B = 20 } }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();

    let mut e: Vec<_> = index
        .types()
        .filter(|t| t.name() == "E")
        .filter_map(|t| type_arch(&t))
        .collect();
    e.sort();
    assert_eq!(
        e,
        vec![2, 4],
        "enum with divergent per-arch member values must split into x64 (2) + arm64 (4)"
    );
}

// Identical shapes share the union of their arch bits.
#[test]
fn structurally_identical_arch_copies_coalesce() {
    let dir = std::env::temp_dir().join("win_arch_coalesce");
    std::fs::create_dir_all(&dir).unwrap();

    let same = "#[win32] mod Test { struct CTX { a: i32, b: i32 } }";
    let x64 = winmd(&dir, "x64", same);
    let x86 = winmd(&dir, "x86", same);
    let arm = winmd(&dir, "arm", "#[win32] mod Test { struct CTX { x: i32 } }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&x86, 1)
        .arch_input(&arm, 4)
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();
    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();

    let mut ctx: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CTX")
        .filter_map(|t| type_arch(&t))
        .collect();
    ctx.sort();
    assert_eq!(
        ctx,
        vec![3, 4],
        "x64+x86 identical copies must coalesce to a single x64|x86 (3) def, arm64 (4) separate"
    );
}
