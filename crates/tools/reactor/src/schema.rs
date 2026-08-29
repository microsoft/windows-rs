use crate::helpers::to_snake_case;
use crate::metadata::{MetadataResolver, ParamClass, ReadValueConversion};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(crate) fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(path)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Schema {
    pub(crate) control: Vec<Control>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Control {
    #[serde(rename = "type")]
    pub(crate) type_name: String,
    #[serde(default)]
    pub(crate) placement: Option<Placement>,
    #[serde(default)]
    pub(crate) lifecycle: Option<Lifecycle>,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) capabilities: Vec<Capability>,
    #[serde(default)]
    pub(crate) property: Vec<Property>,
    #[serde(default)]
    pub(crate) event: Vec<Event>,
    #[serde(default)]
    pub(crate) slot: Vec<Slot>,
    pub(crate) selection: Option<Selection>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Placement {
    TooltipAttachment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResolvedPlacement {
    Visual,
    WindowLifetime,
    TooltipAttachment,
    Declaration,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Lifecycle {
    ContentDialog,
}

#[derive(Clone, Copy)]
pub(crate) enum Role {
    Leaf,
    Content,
    Children,
    Slots,
    Virtual,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Capability {
    Layout,
    TextStyle,
    Enabled,
    Content,
    Children,
    ControlledText,
    Items,
    Focus,
    Reference,
    GridDefinitions,
    WindowTitleBar,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Property {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) field: Option<String>,
    #[serde(default)]
    pub(crate) theme_style: bool,
    #[serde(default)]
    pub(crate) controlled: Option<String>,
    #[serde(default)]
    pub(crate) coerces: Option<String>,
    #[serde(default)]
    pub(crate) feedback_contract: Option<FeedbackContract>,
    #[serde(default)]
    pub(crate) clear_feedback: Option<bool>,
    #[serde(default)]
    pub(crate) adapter: Option<PropertyAdapter>,
    #[serde(default)]
    pub(crate) validation: Option<ValueValidation>,
    #[serde(default)]
    pub(crate) variants: Vec<ResourceStyleVariant>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PropertyAdapter {
    ClockIdentifier,
    ContentDialogResult,
    ImageUri,
    InspectableString,
    InspectableStringList,
    ImplicitOpacityTransition,
    ImplicitScale,
    ImplicitScaleTransition,
    KeyAccelerators,
    ItemTag,
    ItemTags,
    NavigationDisplayMode,
    NumberBoxValue,
    PathData,
    PointerCapture,
    PointerEvent,
    DragInfo,
    DropData,
    DropPolicy,
    FontWeight,
    HorizontalContentAlignment,
    ResourceOverrides,
    ResourceStyle,
    RatingValue,
    RichEditText,
    RichTextBlocks,
    TreeNodeContent,
    Uri,
    VerticalContentAlignment,
    SelectionIndex,
}

#[derive(Clone, Copy)]
pub(crate) struct PropertyAdapterCapabilities {
    pub(crate) uses_dependency_property: bool,
    pub(crate) uses_property_setter: bool,
}

#[derive(Clone, Copy)]
enum PropertyAdapterTargets {
    Any,
    Property(&'static str),
    OneOf(&'static [(&'static str, &'static str)]),
}

#[derive(Clone, Copy)]
enum PropertyAdapterMetadata {
    None,
    Param(ParamClass),
    ParamType(&'static str),
    ValueType(&'static str, bool),
    SingleField(&'static str, &'static str),
}

#[derive(Clone, Copy)]
struct PropertyAdapterContract {
    targets: PropertyAdapterTargets,
    metadata: PropertyAdapterMetadata,
    value: &'static str,
    copy: bool,
    requirement: &'static str,
}

enum PropertyAdapterKind {
    Property(PropertyAdapterContract),
    ResourceStyle,
    EventOnly(&'static str),
}

impl PropertyAdapter {
    pub(crate) fn capabilities(self) -> PropertyAdapterCapabilities {
        match self {
            Self::ResourceStyle => PropertyAdapterCapabilities {
                uses_dependency_property: true,
                uses_property_setter: false,
            },
            Self::DropPolicy
            | Self::KeyAccelerators
            | Self::PointerCapture
            | Self::ResourceOverrides
            | Self::RichEditText
            | Self::RichTextBlocks => PropertyAdapterCapabilities {
                uses_dependency_property: false,
                uses_property_setter: false,
            },
            Self::ImplicitOpacityTransition
            | Self::ImplicitScale
            | Self::ImplicitScaleTransition => PropertyAdapterCapabilities {
                uses_dependency_property: false,
                uses_property_setter: true,
            },
            Self::ClockIdentifier
            | Self::ContentDialogResult
            | Self::DragInfo
            | Self::DropData
            | Self::FontWeight
            | Self::HorizontalContentAlignment
            | Self::ImageUri
            | Self::InspectableString
            | Self::InspectableStringList
            | Self::ItemTag
            | Self::ItemTags
            | Self::NavigationDisplayMode
            | Self::NumberBoxValue
            | Self::PathData
            | Self::PointerEvent
            | Self::RatingValue
            | Self::SelectionIndex
            | Self::TreeNodeContent
            | Self::Uri
            | Self::VerticalContentAlignment => PropertyAdapterCapabilities {
                uses_dependency_property: true,
                uses_property_setter: true,
            },
        }
    }

    fn property_kind(self) -> PropertyAdapterKind {
        use PropertyAdapterMetadata::{None, Param, ParamType, SingleField, ValueType};
        use PropertyAdapterTargets::{Any, OneOf, Property};

        const BORDER: &str = "Microsoft.UI.Xaml.Controls.Border";
        const BUTTON: &str = "Microsoft.UI.Xaml.Controls.Button";
        const GRID: &str = "Microsoft.UI.Xaml.Controls.Grid";
        const IMAGE: &str = "Microsoft.UI.Xaml.Controls.Image";
        const IMAGE_ICON: &str = "Microsoft.UI.Xaml.Controls.ImageIcon";
        const NUMBER_BOX: &str = "Microsoft.UI.Xaml.Controls.NumberBox";
        const PATH_ICON: &str = "Microsoft.UI.Xaml.Controls.PathIcon";
        const RATING_CONTROL: &str = "Microsoft.UI.Xaml.Controls.RatingControl";
        const RICH_EDIT_BOX: &str = "Microsoft.UI.Xaml.Controls.RichEditBox";
        const RICH_TEXT_BLOCK: &str = "Microsoft.UI.Xaml.Controls.RichTextBlock";
        const TEXT_BLOCK: &str = "Microsoft.UI.Xaml.Controls.TextBlock";
        const TIME_PICKER: &str = "Microsoft.UI.Xaml.Controls.TimePicker";

        let property = |targets, metadata, value, copy, requirement| {
            PropertyAdapterKind::Property(PropertyAdapterContract {
                targets,
                metadata,
                value,
                copy,
                requirement,
            })
        };
        match self {
            Self::ImageUri => property(
                OneOf(&[(IMAGE, "Source"), (IMAGE_ICON, "Source")]),
                None,
                "ImageValue",
                false,
                "image_uri requires Image.Source or ImageIcon.Source",
            ),
            Self::NumberBoxValue => property(
                OneOf(&[(NUMBER_BOX, "Value")]),
                ValueType("F64", true),
                "OptionalF64",
                true,
                "number_box_value requires NumberBox.Value",
            ),
            Self::InspectableString => property(
                Any,
                Param(ParamClass::IInspectable),
                "Str",
                false,
                "inspectable_string requires an IInspectable property",
            ),
            Self::InspectableStringList => property(
                Any,
                Param(ParamClass::IInspectable),
                "StrList",
                false,
                "inspectable_string_list requires an IInspectable property",
            ),
            Self::ImplicitOpacityTransition => property(
                OneOf(&[(BORDER, "OpacityTransition")]),
                None,
                "Duration",
                false,
                "implicit_opacity_transition requires Border.OpacityTransition",
            ),
            Self::ImplicitScale => property(
                OneOf(&[(BORDER, "Scale")]),
                None,
                "F64",
                true,
                "implicit_scale requires Border.Scale",
            ),
            Self::ImplicitScaleTransition => property(
                OneOf(&[(BORDER, "ScaleTransition")]),
                None,
                "Duration",
                false,
                "implicit_scale_transition requires Border.ScaleTransition",
            ),
            Self::KeyAccelerators => property(
                OneOf(&[
                    (BUTTON, "KeyboardAccelerators"),
                    (GRID, "KeyboardAccelerators"),
                ]),
                None,
                "KeyAccelerators",
                false,
                "key_accelerators requires Button.KeyboardAccelerators or Grid.KeyboardAccelerators",
            ),
            Self::ClockIdentifier => property(
                OneOf(&[(TIME_PICKER, "ClockIdentifier")]),
                None,
                "Str",
                false,
                "clock_identifier requires TimePicker.ClockIdentifier",
            ),
            Self::PathData => property(
                OneOf(&[(PATH_ICON, "Data")]),
                ParamType("Microsoft.UI.Xaml.Media.Geometry"),
                "Str",
                false,
                "path_data requires PathIcon.Data",
            ),
            Self::DropPolicy => property(
                OneOf(&[(BORDER, "AllowDrop")]),
                None,
                "DragDropPolicy",
                false,
                "drop_policy requires Border.AllowDrop",
            ),
            Self::FontWeight => property(
                OneOf(&[(TEXT_BLOCK, "FontWeight")]),
                SingleField("Windows.UI.Text.FontWeight", "weight"),
                "FontWeight",
                true,
                "font_weight requires TextBlock.FontWeight",
            ),
            Self::HorizontalContentAlignment => property(
                OneOf(&[(BUTTON, "HorizontalContentAlignment")]),
                ValueType("HorizontalAlignment", true),
                "HorizontalAlignment",
                true,
                "horizontal_content_alignment requires Button.HorizontalContentAlignment",
            ),
            Self::PointerCapture => property(
                OneOf(&[(BORDER, "CapturePointerOnPress")]),
                None,
                "Bool",
                true,
                "pointer_capture requires Border.CapturePointerOnPress",
            ),
            Self::RatingValue => property(
                OneOf(&[(RATING_CONTROL, "Value")]),
                ValueType("F64", true),
                "OptionalF64",
                true,
                "rating_value requires RatingControl.Value",
            ),
            Self::RichEditText => property(
                OneOf(&[(RICH_EDIT_BOX, "Document")]),
                None,
                "Str",
                false,
                "rich_edit_text requires RichEditBox.Document",
            ),
            Self::RichTextBlocks => property(
                OneOf(&[(RICH_TEXT_BLOCK, "Blocks")]),
                None,
                "RichText",
                false,
                "rich_text_blocks requires RichTextBlock.Blocks",
            ),
            Self::ResourceOverrides => property(
                OneOf(&[(BUTTON, "Resources")]),
                None,
                "ResourceOverrides",
                false,
                "resource_overrides requires Button.Resources",
            ),
            Self::Uri => property(
                Any,
                ParamType("Windows.Foundation.Uri"),
                "Str",
                false,
                "uri requires a Windows.Foundation.Uri property",
            ),
            Self::VerticalContentAlignment => property(
                OneOf(&[(BUTTON, "VerticalContentAlignment")]),
                ValueType("VerticalAlignment", true),
                "VerticalAlignment",
                true,
                "vertical_content_alignment requires Button.VerticalContentAlignment",
            ),
            Self::SelectionIndex => property(
                Property("SelectedIndex"),
                ValueType("I32", true),
                "SelectionIndex",
                true,
                "selection_index requires an I32 SelectedIndex property",
            ),
            Self::ResourceStyle => PropertyAdapterKind::ResourceStyle,
            Self::ContentDialogResult => {
                PropertyAdapterKind::EventOnly("content_dialog_result is an event-only adapter")
            }
            Self::ItemTag => PropertyAdapterKind::EventOnly("item_tag is an event-only adapter"),
            Self::ItemTags => PropertyAdapterKind::EventOnly("item_tags is an event-only adapter"),
            Self::TreeNodeContent => {
                PropertyAdapterKind::EventOnly("tree_node_content is an event-only adapter")
            }
            Self::NavigationDisplayMode => {
                PropertyAdapterKind::EventOnly("navigation_display_mode is an event-only adapter")
            }
            Self::PointerEvent => {
                PropertyAdapterKind::EventOnly("pointer_event is an event-only adapter")
            }
            Self::DragInfo | Self::DropData => {
                PropertyAdapterKind::EventOnly("drag adapters are event-only")
            }
        }
    }
}

impl PropertyAdapterContract {
    fn validate(
        self,
        control: &str,
        property: &str,
        metadata: &MetadataResolver,
        class: &str,
        method: &str,
    ) -> bool {
        let target_matches = match self.targets {
            PropertyAdapterTargets::Any => true,
            PropertyAdapterTargets::Property(expected) => property == expected,
            PropertyAdapterTargets::OneOf(targets) => targets.contains(&(control, property)),
        };
        target_matches
            && match self.metadata {
                PropertyAdapterMetadata::None => true,
                PropertyAdapterMetadata::Param(expected) => {
                    metadata.classify_param(class, method) == Some(expected)
                }
                PropertyAdapterMetadata::ParamType(expected) => {
                    metadata.param_class_name(class, method).as_deref() == Some(expected)
                }
                PropertyAdapterMetadata::ValueType(value, copy) => {
                    metadata.infer_value_type(class, method) == Some((value.to_string(), copy))
                }
                PropertyAdapterMetadata::SingleField(path, field) => {
                    metadata.single_field_param(class, method)
                        == Some((path.to_string(), field.to_string()))
                }
            }
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ResourceStyleVariant {
    pub(crate) name: String,
    pub(crate) resource: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ValueValidation {
    Finite,
    FiniteNonNegative,
    FinitePositive,
    NonNegative,
    ZeroToFiftyNine,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FeedbackContract {
    SynchronousExact,
    SynchronousNormalized,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Event {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) field: Option<String>,
    #[serde(default)]
    pub(crate) property: Option<String>,
    #[serde(default)]
    pub(crate) observe: Option<String>,
    #[serde(default)]
    pub(crate) adapter: Option<PropertyAdapter>,
    #[serde(default)]
    pub(crate) active_property: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Slot {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) collection: bool,
    #[serde(default)]
    pub(crate) item_controls: Vec<String>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SlotTarget {
    IconElement,
    Inspectable,
    UiElement,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Selection {
    pub(crate) slot: String,
    pub(crate) item: String,
    pub(crate) selected_property: String,
    pub(crate) selected_item_property: String,
    pub(crate) event: String,
    pub(crate) event_args: Option<String>,
    pub(crate) payload_property: String,
}

pub(crate) struct ResolvedSchema {
    pub(crate) controls: Vec<ResolvedControl>,
}

impl ResolvedControl {
    pub(crate) fn event_always_active(&self, event: &ResolvedEvent) -> bool {
        self.properties
            .iter()
            .any(|property| property.feedback.as_deref() == Some(event.name.as_str()))
            || event.conversion == EventPayloadConversion::Selection
            || (self.lifecycle == Some(Lifecycle::ContentDialog) && event.name == "Closed")
    }
}

pub(crate) struct ResolvedControl {
    pub(crate) name: String,
    pub(crate) type_name: String,
    pub(crate) role: Role,
    pub(crate) placement: ResolvedPlacement,
    pub(crate) lifecycle: Option<Lifecycle>,
    pub(crate) content: Option<ResolvedContent>,
    pub(crate) capabilities: Vec<Capability>,
    pub(crate) properties: Vec<ResolvedProperty>,
    pub(crate) events: Vec<ResolvedEvent>,
    pub(crate) slots: Vec<ResolvedSlot>,
    pub(crate) selection: Option<ResolvedSelection>,
}

pub(crate) struct ResolvedProperty {
    pub(crate) name: String,
    pub(crate) field: String,
    pub(crate) value: String,
    pub(crate) copy: bool,
    pub(crate) interface: String,
    pub(crate) static_owner: String,
    pub(crate) feedback: Option<String>,
    pub(crate) feedback_contract: Option<FeedbackContract>,
    pub(crate) clear_feedback: Option<bool>,
    pub(crate) adapter: Option<PropertyAdapter>,
    pub(crate) nullable_bool: bool,
    pub(crate) observes_feedback: bool,
    pub(crate) enum_variants: Vec<String>,
    pub(crate) resource_style_variants: Vec<ResourceStyleVariant>,
    pub(crate) native_value: Option<String>,
    pub(crate) native_wrapper: Option<(String, String)>,
    pub(crate) validation: Option<ValueValidation>,
    pub(crate) theme_style: bool,
}

pub(crate) struct ResolvedContent {
    pub(crate) name: String,
    pub(crate) interface: String,
    pub(crate) target: SlotTarget,
}

pub(crate) struct ResolvedEvent {
    pub(crate) name: String,
    pub(crate) field: String,
    pub(crate) payload: String,
    pub(crate) interface: String,
    pub(crate) source: EventPayloadSource,
    pub(crate) conversion: EventPayloadConversion,
    pub(crate) subscription: EventSubscription,
    pub(crate) active_property: Option<String>,
}

pub(crate) struct ResolvedSlot {
    pub(crate) name: String,
    pub(crate) interface: String,
    pub(crate) shape: SlotShape,
    pub(crate) collection_cast: bool,
    pub(crate) collection_observable: bool,
    pub(crate) collection_item: Option<String>,
    pub(crate) item_controls: Vec<String>,
}

pub(crate) struct ResolvedSelection {
    pub(crate) slot: String,
    pub(crate) item: String,
    pub(crate) selected_property: String,
    pub(crate) selected_item_property: String,
    pub(crate) selected_item: Option<String>,
    pub(crate) event: String,
    pub(crate) payload_property: String,
    pub(crate) owner_interface: String,
    pub(crate) selected_interface: String,
    pub(crate) payload_interface: String,
    pub(crate) payload_inspectable: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum SlotShape {
    Single(SlotTarget),
    Collection,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum EventPayloadSource {
    Unit,
    SenderProperty { interface: String, property: String },
    EventArgsProperty { interface: String, property: String },
    DragInfo { interface: String },
    DropData { interface: String },
    EventArgsInspectableString { interface: String, property: String },
    EventArgsItemTag { interface: String, property: String },
    EventArgsTreeNodeContent { interface: String, property: String },
    SenderRichEditText { interface: String, property: String },
    PointerEvent,
    SenderItemTags { interface: String, property: String },
}

impl EventPayloadSource {
    pub(crate) fn getter(&self) -> Option<(&str, &str)> {
        match self {
            Self::SenderProperty {
                interface,
                property,
            }
            | Self::EventArgsProperty {
                interface,
                property,
            }
            | Self::EventArgsInspectableString {
                interface,
                property,
            }
            | Self::EventArgsItemTag {
                interface,
                property,
            }
            | Self::EventArgsTreeNodeContent {
                interface,
                property,
            }
            | Self::SenderRichEditText {
                interface,
                property,
            }
            | Self::SenderItemTags {
                interface,
                property,
            } => Some((interface, property)),
            Self::Unit | Self::DragInfo { .. } | Self::DropData { .. } | Self::PointerEvent => None,
        }
    }

    pub(crate) fn property(&self) -> Option<&str> {
        self.getter().map(|(_, property)| property)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum EventSubscription {
    Metadata,
    PropertyChanged {
        property: String,
        static_owner: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EventPayloadConversion {
    Identity,
    Field(String),
    Nullable,
    NumberBoxValue,
    RatingValue,
    Selection,
    SelectionIndex,
}

impl Schema {
    pub(crate) fn parse(source: &str) -> Result<Self, String> {
        toml::from_str(source).map_err(|error| error.to_string())
    }

    pub(crate) fn resolve(self, metadata: &MetadataResolver) -> Result<ResolvedSchema, String> {
        let mut controls = Vec::with_capacity(self.control.len());
        let mut control_names = HashSet::new();

        for control in self.control {
            let (_, name) = control.type_name.rsplit_once('.').ok_or_else(|| {
                format!(
                    "control type must be fully qualified: {}",
                    control.type_name
                )
            })?;
            let name = name.to_string();
            if !control_names.insert(control.type_name.clone()) {
                return Err(format!("duplicate control {}", control.type_name));
            }
            if (control.type_name == "Microsoft.UI.Xaml.Controls.ToolTip")
                != (control.placement == Some(Placement::TooltipAttachment))
            {
                return Err(format!(
                    "{} has no valid native placement contract",
                    control.type_name
                ));
            }
            let role = derive_role(&control)?;
            validate_role(&control, role)?;
            validate_native_role(&control, role, &name, metadata)?;
            let placement = match (
                control.placement,
                control.lifecycle,
                control.capabilities.contains(&Capability::WindowTitleBar),
                metadata.class_derives_from(&control.type_name, "Microsoft.UI.Xaml.UIElement"),
            ) {
                (Some(Placement::TooltipAttachment), None, false, _) => {
                    ResolvedPlacement::TooltipAttachment
                }
                (None, Some(Lifecycle::ContentDialog), false, _) => ResolvedPlacement::Declaration,
                (None, None, true, true) => ResolvedPlacement::WindowLifetime,
                (None, None, false, true) => ResolvedPlacement::Visual,
                _ => {
                    return Err(format!(
                        "{} has no valid native placement contract",
                        control.type_name
                    ));
                }
            };

            let content = if matches!(role, Role::Content) {
                let metadata_content =
                    metadata
                        .content_property(&control.type_name)
                        .ok_or_else(|| {
                            format!("{} has no metadata content property", control.type_name)
                        })?;
                if let Some(content) = control.content.as_deref()
                    && content != metadata_content
                {
                    return Err(format!(
                        "{} content property {} does not match metadata property {}",
                        control.type_name, content, metadata_content
                    ));
                }
                let content = metadata_content.as_str();
                let method = format!("put_{content}");
                let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} is not a metadata content property",
                        control.type_name, content
                    )
                })?;
                let target = match metadata.classify_param(&name, &method) {
                    Some(ParamClass::IInspectable) => SlotTarget::Inspectable,
                    Some(ParamClass::Complex)
                        if metadata.param_class_name(&name, &method).as_deref()
                            == Some("Microsoft.UI.Xaml.UIElement") =>
                    {
                        SlotTarget::UiElement
                    }
                    _ => {
                        return Err(format!(
                            "{}.{} has an unsupported content parameter",
                            control.type_name, content
                        ));
                    }
                };
                Some(ResolvedContent {
                    name: content.to_string(),
                    interface: interface.full_path(),
                    target,
                })
            } else {
                None
            };

            let mut properties = Vec::with_capacity(control.property.len());
            let mut events = Vec::with_capacity(control.event.len());
            let mut slots = Vec::with_capacity(control.slot.len());
            let mut member_names = HashSet::new();
            let mut field_names = HashSet::new();

            validate_required_property_adapters(&control.type_name, &control.property)?;
            for property in control.property {
                let feedback = match (&property.controlled, &property.coerces) {
                    (Some(_), Some(_)) => {
                        return Err(format!(
                            "{}.{} cannot be controlled and coercing",
                            control.type_name, property.name
                        ));
                    }
                    (Some(event), None) | (None, Some(event)) => Some(event.clone()),
                    (None, None) => None,
                };
                if property.coerces.is_some()
                    && property.feedback_contract != Some(FeedbackContract::SynchronousNormalized)
                {
                    return Err(format!(
                        "{}.{} coercion needs synchronous_normalized feedback",
                        control.type_name, property.name
                    ));
                }
                match (feedback.as_ref(), property.feedback_contract) {
                    (Some(_), Some(_)) => {}
                    (Some(_), None) => {
                        return Err(format!(
                            "{}.{} needs a feedback contract",
                            control.type_name, property.name
                        ));
                    }
                    (None, Some(_)) => {
                        return Err(format!(
                            "{}.{} has a feedback contract but is not controlled",
                            control.type_name, property.name
                        ));
                    }
                    (None, None) => {}
                }
                validate_member(
                    &control.type_name,
                    &property.name,
                    property.field.as_deref(),
                    &mut member_names,
                    &mut field_names,
                )?;
                let method = match property.adapter {
                    Some(
                        PropertyAdapter::KeyAccelerators
                        | PropertyAdapter::ResourceOverrides
                        | PropertyAdapter::RichEditText
                        | PropertyAdapter::RichTextBlocks,
                    ) => {
                        format!("get_{}", property.name)
                    }
                    Some(PropertyAdapter::PointerCapture) => "add_PointerPressed".to_string(),
                    Some(PropertyAdapter::DropPolicy) => "put_AllowDrop".to_string(),
                    _ => format!("put_{}", property.name),
                };
                let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} is not a metadata property",
                        control.type_name, property.name
                    )
                })?;
                let static_owner = metadata.runtime_class(interface).ok_or_else(|| {
                    format!(
                        "{}.{} has no exclusive runtime class owner",
                        control.type_name, property.name
                    )
                })?;
                if property.theme_style
                    && (feedback.is_some()
                        || property.validation.is_some()
                        || property.feedback_contract.is_some())
                {
                    return Err(format!(
                        "{}.{} theme style properties cannot use feedback or validation",
                        control.type_name, property.name
                    ));
                }
                if property.adapter.is_some()
                    && !matches!(
                        property.adapter,
                        Some(
                            PropertyAdapter::ImplicitScale
                                | PropertyAdapter::NumberBoxValue
                                | PropertyAdapter::RatingValue
                                | PropertyAdapter::RichEditText
                                | PropertyAdapter::SelectionIndex
                        )
                    )
                    && (property.theme_style
                        || feedback.is_some()
                        || property.validation.is_some()
                        || property.feedback_contract.is_some())
                {
                    return Err(format!(
                        "{}.{} adapted properties cannot use theme style, feedback, or validation",
                        control.type_name, property.name
                    ));
                }
                if property.adapter != Some(PropertyAdapter::ResourceStyle)
                    && !property.variants.is_empty()
                {
                    return Err(format!(
                        "{}.{} variants require the resource_style adapter",
                        control.type_name, property.name
                    ));
                }
                let (value, copy) = match property.adapter {
                    Some(adapter) => match adapter.property_kind() {
                        PropertyAdapterKind::Property(contract) => {
                            if !contract.validate(
                                &control.type_name,
                                &property.name,
                                metadata,
                                &name,
                                &method,
                            ) {
                                return Err(format!(
                                    "{}.{} {}",
                                    control.type_name, property.name, contract.requirement
                                ));
                            }
                            (contract.value.to_string(), contract.copy)
                        }
                        PropertyAdapterKind::EventOnly(requirement) => {
                            return Err(format!(
                                "{}.{} {requirement}",
                                control.type_name, property.name
                            ));
                        }
                        PropertyAdapterKind::ResourceStyle => {
                            if metadata.param_class_name(&name, &method).as_deref()
                                != Some("Microsoft.UI.Xaml.Style")
                                || property.variants.is_empty()
                            {
                                return Err(format!(
                                    "{}.{} resource_style requires a Style property and variants",
                                    control.type_name, property.name
                                ));
                            }
                            let mut names = HashSet::new();
                            if property
                                .variants
                                .iter()
                                .any(|variant| !names.insert(variant.name.clone()))
                            {
                                return Err(format!(
                                    "{}.{} resource_style has duplicate variants",
                                    control.type_name, property.name
                                ));
                            }
                            (format!("{}{}", name, property.name), true)
                        }
                    },
                    None if property.theme_style => ("Brush".to_string(), true),
                    None => metadata.infer_value_type(&name, &method).ok_or_else(|| {
                        format!(
                            "{}.{} has an unsupported metadata type",
                            control.type_name, property.name
                        )
                    })?,
                };
                if property.clear_feedback.is_some()
                    && (value != "Bool"
                        || property.feedback_contract != Some(FeedbackContract::SynchronousExact))
                {
                    return Err(format!(
                        "{}.{} clear_feedback requires a Bool property with synchronous_exact feedback",
                        control.type_name, property.name
                    ));
                }
                if value == "Bool"
                    && property.feedback_contract == Some(FeedbackContract::SynchronousExact)
                    && property.clear_feedback.is_none()
                {
                    return Err(format!(
                        "{}.{} controlled Bool requires an explicit clear_feedback value",
                        control.type_name, property.name
                    ));
                }
                let enum_variants = if property.adapter == Some(PropertyAdapter::ResourceStyle) {
                    property
                        .variants
                        .iter()
                        .map(|variant| variant.name.clone())
                        .collect()
                } else {
                    property
                        .adapter
                        .is_none()
                        .then(|| metadata.enum_info(&name, &method))
                        .flatten()
                        .map(|(_, variants)| variants.to_vec())
                        .unwrap_or_default()
                };
                let native_wrapper = (!property.theme_style && property.adapter.is_none())
                    .then(|| metadata.single_field_param(&name, &method))
                    .flatten();
                let native_value = (!property.theme_style && property.adapter.is_none())
                    .then(|| metadata.enum_path(&name, &method))
                    .flatten()
                    .or_else(|| native_wrapper.as_ref().map(|(path, _)| path.clone()));
                let nullable_bool =
                    metadata.classify_param(&name, &method) == Some(ParamClass::NullableBool);
                if nullable_bool && (value != "Bool" || property.adapter.is_some()) {
                    return Err(format!(
                        "{}.{} has an invalid nullable boolean contract",
                        control.type_name, property.name
                    ));
                }
                validate_property_value(
                    &control.type_name,
                    &property.name,
                    &value,
                    property.validation,
                )?;

                properties.push(ResolvedProperty {
                    field: property
                        .field
                        .unwrap_or_else(|| to_snake_case(&property.name)),
                    name: property.name,
                    value,
                    copy,
                    interface: interface.full_path(),
                    static_owner,
                    feedback,
                    feedback_contract: property.feedback_contract,
                    clear_feedback: property.clear_feedback,
                    adapter: property.adapter,
                    nullable_bool,
                    observes_feedback: property.controlled.is_some(),
                    enum_variants,
                    resource_style_variants: property.variants,
                    native_value,
                    native_wrapper,
                    validation: property.validation,
                    theme_style: property.theme_style,
                });
            }
            if properties
                .iter()
                .filter(|property| property.theme_style)
                .count()
                > 4
            {
                return Err(format!(
                    "{} has more than four theme style properties",
                    control.type_name
                ));
            }

            for event in control.event {
                validate_member(
                    &control.type_name,
                    &event.name,
                    event.field.as_deref(),
                    &mut member_names,
                    &mut field_names,
                )?;
                if event.observe.is_some() && event.property.is_some() {
                    return Err(format!(
                        "{}.{} cannot set both property and observe",
                        control.type_name, event.name
                    ));
                }
                if let Some(active_property) = event.active_property.as_deref()
                    && !properties
                        .iter()
                        .any(|property| property.field == active_property)
                {
                    return Err(format!(
                        "{}.{} active_property `{active_property}` is not a property field",
                        control.type_name, event.name
                    ));
                }
                if event.adapter.is_some()
                    && !matches!(
                        event.adapter,
                        Some(
                            PropertyAdapter::ItemTags
                                | PropertyAdapter::PointerEvent
                                | PropertyAdapter::DragInfo
                                | PropertyAdapter::DropData
                        )
                    )
                    && event.property.is_none()
                {
                    return Err(format!(
                        "{}.{} event adapters require property",
                        control.type_name, event.name
                    ));
                }
                let selection = control
                    .selection
                    .as_ref()
                    .filter(|selection| selection.event == event.name);
                let (payload, interface, source, conversion, subscription) = if let Some(
                    selection,
                ) = selection
                {
                    if event.property.is_some() || event.observe.is_some() {
                        return Err(format!(
                            "{}.{} selection event cannot set property or observe",
                            control.type_name, event.name
                        ));
                    }
                    let method = format!("add_{}", event.name);
                    let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                        format!(
                            "{}.{} is not a metadata event",
                            control.type_name, event.name
                        )
                    })?;
                    metadata
                        .resolve(&name, &format!("get_{}", selection.selected_item_property))
                        .ok_or_else(|| {
                            format!(
                                "{}.{} has no selected-item property {}",
                                control.type_name, event.name, selection.selected_item_property
                            )
                        })?;
                    let source = if let Some(event_args) = &selection.event_args {
                        let property_interface = metadata
                            .resolve(
                                event_args,
                                &format!("get_{}", selection.selected_item_property),
                            )
                            .ok_or_else(|| {
                                format!(
                                    "{}.{} event args have no selected-item property {}",
                                    control.type_name, event.name, selection.selected_item_property
                                )
                            })?
                            .full_path();
                        EventPayloadSource::EventArgsProperty {
                            interface: property_interface,
                            property: selection.selected_item_property.clone(),
                        }
                    } else {
                        EventPayloadSource::SenderProperty {
                            interface: metadata
                                .resolve(
                                    &name,
                                    &format!("get_{}", selection.selected_item_property),
                                )
                                .unwrap()
                                .full_path(),
                            property: selection.selected_item_property.clone(),
                        }
                    };
                    (
                        "SelectionChange".to_string(),
                        interface.full_path(),
                        source,
                        ReadValueConversion::Identity,
                        EventSubscription::Metadata,
                    )
                } else if let Some(observe) = event.observe.as_deref() {
                    let observed = properties
                        .iter()
                        .find(|property| property.name == observe)
                        .ok_or_else(|| {
                            format!(
                                "{}.{} observes missing property {}",
                                control.type_name, event.name, observe
                            )
                        })?;
                    let (payload, interface, conversion) = metadata
                        .resolve_property_read(&name, observe)
                        .ok_or_else(|| {
                            format!(
                                "{}.{} observes unsupported property {}",
                                control.type_name, event.name, observe
                            )
                        })?;
                    if payload != observed.value {
                        return Err(format!(
                            "{}.{} observed payload {} does not match property value {}",
                            control.type_name, event.name, payload, observed.value
                        ));
                    }
                    (
                        payload,
                        interface,
                        EventPayloadSource::SenderProperty {
                            interface: observed.interface.clone(),
                            property: observe.to_string(),
                        },
                        conversion,
                        EventSubscription::PropertyChanged {
                            property: observe.to_string(),
                            static_owner: observed.static_owner.clone(),
                        },
                    )
                } else {
                    let method = format!("add_{}", event.name);
                    let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                        format!(
                            "{}.{} is not a metadata event",
                            control.type_name, event.name
                        )
                    })?;
                    let (payload, source, conversion) = if event.adapter
                        == Some(PropertyAdapter::ItemTags)
                    {
                        let property = event
                            .property
                            .clone()
                            .unwrap_or_else(|| "TabItems".to_string());
                        let property_interface = metadata
                            .resolve(&name, &format!("get_{property}"))
                            .ok_or_else(|| {
                                format!(
                                    "{}.{} has unsupported sender property {}",
                                    control.type_name, event.name, property
                                )
                            })?
                            .full_path();
                        (
                            "StrList".to_string(),
                            EventPayloadSource::SenderItemTags {
                                interface: property_interface,
                                property,
                            },
                            ReadValueConversion::Identity,
                        )
                    } else if event.adapter == Some(PropertyAdapter::PointerEvent) {
                        if event.property.is_some()
                            || !matches!(
                                event.name.as_str(),
                                "PointerEntered"
                                    | "PointerExited"
                                    | "PointerMoved"
                                    | "PointerPressed"
                                    | "PointerReleased"
                            )
                        {
                            return Err(format!(
                                "{}.{} has an invalid pointer_event contract",
                                control.type_name, event.name
                            ));
                        }
                        (
                            "PointerEventInfo".to_string(),
                            EventPayloadSource::PointerEvent,
                            ReadValueConversion::Identity,
                        )
                    } else if matches!(
                        event.adapter,
                        Some(PropertyAdapter::DragInfo | PropertyAdapter::DropData)
                    ) {
                        let expected = if event.adapter == Some(PropertyAdapter::DropData) {
                            "Drop"
                        } else {
                            event.name.as_str()
                        };
                        if !matches!(expected, "DragEnter" | "DragOver" | "Drop")
                            || event.property.is_some()
                        {
                            return Err(format!(
                                "{}.{} drag adapters require DragEnter, DragOver, or Drop without a property",
                                control.type_name, event.name
                            ));
                        }
                        let interface = metadata
                            .resolve_event_args_property_interface(&name, &method, "DataView")
                            .ok_or_else(|| {
                                format!(
                                    "{}.{} event args have no DataView property",
                                    control.type_name, event.name
                                )
                            })?;
                        if event.adapter == Some(PropertyAdapter::DropData) {
                            (
                                "DroppedData".to_string(),
                                EventPayloadSource::DropData { interface },
                                ReadValueConversion::Identity,
                            )
                        } else {
                            (
                                "DragKind".to_string(),
                                EventPayloadSource::DragInfo { interface },
                                ReadValueConversion::Identity,
                            )
                        }
                    } else if let Some(property) = event.property.as_deref() {
                        let sender_property = format!("put_{property}");
                        if event.adapter == Some(PropertyAdapter::NavigationDisplayMode) {
                            if control.type_name != "Microsoft.UI.Xaml.Controls.NavigationView"
                                || event.name != "DisplayModeChanged"
                                || property != "DisplayMode"
                            {
                                return Err(format!(
                                    "{}.{} navigation_display_mode requires NavigationView.DisplayModeChanged.DisplayMode",
                                    control.type_name, event.name
                                ));
                            }
                            let interface = metadata
                                .resolve(
                                    "NavigationViewDisplayModeChangedEventArgs",
                                    "get_DisplayMode",
                                )
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported display mode",
                                        control.type_name, event.name
                                    )
                                })?
                                .full_path();
                            (
                                "NavigationViewDisplayMode".to_string(),
                                EventPayloadSource::EventArgsProperty {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else if event.adapter == Some(PropertyAdapter::RichEditText) {
                            if control.type_name != "Microsoft.UI.Xaml.Controls.RichEditBox"
                                || event.name != "TextChanged"
                                || property != "Document"
                            {
                                return Err(format!(
                                    "{}.{} rich_edit_text requires RichEditBox.TextChanged.Document",
                                    control.type_name, event.name
                                ));
                            }
                            let interface = metadata
                                .resolve(&name, "get_Document")
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported document property",
                                        control.type_name, event.name
                                    )
                                })?
                                .full_path();
                            (
                                "Str".to_string(),
                                EventPayloadSource::SenderRichEditText {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else if metadata.has_method(&name, &sender_property) {
                            let (payload, interface, conversion) = metadata
                                .resolve_property_read(&name, property)
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported sender property {}",
                                        control.type_name, event.name, property
                                    )
                                })?;
                            (
                                payload,
                                EventPayloadSource::SenderProperty {
                                    interface,
                                    property: property.to_string(),
                                },
                                conversion,
                            )
                        } else if event.adapter == Some(PropertyAdapter::ContentDialogResult) {
                            if control.lifecycle != Some(Lifecycle::ContentDialog)
                                || event.name != "Closed"
                                || property != "Result"
                            {
                                return Err(format!(
                                    "{}.{} content_dialog_result requires ContentDialog.Closed.Result",
                                    control.type_name, event.name
                                ));
                            }
                            let interface = metadata
                                .resolve("ContentDialogClosedEventArgs", "get_Result")
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported event property {}",
                                        control.type_name, event.name, property
                                    )
                                })?
                                .full_path();
                            (
                                "ContentDialogResult".to_string(),
                                EventPayloadSource::EventArgsProperty {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else if event.adapter == Some(PropertyAdapter::ItemTag) {
                            let interface = metadata
                                .resolve_event_args_class_property(&name, &method, property)
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} item_tag requires a class-typed event property {}",
                                        control.type_name, event.name, property
                                    )
                                })?;
                            (
                                "Str".to_string(),
                                EventPayloadSource::EventArgsItemTag {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else if event.adapter == Some(PropertyAdapter::InspectableString) {
                            let interface = metadata
                                        .resolve_event_args_object_property(
                                            &name,
                                            &method,
                                            property,
                                        )
                                        .ok_or_else(|| {
                                            format!(
                                                "{}.{} inspectable_string requires an object event property {}",
                                                control.type_name, event.name, property
                                            )
                                        })?;
                            (
                                "Str".to_string(),
                                EventPayloadSource::EventArgsInspectableString {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else if event.adapter == Some(PropertyAdapter::TreeNodeContent) {
                            if control.type_name != "Microsoft.UI.Xaml.Controls.TreeView"
                                || event.name != "ItemInvoked"
                                || property != "InvokedItem"
                            {
                                return Err(format!(
                                    "{}.{} tree_node_content requires TreeView.ItemInvoked.InvokedItem",
                                    control.type_name, event.name
                                ));
                            }
                            let interface = metadata
                                .resolve_event_args_object_property(&name, &method, property)
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported tree-node property {}",
                                        control.type_name, event.name, property
                                    )
                                })?;
                            (
                                "Str".to_string(),
                                EventPayloadSource::EventArgsTreeNodeContent {
                                    interface,
                                    property: property.to_string(),
                                },
                                ReadValueConversion::Identity,
                            )
                        } else {
                            let (payload, interface, conversion) = metadata
                                .resolve_event_args_property(&name, &method, property)
                                .ok_or_else(|| {
                                    format!(
                                        "{}.{} has unsupported event property {}",
                                        control.type_name, event.name, property
                                    )
                                })?;
                            (
                                payload,
                                EventPayloadSource::EventArgsProperty {
                                    interface,
                                    property: property.to_string(),
                                },
                                conversion,
                            )
                        }
                    } else {
                        (
                            "Unit".to_string(),
                            EventPayloadSource::Unit,
                            ReadValueConversion::Identity,
                        )
                    };
                    (
                        payload,
                        interface.full_path(),
                        source,
                        conversion,
                        EventSubscription::Metadata,
                    )
                };
                let mut conversion = if selection.is_some() {
                    EventPayloadConversion::Selection
                } else {
                    match conversion {
                        ReadValueConversion::Identity => EventPayloadConversion::Identity,
                        ReadValueConversion::Field(field) => EventPayloadConversion::Field(field),
                        ReadValueConversion::Nullable => EventPayloadConversion::Nullable,
                    }
                };
                let mut payload = payload;
                if let Some(property) = properties.iter().find(|property| {
                    property.observes_feedback
                        && property.feedback.as_deref() == Some(event.name.as_str())
                }) {
                    match property.adapter {
                        Some(PropertyAdapter::NumberBoxValue) => {
                            payload = "OptionalF64".to_string();
                            conversion = EventPayloadConversion::NumberBoxValue;
                        }
                        Some(PropertyAdapter::RatingValue) => {
                            payload = "OptionalF64".to_string();
                            conversion = EventPayloadConversion::RatingValue;
                        }
                        Some(PropertyAdapter::SelectionIndex) => {
                            payload = "SelectionIndex".to_string();
                            conversion = EventPayloadConversion::SelectionIndex;
                        }
                        _ => {}
                    }
                } else if conversion == EventPayloadConversion::Nullable {
                    payload = format!("Optional{payload}");
                }

                events.push(ResolvedEvent {
                    field: event
                        .field
                        .unwrap_or_else(|| format!("on_{}", to_snake_case(&event.name))),
                    name: event.name,
                    payload,
                    interface,
                    source,
                    conversion,
                    subscription,
                    active_property: event.active_property,
                });
            }

            for slot in control.slot {
                validate_member(
                    &control.type_name,
                    &slot.name,
                    None,
                    &mut member_names,
                    &mut field_names,
                )?;
                let method = format!(
                    "{}_{}",
                    if slot.collection { "get" } else { "put" },
                    slot.name
                );
                let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} is not a metadata {} slot property",
                        control.type_name,
                        slot.name,
                        if slot.collection {
                            "collection"
                        } else {
                            "single"
                        }
                    )
                })?;
                let collection_cast = slot.collection
                    && !metadata.returns_inspectable_vector(&name, &method)
                    && metadata.return_class_name(&name, &method).as_deref()
                        == Some("Microsoft.UI.Xaml.Controls.ItemCollection");
                let collection_item = slot
                    .collection
                    .then(|| metadata.return_vector_element_class_name(&name, &method))
                    .flatten();
                if slot.collection
                    && !collection_cast
                    && collection_item.is_none()
                    && !metadata.returns_inspectable_vector(&name, &method)
                {
                    return Err(format!(
                        "{}.{} collection slot must return IVector<IInspectable>, a typed class \
                         vector, or ItemCollection",
                        control.type_name, slot.name
                    ));
                }
                let shape = if slot.collection {
                    SlotShape::Collection
                } else {
                    match metadata.classify_param(&name, &method) {
                        Some(ParamClass::IInspectable) => {
                            SlotShape::Single(SlotTarget::Inspectable)
                        }
                        Some(ParamClass::Complex)
                            if metadata.param_class_name(&name, &method).as_deref()
                                == Some("Microsoft.UI.Xaml.UIElement") =>
                        {
                            SlotShape::Single(SlotTarget::UiElement)
                        }
                        Some(ParamClass::Complex)
                            if metadata.param_class_name(&name, &method).as_deref()
                                == Some("Microsoft.UI.Xaml.Controls.IconElement") =>
                        {
                            SlotShape::Single(SlotTarget::IconElement)
                        }
                        _ => {
                            return Err(format!(
                                "{}.{} has an unsupported slot parameter",
                                control.type_name, slot.name
                            ));
                        }
                    }
                };
                if !slot.collection && !slot.item_controls.is_empty() {
                    return Err(format!(
                        "{}.{} item_controls requires a collection slot",
                        control.type_name, slot.name
                    ));
                }
                slots.push(ResolvedSlot {
                    name: slot.name,
                    interface: interface.full_path(),
                    shape,
                    collection_cast,
                    collection_observable: slot.collection
                        && metadata.returns_observable_vector(&name, &method),
                    collection_item,
                    item_controls: slot.item_controls,
                });
            }

            let coercing_events = properties
                .iter()
                .filter(|property| !property.observes_feedback)
                .filter_map(|property| property.feedback.as_deref())
                .collect::<HashSet<_>>();
            let mut observed_feedback_events = HashSet::new();
            for property in properties
                .iter()
                .filter(|property| property.feedback.is_some())
            {
                let feedback = property.feedback.as_deref().unwrap();
                if coercing_events.contains(feedback)
                    && property.feedback_contract != Some(FeedbackContract::SynchronousNormalized)
                {
                    return Err(format!(
                        "{} feedback event {} is coercing and requires synchronous_normalized",
                        control.type_name, feedback
                    ));
                }
                if property.observes_feedback && !observed_feedback_events.insert(feedback) {
                    return Err(format!(
                        "{} assigns feedback event {} to multiple controlled properties",
                        control.type_name, feedback
                    ));
                }
                let Some(event) = events.iter().find(|event| event.name == feedback) else {
                    return Err(format!(
                        "{}.{} names missing feedback event {}",
                        control.type_name, property.name, feedback
                    ));
                };
                if property.observes_feedback && event.payload != property.value {
                    return Err(format!(
                        "{}.{} value {} does not match {} payload {}",
                        control.type_name, property.name, property.value, feedback, event.payload
                    ));
                }
                if property.observes_feedback
                    && matches!(
                        event.subscription,
                        EventSubscription::PropertyChanged { .. }
                    )
                    && event.source.property() != Some(property.name.as_str())
                {
                    return Err(format!(
                        "{}.{} feedback event {} observes a different property",
                        control.type_name, property.name, feedback
                    ));
                }
            }

            let selection = control
                .selection
                .map(|selection| -> Result<ResolvedSelection, String> {
                    let owner_interface = metadata
                        .resolve(&name, &format!("get_{}", selection.selected_item_property))
                        .ok_or_else(|| {
                            format!(
                                "{} has no selected-item property {}",
                                control.type_name, selection.selected_item_property
                            )
                        })?
                        .full_path();
                    let selected_item = metadata.return_class_name(
                        &name,
                        &format!("get_{}", selection.selected_item_property),
                    );
                    let selected_interface = metadata
                        .resolve(
                            &selection.item,
                            &format!("get_{}", selection.selected_property),
                        )
                        .ok_or_else(|| {
                            format!(
                                "{} has no selected property {}",
                                selection.item, selection.selected_property
                            )
                        })?
                        .full_path();
                    let payload_interface = metadata
                        .resolve(
                            &selection.item,
                            &format!("get_{}", selection.payload_property),
                        )
                        .ok_or_else(|| {
                            format!(
                                "{} has no payload property {}",
                                selection.item, selection.payload_property
                            )
                        })?
                        .full_path();
                    let payload_inspectable = metadata.returns_object(
                        &selection.item,
                        &format!("get_{}", selection.payload_property),
                    );
                    Ok(ResolvedSelection {
                        slot: selection.slot,
                        item: selection.item,
                        selected_property: selection.selected_property,
                        selected_item_property: selection.selected_item_property,
                        selected_item,
                        event: selection.event,
                        payload_property: selection.payload_property,
                        owner_interface,
                        selected_interface,
                        payload_interface,
                        payload_inspectable,
                    })
                })
                .transpose()?;
            controls.push(ResolvedControl {
                name,
                type_name: control.type_name,
                role,
                placement,
                lifecycle: control.lifecycle,
                content,
                capabilities: control.capabilities,
                properties,
                events,
                slots,
                selection,
            });
        }

        validate_selections(&controls)?;
        validate_slot_item_controls(&controls)?;
        Ok(ResolvedSchema { controls })
    }
}

fn validate_required_property_adapters(
    control: &str,
    properties: &[Property],
) -> Result<(), String> {
    let required = [
        (
            "Microsoft.UI.Xaml.Controls.PathIcon",
            "Data",
            PropertyAdapter::PathData,
            "path_data",
        ),
        (
            "Microsoft.UI.Xaml.Controls.TimePicker",
            "ClockIdentifier",
            PropertyAdapter::ClockIdentifier,
            "clock_identifier",
        ),
    ];
    for (owner, property, adapter, adapter_name) in required {
        if control == owner
            && properties
                .iter()
                .find(|candidate| candidate.name == property)
                .is_some_and(|candidate| candidate.adapter != Some(adapter))
        {
            return Err(format!(
                "{control}.{property} must use the {adapter_name} adapter"
            ));
        }
    }
    Ok(())
}

fn validate_slot_item_controls(controls: &[ResolvedControl]) -> Result<(), String> {
    for control in controls {
        for slot in &control.slots {
            for item in &slot.item_controls {
                if !controls.iter().any(|candidate| candidate.name == *item) {
                    return Err(format!(
                        "{}.{} names missing item control {}",
                        control.type_name, slot.name, item
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_selections(controls: &[ResolvedControl]) -> Result<(), String> {
    for control in controls {
        let Some(selection) = control.selection.as_ref() else {
            continue;
        };
        let slot = control
            .slots
            .iter()
            .find(|slot| slot.name == selection.slot)
            .ok_or_else(|| {
                format!(
                    "{} selection names missing slot {}",
                    control.type_name, selection.slot
                )
            })?;
        if !matches!(slot.shape, SlotShape::Collection) {
            return Err(format!(
                "{} selection slot {} is not a collection",
                control.type_name, selection.slot
            ));
        }
        let event = control
            .events
            .iter()
            .find(|event| event.name == selection.event)
            .ok_or_else(|| {
                format!(
                    "{} selection names missing event {}",
                    control.type_name, selection.event
                )
            })?;
        if event.conversion != EventPayloadConversion::Selection {
            return Err(format!(
                "{} selection event {} has the wrong payload conversion",
                control.type_name, selection.event
            ));
        }
        let item = controls
            .iter()
            .find(|candidate| candidate.name == selection.item)
            .ok_or_else(|| {
                format!(
                    "{} selection names missing item control {}",
                    control.type_name, selection.item
                )
            })?;
        let selected = item
            .properties
            .iter()
            .find(|property| property.name == selection.selected_property)
            .ok_or_else(|| {
                format!(
                    "{} selection item {} has no selected property {}",
                    control.type_name, selection.item, selection.selected_property
                )
            })?;
        if selected.value != "Bool" {
            return Err(format!(
                "{} selection item property {}.{} is not Bool",
                control.type_name, selection.item, selection.selected_property
            ));
        }
        let payload = item
            .properties
            .iter()
            .find(|property| property.name == selection.payload_property)
            .ok_or_else(|| {
                format!(
                    "{} selection item {} has no payload property {}",
                    control.type_name, selection.item, selection.payload_property
                )
            })?;
        if payload.value != "Str" {
            return Err(format!(
                "{} selection payload property {}.{} is not Str",
                control.type_name, selection.item, selection.payload_property
            ));
        }
        if selection.payload_inspectable
            && payload.adapter != Some(PropertyAdapter::InspectableString)
        {
            return Err(format!(
                "{} selection payload property {}.{} must use inspectable_string",
                control.type_name, selection.item, selection.payload_property
            ));
        }
        if !selection.payload_inspectable && payload.adapter.is_some() {
            return Err(format!(
                "{} selection payload property {}.{} must use its native string type",
                control.type_name, selection.item, selection.payload_property
            ));
        }
    }
    Ok(())
}

fn derive_role(control: &Control) -> Result<Role, String> {
    let has = |capability| control.capabilities.contains(&capability);
    let structural = [
        (!control.slot.is_empty(), Role::Slots, "slots"),
        (has(Capability::Items), Role::Virtual, "items"),
        (has(Capability::Children), Role::Children, "children"),
        (
            has(Capability::Content) || control.content.is_some(),
            Role::Content,
            "content",
        ),
    ]
    .into_iter()
    .filter(|(present, _, _)| *present)
    .collect::<Vec<_>>();
    match structural.as_slice() {
        [] => Ok(Role::Leaf),
        [(_, role, _)] => Ok(*role),
        _ => Err(format!(
            "{} has ambiguous structural declarations: {}",
            control.type_name,
            structural
                .iter()
                .map(|(_, _, name)| *name)
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn validate_role(control: &Control, role: Role) -> Result<(), String> {
    let has = |capability| control.capabilities.contains(&capability);
    if has(Capability::GridDefinitions)
        && (control.type_name != "Microsoft.UI.Xaml.Controls.Grid"
            || !matches!(role, Role::Children))
    {
        return Err(format!(
            "{} grid_definitions capability requires the Grid children role",
            control.type_name
        ));
    }
    if has(Capability::WindowTitleBar) && control.type_name != "Microsoft.UI.Xaml.Controls.TitleBar"
    {
        return Err(format!(
            "{} window_title_bar capability requires the WinUI TitleBar control",
            control.type_name
        ));
    }
    match role {
        Role::Content if !has(Capability::Content) => Err(format!(
            "{} content declaration needs content capability",
            control.type_name
        )),
        Role::Leaf | Role::Content | Role::Children | Role::Slots | Role::Virtual => Ok(()),
    }
}

fn validate_native_role(
    control: &Control,
    role: Role,
    name: &str,
    metadata: &MetadataResolver,
) -> Result<(), String> {
    let expected = match role {
        Role::Children => Some(("get_Children", "IPanel")),
        Role::Leaf | Role::Content | Role::Slots | Role::Virtual => None,
    };
    let Some((method, interface)) = expected else {
        return Ok(());
    };
    if metadata
        .resolve(name, method)
        .is_some_and(|resolved| resolved.short_name() == interface)
    {
        Ok(())
    } else {
        Err(format!(
            "{} {} role requires metadata interface {}",
            control.type_name,
            match role {
                Role::Children => "children",
                _ => unreachable!(),
            },
            interface
        ))
    }
}

fn validate_property_value(
    control: &str,
    property: &str,
    value: &str,
    validation: Option<ValueValidation>,
) -> Result<(), String> {
    let valid = match validation {
        None => true,
        Some(ValueValidation::Finite) => matches!(value, "F64" | "Thickness"),
        Some(ValueValidation::FinitePositive) => value == "F64",
        Some(ValueValidation::FiniteNonNegative) => {
            matches!(value, "F64" | "Thickness" | "CornerRadius")
        }
        Some(ValueValidation::NonNegative | ValueValidation::ZeroToFiftyNine) => value == "I32",
    };
    if valid {
        Ok(())
    } else {
        Err(format!(
            "{control}.{property} value {value} does not support {validation:?}"
        ))
    }
}

fn validate_member(
    control: &str,
    member: &str,
    field: Option<&str>,
    members: &mut HashSet<String>,
    fields: &mut HashSet<String>,
) -> Result<(), String> {
    if !members.insert(member.to_string()) {
        return Err(format!("{control} has duplicate member {member}"));
    }
    let field = field.map_or_else(|| to_snake_case(member), ToOwned::to_owned);
    if !fields.insert(field.clone()) {
        return Err(format!("{control} has duplicate field {field}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_selection_contracts() {
        let source = include_str!("winui.toml");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let cases = [
            (
                "slot = \"MenuItems\"",
                "slot = \"Missing\"",
                "selection names missing slot Missing",
            ),
            (
                "slot = \"MenuItems\"",
                "slot = \"Content\"",
                "selection slot Content is not a collection",
            ),
            (
                "item = \"NavigationViewItem\"",
                "item = \"Missing\"",
                "Missing has no selected property IsSelected",
            ),
            (
                "selected_property = \"IsSelected\"",
                "selected_property = \"Missing\"",
                "NavigationViewItem has no selected property Missing",
            ),
            (
                "selected_property = \"IsSelected\"",
                "selected_property = \"Tag\"",
                "selection item property NavigationViewItem.Tag is not Bool",
            ),
            (
                "payload_property = \"Tag\"",
                "payload_property = \"Missing\"",
                "NavigationViewItem has no payload property Missing",
            ),
            (
                "payload_property = \"Tag\"",
                "payload_property = \"IsSelected\"",
                "selection payload property NavigationViewItem.IsSelected is not Str",
            ),
            (
                "adapter = \"inspectable_string\"",
                "# adapter removed",
                "selection payload property NavigationViewItem.Tag must use inspectable_string",
            ),
            (
                "selected_item_property = \"SelectedItem\"",
                "selected_item_property = \"Missing\"",
                "has no selected-item property Missing",
            ),
            (
                "selected_item_property = \"SelectedItem\"",
                "selected_item_property = \"IsPaneOpen\"",
                "event args have no selected-item property IsPaneOpen",
            ),
            (
                "event = \"SelectionChanged\"",
                "event = \"Missing\"",
                "selection names missing event Missing",
            ),
        ];

        for (from, to, expected) in cases {
            let changed = if expected
                == "selection payload property NavigationViewItem.Tag must use inspectable_string"
            {
                let marker = "type = \"Microsoft.UI.Xaml.Controls.NavigationViewItem\"";
                let offset = source.find(marker).unwrap();
                format!(
                    "{}{}",
                    &source[..offset],
                    source[offset..].replacen(from, to, 1)
                )
            } else {
                source.replacen(from, to, 1)
            };
            let error = Schema::parse(&changed)
                .unwrap()
                .resolve(&metadata)
                .err()
                .unwrap();
            assert!(error.contains(expected), "{error}");
        }
    }

    #[test]
    fn rejects_image_uri_on_other_properties() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "image_uri"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("image_uri requires Image.Source or ImageIcon.Source"));
    }

    #[test]
    fn rejects_uri_adapter_on_non_uri_properties() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "uri"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("uri requires a Windows.Foundation.Uri property"));
    }

    #[test]
    fn rejects_string_list_adapter_on_non_object_properties() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "inspectable_string_list"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("inspectable_string_list requires an IInspectable property"));
    }

    #[test]
    fn rejects_semantic_value_adapters_on_unrelated_properties() {
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        for (adapter, expected) in [
            (
                "clock_identifier",
                "clock_identifier requires TimePicker.ClockIdentifier",
            ),
            (
                "number_box_value",
                "number_box_value requires NumberBox.Value",
            ),
            ("rating_value", "rating_value requires RatingControl.Value"),
            (
                "selection_index",
                "selection_index requires an I32 SelectedIndex property",
            ),
        ] {
            let source = format!(
                r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "{adapter}"
"#
            );
            let error = Schema::parse(&source)
                .unwrap()
                .resolve(&metadata)
                .err()
                .unwrap();
            assert!(error.contains(expected), "{error}");
        }
    }

    #[test]
    fn requires_native_string_semantic_adapters() {
        let source = include_str!("winui.toml");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        for (adapter, expected) in [
            (
                "adapter = \"path_data\"",
                "Microsoft.UI.Xaml.Controls.PathIcon.Data must use the path_data adapter",
            ),
            (
                "adapter = \"clock_identifier\"",
                "Microsoft.UI.Xaml.Controls.TimePicker.ClockIdentifier must use the clock_identifier adapter",
            ),
        ] {
            let changed = source.replacen(adapter, "# adapter removed", 1);
            let error = Schema::parse(&changed)
                .unwrap()
                .resolve(&metadata)
                .err()
                .unwrap();
            assert!(error.contains(expected), "{error}");
        }
    }

    #[test]
    fn resolves_event_subscription_lifetimes() {
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let schema = Schema::parse(include_str!("winui.toml"))
            .unwrap()
            .resolve(&metadata)
            .unwrap();
        for (control, event, expected) in [
            ("Button", "Click", false),
            ("TextBox", "TextChanged", true),
            ("ListBox", "SelectionChanged", true),
            ("ContentDialog", "Closed", true),
        ] {
            let control = schema
                .controls
                .iter()
                .find(|candidate| candidate.name == control)
                .unwrap();
            let event = control
                .events
                .iter()
                .find(|candidate| candidate.name == event)
                .unwrap();
            assert_eq!(control.event_always_active(event), expected);
        }
    }

    #[test]
    fn validates_collection_slot_item_controls() {
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let non_collection = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
capabilities = ["layout"]

[[control.slot]]
name = "Header"
item_controls = ["TextBlock"]

[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]
"#;
        let error = Schema::parse(non_collection)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("item_controls requires a collection slot"));

        let source = include_str!("winui.toml").replacen(
            "item_controls = [\"PivotItem\"]",
            "item_controls = [\"Missing\"]",
            1,
        );
        let error = Schema::parse(&source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("names missing item control Missing"));
    }

    #[test]
    fn rejects_invalid_resource_styles() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "resource_style"
variants = [{ name = "Accent", resource = "AccentButtonStyle" }]
"#;
        let schema = Schema::parse(source).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = schema.resolve(&metadata).err().unwrap();
        assert!(error.contains("resource_style requires a Style property and variants"));
    }

    #[test]
    fn rejects_duplicate_and_non_object_slots() {
        let duplicate = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NavigationView"
capabilities = ["layout"]

[[control.slot]]
name = "Content"

[[control.slot]]
name = "Content"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(duplicate)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("duplicate member Content"));

        let non_object = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.Slider"
capabilities = ["layout"]

[[control.slot]]
name = "Value"

[[control.slot]]
name = "IsEnabled"
"#;
        let error = Schema::parse(non_object)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("unsupported slot parameter"));

        let non_collection = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NavigationView"
capabilities = ["layout"]

[[control.slot]]
name = "Content"
collection = true

[[control.slot]]
name = "Header"
"#;
        let error = Schema::parse(non_collection)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(
            error.contains("collection slot must return IVector<IInspectable>"),
            "{error}"
        );

        let ambiguous = duplicate.replace(
            "capabilities = [\"layout\"]",
            "capabilities = [\"layout\", \"children\"]",
        );
        let error = Schema::parse(&ambiguous)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("ambiguous structural declarations"));
    }

    #[test]
    fn rejects_structural_role_not_supported_by_metadata() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.StackPanel"
capabilities = ["layout", "content"]
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("not a metadata content property"));
    }

    #[test]
    fn rejects_grid_definitions_on_non_grid_control() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.StackPanel"
capabilities = ["layout", "children", "grid_definitions"]
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(error.contains("grid_definitions capability requires the Grid children role"));
    }

    #[test]
    fn rejects_window_title_bar_on_other_controls() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout", "window_title_bar"]
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("window_title_bar capability requires the WinUI TitleBar control"));
    }

    #[test]
    fn retains_event_args_payload_source() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
