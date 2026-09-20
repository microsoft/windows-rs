use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SCHEMA: &str = "crates/tools/reactor2/src/schema.toml";
const OUTPUT: &str = "crates/libs/reactor2/src/generated.rs";
const DECLARATIONS_OUTPUT: &str = "crates/libs/reactor2/src/generated_declarations.rs";
const NATIVE_OUTPUT: &str = "crates/libs/reactor2/src/native/generated.rs";
const WINMD: &str = "crates/tools/reactor/winmd";
const BINDINGS_BASE: &str = "crates/tools/reactor2/src/bindings_base.txt";
const BINDINGS_FILTER: &str = "crates/tools/reactor2/src/bindings.txt";
const BINDINGS_OUTPUT: &str = "crates/libs/reactor2/src/native/bindings.rs";

#[derive(Deserialize)]
struct Schema {
    #[serde(default)]
    attached_properties: Vec<AttachedProperty>,
    objects: Vec<Object>,
}

#[derive(Deserialize)]
struct AttachedProperty {
    name: String,
    owner: String,
    value: String,
}

#[derive(Deserialize)]
struct Object {
    name: String,
    category: String,
    native: String,
    #[serde(default)]
    key: bool,
    #[serde(default)]
    properties: Vec<Property>,
    #[serde(default)]
    relations: Vec<Relation>,
    #[serde(default)]
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Property {
    name: String,
    value: String,
    #[serde(default)]
    required: bool,
    default: Option<String>,
}

#[derive(Deserialize)]
struct Relation {
    name: String,
    native: Option<String>,
    child: String,
    cardinality: String,
    identity: String,
    realization: String,
    method: Option<String>,
    item: Option<String>,
}

#[derive(Deserialize)]
struct Event {
    name: String,
    value: String,
    observes: Option<String>,
}

fn main() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema: Schema = toml::from_str(&source).unwrap();
    validate(&schema);
    let metadata = tool_reactor::metadata::MetadataResolver::load(&workspace_path(WINMD));
    let generated = rustfmt(&generate(&schema, &metadata));
    let declarations = rustfmt(&generate_declarations(&schema, &metadata));
    let native = rustfmt(&generate_native(&schema, &metadata));
    let bindings = generate_bindings(&schema, &metadata);
    let output = workspace_path(OUTPUT);
    if fs::read_to_string(&output).ok().as_deref() != Some(&generated) {
        fs::write(output, generated).unwrap();
    }
    let declarations_output = workspace_path(DECLARATIONS_OUTPUT);
    if fs::read_to_string(&declarations_output).ok().as_deref() != Some(&declarations) {
        fs::write(declarations_output, declarations).unwrap();
    }
    let native_output = workspace_path(NATIVE_OUTPUT);
    if fs::read_to_string(&native_output).ok().as_deref() != Some(&native) {
        fs::write(native_output, native).unwrap();
    }
    let bindings_output = workspace_path(BINDINGS_FILTER);
    if fs::read_to_string(&bindings_output).ok().as_deref() != Some(&bindings) {
        fs::write(bindings_output, bindings).unwrap();
    }
    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(BINDINGS_OUTPUT))
        .minimal()
        .dead_code()
        .flat()
        .filter_file(workspace_path(BINDINGS_FILTER))
        .write();
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join(path)
}

fn validate(schema: &Schema) {
    let mut objects = BTreeSet::new();
    let mut property_types = BTreeMap::new();
    let categories = ["Visual", "Structural", "Data"];

    for property in &schema.attached_properties {
        assert_identifier(&property.name);
        assert_identifier(&property.value);
        assert!(
            property.owner.rsplit_once('.').is_some(),
            "invalid attached property owner"
        );
        assert!(
            property_types
                .insert(&property.name, &property.value)
                .is_none(),
            "duplicate attached property"
        );
    }

    for object in &schema.objects {
        assert_identifier(&object.name);
        assert!(
            object.native == "handwritten" || object.native.rsplit_once('.').is_some(),
            "invalid native type"
        );
        assert!(objects.insert(object.name.as_str()), "duplicate object");
        assert!(categories.contains(&object.category.as_str()));
        let mut properties = BTreeSet::new();
        for property in &object.properties {
            assert_identifier(&property.name);
            assert!(
                properties.insert(property.name.as_str()),
                "duplicate property"
            );
            assert_identifier(&property.value);
            if object.native != "handwritten" {
                assert!(
                    property.default.is_some(),
                    "{}.{} requires a default",
                    object.name,
                    property.name
                );
            }
            if let Some(previous) = property_types.insert(&property.name, &property.value) {
                assert_eq!(
                    previous, &property.value,
                    "conflicting property value types"
                );
            }
        }
    }
    validate_relations(schema);
}

