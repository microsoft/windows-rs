use std::cell::Cell;
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Element;
use windows_reactor::Reconciler;
use windows_reactor::{
    AutoSuggestBox, CalendarDatePicker, CalendarView, ColorArgb, ColorPicker, DatePicker,
    DropDownButton, ListBox, PersonPicture, SplitButton, TimePicker,
};
use windows_reactor::{ControlKind, Event, Prop, PropValue};

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

fn first_create(r: &Reconciler<RecordingBackend>) -> (ControlKind, windows_reactor::ControlId) {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create { id, kind } => Some((*kind, *id)),
            _ => None,
        })
        .expect("no Create op")
}

#[test]
fn auto_suggest_box_mounts_with_text_items_placeholder_and_header() {
    let el: Element = AutoSuggestBox::new("init")
        .items(["A", "B"])
        .placeholder_text("Search...")
        .header("Query")
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::AutoSuggestBox);

    let mut saw_text = false;
    let mut saw_items = false;
    let mut saw_placeholder = false;
    let mut saw_header = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Text, PropValue::Str(s)) if s == "init" => saw_text = true,
                (Prop::Items, PropValue::StrList(v))
                    if v == &["A".to_string(), "B".to_string()] =>
                {
                    saw_items = true;
                }
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "Search..." => {
                    saw_placeholder = true;
                }
                (Prop::Header, PropValue::Str(s)) if s == "Query" => saw_header = true,
                _ => {}
            }
        }
    }
    assert!(saw_text && saw_items && saw_placeholder && saw_header);
}

#[test]
fn calendar_date_picker_mounts_with_header_placeholder_and_today_highlighted() {
    let el: Element = CalendarDatePicker::new()
        .header("Date")
        .placeholder_text("Pick...")
        .today_highlighted(false)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::CalendarDatePicker);

    let mut saw_header = false;
    let mut saw_placeholder = false;
    let mut saw_today = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Header, PropValue::Str(s)) if s == "Date" => saw_header = true,
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "Pick..." => {
                    saw_placeholder = true;
                }
                (Prop::IsTodayHighlighted, PropValue::Bool(false)) => saw_today = true,
                _ => {}
            }
        }
    }
    assert!(saw_header && saw_placeholder && saw_today);
}

#[test]
fn calendar_view_mounts_with_today_highlight_and_group_label_visibility() {
    let el: Element = CalendarView::new()
        .today_highlighted(false)
        .group_label_visible(false)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::CalendarView);

    let mut saw_today = false;
    let mut saw_group_labels = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::IsTodayHighlighted, PropValue::Bool(false)) => saw_today = true,
                (Prop::IsGroupLabelVisible, PropValue::Bool(false)) => saw_group_labels = true,
                _ => {}
            }
        }
    }
    assert!(saw_today && saw_group_labels);
}

#[test]
fn color_picker_mounts_with_alpha_and_hex_visibility() {
    let el: Element = ColorPicker::new(ColorArgb::new(1, 2, 3))
        .alpha_enabled(false)
        .hex_input_visible(false)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ColorPicker);

    let mut saw_alpha = false;
    let mut saw_hex = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::IsAlphaEnabled, PropValue::Bool(false)) => saw_alpha = true,
                (Prop::IsHexInputVisible, PropValue::Bool(false)) => saw_hex = true,
                _ => {}
            }
        }
    }
    assert!(saw_alpha && saw_hex);
}

#[test]
fn date_picker_mounts_with_header_and_component_visibility() {
    let el: Element = DatePicker::new()
        .header("Birthday")
        .day_visible(false)
        .year_visible(false)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::DatePicker);

    let mut saw_header = false;
    let mut saw_day = false;
    let mut saw_year = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Header, PropValue::Str(s)) if s == "Birthday" => saw_header = true,
                (Prop::DayVisible, PropValue::Bool(false)) => saw_day = true,
                (Prop::YearVisible, PropValue::Bool(false)) => saw_year = true,
                _ => {}
            }
        }
    }
    assert!(saw_header && saw_day && saw_year);
}

#[test]
fn time_picker_mounts_with_header_clock_identifier_and_minute_increment() {
    let el: Element = TimePicker::new()
        .header("Time")
        .clock_identifier("24HourClock")
        .minute_increment(15)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::TimePicker);

    let mut saw_header = false;
    let mut saw_clock = false;
    let mut saw_increment = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Header, PropValue::Str(s)) if s == "Time" => saw_header = true,
                (Prop::ClockIdentifier, PropValue::Str(s)) if s == "24HourClock" => {
                    saw_clock = true;
                }
                (Prop::MinuteIncrement, PropValue::I32(15)) => saw_increment = true,
                _ => {}
            }
        }
    }
    assert!(saw_header && saw_clock && saw_increment);
}

#[test]
fn drop_down_button_mounts_with_content_and_disabled() {
    let el: Element = DropDownButton::new("Options").enabled(false).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::DropDownButton);

    let mut saw_content = false;
    let mut saw_enabled = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Content, PropValue::Str(s)) if s == "Options" => saw_content = true,
                (Prop::IsEnabled, PropValue::Bool(false)) => saw_enabled = true,
                _ => {}
            }
        }
    }
    assert!(saw_content && saw_enabled);
}

#[test]
fn drop_down_button_click_event_fires() {
    let count = Rc::new(Cell::new(0));
    let count_c = Rc::clone(&count);
    let el: Element = DropDownButton::new("Options")
        .on_click(move || count_c.set(count_c.get() + 1))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire(id, Event::Click);
    assert_eq!(count.get(), 1);
}

#[test]
fn split_button_mounts_with_content_and_disabled() {
    let el: Element = SplitButton::new("Paste").enabled(false).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::SplitButton);

    let mut saw_content = false;
    let mut saw_enabled = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Content, PropValue::Str(s)) if s == "Paste" => saw_content = true,
                (Prop::IsEnabled, PropValue::Bool(false)) => saw_enabled = true,
                _ => {}
            }
        }
    }
    assert!(saw_content && saw_enabled);
}

#[test]
fn split_button_click_event_fires() {
    let count = Rc::new(Cell::new(0));
    let count_c = Rc::clone(&count);
    let el: Element = SplitButton::new("Paste")
        .on_click(move || count_c.set(count_c.get() + 1))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire(id, Event::Click);
    assert_eq!(count.get(), 1);
}

#[test]
fn list_box_mounts_with_items_selection_and_disabled() {
    let el: Element = ListBox::new()
        .items(["A", "B", "C"])
        .selected_index(1)
        .enabled(false)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ListBox);

    let mut saw_items = false;
    let mut saw_selected = false;
    let mut saw_enabled = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Items, PropValue::StrList(v))
                    if v == &["A".to_string(), "B".to_string(), "C".to_string()] =>
                {
                    saw_items = true;
                }
                (Prop::SelectedIndex, PropValue::I32(1)) => saw_selected = true,
                (Prop::IsEnabled, PropValue::Bool(false)) => saw_enabled = true,
                _ => {}
            }
        }
    }
    assert!(saw_items && saw_selected && saw_enabled);
}

#[test]
fn person_picture_mounts_with_display_name_and_initials() {
    let el: Element = PersonPicture::new()
        .display_name("John")
        .initials("JD")
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::PersonPicture);

    let mut saw_display_name = false;
    let mut saw_initials = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::DisplayName, PropValue::Str(s)) if s == "John" => saw_display_name = true,
                (Prop::Initials, PropValue::Str(s)) if s == "JD" => saw_initials = true,
                _ => {}
            }
        }
    }
    assert!(saw_display_name && saw_initials);
}
