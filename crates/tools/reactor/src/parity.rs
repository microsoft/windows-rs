use super::Schema;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldSchema {
    control: Vec<OldControl>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldControl {
    #[serde(rename = "type")]
    type_name: String,
    #[serde(default)]
    placement: Option<String>,
    #[serde(default)]
    lifecycle: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default)]
    property: Vec<OldProperty>,
    #[serde(default)]
    event: Vec<OldEvent>,
    #[serde(default)]
    slot: Vec<OldSlot>,
    selection: Option<OldSelection>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldProperty {
    name: String,
    #[serde(default)]
    field: Option<String>,
    #[serde(default)]
    theme_style: bool,
    #[serde(default)]
    controlled: Option<String>,
    #[serde(default)]
    coerces: Option<String>,
    #[serde(default)]
    feedback_contract: Option<String>,
    #[serde(default)]
    clear_feedback: Option<bool>,
    #[serde(default)]
    adapter: Option<String>,
    #[serde(default)]
    validation: Option<String>,
    #[serde(default)]
    variants: Vec<toml::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldEvent {
    name: String,
    #[serde(default)]
    field: Option<String>,
    #[serde(default)]
    property: Option<String>,
    #[serde(default)]
    observe: Option<String>,
    #[serde(default)]
    adapter: Option<String>,
    #[serde(default)]
    active_properties: Vec<String>,
    #[serde(default)]
    routed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldSlot {
    name: String,
    #[serde(default)]
    collection: bool,
    #[serde(default)]
    item_controls: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OldSelection {
    slots: Vec<String>,
    item: String,
    selected_property: String,
    selected_item_property: String,
    event: String,
    #[serde(default)]
    event_args: Option<String>,
    payload_property: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ParityMappings {
    capability: Vec<CapabilityMapping>,
    #[serde(default)]
    property: Vec<PropertyMapping>,
    #[serde(default)]
    event_adapter: Vec<EventAdapterMapping>,
    lifecycle: Vec<LifecycleMapping>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityMapping {
    name: String,
    kind: String,
    relation: Option<String>,
    schema_capability: Option<String>,
    #[serde(default)]
    property_adapters: Vec<String>,
    rationale: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PropertyMapping {
    control: String,
    name: String,
    #[serde(default)]
    old_adapter: String,
    #[serde(default)]
    new_adapter: String,
    #[serde(default)]
    old_feedback: String,
    #[serde(default)]
    new_feedback: String,
    rationale: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EventAdapterMapping {
    #[serde(default)]
    old_adapter: String,
    value: Option<String>,
    payload_adapter: Option<String>,
    property_adapter: Option<String>,
    schema_capability: Option<String>,
    #[serde(default)]
    required_routed: bool,
    #[serde(default)]
    payload_property_equivalent: bool,
    rationale: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LifecycleMapping {
    kind: String,
    old: String,
    schema_capability: String,
    rationale: String,
}

#[derive(Default)]
struct ContractCount {
    total: usize,
    mapped: usize,
}

#[derive(Default)]
pub(super) struct Report {
    controls: ContractCount,
    properties: ContractCount,
    events: ContractCount,
    slots: ContractCount,
    selections: ContractCount,
    capabilities: ContractCount,
    lifecycle: ContractCount,
    unresolved: BTreeMap<String, Vec<String>>,
}

impl Report {
    pub(super) fn is_complete(&self) -> bool {
        self.unresolved.is_empty()
    }

    pub(super) fn render(&self) -> String {
        let mut output = String::new();
        writeln!(
            output,
            "contract       mapped   total   missing\n\
             controls       {:>6}  {:>6}  {:>8}\n\
             properties     {:>6}  {:>6}  {:>8}\n\
             events         {:>6}  {:>6}  {:>8}\n\
             slots          {:>6}  {:>6}  {:>8}\n\
             selections     {:>6}  {:>6}  {:>8}\n\
             capabilities   {:>6}  {:>6}  {:>8}\n\
             lifecycle      {:>6}  {:>6}  {:>8}",
            self.controls.mapped,
            self.controls.total,
            self.controls.total - self.controls.mapped,
            self.properties.mapped,
            self.properties.total,
            self.properties.total - self.properties.mapped,
            self.events.mapped,
            self.events.total,
            self.events.total - self.events.mapped,
            self.slots.mapped,
            self.slots.total,
            self.slots.total - self.slots.mapped,
            self.selections.mapped,
            self.selections.total,
            self.selections.total - self.selections.mapped,
            self.capabilities.mapped,
            self.capabilities.total,
            self.capabilities.total - self.capabilities.mapped,
            self.lifecycle.mapped,
            self.lifecycle.total,
            self.lifecycle.total - self.lifecycle.mapped,
        )
        .unwrap();
        for (reason, contracts) in &self.unresolved {
            writeln!(output, "\n{reason} ({})", contracts.len()).unwrap();
            for contract in contracts {
                writeln!(output, "  {contract}").unwrap();
            }
        }
        output
    }

    fn unresolved(&mut self, reason: impl Into<String>, contract: impl Into<String>) {
        self.unresolved
            .entry(reason.into())
            .or_default()
            .push(contract.into());
    }
}

impl ParityMappings {
    fn parse(source: &str) -> Result<Self, String> {
        let mappings: Self = toml::from_str(source).map_err(|error| error.to_string())?;
        let mut keys = BTreeSet::new();
        for mapping in &mappings.capability {
            if mapping.rationale.trim().is_empty() {
                return Err(format!(
                    "capability mapping `{}` requires a rationale",
                    mapping.name
                ));
            }
            if !keys.insert(("capability", mapping.name.as_str(), "")) {
                return Err(format!("duplicate capability mapping `{}`", mapping.name));
            }
            if !matches!(
                mapping.kind.as_str(),
                "relation"
                    | "virtual_items"
                    | "layout"
                    | "schema_capability"
                    | "controlled_text"
                    | "property_adapters"
            ) {
                return Err(format!(
                    "unknown capability mapping kind `{}`",
                    mapping.kind
                ));
            }
        }
        for mapping in &mappings.property {
            if mapping.rationale.trim().is_empty() {
                return Err(format!(
                    "property mapping `{}.{}` requires a rationale",
                    mapping.control, mapping.name
                ));
            }
            if !keys.insert(("property", mapping.control.as_str(), mapping.name.as_str())) {
                return Err(format!(
                    "duplicate property mapping `{}.{}`",
                    mapping.control, mapping.name
                ));
            }
        }
        for mapping in &mappings.event_adapter {
            if mapping.rationale.trim().is_empty() {
                return Err(format!(
                    "event adapter mapping `{}` requires a rationale",
                    mapping.old_adapter
                ));
            }
        }
        for mapping in &mappings.lifecycle {
            if mapping.rationale.trim().is_empty() {
                return Err(format!(
                    "lifecycle mapping `{}.{}` requires a rationale",
                    mapping.kind, mapping.old
                ));
            }
            if !keys.insert(("lifecycle", mapping.kind.as_str(), mapping.old.as_str())) {
                return Err(format!(
                    "duplicate lifecycle mapping `{}.{}`",
                    mapping.kind, mapping.old
                ));
            }
        }
        Ok(mappings)
    }

    fn capability<'a>(&'a self, name: &str) -> Option<&'a CapabilityMapping> {
        self.capability.iter().find(|mapping| mapping.name == name)
    }

    fn property<'a>(&'a self, control: &str, name: &str) -> Option<&'a PropertyMapping> {
        self.property
            .iter()
            .find(|mapping| mapping.control == control && mapping.name == name)
    }

    fn lifecycle<'a>(&'a self, kind: &str, old: &str) -> Option<&'a LifecycleMapping> {
        self.lifecycle
            .iter()
            .find(|mapping| mapping.kind == kind && mapping.old == old)
    }
}

fn schema_capability_contains(schema: &Schema, capability: &str, object: &str) -> bool {
    let objects = match capability {
        "enabled" => &schema.capabilities.enabled,
        "focus" => &schema.capabilities.focus,
        "reference" => &schema.capabilities.reference,
        "text_style" => &schema.capabilities.text_style,
        "tooltip_attachment" => &schema.capabilities.tooltip_attachment,
        "content_dialog_attachment" => &schema.capabilities.content_dialog_attachment,
        "window_title_bar" => &schema.capabilities.window_title_bar,
        _ => return false,
    };
    objects.iter().any(|name| name == object)
}

fn capability_matches(
    mapping: &CapabilityMapping,
    control: &OldControl,
    object: &super::Object,
    schema: &Schema,
) -> bool {
    match mapping.kind.as_str() {
        "relation" => object.relations.iter().any(|relation| {
            mapping
                .relation
                .as_ref()
                .is_some_and(|name| relation.name == *name)
        }),
        "virtual_items" => {
            object.virtual_items
                && object.relations.iter().any(|relation| {
                    relation.name == "Items"
                        && relation.child == "Visual"
                        && relation.cardinality == "Many"
                        && relation.identity == "Keyed"
                        && relation.realization == "Container"
                })
        }
        "layout" => {
            let visual = [
                "Width",
                "Height",
                "MinWidth",
                "MaxWidth",
                "MinHeight",
                "MaxHeight",
                "Opacity",
                "HorizontalAlignment",
                "VerticalAlignment",
                "Margin",
                "Transitions",
            ];
            let attached = [
                "GridRow",
                "GridColumn",
                "GridRowSpan",
                "GridColumnSpan",
                "RelativeAlignLeft",
                "RelativeAlignTop",
                "RelativeAlignRight",
                "RelativeAlignBottom",
                "RelativeAlignHorizontalCenter",
                "RelativeAlignVerticalCenter",
                "CanvasLeft",
                "CanvasTop",
                "AutomationName",
                "AutomationId",
                "AutomationHeadingLevel",
            ];
            schema.capabilities.layout_exit_transition
                && visual.iter().all(|name| {
                    schema
                        .visual_properties
                        .iter()
                        .any(|property| property.name == *name)
                })
                && attached.iter().all(|name| {
                    schema.attached_properties.iter().any(|property| {
                        property.name == *name
                            && match *name {
                                "GridRow" | "GridColumn" => {
                                    property.validation.as_deref() == Some("non_negative")
                                }
                                "GridRowSpan" | "GridColumnSpan" => {
                                    property.validation.as_deref() == Some("positive")
                                }
                                _ => true,
                            }
                    })
                })
        }
        "schema_capability" => mapping
            .schema_capability
            .as_deref()
            .is_some_and(|capability| schema_capability_contains(schema, capability, &object.name)),
        "controlled_text" => control.property.iter().any(|old| {
            old.controlled.as_deref() == Some("TextChanged")
                && object.properties.iter().any(|property| {
                    property.name == old.name
                        && property.value == "String"
                        && property.controlled == old.controlled
                        && property.feedback == old.feedback_contract
                        && property.adapter == old.adapter
                })
        }),
        "property_adapters" => mapping.property_adapters.iter().all(|adapter| {
            object.properties.iter().any(|property| {
                property.value == "GridLengths"
                    && property.adapter.as_deref() == Some(adapter.as_str())
            })
        }),
        _ => false,
    }
}

fn event_adapter_matches(
    mappings: &ParityMappings,
    event: &OldEvent,
    candidate: &super::Event,
    object: &super::Object,
    schema: &Schema,
) -> bool {
    let old_adapter = event.adapter.as_deref().unwrap_or_default();
    if old_adapter.is_empty() && candidate.payload_adapter.is_none() {
        return true;
    }
    mappings.event_adapter.iter().any(|mapping| {
        mapping.old_adapter == old_adapter
            && mapping
                .value
                .as_ref()
                .is_none_or(|value| candidate.value == *value)
            && mapping
                .payload_adapter
                .as_ref()
                .is_none_or(|adapter| candidate.payload_adapter.as_ref() == Some(adapter))
            && mapping.property_adapter.as_ref().is_none_or(|adapter| {
                candidate.observes.as_deref().is_some_and(|observed| {
                    object.properties.iter().any(|property| {
                        property.name == observed && property.adapter.as_ref() == Some(adapter)
                    })
                })
            })
            && mapping
                .schema_capability
                .as_deref()
                .is_none_or(|capability| {
                    schema_capability_contains(schema, capability, &object.name)
                })
            && (!mapping.required_routed || candidate.routed)
    })
}

pub(super) fn compare(
    old_source: &str,
    schema: &Schema,
    mappings_source: &str,
) -> Result<Report, String> {
    let old: OldSchema = toml::from_str(old_source).map_err(|error| error.to_string())?;
    let mappings = ParityMappings::parse(mappings_source)?;
    let mut report = Report::default();

    for control in old.control {
        report.controls.total += 1;
        let object = schema.objects.iter().find(|object| {
            object.native == control.type_name
                || (object.native == "handwritten"
                    && object.native_type.as_deref() == Some(control.type_name.as_str()))
        });
        let Some(object) = object else {
            report.unresolved("missing object", &control.type_name);
            report.properties.total += control.property.len();
            report.events.total += control.event.len();
            report.slots.total += control.slot.len();
            report.selections.total += usize::from(control.selection.is_some());
            report.capabilities.total += control.capabilities.len();
            report.lifecycle.total += usize::from(control.placement.is_some())
                + usize::from(control.lifecycle.is_some())
                + usize::from(control.content.is_some());
            for capability in &control.capabilities {
                report.unresolved(
                    "capability blocked by missing object",
                    format!("{}.{}", control.type_name, capability),
                );
            }

            for property in &control.property {
                report.unresolved(
                    "property blocked by missing object",
                    format!("{}.{}", control.type_name, property.name),
                );
            }

            for event in &control.event {
                report.unresolved(
                    "event blocked by missing object",
                    format!("{}.{}", control.type_name, event.name),
                );
            }
            for slot in &control.slot {
                report.unresolved(
                    "slot blocked by missing object",
                    format!("{}.{}", control.type_name, slot.name),
                );
            }
            if control.selection.is_some() {
                report.unresolved("selection blocked by missing object", &control.type_name);
            }
            for (kind, value) in [
                ("placement", control.placement.as_ref()),
                ("lifecycle", control.lifecycle.as_ref()),
                ("content override", control.content.as_ref()),
            ] {
                if let Some(value) = value {
                    report.unresolved(
                        format!("{kind} `{value}` blocked by missing object"),
                        &control.type_name,
                    );
                }
            }
            continue;
        };
        report.controls.mapped += 1;

        for capability in &control.capabilities {
            report.capabilities.total += 1;
            let mapped = mappings
                .capability(capability)
                .is_some_and(|mapping| capability_matches(mapping, &control, object, schema));
            if mapped {
                report.capabilities.mapped += 1;
            } else {
                let reason = if capability == "layout" && object.category == "Visual" {
                    "partial capability `layout`".to_string()
                } else {
                    format!("unmapped capability `{capability}`")
                };
                report.unresolved(reason, &control.type_name);
            }
        }

        for property in &control.property {
            report.properties.total += 1;
            let contract = format!("{}.{}", control.type_name, property.name);
            let mapped_property = object.properties.iter().any(|candidate| {
                candidate.name == property.name
                    || candidate.native.as_deref() == Some(property.name.as_str())
            }) || (object.category == "Visual"
                && schema
                    .visual_properties
                    .iter()
                    .any(|candidate| candidate.name == property.name))
                || (property.name == "IsEnabled"
                    && schema
                        .capabilities
                        .enabled
                        .iter()
                        .any(|name| name == &object.name));
            if !mapped_property {
                report.unresolved("missing property", contract);
                continue;
            }
            let mapped_property = object.properties.iter().find(|candidate| {
                candidate.name == property.name
                    || candidate.native.as_deref() == Some(property.name.as_str())
            });
            let mut semantics = Vec::new();
            let property_mapping = mappings.property(&control.type_name, &property.name);
            let adapter_matches = property.adapter.as_ref()
                == mapped_property.and_then(|candidate| candidate.adapter.as_ref())
                || property_mapping.is_some_and(|mapping| {
                    property.adapter.as_deref().unwrap_or_default() == mapping.old_adapter
                        && mapped_property
                            .and_then(|candidate| candidate.adapter.as_deref())
                            .unwrap_or_default()
                            == mapping.new_adapter
                });
            if !adapter_matches && let Some(value) = &property.adapter {
                semantics.push(format!("adapter={value}"));
            }
            if property.controlled.as_ref()
                != mapped_property.and_then(|candidate| candidate.controlled.as_ref())
                && let Some(value) = &property.controlled
            {
                semantics.push(format!("controlled={value}"));
            }
            if property.coerces.as_ref()
                != mapped_property.and_then(|candidate| candidate.coerces.as_ref())
                && let Some(value) = &property.coerces
            {
                semantics.push(format!("coerces={value}"));
            }
            let feedback_matches = property.feedback_contract.as_ref()
                == mapped_property.and_then(|candidate| candidate.feedback.as_ref())
                || property_mapping.is_some_and(|mapping| {
                    property.feedback_contract.as_deref().unwrap_or_default()
                        == mapping.old_feedback
                        && mapped_property
                            .and_then(|candidate| candidate.feedback.as_deref())
                            .unwrap_or_default()
                            == mapping.new_feedback
                });
            if !feedback_matches && let Some(value) = &property.feedback_contract {
                semantics.push(format!("feedback={value}"));
            }
            if property.feedback_contract.is_some()
                && mapped_property.is_some_and(|candidate| {
                    let feedback_event =
                        candidate.controlled.as_ref().or(candidate.coerces.as_ref());
                    !matches!(
                        candidate.feedback.as_deref(),
                        Some("synchronous_exact" | "synchronous_normalized" | "deferred_exact")
                    ) || !feedback_event.is_some_and(|event| {
                        object.events.iter().any(|candidate_event| {
                            candidate_event.name == *event
                                && candidate_event.observes.as_deref().is_some_and(|observed| {
                                    candidate.coerces.is_some() || observed == candidate.name
                                })
                        })
                    })
                })
            {
                semantics.push("feedback_runtime".to_string());
            }
            if property.clear_feedback.unwrap_or(false)
                != mapped_property.is_some_and(|candidate| candidate.clear_feedback)
            {
                semantics.push("clear_feedback".to_string());
            }
            if property.validation.as_ref()
                != mapped_property.and_then(|candidate| candidate.validation.as_ref())
                && let Some(value) = &property.validation
            {
                semantics.push(format!("validation={value}"));
            }
            if property.theme_style
                && !mapped_property.is_some_and(|candidate| {
                    candidate.adapter.as_deref() == Some("theme_brush")
                        && candidate.value == "Brush"
                })
            {
                semantics.push("theme_style".to_string());
            }
            if !property.variants.is_empty()
                && !mapped_property.is_some_and(|candidate| {
                    candidate.adapter.as_deref() == Some("resource_style")
                        && candidate.value == "ButtonStyle"
                })
            {
                semantics.push("resource_variants".to_string());
            }
            if property.field.is_some()
                && !mapped_property.is_some_and(|candidate| {
                    candidate.adapter.as_ref() == property.adapter.as_ref()
                })
            {
                semantics.push("renamed_field".to_string());
            }
            if semantics.is_empty() {
                report.properties.mapped += 1;
            } else {
                report.unresolved(
                    format!("unmapped property semantics `{}`", semantics.join(", ")),
                    contract,
                );
            }
        }

        for event in &control.event {
            report.events.total += 1;
            let contract = format!("{}.{}", control.type_name, event.name);
            if !object
                .events
                .iter()
                .any(|candidate| candidate.name == event.name)
            {
                report.unresolved("missing event", contract);
                continue;
            }
            let candidate = object
                .events
                .iter()
                .find(|candidate| candidate.name == event.name)
                .unwrap();
            let expected_observed = event.observe.as_ref().or_else(|| {
                control
                    .property
                    .iter()
                    .find(|property| property.controlled.as_deref() == Some(event.name.as_str()))
                    .map(|property| &property.name)
            });
            let observed_matches = expected_observed.is_none_or(|observed| {
                candidate.observes.as_deref().is_some_and(|candidate| {
                    candidate == observed
                        || object.properties.iter().any(|property| {
                            property.name == candidate
                                && property.native.as_deref() == Some(observed.as_str())
                        })
                })
            });
            let adapter_matches =
                event_adapter_matches(&mappings, event, candidate, object, schema);
            if event.field.as_ref() != candidate.field.as_ref()
                || !adapter_matches
                || event.active_properties != candidate.active_properties
                || event.routed != candidate.routed
                || !observed_matches
            {
                report.unresolved("unmapped event semantics", contract);
            } else if let Some(property) = &event.property {
                if mappings.event_adapter.iter().any(|mapping| {
                    mapping.payload_property_equivalent
                        && mapping.old_adapter == event.adapter.as_deref().unwrap_or_default()
                        && mapping
                            .value
                            .as_ref()
                            .is_none_or(|value| candidate.value == *value)
                }) || candidate.payload.as_deref() == Some(property.as_str())
                    || candidate.observes.as_deref().is_some_and(|observed| {
                        object.properties.iter().any(|candidate_property| {
                            candidate_property.name == *observed
                                && (candidate_property.name == *property
                                    || candidate_property.native.as_deref()
                                        == Some(property.as_str()))
                                && candidate.value == candidate_property.value
                        })
                    })
                {
                    report.events.mapped += 1;
                } else {
                    report.unresolved(
                        format!("unmapped event payload property `{property}`"),
                        contract,
                    );
                }
            } else {
                report.events.mapped += 1;
            }
        }

        for slot in &control.slot {
            report.slots.total += 1;
            let contract = format!("{}.{}", control.type_name, slot.name);
            let relation = object
                .relations
                .iter()
                .find(|relation| relation.name == slot.name);
            let mapped = relation.is_some_and(|relation| {
                let cardinality_matches = slot.collection == (relation.cardinality == "Many");
                cardinality_matches && relation.allowed_objects == slot.item_controls
            });
            if mapped {
                report.slots.mapped += 1;
            } else {
                report.unresolved("missing or incompatible slot relation", contract);
            }
        }

        if let Some(selection) = &control.selection {
            report.selections.total += 1;
            let mapped = object.selection.as_ref().is_some_and(|candidate| {
                candidate.relations == selection.slots
                    && candidate.item == selection.item
                    && candidate.selected_property == selection.selected_property
                    && candidate.selected_item_property == selection.selected_item_property
                    && candidate.event == selection.event
                    && candidate.event_item_source
                        == if selection.event_args.is_some() {
                            "EventArgs"
                        } else {
                            "Owner"
                        }
                    && candidate.event_args == selection.event_args
                    && candidate.payload_property == selection.payload_property
                    && candidate.relations.iter().all(|relation| {
                        object.relations.iter().any(|candidate_relation| {
                            candidate_relation.name == *relation
                                && candidate_relation.cardinality == "Many"
                                && candidate_relation.identity == "Keyed"
                                && candidate_relation.realization == "Owned"
                                && (candidate_relation.allowed_objects.is_empty()
                                    || candidate_relation.allowed_objects.contains(&selection.item))
                        })
                    })
                    && object
                        .events
                        .iter()
                        .any(|event| event.name == selection.event && event.value == "Selection")
            });
            if mapped {
                report.selections.mapped += 1;
            } else {
                report.unresolved(
                    "missing or incompatible selection contract",
                    &control.type_name,
                );
            }
        }
        for (kind, value) in [
            ("placement", control.placement.as_ref()),
            ("lifecycle", control.lifecycle.as_ref()),
            ("content override", control.content.as_ref()),
        ] {
            if let Some(value) = value {
                report.lifecycle.total += 1;
                let mapped = mappings.lifecycle(kind, value).is_some_and(|mapping| {
                    schema_capability_contains(schema, &mapping.schema_capability, &object.name)
                });
                if mapped {
                    report.lifecycle.mapped += 1;
                } else {
                    report.unresolved(format!("unmapped {kind} `{value}`"), &control.type_name);
                }
            }
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_path;
    use std::fs;

    #[test]
    fn inventories_complete_old_surface() {
        let old = fs::read_to_string(workspace_path(
            "crates/tools/reactor/src/parity-baseline.toml",
        ))
        .unwrap();
        let new =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/schema.toml")).unwrap();
        let mappings =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/parity.toml")).unwrap();
        let schema: Schema = toml::from_str(&new).unwrap();
        let report = compare(&old, &schema, &mappings).unwrap();
        assert_eq!(report.controls.total, 79);
        assert_eq!(report.properties.total, 233);
        assert_eq!(report.events.total, 68);
        assert_eq!(report.slots.total, 42);
        assert_eq!(report.selections.total, 3);
        assert_eq!(report.properties.mapped, 233);
        assert_eq!(report.events.mapped, 68);
        assert_eq!(report.slots.mapped, 42);
        assert_eq!(report.selections.mapped, 3);
        assert_eq!(report.lifecycle.mapped, 2);
        assert!(report.is_complete());
    }

    #[test]
    fn navigation_selection_requires_exact_event_args_source() {
        let old = fs::read_to_string(workspace_path(
            "crates/tools/reactor/src/parity-baseline.toml",
        ))
        .unwrap();
        let new =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/schema.toml")).unwrap();
        let mappings =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/parity.toml")).unwrap();
        let mut schema: Schema = toml::from_str(&new).unwrap();
        let navigation = schema
            .objects
            .iter_mut()
            .find(|object| object.name == "NavigationView")
            .unwrap();
        navigation.selection.as_mut().unwrap().event_item_source = "Owner".to_string();
        navigation.selection.as_mut().unwrap().event_args = None;

        let report = compare(&old, &schema, &mappings).unwrap();
        assert_eq!(report.selections.mapped, 2);
    }
}
