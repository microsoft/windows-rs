use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod parity;

const SCHEMA: &str = "crates/tools/reactor2/src/schema.toml";
const OUTPUT: &str = "crates/libs/reactor2/src/generated.rs";
const DECLARATIONS_OUTPUT: &str = "crates/libs/reactor2/src/generated_declarations.rs";
const NATIVE_OUTPUT: &str = "crates/libs/reactor2/src/native/generated.rs";
const WINMD: &str = "crates/tools/reactor/winmd";
const BINDINGS_BASE: &str = "crates/tools/reactor2/src/bindings_base.txt";
const BINDINGS_FILTER: &str = "crates/tools/reactor2/src/bindings.txt";
const BINDINGS_OUTPUT: &str = "crates/libs/reactor2/src/native/bindings.rs";

#[derive(Clone, Deserialize, Serialize)]
struct Schema {
    #[serde(default)]
    attached_properties: Vec<AttachedProperty>,
    #[serde(default)]
    visual_properties: Vec<VisualProperty>,
    #[serde(default)]
    capabilities: Capabilities,
    objects: Vec<Object>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Capabilities {
    #[serde(default)]
    layout_exit_transition: bool,
    #[serde(default)]
    enabled: Vec<String>,
    #[serde(default)]
    focus: Vec<String>,
    #[serde(default)]
    reference: Vec<String>,
    #[serde(default)]
    text_style: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct AttachedProperty {
    name: String,
    owner: String,
    native: String,
    value: String,
    #[serde(default)]
    flag: bool,
    default: Option<String>,
    validation: Option<String>,
    #[serde(default)]
    variants: Vec<String>,
    #[serde(default)]
    readback: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct VisualProperty {
    name: String,
    owner: String,
    value: String,
    default: Option<String>,
    #[serde(default)]
    readback: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct Object {
    name: String,
    category: String,
    native: String,
    native_type: Option<String>,
    #[serde(default)]
    key: bool,
    #[serde(default)]
    virtual_items: bool,
    #[serde(default)]
    properties: Vec<Property>,
    #[serde(default)]
    relations: Vec<Relation>,
    #[serde(default)]
    events: Vec<Event>,
    selection: Option<Selection>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Property {
    name: String,
    native: Option<String>,
    value: String,
    adapter: Option<String>,
    controlled: Option<String>,
    coerces: Option<String>,
    feedback: Option<String>,
    #[serde(default)]
    clear_feedback: bool,
    #[serde(default)]
    required: bool,
    default: Option<String>,
    validation: Option<String>,
    #[serde(default)]
    readback: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct Relation {
    name: String,
    native: Option<String>,
    child: String,
    #[serde(default)]
    allowed_objects: Vec<String>,
    cardinality: String,
    identity: String,
    realization: String,
    method: Option<String>,
    item: Option<String>,
    native_collection: Option<String>,
    native_item: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Event {
    name: String,
    field: Option<String>,
    value: String,
    observes: Option<String>,
    payload: Option<String>,
    payload_adapter: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Selection {
    relations: Vec<String>,
    item: String,
    selected_property: String,
    selected_item_property: String,
    event: String,
    event_item_source: String,
    event_args: Option<String>,
    payload_property: String,
}

fn main() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema: Schema = toml::from_str(&source).unwrap();
    let mut args = std::env::args().skip(1);
    if let Some(argument) = args.next() {
        assert!(args.next().is_none(), "unexpected additional argument");
        assert!(
            matches!(
                argument.as_str(),
                "--parity-report" | "--check-parity" | "--convert-old-schema"
            ),
            "unknown argument `{argument}`"
        );
        let old =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        if argument == "--convert-old-schema" {
            let metadata = tool_reactor::metadata::MetadataResolver::load(&workspace_path(WINMD));
            print!("{}", parity::convert(&old, &schema, &metadata).unwrap());
            return;
        }
        validate(&schema);
        let report = parity::compare(&old, &schema).unwrap();
        print!("{}", report.render());
        if argument == "--check-parity" && !report.is_complete() {
            std::process::exit(1);
        }
        return;
    }
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
        .implements([
            "Microsoft.UI.Xaml.IApplicationOverrides",
            "Microsoft.UI.Xaml.IElementFactory",
            "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
        ])
        .compose("Microsoft.UI.Xaml.Application")
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
        assert_identifier(&property.native);
        assert_identifier(&property.value);
        assert!(
            !property.flag || property.value == "Bool",
            "attached property flags must be Bool"
        );
        if let Some(validation) = &property.validation {
            assert!(
                validation_supported(validation, &property.value),
                "{} has unsupported validation {validation}",
                property.name
            );
        }
        assert_eq!(
            property.variants.is_empty(),
            is_builtin_value(&property.value),
            "attached enum variants must match the value type"
        );
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
    for property in &schema.visual_properties {
        assert_identifier(&property.name);
        assert_identifier(&property.value);
        assert!(
            property.owner.rsplit_once('.').is_some(),
            "invalid visual property owner"
        );
        assert!(
            property_types
                .insert(&property.name, &property.value)
                .is_none(),
            "duplicate visual property"
        );
    }

    for object in &schema.objects {
        assert_identifier(&object.name);
        assert!(
            object.native == "handwritten" || object.native.rsplit_once('.').is_some(),
            "invalid native type"
        );
        if object.native == "handwritten" && object.category == "Visual" {
            assert!(
                object
                    .native_type
                    .as_ref()
                    .is_some_and(|native| native.rsplit_once('.').is_some()),
                "{} requires a native_type",
                object.name
            );
        } else {
            assert!(
                object.native_type.is_none(),
                "{} has an unused native_type",
                object.name
            );
        }
        assert!(objects.insert(object.name.as_str()), "duplicate object");
        assert!(categories.contains(&object.category.as_str()));
        let mut properties = BTreeSet::new();
        for property in &object.properties {
            assert_identifier(&property.name);
            if let Some(native) = &property.native {
                assert_identifier(native);
            }
            if let Some(adapter) = &property.adapter {
                assert!(matches!(
                    adapter.as_str(),
                    "clock_identifier"
                        | "font_weight"
                        | "grid_columns"
                        | "grid_rows"
                        | "horizontal_content_alignment"
                        | "inspectable_string"
                        | "inspectable_string_list"
                        | "number_box_value"
                        | "rating_value"
                        | "rich_edit_text"
                        | "selection_index"
                        | "vertical_content_alignment"
                ));
            }
            if let Some(event) = &property.controlled {
                assert_identifier(event);
            }
            if let Some(event) = &property.coerces {
                assert_identifier(event);
            }
            if let Some(feedback) = &property.feedback {
                assert!(matches!(
                    feedback.as_str(),
                    "synchronous_exact" | "synchronous_normalized" | "deferred_exact"
                ));
                assert!(
                    property.controlled.is_some() || property.coerces.is_some(),
                    "{}.{} has feedback without an event",
                    object.name,
                    property.name
                );
            }
            assert!(
                properties.insert(property.name.as_str()),
                "duplicate property"
            );
            assert!(
                !schema
                    .visual_properties
                    .iter()
                    .any(|visual| visual.name == property.name),
                "{}.{} duplicates a visual property",
                object.name,
                property.name
            );
            assert_identifier(&property.value);
            if let Some(validation) = &property.validation {
                assert!(
                    validation_supported(validation, &property.value),
                    "{}.{} has unsupported validation {validation}",
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
        for property in object
            .properties
            .iter()
            .filter(|property| property.feedback.is_some())
        {
            let event = property
                .controlled
                .as_ref()
                .or(property.coerces.as_ref())
                .unwrap();
            assert!(
                object.events.iter().any(|candidate| {
                    candidate.name == *event
                        && candidate.observes.as_deref().is_some_and(|observed| {
                            property.coerces.is_some() || observed == property.name
                        })
                }),
                "{}.{} feedback event {event} must observe the property",
                object.name,
                property.name
            );
        }
        if let Some(selection) = &object.selection {
            assert!(
                !selection.relations.is_empty(),
                "{} has no selection relations",
                object.name
            );
            let item = schema
                .objects
                .iter()
                .find(|candidate| candidate.name == selection.item)
                .unwrap_or_else(|| {
                    panic!(
                        "{} selection item {} does not exist",
                        object.name, selection.item
                    )
                });
            let selected_property = item
                .properties
                .iter()
                .find(|property| property.name == selection.selected_property)
                .unwrap_or_else(|| {
                    panic!(
                        "{} selection item {} has no property {}",
                        object.name, selection.item, selection.selected_property
                    )
                });
            assert_eq!(selected_property.value, "Bool");
            let payload_property = item
                .properties
                .iter()
                .find(|property| property.name == selection.payload_property)
                .unwrap_or_else(|| {
                    panic!(
                        "{} selection item {} has no property {}",
                        object.name, selection.item, selection.payload_property
                    )
                });
            assert_eq!(payload_property.value, "String");
            for relation in &selection.relations {
                let relation = object
                    .relations
                    .iter()
                    .find(|candidate| candidate.name == *relation)
                    .unwrap_or_else(|| {
                        panic!(
                            "{} selection relation {relation} does not exist",
                            object.name
                        )
                    });
                assert_eq!(relation.cardinality, "Many");
                assert_eq!(relation.identity, "Keyed");
                assert_eq!(relation.realization, "Owned");
                assert!(
                    relation.allowed_objects.is_empty()
                        || relation.allowed_objects.contains(&selection.item)
                );
            }
            let event = object
                .events
                .iter()
                .find(|event| event.name == selection.event)
                .unwrap_or_else(|| {
                    panic!(
                        "{} selection event {} does not exist",
                        object.name, selection.event
                    )
                });
            assert_eq!(event.value, "Selection");
            assert!(event.observes.is_none());
            assert!(event.payload.is_none());
            assert!(matches!(
                selection.event_item_source.as_str(),
                "Owner" | "EventArgs"
            ));
            match selection.event_item_source.as_str() {
                "Owner" => assert!(selection.event_args.is_none()),
                "EventArgs" => {
                    assert!(selection.event_args.is_some());
                }
                _ => unreachable!(),
            }
        }
    }
    for (capability, members) in [
        ("enabled", &schema.capabilities.enabled),
        ("focus", &schema.capabilities.focus),
        ("reference", &schema.capabilities.reference),
        ("text_style", &schema.capabilities.text_style),
    ] {
        let mut unique = BTreeSet::new();
        for member in members {
            assert!(
                objects.contains(member.as_str()),
                "unknown {capability} capability object {member}"
            );
            assert!(
                unique.insert(member),
                "duplicate {capability} capability object {member}"
            );
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

    for property in &schema.attached_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        let (declaring_owner, _) = metadata
            .dependency_property(owner, &property.native)
            .unwrap_or_else(|| {
                panic!(
                    "cannot resolve {}.{} dependency property",
                    property.owner, property.native
                )
            });
        assert_eq!(
            declaring_owner, property.owner,
            "{}.{} has declaring owner {declaring_owner}",
            property.owner, property.native
        );
    }
    for property in &schema.visual_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        if property.value != "ThemeTransitions" {
            validate_native_property(
                metadata,
                owner,
                &property.name,
                &property.value,
                property.default.as_deref(),
                None,
            );
        }
        let (declaring_owner, _) = metadata
            .dependency_property(owner, &property.name)
            .unwrap_or_else(|| {
                panic!(
                    "cannot resolve {}.{} dependency property",
                    property.owner, property.name
                )
            });
        assert_eq!(
            declaring_owner, property.owner,
            "{}.{} has declaring owner {declaring_owner}",
            property.owner, property.name
        );
        for object in schema
            .objects
            .iter()
            .filter(|object| object.category == "Visual")
        {
            let native = if object.native == "handwritten" {
                object
                    .native_type
                    .as_ref()
                    .unwrap()
                    .rsplit('.')
                    .next()
                    .unwrap()
                    .to_string()
            } else {
                native_name(object)
            };
            assert!(
                metadata
                    .resolve(&native, &format!("put_{}", property.name))
                    .is_some(),
                "{} does not support visual property {}",
                object.name,
                property.name
            );
        }
    }

    for object in &objects {
        for property in &object.properties {
            let native = native_property(property);
            validate_native_property(
                metadata,
                &native_name(object),
                native,
                &property.value,
                property.default.as_deref(),
                property.adapter.as_deref(),
            );
        }
        for event in &object.events {
            assert!(matches!(
                event.value.as_str(),
                "Bool"
                    | "F64"
                    | "FontWeight"
                    | "OptionalBool"
                    | "OptionalF64"
                    | "PointerEventInfo"
                    | "Selection"
                    | "SelectionIndex"
                    | "String"
                    | "Unit"
            ));
            metadata
                .resolve(&native_name(object), &format!("add_{}", event.name))
                .unwrap_or_else(|| {
                    panic!("cannot resolve {}.add_{}", native_name(object), event.name)
                });
            if let Some(payload) = &event.payload {
                assert!(matches!(
                    event.payload_adapter.as_deref(),
                    None | Some("number_box_value" | "rating_value" | "selection_index")
                ));
                let (value, _, conversion) = metadata
                    .resolve_event_args_property(
                        &native_name(object),
                        &format!("add_{}", event.name),
                        payload,
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "cannot resolve {}.{} event payload {payload}",
                            native_name(object),
                            event.name
                        )
                    });
                assert_eq!(
                    conversion,
                    tool_reactor::metadata::ReadValueConversion::Identity
                );
                let expected = match event.payload_adapter.as_deref() {
                    Some("number_box_value" | "rating_value") => "OptionalF64",
                    Some("selection_index") => "SelectionIndex",
                    None if value == "Str" => "String",
                    None => value.as_str(),
                    _ => unreachable!(),
                };
                assert_eq!(event.value, expected);
            } else {
                assert!(event.payload_adapter.is_none());
            }
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
                let observed_native = native_property(property);
                metadata
                    .resolve(&native_name(object), &format!("get_{observed_native}"))
                    .unwrap_or_else(|| {
                        panic!(
                            "cannot resolve {}.get_{observed_native}",
                            native_name(object)
                        )
                    });
                if event.value != "Unit" {
                    assert_eq!(event.value, property.value);
                }
            }
        }
        for relation in &object.relations {
            if relation.realization == "Owned" && relation.cardinality == "One" {
                let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                metadata
                    .resolve(&native_name(object), &format!("put_{native_relation}"))
                    .unwrap_or_else(|| {
                        panic!(
                            "cannot resolve {}.put_{native_relation} for relation {}",
                            native_name(object),
                            relation.name
                        )
                    });
            } else if relation.realization == "Owned" && relation.cardinality == "Many" {
                if relation.name == "Children" && relation.native_item.is_none() {
                    assert_eq!(
                        metadata
                            .resolve(&native_name(object), "get_Children")
                            .map(tool_reactor::metadata::InterfaceRef::short_name),
                        Some("IPanel"),
                        "{} child collection is not a Panel.Children collection",
                        object.name
                    );
                } else {
                    let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                    let collection = metadata
                        .classify_collection(
                            &native_name(object),
                            &format!("get_{native_relation}"),
                        )
                        .unwrap_or_else(|| {
                            panic!(
                                "{}.{native_relation} is not a supported collection",
                                object.name
                            )
                        });
                    assert!(relation.native_item.is_some());
                    assert!(matches!(
                        (relation.native_collection.as_deref(), collection),
                        (
                            Some("Vector"),
                            tool_reactor::metadata::CollectionType::InspectableVector
                                | tool_reactor::metadata::CollectionType::TypedVector(_)
                        ) | (
                            Some("ObservableVector"),
                            tool_reactor::metadata::CollectionType::ObservableVector(_)
                        ) | (
                            Some("ItemCollection"),
                            tool_reactor::metadata::CollectionType::ItemCollection
                        )
                    ));
                }
            }
        }
        if let Some(selection) = &object.selection {
            metadata
                .resolve(
                    &native_name(object),
                    &format!("get_{}", selection.selected_item_property),
                )
                .unwrap_or_else(|| {
                    panic!(
                        "cannot resolve {}.get_{}",
                        native_name(object),
                        selection.selected_item_property
                    )
                });
            metadata
                .resolve(
                    &native_name(object),
                    &format!("put_{}", selection.selected_item_property),
                )
                .unwrap_or_else(|| {
                    panic!(
                        "cannot resolve {}.put_{}",
                        native_name(object),
                        selection.selected_item_property
                    )
                });
            let item = schema
                .objects
                .iter()
                .find(|candidate| candidate.name == selection.item)
                .unwrap();
            metadata
                .resolve(
                    &native_name(item),
                    &format!("get_{}", selection.selected_property),
                )
                .unwrap_or_else(|| {
                    panic!(
                        "cannot resolve {}.get_{}",
                        native_name(item),
                        selection.selected_property
                    )
                });
            metadata
                .resolve(
                    &native_name(item),
                    &format!("get_{}", selection.payload_property),
                )
                .unwrap_or_else(|| {
                    panic!(
                        "cannot resolve {}.get_{}",
                        native_name(item),
                        selection.payload_property
                    )
                });
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

    let collection_items = objects
        .iter()
        .flat_map(|object| &object.relations)
        .filter(|relation| {
            relation.realization == "Owned"
                && relation.cardinality == "Many"
                && relation.native_item.as_deref() != Some("IInspectable")
        })
        .filter_map(|relation| relation.native_item.as_deref())
        .collect::<BTreeSet<_>>();
    output.push_str(
        "enum GeneratedCollection {\n\
         Visual(native::UIElementCollection),\n\
         Inspectable(windows_collections::IVector<IInspectable>),\n",
    );
    for item in &collection_items {
        let item = item.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "{item}(windows_collections::IVector<native::{item}>),\n"
        ));
    }
    output.push_str("}\nimpl GeneratedCollection {\n");
    output.push_str(
        "fn size(&self) -> Result<u32, WinUiError> { match self { \
         Self::Visual(value) => value.Size().map_err(Into::into), \
         Self::Inspectable(value) => value.Size().map_err(Into::into),",
    );
    for item in &collection_items {
        let item = item.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "Self::{item}(value) => value.Size().map_err(Into::into),"
        ));
    }
    output.push_str("} }\n");
    output.push_str(
        "fn get_at(&self, index: u32) -> Result<IInspectable, WinUiError> { match self { \
         Self::Visual(value) => value.GetAt(index).map(Into::into).map_err(Into::into), \
         Self::Inspectable(value) => value.GetAt(index).map_err(Into::into),",
    );
    for item in &collection_items {
        let item = item.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "Self::{item}(value) => value.GetAt(index).map(Into::into).map_err(Into::into),"
        ));
    }
    output.push_str("} }\n");
    output.push_str(
        "fn insert_at(&self, index: u32, child: &IInspectable) -> Result<(), WinUiError> { \
         match self { Self::Visual(value) => value.InsertAt(index, \
         &child.cast::<native::UIElement>()?).map_err(Into::into), \
         Self::Inspectable(value) => value.InsertAt(index, child).map_err(Into::into),",
    );
    for item in &collection_items {
        let item = item.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "Self::{item}(value) => value.InsertAt(index, &child.cast::<native::{item}>()?)\
             .map_err(Into::into),"
        ));
    }
    output.push_str(
        "} }\n\
         fn remove_at(&self, index: u32) -> Result<(), WinUiError> { match self { \
         Self::Visual(value) => value.RemoveAt(index).map_err(Into::into), \
         Self::Inspectable(value) => value.RemoveAt(index).map_err(Into::into),",
    );
    for item in &collection_items {
        let item = item.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "Self::{item}(value) => value.RemoveAt(index).map_err(Into::into),"
        ));
    }
    output.push_str("} }\n}\n");

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
                let selection = object
                    .selection
                    .as_ref()
                    .filter(|selection| selection.event == event.name);
                let interface = metadata
                    .resolve(&native_name(object), &format!("add_{}", event.name))
                    .unwrap()
                    .short_name();
                if selection.is_none()
                    && let Some(observed) = &event.observes
                {
                    let property = object
                        .properties
                        .iter()
                        .find(|property| property.name == *observed)
                        .unwrap();
                    let observed_native = native_property(property);
                    let observed_interface = metadata
                        .resolve(&native_name(object), &format!("get_{observed_native}"))
                        .unwrap()
                        .short_name();
                    output.push_str(&format!("let source_{field} = value.clone();\n"));
                    let read = if property.adapter.as_deref() == Some("rich_edit_text") {
                        format!("read_rich_edit_text(&source_{field})")
                    } else {
                        match property.value.as_str() {
                            "String" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}()).map(Rc::<str>::from)"
                            ),
                            "Bool" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}())"
                            ),
                            "F64" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}())"
                            ),
                            "OptionalBool" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}()).map(Some)"
                            ),
                            "OptionalF64" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}())\
                             .map({})",
                                if property.adapter.as_deref() == Some("rating_value") {
                                    "rating_value"
                                } else {
                                    "number_box_value"
                                }
                            ),
                            "SelectionIndex" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}())\
                             .map(|value| usize::try_from(value).ok())"
                            ),
                            _ => unreachable!("unsupported generated observation type"),
                        }
                    };
                    output.push_str(&format!("let read_{field} = move || {read};\n"));
                }
                output.push_str(&format!(
                    "let {field} = Rc::new(RefCell::new(Native{}Event::default()));\n\
                     let event_for_callback = Rc::clone(&{field});\n\
                     let event_queue_{field} = Rc::clone(event_queue);\n",
                    event.value
                ));
                if let Some(selection) = selection {
                    if selection.event_item_source == "Owner" {
                        let selected_interface = metadata
                            .resolve(
                                &native_name(object),
                                &format!("get_{}", selection.selected_item_property),
                            )
                            .unwrap()
                            .short_name();
                        output.push_str(&format!(
                            "let source_{field} = value.cast::<native::{selected_interface}>()?;\n"
                        ));
                    }
                } else if event.value == "PointerEventInfo" {
                    output.push_str(&format!(
                        "let source_{field} = value.cast::<native::UIElement>()?;\n"
                    ));
                }
                let args = if event.value == "PointerEventInfo"
                    || event.payload.is_some()
                    || selection.is_some_and(|selection| selection.event_item_source == "EventArgs")
                {
                    "args"
                } else {
                    "_"
                };
                output.push_str(&format!(
                    "let revoker = value.cast::<native::{interface}>()?.{}(move |_, {args}| {{\n",
                    event.name
                ));
                if let Some(selection) = selection {
                    let selected = match selection.event_item_source.as_str() {
                        "Owner" => format!(
                            "source_{field}.{}().and_then(|selected| \
                             selected.cast::<IInspectable>())",
                            selection.selected_item_property
                        ),
                        "EventArgs" => {
                            output.push_str(
                                "let Some(args) = args.as_ref() else {\n\
                                 super::app::report_error(WinUiError::InvalidEventArgs.into());\n\
                                 return;\n\
                                 };\n",
                            );
                            format!(
                                "args.{}().and_then(|selected| \
                                 selected.cast::<IInspectable>())",
                                selection.selected_item_property
                            )
                        }
                        _ => unreachable!(),
                    };
                    output.push_str(&format!(
                        "WinUiAdapter::handle_selection_changed(\n\
                         &event_for_callback,\n\
                         &event_queue_{field},\n\
                         object,\n\
                         EventId::{},\n\
                         {selected},\n\
                         PropertyId::{},\n\
                         );\n",
                        event.name, selection.payload_property,
                    ));
                } else if let Some(observed) = &event.observes {
                    let property = object
                        .properties
                        .iter()
                        .find(|property| property.name == *observed)
                        .unwrap();
                    let variant = property.value.as_str();
                    let observed_value = if variant == "String" {
                        "Rc::clone(&observed)"
                    } else {
                        "observed"
                    };
                    output.push_str(&format!(
                        "let observed = match read_{field}() {{ Ok(value) => value, Err(error) => {{ \
                         super::app::report_error(error); return; }} }};\n\
                         let observation = Observation::SetProperty {{ \
                         object, property: Property {{ id: PropertyId::{observed}, \
                         value: PropertyValue::{variant}({observed_value}) }} }};\n\
                         let dispatch = event_queue_{field}.observe(object, EventId::{}, \
                         observation.clone());\n\
                         let observation = dispatch.then_some(observation);\n",
                        event.name
                    ));
                } else {
                    output.push_str("let dispatch = true;\nlet observation = None;\n");
                }
                if selection.is_none()
                    && let Some(payload) = &event.payload
                {
                    let conversion = match event.payload_adapter.as_deref() {
                        Some("number_box_value") => ".map(number_box_value)",
                        Some("rating_value") => ".map(rating_value)",
                        Some("selection_index") => ".map(|value| usize::try_from(value).ok())",
                        None if event.value == "String" => ".map(Rc::<str>::from)",
                        None => "",
                        _ => unreachable!(),
                    };
                    output.push_str(&format!(
                        "let Some(args) = args.as_ref() else {{ super::app::report_error(\
                         WinUiError::InvalidEventArgs.into()); return; }};\n\
                         let payload = match args.{payload}(){conversion} {{ Ok(value) => value, \
                         Err(error) => {{ super::app::report_error(error); return; }} }};\n"
                    ));
                }
                let dispatch_value = if event.payload.is_some() {
                    "payload"
                } else {
                    "observed"
                };
                if selection.is_some() {
                    output.push_str("})?;\n");
                    output.push_str(&format!("let _{field} = revoker;\n"));
                    continue;
                }
                match event.value.as_str() {
                    "Bool" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_bool(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "F64" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_f64(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "OptionalBool" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_optional_bool(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "OptionalF64" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_optional_f64(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "PointerEventInfo" => output.push_str(&format!(
                        "let value = match WinUiAdapter::pointer_event_info(&source_{field}, args) \
                         {{ Ok(value) => value, Err(error) => {{ \
                         super::app::report_error(error.into()); return; }} }};\n\
                         if dispatch {{ WinUiAdapter::dispatch_pointer_event_info(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, value); }}\n",
                        event.name
                    )),
                    "Unit" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_unit(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation); }}\n",
                        event.name
                    )),
                    "SelectionIndex" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_selection_index(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "String" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_string(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
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
        "fn owned_collection(&self, relation: RelationId) -> \
         Option<Result<GeneratedCollection, WinUiError>> { match (self, relation) {\n",
    );
    for object in &objects {
        for relation in &object.relations {
            if relation.realization != "Owned" || relation.cardinality != "Many" {
                continue;
            }
            let target = if object.events.is_empty() {
                "value"
            } else {
                "value.value"
            };
            if relation.name == "Children" && relation.native_item.is_none() {
                output.push_str(&format!(
                    "(Self::{}(value), RelationId::{}) => Some({target}.cast::<native::IPanel>()\
                     .map_err(Into::into).and_then(|value| value.Children().map(\
                     GeneratedCollection::Visual).map_err(Into::into))),\n",
                    object.name, relation.name
                ));
                continue;
            }
            let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
            let interface = metadata
                .resolve(&native_name(object), &format!("get_{native_relation}"))
                .unwrap()
                .short_name();
            let item = relation.native_item.as_deref().unwrap();
            let conversion = if relation.native_collection.as_deref() == Some("ItemCollection") {
                ".and_then(|value| value.cast::<windows_collections::IVector<IInspectable>>()\
                 .map(GeneratedCollection::Inspectable).map_err(Into::into))"
                    .to_string()
            } else if item == "IInspectable" {
                ".map(GeneratedCollection::Inspectable)".to_string()
            } else {
                let item = item.rsplit('.').next().unwrap();
                format!(
                    ".and_then(|value| value.cast::<windows_collections::IVector<native::{item}>>()\
                     .map(GeneratedCollection::{item}).map_err(Into::into))"
                )
            };
            output.push_str(&format!(
                "(Self::{}(value), RelationId::{}) => Some({target}.cast::<native::{interface}>()\
                 .map_err(Into::into).and_then(|value| value.{native_relation}()\
                 .map_err(Into::into)){conversion}),\n",
                object.name, relation.name
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn selected_item(&self, event: EventId) -> \
         Option<Result<Option<IInspectable>, WinUiError>> { match (self, event) {\n",
    );
    for object in &objects {
        let Some(selection) = &object.selection else {
            continue;
        };
        let target = if object.events.is_empty() {
            "object"
        } else {
            "object.value"
        };
        let interface = metadata
            .resolve(
                &native_name(object),
                &format!("get_{}", selection.selected_item_property),
            )
            .unwrap()
            .short_name();
        output.push_str(&format!(
            "(Self::{}(object), EventId::{}) => Some({target}.cast::<native::{interface}>()\
             .map_err(Into::into).and_then(|object| match object.{}() {{ \
             Ok(selected) => selected.cast::<IInspectable>().map(Some).map_err(Into::into), \
             Err(error) if error.code().is_ok() => Ok(None), Err(error) => Err(error.into()) }})),\n",
            object.name, selection.event, selection.selected_item_property
        ));
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_selected_item(&self, event: EventId, selected: Option<&IInspectable>) -> \
         Option<Result<(), WinUiError>> { match (self, event) {\n",
    );
    for object in &objects {
        let Some(selection) = &object.selection else {
            continue;
        };
        let target = if object.events.is_empty() {
            "object"
        } else {
            "object.value"
        };
        let interface = metadata
            .resolve(
                &native_name(object),
                &format!("put_{}", selection.selected_item_property),
            )
            .unwrap()
            .short_name();
        let item_type = metadata
            .parameter_type_name(
                &native_name(object),
                &format!("put_{}", selection.selected_item_property),
            )
            .unwrap();
        let set = if item_type == "IInspectable" {
            format!(
                "object.Set{}(selected).map_err(Into::into)",
                selection.selected_item_property
            )
        } else {
            format!(
                "match selected {{ Some(selected) => selected.cast::<native::{item_type}>()\
                 .and_then(|selected| object.Set{}(&selected)).map_err(Into::into), \
                 None => object.Set{}(None::<&native::{item_type}>).map_err(Into::into) }}",
                selection.selected_item_property, selection.selected_item_property
            )
        };
        output.push_str(&format!(
            "(Self::{}(object), EventId::{}) => Some({target}.cast::<native::{interface}>()\
             .map_err(Into::into).and_then(|object| {set})),\n",
            object.name, selection.event
        ));
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn selection_item_is_selected(&self, property: PropertyId) -> \
         Option<Result<bool, WinUiError>> { match (self, property) {\n",
    );
    for object in &objects {
        for owner in &objects {
            let Some(selection) = &owner.selection else {
                continue;
            };
            if selection.item != object.name {
                continue;
            }
            let target = if object.events.is_empty() {
                "object"
            } else {
                "object.value"
            };
            let interface = metadata
                .resolve(
                    &native_name(object),
                    &format!("get_{}", selection.selected_property),
                )
                .unwrap()
                .short_name();
            output.push_str(&format!(
                "(Self::{}(object), PropertyId::{}) => Some({target}.cast::<native::{interface}>()\
                 .map_err(Into::into).and_then(|object| object.{}().map_err(Into::into))),\n",
                object.name, selection.selected_property, selection.selected_property
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn selection_payload(property: PropertyId, item: &IInspectable) -> \
         Result<Option<Rc<str>>, WinUiError> { match property {\n",
    );
    let mut payload_properties = BTreeSet::new();
    for object in &objects {
        for owner in &objects {
            let Some(selection) = &owner.selection else {
                continue;
            };
            if selection.item != object.name {
                continue;
            }
            if !payload_properties.insert(selection.payload_property.as_str()) {
                continue;
            }
            let property = object
                .properties
                .iter()
                .find(|property| property.name == selection.payload_property)
                .unwrap();
            let native = native_property(property);
            let interface = metadata
                .resolve(&native_name(object), &format!("get_{native}"))
                .unwrap()
                .short_name();
            let read = if property.adapter.as_deref() == Some("inspectable_string") {
                format!(
                    "item.cast::<native::{interface}>().and_then(|item| item.{native}())\
                     .and_then(|value| value.cast::<windows_reference::IReference<HSTRING>>())\
                     .and_then(|value| value.Value()).map(|value| Rc::<str>::from(\
                     value.to_string_lossy()))"
                )
            } else {
                format!(
                    "item.cast::<native::{interface}>().and_then(|item| item.{native}())\
                     .map(Rc::<str>::from)"
                )
            };
            output.push_str(&format!(
                "PropertyId::{} => match {read} {{ Ok(value) => Ok(Some(value)), \
                 Err(error) if error.code().is_ok() => Ok(None), \
                 Err(error) => Err(error.into()) }},\n",
                selection.payload_property
            ));
        }
    }
    output.push_str("_ => Err(WinUiError::InvalidEventArgs),\n} }\n");

    output.push_str(
        "fn set_attached_property(element: &native::UIElement, property: PropertyId, \
         value: Option<&PropertyValue>) -> Option<Result<(), WinUiError>> { match (property, value) {\n",
    );
    for property in &schema.attached_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        let (variant, expression) = match property.value.as_str() {
            "F64" => ("F64", "*value"),
            "FontWeight" => ("FontWeight", "native::FontWeight { weight: value.value() }"),
            "I32" => ("I32", "*value"),
            "Bool" => ("Bool", "*value"),
            "String" => ("String", "value.as_ref()"),
            value => {
                let arms = property
                    .variants
                    .iter()
                    .map(|variant| format!("\"{variant}\" => native::{value}::{variant},"))
                    .collect::<String>();
                output.push_str(&format!(
                    "(PropertyId::{}, Some(PropertyValue::Enum {{ kind: \"{value}\", variant }})) \
                     => Some(element.cast::<native::FrameworkElement>().map_err(Into::into)\
                     .and_then(|element| {{ let _ = native::{value}::None; \
                     native::{owner}::Set{}(&element, match *variant {{ {arms} \
                     _ => unreachable!(\"validated enum variant\") }}).map_err(Into::into) }})),\n",
                    property.name, property.native
                ));
                if property.default.is_none() {
                    output.push_str(&format!(
                        "(PropertyId::{}, None) => \
                         Some(element.cast::<native::IDependencyObject>().map_err(Into::into)\
                         .and_then(|element| native::{owner}::{}Property().map_err(Into::into)\
                         .and_then(|property| element.ClearValue(&property).map_err(Into::into)))),\n",
                        property.name, property.native
                    ));
                }
                continue;
            }
        };
        output.push_str(&format!(
            "(PropertyId::{}, Some(PropertyValue::{variant}(value))) => \
             Some(element.cast::<native::FrameworkElement>().map_err(Into::into)\
             .and_then(|element| native::{owner}::Set{}(&element, {expression})\
             .map_err(Into::into))),\n",
            property.name, property.native
        ));
        if let Some(default) = property.default.as_deref() {
            let default = property_default_parts(&property.value, default);
            output.push_str(&format!(
                "(PropertyId::{}, None) => \
                 Some(element.cast::<native::FrameworkElement>().map_err(Into::into)\
                 .and_then(|element| native::{owner}::Set{}(&element, {default})\
                 .map_err(Into::into))),\n",
                property.name, property.native
            ));
        } else {
            output.push_str(&format!(
                "(PropertyId::{}, None) => \
                 Some(element.cast::<native::IDependencyObject>().map_err(Into::into)\
                 .and_then(|element| native::{owner}::{}Property().map_err(Into::into)\
                 .and_then(|property| element.ClearValue(&property).map_err(Into::into)))),\n",
                property.name, property.native
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_visual_property(element: &native::UIElement, property: PropertyId, \
         value: Option<&PropertyValue>) -> Option<Result<(), WinUiError>> { match (property, value) {\n",
    );
    if !schema.capabilities.enabled.is_empty() {
        emit_native_property_arms(
            &mut output,
            "(PropertyId::IsEnabled, ",
            "element",
            "IControl",
            "Control",
            "IsEnabled",
            "Bool",
            None,
            None,
            metadata,
        );
    }
    for property in &schema.visual_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        if property.value == "ThemeTransitions" {
            output.push_str(&format!(
                "(PropertyId::{}, Some(PropertyValue::ThemeTransitions(values))) => Some((|| {{\n\
                 let collection = native::TransitionCollection::new()?;\n\
                 for value in values.iter() {{\n\
                 let transition = match value {{\n\
                 crate::ThemeTransition::Reposition => native::RepositionThemeTransition::new()?\
                 .cast::<native::Transition>()?,\n\
                 }};\n\
                 collection.Append(&transition)?;\n\
                 }}\n\
                 element.cast::<native::I{owner}>()?.Set{}(&collection)?;\n\
                 Ok(())\n\
                 }})()),\n",
                property.name, property.name
            ));
            output.push_str(&format!(
                "(PropertyId::{}, None) => \
                 Some(element.cast::<native::IDependencyObject>().map_err(Into::into)\
                 .and_then(|element| native::{owner}::{}Property().map_err(Into::into)\
                 .and_then(|property| element.ClearValue(&property).map_err(Into::into)))),\n",
                property.name, property.name
            ));
        } else {
            emit_native_property_arms(
                &mut output,
                &format!("(PropertyId::{}, ", property.name),
                "element",
                &format!("I{owner}"),
                owner,
                &property.name,
                &property.value,
                property.default.as_deref(),
                None,
                metadata,
            );
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn feedback_expectation(kind: ObjectType, property: PropertyId, \
         value: Option<&PropertyValue>) -> Option<(EventId, FeedbackExpectation)> { \
         match (kind, property, value) {\n",
    );
    for object in &objects {
        for property in &object.properties {
            let Some(feedback) = &property.feedback else {
                continue;
            };
            let event = property
                .controlled
                .as_ref()
                .or(property.coerces.as_ref())
                .unwrap();
            match feedback.as_str() {
                "synchronous_exact" => {
                    output.push_str(&format!(
                        "(ObjectType::{}, PropertyId::{}, Some(value)) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::Exact(Property {{ id: PropertyId::{}, \
                         value: value.clone() }}))),\n",
                        object.name, property.name, property.name
                    ));
                    let clear = match property.value.as_str() {
                        "Bool" => format!("PropertyValue::Bool({})", property.clear_feedback),
                        "String" => "PropertyValue::String(Rc::from(\"\"))".to_string(),
                        "F64" => "PropertyValue::F64(0.0)".to_string(),
                        "I32" => "PropertyValue::I32(0)".to_string(),
                        "OptionalBool" => "PropertyValue::OptionalBool(None)".to_string(),
                        "OptionalF64" => "PropertyValue::OptionalF64(None)".to_string(),
                        "SelectionIndex" => {
                            output.push_str(&format!(
                                "(ObjectType::{}, PropertyId::{}, None) => \
                                 Some((EventId::{event}, \
                                 FeedbackExpectation::Normalized {{ observation: None }})),\n",
                                object.name, property.name
                            ));
                            continue;
                        }
                        value => panic!(
                            "{}.{} has unsupported exact feedback value {value}",
                            object.name, property.name
                        ),
                    };
                    output.push_str(&format!(
                        "(ObjectType::{}, PropertyId::{}, None) => Some((EventId::{event}, \
                         FeedbackExpectation::Exact(Property {{ id: PropertyId::{}, \
                         value: {clear} }}))),\n",
                        object.name, property.name, property.name
                    ));
                }
                "synchronous_normalized" => output.push_str(&format!(
                    "(ObjectType::{}, PropertyId::{}, _) => Some((EventId::{event}, \
                     FeedbackExpectation::Normalized {{ observation: None }})),\n",
                    object.name, property.name
                )),
                "deferred_exact" => {
                    assert_eq!(property.value, "String");
                    output.push_str(&format!(
                        "(ObjectType::{}, PropertyId::{}, Some(value)) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::DeferredExact(Property {{ id: PropertyId::{}, \
                         value: value.clone() }}))),\n\
                         (ObjectType::{}, PropertyId::{}, None) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::DeferredExact(Property {{ id: PropertyId::{}, \
                         value: PropertyValue::String(Rc::from(\"\")) }}))),\n",
                        object.name,
                        property.name,
                        property.name,
                        object.name,
                        property.name,
                        property.name
                    ));
                }
                _ => unreachable!(),
            }
        }
    }
    output.push_str("_ => None,\n} }\n");

    output.push_str(
        "fn set_property(&self, property: PropertyId, value: Option<&PropertyValue>) -> \
                     Option<Result<(), WinUiError>> { match (self, property, value) {\n",
    );
    for object in &objects {
        for property in &object.properties {
            let native = native_property(property);
            let method = if matches!(
                property.adapter.as_deref(),
                Some("rich_edit_text" | "grid_rows" | "grid_columns")
            ) {
                format!("get_{native}")
            } else {
                format!("put_{native}")
            };
            let interface = metadata
                .resolve(&native_name(object), &method)
                .unwrap()
                .short_name();
            let target = if object.events.is_empty() {
                "object"
            } else {
                "object.value"
            };
            emit_native_property_arms(
                &mut output,
                &format!(
                    "(Self::{}(object), PropertyId::{}, ",
                    object.name, property.name
                ),
                target,
                interface,
                &native_name(object),
                native,
                &property.value,
                property.default.as_deref(),
                property.adapter.as_deref(),
                metadata,
            );
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
            let set = if let Some(item) = &relation.native_item {
                let item = item.rsplit('.').next().unwrap();
                format!(
                    "match child {{ Some(child) => object.Set{native_relation}(\
                     &child.cast::<native::{item}>()?).map_err(Into::into), \
                     None => object.Set{native_relation}(None::<&{empty_type}>).map_err(Into::into) }}"
                )
            } else {
                format!(
                    "match child {{ Some(child) => object.Set{native_relation}(child)\
                     .map_err(Into::into), None => object.Set{native_relation}(\
                     None::<&{empty_type}>).map_err(Into::into) }}"
                )
            };
            output.push_str(&format!(
                "(Self::{}(object), RelationId::{}) => Some({target}\
                 .cast::<native::{interface}>().map_err(Into::into)\
                 .and_then(|object| {set})),\n",
                object.name, relation.name
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

fn validate_native_property(
    metadata: &tool_reactor::metadata::MetadataResolver,
    owner: &str,
    native: &str,
    value: &str,
    default: Option<&str>,
    adapter: Option<&str>,
) {
    if adapter == Some("rich_edit_text") {
        assert_eq!(value, "String");
        assert!(default.is_none());
        metadata
            .resolve(owner, &format!("get_{native}"))
            .unwrap_or_else(|| panic!("cannot resolve {owner}.get_{native}"));
        return;
    }
    if matches!(adapter, Some("grid_rows" | "grid_columns")) {
        assert_eq!(value, "GridLengths");
        assert!(default.is_none());
        metadata
            .resolve(owner, &format!("get_{native}"))
            .unwrap_or_else(|| panic!("cannot resolve {owner}.get_{native}"));
        return;
    }
    let method = format!("put_{native}");
    metadata
        .resolve(owner, &method)
        .unwrap_or_else(|| panic!("cannot resolve {owner}.{method}"));
    let class = metadata.classify_param(owner, &method).unwrap();
    let metadata_value = if value == "Color" {
        metadata
            .parameter_type_name(owner, &method)
            .unwrap()
            .to_string()
    } else {
        metadata.parameter_value(owner, &method).unwrap()
    };
    match value {
        "StringList" => {
            assert_eq!(adapter, Some("inspectable_string_list"));
            assert_eq!(class, tool_reactor::metadata::ParamClass::IInspectable);
        }
        "SelectionIndex" => {
            assert_eq!(adapter, Some("selection_index"));
            assert_eq!(metadata_value, "I32");
        }
        "OptionalF64" => {
            assert!(matches!(adapter, Some("number_box_value" | "rating_value")));
            assert_eq!(metadata_value, "F64");
        }
        "String" => {
            if adapter == Some("inspectable_string") {
                assert_eq!(class, tool_reactor::metadata::ParamClass::IInspectable);
            } else {
                assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
                assert_eq!(metadata_value, "Str");
            }
        }
        "Bool" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
            assert_eq!(metadata_value, "Bool");
        }
        "Color" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
            assert_eq!(metadata_value, "Brush");
        }
        "CornerRadius" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
            assert_eq!(metadata_value, "CornerRadius");
        }
        "F64" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
            assert_eq!(metadata_value, "F64");
        }
        "FontWeight" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
            assert_eq!(metadata_value, "U16");
        }
        "I32" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Primitive);
            assert_eq!(metadata_value, "I32");
        }
        "OptionalBool" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::NullableBool);
            assert_eq!(metadata_value, "Bool");
        }
        "Thickness" => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
            assert_eq!(metadata_value, "Thickness");
        }
        value => {
            assert_eq!(class, tool_reactor::metadata::ParamClass::Complex);
            let (name, variants) = metadata
                .enum_info(owner, &method)
                .unwrap_or_else(|| panic!("{owner}.{native} is not an enum property"));
            assert_eq!(name, value);
            assert!(!variants.is_empty());
            if let Some(default) = default {
                assert!(
                    variants.contains(&default.to_string()),
                    "{owner}.{native} has invalid default"
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_native_property_arms(
    output: &mut String,
    pattern: &str,
    target: &str,
    interface: &str,
    metadata_owner: &str,
    native: &str,
    value: &str,
    default: Option<&str>,
    adapter: Option<&str>,
    metadata: &tool_reactor::metadata::MetadataResolver,
) {
    if adapter == Some("rich_edit_text") {
        output.push_str(&format!(
            "{pattern}None) => Some(set_rich_edit_text(&{target}, \"\")),\n\
             {pattern}Some(PropertyValue::String(value))) => \
             Some(set_rich_edit_text(&{target}, value)),\n"
        ));
        return;
    }
    if matches!(adapter, Some("grid_rows" | "grid_columns")) {
        let rows = adapter == Some("grid_rows");
        output.push_str(&format!(
            "{pattern}None) => Some(set_grid_definitions({target}, &[], {rows})),\n\
             {pattern}Some(PropertyValue::GridLengths(value))) => \
             Some(set_grid_definitions({target}, value, {rows})),\n"
        ));
        return;
    }
    let clear = default.map(|default| property_default_parts(value, default));
    if let Some(clear) = &clear {
        output.push_str(&format!(
            "{pattern}None) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| object.Set{native}({clear}).map_err(Into::into))),\n"
        ));
    } else {
        let (declaring_class, _) = metadata
            .dependency_property(metadata_owner, native)
            .unwrap_or_else(|| {
                panic!("cannot resolve {metadata_owner}.{native} dependency property")
            });
        let declaring_class = declaring_class.rsplit('.').next().unwrap();
        output.push_str(&format!(
            "{pattern}None) => \
             Some({target}.cast::<native::IDependencyObject>().map_err(Into::into)\
             .and_then(|object| native::{declaring_class}::{native}Property().map_err(Into::into)\
             .and_then(|property| object.ClearValue(&property).map_err(Into::into)))),\n"
        ));
    }
    if value == "Color" {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::Color(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| solid_color_brush(*value)\
             .and_then(|brush| object.Set{native}(&brush).map_err(Into::into)))),\n"
        ));
        return;
    }
    if adapter == Some("inspectable_string") {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::String(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| {{ let value: IInspectable = \
             windows_reference::IReference::from(value.as_ref()).into(); \
             object.Set{native}(&value).map_err(Into::into) }})),\n"
        ));
        return;
    }
    if adapter == Some("inspectable_string_list") {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::StringList(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| {{ let values: Vec<Option<IInspectable>> = value.iter()\
             .map(|value| Some(windows_reference::IReference::from(value.as_ref()).into()))\
             .collect(); let values: windows_collections::IVector<IInspectable> = values.into(); \
             object.Set{native}(&values).map_err(Into::into) }})),\n"
        ));
        return;
    }
    if adapter == Some("selection_index") {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::SelectionIndex(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| {{ let value = native_selection_index(*value)?; \
             object.Set{native}(value).map_err(Into::into) }})),\n"
        ));
        return;
    }
    if adapter == Some("number_box_value") {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::OptionalF64(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| object.Set{native}(native_number_box_value(*value))\
             .map_err(Into::into))),\n"
        ));
        return;
    }
    if adapter == Some("rating_value") {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::OptionalF64(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| object.Set{native}(native_rating_value(*value))\
             .map_err(Into::into))),\n"
        ));
        return;
    }
    let (variant, expression) = match value {
        "String" => ("String", "value.as_ref()"),
        "Bool" => ("Bool", "*value"),
        "F64" => ("F64", "*value"),
        "FontWeight" => ("FontWeight", "native::FontWeight { weight: value.value() }"),
        "I32" => ("I32", "*value"),
        "OptionalBool" => ("OptionalBool", "*value"),
        "CornerRadius" => (
            "CornerRadius",
            "native::CornerRadius { top_left: value.top_left, top_right: value.top_right, \
             bottom_right: value.bottom_right, bottom_left: value.bottom_left }",
        ),
        "Thickness" => (
            "Thickness",
            "native::Thickness { left: value.left, top: value.top, right: value.right, \
             bottom: value.bottom }",
        ),
        value => {
            let (_, variants) = metadata
                .enum_info(metadata_owner, &format!("put_{native}"))
                .unwrap();
            let arms = variants
                .iter()
                .map(|variant| format!("\"{variant}\" => native::{value}::{variant},"))
                .collect::<String>();
            output.push_str(&format!(
                "{pattern}Some(PropertyValue::Enum {{ kind: \"{value}\", variant }})) => \
                 Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                 .and_then(|object| object.Set{native}(match *variant {{ {arms} \
                 _ => unreachable!(\"validated enum variant\") }}).map_err(Into::into))),\n"
            ));
            return;
        }
    };
    output.push_str(&format!(
        "{pattern}Some(PropertyValue::{variant}(value))) => \
         Some({target}.cast::<native::{interface}>().map_err(Into::into)\
         .and_then(|object| object.Set{native}({expression}).map_err(Into::into))),\n"
    ));
}

fn has_capability(objects: &[String], object: &Object) -> bool {
    objects.iter().any(|name| name == &object.name)
}

fn has_local_property(object: &Object, name: &str) -> bool {
    object
        .properties
        .iter()
        .any(|property| property.name == name)
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
    if !schema.attached_properties.is_empty() || !schema.visual_properties.is_empty() {
        generated.insert("Microsoft::UI::Xaml::IDependencyObject::ClearValue".to_string());
    }
    if !schema.capabilities.enabled.is_empty() {
        generated.insert("Microsoft::UI::Xaml::Controls::IControl::get_IsEnabled".to_string());
        generated.insert("Microsoft::UI::Xaml::Controls::IControl::put_IsEnabled".to_string());
        generated.insert("Microsoft::UI::Xaml::Controls::Control::IsEnabledProperty".to_string());
    }
    if schema.capabilities.layout_exit_transition {
        generated.insert("Microsoft::UI::Xaml::IScalarTransition::put_Duration".to_string());
        generated.insert("Microsoft::UI::Xaml::IUIElement::get_Opacity".to_string());
        generated.insert("Microsoft::UI::Xaml::IUIElement::put_OpacityTransition".to_string());
        generated.insert("Microsoft::UI::Xaml::ScalarTransition::CreateInstance".to_string());
    }
    if schema.objects.iter().any(|object| object.virtual_items) {
        generated.extend([
            "Microsoft::UI::Xaml::Controls::ContentControl::CreateInstance".to_string(),
            "Microsoft::UI::Xaml::Controls::IContentControl::put_Content".to_string(),
            "Microsoft::UI::Xaml::Controls::IItemsRepeater::GetOrCreateElement".to_string(),
            "Microsoft::UI::Xaml::Controls::IItemsRepeater::put_ItemsSource".to_string(),
            "Microsoft::UI::Xaml::Controls::IItemsRepeater::put_ItemTemplate".to_string(),
            "Microsoft::UI::Xaml::IElementFactory::{}".to_string(),
            "Microsoft::UI::Xaml::IElementFactoryGetArgs::get_Data".to_string(),
            "Microsoft::UI::Xaml::IElementFactoryRecycleArgs::get_Element".to_string(),
        ]);
    }
    for property in &schema.attached_properties {
        let owner = binding_path(&property.owner);
        if property.readback {
            generated.insert(format!("{owner}::Get{}", property.native));
        }
        generated.insert(format!("{owner}::Set{}", property.native));
        if property.default.is_none() {
            generated.insert(format!("{owner}::{}Property", property.native));
        }
    }
    for property in &schema.visual_properties {
        let (namespace, owner) = property.owner.rsplit_once('.').unwrap();
        let namespace = binding_path(namespace);
        if property.readback {
            generated.insert(format!("{namespace}::I{owner}::get_{}", property.name));
        }
        generated.insert(format!("{namespace}::I{owner}::put_{}", property.name));
        if property.default.is_none() {
            generated.insert(format!(
                "{namespace}::I{owner}Statics::get_{}Property",
                property.name
            ));
        }
        match property.value.as_str() {
            "ThemeTransitions" => {
                generated.insert(
                    "Microsoft::UI::Xaml::Media::Animation::RepositionThemeTransition::CreateInstance"
                        .to_string(),
                );
                generated
                    .insert("Microsoft::UI::Xaml::Media::Animation::Transition::{}".to_string());
                generated.insert(
                    "Microsoft::UI::Xaml::Media::Animation::TransitionCollection::CreateInstance"
                        .to_string(),
                );
            }
            _ if !is_builtin_value(&property.value) => {
                let owner = property.owner.rsplit('.').next().unwrap();
                let enum_path = metadata
                    .enum_path(owner, &format!("put_{}", property.name))
                    .unwrap();
                generated.insert(binding_path(&enum_path));
            }
            _ => {}
        }
    }
    for object in schema
        .objects
        .iter()
        .filter(|object| object.native != "handwritten")
    {
        generated.insert(format!("{}::CreateInstance", binding_path(&object.native)));
        for property in &object.properties {
            let native = native_property(property);
            if property.adapter.as_deref() == Some("rich_edit_text") {
                generated.extend([
                    "Microsoft::UI::Xaml::Controls::IRichEditBox::get_IsReadOnly".to_string(),
                    "Microsoft::UI::Text::ITextDocument::GetText".to_string(),
                    "Microsoft::UI::Text::ITextDocument::SetText".to_string(),
                    "Microsoft::UI::Text::TextGetOptions".to_string(),
                    "Microsoft::UI::Text::TextSetOptions".to_string(),
                ]);
                let getter = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(object), &getter)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
                continue;
            }
            if matches!(
                property.adapter.as_deref(),
                Some("grid_rows" | "grid_columns")
            ) {
                let getter = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(object), &getter)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
                let (definition, dimension) = if property.adapter.as_deref() == Some("grid_rows") {
                    ("RowDefinition", "Height")
                } else {
                    ("ColumnDefinition", "Width")
                };
                generated.extend([
                    format!("Microsoft::UI::Xaml::Controls::{definition}::CreateInstance"),
                    format!("Microsoft::UI::Xaml::Controls::I{definition}::put_{dimension}"),
                    format!("Microsoft::UI::Xaml::Controls::I{definition}::put_Min{dimension}"),
                    format!("Microsoft::UI::Xaml::Controls::I{definition}::put_Max{dimension}"),
                    format!("Microsoft::UI::Xaml::Controls::{definition}Collection::{{}}"),
                    "Microsoft::UI::Xaml::GridLength".to_string(),
                    "Microsoft::UI::Xaml::GridUnitType".to_string(),
                ]);
                continue;
            }
            let setter = format!("put_{native}");
            let interface = metadata
                .resolve(&native_name(object), &setter)
                .unwrap()
                .full_path();
            generated.insert(format!("{}::{setter}", binding_path(&interface)));
            if property.readback {
                let getter = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(object), &getter)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
            }
            if property.default.is_none() {
                let (_, interface) = metadata
                    .dependency_property(&native_name(object), native)
                    .unwrap_or_else(|| {
                        panic!(
                            "cannot resolve {}.{native} dependency property",
                            native_name(object)
                        )
                    });
                generated.insert(format!(
                    "{}::get_{native}Property",
                    binding_path(&interface.full_path())
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
            if event.value == "PointerEventInfo" {
                generated.extend([
                    "Microsoft::UI::Input::IPointerPoint::get_PointerId".to_string(),
                    "Microsoft::UI::Input::IPointerPoint::get_Position".to_string(),
                    "Microsoft::UI::Input::IPointerPoint::get_Properties".to_string(),
                    "Microsoft::UI::Input::IPointerPointProperties::get_IsLeftButtonPressed"
                        .to_string(),
                    "Microsoft::UI::Input::IPointerPointProperties::get_IsMiddleButtonPressed"
                        .to_string(),
                    "Microsoft::UI::Input::IPointerPointProperties::get_IsRightButtonPressed"
                        .to_string(),
                    "Microsoft::UI::Xaml::IUIElement::get_PointerCaptures".to_string(),
                    "Microsoft::UI::Xaml::Input::IPointerRoutedEventArgs::GetCurrentPoint"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::IPointerRoutedEventArgs::get_Pointer".to_string(),
                ]);
            }
            if let Some(payload) = &event.payload {
                let (_, interface, _) = metadata
                    .resolve_event_args_property(
                        &native_name(object),
                        &format!("add_{}", event.name),
                        payload,
                    )
                    .unwrap();
                generated.insert(format!("{}::get_{payload}", binding_path(&interface)));
            }
            if let Some(observed) = &event.observes {
                let property = object
                    .properties
                    .iter()
                    .find(|property| property.name == *observed)
                    .unwrap();
                let observed_native = native_property(property);
                let interface = metadata
                    .resolve(&native_name(object), &format!("get_{observed_native}"))
                    .unwrap()
                    .full_path();
                generated.insert(format!(
                    "{}::get_{observed_native}",
                    binding_path(&interface)
                ));
                if property.adapter.as_deref() == Some("rich_edit_text") {
                    generated.insert("Microsoft::UI::Text::ITextDocument::GetText".to_string());
                    generated.insert("Microsoft::UI::Text::TextGetOptions".to_string());
                }
            }
        }
        if let Some(selection) = &object.selection
            && selection.event_item_source == "EventArgs"
        {
            let interface = metadata
                .resolve_event_args_object_property(
                    &native_name(object),
                    &format!("add_{}", selection.event),
                    &selection.selected_item_property,
                )
                .unwrap();
            let event_args = selection.event_args.as_deref().unwrap();
            assert!(
                interface.ends_with(&format!(".I{event_args}")),
                "{}.{} event args must be {event_args}",
                object.name,
                selection.event
            );
            generated.insert(format!(
                "{}::get_{}",
                binding_path(&interface),
                selection.selected_item_property
            ));
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
                if relation.name == "Children" && relation.native_item.is_none() {
                    generated
                        .insert("Microsoft::UI::Xaml::Controls::IPanel::get_Children".to_string());
                } else {
                    let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                    let interface = metadata
                        .resolve(&native_name(object), &format!("get_{native_relation}"))
                        .unwrap()
                        .full_path();
                    generated.insert(format!(
                        "{}::get_{native_relation}",
                        binding_path(&interface)
                    ));
                }
            }
        }
        if let Some(selection) = &object.selection {
            for method in [
                format!("get_{}", selection.selected_item_property),
                format!("put_{}", selection.selected_item_property),
            ] {
                let interface = metadata
                    .resolve(&native_name(object), &method)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{method}", binding_path(&interface)));
            }
            let item = schema
                .objects
                .iter()
                .find(|candidate| candidate.name == selection.item)
                .unwrap();
            for property in [&selection.selected_property, &selection.payload_property] {
                let native = item
                    .properties
                    .iter()
                    .find(|candidate| candidate.name == *property)
                    .map(native_property)
                    .unwrap();
                let method = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(item), &method)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{method}", binding_path(&interface)));
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
            for allowed in &relation.allowed_objects {
                let allowed = schema
                    .objects
                    .iter()
                    .find(|object| object.name == *allowed)
                    .unwrap_or_else(|| panic!("unknown allowed object `{allowed}`"));
                assert_eq!(allowed.category, relation.child);
            }
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
                "Container" if object.virtual_items => {
                    assert_eq!(relation.child, "Visual");
                    assert_eq!(relation.cardinality, "Many");
                    assert_eq!(relation.identity, "Keyed");
                }
                "Container" => assert_eq!(relation.child, "Data"),
                _ => unreachable!(),
            }
        }
        assert_eq!(
            object
                .relations
                .iter()
                .filter(|relation| {
                    relation.realization == "Container" && relation.child == "Visual"
                })
                .count(),
            usize::from(object.virtual_items),
            "virtual objects require exactly one container-realized relation"
        );
        let mut events = BTreeSet::new();
        for event in &object.events {
            assert_identifier(&event.name);
            if let Some(field) = &event.field {
                assert_identifier(field);
            }
            assert!(events.insert(event.name.as_str()), "duplicate event");
            assert!(matches!(
                event.value.as_str(),
                "Bool"
                    | "F64"
                    | "OptionalBool"
                    | "OptionalF64"
                    | "PointerEventInfo"
                    | "Selection"
                    | "SelectionIndex"
                    | "String"
                    | "Unit"
            ));
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
    for property in &schema.visual_properties {
        properties.insert(property.name.as_str(), property.value.as_str());
    }
    if !schema.capabilities.enabled.is_empty() {
        properties.insert("IsEnabled", "Bool");
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
              Color,\n\
              CornerRadius,\n\
              F64,\n\
              FontWeight,\n\
              GridLengths,\n\
              I32,\n\
              OptionalBool,\n\
              OptionalF64,\n\
              PointerEventInfo,\n\
               Selection,\n\
               ThemeTransitions,\n\
               Thickness,\n\
               SelectionIndex,\n\
               StringList,\n\
             Enum { kind: &'static str, variants: &'static [&'static str] },\n\
             Unit,\n\
         }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct RelationContract {\n\
             pub id: RelationId,\n\
             pub child: ObjectCategory,\n\
             pub allowed_objects: &'static [ObjectType],\n\
             pub cardinality: Cardinality,\n\
             pub identity: Identity,\n\
             pub realization: Realization,\n\
         }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum SelectionEventItemSource { Owner, EventArgs }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct SelectionContract {\n\
             pub relations: &'static [RelationId],\n\
             pub item: ObjectType,\n\
             pub selected_property: PropertyId,\n\
             pub event: EventId,\n\
             pub event_item_source: SelectionEventItemSource,\n\
             pub event_args: Option<&'static str>,\n\
             pub payload_property: PropertyId,\n\
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
        "pub fn selection_contract(kind: ObjectType) -> Option<SelectionContract> { match kind {\n",
    );
    for object in &schema.objects {
        if let Some(selection) = &object.selection {
            output.push_str(&format!(
                "ObjectType::{} => Some(SelectionContract {{ relations: &[{}], \
                 item: ObjectType::{}, selected_property: PropertyId::{}, \
                 event: EventId::{}, event_item_source: SelectionEventItemSource::{}, \
                 event_args: {}, payload_property: PropertyId::{} }}),\n",
                object.name,
                selection
                    .relations
                    .iter()
                    .map(|relation| format!("RelationId::{relation},"))
                    .collect::<String>(),
                selection.item,
                selection.selected_property,
                selection.event,
                selection.event_item_source,
                selection
                    .event_args
                    .as_ref()
                    .map_or_else(|| "None".to_string(), |value| format!("Some({value:?})")),
                selection.payload_property
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");
    output.push_str(
        "pub fn selection_for_relation(kind: ObjectType, relation: RelationId) -> \
         Option<SelectionContract> { selection_contract(kind)\
         .filter(|selection| selection.relations.contains(&relation)) }\n",
    );
    output.push_str(
        "pub fn selection_for_item_property(owner: ObjectType, relation: RelationId, \
         item: ObjectType, property: PropertyId) -> Option<SelectionContract> { \
         selection_for_relation(owner, relation).filter(|selection| \
         selection.item == item && selection.selected_property == property) }\n",
    );
    output.push_str(
        "pub fn relation_accepts(contract: &RelationContract, kind: ObjectType) -> bool {\n\
             object_category(kind) == contract.child\n\
                 && (contract.allowed_objects.is_empty() || contract.allowed_objects.contains(&kind))\n\
         }\n",
    );

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
        "pub fn property_contract(kind: ObjectType, id: PropertyId) -> \
         Option<PropertyContract> {\n\
         if object_category(kind) == ObjectCategory::Visual { match id {\n",
    );
    for property in &schema.attached_properties {
        let value = if is_builtin_value(&property.value) {
            format!("ValueType::{}", property.value)
        } else {
            let variants = property
                .variants
                .iter()
                .map(|variant| format!("\"{variant}\""))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "ValueType::Enum {{ kind: \"{}\", variants: &[{variants}] }}",
                property.value
            )
        };
        output.push_str(&format!(
            "PropertyId::{} => return Some(PropertyContract {{ id, value: {value} }}),",
            property.name
        ));
    }
    for property in &schema.visual_properties {
        let value = if is_builtin_value(&property.value) {
            format!("ValueType::{}", property.value)
        } else {
            let owner = property.owner.rsplit('.').next().unwrap();
            let (_, variants) = metadata
                .enum_info(owner, &format!("put_{}", property.name))
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
            "PropertyId::{} => return Some(PropertyContract {{ id, value: {value} }}),",
            property.name
        ));
    }
    output.push_str("_ => {} } }\n");
    if !schema.capabilities.enabled.is_empty() {
        output.push_str("if id == PropertyId::IsEnabled && matches!(kind,");
        for (index, object) in schema.capabilities.enabled.iter().enumerate() {
            if index != 0 {
                output.push('|');
            }
            output.push_str(&format!("ObjectType::{object}"));
        }
        output.push_str(") { return Some(PropertyContract { id, value: ValueType::Bool }); }\n");
    }
    output.push_str("match (kind, id) {\n");
    for object in &schema.objects {
        for property in &object.properties {
            if property.name == "IsEnabled" && has_capability(&schema.capabilities.enabled, object)
            {
                continue;
            }
            let value = if is_builtin_value(&property.value) {
                format!("ValueType::{}", property.value)
            } else {
                let (_, variants) = metadata
                    .enum_info(
                        &native_name(object),
                        &format!("put_{}", native_property(property)),
                    )
                    .unwrap_or_else(|| {
                        panic!("{}.{} is not an enum property", object.name, property.name)
                    });
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
                "(ObjectType::{}, PropertyId::{}) => \
                 Some(PropertyContract {{ id, value: {value} }}),",
                object.name, property.name
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");
    output.push_str(
        "pub(crate) fn property_order(kind: ObjectType, id: PropertyId) -> usize {\n\
         if object_category(kind) == ObjectCategory::Visual { match id {\n",
    );
    for (index, property) in schema.attached_properties.iter().enumerate() {
        output.push_str(&format!("PropertyId::{} => return {index},", property.name));
    }
    for (offset, property) in schema.visual_properties.iter().enumerate() {
        let index = schema.attached_properties.len() + offset;
        output.push_str(&format!("PropertyId::{} => return {index},", property.name));
    }
    output.push_str("_ => {} } }\nmatch (kind, id) {\n");
    let shared_count = schema.attached_properties.len() + schema.visual_properties.len();
    for object in &schema.objects {
        for (index, property) in object.properties.iter().enumerate() {
            output.push_str(&format!(
                "(ObjectType::{}, PropertyId::{}) => {},",
                object.name,
                property.name,
                shared_count + index
            ));
        }
        if has_capability(&schema.capabilities.enabled, object)
            && !has_local_property(object, "IsEnabled")
        {
            output.push_str(&format!(
                "(ObjectType::{}, PropertyId::IsEnabled) => {},",
                object.name,
                shared_count + object.properties.len()
            ));
        }
    }
    output.push_str("_ => usize::MAX,\n} }\n");
    output.push_str("pub(crate) const ALL_OBJECT_TYPES: &[ObjectType] = &[");
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{},", object.name));
    }
    output.push_str("];\n");
    output.push_str("pub(crate) const ALL_PROPERTY_IDS: &[PropertyId] = &[");
    for property in properties.keys() {
        output.push_str(&format!("PropertyId::{property},"));
    }
    output.push_str("];\n");
    output.push_str("fn object_index(kind: ObjectType) -> usize { match kind {\n");
    for (index, object) in schema.objects.iter().enumerate() {
        output.push_str(&format!("ObjectType::{} => {index},", object.name));
    }
    output.push_str("} }\n");
    output.push_str(
        "pub fn property_contracts(kind: ObjectType) -> &'static [PropertyContract] {\n\
         static CONTRACTS: std::sync::OnceLock<Vec<&'static [PropertyContract]>> = \
         std::sync::OnceLock::new();\n\
         let contracts = CONTRACTS.get_or_init(|| {\n\
         let mut unique = Vec::<&'static [PropertyContract]>::new();\n\
         ALL_OBJECT_TYPES.iter().copied().map(|kind| {\n\
         let mut current = ALL_PROPERTY_IDS.iter().copied()\
         .filter_map(|id| property_contract(kind, id)).collect::<Vec<_>>();\n\
         current.sort_by_key(|contract| property_order(kind, contract.id));\n\
         if let Some(existing) = unique.iter().copied()\
         .find(|existing| *existing == current.as_slice()) { existing } else {\n\
         let current: &'static [PropertyContract] = Box::leak(current.into_boxed_slice());\n\
         unique.push(current);\n\
         current\n\
         }\n\
         }).collect()\n\
         });\n\
         contracts[object_index(kind)]\n\
         }\n",
    );

    output.push_str("pub(crate) fn focus_capable(kind: ObjectType) -> bool { matches!(kind,");
    if schema.capabilities.focus.is_empty() {
        output.push_str("_ if false");
    } else {
        for (index, object) in schema.capabilities.focus.iter().enumerate() {
            if index != 0 {
                output.push('|');
            }
            output.push_str(&format!("ObjectType::{object}"));
        }
    }
    output.push_str(") }\n");

    output.push_str(
        "pub fn relation_contracts(kind: ObjectType) -> &'static [RelationContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for relation in &object.relations {
            output.push_str(&format!(
                "RelationContract {{ id: RelationId::{}, child: ObjectCategory::{}, \
                 allowed_objects: &[{}], \
                 cardinality: Cardinality::{}, identity: Identity::{}, \
                 realization: Realization::{} }},",
                relation.name,
                relation.child,
                relation
                    .allowed_objects
                    .iter()
                    .map(|object| format!("ObjectType::{object},"))
                    .collect::<String>(),
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
    for property in &schema.attached_properties {
        if is_builtin_value(&property.value) {
            continue;
        }
        if let Some(previous) = enums.insert(property.value.as_str(), property.variants.clone()) {
            assert_eq!(previous, property.variants, "conflicting enum definitions");
        }
    }
    for property in &schema.visual_properties {
        if is_builtin_value(&property.value) {
            continue;
        }
        let owner = property.owner.rsplit('.').next().unwrap();
        let (_, variants) = metadata
            .enum_info(owner, &format!("put_{}", property.name))
            .unwrap();
        if let Some(previous) = enums.insert(property.value.as_str(), variants.to_vec()) {
            assert_eq!(previous, variants, "conflicting enum definitions");
        }
    }
    for object in &schema.objects {
        if object.native == "handwritten" {
            continue;
        }
        for property in &object.properties {
            if is_builtin_value(&property.value) {
                continue;
            }
            let (_, variants) = metadata
                .enum_info(
                    &native_name(object),
                    &format!("put_{}", native_property(property)),
                )
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
    output.push_str("macro_rules! enabled_methods { () => {\n");
    output.push_str(
        "pub fn is_enabled(mut self, is_enabled: bool) -> Self {\n\
         self.0 = self.0.property(PropertyId::IsEnabled, \
         PropertyValue::Bool(is_enabled));\nself\n}\n",
    );
    output.push_str("}; }\n");
    output.push_str("macro_rules! focus_methods { () => {\n");
    output.push_str(
        "pub fn element_ref(mut self, reference: &ElementRef) -> Self {\n\
         self.0.reference = Some(reference.clone());\nself\n}\n",
    );
    output.push_str("}; }\n");
    output.push_str("macro_rules! reference_methods { ($type:ty) => {\n");
    output.push_str(
        "pub fn element_ref(mut self, reference: &ElementRef<$type>) -> Self {\n\
         self.0.reference = Some(reference.erased());\nself\n}\n",
    );
    output.push_str("}; }\n");
    output.push_str("macro_rules! visual_methods { () => {\n");
    output.push_str(
        "pub fn exit_transition(mut self, transition: Option<ExitTransition>) -> Self {\n\
         self.0.exit_transition = transition;\nself\n}\n\
         pub fn exit_fade(self, duration: std::time::Duration) -> Self {\n\
         self.exit_transition(ExitTransition::fade(duration))\n}\n",
    );
    for property in &schema.attached_properties {
        let name = snake_case(&property.name);
        if property.flag {
            output.push_str(&format!("pub fn {name}(mut self) -> Self {{\n"));
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, \
                 PropertyValue::Bool(true));\nself\n}}\n",
                property.name
            ));
            continue;
        }
        output.push_str(&format!(
            "pub fn {name}(mut self, {}) -> Self {{\n",
            property_argument_parts(&property.name, &property.value)
        ));
        if let Some(validation) = &property.validation {
            let expression = validation_expression(&name, &property.value, validation);
            output.push_str(&format!(
                "assert!({expression}, \"{} requires {validation}\");\n",
                property.name
            ));
        }
        output.push_str(&format!(
            "self.0 = self.0.property(PropertyId::{}, {});\nself\n}}\n",
            property.name,
            property_value_parts(&property.value, &name)
        ));
    }
    for property in &schema.visual_properties {
        let name = snake_case(&property.name);
        if property.value == "ThemeTransitions" {
            output.push_str(&format!(
                "pub fn {name}<T>(self, {name}: T) -> Self where T: \
                 IntoIterator<Item = ThemeTransition> {{ \
                 self.{name}_optional(Some({name})) }}\n\
                 pub fn {name}_optional<T>(mut self, {name}: Option<T>) -> Self where T: \
                 IntoIterator<Item = ThemeTransition> {{\n\
                 if let Some({name}) = {name} {{\n\
                 self.0 = self.0.property(PropertyId::{}, \
                 PropertyValue::ThemeTransitions({name}.into_iter().collect()));\n\
                 }}\n\
                 self\n\
                 }}\n",
                property.name
            ));
        } else {
            output.push_str(&format!(
                "pub fn {name}(mut self, {}) -> Self {{\n",
                property_argument_parts(&property.name, &property.value)
            ));
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, {});\nself\n}}\n",
                property.name,
                property_value_parts(&property.value, &name)
            ));
        }
    }
    output.push_str("}; }\n");
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
            let name = match property.adapter.as_deref() {
                Some("rich_edit_text") => "text".to_string(),
                Some("grid_rows") => "rows".to_string(),
                Some("grid_columns") => "columns".to_string(),
                _ => snake_case(&property.name),
            };
            let argument = match property.adapter.as_deref() {
                Some("rich_edit_text") => "text: impl Into<Rc<str>>".to_string(),
                Some("grid_rows" | "grid_columns") => {
                    "values: impl IntoIterator<Item = GridLength>".to_string()
                }
                _ => property_argument(property),
            };
            output.push_str(&format!("pub fn {name}(mut self, {argument}) -> Self {{\n"));
            if let Some(validation) = &property.validation {
                let expression = validation_expression(&name, &property.value, validation);
                output.push_str(&format!(
                    "assert!({expression}, \"{}.{} requires {validation}\");\n",
                    object.name, property.name
                ));
            }
            let value = if property.adapter.as_deref() == Some("rich_edit_text") {
                format!("PropertyValue::String(canonical_rich_edit_text({name}.into()))")
            } else if matches!(
                property.adapter.as_deref(),
                Some("grid_rows" | "grid_columns")
            ) {
                "PropertyValue::GridLengths(values.into_iter().collect())".to_string()
            } else {
                property_value(property, &name)
            };
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, {value});\nself\n}}\n",
                property.name
            ));
        }
        if has_capability(&schema.capabilities.enabled, object)
            && !has_local_property(object, "IsEnabled")
        {
            output.push_str("enabled_methods!();\n");
        }
        if has_capability(&schema.capabilities.focus, object) {
            output.push_str("focus_methods!();\n");
        }
        if has_capability(&schema.capabilities.reference, object) {
            output.push_str(&format!("reference_methods!({});\n", object.name));
        }

        if object.category == "Visual" {
            output.push_str("visual_methods!();\n");
        }

        if object.virtual_items {
            let relation = object
                .relations
                .iter()
                .find(|relation| relation.realization == "Container")
                .unwrap();
            output.push_str(&format!(
                "                pub fn item(mut self, key: impl Into<Key>, visual: impl Into<Visual>) -> Self {{\n\
                self.0 = self.0.virtual_item(RelationId::{}, keyed(key, visual));\nself\n}}\n\
                 pub fn items(mut self, items: impl IntoIterator<Item = KeyedVisual>) -> Self {{\n\
                 self.0 = self.0.virtual_items(RelationId::{}, \
                 VirtualItems::eager(items));\nself\n}}\n\
                 pub fn virtual_source(mut self, source: VirtualSource) -> Self {{\n\
                 self.0 = self.0.virtual_items(RelationId::{}, \
                 VirtualItems::Lazy(source));\nself\n}}\n",
                relation.name, relation.name, relation.name
            ));
        }

        for relation in &object.relations {
            if object.virtual_items && relation.realization == "Container" {
                continue;
            }
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
            let method = event
                .field
                .as_deref()
                .and_then(|field| field.strip_prefix("on_"))
                .map_or_else(|| snake_case(&event.name), str::to_string);
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
                "Bool" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(bool) + 'static) -> Self {{ \
                         self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<bool>) -> Self {{ \
                         self.0 = self.0.event(EventId::{}, EventValue::Bool(callback)); self }}\n",
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
                "OptionalBool" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(Option<bool>) + 'static) -> Self \
                         {{ self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<Option<bool>>) -> \
                         Self {{ self.0 = self.0.event(EventId::{}, \
                         EventValue::OptionalBool(callback)); self }}\n",
                        event.name
                    ));
                }
                "OptionalF64" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(Option<f64>) + 'static) -> Self \
                         {{ self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: Callback<Option<f64>>) -> \
                         Self {{ self.0 = self.0.event(EventId::{}, \
                         EventValue::OptionalF64(callback)); self }}\n",
                        event.name
                    ));
                }
                "Selection" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(Option<Rc<str>>) + 'static) -> \
                         Self {{ self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: \
                         Callback<Option<Rc<str>>>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, EventValue::Selection(callback)); self }}\n",
                        event.name
                    ));
                }
                "PointerEventInfo" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(PointerEventInfo) + 'static) -> \
                         Self {{ self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: \
                         Callback<PointerEventInfo>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, EventValue::PointerEventInfo(callback)); self \
                         }}\n",
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
                "SelectionIndex" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(self, callback: impl Fn(Option<usize>) + 'static) -> \
                         Self {{ self.on_{method}_callback(Callback::new(callback)) }}\n"
                    ));
                    output.push_str(&format!(
                        "pub fn on_{method}_callback(mut self, callback: \
                         Callback<Option<usize>>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, EventValue::SelectionIndex(callback)); self }}\n",
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
    property_argument_parts(&property.name, &property.value)
}

fn validation_supported(validation: &str, value: &str) -> bool {
    matches!(
        (validation, value),
        (
            "finite_positive" | "finite_non_negative",
            "F64" | "Thickness" | "CornerRadius"
        ) | ("finite", "F64" | "Thickness")
            | ("non_negative" | "positive" | "zero_to_fifty_nine", "I32")
    )
}

fn validation_expression(name: &str, value: &str, validation: &str) -> String {
    match validation {
        "finite" => format!("{name}.is_finite()"),
        "finite_positive" => format!("{name}.is_finite() && {name} > 0.0"),
        "finite_non_negative" if value == "F64" => {
            format!("{name}.is_finite() && {name} >= 0.0")
        }
        "finite_non_negative" => format!("{name}.is_finite_non_negative()"),
        "non_negative" => format!("{name} >= 0"),
        "positive" => format!("{name} > 0"),
        "zero_to_fifty_nine" => format!("(0..=59).contains(&{name})"),
        _ => unreachable!(),
    }
}

fn property_argument_parts(name: &str, value: &str) -> String {
    let name = snake_case(name);
    match value {
        "String" => format!("{name}: impl Into<Rc<str>>"),
        "Bool" => format!("{name}: bool"),
        "Color" => format!("{name}: Color"),
        "CornerRadius" => format!("{name}: CornerRadius"),
        "F64" => format!("{name}: f64"),
        "FontWeight" => format!("{name}: FontWeight"),
        "GridLengths" => format!("{name}: impl IntoIterator<Item = GridLength>"),
        "I32" => format!("{name}: i32"),
        "OptionalF64" => format!("{name}: Option<f64>"),
        "OptionalBool" => format!("{name}: Option<bool>"),
        "SelectionIndex" => format!("{name}: Option<usize>"),
        "StringList" => format!("{name}: impl IntoIterator<Item = impl Into<Rc<str>>>"),
        "Thickness" => format!("{name}: Thickness"),
        value => format!("{name}: {value}"),
    }
}

fn property_value(property: &Property, name: &str) -> String {
    property_value_parts(&property.value, name)
}

fn property_value_parts(value: &str, name: &str) -> String {
    match value {
        "String" => format!("PropertyValue::String({name}.into())"),
        "Bool" => format!("PropertyValue::Bool({name})"),
        "Color" => format!("PropertyValue::Color({name})"),
        "CornerRadius" => format!("PropertyValue::CornerRadius({name})"),
        "F64" => format!("PropertyValue::F64({name})"),
        "FontWeight" => format!("PropertyValue::FontWeight({name})"),
        "GridLengths" => {
            format!("PropertyValue::GridLengths({name}.into_iter().collect())")
        }
        "I32" => format!("PropertyValue::I32({name})"),
        "OptionalF64" => format!("PropertyValue::OptionalF64({name})"),
        "OptionalBool" => format!("PropertyValue::OptionalBool({name})"),
        "SelectionIndex" => format!("PropertyValue::SelectionIndex({name})"),
        "StringList" => {
            format!("PropertyValue::StringList({name}.into_iter().map(Into::into).collect())")
        }
        "Thickness" => format!("PropertyValue::Thickness({name})"),
        _ => format!("{name}.property_value()"),
    }
}

fn property_default_parts(value_type: &str, value: &str) -> String {
    match value_type {
        "String" => format!("{value:?}"),
        "Bool" => value.to_string(),
        "Color" => "None::<&native::Brush>".to_string(),
        "CornerRadius" => "native::CornerRadius::default()".to_string(),
        "F64" => value.to_string(),
        "I32" => value.to_string(),
        "OptionalF64" => "None".to_string(),
        "OptionalBool" if value == "none" => "None".to_string(),
        "OptionalBool" => format!("Some({value})"),
        "Thickness" => "native::Thickness::default()".to_string(),
        "SelectionIndex" => "None".to_string(),
        "StringList" => "Vec::<Option<IInspectable>>::new().into()".to_string(),
        _ => format!("native::{value_type}::{value}"),
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
            "Color" => "Color::rgb(0, 0, 0)".to_string(),
            "CornerRadius" => "CornerRadius::uniform(0.0)".to_string(),
            "F64" => "0.0".to_string(),
            "I32" => "0".to_string(),
            "OptionalF64" => "None".to_string(),
            "OptionalBool" => "None".to_string(),
            "SelectionIndex" => "None".to_string(),
            "StringList" => "std::iter::empty::<&str>()".to_string(),
            "Thickness" => "Thickness::uniform(0.0)".to_string(),
            value => format!("{value}::{}", property.default.as_deref().unwrap()),
        });
    }
    format!("{}::new({})", object.name, arguments.join(", "))
}

