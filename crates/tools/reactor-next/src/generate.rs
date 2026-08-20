use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

use crate::schema::{Capability, ResolvedControl, ResolvedSchema, Role};

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
        }
    });
    let mounted_variants = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name }
    });
    let mounted_props_variants = schema.controls.iter().map(generate_mounted_props_variant);
    let mounted_props_visitors = schema.controls.iter().map(generate_mounted_props_visitor);
    let mounted_event_visitors = schema.controls.iter().map(generate_mounted_event_visitor);
    let mounted_event_dispatchers = schema.controls.iter().flat_map(generate_event_dispatchers);
    let mounted_event_observers = schema.controls.iter().flat_map(generate_event_observers);
    let element_parts = schema.controls.iter().map(generate_element_parts);
    let element_props_matches = schema.controls.iter().map(generate_element_props_match);
    let element_structures = schema.controls.iter().map(generate_element_structure);
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
    let property_values = generate_property_values(schema);
    let event_payloads = generate_event_payloads(schema);
    let descriptors = schema.controls.iter().map(generate_descriptors);
    let controls = schema.controls.iter().map(generate_control);

    let tokens = quote! {
        use crate::element::*;

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

                fn structure(&self) -> ElementStructureRef<'_> {
                    match self {
                        #(#element_structures),*
                    }
                }
            }
        }

        use public::*;

        pub trait ElementPartsExt {
            fn kind(&self) -> MountedKind;
            fn into_parts(self) -> ElementParts;
            fn props_match(&self, props: &MountedProps) -> bool;
            fn structure(&self) -> ElementStructureRef<'_>;
        }

        pub trait MountedPropsExt {
            fn visit_properties(
                &self,
                visit: &mut dyn FnMut(PropertyId, Option<PropertyValue>),
            );
        }

        pub trait MountedEventsExt {
            fn visit_events(&self, visit: &mut dyn FnMut(EventId, bool));
            fn dispatch_event(&self, event: EventId, payload: &EventPayload) -> bool;
            fn observe_event(
                &self,
                event: EventId,
                payload: &EventPayload,
            ) -> Option<(PropertyId, PropertyValue)>;
        }

        impl MountedPropsExt for MountedProps {
            fn visit_properties(
                &self,
                visit: &mut dyn FnMut(PropertyId, Option<PropertyValue>),
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

            fn dispatch_event(&self, event: EventId, payload: &EventPayload) -> bool {
                match (self, event, payload) {
                    #(#mounted_event_dispatchers,)*
                    _ => false,
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

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum MountedKind {
            #(#mounted_variants),*
        }

        #[derive(Clone, Debug, PartialEq)]
        pub enum MountedProps {
            #(#mounted_props_variants),*
        }

        #[derive(Clone, Debug, PartialEq)]
        pub enum ElementStructure {
            None,
            Content(Option<Element>),
            Children(std::rc::Rc<Vec<KeyedElement>>),
            Virtual(std::rc::Rc<Vec<KeyedElement>>),
        }

        #[derive(Clone, Copy)]
        pub enum ElementStructureRef<'a> {
            None,
            Content(Option<&'a Element>),
            Children(&'a [KeyedElement]),
            Virtual(&'a [KeyedElement]),
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct ElementParts {
            pub kind: MountedKind,
            pub props: MountedProps,
            pub structure: ElementStructure,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum PropertyId {
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
        pub struct ControlDescriptor {
            pub kind: MountedKind,
            pub name: &'static str,
            pub type_name: &'static str,
            pub role: ControlRole,
            pub capabilities: &'static [Capability],
            pub properties: &'static [PropertyDescriptor],
            pub events: &'static [EventDescriptor],
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
                control_index += 1;
            }
        };
    };

    format!("// Generated by `tool_reactor_next`. Do not edit.\n\n{tokens}\n")
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
    quote! {
        #name {
            #(#property_fields,)*
            #(#event_fields,)*
        }
    }
}

fn generate_element_parts(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field))
        .chain(control.events.iter().map(|event| ident(&event.field)))
        .collect::<Vec<_>>();
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
        Role::Leaf | Role::Controlled => (TokenStream::new(), quote! { ElementStructure::None }),
    };

    quote! {
        Self::#name(#name {
            #(#fields,)*
            #structural_pattern
        }) => ElementParts {
            kind: MountedKind::#name,
            props: MountedProps::#name {
                #(#fields),*
            },
            structure: #structure,
        }
    }
}

