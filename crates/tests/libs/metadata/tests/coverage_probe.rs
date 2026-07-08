use std::collections::BTreeSet;
use windows_metadata::reader::Index;

/// Namespaces windows-rs deliberately drops from the win32metadata winmd (see
/// `crates/tools/package/src/windows.txt`). We exclude them on both sides so the
/// parity metric reflects the surface windows-rs actually ships — chiefly the very
/// large, very old MsHtml (HTML DOM) API, which we likewise do not scrape.
const EXCLUDED: &[&str] = &[
    // The Win32 attribute vocabulary: `Windows.Win32.Foundation.Metadata` in the
    // reference win32metadata winmd, `Windows.Win32.Metadata` in our in-house winmd.
    // Both are metadata plumbing, not API surface, so drop them on both sides.
    "Windows.Win32.Foundation.Metadata",
    "Windows.Win32.Metadata",
    "Windows.Win32.System.Diagnostics.Debug.WebApp",
    "Windows.Win32.System.WinRT.Xaml",
    "Windows.Win32.UI.Xaml",
    "Windows.Win32.Web.MsHtml",
];

fn excluded(ns: &str) -> bool {
    EXCLUDED
        .iter()
        .any(|e| ns == *e || ns.starts_with(&format!("{e}.")))
}

fn names(path: &str) -> (BTreeSet<String>, usize, usize, usize) {
    let index = Index::read(path).unwrap();
    let mut all = BTreeSet::new();
    let mut types = 0;
    let mut funcs = 0;
    let mut consts = 0;
    for ty in index.types() {
        if excluded(ty.namespace()) {
            continue;
        }
        let n = ty.name();
        if n == "Apis" {
            for m in ty.methods() {
                all.insert(m.name().to_string());
                funcs += 1;
            }
            for f in ty.fields() {
                all.insert(f.name().to_string());
                consts += 1;
            }
        } else {
            all.insert(n.to_string());
            types += 1;
            // Count enum members too: win32metadata groups thousands of constants
            // as enum fields while we may emit them flat in Apis. Counting member
            // names on both sides keeps the parity metric apples-to-apples.
            for f in ty.fields() {
                if f.constant().is_some() {
                    all.insert(f.name().to_string());
                    consts += 1;
                }
            }
        }
    }
    (all, types, funcs, consts)
}

#[ignore = "requires win32metadata reference winmd + a full in-house build"]
#[test]
fn coverage() {
    let ours = "../../../../target/win32/Windows.Win32.winmd";
    let theirs = "../../../../crates/tools/package/reference/Windows.Win32.winmd";
    let (o, ot, of, oc) = names(ours);
    let (t, tt, tf, tc) = names(theirs);
    println!(
        "OURS:  names={} types={} funcs={} consts={}",
        o.len(),
        ot,
        of,
        oc
    );
    println!(
        "THEIRS: names={} types={} funcs={} consts={}",
        t.len(),
        tt,
        tf,
        tc
    );
    let both: usize = o.intersection(&t).count();
    println!(
        "overlap names={} ({:.1}% of theirs)",
        both,
        100.0 * both as f64 / t.len() as f64
    );
    let ours_only = o.difference(&t).count();
    println!(
        "ours-only={}  theirs-only={}",
        ours_only,
        t.difference(&o).count()
    );
}
