use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

use crate::schema::{Capability, FeedbackContract, ResolvedControl, ResolvedSchema, Role};

pub(crate) fn generate(schema: &ResolvedSchema) -> String {
    let value_enums = generate_value_enums(schema);
    let elements = schema.controls.iter().map(generate_element);
    let element_variants = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name(#name) }
    });
    let element_conversions = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! {
            impl From<#name> for Element {
                fn from(value: #name) -> Self {
                    Self::#name(value)
                }
            }

            impl From<#name> for View {
                fn from(value: #name) -> Self {
                    Self::native(value)
                }
            }
        }
    });
    let mounted_variants = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name }
    });
    let mounted_props_variants = schema.controls.iter().map(generate_mounted_props_variant);
    let mounted_props_equalities = schema.controls.iter().map(generate_mounted_props_equality);
    let mounted_props_visitors = schema.controls.iter().map(generate_mounted_props_visitor);
    let mounted_event_visitors = schema.controls.iter().map(generate_mounted_event_visitor);
    let mounted_event_dispatchers = schema.controls.iter().flat_map(generate_event_dispatchers);
    let mounted_event_observers = schema.controls.iter().flat_map(generate_event_observers);
    let element_parts = schema.controls.iter().map(generate_element_parts);
    let element_props_matches = schema.controls.iter().map(generate_element_props_match);
    let element_structures = schema.controls.iter().map(generate_element_structure);
    let element_references = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        if control.capabilities.contains(&Capability::Focus) {
            quote! { Self::#name(value) => value.reference.as_ref() }
        } else {
            quote! { Self::#name(_) => None }
        }
    });
    let element_grid_placements = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        if has_grid_placement(control) {
            quote! { Self::#name(value) => value.grid_placement.as_deref() }
        } else {
            quote! { Self::#name(_) => None }
        }
    });
    let element_event_visitors = schema.controls.iter().map(generate_element_event_visitor);
    let element_kinds = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { Self::#name(_) => MountedKind::#name }
    });
    let property_ids = schema.controls.iter().flat_map(|control| {
        control
            .properties
            .iter()
            .map(|property| ident(&format!("{}{}", control.name, property.name)))
    });
    let event_ids = schema.controls.iter().flat_map(|control| {
        control
            .events
            .iter()
            .map(|event| ident(&format!("{}{}", control.name, event.name)))
    });
    let slot_ids = schema.controls.iter().flat_map(|control| {
        control
            .slots
            .iter()
            .map(|slot| ident(&format!("{}{}", control.name, slot.name)))
    });
    let slot_id_lookups = schema
        .controls
        .iter()
        .filter(|control| !control.slots.is_empty())
        .map(|control| {
            let kind = ident(&control.name);
            let indexes = control.slots.iter().enumerate().map(|(index, slot)| {
                let index = u8::try_from(index).unwrap();
                let slot = ident(&format!("{}{}", control.name, slot.name));
                quote! { #index => Some(SlotId::#slot) }
            });
            quote! {
                MountedKind::#kind => match index {
                    #(#indexes,)*
                    _ => None,
                }
            }
        });
    let slot_lists = schema.controls.iter().map(|control| {
        let kind = ident(&control.name);
        let slots = control.slots.iter().map(|slot| {
            let slot = ident(&format!("{}{}", control.name, slot.name));
            quote! { SlotId::#slot }
        });
        quote! { MountedKind::#kind => &[#(#slots),*] }
    });
    let property_values = generate_property_values(schema);
    let event_payloads = generate_event_payloads(schema);
    let descriptors = schema.controls.iter().map(generate_descriptors);
    let controls = schema.controls.iter().map(generate_control);

    let tokens = quote! {
        use crate::element::*;
        use crate::reference::{ElementRef, FocusControl, NativeElementRef};

        pub mod public {
            use super::*;

            #value_enums

            #(#elements)*

            #[derive(Clone, Debug, PartialEq)]
            pub enum Element {
                #(#element_variants),*
            }

            #(#element_conversions)*

            impl ElementPartsExt for Element {
                fn kind(&self) -> MountedKind {
                    match self {
                        #(#element_kinds),*
                    }
                }

                fn into_parts(self) -> ElementParts {
                    match self {
                        #(#element_parts),*
                    }
                }

                fn props_match(&self, props: &MountedProps) -> bool {
                    match (self, props) {
                        #(#element_props_matches),*,
                        _ => false,
                    }
                }

                fn reference(&self) -> Option<&NativeElementRef> {
                    match self {
                        #(#element_references),*
                    }
                }

                fn grid_placement(&self) -> Option<&GridPlacement> {
                    match self {
                        #(#element_grid_placements),*
                    }
                }

                fn structure(&self) -> ElementStructureRef<'_> {
                    match self {
                        #(#element_structures),*
                    }
                }

                fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool)) {
                    match self {
                        #(#element_event_visitors),*
                    }
                }
            }
        }

        use public::*;

        pub trait ElementPartsExt {
            fn kind(&self) -> MountedKind;
            fn into_parts(self) -> ElementParts;
            fn props_match(&self, props: &MountedProps) -> bool;
            fn reference(&self) -> Option<&NativeElementRef>;
            fn grid_placement(&self) -> Option<&GridPlacement>;
            fn structure(&self) -> ElementStructureRef<'_>;
            fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool));
        }

        pub trait MountedPropsExt {
            fn visit_properties(
                &self,
                visit: &mut dyn FnMut(PropertyId, Option<PropertyValueRef<'_>>),
            );
        }

        pub trait MountedEventsExt {
            fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool));
            fn dispatch_event(&self, event: EventId, payload: &EventPayload) -> Option<bool>;
            fn observe_event(
                &self,
                event: EventId,
                payload: &EventPayload,
            ) -> Option<(PropertyId, PropertyValue)>;
        }

        impl MountedPropsExt for MountedProps {
            fn visit_properties(
                &self,
                visit: &mut dyn FnMut(PropertyId, Option<PropertyValueRef<'_>>),
            ) {
                match self {
                    #(#mounted_props_visitors),*
                }
            }
        }

        impl MountedEventsExt for MountedProps {
            fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool)) {
                match self {
                    #(#mounted_event_visitors),*
                }
            }

            fn dispatch_event(&self, event: EventId, payload: &EventPayload) -> Option<bool> {
                match (self, event, payload) {
                    #(#mounted_event_dispatchers,)*
                    _ => None,
                }
            }

            fn observe_event(
                &self,
                event: EventId,
                payload: &EventPayload,
            ) -> Option<(PropertyId, PropertyValue)> {
                match (self, event, payload) {
                    #(#mounted_event_observers,)*
                    _ => None,
                }
            }
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum MountedKind {
            #(#mounted_variants),*
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum SlotId {
            #(#slot_ids),*
        }

        pub fn slot_id(kind: MountedKind, index: u8) -> Option<SlotId> {
            match kind {
                #(#slot_id_lookups,)*
                _ => None,
            }
        }

        pub fn slots(kind: MountedKind) -> &'static [SlotId] {
            match kind {
                #(#slot_lists),*
            }
        }

        #[derive(Clone, Debug)]
        pub enum MountedProps {
            #(#mounted_props_variants),*
        }

        impl PartialEq for MountedProps {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#mounted_props_equalities),*,
                    _ => false,
                }
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub enum ElementStructure {
            None,
            Content(Option<Element>),
            Children(std::rc::Rc<Vec<KeyedElement>>),
            Virtual(std::rc::Rc<Vec<KeyedView>>),
        }

        #[derive(Clone, Copy)]
        pub enum ElementStructureRef<'a> {
            None,
            Content(Option<&'a Element>),
            Children(&'a [KeyedElement]),
            Virtual(&'a [KeyedView]),
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct ElementParts {
            pub kind: MountedKind,
            pub props: MountedProps,
            pub reference: Option<NativeElementRef>,
            pub grid_placement: Option<std::rc::Rc<GridPlacement>>,
            pub structure: ElementStructure,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum PropertyId {
            GridRow,
            GridColumn,
            GridRowSpan,
            GridColumnSpan,
            GridRows,
            GridColumns,
            #(#property_ids),*
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum EventId {
            #(#event_ids),*
        }

        #property_values
        #event_payloads

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum ControlRole {
            Leaf,
            Content,
            Children,
            Controlled,
            Slots,
            Virtual,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Capability {
            Layout,
            TextStyle,
            Enabled,
            Content,
            Children,
            ControlledText,
            Items,
            Focus,
            GridDefinitions,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct PropertyDescriptor {
            pub id: PropertyId,
            pub name: &'static str,
            pub field: &'static str,
            pub value: &'static str,
            pub interface: &'static str,
            pub clearable: bool,
            pub feedback: Option<&'static str>,
            pub feedback_contract: Option<&'static str>,
            pub observes_feedback: bool,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct EventDescriptor {
            pub id: EventId,
            pub name: &'static str,
            pub field: &'static str,
            pub payload: &'static str,
            pub interface: &'static str,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct SlotDescriptor {
            pub id: SlotId,
            pub name: &'static str,
            pub interface: &'static str,
            pub target: &'static str,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct ControlDescriptor {
            pub kind: MountedKind,
            pub name: &'static str,
            pub type_name: &'static str,
            pub role: ControlRole,
            pub capabilities: &'static [Capability],
            pub properties: &'static [PropertyDescriptor],
            pub events: &'static [EventDescriptor],
            pub slots: &'static [SlotDescriptor],
        }

        #(#descriptors)*

        pub const CONTROLS: &[ControlDescriptor] = &[
            #(#controls),*
        ];

        const _: () = {
            let _: Option<PropertyValue> = None;
            let mut control_index = 0;
            while control_index < CONTROLS.len() {
                let control = &CONTROLS[control_index];
                let _ = (
                    control.name,
                    control.type_name,
                    control.kind,
                    control.role,
                    control.capabilities,
                    control.slots,
                );
                let mut property_index = 0;
                while property_index < control.properties.len() {
                    let property = &control.properties[property_index];
                    let _ = (
                        property.name,
                        property.id,
                        property.field,
                        property.value,
                        property.interface,
                        property.clearable,
                        property.feedback,
                        property.feedback_contract,
                        property.observes_feedback,
                    );
                    property_index += 1;
                }
                let mut event_index = 0;
                while event_index < control.events.len() {
                    let event = &control.events[event_index];
                    let _ = (
                        event.id,
                        event.name,
                        event.field,
                        event.payload,
                        event.interface,
                    );
                    event_index += 1;
                }
                let mut slot_index = 0;
                while slot_index < control.slots.len() {
                    let slot = &control.slots[slot_index];
                    let _ = (slot.id, slot.name, slot.interface, slot.target);
                    slot_index += 1;
                }
                control_index += 1;
            }
        };
    };

    format!("// Generated by `tool_reactor_next`. Do not edit.\n\n{tokens}\n")
}

fn has_grid_definitions(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::GridDefinitions)
}

fn has_grid_placement(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::Layout)
}

fn generate_mounted_props_variant(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let property_fields = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let value = value_type(&property.value);
        quote! { #field: Property<#value> }
    });
    let event_fields = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let payload = value_type(&event.payload);
        quote! { #field: Option<Callback<#payload>> }
    });
    let grid_definition_fields = has_grid_definitions(control).then(|| {
        quote! {
            rows: Property<std::rc::Rc<Vec<GridLength>>>,
            columns: Property<std::rc::Rc<Vec<GridLength>>>,
        }
    });
    quote! {
        #name {
            #(#property_fields,)*
            #(#event_fields,)*
            #grid_definition_fields
        }
    }
}

fn generate_mounted_props_equality(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| (property.field.as_str(), Some(property.value.as_str())))
        .chain(
            control
                .events
                .iter()
                .map(|event| (event.field.as_str(), None)),
        )
        .chain(
            has_grid_definitions(control)
                .then_some([
                    ("rows", Some("GridLengths")),
                    ("columns", Some("GridLengths")),
                ])
                .into_iter()
                .flatten(),
        )
        .collect::<Vec<_>>();
    let left = fields
        .iter()
        .map(|(field, _)| ident(&format!("left_{field}")))
        .collect::<Vec<_>>();
    let right = fields
        .iter()
        .map(|(field, _)| ident(&format!("right_{field}")))
        .collect::<Vec<_>>();
    let patterns = fields.iter().zip(&left).map(|((field, _), value)| {
        let field = ident(field);
        quote! { #field: #value }
    });
    let other_patterns = fields.iter().zip(&right).map(|((field, _), value)| {
        let field = ident(field);
        quote! { #field: #value }
    });
    let comparisons = fields
        .iter()
        .zip(left.iter().zip(&right))
        .map(|((_, value), (left, right))| value_equality(*value, left, right));

    quote! {
        (
            Self::#name { #(#patterns),* },
            Self::#name { #(#other_patterns),* },
        ) => true #(&& #comparisons)*
    }
}

fn generate_element_parts(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let (reference_pattern, reference_field) = if control.capabilities.contains(&Capability::Focus)
    {
        (quote! { reference, }, quote! { reference, })
    } else {
        (TokenStream::new(), quote! { reference: None, })
    };
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field))
        .chain(control.events.iter().map(|event| ident(&event.field)))
        .chain(
            has_grid_definitions(control)
                .then_some(["rows", "columns"])
                .into_iter()
                .flatten()
                .map(ident),
        )
        .collect::<Vec<_>>();
    let (grid_pattern, grid_field) = if has_grid_placement(control) {
        (quote! { grid_placement, }, quote! { grid_placement, })
    } else {
        (TokenStream::new(), quote! { grid_placement: None, })
    };
    let (structural_pattern, structure) = match control.role {
        Role::Content => (
            quote! { content },
            quote! { ElementStructure::Content(content.map(|element| *element)) },
        ),
        Role::Children => (
            quote! { children },
            quote! { ElementStructure::Children(children) },
        ),
        Role::Virtual => (
            quote! { items },
            quote! { ElementStructure::Virtual(items) },
        ),
        Role::Leaf | Role::Controlled | Role::Slots => {
            (TokenStream::new(), quote! { ElementStructure::None })
        }
    };

    quote! {
        Self::#name(#name {
            #(#fields,)*
            #reference_pattern
            #grid_pattern
            #structural_pattern
        }) => ElementParts {
            kind: MountedKind::#name,
            props: MountedProps::#name {
                #(#fields),*
            },
            #reference_field
            #grid_field
            structure: #structure,
        }
    }
}

