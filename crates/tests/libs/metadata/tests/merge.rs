use windows_metadata::*;

/// Builds a winmd from inline RDL into `dir/name.winmd` and returns its path.
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
        .output(merged.to_string_lossy().as_ref())
        .merge()
        .unwrap();

    let index = reader::Index::read(merged.to_string_lossy().as_ref()).unwrap();
    let apis = index.types().find(|t| t.name() == "Apis").unwrap();
    let consts: Vec<_> = apis.fields().filter(|f| f.constant().is_some()).collect();

    // SHARED identical on both → exactly one, untagged.
    let shared: Vec<_> = consts.iter().filter(|f| f.name() == "SHARED").collect();
    assert_eq!(shared.len(), 1);
    assert_eq!(arch_bits(*shared[0]), None);

    // CTX_ALL differs by value → two copies, each tagged with its arch.
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
fn arch_merge_divergent_struct() {
    let dir = std::env::temp_dir().join("win_merge_div");
    std::fs::create_dir_all(&dir).unwrap();

    // CTX has a different shape per arch (the CONTEXT pattern): a 2-field struct on x64,
    // a 1-field struct on arm64. It must NOT collapse — both copies survive, arch-tagged.
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
        .output(merged.to_string_lossy().as_ref())
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
