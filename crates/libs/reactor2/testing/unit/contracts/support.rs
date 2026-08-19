use super::*;
use crate::element::props::{MenuBarItemSpec, MenuItemSpec};
pub(super) fn button_node(reactor: &Reactor<RecordingRuntime>) -> NodeId {
    native_node(reactor, NativeKind::Button)
}

pub(super) fn native_node(reactor: &Reactor<RecordingRuntime>, native_kind: NativeKind) -> NodeId {
    reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create { id, kind } if *kind == native_kind => Some(*id),
            _ => None,
        })
        .unwrap()
}

pub(super) fn created_nodes(
    reactor: &Reactor<RecordingRuntime>,
    native_kind: NativeKind,
) -> Vec<NodeId> {
    reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::Create { id, kind } if *kind == native_kind => Some(*id),
            _ => None,
        })
        .collect()
}

pub(super) fn virtual_host(reactor: &Reactor<RecordingRuntime>) -> NodeId {
    reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| item_count_update(command).map(|(id, _)| id))
        .unwrap()
}

pub(super) fn text_update(command: &Command) -> Option<(NodeId, &str)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TextBlockText(text)),
        } => Some((*id, text)),
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TextBox(update)),
        } => match update.as_ref() {
            TextBoxUpdate::Text(text) => Some((*id, text)),
            _ => None,
        },
        _ => None,
    }
}

fn control_update(command: &Command) -> Option<(NodeId, &ControlUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(update),
        } => Some((*id, update)),
        _ => None,
    }
}

pub(super) fn pump_until_text(reactor: &mut Reactor<RecordingRuntime>, expected: &str) {
    for _ in 0..1000 {
        reactor.pump();
        if reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == expected))
        {
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("timed out waiting for text {expected:?}");
}

pub(super) fn item_count_update(command: &Command) -> Option<(NodeId, usize)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemCount(count))),
        } => Some((*id, *count)),
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemKeys(keys))),
        } => Some((*id, keys.len())),
        _ => None,
    }
}

pub(super) fn items_update(command: &Command) -> Option<(NodeId, &[u64])> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemKeys(keys))),
        } => Some((*id, keys)),
        _ => None,
    }
}

pub(super) fn selection_mode_update(command: &Command) -> Option<(NodeId, SelectionMode)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::SelectionMode(value))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn selection_update(command: &Command) -> Option<(NodeId, &CollectionSelection)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::Selection(value))),
        } => Some((*id, value)),
        _ => None,
    }
}

pub(super) fn repeat_button_update(command: &Command) -> Option<(NodeId, RepeatButtonUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::RepeatButton(update)),
        } => Some((*id, *update)),
        _ => None,
    }
}

pub(super) fn hyperlink_button_update(command: &Command) -> Option<(NodeId, Option<String>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::HyperlinkButtonNavigateUri(value)),
        } => Some((*id, value.clone())),
        _ => None,
    }
}

pub(super) fn enabled_update(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::Enabled(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn visibility_update(command: &Command) -> Option<(NodeId, Option<Visibility>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::Visibility(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn opacity_update(command: &Command) -> Option<(NodeId, Option<f32>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::Opacity(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn font_size_update(command: &Command) -> Option<(NodeId, Option<f32>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::FontSize(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn character_spacing_update(command: &Command) -> Option<(NodeId, Option<i32>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::CharacterSpacing(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn font_weight_update(command: &Command) -> Option<(NodeId, Option<FontWeight>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::FontWeight(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn font_style_update(command: &Command) -> Option<(NodeId, Option<FontStyle>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::FontStyle(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn font_stretch_update(command: &Command) -> Option<(NodeId, Option<FontStretch>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::FontStretch(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn font_family_update(command: &Command) -> Option<(NodeId, Option<&str>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::TextStyle(TextStyleUpdate::FontFamily(value)),
        } => Some((*id, value.as_deref())),
        _ => None,
    }
}

pub(super) fn foreground_update(command: &Command) -> Option<(NodeId, Option<Brush>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::TextStyle(TextStyleUpdate::Foreground(value)),
        } => Some((*id, value.clone())),
        _ => None,
    }
}

pub(super) fn resource_update(command: &Command) -> Option<(NodeId, ElementResources)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Resources(resources),
        } => Some((*id, (**resources).clone())),
        _ => None,
    }
}

pub(super) fn visual_update(command: &Command) -> Option<(NodeId, VisualUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Visual(update),
        } => Some((*id, *update)),
        _ => None,
    }
}

pub(super) fn keyboard_accelerator_update(
    command: &Command,
) -> Option<(NodeId, Vec<KeyboardAcceleratorSpec>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Input(InputUpdate::KeyboardAccelerators(value)),
        } => Some((*id, value.clone())),
        _ => None,
    }
}

pub(super) fn pointer_subscription_update(
    command: &Command,
) -> Option<(NodeId, PointerSubscription)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Input(InputUpdate::Pointer(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn drop_target_update(command: &Command) -> Option<(NodeId, Option<DropTarget>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Input(InputUpdate::Drop(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn pointer_logger(
    log: Rc<RefCell<Vec<(&'static str, PointerEvent)>>>,
    kind: &'static str,
) -> impl Fn(PointerEvent) {
    move |event| log.borrow_mut().push((kind, event))
}

pub(super) fn drop_logger(
    log: Rc<RefCell<Vec<(&'static str, DropEvent)>>>,
    kind: &'static str,
) -> impl Fn(windows_core::Result<DropEvent>) {
    move |result| log.borrow_mut().push((kind, result.unwrap()))
}

pub(super) fn grid_update(command: &Command) -> Option<(NodeId, AttachedUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Attached(update),
        } => Some((*id, *update)),
        _ => None,
    }
}

pub(super) fn grid_definition_update(command: &Command) -> Option<(NodeId, GridUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::Grid(update)),
        } => Some((*id, update.clone())),
        _ => None,
    }
}

pub(super) fn border_update(command: &Command) -> Option<(NodeId, BorderUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::Border(update)),
        } => Some((*id, (**update).clone())),
        _ => None,
    }
}

pub(super) fn button_emphasis_update(command: &Command) -> Option<(NodeId, ButtonEmphasis)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ButtonEmphasis(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn flyout_placement_update(command: &Command) -> Option<(NodeId, FlyoutPlacement)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::FlyoutPlacement(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn menu_bar_update(command: &Command) -> Option<(NodeId, &[MenuBarItemSpec])> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::MenuBar(items)),
        } => Some((*id, items)),
        _ => None,
    }
}

pub(super) fn menu_flyout_update(command: &Command) -> Option<(NodeId, &[MenuItemSpec])> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::MenuFlyout(items)),
        } => Some((*id, items)),
        _ => None,
    }
}