capabilities = ["layout"]

[[control.event]]
name = "ValueChanged"
property = "NewValue"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(source).unwrap().resolve(&metadata).unwrap();
        let event = &resolved.controls[0].events[0];

        assert_eq!(event.payload, "F64");
        assert!(matches!(
            &event.source,
            EventPayloadSource::EventArgsProperty {
                interface,
                property
            } if interface.ends_with("INumberBoxValueChangedEventArgs") && property == "NewValue"
        ));
        assert_eq!(event.conversion, EventPayloadConversion::Identity);
    }

    #[test]
    fn retains_single_field_payload_conversion() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.event]]
name = "Tapped"
property = "FontWeight"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(source).unwrap().resolve(&metadata).unwrap();
        let event = &resolved.controls[0].events[0];

        assert_eq!(event.payload, "U16");
        assert_eq!(
            event.conversion,
            EventPayloadConversion::Field("weight".to_string())
        );
    }

    #[test]
    fn rejects_multi_field_event_payload() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.event]]
name = "SizeChanged"
property = "NewSize"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("unsupported event property NewSize"));
    }

    #[test]
    fn rejects_object_event_payload() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.BreadcrumbBar"
capabilities = ["layout"]

[[control.event]]
name = "ItemClicked"
property = "Item"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("unsupported event property Item"));
    }

    #[test]
    fn rejects_missing_controlled_feedback_event() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
