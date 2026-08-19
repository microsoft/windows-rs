use super::*;
use crate::element::props::{
    CalendarDatePickerProps, CalendarViewProps, ColorPickerProps, DatePickerProps, NumberBoxProps,
    ProgressBarProps, ProgressRingProps, RatingControlProps, SliderProps, TimePickerProps,
    ToggleSwitchProps,
};

struct ToggleSwitchChanges {
    on: bool,
    content: bool,
}

struct ProgressBarChanges {
    range: bool,
    indeterminate: bool,
}

struct ProgressRingChanges {
    range: bool,
    active: bool,
    indeterminate: bool,
}

struct SliderChanges {
    range: bool,
    orientation: bool,
    step: bool,
    header: bool,
}

struct NumberBoxChanges {
    bounds: bool,
    value: bool,
    header: bool,
}

struct RatingChanges {
    max: bool,
    placeholder: bool,
    caption: bool,
    read_only: bool,
    value: bool,
}

struct ColorPickerChanges {
    color: bool,
    alpha: bool,
    hex: bool,
    slider: bool,
    channel: bool,
}

struct DatePickerChanges {
    date: bool,
    header: bool,
    day: bool,
    month: bool,
    year: bool,
}

struct CalendarDatePickerChanges {
    date: bool,
    header: bool,
    placeholder: bool,
    today_highlighted: bool,
}

struct TimePickerChanges {
    time: bool,
    header: bool,
    minute_increment: bool,
}

struct CalendarViewChanges {
    selected_dates: bool,
    selection_mode: bool,
    today_highlighted: bool,
    group_label_visible: bool,
}

pub(super) fn mount_toggle_switch<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ToggleSwitchProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ToggleSwitch)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::On(props.on)),
    )?;
    if props.header.is_some() || props.on_content.is_some() || props.off_content.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::Content(Box::new(
                ToggleSwitchContentUpdate {
                    header: props.header.clone(),
                    on_content: props.on_content.clone(),
                    off_content: props.off_content.clone(),
                },
            ))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::ToggleSwitch(props));
    Ok(id)
}

pub(super) fn mount_progress_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ProgressBarProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ProgressBar)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ProgressBar(Box::new(ProgressBarUpdate::Range(RangeState {
            value: props.value,
            minimum: props.minimum,
            maximum: props.maximum,
        }))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ProgressBar(Box::new(ProgressBarUpdate::Indeterminate(
            props.indeterminate,
        ))),
    )?;
    set_mounted(engine, id, key, MountedKind::ProgressBar(props));
    Ok(id)
}

pub(super) fn mount_progress_ring<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ProgressRingProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ProgressRing)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Range(RangeState {
            value: props.value,
            minimum: props.minimum,
            maximum: props.maximum,
        }))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Active(props.active))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Indeterminate(
            props.indeterminate,
        ))),
    )?;
    set_mounted(engine, id, key, MountedKind::ProgressRing(props));
    Ok(id)
}

pub(super) fn mount_slider<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: SliderProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::Slider)?;
    engine.queue_control_update(
        id,
        ControlUpdate::Slider(Box::new(SliderUpdate::Range(RangeState {
            value: props.value,
            minimum: props.minimum,
            maximum: props.maximum,
        }))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::Slider(Box::new(SliderUpdate::Orientation(props.orientation))),
    )?;
    if props.step != 1.0 {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Step(props.step))),
        )?;
    }
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Header(props.header.clone()))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::Slider(props));
    Ok(id)
}

pub(super) fn mount_number_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: NumberBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::NumberBox)?;
    engine.queue_control_update(
        id,
        ControlUpdate::NumberBox(NumberBoxUpdate::Bounds {
            minimum: props.minimum,
            maximum: props.maximum,
        }),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::NumberBox(NumberBoxUpdate::Value(props.value)),
    )?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::NumberBox(NumberBoxUpdate::Header(props.header.clone())),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::NumberBox(props));
    Ok(id)
}

