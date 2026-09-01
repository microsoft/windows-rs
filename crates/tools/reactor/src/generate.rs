use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

use crate::schema::{
    Capability, EventPayloadConversion, FeedbackContract, Lifecycle, PropertyAdapter,
    ResolvedControl, ResolvedPlacement, ResolvedSchema, Role, ValueValidation,
};

pub(crate) fn generate(schema: &ResolvedSchema) -> String {
    let value_enums = generate_value_enums(schema);
    let event_groups = schema
        .controls
        .iter()
        .filter(|control| has_grouped_events(control))
        .map(generate_event_group);
    let elements = schema.controls.iter().map(generate_element);
    let element_variants = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name(std::rc::Rc<#name>) }
    });
    let element_conversions = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        let view = if control.lifecycle == Some(Lifecycle::ContentDialog) {
            quote! {
                let open = value.is_open;
                Self::content_dialog(value.into(), None, open)
            }
        } else {
            quote! { Self::native(value) }
        };
        quote! {
            impl From<#name> for Element {
                fn from(value: #name) -> Self {
                    Self::#name(std::rc::Rc::new(value))
                }

            }

            impl From<#name> for View {
                fn from(value: #name) -> Self {
                    #view
                }
            }
        }
    });
    let mounted_variants = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name }
    });
    let mounted_props_structures = schema.controls.iter().map(generate_mounted_props_structure);
    let mounted_props_variants = schema.controls.iter().map(generate_mounted_props_variant);
    let mounted_props_visitors = schema.controls.iter().map(generate_mounted_props_visitor);
    let mounted_theme_styles = schema.controls.iter().map(generate_mounted_theme_style);
    let mounted_event_visitors = schema.controls.iter().map(generate_mounted_event_visitor);
    let mounted_event_dispatchers = schema.controls.iter().flat_map(generate_event_dispatchers);
    let mounted_event_observers = schema.controls.iter().flat_map(generate_event_observers);
    let element_parts = schema.controls.iter().map(generate_element_parts);
    let element_props_matches = schema.controls.iter().map(generate_element_props_match);
    let element_structures = schema.controls.iter().map(generate_element_structure);
    let element_references = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        if has_reference(control) {
            quote! { Self::#name(value) => value.reference.as_ref() }
        } else {
            quote! { Self::#name(_) => None }
        }
    });
    let element_window_title_bars = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        if has_window_title_bar(control) {
            quote! { Self::#name(value) => Some(value.preferred_height) }
        } else {
            quote! { Self::#name(_) => None }
        }
    });
    let element_states = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        if has_element_state(control) {
            quote! { Self::#name(value) => value.element_state.as_deref() }
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
    let collection_slot_lookups = schema.controls.iter().flat_map(|control| {
        control
            .slots
            .iter()
            .filter(|slot| matches!(&slot.shape, crate::schema::SlotShape::Collection(_)))
            .map(move |slot| {
                let slot = ident(&format!("{}{}", control.name, slot.name));
                quote! { SlotId::#slot }
            })
    });
    let property_values = generate_property_values(schema);
    let event_payloads = generate_event_payloads(schema);
    let descriptors = schema.controls.iter().map(generate_descriptors);
    let controls = schema.controls.iter().map(generate_control);

    let tokens = quote! {
        use crate::core::ThemeStyle;
        use crate::element::*;
        use crate::reference::{ElementRef, FocusControl, NativeElementRef};

        #(#event_groups)*

        pub mod public {
            use super::*;

            #value_enums

            #(#elements)*

            #[derive(Clone, Debug, PartialEq)]
            pub(crate) enum Element {
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

                fn element_state(&self) -> Option<&ElementState> {
                    match self {
                        #(#element_states),*
                    }
                }

                fn window_title_bar(&self) -> Option<WindowTitleBarHeight> {
                    match self {
                        #(#element_window_title_bars),*
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
            fn element_state(&self) -> Option<&ElementState>;
            fn window_title_bar(&self) -> Option<WindowTitleBarHeight>;
            fn structure(&self) -> ElementStructureRef<'_>;
            fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool));
        }

        pub trait MountedPropsExt {
            fn visit_properties(
                &self,
                visit: &mut dyn FnMut(PropertyId, Option<PropertyValueRef<'_>>),
            );
            fn theme_style(&self) -> ThemeStyle;
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

            fn theme_style(&self) -> ThemeStyle {
                match self {
                    #(#mounted_theme_styles),*
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

        pub fn slot_is_collection(slot: SlotId) -> bool {
            matches!(slot, #(#collection_slot_lookups)|*)
        }

        #(#mounted_props_structures)*

        #[derive(Clone, Debug, PartialEq)]
        pub enum MountedProps {
            #(#mounted_props_variants),*
        }

        #[derive(Clone, Debug, PartialEq)]
        pub enum ElementStructure {
            None,
            Content(Option<Element>),
            Children(std::rc::Rc<Vec<KeyedElement>>),
            Virtual(VirtualItems),
        }

        #[derive(Clone, Copy)]
        pub enum ElementStructureRef<'a> {
            None,
            Content(Option<&'a Element>),
            Children(&'a [KeyedElement]),
            Virtual(&'a VirtualItems),
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct ElementParts {
            pub kind: MountedKind,
            pub props: MountedProps,
            pub reference: Option<NativeElementRef>,
            pub element_state: Option<std::rc::Rc<ElementState>>,
            pub window_title_bar: Option<WindowTitleBarHeight>,
            pub structure: ElementStructure,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum PropertyId {
            Width,
            Height,
            MinWidth,
            MaxWidth,
            MinHeight,
            MaxHeight,
            Opacity,
            HorizontalAlignment,
            VerticalAlignment,
            Margin,
            GridRow,
            GridColumn,
            GridRowSpan,
            GridColumnSpan,
            GridRows,
            GridColumns,
            RelativeAlignLeft,
            RelativeAlignTop,
            RelativeAlignRight,
            RelativeAlignBottom,
            RelativeAlignHorizontalCenter,
            RelativeAlignVerticalCenter,
            CanvasLeft,
            CanvasTop,
            AutomationName,
            AutomationId,
            AutomationHeadingLevel,
            #(#property_ids),*
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum EventId {
            OwnedCommandInvoked,
            OwnedMenuItemInvoked,
            #(#event_ids),*
        }

        #property_values
        #event_payloads

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum ControlRole {
            Leaf,
            Content,
            Children,
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
            Reference,
            GridDefinitions,
            WindowTitleBar,
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
            pub collection: bool,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct SelectionDescriptor {
            pub slot: SlotId,
            pub item: MountedKind,
            pub selected_property: PropertyId,
            pub event: EventId,
            pub payload_property: PropertyId,
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct ControlledCollectionDescriptor {
            pub slot: SlotId,
            pub property: PropertyId,
            pub event: EventId,
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
            pub selection: Option<SelectionDescriptor>,
            pub controlled_collection: Option<ControlledCollectionDescriptor>,
        }

        #(#descriptors)*

        pub const CONTROLS: &[ControlDescriptor] = &[
            #(#controls),*
        ];

        pub fn selection_for_event(event: EventId) -> Option<SelectionDescriptor> {
            CONTROLS
                .iter()
                .find_map(|control| control.selection.filter(|selection| selection.event == event))
        }

        pub fn selection_for_slot(slot: SlotId) -> Option<SelectionDescriptor> {
            CONTROLS
                .iter()
                .find_map(|control| control.selection.filter(|selection| selection.slot == slot))
        }

        pub fn selection_for_item_property(
            property: PropertyId,
            slot: SlotId,
        ) -> Option<SelectionDescriptor> {
            CONTROLS.iter().find_map(|control| {
                control.selection.filter(|selection| {
                    selection.selected_property == property && selection.slot == slot
                })
            })
        }

        pub fn controlled_collection_for_slot(
            slot: SlotId,
        ) -> Option<ControlledCollectionDescriptor> {
            CONTROLS.iter().find_map(|control| {
                control
                    .controlled_collection
                    .filter(|collection| collection.slot == slot)
            })
        }

        pub fn controlled_collection_for_property(
            property: PropertyId,
        ) -> Option<ControlledCollectionDescriptor> {
            CONTROLS.iter().find_map(|control| {
                control
                    .controlled_collection
                    .filter(|collection| collection.property == property)
            })
        }

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
                    control.controlled_collection,
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

    format!("// Generated by `tool_reactor`. Do not edit.\n\n{tokens}\n")
}

fn has_grid_definitions(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::GridDefinitions)
}

fn has_element_state(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::Layout)
}

fn has_grouped_events(control: &ResolvedControl) -> bool {
    control.events.len() > 1
}

fn generate_event_group(control: &ResolvedControl) -> TokenStream {
    let name = ident(&format!("{}Events", control.name));
    let fields = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let payload = event_callback_type(event);
        quote! { #field: Option<Callback<#payload>> }
    });
    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub(crate) struct #name {
            #(#fields,)*
        }
    }
}

fn generate_mounted_props_variant(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let props = ident(&format!("{}MountedProps", control.name));
    quote! { #name(std::rc::Rc<#props>) }
}

fn generate_mounted_props_structure(control: &ResolvedControl) -> TokenStream {
    let name = ident(&format!("{}MountedProps", control.name));
    let fields = control
        .properties
        .iter()
        .map(|property| (property.field.as_str(), Some(property.value.as_str())))
        .chain(if has_grouped_events(control) {
            vec![("events", None)]
        } else {
            control
                .events
                .iter()
                .map(|event| (event.field.as_str(), None))
                .collect()
        })
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
    let comparisons = fields.iter().map(|(field, value)| {
        let field = ident(field);
        value_equality(*value, &quote! { self.#field }, &quote! { other.#field })
    });
    let other = if fields.is_empty() {
        ident("_other")
    } else {
        ident("other")
    };
    let property_fields = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let value = value_type(&property.value);
        quote! { #field: Property<#value> }
    });
    let event_fields = if has_grouped_events(control) {
        let events = ident(&format!("{}Events", control.name));
        quote! { events: Option<std::rc::Rc<#events>>, }
    } else {
        let fields = control.events.iter().map(|event| {
            let field = ident(&event.field);
            let payload = event_callback_type(event);
            quote! { #field: Option<Callback<#payload>>, }
        });
        quote! { #(#fields)* }
    };
    let grid_definition_fields = has_grid_definitions(control).then(|| {
        quote! {
            rows: Property<std::rc::Rc<Vec<GridLength>>>,
            columns: Property<std::rc::Rc<Vec<GridLength>>>,
        }
    });
    quote! {
        #[derive(Clone, Debug)]
        pub(crate) struct #name {
            #(#property_fields,)*
            #event_fields
            #grid_definition_fields
        }

        impl PartialEq for #name {
            fn eq(&self, #other: &Self) -> bool {
                true #(&& #comparisons)*
            }
        }
    }
}

fn has_reference(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::Focus)
        || control.capabilities.contains(&Capability::Reference)
}

fn has_window_title_bar(control: &ResolvedControl) -> bool {
    control.capabilities.contains(&Capability::WindowTitleBar)
}

fn generate_element_parts(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let props = ident(&format!("{}MountedProps", control.name));
    let (reference_pattern, reference_field) = if has_reference(control) {
        (quote! { reference, }, quote! { reference, })
    } else {
        (TokenStream::new(), quote! { reference: None, })
    };
    let event_fields = if has_grouped_events(control) {
        vec![ident("events")]
    } else {
        control
            .events
            .iter()
            .map(|event| ident(&event.field))
            .collect()
    };
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field))
        .chain(event_fields)
        .chain(
            has_grid_definitions(control)
                .then_some(["rows", "columns"])
                .into_iter()
                .flatten()
                .map(ident),
        )
        .collect::<Vec<_>>();
    let (element_state_pattern, element_state_field) = if has_element_state(control) {
        (quote! { element_state, }, quote! { element_state, })
    } else {
        (TokenStream::new(), quote! { element_state: None, })
    };
    let (window_title_bar_pattern, window_title_bar_field) = if has_window_title_bar(control) {
        (
            quote! { preferred_height, },
            quote! { window_title_bar: Some(preferred_height), },
        )
    } else {
        (TokenStream::new(), quote! { window_title_bar: None, })
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
        Role::Leaf | Role::Slots => (TokenStream::new(), quote! { ElementStructure::None }),
    };
    let lifecycle_pattern =
        (control.lifecycle == Some(Lifecycle::ContentDialog)).then(|| quote! { , is_open: _ });

    quote! {
        Self::#name(value) => {
            let value = std::rc::Rc::unwrap_or_clone(value);
            let #name {
                #(#fields,)*
                #reference_pattern
                #element_state_pattern
                #window_title_bar_pattern
                #structural_pattern
                #lifecycle_pattern
            } = value;
            ElementParts {
                kind: MountedKind::#name,
                props: MountedProps::#name(std::rc::Rc::new(#props {
                    #(#fields),*
                })),
                #reference_field
                #element_state_field
                #window_title_bar_field
                structure: #structure,
            }
        }
    }
}

fn generate_element_props_match(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| (property.field.as_str(), Some(property.value.as_str())))
        .chain(if has_grouped_events(control) {
            vec![("events", None)]
        } else {
            control
                .events
                .iter()
                .map(|event| (event.field.as_str(), None))
                .collect()
        })
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
    let comparisons = fields.iter().map(|(field, value)| {
        let field = ident(field);
        value_equality(*value, &quote! { value.#field }, &quote! { mounted.#field })
    });
    let mounted_pattern = if fields.is_empty() {
        quote! { _ }
    } else {
        quote! { mounted }
    };
    let value_pattern = if fields.is_empty() {
        quote! { _ }
    } else {
        quote! { value }
    };

    quote! {
        (
            Self::#name(#value_pattern),
            MountedProps::#name(#mounted_pattern),
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
            quote! { Self::#name(value) => ElementStructureRef::Virtual(&value.items) }
        }
        Role::Leaf | Role::Slots => {
            quote! { Self::#name(_) => ElementStructureRef::None }
        }
    }
}

fn generate_element_event_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    if has_grouped_events(control) {
        let uses_events = control
            .events
            .iter()
            .any(|event| !control.event_always_active(event));
        let events = control.events.iter().map(|event| {
            let field = ident(&event.field);
            let id = ident(&format!("{}{}", control.name, event.name));
            if control.event_always_active(event) {
                quote! { visit(EventId::#id, true); }
            } else {
                let active_property = event.active_property.as_ref().map(|property| {
                    let property = ident(property);
                    quote! { || !matches!(value.#property, Property::Inherited) }
                });
                quote! {
                    visit(
                        EventId::#id,
                        value
                            .events
                            .as_ref()
                            .is_some_and(|events| events.#field.is_some())
                            #active_property,
                    );
                }
            }
        });
        return if uses_events {
            quote! {
                Self::#name(value) => {
                    #(#events)*
                }
            }
        } else {
            quote! {
                Self::#name(_) => {
                    #(#events)*
                }
            }
        };
    }
    let events = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let id = ident(&format!("{}{}", control.name, event.name));
        if control.event_always_active(event) {
            quote! { visit(EventId::#id, true); }
        } else {
            quote! { visit(EventId::#id, value.#field.is_some()); }
        }
    });
    let uses_value = control
        .events
        .iter()
        .any(|event| !control.event_always_active(event));
    let pattern = if uses_value {
        quote! { Self::#name(value) }
    } else {
        quote! { Self::#name(_) }
    };
    quote! {
        #pattern => {
            #(#events)*
        }
    }
}

fn generate_mounted_props_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let uses_values = !control.properties.is_empty() || has_grid_definitions(control);
    let values = if uses_values {
        quote! { values }
    } else {
        quote! { _ }
    };
    let properties = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let id = ident(&format!("{}{}", control.name, property.name));
        if property.adapter == Some(PropertyAdapter::ImageUri) {
            return quote! {
                visit(
                    PropertyId::#id,
                    match &values.#field {
                        Property::Inherited => None,
                        Property::Set(ImageValue::Uri(value)) => {
                            Some(PropertyValueRef::Str(value.as_str()))
                        }
                        Property::Set(ImageValue::Encoded(value)) => {
                            Some(PropertyValueRef::EncodedImage(value))
                        }
                    },
                );
            };
        }
        let variant = ident(&property.value);
        if property.theme_style {
            return quote! {
                visit(
                    PropertyId::#id,
                    match &values.#field {
                        Property::Set(Brush::Solid(color)) => {
                            Some(PropertyValueRef::#variant(Brush::Solid(*color)))
                        }
                        Property::Inherited | Property::Set(Brush::Theme(_)) => None,
                    },
                );
            };
        }
        let value = match property.value.as_str() {
            "Str" => quote! { value.as_str() },
            "KeyAccelerators" | "ResourceOverrides" | "RichText" | "StrList" | "Thickness"
            | "CornerRadius" | "DragDropPolicy" => {
                quote! { value }
            }
            _ => quote! { *value },
        };
        quote! {
            visit(
                PropertyId::#id,
                match &values.#field {
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
                values.rows.as_set().map(PropertyValueRef::GridLengths),
            );
            visit(
                PropertyId::GridColumns,
                values
                    .columns
                    .as_set()
                    .map(PropertyValueRef::GridLengths),
            );
        }
    });

    quote! {
        Self::#name(#values) => {
            #(#properties)*
            #grid_definitions
        }
    }
}

fn generate_mounted_theme_style(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let properties = control
        .properties
        .iter()
        .filter(|property| property.theme_style)
        .collect::<Vec<_>>();
    if properties.is_empty() {
        return quote! { Self::#name(_) => ThemeStyle::default() };
    }
    let fields = properties
        .iter()
        .map(|property| ident(&property.field))
        .collect::<Vec<_>>();
    let mut values = fields
        .iter()
        .map(|field| {
            quote! {
                values
                    .#field
                    .as_set()
                    .copied()
                    .and_then(Brush::theme)
            }
        })
        .collect::<Vec<_>>();
    values.resize(4, quote! { None });
    quote! {
        Self::#name(values) => ThemeStyle::new([#(#values),*])
    }
}

fn generate_mounted_event_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    if has_grouped_events(control) {
        let uses_events = control
            .events
            .iter()
            .any(|event| !control.event_always_active(event));
        let events = control.events.iter().map(|event| {
            let field = ident(&event.field);
            let id = ident(&format!("{}{}", control.name, event.name));
            if control.event_always_active(event) {
                quote! { visit(EventId::#id, true); }
            } else {
                let active_property = event.active_property.as_ref().map(|property| {
                    let property = ident(property);
                    quote! { || !matches!(values.#property, Property::Inherited) }
                });
                quote! {
                    visit(
                        EventId::#id,
                        values
                            .events
                            .as_ref()
                            .is_some_and(|events| events.#field.is_some())
                            #active_property,
                    );
                }
            }
        });
        return if uses_events {
            quote! {
                Self::#name(values) => {
                    #(#events)*
                }
            }
        } else {
            quote! {
                Self::#name(_) => {
                    #(#events)*
                }
            }
        };
    }
    let events = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let id = ident(&format!("{}{}", control.name, event.name));
        if control.event_always_active(event) {
            quote! { visit(EventId::#id, true); }
        } else {
            quote! { visit(EventId::#id, values.#field.is_some()); }
        }
    });
    let uses_values = control
        .events
        .iter()
        .any(|event| !control.event_always_active(event));
    let values = if uses_values {
        quote! { values }
    } else {
        quote! { _ }
    };
    quote! {
        Self::#name(#values) => {
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
            } else if event.conversion == EventPayloadConversion::Selection {
                quote! { callback.call(value.tag.clone()) }
            } else if event.payload == "StrList" {
                quote! { callback.call(value.as_ref().clone()) }
            } else if matches!(event.payload.as_str(), "Str" | "DroppedData") {
                quote! { callback.call(value.clone()) }
            } else {
                quote! { callback.call(*value) }
            };
            let payload_pattern = if event.payload == "Unit" {
                quote! { EventPayload::#payload }
            } else {
                quote! { EventPayload::#payload(value) }
            };
            if has_grouped_events(control) {
                quote! {
                    (
                        Self::#name(values),
                        EventId::#id,
                        #payload_pattern,
                    ) => values
                        .events
                        .as_ref()
                        .and_then(|events| events.#field.as_ref())
                        .map(|callback| #call)
                }
            } else {
                quote! {
                    (
                        Self::#name(values),
                        EventId::#id,
                        #payload_pattern,
                    ) => values.#field.as_ref().map(|callback| #call)
                }
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
                    Self::#name(_),
                    EventId::#event_id,
                    EventPayload::#payload(value),
                ) => Some((PropertyId::#property_id, (#value).into()))
            })
        })
        .collect()
}

fn generate_property_values(schema: &ResolvedSchema) -> TokenStream {
    let mut values = BTreeMap::from([
        ("EncodedImage".to_string(), quote! { EncodedImage }),
        ("F64".to_string(), quote! { f64 }),
        (
            "GridLengths".to_string(),
            quote! { std::rc::Rc<Vec<GridLength>> },
        ),
        (
            "HorizontalAlignment".to_string(),
            quote! { HorizontalAlignment },
        ),
        ("I32".to_string(), quote! { i32 }),
        ("StrList".to_string(), quote! { std::rc::Rc<Vec<String>> }),
        ("Thickness".to_string(), quote! { Thickness }),
        (
            "VerticalAlignment".to_string(),
            quote! { VerticalAlignment },
        ),
    ]);
    for property in schema
        .controls
        .iter()
        .flat_map(|control| &control.properties)
        .filter(|property| property.adapter != Some(PropertyAdapter::ImageUri))
        .filter(|property| !property.theme_style || property.value == "Brush")
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
            "StrList" => quote! { &'a std::rc::Rc<Vec<String>> },
            "RichText" => quote! { &'a RichText },
            "ResourceOverrides" => quote! { &'a ResourceOverrides },
            "KeyAccelerators" => quote! { &'a KeyAccelerators },
            "DragDropPolicy" => quote! { &'a DragDropPolicy },
            "EncodedImage" => quote! { &'a EncodedImage },
            "Str" => quote! { &'a str },
            "Thickness" => quote! { &'a Thickness },
            "CornerRadius" => quote! { &'a CornerRadius },
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
        } else if name == "OptionalF64" {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => match (left, right) {
                    (Some(left), Some(right)) => f64_eq(*left, *right),
                    (None, None) => true,
                    _ => false,
                }
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
        } else if name == "OptionalF64" {
            quote! {
                (Self::#variant(left), PropertyValue::#variant(right)) => match (left, right) {
                    (Some(left), Some(right)) => f64_eq(left, *right),
                    (None, None) => true,
                    _ => false,
                }
            }
        } else if matches!(
            name.as_str(),
            "GridLengths"
                | "ResourceOverrides"
                | "KeyAccelerators"
                | "RichText"
                | "StrList"
                | "Str"
                | "Thickness"
                | "CornerRadius"
                | "DragDropPolicy"
                | "EncodedImage"
        ) {
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
            "GridLengths" | "StrList" => quote! {
                Self::#variant(value) => PropertyValue::#variant(value.clone())
            },
            "Str" => quote! {
                Self::#variant(value) => PropertyValue::#variant(value.to_string())
            },
            "KeyAccelerators" | "ResourceOverrides" | "RichText" | "Thickness" | "CornerRadius"
            | "DragDropPolicy" | "EncodedImage" => {
                quote! {
                    Self::#variant(value) => PropertyValue::#variant(value.clone())
                }
            }
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
    payloads.insert("String".to_string(), value_type("String"));
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
        } else if name == "OptionalF64" {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => match (left, right) {
                    (Some(left), Some(right)) => f64_eq(*left, *right),
                    (None, None) => true,
                    _ => false,
                }
            }
        } else {
            quote! {
                (Self::#variant(left), Self::#variant(right)) => left == right
            }
        }
    });
    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub struct SelectionChange {
            pub item: Option<crate::core::NodeId>,
            pub tag: Option<String>,
        }

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
            #[non_exhaustive]
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum #name {
                #(#variants),*
            }
        }
    });
    let content_dialog_result = schema
        .controls
        .iter()
        .any(|control| control.lifecycle == Some(Lifecycle::ContentDialog))
        .then(|| {
            quote! {
                #[non_exhaustive]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub enum ContentDialogResult {
                    #[default]
                    None,
                    Primary,
                    Secondary,
                }
            }
        });
    quote! {
        #content_dialog_result
        #(#enums)*
    }
}

fn generate_element(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let visibility = if control.placement == ResolvedPlacement::TooltipAttachment {
        quote! { pub(crate) }
    } else {
        quote! { pub }
    };
    let (reference_field, reference_method, reference_impls) = if has_reference(control) {
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
                impl crate::reference::ReferenceControl for #name {}
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
    let event_fields = if has_grouped_events(control) {
        let events = ident(&format!("{}Events", control.name));
        quote! { events: Option<std::rc::Rc<#events>>, }
    } else {
        let fields = control.events.iter().map(|event| {
            let field = ident(&event.field);
            let payload = event_callback_type(event);
            quote! { #field: Option<Callback<#payload>>, }
        });
        quote! { #(#fields)* }
    };
    let element_state_field = has_element_state(control)
        .then(|| quote! { element_state: Option<std::rc::Rc<ElementState>>, });
    let window_title_bar_field =
        has_window_title_bar(control).then(|| quote! { preferred_height: WindowTitleBarHeight, });
    let window_title_bar_method = has_window_title_bar(control).then(|| {
        quote! {
            pub fn preferred_height(mut self, height: WindowTitleBarHeight) -> Self {
                self.preferred_height = height;
                self
            }
        }
    });
    let grid_definition_fields = has_grid_definitions(control).then(|| {
        quote! {
            rows: Property<std::rc::Rc<Vec<GridLength>>>,
            columns: Property<std::rc::Rc<Vec<GridLength>>>,
        }
    });
    let structural_field = match control.role {
        Role::Content => quote! { content: Option<Box<Element>> },
        Role::Children => quote! { children: std::rc::Rc<Vec<KeyedElement>> },
        Role::Virtual => quote! { items: VirtualItems },
        Role::Leaf | Role::Slots => TokenStream::new(),
    };
    let lifecycle_field =
        (control.lifecycle == Some(Lifecycle::ContentDialog)).then(|| quote! { , is_open: bool });
    let lifecycle_method = (control.lifecycle == Some(Lifecycle::ContentDialog)).then(|| {
        quote! {
            pub fn is_open(mut self, value: bool) -> Self {
                self.is_open = value;
                self
            }
        }
    });
    let property_methods = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let value = value_type(&property.value);
        let validation = generate_value_validation(control, property);
        if property.theme_style {
            let optional = ident(&format!("{}_optional", property.field));
            quote! {
                pub fn #field(mut self, value: impl Into<Brush>) -> Self {
                    self.#field = Property::Set(value.into());
                    self
                }

                pub fn #optional<T>(mut self, value: Option<T>) -> Self
                where
                    T: Into<Brush>,
                {
                    self.#field = Property::from(value.map(Into::into));
                    self
                }
            }
        } else if property.adapter == Some(PropertyAdapter::ImageUri) {
            let optional = ident(&format!("{}_optional", property.field));
            let file = ident(&format!("{}_file", property.field));
            let data = ident(&format!("{}_data", property.field));
            quote! {
                pub fn #field(
                    mut self,
                    value: impl Into<String>,
                ) -> windows_core::Result<Self> {
                    let value = value.into();
                    validate_image_uri(&value)?;
                    self.#field = Property::Set(ImageValue::Uri(value));
                    Ok(self)
                }

                pub fn #optional<T>(
                    mut self,
                    value: Option<T>,
                ) -> windows_core::Result<Self>
                where
                    T: Into<String>,
                {
                    self.#field = match value {
                        Some(value) => {
                            let value = value.into();
                            validate_image_uri(&value)?;
                            Property::Set(ImageValue::Uri(value))
                        }
                        None => Property::Inherited,
                    };
                    Ok(self)
                }

                pub fn #file(
                    self,
                    value: impl AsRef<std::path::Path>,
                ) -> windows_core::Result<Self> {
                    self.#field(file_uri(value.as_ref())?)
                }

                pub fn #data(mut self, value: EncodedImage) -> Self {
                    self.#field = Property::Set(ImageValue::Encoded(value));
                    self
                }
            }
        } else if property.adapter == Some(PropertyAdapter::Uri) {
            let optional = ident(&format!("{}_optional", property.field));
            quote! {
                pub fn #field(
                    mut self,
                    value: impl Into<String>,
                ) -> windows_core::Result<Self> {
                    let value = value.into();
                    validate_uri(&value)?;
                    self.#field = Property::Set(value);
                    Ok(self)
                }

                pub fn #optional<T>(
                    mut self,
                    value: Option<T>,
                ) -> windows_core::Result<Self>
                where
                    T: Into<String>,
                {
                    self.#field = match value {
                        Some(value) => {
                            let value = value.into();
                            validate_uri(&value)?;
                            Property::Set(value)
                        }
                        None => Property::Inherited,
                    };
                    Ok(self)
                }
            }
        } else if property.adapter == Some(PropertyAdapter::InspectableStringList) {
            let optional = ident(&format!("{}_optional", property.field));
            quote! {
                pub fn #field<I, S>(mut self, value: I) -> Self
                where
                    I: IntoIterator<Item = S>,
                    S: Into<String>,
                {
                    self.#field =
                        Property::Set(std::rc::Rc::new(value.into_iter().map(Into::into).collect()));
                    self
                }

                pub fn #optional<I, S>(mut self, value: Option<I>) -> Self
                where
                    I: IntoIterator<Item = S>,
                    S: Into<String>,
                {
                    self.#field = Property::from(value.map(|value| {
                        std::rc::Rc::new(value.into_iter().map(Into::into).collect())
                    }));
                    self
                }
            }
        } else if property.value == "Str" {
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
        } else if property.adapter == Some(PropertyAdapter::NumberBoxValue) {
            quote! {
                pub fn #field(mut self, value: impl Into<#value>) -> Self {
                    let value = value.into().filter(|value| !value.is_nan());
                    #validation
                    self.#field = Property::Set(value);
                    self
                }
            }
        } else if property.adapter == Some(PropertyAdapter::RatingValue) {
            quote! {
                pub fn #field(mut self, value: impl Into<#value>) -> Self {
                    let value = value.into().filter(|value| *value != -1.0);
                    #validation
                    self.#field = Property::Set(value);
                    self
                }
            }
        } else if property.adapter == Some(PropertyAdapter::SelectionIndex) {
            quote! {
                pub fn #field(mut self, value: impl Into<#value>) -> Self {
                    let value = value.into();
                    #validation
                    self.#field = Property::Set(value);
                    self
                }
            }
        } else if matches!(property.value.as_str(), "Thickness" | "CornerRadius") {
            let optional = ident(&format!("{}_optional", property.field));
            let direct_validation = generate_direct_value_validation(control, property);
            quote! {
                pub fn #field(mut self, value: impl Into<#value>) -> Self {
                    let value = value.into();
                    #direct_validation
                    self.#field = Property::Set(value);
                    self
                }

                pub fn #optional<T>(mut self, value: Option<T>) -> Self
                where
                    T: Into<#value>,
                {
                    let value = value.map(Into::into);
                    #validation
                    self.#field = Property::from(value);
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, value: impl Into<Option<#value>>) -> Self {
                    let value = value.into();
                    #validation
                    self.#field = Property::from(value);
                    self
                }
            }
        }
    });
    let event_methods = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let payload = event_callback_type(event);
        let assignment = if has_grouped_events(control) {
            quote! {
                std::rc::Rc::make_mut(
                    self.events
                        .get_or_insert_with(|| std::rc::Rc::new(Default::default())),
                )
                .#field
            }
        } else {
            quote! { self.#field }
        };
        if event.payload == "Unit" {
            quote! {
                pub fn #field(mut self, callback: impl IntoUnitCallback) -> Self {
                    #assignment = Some(callback.into_unit_callback());
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, callback: impl IntoPayloadCallback<#payload>) -> Self {
                    #assignment = Some(callback.into_payload_callback());
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
    let (structural_methods, structural_test_impl) = match control.role {
        Role::Content => (
            TokenStream::new(),
            quote! {
                #[cfg(test)]
                impl NativeContentTestExt for #name {
                    fn native_content(mut self, content: impl Into<Element>) -> Self {
                        self.content = Some(Box::new(content.into()));
                        self
                    }
                }
            },
        ),
        Role::Children => (
            TokenStream::new(),
            quote! {
                #[cfg(test)]
                impl NativeChildrenTestExt for #name {
                    fn native_child(
                        mut self,
                        key: impl Into<Key>,
                        child: impl Into<Element>,
                    ) -> Self {
                        std::rc::Rc::make_mut(&mut self.children)
                            .push(KeyedElement::new(key, child));
                        self
                    }

                    fn native_children(
                        mut self,
                        children: impl IntoIterator<Item = KeyedElement>,
                    ) -> Self {
                        self.children = std::rc::Rc::new(children.into_iter().collect());
                        self
                    }
                }
            },
        ),
        Role::Virtual => (
            quote! {
                pub fn item(mut self, key: impl Into<Key>, item: impl Into<View>) -> Self {
                    if let VirtualItems::Eager(items) = &mut self.items {
                        std::rc::Rc::make_mut(items).push(KeyedView::new(key, item));
                    } else {
                        self.items = VirtualItems::Eager(std::rc::Rc::new(vec![
                            KeyedView::new(key, item)
                        ]));
                    }
                    self
                }

                pub fn items<T>(
                    mut self,
                    items: impl IntoIterator<Item = T>,
                ) -> Self
                where
                    T: Into<KeyedView>,
                {
                    self.items = VirtualItems::Eager(std::rc::Rc::new(
                        items.into_iter().map(Into::into).collect()
                    ));
                    self
                }

                /// Uses an indexed source that constructs views only for realized items.
                pub fn virtual_source(mut self, source: VirtualSource) -> Self {
                    self.items = VirtualItems::Lazy(source);
                    self
                }
            },
            TokenStream::new(),
        ),
        Role::Leaf | Role::Slots => (TokenStream::new(), TokenStream::new()),
    };
    let capability_impls = control.capabilities.iter().filter_map(|capability| {
        if *capability == Capability::Layout && has_element_state(control) {
            return Some(quote! {
                impl sealed::LayoutControl for #name {
                    fn element_state_mut(
                        &mut self,
                    ) -> &mut Option<std::rc::Rc<ElementState>> {
                        &mut self.element_state
                    }
                }

                impl LayoutControl for #name {}
            });
        }
        let content_capability = *capability == Capability::Content;
        let capability = match capability {
            Capability::Content => "ContentControl",
            Capability::Children => "ChildrenControl",
            Capability::Focus => "FocusControl",
            Capability::Layout
            | Capability::TextStyle
            | Capability::Enabled
            | Capability::ControlledText
            | Capability::Items
            | Capability::Reference
            | Capability::GridDefinitions
            | Capability::WindowTitleBar => return None,
        };
        let capability = ident(capability);
        if content_capability && control.lifecycle == Some(Lifecycle::ContentDialog) {
            Some(quote! {
                impl sealed::ContentControl for #name {
                    fn into_content_view(self, content: View) -> View {
                        let open = self.is_open;
                        View::content_dialog(self.into(), Some(content), open)
                    }
                }

                impl ContentControl for #name {}
            })
        } else if content_capability {
            Some(quote! {
                impl sealed::ContentControl for #name {}
                impl ContentControl for #name {}
            })
        } else {
            Some(quote! { impl #capability for #name {} })
        }
    });
    let slots = if control.slots.is_empty() {
        TokenStream::new()
    } else {
        let slot_name = ident(&format!("{}Slot", control.name));
        let variants = control.slots.iter().map(|slot| ident(&slot.name));
        quote! {
            #[non_exhaustive]
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub enum #slot_name {
                #(#variants),*
            }

            impl sealed::SlotIndex<#slot_name> for #name {
                fn slot_index(slot: #slot_name) -> u8 {
                    slot as u8
                }
            }

            impl SlotsControl for #name {
                type Slot = #slot_name;
            }
        }
    };
    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        #visibility struct #name {
            #(#property_fields,)*
            #event_fields
            #reference_field
            #element_state_field
            #window_title_bar_field
            #grid_definition_fields
            #structural_field
            #lifecycle_field
        }

        impl #name {
            #visibility fn new() -> Self {
                Self::default()
            }

            #reference_method
            #lifecycle_method
            #window_title_bar_method

            #(#property_methods)*
            #(#event_methods)*
            #grid_definition_methods
            #structural_methods
        }

        impl sealed::Sealed for #name {}
        impl sealed::NativeControl for #name {
            fn into_element(self) -> Element {
                self.into()
            }
        }
        #reference_impls
        #(#capability_impls)*
        #slots
        #structural_test_impl
    }
}

