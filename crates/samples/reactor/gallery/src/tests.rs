use crate::pages::navigation::{NavigationViewMessage, NavigationViewPage};
use crate::registry::ALL_CONTROLS;
use crate::shell::{Gallery, Message as GalleryMessage};
use windows_reactor::*;

fn mount<C: Component<Input = ()>>() -> ComponentHost<RecordingAdapter> {
    ComponentHost::mount(RecordingAdapter::default(), [component::<C>("root", ())]).unwrap()
}

fn property_node(
    host: &ComponentHost<RecordingAdapter>,
    property: PropertyId,
    value: &PropertyValue,
) -> Option<ObjectId> {
    host.runtime().graph().objects().find(|object| {
        host.runtime()
            .graph()
            .properties(*object)
            .is_some_and(|properties| {
                properties
                    .iter()
                    .any(|candidate| candidate.id == property && candidate.value == *value)
            })
    })
}

fn active_property_node(
    host: &ComponentHost<RecordingAdapter>,
    property: PropertyId,
    value: &PropertyValue,
) -> ObjectId {
    property_node(host, property, value)
        .unwrap_or_else(|| panic!("active {property:?} with value {value:?} not found"))
}

fn text_node(host: &ComponentHost<RecordingAdapter>, text: &str) -> ObjectId {
    active_property_node(host, PropertyId::Text, &PropertyValue::String(text.into()))
}

fn event(host: &ComponentHost<RecordingAdapter>, object: ObjectId, event: EventId) -> EventValue {
    host.runtime()
        .graph()
        .events(object)
        .unwrap()
        .iter()
        .find(|candidate| candidate.id == event)
        .unwrap()
        .value
        .clone()
}

fn send<C: Component>(
    host: &mut ComponentHost<RecordingAdapter>,
    message: C::Message,
) -> ComponentDrain
where
    C::Message: Send,
{
    assert!(host.sender::<C>(&Key::from("root")).unwrap().send(message));
    host.drain(64).unwrap()
}

#[test]
fn gallery_mounts_and_replaces_every_registered_page() {
    let mut host = mount::<Gallery>();
    assert!(
        property_node(
            &host,
            PropertyId::Subtitle,
            &PropertyValue::String("Home".into())
        )
        .is_some()
    );
    text_node(&host, "Browse by category");

    send::<Gallery>(&mut host, GalleryMessage::Navigate("button".to_string()));
    text_node(&host, "Basic Button");
    send::<Gallery>(&mut host, GalleryMessage::Navigate("slider".to_string()));
    assert!(
        property_node(
            &host,
            PropertyId::Subtitle,
            &PropertyValue::String("Slider".into())
        )
        .is_some()
    );
    assert!(
        property_node(
            &host,
            PropertyId::Text,
            &PropertyValue::String("Basic Button".into())
        )
        .is_none()
    );

    send::<Gallery>(&mut host, GalleryMessage::Back);
    text_node(&host, "Basic Button");

    for control in ALL_CONTROLS {
        send::<Gallery>(&mut host, GalleryMessage::Navigate(control.tag.to_string()));
        active_property_node(
            &host,
            PropertyId::Subtitle,
            &PropertyValue::String(control.title.into()),
        );
    }
}

#[test]
fn empty_navigation_selection_does_not_replace_a_leaf_with_settings() {
    let mut host = mount::<Gallery>();
    send::<Gallery>(&mut host, GalleryMessage::Navigate("flip-view".to_string()));
    send::<Gallery>(&mut host, GalleryMessage::SelectedTagChanged(None));
    active_property_node(
        &host,
        PropertyId::Subtitle,
        &PropertyValue::String("FlipView".into()),
    );

    send::<Gallery>(&mut host, GalleryMessage::Navigate("settings".to_string()));
    active_property_node(
        &host,
        PropertyId::Subtitle,
        &PropertyValue::String("Settings".into()),
    );
}

#[test]
fn navigation_view_page_updates_controlled_selection_without_replacing_the_page() {
    let mut host = mount::<NavigationViewPage>();
    let root = host.reference(&Key::from("root")).unwrap().get().unwrap();
    send::<NavigationViewPage>(
        &mut host,
        NavigationViewMessage::Left(Some("browse".to_string())),
    );
    text_node(&host, "Browse page content");
    assert_eq!(
        host.reference(&Key::from("root")).unwrap().get(),
        Some(root)
    );
}

#[test]
fn controlled_slider_updates_and_page_retirement_recreates_state() {
    let mut host = mount::<Gallery>();
    send::<Gallery>(&mut host, GalleryMessage::Navigate("slider".to_string()));
    let slider = active_property_node(&host, PropertyId::Value, &PropertyValue::F64(35.0));
    let callback = event(&host, slider, EventId::ValueChanged);
    host.test_adapter_mut().queue_event(EventDispatch::new(
        slider,
        EventId::ValueChanged,
        callback.clone(),
        EventPayload::F64(72.0),
    ));
    assert_eq!(host.drain(64).unwrap().dispatched, 1);
    text_node(&host, "Volume: 72%");

    send::<Gallery>(&mut host, GalleryMessage::Navigate("button".to_string()));
    host.test_adapter_mut().queue_event(EventDispatch::new(
        slider,
        EventId::ValueChanged,
        callback,
        EventPayload::F64(99.0),
    ));
    assert_eq!(host.drain(64).unwrap().dispatched, 0);

    send::<Gallery>(&mut host, GalleryMessage::Navigate("slider".to_string()));
    active_property_node(&host, PropertyId::Value, &PropertyValue::F64(35.0));
    text_node(&host, "Volume: 35%");
}

#[test]
fn shell_objects_survive_page_replacement() {
    let mut host = mount::<Gallery>();
    let title_bar = host
        .runtime()
        .graph()
        .objects()
        .find(|object| host.runtime().graph().kind(*object) == Some(ObjectType::TitleBar))
        .unwrap();
    let shell_root = host.reference(&Key::from("root")).unwrap().get().unwrap();

    send::<Gallery>(&mut host, GalleryMessage::Navigate("materials".to_string()));
    send::<Gallery>(
        &mut host,
        GalleryMessage::BackdropChanged(WindowBackdrop::Acrylic),
    );
    send::<Gallery>(&mut host, GalleryMessage::Navigate("button".to_string()));
    send::<Gallery>(&mut host, GalleryMessage::CycleTheme);

    assert_eq!(
        host.reference(&Key::from("root")).unwrap().get(),
        Some(shell_root)
    );
    assert_eq!(
        host.runtime()
            .graph()
            .objects()
            .find(|object| host.runtime().graph().kind(*object) == Some(ObjectType::TitleBar)),
        Some(title_bar)
    );
    active_property_node(
        &host,
        PropertyId::Subtitle,
        &PropertyValue::String("Button".into()),
    );
}
