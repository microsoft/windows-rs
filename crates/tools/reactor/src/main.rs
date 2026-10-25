use serde::Deserialize;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod native;
use native::{dependency_filters, dependency_properties, generate_native};

const SCHEMA: &str = "crates/tools/reactor/src/schema.toml";
const OUTPUT: &str = "crates/libs/reactor/src/generated.rs";
const DECLARATIONS_OUTPUT: &str = "crates/libs/reactor/src/generated_declarations.rs";
const NATIVE_OUTPUT: &str = "crates/libs/reactor/src/native/generated.rs";
const WINMD: &str = "crates/tools/reactor-metadata/winmd";
const BINDINGS_FILTER: &str = "crates/tools/reactor/src/bindings.txt";
const BINDINGS_OUTPUT: &str = "crates/libs/reactor/src/native/bindings.rs";
const CANVAS_FILTER: &str = "crates/tools/reactor/src/canvas.txt";
const CANVAS_BINDINGS_OUTPUT: &str = "crates/libs/canvas/src/reactor_bindings.rs";

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Schema {
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
    value: Value,
    #[serde(default)]
    flag: bool,
    validation: Option<Validation>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct VisualProperty {
    name: String,
    owner: String,
    #[serde(default)]
    value: Value,
    adapter: Option<PropertyAdapter>,
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
    #[serde(rename = "datetime")]
    DateTime,
    DropPolicy,
    EditText,
    FontFamily,
    GridColumns,
    GridRows,
    IconElement,
    IconSource,
    ImageUri,
    ImplicitOpacityTransition,
    ImplicitScale,
    ImplicitScaleTransition,
    ImplicitTranslation,
    ImplicitTranslationTransition,
    InspectableString,
    InspectableStringList,
    KeyAccelerators,
    NativeColor,
    NumberBoxValue,
    #[serde(rename = "optional_datetime")]
    OptionalDateTime,
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
    ThemeTransitions,
    Uri,
}

impl PropertyAdapter {
    fn value(self) -> Value {
        match self {
            Self::DateTime => Value::DateTime,
            Self::DropPolicy => Value::DragDropPolicy,
            Self::GridColumns | Self::GridRows => Value::GridLengths,
            Self::IconElement | Self::IconSource => Value::Icon,
            Self::ImageUri => Value::ImageSource,
            Self::ImplicitOpacityTransition
            | Self::ImplicitScaleTransition
            | Self::ImplicitTranslationTransition => Value::Duration,
            Self::ImplicitScale => Value::F64,
            Self::ImplicitTranslation => Value::Vector2,
            Self::EditText
            | Self::FontFamily
            | Self::InspectableString
            | Self::PathData
            | Self::RichEditText
            | Self::Uri => Value::String,
            Self::InspectableStringList => Value::StringList,
            Self::KeyAccelerators => Value::KeyAccelerators,
            Self::NativeColor => Value::Color,
            Self::NumberBoxValue | Self::RatingValue => Value::OptionalF64,
            Self::OptionalDateTime => Value::OptionalDateTime,
            Self::PointerCapture | Self::PointerFocus => Value::Bool,
            Self::ResourceOverrides => Value::ResourceOverrides,
            Self::ResourceStyle => Value::ButtonStyle,
            Self::RichTextBlocks => Value::RichText,
            Self::SelectionIndex => Value::SelectionIndex,
            Self::ThemeBrush => Value::Brush,
            Self::ThemeTransitions => Value::ThemeTransitions,
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
            Self::PointerCapture | Self::PointerFocus | Self::DropPolicy | Self::ImageUri
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
    ItemKey,
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
    fn value(self) -> Value {
        match self {
            Self::ContentDialogResult => Value::ContentDialogResult,
            Self::InspectableString => Value::String,
            Self::ItemKey => Value::Key,
            Self::NativeColor => Value::Color,
            Self::NavigationDisplayMode => Value::NavigationViewDisplayMode,
            Self::NumberBoxValue | Self::RatingValue => Value::OptionalF64,
            Self::OptionalDateTime => Value::OptionalDateTime,
            Self::OptionalTimeSpan => Value::OptionalTimeSpan,
            Self::SelectionIndex => Value::SelectionIndex,
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
}

impl EventAdapter {
    fn value(self) -> Value {
        match self {
            Self::Character => Value::CharacterEventInfo,
            Self::DragKind => Value::DragKind,
            Self::DroppedData => Value::DroppedData,
            Self::Focus => Value::FocusEventInfo,
            Self::ItemTag => Value::String,
            Self::Key => Value::KeyEventInfo,
            Self::Pointer => Value::PointerEventInfo,
        }
    }
}

macro_rules! values {
    ($($name:ident),*) => {
        // A schema value type. Names not listed here are schema enums such as `Orientation`.
        #[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
        #[serde(from = "String")]
        enum Value {
            #[default]
            Unresolved,
            $($name,)*
            Enum(String),
        }

        impl From<String> for Value {
            fn from(value: String) -> Self {
                match value.as_str() {
                    "" => Self::Unresolved,
                    "Str" => Self::String,
                    $(stringify!($name) => Self::$name,)*
                    _ => Self::Enum(value),
                }
            }
        }

        impl Value {
            fn name(&self) -> &str {
                match self {
                    Self::Unresolved => panic!("unresolved value"),
                    $(Self::$name => stringify!($name),)*
                    Self::Enum(name) => name,
                }
            }
        }
    };
}

values!(
    Bool,
    Brush,
    ButtonStyle,
    CharacterEventInfo,
    Color,
    ContentDialogResult,
    CornerRadius,
    DateTime,
    DragDropPolicy,
    DragKind,
    DroppedData,
    Duration,
    F64,
    FocusEventInfo,
    FontWeight,
    GridLengths,
    I32,
    Icon,
    ImageSource,
    Key,
    KeyAccelerators,
    KeyEventInfo,
    Keys,
    NavigationViewDisplayMode,
    OptionalBool,
    OptionalDateTime,
    OptionalF64,
    OptionalTimeSpan,
    PointerEventInfo,
    ResourceOverrides,
    RichText,
    Selection,
    SelectionIndex,
    String,
    StringList,
    ThemeTransitions,
    Thickness,
    Unit,
    Vector2
);

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.name())
    }
}

impl Value {
    fn enum_name(&self) -> Option<&str> {
        match self {
            Self::Enum(name) => Some(name),
            _ => None,
        }
    }

    fn argument(&self, name: &str) -> String {
        let name = snake_case(name);
        match self {
            Self::String => format!("{name}: impl AsRef<str>"),
            Self::Brush => format!("{name}: impl Into<Brush>"),
            Self::CornerRadius => format!("{name}: impl Into<CornerRadius>"),
            Self::F64 => format!("{name}: f64"),
            Self::GridLengths => format!("{name}: impl IntoIterator<Item = GridLength>"),
            Self::I32 => format!("{name}: i32"),
            Self::Icon => format!("{name}: impl Into<Icon>"),
            Self::OptionalF64 => format!("{name}: impl Into<Option<f64>>"),
            Self::OptionalBool => format!("{name}: impl Into<Option<bool>>"),
            Self::OptionalDateTime => format!("{name}: impl Into<Option<DateTime>>"),
            Self::SelectionIndex => format!("{name}: impl Into<Option<usize>>"),
            Self::StringList => format!("{name}: impl IntoIterator<Item = impl Into<Rc<str>>>"),
            Self::Thickness => format!("{name}: impl Into<Thickness>"),
            Self::Vector2 => "x: f64, y: f64".to_string(),
            Self::Bool => format!("{name}: bool"),
            value => format!("{name}: {value}"),
        }
    }

    fn expression(&self, name: &str) -> String {
        match self {
            Self::String => format!("PropertyValue::String(Rc::from({name}.as_ref()))"),
            Self::Brush | Self::Icon => format!("PropertyValue::{self}({name}.into())"),
            Self::DragDropPolicy => format!("PropertyValue::DragDropPolicy(Rc::new({name}))"),
            Self::GridLengths => {
                format!("PropertyValue::GridLengths({name}.into_iter().collect())")
            }
            Self::OptionalF64
            | Self::OptionalBool
            | Self::OptionalDateTime
            | Self::SelectionIndex => format!("PropertyValue::{self}({name}.into())"),
            Self::StringList => {
                format!("PropertyValue::StringList({name}.into_iter().map(Into::into).collect())")
            }
            Self::Enum(_) => format!("{name}.property_value()"),
            _ => format!("PropertyValue::{self}({name})"),
        }
    }

    fn example(&self) -> Option<String> {
        Some(
            match self {
                Self::String => "\"Text\"",
                Self::Bool => "false",
                Self::Brush | Self::Color => "Color::rgb(0, 0, 0)",
                Self::ButtonStyle => "ButtonStyle::Default",
                Self::CornerRadius => "CornerRadius::uniform(0.0)",
                Self::Duration => "std::time::Duration::ZERO",
                Self::F64 => "0.0",
                Self::I32 => "0",
                Self::Icon => "Symbol::Previous",
                Self::KeyAccelerators => "KeyAccelerators::default()",
                Self::OptionalF64 | Self::OptionalBool | Self::SelectionIndex => "None",
                Self::ResourceOverrides => "ResourceOverrides::default()",
                Self::StringList => "std::iter::empty::<&str>()",
                Self::Thickness => "Thickness::uniform(0.0)",
                _ => return None,
            }
            .to_string(),
        )
    }

    fn callback_type(&self) -> &'static str {
        match self {
            Self::String => "Rc<str>",
            Self::Key => "Key",
            Self::Keys => "Vec<Key>",
            Self::Bool => "bool",
            Self::Color => "Color",
            Self::ContentDialogResult => "ContentDialogResult",
            Self::F64 => "f64",
            Self::FocusEventInfo => "FocusEventInfo",
            Self::DragKind => "DragKind",
            Self::DroppedData => "DroppedData",
            Self::OptionalBool => "Option<bool>",
            Self::OptionalDateTime => "Option<DateTime>",
            Self::OptionalF64 => "Option<f64>",
            Self::OptionalTimeSpan => "Option<TimeSpan>",
            Self::NavigationViewDisplayMode => "NavigationViewDisplayMode",
            Self::Selection => "Option<Rc<str>>",
            Self::PointerEventInfo => "PointerEventInfo",
            Self::SelectionIndex => "Option<usize>",
            value => unreachable!("{value} has no payload callback"),
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
    value: Value,
    adapter: Option<PropertyAdapter>,
    controlled: Option<String>,
    coerces: Option<String>,
    indexes: Option<String>,
    #[serde(default)]
    requires_selection: bool,
    #[serde(default)]
    realize_items: bool,
    feedback: Option<Feedback>,
    #[serde(default)]
    clear_feedback: bool,
    #[serde(default)]
    required: bool,
    validation: Option<Validation>,
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
    value: Value,
    #[serde(default)]
    property_changed: bool,
    observes: Option<String>,
    /// A `Many` relation the native control reorders; the event reports the new key order.
    reorders: Option<String>,
    payload: Option<String>,
    payload_adapter: Option<PayloadAdapter>,
    #[serde(default)]
    active_properties: Vec<String>,
    #[serde(default)]
    routed: bool,
    /// Records the observed property without exposing a callback, for native changes that are
    /// never direct user input, such as a `RadioButton` unchecked by its group.
    #[serde(default)]
    observe_only: bool,
    /// The event-args `Reason` value that reports a change made through Reactor's own write.
    /// Such a change records the observed property without calling back the app, which matters
    /// when the control drops a later write and reports the earlier one after it.
    programmatic_reason: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    relations: Vec<String>,
    item: String,
    selected_property: String,
    selected_item_property: String,
    event: String,
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
        .filters(dependency_filters(&dependency_properties(
            &schema, &metadata,
        )))
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
                Value::from,
            );
        if property.value == Value::Unresolved {
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
            property.adapter,
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
            .map(|property| (property.name.as_str(), &property.value))
            .collect::<BTreeMap<_, _>>();
        for event in &mut object.events {
            let inferred = event
                .observes
                .as_deref()
                .map(|observed| {
                    property_values
                        .get(observed)
                        .copied()
                        .unwrap_or_else(|| {
                            panic!(
                                "{}.{} observes missing property {observed}",
                                object.name, event.name
                            )
                        })
                        .clone()
                })
                .or_else(|| {
                    object
                        .selection
                        .as_ref()
                        .filter(|selection| selection.event == event.name)
                        .map(|_| Value::Selection)
                })
                .or_else(|| event.reorders.as_ref().map(|_| Value::Keys))
                .or_else(|| event.adapter.map(EventAdapter::value))
                .or_else(|| event.payload_adapter.map(PayloadAdapter::value))
                .or_else(|| {
                    event.payload.as_deref().and_then(|payload| {
                        metadata
                            .resolve_event_args_property(
                                &owner,
                                &format!("add_{}", event.name),
                                payload,
                            )
                            .map(|(value, _, _)| Value::from(value))
                    })
                });
            if event.value == Value::Unresolved {
                event.value = inferred.unwrap_or(Value::Unit);
            } else if let Some(inferred) = inferred {
                assert!(
                    event.value == Value::Unit
                        || event.value == inferred
                        || matches!(
                            (&event.value, &inferred),
                            (Value::FontWeight, Value::Enum(name)) if name == "U16"
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
                property.value != Value::Unresolved,
                "{}.{} has no resolved value",
                object.name,
                property.name
            );
            if let Some(event) = &property.controlled {
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
            if let Some(coerced) = &property.coerces {
                let position = |name: &str| {
                    object
                        .properties
                        .iter()
                        .position(|candidate| candidate.name == name)
                };
                assert!(
                    object
                        .properties
                        .iter()
                        .any(|candidate| candidate.name == *coerced
                            && candidate.controlled.is_some()),
                    "{}.{} must coerce a controlled property",
                    object.name,
                    property.name
                );
                assert!(
                    position(&property.name) < position(coerced),
                    "{}.{} must precede the {coerced} property it coerces",
                    object.name,
                    property.name
                );
            }
            assert_eq!(
                property.value == Value::SelectionIndex,
                property.indexes.is_some(),
                "{}.{} must name the collection it indexes",
                object.name,
                property.name
            );
            assert!(
                property.indexes.is_some()
                    || !(property.requires_selection || property.realize_items),
                "{}.{} declares index behavior but indexes nothing",
                object.name,
                property.name
            );
            if let Some(items) = &property.indexes {
                assert!(
                    property.controlled.is_some()
                        && matches!(property.feedback, Some(Feedback::DeferredExact)),
                    "{}.{} must be controlled with deferred_exact feedback",
                    object.name,
                    property.name
                );
                assert!(
                    object.relations.iter().any(|candidate| {
                        candidate.name == *items && candidate.cardinality == Cardinality::Many
                    }) || object.properties.iter().any(|candidate| {
                        candidate.name == *items && candidate.value == Value::StringList
                    }),
                    "{}.{} must index a collection",
                    object.name,
                    property.name
                );
                assert_eq!(
                    object
                        .properties
                        .iter()
                        .filter(|property| property.indexes.is_some())
                        .count(),
                    1,
                    "{} has more than one index property",
                    object.name
                );
            }
        }
        for event in &object.events {
            if event.property_changed || event.observe_only || event.programmatic_reason.is_some() {
                assert!(
                    event.observes.is_some(),
                    "{}.{} must observe a property",
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
            if let Some(relation) = &event.reorders {
                assert!(
                    object
                        .relations
                        .iter()
                        .any(|candidate| candidate.name == *relation
                            && candidate.cardinality == Cardinality::Many),
                    "{}.{} reorders missing collection {relation}",
                    object.name,
                    event.name
                );
            }
            if event.payload_adapter == Some(PayloadAdapter::ItemKey) {
                assert!(
                    object.events.iter().any(|event| event.reorders.is_some()),
                    "{}.{} item keys require a reordered relation",
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
            if matches!(
                event.adapter,
                Some(EventAdapter::DragKind | EventAdapter::DroppedData)
            ) {
                assert!(
                    event
                        .active_properties
                        .iter()
                        .any(|property| property == "drop_policy"),
                    "{}.{} must list drop_policy in active_properties",
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
    value: &mut Value,
    metadata: &tool_reactor_metadata::MetadataResolver,
    owner: &str,
    method: &str,
    adapter: Option<PropertyAdapter>,
    schema_name: &str,
) {
    let inferred = adapter.map(PropertyAdapter::value).or_else(|| {
        metadata
            .infer_value_type(owner, method)
            .map(|(value, _)| Value::from(value))
    });
    if *value == Value::Unresolved {
        *value = inferred.unwrap_or_else(|| {
            panic!("cannot infer value for {schema_name} from {owner}.{method}")
        });
    } else if let Some(inferred) = inferred {
        assert!(
            *value == inferred
                || matches!(
                    (&*value, &inferred),
                    (Value::FontWeight, Value::Enum(name)) if name == "U16"
                )
                || (*value == Value::OptionalBool && inferred == Value::Bool),
            "{schema_name} value {value} does not match inferred value {inferred}"
        );
    }
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join(path)
}

fn native_name(object: &Object) -> String {
    object.native_path().rsplit('.').next().unwrap().to_string()
}

fn has_local_property(object: &Object, name: &str) -> bool {
    object
        .properties
        .iter()
        .any(|property| property.name == name)
}

fn generate(schema: &Schema, metadata: &tool_reactor_metadata::MetadataResolver) -> String {
    let mut properties = BTreeMap::new();
    let mut relations = BTreeMap::new();
    let mut events = BTreeMap::new();
    for property in &schema.attached_properties {
        properties.insert(property.name.as_str(), &property.value);
    }
    for property in &schema.visual_properties {
        properties.insert(property.name.as_str(), &property.value);
    }
    if schema.objects.iter().any(|object| object.enabled) {
        properties.insert("IsEnabled", &Value::Bool);
    }
    events.insert("MenuItemInvoked", &Value::String);
    events.insert("CommandInvoked", &Value::String);
    for object in &schema.objects {
        for property in &object.properties {
            properties
                .entry(property.name.as_str())
                .or_insert(&property.value);
        }
        for relation in &object.relations {
            relations.insert(relation.name.as_str(), ());
        }
        for event in &object.events {
            events.entry(event.name.as_str()).or_insert(&event.value);
        }
    }

    let mut output = String::from("// This file is generated by tool-reactor.\nuse super::*;\n");
    emit_enum(
        &mut output,
        "ObjectType",
        schema.objects.iter().map(|object| object.name.as_str()),
    );
    emit_enum(&mut output, "PropertyId", properties.keys().copied());
    emit_enum(&mut output, "RelationId", relations.keys().copied());
    emit_enum(&mut output, "EventId", events.keys().copied());
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
                 event: EventId::{}, payload_property: PropertyId::{} }}),\n",
                object.name,
                selection
                    .relations
                    .iter()
                    .map(|relation| format!("RelationId::{relation},"))
                    .collect::<String>(),
                selection.item,
                selection.selected_property,
                selection.event,
                selection.payload_property
            ));
        }
    }
    output.push_str("_ => None,\n} }\n");
    output.push_str(
        "pub fn index_contract(kind: ObjectType) -> Option<IndexContract> { match kind {\n",
    );
    for object in &schema.objects {
        for property in &object.properties {
            if let Some(items) = &property.indexes {
                let items = if object
                    .relations
                    .iter()
                    .any(|relation| relation.name == *items)
                {
                    format!("IndexItems::Relation(RelationId::{items})")
                } else {
                    format!("IndexItems::Property(PropertyId::{items})")
                };
                output.push_str(&format!(
                    "ObjectType::{} => Some(IndexContract {{ property: PropertyId::{}, \
                     items: {items}, requires_selection: {}, realize_items: {}, \
                     event: EventId::{} }}),\n",
                    object.name,
                    property.name,
                    property.requires_selection,
                    property.realize_items,
                    property.controlled.as_ref().unwrap()
                ));
            }
        }
    }
    output.push_str("_ => None,\n} }\n");
    output.push_str(
        "pub fn coerced_property(kind: ObjectType, id: PropertyId) -> Option<PropertyId> { \
         match (kind, id) {\n",
    );
    for object in &schema.objects {
        for property in &object.properties {
            if let Some(coerced) = &property.coerces {
                output.push_str(&format!(
                    "(ObjectType::{}, PropertyId::{}) => Some(PropertyId::{coerced}),\n",
                    object.name, property.name
                ));
            }
        }
    }
    output.push_str("_ => None,\n} }\n");
    output.push_str(
        "pub fn reorder_contracts(kind: ObjectType) -> &'static [ReorderContract] { match kind {\n",
    );
    for object in &schema.objects {
        let contracts = object
            .events
            .iter()
            .filter_map(|event| {
                Some(format!(
                    "ReorderContract {{ relation: RelationId::{}, event: EventId::{} }},",
                    event.reorders.as_ref()?,
                    event.name
                ))
            })
            .collect::<String>();
        if !contracts.is_empty() {
            output.push_str(&format!("ObjectType::{} => &[{contracts}],\n", object.name));
        }
    }
    output.push_str("_ => &[],\n} }\n");
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
        let value = if property.value.enum_name().is_none() {
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
        let value = if property.value.enum_name().is_none() {
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
            let value = if property.value.enum_name().is_none() {
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
    output.push_str(
        "#[cfg(any(test, feature = \"test\"))]\n\
         pub(crate) const ALL_OBJECT_TYPES: &[ObjectType] = &[",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{},", object.name));
    }
    output.push_str("];\n");
    output.push_str(
        "#[cfg(any(test, feature = \"test\"))]\n\
         pub(crate) const ALL_PROPERTY_IDS: &[PropertyId] = &[",
    );
    for property in properties.keys() {
        output.push_str(&format!("PropertyId::{property},"));
    }
    output.push_str("];\n");
    output.push_str(
        "#[cfg(any(test, feature = \"test\"))]\n\
         pub(crate) fn object_index(kind: ObjectType) -> usize { match kind {\n",
    );
    for (index, object) in schema.objects.iter().enumerate() {
        output.push_str(&format!("ObjectType::{} => {index},", object.name));
    }
    output.push_str("} }\n");
    output.push_str(
        "#[cfg(any(test, feature = \"test\"))]\n\
         pub(crate) fn focus_capable(kind: ObjectType) -> bool { matches!(kind,",
    );
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
        let Some(kind) = property.value.enum_name() else {
            continue;
        };
        let owner = property.owner.rsplit('.').next().unwrap();
        let (_, variants) = metadata
            .static_enum_info(owner, &format!("Set{}", property.native))
            .unwrap();
        if let Some(previous) = enums.insert(kind, variants.to_vec()) {
            assert_eq!(previous, variants, "conflicting enum definitions");
        }
    }
    for property in &schema.visual_properties {
        let Some(kind) = property.value.enum_name() else {
            continue;
        };
        let owner = property.owner.rsplit('.').next().unwrap();
        let (_, variants) = metadata
            .enum_info(owner, &format!("put_{}", property.name))
            .unwrap();
        if let Some(previous) = enums.insert(kind, variants.to_vec()) {
            assert_eq!(previous, variants, "conflicting enum definitions");
        }
    }
    for object in &schema.objects {
        for property in &object.properties {
            let Some(kind) = property.value.enum_name() else {
                continue;
            };
            let (_, variants) = metadata
                .enum_info(
                    &native_name(object),
                    &format!("put_{}", native_property(property)),
                )
                .unwrap();
            if let Some(previous) = enums.insert(kind, variants.to_vec()) {
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
    output.push_str("macro_rules! attached_methods { () => {\n");
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
            property.value.argument(&property.name)
        ));
        if matches!(property.value, Value::CornerRadius | Value::Thickness) {
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
            property.value.expression(&name)
        ));
    }
    for property in &schema.visual_properties {
        let name = snake_case(&property.name);
        if property.value == Value::ThemeTransitions {
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
                property.value.argument(&property.name)
            ));
            if matches!(property.value, Value::CornerRadius | Value::Thickness) {
                output.push_str(&format!("let {name} = {name}.into();\n"));
            }
            output.push_str(&format!(
                "self.0 = self.0.property(PropertyId::{}, {});\nself\n}}\n",
                property.name,
                property.value.expression(&name)
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
            output.push_str(&format!("impl Sealed for {} {{}}\n", object.name));
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
                    "image_uri_methods!({name}, {name}_optional, {name}_file, {name}_data, \
                     {name}_svg, {});\n",
                    property.name
                ));
                continue;
            }
            if property.adapter == Some(PropertyAdapter::Uri) {
                output.push_str(&format!(
                    "uri_methods!({name}, {name}_optional, {});\n",
                    property.name
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
                _ => property.value.argument(&name),
            };
            output.push_str(&format!("pub fn {name}(mut self, {argument}) -> Self {{\n"));
            if property.value == Value::Vector2 {
                output.push_str(&format!("let {name} = [x, y];\n"));
            }
            if matches!(property.value, Value::CornerRadius | Value::Thickness) {
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
        if object.focus || object.reference {
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

        for event in object.events.iter().filter(|event| !event.observe_only) {
            let method = event
                .field
                .as_deref()
                .and_then(|field| field.strip_prefix("on_"))
                .map_or_else(|| snake_case(&event.name), str::to_string);
            match &event.value {
                Value::CharacterEventInfo => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: \
                         RoutedCallback<CharacterEventInfo>) -> Self {{ self.0 = \
                         self.0.event(EventId::{}, EventValue::CharacterEventInfo(callback)); self \
                         }}\n",
                        event.name
                    ));
                }
                Value::KeyEventInfo => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: RoutedCallback<KeyEventInfo>) -> \
                         Self {{ self.0 = self.0.event(EventId::{}, \
                         EventValue::KeyEventInfo(callback)); self }}\n",
                        event.name
                    ));
                }
                Value::Unit => {
                    output.push_str(&format!(
                        "pub fn on_{method}(mut self, callback: impl IntoUnitCallback) -> Self {{ \
                         self.0 = self.0.event(EventId::{}, \
                         EventValue::Unit(callback.into_unit_callback())); self }}\n",
                        event.name
                    ));
                }
                value => {
                    let callback_type = value.callback_type();
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
    output
}

fn property_argument(property: &Property) -> String {
    property.value.argument(&property.name)
}

fn validation_expression(name: &str, value: &Value, validation: Validation) -> String {
    match validation {
        Validation::Finite if *value == Value::Vector2 => {
            format!("{name}.iter().all(|value| (*value as f32).is_finite())")
        }
        Validation::Finite => format!("{name}.is_finite()"),
        Validation::FinitePositive => format!("{name}.is_finite() && {name} > 0.0"),
        Validation::FiniteNonNegative if *value == Value::F64 => {
            format!("{name}.is_finite() && {name} >= 0.0")
        }
        Validation::FiniteNonNegative => format!("{name}.is_finite_non_negative()"),
        Validation::NonNegative => format!("{name} >= 0"),
        Validation::Positive => format!("{name} > 0"),
        Validation::ZeroToFiftyNine => format!("(0..=59).contains(&{name})"),
    }
}

fn property_value(property: &Property, name: &str) -> String {
    property.value.expression(name)
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
        arguments.push(property.value.example().unwrap_or_else(|| {
            panic!(
                "{}.{} has no example for {}",
                object.name, property.name, property.value
            )
        }));
    }
    format!("{}::new({})", object.name, arguments.join(", "))
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
