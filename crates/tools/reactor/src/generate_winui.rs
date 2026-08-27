use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

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
                entries.insert("Microsoft::UI::Xaml::Controls::UIElementCollection".to_string());
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
                Some(PropertyAdapter::ContentDialogResult)
                | Some(PropertyAdapter::ImageUri)
                | Some(PropertyAdapter::InspectableString)
                | Some(PropertyAdapter::InspectableStringList)
                | Some(PropertyAdapter::ItemTag)
                | Some(PropertyAdapter::ItemTags)
                | Some(PropertyAdapter::NavigationDisplayMode)
                | Some(PropertyAdapter::PathData)
                | Some(PropertyAdapter::PointerCapture)
                | Some(PropertyAdapter::PointerEvent)
                | Some(PropertyAdapter::DragInfo)
                | Some(PropertyAdapter::DropData)
                | Some(PropertyAdapter::DropPolicy)
                | Some(PropertyAdapter::ResourceOverrides)
                | Some(PropertyAdapter::ResourceStyle)
                | Some(PropertyAdapter::RichEditText)
                | Some(PropertyAdapter::RichTextBlocks)
                | Some(PropertyAdapter::TreeNodeContent)
                | Some(PropertyAdapter::Uri)
                | None => {}
            }
            if !matches!(
                property.adapter,
                Some(
                    PropertyAdapter::ResourceStyle
                        | PropertyAdapter::RichEditText
                        | PropertyAdapter::RichTextBlocks
                        | PropertyAdapter::PointerCapture
                        | PropertyAdapter::DropPolicy
                        | PropertyAdapter::KeyAccelerators
                        | PropertyAdapter::ResourceOverrides
                )
            ) {
                entries.insert(format!(
                    "{}::put_{}",
                    filter_path(&property.interface),
                    property.name
                ));
            }
            if property.observes_feedback {
                entries.insert(format!(
                    "{}::get_{}",
                    filter_path(&property.interface),
                    property.name
                ));
            }
            if !matches!(
                property.adapter,
                Some(
                    PropertyAdapter::ImplicitOpacityTransition
                        | PropertyAdapter::ImplicitScale
                        | PropertyAdapter::ImplicitScaleTransition
                        | PropertyAdapter::RichEditText
                        | PropertyAdapter::RichTextBlocks
                        | PropertyAdapter::PointerCapture
                        | PropertyAdapter::DropPolicy
                        | PropertyAdapter::KeyAccelerators
                        | PropertyAdapter::ResourceOverrides
                )
            ) {
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
                entries
                    .insert("Windows::Foundation::IUriRuntimeClass::get_AbsoluteUri".to_string());
                entries.insert("Microsoft::UI::Xaml::Media::ImageSource".to_string());
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::BitmapImage::CreateInstance".to_string(),
                );
                entries.insert(
                    "Microsoft::UI::Xaml::Media::Imaging::IBitmapImage::{\
                     get_UriSource, put_UriSource}"
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
                entries.insert("Microsoft::UI::Xaml::Controls::IImage::get_Source".to_string());
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
            if let Some(property) = &event.property {
                let interface = match &event.source {
                    EventPayloadSource::SenderProperty { interface }
                    | EventPayloadSource::EventArgsProperty { interface }
                    | EventPayloadSource::DragInfo { interface }
                    | EventPayloadSource::DropData { interface }
                    | EventPayloadSource::EventArgsInspectableString { interface }
                    | EventPayloadSource::EventArgsItemTag { interface }
                    | EventPayloadSource::EventArgsTreeNodeContent { interface }
                    | EventPayloadSource::SenderRichEditText { interface } => interface,
                    EventPayloadSource::PointerEvent
                    | EventPayloadSource::SenderItemTags { .. } => {
                        continue;
                    }
                    EventPayloadSource::Unit => continue,
                };
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
            if let EventPayloadSource::SenderItemTags {
                interface,
                property,
            } = &event.source
            {
                entries.insert(format!("{}::get_{}", filter_path(interface), property));
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
            let accessor = match slot.shape {
                SlotShape::Single(_) => "put",
                SlotShape::Collection => "get",
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
    let virtual_create = schema
        .controls
        .iter()
        .filter(|control| matches!(control.role, Role::Virtual))
        .map(|control| {
            let name = ident(&control.name);
            quote! {
                MountedKind::#name => return Err(RuntimeError::UnsupportedKind)
            }
        });
    let ui_elements = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { Self::#name(value) => value.cast() }
    });
    let dependency_objects = native_controls.iter().map(|control| {
        let name = ident(&control.name);
        quote! { Self::#name(value) => value.cast() }
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
            .map(move |property| generate_set_property(control, property))
    });
    let clear_properties = schema.controls.iter().flat_map(|control| {
        control
            .properties
            .iter()
            .filter(|property| property.clearable)
            .map(move |property| generate_clear_property(control, property))
    });
    let read_properties = schema.controls.iter().flat_map(|control| {
        control
            .properties
            .iter()
            .filter(|property| property.observes_feedback)
            .map(move |property| generate_read_property(control, property))
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
            .filter(|slot| matches!(slot.shape, SlotShape::Single(_)))
            .map(move |slot| generate_set_slot(control, slot))
    });
    let collection_slots = schema.controls.iter().flat_map(|control| {
        control
            .slots
            .iter()
            .filter(|slot| matches!(slot.shape, SlotShape::Collection))
            .map(move |slot| generate_slot_collection(control, slot))
    });
    let collection_items = schema
        .controls
        .iter()
        .flat_map(|control| &control.slots)
        .filter_map(|slot| slot.collection_item.as_deref())
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
                .and_then(|value| value.cast::<windows_core::IInspectable>())
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
                FeedbackContract::SynchronousExact => {
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
                FeedbackContract::SynchronousNormalized => Some(quote! {
                    (PropertyId::#property_id, Some(_)) => {
                        Some((
                            EventId::#event_id,
                            FeedbackExpectation::Normalized { observation: None },
                        ))
                    }
                }),
                _ => unreachable!(),
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
                FeedbackContract::SynchronousExact => {
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
                FeedbackContract::SynchronousNormalized => Some(quote! {
                    (PropertyId::#property_id, None) => {
                        Some((
                            EventId::#event_id,
                            FeedbackExpectation::Normalized { observation: None },
                        ))
                    }
                }),
                _ => unreachable!(),
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
        let slot = ident(&format!("{}{}", control.name, selection.slot));
        let interface = path_ident(&selection.owner_interface);
        let getter = ident(&selection.selected_item_property);
        let selected = if selection.selected_item.is_some() {
            quote! {
                selected
                    .cast::<windows_core::IInspectable>()
                    .map(Some)
                    .map_err(native_error)
            }
        } else {
            quote! { Ok(Some(selected)) }
        };
        Some(quote! {
            (Handle::#control_name(value), SlotId::#slot) => {
                match value.cast::<#interface>().and_then(|value| value.#getter()) {
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
        let slot = ident(&format!("{}{}", control.name, selection.slot));
        let interface = path_ident(&selection.owner_interface);
        let setter = ident(&format!("Set{}", selection.selected_item_property));
        let set = if let Some(item) = selection.selected_item.as_deref() {
            let item = path_ident(item);
            quote! {
                value.cast::<#interface>().and_then(|value| {
                    let selected = selected.cast::<bindings::#item>()?;
                    value.#setter(&selected)
                })
            }
        } else {
            quote! {
                value
                    .cast::<#interface>()
                    .and_then(|value| value.#setter(selected))
            }
        };
        Some(quote! {
            (Handle::#control_name(value), SlotId::#slot) => #set.map_err(native_error),
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
                    #(#virtual_create),*
                })
            }

            pub fn ui_element(&self) -> windows_core::Result<UIElement> {
                match self {
                    #(#ui_elements),*
                }
            }

            pub fn dependency_object(&self) -> windows_core::Result<IDependencyObject> {
                match self {
                    #(#dependency_objects),*
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
            let dependency_object = handle.dependency_object().map_err(native_error)?;
            match (handle, property) {
                #(#clear_properties,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }

        #[cfg(feature = "test")]
        pub fn read_property(
            handle: &Handle,
            property: PropertyId,
        ) -> Result<PropertyValue, RuntimeError> {
            match (handle, property) {
                #(#read_properties,)*
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
            match (handle, selection.slot) {
                #(#selected_items,)*
                _ => Ok(None),
            }
        }

        pub fn set_selected_item(
            handle: &Handle,
            selection: SelectionDescriptor,
            selected: &windows_core::IInspectable,
        ) -> Result<(), RuntimeError> {
            match (handle, selection.slot) {
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

    format!("// Generated by `tool_reactor`. Do not edit.\n\n{tokens}\n")
}

fn generate_set_content(
    control: &ResolvedControl,
    content: &crate::schema::ResolvedContent,
) -> TokenStream {
    let control_name = ident(&control.name);
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
    let set = if content.interface.ends_with(&format!(".I{}", control.name)) {
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
    let get = if slot.interface.ends_with(&format!(".I{}", control.name)) {
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
    let get = if slot.collection_cast {
        quote! {
            #get?
                .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                .map_err(native_error)
        }
    } else {
        get
    };
    let get = if slot.collection_observable {
        let item = path_ident(slot.collection_item.as_deref().unwrap());
        quote! {
            #get?
                .cast::<windows_collections::IVector<bindings::#item>>()
                .map_err(native_error)
        }
    } else {
        get
    };
    let collection = if let Some(item) = slot.collection_item.as_deref() {
        let variant = ident(item.rsplit('.').next().unwrap());
        quote! { SlotCollection::#variant(#get?) }
    } else {
        quote! { SlotCollection::Inspectable(#get?) }
    };
    quote! { (Handle::#control_name(control), SlotId::#slot_id) => Ok(#collection) }
}

fn generate_set_slot(control: &ResolvedControl, slot: &crate::schema::ResolvedSlot) -> TokenStream {
    let control_name = ident(&control.name);
    let slot_id = ident(&format!("{}{}", control.name, slot.name));
    let interface = path_ident(&slot.interface);
    let setter = ident(&format!("Set{}", slot.name));
    let SlotShape::Single(target) = slot.shape else {
        unreachable!()
    };
    let value = match target {
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
    let set = if slot.interface.ends_with(&format!(".I{}", control.name)) {
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

fn generate_payload_value(conversion: &EventPayloadConversion) -> TokenStream {
    match conversion {
        EventPayloadConversion::Identity => quote! { value },
        EventPayloadConversion::Field(field) => {
            let field = ident(field);
            quote! { value.#field }
        }
        EventPayloadConversion::Selection => quote! { value },
    }
}

fn generate_selection_dispatch(control: &ResolvedControl, event: &ResolvedEvent) -> TokenStream {
    let event_id = ident(&format!("{}{}", control.name, event.name));
    let payload = ident(&event.payload);
    let erase_item = if control.selection.as_ref().unwrap().selected_item.is_some() {
        quote! {
            let item = match item.cast::<windows_core::IInspectable>() {
                Ok(item) => item,
                Err(error) => {
                    sink.error(
                        node,
                        EventId::#event_id,
                        revision,
                        native_error(error),
                    );
                    return;
                }
            };
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
                    Err(error) => sink.error(
                        node,
                        EventId::#event_id,
                        revision,
                        error,
                    ),
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
    let method = ident(&event.name);
    let payload = ident(&event.payload);
    let pointer_capture = (event.name == "PointerPressed").then(|| {
        quote! {
            info.capture_succeeded = match sink.capture_pointer_on_press(node, &element, args) {
                Ok(value) => value,
                Err(error) => {
                    sink.error(node, EventId::#event_id, revision, error);
                    return;
                }
            };
        }
    });
    let pointer_release = (event.name == "PointerReleased").then(|| {
        quote! {
            if let Err(error) = sink.release_pointer_after_event(node, &element, args) {
                sink.error(node, EventId::#event_id, revision, error);
                return;
            }
        }
    });
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
        generate_payload_value(&event.conversion)
    };
    let content_dialog_closed = control.lifecycle == Some(crate::schema::Lifecycle::ContentDialog)
        && event.name == "Closed";
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
    let callback = match &event.source {
        EventPayloadSource::Unit => quote! {
            move |_, _| {
                sink.enqueue(
                    node,
                    EventId::#event_id,
                    revision,
                    EventPayload::Unit,
                );
            }
        },
        EventPayloadSource::DragInfo {
            interface: property_interface,
        } => {
            let property_interface = path_ident(property_interface);
            quote! {
                move |_, args| {
                    let result = args
                        .as_ref()
                        .ok_or_else(windows_core::Error::empty)
                        .and_then(|args| args.cast::<#property_interface>())
                        .and_then(|args| {
                            let data = args.DataView()?;
                            let kind = if data.Contains("Shell IDList Array")? {
                                DragKind::StorageItems
                            } else if data.Contains("Text")? {
                                DragKind::Text
                            } else {
                                DragKind::Unsupported
                            };
                            let action = sink.drag_action(node, kind);
                            args.SetAcceptedOperation(
                                action
                                    .as_ref()
                                    .map_or(DataPackageOperation::None, |action| {
                                        native_drag_operation(action.operation)
                                    }),
                            )?;
                            let ui = args.DragUIOverride()?;
                            if let Some(caption) =
                                action.as_ref().and_then(|action| action.caption.as_deref())
                            {
                                ui.SetCaption(caption)?;
                                ui.SetIsCaptionVisible(true)?;
                            } else {
                                ui.SetIsCaptionVisible(false)?;
                            }
                            Ok(action.map_or(DragKind::Unsupported, |_| kind))
                        });
                    match result {
                        Ok(kind) => sink.enqueue(
                            node,
                            EventId::#event_id,
                            revision,
                            EventPayload::DragKind(kind),
                        ),
                        Err(error) => {
                            sink.error(
                                node, EventId::#event_id, revision, native_error(error),
                            );
                        }
                    };
                }
            }
        }
        EventPayloadSource::DropData {
            interface: property_interface,
        } => {
            let property_interface = path_ident(property_interface);
            quote! {
                move |_, args| {
                    let result = args
                        .as_ref()
                        .ok_or_else(windows_core::Error::empty)
                        .and_then(|args| args.cast::<#property_interface>())
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
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            if event.conversion == EventPayloadConversion::Selection {
                let dispatch = generate_selection_dispatch(control, event);
                quote! {
                    {
                        let event_source = value
                            .cast::<#property_interface>()
                            .map_err(native_error)?;
                        move |_, _| {
                            let value = event_source.#property();
                            #dispatch
                        }
                    }
                }
            } else {
                quote! {
                    {
                        let event_source = value
                            .cast::<#property_interface>()
                            .map_err(native_error)?;
                        move |_, _| {
                            match event_source.#property() {
                                Ok(value) => #enqueue_payload,
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
        EventPayloadSource::EventArgsProperty {
            interface: property_interface,
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            if event.conversion == EventPayloadConversion::Selection {
                let dispatch = generate_selection_dispatch(control, event);
                quote! {
                    move |_, args| {
                        if let Some(args) = args.as_ref() {
                            let value = args
                                .cast::<#property_interface>()
                                .and_then(|args| args.#property());
                            #dispatch
                        }
                    }
                }
            } else {
                quote! {
                    move |_, args| {
                        #lifecycle_completion
                        if let Some(args) = args.as_ref() {
                            match args
                                .cast::<#property_interface>()
                                .and_then(|args| args.#property())
                            {
                                Ok(value) => #enqueue_payload,
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
            interface: property_interface,
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .cast::<#property_interface>()
                            .and_then(|args| args.#property())
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
        } => {
            let property_interface = path_ident(property_interface);
            quote! {
                {
                    let event_source = value
                        .cast::<#property_interface>()
                        .map_err(native_error)?;
                    move |_, _| {
                        let value = event_source.Document().and_then(|document| {
                            let mut value = windows_core::HSTRING::new();
                            document
                                .GetText(bindings::TextGetOptions::None, &mut value)
                                .map(|_| value)
                        });
                        match value {
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
        EventPayloadSource::PointerEvent => quote! {
            {
                let element = value.cast::<UIElement>().map_err(native_error)?;
                move |_, args| {
                    let mut info = crate::PointerEventInfo::default();
                    if let Some(args) = args.as_ref() {
                        if let Ok(point) = args.GetCurrentPoint(&element) {
                            if let Ok(position) = point.Position() {
                                info.x = f64::from(position.x);
                                info.y = f64::from(position.y);
                            }
                            if let Ok(properties) = point.Properties() {
                                info.is_left_button_pressed =
                                    properties.IsLeftButtonPressed().unwrap_or(false);
                                info.is_right_button_pressed =
                                    properties.IsRightButtonPressed().unwrap_or(false);
                                info.is_middle_button_pressed =
                                    properties.IsMiddleButtonPressed().unwrap_or(false);
                            }
                        }
                        if let Ok(point) = args.GetCurrentPoint(None::<&UIElement>)
                            && let Ok(position) = point.Position()
                        {
                            info.window_x = f64::from(position.x);
                            info.window_y = f64::from(position.y);
                        }
                    }
                    #pointer_capture
                    #pointer_release
                    sink.enqueue(
                        node,
                        EventId::#event_id,
                        revision,
                        EventPayload::PointerEventInfo(info),
                    );
                }
            }
        },
        EventPayloadSource::EventArgsItemTag {
            interface: property_interface,
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .cast::<#property_interface>()
                            .and_then(|args| args.#property())
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
            interface: property_interface,
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            quote! {
                move |_, args| {
                    if let Some(args) = args.as_ref() {
                        match args
                            .cast::<#property_interface>()
                            .and_then(|args| args.#property())
                            .and_then(|node| node.cast::<ITreeViewNode>())
                            .and_then(|node| node.Content())
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
        EventPayloadSource::SenderItemTags {
            interface: property_interface,
            property,
        } => {
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
    };
    match &event.subscription {
        EventSubscription::Metadata => quote! {
            (Handle::#control_name(value), EventId::#event_id) => {
                let source = value.cast::<#interface>().map_err(native_error)?;
                source
                    .#method(#callback)
                    .map(|revoker| NativeSubscription::Event { _revoker: revoker })
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

fn generate_set_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let value_variant = ident(&property.value);
    let interface = path_ident(&property.interface);
    let setter = ident(&format!("Set{}", property.name));
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
                control
                    .cast::<#interface>()
                    .and_then(|control| control.#setter(&transition))
                    .map_err(native_error)
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
                let control = control.cast::<#interface>().map_err(native_error)?;
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
                control
                    .cast::<#interface>()
                    .and_then(|control| control.#setter(&transition))
                    .map_err(native_error)
            }
        };
    }
    if property.value == "Color" {
        let set = if property.interface.ends_with(&format!(".I{}", control.name)) {
            quote! { control.#setter(value).map_err(native_error) }
        } else {
            quote! {
                control
                    .cast::<#interface>()
                    .map_err(native_error)?
                    .#setter(value)
                    .map_err(native_error)
            }
        };
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
                #set
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
            ) => set_rich_edit_text(control, value)
        };
    }
    if property.adapter == Some(PropertyAdapter::PointerCapture) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id, PropertyValue::Bool(_)) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::DropPolicy) {
        return quote! {
            (
                Handle::#control_name(_),
                PropertyId::#property_id,
                PropertyValue::DragDropPolicy(_),
            ) => Err(RuntimeError::UnsupportedKind)
        };
    }
    if property.adapter == Some(PropertyAdapter::ResourceOverrides) {
        return quote! {
            (
                Handle::#control_name(_),
                PropertyId::#property_id,
                PropertyValue::ResourceOverrides(_),
            ) => Err(RuntimeError::UnsupportedKind)
        };
    }
    if property.adapter == Some(PropertyAdapter::KeyAccelerators) {
        return quote! {
            (
                Handle::#control_name(_),
                PropertyId::#property_id,
                PropertyValue::KeyAccelerators(_),
            ) => Err(RuntimeError::UnsupportedKind)
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
                control
                    .cast::<#interface>()
                    .map_err(native_error)?
                    .#setter(&value)
                    .map_err(native_error)
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
                control
                    .cast::<#interface>()
                    .map_err(native_error)?
                    .#setter(&values)
                    .map_err(native_error)
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
                control
                    .cast::<#interface>()
                    .map_err(native_error)?
                    .#setter(&value)
                    .map_err(native_error)
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
        let set = if property.interface.ends_with(&format!(".I{}", control.name)) {
            quote! { control.#setter(&brush).map_err(native_error) }
        } else {
            quote! {
                control
                    .cast::<#interface>()
                    .map_err(native_error)?
                    .#setter(&brush)
                    .map_err(native_error)
            }
        };
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
                #set
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
    let set = if property.interface.ends_with(&format!(".I{}", control.name)) {
        quote! { control.#setter(#value).map_err(native_error) }
    } else {
        quote! {
            control
                .cast::<#interface>()
                .map_err(native_error)?
                .#setter(#value)
                .map_err(native_error)
        }
    };
    quote! {
        (
            Handle::#control_name(control),
            PropertyId::#property_id,
            PropertyValue::#value_variant(value),
        ) => #set
    }
}

fn generate_read_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let value_variant = ident(&property.value);
    let interface = path_ident(&property.interface);
    let getter = ident(&property.name);
    if matches!(
        property.adapter,
        Some(
            PropertyAdapter::ImplicitOpacityTransition
                | PropertyAdapter::ImplicitScale
                | PropertyAdapter::ImplicitScaleTransition
        )
    ) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.value == "Color" {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .cast::<#interface>()
                .and_then(|control| control.#getter())
                .map(|value| PropertyValue::Color(crate::Color {
                    a: value.a,
                    r: value.r,
                    g: value.g,
                    b: value.b,
                }))
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::RichEditText) {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .Document()
                .and_then(|document| {
                    let mut value = windows_core::HSTRING::new();
                    document.GetText(bindings::TextGetOptions::None, &mut value)?;
                    Ok(PropertyValue::Str(value.to_string_lossy()))
                })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::PointerCapture) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::DropPolicy) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ResourceOverrides) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::KeyAccelerators) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::RichTextBlocks) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if !property.enum_variants.is_empty() {
        let value_type = path_ident(&property.value);
        let variants = property.enum_variants.iter().map(|variant| {
            let variant = ident(variant);
            quote! {
                bindings::#value_type::#variant => crate::#value_type::#variant
            }
        });
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => {
                let value = control
                    .cast::<#interface>()
                    .and_then(|control| control.#getter())
                    .map_err(native_error)?;
                Ok(PropertyValue::#value_variant(match value {
                    #(#variants),*,
                    _ => return Err(RuntimeError::UnsupportedKind),
                }))
            }
        };
    }
    if let Some((_, field)) = &property.native_wrapper {
        let field = ident(field);
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .cast::<#interface>()
                .and_then(|control| control.#getter())
                .map(|value| PropertyValue::#value_variant(value.#field))
                .map_err(native_error)
        };
    }
    quote! {
        (Handle::#control_name(control), PropertyId::#property_id) => control
            .cast::<#interface>()
            .and_then(|control| control.#getter())
            .map(PropertyValue::#value_variant)
            .map_err(native_error)
    }
}

fn generate_clear_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let owner = path_ident(&property.static_owner);
    let property_method = ident(&format!("{}Property", property.name));
    if property.adapter == Some(PropertyAdapter::ImplicitOpacityTransition)
        || property.adapter == Some(PropertyAdapter::ImplicitScaleTransition)
    {
        let interface = path_ident(&property.interface);
        let setter = ident(&format!("Set{}", property.name));
        let transition = if property.adapter == Some(PropertyAdapter::ImplicitOpacityTransition) {
            ident("ScalarTransition")
        } else {
            ident("Vector3Transition")
        };
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .cast::<#interface>()
                .and_then(|control| control.#setter(None::<&#transition>))
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::ImplicitScale) {
        let interface = path_ident(&property.interface);
        let setter = ident(&format!("Set{}", property.name));
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .cast::<#interface>()
                .and_then(|control| {
                    control.#setter(windows_numerics::Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    })
                })
                .map_err(native_error)
        };
    }
    if property.adapter == Some(PropertyAdapter::RichEditText) {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => {
                set_rich_edit_text(control, "")
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::PointerCapture) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::DropPolicy) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::ResourceOverrides) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::KeyAccelerators) {
        return quote! {
            (Handle::#control_name(_), PropertyId::#property_id) => {
                Err(RuntimeError::UnsupportedKind)
            }
        };
    }
    if property.adapter == Some(PropertyAdapter::RichTextBlocks) {
        return quote! {
            (Handle::#control_name(control), PropertyId::#property_id) => control
                .Blocks()
                .and_then(|blocks| blocks.cast::<windows_collections::IVector<Block>>())
                .and_then(|blocks| blocks.Clear())
                .map_err(native_error)
        };
    }
    quote! {
        (Handle::#control_name(_), PropertyId::#property_id) => dependency_object
            .ClearValue(&bindings::#owner::#property_method().map_err(native_error)?)
            .map_err(native_error)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::MetadataResolver;
    use crate::schema::{Schema, workspace_path};

    fn assert_compiles(body: TokenStream) {
        let unique = format!(
            "reactor_codegen_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let source = std::env::temp_dir().join(format!("{unique}.rs"));
        let output = std::env::temp_dir().join(format!("{unique}.rmeta"));
        std::fs::write(&source, body.to_string()).unwrap();
        let result = std::process::Command::new("rustc")
            .args(["--edition=2024", "--crate-type=lib", "--emit=metadata"])
            .arg(&source)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap();
        _ = std::fs::remove_file(source);
        _ = std::fs::remove_file(output);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    fn schema() -> ResolvedSchema {
        let source = include_str!("winui.toml");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        Schema::parse(source).unwrap().resolve(&metadata).unwrap()
    }

    #[test]
    fn versioned_interfaces_use_the_metadata_runtime_class_for_static_properties() {
        let schema = schema();
        let navigation = schema
            .controls
            .iter()
            .find(|control| control.name == "NavigationView")
            .unwrap();
        let property = navigation
            .properties
            .iter()
            .find(|property| property.name == "IsBackButtonVisible")
            .unwrap();

        assert_eq!(
            property.static_owner,
            "Microsoft.UI.Xaml.Controls.NavigationView"
        );
    }

    #[test]
    fn event_args_use_the_getter_interface_and_wrapper_conversion() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
capabilities = ["layout"]

[[control.event]]
name = "ValueChanged"
property = "NewValue"

[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.event]]
name = "Tapped"
property = "FontWeight"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let schema = Schema::parse(source).unwrap().resolve(&metadata).unwrap();
        let generated = generate(&schema);

        assert!(generated.contains("cast :: < INumberBoxValueChangedEventArgs >"));
        assert!(generated.contains("EventPayload :: U16 (value . weight)"));
    }

    #[test]
    fn accepted_event_payload_expressions_compile() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
capabilities = ["layout"]

[[control.event]]
name = "ValueChanged"
property = "NewValue"

[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
capabilities = ["layout"]

[[control.event]]
name = "Tapped"
property = "FontWeight"
"#;
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        let schema = Schema::parse(source).unwrap().resolve(&metadata).unwrap();
        let number = generate_payload_value(&schema.controls[0].events[0].conversion);
        let weight = generate_payload_value(&schema.controls[1].events[0].conversion);

        assert_compiles(quote! {
            struct FontWeight {
                weight: u16,
            }

            fn number_payload(value: f64) -> f64 {
                #number
            }

            fn weight_payload(value: FontWeight) -> u16 {
                #weight
            }
        });

        let number_arm = generate_event_arm(&schema.controls[0], &schema.controls[0].events[0]);
        assert_compiles(quote! {
            #[derive(Clone, Copy)]
            struct RuntimeError;

            fn native_error(error: RuntimeError) -> RuntimeError {
                error
            }

            struct EventRevoker;

            enum NativeSubscription {
                Event { _revoker: EventRevoker },
            }

            #[derive(Clone, Copy)]
            enum EventId {
                NumberBoxValueChanged,
                Other,
            }

            enum EventPayload {
                F64(f64),
            }

            #[derive(Clone)]
            struct EventSink;

            impl EventSink {
                fn enqueue(
                    &self,
                    _node: u32,
                    _event: EventId,
                    _revision: u32,
                    _payload: EventPayload,
                ) {
                }

                fn error(
                    &self,
                    _node: u32,
                    _event: EventId,
                    _revision: u32,
                    _error: RuntimeError,
                ) {
                }
            }

            #[derive(Default)]
            struct INumberBox;

            impl INumberBox {
                fn ValueChanged<F>(&self, _callback: F) -> Result<EventRevoker, RuntimeError>
                where
                    F: Fn((), EventArgsRef) + 'static,
                {
                    Ok(EventRevoker)
                }
            }

            struct EventArgsRef(Option<EventArgs>);

            impl EventArgsRef {
                fn as_ref(&self) -> Option<&EventArgs> {
                    self.0.as_ref()
                }
            }

            struct EventArgs;

            impl EventArgs {
                fn cast<T: Default>(&self) -> Result<T, RuntimeError> {
                    Ok(T::default())
                }
            }

            #[derive(Default)]
            struct INumberBoxValueChangedEventArgs;

            impl INumberBoxValueChangedEventArgs {
                fn NewValue(&self) -> Result<f64, RuntimeError> {
                    Ok(1.0)
                }
            }

            struct NumberBox;

            impl NumberBox {
                fn cast<T: Default>(&self) -> Result<T, RuntimeError> {
                    Ok(T::default())
                }
            }

            enum Handle {
                NumberBox(NumberBox),
                Other,
            }

            fn subscribe(
                handle: &Handle,
                node: u32,
                event: EventId,
                revision: u32,
                sink: EventSink,
            ) -> Result<NativeSubscription, RuntimeError> {
                match (handle, event) {
                    #number_arm,
                    _ => Err(RuntimeError),
                }
            }
        });
    }
}