fn generate_element_props_match(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| (property.field.as_str(), Some(property.value.as_str())))
        .chain(
            control
                .events
                .iter()
                .map(|event| (event.field.as_str(), None)),
        )
        .chain(
            has_grid_definitions(control)
                .then_some([
                    ("rows", Some("GridLengths")),
                    ("columns", Some("GridLengths")),
                ])
                .into_iter()
                .flatten(),
        )
        .collect::<Vec<_>>();
    let mounted = fields
        .iter()
        .map(|(field, _)| ident(&format!("mounted_{field}")))
        .collect::<Vec<_>>();
    let field_patterns = fields
        .iter()
        .map(|(field, _)| ident(field))
        .collect::<Vec<_>>();
    let comparisons = fields
        .iter()
        .zip(&mounted)
        .map(|((field, value), mounted)| value_equality(*value, &ident(field), mounted));

    quote! {
        (
            Self::#name(#name { #(#field_patterns,)* .. }),
            MountedProps::#name { #(#field_patterns: #mounted),* },
        ) => true #(&& #comparisons)*
    }
}

fn generate_element_structure(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    match control.role {
        Role::Content => {
            quote! { Self::#name(value) => ElementStructureRef::Content(value.content.as_deref()) }
        }
        Role::Children => {
            quote! { Self::#name(value) => ElementStructureRef::Children(value.children.as_slice()) }
        }
        Role::Virtual => {
            quote! { Self::#name(value) => ElementStructureRef::Virtual(value.items.as_slice()) }
        }
        Role::Leaf | Role::Controlled | Role::Slots => {
            quote! { Self::#name(_) => ElementStructureRef::None }
        }
    }
}

fn generate_element_event_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control.events.iter().filter_map(|event| {
        let feedback = control
            .properties
            .iter()
            .any(|property| property.feedback.as_deref() == Some(event.name.as_str()));
        (!feedback).then(|| ident(&event.field))
    });
    let fields = fields.collect::<Vec<_>>();
    let events = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let id = ident(&format!("{}{}", control.name, event.name));
        let feedback = control
            .properties
            .iter()
            .any(|property| property.feedback.as_deref() == Some(event.name.as_str()));
        if feedback {
            quote! { visit(EventId::#id, true); }
        } else {
            quote! { visit(EventId::#id, #field.is_some()); }
        }
    });
    let pattern = if fields.is_empty() {
        quote! { Self::#name(_) }
    } else {
        quote! { Self::#name(#name { #(#fields,)* .. }) }
    };
    quote! {
        #pattern => {
            #(#events)*
        }
    }
}

fn generate_mounted_props_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field))
        .chain(
            has_grid_definitions(control)
                .then_some(["rows", "columns"])
                .into_iter()
                .flatten()
                .map(ident),
        );
    let properties = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let id = ident(&format!("{}{}", control.name, property.name));
        let variant = ident(&property.value);
        let value = if property.value == "Str" {
            quote! { value.as_str() }
        } else {
            quote! { *value }
        };
        quote! {
            visit(
                PropertyId::#id,
                match #field {
                    Property::Inherited => None,
                    Property::Set(value) => Some(PropertyValueRef::#variant(#value)),
                },
            );
        }
    });
    let grid_definitions = has_grid_definitions(control).then(|| {
        quote! {
            visit(
                PropertyId::GridRows,
                rows.as_set().map(PropertyValueRef::GridLengths),
            );
            visit(
                PropertyId::GridColumns,
                columns
                    .as_set()
                    .map(PropertyValueRef::GridLengths),
            );
        }
    });

    quote! {
        Self::#name { #(#fields,)* .. } => {
            #(#properties)*
            #grid_definitions
        }
    }
}