pub(super) fn mount_rating_control<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: RatingControlProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RatingControl)?;
    engine.queue_control_update(
        id,
        ControlUpdate::RatingControl(RatingControlUpdate::Max(props.max_rating)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::RatingControl(RatingControlUpdate::Placeholder(props.placeholder)),
    )?;
    engine.set_rating_caption(id, props.caption.clone())?;
    engine.queue_control_update(
        id,
        ControlUpdate::RatingControl(RatingControlUpdate::ReadOnly(props.read_only)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::RatingControl(RatingControlUpdate::Value(props.value)),
    )?;
    set_mounted(engine, id, key, MountedKind::RatingControl(props));
    Ok(id)
}

pub(super) fn mount_color_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ColorPickerProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ColorPicker)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ColorPicker(ColorPickerUpdate::Color(props.color)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ColorPicker(ColorPickerUpdate::AlphaEnabled(props.alpha_enabled)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ColorPicker(ColorPickerUpdate::HexInputVisible(props.hex_input_visible)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ColorPicker(ColorPickerUpdate::SliderVisible(props.color_slider_visible)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ColorPicker(ColorPickerUpdate::ChannelInputVisible(
            props.color_channel_text_input_visible,
        )),
    )?;
    set_mounted(engine, id, key, MountedKind::ColorPicker(props));
    Ok(id)
}

pub(super) fn mount_date_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: DatePickerProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::DatePicker)?;
    engine.queue_control_update(
        id,
        ControlUpdate::DatePicker(DatePickerUpdate::Date(props.date)),
    )?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::Header(props.header.clone())),
        )?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::DatePicker(DatePickerUpdate::DayVisible(props.day_visible)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::DatePicker(DatePickerUpdate::MonthVisible(props.month_visible)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::DatePicker(DatePickerUpdate::YearVisible(props.year_visible)),
    )?;
    set_mounted(engine, id, key, MountedKind::DatePicker(props));
    Ok(id)
}

pub(super) fn mount_calendar_date_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: CalendarDatePickerProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::CalendarDatePicker)?;
    engine.queue_control_update(
        id,
        ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Date(props.date))),
    )?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Header(
                props.header.clone(),
            ))),
        )?;
    }
    if props.placeholder.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::TodayHighlighted(
            props.today_highlighted,
        ))),
    )?;
    set_mounted(engine, id, key, MountedKind::CalendarDatePicker(props));
    Ok(id)
}

pub(super) fn mount_time_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: TimePickerProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::TimePicker)?;
    engine.queue_control_update(
        id,
        ControlUpdate::TimePicker(Box::new(time_picker_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::TimePicker(props));
    Ok(id)
}

pub(super) fn mount_calendar_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: CalendarViewProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::CalendarView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::CalendarView(Box::new(calendar_view_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::CalendarView(props));
    Ok(id)
}

pub(super) fn mount_rich_edit_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: RichEditBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RichEditBox)?;
    engine.queue_control_update(
        id,
        ControlUpdate::RichEditBox(Box::new(rich_edit_box_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::RichEditBox(Box::new(props)));
    Ok(id)
}

pub(super) fn mount_rich_text_block<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: RichTextBlockProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RichTextBlock)?;
    engine.queue_control_update(
        id,
        ControlUpdate::RichTextBlock(Box::new(rich_text_block_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::RichTextBlock(Box::new(props)));
    Ok(id)
}

pub(super) fn mount_tree_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: TreeViewProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::TreeView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::TreeView(Box::new(TreeViewUpdate::Nodes(Rc::clone(&props.nodes)))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TreeView(Box::new(TreeViewUpdate::ExpandedChanged(
            props.on_expanded_changed.is_some(),
        ))),
    )?;
    set_mounted(engine, id, key, MountedKind::TreeView(props));
    Ok(id)
}

