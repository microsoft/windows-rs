mod heaps;
mod tables;
use super::*;

use std::collections::BTreeMap;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub fn write(_name: &str, _winrt: bool, items: &[Item], _assemblies: &[&str]) -> Vec<u8> {
    let mut references = heaps::References::default();
    let mut definitions = heaps::Definitions::default();

    // Build sorted list of definitions to write.
    items.iter().for_each(|item| definitions.insert(item));

    // Definitions are now staged, meaning they cannot change and each has a stable index.
    let definitions = definitions.stage();

    // Build sorted list of references used by definitions.
    for item in items {
        match item {
            Item::Struct(ty) => ty.fields.iter().for_each(|field| type_reference(&field.ty, &definitions, &mut references)),
            // TODO: find remaining type references
            _ => {}
        }
    }

    // References are now staged, meaning they cannot change and each has a stable index.
    let _references = references.stage();

    let mut _blobs = heaps::Blobs::default();
    let mut _string = heaps::Strings::default();

    // TODO: now that we have stable type indexes, walk the items and build blobs and index strings.

    todo!()
}

fn type_reference<'a>(ty: &'a Type, definitions: &heaps::StagedDefinitions, references: &mut heaps::References<'a>) {
    match ty {
        Type::Named((namespace, name)) => {
            if definitions.get(namespace, name).is_none() {
                references.insert(namespace, name);
            }
        }
        _ => {}
    }
}
