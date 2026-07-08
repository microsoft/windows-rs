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

/// The full multi-arch pipeline must round-trip `SupportedArchitecture` through the
/// RDL text form: per-arch winmd -> arch-merge -> RDL (writer) -> winmd (reader). The
/// merged winmd is the source of truth; the committed RDL is its faithful text view,
/// so the writer and reader must agree on the `#[arch(...)]` form for both types and
/// constants (the CONTEXT / CONTEXT_ARM64_* pattern).
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

    // Writer: merged winmd -> RDL (split per namespace).
    let rdl_dir = dir.join("rdl");
    std::fs::create_dir_all(&rdl_dir).unwrap();
    windows_rdl::writer()
        .input(merged.to_string_lossy().as_ref())
        .output(rdl_dir.to_string_lossy().as_ref())
        .split(true)
        .write()
        .unwrap();

    // The text form must use the `#[arch(...)]` sugar uniformly — including on
    // constants, which previously leaked the raw long-form attribute the reader
    // could not parse.
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

    // Reader: RDL -> winmd, arch tags must match the merged winmd exactly.
    let out = dir.join("roundtrip.winmd");
    windows_rdl::reader()
        .input(rdl_dir.to_string_lossy().as_ref())
        .output(out.to_string_lossy().as_ref())
        .write()
        .unwrap();
    let index = reader::Index::read(out.to_string_lossy().as_ref()).unwrap();

    // CTX: two arch-tagged copies (the divergent CONTEXT shape).
    let mut ctx: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CTX")
        .filter_map(|t| type_arch(&t))
        .collect();
    ctx.sort();
    assert_eq!(ctx, vec![2, 4], "CTX struct arch tags lost on round-trip");

    // CB: an arch-divergent Win32 callback (different signature per arch) must also
    // survive as two arch-tagged copies — callbacks are reference TypeDefs, so this
    // exercises the merge split + the callback #[arch] writer/reader path.
    let mut cb: Vec<_> = index
        .types()
        .filter(|t| t.name() == "CB")
        .filter_map(|t| type_arch(&t))
        .collect();
    cb.sort();
    assert_eq!(cb, vec![2, 4], "CB callback arch tags lost on round-trip");

    // AL: an arch-divergent `type X = Y` alias (NativeTypedef) with a different underlying
    // type per arch. These decompile to the bare `type` form, which must carry `#[arch(...)]`
    // through the writer and reader (otherwise arch-only pointer/scalar aliases lose their tag).
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

    // Constants: CTX_ALL split per arch, plus the arch-only constants.
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

/// A struct whose *inline nested* type diverges per arch must be split per arch,
/// with the `#[arch(...)]` hoisted onto the enclosing struct. The enclosing struct's
/// own fields reference the nested type by its (arch-invariant) leaf name, so the
/// divergence is invisible unless the merge signature recurses into nested types.
/// This is the `HSTRING_HEADER` case: its inline union is `char[24]` on 64-bit and
/// `char[20]` on x86. Without the recursion the merge collapses the enclosing struct
/// to one neutral copy and silently drops the other arch's nested shape.
#[test]
fn arch_divergent_nested_type_hoists_arch_to_enclosing() {
    let dir = std::env::temp_dir().join("win_arch_nested");
    std::fs::create_dir_all(&dir).unwrap();

    // 64-bit (x64, arm64) share the `[i8; 24]` inline union; x86 has `[i8; 20]`.
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

    // The enclosing struct is split: x64|arm64 (2|4 = 6) share `[24]`, x86 (1) has `[20]`.
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

    // Both copies must retain their nested union (nesting not dropped by the split).
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

/// Two arch copies identical in fields and layout but differing *only* in forced
/// over-alignment (`__declspec(align(N))`, carried by `AlignmentAttribute`) must be
/// split per arch. `ClassLayout` can only lower alignment, so the raised value lives
/// solely in the attribute; if the merge signature ignores it, the copies collapse to
/// one arch-neutral definition and the other arch's `#[repr(C, align(N))]` is wrong.
#[test]
fn arch_divergent_forced_alignment_splits() {
    let dir = std::env::temp_dir().join("win_arch_align");
    std::fs::create_dir_all(&dir).unwrap();

    // Same single `i64` field on both arches; only the forced alignment differs.
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

/// A type present on only a *subset* of the arches in the run, whose shape *diverges*
/// across that subset, must still be split per arch — not emitted once from `copies[0]`
/// tagged with the whole subset union (which would give the other arch the wrong layout).
#[test]
fn subset_present_divergent_type_splits() {
    let dir = std::env::temp_dir().join("win_arch_subset");
    std::fs::create_dir_all(&dir).unwrap();

    // CTX exists only on x64 (2) and arm64 (4) — x86 (1) has an unrelated type — and its
    // shape diverges between the two arches it appears on.
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

/// An enum whose members share names and types but hold *different constant values*
/// per arch must be split per arch. Fields are keyed by name + type + constant value so
/// the divergence surfaces; otherwise the copies collapse and one arch's values are lost.
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

/// When a type's shape differs on *some* arches but is byte-identical on others,
/// the merge coalesces the identical arches into a single definition tagged with
/// the union of their arch bits, instead of emitting duplicate copies. (E.g.
/// `ARM64_NT_CONTEXT` is identical on x64 and x86 but differs on arm64.)
#[test]
fn structurally_identical_arch_copies_coalesce() {
    let dir = std::env::temp_dir().join("win_arch_coalesce");
    std::fs::create_dir_all(&dir).unwrap();

    // CTX is identical on x64 (2) and x86 (1); arm64 (4) diverges.
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