fn generate_element_props_match(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field))
        .chain(control.events.iter().map(|event| ident(&event.field)))
        .collect::<Vec<_>>();
    let mounted = fields
        .iter()
        .map(|field| ident(&format!("mounted_{field}")))
        .collect::<Vec<_>>();
    let comparisons = fields
        .iter()
        .zip(&mounted)
        .map(|(field, mounted)| quote! { #field == #mounted });

    quote! {
        (
            Self::#name(#name { #(#fields,)* .. }),
            MountedProps::#name { #(#fields: #mounted),* },
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
        Role::Leaf | Role::Controlled => quote! { Self::#name(_) => ElementStructureRef::None },
    }
}

fn generate_mounted_props_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control
        .properties
        .iter()
        .map(|property| ident(&property.field));
    let properties = control.properties.iter().map(|property| {
        let field = ident(&property.field);
        let id = ident(&format!("{}{}", control.name, property.name));
        let value = if property.value == "Str" {
            quote! { value.clone() }
        } else {
            quote! { *value }
        };
        quote! {
            visit(
                PropertyId::#id,
                match #field {
                    Property::Inherited => None,
                    Property::Set(value) => Some((#value).into()),
                },
            );
        }
    });

    quote! {
        Self::#name { #(#fields,)* .. } => {
            #(#properties)*
        }
    }
}

fn generate_mounted_event_visitor(control: &ResolvedControl) -> TokenStream {
    let name = ident(&control.name);
    let fields = control.events.iter().map(|event| ident(&event.field));
    let events = control.events.iter().map(|event| {
        let field = ident(&event.field);
        let id = ident(&format!("{}{}", control.name, event.name));
        quote! { visit(EventId::#id, #field.is_some()); }
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
            } else {
                quote! { callback.call(value.clone()) }
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
                ) => {
                    #call;
                    true
                }
            }
        })
        .collect()
}

fn generate_event_observers(control: &ResolvedControl) -> Vec<TokenStream> {
    let name = ident(&control.name);
    control
        .events
        .iter()
        .filter_map(|event| {
            let property = event.property.as_ref()?;
            let event_id = ident(&format!("{}{}", control.name, event.name));
            let property_id = ident(&format!("{}{}", control.name, property));
            let payload = ident(&event.payload);
            Some(quote! {
                (
                    Self::#name { .. },
                    EventId::#event_id,
                    EventPayload::#payload(value),
                ) => Some((PropertyId::#property_id, value.clone().into()))
            })
        })
        .collect()
}

fn generate_property_values(schema: &ResolvedSchema) -> TokenStream {
    let mut values = BTreeMap::new();
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

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum PropertyValue {
            #(#variants),*
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
    let variants = payloads.into_iter().map(|(name, value)| {
        let is_unit = name == "Unit";
        let name = ident(&name);
        if is_unit {
            quote! { #name }
        } else {
            quote! { #name(#value) }
        }
    });
    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum EventPayload {
            #(#variants),*
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
    let structural_field = match control.role {
        Role::Content => quote! { content: Option<Box<Element>> },
        Role::Children => quote! { children: std::rc::Rc<Vec<KeyedElement>> },
        Role::Virtual => quote! { items: std::rc::Rc<Vec<KeyedElement>> },
        Role::Leaf | Role::Controlled => TokenStream::new(),
    };
    let property_methods = control.properties.iter().flat_map(|property| {
        let field = ident(&property.field);
        let getter = ident(&format!("{}_property", property.field));
        let value = value_type(&property.value);
        let setter = if property.value == "Str" {
            quote! {
                pub fn #field(mut self, value: impl Into<String>) -> Self {
                    self.#field = Property::Set(value.into());
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, value: #value) -> Self {
                    self.#field = Property::Set(value);
                    self
                }
            }
        };
        [
            setter,
            quote! {
                pub fn #getter(&self) -> &Property<#value> {
                    &self.#field
                }
            },
        ]
    });
    let event_methods = control.events.iter().flat_map(|event| {
        let field = ident(&event.field);
        let getter = ident(&format!("{}_callback", event.field));
        let payload = value_type(&event.payload);
        let setter = if event.payload == "Unit" {
            quote! {
                pub fn #field(mut self, callback: impl Fn() + 'static) -> Self {
                    self.#field = Some(Callback::new(move |()| callback()));
                    self
                }
            }
        } else {
            quote! {
                pub fn #field(mut self, callback: impl Fn(#payload) + 'static) -> Self {
                    self.#field = Some(Callback::new(callback));
                    self
                }
            }
        };
        [
            setter,
            quote! {
                pub fn #getter(&self) -> Option<&Callback<#payload>> {
                    self.#field.as_ref()
                }
            },
        ]
    });
    let structural_methods = match control.role {
        Role::Content => quote! {
            pub fn content(mut self, content: impl Into<Element>) -> Self {
                self.content = Some(Box::new(content.into()));
                self
            }

            pub fn content_element(&self) -> Option<&Element> {
                self.content.as_deref()
            }
        },
        Role::Children => quote! {
            pub fn child(mut self, key: impl Into<Key>, child: impl Into<Element>) -> Self {
                std::rc::Rc::make_mut(&mut self.children).push(KeyedElement::new(key, child));
                self
            }

            pub fn children(
                mut self,
                children: impl IntoIterator<Item = KeyedElement>,
            ) -> Self {
                self.children = std::rc::Rc::new(children.into_iter().collect());
                self
            }

            pub fn child_elements(&self) -> &[KeyedElement] {
                self.children.as_slice()
            }
        },
        Role::Virtual => quote! {
            pub fn item(mut self, key: impl Into<Key>, item: impl Into<Element>) -> Self {
                std::rc::Rc::make_mut(&mut self.items).push(KeyedElement::new(key, item));
                self
            }

            pub fn items(
                mut self,
                items: impl IntoIterator<Item = KeyedElement>,
            ) -> Self {
                self.items = std::rc::Rc::new(items.into_iter().collect());
                self
            }

            pub fn item_elements(&self) -> &[KeyedElement] {
                self.items.as_slice()
            }
        },
        Role::Leaf | Role::Controlled => TokenStream::new(),
    };
    let capability_impls = control.capabilities.iter().map(|capability| {
        let capability = ident(match capability {
            Capability::Layout => "LayoutControl",
            Capability::TextStyle => "TextStyleControl",
            Capability::Enabled => "EnabledControl",
            Capability::Content => "ContentControl",
            Capability::Children => "ChildrenControl",
            Capability::ControlledText => "ControlledTextControl",
            Capability::Items => "ItemsControl",
        });
        quote! { impl #capability for #name {} }
    });

    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct #name {
            #(#property_fields,)*
            #(#event_fields,)*
            #structural_field
        }

        impl #name {
            pub fn new() -> Self {
                Self::default()
            }

            #(#property_methods)*
            #(#event_methods)*
            #structural_methods
        }

        impl sealed::Sealed for #name {}
        #(#capability_impls)*
    }
}

fn generate_descriptors(control: &ResolvedControl) -> TokenStream {
    let properties_ident = descriptor_ident(&control.name, "PROPERTIES");
    let events_ident = descriptor_ident(&control.name, "EVENTS");
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
        quote! {
            PropertyDescriptor {
                id: PropertyId::#id,
                name: #name,
                field: #field,
                value: #value,
                interface: #interface,
                clearable: #clearable,
                feedback: #feedback,
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

    quote! {
        const #properties_ident: &[PropertyDescriptor] = &[
            #(#properties),*
        ];
        const #events_ident: &[EventDescriptor] = &[
            #(#events),*
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
            },
            Span::call_site(),
        );
        quote! { Capability::#capability }
    });
    let properties = descriptor_ident(name, "PROPERTIES");
    let events = descriptor_ident(name, "EVENTS");

    quote! {
        ControlDescriptor {
            kind: MountedKind::#kind,
            name: #name,
            type_name: #type_name,
            role: ControlRole::#role,
            capabilities: &[#(#capabilities),*],
            properties: #properties,
            events: #events,
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

        assert_eq!(output.matches("ControlDescriptor").count(), 8);
        assert!(output.contains("feedback : Some"));
        assert!(output.contains("ControlRole :: Children"));
        assert!(output.contains("pub struct TextBox"));
        assert!(output.contains("impl ControlledTextControl for TextBox"));
        assert!(output.contains("impl ItemsControl for ItemsRepeater"));
        assert!(output.contains("pub enum Orientation"));
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
        assert!(
            output.contains("interface : \"Microsoft.UI.Xaml.Controls.Primitives.IRangeBase\"")
        );
    }
}