fn generate_native(schema: &Schema, metadata: &tool_reactor::metadata::MetadataResolver) -> String {
    let objects = schema
        .objects
        .iter()
        .filter(|object| object.native != "handwritten")
        .collect::<Vec<_>>();
    let mut output = String::from("// This file is generated by tool-reactor2.\n");

    for object in &objects {
        for property in &object.properties {
            let method = format!("put_{}", property.name);
            metadata
                .resolve(&native_name(object), &format!("put_{}", property.name))
                .unwrap_or_else(|| {
                    panic!(
                        "cannot resolve {}.put_{}",
                        native_name(object),
                        property.name
                    )
                });
            let class = metadata
                .classify_param(&native_name(object), &method)
                .unwrap();
            let value = metadata
                .parameter_value(&native_name(object), &method)
                .unwrap();
            match property.value.as_str() {
                "String" => {
                    assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
                    assert_eq!(value, "Str");
                }
                "Bool" => {
                    assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
                    assert_eq!(value, "Bool");
                }
                "F64" => {
                    assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
                    assert_eq!(value, "F64");
                }
                "OptionalBool" => {
                    assert_eq!(class, tool_reactor::metadata::ParamClass::NullableBool);
                    assert_eq!(value, "Bool");
                }
                value => {
                    assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
                    let (name, variants) = metadata
                        .enum_info(&native_name(object), &method)
                        .unwrap_or_else(|| {
                            panic!("{}.{} is not an enum property", object.name, property.name)
                        });
                    assert_eq!(name, value);
                    assert!(!variants.is_empty());
                    if let Some(default) = &property.default {
                        assert!(
                            variants.contains(default),
                            "{}.{} has invalid default",
                            object.name,
                            property.name
                        );
                    }
                }
            }
        }
        for event in &object.events {
            assert!(matches!(event.value.as_str(), "F64" | "String" | "Unit"));
            metadata
                .resolve(&native_name(object), &format!("add_{}", event.name))
                .unwrap_or_else(|| {
                    panic!("cannot resolve {}.add_{}", native_name(object), event.name)
                });
            if let Some(observed) = &event.observes {
                let property = object
                    .properties
                    .iter()
                    .find(|property| property.name == *observed)
                    .unwrap_or_else(|| {
                        panic!(
                            "{}.{} observes missing property {observed}",
                            object.name, event.name
                        )
                    });
                metadata
                    .resolve(&native_name(object), &format!("get_{observed}"))
                    .unwrap_or_else(|| {
                        panic!("cannot resolve {}.get_{observed}", native_name(object))
                    });
                if event.value != "Unit" {
                    assert_eq!(event.value, property.value);
                }
            }
        }
        for relation in &object.relations {
            if relation.realization == "Owned" && relation.cardinality == "One" {
                let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                assert_eq!(
                    metadata.content_property(&object.native).as_deref(),
                    Some(native_relation),
                    "{} content relation does not match metadata",
                    object.name
                );
            } else if relation.realization == "Owned" && relation.cardinality == "Many" {
                assert_eq!(
                    metadata
                        .resolve(&native_name(object), "get_Children")
                        .map(tool_reactor::metadata::InterfaceRef::short_name),
                    Some("IPanel"),
                    "{} child collection is not a Panel.Children collection",
                    object.name
                );
            }
        }
    }

    for object in &objects {
        if !object.events.is_empty() {
            output.push_str(&format!("struct Generated{} {{\n", object.name));
            output.push_str(&format!("value: native::{},\n", native_name(object)));
            for event in &object.events {
                output.push_str(&format!(
                    "{}: Rc<RefCell<Native{}Event>>,\n_{}: windows_core::EventRevoker,\n",
                    snake_case(&event.name),
                    event.value,
                    snake_case(&event.name)
                ));
            }
            output.push_str("}\n");
        }
    }

    output.push_str("enum GeneratedHandle {\n");
    for object in &objects {
        let value = if object.events.is_empty() {
            format!("native::{}", native_name(object))
        } else {
            format!("Generated{}", object.name)
        };
        output.push_str(&format!("{}({value}),\n", object.name));
    }
    output.push_str("}\nimpl GeneratedHandle {\n");

    output.push_str(
                    "fn create(kind: ObjectType, object: ObjectId, event_queue: \
                     &Rc<NativeEventQueue>) -> Result<Option<Self>, WinUiError> {\nOk(Some(match kind {\n",
                );
    for object in &objects {
        if object.events.is_empty() {
            output.push_str(&format!(
                "ObjectType::{} => Self::{}(native::{}::new()?),\n",
                object.name,
                object.name,
                native_name(object)
            ));
        } else {
            output.push_str(&format!("ObjectType::{} => {{\n", object.name));
            output.push_str(&format!(
                "let value = native::{}::new()?;\n",
                native_name(object)
            ));
            for event in &object.events {
                let field = snake_case(&event.name);
                let interface = metadata
                    .resolve(&native_name(object), &format!("add_{}", event.name))
                    .unwrap()
                    .short_name();
                if let Some(observed) = &event.observes {
                    let property = object
                        .properties
                        .iter()
                        .find(|property| property.name == *observed)
                        .unwrap();
                    let observed_interface = metadata
                        .resolve(&native_name(object), &format!("get_{observed}"))
                        .unwrap()
                        .short_name();
                    output.push_str(&format!("let source_{field} = value.clone();\n"));
                    let read = match property.value.as_str() {
                        "F64" => format!(
                            "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed}())"
                        ),
                        "OptionalBool" => format!(
                            "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed}()).map(Some)"
                        ),
                        _ => unreachable!("unsupported generated observation type"),
                    };
                    output.push_str(&format!("let read_{field} = move || {read};\n"));
                }
                output.push_str(&format!(
                                "let {field} = Rc::new(RefCell::new(Native{}Event::default()));\n\
                                 let event_for_callback = Rc::clone(&{field});\n\
                                 let event_queue = Rc::clone(event_queue);\n\
                                 let revoker = value.cast::<native::{interface}>()?.{}(move |_, _| {{\n",
                                event.value, event.name
                            ));
                if let Some(observed) = &event.observes {
                    let property = object
                        .properties
                        .iter()
                        .find(|property| property.name == *observed)
                        .unwrap();
                    let variant = property.value.as_str();
                    output.push_str(&format!(
                        "let Ok(observed) = read_{field}() else {{ std::process::abort(); }};\n\
                         event_queue.observations.borrow_mut().push(Observation::SetProperty {{ \
                         object, property: Property {{ id: PropertyId::{observed}, \
                         value: PropertyValue::{variant}(observed) }} }});\n\
                         WinUiAdapter::schedule_event_wake(&event_queue);\n"
                    ));
                }
                match event.value.as_str() {
                    "F64" => output.push_str(&format!(
                        "WinUiAdapter::dispatch_f64(&event_for_callback, &event_queue, object, \
                         EventId::{}, observed);\n",
                        event.name
                    )),
                    "Unit" => output.push_str(&format!(
                        "WinUiAdapter::dispatch_unit(&event_for_callback, &event_queue, object, \
                                     EventId::{});\n",
                        event.name
                    )),
                    _ => unreachable!("unsupported generated native event"),
                }
                output.push_str("})?;\n");
                output.push_str(&format!("let _{field} = revoker;\n"));
            }
            output.push_str(&format!(
                "Self::{}(Generated{} {{ value,",
                object.name, object.name
            ));
            for event in &object.events {
                let field = snake_case(&event.name);
                output.push_str(&format!("{field}, _{field},"));
            }
            output.push_str("})\n}\n");
        }
    }
    output.push_str("_ => return Ok(None),\n}))\n}\n");

    output.push_str("fn kind(&self) -> ObjectType { match self {\n");
    for object in &objects {
        output.push_str(&format!(
            "Self::{}(_) => ObjectType::{},\n",
            object.name, object.name
        ));
    }
    output.push_str("} }\n");

    output
        .push_str("fn ui_element(&self) -> Result<native::UIElement, WinUiError> { match self {\n");
    for object in &objects {
        let value = if object.events.is_empty() {
            "value"
        } else {
            "value.value"
        };
        output.push_str(&format!(
            "Self::{}(value) => Ok({value}.cast()?),\n",
            object.name
        ));
    }
    output.push_str("} }\n");

    output.push_str(
        "fn panel_children(&self) -> Option<Result<native::UIElementCollection, WinUiError>> { \
                     match self {\n",
    );
    for object in &objects {
        if object
            .relations
            .iter()
            .any(|relation| relation.realization == "Owned" && relation.cardinality == "Many")
        {
            output.push_str(&format!(
                            "Self::{}(value) => Some(value.cast::<native::IPanel>()\
                             .map_err(Into::into).and_then(|value| value.Children().map_err(Into::into))),\n",
                            object.name
                        ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_attached_property(element: &native::UIElement, property: PropertyId, \
         value: Option<&PropertyValue>) -> Option<Result<(), WinUiError>> { match (property, value) {\n",
    );
    for property in &schema.attached_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        let name = property
            .name
            .strip_prefix(owner)
            .unwrap_or_else(|| panic!("{} must start with {owner}", property.name));
        let (variant, expression) = match property.value.as_str() {
            "F64" => ("F64", "*value"),
            _ => panic!("unsupported attached property value"),
        };
        output.push_str(&format!(
            "(PropertyId::{}, Some(PropertyValue::{variant}(value))) => \
             Some(element.cast::<native::FrameworkElement>().map_err(Into::into)\
             .and_then(|element| native::{owner}::Set{name}(&element, {expression})\
             .map_err(Into::into))),\n",
            property.name
        ));
        output.push_str(&format!(
            "(PropertyId::{}, None) => \
             Some(element.cast::<native::IDependencyObject>().map_err(Into::into)\
             .and_then(|element| native::{owner}::{name}Property().map_err(Into::into)\
             .and_then(|property| element.ClearValue(&property).map_err(Into::into)))),\n",
            property.name
        ));
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_property(&self, property: PropertyId, value: Option<&PropertyValue>) -> \
                     Option<Result<(), WinUiError>> { match (self, property, value) {\n",
    );
    for object in &objects {
        for property in &object.properties {
            let interface = metadata
                .resolve(&native_name(object), &format!("put_{}", property.name))
                .unwrap()
                .short_name();
            let target = if object.events.is_empty() {
                "object"
            } else {
                "object.value"
            };
            let clear = property_default(property);
            let (variant, expression) = match property.value.as_str() {
                "String" => ("String", "value.as_ref()"),
                "Bool" => ("Bool", "*value"),
                "F64" => ("F64", "*value"),
                "OptionalBool" => ("OptionalBool", "*value"),
                value => {
                    let (_, variants) = metadata
                        .enum_info(&native_name(object), &format!("put_{}", property.name))
                        .unwrap();
                    let arms = variants
                        .iter()
                        .map(|variant| format!("\"{variant}\" => native::{value}::{variant},"))
                        .collect::<String>();
                    output.push_str(&format!(
                        "(Self::{}(object), PropertyId::{}, None) => \
                         Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                         .and_then(|object| object.Set{}(native::{value}::{clear})\
                         .map_err(Into::into))),\n",
                        object.name, property.name, property.name
                    ));
                    output.push_str(&format!(
                        "(Self::{}(object), PropertyId::{}, \
                         Some(PropertyValue::Enum {{ kind: \"{value}\", variant }})) => \
                         Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                         .and_then(|object| object.Set{}(match *variant {{ {arms} \
                         _ => unreachable!(\"validated enum variant\") }}).map_err(Into::into))),\n",
                        object.name, property.name, property.name
                    ));
                    continue;
                }
            };
            output.push_str(&format!(
                "(Self::{}(object), PropertyId::{}, None) => \
                             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                             .and_then(|object| object.Set{}({clear}).map_err(Into::into))),\n",
                object.name, property.name, property.name
            ));
            output.push_str(&format!(
                            "(Self::{}(object), PropertyId::{}, Some(PropertyValue::{variant}(value))) => \
                             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                             .and_then(|object| object.Set{}({expression}).map_err(Into::into))),\n",
                            object.name, property.name, property.name
                        ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_events(&mut self, object_id: ObjectId, set: &[Event], clear: &[EventId]) -> \
                     Option<Result<(), WinUiError>> { match self {\n",
    );
    for object in &objects {
        if object.events.is_empty() {
            continue;
        }
        output.push_str(&format!("Self::{}(object) => Some((|| {{\n", object.name));
        for event in &object.events {
            let field = snake_case(&event.name);
            output.push_str(&format!(
                            "if clear.contains(&EventId::{}) {{ let mut native_event = \
                             object.{field}.borrow_mut(); native_event.revision = \
                             native_event.revision.wrapping_add(1); native_event.callback = None; }}\n",
                            event.name
                        ));
        }
        output.push_str("for event in set { match (event.id, &event.value) {\n");
        for event in &object.events {
            let field = snake_case(&event.name);
            output.push_str(&format!(
                "(EventId::{}, EventValue::{}(callback)) => {{ let mut native_event = \
                             object.{field}.borrow_mut(); native_event.revision = \
                             native_event.revision.wrapping_add(1); native_event.callback = \
                             Some(callback.clone()); }}\n",
                event.name, event.value
            ));
        }
        output.push_str("_ => return Err(WinUiError::InvalidObject(object_id)),\n} }\n");
        output.push_str("Ok(())\n})()),\n");
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_content(&self, relation: RelationId, child: Option<&native::UIElement>) -> \
                     Option<Result<(), WinUiError>> { match (self, relation) {\n",
    );
    for object in &objects {
        for relation in &object.relations {
            if relation.realization != "Owned" || relation.cardinality != "One" {
                continue;
            }
            let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
            let interface = metadata
                .resolve(&native_name(object), &format!("put_{native_relation}"))
                .unwrap()
                .short_name();
            let empty_type = metadata
                .parameter_type_name(&native_name(object), &format!("put_{native_relation}"))
                .unwrap();
            let empty_type = if empty_type == "IInspectable" {
                "IInspectable".to_string()
            } else {
                format!("native::{empty_type}")
            };
            let target = if object.events.is_empty() {
                "object"
            } else {
                "object.value"
            };
            output.push_str(&format!(
                            "(Self::{}(object), RelationId::{}) => Some({target}\
                             .cast::<native::{interface}>().map_err(Into::into).and_then(|object| \
                             match child {{ Some(child) => object.Set{}(child).map_err(Into::into), \
                             None => object.Set{}(None::<&{empty_type}>).map_err(Into::into) }})),\n",
                            object.name, relation.name, native_relation, native_relation
                        ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn event_callback(&self, event: EventId, revision: u64) -> Option<EventValue> { \
                     match (self, event) {\n",
    );
    for object in &objects {
        for event in &object.events {
            let field = snake_case(&event.name);
            output.push_str(&format!(
                "(Self::{}(object), EventId::{}) => {{ let event = object.{field}.borrow(); \
                             (event.revision == revision).then(|| event.callback.clone()).flatten()\
                             .map(EventValue::{}) }},\n",
                object.name, event.name, event.value
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn unit_event(&self, event: EventId) -> Option<&Rc<RefCell<NativeUnitEvent>>> { \
                     match (self, event) {\n",
    );
    for object in &objects {
        for event in &object.events {
            if event.value == "Unit" {
                output.push_str(&format!(
                    "(Self::{}(object), EventId::{}) => Some(&object.{}),\n",
                    object.name,
                    event.name,
                    snake_case(&event.name)
                ));
            }
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str("}\n");
    output
}

fn native_name(object: &Object) -> String {
    object.native.rsplit('.').next().unwrap().to_string()
}

fn generate_bindings(
    schema: &Schema,
    metadata: &tool_reactor::metadata::MetadataResolver,
) -> String {
    let mut output = fs::read_to_string(workspace_path(BINDINGS_BASE)).unwrap();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    let mut generated = BTreeSet::new();
    if !schema.attached_properties.is_empty() {
        generated.insert("Microsoft::UI::Xaml::IDependencyObject::ClearValue".to_string());
    }
    for property in &schema.attached_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        let name = property
            .name
            .strip_prefix(owner)
            .unwrap_or_else(|| panic!("{} must start with {owner}", property.name));
        let owner = binding_path(&property.owner);
        generated.insert(format!("{owner}::Get{name}"));
        generated.insert(format!("{owner}::Set{name}"));
        generated.insert(format!("{owner}::{name}Property"));
    }
    for object in schema
        .objects
        .iter()
        .filter(|object| object.native != "handwritten")
    {
        generated.insert(format!("{}::CreateInstance", binding_path(&object.native)));
        for property in &object.properties {
            let setter = format!("put_{}", property.name);
            let interface = metadata
                .resolve(&native_name(object), &setter)
                .unwrap()
                .full_path();
            generated.insert(format!("{}::{setter}", binding_path(&interface)));
            if let Some(interface) =
                metadata.resolve(&native_name(object), &format!("get_{}", property.name))
            {
                generated.insert(format!(
                    "{}::get_{}",
                    binding_path(&interface.full_path()),
                    property.name
                ));
            }
            if !is_builtin_value(&property.value) {
                let enum_path = metadata.enum_path(&native_name(object), &setter).unwrap();
                generated.insert(binding_path(&enum_path));
            }
        }
        for event in &object.events {
            let interface = metadata
                .resolve(&native_name(object), &format!("add_{}", event.name))
                .unwrap()
                .full_path();
            let interface = binding_path(&interface);
            generated.insert(format!("{interface}::add_{}", event.name));
            generated.insert(format!("{interface}::remove_{}", event.name));
            if let Some(observed) = &event.observes {
                let interface = metadata
                    .resolve(&native_name(object), &format!("get_{observed}"))
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::get_{observed}", binding_path(&interface)));
            }
        }
        for relation in &object.relations {
            if relation.realization == "Owned" && relation.cardinality == "One" {
                let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                let method = format!("put_{native_relation}");
                let interface = metadata
                    .resolve(&native_name(object), &method)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{method}", binding_path(&interface)));
            } else if relation.realization == "Owned" && relation.cardinality == "Many" {
                generated.insert("Microsoft::UI::Xaml::Controls::IPanel::get_Children".to_string());
            }
        }
    }
    for filter in generated {
        output.push_str(&filter);
        output.push('\n');
    }
    output
}

fn binding_path(value: &str) -> String {
    value.replace('.', "::")
}

fn validate_relations(schema: &Schema) {
    let categories = ["Visual", "Structural", "Data"];
    let cardinalities = ["One", "Many"];
    let identities = ["Positional", "Keyed"];
    let realizations = ["Owned", "Structural", "Container"];
    for object in &schema.objects {
        let mut relations = BTreeSet::new();
        for relation in &object.relations {
            assert_identifier(&relation.name);
            assert!(
                relations.insert(relation.name.as_str()),
                "duplicate relation"
            );
            assert!(categories.contains(&relation.child.as_str()));
            assert!(cardinalities.contains(&relation.cardinality.as_str()));
            assert!(identities.contains(&relation.identity.as_str()));
            assert!(realizations.contains(&relation.realization.as_str()));
            assert!(
                relation.cardinality != "One" || relation.identity == "Positional",
                "single relations must be positional"
            );
            match relation.realization.as_str() {
                "Owned" => assert_eq!(relation.child, "Visual"),
                "Structural" => assert_eq!(relation.child, "Structural"),
                "Container" => assert_eq!(relation.child, "Data"),
                _ => unreachable!(),
            }
        }
        let mut events = BTreeSet::new();
        for event in &object.events {
            assert_identifier(&event.name);
            assert!(events.insert(event.name.as_str()), "duplicate event");
            assert!(matches!(event.value.as_str(), "String" | "F64" | "Unit"));
        }
    }
}

fn assert_identifier(value: &str) {
    let mut characters = value.chars();
    assert!(
        characters
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
            && characters.all(|character| character == '_' || character.is_ascii_alphanumeric()),
        "invalid Rust identifier: {value}"
    );
    assert!(
        !matches!(
            value,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
        ),
        "Rust keyword cannot be used as an identifier: {value}"
    );
}

fn generate(schema: &Schema, metadata: &tool_reactor::metadata::MetadataResolver) -> String {
    let mut properties = BTreeMap::new();
    let mut relations = BTreeMap::new();
    let mut events = BTreeMap::new();
    for property in &schema.attached_properties {
        properties.insert(property.name.as_str(), property.value.as_str());
    }
    for object in &schema.objects {
        for property in &object.properties {
            properties
                .entry(property.name.as_str())
                .or_insert(property.value.as_str());
        }
        for relation in &object.relations {
            relations.insert(relation.name.as_str(), ());
        }
        for event in &object.events {
            events
                .entry(event.name.as_str())
                .or_insert(event.value.as_str());
        }
    }

    let mut output = String::from("// This file is generated by tool-reactor2.\n");
    emit_enum(
        &mut output,
        "ObjectType",
        schema.objects.iter().map(|object| object.name.as_str()),
    );
    emit_enum(&mut output, "PropertyId", properties.keys().copied());
    emit_enum(&mut output, "RelationId", relations.keys().copied());
    emit_enum(&mut output, "EventId", events.keys().copied());
    output.push_str(
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum ObjectCategory { Visual, Structural, Data }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Cardinality { One, Many }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Identity { Positional, Keyed }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Realization { Owned, Structural, Container }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct PropertyContract { pub id: PropertyId, pub value: ValueType }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct EventContract { pub id: EventId, pub value: ValueType }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum ValueType {\n\
             String,\n\
             Bool,\n\
             F64,\n\
             OptionalBool,\n\
             Enum { kind: &'static str, variants: &'static [&'static str] },\n\
             Unit,\n\
         }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct RelationContract {\n\
             pub id: RelationId,\n\
             pub child: ObjectCategory,\n\
             pub cardinality: Cardinality,\n\
             pub identity: Identity,\n\
             pub realization: Realization,\n\
         }\n",
    );

    output.push_str("pub fn object_category(kind: ObjectType) -> ObjectCategory { match kind {\n");
    for object in &schema.objects {
        output.push_str(&format!(
            "ObjectType::{} => ObjectCategory::{},\n",
            object.name, object.category
        ));
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn event_contracts(kind: ObjectType) -> &'static [EventContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for event in &object.events {
            output.push_str(&format!(
                "EventContract {{ id: EventId::{}, value: ValueType::{} }},",
                event.name, event.value
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn property_contracts(kind: ObjectType) -> &'static [PropertyContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        if object.category == "Visual" {
            for property in &schema.attached_properties {
                output.push_str(&format!(
                    "PropertyContract {{ id: PropertyId::{}, value: ValueType::{} }},",
                    property.name, property.value
                ));
            }
        }
        for property in &object.properties {
            let value = if is_builtin_value(&property.value) {
                format!("ValueType::{}", property.value)
            } else {
                let (_, variants) = metadata
                    .enum_info(&native_name(object), &format!("put_{}", property.name))
                    .unwrap();
                format!(
                    "ValueType::Enum {{ kind: \"{}\", variants: &[{}] }}",
                    property.value,
                    variants
                        .iter()
                        .map(|variant| format!("\"{variant}\","))
                        .collect::<String>()
                )
            };
            output.push_str(&format!(
                "PropertyContract {{ id: PropertyId::{}, value: {} }},",
                property.name, value
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn relation_contracts(kind: ObjectType) -> &'static [RelationContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for relation in &object.relations {
            output.push_str(&format!(
                "RelationContract {{ id: RelationId::{}, child: ObjectCategory::{}, \
                 cardinality: Cardinality::{}, identity: Identity::{}, \
                 realization: Realization::{} }},",
                relation.name,
                relation.child,
                relation.cardinality,
                relation.identity,
                relation.realization
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");
    output
}

fn generate_declarations(
    schema: &Schema,
    metadata: &tool_reactor::metadata::MetadataResolver,
) -> String {
    let mut output = String::from("// This file is generated by tool-reactor2.\n");
    let mut enums = BTreeMap::new();
    for object in &schema.objects {
        if object.native == "handwritten" {
            continue;
        }
        for property in &object.properties {
            if is_builtin_value(&property.value) {
                continue;
            }
            let (_, variants) = metadata
                .enum_info(&native_name(object), &format!("put_{}", property.name))
                .unwrap();
            if let Some(previous) = enums.insert(property.value.as_str(), variants.to_vec()) {
                assert_eq!(previous, variants, "conflicting enum definitions");
            }
        }
    }
    for (name, variants) in enums {
        output.push_str("#[non_exhaustive]\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n");
        output.push_str(&format!("pub enum {name} {{\n"));
        for variant in &variants {
            output.push_str(&format!("{variant},\n"));
        }
        output.push_str("}\n");
        output.push_str(&format!(
            "impl {name} {{ fn property_value(self) -> PropertyValue {{\n"
        ));
        output.push_str("match self {\n");
        for variant in &variants {
            output.push_str(&format!(
                "Self::{variant} => PropertyValue::Enum {{ kind: \"{name}\", \
                 variant: \"{variant}\" }},\n"
            ));
        }
        output.push_str("}\n} }\n");
    }
    for object in &schema.objects {
        output.push_str("#[derive(Clone, Debug, PartialEq)]\n");
        output.push_str(&format!("pub struct {}(Declaration);\n", object.name));

        output.push_str(&format!("impl {} {{\n", object.name));
        let mut arguments = Vec::new();
        if object.key {
            arguments.push("key: impl Into<Key>".to_string());
        }
        for property in object
            .properties
            .iter()
            .filter(|property| property.required)
        {
            arguments.push(property_argument(property));
        }
        output.push_str(&format!(
            "pub fn new({}) -> Self {{\n",
            arguments.join(", ")
        ));
        output.push_str(&format!(
            "let declaration = Declaration::new(ObjectType::{});\n",
            object.name
        ));
        if object.key {
            output.push_str("let declaration = declaration.key(key);\n");
        }
        for property in object
            .properties
            .iter()
            .filter(|property| property.required)
        {
            output.push_str(&format!(
                "let declaration = declaration.property(PropertyId::{}, {});\n",
                property.name,
                property_value(property, &snake_case(&property.name))
            ));
        }
        output.push_str("Self(declaration)\n}\n");

        for property in object
            .properties
            .iter()
            .filter(|property| !property.required)
        {
            let name = snake_case(&property.name);
            output.push_str(&format!(
                "pub fn {name}(mut self, {}) -> Self {{\n",
                property_argument(property)
            ));
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, {});\nself\n}}\n",
                property.name,
                property_value(property, &name)
            ));
        }

        if object.category == "Visual" {
            for property in &schema.attached_properties {
                let name = snake_case(&property.name);
                output.push_str(&format!(
                    "pub fn {name}(mut self, {name}: f64) -> Self {{\n"
                ));
                output.push_str(&format!(
                    "self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::F64({name}));\nself\n}}\n",
                    property.name
                ));
            }
        }

        for relation in &object.relations {
            let method = relation
                .method
                .clone()
                .unwrap_or_else(|| snake_case(&relation.name));
            let constructor = example_constructor(object);
            let invalid = match (
                relation.child.as_str(),
                relation.cardinality.as_str(),
                relation.identity.as_str(),
            ) {
                ("Visual", "One", _) => "TreeNode::new(\"node\", \"Node\")",
                ("Visual", "Many", "Keyed") => "TextBlock::new(\"Text\")",
                ("Visual", "Many", "Positional") => "keyed(\"text\", TextBlock::new(\"Text\"))",
                ("Structural", "Many", _) | ("Data", "Many", _) => "TextBlock::new(\"Text\")",
                _ => unreachable!("unsupported relation shape"),
            };
            let argument = if relation.cardinality == "One" {
                invalid.to_string()
            } else {
                format!("[{invalid}]")
            };
            output.push_str(&format!(
                "/// Rejects values outside this relation's generated type contract.\n\
                 ///\n\
                 /// ```compile_fail\n\
                 /// use windows_reactor2::*;\n\
                 /// let _ = {constructor}.{method}({argument});\n\
                 /// ```\n",
            ));
            match relation.cardinality.as_str() {
                "One" => {
                    output.push_str(&format!(
                        "pub fn {method}(mut self, content: impl Into<Visual>) -> Self {{\n"
                    ));
                    output.push_str(&format!(
                        "self.0 = self.0.relation(RelationId::{}, \
                         RelationValue::One(Some(Rc::new(content.into().0))));\nself\n}}\n",
                        relation.name
                    ));
                }
                "Many" => {
                    let item = relation.item.as_deref().unwrap_or_else(|| {
                        if relation.identity == "Keyed" {
                            "KeyedVisual"
                        } else {
                            "Visual"
                        }
                    });
                    let item = if item == object.name { "Self" } else { item };
                    let extract = match item {
                        "KeyedVisual" => "child.0.0",
                        "Visual" => "child.0",
                        _ => "DeclaredNode::Object(child.0)",
                    };
                    output.push_str(&format!(
                        "pub fn {method}(mut self, children: impl IntoIterator<Item = {item}>) -> \
                         Self {{\n"
                    ));
                    output.push_str(&format!(
                        "self.0 = self.0.relation(RelationId::{}, RelationValue::Many(Rc::new(\
                         children.into_iter().map(|child| {extract}).collect())));\nself\n}}\n",
                        relation.name
                    ));
                }
                _ => unreachable!(),
            }
        }

        for event in &object.events {
            let method = snake_case(&event.name);
            match event.value.as_str() {
                "String" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(Rc<str>) + 'static) -> Self {{ \
                         self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<Rc<str>>) -> Self \
                         {{ self.0 = self.0.event(EventId::{}, \
                         EventValue::String(callback)); self }}\n",
                        event.name
                    ));
                }
                "F64" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(f64) + 'static) -> Self {{ \
                         self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<f64>) -> Self {{ \
                         self.0 = self.0.event(EventId::{}, EventValue::F64(callback)); self }}\n",
                        event.name
                    ));
                }
                "Unit" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn() + 'static) -> Self {{ \
                         self.on_{method}_callback(Callback::new(move |()| callback())) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<()>) -> Self {{ \
                         self.0 = self.0.event(EventId::{}, EventValue::Unit(callback)); self }}\n",
                        event.name
                    ));
                }
                _ => unreachable!(),
            }
        }
        output.push_str("}\n");

        if !object.key && object.properties.iter().all(|property| !property.required) {
            output.push_str(&format!(
                "impl Default for {} {{ fn default() -> Self {{ Self::new() }} }}\n",
                object.name
            ));
        }
        if object.category == "Visual" {
            output.push_str(&format!(
                "impl From<{}> for Visual {{ fn from(value: {}) -> Self {{ \
                 Self(DeclaredNode::Object(value.0)) }} }}\n",
                object.name, object.name
            ));
        }
    }
    output
}

fn property_argument(property: &Property) -> String {
    let name = snake_case(&property.name);
    match property.value.as_str() {
        "String" => format!("{name}: impl Into<Rc<str>>"),
        "Bool" => format!("{name}: bool"),
        "F64" => format!("{name}: f64"),
        "OptionalBool" => format!("{name}: Option<bool>"),
        value => format!("{name}: {value}"),
    }
}

fn property_value(property: &Property, name: &str) -> String {
    match property.value.as_str() {
        "String" => format!("PropertyValue::String({name}.into())"),
        "Bool" => format!("PropertyValue::Bool({name})"),
        "F64" => format!("PropertyValue::F64({name})"),
        "OptionalBool" => format!("PropertyValue::OptionalBool({name})"),
        _ => format!("{name}.property_value()"),
    }
}

fn property_default(property: &Property) -> String {
    let value = property.default.as_deref().unwrap();
    match property.value.as_str() {
        "String" => format!("{value:?}"),
        "Bool" => value.to_string(),
        "F64" => value.to_string(),
        "OptionalBool" if value == "none" => "None".to_string(),
        "OptionalBool" => format!("Some({value})"),
        _ => value.to_string(),
    }
}

fn example_constructor(object: &Object) -> String {
    let mut arguments = Vec::new();
    if object.key {
        arguments.push("\"key\"".to_string());
    }
    for property in object
        .properties
        .iter()
        .filter(|property| property.required)
    {
        arguments.push(match property.value.as_str() {
            "String" => "\"Text\"".to_string(),
            "Bool" => "false".to_string(),
            "F64" => "0.0".to_string(),
            "OptionalBool" => "None".to_string(),
            value => format!("{value}::{}", property.default.as_deref().unwrap()),
        });
    }
    format!("{}::new({})", object.name, arguments.join(", "))
}

fn is_builtin_value(value: &str) -> bool {
    matches!(value, "String" | "Bool" | "F64" | "OptionalBool")
}

fn snake_case(value: &str) -> String {
    let mut output = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index != 0 {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else {
            output.push(character);
        }
    }
    output
}

fn emit_enum<'a>(output: &mut String, name: &str, values: impl Iterator<Item = &'a str>) {
    output.push_str(&format!(
        "#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]\n\
         pub enum {name} {{\n"
    ));
    for value in values {
        output.push_str(value);
        output.push_str(",\n");
    }
    output.push_str("}\n");
}

fn rustfmt(source: &str) -> String {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::io::Write::write_all(child.stdin.as_mut().unwrap(), source.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn checked_output_is_current() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema: Schema = toml::from_str(&source).unwrap();
    validate(&schema);
    let metadata = tool_reactor::metadata::MetadataResolver::load(&workspace_path(WINMD));
    assert_eq!(
        fs::read_to_string(workspace_path(OUTPUT)).unwrap(),
        rustfmt(&generate(&schema, &metadata))
    );
    assert_eq!(
        fs::read_to_string(workspace_path(DECLARATIONS_OUTPUT)).unwrap(),
        rustfmt(&generate_declarations(&schema, &metadata))
    );
    assert_eq!(
        fs::read_to_string(workspace_path(NATIVE_OUTPUT)).unwrap(),
        rustfmt(&generate_native(&schema, &metadata))
    );
    assert_eq!(
        fs::read_to_string(workspace_path(BINDINGS_FILTER)).unwrap(),
        generate_bindings(&schema, &metadata)
    );
}
