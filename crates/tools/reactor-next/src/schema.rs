use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tool_reactor::helpers::to_snake_case;
use tool_reactor::metadata::{MetadataResolver, ReadValueConversion};

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
    pub(crate) role: Role,
    #[serde(default)]
    pub(crate) capabilities: Vec<Capability>,
    #[serde(default)]
    pub(crate) property: Vec<Property>,
    #[serde(default)]
    pub(crate) event: Vec<Event>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Role {
    Leaf,
    Content,
    Children,
    Controlled,
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
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Property {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) field: Option<String>,
    #[serde(default)]
    pub(crate) clearable: bool,
    #[serde(default)]
    pub(crate) controlled: Option<String>,
    #[serde(default)]
    pub(crate) coerces: Option<String>,
    #[serde(default)]
    pub(crate) feedback_contract: Option<FeedbackContract>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FeedbackContract {
    SynchronousExact,
    SynchronousNormalized,
    DeferredOrdered,
    DeferredCoalesced,
    Unknown,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Event {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) field: Option<String>,
    #[serde(default)]
    pub(crate) property: Option<String>,
}

pub(crate) struct ResolvedSchema {
    pub(crate) controls: Vec<ResolvedControl>,
}

pub(crate) struct ResolvedControl {
    pub(crate) name: String,
    pub(crate) type_name: String,
    pub(crate) role: Role,
    pub(crate) capabilities: Vec<Capability>,
    pub(crate) properties: Vec<ResolvedProperty>,
    pub(crate) events: Vec<ResolvedEvent>,
}

pub(crate) struct ResolvedProperty {
    pub(crate) name: String,
    pub(crate) field: String,
    pub(crate) value: String,
    pub(crate) interface: String,
    pub(crate) clearable: bool,
    pub(crate) feedback: Option<String>,
    pub(crate) feedback_contract: Option<FeedbackContract>,
    pub(crate) observes_feedback: bool,
    pub(crate) enum_variants: Vec<String>,
    pub(crate) native_value: Option<String>,
}

pub(crate) struct ResolvedEvent {
    pub(crate) name: String,
    pub(crate) field: String,
    pub(crate) payload: String,
    pub(crate) interface: String,
    pub(crate) property: Option<String>,
    pub(crate) source: EventPayloadSource,
    pub(crate) conversion: EventPayloadConversion,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum EventPayloadSource {
    Unit,
    SenderProperty { interface: String },
    EventArgsProperty { interface: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EventPayloadConversion {
    Identity,
    Field(String),
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
            validate_role(&control)?;

            let mut properties = Vec::with_capacity(control.property.len());
            let mut events = Vec::with_capacity(control.event.len());
            let mut member_names = HashSet::new();
            let mut field_names = HashSet::new();

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
                    (
                        Some(_),
                        Some(
                            FeedbackContract::SynchronousExact
                            | FeedbackContract::SynchronousNormalized,
                        ),
                    ) => {}
                    (Some(_), Some(_)) => {
                        return Err(format!(
                            "{}.{} uses an unsupported feedback contract",
                            control.type_name, property.name
                        ));
                    }
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
                if !property.clearable {
                    return Err(format!(
                        "{}.{} must be clearable until required properties are supported",
                        control.type_name, property.name
                    ));
                }
                validate_member(
                    &control.type_name,
                    &property.name,
                    property.field.as_deref(),
                    &mut member_names,
                    &mut field_names,
                )?;
                let method = format!("put_{}", property.name);
                let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} is not a metadata property",
                        control.type_name, property.name
                    )
                })?;
                let (value, _) = metadata.infer_value_type(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} has an unsupported metadata type",
                        control.type_name, property.name
                    )
                })?;
                let enum_variants = metadata
                    .enum_info(&name, &method)
                    .map(|(_, variants)| variants.to_vec())
                    .unwrap_or_default();
                let native_value = metadata.enum_path(&name, &method);

                properties.push(ResolvedProperty {
                    field: property
                        .field
                        .unwrap_or_else(|| to_snake_case(&property.name)),
                    name: property.name,
                    value,
                    interface: interface.full_path(),
                    clearable: property.clearable,
                    feedback,
                    feedback_contract: property.feedback_contract,
                    observes_feedback: property.controlled.is_some(),
                    enum_variants,
                    native_value,
                });
            }

            for event in control.event {
                validate_member(
                    &control.type_name,
                    &event.name,
                    event.field.as_deref(),
                    &mut member_names,
                    &mut field_names,
                )?;
                let method = format!("add_{}", event.name);
                let interface = metadata.resolve(&name, &method).ok_or_else(|| {
                    format!(
                        "{}.{} is not a metadata event",
                        control.type_name, event.name
                    )
                })?;
                let (payload, source, conversion) =
                    if let Some(property) = event.property.as_deref() {
                        let sender_property = format!("put_{property}");
                        if metadata.has_method(&name, &sender_property) {
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
                                EventPayloadSource::SenderProperty { interface },
                                conversion,
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
                                EventPayloadSource::EventArgsProperty { interface },
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
                let conversion = match conversion {
                    ReadValueConversion::Identity => EventPayloadConversion::Identity,
                    ReadValueConversion::Field(field) => EventPayloadConversion::Field(field),
                };

                events.push(ResolvedEvent {
                    field: event
                        .field
                        .unwrap_or_else(|| format!("on_{}", to_snake_case(&event.name))),
                    name: event.name,
                    payload,
                    interface: interface.full_path(),
                    property: event.property,
                    source,
                    conversion,
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
            }

            controls.push(ResolvedControl {
                name,
                type_name: control.type_name,
                role: control.role,
                capabilities: control.capabilities,
                properties,
                events,
            });
        }

        Ok(ResolvedSchema { controls })
    }
}

fn validate_role(control: &Control) -> Result<(), String> {
    let has = |capability| control.capabilities.contains(&capability);
    match control.role {
        Role::Content if !has(Capability::Content) => Err(format!(
            "{} content role needs content capability",
            control.type_name
        )),
        Role::Children if !has(Capability::Children) => Err(format!(
            "{} children role needs children capability",
            control.type_name
        )),
        Role::Virtual if !has(Capability::Items) => Err(format!(
            "{} virtual role needs items capability",
            control.type_name
        )),
        Role::Controlled
            if !control
                .property
                .iter()
                .any(|property| property.controlled.is_some()) =>
        {
            Err(format!(
                "{} controlled role needs a controlled property",
                control.type_name
            ))
        }
        Role::Leaf | Role::Content | Role::Children | Role::Controlled | Role::Virtual => Ok(()),
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
    fn resolves_pilot_schema() {
        let schema = Schema::parse(include_str!("winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = schema.resolve(&metadata).unwrap();

        assert_eq!(resolved.controls.len(), 8);
        assert_eq!(resolved.controls[0].name, "TextBlock");
        assert_eq!(resolved.controls[0].properties[0].value, "Str");
        assert_eq!(resolved.controls[1].events[0].payload, "Unit");
        assert_eq!(
            resolved.controls[3].properties[0].feedback.as_deref(),
            Some("TextChanged")
        );
        assert_eq!(resolved.controls[3].events[0].payload, "Str");
        assert!(matches!(
            resolved.controls[3].events[0].source,
            EventPayloadSource::SenderProperty { .. }
        ));
        assert_eq!(resolved.controls[4].name, "NumberBox");
        assert_eq!(
            resolved.controls[4].properties[0]
                .feedback_contract
                .unwrap(),
            FeedbackContract::SynchronousNormalized
        );
        assert!(!resolved.controls[4].properties[0].observes_feedback);
        assert!(resolved.controls[4].properties[2].observes_feedback);
        assert_eq!(resolved.controls[5].name, "Slider");
        assert_eq!(
            resolved.controls[5].properties[0]
                .feedback_contract
                .unwrap(),
            FeedbackContract::SynchronousNormalized
        );
        assert!(!resolved.controls[5].properties[0].observes_feedback);
        assert!(resolved.controls[5].properties[2].observes_feedback);
        assert!(matches!(resolved.controls[6].role, Role::Virtual));
    }

    #[test]
    fn retains_event_args_payload_source() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
role = "leaf"
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
            EventPayloadSource::EventArgsProperty { interface }
                if interface.ends_with("INumberBoxValueChangedEventArgs")
        ));
        assert_eq!(event.conversion, EventPayloadConversion::Identity);
    }

    #[test]
    fn retains_single_field_payload_conversion() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
role = "leaf"
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
role = "leaf"
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
role = "leaf"
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
role = "controlled"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
clearable = true
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
role = "controlled"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
clearable = true
controlled = "TextChanged"
feedback_contract = "synchronous_exact"

[[control.property]]
name = "SelectedText"
clearable = true
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
role = "controlled"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
clearable = true
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
    fn rejects_unsupported_controlled_feedback_contract() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBox"
role = "controlled"
capabilities = ["controlled_text"]

[[control.property]]
name = "Text"
clearable = true
controlled = "TextChanged"
feedback_contract = "deferred_coalesced"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("unsupported feedback contract"));
    }

    #[test]
    fn rejects_exact_coercion_feedback() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
role = "leaf"
capabilities = ["layout"]

[[control.property]]
name = "Minimum"
clearable = true
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
role = "controlled"
capabilities = ["layout"]

[[control.property]]
name = "Minimum"
clearable = true
coerces = "ValueChanged"
feedback_contract = "synchronous_normalized"

[[control.property]]
name = "Value"
clearable = true
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
    fn rejects_non_clearable_property_without_required_value_contract() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.ProgressBar"
role = "leaf"
capabilities = ["layout"]

[[control.property]]
name = "Value"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let error = Schema::parse(source)
            .unwrap()
            .resolve(&metadata)
            .err()
            .unwrap();

        assert!(error.contains("must be clearable"));
    }

    #[test]
    fn rejects_unknown_capability() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
role = "leaf"
capabilities = ["typo"]
"#;

        assert!(Schema::parse(source).is_err());
    }
}
