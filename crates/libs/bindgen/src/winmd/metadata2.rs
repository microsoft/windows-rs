use super::*;
use windows_metadata2 as new;

#[derive(Clone, Copy)]
enum ProjectedItem {
    CppFn(new::Entity<new::tables::MethodDef>),
    Class(new::Entity<new::tables::TypeDef>),
    Delegate(new::Entity<new::tables::TypeDef>),
    Enum(new::Entity<new::tables::TypeDef>),
    Interface(new::Entity<new::tables::TypeDef>),
    Struct(new::Entity<new::tables::TypeDef>),
    CppDelegate(new::Entity<new::tables::TypeDef>),
    CppEnum(new::Entity<new::tables::TypeDef>),
    CppInterface(new::Entity<new::tables::TypeDef>),
    CppStruct(new::Entity<new::tables::TypeDef>),
    CppConst(new::Entity<new::tables::Field>),
}

impl ProjectedItem {
    fn kind(self) -> u8 {
        match self {
            Self::CppFn(_) => 0,
            Self::Class(_) => 1,
            Self::Delegate(_) => 2,
            Self::Enum(_) => 3,
            Self::Interface(_) => 4,
            Self::Struct(_) => 5,
            Self::CppDelegate(_) => 6,
            Self::CppEnum(_) => 7,
            Self::CppInterface(_) => 8,
            Self::CppStruct(_) => 9,
            Self::CppConst(_) => 10,
        }
    }

    fn metadata_name(self, database: &new::Database) -> &str {
        match self {
            Self::CppFn(entity) => database.method(entity).unwrap().name().unwrap(),
            Self::Class(entity)
            | Self::Delegate(entity)
            | Self::Enum(entity)
            | Self::Interface(entity)
            | Self::Struct(entity)
            | Self::CppDelegate(entity)
            | Self::CppEnum(entity)
            | Self::CppInterface(entity)
            | Self::CppStruct(entity) => database.definition(entity).unwrap().name().unwrap(),
            Self::CppConst(entity) => database.field(entity).unwrap().name().unwrap(),
        }
    }
}

struct Reader2 {
    database: new::Database,
    map: HashMap<String, HashMap<String, Vec<ProjectedItem>>>,
}

impl Reader2 {
    fn new(database: new::Database) -> Self {
        let mut map: HashMap<String, HashMap<String, Vec<ProjectedItem>>> = HashMap::new();

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
                for method in definition.methods().unwrap() {
                    insert_item(
                        &mut map,
                        namespace,
                        method.name().unwrap(),
                        ProjectedItem::CppFn(method.entity()),
                    );
                }
                for field in definition.fields().unwrap() {
                    insert_item(
                        &mut map,
                        namespace,
                        field.name().unwrap(),
                        ProjectedItem::CppConst(field.entity()),
                    );
                }
                continue;
            }

            let item = if definition.is_windows_runtime().unwrap() {
                match category {
                    new::TypeCategory::Attribute => continue,
                    new::TypeCategory::Class => ProjectedItem::Class(definition.entity()),
                    new::TypeCategory::Delegate => ProjectedItem::Delegate(definition.entity()),
                    new::TypeCategory::Enum => ProjectedItem::Enum(definition.entity()),
                    new::TypeCategory::Interface => ProjectedItem::Interface(definition.entity()),
                    new::TypeCategory::Struct => {
                        if definition.has_attribute("ApiContractAttribute").unwrap() {
                            continue;
                        }
                        ProjectedItem::Struct(definition.entity())
                    }
                }
            } else {
                match category {
                    new::TypeCategory::Attribute | new::TypeCategory::Class => continue,
                    new::TypeCategory::Delegate => ProjectedItem::CppDelegate(definition.entity()),
                    new::TypeCategory::Enum => {
                        if !definition.has_attribute("ScopedEnumAttribute").unwrap() {
                            for field in definition
                                .fields()
                                .unwrap()
                                .filter(|field| field.is_literal().unwrap())
                            {
                                insert_item(
                                    &mut map,
                                    namespace,
                                    field.name().unwrap(),
                                    ProjectedItem::CppConst(field.entity()),
                                );
                            }
                        }
                        ProjectedItem::CppEnum(definition.entity())
                    }
                    new::TypeCategory::Interface => {
                        ProjectedItem::CppInterface(definition.entity())
                    }
                    new::TypeCategory::Struct => ProjectedItem::CppStruct(definition.entity()),
                }
            };
            insert_item(&mut map, namespace, name, item);
        }

        Self { database, map }
    }

    fn iter(&self) -> impl Iterator<Item = (&str, &str, ProjectedItem)> {
        self.map.iter().flat_map(|(namespace, types)| {
            types.iter().flat_map(move |(name, items)| {
                items
                    .iter()
                    .copied()
                    .map(move |item| (namespace.as_str(), name.as_str(), item))
            })
        })
    }

    fn database(&self) -> &new::Database {
        &self.database
    }

    fn with_full_name(&self, namespace: &str, name: &str) -> &[ProjectedItem] {
        self.map
            .get(namespace)
            .and_then(|types| types.get(windows_metadata::trim_tick(name)))
            .map_or(&[], Vec::as_slice)
    }
}

fn insert_item(
    map: &mut HashMap<String, HashMap<String, Vec<ProjectedItem>>>,
    namespace: &str,
    name: &str,
    item: ProjectedItem,
) {
    map.entry(namespace.to_string())
        .or_default()
        .entry(name.to_string())
        .or_default()
        .push(item);
}

#[test]
fn reader_selection_multiplicities_match() {
    let reader2 = Reader2::new(
        new::Database::new([
            new::Image::new(windows_default::WINRT).unwrap(),
            new::Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    );
    let mut actual: Vec<_> = reader2
        .iter()
        .map(|(namespace, name, item)| (namespace.to_string(), name.to_string(), item.kind()))
        .collect();
    assert_eq!(reader2.database().images().len(), 2);

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

#[test]
fn reader2_owns_entity_storage() {
    let reader = Reader2::new(
        new::Database::new([
            new::Image::new(windows_default::WINRT).unwrap(),
            new::Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap(),
    );
    let item = reader
        .with_full_name("Windows.Foundation", "Point")
        .first()
        .copied()
        .unwrap();
    let ProjectedItem::Struct(entity) = item else {
        panic!("Point is not a WinRT struct");
    };
    let definition = reader.database().definition(entity).unwrap();
    assert_eq!(definition.name().unwrap(), "Point");

    for (_, name, item) in reader.iter() {
        assert_eq!(
            windows_metadata::trim_tick(item.metadata_name(reader.database())),
            name
        );
    }
}
