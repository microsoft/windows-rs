use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tool_reactor::helpers::to_snake_case;
use tool_reactor::metadata::MetadataResolver;

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
    pub(crate) enum_variants: Vec<String>,
}

pub(crate) struct ResolvedEvent {
    pub(crate) name: String,
    pub(crate) field: String,
    pub(crate) payload: String,
    pub(crate) interface: String,
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

                properties.push(ResolvedProperty {
                    field: property
                        .field
                        .unwrap_or_else(|| to_snake_case(&property.name)),
                    name: property.name,
                    value,
                    interface: interface.full_path(),
                    clearable: property.clearable,
                    feedback: property.controlled,
                    enum_variants,
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
                let payload = if let Some(property) = event.property.as_deref() {
                    let sender_property = format!("put_{property}");
                    if metadata.has_method(&name, &sender_property) {
                        metadata
                            .infer_value_type(&name, &sender_property)
                            .map(|(value, _)| value)
                    } else {
                        metadata.infer_event_args_type(&name, &method, property)
                    }
                    .ok_or_else(|| {
                        format!(
                            "{}.{} cannot infer event property {}",
                            control.type_name, event.name, property
                        )
                    })?
                } else {
                    "Unit".to_string()
                };

                events.push(ResolvedEvent {
                    field: event
                        .field
                        .unwrap_or_else(|| format!("on_{}", to_snake_case(&event.name))),
                    name: event.name,
                    payload,
                    interface: interface.full_path(),
                });
            }

            for property in properties
                .iter()
                .filter(|property| property.feedback.is_some())
            {
                let feedback = property.feedback.as_deref().unwrap();
                let Some(event) = events.iter().find(|event| event.name == feedback) else {
                    return Err(format!(
                        "{}.{} names missing feedback event {}",
                        control.type_name, property.name, feedback
                    ));
                };
                if event.payload != property.value {
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
        Role::Leaf | Role::Content | Role::Children | Role::Controlled => Ok(()),
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

        assert_eq!(resolved.controls.len(), 4);
        assert_eq!(resolved.controls[0].name, "TextBlock");
        assert_eq!(resolved.controls[0].properties[0].value, "Str");
        assert_eq!(resolved.controls[1].events[0].payload, "Unit");
        assert_eq!(
            resolved.controls[3].properties[0].feedback.as_deref(),
            Some("TextChanged")
        );
        assert_eq!(resolved.controls[3].events[0].payload, "Str");
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
controlled = "Missing"
"#;
        let schema = Schema::parse(source).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));

        assert_eq!(
            schema.resolve(&metadata).err().unwrap(),
            "Microsoft.UI.Xaml.Controls.TextBox.Text names missing feedback event Missing"
        );
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