pub(super) fn reconcile_toggle_switch<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ToggleSwitchProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ToggleSwitch(old) => ToggleSwitchChanges {
            on: old.on != props.on,
            content: (
                old.header.as_ref(),
                old.on_content.as_ref(),
                old.off_content.as_ref(),
            ) != (
                props.header.as_ref(),
                props.on_content.as_ref(),
                props.off_content.as_ref(),
            ),
        },
        _ => unreachable!(),
    };
    if changes.on {
        engine.queue_control_update(
            id,
            ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::On(props.on)),
        )?;
    }
    if changes.content {
        engine.queue_control_update(
            id,
            ControlUpdate::ToggleSwitch(ToggleSwitchUpdate::Content(Box::new(
                ToggleSwitchContentUpdate {
                    header: props.header.clone(),
                    on_content: props.on_content.clone(),
                    off_content: props.off_content.clone(),
                },
            ))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::ToggleSwitch(props));
    Ok(())
}

pub(super) fn reconcile_progress_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ProgressBarProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ProgressBar(old) => ProgressBarChanges {
            range: (old.value, old.minimum, old.maximum)
                != (props.value, props.minimum, props.maximum),
            indeterminate: old.indeterminate != props.indeterminate,
        },
        _ => unreachable!(),
    };
    if changes.range {
        engine.queue_control_update(
            id,
            ControlUpdate::ProgressBar(Box::new(ProgressBarUpdate::Range(RangeState {
                value: props.value,
                minimum: props.minimum,
                maximum: props.maximum,
            }))),
        )?;
    }
    if changes.indeterminate {
        engine.queue_control_update(
            id,
            ControlUpdate::ProgressBar(Box::new(ProgressBarUpdate::Indeterminate(
                props.indeterminate,
            ))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::ProgressBar(props));
    Ok(())
}

pub(super) fn reconcile_progress_ring<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ProgressRingProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ProgressRing(old) => ProgressRingChanges {
            range: (old.value, old.minimum, old.maximum)
                != (props.value, props.minimum, props.maximum),
            active: old.active != props.active,
            indeterminate: old.indeterminate != props.indeterminate,
        },
        _ => unreachable!(),
    };
    if changes.range {
        engine.queue_control_update(
            id,
            ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Range(RangeState {
                value: props.value,
                minimum: props.minimum,
                maximum: props.maximum,
            }))),
        )?;
    }
    if changes.active {
        engine.queue_control_update(
            id,
            ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Active(props.active))),
        )?;
    }
    if changes.indeterminate {
        engine.queue_control_update(
            id,
            ControlUpdate::ProgressRing(Box::new(ProgressRingUpdate::Indeterminate(
                props.indeterminate,
            ))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::ProgressRing(props));
    Ok(())
}

pub(super) fn reconcile_slider<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: SliderProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Slider(old) => SliderChanges {
            range: (old.value, old.minimum, old.maximum)
                != (props.value, props.minimum, props.maximum),
            orientation: old.orientation != props.orientation,
            step: old.step != props.step,
            header: old.header != props.header,
        },
        _ => unreachable!(),
    };
    if changes.range {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Range(RangeState {
                value: props.value,
                minimum: props.minimum,
                maximum: props.maximum,
            }))),
        )?;
    }
    if changes.orientation {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Orientation(props.orientation))),
        )?;
    }
    if changes.step {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Step(props.step))),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::Slider(Box::new(SliderUpdate::Header(props.header.clone()))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::Slider(props));
    Ok(())
}

pub(super) fn reconcile_number_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: NumberBoxProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::NumberBox(old) => NumberBoxChanges {
            bounds: (old.minimum, old.maximum) != (props.minimum, props.maximum),
            value: old.value != props.value,
            header: old.header != props.header,
        },
        _ => unreachable!(),
    };
    if changes.bounds {
        engine.queue_control_update(
            id,
            ControlUpdate::NumberBox(NumberBoxUpdate::Bounds {
                minimum: props.minimum,
                maximum: props.maximum,
            }),
        )?;
    }
    if changes.bounds || changes.value {
        engine.queue_control_update(
            id,
            ControlUpdate::NumberBox(NumberBoxUpdate::Value(props.value)),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::NumberBox(NumberBoxUpdate::Header(props.header.clone())),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::NumberBox(props));
    Ok(())
}