controlled = "Missing"
feedback_contract = "synchronous_exact"
"#;
        let schema = Schema::parse(source).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));

        assert_eq!(
            schema.resolve(&metadata).err().unwrap(),
            "Microsoft.UI.Xaml.Controls.TextBox.Text names missing feedback event Missing"
        );
    }

    #[test]
    fn rejects_feedback_event_shared_by_controlled_properties() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
controlled = "TextChanged"
feedback_contract = "synchronous_exact"

[[control.property]]
name = "SelectedText"
controlled = "TextChanged"
feedback_contract = "synchronous_exact"

[[control.event]]
name = "TextChanged"
property = "Text"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert_eq!(
            error,
            "Microsoft.UI.Xaml.Controls.TextBox assigns feedback event TextChanged to multiple \
             controlled properties"
        );
    }

    #[test]
    fn rejects_missing_controlled_feedback_contract() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
controlled = "TextChanged"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("needs a feedback contract"));
    }

    #[test]
    fn rejects_unknown_feedback_contract() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
controlled = "TextChanged"
feedback_contract = "deferred_coalesced"
"#;
        let error = Schema::parse(source).err().unwrap();

        assert!(error.contains("unknown variant `deferred_coalesced`"));
    }

    #[test]
    fn rejects_exact_coercion_feedback() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