fn is_builtin_value(value: &str) -> bool {
    matches!(
        value,
        "String"
            | "Bool"
            | "Color"
            | "CornerRadius"
            | "F64"
            | "FontWeight"
            | "GridLengths"
            | "I32"
            | "OptionalF64"
            | "OptionalBool"
            | "SelectionIndex"
            | "Selection"
            | "StringList"
            | "ThemeTransitions"
            | "Thickness"
    )
}

fn native_property(property: &Property) -> &str {
    property.native.as_deref().unwrap_or(&property.name)
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
    let native = rustfmt(&generate_native(&schema, &metadata));
    assert_eq!(
        fs::read_to_string(workspace_path(NATIVE_OUTPUT)).unwrap(),
        native
    );
    assert!(!native.contains("std::process::abort"));
    assert!(native.contains("super::app::report_error(error)"));
    assert!(native.contains("args.NewValue().map(number_box_value)"));
    assert!(native.contains("args.NewValue()"));
    assert!(native.contains("WinUiAdapter::handle_selection_changed"));
    assert!(native.contains("let Some(args) = args.as_ref() else"));
    assert!(native.contains("args.SelectedItem()"));
    assert!(native.contains("native::Grid::SetRow(&element, *value)"));
    assert!(native.contains("native::AutomationProperties::SetName"));
    assert!(native.contains("native::Control::IsEnabledProperty()"));
    assert!(native.contains("native::FontWeight {"));
    assert!(native.contains("read_rich_edit_text(&source_text_changed)"));
    assert!(native.contains("set_rich_edit_text(&object.value"));
    assert!(native.contains("(ObjectType::RichEditBox, PropertyId::Document, None) => Some(("));
    assert!(native.contains("value: PropertyValue::String(Rc::from(\"\"))"));
    assert!(native.contains("set_grid_definitions(object"));
    let declarations = fs::read_to_string(workspace_path(DECLARATIONS_OUTPUT)).unwrap();
    assert!(declarations.contains("pub fn relative_align_left(mut self)"));
    assert!(declarations.contains("pub fn exit_fade(self, duration: std::time::Duration)"));
    assert!(declarations.contains("GridRow requires non_negative"));
    assert!(declarations.contains("GridRowSpan requires positive"));
    assert!(declarations.contains("pub fn text(mut self, text: impl Into<Rc<str>>)"));
    assert!(declarations.contains("PropertyId::Document"));
    assert!(
        declarations
            .contains("pub fn rows(mut self, values: impl IntoIterator<Item = GridLength>)")
    );
    assert!(
        declarations
            .contains("pub fn columns(mut self, values: impl IntoIterator<Item = GridLength>)")
    );
    assert!(declarations.contains("pub fn element_ref(mut self, reference: &ElementRef)"));
    for object in ["Grid", "Image", "WebView2", "SwapChainPanel"] {
        assert!(declarations.contains(&format!("reference_methods!({object});")));
    }
    assert!(declarations.contains("pub fn font_weight"));
    let contracts = fs::read_to_string(workspace_path(OUTPUT)).unwrap();
    assert!(contracts.contains("pub(crate) fn focus_capable"));
    assert!(
        contracts
            .contains("pub fn property_contracts(kind: ObjectType) -> &'static [PropertyContract]")
    );
    let navigation = native
        .split("ObjectType::NavigationView =>")
        .nth(1)
        .unwrap()
        .split("ObjectType::NavigationViewItem =>")
        .next()
        .unwrap();
    assert!(!navigation.contains("source_selection_changed"));
    assert!(native.contains("fn selected_item("));
    assert!(native.contains("fn set_selected_item("));
    let number_box = schema
        .objects
        .iter()
        .find(|object| object.name == "NumberBox")
        .unwrap();
    let value_changed = number_box
        .events
        .iter()
        .find(|event| event.name == "ValueChanged")
        .unwrap();
    assert_eq!(value_changed.payload.as_deref(), Some("NewValue"));
    assert_eq!(
        value_changed.payload_adapter.as_deref(),
        Some("number_box_value")
    );
    assert_eq!(
        schema
            .objects
            .iter()
            .filter(|object| object.selection.is_some())
            .count(),
        3
    );
    assert_eq!(
        fs::read_to_string(workspace_path(BINDINGS_FILTER)).unwrap(),
        generate_bindings(&schema, &metadata)
    );
    let bindings = fs::read_to_string(workspace_path(BINDINGS_FILTER)).unwrap();
    assert!(bindings.contains(
        "Microsoft::UI::Xaml::Controls::INavigationViewSelectionChangedEventArgs::get_SelectedItem"
    ));
    assert!(bindings.contains("Microsoft::UI::Xaml::ScalarTransition::CreateInstance"));
    assert!(bindings.contains("Microsoft::UI::Xaml::IElementFactory::{}"));
    let declarations = fs::read_to_string(workspace_path(DECLARATIONS_OUTPUT)).unwrap();
    assert!(declarations.contains("pub fn virtual_source(mut self, source: VirtualSource)"));
    assert!(declarations.contains("self.0.virtual_item(RelationId::Items, keyed(key, visual))"));
    assert!(schema.capabilities.layout_exit_transition);
}

