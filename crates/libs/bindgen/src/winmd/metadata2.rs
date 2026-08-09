use super::*;
use windows_metadata2 as new;

#[test]
fn projected_item_multiplicities_match() {
    let database = new::Database::new([
        new::Image::new(windows_default::WINRT).unwrap(),
        new::Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut actual = Vec::new();

    for definition in database.definitions() {
        let namespace = definition.namespace().unwrap();
        let name = definition.name().unwrap();
        if !definition.is_windows_runtime().unwrap()
            && definition.category().unwrap() == new::TypeCategory::Class
            && name == "Apis"
        {
            actual.extend(
                definition
                    .methods()
                    .unwrap()
                    .map(|method| (namespace.to_string(), method.name().unwrap().to_string(), 1)),
            );
            actual.extend(
                definition
                    .fields()
                    .unwrap()
                    .map(|field| (namespace.to_string(), field.name().unwrap().to_string(), 2)),
            );
        } else {
            actual.push((
                namespace.to_string(),
                windows_metadata::trim_tick(name).to_string(),
                0,
            ));
        }
    }

    let index = windows_metadata::reader::Index::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
        File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let mut expected: Vec<_> = index
        .iter_items()
        .map(|(namespace, name, item)| {
            let kind = match item {
                windows_metadata::reader::Item::Type(_) => 0,
                windows_metadata::reader::Item::Fn(_) => 1,
                windows_metadata::reader::Item::Const(_) => 2,
            };
            (namespace.to_string(), name.to_string(), kind)
        })
        .collect();

    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn reader_selection_multiplicities_match() {
    let database = new::Database::new([
        new::Image::new(windows_default::WINRT).unwrap(),
        new::Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut actual = Vec::new();

    for definition in database.definitions() {
        let namespace = definition.namespace().unwrap();
        let raw_name = definition.name().unwrap();
        let name = windows_metadata::trim_tick(raw_name);
        if Type::remap(namespace, name) != Remap::None {
            continue;
        }

        let category = definition.category().unwrap();
        if !definition.is_windows_runtime().unwrap()
            && category == new::TypeCategory::Class
            && raw_name == "Apis"
        {
            actual.extend(
                definition
                    .methods()
                    .unwrap()
                    .map(|method| (namespace.to_string(), method.name().unwrap().to_string(), 0)),
            );
            actual.extend(
                definition
                    .fields()
                    .unwrap()
                    .map(|field| (namespace.to_string(), field.name().unwrap().to_string(), 10)),
            );
            continue;
        }

        let kind = if definition.is_windows_runtime().unwrap() {
            match category {
                new::TypeCategory::Attribute => continue,
                new::TypeCategory::Class => 1,
                new::TypeCategory::Delegate => 2,
                new::TypeCategory::Enum => 3,
                new::TypeCategory::Interface => 4,
                new::TypeCategory::Struct => {
                    if definition.has_attribute("ApiContractAttribute").unwrap() {
                        continue;
                    }
                    5
                }
            }
        } else {
            match category {
                new::TypeCategory::Attribute | new::TypeCategory::Class => continue,
                new::TypeCategory::Delegate => 6,
                new::TypeCategory::Enum => {
                    if !definition.has_attribute("ScopedEnumAttribute").unwrap() {
                        actual.extend(
                            definition
                                .fields()
                                .unwrap()
                                .filter(|field| field.is_literal().unwrap())
                                .map(|field| {
                                    (namespace.to_string(), field.name().unwrap().to_string(), 10)
                                }),
                        );
                    }
                    7
                }
                new::TypeCategory::Interface => 8,
                new::TypeCategory::Struct => 9,
            }
        };
        actual.push((namespace.to_string(), name.to_string(), kind));
    }

    let reader = Reader::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
        File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let mut expected = Vec::new();
    for (namespace, types) in reader.iter() {
        for (name, items) in types {
            expected.extend(items.iter().map(|item| {
                let kind = match item {
                    Type::CppFn(_) => 0,
                    Type::Class(_) => 1,
                    Type::Delegate(_) => 2,
                    Type::Enum(_) => 3,
                    Type::Interface(_) => 4,
                    Type::Struct(_) => 5,
                    Type::CppDelegate(_) => 6,
                    Type::CppEnum(_) => 7,
                    Type::CppInterface(_) => 8,
                    Type::CppStruct(_) => 9,
                    Type::CppConst(_) => 10,
                    _ => unreachable!(),
                };
                (namespace.to_string(), name.to_string(), kind)
            }));
        }
    }

    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}
