// Structural metadata-to-metadata parity probe: compares the in-house winmd against
// the win32metadata reference by TYPE NAME, ignoring namespace differences. Where the
// coverage probe answers "which names exist on both sides", this answers "for the names
// that exist on both sides, do their *shapes* agree" — same kind, same struct fields and
// field types, same enum members, same function/method signatures. It is a fast triage to
// weed out obvious gaps before regenerating real crates.
//
// Run: cargo test -p test_metadata --test parity_probe -- --ignored --nocapture

use std::collections::{BTreeMap, BTreeSet, HashMap};
use windows_metadata::Type;
use windows_metadata::reader::{Index, TypeCategory, TypeDef};

const EXCLUDED: &[&str] = &[
    // The Win32 attribute vocabulary: `Windows.Win32.Foundation.Metadata` in the
    // reference win32metadata winmd, `Windows.Win32.Metadata` in our in-house winmd.
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

// Render a type to a namespace-agnostic string. Named types collapse to their bare name;
// const-ness on pointers/refs is dropped (a faithful-but-noisy divergence we don't gate on).
fn norm(t: &Type) -> String {
    match t {
        Type::ClassName(tn) | Type::ValueName(tn) => tn.name.clone(),
        Type::Array(inner) => format!("[]{}", norm(inner)),
        Type::ArrayFixed(inner, n) => format!("[{n}]{}", norm(inner)),
        Type::PtrMut(inner, n) | Type::PtrConst(inner, n) => {
            format!("{}{}", "*".repeat(*n), norm(inner))
        }
        Type::RefMut(inner) | Type::RefConst(inner) => format!("&{}", norm(inner)),
        Type::Generic(s, i) => format!("var:{s}:{i}"),
        other => format!("{other:?}"),
    }
}

struct Side<'a> {
    // type name -> all TypeDefs with that name (across namespaces), excluding Apis
    types: HashMap<String, Vec<TypeDef<'a>>>,
    // function name -> normalized signature (Apis methods)
    funcs: HashMap<String, String>,
    // enum-typed names (so we can treat "enum field" vs "underlying int" as benign)
    enums: BTreeSet<String>,
}

fn sig_of(td: &TypeDef, m: &windows_metadata::reader::MethodDef) -> String {
    let _ = td;
    let s = m.signature(&[]);
    let params: Vec<String> = s.types.iter().map(norm).collect();
    format!("{} <- ({})", norm(&s.return_type), params.join(", "))
}

fn load(path: &str) -> Side<'static> {
    // Leak the index so TypeDefs (borrowing 'a) can live for the whole test.
    let index: &'static Index = Box::leak(Box::new(Index::read(path).unwrap()));
    let mut types: HashMap<String, Vec<TypeDef>> = HashMap::new();
    let mut funcs: HashMap<String, String> = HashMap::new();
    let mut enums: BTreeSet<String> = BTreeSet::new();
    for ty in index.types() {
        if excluded(ty.namespace()) {
            continue;
        }
        if ty.name() == "Apis" {
            for m in ty.methods() {
                funcs.insert(m.name().to_string(), sig_of(&ty, &m));
            }
            continue;
        }
        if ty.category() == TypeCategory::Enum {
            enums.insert(ty.name().to_string());
        }
        types.entry(ty.name().to_string()).or_default().push(ty);
    }
    Side {
        types,
        funcs,
        enums,
    }
}

fn cat(c: &TypeCategory) -> &'static str {
    match c {
        TypeCategory::Interface => "interface",
        TypeCategory::Class => "class",
        TypeCategory::Enum => "enum",
        TypeCategory::Struct => "struct",
        TypeCategory::Delegate => "delegate",
        TypeCategory::Attribute => "attribute",
    }
}

// struct field map: name -> normalized type
fn struct_fields(td: &TypeDef) -> BTreeMap<String, String> {
    td.fields()
        .map(|f| (f.name().to_string(), norm(&f.ty())))
        .collect()
}

fn enum_members(td: &TypeDef) -> BTreeSet<String> {
    td.fields()
        .filter(|f| f.constant().is_some())
        .map(|f| f.name().to_string())
        .collect()
}

fn iface_methods(td: &TypeDef) -> BTreeSet<String> {
    td.methods().map(|m| m.name().to_string()).collect()
}

