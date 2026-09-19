use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

use crate::metadata::CollectionType;
use crate::schema::{
    EventPayloadConversion, EventPayloadSource, EventSubscription, FeedbackContract,
    PropertyAdapter, ResolvedControl, ResolvedEvent, ResolvedProperty, ResolvedSchema, Role,
    SlotShape, SlotTarget,
};

pub(crate) fn generate_control_bindings_filter(schema: &ResolvedSchema) -> String {
    let mut entries = BTreeSet::new();

    for control in &schema.controls {
        if matches!(control.role, Role::Virtual) {
            continue;
        }
        entries.insert(format!(
            "{}::CreateInstance",
            filter_path(&control.type_name)
        ));
        match control.role {
            Role::Children => {
                entries.insert("Microsoft::UI::Xaml::Controls::IPanel::Children".to_string());
                entries
                    .insert("Microsoft::UI::Xaml::Controls::UIElementCollection::{}".to_string());
                entries.insert(
                    "Microsoft::UI::Xaml::Controls::IUIElementCollection::Move".to_string(),
                );
            }
            Role::Leaf | Role::Content | Role::Slots | Role::Virtual => {}
        }
        if let Some(content) = &control.content {
            entries.insert(format!(
                "{}::put_{}",
                filter_path(&content.interface),
                content.name
            ));
        }

        for property in &control.properties {
            match property.adapter {
                Some(PropertyAdapter::ImplicitOpacityTransition) => {
                    entries.insert(
                        "Microsoft::UI::Xaml::ScalarTransition::CreateInstance".to_string(),
                    );
                    entries
                        .insert("Microsoft::UI::Xaml::IScalarTransition::put_Duration".to_string());
                }
                Some(PropertyAdapter::ImplicitScaleTransition) => {
                    entries.insert(
                        "Microsoft::UI::Xaml::Vector3Transition::CreateInstance".to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::IVector3Transition::put_Duration".to_string(),
                    );
                }
                Some(PropertyAdapter::ImplicitScale) => {
                    entries.insert(
                        "Microsoft::UI::Xaml::IFrameworkElement::get_ActualWidth".to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::IFrameworkElement::get_ActualHeight".to_string(),
                    );
                    entries.insert("Microsoft::UI::Xaml::IUIElement::put_CenterPoint".to_string());
                }
                Some(PropertyAdapter::KeyAccelerators) => {
                    entries.insert(
                        "Microsoft::UI::Xaml::IUIElement::{get_KeyboardAccelerators, \
                         put_KeyboardAcceleratorPlacementMode}"
                            .to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::Input::KeyboardAccelerator::CreateInstance"
                            .to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::Input::IKeyboardAccelerator::{put_Key, \
                         put_Modifiers, Invoked}"
                            .to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::Input::IKeyboardAcceleratorInvokedEventArgs::put_Handled"
                            .to_string(),
                    );
                }
                _ => {}
            }
            if property.uses_property_setter {
                entries.insert(format!(
                    "{}::put_{}",
                    filter_path(&property.interface),
                    property.name
                ));
            }
            if property.uses_dependency_property {
                entries.insert(format!(
                    "{}::{}Property",
                    filter_path(&property.static_owner),
                    property.name
                ));
            }
            if let Some(native_value) = &property.native_value {
                entries.insert(filter_path(native_value));
            }
            if property.theme_style {
                entries.insert(
                    "Microsoft::UI::Xaml::Media::SolidColorBrush::CreateInstance".to_string(),
                );
                entries
                    .insert("Microsoft::UI::Xaml::Media::ISolidColorBrush::put_Color".to_string());
                entries.insert("Windows::UI::Color".to_string());
            }
            if property.adapter == Some(PropertyAdapter::ImageUri) {
                entries.insert("Windows::Foundation::Uri::CreateUri".to_string());
                entries.insert("Microsoft::UI::Xaml::Media::ImageSource".to_string());
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::BitmapImage::CreateInstance".to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::IBitmapImage::put_UriSource".to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::IBitmapSource::SetSourceAsync"
                        .to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::SvgImageSource::CreateInstance"
                        .to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::ISvgImageSource::put_UriSource"
                        .to_string(),
                );
                entries.insert(
                    "Windows::Storage::Streams::InMemoryRandomAccessStream::CreateInstance"
                        .to_string(),
                );
                entries.insert(
                    "Windows::Storage::Streams::IRandomAccessStream::{\
                     GetOutputStreamAt, Seek}"
                        .to_string(),
                );
                entries
                    .insert("Windows::Storage::Streams::DataWriter::CreateDataWriter".to_string());
                entries.insert(
                    "Windows::Storage::Streams::IDataWriter::{\
                     WriteBytes, StoreAsync, DetachStream}"
                        .to_string(),
                );
            }
        }
        for event in &control.events {
            match &event.subscription {
                EventSubscription::Metadata => {
                    entries.insert(format!(
                        "{}::{{add_{}, remove_{}}}",
                        filter_path(&event.interface),
                        event.name,
                        event.name
                    ));
                }
                EventSubscription::PropertyChanged {
                    property,
                    static_owner,
                } => {
                    entries.insert(
                        "Microsoft::UI::Xaml::DependencyPropertyChangedCallback".to_string(),
                    );
                    entries.insert(
                        "Microsoft::UI::Xaml::IDependencyObject::{\
                         RegisterPropertyChangedCallback, UnregisterPropertyChangedCallback}"
                            .to_string(),
                    );
                    entries.insert(format!(
                        "{}::{}Property",
                        filter_path(static_owner),
                        property
                    ));
                }
            }
            if let Some((interface, property)) = event.source.getter() {
                entries.insert(format!("{}::get_{}", filter_path(interface), property));
            }
            if matches!(
                event.source,
                EventPayloadSource::DragInfo { .. } | EventPayloadSource::DropData { .. }
            ) {
                entries.insert(
                    "Windows::ApplicationModel::DataTransfer::IDataPackageView::{Contains, \
                     GetTextAsync, GetStorageItemsAsync}"
                        .to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::IDragEventArgs::{put_AcceptedOperation, \
                     get_DataView, get_DragUIOverride, GetDeferral}"
                        .to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::IDragUIOverride::{put_Caption, \
                     put_IsCaptionVisible}"
                        .to_string(),
                );
                entries.insert("Microsoft::UI::Xaml::IDragOperationDeferral::Complete".to_string());
                entries.insert("Windows::Storage::IStorageItem::{get_Name, get_Path}".to_string());
            }
        }
        if let Some(selection) = &control.selection {
            entries.insert(format!(
                "{}::{{get_{}, put_{}}}",
                filter_path(&selection.owner_interface),
                selection.selected_item_property,
                selection.selected_item_property,
            ));
            entries.insert(format!(
                "{}::get_{}",
                filter_path(&selection.selected_interface),
                selection.selected_property,
            ));
            entries.insert(format!(
                "{}::get_{}",
                filter_path(&selection.payload_interface),
                selection.payload_property,
            ));
        }
        for slot in &control.slots {
            let accessor = match &slot.shape {
                SlotShape::Single(_) => "put",
                SlotShape::Collection(_) => "get",
            };
            entries.insert(format!(
                "{}::{}_{}",
                filter_path(&slot.interface),
                accessor,
                slot.name
            ));
        }
    }

    let mut result = entries.into_iter().collect::<Vec<_>>().join("\n");
    result.push('\n');
    result
}

