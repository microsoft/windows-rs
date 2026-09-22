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
                "items" => {
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
                "enabled" => schema
                    .capabilities
                    .enabled
                    .iter()
                    .any(|name| name == &object.name),
                "focus" => schema
                    .capabilities
                    .focus
                    .iter()
                    .any(|name| name == &object.name),
                "reference" => schema
                    .capabilities
                    .reference
                    .iter()
                    .any(|name| name == &object.name),
                "text_style" => schema
                    .capabilities
                    .text_style
                    .iter()
                    .any(|name| name == &object.name),
                "controlled_text" => object.properties.iter().any(|property| {
                    property.value == "String"
                        && property.controlled.as_deref() == Some("TextChanged")
                        && property.feedback.as_deref()
                            == Some(if object.name == "RichEditBox" {
                                "deferred_exact"
                            } else {
                                "synchronous_exact"
                            })
                        && (object.name != "RichEditBox"
                            || property.adapter.as_deref() == Some("rich_edit_text"))
                }),
                "grid_definitions" => ["grid_rows", "grid_columns"].iter().all(|adapter| {
                    object.properties.iter().any(|property| {
                        property.value == "GridLengths"
                            && property.adapter.as_deref() == Some(adapter)
                    })
                }),
                _ => false,
            };
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
            if property.adapter.as_ref()
                != mapped_property.and_then(|candidate| candidate.adapter.as_ref())
                && let Some(value) = &property.adapter
            {
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
            if property.feedback_contract.as_ref()
                != mapped_property.and_then(|candidate| candidate.feedback.as_ref())
                && let Some(value) = &property.feedback_contract
            {
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
            if property.theme_style {
                semantics.push("theme_style".to_string());
            }
            if !property.variants.is_empty() {
                semantics.push("resource_variants".to_string());
            }
            if property.field.is_some()
                && !mapped_property.is_some_and(|candidate| {
                    candidate.adapter.as_deref() == Some("rich_edit_text")
                        && property.field.as_deref() == Some("text")
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
            let adapter_matches = event.adapter.is_none()
                || event.adapter.as_deref() == Some("rich_edit_text")
                    && candidate.observes.as_deref().is_some_and(|observed| {
                        object.properties.iter().any(|property| {
                            property.name == *observed
                                && property.adapter.as_deref() == Some("rich_edit_text")
                        })
                    });
            if event.field.as_ref() != candidate.field.as_ref()
                || !adapter_matches
                || !event.active_properties.is_empty()
                || event.routed
                || !observed_matches
            {
                report.unresolved("unmapped event semantics", contract);
            } else if let Some(property) = &event.property {
                if candidate.payload.as_deref() == Some(property.as_str())
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
                report.unresolved(format!("unmapped {kind} `{value}`"), &control.type_name);
            }
        }
    }

    Ok(report)
}

pub(super) fn convert(
    old_source: &str,
    schema: &Schema,
    metadata: &tool_reactor::metadata::MetadataResolver,
) -> Result<String, String> {
    let old: OldSchema = toml::from_str(old_source).map_err(|error| error.to_string())?;
    let mut converted = schema.clone();
    let mut property_types = BTreeMap::new();
    for object in &mut converted.objects {
        for property in &mut object.properties {
            if property_types
                .get(&property.name)
                .is_some_and(|value| *value != property.value)
            {
                property.native = Some(property.name.clone());
                property.name = format!("{}{}", object.name, property.name);
            } else {
                property_types.insert(property.name.clone(), property.value.clone());
            }
        }
    }

    for control in old.control {
        let object_index = if let Some(index) = converted.objects.iter().position(|object| {
            object.native == control.type_name
                || object.native_type.as_deref() == Some(control.type_name.as_str())
        }) {
            index
        } else {
            converted.objects.push(super::Object {
                name: control.type_name.rsplit('.').next().unwrap().to_string(),
                category: "Visual".to_string(),
                native: control.type_name.clone(),
                native_type: None,
                key: false,
                virtual_items: false,
                properties: Vec::new(),
                relations: Vec::new(),
                events: Vec::new(),
                selection: None,
            });
            converted.objects.len() - 1
        };
        let handwritten = converted.objects[object_index].native == "handwritten";

        let name = control.type_name.rsplit('.').next().unwrap();
        for property in &control.property {
            let feedback_supported = property
                .controlled
                .as_ref()
                .or(property.coerces.as_ref())
                .is_none_or(|event| {
                    metadata.resolve(name, &format!("add_{event}")).is_some()
                        || (handwritten
                            && converted.objects[object_index]
                                .events
                                .iter()
                                .any(|candidate| candidate.name == *event))
                });
            if let Some(existing) =
                converted.objects[object_index]
                    .properties
                    .iter_mut()
                    .find(|candidate| {
                        candidate.name == property.name
                            || candidate.native.as_deref() == Some(property.name.as_str())
                    })
            {
                if supported_property_adapter(property.adapter.as_deref())
                    && !property.theme_style
                    && property.variants.is_empty()
                    && property.field.is_none()
                {
                    existing.validation.clone_from(&property.validation);
                    existing.adapter.clone_from(&property.adapter);
                    if feedback_supported {
                        existing.controlled.clone_from(&property.controlled);
                        existing.coerces.clone_from(&property.coerces);
                        existing.feedback.clone_from(&property.feedback_contract);
                        existing.clear_feedback = property.clear_feedback.unwrap_or(false);
                    } else {
                        existing.controlled = None;
                        existing.coerces = None;
                        existing.feedback = None;
                        existing.clear_feedback = false;
                    }
                }
                continue;
            }
            if handwritten
                || !supported_property_adapter(property.adapter.as_deref())
                || !feedback_supported
                || property.theme_style
                || !property.variants.is_empty()
                || property.field.is_some()
            {
                continue;
            }
            let method = format!("put_{}", property.name);
            let Some(value) =
                converted_property_value(property.adapter.as_deref(), metadata, name, &method)
            else {
                continue;
            };
            let conflict = converted
                .objects
                .iter()
                .flat_map(|object| &object.properties)
                .any(|candidate| candidate.name == property.name && candidate.value != value);
            let generated_name = if conflict {
                format!("{}{}", converted.objects[object_index].name, property.name)
            } else {
                property.name.clone()
            };
            converted.objects[object_index]
                .properties
                .push(super::Property {
                    name: generated_name,
                    native: conflict.then(|| property.name.clone()),
                    value,
                    adapter: property.adapter.clone(),
                    controlled: property.controlled.clone(),
                    coerces: property.coerces.clone(),
                    feedback: property.feedback_contract.clone(),
                    clear_feedback: property.clear_feedback.unwrap_or(false),
                    required: false,
                    default: None,
                    validation: property.validation.clone(),
                    readback: false,
                });
        }

        if control.capabilities.iter().any(|value| value == "content")
            && metadata.content_property(&control.type_name).is_some()
            && !converted.objects[object_index]
                .relations
                .iter()
                .any(|relation| relation.name == "Content")
        {
            let native = metadata.content_property(&control.type_name).unwrap();
            converted.objects[object_index].relations.push(relation(
                "Content",
                Some(native),
                false,
            ));
        }
        if control.capabilities.iter().any(|value| value == "children")
            && !converted.objects[object_index]
                .relations
                .iter()
                .any(|relation| relation.name == "Children")
        {
            converted.objects[object_index]
                .relations
                .push(relation("Children", None, true));
        }
        for slot in &control.slot {
            if !slot.collection {
                let native_item = metadata.param_class_name(name, &format!("put_{}", slot.name));
                let supported = metadata
                    .parameter_type_name(name, &format!("put_{}", slot.name))
                    .is_some_and(|value| matches!(value, "IInspectable" | "Object" | "UIElement"))
                    || native_item.as_deref() == Some("Microsoft.UI.Xaml.Controls.IconElement");
                if supported {
                    if let Some(existing) = converted.objects[object_index]
                        .relations
                        .iter_mut()
                        .find(|relation| relation.name == slot.name)
                    {
                        existing.item = None;
                        existing.native_item =
                            native_item.filter(|item| item.ends_with(".IconElement"));
                    } else {
                        let mut value = relation(&slot.name, None, false);
                        value.native_item =
                            native_item.filter(|item| item.ends_with(".IconElement"));
                        converted.objects[object_index].relations.push(value);
                    }
                }
            } else if let Some(collection) =
                metadata.classify_collection(name, &format!("get_{}", slot.name))
            {
                let (method, item) = match collection {
                    tool_reactor::metadata::CollectionType::InspectableVector => {
                        ("Vector", "IInspectable".to_string())
                    }
                    tool_reactor::metadata::CollectionType::ItemCollection => {
                        ("ItemCollection", "IInspectable".to_string())
                    }
                    tool_reactor::metadata::CollectionType::TypedVector(item) => ("Vector", item),
                    tool_reactor::metadata::CollectionType::ObservableVector(item) => {
                        ("ObservableVector", item)
                    }
                };
                if let Some(existing) = converted.objects[object_index]
                    .relations
                    .iter_mut()
                    .find(|relation| relation.name == slot.name)
                {
                    if existing.realization == "Owned" {
                        existing.method = None;
                        existing.item =
                            (slot.item_controls.len() == 1).then(|| slot.item_controls[0].clone());
                        existing.allowed_objects.clone_from(&slot.item_controls);
                        existing.native_collection = Some(method.to_string());
                        existing.native_item = Some(item);
                    } else {
                        existing.item = if existing.child == "Data" {
                            Some("DataItem".to_string())
                        } else {
                            (existing.identity == "Keyed").then(|| "KeyedVisual".to_string())
                        };
                        existing.native_collection = None;
                        existing.native_item = None;
                    }
                } else {
                    let mut value = relation(&slot.name, None, true);
                    value.item =
                        (slot.item_controls.len() == 1).then(|| slot.item_controls[0].clone());
                    value.allowed_objects.clone_from(&slot.item_controls);
                    value.native_collection = Some(method.to_string());
                    value.native_item = Some(item);
                    converted.objects[object_index].relations.push(value);
                }
            }
        }

        if !handwritten {
            converted.objects[object_index].events.retain(|event| {
                metadata
                    .resolve(name, &format!("add_{}", event.name))
                    .is_some()
            });
        }
        for event in &control.event {
            let observed = event.observe.clone().or_else(|| {
                control
                    .property
                    .iter()
                    .find(|property| property.controlled.as_deref() == Some(event.name.as_str()))
                    .map(|property| property.name.clone())
            });
            let payload = converted_event_payload(
                event,
                observed.as_deref(),
                &converted.objects[object_index],
                metadata,
                name,
            );
            if let Some(existing) = converted.objects[object_index]
                .events
                .iter_mut()
                .find(|candidate| candidate.name == event.name)
            {
                existing.field.clone_from(&event.field);
                if existing.observes.is_none() {
                    existing.observes.clone_from(&observed);
                }
                if let Some((payload, value, adapter)) = payload {
                    existing.payload = Some(payload);
                    existing.payload_adapter = adapter;
                    existing.value = value;
                }
                continue;
            }
            if event.adapter.is_none()
                && event.active_properties.is_empty()
                && !event.routed
                && metadata
                    .resolve(name, &format!("add_{}", event.name))
                    .is_some()
                && let Some(observed) = observed
                && let Some((property_name, property_value)) = converted.objects[object_index]
                    .properties
                    .iter()
                    .find(|property| {
                        property.name == observed
                            || property.native.as_deref() == Some(observed.as_str())
                    })
                    .map(|property| (property.name.clone(), property.value.clone()))
            {
                converted.objects[object_index].events.push(super::Event {
                    name: event.name.clone(),
                    field: event.field.clone(),
                    value: payload
                        .as_ref()
                        .map_or(property_value, |(_, value, _)| value.clone()),
                    observes: Some(property_name),
                    payload: payload.as_ref().map(|(payload, _, _)| payload.clone()),
                    payload_adapter: payload.and_then(|(_, _, adapter)| adapter),
                });
            } else if event.adapter.is_none()
                && event.active_properties.is_empty()
                && !event.routed
                && event.property.is_none()
                && event.observe.is_none()
                && metadata
                    .resolve(name, &format!("add_{}", event.name))
                    .is_some()
            {
                converted.objects[object_index].events.push(super::Event {
                    name: event.name.clone(),
                    field: event.field.clone(),
                    value: "Unit".to_string(),
                    observes: None,
                    payload: None,
                    payload_adapter: None,
                });
            }

            fn converted_event_payload(
                event: &OldEvent,
                observed: Option<&str>,
                object: &super::Object,
                metadata: &tool_reactor::metadata::MetadataResolver,
                owner: &str,
            ) -> Option<(String, String, Option<String>)> {
                let payload = event.property.as_ref()?;
                let (value, _, conversion) = metadata.resolve_event_args_property(
                    owner,
                    &format!("add_{}", event.name),
                    payload,
                )?;
                if conversion != tool_reactor::metadata::ReadValueConversion::Identity {
                    return None;
                }
                let adapter = observed
                    .and_then(|observed| {
                        object.properties.iter().find(|property| {
                            property.name == observed
                                || property.native.as_deref() == Some(observed)
                        })
                    })
                    .and_then(|property| property.adapter.clone())
                    .filter(|adapter| {
                        matches!(
                            adapter.as_str(),
                            "number_box_value" | "rating_value" | "selection_index"
                        )
                    });
                let value = match adapter.as_deref() {
                    Some("number_box_value" | "rating_value") => "OptionalF64".to_string(),
                    Some("selection_index") => "SelectionIndex".to_string(),
                    _ => match value.as_str() {
                        "Str" => "String".to_string(),
                        "Bool" | "F64" => value,
                        _ => return None,
                    },
                };
                Some((payload.clone(), value, adapter))
            }
        }
    }

    toml::to_string_pretty(&converted).map_err(|error| error.to_string())
}

fn direct_property_value(
    metadata: &tool_reactor::metadata::MetadataResolver,
    owner: &str,
    method: &str,
) -> Option<String> {
    match metadata.classify_param(owner, method)? {
        tool_reactor::metadata::ParamClass::Primitive => {
            match metadata.parameter_value(owner, method)?.as_str() {
                "Str" => Some("String".to_string()),
                "Bool" => Some("Bool".to_string()),
                "F64" => Some("F64".to_string()),
                "I32" => Some("I32".to_string()),
                _ => None,
            }
        }
        tool_reactor::metadata::ParamClass::NullableBool => Some("OptionalBool".to_string()),
        tool_reactor::metadata::ParamClass::Complex => metadata
            .enum_info(owner, method)
            .map(|(name, _)| name.to_string()),
        tool_reactor::metadata::ParamClass::IInspectable => None,
    }
}

fn converted_property_value(
    adapter: Option<&str>,
    metadata: &tool_reactor::metadata::MetadataResolver,
    owner: &str,
    method: &str,
) -> Option<String> {
    match adapter {
        Some("inspectable_string") => Some("String".to_string()),
        Some("inspectable_string_list") => Some("StringList".to_string()),
        Some("number_box_value" | "rating_value") => Some("OptionalF64".to_string()),
        Some("selection_index") => Some("SelectionIndex".to_string()),
        Some(
            "clock_identifier" | "horizontal_content_alignment" | "vertical_content_alignment",
        )
        | None => direct_property_value(metadata, owner, method),
        _ => None,
    }
}

fn supported_property_adapter(adapter: Option<&str>) -> bool {
    matches!(
        adapter,
        None | Some(
            "clock_identifier"
                | "horizontal_content_alignment"
                | "inspectable_string"
                | "inspectable_string_list"
                | "number_box_value"
                | "rating_value"
                | "selection_index"
                | "vertical_content_alignment"
        )
    )
}

fn relation(name: &str, native: Option<String>, many: bool) -> super::Relation {
    super::Relation {
        name: name.to_string(),
        native: native.filter(|native| native != name),
        child: "Visual".to_string(),
        allowed_objects: Vec::new(),
        cardinality: if many { "Many" } else { "One" }.to_string(),
        identity: "Positional".to_string(),
        realization: "Owned".to_string(),
        method: None,
        item: None,
        native_collection: None,
        native_item: None,
    }
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
        assert_eq!(report.events.mapped, 37);
        assert_eq!(report.slots.mapped, 42);
        assert_eq!(report.selections.mapped, 3);
        assert!(!report.is_complete());
    }

    #[test]
    fn navigation_selection_requires_exact_event_args_source() {
        let old =
            fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let new =
            fs::read_to_string(workspace_path("crates/tools/reactor2/src/schema.toml")).unwrap();
        let mut schema: Schema = toml::from_str(&new).unwrap();
        let navigation = schema
            .objects
            .iter_mut()
            .find(|object| object.name == "NavigationView")
            .unwrap();
        navigation.selection.as_mut().unwrap().event_item_source = "Owner".to_string();
        navigation.selection.as_mut().unwrap().event_args = None;

        let report = compare(&old, &schema).unwrap();
        assert_eq!(report.selections.mapped, 2);
    }
}