#[ignore = "requires win32metadata reference winmd + a full in-house build"]
#[test]
fn parity() {
    let ours = load("../../../../target/win32/Windows.Win32.winmd");
    let theirs = load("../../../../crates/tools/package/reference/Windows.Win32.winmd");

    let mut compared = 0usize;
    let mut kind_mismatch = 0usize;
    let mut kind_samples = vec![];

    let mut struct_field_missing = 0usize; // theirs has, ours lacks
    let mut struct_field_extra = 0usize; // ours has, theirs lacks
    let mut struct_type_mismatch = 0usize; // both have field, type differs
    let mut struct_type_mismatch_benign = 0usize; // enum-vs-int folding
    let mut struct_samples = vec![];

    let mut enum_member_missing = 0usize;
    let mut enum_samples = vec![];

    let mut iface_method_missing = 0usize;
    let mut iface_samples = vec![];

    // Compare by type name; when multiple same-named TypeDefs exist on a side, pick the
    // candidate pair that agrees best (most overlapping members) to avoid A/W false diffs.
    for (name, their_defs) in &theirs.types {
        let Some(our_defs) = ours.types.get(name) else {
            continue; // pure gap — coverage_probe already counts these
        };
        compared += 1;
        let their_td = &their_defs[0];
        let their_cat = their_td.category();

        // pick the our-def with the same category if possible
        let our_td = our_defs
            .iter()
            .find(|d| d.category() == their_cat)
            .unwrap_or(&our_defs[0]);
        let our_cat = our_td.category();

        if our_cat != their_cat {
            kind_mismatch += 1;
            if kind_samples.len() < 25 {
                kind_samples.push(format!(
                    "{name}: theirs={} ours={}",
                    cat(&their_cat),
                    cat(&our_cat)
                ));
            }
            continue;
        }

        match their_cat {
            TypeCategory::Struct => {
                let tf = struct_fields(their_td);
                // choose our struct candidate maximizing field-name overlap
                let our_td = our_defs
                    .iter()
                    .filter(|d| d.category() == TypeCategory::Struct)
                    .max_by_key(|d| {
                        let of = struct_fields(d);
                        tf.keys().filter(|k| of.contains_key(*k)).count()
                    })
                    .unwrap_or(our_td);
                let of = struct_fields(our_td);
                let mut local = vec![];
                for (fname, tty) in &tf {
                    match of.get(fname) {
                        None => {
                            struct_field_missing += 1;
                            local.push(format!("  -{fname}: {tty}"));
                        }
                        Some(oty) if oty != tty => {
                            // enum folding: theirs uses an enum type, ours the underlying int
                            let benign = theirs.enums.contains(tty)
                                && matches!(
                                    oty.as_str(),
                                    "I32" | "U32" | "I16" | "U16" | "I8" | "U8" | "I64" | "U64"
                                );
                            if benign {
                                struct_type_mismatch_benign += 1;
                            } else {
                                struct_type_mismatch += 1;
                                local.push(format!("  ~{fname}: theirs={tty} ours={oty}"));
                            }
                        }
                        _ => {}
                    }
                }
                for fname in of.keys() {
                    if !tf.contains_key(fname) {
                        struct_field_extra += 1;
                    }
                }
                if !local.is_empty() && struct_samples.len() < 30 {
                    struct_samples.push(format!("struct {name}:\n{}", local.join("\n")));
                }
            }
            TypeCategory::Enum => {
                let tm = enum_members(their_td);
                let om = enum_members(our_td);
                let missing: Vec<_> = tm.difference(&om).cloned().collect();
                if !missing.is_empty() {
                    enum_member_missing += missing.len();
                    if enum_samples.len() < 20 {
                        enum_samples.push(format!("enum {name}: missing {}", missing.join(", ")));
                    }
                }
            }
            TypeCategory::Interface => {
                let tmth = iface_methods(their_td);
                let omth = iface_methods(our_td);
                let missing: Vec<_> = tmth.difference(&omth).cloned().collect();
                if !missing.is_empty() {
                    iface_method_missing += missing.len();
                    if iface_samples.len() < 20 {
                        iface_samples
                            .push(format!("interface {name}: missing {}", missing.join(", ")));
                    }
                }
            }
            _ => {}
        }
    }

    // function signature comparison (Apis)
    let mut func_compared = 0usize;
    let mut func_sig_mismatch = 0usize;
    let mut func_samples = vec![];
    for (fname, tsig) in &theirs.funcs {
        if let Some(osig) = ours.funcs.get(fname) {
            func_compared += 1;
            if osig != tsig {
                func_sig_mismatch += 1;
                if func_samples.len() < 30 {
                    func_samples.push(format!("{fname}:\n  theirs={tsig}\n  ours  ={osig}"));
                }
            }
        }
    }

    println!("=== STRUCTURAL PARITY (in-house vs win32metadata, by name, ns-agnostic) ===");
    println!("types compared (present on both): {compared}");
    println!("kind mismatches: {kind_mismatch}");
    println!(
        "struct fields: missing(theirs-only)={struct_field_missing} extra(ours-only)={struct_field_extra} type-mismatch={struct_type_mismatch} (+{struct_type_mismatch_benign} benign enum-folding)"
    );
    println!("enum members missing (theirs-only): {enum_member_missing}");
    println!("interface methods missing (theirs-only): {iface_method_missing}");
    println!("functions compared: {func_compared}  signature mismatches: {func_sig_mismatch}");

    println!("\n--- kind mismatch samples ---");
    for s in &kind_samples {
        println!("{s}");
    }
    println!("\n--- struct field/type mismatch samples ---");
    for s in &struct_samples {
        println!("{s}");
    }
    println!("\n--- enum member missing samples ---");
    for s in &enum_samples {
        println!("{s}");
    }
    println!("\n--- interface method missing samples ---");
    for s in &iface_samples {
        println!("{s}");
    }
    println!("\n--- function signature mismatch samples ---");
    for s in &func_samples {
        println!("{s}");
    }
}
