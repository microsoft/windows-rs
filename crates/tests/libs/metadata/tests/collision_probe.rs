use std::collections::BTreeMap;
use windows_metadata::reader::{Index, TypeCategory};

// Measures the true scale of the single-TU enum-constant collision ceiling against
// the win32metadata reference winmd. In C, enumerators live in the TU-global scope,
// so two distinct enums (in different headers) that declare the same enumerator name
// cannot be parsed in one translation unit. This probe counts, across the full
// reference surface, how many enumerator names are shared by 2+ distinct enum types
// and how many enum types are implicated.
#[ignore = "requires win32metadata reference winmd"]
#[test]
fn enum_constant_collisions() {
    let theirs = "../../../../crates/tools/package/reference/Windows.Win32.winmd";
    let index = Index::read(theirs).unwrap();

    // enumerator name -> set of "Namespace.EnumType" that declare it
    let mut by_const: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut enum_count = 0usize;
    for ty in index.types() {
        if ty.category() != TypeCategory::Enum {
            continue;
        }
        enum_count += 1;
        let owner = format!("{}.{}", ty.namespace(), ty.name());
        for f in ty.fields() {
            let n = f.name();
            if n == "value__" {
                continue;
            }
            by_const
                .entry(n.to_string())
                .or_default()
                .push(owner.clone());
        }
    }

    let mut colliding_names = 0usize;
    let mut max_fanout = 0usize;
    let mut implicated_enums = std::collections::BTreeSet::new();
    let mut worst: Vec<(String, usize)> = Vec::new();
    for (name, owners) in &by_const {
        // distinct owners only
        let mut distinct: Vec<&String> = owners.iter().collect();
        distinct.sort();
        distinct.dedup();
        if distinct.len() >= 2 {
            colliding_names += 1;
            max_fanout = max_fanout.max(distinct.len());
            worst.push((name.clone(), distinct.len()));
            for o in distinct {
                implicated_enums.insert(o.clone());
            }
        }
    }
    worst.sort_by_key(|(_, n)| std::cmp::Reverse(*n));

    println!("enum types in reference: {enum_count}");
    println!("distinct enumerator names: {}", by_const.len());
    println!("enumerator names shared by 2+ enums: {colliding_names}");
    println!(
        "enum types implicated in a collision: {}",
        implicated_enums.len()
    );
    println!("max fan-out (one name across N enums): {max_fanout}");
    println!("top 20 worst-colliding enumerator names:");
    for (name, n) in worst.iter().take(20) {
        println!("  {name}: {n} enums");
    }
}
