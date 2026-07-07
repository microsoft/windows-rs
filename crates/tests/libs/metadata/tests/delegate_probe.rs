// Probe (ignored; needs the win32metadata reference winmd + a full in-house build):
// categorize every name that win32metadata emits as a `delegate`. Confirms our faithful
// function-pointer handling: direct callbacks are delegates; `type X = <delegate>` and
// `type PX = *mut <delegate>` stay faithful aliases (win32metadata collapses them); variadic
// callbacks stay opaque (cannot be a metadata delegate); the rest are unscraped-header breadth.
use std::collections::HashMap;
use windows_metadata::reader::{Index, TypeCategory, TypeDef};

fn load(path: &str) -> &'static Index {
    Box::leak(Box::new(Index::read(path).unwrap()))
}

fn by_name(index: &'static Index) -> HashMap<String, Vec<TypeDef<'static>>> {
    let mut m: HashMap<String, Vec<TypeDef>> = HashMap::new();
    for t in index.types() {
        if t.name() == "Apis" || t.flags().is_nested() {
            continue;
        }
        m.entry(t.name().to_string()).or_default().push(t);
    }
    m
}

// If `td` is a NativeTypedef alias (`type X = Y` or `type X = *mut Y`), return the
// target type's bare name (peeling any pointer indirection).
fn alias_target(td: &TypeDef) -> Option<String> {
    if td.category() != TypeCategory::Struct {
        return None;
    }
    let fields: Vec<_> = td.fields().collect();
    if fields.len() != 1 {
        return None;
    }
    fn peel(t: &windows_metadata::Type) -> Option<String> {
        match t {
            windows_metadata::Type::ClassName(tn) | windows_metadata::Type::ValueName(tn) => {
                Some(tn.name.clone())
            }
            windows_metadata::Type::PtrMut(inner, _)
            | windows_metadata::Type::PtrConst(inner, _) => peel(inner),
            _ => None,
        }
    }
    peel(&fields[0].ty())
}

// A single-field NativeTypedef whose field is a pointer to a primitive scalar, i.e. an
// opaque function-pointer alias (`type X = *mut u8`) — the faithful form for a variadic
// callback that cannot be expressed as a metadata delegate.
fn is_opaque_ptr_alias(td: &TypeDef) -> bool {
    if td.category() != TypeCategory::Struct {
        return false;
    }
    let fields: Vec<_> = td.fields().collect();
    if fields.len() != 1 {
        return false;
    }
    matches!(
        fields[0].ty(),
        windows_metadata::Type::PtrMut(_, _) | windows_metadata::Type::PtrConst(_, _)
    ) && alias_target(td).is_none()
}

#[test]
#[ignore = "requires win32metadata reference winmd + a full in-house build"]
fn delegate_gap() {
    let ours = load("../../../../target/win32/Windows.Win32.winmd");
    let theirs = load("../../../../crates/tools/package/reference/Windows.Win32.winmd");
    let ours_by = by_name(ours);
    let theirs_by = by_name(theirs);

    // Names of our delegates, for transitive alias resolution.
    let our_delegates: std::collections::HashSet<String> = ours_by
        .iter()
        .filter(|(_, v)| v.iter().any(|t| t.category() == TypeCategory::Delegate))
        .map(|(k, _)| k.clone())
        .collect();

    let resolves_to_delegate = |start: &str| -> bool {
        let mut cur = start.to_string();
        for _ in 0..16 {
            if our_delegates.contains(&cur) {
                return true;
            }
            let Some(defs) = ours_by.get(&cur) else {
                return false;
            };
            let mut next = None;
            for d in defs {
                if let Some(t) = alias_target(d) {
                    next = Some(t);
                    break;
                }
            }
            match next {
                Some(t) => cur = t,
                None => return false,
            }
        }
        false
    };

    let mut alias_to_delegate = vec![];
    let mut opaque_variadic = vec![];
    let mut real_struct = vec![];
    let mut missing = vec![];
    let mut total = 0;

    for (name, tdefs) in &theirs_by {
        if !tdefs.iter().any(|t| t.category() == TypeCategory::Delegate) {
            continue;
        }
        total += 1;
        let Some(ours_defs) = ours_by.get(name) else {
            missing.push(name.clone());
            continue;
        };
        if ours_defs
            .iter()
            .any(|t| t.category() == TypeCategory::Delegate)
        {
            continue; // match
        }
        // Not a delegate on our side. Classify.
        if resolves_to_delegate(name) {
            alias_to_delegate.push(name.clone());
        } else if ours_defs.iter().any(is_opaque_ptr_alias) {
            opaque_variadic.push(name.clone());
        } else {
            real_struct.push(name.clone());
        }
    }

    alias_to_delegate.sort();
    opaque_variadic.sort();
    real_struct.sort();
    missing.sort();

    println!("=== DELEGATE GAP (theirs=delegate) ===");
    println!("total theirs delegates: {total}");
    println!(
        "  ours delegate (match):        {}",
        total - alias_to_delegate.len() - opaque_variadic.len() - real_struct.len() - missing.len()
    );
    println!(
        "  ours alias->delegate:         {}",
        alias_to_delegate.len()
    );
    println!("  ours opaque variadic:         {}", opaque_variadic.len());
    println!("  ours real struct (BUG):       {}", real_struct.len());
    println!("  missing (unscraped headers):  {}", missing.len());
    println!("--- alias->delegate ---");
    for n in &alias_to_delegate {
        println!("  {n}");
    }
    println!("--- opaque variadic ---");
    for n in &opaque_variadic {
        println!("  {n}");
    }
    println!("--- real struct (genuine mis-emission) ---");
    for n in &real_struct {
        println!("  {n}");
    }
    println!("--- missing (sample) ---");
    for n in missing.iter().take(20) {
        println!("  {n}");
    }

    // Faithful handling: no function-pointer typedef should land as a plain struct.
    assert!(
        real_struct.is_empty(),
        "function-pointer typedefs mis-emitted as struct: {real_struct:?}"
    );
}
