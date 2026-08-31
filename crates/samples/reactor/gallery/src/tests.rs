use crate::registry::ALL_CONTROLS;
use crate::shell::Gallery;
use windows_reactor::test::*;
use windows_reactor::*;

fn find_active_property_node(
    pump: &Pump<RecordingRuntime>,
    property: PropertyId,
    value: &PropertyValue,
) -> Option<NodeId> {
    pump.runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::SetProperty {
                node,
                property: candidate,
                ..
            } if *candidate == property
                && pump
                    .runtime()
                    .node(*node)
                    .and_then(|node| node.property(property))
                    == Some(value) =>
            {
                Some(*node)
            }
            _ => None,
        })
}

fn active_property_node(
    pump: &Pump<RecordingRuntime>,
    property: PropertyId,
    value: &PropertyValue,
) -> NodeId {
    find_active_property_node(pump, property, value)
        .unwrap_or_else(|| panic!("active {property:?} with value {value:?} not found"))
}

fn active_event_node(pump: &Pump<RecordingRuntime>, event: EventId) -> NodeId {
    pump.runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::SubscribeEvent {
                node,
                event: candidate,
                ..
            } if *candidate == event && pump.event_revision(*node, event).is_some() => Some(*node),
            _ => None,
        })
        .unwrap_or_else(|| panic!("active {event:?} subscription not found"))
}

fn active_button(pump: &Pump<RecordingRuntime>, label: &str) -> NodeId {
    let label = active_property_node(
        pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str(label.into()),
    );
    pump.runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::InsertChild { parent, child, .. }
                if *child == label
                    && pump.event_revision(*parent, EventId::ButtonClick).is_some() =>
            {
                Some(*parent)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("active button labeled {label:?} not found"))
}

fn queue_event(
    pump: &mut Pump<RecordingRuntime>,
    node: NodeId,
    event: EventId,
    payload: EventPayload,
) {
    let revision = pump.event_revision(node, event).unwrap();
    pump.queue_event(QueuedEvent::new(node, event, revision, payload));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(pump.dispatch_components(64).unwrap() >= 1);
}

fn navigate(pump: &mut Pump<RecordingRuntime>, navigation: NodeId, tag: &str) {
    queue_event(
        pump,
        navigation,
        EventId::NavigationViewSelectionChanged,
        EventPayload::SelectionChange(SelectionChange {
            item: None,
            tag: Some(tag.into()),
        }),
    );
}

fn click(pump: &mut Pump<RecordingRuntime>, label: &str) {
    let button = active_button(pump, label);
    queue_event(pump, button, EventId::ButtonClick, EventPayload::Unit);
}

#[test]
fn gallery_mounts_and_replaces_every_registered_page() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Gallery>(())).unwrap();
    let window = pump.window().unwrap();
    let navigation = active_event_node(&pump, EventId::NavigationViewSelectionChanged);

    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - Home")
    );
    assert_eq!(
        pump.runtime().window_visuals(window),
        Some(
            WindowVisuals::new()
                .theme(WindowTheme::System)
                .backdrop(WindowBackdrop::Mica)
                .client_size(1400.0, 900.0)
        )
    );
    assert_eq!(
        pump.runtime()
            .window_title_bar(window)
            .map(|(_, height)| height),
        Some(WindowTitleBarHeight::Tall)
    );
    let initial = pump.runtime().commands().last().unwrap();
    let title_bar = initial
        .iter()
        .position(|command| matches!(command, Command::SetWindowTitleBar { .. }))
        .unwrap();
    let title = initial
        .iter()
        .position(|command| matches!(command, Command::SetWindowTitle { .. }))
        .unwrap();
    assert!(title_bar < title);

    let home_heading = active_property_node(
        &pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str("Browse by category".into()),
    );
    navigate(&mut pump, navigation, "button");
    assert!(pump.runtime().node(home_heading).is_none());
    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - Button")
    );
    let button_heading = active_property_node(
        &pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str("Basic Button".into()),
    );

    navigate(&mut pump, navigation, "slider");
    assert!(pump.runtime().node(button_heading).is_none());
    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - Slider")
    );

    let title_bar = active_event_node(&pump, EventId::TitleBarBackRequested);
    queue_event(
        &mut pump,
        title_bar,
        EventId::TitleBarBackRequested,
        EventPayload::Unit,
    );
    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - Button")
    );

    for control in ALL_CONTROLS {
        navigate(&mut pump, navigation, control.tag);
        assert_eq!(
            pump.runtime().window_title(window),
            Some(format!("Reactor gallery - {}", control.title).as_str()),
            "failed to mount route {}",
            control.tag,
        );
        active_property_node(
            &pump,
            PropertyId::TitleBarSubtitle,
            &PropertyValue::Str(control.title.into()),
        );
    }
}