pub(super) fn stack_panel_update(command: &Command) -> Option<(NodeId, StackPanelUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::StackPanel(update)),
        } => Some((*id, *update)),
        _ => None,
    }
}

pub(super) fn padding_update(command: &Command) -> Option<(NodeId, Thickness)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::Padding(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn text_wrapping_update(command: &Command) -> Option<(NodeId, Option<TextWrapping>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::TextWrapping(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn text_trimming_update(command: &Command) -> Option<(NodeId, Option<TextTrimming>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::TextTrimming(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn text_selection_enabled_update(command: &Command) -> Option<(NodeId, Option<bool>)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::TextSelectionEnabled(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn accessibility_update(command: &Command) -> Option<(NodeId, AccessibilityUpdate)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Accessibility(update),
        } => Some((*id, update.clone())),
        _ => None,
    }
}

pub(super) fn height_update(command: &Command) -> Option<(NodeId, Dimension)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(FrameworkUpdate::Height(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn framework_updates(
    reactor: &Reactor<RecordingRuntime>,
    target: NodeId,
) -> Vec<FrameworkUpdate> {
    reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(|command| framework_update(command, target))
        .collect()
}

pub(super) fn framework_updates_in_last_batch(
    reactor: &Reactor<RecordingRuntime>,
    target: NodeId,
) -> Vec<FrameworkUpdate> {
    reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .filter_map(|command| framework_update(command, target))
        .collect()
}

pub(super) fn framework_update(command: &Command, target: NodeId) -> Option<FrameworkUpdate> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Framework(update),
        } if *id == target => Some(*update),
        _ => None,
    }
}

pub(super) fn checked_update(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ToggleChecked(value)),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn toggle_switch_update(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::On(value))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn toggle_switch_content_update(
    command: &Command,
) -> Option<(NodeId, ToggleSwitchContentUpdate)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::Content(value))),
        } => Some((*id, (**value).clone())),
        _ => None,
    }
}

pub(super) fn progress_bar_range(command: &Command) -> Option<(NodeId, RangeState)> {
    let (id, ControlUpdate::ProgressBar(update)) = control_update(command)? else {
        return None;
    };
    let ProgressBarUpdate::Range(range) = update.as_ref() else {
        return None;
    };
    Some((id, *range))
}

pub(super) fn progress_bar_indeterminate(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::ProgressBar(update)) = control_update(command)? else {
        return None;
    };
    let ProgressBarUpdate::Indeterminate(value) = update.as_ref() else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn progress_ring_range(command: &Command) -> Option<(NodeId, RangeState)> {
    let (id, ControlUpdate::ProgressRing(update)) = control_update(command)? else {
        return None;
    };
    let ProgressRingUpdate::Range(range) = update.as_ref() else {
        return None;
    };
    Some((id, *range))
}

pub(super) fn progress_ring_active(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::ProgressRing(update)) = control_update(command)? else {
        return None;
    };
    let ProgressRingUpdate::Active(value) = update.as_ref() else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn progress_ring_indeterminate(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::ProgressRing(update)) = control_update(command)? else {
        return None;
    };
    let ProgressRingUpdate::Indeterminate(value) = update.as_ref() else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn slider_range(command: &Command) -> Option<(NodeId, RangeState)> {
    let (id, ControlUpdate::Slider(update)) = control_update(command)? else {
        return None;
    };
    let SliderUpdate::Range(range) = update.as_ref() else {
        return None;
    };
    Some((id, *range))
}

