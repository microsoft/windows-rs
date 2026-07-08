// Multi-arch integrity probe. Dumps every TypeDef named CONTEXT (and ARM64_NT_CONTEXT)
// with its SupportedArchitecture bitmask and field list, plus a count of arch-tagged
// types, so we can verify the arch-merge produced correct per-arch definitions.
//
// Run: cargo test -p test_metadata --test arch_probe -- --ignored --nocapture

use windows_metadata::reader::{HasAttributes, Index, TypeCategory};

fn arch_name(bits: i32) -> String {
    if bits == 0 {
        return "neutral".to_string();
    }
    let mut parts = vec![];
    if bits & 1 != 0 {
        parts.push("x86");
    }
    if bits & 2 != 0 {
        parts.push("x64");
    }
    if bits & 4 != 0 {
        parts.push("arm64");
    }
    format!("{} ({bits})", parts.join("+"))
}

fn dump(path: &str) {
    let index = Index::read(path).unwrap();

    // Per-arch tag histogram + a count of how many distinct types are arch-tagged.
    let mut tagged = 0usize;
    let mut total = 0usize;
    let mut by_mask: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    for ty in index.types() {
        total += 1;
        let a = ty.arches();
        if a != 0 {
            tagged += 1;
            *by_mask.entry(a).or_default() += 1;
        }
    }
    println!("\n===== {path} =====");
    println!("types: {total}  arch-tagged: {tagged}");
    for (mask, n) in &by_mask {
        println!("  tag {:<14} : {n}", arch_name(*mask));
    }

    for target in ["CONTEXT", "ARM64_NT_CONTEXT"] {
        let mut copies: Vec<_> = index.types().filter(|t| t.name() == target).collect();
        copies.sort_by_key(|t| t.arches());
        println!("\n  --- {target}: {} TypeDef(s) ---", copies.len());
        for ty in copies {
            let kind = match ty.category() {
                TypeCategory::Struct => "struct",
                TypeCategory::Class => "class",
                _ => "other",
            };
            let fields: Vec<String> = ty.fields().map(|f| f.name().to_string()).collect();
            println!(
                "    {} {}  arch={}  fields={} [{}]",
                kind,
                ty.name(),
                arch_name(ty.arches()),
                fields.len(),
                fields
                    .iter()
                    .take(8)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
}

#[test]
#[ignore]
fn arch_reference() {
    dump("../../../../crates/tools/package/reference/Windows.Win32.winmd");
}

#[test]
#[ignore]
fn arch_inhouse() {
    dump("../../../../target/win32/Windows.Win32.winmd");
}