pub(super) fn reconcile_rating_control<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: RatingControlProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RatingControl(old) => RatingChanges {
            max: old.max_rating != props.max_rating,
            placeholder: old.placeholder != props.placeholder,
            caption: old.caption != props.caption,
            read_only: old.read_only != props.read_only,
            value: old.value != props.value,
        },
        _ => unreachable!(),
    };
    if changes.max {
        engine.queue_control_update(
            id,
            ControlUpdate::RatingControl(RatingControlUpdate::Max(props.max_rating)),
        )?;
    }
    if changes.max || changes.placeholder {
        engine.queue_control_update(
            id,
            ControlUpdate::RatingControl(RatingControlUpdate::Placeholder(props.placeholder)),
        )?;
    }
    if changes.caption {
        engine.set_rating_caption(id, props.caption.clone())?;
    }
    if changes.read_only {
        engine.queue_control_update(
            id,
            ControlUpdate::RatingControl(RatingControlUpdate::ReadOnly(props.read_only)),
        )?;
    }
    if changes.max || changes.value {
        engine.queue_control_update(
            id,
            ControlUpdate::RatingControl(RatingControlUpdate::Value(props.value)),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::RatingControl(props));
    Ok(())
}

pub(super) fn reconcile_color_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ColorPickerProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ColorPicker(old) => ColorPickerChanges {
            color: old.color != props.color,
            alpha: old.alpha_enabled != props.alpha_enabled,
            hex: old.hex_input_visible != props.hex_input_visible,
            slider: old.color_slider_visible != props.color_slider_visible,
            channel: old.color_channel_text_input_visible != props.color_channel_text_input_visible,
        },
        _ => unreachable!(),
    };
    if changes.color {
        engine.queue_control_update(
            id,
            ControlUpdate::ColorPicker(ColorPickerUpdate::Color(props.color)),
        )?;
    }
    if changes.alpha {
        engine.queue_control_update(
            id,
            ControlUpdate::ColorPicker(ColorPickerUpdate::AlphaEnabled(props.alpha_enabled)),
        )?;
    }
    if changes.hex {
        engine.queue_control_update(
            id,
            ControlUpdate::ColorPicker(ColorPickerUpdate::HexInputVisible(props.hex_input_visible)),
        )?;
    }
    if changes.slider {
        engine.queue_control_update(
            id,
            ControlUpdate::ColorPicker(ColorPickerUpdate::SliderVisible(
                props.color_slider_visible,
            )),
        )?;
    }
    if changes.channel {
        engine.queue_control_update(
            id,
            ControlUpdate::ColorPicker(ColorPickerUpdate::ChannelInputVisible(
                props.color_channel_text_input_visible,
            )),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::ColorPicker(props));
    Ok(())
}

pub(super) fn reconcile_date_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: DatePickerProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::DatePicker(old) => DatePickerChanges {
            date: old.date != props.date,
            header: old.header != props.header,
            day: old.day_visible != props.day_visible,
            month: old.month_visible != props.month_visible,
            year: old.year_visible != props.year_visible,
        },
        _ => unreachable!(),
    };
    if changes.date {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::Date(props.date)),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::Header(props.header.clone())),
        )?;
    }
    if changes.day {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::DayVisible(props.day_visible)),
        )?;
    }
    if changes.month {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::MonthVisible(props.month_visible)),
        )?;
    }
    if changes.year {
        engine.queue_control_update(
            id,
            ControlUpdate::DatePicker(DatePickerUpdate::YearVisible(props.year_visible)),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::DatePicker(props));
    Ok(())
}