pub(crate) fn generate(schema: &ResolvedSchema) -> String {
    let native_controls = schema
        .controls
        .iter()
        .filter(|control| !matches!(control.role, Role::Virtual))
        .collect::<Vec<_>>();
    let variants = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { #name(bindings::#name) }
    });
    let create = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! {
            MountedKind::#name => {
                Self::#name(bindings::#name::new().map_err(native_error)?)
            }
        }
    });
    let unsupported_create = schema
        .controls
        .iter()
        .any(|control| matches!(control.role, Role::Virtual))
        .then(|| quote! { _ => return Err(RuntimeError::UnsupportedKind) });
    let inspectables = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { Self::#name(value) => value.into() }
    });
    let kinds = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { Self::#name(_) => MountedKind::#name }
    });
    let content_kinds = schema
        .controls
        .iter()
        .filter(|control| control.content.is_some())
        .map(|control| {
            let name = ident(&control.name);
            quote! { Self::#name(_) }
        });
    let contents = schema.controls.iter().filter_map(|control| {
        control
            .content
            .as_ref()
            .map(|content| generate_set_content(control, content))
    });
    let child_collections = schema
        .controls
        .iter()
        .filter(|control| matches!(control.role, Role::Children))
        .map(|control| {
            let name = ident(&control.name);
            quote! {
                Self::#name(value) => Some(
                    value
                        .cast::<IPanel>()
                        .map_err(native_error)?
                        .Children()
                        .map_err(native_error)?,
                )
            }
        });
    let set_properties = schema.controls.iter().flat_map(|control| {
        control
            .properties
            .iter()
            .filter(|property| !is_handwritten_property_adapter(property))
            .map(move |property| generate_set_property(control, property))
    });
    let clear_properties = schema.controls.iter().flat_map(|control| {
        control
            .properties
            .iter()
            .filter(|property| !is_handwritten_property_adapter(property))
            .map(move |property| generate_clear_property(control, property))
    });
    let events = schema.controls.iter().flat_map(|control| {
        control
            .events
            .iter()
            .map(move |event| generate_event_arm(control, event))
    });
    let slots = schema.controls.iter().flat_map(|control| {
        control
            .slots
            .iter()
            .filter(|slot| matches!(&slot.shape, SlotShape::Single(_)))
            .map(move |slot| generate_set_slot(control, slot))
    });
    let collection_slots = schema.controls.iter().flat_map(|control| {
        control
            .slots
            .iter()
            .filter(|slot| matches!(&slot.shape, SlotShape::Collection(_)))
            .map(move |slot| generate_slot_collection(control, slot))
    });
    let collection_items = schema
        .controls
        .iter()
        .flat_map(|control| &control.slots)
        .filter_map(|slot| slot.shape.collection_item())
        .collect::<BTreeSet<_>>();
    let collection_variants = collection_items.iter().map(|item| {
        let variant = ident(item.rsplit('.').next().unwrap());
        let item = path_ident(item);
        quote! { #variant(windows_collections::IVector<bindings::#item>) }
    });
    let collection_sizes = collection_items.iter().map(|item| {
        let variant = ident(item.rsplit('.').next().unwrap());
        quote! { Self::#variant(value) => value.Size().map_err(native_error) }
    });
    let collection_items_at = collection_items.iter().map(|item| {
        let variant = ident(item.rsplit('.').next().unwrap());
        quote! {
            Self::#variant(value) => value
                .GetAt(index)
                .map(Into::into)
                .map_err(native_error)
        }
    });
    let collection_inserts = collection_items.iter().map(|item| {
        let variant = ident(item.rsplit('.').next().unwrap());
        let item = path_ident(item);
        quote! {
            Self::#variant(value) => {
                let child = child.cast::<bindings::#item>().map_err(native_error)?;
                value.InsertAt(index, &child).map_err(native_error)
            }
        }
    });
    let collection_removes = collection_items.iter().map(|item| {
        let variant = ident(item.rsplit('.').next().unwrap());
        quote! { Self::#variant(value) => value.RemoveAt(index).map_err(native_error) }
    });
    let feedback_values = schema.controls.iter().flat_map(|control| {
        control.properties.iter().filter_map(move |property| {
            let feedback = property.feedback.as_ref()?;
            let property_id = ident(&format!("{}{}", control.name, property.name));
            let event_id = ident(&format!("{}{}", control.name, feedback));
            let value_variant = ident(&property.value);
            match property.feedback_contract.unwrap() {
                FeedbackContract::Exact => {
                    let value = if property.copy {
                        quote! { *value }
                    } else {
                        quote! { value.clone() }
                    };
                    Some(quote! {
                        (
                            PropertyId::#property_id,
                            Some(PropertyValue::#value_variant(value)),
                        ) => Some((
                            EventId::#event_id,
                            FeedbackExpectation::Exact(EventPayload::#value_variant(#value)),
                        ))
                    })
                }
                FeedbackContract::Normalized => Some(quote! {
                    (PropertyId::#property_id, Some(_)) => {
                        Some((
                            EventId::#event_id,
                            FeedbackExpectation::Normalized { observation: None },
                        ))
                    }
                }),
                FeedbackContract::DeferredExact => None,
            }
        })
    });
    let feedback_defaults = schema.controls.iter().flat_map(|control| {
        control.properties.iter().filter_map(move |property| {
            let feedback = property.feedback.as_ref()?;
            let property_id = ident(&format!("{}{}", control.name, property.name));
            let event_id = ident(&format!("{}{}", control.name, feedback));
            let value_variant = ident(&property.value);
            match property.feedback_contract.unwrap() {
                FeedbackContract::Exact => {
                    if property.adapter == Some(PropertyAdapter::SelectionIndex) {
                        return Some(quote! {
                            (PropertyId::#property_id, None) => Some((
                                EventId::#event_id,
                                FeedbackExpectation::Normalized { observation: None },
                            ))
                        });
                    }
                    let value = property
                        .clear_feedback
                        .map_or_else(|| quote! { Default::default() }, |value| quote! { #value });
                    Some(quote! {
                        (PropertyId::#property_id, None) => Some((
                            EventId::#event_id,
                            FeedbackExpectation::Exact(EventPayload::#value_variant(#value)),
                        ))
                    })
                }
                FeedbackContract::Normalized => Some(quote! {
                    (PropertyId::#property_id, None) => {
                        Some((
                            EventId::#event_id,
                            FeedbackExpectation::Normalized { observation: None },
                        ))
                    }
                }),
                FeedbackContract::DeferredExact => None,
            }
        })
    });
    let theme_style_info = schema.controls.iter().filter_map(|control| {
        let properties = control
            .properties
            .iter()
            .filter(|property| property.theme_style)
            .collect::<Vec<_>>();
        if properties.is_empty() {
            return None;
        }
        let kind = ident(&control.name);
        let target = &control.name;
        let names = properties.iter().map(|property| &property.name);
        Some(quote! {
            MountedKind::#kind => Some((#target, &[#(#names),*]))
        })
    });
    let selected_items = schema.controls.iter().filter_map(|control| {
        let selection = control.selection.as_ref()?;
        let control_name = ident(&control.name);
        let event = ident(&format!("{}{}", control.name, selection.event));
        let interface = path_ident(&selection.owner_interface);
        let getter = ident(&selection.selected_item_property);
        let get = if is_default_interface(control, &selection.owner_interface) {
            quote! { value.#getter() }
        } else {
            quote! { value.cast::<#interface>().and_then(|value| value.#getter()) }
        };
        let selected = if selection.selected_item.is_some() {
            quote! { Ok(Some(selected.into())) }
        } else {
            quote! { Ok(Some(selected)) }
        };
        Some(quote! {
            (Handle::#control_name(value), EventId::#event) => {
                match #get {
                    Ok(selected) => #selected,
                    Err(error) if error.code().is_ok() => Ok(None),
                    Err(error) => Err(native_error(error)),
                }
            }
        })
    });
    let set_selected_items = schema.controls.iter().filter_map(|control| {
        let selection = control.selection.as_ref()?;
        let control_name = ident(&control.name);
        let event = ident(&format!("{}{}", control.name, selection.event));
        let interface = path_ident(&selection.owner_interface);
        let setter = ident(&format!("Set{}", selection.selected_item_property));
        let set = if let Some(item) = selection.selected_item.as_deref() {
            let item = path_ident(item);
            if is_default_interface(control, &selection.owner_interface) {
                quote! {
                    selected
                        .cast::<bindings::#item>()
                        .and_then(|selected| value.#setter(&selected))
                }
            } else {
                quote! {
                    value.cast::<#interface>().and_then(|value| {
                        let selected = selected.cast::<bindings::#item>()?;
                        value.#setter(&selected)
                    })
                }
            }
        } else if is_default_interface(control, &selection.owner_interface) {
            quote! { value.#setter(selected) }
        } else {
            quote! {
                value
                    .cast::<#interface>()
                    .and_then(|value| value.#setter(selected))
            }
        };
        Some(quote! {
            (Handle::#control_name(value), EventId::#event) => #set.map_err(native_error),
        })
    });
    let selection_item_states = schema.controls.iter().filter_map(|control| {
        let selection = control.selection.as_ref()?;
        let property = ident(&format!(
            "{}{}",
            selection.item, selection.selected_property
        ));
        let interface = path_ident(&selection.selected_interface);
        let getter = ident(&selection.selected_property);
        Some(quote! {
            PropertyId::#property => {
                let Ok(item) = item.cast::<#interface>() else {
                    return Ok(false);
                };
                item.#getter().map_err(native_error)
            }
        })
    });
    let selection_payloads = schema.controls.iter().filter_map(|control| {
        let selection = control.selection.as_ref()?;
        let property = ident(&format!("{}{}", selection.item, selection.payload_property));
        let interface = path_ident(&selection.payload_interface);
        let getter = ident(&selection.payload_property);
        let read = if selection.payload_inspectable {
            quote! {
                item
                    .cast::<#interface>()
                    .and_then(|item| item.#getter())
                    .and_then(|value| {
                        value.cast::<windows_reference::IReference<windows_core::HSTRING>>()
                    })
                    .and_then(|value| value.Value())
                    .map(|value| value.to_string_lossy())
            }
        } else {
            quote! {
                item
                    .cast::<#interface>()
                    .and_then(|item| item.#getter())
            }
        };
        Some(quote! {
            PropertyId::#property => match #read {
                Ok(value) => Ok(Some(value)),
                Err(error) if error.code().is_ok() => Ok(None),
                Err(error) => Err(native_error(error)),
            }
        })
    });

    let tokens = quote! {
        use super::*;

        pub enum Handle {
            #(#variants),*
        }

        pub enum SlotCollection {
            Inspectable(windows_collections::IVector<windows_core::IInspectable>),
            #(#collection_variants),*
        }

        #[allow(non_snake_case)]
        impl SlotCollection {
            pub fn Size(&self) -> Result<u32, RuntimeError> {
                match self {
                    Self::Inspectable(value) => value.Size().map_err(native_error),
                    #(#collection_sizes),*
                }
            }

            pub fn GetAt(
                &self,
                index: u32,
            ) -> Result<windows_core::IInspectable, RuntimeError> {
                match self {
                    Self::Inspectable(value) => value.GetAt(index).map_err(native_error),
                    #(#collection_items_at),*
                }
            }

            pub fn InsertAt(
                &self,
                index: u32,
                child: &windows_core::IInspectable,
            ) -> Result<(), RuntimeError> {
                match self {
                    Self::Inspectable(value) => value.InsertAt(index, child).map_err(native_error),
                    #(#collection_inserts),*
                }
            }

            pub fn RemoveAt(&self, index: u32) -> Result<(), RuntimeError> {
                match self {
                    Self::Inspectable(value) => value.RemoveAt(index).map_err(native_error),
                    #(#collection_removes),*
                }
            }
        }

        impl Handle {
            pub fn create(kind: MountedKind) -> Result<Self, RuntimeError> {
                Ok(match kind {
                    #(#create,)*
                    #unsupported_create
                })
            }

            #[inline]
            pub fn inspectable(&self) -> &windows_core::IInspectable {
                match self {
                    #(#inspectables),*
                }
            }

            pub fn kind(&self) -> MountedKind {
                match self {
                    #(#kinds),*
                }
            }

            pub fn is_content(&self) -> bool {
                matches!(self, #(#content_kinds)|*)
            }

            pub fn child_collection(
                &self,
            ) -> Result<Option<UIElementCollection>, RuntimeError> {
                Ok(match self {
                    #(#child_collections,)*
                    _ => None,
                })
            }
        }

        pub fn set_content(
            handle: &Handle,
            child: Option<&UIElement>,
        ) -> Result<(), RuntimeError> {
            match handle {
                #(#contents,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn set_property(
            handle: &Handle,
            property: PropertyId,
            value: &PropertyValue,
        ) -> Result<(), RuntimeError> {
            match (handle, property, value) {
                #(#set_properties,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn clear_property(
            handle: &Handle,
            property: PropertyId,
        ) -> Result<(), RuntimeError> {
            match (handle, property) {
                #(#clear_properties,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn set_slot(
            handle: &Handle,
            slot: SlotId,
            child: Option<&UIElement>,
        ) -> Result<(), RuntimeError> {
            match (handle, slot) {
                #(#slots,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn slot_collection(
            handle: &Handle,
            slot: SlotId,
        ) -> Result<SlotCollection, RuntimeError> {
            match (handle, slot) {
                #(#collection_slots,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn selected_item(
            handle: &Handle,
            selection: SelectionDescriptor,
        ) -> Result<Option<windows_core::IInspectable>, RuntimeError> {
            match (handle, selection.event) {
                #(#selected_items,)*
                _ => Ok(None),
            }
        }

        pub fn set_selected_item(
            handle: &Handle,
            selection: SelectionDescriptor,
            selected: &windows_core::IInspectable,
        ) -> Result<(), RuntimeError> {
            match (handle, selection.event) {
                #(#set_selected_items)*
                _ => Ok(()),
            }
        }

        pub fn selection_item_is_selected(
            selection: SelectionDescriptor,
            item: &windows_core::IInspectable,
        ) -> Result<bool, RuntimeError> {
            match selection.selected_property {
                #(#selection_item_states,)*
                _ => Ok(false),
            }
        }

        pub fn selection_payload(
            selection: SelectionDescriptor,
            item: &windows_core::IInspectable,
        ) -> Result<Option<String>, RuntimeError> {
            match selection.payload_property {
                #(#selection_payloads,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        pub fn expected_feedback(
            property: PropertyId,
            value: Option<&PropertyValue>,
        ) -> Option<(EventId, FeedbackExpectation)> {
            match (property, value) {
                #(#feedback_values,)*
                #(#feedback_defaults,)*
                _ => None,
            }
        }

        pub fn theme_style_info(
            kind: MountedKind,
        ) -> Option<(&'static str, &'static [&'static str])> {
            match kind {
                #(#theme_style_info,)*
                _ => None,
            }
        }

        pub fn subscribe_event(
            handle: &Handle,
            node: NodeId,
            event: EventId,
            revision: u32,
            sink: EventSink,
        ) -> Result<NativeSubscription, RuntimeError> {
            match (handle, event) {
                #(#events,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }
    };

    format!("// Generated by `tool-reactor`. Do not edit.\n\n{tokens}\n")
}

fn generate_set_content(
    control: &ResolvedControl,
    content: &crate::schema::ResolvedContent,
) -> TokenStream {
    let control_name = ident(&control.name);
    if content.name == "Content"
        && content.interface == "Microsoft.UI.Xaml.Controls.IContentControl"
        && matches!(content.target, SlotTarget::Inspectable)
    {
        return quote! {
            Handle::#control_name(control) => set_content_control(control, child)
        };
    }
    let interface = path_ident(&content.interface);
    let setter = ident(&format!("Set{}", content.name));
    let value = match content.target {
        SlotTarget::IconElement => quote! {
            match child {
                Some(child) => {
                    let child = child.cast::<IconElement>().map_err(native_error)?;
                    control.#setter(&child).map_err(native_error)
                }
                None => control.#setter(None::<&IconElement>).map_err(native_error),
            }
        },
        SlotTarget::Inspectable => quote! {
            match child {
                Some(child) => control.#setter(child).map_err(native_error),
                None => control
                    .#setter(None::<&windows_core::IInspectable>)
                    .map_err(native_error),
            }
        },
        SlotTarget::UiElement => quote! {
            match child {
                Some(child) => control.#setter(child).map_err(native_error),
                None => control.#setter(None::<&UIElement>).map_err(native_error),
            }
        },
    };
    let set = if is_default_interface(control, &content.interface) {
        value
    } else {
        quote! {
            {
                let control = control.cast::<#interface>().map_err(native_error)?;
                #value
            }
        }
    };
    quote! { Handle::#control_name(control) => #set }
}

fn generate_slot_collection(
    control: &ResolvedControl,
    slot: &crate::schema::ResolvedSlot,
) -> TokenStream {
    let control_name = ident(&control.name);
    let slot_id = ident(&format!("{}{}", control.name, slot.name));
    let interface = path_ident(&slot.interface);
    let getter = ident(&slot.name);
    let get = if is_default_interface(control, &slot.interface) {
        quote! { control.#getter().map_err(native_error) }
    } else {
        quote! {
            control
                .cast::<#interface>()
                .map_err(native_error)?
                .#getter()
                .map_err(native_error)
        }
    };
    let collection = match &slot.shape {
        SlotShape::Collection(CollectionType::InspectableVector) => {
            quote! { SlotCollection::Inspectable(#get?) }
        }
        SlotShape::Collection(CollectionType::ItemCollection) => quote! {
            SlotCollection::Inspectable(
                #get?
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .map_err(native_error)?
            )
        },
        SlotShape::Collection(CollectionType::TypedVector(item)) => {
            let variant = ident(item.rsplit('.').next().unwrap());
            quote! { SlotCollection::#variant(#get?) }
        }
        SlotShape::Collection(CollectionType::ObservableVector(item)) => {
            let variant = ident(item.rsplit('.').next().unwrap());
            let item = path_ident(item);
            quote! {
                SlotCollection::#variant(
                    #get?
                        .cast::<windows_collections::IVector<bindings::#item>>()
                        .map_err(native_error)?
                )
            }
        }
        SlotShape::Single(_) => unreachable!(),
    };
    quote! { (Handle::#control_name(control), SlotId::#slot_id) => Ok(#collection) }
}

fn generate_set_slot(control: &ResolvedControl, slot: &crate::schema::ResolvedSlot) -> TokenStream {
    let control_name = ident(&control.name);
    let slot_id = ident(&format!("{}{}", control.name, slot.name));
    let interface = path_ident(&slot.interface);
    let setter = ident(&format!("Set{}", slot.name));
    let SlotShape::Single(target) = &slot.shape else {
        unreachable!()
    };
    let value = match *target {
        SlotTarget::IconElement => quote! {
            match child {
                Some(child) => {
                    let child = child.cast::<IconElement>().map_err(native_error)?;
                    control.#setter(&child).map_err(native_error)
                }
                None => control.#setter(None::<&IconElement>).map_err(native_error),
            }
        },
        SlotTarget::Inspectable => quote! {
            match child {
                Some(child) => control.#setter(child).map_err(native_error),
                None => control
                    .#setter(None::<&windows_core::IInspectable>)
                    .map_err(native_error),
            }
        },
        SlotTarget::UiElement => quote! {
            match child {
                Some(child) => control.#setter(child).map_err(native_error),
                None => control.#setter(None::<&UIElement>).map_err(native_error),
            }
        },
    };
    let set = if is_default_interface(control, &slot.interface) {
        value
    } else {
        quote! {
            {
                let control = control.cast::<#interface>().map_err(native_error)?;
                #value
            }
        }
    };
    quote! {
        (Handle::#control_name(control), SlotId::#slot_id) => #set
    }
}

fn generate_payload_value(conversion: &EventPayloadConversion, event_id: &Ident) -> TokenStream {
    match conversion {
        EventPayloadConversion::Identity => quote! { value },
        EventPayloadConversion::Field(field) => {
            let field = ident(field);
            quote! { value.#field }
        }
        EventPayloadConversion::Nullable => quote! { Some(value) },
        EventPayloadConversion::NumberBoxValue => quote! { number_box_value(value) },
        EventPayloadConversion::RatingValue => quote! { rating_value(value) },
        EventPayloadConversion::Selection => quote! { value },
        EventPayloadConversion::SelectionIndex => quote! {
            match selection_index(value) {
                Ok(value) => value,
                Err(error) => {
                    sink.error(node, EventId::#event_id, revision, error);
                    return;
                }
            }
        },
    }
}

fn generate_selection_dispatch(control: &ResolvedControl, event: &ResolvedEvent) -> TokenStream {
    let event_id = ident(&format!("{}{}", control.name, event.name));
    let payload = ident(&event.payload);
    let erase_item = if control.selection.as_ref().unwrap().selected_item.is_some() {
        quote! {
            let item: windows_core::IInspectable = item.into();
        }
    } else {
        quote! {}
    };
    quote! {
        match value {
            Ok(item) => {
                #erase_item
                let selected = sink.selection_item(&item);
                match selection_payload(
                    selection_for_event(EventId::#event_id).unwrap(),
                    &item,
                ) {
                    Ok(tag) => sink.enqueue(
                        node,
                        EventId::#event_id,
                        revision,
                        EventPayload::#payload(SelectionChange {
                            item: selected,
                            tag,
                        }),
                    ),
                    Err(error) => {
                        sink.error(node, EventId::#event_id, revision, error);
                    }
                }
            }
            Err(error) if error.code().is_ok() => sink.enqueue(
                node,
                EventId::#event_id,
                revision,
                EventPayload::#payload(SelectionChange {
                    item: None,
                    tag: None,
                }),
            ),
            Err(error) => sink.error(
                node,
                EventId::#event_id,
                revision,
                native_error(error),
            ),
        }
    }
}

fn generate_event_arm(control: &ResolvedControl, event: &ResolvedEvent) -> TokenStream {
    let control_name = ident(&control.name);
    let event_id = ident(&format!("{}{}", control.name, event.name));
    let interface = path_ident(&event.interface);
    let payload = ident(&event.payload);
    let payload_value = if event.payload == "ContentDialogResult" {
        quote! {
            match value.0 {
                1 => crate::ContentDialogResult::Primary,
                2 => crate::ContentDialogResult::Secondary,
                _ => crate::ContentDialogResult::None,
            }
        }
    } else if event.payload == "Color" {
        quote! {
            crate::Color {
                a: value.a,
                r: value.r,
                g: value.g,
                b: value.b,
            }
        }
    } else if event.payload == "NavigationViewDisplayMode" {
        quote! {
            match value {
                bindings::NavigationViewDisplayMode::Minimal => {
                    crate::NavigationViewDisplayMode::Minimal
                }
                bindings::NavigationViewDisplayMode::Compact => {
                    crate::NavigationViewDisplayMode::Compact
                }
                bindings::NavigationViewDisplayMode::Expanded => {
                    crate::NavigationViewDisplayMode::Expanded
                }
                _ => return,
            }
        }
    } else {
        generate_payload_value(&event.conversion, &event_id)
    };
    let content_dialog_closed = control.lifecycle == Some(crate::schema::Lifecycle::ContentDialog)
        && event.name == "Closed";
    let method = ident(&event.name);
    let lifecycle_completion = content_dialog_closed.then(|| {
        quote! {
            let invoke_callback = match sink.content_dialog_closed(node, revision) {
                Ok(value) => value,
                Err(error) => {
                    sink.error(node, EventId::#event_id, revision, error);
                    return;
                }
            };
        }
    });
    let enqueue_payload = if content_dialog_closed {
        quote! {
            sink.enqueue_or_observe(
                node,
                EventId::#event_id,
                revision,
                EventPayload::#payload(#payload_value),
                invoke_callback,
            )
        }
    } else {
        quote! {
            sink.enqueue(
                node,
                EventId::#event_id,
                revision,
                EventPayload::#payload(#payload_value),
            )
        }
    };
    let nullable_error = (event.conversion == EventPayloadConversion::Nullable).then(|| {
        quote! {
            Err(error) if error.code().is_ok() => sink.enqueue(
                node,
                EventId::#event_id,
                revision,
                EventPayload::#payload(None),
            ),
        }
    });
    let shared_event_source = matches!(&event.subscription, EventSubscription::Metadata)
        && !inspectable_routed_event(event)
        && !inspectable_selection_changed_event(event)
        && matches!(
            &event.source,
            EventPayloadSource::SenderProperty { interface, .. } if interface == &event.interface
        );
    let text_input_probe = control.name == "TextBox" && event.name == "TextChanged";
    let callback = match &event.source {
        EventPayloadSource::Unit(sender, args) => {
            if inspectable_routed_event(event) {
                quote! {
                    routed_event_handler(
                        sink,
                        node,
                        EventId::#event_id,
                        revision,
                        RoutedEventAction::Unit,
                    )
                }
            } else {
                let sender = event_handler_type(sender);
                let args = event_handler_type(args);
                quote! {
                    unit_event_handler::<#sender, #args>(
                        sink,
                        node,
                        EventId::#event_id,
                        revision,
                    )
                }
            }
        }
        EventPayloadSource::DragInfo { interface: _ } => {
            quote! {
                drag_info_event_handler(
                    sink,
                    node,
                    EventId::#event_id,
                    revision,
                )
            }
        }
        EventPayloadSource::DropData { interface: _ } => {
            quote! {
                move |_, args| {
                    let result = args
                        .as_ref()
                        .ok_or_else(windows_core::Error::empty)
                        .and_then(|args| {
                            let deferral = args.GetDeferral()?;
                            let result = (|| {
                                let data = args.DataView()?;
                                if data.Contains("Shell IDList Array")? {
                                    let Some(action) =
                                        sink.drag_action(node, DragKind::StorageItems)
                                    else {
                                        args.SetAcceptedOperation(DataPackageOperation::None)?;
                                        deferral.Complete()?;
                                        sink.enqueue(
                                            node,
                                            EventId::#event_id,
                                            revision,
                                            EventPayload::DroppedData(DroppedData::Unsupported),
                                        );
                                        return Ok(());
                                    };
                                    args.SetAcceptedOperation(native_drag_operation(
                                        action.operation,
                                    ))?;
                                    let operation = data.GetStorageItemsAsync()?;
                                    let sender = sink.begin_async_event(
                                        node,
                                        EventId::#event_id,
                                        revision,
                                        deferral.clone(),
                                    );
                                    let completion = sender.clone();
                                    if let Err(error) = operation.when(move |result| {
                                        let result = result.and_then(|items| {
                                            let mut dropped =
                                                Vec::with_capacity(items.Size()? as usize);
                                            for index in 0..items.Size()? {
                                                let item = items.GetAt(index)?;
                                                dropped.push(DroppedStorageItem {
                                                    name: item.Name()?,
                                                    path: item.Path()?,
                                                });
                                            }
                                            Ok(DroppedData::StorageItems(dropped))
                                        });
                                        _ = completion.complete(result.map_err(native_error));
                                    }) {
                                        _ = sender.complete(Err(native_error(error)));
                                    }
                                    Ok(())
                                } else if data.Contains("Text")? {
                                    let Some(action) = sink.drag_action(node, DragKind::Text) else {
                                        args.SetAcceptedOperation(DataPackageOperation::None)?;
                                        deferral.Complete()?;
                                        sink.enqueue(
                                            node,
                                            EventId::#event_id,
                                            revision,
                                            EventPayload::DroppedData(DroppedData::Unsupported),
                                        );
                                        return Ok(());
                                    };
                                    args.SetAcceptedOperation(native_drag_operation(
                                        action.operation,
                                    ))?;
                                    let operation = data.GetTextAsync()?;
                                    let sender = sink.begin_async_event(
                                        node,
                                        EventId::#event_id,
                                        revision,
                                        deferral.clone(),
                                    );
                                    let completion = sender.clone();
                                    if let Err(error) = operation.when(move |result| {
                                        _ = completion.complete(
                                            result
                                                .map(|text| {
                                                    DroppedData::Text(text.to_string_lossy())
                                                })
                                                .map_err(native_error),
                                        );
                                    }) {
                                        _ = sender.complete(Err(native_error(error)));
                                    }
                                    Ok(())
                                } else {
                                    args.SetAcceptedOperation(DataPackageOperation::None)?;
                                    deferral.Complete()?;
                                    sink.enqueue(
                                        node,
                                        EventId::#event_id,
                                        revision,
                                        EventPayload::DroppedData(DroppedData::Unsupported),
                                    );
                                    Ok(())
                                }
                            })();
                            if result.is_err() {
                                deferral.Complete()?;
                            }
                            result
                        });
                    match result {
                        Ok(()) => {}
                        Err(error) => {
                            sink.error(
                                node, EventId::#event_id, revision, native_error(error),
                            );
                        }
                    };
                }
            }
        }
        EventPayloadSource::SenderProperty {
            interface: property_interface,
            property,
        } => {
            let routed_action = inspectable_routed_event(event)
                .then(|| match property_interface.as_str() {
                    "Microsoft.UI.Xaml.Controls.IPasswordBox" if property == "Password" => {
                        Some(quote! { RoutedEventAction::Password(event_source) })
                    }
                    "Microsoft.UI.Xaml.Controls.IToggleSwitch" if property == "IsOn" => {
                        Some(quote! { RoutedEventAction::ToggleSwitch(event_source) })
                    }
                    _ => None,
                })
                .flatten();
            let selection_index_action = (event.conversion
                == EventPayloadConversion::SelectionIndex
                && inspectable_selection_changed_event(event))
            .then(|| match property_interface.as_str() {
                "Microsoft.UI.Xaml.Controls.Primitives.ISelector" => {
                    Some(quote! { SelectionChangedAction::IndexSelector(event_source) })
                }
                "Microsoft.UI.Xaml.Controls.IRadioButtons" => {
                    Some(quote! { SelectionChangedAction::IndexRadioButtons(event_source) })
                }
                "Microsoft.UI.Xaml.Controls.IPivot" => {
                    Some(quote! { SelectionChangedAction::IndexPivot(event_source) })
                }
                "Microsoft.UI.Xaml.Controls.ITabView" => {
                    Some(quote! { SelectionChangedAction::IndexTabView(event_source) })
                }
                _ => None,
            })
            .flatten();
            let toggle_button_checked = matches!(
                &event.subscription,
                EventSubscription::PropertyChanged { .. }
            ) && property_interface
                == "Microsoft.UI.Xaml.Controls.Primitives.IToggleButton"
                && property == "IsChecked";
            let property = ident(property);
            let default_property_interface = is_default_interface(control, property_interface);
            let property_interface = path_ident(property_interface);
            let event_source = if routed_action.is_some()
                || selection_index_action.is_some()
                || toggle_button_checked
            {
                Some(quote! {
                    let event_source = value
                        .cast::<#property_interface>()
                        .map_err(native_error)?;
                })
            } else if shared_event_source {
                None
            } else if default_property_interface {
                Some(quote! {
                    let event_source = (*value).clone();
                })
            } else {
                Some(quote! {
                    let event_source = value
                        .cast::<#property_interface>()
                        .map_err(native_error)?;
                })
            };
            if let Some(action) = routed_action {
                quote! {
                    {
                        #event_source
                        routed_event_handler(
                            sink,
                            node,
                            EventId::#event_id,
                            revision,
                            #action,
                        )
                    }
                }
            } else if toggle_button_checked {
                quote! {
                    {
                        #event_source
                        toggle_button_checked_handler(
                            sink,
                            node,
                            EventId::#event_id,
                            revision,
                            event_source,
                        )
                    }
                }
            } else if let Some(action) = selection_index_action {
                quote! {
                    {
                        #event_source
                        selection_changed_handler(
                            sink,
                            node,
                            EventId::#event_id,
                            revision,
                            #action,
                        )
                    }
                }
            } else if event.conversion == EventPayloadConversion::Selection {
                if inspectable_selection_changed_event(event) && control.name == "ListBox" {
                    quote! {
                        {
                            #event_source
                            selection_changed_handler(
                                sink,
                                node,
                                EventId::#event_id,
                                revision,
                                SelectionChangedAction::ListBox(event_source),
                            )
                        }
                    }
                } else {
                    let dispatch = generate_selection_dispatch(control, event);
                    quote! {
                        {
                            #event_source
                            move |_, _| {
                                let value = event_source.#property();
                                #dispatch
                            }
                        }
                    }
                }
            } else if text_input_probe {
                quote! {
                    {
                        #event_source
                        move |_, _| {
                            #[cfg(feature = "test")]
                            test::record_live_input_probe_stage(
                                test::LiveInputProbeStage::NativeTextChanged,
                            );
                            match event_source.#property() {
                                Ok(value) => {
                                    #[cfg(feature = "test")]
                                    test::record_live_input_probe_stage(
                                        test::LiveInputProbeStage::NativeTextReady,
                                    );
                                    #enqueue_payload;
                                }
                                #nullable_error
                                Err(error) => sink.error(
                                    node,
                                    EventId::#event_id,
                                    revision,
                                    native_error(error),
                                ),
                            }
                        }
                    }
                }
            } else {
                quote! {
                    {
                        #event_source
                        move |_, _| match event_source.#property() {
                            Ok(value) => #enqueue_payload,
                            #nullable_error
                            Err(error) => sink.error(
                                node,
                                EventId::#event_id,
                                revision,
                                native_error(error),
                            ),
                        }
                    }
                }
            }
        }
        EventPayloadSource::EventArgsProperty {
            interface: _,
            property,
        } => {
            let property = ident(property);
            if event.conversion == EventPayloadConversion::Selection {
                let dispatch = generate_selection_dispatch(control, event);
                quote! {
                    move |_, args| {
                        if let Some(args) = args.as_ref() {
                            let value = args.#property();
                            #dispatch
                        }
                    }
                }
            } else {
                quote! {
                    move |_, args| {
                        #lifecycle_completion
                        if let Some(args) = args.as_ref() {
                            match args.#property() {
                                Ok(value) => #enqueue_payload,
                                #nullable_error
                                Err(error) => sink.error(
                                    node,
                                    EventId::#event_id,
                                    revision,
                                    native_error(error),
                                ),
                            }
                        }
                    }
                }
            }
        }
        EventPayloadSource::EventArgsInspectableString {
            interface: _,
            property,
        } => {
            let property = ident(property);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .#property()
                            .and_then(|value| {
                                value.cast::<windows_reference::IReference<windows_core::HSTRING>>()
                            })
                            .and_then(|value| value.Value())
                        {
                            Ok(value) => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::Str(value.to_string_lossy()),
                            ),
                            Err(error) => sink.error(
                                node,
                                EventId::#event_id,
                                revision,
                                native_error(error),
                            ),
                        }
                    }
                }
            }
        }
        EventPayloadSource::SenderRichEditText {
            interface: property_interface,
            property: _,
        } => {
            assert!(inspectable_routed_event(event));
            let property_interface = path_ident(property_interface);
            quote! {
                {
                    let event_source = value
                        .cast::<#property_interface>()
                        .map_err(native_error)?;
                    routed_event_handler(
                        sink,
                        node,
                        EventId::#event_id,
                        revision,
                        RoutedEventAction::RichEdit(event_source),
                    )
                }
            }
        }
        EventPayloadSource::PointerEvent => {
            let phase = match event.name.as_str() {
                "PointerPressed" => quote! { PointerEventPhase::Press },
                "PointerReleased" => quote! { PointerEventPhase::Release },
                _ => quote! { PointerEventPhase::Plain },
            };
            quote! {
                pointer_event_handler(
                    sink,
                    node,
                    EventId::#event_id,
                    revision,
                    value.cast::<UIElement>().map_err(native_error)?,
                    #phase,
                )
            }
        }
        EventPayloadSource::KeyEvent => quote! {
            key_event_handler(
                sink,
                node,
                EventId::#event_id,
                revision,
            )
        },
        EventPayloadSource::CharacterEvent => quote! {
            move |_, args| {
                let result = args
                    .as_ref()
                    .ok_or_else(windows_core::Error::empty)
                    .and_then(character_event_info);
                match result {
                    Ok(info) => {
                        let handled =
                            sink.route_character(node, EventId::#event_id, revision, info);
                        if let Some(args) = args.as_ref()
                            && let Err(error) = args.SetHandled(handled)
                        {
                            sink.error(
                                node,
                                EventId::#event_id,
                                revision,
                                native_error(error),
                            );
                        }
                    }
                    Err(error) => {
                        sink.error(
                            node,
                            EventId::#event_id,
                            revision,
                            native_error(error),
                        );
                    }
                }
            }
        },
        EventPayloadSource::FocusEvent => {
            let got_focus = event.name == "GotFocus";
            quote! {
                routed_event_handler(
                    sink,
                    node,
                    EventId::#event_id,
                    revision,
                    RoutedEventAction::Focus(
                        value.cast::<UIElement>().map_err(native_error)?,
                        #got_focus,
                    ),
                )
            }
        }
        EventPayloadSource::EventArgsItemTag {
            interface: _,
            property,
        } => {
            let property = ident(property);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .#property()
                            .and_then(|item| item.cast::<IFrameworkElement>())
                            .and_then(|item| item.Tag())
                            .and_then(|value| {
                                value.cast::<windows_reference::IReference<windows_core::HSTRING>>()
                            })
                            .and_then(|value| value.Value())
                        {
                            Ok(value) => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::Str(value.to_string_lossy()),
                            ),
                            Err(error) if error.code().is_ok() => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::Str(String::new()),
                            ),
                            Err(error) => sink.error(
                                node,
                                EventId::#event_id,
                                revision,
                                native_error(error),
                            ),
                        }
                    }
                }
            }
        }
        EventPayloadSource::EventArgsTreeNodeContent {
            interface: _,
            property,
        } => {
            let property = ident(property);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .#property()
                            .and_then(|node| node.cast::<ITreeViewNode>())
                            .map_err(native_error)
                            .and_then(|node| sink.tree_node_label(&node))
                        {
                            Ok(value) => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::Str(value),
                            ),
                            Err(error) => {
                                sink.error(node, EventId::#event_id, revision, error);
                            }
                        }
                    }
                }
            }
        }
        EventPayloadSource::SenderItemTags {
            interface: property_interface,
            property,
        } => {
            if matches!(control.name.as_str(), "ListView" | "GridView")
                && event.name == "DragItemsCompleted"
                && property_interface == "Microsoft.UI.Xaml.Controls.IItemsControl"
                && property == "Items"
            {
                quote! {
                    list_view_items_changed_handler(
                        sink,
                        node,
                        EventId::#event_id,
                        revision,
                    )
                }
            } else {
                let property = ident(property);
                let property_interface = path_ident(property_interface);
                quote! {
                    move |sender, _| {
                        if let Some(sender) = sender.as_ref() {
                            let result = sender
                                .cast::<#property_interface>()
                                .and_then(|sender| sender.#property())
                                .and_then(|items| {
                                    let mut tags = Vec::with_capacity(items.Size()? as usize);
                                    for index in 0..items.Size()? {
                                        let tag = items
                                            .GetAt(index)?
                                            .cast::<IFrameworkElement>()?
                                            .Tag()?
                                            .cast::<windows_reference::IReference<windows_core::HSTRING>>()?
                                            .Value()?;
                                        tags.push(tag.to_string_lossy());
                                    }
                                    Ok(tags)
                                });
                            match result {
                                Ok(value) => sink.enqueue(
                                    node,
                                    EventId::#event_id,
                                    revision,
                                    EventPayload::StrList(std::rc::Rc::new(value)),
                                ),
                                Err(error) => sink.error(
                                    node,
                                    EventId::#event_id,
                                    revision,
                                    native_error(error),
                                ),
                            }
                        }
                    }
                }
            }
        }
    };
    let subscribe = if is_default_interface(control, &event.interface) {
        let event_source =
            shared_event_source.then(|| quote! { let event_source = (*value).clone(); });
        quote! {
            {
                #event_source
                value.#method(#callback)
            }
        }
    } else {
        let event_source =
            shared_event_source.then(|| quote! { let event_source = source.clone(); });
        quote! {
            {
                let source = value.cast::<#interface>().map_err(native_error)?;
                #event_source
                source.#method(#callback)
            }
        }
    };
    match &event.subscription {
        EventSubscription::Metadata => quote! {
            (Handle::#control_name(value), EventId::#event_id) => {
                #subscribe
                    .map(|revoker| NativeSubscription::Event {
                        _revoker: revoker,
                        revision,
                    })
                    .map_err(native_error)
            }
        },
        EventSubscription::PropertyChanged {
            property,
            static_owner,
        } => {
            let property_method = ident(&format!("{property}Property"));
            let static_owner = path_ident(static_owner);
            quote! {
                (Handle::#control_name(value), EventId::#event_id) => {
                    let object = value
                        .cast::<DependencyObject>()
                        .map_err(native_error)?;
                    let property =
                        bindings::#static_owner::#property_method().map_err(native_error)?;
                    let callback = DependencyPropertyChangedCallback::new(#callback);
                    let token = object
                        .RegisterPropertyChangedCallback(&property, &callback)
                        .map_err(native_error)?;
                    Ok(NativeSubscription::Property {
                        object,
                        property,
                        token,
                    })
                }
            }
        }
    }
}

fn is_default_interface(control: &ResolvedControl, interface: &str) -> bool {
    interface.ends_with(&format!(".I{}", control.name))
}

fn property_receiver(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    if is_default_interface(control, &property.interface) {
        quote! { control }
    } else {
        let interface = path_ident(&property.interface);
        quote! { control.cast::<#interface>().map_err(native_error)? }
    }
}

fn is_handwritten_property_adapter(property: &ResolvedProperty) -> bool {
    matches!(
        property.adapter,
        Some(
            PropertyAdapter::PointerCapture
                | PropertyAdapter::PointerFocus
                | PropertyAdapter::DropPolicy
                | PropertyAdapter::ResourceOverrides
                | PropertyAdapter::KeyAccelerators
        )
    )
}

fn generate_set_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let value_variant = ident(&property.value);
    let receiver = property_receiver(control, property);
    let setter = ident(&format!("Set{}", property.name));
    if property.adapter == Some(PropertyAdapter::FontWeight) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::FontWeight(value),
            ) => #receiver
                .#setter(bindings::FontWeight { weight: value.get() })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::HorizontalContentAlignment) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::HorizontalAlignment(value),
            ) => #receiver
                .#setter(match value {
                    crate::HorizontalAlignment::Left => bindings::HorizontalAlignment::Left,
                    crate::HorizontalAlignment::Center => bindings::HorizontalAlignment::Center,
                    crate::HorizontalAlignment::Right => bindings::HorizontalAlignment::Right,
                    crate::HorizontalAlignment::Stretch => bindings::HorizontalAlignment::Stretch,
                })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::VerticalContentAlignment) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::VerticalAlignment(value),
            ) => #receiver
                .#setter(match value {
                    crate::VerticalAlignment::Top => bindings::VerticalAlignment::Top,
                    crate::VerticalAlignment::Center => bindings::VerticalAlignment::Center,
                    crate::VerticalAlignment::Bottom => bindings::VerticalAlignment::Bottom,
                    crate::VerticalAlignment::Stretch => bindings::VerticalAlignment::Stretch,
                })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::SelectionIndex) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::SelectionIndex(value),
            ) => {
                let value = native_selection_index(*value)?;
                #receiver.#setter(value).map_err(native_error)
            }
        };
    }
    if matches!(
        property.adapter,
        Some(PropertyAdapter::NumberBoxValue | PropertyAdapter::RatingValue)
    ) {
        let conversion = if property.adapter == Some(PropertyAdapter::NumberBoxValue) {
            quote! { native_number_box_value(*value) }
        } else {
            quote! { native_rating_value(*value) }
        };
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::OptionalF64(value),
            ) => #receiver
                .#setter(#conversion)
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::ImplicitOpacityTransition) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Duration(value),
            ) => {
                let duration = windows_time::TimeSpan::try_from(*value)
                    .map_err(|_| RuntimeError::UnsupportedKind)?;
                let transition = ScalarTransition::new().map_err(native_error)?;
                transition.SetDuration(duration).map_err(native_error)?;
                #receiver.#setter(&transition).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ImplicitScale) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::F64(value),
            ) => {
                let control = #receiver;
                let element = control
                    .cast::<IFrameworkElement>()
                    .map_err(native_error)?;
                let width = element.ActualWidth().map_err(native_error)? as f32;
                let height = element.ActualHeight().map_err(native_error)? as f32;
                control
                    .SetCenterPoint(windows_numerics::Vector3 {
                        x: width / 2.0,
                        y: height / 2.0,
                        z: 0.0,
                    })
                    .map_err(native_error)?;
                let value = *value as f32;
                control
                    .#setter(windows_numerics::Vector3 {
                        x: value,
                        y: value,
                        z: 1.0,
                    })
                    .map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ImplicitScaleTransition) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Duration(value),
            ) => {
                let duration = windows_time::TimeSpan::try_from(*value)
                    .map_err(|_| RuntimeError::UnsupportedKind)?;
                let transition = Vector3Transition::new().map_err(native_error)?;
                transition.SetDuration(duration).map_err(native_error)?;
                #receiver.#setter(&transition).map_err(native_error)
            }
        };
    }
    if property.value == "Color" {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Color(value),
            ) => {
                let value = bindings::Color {
                    a: value.a,
                    r: value.r,
                    g: value.g,
                    b: value.b,
                };
                #receiver.#setter(value).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ImageUri) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Str(value),
            ) => {
                let uri = Uri::CreateUri(value).map_err(native_error)?;
                let path = value.split(['?', '#']).next().unwrap_or(value);
                let source: ImageSource = if path.to_ascii_lowercase().ends_with(".svg") {
                    let source = SvgImageSource::new().map_err(native_error)?;
                    source.SetUriSource(&uri).map_err(native_error)?;
                    source.cast().map_err(native_error)?
                } else {
                    let source = BitmapImage::new().map_err(native_error)?;
                    source.SetUriSource(&uri).map_err(native_error)?;
                    source.cast().map_err(native_error)?
                };
                control.#setter(&source).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::RichEditText) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Str(value),
            ) => set_rich_edit_text(control, value).map(|_| ())
        };
    }
    if property.adapter == Some(PropertyAdapter::RichTextBlocks) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::RichText(value),
            ) => {
                let blocks: windows_collections::IVector<Block> = control
                    .Blocks()
                    .and_then(|blocks| blocks.cast())
                    .map_err(native_error)?;
                blocks.Clear().map_err(native_error)?;
                let append_run =
                    |inlines: &windows_collections::IVector<Inline>,
                     value: &crate::RichTextRun| {
                    let run = Run::new().map_err(native_error)?;
                    run.SetText(&value.text).map_err(native_error)?;
                    if value.is_bold {
                        run.cast::<ITextElement>()
                            .and_then(|run| run.SetFontWeight(bindings::FontWeight { weight: 700 }))
                            .map_err(native_error)?;
                    }
                    if value.is_italic {
                        run.cast::<ITextElement>()
                            .and_then(|run| run.SetFontStyle(bindings::FontStyle::Italic))
                            .map_err(native_error)?;
                    }
                    let run = run.cast::<Inline>().map_err(native_error)?;
                    inlines.Append(&run).map_err(native_error)
                };
                for paragraph in value.paragraphs.iter() {
                    let native = Paragraph::new().map_err(native_error)?;
                    let inlines: windows_collections::IVector<Inline> = native
                        .Inlines()
                        .and_then(|inlines| inlines.cast())
                        .map_err(native_error)?;
                    for inline in &paragraph.inlines {
                        match inline {
                            crate::RichTextInline::Run(value) => append_run(&inlines, value)?,
                            crate::RichTextInline::Hyperlink(value) => {
                                let hyperlink = Hyperlink::new().map_err(native_error)?;
                                let uri = Uri::CreateUri(&value.uri).map_err(native_error)?;
                                hyperlink.SetNavigateUri(&uri).map_err(native_error)?;
                                let hyperlink_inlines: windows_collections::IVector<Inline> = hyperlink
                                    .cast::<ISpan>()
                                    .and_then(|hyperlink| hyperlink.Inlines())
                                    .and_then(|inlines| inlines.cast())
                                    .map_err(native_error)?;
                                append_run(
                                    &hyperlink_inlines,
                                    &crate::RichTextRun::plain(&value.text),
                                )?;
                                let hyperlink =
                                    hyperlink.cast::<Inline>().map_err(native_error)?;
                                inlines.Append(&hyperlink).map_err(native_error)?;
                            }
                            crate::RichTextInline::LineBreak => {
                                let line_break = LineBreak::new().map_err(native_error)?;
                                let line_break =
                                    line_break.cast::<Inline>().map_err(native_error)?;
                                inlines.Append(&line_break).map_err(native_error)?;
                            }
                        }
                    }
                    let native = native.cast::<Block>().map_err(native_error)?;
                    blocks.Append(&native).map_err(native_error)?;
                }
                Ok(())
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::PathData) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Str(value),
            ) => {
                let type_name = TypeName {
                    name: "Microsoft.UI.Xaml.Media.Geometry".into(),
                    kind: TypeKind::Metadata,
                };
                let value = windows_reference::IReference::<windows_core::HSTRING>::from(
                    value.as_str(),
                );
                let parsed = XamlBindingHelper::ConvertValue(&type_name, &value)
                    .and_then(|value| value.cast::<bindings::Geometry>())
                    .map_err(native_error)?;
                control.#setter(&parsed).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::InspectableString) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Str(value),
            ) => {
                let value = windows_reference::IReference::from(value.as_str());
                #receiver.#setter(&value).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::InspectableStringList) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::StrList(value),
            ) => {
                let values: Vec<Option<windows_core::IInspectable>> = value
                    .iter()
                    .map(|value| {
                        Some(windows_reference::IReference::from(value.as_str()).into())
                    })
                    .collect();
                let values: windows_collections::IVector<windows_core::IInspectable> =
                    values.into();
                #receiver.#setter(&values).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::Uri) {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::Str(value),
            ) => {
                let value = Uri::CreateUri(value).map_err(native_error)?;
                #receiver.#setter(&value).map_err(native_error)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ResourceStyle) {
        let value_type = ident(&property.value);
        let variants = property.resource_style_variants.iter().map(|variant| {
            let variant_name = ident(&variant.name);
            if let Some(resource) = variant.resource.as_deref() {
                quote! {
                    crate::#value_type::#variant_name => Some(#resource)
                }
            } else {
                quote! {
                    crate::#value_type::#variant_name => None
                }
            }
        });
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::#value_variant(value),
            ) => {
                let resource = match value {
                    #(#variants),*
                };
                let element = control
                    .cast::<IFrameworkElement>()
                    .map_err(native_error)?;
                set_resource_style(&element, resource)
            }
        };
    }
    if property.theme_style {
        return quote! {
            (
                Handle::#control_name(control),
                PropertyId::#property_id,
                PropertyValue::#value_variant(crate::Brush::Solid(color)),
            ) => {
                let brush = SolidColorBrush::new().map_err(native_error)?;
                brush
                    .SetColor(bindings::Color {
                        a: color.a,
                        r: color.r,
                        g: color.g,
                        b: color.b,
                    })
                    .map_err(native_error)?;
                #receiver.#setter(&brush).map_err(native_error)
            }
        };
    }
    let value = if property.value == "Str" {
        quote! { value }
    } else if property.value == "Thickness" {
        quote! {
            {
                let [left, top, right, bottom] = value.values();
                bindings::Thickness {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
        }
    } else if property.value == "CornerRadius" {
        quote! {
            {
                let [top_left, top_right, bottom_right, bottom_left] = value.values();
                bindings::CornerRadius {
                    top_left,
                    top_right,
                    bottom_right,
                    bottom_left,
                }
            }
        }
    } else if let Some((native_type, field)) = &property.native_wrapper {
        let native_type = ident(native_type.rsplit('.').next().unwrap());
        let field = ident(field);
        quote! { bindings::#native_type { #field: *value } }
    } else if property.nullable_bool {
        quote! { Some(*value) }
    } else if property.enum_variants.is_empty() {
        quote! { *value }
    } else {
        let value_type = path_ident(&property.value);
        let variants = property.enum_variants.iter().map(|variant| {
            let variant = ident(variant);
            quote! {
                crate::#value_type::#variant => bindings::#value_type::#variant
            }
        });
        quote! {
            match value {
                #(#variants),*
            }
        }
    };
    quote! {
        (
            Handle::#control_name(control),
            PropertyId::#property_id,
            PropertyValue::#value_variant(value),
        ) => #receiver.#setter(#value).map_err(native_error)
    }
}

fn generate_clear_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let owner = path_ident(&property.static_owner);
    let property_method = ident(&format!("{}Property", property.name));
    let receiver = property_receiver(control, property);
    if property.adapter == Some(PropertyAdapter::ImplicitOpacityTransition)
        || property.adapter == Some(PropertyAdapter::ImplicitScaleTransition)
    {
        let setter = ident(&format!("Set{}", property.name));
        let transition = if property.adapter == Some(PropertyAdapter::ImplicitOpacityTransition) {
            ident("ScalarTransition")
        } else {
            ident("Vector3Transition")
        };
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => #receiver
                .#setter(None::<&#transition>)
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::ImplicitScale) {
        let setter = ident(&format!("Set{}", property.name));
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => #receiver
                .#setter(windows_numerics::Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::RichEditText) {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => {
                set_rich_edit_text(control, "").map(|_| ())
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::RichTextBlocks) {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .Blocks()
                .and_then(|blocks| blocks.Clear())
                .map_err(native_error)
        };
    }
    quote! {
        (Handle::#control_name(_), PropertyId::#property_id) =>
            clear_value(handle, bindings::#owner::#property_method)
    }
}

fn filter_path(value: &str) -> String {
    value.replace('.', "::")
}

fn ident(value: &str) -> Ident {
    Ident::new(value, Span::call_site())
}

fn path_ident(value: &str) -> Ident {
    ident(value.rsplit_once('.').map_or(value, |(_, name)| name))
}

fn event_handler_type(value: &crate::metadata::EventHandlerType) -> TokenStream {
    match value {
        crate::metadata::EventHandlerType::Inspectable => {
            quote! { windows_core::IInspectable }
        }

        crate::metadata::EventHandlerType::Binding(value) => {
            let value = ident(value);
            quote! { bindings::#value }
        }
    }
}

fn inspectable_routed_event(event: &ResolvedEvent) -> bool {
    matches!(
        &event.handler,
        Some((
            crate::metadata::EventHandlerType::Inspectable,
            crate::metadata::EventHandlerType::Binding(args),
        )) if args == "RoutedEventArgs"
    )
}

fn inspectable_selection_changed_event(event: &ResolvedEvent) -> bool {
    matches!(
        &event.handler,
        Some((
            crate::metadata::EventHandlerType::Inspectable,
            crate::metadata::EventHandlerType::Binding(args),
        )) if args == "SelectionChangedEventArgs"
    )
}
