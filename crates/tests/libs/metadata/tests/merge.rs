use windows_metadata::*;

/// Builds a winmd from inline RDL into `dir/name.winmd` and returns its path.
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

fn arch_bits(field: reader::Field) -> Option<i32> {
    field.attributes().find_map(|a| {
        (a.ctor().parent().name() == "SupportedArchitectureAttribute").then(|| {
            match a.value().first() {
                Some((_, Value::I32(v))) => *v,
                _ => 0,
            }
        })
    })
}

fn type_arch_bits(ty: reader::TypeDef) -> Option<i32> {
    ty.has_attribute("SupportedArchitectureAttribute")
        .then(|| ty.arches())
}

#[test]
fn arch_merge_constants() {
    let dir = std::env::temp_dir().join("win_merge_test");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "mod Test { const SHARED: i32 = 7; const CTX_ALL: i32 = 100; const X64_ONLY: i32 = 1; }",
    );
    let arm = winmd(
        &dir,
        "arm",
        "mod Test { const SHARED: i32 = 7; const CTX_ALL: i32 = 200; const ARM_ONLY: i32 = 2; }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let apis = index.types().find(|t| t.name() == "Apis").unwrap();
    let consts: Vec<_> = apis.fields().filter(|f| f.constant().is_some()).collect();

    let shared: Vec<_> = consts.iter().filter(|f| f.name() == "SHARED").collect();
    assert_eq!(shared.len(), 1);
    assert_eq!(arch_bits(*shared[0]), None);

    let mut ctx: Vec<_> = consts
        .iter()
        .filter(|f| f.name() == "CTX_ALL")
        .filter_map(|f| arch_bits(*f))
        .collect();
    ctx.sort();
    assert_eq!(ctx, vec![2, 4]);

    // Arch-only constants are present and tagged.
    let x64_only = consts.iter().find(|f| f.name() == "X64_ONLY").unwrap();
    assert_eq!(arch_bits(*x64_only), Some(2));
    let arm_only = consts.iter().find(|f| f.name() == "ARM_ONLY").unwrap();
    assert_eq!(arch_bits(*arm_only), Some(4));
}

#[test]
fn union_enums_merges_members() {
    let dir = std::env::temp_dir().join("win_merge_enum_union");
    std::fs::create_dir_all(&dir).unwrap();

    // A `um` header truncates the enum; the `km` header defines it fully.
    let um = winmd(
        &dir,
        "um",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 0 } }",
    );
    let km = winmd(
        &dir,
        "km",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 0, B = 1, C = 2 } }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .input(&um)
        .input(&km)
        .union_enums(true)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let enums: Vec<_> = index.types().filter(|t| t.name() == "E").collect();
    assert_eq!(enums.len(), 1, "same-named enums should union into one");

    let mut members: Vec<_> = enums[0]
        .fields()
        .filter(|f| f.constant().is_some())
        .map(|f| f.name().to_string())
        .collect();
    members.sort();
    assert_eq!(members, vec!["A", "B", "C"]);
}

#[test]
fn union_enums_rejects_conflicting_values() {
    let dir = std::env::temp_dir().join("win_merge_enum_conflict");
    std::fs::create_dir_all(&dir).unwrap();

    let a = winmd(
        &dir,
        "a",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 0 } }",
    );
    let b = winmd(
        &dir,
        "b",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 9 } }",
    );

    let merged = dir.join("merged.winmd");
    let result = merge()
        .input(&a)
        .input(&b)
        .union_enums(true)
        .output(&merged)
        .merge();

    assert!(result.is_err(), "conflicting member values must error");
}

#[test]
fn union_enums_rejects_conflicting_non_sentinel_max() {
    let dir = std::env::temp_dir().join("win_merge_enum_max_conflict");
    std::fs::create_dir_all(&dir).unwrap();

    // `FOO_MAX` contains "Max" but is a real value, not an NT count sentinel (which is spelled
    // `Max*` or `*Maximum`). A differing value across copies is a genuine conflict and must be
    // rejected, not silently reconciled by the sentinel tolerance.
    let a = winmd(
        &dir,
        "a",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 0, FOO_MAX = 1 } }",
    );
    let b = winmd(
        &dir,
        "b",
        "#[win32] mod Test { #[repr(i32)] enum E { A = 0, FOO_MAX = 2 } }",
    );

    let merged = dir.join("merged.winmd");
    let result = merge()
        .input(&a)
        .input(&b)
        .union_enums(true)
        .output(&merged)
        .merge();

    assert!(
        result.is_err(),
        "a conflicting non-sentinel `Max` member must error"
    );
}

