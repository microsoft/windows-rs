use super::*;
use windows_metadata2 as new;

mod delegate_model;
mod value_model;

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

fn guid_attribute(definition: new::TypeDefinition<'_>) -> Option<GUID> {
    fn u32_value(value: &new::AttributeValue) -> u32 {
        let new::AttributeValue::U32(value) = value else {
            panic!("GuidAttribute data1 is not u32");
        };
        *value
    }

    fn u16_value(value: &new::AttributeValue) -> u16 {
        let new::AttributeValue::U16(value) = value else {
            panic!("GuidAttribute value is not u16");
        };
        *value
    }

    fn u8_value(value: &new::AttributeValue) -> u8 {
        let new::AttributeValue::U8(value) = value else {
            panic!("GuidAttribute value is not u8");
        };
        *value
    }

    let attribute = definition.find_attribute("GuidAttribute").unwrap()?;
    let arguments = attribute.arguments(&()).unwrap();
    let values: Vec<_> = arguments
        .iter()
        .map(|argument| {
            let new::AttributeArgument::Fixed { value, .. } = argument else {
                panic!("GuidAttribute has a named argument");
            };
            value
        })
        .collect();
    let [
        data1,
        data2,
        data3,
        data4,
        data5,
        data6,
        data7,
        data8,
        data9,
        data10,
        data11,
    ] = values.as_slice()
    else {
        panic!("GuidAttribute does not have 11 arguments");
    };
    Some(GUID(
        u32_value(data1),
        u16_value(data2),
        u16_value(data3),
        u8_value(data4),
        u8_value(data5),
        u8_value(data6),
        u8_value(data7),
        u8_value(data8),
        u8_value(data9),
        u8_value(data10),
        u8_value(data11),
    ))
}

fn old_guid_attribute(definition: windows_metadata::reader::TypeDef<'_>) -> Option<GUID> {
    fn u32_value(value: &Value) -> u32 {
        let Value::U32(value) = value else {
            panic!("GuidAttribute data1 is not u32");
        };
        *value
    }

    fn u16_value(value: &Value) -> u16 {
        let Value::U16(value) = value else {
            panic!("GuidAttribute value is not u16");
        };
        *value
    }

    fn u8_value(value: &Value) -> u8 {
        let Value::U8(value) = value else {
            panic!("GuidAttribute value is not u8");
        };
        *value
    }

    let arguments = definition.find_attribute("GuidAttribute")?.value();
    let [
        data1,
        data2,
        data3,
        data4,
        data5,
        data6,
        data7,
        data8,
        data9,
        data10,
        data11,
    ] = arguments.as_slice()
    else {
        panic!("GuidAttribute does not have 11 arguments");
    };
    Some(GUID(
        u32_value(&data1.1),
        u16_value(&data2.1),
        u16_value(&data3.1),
        u8_value(&data4.1),
        u8_value(&data5.1),
        u8_value(&data6.1),
        u8_value(&data7.1),
        u8_value(&data8.1),
        u8_value(&data9.1),
        u8_value(&data10.1),
        u8_value(&data11.1),
    ))
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

#[test]
fn guid_attributes_match() {
    let database = new::Database::new([
        new::Image::new(windows_default::WINRT).unwrap(),
        new::Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let mut actual: Vec<_> = database
        .definitions()
        .filter_map(|definition| {
            Some((
                definition.namespace().unwrap().to_string(),
                definition.name().unwrap().to_string(),
                guid_attribute(definition)?.to_string(),
            ))
        })
        .collect();

    let old = windows_metadata::reader::Index::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
        File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let mut expected: Vec<_> = old
        .iter()
        .filter_map(|(_, _, definition)| {
            Some((
                definition.namespace().to_string(),
                definition.name().to_string(),
                old_guid_attribute(definition)?.to_string(),
            ))
        })
        .collect();

    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}