pub(super) fn slider_orientation(command: &Command) -> Option<(NodeId, Orientation)> {
    let (id, ControlUpdate::Slider(update)) = control_update(command)? else {
        return None;
    };
    let SliderUpdate::Orientation(value) = update.as_ref() else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn slider_step(command: &Command) -> Option<(NodeId, f64)> {
    let (id, ControlUpdate::Slider(update)) = control_update(command)? else {
        return None;
    };
    let SliderUpdate::Step(value) = update.as_ref() else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn slider_header(command: &Command) -> Option<(NodeId, Option<String>)> {
    let (id, ControlUpdate::Slider(update)) = control_update(command)? else {
        return None;
    };
    let SliderUpdate::Header(value) = update.as_ref() else {
        return None;
    };
    Some((id, value.clone()))
}

pub(super) fn number_box_bounds(command: &Command) -> Option<(NodeId, f64, f64)> {
    let (id, ControlUpdate::NumberBox(update)) = control_update(command)? else {
        return None;
    };
    let NumberBoxUpdate::Bounds { minimum, maximum } = update else {
        return None;
    };
    Some((id, *minimum, *maximum))
}

pub(super) fn number_box_value(command: &Command) -> Option<(NodeId, Option<f64>)> {
    let (id, ControlUpdate::NumberBox(update)) = control_update(command)? else {
        return None;
    };
    let NumberBoxUpdate::Value(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn number_box_header(command: &Command) -> Option<(NodeId, Option<String>)> {
    let (id, ControlUpdate::NumberBox(update)) = control_update(command)? else {
        return None;
    };
    let NumberBoxUpdate::Header(value) = update else {
        return None;
    };
    Some((id, value.clone()))
}

pub(super) fn rating_max(command: &Command) -> Option<(NodeId, i32)> {
    let (id, ControlUpdate::RatingControl(update)) = control_update(command)? else {
        return None;
    };
    let RatingControlUpdate::Max(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn rating_placeholder(command: &Command) -> Option<(NodeId, Option<f64>)> {
    let (id, ControlUpdate::RatingControl(update)) = control_update(command)? else {
        return None;
    };
    let RatingControlUpdate::Placeholder(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn rating_caption(command: &Command) -> Option<(NodeId, &str)> {
    let (id, ControlUpdate::RatingControl(update)) = control_update(command)? else {
        return None;
    };
    let RatingControlUpdate::Caption(value) = update else {
        return None;
    };
    Some((id, value))
}

pub(super) fn rating_read_only(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::RatingControl(update)) = control_update(command)? else {
        return None;
    };
    let RatingControlUpdate::ReadOnly(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn rating_value(command: &Command) -> Option<(NodeId, Option<f64>)> {
    let (id, ControlUpdate::RatingControl(update)) = control_update(command)? else {
        return None;
    };
    let RatingControlUpdate::Value(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn color_picker_color(command: &Command) -> Option<(NodeId, Color)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::Color(value))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn color_picker_alpha(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::AlphaEnabled(
                    value,
                ))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn color_picker_hex(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::HexInputVisible(
                    value,
                ))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn color_picker_slider(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ColorPicker(ColorPickerUpdate::SliderVisible(
                    value,
                ))),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn color_picker_channel(command: &Command) -> Option<(NodeId, bool)> {
    match command {
        Command::Update {
            id,
            update:
                NativeUpdate::Control(ControlUpdate::ColorPicker(
                    ColorPickerUpdate::ChannelInputVisible(value),
                )),
        } => Some((*id, *value)),
        _ => None,
    }
}

pub(super) fn date_picker_date(command: &Command) -> Option<(NodeId, Option<DateTime>)> {
    let (id, ControlUpdate::DatePicker(update)) = control_update(command)? else {
        return None;
    };
    let DatePickerUpdate::Date(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn date_picker_day(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::DatePicker(update)) = control_update(command)? else {
        return None;
    };
    let DatePickerUpdate::DayVisible(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn date_picker_month(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::DatePicker(update)) = control_update(command)? else {
        return None;
    };
    let DatePickerUpdate::MonthVisible(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn date_picker_year(command: &Command) -> Option<(NodeId, bool)> {
    let (id, ControlUpdate::DatePicker(update)) = control_update(command)? else {
        return None;
    };
    let DatePickerUpdate::YearVisible(value) = update else {
        return None;
    };
    Some((id, *value))
}

pub(super) fn password_update_matches(command: &Command, target: NodeId, expected: &str) -> bool {
    let Some((id, ControlUpdate::PasswordBox(update))) = control_update(command) else {
        return false;
    };
    matches!(update.as_ref(), PasswordBoxUpdate::Password(value) if id == target && value == expected)
}