capabilities = ["layout"]

[[control.property]]
name = "Minimum"
coerces = "ValueChanged"
feedback_contract = "synchronous_exact"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("coercion needs synchronous_normalized feedback"));
    }

    #[test]
    fn rejects_exact_observer_for_coercing_event() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
capabilities = ["layout"]

[[control.property]]
name = "Minimum"
coerces = "ValueChanged"
feedback_contract = "synchronous_normalized"

[[control.property]]
name = "Value"
controlled = "ValueChanged"
feedback_contract = "synchronous_exact"

[[control.event]]
name = "ValueChanged"
property = "NewValue"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains(
            "feedback event ValueChanged is coercing and requires synchronous_normalized"
        ));
    }

    #[test]
    fn rejects_observation_of_missing_property() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NavigationView"
capabilities = ["layout"]

[[control.event]]
name = "IsPaneOpenChanged"
observe = "IsPaneOpen"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(
            error.contains("observes missing property IsPaneOpen"),
            "{error}"
        );
    }

    #[test]
    fn rejects_ambiguous_event_payload_source() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NavigationView"
capabilities = ["layout"]

[[control.property]]
name = "IsPaneOpen"

[[control.event]]
name = "IsPaneOpenChanged"
property = "IsPaneOpen"
observe = "IsPaneOpen"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(
            error.contains("cannot set both property and observe"),
            "{error}"
        );
    }

    #[test]
    fn rejects_controlled_feedback_observing_a_different_property() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NavigationView"
