use serde::Deserialize;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SCHEMA: &str = "crates/tools/reactor/src/schema.toml";
const OUTPUT: &str = "crates/libs/reactor/src/generated.rs";
const DECLARATIONS_OUTPUT: &str = "crates/libs/reactor/src/generated_declarations.rs";
const NATIVE_OUTPUT: &str = "crates/libs/reactor/src/native/generated.rs";
const WINMD: &str = "crates/tools/reactor-metadata/winmd";
const BINDINGS_BASE: &str = "crates/tools/reactor/src/bindings_base.txt";
const BINDINGS_FILTER: &str = "crates/tools/reactor/src/bindings.txt";
const BINDINGS_OUTPUT: &str = "crates/libs/reactor/src/native/bindings.rs";
const CANVAS_FILTER: &str = "crates/tools/reactor/src/canvas.txt";
const CANVAS_BINDINGS_OUTPUT: &str = "crates/libs/canvas/src/reactor_bindings.rs";
const LIVE_COVERAGE_OUTPUT: &str = "crates/tests/libs/reactor_selftest/src/generated_coverage.rs";
const LIVE_COVERAGE_REPORT: &str = "crates/tests/libs/reactor_selftest/coverage.md";

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Schema {
    #[serde(default)]
    layout_exit_transition: bool,
    #[serde(default)]
    attached_properties: Vec<AttachedProperty>,
    #[serde(default)]
    visual_properties: Vec<VisualProperty>,
    objects: Vec<Object>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct AttachedProperty {
    name: String,
    owner: String,
    native: String,
    #[serde(default)]
    value: String,
    #[serde(default)]
    flag: bool,
    default: Option<String>,
    validation: Option<Validation>,
    #[serde(default)]
    readback: bool,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct VisualProperty {
    name: String,
    owner: String,
    #[serde(default)]
    value: String,
    default: Option<String>,
    #[serde(default)]
    readback: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
enum ObjectCategory {
    #[default]
    Visual,
    Structural,
    Data,
}

impl ObjectCategory {
    fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "Visual",
            Self::Structural => "Structural",
            Self::Data => "Data",
        }
    }
}

impl std::fmt::Display for ObjectCategory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
enum Cardinality {
    #[default]
    One,
    Many,
}

impl std::fmt::Display for Cardinality {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Cardinality {
    fn as_str(self) -> &'static str {
        match self {
            Self::One => "One",
            Self::Many => "Many",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
enum Identity {
    #[default]
    Positional,
    Keyed,
}

impl std::fmt::Display for Identity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Identity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Positional => "Positional",
            Self::Keyed => "Keyed",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
enum Realization {
    #[default]
    Owned,
    Structural,
    Container,
}

impl std::fmt::Display for Realization {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
enum NativeCollection {
    Vector,
    ObservableVector,
    ItemCollection,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Validation {
    Finite,
    FinitePositive,
    FiniteNonNegative,
    NonNegative,
    Positive,
    ZeroToFiftyNine,
}

impl std::fmt::Display for Validation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Finite => "finite",
            Self::FinitePositive => "finite_positive",
            Self::FiniteNonNegative => "finite_non_negative",
            Self::NonNegative => "non_negative",
            Self::Positive => "positive",
            Self::ZeroToFiftyNine => "zero_to_fifty_nine",
        };
        formatter.write_str(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum PropertyAdapter {
    DropPolicy,
    GridColumns,
    GridRows,
    ImageUri,
    ImplicitOpacityTransition,
    ImplicitScale,
    ImplicitScaleTransition,
    InspectableString,
    InspectableStringList,
    KeyAccelerators,
    NativeColor,
    NumberBoxValue,
    PathData,
    PointerCapture,
    PointerFocus,
    RatingValue,
    ResourceOverrides,
    ResourceStyle,
    RichEditText,
    RichTextBlocks,
    SelectionIndex,
    ThemeBrush,
    Uri,
}

impl PropertyAdapter {
    fn value(self) -> &'static str {
        match self {
            Self::DropPolicy => "DragDropPolicy",
            Self::GridColumns | Self::GridRows => "GridLengths",
            Self::ImageUri => "ImageSource",
            Self::ImplicitOpacityTransition | Self::ImplicitScaleTransition => "Duration",
            Self::ImplicitScale => "F64",
            Self::InspectableString | Self::PathData | Self::RichEditText | Self::Uri => "String",
            Self::InspectableStringList => "StringList",
            Self::KeyAccelerators => "KeyAccelerators",
            Self::NativeColor => "Color",
            Self::NumberBoxValue | Self::RatingValue => "OptionalF64",
            Self::PointerCapture | Self::PointerFocus => "Bool",
            Self::ResourceOverrides => "ResourceOverrides",
            Self::ResourceStyle => "ButtonStyle",
            Self::RichTextBlocks => "RichText",
            Self::SelectionIndex => "SelectionIndex",
            Self::ThemeBrush => "Brush",
        }
    }

    fn method(self) -> Option<&'static str> {
        match self {
            Self::RichEditText => Some("text"),
            Self::GridRows => Some("rows"),
            Self::GridColumns => Some("columns"),
            Self::KeyAccelerators => Some("key_accelerators"),
            Self::ResourceOverrides => Some("resource_overrides"),
            _ => None,
        }
    }

    fn has_managed_state(self) -> bool {
        matches!(
            self,
            Self::PointerCapture | Self::PointerFocus | Self::DropPolicy
        )
    }

    fn mutates_native_collection(self) -> bool {
        matches!(
            self,
            Self::RichEditText | Self::RichTextBlocks | Self::GridRows | Self::GridColumns
        )
    }

    fn uses_framework_element(self) -> bool {
        matches!(
            self,
            Self::KeyAccelerators | Self::ResourceOverrides | Self::ResourceStyle
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum PayloadAdapter {
    ContentDialogResult,
    InspectableString,
    ItemTag,
    NativeColor,
    NavigationDisplayMode,
    NumberBoxValue,
    #[serde(rename = "optional_datetime")]
    OptionalDateTime,
    #[serde(rename = "optional_timespan")]
    OptionalTimeSpan,
    RatingValue,
    SelectionIndex,
}

impl PayloadAdapter {
    fn value(self) -> &'static str {
        match self {
            Self::ContentDialogResult => "ContentDialogResult",
            Self::InspectableString | Self::ItemTag => "String",
            Self::NativeColor => "Color",
            Self::NavigationDisplayMode => "NavigationViewDisplayMode",
            Self::NumberBoxValue | Self::RatingValue => "OptionalF64",
            Self::OptionalDateTime => "OptionalDateTime",
            Self::OptionalTimeSpan => "OptionalTimeSpan",
            Self::SelectionIndex => "SelectionIndex",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum EventAdapter {
    Character,
    DragKind,
    DroppedData,
    Focus,
    ItemTag,
    Key,
    Pointer,
    StringList,
}

impl EventAdapter {
    fn value(self) -> &'static str {
        match self {
            Self::Character => "CharacterEventInfo",
            Self::DragKind => "DragKind",
            Self::DroppedData => "DroppedData",
            Self::Focus => "FocusEventInfo",
            Self::ItemTag => "String",
            Self::Key => "KeyEventInfo",
            Self::Pointer => "PointerEventInfo",
            Self::StringList => "StringList",
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Feedback {
    Exact,
    Normalized,
    DeferredExact,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Object {
    name: String,
    #[serde(default)]
    category: ObjectCategory,
    native: Option<String>,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    focus: bool,
    #[serde(default)]
    reference: bool,
    #[serde(default)]
    tooltip_attachment: bool,
    #[serde(default)]
    content_dialog_attachment: bool,
    #[serde(default)]
    window_title_bar: bool,
    #[serde(default)]
    handwritten: bool,
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

impl Object {
    fn is_handwritten(&self) -> bool {
        self.handwritten
    }

    fn native_path(&self) -> Cow<'_, str> {
        self.native.as_deref().map_or_else(
            || Cow::Owned(format!("Microsoft.UI.Xaml.Controls.{}", self.name)),
            Cow::Borrowed,
        )
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Property {
    name: String,
    native: Option<String>,
    method: Option<String>,
    #[serde(default)]
    value: String,
    adapter: Option<PropertyAdapter>,
    controlled: Option<String>,
    coerces: Option<String>,
    feedback: Option<Feedback>,
    #[serde(default)]
    clear_feedback: bool,
    #[serde(default)]
    required: bool,
    default: Option<String>,
    validation: Option<Validation>,
    #[serde(default)]
    readback: bool,
}

impl Property {
    fn method(&self) -> String {
        self.method
            .clone()
            .or_else(|| {
                self.adapter
                    .and_then(PropertyAdapter::method)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| snake_case(&self.name))
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Relation {
    name: String,
    native: Option<String>,
    #[serde(default)]
    child: ObjectCategory,
    #[serde(default)]
    allowed_objects: Vec<String>,
    #[serde(default)]
    cardinality: Cardinality,
    #[serde(default)]
    identity: Identity,
    #[serde(default)]
    realization: Realization,
    method: Option<String>,
    item: Option<String>,
    native_collection: Option<NativeCollection>,
    native_item: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    name: String,
    field: Option<String>,
    adapter: Option<EventAdapter>,
    #[serde(default)]
    value: String,
    #[serde(default)]
    property_changed: bool,
    observes: Option<String>,
    payload: Option<String>,
    payload_adapter: Option<PayloadAdapter>,
    #[serde(default)]
    active_properties: Vec<String>,
    #[serde(default)]
    routed: bool,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
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
    let mut schema: Schema = toml::from_str(&source).unwrap();
    let metadata = tool_reactor_metadata::MetadataResolver::load(&workspace_path(WINMD));
    normalize_schema(&mut schema, &metadata);
    let generated = rustfmt(&generate(&schema, &metadata));
    let declarations = rustfmt(&generate_declarations(&schema, &metadata));
    let native = rustfmt(&generate_native(&schema, &metadata));
    let live_coverage = rustfmt(&generate_live_coverage(&schema));
    let live_coverage_report = generate_live_coverage_report(&schema);
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
    let live_coverage_output = workspace_path(LIVE_COVERAGE_OUTPUT);
    if fs::read_to_string(&live_coverage_output).ok().as_deref() != Some(&live_coverage) {
        fs::write(live_coverage_output, live_coverage).unwrap();
    }
    let live_coverage_report_output = workspace_path(LIVE_COVERAGE_REPORT);
    if fs::read_to_string(&live_coverage_report_output)
        .ok()
        .as_deref()
        != Some(&live_coverage_report)
    {
        fs::write(live_coverage_report_output, live_coverage_report).unwrap();
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
    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(CANVAS_BINDINGS_OUTPUT))
        .minimal()
        .dead_code()
        .flat()
        .filter_file(workspace_path(CANVAS_FILTER))
        .write();
}

fn normalize_schema(schema: &mut Schema, metadata: &tool_reactor_metadata::MetadataResolver) {
    for property in &mut schema.attached_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        let method = format!("Set{}", property.native);
        let inferred = metadata
            .infer_static_value_type(owner, &method)
            .map_or_else(
                || {
                    panic!(
                        "cannot infer value for {}.{} from {owner}.{method}",
                        property.owner, property.native
                    )
                },
                normalize_metadata_value,
            );
        if property.value.is_empty() {
            property.value = inferred;
        } else {
            assert_eq!(
                property.value, inferred,
                "{}.{} value does not match metadata",
                property.owner, property.native
            );
        }
    }

    for property in &mut schema.visual_properties {
        let owner = property.owner.rsplit('.').next().unwrap();
        normalize_property_value(
            &mut property.value,
            metadata,
            owner,
            &format!("put_{}", property.name),
            None,
            &format!("{}.{}", property.owner, property.name),
        );
    }

    for object in &mut schema.objects {
        if let Some(inferred) = metadata.class_path(&object.name) {
            if let Some(explicit) = &object.native {
                assert_eq!(
                    explicit, inferred,
                    "{} native class does not match metadata",
                    object.name
                );
            } else {
                object.native = Some(inferred.to_string());
            }
        }
        let owner = object.native_path().rsplit('.').next().unwrap().to_string();
        for property in &mut object.properties {
            let native = native_property(property).to_string();
            let method = if property
                .adapter
                .is_some_and(PropertyAdapter::mutates_native_collection)
            {
                format!("get_{native}")
            } else {
                format!("put_{native}")
            };
            normalize_property_value(
                &mut property.value,
                metadata,
                &owner,
                &method,
                property.adapter,
                &format!("{}.{}", object.name, property.name),
            );
        }

        let property_values = object
            .properties
            .iter()
            .map(|property| (property.name.as_str(), property.value.as_str()))
            .collect::<BTreeMap<_, _>>();
        for event in &mut object.events {
            let inferred = event
                .observes
                .as_deref()
                .map(|observed| {
                    property_values
                        .get(observed)
                        .unwrap_or_else(|| {
                            panic!(
                                "{}.{} observes missing property {observed}",
                                object.name, event.name
                            )
                        })
                        .to_string()
                })
                .or_else(|| {
                    object
                        .selection
                        .as_ref()
                        .filter(|selection| selection.event == event.name)
                        .map(|_| "Selection".to_string())
                })
                .or_else(|| event.adapter.map(EventAdapter::value).map(str::to_string))
                .or_else(|| {
                    event
                        .payload_adapter
                        .map(PayloadAdapter::value)
                        .map(str::to_string)
                })
                .or_else(|| {
                    event.payload.as_deref().and_then(|payload| {
                        metadata
                            .resolve_event_args_property(
                                &owner,
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .map(|(value, _, _)| normalize_metadata_value(value))
                    })
                });
            if event.value.is_empty() {
                event.value = inferred.unwrap_or_else(|| "Unit".to_string());
            } else if let Some(inferred) = inferred {
                assert!(
                    event.value == "Unit"
                        || event.value == inferred
                        || matches!(
                            (event.value.as_str(), inferred.as_str()),
                            ("FontWeight", "U16")
                        ),
                    "{}.{} value {} does not match inferred value {inferred}",
                    object.name,
                    event.name,
                    event.value
                );
            }
        }

        for relation in &mut object.relations {
            if relation.realization != Realization::Owned {
                continue;
            }
            let native = relation.native.as_deref().unwrap_or(&relation.name);
            let Some(collection) = metadata.classify_collection(&owner, &format!("get_{native}"))
            else {
                assert_eq!(
                    relation.cardinality,
                    Cardinality::One,
                    "{}.{} is declared as a collection but metadata is not",
                    object.name,
                    relation.name
                );
                continue;
            };
            relation.cardinality = Cardinality::Many;
            let (native_collection, native_item) = match collection {
                tool_reactor_metadata::CollectionType::UiElementCollection => continue,
                tool_reactor_metadata::CollectionType::InspectableVector => {
                    (NativeCollection::Vector, "IInspectable".to_string())
                }
                tool_reactor_metadata::CollectionType::ItemCollection => {
                    (NativeCollection::ItemCollection, "IInspectable".to_string())
                }
                tool_reactor_metadata::CollectionType::TypedVector(item) => {
                    (NativeCollection::Vector, item)
                }
                tool_reactor_metadata::CollectionType::ObservableVector(item) => {
                    (NativeCollection::ObservableVector, item)
                }
            };
            if let Some(explicit) = relation.native_collection {
                assert_eq!(
                    explicit, native_collection,
                    "{}.{} native collection does not match metadata",
                    object.name, relation.name
                );
            } else {
                relation.native_collection = Some(native_collection);
            }
            if let Some(explicit) = &relation.native_item {
                assert_eq!(
                    explicit, &native_item,
                    "{}.{} native item does not match metadata",
                    object.name, relation.name
                );
            } else {
                relation.native_item = Some(native_item);
            }
        }
    }

    for object in &schema.objects {
        for property in &object.properties {
            assert!(
                !property.value.is_empty(),
                "{}.{} has no resolved value",
                object.name,
                property.name
            );
            for event in [&property.controlled, &property.coerces]
                .into_iter()
                .flatten()
            {
                assert!(
                    object
                        .events
                        .iter()
                        .any(|candidate| candidate.name == *event),
                    "{}.{} references missing event {event}",
                    object.name,
                    property.name
                );
            }
        }
        for event in &object.events {
            if event.property_changed {
                assert!(
                    event.observes.is_some(),
                    "{}.{} property change event must observe a property",
                    object.name,
                    event.name
                );
            }
            if let Some(observed) = &event.observes {
                assert!(
                    object
                        .properties
                        .iter()
                        .any(|property| property.name == *observed),
                    "{}.{} observes missing property {observed}",
                    object.name,
                    event.name
                );
            }
            if let Some(field) = &event.field {
                assert!(
                    field.starts_with("on_"),
                    "{}.{} event field must start with on_",
                    object.name,
                    event.name
                );
            }
            for property in &event.active_properties {
                assert!(
                    object
                        .properties
                        .iter()
                        .any(|candidate| snake_case(&candidate.name) == *property),
                    "{}.{} references missing active property {property}",
                    object.name,
                    event.name
                );
            }
        }
        for relation in &object.relations {
            if let Some(item) = &relation.item {
                assert!(
                    schema
                        .objects
                        .iter()
                        .any(|candidate| candidate.name == *item),
                    "{}.{} references missing item {item}",
                    object.name,
                    relation.name
                );
            }
            for allowed in &relation.allowed_objects {
                assert!(
                    schema
                        .objects
                        .iter()
                        .any(|candidate| candidate.name == *allowed),
                    "{}.{} references missing allowed object {allowed}",
                    object.name,
                    relation.name
                );
            }
        }
        if let Some(selection) = &object.selection {
            assert!(
                object
                    .events
                    .iter()
                    .any(|event| event.name == selection.event),
                "{} selection references missing event {}",
                object.name,
                selection.event
            );
            for relation in &selection.relations {
                assert!(
                    object
                        .relations
                        .iter()
                        .any(|candidate| candidate.name == *relation),
                    "{} selection references missing relation {relation}",
                    object.name
                );
            }
        }
    }
}

fn normalize_property_value(
    value: &mut String,
    metadata: &tool_reactor_metadata::MetadataResolver,
    owner: &str,
    method: &str,
    adapter: Option<PropertyAdapter>,
    schema_name: &str,
) {
    let inferred = adapter
        .map(PropertyAdapter::value)
        .map(str::to_string)
        .or_else(|| {
            metadata
                .infer_value_type(owner, method)
                .map(|(value, _)| normalize_metadata_value(value))
        });
    if value.is_empty() {
        *value = inferred.unwrap_or_else(|| {
            panic!("cannot infer value for {schema_name} from {owner}.{method}")
        });
    } else if let Some(inferred) = inferred {
        assert!(
            *value == inferred
                || matches!(
                    (value.as_str(), inferred.as_str()),
                    ("FontWeight", "U16") | ("OptionalBool", "Bool")
                ),
            "{schema_name} value {value} does not match inferred value {inferred}"
        );
    }
}

fn normalize_metadata_value(value: String) -> String {
    if value == "Str" {
        "String".to_string()
    } else {
        value
    }
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join(path)
}

fn generate_native(schema: &Schema, metadata: &tool_reactor_metadata::MetadataResolver) -> String {
    let objects = schema
        .objects
        .iter()
        .filter(|object| !object.is_handwritten())
        .collect::<Vec<_>>();
    let mut output = String::from("// This file is generated by tool-reactor.\n");

    for object in &objects {
        if !object.events.is_empty() {
            output.push_str(&format!("struct Generated{} {{\n", object.name));
            output.push_str(&format!("value: native::{},\n", native_name(object)));
            for property in object.properties.iter().filter(|property| {
                property
                    .adapter
                    .is_some_and(PropertyAdapter::has_managed_state)
            }) {
                let field = snake_case(&property.name);
                if property.adapter == Some(PropertyAdapter::DropPolicy) {
                    output.push_str(&format!(
                        "{field}: Rc<RefCell<Option<Rc<DragDropPolicy>>>>,\n"
                    ));
                } else {
                    output.push_str(&format!("{field}: Rc<Cell<bool>>,\n"));
                }
            }
            for event in &object.events {
                output.push_str(&format!(
                    "{}: Rc<RefCell<Native{}Event>>,\n_{}: GeneratedRevoker,\n",
                    snake_case(&event.name),
                    event.value,
                    snake_case(&event.name)
                ));
            }
            output.push_str("}\n");
        }
    }

    output.push_str(
        "fn theme_style_info(kind: ObjectType) -> \
         Option<(&'static str, &'static [(PropertyId, &'static str)])> { match kind {\n",
    );
    for object in &schema.objects {
        let properties = object
            .properties
            .iter()
            .filter(|property| property.adapter == Some(PropertyAdapter::ThemeBrush))
            .collect::<Vec<_>>();
        if properties.is_empty() {
            continue;
        }
        let target = object.native_path().rsplit('.').next().unwrap().to_string();
        output.push_str(&format!(
            "ObjectType::{} => Some((\"{target}\", &[{}])),\n",
            object.name,
            properties
                .iter()
                .map(|property| format!(
                    "(PropertyId::{}, \"{}\"),",
                    property.name,
                    native_property(property)
                ))
                .collect::<String>()
        ));
    }
    output.push_str("_ => None,\n} }\n");

    let collection_items = objects
        .iter()
        .flat_map(|object| &object.relations)
        .filter(|relation| {
            relation.realization == Realization::Owned
                && relation.cardinality == Cardinality::Many
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
            format!("Box<Generated{}>", object.name)
        };
        output.push_str(&format!("{}({value}),\n", object.name));
    }
    output.push_str("}\nimpl GeneratedHandle {\n");

    output.push_str(
        "fn create(kind: ObjectType, object: ObjectId, event_queue: \
                     &Rc<NativeEventQueue>, pending_focus_states: \
                     &Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>) -> \
                     Result<Option<Self>, WinUiError> {\nOk(Some(match kind {\n",
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
            for property in object.properties.iter().filter(|property| {
                property
                    .adapter
                    .is_some_and(PropertyAdapter::has_managed_state)
            }) {
                let field = snake_case(&property.name);
                if property.adapter == Some(PropertyAdapter::DropPolicy) {
                    output.push_str(&format!("let {field} = Rc::new(RefCell::new(None));\n"));
                } else {
                    output.push_str(&format!("let {field} = Rc::new(Cell::new(false));\n"));
                }
            }
            for event in &object.events {
                let field = snake_case(&event.name);
                let selection = object
                    .selection
                    .as_ref()
                    .filter(|selection| selection.event == event.name);
                let interface = (!event.property_changed).then(|| {
                    metadata
                        .resolve(&native_name(object), &format!("add_{}", event.name))
                        .unwrap()
                        .short_name()
                });
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
                    let read = if property.adapter == Some(PropertyAdapter::RichEditText) {
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
                                if property.adapter == Some(PropertyAdapter::RatingValue) {
                                    "rating_value"
                                } else {
                                    "number_box_value"
                                }
                            ),
                            "Color" => format!(
                                "source_{field}.cast::<native::{observed_interface}>()\
                             .and_then(|source| source.{observed_native}())\
                             .map(from_native_color)"
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
                     let event_for_callback = Rc::clone(&{field});\n",
                    event.value
                ));
                if !event.routed {
                    output.push_str(&format!(
                        "let event_queue_{field} = Rc::clone(event_queue);\n"
                    ));
                }
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
                        "let source_{field} = value.cast::<native::UIElement>()?;\n\
                         let pending_focus_states_{field} = Rc::clone(pending_focus_states);\n"
                    ));
                    for property in &event.active_properties {
                        let property = snake_case(property);
                        output.push_str(&format!(
                            "let {property}_{field} = Rc::clone(&{property});\n"
                        ));
                    }
                } else if event.value == "FocusEventInfo" {
                    output.push_str(&format!(
                        "let source_{field} = value.cast::<native::UIElement>()?;\n\
                         let pending_focus_states_{field} = Rc::clone(pending_focus_states);\n"
                    ));
                } else if matches!(event.value.as_str(), "DragKind" | "DroppedData") {
                    output.push_str(&format!(
                        "let drop_policy_{field} = Rc::clone(&drop_policy);\n"
                    ));
                } else if event.value == "StringList" {
                    let interface = if object.name == "TabView" {
                        "ITabView"
                    } else {
                        "IItemsControl"
                    };
                    output.push_str(&format!(
                        "let source_{field} = value.cast::<native::{interface}>()?;\n"
                    ));
                }
                let args = if event.routed
                    || event.value == "PointerEventInfo"
                    || event.value == "FocusEventInfo"
                    || matches!(event.value.as_str(), "DragKind" | "DroppedData")
                    || event.payload.is_some()
                    || selection.is_some_and(|selection| selection.event_item_source == "EventArgs")
                {
                    "args"
                } else {
                    "_"
                };
                if event.property_changed {
                    let observed = event.observes.as_ref().unwrap();
                    let property = object
                        .properties
                        .iter()
                        .find(|property| property.name == *observed)
                        .unwrap();
                    let native = native_property(property);
                    let (owner, _) = metadata
                        .dependency_property(&native_name(object), native)
                        .unwrap();
                    let owner = owner.rsplit('.').next().unwrap();
                    output.push_str(&format!(
                        "let property_{field} = native::{owner}::{native}Property()?;\n\
                         let object_{field} = value.cast::<native::DependencyObject>()?;\n\
                         let callback_{field} = native::DependencyPropertyChangedCallback::new(\
                         move |_, _| {{\n"
                    ));
                } else {
                    output.push_str(&format!(
                        "let revoker = value.cast::<native::{}>()?.{}(move |_, {args}| {{\n",
                        interface.unwrap(),
                        event.name
                    ));
                }
                if let Some(selection) = selection {
                    let selected = match selection.event_item_source.as_str() {
                        "Owner" => format!(
                            "source_{field}.{}().and_then(|selected| \
                             selected.cast::<IInspectable>())",
                            selection.selected_item_property
                        ),
                        "EventArgs" => {
                            output.push_str("let args = args.unwrap();\n");
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
                } else if !event.routed {
                    output.push_str("let dispatch = true;\nlet observation = None;\n");
                }
                if selection.is_none()
                    && let Some(payload) = &event.payload
                {
                    if event.payload_adapter == Some(PayloadAdapter::ItemTag) {
                        output.push_str(&format!(
                            "let args = args.unwrap();\n\
                             let payload = match args.{payload}()\
                             .and_then(|value| value.cast::<native::IFrameworkElement>())\
                             .and_then(|value| value.Tag()) {{\
                             Ok(value) => match value\
                             .cast::<windows_reference::IReference<HSTRING>>()\
                             .and_then(|value| value.Value()) {{\
                             Ok(value) => Rc::<str>::from(value.to_string_lossy()),\
                             Err(error) => {{ super::app::report_error(error); return; }} }},\
                             Err(error) if error.code().is_ok() => Rc::<str>::from(\"\"),\
                             Err(error) => {{ super::app::report_error(error); return; }} }};\n"
                        ));
                    } else if matches!(
                        event.payload_adapter,
                        Some(PayloadAdapter::OptionalDateTime | PayloadAdapter::OptionalTimeSpan)
                    ) {
                        output.push_str(&format!(
                            "let args = args.unwrap();\n\
                             let payload = match args.{payload}() {{ Ok(value) => Some(value), \
                             Err(error) if error.code().is_ok() => None, Err(error) => {{ \
                             super::app::report_error(error); return; }} }};\n"
                        ));
                    } else {
                        let conversion = match event.payload_adapter {
                            Some(PayloadAdapter::ContentDialogResult) => {
                                ".map(content_dialog_result)"
                            }
                            Some(PayloadAdapter::InspectableString) => {
                                ".and_then(|value| \
                                value.cast::<windows_reference::IReference<HSTRING>>())\
                                .and_then(|value| value.Value())\
                                .map(|value| Rc::<str>::from(value.to_string_lossy()))"
                            }
                            Some(PayloadAdapter::NumberBoxValue) => ".map(number_box_value)",
                            Some(PayloadAdapter::NativeColor) => ".map(from_native_color)",
                            Some(PayloadAdapter::NavigationDisplayMode) => {
                                ".map(navigation_view_display_mode)"
                            }
                            Some(PayloadAdapter::RatingValue) => ".map(rating_value)",
                            Some(PayloadAdapter::SelectionIndex) => {
                                ".map(|value| usize::try_from(value).ok())"
                            }
                            None if event.value == "String" => ".map(Rc::<str>::from)",
                            None => "",
                            _ => unreachable!(),
                        };
                        output.push_str(&format!(
                            "let args = args.unwrap();\n\
                             let payload = match args.{payload}(){conversion} {{ Ok(value) => value, \
                             Err(error) => {{ super::app::report_error(error); return; }} }};\n"
                        ));
                    }
                }
                let dispatch_value = if event.payload.is_some() {
                    "payload"
                } else {
                    "observed"
                };
                if selection.is_some() {
                    output.push_str("})?;\n");
                    output.push_str(&format!(
                        "let _{field} = GeneratedRevoker::Event(revoker);\n"
                    ));
                    continue;
                }
                match event.value.as_str() {
                    "Bool" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_bool(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "Color" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_color(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "ContentDialogResult" => output.push_str(&format!(
                        "match WinUiAdapter::content_dialog_closed(&event_queue_{field}, object) {{ \
                         Ok(true) if dispatch => WinUiAdapter::dispatch_content_dialog_result(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, {dispatch_value}), Ok(_) => {{}}, Err(error) => \
                         super::app::report_error(error.into()), }}\n",
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
                    "OptionalDateTime" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_optional_date_time(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "OptionalF64" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_optional_f64(&event_for_callback, \
                         &event_queue_{field}, object, EventId::{}, observation, \
                         {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "OptionalTimeSpan" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_optional_time_span(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "NavigationViewDisplayMode" => output.push_str(&format!(
                        "if dispatch {{ WinUiAdapter::dispatch_navigation_view_display_mode(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, {dispatch_value}); }}\n",
                        event.name,
                    )),
                    "PointerEventInfo" => {
                        let capture_on_press = if event.name == "PointerPressed" {
                            "Some(capture_pointer_on_press_pointer_pressed.get())"
                        } else {
                            "None"
                        };
                        let release_capture = if event.name == "PointerReleased" {
                            "capture_pointer_on_press_pointer_released.get()"
                        } else {
                            "false"
                        };
                        let focus_on_release = if event.name == "PointerReleased" {
                            "focus_on_pointer_release_pointer_released.get()"
                        } else {
                            "false"
                        };
                        output.push_str(&format!(
                            "let value = match WinUiAdapter::pointer_event_info(\
                             object, &source_{field}, args, {capture_on_press}, {release_capture}, \
                             {focus_on_release}, &pending_focus_states_{field}) {{ \
                             Ok(value) => value, Err(error) => {{ \
                             super::app::report_error(error.into()); return; }} }};\n\
                             if dispatch {{ WinUiAdapter::dispatch_pointer_event_info(\
                             &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                             observation, value); }}\n",
                            event.name
                        ));
                    }
                    "FocusEventInfo" => {
                        let got_focus = event.name == "GotFocus";
                        output.push_str(&format!(
                            "let value = match WinUiAdapter::focus_event_info(\
                             object, &source_{field}, args, {got_focus}, \
                             &pending_focus_states_{field}) {{ Ok(value) => value, Err(error) => {{ \
                             super::app::report_error(error.into()); return; }} }};\n\
                             if dispatch {{ WinUiAdapter::dispatch_focus_event_info(\
                             &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                             observation, value); }}\n",
                            event.name
                        ));
                    }
                    "DragKind" => output.push_str(&format!(
                        "let value = match WinUiAdapter::drag_kind(\
                         args, &drop_policy_{field}) {{ Ok(value) => value, Err(error) => {{ \
                         super::app::report_error(error.into()); return; }} }};\n\
                         if dispatch {{ WinUiAdapter::dispatch_drag_kind(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, value); }}\n",
                        event.name
                    )),
                    "DroppedData" => output.push_str(&format!(
                        "let _ = dispatch;\n\
                         if let Err(error) = WinUiAdapter::dispatch_dropped_data(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, args, &drop_policy_{field}) {{ \
                         super::app::report_error(error.into()); }}\n",
                        event.name
                    )),
                    "StringList" => output.push_str(&format!(
                        "let value = match WinUiAdapter::{}(&source_{field}) {{ \
                         Ok(value) => value, Err(error) => {{ \
                         super::app::report_error(error.into()); return; }} }};\n\
                         if dispatch {{ WinUiAdapter::dispatch_string_list(\
                         &event_for_callback, &event_queue_{field}, object, EventId::{}, \
                         observation, value); }}\n",
                        if object.name == "TabView" {
                            "tab_item_tags"
                        } else {
                            "item_tags"
                        },
                        event.name
                    )),
                    "KeyEventInfo" => output.push_str(
                        "let args = args.unwrap();\n\
                         let value = match WinUiAdapter::key_event_info(args) { Ok(value) => value, \
                         Err(error) => { super::app::report_error(error.into()); return; } };\n\
                         let handled = event_for_callback.borrow().callback.as_ref()\
                         .is_some_and(|callback| callback.call(value));\n\
                         if let Err(error) = args.SetHandled(handled) { \
                         super::app::report_error(error); }\n",
                    ),
                    "CharacterEventInfo" => output.push_str(
                        "let args = args.unwrap();\n\
                         let value = match WinUiAdapter::character_event_info(args) { \
                         Ok(value) => value, Err(error) => { \
                         super::app::report_error(error.into()); return; } };\n\
                         let handled = event_for_callback.borrow().callback.as_ref()\
                         .is_some_and(|callback| callback.call(value));\n\
                         if let Err(error) = args.SetHandled(handled) { \
                         super::app::report_error(error); }\n",
                    ),
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
                if event.property_changed {
                    output.push_str(&format!(
                        "}});\n\
                         let token_{field} = object_{field}.RegisterPropertyChangedCallback(\
                         &property_{field}, &callback_{field})?;\n\
                         let _{field} = GeneratedRevoker::Property(PropertyChangedRevoker {{ \
                         object: object_{field}, property: property_{field}, token: token_{field} \
                         }});\n"
                    ));
                } else {
                    output.push_str("})?;\n");
                    output.push_str(&format!(
                        "let _{field} = GeneratedRevoker::Event(revoker);\n"
                    ));
                }
            }
            output.push_str(&format!(
                "Self::{}(Box::new(Generated{} {{ value,",
                object.name, object.name
            ));
            for property in object.properties.iter().filter(|property| {
                property
                    .adapter
                    .is_some_and(PropertyAdapter::has_managed_state)
            }) {
                output.push_str(&format!("{},", snake_case(&property.name)));
            }
            for event in &object.events {
                let field = snake_case(&event.name);
                output.push_str(&format!("{field}, _{field},"));
            }
            output.push_str("}))\n}\n");
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
            if relation.realization != Realization::Owned
                || relation.cardinality != Cardinality::Many
            {
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
            let conversion = if relation.native_collection == Some(NativeCollection::ItemCollection)
            {
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
            let read = if property.adapter == Some(PropertyAdapter::InspectableString) {
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
    output.push_str("_ => unreachable!(\"not a selection payload property\"),\n} }\n");

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
                let (_, variants) = metadata
                    .static_enum_info(owner, &format!("Set{}", property.native))
                    .unwrap();
                let arms = variants
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
    if schema.objects.iter().any(|object| object.enabled) {
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
        "fn set_handwritten_property(kind: ObjectType, element: &native::UIElement, \
         property: PropertyId, value: Option<&PropertyValue>) -> \
         Option<Result<(), WinUiError>> { match (kind, property, value) {\n",
    );
    for object in schema
        .objects
        .iter()
        .filter(|object| object.is_handwritten() && object.category == ObjectCategory::Visual)
    {
        let native = object.native_path();
        let owner = native.rsplit('.').next().unwrap();
        for property in object
            .properties
            .iter()
            .filter(|property| property.controlled.is_none() && property.coerces.is_none())
        {
            let native = native_property(property);
            let method = if property
                .adapter
                .is_some_and(PropertyAdapter::mutates_native_collection)
            {
                format!("get_{native}")
            } else {
                format!("put_{native}")
            };
            let interface = metadata.resolve(owner, &method).unwrap().short_name();
            emit_native_property_arms(
                &mut output,
                &format!(
                    "(ObjectType::{}, PropertyId::{}, ",
                    object.name, property.name
                ),
                "element",
                interface,
                owner,
                native,
                &property.value,
                property.default.as_deref(),
                property.adapter,
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
    for object in &schema.objects {
        for property in &object.properties {
            let Some(feedback) = &property.feedback else {
                continue;
            };
            let event = property
                .controlled
                .as_ref()
                .or(property.coerces.as_ref())
                .unwrap();
            match feedback {
                Feedback::Exact => {
                    output.push_str(&format!(
                        "(ObjectType::{}, PropertyId::{}, Some(value)) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::Exact(Property {{ id: PropertyId::{}, \
                         value: value.clone() }}))),\n",
                        object.name, property.name, property.name
                    ));
                    let clear = match property.value.as_str() {
                        "Bool" => format!("PropertyValue::Bool({})", property.clear_feedback),
                        "Color" => {
                            "PropertyValue::Color(crate::Color::rgb(255, 255, 255))".to_string()
                        }
                        "String" => "PropertyValue::String(Rc::from(\"\"))".to_string(),
                        "F64" => "PropertyValue::F64(0.0)".to_string(),
                        "I32" => "PropertyValue::I32(0)".to_string(),
                        "OptionalBool" => format!(
                            "PropertyValue::OptionalBool(Some({}))",
                            property.clear_feedback
                        ),
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
                Feedback::Normalized => output.push_str(&format!(
                    "(ObjectType::{}, PropertyId::{}, _) => Some((EventId::{event}, \
                     FeedbackExpectation::Normalized {{ observation: None }})),\n",
                    object.name, property.name
                )),
                Feedback::DeferredExact => {
                    let clear = match property.value.as_str() {
                        "Bool" => format!("PropertyValue::Bool({})", property.clear_feedback),
                        "String" => "PropertyValue::String(Rc::from(\"\"))".to_string(),
                        "OptionalBool" => format!(
                            "PropertyValue::OptionalBool(Some({}))",
                            property.clear_feedback
                        ),
                        "OptionalF64" => "PropertyValue::OptionalF64(None)".to_string(),
                        "SelectionIndex" => "PropertyValue::SelectionIndex(None)".to_string(),
                        value => panic!(
                            "{}.{} has unsupported deferred exact feedback value {value}",
                            object.name, property.name
                        ),
                    };
                    output.push_str(&format!(
                        "(ObjectType::{}, PropertyId::{}, Some(value)) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::DeferredExact(Property {{ id: PropertyId::{}, \
                         value: value.clone() }}))),\n\
                         (ObjectType::{}, PropertyId::{}, None) => \
                         Some((EventId::{event}, \
                         FeedbackExpectation::DeferredExact(Property {{ id: PropertyId::{}, \
                         value: {clear} }}))),\n",
                        object.name,
                        property.name,
                        property.name,
                        object.name,
                        property.name,
                        property.name
                    ));
                }
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
            if matches!(
                property.adapter,
                Some(PropertyAdapter::PointerCapture | PropertyAdapter::PointerFocus)
            ) {
                let field = snake_case(&property.name);
                output.push_str(&format!(
                    "(Self::{}(object), PropertyId::{}, None) => {{ \
                     object.{field}.set(false); Some(Ok(())) }},\n\
                     (Self::{}(object), PropertyId::{}, \
                     Some(PropertyValue::Bool(value))) => {{ \
                     object.{field}.set(*value); Some(Ok(())) }},\n",
                    object.name, property.name, object.name, property.name
                ));
                continue;
            }
            if property.adapter == Some(PropertyAdapter::DropPolicy) {
                let field = snake_case(&property.name);
                output.push_str(&format!(
                    "(Self::{}(object), PropertyId::{}, None) => {{ \
                     *object.{field}.borrow_mut() = None; \
                     Some(object.value.cast::<native::IUIElement>().map_err(Into::into)\
                     .and_then(|object| object.SetAllowDrop(false).map_err(Into::into))) }},\n\
                     (Self::{}(object), PropertyId::{}, \
                     Some(PropertyValue::DragDropPolicy(value))) => {{ \
                     *object.{field}.borrow_mut() = Some(Rc::clone(value)); \
                     Some(object.value.cast::<native::IUIElement>().map_err(Into::into)\
                     .and_then(|object| object.SetAllowDrop(true).map_err(Into::into))) }},\n",
                    object.name, property.name, object.name, property.name
                ));
                continue;
            }
            let native = native_property(property);
            let method = if property
                .adapter
                .is_some_and(PropertyAdapter::mutates_native_collection)
            {
                format!("get_{native}")
            } else {
                format!("put_{native}")
            };
            let interface = if property
                .adapter
                .is_some_and(PropertyAdapter::uses_framework_element)
            {
                "IFrameworkElement"
            } else {
                metadata
                    .resolve(&native_name(object), &method)
                    .unwrap()
                    .short_name()
            };
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
                property.adapter,
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
            if relation.realization != Realization::Owned
                || relation.cardinality != Cardinality::One
            {
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
    object.native_path().rsplit('.').next().unwrap().to_string()
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
    adapter: Option<PropertyAdapter>,
    metadata: &tool_reactor_metadata::MetadataResolver,
) {
    if adapter.is_some_and(PropertyAdapter::uses_framework_element) {
        return;
    }
    if adapter == Some(PropertyAdapter::Uri) {
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
             .and_then(|property| object.ClearValue(&property).map_err(Into::into)))),\n\
             {pattern}Some(PropertyValue::String(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| native::Uri::CreateUri(value.as_ref())\
             .and_then(|value| object.Set{native}(&value)).map_err(Into::into))),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::PathData) {
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
             .and_then(|property| object.ClearValue(&property).map_err(Into::into)))),\n\
             {pattern}Some(PropertyValue::String(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| parse_path_data(value)\
             .and_then(|value| object.Set{native}(&value).map_err(Into::into)))),\n"
        ));
        return;
    }
    if matches!(
        adapter,
        Some(
            PropertyAdapter::ImplicitOpacityTransition
                | PropertyAdapter::ImplicitScale
                | PropertyAdapter::ImplicitScaleTransition
        )
    ) {
        if adapter == Some(PropertyAdapter::ImplicitScale) {
            output.push_str(&format!(
                "{pattern}None) => Some({target}.cast::<native::UIElement>().map_err(Into::into)\
                 .and_then(|object| set_implicit_scale(&object, 1.0))),\n\
                 {pattern}Some(PropertyValue::F64(value))) => \
                 Some({target}.cast::<native::UIElement>().map_err(Into::into)\
                 .and_then(|object| set_implicit_scale(&object, *value))),\n"
            ));
        } else {
            let helper = if adapter == Some(PropertyAdapter::ImplicitOpacityTransition) {
                "set_opacity_transition"
            } else {
                "set_scale_transition"
            };
            output.push_str(&format!(
                "{pattern}None) => Some({target}.cast::<native::UIElement>().map_err(Into::into)\
                 .and_then(|object| {helper}(&object, None))),\n\
                 {pattern}Some(PropertyValue::Duration(value))) => \
                 Some({target}.cast::<native::UIElement>().map_err(Into::into)\
                 .and_then(|object| {helper}(&object, Some(*value)))),\n"
            ));
        }
        return;
    }
    if adapter == Some(PropertyAdapter::RichEditText) {
        output.push_str(&format!(
            "{pattern}None) => Some(set_rich_edit_text(&{target}, \"\")),\n\
             {pattern}Some(PropertyValue::String(value))) => \
             Some(set_rich_edit_text(&{target}, value)),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::RichTextBlocks) {
        output.push_str(&format!(
            "{pattern}None) => Some(set_rich_text_blocks({target}, None)),\n\
             {pattern}Some(PropertyValue::RichText(value))) => \
             Some(set_rich_text_blocks({target}, Some(value))),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::ImageUri) {
        output.push_str(&format!(
            "{pattern}None) => Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| object.Set{native}(None).map_err(Into::into))),\n\
             {pattern}Some(PropertyValue::ImageSource(value))) => match value.value() {{\n\
             ImageSourceValue::Uri(value) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| uri_image(value)\
             .and_then(|image| object.Set{native}(&image).map_err(Into::into)))),\n\
             ImageSourceValue::Encoded(_) => None,\n\
             }},\n"
        ));
        return;
    }
    if matches!(
        adapter,
        Some(PropertyAdapter::GridRows | PropertyAdapter::GridColumns)
    ) {
        let rows = adapter == Some(PropertyAdapter::GridRows);
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
        if adapter == Some(PropertyAdapter::NativeColor) {
            output.push_str(&format!(
                "{pattern}Some(PropertyValue::Color(value))) => \
                 Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                 .and_then(|object| object.Set{native}(to_native_color(*value))\
                 .map_err(Into::into))),\n"
            ));
        } else {
            output.push_str(&format!(
                "{pattern}Some(PropertyValue::Color(value))) => \
                 Some({target}.cast::<native::{interface}>().map_err(Into::into)\
                 .and_then(|object| solid_color_brush(*value)\
                 .and_then(|brush| object.Set{native}(&brush).map_err(Into::into)))),\n"
            ));
        }
        return;
    }
    if value == "Brush" {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::Brush(Brush::Solid(value)))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| solid_color_brush(*value)\
             .and_then(|brush| object.Set{native}(&brush).map_err(Into::into)))),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::InspectableString) {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::String(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| {{ let value: IInspectable = \
             windows_reference::IReference::from(value.as_ref()).into(); \
             object.Set{native}(&value).map_err(Into::into) }})),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::InspectableStringList) {
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
    if adapter == Some(PropertyAdapter::SelectionIndex) {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::SelectionIndex(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| {{ let value = native_selection_index(*value)?; \
             object.Set{native}(value).map_err(Into::into) }})),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::NumberBoxValue) {
        output.push_str(&format!(
            "{pattern}Some(PropertyValue::OptionalF64(value))) => \
             Some({target}.cast::<native::{interface}>().map_err(Into::into)\
             .and_then(|object| object.Set{native}(native_number_box_value(*value))\
             .map_err(Into::into))),\n"
        ));
        return;
    }
    if adapter == Some(PropertyAdapter::RatingValue) {
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

fn has_local_property(object: &Object, name: &str) -> bool {
    object
        .properties
        .iter()
        .any(|property| property.name == name)
}

fn generate_bindings(
    schema: &Schema,
    metadata: &tool_reactor_metadata::MetadataResolver,
) -> String {
    let mut output = fs::read_to_string(workspace_path(BINDINGS_BASE)).unwrap();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    let mut generated = BTreeSet::new();
    if !schema.attached_properties.is_empty() || !schema.visual_properties.is_empty() {
        generated.insert("Microsoft::UI::Xaml::IDependencyObject::ClearValue".to_string());
    }
    if schema.objects.iter().any(|object| object.enabled) {
        generated.insert("Microsoft::UI::Xaml::Controls::IControl::get_IsEnabled".to_string());
        generated.insert("Microsoft::UI::Xaml::Controls::IControl::put_IsEnabled".to_string());
        generated.insert("Microsoft::UI::Xaml::Controls::Control::IsEnabledProperty".to_string());
    }
    if schema.layout_exit_transition {
        generated.insert("Microsoft::UI::Xaml::IScalarTransition::put_Duration".to_string());
        generated.insert("Microsoft::UI::Xaml::IUIElement::get_Opacity".to_string());
        generated.insert("Microsoft::UI::Xaml::IUIElement::put_OpacityTransition".to_string());
        generated.insert("Microsoft::UI::Xaml::ScalarTransition::CreateInstance".to_string());
    }
    if schema.objects.iter().any(|object| {
        object
            .properties
            .iter()
            .any(|property| property.adapter == Some(PropertyAdapter::ThemeBrush))
    }) {
        generated.extend([
            "Microsoft::UI::Xaml::IFrameworkElement::put_Style".to_string(),
            "Microsoft::UI::Xaml::Markup::XamlReader::Load".to_string(),
            "Microsoft::UI::Xaml::Style::{}".to_string(),
        ]);
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
        .filter(|object| !object.is_handwritten())
    {
        generated.insert(format!(
            "{}::CreateInstance",
            binding_path(&object.native_path())
        ));
        for property in &object.properties {
            let native = native_property(property);
            if matches!(
                property.adapter,
                Some(PropertyAdapter::PointerCapture | PropertyAdapter::PointerFocus)
            ) {
                continue;
            }
            if property.adapter == Some(PropertyAdapter::ResourceOverrides) {
                generated.extend([
                    "Microsoft::UI::Xaml::IFrameworkElement::get_Resources".to_string(),
                    "Microsoft::UI::Xaml::ResourceDictionary::{}".to_string(),
                ]);
                continue;
            }
            if property.adapter == Some(PropertyAdapter::ResourceStyle) {
                generated.extend([
                    "Microsoft::UI::Xaml::Application::Current".to_string(),
                    "Microsoft::UI::Xaml::IApplication::get_Resources".to_string(),
                    "Microsoft::UI::Xaml::IFrameworkElement::put_Style".to_string(),
                    "Microsoft::UI::Xaml::ResourceDictionary::{}".to_string(),
                    "Microsoft::UI::Xaml::Style::{}".to_string(),
                ]);
                continue;
            }
            if property.adapter == Some(PropertyAdapter::KeyAccelerators) {
                generated.extend([
                    "Microsoft::UI::Xaml::IUIElement::get_KeyboardAccelerators".to_string(),
                    "Microsoft::UI::Xaml::IUIElement::put_KeyboardAcceleratorPlacementMode"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyboardAccelerator::put_Key".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyboardAccelerator::put_Modifiers".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyboardAccelerator::Invoked".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyboardAcceleratorInvokedEventArgs::put_Handled"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::KeyboardAccelerator::CreateInstance".to_string(),
                    "Microsoft::UI::Xaml::Input::KeyboardAcceleratorPlacementMode::Hidden"
                        .to_string(),
                    "Windows::System::VirtualKey".to_string(),
                    "Windows::System::VirtualKeyModifiers".to_string(),
                ]);
                continue;
            }
            if property.adapter == Some(PropertyAdapter::RichEditText) {
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
            if property.adapter == Some(PropertyAdapter::RichTextBlocks) {
                generated.extend([
                    "Microsoft::UI::Xaml::Documents::Block::{}".to_string(),
                    "Microsoft::UI::Xaml::Documents::Hyperlink::CreateInstance".to_string(),
                    "Microsoft::UI::Xaml::Documents::IHyperlink::put_NavigateUri".to_string(),
                    "Microsoft::UI::Xaml::Documents::IParagraph::get_Inlines".to_string(),
                    "Microsoft::UI::Xaml::Documents::IRun::put_Text".to_string(),
                    "Microsoft::UI::Xaml::Documents::ISpan::get_Inlines".to_string(),
                    "Microsoft::UI::Xaml::Documents::ITextElement::put_FontStyle".to_string(),
                    "Microsoft::UI::Xaml::Documents::ITextElement::put_FontWeight".to_string(),
                    "Microsoft::UI::Xaml::Documents::Inline::{}".to_string(),
                    "Microsoft::UI::Xaml::Documents::LineBreak::CreateInstance".to_string(),
                    "Microsoft::UI::Xaml::Documents::Paragraph::CreateInstance".to_string(),
                    "Microsoft::UI::Xaml::Documents::Run::CreateInstance".to_string(),
                    "Windows::UI::Text::FontStyle::Italic".to_string(),
                    "Windows::UI::Text::FontWeight".to_string(),
                    "Windows::Foundation::Uri::CreateUri".to_string(),
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
                property.adapter,
                Some(PropertyAdapter::GridRows | PropertyAdapter::GridColumns)
            ) {
                let getter = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(object), &getter)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
                let (definition, dimension) = if property.adapter == Some(PropertyAdapter::GridRows)
                {
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
            match property.adapter {
                Some(PropertyAdapter::Uri) => {
                    generated.insert("Windows::Foundation::Uri::CreateUri".to_string());
                }
                Some(PropertyAdapter::ImageUri) => {
                    generated.extend([
                        "Microsoft::UI::Xaml::Media::ImageSource::{}".to_string(),
                        "Microsoft::UI::Xaml::Media::Imaging::BitmapImage::CreateInstance"
                            .to_string(),
                        "Microsoft::UI::Xaml::Media::Imaging::IBitmapImage::put_UriSource"
                            .to_string(),
                        "Microsoft::UI::Xaml::Media::Imaging::IBitmapSource::SetSourceAsync"
                            .to_string(),
                        "Microsoft::UI::Xaml::Media::Imaging::SvgImageSource::CreateInstance"
                            .to_string(),
                        "Microsoft::UI::Xaml::Media::Imaging::ISvgImageSource::put_UriSource"
                            .to_string(),
                        "Windows::Storage::Streams::InMemoryRandomAccessStream::CreateInstance"
                            .to_string(),
                        "Windows::Storage::Streams::IRandomAccessStream::{GetOutputStreamAt, Seek}"
                            .to_string(),
                        "Windows::Storage::Streams::DataWriter::CreateDataWriter".to_string(),
                        "Windows::Storage::Streams::IDataWriter::{WriteBytes, StoreAsync, DetachStream}"
                            .to_string(),
                        "Windows::Foundation::Uri::CreateUri".to_string(),
                    ]);
                    let getter = format!("get_{native}");
                    let interface = metadata
                        .resolve(&native_name(object), &getter)
                        .unwrap()
                        .full_path();
                    generated.insert(format!("{}::{getter}", binding_path(&interface)));
                }
                Some(PropertyAdapter::PathData) => {
                    generated.extend([
                        "Microsoft::UI::Xaml::Markup::XamlBindingHelper::ConvertValue".to_string(),
                        "Microsoft::UI::Xaml::Media::Geometry::{}".to_string(),
                        "Windows::UI::Xaml::Interop::TypeKind::Metadata".to_string(),
                        "Windows::UI::Xaml::Interop::TypeName".to_string(),
                    ]);
                }
                Some(PropertyAdapter::ImplicitOpacityTransition) => {
                    generated.extend([
                        "Microsoft::UI::Xaml::IScalarTransition::put_Duration".to_string(),
                        "Microsoft::UI::Xaml::IUIElement::put_OpacityTransition".to_string(),
                        "Microsoft::UI::Xaml::ScalarTransition::CreateInstance".to_string(),
                    ]);
                }
                Some(PropertyAdapter::ImplicitScale) => {
                    generated.extend([
                        "Microsoft::UI::Xaml::IFrameworkElement::get_ActualHeight".to_string(),
                        "Microsoft::UI::Xaml::IFrameworkElement::get_ActualWidth".to_string(),
                        "Microsoft::UI::Xaml::IUIElement::put_CenterPoint".to_string(),
                        "Microsoft::UI::Xaml::IUIElement::put_Scale".to_string(),
                        "Windows::Foundation::Numerics::Vector3".to_string(),
                    ]);
                }
                Some(PropertyAdapter::ImplicitScaleTransition) => {
                    generated.extend([
                        "Microsoft::UI::Xaml::IVector3Transition::put_Duration".to_string(),
                        "Microsoft::UI::Xaml::IUIElement::put_ScaleTransition".to_string(),
                        "Microsoft::UI::Xaml::Vector3Transition::CreateInstance".to_string(),
                    ]);
                }
                _ => {}
            }
            if property.readback {
                let getter = format!("get_{native}");
                let interface = metadata
                    .resolve(&native_name(object), &getter)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
            }
            if property.default.is_none()
                && !matches!(
                    property.adapter,
                    Some(
                        PropertyAdapter::ImplicitOpacityTransition
                            | PropertyAdapter::ImplicitScale
                            | PropertyAdapter::ImplicitScaleTransition
                            | PropertyAdapter::ImageUri
                    )
                )
            {
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
            if event.property_changed {
                let observed = event.observes.as_ref().unwrap();
                let property = object
                    .properties
                    .iter()
                    .find(|property| property.name == *observed)
                    .unwrap();
                let native = native_property(property);
                let (_, interface) = metadata
                    .dependency_property(&native_name(object), native)
                    .unwrap();
                generated.insert(format!(
                    "{}::get_{native}Property",
                    binding_path(&interface.full_path())
                ));
                generated.extend([
                    "Microsoft::UI::Xaml::DependencyPropertyChangedCallback".to_string(),
                    "Microsoft::UI::Xaml::IDependencyObject::{RegisterPropertyChangedCallback, UnregisterPropertyChangedCallback}".to_string(),
                ]);
            } else {
                let interface = metadata
                    .resolve(&native_name(object), &format!("add_{}", event.name))
                    .unwrap()
                    .full_path();
                let interface = binding_path(&interface);
                generated.insert(format!("{interface}::add_{}", event.name));
                generated.insert(format!("{interface}::remove_{}", event.name));
            }
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
                    "Microsoft::UI::Xaml::Input::IPointerRoutedEventArgs::get_KeyModifiers"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::IPointerRoutedEventArgs::get_Pointer".to_string(),
                ]);
                if event
                    .active_properties
                    .iter()
                    .any(|property| property == "capture_pointer_on_press")
                {
                    generated.extend([
                        "Microsoft::UI::Xaml::IUIElement::CapturePointer".to_string(),
                        "Microsoft::UI::Xaml::IUIElement::ReleasePointerCapture".to_string(),
                    ]);
                }
                if event
                    .active_properties
                    .iter()
                    .any(|property| property == "focus_on_pointer_release")
                {
                    generated.insert("Microsoft::UI::Xaml::IUIElement::Focus".to_string());
                    generated.insert("Microsoft::UI::Xaml::FocusState::Pointer".to_string());
                }
            }
            if event.value == "KeyEventInfo" {
                generated.extend([
                    "Microsoft::UI::Xaml::Input::IKeyRoutedEventArgs::get_Key".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyRoutedEventArgs::get_OriginalKey".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyRoutedEventArgs::get_KeyStatus".to_string(),
                    "Microsoft::UI::Xaml::Input::IKeyRoutedEventArgs::put_Handled".to_string(),
                    "Windows::System::VirtualKey".to_string(),
                    "Windows::UI::Core::CorePhysicalKeyStatus".to_string(),
                    "Windows::Win32::GetKeyboardState".to_string(),
                ]);
            }
            if event.value == "CharacterEventInfo" {
                generated.extend([
                    "Microsoft::UI::Xaml::Input::ICharacterReceivedRoutedEventArgs::get_Character"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::ICharacterReceivedRoutedEventArgs::get_KeyStatus"
                        .to_string(),
                    "Microsoft::UI::Xaml::Input::ICharacterReceivedRoutedEventArgs::put_Handled"
                        .to_string(),
                    "Windows::UI::Core::CorePhysicalKeyStatus".to_string(),
                    "Windows::Win32::GetKeyboardState".to_string(),
                ]);
            }
            if event.value == "FocusEventInfo" {
                generated.extend([
                    "Microsoft::UI::Xaml::IUIElement::get_FocusState".to_string(),
                    "Microsoft::UI::Xaml::IRoutedEventArgs::get_OriginalSource".to_string(),
                    "Microsoft::UI::Xaml::FocusState::Keyboard".to_string(),
                    "Microsoft::UI::Xaml::FocusState::Pointer".to_string(),
                    "Microsoft::UI::Xaml::FocusState::Programmatic".to_string(),
                    "Microsoft::UI::Xaml::FocusState::Unfocused".to_string(),
                    "Microsoft::UI::Dispatching::DispatcherQueuePriority::Low".to_string(),
                ]);
            }
            if matches!(event.value.as_str(), "DragKind" | "DroppedData") {
                generated.extend([
                    "Microsoft::UI::Xaml::IDragEventArgs::get_DataView".to_string(),
                    "Microsoft::UI::Xaml::IDragEventArgs::put_AcceptedOperation".to_string(),
                    "Windows::ApplicationModel::DataTransfer::DataPackageOperation::None"
                        .to_string(),
                    "Windows::ApplicationModel::DataTransfer::DataPackageOperation::Copy"
                        .to_string(),
                    "Windows::ApplicationModel::DataTransfer::DataPackageOperation::Move"
                        .to_string(),
                    "Windows::ApplicationModel::DataTransfer::DataPackageOperation::Link"
                        .to_string(),
                    "Windows::ApplicationModel::DataTransfer::IDataPackageView::Contains"
                        .to_string(),
                ]);
            }
            if event.value == "DragKind" {
                generated.extend([
                    "Microsoft::UI::Xaml::IDragEventArgs::get_DragUIOverride".to_string(),
                    "Microsoft::UI::Xaml::IDragUIOverride::put_Caption".to_string(),
                    "Microsoft::UI::Xaml::IDragUIOverride::put_IsCaptionVisible".to_string(),
                ]);
            }
            if event.value == "DroppedData" {
                generated.extend([
                    "Microsoft::UI::Xaml::IDragEventArgs::GetDeferral".to_string(),
                    "Microsoft::UI::Xaml::IDragOperationDeferral::Complete".to_string(),
                    "Windows::ApplicationModel::DataTransfer::IDataPackageView::GetStorageItemsAsync"
                        .to_string(),
                    "Windows::ApplicationModel::DataTransfer::IDataPackageView::GetTextAsync"
                        .to_string(),
                    "Windows::Storage::IStorageItem::get_Name".to_string(),
                    "Windows::Storage::IStorageItem::get_Path".to_string(),
                ]);
            }
            if event.value == "StringList" {
                generated.insert("Microsoft::UI::Xaml::IFrameworkElement::get_Tag".to_string());
                generated.insert(if object.name == "TabView" {
                    "Microsoft::UI::Xaml::Controls::ITabView::get_TabItems".to_string()
                } else {
                    "Microsoft::UI::Xaml::Controls::IItemsControl::get_Items".to_string()
                });
            }
            if let Some(payload) = &event.payload {
                let interface =
                    if event.payload_adapter == Some(PayloadAdapter::ContentDialogResult) {
                        metadata
                            .resolve("ContentDialogClosedEventArgs", "get_Result")
                            .unwrap()
                            .full_path()
                    } else if event.payload_adapter == Some(PayloadAdapter::InspectableString) {
                        metadata
                            .resolve_event_args_object_property(
                                &native_name(object),
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .unwrap()
                    } else if event.payload_adapter == Some(PayloadAdapter::ItemTag) {
                        metadata
                            .resolve_event_args_class_property(
                                &native_name(object),
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .unwrap()
                    } else if event.payload_adapter == Some(PayloadAdapter::NavigationDisplayMode) {
                        metadata
                            .resolve_event_args_property_interface(
                                &native_name(object),
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .unwrap()
                    } else {
                        metadata
                            .resolve_event_args_property(
                                &native_name(object),
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .unwrap()
                            .1
                    };
                generated.insert(format!("{}::get_{payload}", binding_path(&interface)));
            }
            if event.payload_adapter == Some(PayloadAdapter::ItemTag) {
                generated.insert("Microsoft::UI::Xaml::IFrameworkElement::get_Tag".to_string());
            }
            if event.payload_adapter == Some(PayloadAdapter::OptionalDateTime)
                && object.name == "CalendarDatePicker"
            {
                let interface = metadata
                    .resolve(&native_name(object), "put_Date")
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::put_Date", binding_path(&interface)));
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
                if property.adapter == Some(PropertyAdapter::RichEditText) {
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
            if relation.realization == Realization::Owned
                && relation.cardinality == Cardinality::One
            {
                let native_relation = relation.native.as_deref().unwrap_or(&relation.name);
                let method = format!("put_{native_relation}");
                let interface = metadata
                    .resolve(&native_name(object), &method)
                    .unwrap()
                    .full_path();
                generated.insert(format!("{}::{method}", binding_path(&interface)));
            } else if relation.realization == Realization::Owned
                && relation.cardinality == Cardinality::Many
            {
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
    for object in schema
        .objects
        .iter()
        .filter(|object| object.is_handwritten() && object.category == ObjectCategory::Visual)
    {
        let owner = object.native_path();
        let owner_name = owner.rsplit('.').next().unwrap();
        for property in object
            .properties
            .iter()
            .filter(|property| property.controlled.is_none() && property.coerces.is_none())
        {
            let native = native_property(property);
            let setter = format!("put_{native}");
            let interface = metadata.resolve(owner_name, &setter).unwrap().full_path();
            generated.insert(format!("{}::{setter}", binding_path(&interface)));
            if property.readback {
                let getter = format!("get_{native}");
                let interface = metadata.resolve(owner_name, &getter).unwrap().full_path();
                generated.insert(format!("{}::{getter}", binding_path(&interface)));
            }
            if property.default.is_none() {
                let (_, interface) = metadata
                    .dependency_property(owner_name, native)
                    .unwrap_or_else(|| {
                        panic!("cannot resolve {owner_name}.{native} dependency property")
                    });
                generated.insert(format!(
                    "{}::get_{native}Property",
                    binding_path(&interface.full_path())
                ));
            }
        }
        for event in &object.events {
            let interface = metadata
                .resolve(owner_name, &format!("add_{}", event.name))
                .unwrap()
                .full_path();
            let interface = binding_path(&interface);
            generated.insert(format!("{interface}::add_{}", event.name));
            generated.insert(format!("{interface}::remove_{}", event.name));
            if object.name == "TreeView" && event.name == "ItemInvoked" {
                generated.extend([
                    "Microsoft::UI::Xaml::Controls::ITreeViewItemInvokedEventArgs::get_InvokedItem"
                        .to_string(),
                ]);
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

fn generate(schema: &Schema, metadata: &tool_reactor_metadata::MetadataResolver) -> String {
    let mut properties = BTreeMap::new();
    let mut relations = BTreeMap::new();
    let mut events = BTreeMap::new();
    for property in &schema.attached_properties {
        properties.insert(property.name.as_str(), property.value.as_str());
    }
    for property in &schema.visual_properties {
        properties.insert(property.name.as_str(), property.value.as_str());
    }
    if schema.objects.iter().any(|object| object.enabled) {
        properties.insert("IsEnabled", "Bool");
    }
    events.insert("MenuItemInvoked", "String");
    events.insert("CommandInvoked", "String");
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

    let mut output = String::from("// This file is generated by tool-reactor.\n");
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
             Brush,\n\
             ButtonStyle,\n\
             ContentDialogResult,\n\
             CharacterEventInfo,\n\
             Color,\n\
             CornerRadius,\n\
             Duration,\n\
             DragDropPolicy,\n\
             DragKind,\n\
             DroppedData,\n\
             F64,\n\
             FocusEventInfo,\n\
             FontWeight,\n\
             GridLengths,\n\
             I32,\n\
             ImageSource,\n\
             KeyAccelerators,\n\
             KeyEventInfo,\n\
             NavigationViewDisplayMode,\n\
             OptionalBool,\n\
             OptionalDateTime,\n\
             OptionalF64,\n\
             OptionalTimeSpan,\n\
             PointerEventInfo,\n\
             ResourceOverrides,\n\
             RichText,\n\
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
            object.name,
            object.category.as_str()
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
            let owner = property.owner.rsplit('.').next().unwrap();
            let (_, variants) = metadata
                .static_enum_info(owner, &format!("Set{}", property.native))
                .unwrap();
            let variants = variants
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
    if schema.objects.iter().any(|object| object.enabled) {
        output.push_str("if id == PropertyId::IsEnabled && matches!(kind,");
        for (index, object) in schema
            .objects
            .iter()
            .filter(|object| object.enabled)
            .enumerate()
        {
            if index != 0 {
                output.push('|');
            }
            output.push_str(&format!("ObjectType::{}", object.name));
        }
        output.push_str(") { return Some(PropertyContract { id, value: ValueType::Bool }); }\n");
    }
    output.push_str("match (kind, id) {\n");
    for object in &schema.objects {
        for property in &object.properties {
            if property.name == "IsEnabled" && object.enabled {
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
        if object.enabled && !has_local_property(object, "IsEnabled") {
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
    if !schema.objects.iter().any(|object| object.focus) {
        output.push_str("_ if false");
    } else {
        for (index, object) in schema
            .objects
            .iter()
            .filter(|object| object.focus)
            .enumerate()
        {
            if index != 0 {
                output.push('|');
            }
            output.push_str(&format!("ObjectType::{}", object.name));
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
    metadata: &tool_reactor_metadata::MetadataResolver,
) -> String {
    let mut output = String::from("// This file is generated by tool-reactor.\n");
    let mut enums = BTreeMap::new();
    for property in &schema.attached_properties {
        if is_builtin_value(&property.value) {
            continue;
        }
        let owner = property.owner.rsplit('.').next().unwrap();
        let (_, variants) = metadata
            .static_enum_info(owner, &format!("Set{}", property.native))
            .unwrap();
        if let Some(previous) = enums.insert(property.value.as_str(), variants.to_vec()) {
            assert_eq!(previous, variants, "conflicting enum definitions");
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
            "impl {name} {{ pub(crate) fn property_value(self) -> PropertyValue {{\n"
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
    output.push_str("macro_rules! focus_methods { ($type:ty) => {\n");
    output.push_str(
        "pub fn element_ref<R>(mut self, reference: &R) -> Self\n\
         where R: CompatibleElementRef<$type> {\n\
         self.0.reference = Some(reference.erased_ref());\nself\n}\n",
    );
    output.push_str("}; }\n");
    output.push_str("macro_rules! reference_methods { ($type:ty) => {\n");
    output.push_str(
        "pub fn element_ref<R>(mut self, reference: &R) -> Self\n\
         where R: CompatibleElementRef<$type> {\n\
         self.0.reference = Some(reference.erased_ref());\nself\n}\n",
    );
    output.push_str("}; }\n");
    output.push_str("macro_rules! window_title_bar_methods { () => {\n");
    output.push_str(
        "pub fn preferred_height(mut self, height: WindowTitleBarHeight) -> Self {\n\
         self.0.window_title_bar = Some(height);\nself\n}\n",
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
        if matches!(property.value.as_str(), "CornerRadius" | "Thickness") {
            output.push_str(&format!("let {name} = {name}.into();\n"));
        }
        if let Some(validation) = property.validation {
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
            if matches!(property.value.as_str(), "CornerRadius" | "Thickness") {
                output.push_str(&format!("let {name} = {name}.into();\n"));
            }
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, {});\nself\n}}\n",
                property.name,
                property_value_parts(&property.value, &name)
            ));
        }
    }
    output.push_str("}; }\n");
    for object in &schema.objects {
        if object.tooltip_attachment {
            continue;
        }
        let content_dialog = object.content_dialog_attachment;
        output.push_str("#[derive(Clone, Debug, PartialEq)]\n");
        if content_dialog {
            output.push_str(&format!(
                "pub struct {}(pub(crate) Declaration, pub(crate) bool);\n",
                object.name
            ));
        } else {
            output.push_str(&format!("pub struct {}(Declaration);\n", object.name));
        }
        if object.focus || object.reference {
            output.push_str(&format!("impl ReferenceElement for {} {{}}\n", object.name));
        }

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
        if object.window_title_bar {
            output.push_str(
                "let declaration = declaration.window_title_bar(WindowTitleBarHeight::Standard);\n",
            );
        }
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
        if content_dialog {
            output.push_str("Self(declaration, false)\n}\n");
            output
                .push_str("pub fn is_open(mut self, open: bool) -> Self { self.1 = open; self }\n");
        } else {
            output.push_str("Self(declaration)\n}\n");
        }

        for property in object
            .properties
            .iter()
            .filter(|property| !property.required)
        {
            let name = property.method();
            if property.adapter == Some(PropertyAdapter::ImageUri) {
                output.push_str(&format!(
                    "pub fn {name}(mut self, value: impl Into<Rc<str>>) -> \
                     windows_core::Result<Self> {{\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::ImageSource(ImageSource::uri(value)?));\n\
                     Ok(self)\n\
                     }}\n\
                     pub fn {name}_optional<T>(mut self, value: Option<T>) -> \
                     windows_core::Result<Self> where T: Into<Rc<str>> {{\n\
                     if let Some(value) = value {{\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::ImageSource(ImageSource::uri(value)?));\n\
                     }}\n\
                     Ok(self)\n\
                     }}\n\
                     pub fn {name}_file(mut self, path: impl AsRef<Path>) -> \
                     windows_core::Result<Self> {{\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::ImageSource(ImageSource::file(path)?));\n\
                     Ok(self)\n\
                     }}\n\
                     pub fn {name}_data(mut self, value: EncodedImage) -> Self {{\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::ImageSource(ImageSource::encoded(value)));\n\
                     self\n\
                     }}\n",
                    property.name, property.name, property.name, property.name
                ));
                continue;
            }
            if property.adapter == Some(PropertyAdapter::Uri) {
                output.push_str(&format!(
                    "pub fn {name}(mut self, value: impl Into<Rc<str>>) -> \
                     windows_core::Result<Self> {{\n\
                     let value = value.into();\n\
                     validate_uri(&value)?;\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::String(value));\n\
                     Ok(self)\n\
                     }}\n\
                     pub fn {name}_optional<T>(mut self, value: Option<T>) -> \
                     windows_core::Result<Self> where T: Into<Rc<str>> {{\n\
                     if let Some(value) = value {{\n\
                     let value = value.into();\n\
                     validate_uri(&value)?;\n\
                     self.0 = self.0.property(PropertyId::{}, \
                     PropertyValue::String(value));\n\
                     }}\n\
                     Ok(self)\n\
                     }}\n",
                    property.name, property.name
                ));
                continue;
            }
            let argument = match property.adapter {
                Some(PropertyAdapter::RichEditText) => "text: impl Into<Rc<str>>".to_string(),
                Some(PropertyAdapter::GridRows | PropertyAdapter::GridColumns) => {
                    "values: impl IntoIterator<Item = GridLength>".to_string()
                }
                Some(PropertyAdapter::KeyAccelerators) => {
                    "key_accelerators: KeyAccelerators".to_string()
                }
                Some(PropertyAdapter::ResourceOverrides) => {
                    "resource_overrides: ResourceOverrides".to_string()
                }
                _ => property_argument_parts(&name, &property.value),
            };
            output.push_str(&format!("pub fn {name}(mut self, {argument}) -> Self {{\n"));
            if matches!(property.value.as_str(), "CornerRadius" | "Thickness") {
                output.push_str(&format!("let {name} = {name}.into();\n"));
            }
            if let Some(validation) = property.validation {
                let expression = validation_expression(&name, &property.value, validation);
                output.push_str(&format!(
                    "assert!({expression}, \"{}.{} requires {validation}\");\n",
                    object.name, property.name
                ));
            }
            let value = if property.adapter == Some(PropertyAdapter::RichEditText) {
                format!("PropertyValue::String(canonical_rich_edit_text({name}.into()))")
            } else if matches!(
                property.adapter,
                Some(PropertyAdapter::GridRows | PropertyAdapter::GridColumns)
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
        if object.enabled && !has_local_property(object, "IsEnabled") {
            output.push_str("enabled_methods!();\n");
        }
        if object.focus {
            output.push_str(&format!("focus_methods!({});\n", object.name));
        }
        if object.reference {
            output.push_str(&format!("reference_methods!({});\n", object.name));
        }
        if object.window_title_bar {
            output.push_str("window_title_bar_methods!();\n");
        }

        if object.category == ObjectCategory::Visual && !content_dialog {
            output.push_str("visual_methods!();\n");
        }

        if object.virtual_items {
            let relation = object
                .relations
                .iter()
                .find(|relation| relation.realization == Realization::Container)
                .unwrap();
            output.push_str(&format!(
                "                pub fn item(mut self, key: impl Into<Key>, visual: impl Into<View>) -> Self {{\n\
                self.0 = self.0.virtual_item(RelationId::{}, keyed(key, visual));\nself\n}}\n\
                 pub fn items(mut self, items: impl IntoIterator<Item = KeyedView>) -> Self {{\n\
                 self.0 = self.0.virtual_items(RelationId::{}, \
                 VirtualItems::eager(items));\nself\n}}\n\
                 pub fn virtual_source(mut self, source: VirtualSource) -> Self {{\n\
                 self.0 = self.0.virtual_items(RelationId::{}, \
                 VirtualItems::Lazy(source));\nself\n}}\n",
                relation.name, relation.name, relation.name
            ));
        }

        for relation in &object.relations {
            if object.virtual_items && relation.realization == Realization::Container {
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
                ("Visual", "Many", "Keyed") => "TreeNode::new(\"node\", \"Node\")",
                ("Visual", "Many", "Positional") => {
                    "keyed(\"text\", TextBlock::new().text(\"Text\"))"
                }
                ("Structural", "Many", _) | ("Data", "Many", _) => {
                    "TextBlock::new().text(\"Text\")"
                }
                _ => unreachable!("unsupported relation shape"),
            };
            let argument = if relation.cardinality == Cardinality::One {
                invalid.to_string()
            } else {
                format!("[{invalid}]")
            };
            output.push_str(&format!(
                "/// Rejects values outside this relation's generated type contract.\n\
                 ///\n\
                 /// ```compile_fail\n\
                 /// use windows_reactor::*;\n\
                 /// let _ = {constructor}.{method}({argument});\n\
                 /// ```\n",
            ));
            match relation.cardinality {
                Cardinality::One => {
                    if relation.name == "Icon" {
                        output.push_str(&format!(
                            "pub fn {method}(mut self, content: impl Into<Icon>) -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::One(Some(Rc::new(\
                             content.into().into_view().0))));\nself\n}}\n",
                            relation.name
                        ));
                    } else {
                        output.push_str(&format!(
                            "pub fn {method}(mut self, content: impl Into<View>) -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::One(Some(Rc::new(content.into().0))));\nself\n}}\n",
                            relation.name
                        ));
                    }
                }
                Cardinality::Many => {
                    let item = relation.item.as_deref().unwrap_or_else(|| {
                        if relation.identity == Identity::Keyed {
                            "KeyedView"
                        } else {
                            "View"
                        }
                    });
                    let item = if item == object.name { "Self" } else { item };
                    let extract = match item {
                        "KeyedView" => "child.0.0",
                        "View" => "child.0",
                        _ => "DeclaredNode::Object(child.0)",
                    };
                    if item == "View" {
                        output.push_str(&format!(
                            "pub fn {method}(mut self, children: impl IntoViews) -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::Many(Rc::new(children.into_visuals().into_iter().map(\
                             |child| {extract}).collect())));\nself\n}}\n",
                            relation.name
                        ));
                    } else if item == "KeyedView" {
                        output.push_str(&format!(
                            "pub fn {method}(mut self, children: impl IntoViews) -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::Many(Rc::new(children.into_visuals().into_iter().enumerate().map(\
                             |(index, child)| child.0.positional_key(index)).collect())));\nself\n}}\n",
                            relation.name
                        ));
                        output.push_str(&format!(
                            "pub fn keyed_{method}(mut self, children: impl IntoIterator<Item = \
                             KeyedView>) -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::Many(Rc::new(children.into_iter().map(|child| \
                             child.0.0).collect())));\nself\n}}\n",
                            relation.name
                        ));
                    } else {
                        output.push_str(&format!(
                            "pub fn {method}(mut self, children: impl IntoIterator<Item = {item}>) \
                             -> Self {{\n"
                        ));
                        output.push_str(&format!(
                            "self.0 = self.0.relation(RelationId::{}, \
                             RelationValue::Many(Rc::new(children.into_iter().map(|child| \
                             {extract}).collect())));\nself\n}}\n",
                            relation.name
                        ));
                    }
                }
            }
        }

        for event in &object.events {
            let method = event
                .field
                .as_deref()
                .and_then(|field| field.strip_prefix("on_"))
                .map_or_else(|| snake_case(&event.name), str::to_string);
            match event.value.as_str() {
                "CharacterEventInfo" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: \
                         RoutedCallback<CharacterEventInfo>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, EventValue::CharacterEventInfo(callback)); self \
                         }}\n",
                        event.name
                    ));
                }
                "KeyEventInfo" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: RoutedCallback<KeyEventInfo>) -> \
                         Self {{ self.0 = self.0.event(EventId::{}, \
                         EventValue::KeyEventInfo(callback)); self }}\n",
                        event.name
                    ));
                }
                "Unit" => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: impl IntoUnitCallback) -> Self {{ \
                         self.0 = self.0.event(EventId::{}, \
                         EventValue::Unit(callback.into_unit_callback())); self }}\n",
                        event.name
                    ));
                }
                value => {
                    let callback_type = match value {
                        "String" => "Rc<str>",
                        "StringList" => "Vec<String>",
                        "Bool" => "bool",
                        "Color" => "Color",
                        "ContentDialogResult" => "ContentDialogResult",
                        "F64" => "f64",
                        "FocusEventInfo" => "FocusEventInfo",
                        "DragKind" => "DragKind",
                        "DroppedData" => "DroppedData",
                        "OptionalBool" => "Option<bool>",
                        "OptionalDateTime" => "Option<DateTime>",
                        "OptionalF64" => "Option<f64>",
                        "OptionalTimeSpan" => "Option<TimeSpan>",
                        "NavigationViewDisplayMode" => "NavigationViewDisplayMode",
                        "Selection" => "Option<Rc<str>>",
                        "PointerEventInfo" => "PointerEventInfo",
                        "SelectionIndex" => "Option<usize>",
                        _ => unreachable!(),
                    };
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: impl \
                         IntoPayloadCallback<{callback_type}>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, \
                         EventValue::{value}(callback.into_payload_callback())); self }}\n",
                        event.name
                    ));
                }
            }
        }
        output.push_str("}\n");

        if !object.key && object.properties.iter().all(|property| !property.required) {
            output.push_str(&format!(
                "impl Default for {} {{ fn default() -> Self {{ Self::new() }} }}\n",
                object.name
            ));
        }
        if object.category == ObjectCategory::Visual && !content_dialog {
            output.push_str(&format!(
                "impl From<{}> for View {{ fn from(value: {}) -> Self {{ \
                 Self(DeclaredNode::Object(value.0)) }} }}\n",
                object.name, object.name
            ));
        }
    }
    output.push_str(
        "impl From<&str> for View { fn from(value: &str) -> Self { TextBlock::new().text(value).into() } }\n\
         impl From<String> for View { fn from(value: String) -> Self { TextBlock::new().text(value).into() } }\n\
         impl From<Rc<str>> for View { fn from(value: Rc<str>) -> Self { TextBlock::new().text(value).into() } }\n",
    );
    output
}

fn property_argument(property: &Property) -> String {
    property_argument_parts(&property.name, &property.value)
}

fn validation_expression(name: &str, value: &str, validation: Validation) -> String {
    match validation {
        Validation::Finite => format!("{name}.is_finite()"),
        Validation::FinitePositive => format!("{name}.is_finite() && {name} > 0.0"),
        Validation::FiniteNonNegative if value == "F64" => {
            format!("{name}.is_finite() && {name} >= 0.0")
        }
        Validation::FiniteNonNegative => format!("{name}.is_finite_non_negative()"),
        Validation::NonNegative => format!("{name} >= 0"),
        Validation::Positive => format!("{name} > 0"),
        Validation::ZeroToFiftyNine => format!("(0..=59).contains(&{name})"),
    }
}

fn property_argument_parts(name: &str, value: &str) -> String {
    let name = snake_case(name);
    match value {
        "String" => format!("{name}: impl AsRef<str>"),
        "Bool" => format!("{name}: bool"),
        "Brush" => format!("{name}: impl Into<Brush>"),
        "ButtonStyle" => format!("{name}: ButtonStyle"),
        "Color" => format!("{name}: Color"),
        "CornerRadius" => format!("{name}: impl Into<CornerRadius>"),
        "Duration" => format!("{name}: Duration"),
        "DragDropPolicy" => format!("{name}: DragDropPolicy"),
        "F64" => format!("{name}: f64"),
        "FontWeight" => format!("{name}: FontWeight"),
        "GridLengths" => format!("{name}: impl IntoIterator<Item = GridLength>"),
        "I32" => format!("{name}: i32"),
        "ImageSource" => format!("{name}: ImageSource"),
        "KeyAccelerators" => format!("{name}: KeyAccelerators"),
        "OptionalF64" => format!("{name}: impl Into<Option<f64>>"),
        "OptionalBool" => format!("{name}: impl Into<Option<bool>>"),
        "ResourceOverrides" => format!("{name}: ResourceOverrides"),
        "SelectionIndex" => format!("{name}: impl Into<Option<usize>>"),
        "StringList" => format!("{name}: impl IntoIterator<Item = impl Into<Rc<str>>>"),
        "Thickness" => format!("{name}: impl Into<Thickness>"),
        value => format!("{name}: {value}"),
    }
}

fn property_value(property: &Property, name: &str) -> String {
    property_value_parts(&property.value, name)
}

fn property_value_parts(value: &str, name: &str) -> String {
    match value {
        "String" => format!("PropertyValue::String(Rc::from({name}.as_ref()))"),
        "Bool" => format!("PropertyValue::Bool({name})"),
        "Brush" => format!("PropertyValue::Brush({name}.into())"),
        "ButtonStyle" => format!("PropertyValue::ButtonStyle({name})"),
        "Color" => format!("PropertyValue::Color({name})"),
        "CornerRadius" => format!("PropertyValue::CornerRadius({name})"),
        "Duration" => format!("PropertyValue::Duration({name})"),
        "DragDropPolicy" => format!("PropertyValue::DragDropPolicy(Rc::new({name}))"),
        "F64" => format!("PropertyValue::F64({name})"),
        "FontWeight" => format!("PropertyValue::FontWeight({name})"),
        "GridLengths" => {
            format!("PropertyValue::GridLengths({name}.into_iter().collect())")
        }
        "I32" => format!("PropertyValue::I32({name})"),
        "ImageSource" => format!("PropertyValue::ImageSource({name})"),
        "KeyAccelerators" => format!("PropertyValue::KeyAccelerators({name})"),
        "OptionalF64" => format!("PropertyValue::OptionalF64({name}.into())"),
        "OptionalBool" => format!("PropertyValue::OptionalBool({name}.into())"),
        "SelectionIndex" => format!("PropertyValue::SelectionIndex({name}.into())"),
        "ResourceOverrides" => format!("PropertyValue::ResourceOverrides({name})"),
        "RichText" => format!("PropertyValue::RichText({name})"),
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
        "Brush" => "None::<&native::Brush>".to_string(),
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

fn generate_live_coverage(schema: &Schema) -> String {
    let mut output = String::from(
        "use windows_reactor as reactor;\n\
         use reactor::*;\n\
         pub struct CoverageCase {\n\
             pub contract: &'static str,\n\
             pub set: fn() -> View,\n\
             pub clear: fn() -> View,\n\
         }\n\
         pub fn cases() -> Vec<CoverageCase> {\n\
             vec![\n",
    );
    let mut functions = String::new();
    let mut index = 0;
    let representative = schema
        .objects
        .iter()
        .find(|object| object.name == "TextBlock")
        .unwrap();

    for property in &schema.attached_properties {
        write_live_property_case(
            &mut output,
            &mut functions,
            &mut index,
            "attached",
            representative,
            &property.name,
            &snake_case(&property.name),
            &property.value,
            None,
            property.default.as_deref(),
            property.flag,
            false,
        );
    }
    for property in &schema.visual_properties {
        write_live_property_case(
            &mut output,
            &mut functions,
            &mut index,
            "visual",
            representative,
            &property.name,
            &snake_case(&property.name),
            &property.value,
            None,
            property.default.as_deref(),
            false,
            false,
        );
    }
    for object in &schema.objects {
        if object.category != ObjectCategory::Visual || live_coverage_specialized(object) {
            continue;
        }
        for property in &object.properties {
            let method = property.method();
            write_live_property_case(
                &mut output,
                &mut functions,
                &mut index,
                &object.name,
                object,
                &property.name,
                &method,
                &property.value,
                property.adapter,
                property.default.as_deref(),
                false,
                property.required,
            );
        }
        for event in &object.events {
            let method = event
                .field
                .as_deref()
                .and_then(|field| field.strip_prefix("on_"))
                .map_or_else(|| snake_case(&event.name), str::to_string);
            let set_name = format!("coverage_{index}_set");
            let clear_name = format!("coverage_{index}_clear");
            writeln!(
                output,
                "CoverageCase {{ contract: \"{}.{}\", set: {set_name}, clear: {clear_name} }},",
                object.name, event.name
            )
            .unwrap();
            let constructor = example_constructor(object);
            let callback = if event.routed {
                match event.value.as_str() {
                    "CharacterEventInfo" => "RoutedCallback::new(|_| false)",
                    "KeyEventInfo" => "RoutedCallback::new(|_| false)",
                    value => unreachable!("unsupported routed live coverage event {value}"),
                }
                .to_string()
            } else if event.value == "Unit" {
                "|| {}".to_string()
            } else {
                "|_| {}".to_string()
            };
            writeln!(
                functions,
                "fn {set_name}() -> View {{ {constructor}.on_{method}({callback}).into() }}\n\
                 fn {clear_name}() -> View {{ {constructor}.into() }}"
            )
            .unwrap();
            index += 1;
        }
    }
    output.push_str("]\n}\n");
    output.push_str(&functions);
    output
}

fn generate_live_coverage_report(schema: &Schema) -> String {
    let property_count = schema.attached_properties.len()
        + schema.visual_properties.len()
        + schema
            .objects
            .iter()
            .map(|object| object.properties.len())
            .sum::<usize>();
    let event_count = schema
        .objects
        .iter()
        .map(|object| object.events.len())
        .sum::<usize>();
    let mut output = format!(
        "# Reactor live coverage matrix\n\n\
         Generated by `cargo run -p tool-reactor --quiet`.\n\n\
         The live selftest executes all {property_count} schema property definitions and \
         {event_count} event definitions through the evidence routes below.\n\n\
         | Contract | Kind | Native evidence | Behavioral evidence |\n\
         | --- | --- | --- | --- |\n"
    );
    for property in &schema.attached_properties {
        writeln!(
            output,
            "| attached.{} | property | generated set/clear | shared attached-property path |",
            property.name
        )
        .unwrap();
    }
    for property in &schema.visual_properties {
        writeln!(
            output,
            "| visual.{} | property | generated set/clear | shared visual-property path |",
            property.name
        )
        .unwrap();
    }
    for object in &schema.objects {
        for property in &object.properties {
            writeln!(
                output,
                "| {}.{} | property | {} | {} |",
                object.name,
                property.name,
                live_native_evidence(object, "generated set/clear"),
                live_property_behavior(property)
            )
            .unwrap();
        }
        for event in &object.events {
            writeln!(
                output,
                "| {}.{} | event | {} | {} |",
                object.name,
                event.name,
                live_native_evidence(object, "generated subscribe/remove"),
                live_event_behavior(event)
            )
            .unwrap();
        }
    }
    output
}

fn live_native_evidence<'a>(object: &Object, generated: &'a str) -> &'a str {
    if live_coverage_specialized(object) {
        "handwritten lifecycle phase"
    } else if object.category != ObjectCategory::Visual {
        "handwritten collection phase"
    } else {
        generated
    }
}

fn live_property_behavior(property: &Property) -> &'static str {
    match property.adapter {
        Some(PropertyAdapter::DropPolicy) => "drag/drop policy phase",
        Some(PropertyAdapter::ImageUri) => "image source/readback phase",
        Some(PropertyAdapter::NativeColor) => "color feedback/readback phase",
        Some(PropertyAdapter::RichEditText) => "rich-edit feedback phase",
        Some(PropertyAdapter::RichTextBlocks) => "rich-text collection/readback phase",
        Some(PropertyAdapter::SelectionIndex) => "selection feedback/readback phase",
        Some(PropertyAdapter::Uri) => "validated URI phase",
        Some(_) => "adapter-family phase",
        None if property.controlled.is_some() || property.feedback.is_some() => {
            "controlled feedback phase"
        }
        None if property.readback => "native readback phase",
        None => "native set/clear smoke",
    }
}

fn live_event_behavior(event: &Event) -> &'static str {
    match event.value.as_str() {
        "CharacterEventInfo" | "KeyEventInfo" => "routed input payload phase",
        "Color" => "color feedback/readback phase",
        "ContentDialogResult" => "content-dialog lifecycle phase",
        "DragKind" | "DroppedData" => "drag/drop payload phase",
        "FocusEventInfo" => "focus payload phase",
        "NavigationViewDisplayMode" => "navigation display-mode phase",
        "OptionalDateTime" | "OptionalTimeSpan" => "nullable date/time phase",
        "PointerEventInfo" => "pointer payload and real-input phase",
        "Selection" | "SelectionIndex" => "selection payload phase",
        "String" | "StringList" => "string item/tag payload phase",
        _ => "typed event dispatch phase",
    }
}

#[allow(clippy::too_many_arguments)]
fn write_live_property_case(
    output: &mut String,
    functions: &mut String,
    index: &mut usize,
    owner: &str,
    object: &Object,
    property_name: &str,
    method: &str,
    value: &str,
    adapter: Option<PropertyAdapter>,
    default: Option<&str>,
    flag: bool,
    required: bool,
) {
    let set_name = format!("coverage_{}_set", *index);
    let clear_name = format!("coverage_{}_clear", *index);
    writeln!(
        output,
        "CoverageCase {{ contract: \"{owner}.{property_name}\", set: {set_name}, clear: \
         {clear_name} }},"
    )
    .unwrap();
    let constructor = example_constructor(object);
    let sample = live_property_sample(property_name, value, adapter, default);
    let set = if required {
        constructor.clone()
    } else if flag {
        format!("{constructor}.{method}()")
    } else if matches!(
        adapter,
        Some(PropertyAdapter::Uri | PropertyAdapter::ImageUri)
    ) {
        format!("{constructor}.{method}({sample}).unwrap()")
    } else {
        format!("{constructor}.{method}({sample})")
    };
    writeln!(
        functions,
        "fn {set_name}() -> View {{ {set}.into() }}\n\
         fn {clear_name}() -> View {{ {constructor}.into() }}"
    )
    .unwrap();
    *index += 1;
}

fn live_coverage_specialized(object: &Object) -> bool {
    matches!(
        object.name.as_str(),
        "TitleBar" | "ToolTip" | "ContentDialog"
    )
}

fn live_property_sample(
    name: &str,
    value: &str,
    adapter: Option<PropertyAdapter>,
    default: Option<&str>,
) -> String {
    if matches!(
        adapter,
        Some(PropertyAdapter::Uri | PropertyAdapter::ImageUri)
    ) {
        return "\"https://example.com\"".to_string();
    }
    match value {
        "String" if name == "ClockIdentifier" => "\"24HourClock\"".to_string(),
        "String" if adapter == Some(PropertyAdapter::PathData) => "\"M 0,0 L 1,1\"".to_string(),
        "String" => "\"coverage\"".to_string(),
        "Bool" if matches!(name, "IsOpen" | "IsCalendarOpen") => "false".to_string(),
        "Bool" => match default {
            Some("true") => "false".to_string(),
            _ => "true".to_string(),
        },
        "F64" if name == "Opacity" => "0.5".to_string(),
        "F64" => "1.0".to_string(),
        "I32" => "1".to_string(),
        "Brush" => "Color::rgb(1, 2, 3)".to_string(),
        "ButtonStyle" => "ButtonStyle::Default".to_string(),
        "Color" => "Color::rgb(1, 2, 3)".to_string(),
        "CornerRadius" => "CornerRadius::uniform(1.0)".to_string(),
        "Duration" => "std::time::Duration::from_millis(1)".to_string(),
        "DragDropPolicy" => "DragDropPolicy::new()".to_string(),
        "FontWeight" => "FontWeight::SEMI_BOLD".to_string(),
        "GridLengths" => "[GridLength::Auto]".to_string(),
        "KeyAccelerators" => "KeyAccelerators::default()".to_string(),
        "OptionalBool" => "Some(true)".to_string(),
        "OptionalF64" => "Some(1.0)".to_string(),
        "ResourceOverrides" => "ResourceOverrides::new()".to_string(),
        "RichText" => "RichText::new([RichTextParagraph::new([RichTextInline::Run(\
                       RichTextRun::plain(\"coverage\"))])])"
            .to_string(),
        "SelectionIndex" => "Some(0)".to_string(),
        "StringList" => "[\"coverage\"]".to_string(),
        "ThemeTransitions" => "[ThemeTransition::Reposition]".to_string(),
        "Thickness" => "Thickness::uniform(1.0)".to_string(),
        "AutomationHeadingLevel" => "AutomationHeadingLevel::Level1".to_string(),
        "HorizontalAlignment" => "HorizontalAlignment::Center".to_string(),
        "InfoBarSeverity" => "InfoBarSeverity::Informational".to_string(),
        "ListViewSelectionMode" => "ListViewSelectionMode::Single".to_string(),
        "NavigationViewBackButtonVisible" => "NavigationViewBackButtonVisible::Visible".to_string(),
        "NavigationViewPaneDisplayMode" => "NavigationViewPaneDisplayMode::Left".to_string(),
        "Orientation" => "Orientation::Horizontal".to_string(),
        "PasswordRevealMode" => "PasswordRevealMode::Visible".to_string(),
        "ScrollBarVisibility" => "ScrollBarVisibility::Visible".to_string(),
        "ScrollingScrollBarVisibility" => "ScrollingScrollBarVisibility::Visible".to_string(),
        "SplitViewDisplayMode" => "SplitViewDisplayMode::Inline".to_string(),
        "Stretch" => "Stretch::Uniform".to_string(),
        "Symbol" => "Symbol::Previous".to_string(),
        "TeachingTipPlacementMode" => "TeachingTipPlacementMode::Top".to_string(),
        "TextTrimming" => "TextTrimming::CharacterEllipsis".to_string(),
        "TextWrapping" => "TextWrapping::Wrap".to_string(),
        "TreeViewSelectionMode" => "TreeViewSelectionMode::Single".to_string(),
        "VerticalAlignment" => "VerticalAlignment::Center".to_string(),
        value => unreachable!("unsupported live coverage value {value}"),
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
            "Brush" => "Color::rgb(0, 0, 0)".to_string(),
            "ButtonStyle" => "ButtonStyle::Default".to_string(),
            "Color" => "Color::rgb(0, 0, 0)".to_string(),
            "CornerRadius" => "CornerRadius::uniform(0.0)".to_string(),
            "Duration" => "std::time::Duration::ZERO".to_string(),
            "F64" => "0.0".to_string(),
            "I32" => "0".to_string(),
            "KeyAccelerators" => "KeyAccelerators::default()".to_string(),
            "OptionalF64" => "None".to_string(),
            "OptionalBool" => "None".to_string(),
            "SelectionIndex" => "None".to_string(),
            "ResourceOverrides" => "ResourceOverrides::default()".to_string(),
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
            | "Brush"
            | "ButtonStyle"
            | "Color"
            | "CornerRadius"
            | "Duration"
            | "DragDropPolicy"
            | "DragKind"
            | "DroppedData"
            | "F64"
            | "FontWeight"
            | "GridLengths"
            | "I32"
            | "ImageSource"
            | "KeyAccelerators"
            | "OptionalDateTime"
            | "OptionalF64"
            | "OptionalBool"
            | "SelectionIndex"
            | "Selection"
            | "ResourceOverrides"
            | "RichText"
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