fn generate_direct_value_validation(
    control: &ResolvedControl,
    property: &crate::schema::ResolvedProperty,
) -> TokenStream {
    let Some(validation) = property.validation else {
        return TokenStream::new();
    };
    let message = value_validation_message(control, property, validation);
    match (validation, property.value.as_str()) {
        (ValueValidation::Finite, "Thickness") => quote! {
            assert!(value.is_finite(), #message);
        },
        (ValueValidation::FiniteNonNegative, "Thickness" | "CornerRadius") => quote! {
            assert!(value.is_finite_non_negative(), #message);
        },
        _ => unreachable!(),
    }
}

fn generate_value_validation(
    control: &ResolvedControl,
    property: &crate::schema::ResolvedProperty,
) -> TokenStream {
    let Some(validation) = property.validation else {
        return TokenStream::new();
    };
    let message = value_validation_message(control, property, validation);
    match (validation, property.value.as_str()) {
        (ValueValidation::Finite, "F64") => quote! {
            assert!(
                value.as_ref().is_none_or(|value| value.is_finite()),
                #message,
            );
        },
        (ValueValidation::Finite, "Thickness") => quote! {
            assert!(
                value.as_ref().is_none_or(|value| value.is_finite()),
                #message,
            );
        },
        (ValueValidation::FinitePositive, "F64") => quote! {
            assert!(
                value
                    .as_ref()
                    .is_none_or(|value| value.is_finite() && *value > 0.0),
                #message,
            );
        },
        (ValueValidation::NonNegative, "I32") => quote! {
            assert!(value.as_ref().is_none_or(|value| *value >= 0), #message);
        },
        (ValueValidation::ZeroToFiftyNine, "I32") => quote! {
            assert!(
                value
                    .as_ref()
                    .is_none_or(|value| (0..=59).contains(value)),
                #message,
            );
        },
        (ValueValidation::FiniteNonNegative, "F64") => quote! {
            assert!(
                value
                    .as_ref()
                    .is_none_or(|value| value.is_finite() && *value >= 0.0),
                #message,
            );
        },
        (ValueValidation::FiniteNonNegative, "Thickness" | "CornerRadius") => quote! {
            assert!(
                value
                    .as_ref()
                    .is_none_or(|value| value.is_finite_non_negative()),
                #message,
            );
        },
        _ => unreachable!(),
    }
}

fn value_validation_message(
    control: &ResolvedControl,
    property: &crate::schema::ResolvedProperty,
    validation: ValueValidation,
) -> String {
    match validation {
        ValueValidation::Finite => {
            format!(
                "{} {} must contain finite values",
                control.name, property.name
            )
        }
        ValueValidation::FiniteNonNegative => format!(
            "{} {} must contain finite non-negative values",
            control.name, property.name
        ),
        ValueValidation::FinitePositive => {
            format!(
                "{} {} must be finite and positive",
                control.name, property.name
            )
        }
        ValueValidation::NonNegative => {
            format!("{} {} must be non-negative", control.name, property.name)
        }
        ValueValidation::ZeroToFiftyNine => {
            format!(
                "{} {} must be between 0 and 59",
                control.name, property.name
            )
        }
    }
}

fn value_equality(
    value: Option<&str>,
    left: &impl quote::ToTokens,
    right: &impl quote::ToTokens,
) -> TokenStream {
    if value == Some("F64") {
        quote! { f64_property_eq(&#left, &#right) }
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
                clearable: true,
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
        let (target, collection) = match &slot.shape {
            crate::schema::SlotShape::Single(crate::schema::SlotTarget::Inspectable) => {
                ("inspectable", false)
            }
            crate::schema::SlotShape::Single(crate::schema::SlotTarget::IconElement) => {
                ("icon_element", false)
            }
            crate::schema::SlotShape::Single(crate::schema::SlotTarget::UiElement) => {
                ("ui_element", false)
            }
            crate::schema::SlotShape::Collection(_) => ("inspectable", true),
        };
        quote! {
            SlotDescriptor {
                id: SlotId::#id,
                name: #name,
                interface: #interface,
                target: #target,
                collection: #collection,
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
                Capability::Reference => "Reference",
                Capability::GridDefinitions => "GridDefinitions",
                Capability::WindowTitleBar => "WindowTitleBar",
            },
            Span::call_site(),
        );
        quote! { Capability::#capability }
    });
    let properties = descriptor_ident(name, "PROPERTIES");
    let events = descriptor_ident(name, "EVENTS");
    let slots = descriptor_ident(name, "SLOTS");
    let selection = control.selection.as_ref().map_or_else(
        || quote! { None },
        |selection| {
            let slot = ident(&format!("{}{}", control.name, selection.slot));
            let item = ident(&selection.item);
            let selected_property = ident(&format!(
                "{}{}",
                selection.item, selection.selected_property
            ));
            let event = ident(&format!("{}{}", control.name, selection.event));
            let payload_property =
                ident(&format!("{}{}", selection.item, selection.payload_property));
            quote! {
                Some(SelectionDescriptor {
                    slot: SlotId::#slot,
                    item: MountedKind::#item,
                    selected_property: PropertyId::#selected_property,
                    event: EventId::#event,
                    payload_property: PropertyId::#payload_property,
                })
            }
        },
    );
    let collection_slots = control
        .slots
        .iter()
        .filter(|slot| matches!(&slot.shape, crate::schema::SlotShape::Collection(_)))
        .collect::<Vec<_>>();
    let controlled_indices = control
        .properties
        .iter()
        .filter(|property| property.value == "SelectionIndex" && property.feedback.is_some())
        .collect::<Vec<_>>();
    let controlled_collection = if collection_slots.len() == 1 && controlled_indices.len() == 1 {
        let slot = ident(&format!("{}{}", control.name, collection_slots[0].name));
        let property = ident(&format!("{}{}", control.name, controlled_indices[0].name));
        let event = ident(&format!(
            "{}{}",
            control.name,
            controlled_indices[0].feedback.as_ref().unwrap()
        ));
        quote! {
            Some(ControlledCollectionDescriptor {
                slot: SlotId::#slot,
                property: PropertyId::#property,
                event: EventId::#event,
            })
        }
    } else {
        quote! { None }
    };

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
            selection: #selection,
            controlled_collection: #controlled_collection,
        }
    }
}

fn descriptor_ident(control: &str, suffix: &str) -> Ident {
    let name = crate::helpers::to_snake_case(control).to_ascii_uppercase();
    Ident::new(&format!("{name}_{suffix}"), Span::call_site())
}

fn ident(value: &str) -> Ident {
    Ident::new(value, Span::call_site())
}

fn value_type(value: &str) -> TokenStream {
    match value {
        "Unit" => quote! { () },
        "Str" => quote! { String },
        "StrList" => quote! { std::rc::Rc<Vec<String>> },
        "SelectionChange" => quote! { SelectionChange },
        "DateTime" => quote! { windows_time::DateTime },
        "OptionalDateTime" => quote! { Option<windows_time::DateTime> },
        "Duration" => quote! { std::time::Duration },
        "OptionalF64" => quote! { Option<f64> },
        "SelectionIndex" => quote! { Option<usize> },
        "TimeSpan" => quote! { windows_time::TimeSpan },
        "OptionalTimeSpan" => quote! { Option<windows_time::TimeSpan> },
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

fn event_callback_type(event: &crate::schema::ResolvedEvent) -> TokenStream {
    if event.conversion == EventPayloadConversion::Selection {
        quote! { Option<String> }
    } else if event.payload == "StrList" {
        quote! { Vec<String> }
    } else {
        value_type(&event.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::MetadataResolver;
    use crate::schema::{Schema, workspace_path};

    #[test]
    fn ordinary_control_needs_only_schema_input() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.ProgressBar"
capabilities = ["layout"]

[[control.property]]
name = "Value"
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

    #[test]
    fn semantic_absence_is_exposed_as_option() {
        let source =
            std::fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(&source).unwrap().resolve(&metadata).unwrap();
        let output: String = generate(&resolved)
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();

        assert!(output.contains("pubfnselected_index(mutself,value:implInto<Option<usize>>"));
        assert!(output.contains("callback:implIntoPayloadCallback<Option<usize>>"));
        assert!(output.contains("pubfnvalue(mutself,value:implInto<Option<f64>>"));
        assert!(
            output.contains("callback:implIntoPayloadCallback<Option<windows_time::DateTime>>")
        );
        assert!(
            output.contains("callback:implIntoPayloadCallback<Option<windows_time::TimeSpan>>")
        );
        assert!(!output.contains("selected_index_optional"));
    }

    #[test]
    fn attachment_implementation_control_is_not_public() {
        let source =
            std::fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(&source).unwrap().resolve(&metadata).unwrap();
        let output: String = generate(&resolved)
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();

        assert!(output.contains("pub(crate)structToolTip"));
        assert!(output.contains("pub(crate)fnnew()->Self"));
        assert!(!output.contains("pubstructToolTip"));
    }

    #[test]
    fn constrained_properties_generate_checked_setters() {
        let source =
            std::fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(&source).unwrap().resolve(&metadata).unwrap();
        let output: String = generate(&resolved)
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();

        assert!(output.contains(
            "value.as_ref().is_none_or(|value|*value>=0),\"TextBlockMaxLinesmustbenon-negative\""
        ));
        assert!(output.contains(
            "value.as_ref().is_none_or(|value|(0..=59).contains(value)),\"TimePickerMinuteIncrementmustbebetween0and59\""
        ));
        assert!(output.contains("pubfnfont_weight(mutself,value:implInto<Option<FontWeight>>"));
        assert!(output.contains(
            "pubfnhorizontal_content_alignment(mutself,value:implInto<Option<HorizontalAlignment>>"
        ));
        assert!(output.contains(
            "pubfnvertical_content_alignment(mutself,value:implInto<Option<VerticalAlignment>>"
        ));
        assert!(!output.contains("pubenumHorizontalAlignment"));
        assert!(!output.contains("pubenumVerticalAlignment"));
    }

    #[test]
    fn generated_public_enums_are_non_exhaustive() {
        let source =
            std::fs::read_to_string(workspace_path("crates/tools/reactor/src/winui.toml")).unwrap();
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let resolved = Schema::parse(&source).unwrap().resolve(&metadata).unwrap();
        let output = generate(&resolved);

        for name in ["Orientation", "ContentDialogResult", "NavigationViewSlot"] {
            let enum_start = output.find(&format!("pub enum {name}")).unwrap();
            let attributes = &output[enum_start.saturating_sub(160)..enum_start];
            assert!(
                attributes.contains("# [non_exhaustive]"),
                "{name} must be non-exhaustive"
            );
        }
    }
}