#[test]
fn empty_navigation_selection_does_not_replace_a_leaf_with_settings() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Gallery>(())).unwrap();
    let window = pump.window().unwrap();
    let navigation = active_event_node(&pump, EventId::NavigationViewSelectionChanged);

    navigate(&mut pump, navigation, "flip-view");
    let selected_leaf = active_property_node(
        &pump,
        PropertyId::NavigationViewItemTag,
        &PropertyValue::Str("flip-view".into()),
    );
    let parent_category = active_property_node(
        &pump,
        PropertyId::NavigationViewItemTag,
        &PropertyValue::Str("collections".into()),
    );
    assert_eq!(
        pump.runtime()
            .node(selected_leaf)
            .and_then(|node| node.property(PropertyId::NavigationViewItemIsSelected)),
        Some(&PropertyValue::Bool(true))
    );
    assert!(
        pump.runtime()
            .node(parent_category)
            .unwrap()
            .slot_children(SlotId::NavigationViewItemMenuItems)
            .contains(&selected_leaf)
    );
    assert!(
        !pump
            .runtime()
            .node(navigation)
            .unwrap()
            .slot_children(SlotId::NavigationViewMenuItems)
            .contains(&selected_leaf)
    );
    queue_event(
        &mut pump,
        navigation,
        EventId::NavigationViewSelectionChanged,
        EventPayload::SelectionChange(SelectionChange {
            item: None,
            tag: None,
        }),
    );
    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - FlipView")
    );

    navigate(&mut pump, navigation, "settings");
    assert_eq!(
        pump.runtime().window_title(window),
        Some("Reactor gallery - Settings")
    );
}

#[test]
fn navigation_view_page_updates_controlled_selection_without_replacing_the_page() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Gallery>(())).unwrap();
    let shell_navigation = active_event_node(&pump, EventId::NavigationViewSelectionChanged);

    navigate(&mut pump, shell_navigation, "navigation-view");
    let navigation = active_property_node(
        &pump,
        PropertyId::NavigationViewPaneTitle,
        &PropertyValue::Str("Navigation demo".into()),
    );
    let browse = active_property_node(
        &pump,
        PropertyId::NavigationViewItemTag,
        &PropertyValue::Str("browse".into()),
    );
    let commands_before = pump
        .runtime()
        .commands()
        .iter()
        .map(Vec::len)
        .sum::<usize>();

    queue_event(
        &mut pump,
        navigation,
        EventId::NavigationViewSelectionChanged,
        EventPayload::SelectionChange(SelectionChange {
            item: Some(browse),
            tag: Some("browse".into()),
        }),
    );

    active_property_node(
        &pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str("Browse page content".into()),
    );
    assert!(!pump.native_work_pending());
    let commands_after = pump
        .runtime()
        .commands()
        .iter()
        .map(Vec::len)
        .sum::<usize>();
    assert_eq!(commands_after - commands_before, 1);
}

#[test]
fn controlled_slider_updates_and_page_retirement_recreates_state() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Gallery>(())).unwrap();
    let navigation = active_event_node(&pump, EventId::NavigationViewSelectionChanged);

    navigate(&mut pump, navigation, "slider");
    let slider = active_property_node(&pump, PropertyId::SliderValue, &PropertyValue::F64(35.0));
    let revision = pump
        .event_revision(slider, EventId::SliderValueChanged)
        .unwrap();
    queue_event(
        &mut pump,
        slider,
        EventId::SliderValueChanged,
        EventPayload::F64(72.0),
    );
    active_property_node(
        &pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str("Volume: 72%".into()),
    );

    navigate(&mut pump, navigation, "button");
    assert!(pump.runtime().node(slider).is_none());
    pump.queue_event(QueuedEvent::new(
        slider,
        EventId::SliderValueChanged,
        revision,
        EventPayload::F64(99.0),
    ));
    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(pump.dispatch_components(1), Ok(0));

    navigate(&mut pump, navigation, "slider");
    active_property_node(&pump, PropertyId::SliderValue, &PropertyValue::F64(35.0));
    active_property_node(
        &pump,
        PropertyId::TextBlockText,
        &PropertyValue::Str("Volume: 35%".into()),
    );
}

#[test]
fn shell_controlled_visuals_survive_page_replacement() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Gallery>(())).unwrap();
    let window = pump.window().unwrap();
    let navigation = active_event_node(&pump, EventId::NavigationViewSelectionChanged);

    navigate(&mut pump, navigation, "materials");
    click(&mut pump, "Acrylic");
    assert_eq!(
        pump.runtime().window_visuals(window),
        Some(
            WindowVisuals::new()
                .theme(WindowTheme::System)
                .backdrop(WindowBackdrop::Acrylic)
                .client_size(1400.0, 900.0)
        )
    );

    navigate(&mut pump, navigation, "button");
    assert_eq!(
        pump.runtime().window_visuals(window),
        Some(
            WindowVisuals::new()
                .theme(WindowTheme::System)
                .backdrop(WindowBackdrop::Acrylic)
                .client_size(1400.0, 900.0)
        )
    );
    click(&mut pump, "Theme: System");
    assert_eq!(
        pump.runtime().window_visuals(window),
        Some(
            WindowVisuals::new()
                .theme(WindowTheme::Light)
                .backdrop(WindowBackdrop::Acrylic)
                .client_size(1400.0, 900.0)
        )
    );
}