fn generate_mounted_event_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control.events.iter().filter_map(|event| {
        let feedback = control
            .properties
            .iter()
            .any(|property| property.feedback.as_deref() == Some(event.name.as_str()));
        (!feedback).then(|| ident(&event.field))
    });
    let events = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let id = ident(&format!("{}{}", control.name, event.name));
        let feedback = control
            .properties
            .iter()
            .any(|property| property.feedback.as_deref() == Some(event.name.as_str()));
        if feedback {
            quote! { visit(EventId::#id, true); }
        } else {
            quote! { visit(EventId::#id, #field.is_some()); }
        }
    });
    quote! {
        Self::#name { #(#fields,)* .. } => {
            #(#events)*
        }
    }
}

fn generate_event_dispatchers(control: &ResolvedControl) -> Vec<TokenStream> {
    let name = ident(&control.name);
    control
        .events
        .iter()
        .map(|event| {
            let field = ident(&event.field);
            let id = ident(&format!("{}{}", control.name, event.name));
            let payload = ident(&event.payload);
            let call = if event.payload == "Unit" {
                quote! { callback.call(()) }
            } else if event.payload == "Str" {
                quote! { callback.call(value.clone()) }
            } else {
                quote! { callback.call(*value) }
            };
            let payload_pattern = if event.payload == "Unit" {
                quote! { EventPayload::#payload }
            } else {
                quote! { EventPayload::#payload(value) }
            };
            quote! {
                (
                    Self::#name { #field: Some(callback), .. },
                    EventId::#id,
                    #payload_pattern,
                ) => Some(#call)
            }
        })
        .collect()
}

fn generate_event_observers(control: &ResolvedControl) -> Vec<TokenStream> {
    let name = ident(&control.name);
    control
        .properties
        .iter()
        .filter(|property| property.observes_feedback)
        .filter_map(|property| {
            let feedback = property.feedback.as_ref()?;
            let event = control
                .events
                .iter()
                .find(|event| &event.name == feedback)
                .unwrap();
            let event_id = ident(&format!("{}{}", control.name, event.name));
            let property_id = ident(&format!("{}{}", control.name, property.name));
            let payload = ident(&event.payload);
            let value = if event.payload == "Str" {
                quote! { value.clone() }
            } else {
                quote! { *value }
            };
            Some(quote! {
                (
                    Self::#name { .. },
                    EventId::#event_id,
                    EventPayload::#payload(value),
                ) => Some((PropertyId::#property_id, (#value).into()))
            })
        })
        .collect()
}

fn generate_property_values(schema: &ResolvedSchema) -> TokenStream {
    let mut values = BTreeMap::from([
        (
            "GridLengths".to_string(),
            quote! { std::rc::Rc<Vec<GridLength>> },
        ),
        ("I32".to_string(), quote! { i32 }),
    ]);
    for property in schema
        .controls
        .iter()
        .flat_map(|control| &control.properties)
    {
        values
            .entry(property.value.clone())
            .or_insert_with(|| value_type(&property.value));
    }

    let variants = values.iter().map(|(name, value)| {
        let name = ident(name);
        quote! { #name(#value) }
    });
    let ref_variants = values.iter().map(|(name, value)| {
        let variant = ident(name);
        let value = match name.as_str() {
            "GridLengths" => quote! { &'a std::rc::Rc<Vec<GridLength>> },
            "Str" => quote! { &'a str },
            _ => value.clone(),
        };
        quote! { #variant(#value) }
    });
    let conversions = values.iter().map(|(name, value)| {
        let name = ident(name);
        quote! {
            impl From<#value> for PropertyValue {
                fn from(value: #value) -> Self {
                    Self::#name(value)
                }
            }
        }
    });
    let equalities = values.keys().map(|name| {
        let variant = ident(name);
        if name == "F64" {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => f64_eq(*left, *right)
            }
        } else {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => left == right
            }
        }
    });
    let ref_equalities = values.keys().map(|name| {
        let variant = ident(name);
        if name == "F64" {
            quote! {
                (Self::#variant(left), PropertyValue::#variant(right)) => f64_eq(left, *right)
            }
        } else if matches!(name.as_str(), "GridLengths" | "Str") {
            quote! {
                (Self::#variant(left), PropertyValue::#variant(right)) => left == right
            }
        } else {
            quote! {
                (Self::#variant(left), PropertyValue::#variant(right)) => left == *right
            }
        }
    });
    let ref_to_owned = values.keys().map(|name| {
        let variant = ident(name);
        match name.as_str() {
            "GridLengths" => quote! {
                Self::#variant(value) => PropertyValue::#variant(value.clone())
            },
            "Str" => quote! {
                Self::#variant(value) => PropertyValue::#variant(value.to_string())
            },
            _ => quote! {
                Self::#variant(value) => PropertyValue::#variant(value)
            },
        }
    });

    quote! {
        #[derive(Clone, Debug)]
        pub enum PropertyValue {
            #(#variants),*
        }

        impl PartialEq for PropertyValue {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#equalities),*,
                    _ => false,
                }
            }
        }

        #[derive(Clone, Copy, Debug)]
        pub enum PropertyValueRef<'a> {
            #(#ref_variants),*
        }

        impl PropertyValueRef<'_> {
            pub fn equals_owned(self, value: &PropertyValue) -> bool {
                match (self, value) {
                    #(#ref_equalities),*,
                    _ => false,
                }
            }

            pub fn into_owned(self) -> PropertyValue {
                match self {
                    #(#ref_to_owned),*
                }
            }
        }

        #(#conversions)*
    }
}

fn generate_event_payloads(schema: &ResolvedSchema) -> TokenStream {
    let mut payloads = BTreeMap::new();
    for event in schema.controls.iter().flat_map(|control| &control.events) {
        payloads
            .entry(event.payload.clone())
            .or_insert_with(|| value_type(&event.payload));
    }
    let variants = payloads.iter().map(|(name, value)| {
        let is_unit = name == "Unit";
        let name = ident(name);
        if is_unit {
            quote! { #name }
        } else {
            quote! { #name(#value) }
        }
    });
    let equalities = payloads.keys().map(|name| {
        let variant = ident(name);
        if name == "Unit" {
            quote! { (Self::#variant, Self::#variant) => true }
        } else if name == "F64" {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => f64_eq(*left, *right)
            }
        } else {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => left == right
            }
        }
    });
    quote! {
        #[derive(Clone, Debug)]
        pub enum EventPayload {
            #(#variants),*
        }

        impl PartialEq for EventPayload {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#equalities),*,
                    _ => false,
                }
            }
        }
    }
}

fn generate_value_enums(schema: &ResolvedSchema) -> TokenStream {
    let mut enums = BTreeMap::new();
    for property in schema
        .controls
        .iter()
        .flat_map(|control| &control.properties)
        .filter(|property| !property.enum_variants.is_empty())
    {
        enums
            .entry(property.value.clone())
            .or_insert_with(|| property.enum_variants.clone());
    }

    let enums = enums.into_iter().map(|(name, variants)| {
        let name = ident(&name);
        let variants = variants.iter().map(|variant| ident(variant));
        quote! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum #name {
                #(#variants),*
            }
        }
    });
    quote! { #(#enums)* }
}

fn generate_element(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let (reference_field, reference_method, reference_impls) =
        if control.capabilities.contains(&Capability::Focus) {
            (
                quote! { reference: Option<NativeElementRef>, },
                quote! {
                    pub fn element_ref(mut self, reference: &ElementRef<Self>) -> Self {
                        self.reference = Some(reference.binding());
                        self
                    }
                },
                quote! {
                    impl crate::reference::sealed::Sealed for #name {}
                    impl crate::reference::ReferenceType for #name {}
                },
            )
        } else {
            (TokenStream::new(), TokenStream::new(), TokenStream::new())
        };
    let property_fields = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let value = value_type(&property.value);
        quote! { #field: Property<#value> }
    });
    let event_fields = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let payload = value_type(&event.payload);
        quote! { #field: Option<Callback<#payload>> }
    });
    let grid_placement_field = has_grid_placement(control)
        .then(|| quote! { grid_placement: Option<std::rc::Rc<GridPlacement>>, });
    let grid_definition_fields = has_grid_definitions(control).then(|| {
        quote! {
            rows: Property<std::rc::Rc<Vec<GridLength>>>,
            columns: Property<std::rc::Rc<Vec<GridLength>>>,
        }
    });
    let structural_field = match control.role {
        Role::Content => quote! { content: Option<Box<Element>> },
        Role::Children => quote! { children: std::rc::Rc<Vec<KeyedElement>> },
        Role::Virtual => quote! { items: std::rc::Rc<Vec<KeyedView>> },
        Role::Leaf | Role::Controlled | Role::Slots => TokenStream::new(),
    };
    let property_methods = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let value = value_type(&property.value);
        if property.value == "Str" {
            let optional = ident(&format!("{}_optional", property.field));
            quote! {
                pub fn #field(mut self, value: impl Into<String>) -> Self {
                    self.#field = Property::Set(value.into());
                    self
                }

                pub fn #optional<T>(mut self, value: Option<T>) -> Self
                where
                    T: Into<String>,
                {
                    self.#field = Property::from(value.map(Into::into));
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, value: impl Into<Option<#value>>) -> Self {
                    self.#field = Property::from(value.into());
                    self
                }
            }
        }
    });
    let event_methods = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let payload = value_type(&event.payload);
        if event.payload == "Unit" {
            quote! {
                pub fn #field(mut self, callback: impl IntoUnitCallback) -> Self {
                    self.#field = Some(callback.into_unit_callback());
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, callback: impl IntoPayloadCallback<#payload>) -> Self {
                    self.#field = Some(callback.into_payload_callback());
                    self
                }
            }
        }
    });
    let grid_definition_methods = has_grid_definitions(control).then(|| {
        quote! {
            pub fn rows(
                mut self,
                values: impl IntoIterator<Item = GridLength>,
            ) -> Self {
                let values = values.into_iter().collect::<Vec<_>>();
                assert!(
                    values.iter().all(|value| value.is_valid()),
                    "Grid lengths must be finite and non-negative",
                );
                self.rows = Property::Set(std::rc::Rc::new(values));
                self
            }

            pub fn rows_optional<T>(mut self, values: Option<T>) -> Self
            where
                T: IntoIterator<Item = GridLength>,
            {
                self.rows = Property::from(
                    values.map(|values| {
                        let values = values.into_iter().collect::<Vec<_>>();
                        assert!(
                            values.iter().all(|value| value.is_valid()),
                            "Grid lengths must be finite and non-negative",
                        );
                        std::rc::Rc::new(values)
                    }),
                );
                self
            }

            pub fn columns(
                mut self,
                values: impl IntoIterator<Item = GridLength>,
            ) -> Self {
                let values = values.into_iter().collect::<Vec<_>>();
                assert!(
                    values.iter().all(|value| value.is_valid()),
                    "Grid lengths must be finite and non-negative",
                );
                self.columns = Property::Set(std::rc::Rc::new(values));
                self
            }

            pub fn columns_optional<T>(mut self, values: Option<T>) -> Self
            where
                T: IntoIterator<Item = GridLength>,
            {
                self.columns = Property::from(
                    values.map(|values| {
                        let values = values.into_iter().collect::<Vec<_>>();
                        assert!(
                            values.iter().all(|value| value.is_valid()),
                            "Grid lengths must be finite and non-negative",
                        );
                        std::rc::Rc::new(values)
                    }),
                );
                self
            }
        }
    });
    let structural_methods = match control.role {
        Role::Content => quote! {
            #[allow(dead_code)]
            pub(crate) fn native_content(mut self, content: impl Into<Element>) -> Self {
                self.content = Some(Box::new(content.into()));
                self
            }
        },
        Role::Children => quote! {
            #[allow(dead_code)]
            pub(crate) fn native_child(
                mut self,
                key: impl Into<Key>,
                child: impl Into<Element>,
            ) -> Self {
                std::rc::Rc::make_mut(&mut self.children).push(KeyedElement::new(key, child));
                self
            }

            #[allow(dead_code)]
            pub(crate) fn native_children(
                mut self,
                children: impl IntoIterator<Item = KeyedElement>,
            ) -> Self {
                self.children = std::rc::Rc::new(children.into_iter().collect());
                self
            }
        },
        Role::Virtual => quote! {
            pub fn item(mut self, key: impl Into<Key>, item: impl Into<View>) -> Self {
                std::rc::Rc::make_mut(&mut self.items).push(KeyedView::new(key, item));
                self
            }

            pub fn items(
                mut self,
                items: impl IntoIterator<Item = KeyedView>,
            ) -> Self {
                self.items = std::rc::Rc::new(items.into_iter().collect());
                self
            }
        },
        Role::Leaf | Role::Controlled | Role::Slots => TokenStream::new(),
    };
    let capability_impls = control.capabilities.iter().map(|capability| {
        if *capability == Capability::Layout && has_grid_placement(control) {
            return quote! {
                impl LayoutControl for #name {
                    fn grid_placement_mut(
                        &mut self,
                    ) -> &mut Option<std::rc::Rc<GridPlacement>> {
                        &mut self.grid_placement
                    }
                }
            };
        }
        let capability = ident(match capability {
            Capability::Layout => "LayoutControl",
            Capability::TextStyle => "TextStyleControl",
            Capability::Enabled => "EnabledControl",
            Capability::Content => "ContentControl",
            Capability::Children => "ChildrenControl",
            Capability::ControlledText => "ControlledTextControl",
            Capability::Items => "ItemsControl",
            Capability::Focus => "FocusControl",
            Capability::GridDefinitions => "GridDefinitionsControl",
        });
        quote! { impl #capability for #name {} }
    });
    let slots = if control.slots.is_empty() {
        TokenStream::new()
    } else {
        let slot_name = ident(&format!("{}Slot", control.name));
        let variants = control.slots.iter().map(|slot| ident(&slot.name));
        quote! {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub enum #slot_name {
                #(#variants),*
            }

            impl SlotsControl for #name {
                type Slot = #slot_name;

                fn slot_index(slot: Self::Slot) -> u8 {
                    slot as u8
                }
            }
        }
    };
    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct #name {
            #(#property_fields,)*
            #(#event_fields,)*
            #reference_field
            #grid_placement_field
            #grid_definition_fields
            #structural_field
        }

        impl #name {
            pub fn new() -> Self {
                Self::default()
            }

            #reference_method

            #(#property_methods)*
            #(#event_methods)*
            #grid_definition_methods
            #structural_methods
        }

        impl sealed::Sealed for #name {}
        #reference_impls
        #(#capability_impls)*
        #slots
    }
}

