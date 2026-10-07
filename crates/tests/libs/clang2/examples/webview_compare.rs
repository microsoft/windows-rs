use std::collections::{BTreeMap, BTreeSet};
use windows_metadata::reader::{HasAttributes, Index};

fn facts(index: &Index) -> BTreeMap<String, String> {
    let mut facts = BTreeMap::new();
    for (namespace, name, ty) in index
        .iter()
        .filter(|(namespace, _, _)| *namespace == "WebView2")
    {
        let name = format!("{namespace}.{name}");
        facts.insert(format!("{name}/kind"), format!("{:?}", ty.category()));
        facts.insert(
            format!("{name}/repr"),
            format!("{:?}", ty.underlying_type()),
        );
        let mut attributes: Vec<_> = ty
            .attributes()
            .map(|attribute| format!("{} {:?}", attribute.name(), attribute.value()))
            .collect();
        attributes.sort();
        facts.insert(format!("{name}/attributes"), format!("{attributes:?}"));
        facts.insert(
            format!("{name}/guid"),
            format!(
                "{:?}",
                ty.find_attribute("GuidAttribute")
                    .map(|attribute| attribute.value())
            ),
        );
        facts.insert(
            format!("{name}/bases"),
            format!(
                "{:?}",
                ty.interface_impls()
                    .map(|base| base.interface(&[]))
                    .collect::<Vec<_>>()
            ),
        );
        for (slot, field) in ty.fields().enumerate() {
            facts.insert(
                format!("{name}/field/{slot}"),
                format!(
                    "{} {:?} {:?}",
                    field.name(),
                    field.ty(),
                    field.constant().map(|value| value.value())
                ),
            );
        }
        for (slot, method) in ty.methods().enumerate() {
            let key = format!("{name}/method/{slot}");
            let signature = method.signature(&[]);
            facts.insert(format!("{key}/attributes"), format!("{:?}", method.flags()));
            facts.insert(format!("{key}/name"), method.name().into());
            facts.insert(format!("{key}/signature"), format!("{signature:?}"));
            facts.insert(
                format!("{key}/convention"),
                method.calling_convention().into(),
            );
            facts.insert(
                format!("{key}/import"),
                format!(
                    "{:?}",
                    method
                        .impl_map()
                        .map(|map| (map.import_scope().name(), map.import_name()))
                ),
            );
            let parameters = method.params_by_sequence(signature.types.len()).unwrap();
            for (position, parameter) in parameters.params().iter().enumerate() {
                facts.insert(
                    format!("{key}/param/{position}"),
                    format!(
                        "{:?}",
                        parameter.map(|param| (
                            param.direction(),
                            param.is_optional(),
                            param.buffer_relationship(),
                            param.has_attribute("ComOutPtrAttribute"),
                        ))
                    ),
                );
            }
        }
    }
    facts
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [old, new] = args.as_slice() else {
        return Err("usage: webview_compare <old.winmd> <new.winmd>".into());
    };
    let old = facts(&Index::read(old).ok_or("invalid old metadata")?);
    let new = facts(&Index::read(new).ok_or("invalid new metadata")?);
    if old.is_empty() || new.is_empty() {
        return Err("both inputs must contain WebView2 metadata".into());
    }
    let keys: BTreeSet<_> = old.keys().chain(new.keys()).collect();
    let mut counts = BTreeMap::<&str, usize>::new();
    for key in keys {
        let before = old.get(key);
        let after = new.get(key);
        if before == after {
            continue;
        }
        let category = match (before, after) {
            (None, _) => "added",
            (_, None) => "removed",
            _ if key.contains("/field/") => "field",
            _ if key.contains("/param/") => "parameter",
            _ => key.rsplit('/').next().unwrap(),
        };
        *counts.entry(category).or_default() += 1;
        println!("{key}\n  old: {before:?}\n  new: {after:?}");
    }
    println!(
        "old_facts={} new_facts={} differences={counts:?}",
        old.len(),
        new.len()
    );
    if !counts.is_empty() {
        return Err("metadata differs; inspect and classify the reported differences".into());
    }
    Ok(())
}
