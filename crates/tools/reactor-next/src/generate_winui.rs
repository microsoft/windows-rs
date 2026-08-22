use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

use crate::schema::{
    EventPayloadConversion, EventPayloadSource, FeedbackContract, ResolvedControl, ResolvedEvent,
    ResolvedProperty, ResolvedSchema, Role, SlotTarget,
};

pub(crate) fn generate_bindings_filter(schema: &ResolvedSchema) -> String {
    let mut entries = BTreeSet::from([
        "Microsoft::UI::Dispatching::DispatcherQueue::GetForCurrentThread".to_string(),
        "Microsoft::UI::Dispatching::DispatcherQueueHandler".to_string(),
        "Microsoft::UI::Dispatching::DispatcherQueuePriority".to_string(),
        "Microsoft::UI::Dispatching::IDispatcherQueue::TryEnqueueWithPriority".to_string(),
        "Microsoft::UI::Xaml::Application::{CreateInstance, Current, Start}".to_string(),
        "Microsoft::UI::Xaml::ApplicationInitializationCallback".to_string(),
        "Microsoft::UI::Xaml::Controls::XamlControlsResources::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::IApplicationOverrides".to_string(),
        "Microsoft::UI::Xaml::IApplication::get_Resources".to_string(),
        "Microsoft::UI::Xaml::IDependencyObject::ClearValue".to_string(),
        "Microsoft::UI::Xaml::IElementFactory".to_string(),
        "Microsoft::UI::Xaml::IFrameworkElement::{put_MinHeight, put_RequestedTheme}"
            .to_string(),
        "Microsoft::UI::Xaml::IUIElement::{Focus, StartBringIntoView}".to_string(),
        "Microsoft::UI::Xaml::Media::CompositionTarget::Rendering".to_string(),
        "Microsoft::UI::Xaml::GridLength".to_string(),
        "Microsoft::UI::Xaml::GridUnitType".to_string(),
        "Microsoft::UI::Xaml::FocusState".to_string(),
        "Microsoft::UI::Xaml::IResourceDictionary::get_MergedDictionaries".to_string(),
        "Microsoft::UI::Xaml::IWindow::{Activate, Close, Closed, put_Content, put_Title}"
            .to_string(),
        "Microsoft::UI::Xaml::IWindow2::{get_AppWindow, put_SystemBackdrop}".to_string(),
        "Microsoft::UI::Xaml::LaunchActivatedEventArgs".to_string(),
        "Microsoft::UI::Xaml::Controls::ContentControl::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Controls::ColumnDefinition::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Controls::ColumnDefinitionCollection".to_string(),
        "Microsoft::UI::Xaml::Controls::Grid::{ColumnProperty, ColumnSpanProperty, GetColumn, \
         GetColumnSpan, GetRow, GetRowSpan, RowProperty, RowSpanProperty, SetColumn, SetColumnSpan, \
         SetRow, SetRowSpan}"
            .to_string(),
        "Microsoft::UI::Xaml::Controls::IColumnDefinition::{get_Width, put_Width}".to_string(),
        "Microsoft::UI::Xaml::Controls::IGrid::{get_ColumnDefinitions, get_RowDefinitions}"
            .to_string(),
        "Microsoft::UI::Xaml::Controls::IItemsRepeater::{GetOrCreateElement, put_ItemsSource, \
         put_ItemTemplate}"
            .to_string(),
        "Microsoft::UI::Xaml::Controls::IRowDefinition::{get_Height, put_Height}".to_string(),
        "Microsoft::UI::Xaml::Controls::ISplitView::{get_CompactPaneLength, get_Content, \
         get_DisplayMode, get_IsPaneOpen, get_OpenPaneLength, get_Pane}"
            .to_string(),
        "Microsoft::UI::Xaml::Controls::ItemsRepeater::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Controls::RowDefinition::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Controls::RowDefinitionCollection".to_string(),
        "Microsoft::UI::Xaml::IElementFactoryGetArgs::{get_Data, get_Parent}".to_string(),
        "Microsoft::UI::Xaml::IElementFactoryRecycleArgs::{get_Element, get_Parent}".to_string(),
        "Microsoft::UI::Xaml::Markup::IXamlMetadataProvider".to_string(),
        "Microsoft::UI::Xaml::Markup::IXamlType::{}".to_string(),
        "Microsoft::UI::Xaml::Markup::XmlnsDefinition".to_string(),
        "Microsoft::UI::Xaml::UIElement".to_string(),
        "Microsoft::UI::Xaml::Window::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::XamlTypeInfo::XamlControlsXamlMetaDataProvider::CreateInstance"
            .to_string(),
        "Windows::UI::Xaml::Interop::TypeName".to_string(),
        "Microsoft::UI::Windowing::IAppWindow::get_TitleBar".to_string(),
        "Microsoft::UI::Windowing::IAppWindow2::ResizeClient".to_string(),
        "Microsoft::UI::Windowing::IAppWindowTitleBar3::put_PreferredTheme".to_string(),
        "Microsoft::UI::Xaml::Media::DesktopAcrylicBackdrop::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Media::IMicaBackdrop::put_Kind".to_string(),
        "Microsoft::UI::Xaml::Media::MicaBackdrop::CreateInstance".to_string(),
        "Windows::Graphics::SizeInt32".to_string(),
        "Windows::Win32::COINIT_APARTMENTTHREADED".to_string(),
        "Windows::Win32::CoInitializeEx".to_string(),
        "Windows::Win32::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2".to_string(),
        "Windows::Win32::E_FAIL".to_string(),
        "Windows::Win32::GetDpiForWindow".to_string(),
        "Windows::Win32::RPC_E_CHANGED_MODE".to_string(),
        "Windows::Win32::SetProcessDpiAwarenessContext".to_string(),
        "Windows::Win32::PostQuitMessage".to_string(),
        "extras::MddBootstrapInitialize2".to_string(),
        "extras::IWindowNative::get_WindowHandle".to_string(),
        "extras::MddBootstrapInitializeOptions::{}".to_string(),
        "extras::MddBootstrapInitializeOptions_OnNoMatch_ShowUI".to_string(),
        "extras::MddBootstrapInitializeOptions_OnPackageIdentity_NOOP".to_string(),
        "extras::WINDOWSAPPSDK_RELEASE_MAJORMINOR".to_string(),
        "extras::WINDOWSAPPSDK_RELEASE_VERSION_TAG_W".to_string(),
        "extras::WINDOWSAPPSDK_RUNTIME_VERSION_UINT64".to_string(),
    ]);

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
            Role::Leaf | Role::Content | Role::Controlled | Role::Slots | Role::Virtual => {}
        }
        if let Some(content) = &control.content {
            entries.insert(format!(
                "{}::put_{}",
                filter_path(&content.interface),
                content.name
            ));
        }

        for property in &control.properties {
            entries.insert(format!(
                "{}::put_{}",
                filter_path(&property.interface),
                property.name
            ));
            if property.observes_feedback {
                entries.insert(format!(
                    "{}::get_{}",
                    filter_path(&property.interface),
                    property.name
                ));
            }
            entries.insert(format!(
                "{}::{}Property",
                static_owner(&property.interface),
                property.name
            ));
            if let Some(native_value) = &property.native_value {
                entries.insert(filter_path(native_value));
            }
        }
        for event in &control.events {
            entries.insert(format!(
                "{}::{{add_{}, remove_{}}}",
                filter_path(&event.interface),
                event.name,
                event.name
            ));
            if let Some(property) = &event.property {
                let interface = match &event.source {
                    EventPayloadSource::SenderProperty { interface }
                    | EventPayloadSource::EventArgsProperty { interface } => interface,
                    EventPayloadSource::Unit => continue,
                };
                entries.insert(format!("{}::get_{}", filter_path(interface), property));
            }
        }
        for slot in &control.slots {
            entries.insert(format!(
                "{}::put_{}",
                filter_path(&slot.interface),
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
            .map(move |slot| generate_set_slot(control, slot))
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
                FeedbackContract::SynchronousExact => Some(quote! {
                    (PropertyId::#property_id, None) => Some((
                        EventId::#event_id,
                        FeedbackExpectation::Exact(EventPayload::#value_variant(
                            Default::default(),
                        )),
                    ))
                }),
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

    let tokens = quote! {
        use super::*;

        pub enum Handle {
            #(#variants),*
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

        pub fn subscribe_event(
            handle: &Handle,
            node: NodeId,
            event: EventId,
            revision: u32,
            sink: EventSink,
        ) -> Result<windows_core::EventRevoker, RuntimeError> {
            match (handle, event) {
                #(#events,)*
                _ => Err(RuntimeError::UnsupportedKind),
            }
        }
    };

    format!("// Generated by `tool_reactor_next`. Do not edit.\n\n{tokens}\n")
}

fn generate_set_content(
    control: &ResolvedControl,
    content: &crate::schema::ResolvedContent,
) -> TokenStream {
    let control_name = ident(&control.name);
    let interface = path_ident(&content.interface);
    let setter = ident(&format!("Set{}", content.name));
    let value = match content.target {
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

fn generate_set_slot(control: &ResolvedControl, slot: &crate::schema::ResolvedSlot) -> TokenStream {
    let control_name = ident(&control.name);
    let slot_id = ident(&format!("{}{}", control.name, slot.name));
    let interface = path_ident(&slot.interface);
    let setter = ident(&format!("Set{}", slot.name));
    let value = match slot.target {
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
    }
}

fn generate_event_arm(control: &ResolvedControl, event: &ResolvedEvent) -> TokenStream {
    let control_name = ident(&control.name);
    let event_id = ident(&format!("{}{}", control.name, event.name));
    let interface = path_ident(&event.interface);
    let method = ident(&event.name);
    let payload = ident(&event.payload);
    let payload_value = generate_payload_value(&event.conversion);
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
        EventPayloadSource::SenderProperty {
            interface: property_interface,
        } => {
            let property = ident(event.property.as_deref().unwrap());
            let property_interface = path_ident(property_interface);
            quote! {
                {
                    let event_source = value
                        .cast::<#property_interface>()
                        .map_err(native_error)?;
                    move |_, _| {
                        match event_source.#property() {
                            Ok(value) => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::#payload(#payload_value),
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
        EventPayloadSource::EventArgsProperty {
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
                        {
                            Ok(value) => sink.enqueue(
                                node,
                                EventId::#event_id,
                                revision,
                                EventPayload::#payload(#payload_value),
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
    quote! {
        (Handle::#control_name(value), EventId::#event_id) => {
            let source = value.cast::<#interface>().map_err(native_error)?;
            source.#method(#callback).map_err(native_error)
        }
    }
}

fn generate_set_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let value_variant = ident(&property.value);
    let interface = path_ident(&property.interface);
    let setter = ident(&format!("Set{}", property.name));
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

fn generate_clear_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let owner = ident(&static_owner_name(&property.interface));
    let property_method = ident(&format!("{}Property", property.name));
    quote! {
        (Handle::#control_name(_), PropertyId::#property_id) => dependency_object
            .ClearValue(&bindings::#owner::#property_method().map_err(native_error)?)
            .map_err(native_error)
    }
}

fn filter_path(value: &str) -> String {
    value.replace('.', "::")
}

fn static_owner(interface: &str) -> String {
    let (namespace, name) = interface.rsplit_once('.').unwrap();
    format!(
        "{}::{}",
        filter_path(namespace),
        name.trim_start_matches('I')
    )
}

fn static_owner_name(interface: &str) -> String {
    interface
        .rsplit_once('.')
        .unwrap()
        .1
        .trim_start_matches('I')
        .to_string()
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
    use crate::schema::{Schema, workspace_path};
    use tool_reactor::metadata::MetadataResolver;

    fn assert_compiles(body: TokenStream) {
        let unique = format!(
            "reactor_next_codegen_{}_{}",
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
    fn filter_covers_schema_controls_and_members() {
        let filter = generate_bindings_filter(&schema());

        assert!(filter.contains("TextBox::CreateInstance"));
        assert!(filter.contains("Border::CreateInstance"));
        assert!(filter.contains("MicaBackdrop::CreateInstance"));
        assert!(filter.contains("IWindow2::{get_AppWindow, put_SystemBackdrop}"));
        assert!(filter.contains("IAppWindowTitleBar3::put_PreferredTheme"));
        assert!(filter.contains("IAppWindow2::ResizeClient"));
        assert!(filter.contains("IBorder::put_Child"));
        assert!(filter.contains("IBorder::put_Padding"));
        assert!(filter.contains("ITextBox::put_PlaceholderText"));
        assert!(filter.contains("ITextBox::{add_TextChanged, remove_TextChanged}"));
        assert!(filter.contains("Control::IsEnabledProperty"));
        assert!(filter.contains("IElementFactoryGetArgs"));
        assert!(
            filter.contains(
                "IItemsRepeater::{GetOrCreateElement, put_ItemsSource, put_ItemTemplate}"
            )
        );
        assert!(filter.contains("Grid::{ColumnProperty"));
        assert!(filter.contains("IGrid::{get_ColumnDefinitions, get_RowDefinitions}"));
        assert!(filter.contains("ISplitView::{get_CompactPaneLength, get_Content"));
    }

    #[test]
    fn runtime_dispatch_is_generated_from_schema() {
        let generated = generate(&schema());

        assert!(generated.contains("Handle :: TextBox"));
        assert!(generated.contains("Handle :: Border"));
        assert!(generated.contains("control . SetChild"));
        assert!(generated.contains("bindings :: Thickness"));
        assert!(generated.contains("bindings :: CornerRadius"));
        assert!(generated.contains("PropertyId :: TextBoxPlaceholderText"));
        assert!(generated.contains("EventId :: ButtonClick"));
        assert!(generated.contains("EventId :: TextBoxTextChanged"));
        assert!(generated.contains("expected_feedback"));
        assert!(generated.contains("event_source . Text"));
        assert!(generated.contains("child_collection"));
    }

    #[test]
    fn event_args_use_the_getter_interface_and_wrapper_conversion() {
        let source = r#"
[[control]]
type = "Microsoft.UI.Xaml.Controls.NumberBox"
role = "leaf"
capabilities = ["layout"]

[[control.event]]
name = "ValueChanged"
property = "NewValue"

[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
role = "leaf"
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
role = "leaf"
capabilities = ["layout"]

[[control.event]]
name = "ValueChanged"
property = "NewValue"

[[control]]
type = "Microsoft.UI.Xaml.Controls.TextBlock"
role = "leaf"
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
            ) -> Result<EventRevoker, RuntimeError> {
                match (handle, event) {
                    #number_arm,
                    _ => Err(RuntimeError),
                }
            }
        });
    }
}