fn value_equality(
    value: Option<&str>,
    left: &impl quote::ToTokens,
    right: &impl quote::ToTokens,
) -> TokenStream {
    if value == Some("F64") {
        quote! { f64_property_eq(#left, #right) }
    } else {
        quote! { #left == #right }
    }
}

fn generate_descriptors(control: &ResolvedControl) -> TokenStream {
    let properties_ident = descriptor_ident(&control.name, "PROPERTIES");
    let events_ident = descriptor_ident(&control.name, "EVENTS");
    let slots_ident = descriptor_ident(&control.name, "SLOTS");
    let properties = control.properties.iter().map(|property| {
        let id = ident(&format!("{}{}", control.name, property.name));
        let name = &property.name;
        let field = &property.field;
        let value = &property.value;
        let interface = &property.interface;
        let clearable = property.clearable;
        let feedback = property
            .feedback
            .as_deref()
            .map_or_else(|| quote! { None }, |value| quote! { Some(#value) });
        let observes_feedback = property.observes_feedback;
        let feedback_contract = property.feedback_contract.map_or_else(
            || quote! { None },
            |value| {
                let value = match value {
                    FeedbackContract::SynchronousExact => "synchronous_exact",
                    FeedbackContract::SynchronousNormalized => "synchronous_normalized",
                    FeedbackContract::DeferredOrdered => "deferred_ordered",
                    FeedbackContract::DeferredCoalesced => "deferred_coalesced",
                    FeedbackContract::Unknown => "unknown",
                };
                quote! { Some(#value) }
            },
        );
        quote! {
            PropertyDescriptor {
                id: PropertyId::#id,
                name: #name,
                field: #field,
                value: #value,
                interface: #interface,
                clearable: #clearable,
                feedback: #feedback,
                feedback_contract: #feedback_contract,
                observes_feedback: #observes_feedback,
            }
        }
    });
    let events = control.events.iter().map(|event| {
        let id = ident(&format!("{}{}", control.name, event.name));
        let name = &event.name;
        let field = &event.field;
        let payload = &event.payload;
        let interface = &event.interface;
        quote! {
            EventDescriptor {
                id: EventId::#id,
                name: #name,
                field: #field,
                payload: #payload,
                interface: #interface,
            }
        }
    });
    let slots = control.slots.iter().map(|slot| {
        let id = ident(&format!("{}{}", control.name, slot.name));
        let name = &slot.name;
        let interface = &slot.interface;
        let target = match slot.target {
            crate::schema::SlotTarget::Inspectable => "inspectable",
            crate::schema::SlotTarget::UiElement => "ui_element",
        };
        quote! {
            SlotDescriptor {
                id: SlotId::#id,
                name: #name,
                interface: #interface,
                target: #target,
            }
        }
    });

    quote! {
        const #properties_ident: &[PropertyDescriptor] = &[
            #(#properties),*
        ];
        const #events_ident: &[EventDescriptor] = &[
            #(#events),*
        ];
        const #slots_ident: &[SlotDescriptor] = &[
            #(#slots),*
        ];
    }
}

fn generate_control(control: &ResolvedControl) -> TokenStream {
    let name = &control.name;
    let kind = ident(name);
    let type_name = &control.type_name;
    let role = Ident::new(
        match control.role {
            Role::Leaf => "Leaf",
            Role::Content => "Content",
            Role::Children => "Children",
            Role::Controlled => "Controlled",
            Role::Slots => "Slots",
            Role::Virtual => "Virtual",
        },
        Span::call_site(),
    );
    let capabilities = control.capabilities.iter().map(|capability| {
        let capability = Ident::new(
            match capability {
                Capability::Layout => "Layout",
                Capability::TextStyle => "TextStyle",
                Capability::Enabled => "Enabled",
                Capability::Content => "Content",
                Capability::Children => "Children",
                Capability::ControlledText => "ControlledText",
                Capability::Items => "Items",
                Capability::Focus => "Focus",
                Capability::GridDefinitions => "GridDefinitions",
            },
            Span::call_site(),
        );
        quote! { Capability::#capability }
    });
    let properties = descriptor_ident(name, "PROPERTIES");
    let events = descriptor_ident(name, "EVENTS");
    let slots = descriptor_ident(name, "SLOTS");

    quote! {
        ControlDescriptor {
            kind: MountedKind::#kind,
            name: #name,
            type_name: #type_name,
            role: ControlRole::#role,
            capabilities: &[#(#capabilities),*],
            properties: #properties,
            events: #events,
            slots: #slots,
        }
    }
}

fn descriptor_ident(control: &str, suffix: &str) -> Ident {
    let name = tool_reactor::helpers::to_snake_case(control).to_ascii_uppercase();
    Ident::new(&format!("{name}_{suffix}"), Span::call_site())
}

fn ident(value: &str) -> Ident {
    Ident::new(value, Span::call_site())
}

fn value_type(value: &str) -> TokenStream {
    match value {
        "Unit" => quote! { () },
        "Str" => quote! { String },
        "Bool" => quote! { bool },
        "F64" => quote! { f64 },
        "I32" => quote! { i32 },
        "U16" => quote! { u16 },
        "U32" => quote! { u32 },
        "U8" => quote! { u8 },
        value => {
            let value = ident(value);
            quote! { #value }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Schema, workspace_path};
    use tool_reactor::metadata::MetadataResolver;

    #[test]
    fn output_contains_all_control_rows() {
        let schema = Schema::parse(include_str!("winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = schema.resolve(&metadata).unwrap();
        let output = generate(&resolved);

        assert_eq!(output.matches("ControlDescriptor").count(), 15);
        assert!(output.contains("feedback : Some"));
        assert!(output.contains("feedback_contract : Some (\"synchronous_normalized\")"));
        assert!(output.contains("pub struct NumberBox"));
        assert!(output.contains("pub struct Slider"));
        assert!(output.contains("pub struct NavigationView"));
        assert!(output.contains("pub struct ProgressBar"));
        assert!(output.contains("pub struct ToggleSwitch"));
        assert!(output.contains("pub struct Grid"));
        assert!(output.contains("pub struct SplitView"));
        assert!(output.contains("pub enum SplitViewDisplayMode"));
        assert!(output.contains("impl SlotsControl for SplitView"));
        assert!(output.contains("pub fn rows"));
        assert!(output.contains("PropertyId :: GridRows"));
        assert!(output.contains("impl From < TextBlock > for View"));
        assert!(output.contains("pub enum NavigationViewSlot"));
        assert!(output.contains("impl SlotsControl for NavigationView"));
        assert!(output.contains("NavigationViewContent"));
        assert!(output.contains("NavigationViewHeader"));
        assert!(output.contains("ControlRole :: Children"));
        assert!(output.contains("pub struct TextBox"));
        assert!(output.contains("impl ControlledTextControl for TextBox"));
        assert!(output.contains("impl ItemsControl for ItemsRepeater"));
        assert!(output.contains("item : impl Into < View >"));
        assert!(output.contains("Item = KeyedView"));
        assert!(!output.contains("item_elements"));
        assert!(output.contains("pub enum Orientation"));
        assert!(output.contains("callback : impl IntoUnitCallback"));
        assert!(output.contains("callback : impl IntoPayloadCallback < String >"));
        assert!(output.contains("value : impl Into < Option < bool >"));
        assert!(output.contains("pub fn text_optional"));
        assert!(output.contains("pub fn rows_optional"));
        assert!(output.contains("pub fn columns_optional"));
        assert!(!output.contains("pub fn text_property"));
        assert!(!output.contains("pub fn on_click_callback"));
        assert!(!output.contains("pub fn rows_property"));
        assert!(!output.contains("pub fn columns_property"));
    }

    #[test]
    fn ordinary_control_needs_only_schema_input() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.ProgressBar"
role = "leaf"
capabilities = ["layout"]

[[control.property]]
name = "Value"
clearable = true
"#;
        let schema = Schema::parse(source).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = schema.resolve(&metadata).unwrap();
        let output = generate(&resolved);

        assert!(output.contains("pub struct ProgressBar"));
        assert!(output.contains("value : Property < f64 >"));
        assert!(output.contains("value : impl Into < Option < f64 >"));
        assert!(
            output.contains("interface : \"Microsoft.UI.Xaml.Controls.Primitives.IRangeBase\"")
        );
    }

    #[test]
    fn ordinary_event_payload_is_not_a_property_observation() {
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
        let output = generate(&resolved);

        assert!(output.contains("EventId :: NumberBoxValueChanged"));
        assert!(!output.contains("PropertyId :: NumberBoxNewValue"));
    }
}
