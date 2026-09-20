use super::Schema;
use serde::Deserialize;
use std::collections::BTreeMap;
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
    selection: Option<toml::Value>,
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

pub(super) fn compare(old_source: &str, schema: &Schema) -> Result<Report, String> {
    let old: OldSchema = toml::from_str(old_source).map_err(|error| error.to_string())?;
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
            let mapped = match capability.as_str() {
                "content" => object
                    .relations
                    .iter()
                    .any(|relation| relation.name == "Content"),
                "children" => object
                    .relations
                    .iter()
                    .any(|relation| relation.name == "Children"),
                "items" => object
                    .relations
                    .iter()
                    .any(|relation| relation.name == "Items"),
                "enabled" => object
                    .properties
                    .iter()
                    .any(|property| property.name == "IsEnabled"),
                "controlled_text" => object.name == "TextBox" && object.native == "handwritten",
                _ => false,
            };
            if mapped {
                report.capabilities.mapped += 1;
            } else {
                let reason = if capability == "layout"
                    && object.category == "Visual"
                    && [
                        "Width",
                        "Height",
                        "Margin",
                        "HorizontalAlignment",
                        "VerticalAlignment",
                        "Opacity",
                    ]
                    .iter()
                    .all(|name| {
                        schema
                            .visual_properties
                            .iter()
                            .any(|property| property.name == *name)
                    }) {
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
            let mapped_property = object
                .properties
                .iter()
                .any(|candidate| candidate.name == property.name)
                || (object.category == "Visual"
                    && schema
                        .visual_properties
                        .iter()
                        .any(|candidate| candidate.name == property.name));
            if !mapped_property {
                report.unresolved("missing property", contract);
                continue;
            }
            let mut semantics = Vec::new();
            if let Some(value) = &property.adapter {
                semantics.push(format!("adapter={value}"));
            }
            if let Some(value) = &property.controlled {
                semantics.push(format!("controlled={value}"));
            }
            if let Some(value) = &property.coerces {
                semantics.push(format!("coerces={value}"));
            }
            if let Some(value) = &property.feedback_contract {
                semantics.push(format!("feedback={value}"));
            }
            if property.clear_feedback.is_some() {
                semantics.push("clear_feedback".to_string());
            }
            if let Some(value) = &property.validation {
                semantics.push(format!("validation={value}"));
            }
            if property.theme_style {
                semantics.push("theme_style".to_string());
            }
            if !property.variants.is_empty() {
                semantics.push("resource_variants".to_string());
            }
            if property.field.is_some() {
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
            let specialized = event.field.is_some()
                || event.property.is_some()
                || event.observe.is_some()
                || event.adapter.is_some()
                || !event.active_properties.is_empty()
                || event.routed;
            if specialized {
                report.unresolved("unmapped event semantics", contract);
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
                cardinality_matches && slot.item_controls.is_empty()
            });
            if mapped {
                report.slots.mapped += 1;
            } else {
                report.unresolved("missing or incompatible slot relation", contract);
            }
        }

        if control.selection.is_some() {
            report.selections.total += 1;
            report.unresolved("missing selection contract", &control.type_name);
        }
        for (kind, value) in [
            ("placement", control.placement.as_ref()),
            ("lifecycle", control.lifecycle.as_ref()),
            ("content override", control.content.as_ref()),
        ] {
            if let Some(value) = value {
                report.lifecycle.total += 1;
                report.unresolved(format!("unmapped {kind} `{value}`"), &control.type_name);
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
        let old =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let new =
            fs::read_to_string(workspace_path("crates/tools/reactor2/src/schema.toml")).unwrap();
        let schema: Schema = toml::from_str(&new).unwrap();
        let report = compare(&old, &schema).unwrap();
        assert_eq!(report.controls.total, 79);
        assert_eq!(report.properties.total, 233);
        assert_eq!(report.events.total, 68);
        assert_eq!(report.slots.total, 42);
        assert_eq!(report.selections.total, 3);
        assert!(!report.is_complete());
    }
}
