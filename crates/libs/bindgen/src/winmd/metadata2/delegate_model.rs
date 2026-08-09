use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct DelegateModel {
    namespace: String,
    name: String,
    generics: Vec<GenericParameterModel>,
    guid: GuidModel,
    invoke: MethodModel,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GenericParameterModel {
    sequence: u16,
    flags: u16,
    name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GuidModel {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct MethodModel {
    flags: u8,
    return_type: value_model::ModelType,
    return_parameter: Option<ParameterModel>,
    parameters: Vec<(value_model::ModelType, Option<ParameterModel>)>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ParameterModel {
    flags: u16,
    name: String,
}

impl DelegateModel {
    fn from_old(definition: windows_metadata::reader::TypeDef<'_>) -> Option<Self> {
        let generics: Vec<_> = definition
            .generic_params()
            .map(|parameter| GenericParameterModel {
                sequence: parameter.sequence(),
                flags: parameter.flags().0,
                name: parameter.name().to_string(),
            })
            .collect();
        let generic_types: Vec<_> = generics
            .iter()
            .map(|parameter| {
                windows_metadata::Type::Generic(parameter.name.clone(), parameter.sequence)
            })
            .collect();
        let invoke = definition
            .methods()
            .find(|method| method.name() == "Invoke")?;
        let signature = invoke.signature(&generic_types);
        let parameters = invoke.params_by_sequence(signature.types.len()).ok()?;
        Some(Self {
            namespace: definition.namespace().to_string(),
            name: windows_metadata::trim_tick(definition.name()).to_string(),
            generics,
            guid: GuidModel::from(old_guid_attribute(definition)?),
            invoke: MethodModel {
                flags: signature.flags.0,
                return_type: value_model::ModelType::from_old(signature.return_type)?,
                return_parameter: parameters.return_param().map(ParameterModel::from_old),
                parameters: signature
                    .types
                    .into_iter()
                    .zip(parameters.params())
                    .map(|(ty, parameter)| {
                        Some((
                            value_model::ModelType::from_old(ty)?,
                            parameter.map(ParameterModel::from_old),
                        ))
                    })
                    .collect::<Option<_>>()?,
            },
        })
    }

    fn from_new(database: &new::Database, definition: new::TypeDefinition<'_>) -> Option<Self> {
        let generics = definition
            .generic_parameters()
            .ok()?
            .map(|parameter| {
                Some(GenericParameterModel {
                    sequence: parameter.sequence().ok()?,
                    flags: parameter.flags().ok()?,
                    name: parameter.name().ok()?.to_string(),
                })
            })
            .collect::<Option<Vec<_>>>()?;
        let invoke = definition
            .methods()
            .ok()?
            .find(|method| method.name().ok() == Some("Invoke"))?;
        let signature = invoke.signature().ok()?;
        let parameters = invoke.parameters_by_sequence().ok()?;
        Some(Self {
            namespace: definition.namespace().ok()?.to_string(),
            name: windows_metadata::trim_tick(definition.name().ok()?).to_string(),
            generics,
            guid: GuidModel::from(guid_attribute(definition)?),
            invoke: MethodModel {
                flags: signature.flags,
                return_type: value_model::ModelType::from_new_type(
                    database,
                    definition.entity().file(),
                    &signature.return_type,
                )?,
                return_parameter: match parameters.return_parameter() {
                    Some(parameter) => Some(ParameterModel::from_new(parameter)?),
                    None => None,
                },
                parameters: signature
                    .parameters
                    .iter()
                    .zip(parameters.parameters())
                    .map(|(ty, parameter)| {
                        Some((
                            value_model::ModelType::from_new_type(
                                database,
                                definition.entity().file(),
                                ty,
                            )?,
                            match parameter {
                                Some(parameter) => Some(ParameterModel::from_new(*parameter)?),
                                None => None,
                            },
                        ))
                    })
                    .collect::<Option<_>>()?,
            },
        })
    }
}

impl From<GUID> for GuidModel {
    fn from(value: GUID) -> Self {
        Self {
            data1: value.0,
            data2: value.1,
            data3: value.2,
            data4: [
                value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10,
            ],
        }
    }
}

impl ParameterModel {
    fn from_old(parameter: windows_metadata::reader::MethodParam<'_>) -> Self {
        Self {
            flags: parameter.flags().0,
            name: parameter.name().to_string(),
        }
    }

    fn from_new(parameter: new::ParameterDefinition<'_>) -> Option<Self> {
        Some(Self {
            flags: parameter.flags().ok()?,
            name: parameter.name().ok()?.to_string(),
        })
    }
}

fn old_shapes(ty: &windows_metadata::Type, shapes: &mut BTreeSet<&'static str>) {
    match ty {
        windows_metadata::Type::Void => {
            shapes.insert("Void");
        }
        windows_metadata::Type::Bool
        | windows_metadata::Type::Char
        | windows_metadata::Type::I8
        | windows_metadata::Type::U8
        | windows_metadata::Type::I16
        | windows_metadata::Type::U16
        | windows_metadata::Type::I32
        | windows_metadata::Type::U32
        | windows_metadata::Type::I64
        | windows_metadata::Type::U64
        | windows_metadata::Type::F32
        | windows_metadata::Type::F64
        | windows_metadata::Type::ISize
        | windows_metadata::Type::USize => {
            shapes.insert("Primitive");
        }
        windows_metadata::Type::String => {
            shapes.insert("String");
        }
        windows_metadata::Type::Object => {
            shapes.insert("Object");
        }
        windows_metadata::Type::ClassName(name) => {
            shapes.insert("Class");
            if !name.generics.is_empty() {
                shapes.insert("GenericInstance");
            }
            for argument in &name.generics {
                old_shapes(argument, shapes);
            }
        }
        windows_metadata::Type::ValueName(name) => {
            shapes.insert("Value");
            if !name.generics.is_empty() {
                shapes.insert("GenericInstance");
            }
            for argument in &name.generics {
                old_shapes(argument, shapes);
            }
        }
        windows_metadata::Type::Array(element) => {
            shapes.insert("Vector");
            old_shapes(element, shapes);
        }
        windows_metadata::Type::Generic(_, _) => {
            shapes.insert("GenericType");
        }
        windows_metadata::Type::RefMut(element) => {
            shapes.insert("ByRef");
            old_shapes(element, shapes);
        }
        windows_metadata::Type::RefConst(element) => {
            shapes.insert("RefConst");
            old_shapes(element, shapes);
        }
        windows_metadata::Type::PtrMut(element, _)
        | windows_metadata::Type::PtrConst(element, _) => {
            shapes.insert("Pointer");
            old_shapes(element, shapes);
        }
        windows_metadata::Type::ArrayFixed(element, _) => {
            shapes.insert("Array");
            old_shapes(element, shapes);
        }
    }
}

fn new_shapes(ty: &new::Type, shapes: &mut BTreeSet<&'static str>) {
    if !ty.modifiers.is_empty() {
        shapes.insert("RefConst");
    }
    match &ty.kind {
        new::TypeKind::Void => {
            shapes.insert("Void");
        }
        new::TypeKind::Boolean
        | new::TypeKind::Char
        | new::TypeKind::I8
        | new::TypeKind::U8
        | new::TypeKind::I16
        | new::TypeKind::U16
        | new::TypeKind::I32
        | new::TypeKind::U32
        | new::TypeKind::I64
        | new::TypeKind::U64
        | new::TypeKind::F32
        | new::TypeKind::F64
        | new::TypeKind::ISize
        | new::TypeKind::USize => {
            shapes.insert("Primitive");
        }
        new::TypeKind::String => {
            shapes.insert("String");
        }
        new::TypeKind::Object => {
            shapes.insert("Object");
        }
        new::TypeKind::Value(_) => {
            shapes.insert("Value");
        }
        new::TypeKind::Class(_) => {
            shapes.insert("Class");
        }
        new::TypeKind::GenericType(_) => {
            shapes.insert("GenericType");
        }
        new::TypeKind::Pointer(element) => {
            shapes.insert("Pointer");
            new_shapes(element, shapes);
        }
        new::TypeKind::ByRef(element) => {
            shapes.insert("ByRef");
            new_shapes(element, shapes);
        }
        new::TypeKind::Array { element, .. } => {
            shapes.insert("Array");
            new_shapes(element, shapes);
        }
        new::TypeKind::Vector(element) => {
            shapes.insert("Vector");
            new_shapes(element, shapes);
        }
        new::TypeKind::GenericInstance { arguments, .. } => {
            shapes.insert("GenericInstance");
            for argument in arguments {
                new_shapes(argument, shapes);
            }
        }
        new::TypeKind::GenericMethod(_) => {
            shapes.insert("GenericMethod");
        }
        new::TypeKind::TypedReference => {
            shapes.insert("TypedReference");
        }
        new::TypeKind::FunctionPointer(signature) => {
            shapes.insert("FunctionPointer");
            new_shapes(&signature.return_type, shapes);
            for parameter in &signature.parameters {
                new_shapes(parameter, shapes);
            }
        }
        new::TypeKind::Pinned(element) => {
            shapes.insert("Pinned");
            new_shapes(element, shapes);
        }
    }
}

#[test]
fn delegate_signature_shape_inventory_matches() {
    let database = new::Database::new([new::Image::new(windows_default::WINRT).unwrap()]).unwrap();
    let mut actual = BTreeSet::new();
    let mut actual_count = 0;
    for definition in database
        .definitions()
        .filter(|definition| definition.category().unwrap() == new::TypeCategory::Delegate)
    {
        let invoke = definition
            .methods()
            .unwrap()
            .find(|method| method.name().unwrap() == "Invoke")
            .unwrap();
        let signature = invoke.signature().unwrap();
        new_shapes(&signature.return_type, &mut actual);
        for parameter in &signature.parameters {
            new_shapes(parameter, &mut actual);
        }
        actual_count += 1;
    }

    let old = windows_metadata::reader::Index::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
    ]);
    let mut expected = BTreeSet::new();
    let mut expected_count = 0;
    for (_, _, definition) in old.iter().filter(|(_, _, definition)| {
        definition.category() == windows_metadata::reader::TypeCategory::Delegate
    }) {
        let generics: Vec<_> = definition
            .generic_params()
            .map(|parameter| {
                windows_metadata::Type::Generic(parameter.name().to_string(), parameter.sequence())
            })
            .collect();
        let invoke = definition
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap();
        let signature = invoke.signature(&generics);
        old_shapes(&signature.return_type, &mut expected);
        for parameter in &signature.types {
            old_shapes(parameter, &mut expected);
        }
        expected_count += 1;
    }

    assert_eq!(actual_count, expected_count);
    assert_eq!(
        actual,
        BTreeSet::from([
            "Class",
            "GenericInstance",
            "GenericType",
            "Object",
            "Primitive",
            "String",
            "Value",
            "Vector",
            "Void",
        ])
    );
    assert_eq!(actual, expected);
}

#[test]
fn delegate_models_match() {
    let database = new::Database::new([new::Image::new(windows_default::WINRT).unwrap()]).unwrap();
    let mut actual: Vec<_> = database
        .definitions()
        .filter(|definition| definition.category().unwrap() == new::TypeCategory::Delegate)
        .map(|definition| DelegateModel::from_new(&database, definition).unwrap())
        .collect();

    let old = windows_metadata::reader::Index::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
    ]);
    let mut expected: Vec<_> = old
        .iter()
        .filter(|(_, _, definition)| {
            definition.category() == windows_metadata::reader::TypeCategory::Delegate
        })
        .map(|(_, _, definition)| DelegateModel::from_old(definition).unwrap())
        .collect();

    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
    assert!(actual.len() > 100);
}