#[test]
fn union_enums_merges_partial_copies() {
    let dir = std::env::temp_dir().join("win_merge_enum_partial");
    std::fs::create_dir_all(&dir).unwrap();

    // Neither copy is a superset: `um` contributes `Named` (which `km` omits) and a lower `Max*`
    // count sentinel, while `km` contributes members `um` omits and the larger sentinel. The
    // union carries every member, the larger sentinel wins, and `Named` is appended.
    let um = winmd(
        &dir,
        "um",
        "#[win32] mod Test { #[repr(i32)] enum E { Shared = 1, Named = 38, MaxE = 2 } }",
    );
    let km = winmd(
        &dir,
        "km",
        "#[win32] mod Test { #[repr(i32)] enum E { First = 0, Shared = 1, Second = 2, MaxE = 3 } }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .input(&um)
        .input(&km)
        .union_enums(true)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let enums: Vec<_> = index.types().filter(|t| t.name() == "E").collect();
    assert_eq!(enums.len(), 1, "same-named enums should union into one");

    let members: Vec<(String, i64)> = enums[0]
        .fields()
        .filter_map(|f| {
            f.constant().map(|c| {
                let value = match c.value() {
                    Value::I32(v) => v as i64,
                    other => panic!("unexpected value {other:?}"),
                };
                (f.name().to_string(), value)
            })
        })
        .collect();

    // The larger `MaxE` sentinel (3) wins over the truncated one (2), and `Named` is appended.
    assert!(members.contains(&("First".to_string(), 0)));
    assert!(members.contains(&("Shared".to_string(), 1)));
    assert!(members.contains(&("Second".to_string(), 2)));
    assert!(members.contains(&("Named".to_string(), 38)));
    assert!(members.contains(&("MaxE".to_string(), 3)));
    assert_eq!(
        members.iter().filter(|(n, _)| n == "MaxE").count(),
        1,
        "sentinel must not be duplicated"
    );
}

#[test]
fn arch_merge_divergent_struct() {
    let dir = std::env::temp_dir().join("win_merge_div");
    std::fs::create_dir_all(&dir).unwrap();

    // CTX has a different shape per arch (the CONTEXT pattern): a 2-field struct on x64,
    // a 1-field struct on arm64. It must NOT collapse - both copies survive, arch-tagged.
    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { struct CTX { a: i32, b: i32 } }",
    );
    let arm = winmd(&dir, "arm", "#[win32] mod Test { struct CTX { x: i32 } }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let ctx: Vec<_> = index.types().filter(|t| t.name() == "CTX").collect();
    assert_eq!(ctx.len(), 2);
    let mut tags: Vec<_> =
        ctx.iter()
            .map(|t| {
                t.attributes()
                    .find_map(|a| {
                        (a.ctor().parent().name() == "SupportedArchitectureAttribute").then(|| {
                            match a.value().first() {
                                Some((_, Value::I32(v))) => *v,
                                _ => 0,
                            }
                        })
                    })
                    .unwrap()
            })
            .collect();
    tags.sort();
    assert_eq!(tags, vec![2, 4]);
}

#[test]
fn arch_merge_normalizes_native_sized_callback_signature() {
    let dir = std::env::temp_dir().join("win_merge_native_callback");
    std::fs::create_dir_all(&dir).unwrap();

    let wide = "#[win32] mod Test { extern fn CB(value: usize) -> isize; }";
    let x64 = winmd(&dir, "x64", wide);
    let arm = winmd(&dir, "arm", wide);
    let x86 = winmd(
        &dir,
        "x86",
        "#[win32] mod Test { extern fn CB(value: u32) -> i32; }",
    );

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .arch_input(&x86, 1)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let callbacks: Vec<_> = index.types().filter(|ty| ty.name() == "CB").collect();
    assert_eq!(callbacks.len(), 1);
    assert_eq!(type_arch_bits(callbacks[0]), None);

    let signature = callbacks[0]
        .methods()
        .find(|method| method.name() == "Invoke")
        .unwrap()
        .signature(&[]);
    assert_eq!(signature.return_type, Type::ISize);
    assert_eq!(signature.types, vec![Type::USize]);
}

#[test]
fn arch_merge_does_not_infer_native_size_without_native_evidence() {
    let dir = std::env::temp_dir().join("win_merge_fixed_callback");
    std::fs::create_dir_all(&dir).unwrap();

    let wide = "#[win32] mod Test { extern fn CB() -> i64; }";
    let x64 = winmd(&dir, "x64", wide);
    let arm = winmd(&dir, "arm", wide);
    let x86 = winmd(&dir, "x86", "#[win32] mod Test { extern fn CB() -> i32; }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .arch_input(&x86, 1)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let mut arches: Vec<_> = index
        .types()
        .filter(|ty| ty.name() == "CB")
        .filter_map(type_arch_bits)
        .collect();
    arches.sort();
    assert_eq!(arches, vec![1, 6]);
}

#[test]
fn arch_merge_rejects_fixed_integer_with_wrong_pointer_width() {
    let dir = std::env::temp_dir().join("win_merge_wrong_width_callback");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { extern fn CB() -> isize; }",
    );
    let arm = winmd(&dir, "arm", "#[win32] mod Test { extern fn CB() -> i32; }");
    let x86 = winmd(&dir, "x86", "#[win32] mod Test { extern fn CB() -> i32; }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&arm, 4)
        .arch_input(&x86, 1)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let mut arches: Vec<_> = index
        .types()
        .filter(|ty| ty.name() == "CB")
        .filter_map(type_arch_bits)
        .collect();
    arches.sort();
    assert_eq!(arches, vec![2, 5]);
}

#[test]
fn arch_merge_rejects_callback_attribute_mismatch() {
    let dir = std::env::temp_dir().join("win_merge_callback_attributes");
    std::fs::create_dir_all(&dir).unwrap();

    let x64 = winmd(
        &dir,
        "x64",
        "#[win32] mod Test { extern \"C\" fn CB() -> isize; }",
    );
    let x86 = winmd(&dir, "x86", "#[win32] mod Test { extern fn CB() -> i32; }");

    let merged = dir.join("merged.winmd");
    merge()
        .arch_input(&x64, 2)
        .arch_input(&x86, 1)
        .output(&merged)
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let mut arches: Vec<_> = index
        .types()
        .filter(|ty| ty.name() == "CB")
        .filter_map(type_arch_bits)
        .collect();
    arches.sort();
    assert_eq!(arches, vec![1, 2]);
}
