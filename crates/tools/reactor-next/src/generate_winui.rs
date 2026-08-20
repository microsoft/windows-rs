use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

use crate::schema::{ResolvedControl, ResolvedProperty, ResolvedSchema, Role};

pub(crate) fn generate_bindings_filter(schema: &ResolvedSchema) -> String {
    let mut entries = BTreeSet::from([
        "Microsoft::UI::Dispatching::DispatcherQueue::GetForCurrentThread".to_string(),
        "Microsoft::UI::Dispatching::DispatcherQueueHandler".to_string(),
        "Microsoft::UI::Dispatching::DispatcherQueuePriority".to_string(),
        "Microsoft::UI::Dispatching::IDispatcherQueue::TryEnqueueWithPriority".to_string(),
        "Microsoft::UI::Xaml::Application::{CreateInstance, Start}".to_string(),
        "Microsoft::UI::Xaml::ApplicationInitializationCallback".to_string(),
        "Microsoft::UI::Xaml::Controls::XamlControlsResources::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::IApplicationOverrides".to_string(),
        "Microsoft::UI::Xaml::IApplication::get_Resources".to_string(),
        "Microsoft::UI::Xaml::IDependencyObject::ClearValue".to_string(),
        "Microsoft::UI::Xaml::IElementFactory".to_string(),
        "Microsoft::UI::Xaml::IFrameworkElement::put_MinHeight".to_string(),
        "Microsoft::UI::Xaml::IResourceDictionary::get_MergedDictionaries".to_string(),
        "Microsoft::UI::Xaml::IWindow::{Activate, Close, put_Content}".to_string(),
        "Microsoft::UI::Xaml::LaunchActivatedEventArgs".to_string(),
        "Microsoft::UI::Xaml::Controls::ContentControl::CreateInstance".to_string(),
        "Microsoft::UI::Xaml::Controls::IItemsRepeater::{put_ItemsSource, put_ItemTemplate}"
            .to_string(),
        "Microsoft::UI::Xaml::Controls::ItemsRepeater::CreateInstance".to_string(),
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
        "Windows::Win32::COINIT_APARTMENTTHREADED".to_string(),
        "Windows::Win32::CoInitializeEx".to_string(),
        "Windows::Win32::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2".to_string(),
        "Windows::Win32::E_FAIL".to_string(),
        "Windows::Win32::RPC_E_CHANGED_MODE".to_string(),
        "Windows::Win32::SetProcessDpiAwarenessContext".to_string(),
        "Windows::Win32::PostQuitMessage".to_string(),
        "extras::MddBootstrapInitialize2".to_string(),
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
            Role::Content => {
                entries.insert(
                    "Microsoft::UI::Xaml::Controls::IContentControl::put_Content".to_string(),
                );
            }
            Role::Children => {
                entries.insert("Microsoft::UI::Xaml::Controls::IPanel::Children".to_string());
                entries.insert("Microsoft::UI::Xaml::Controls::UIElementCollection".to_string());
            }
            Role::Leaf | Role::Controlled | Role::Virtual => {}
        }

        for property in &control.properties {
            entries.insert(format!(
                "{}::put_{}",
                filter_path(&property.interface),
                property.name
            ));
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
            if let Some(property) = &event.property
                && let Some(property) = control
                    .properties
                    .iter()
                    .find(|candidate| candidate.name == *property)
            {
                entries.insert(format!(
                    "{}::get_{}",
                    filter_path(&property.interface),
                    property.name
                ));
            }
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
    let content_controls = schema
        .controls
        .iter()
        .filter(|control| matches!(control.role, Role::Content))
        .map(|control| {
            let name = ident(&control.name);
            quote! {
                Self::#name(value) => {
                    Some(value.cast::<IContentControl>().map_err(native_error)?)
                }
            }
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
        control.events.iter().map(move |event| {
            let control_name = ident(&control.name);
            let event_id = ident(&format!("{}{}", control.name, event.name));
            let interface = path_ident(&event.interface);
            let method = ident(&event.name);
            let callback = if event.payload == "Unit" {
                quote! {
                    move |_, _| {
                        sink.enqueue(
                            node,
                            EventId::#event_id,
                            revision,
                            EventPayload::Unit,
                        );
                    }
                }
            } else {
                let property = ident(event.property.as_deref().unwrap());
                quote! {
                    {
                        let event_source = source.clone();
                        move |_, _| {
                            match event_source.#property() {
                                Ok(value) => sink.enqueue(
                                    node,
                                    EventId::#event_id,
                                    revision,
                                    EventPayload::Str(value),
                                ),
                                Err(error) => sink.error(native_error(error)),
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
        })
    });
    let feedback_values = schema.controls.iter().flat_map(|control| {
        control.properties.iter().filter_map(move |property| {
            let feedback = property.feedback.as_ref()?;
            let property_id = ident(&format!("{}{}", control.name, property.name));
            let event_id = ident(&format!("{}{}", control.name, feedback));
            let value_variant = ident(&property.value);
            Some(quote! {
                (
                    PropertyId::#property_id,
                    Some(PropertyValue::#value_variant(value)),
                ) => Some((EventId::#event_id, EventPayload::#value_variant(value.clone())))
            })
        })
    });
    let feedback_defaults = schema.controls.iter().flat_map(|control| {
        control.properties.iter().filter_map(move |property| {
            let feedback = property.feedback.as_ref()?;
            let property_id = ident(&format!("{}{}", control.name, property.name));
            let event_id = ident(&format!("{}{}", control.name, feedback));
            let value_variant = ident(&property.value);
            Some(quote! {
                (PropertyId::#property_id, None) => {
                    Some((EventId::#event_id, EventPayload::#value_variant(Default::default())))
                }
            })
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

            pub fn content_control(&self) -> Result<Option<IContentControl>, RuntimeError> {
                Ok(match self {
                    #(#content_controls,)*
                    _ => None,
                })
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

        pub fn expected_feedback(
            property: PropertyId,
            value: Option<&PropertyValue>,
        ) -> Option<(EventId, EventPayload)> {
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

fn generate_set_property(control: &ResolvedControl, property: &ResolvedProperty) -> TokenStream {
    let control_name = ident(&control.name);
    let property_id = ident(&format!("{}{}", control.name, property.name));
    let value_variant = ident(&property.value);
    let interface = path_ident(&property.interface);
    let setter = ident(&format!("Set{}", property.name));
    let value = if property.value == "Str" {
        quote! { value }
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

    fn schema() -> ResolvedSchema {
        let source = include_str!("winui.toml");
        let metadata = MetadataResolver::load(&workspace_path("crates/tools/reactor/winmd"));
        Schema::parse(source).unwrap().resolve(&metadata).unwrap()
    }

    #[test]
    fn filter_covers_schema_controls_and_members() {
        let filter = generate_bindings_filter(&schema());

        assert!(filter.contains("TextBox::CreateInstance"));
        assert!(filter.contains("ITextBox::put_PlaceholderText"));
        assert!(filter.contains("ITextBox::{add_TextChanged, remove_TextChanged}"));
        assert!(filter.contains("Control::IsEnabledProperty"));
        assert!(filter.contains("IElementFactoryGetArgs"));
        assert!(filter.contains("IItemsRepeater::{put_ItemsSource, put_ItemTemplate}"));
    }

    #[test]
    fn runtime_dispatch_is_generated_from_schema() {
        let generated = generate(&schema());

        assert!(generated.contains("Handle :: TextBox"));
        assert!(generated.contains("PropertyId :: TextBoxPlaceholderText"));
        assert!(generated.contains("EventId :: ButtonClick"));
        assert!(generated.contains("EventId :: TextBoxTextChanged"));
        assert!(generated.contains("expected_feedback"));
        assert!(generated.contains("event_source . Text"));
        assert!(generated.contains("child_collection"));
    }
}
