// Nested-type fidelity probe. The reference win32metadata winmd represents anonymous
// nested struct/union members as genuine NestedClass nesting; our clang->RDL->winmd
// pipeline hoists them to top-level `{Outer}_{n}` siblings because the RDL grammar has
// no nested-type syntax. This dumps NestedClass vs hoisted-sibling counts so the gap can
// be tracked as windows-rdl gains nested support (or the scrape moves direct-to-winmd).
//
// Run: cargo test -p test_metadata --test nested_probe -- --ignored --nocapture
use windows_metadata::reader::Index;

fn stats(label: &str, path: &str) {
    let index = Index::read(path).unwrap();
    let mut total = 0usize;
    let mut nested = 0usize;
    let mut hoisted_siblings = 0usize; // top-level types named like FOO_0 / FOO_1 (hoisted anon)
    for ty in index.types() {
        total += 1;
        nested += index.nested(ty).count();
        let n = ty.name();
        if !ty.flags().is_nested()
            && n.rsplit_once('_')
                .is_some_and(|(_, s)| s.chars().all(|c| c.is_ascii_digit()) && !s.is_empty())
        {
            hoisted_siblings += 1;
        }
    }
    println!(
        "{label}: top-level types={total} NestedClass={nested} hoisted-_N-siblings={hoisted_siblings}"
    );
}

#[test]
#[ignore]
fn nested_stats() {
    stats(
        "reference ",
        "../../../../crates/tools/package/reference/Windows.Win32.winmd",
    );
    stats("in-house  ", "../../../../target/win32/Windows.Win32.winmd");
}