capabilities = ["layout"]

[[control.property]]
name = "IsPaneOpen"
controlled = "IsPaneOpenChanged"
feedback_contract = "synchronous_exact"
clear_feedback = false

[[control.property]]
name = "IsSettingsVisible"

[[control.event]]
name = "IsPaneOpenChanged"
observe = "IsSettingsVisible"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(
            error.contains("feedback event IsPaneOpenChanged observes a different property"),
            "{error}"
        );
    }

    #[test]
    fn rejects_controlled_bool_without_clear_feedback() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.ToggleSwitch"
capabilities = ["layout"]

[[control.property]]
name = "IsOn"
controlled = "Toggled"
feedback_contract = "synchronous_exact"

[[control.event]]
name = "Toggled"
property = "IsOn"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();
        assert!(
            error.contains("controlled Bool requires an explicit clear_feedback value"),
            "{error}"
        );
    }

    #[test]
    fn rejects_unknown_capability() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["typo"]
"#;

        assert!(Schema::parse(source).is_err());
    }

    #[test]
    fn rejects_font_weight_adapter_on_other_properties() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.property]]
name = "Text"
adapter = "font_weight"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(
            error.contains("TextBlock.Text font_weight requires TextBlock.FontWeight"),
            "{error}"
        );
    }

    #[test]
    fn resolves_native_placement_contracts() {
        let source = include_str!("winui.toml");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(source).unwrap().resolve(&metadata).unwrap();
        let placement = |name| {
            resolved
                .controls
                .iter()
                .find(|control| control.name == name)
                .unwrap()
                .placement
        };

        assert_eq!(placement("Button"), ResolvedPlacement::Visual);
        assert_eq!(placement("TitleBar"), ResolvedPlacement::WindowLifetime);
        assert_eq!(placement("ToolTip"), ResolvedPlacement::TooltipAttachment);
        assert_eq!(placement("ContentDialog"), ResolvedPlacement::Declaration);
    }

    #[test]
    fn rejects_non_visual_controls_without_a_placement_contract() {
        let source = include_str!("winui.toml").replace("placement = \"tooltip_attachment\"", "");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));

        assert_eq!(
            Schema::parse(&source).unwrap().resolve(&metadata).err(),
            Some(
                "Microsoft.UI.Xaml.Controls.ToolTip has no valid native placement contract"
                    .to_string()
            )
        );
    }
}