pub(super) fn reconcile_calendar_date_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: CalendarDatePickerProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::CalendarDatePicker(old) => CalendarDatePickerChanges {
            date: old.date != props.date,
            header: old.header != props.header,
            placeholder: old.placeholder != props.placeholder,
            today_highlighted: old.today_highlighted != props.today_highlighted,
        },
        _ => unreachable!(),
    };
    if changes.date {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Date(props.date))),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Header(
                props.header.clone(),
            ))),
        )?;
    }
    if changes.placeholder {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(CalendarDatePickerUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if changes.today_highlighted {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarDatePicker(Box::new(
                CalendarDatePickerUpdate::TodayHighlighted(props.today_highlighted),
            )),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::CalendarDatePicker(props));
    Ok(())
}

pub(super) fn reconcile_time_picker<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: TimePickerProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::TimePicker(old) => TimePickerChanges {
            time: old.time != props.time,
            header: old.header != props.header,
            minute_increment: old.minute_increment != props.minute_increment,
        },
        _ => unreachable!(),
    };
    if changes.time || changes.header || changes.minute_increment {
        engine.queue_control_update(
            id,
            ControlUpdate::TimePicker(Box::new(time_picker_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::TimePicker(props));
    Ok(())
}

pub(super) fn reconcile_calendar_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: CalendarViewProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::CalendarView(old) => CalendarViewChanges {
            selected_dates: old.selected_dates != props.selected_dates,
            selection_mode: old.selection_mode != props.selection_mode,
            today_highlighted: old.today_highlighted != props.today_highlighted,
            group_label_visible: old.group_label_visible != props.group_label_visible,
        },
        _ => unreachable!(),
    };
    if changes.selected_dates
        || changes.selection_mode
        || changes.today_highlighted
        || changes.group_label_visible
    {
        engine.queue_control_update(
            id,
            ControlUpdate::CalendarView(Box::new(calendar_view_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::CalendarView(props));
    Ok(())
}

pub(super) fn reconcile_rich_edit_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: RichEditBoxProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RichEditBox(old) => {
            (
                old.text.as_str(),
                old.header.as_ref(),
                old.placeholder.as_ref(),
                old.read_only,
            ) != (
                props.text.as_str(),
                props.header.as_ref(),
                props.placeholder.as_ref(),
                props.read_only,
            )
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::RichEditBox(Box::new(rich_edit_box_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::RichEditBox(Box::new(props)));
    Ok(())
}

pub(super) fn reconcile_rich_text_block<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: RichTextBlockProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RichTextBlock(old) => {
            (
                old.paragraphs.as_ref(),
                old.font_size,
                old.selectable,
                old.wrap,
            ) != (
                props.paragraphs.as_ref(),
                props.font_size,
                props.selectable,
                props.wrap,
            )
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::RichTextBlock(Box::new(rich_text_block_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::RichTextBlock(Box::new(props)));
    Ok(())
}

pub(super) fn reconcile_tree_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: TreeViewProps,
) -> Result<(), EngineError> {
    let (nodes_changed, feedback_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::TreeView(old) => (
                old.nodes != props.nodes,
                old.on_expanded_changed.is_some() != props.on_expanded_changed.is_some(),
            ),
            _ => unreachable!(),
        };
    if nodes_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TreeView(Box::new(TreeViewUpdate::Nodes(Rc::clone(&props.nodes)))),
        )?;
    }
    if feedback_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TreeView(Box::new(TreeViewUpdate::ExpandedChanged(
                props.on_expanded_changed.is_some(),
            ))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::TreeView(props));
    Ok(())
}

fn calendar_view_update(props: &CalendarViewProps) -> CalendarViewUpdate {
    CalendarViewUpdate {
        selected_dates: Rc::clone(&props.selected_dates),
        selection_mode: props.selection_mode,
        today_highlighted: props.today_highlighted,
        group_label_visible: props.group_label_visible,
    }
}

fn rich_edit_box_update(props: &RichEditBoxProps) -> RichEditBoxUpdate {
    RichEditBoxUpdate {
        text: props.text.clone(),
        header: props.header.clone(),
        placeholder: props.placeholder.clone(),
        read_only: props.read_only,
    }
}

fn rich_text_block_update(props: &RichTextBlockProps) -> RichTextBlockUpdate {
    RichTextBlockUpdate {
        paragraphs: Rc::clone(&props.paragraphs),
        font_size: props.font_size,
        selectable: props.selectable,
        wrap: props.wrap,
    }
}

fn time_picker_update(props: &TimePickerProps) -> TimePickerUpdate {
    TimePickerUpdate {
        time: props.time,
        header: props.header.clone(),
        minute_increment: props.minute_increment,
    }
}

fn set_mounted<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    kind: MountedKind,
) {
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted { key, kind });
}