#[test]
#[should_panic(expected = "TextBlock.Width duplicates a visual property")]
fn rejects_local_visual_property_collision() {
    let schema: Schema = toml::from_str(
        r#"
        [[visual_properties]]
        name = "Width"
        owner = "Microsoft.UI.Xaml.FrameworkElement"
        value = "F64"
        default = "f64::NAN"

        [[objects]]
        name = "TextBlock"
        category = "Visual"
        native = "Microsoft.UI.Xaml.Controls.TextBlock"

        [[objects.properties]]
        name = "Width"
        value = "F64"
        default = "f64::NAN"
        "#,
    )
    .unwrap();
    validate(&schema);
}

#[test]
#[should_panic(expected = "unknown focus capability object Missing")]
fn rejects_unknown_capability_object() {
    let schema: Schema = toml::from_str(
        r#"
        [capabilities]
        focus = ["Missing"]

        [[objects]]
        name = "TextBlock"
        category = "Visual"
        native = "Microsoft.UI.Xaml.Controls.TextBlock"
        "#,
    )
    .unwrap();
    validate(&schema);
}

#[test]
#[should_panic(expected = "attached enum variants must match the value type")]
fn rejects_attached_enum_without_variants() {
    let schema: Schema = toml::from_str(
        r#"
        [[attached_properties]]
        name = "AutomationHeadingLevel"
        owner = "Microsoft.UI.Xaml.Automation.AutomationProperties"
        native = "HeadingLevel"
        value = "AutomationHeadingLevel"

        [[objects]]
        name = "TextBlock"
        category = "Visual"
        native = "Microsoft.UI.Xaml.Controls.TextBlock"
        "#,
    )
    .unwrap();
    validate(&schema);
}

#[test]
#[should_panic(expected = "GridRow has unsupported validation positive")]
fn rejects_incompatible_attached_property_validation() {
    let schema: Schema = toml::from_str(
        r#"
        [[attached_properties]]
        name = "GridRow"
        owner = "Microsoft.UI.Xaml.Controls.Grid"
        native = "Row"
        value = "Bool"
        validation = "positive"

        [[objects]]
        name = "TextBlock"
        category = "Visual"
        native = "Microsoft.UI.Xaml.Controls.TextBlock"
        "#,
    )
    .unwrap();
    validate(&schema);
}

#[test]
fn missing_default_clears_declaring_dependency_property() {
    let metadata = tool_reactor::metadata::MetadataResolver::load(&workspace_path(WINMD));
    let mut output = String::new();
    emit_native_property_arms(
        &mut output,
        "(Self::Button(object), PropertyId::Background, ",
        "object",
        "IControl",
        "Button",
        "Background",
        "Color",
        None,
        None,
        &metadata,
    );
    assert!(output.contains("native::Control::BackgroundProperty()"));
    assert!(output.contains("object.ClearValue(&property)"));
    assert!(!output.contains("SetBackground(None"));
}
