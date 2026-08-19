use std::cell::Cell;
use std::mem::size_of;
use std::rc::Rc;

use super::*;
use crate::id::NodeId;

#[test]
fn rare_element_families_do_not_widen_every_description() {
    assert_eq!(size_of::<ElementKind>(), 144);
    assert_eq!(size_of::<crate::mounted::MountedKind>(), 288);
    assert_eq!(size_of::<Element>(), 160);
}

fn ignore() {}

fn ignore_bool(_: bool) {}

fn ignore_string(_: String) {}

fn ignore_float(_: f64) {}

fn ignore_optional_float(_: Option<f64>) {}

fn ignore_color(_: Color) {}

fn ignore_optional_date(_: Option<DateTime>) {}

fn ignore_dates(_: Vec<DateTime>) {}

fn ignore_pointer(_: PointerEvent) {}

fn ignore_drop(_: windows_core::Result<DropEvent>) {}

fn test_error(message: &str) -> windows_core::Error {
    windows_core::Error::new(windows_core::HRESULT(0x80004005_u32 as i32), message)
}

#[test]
#[should_panic(expected = "InfoBadge value must be nonnegative")]
fn info_badge_rejects_negative_numeric_values() {
    _ = InfoBadge::numeric(-1);
}

#[test]
fn compact_public_values_and_callbacks_cover_their_complete_operations() {
    ignore();
    ignore_bool(false);
    ignore_string(String::new());
    ignore_pointer(PointerEvent::default());
    ignore_drop(Err(test_error("ignored")));

    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let callback = Callback::new(move |value| callback_calls.set(value));
    let clone = callback.clone();
    assert!(callback == clone);
    clone.call(3);
    assert_eq!(calls.get(), 3);

    assert_eq!(VirtualKey::from_code(123).code(), 123);
    assert_eq!(VirtualKey::NUMBER_PAD_0.code(), 96);
    assert_eq!(VirtualKey::NUMBER_PAD_9.code(), 105);
    assert_eq!(VirtualKey::MULTIPLY.code(), 106);
    assert_eq!(VirtualKey::ADD.code(), 107);
    assert_eq!(VirtualKey::SUBTRACT.code(), 109);
    assert_eq!(VirtualKey::DECIMAL.code(), 110);
    assert_eq!(VirtualKey::DIVIDE.code(), 111);
    let mut modifiers = VirtualKeyModifiers::CONTROL;
    modifiers |= VirtualKeyModifiers::SHIFT;
    assert_eq!(
        modifiers.bits(),
        (VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT).bits()
    );

    let mut formats = DropFormats::TEXT;
    formats |= DropFormats::STORAGE_ITEMS;
    assert!(formats.contains(DropFormats::TEXT));
    assert!(formats.intersects(DropFormats::STORAGE_ITEMS));
    assert!(!formats.is_empty());
    let target = DropTarget::new(DropOperation::Link, formats);
    assert_eq!(target.operation(), DropOperation::Link);
    assert_eq!(target.formats(), formats);

    let error = test_error("drop failed");
    assert!(error.to_string().starts_with("drop failed"));

    let mut handlers = PointerHandlers::default();
    assert!(handlers.is_empty());
    handlers.capture_on_press = true;
    assert!(!handlers.is_empty());

    let first_accelerator =
        KeyboardAccelerator::new(VirtualKey::A, VirtualKeyModifiers::CONTROL, ignore);
    let same_accelerator = first_accelerator.clone();
    let different_accelerator =
        KeyboardAccelerator::new(VirtualKey::B, VirtualKeyModifiers::CONTROL, ignore);
    assert!(first_accelerator == same_accelerator);
    assert!(first_accelerator != different_accelerator);

    assert_eq!(DropFormats::TEXT | DropFormats::STORAGE_ITEMS, formats);

    let callback = Callback::new(ignore_drop);
    let handler = DropHandler {
        target,
        callback: callback.clone(),
    };
    assert_eq!(handler.target(), target);
    assert!(handler.callback().ptr_eq(&callback));
    assert!(handler == handler);

    assert!(KeyboardAcceleratorList::from_vec(Vec::new()).is_none());
    let one = KeyboardAcceleratorList::from_vec(vec![KeyboardAccelerator::new(
        VirtualKey::A,
        VirtualKeyModifiers::NONE,
        ignore,
    )])
    .unwrap();
    assert_eq!(one.as_slice().len(), 1);

    assert_eq!(Thickness::from(4.0), Thickness::uniform(4.0));
}

#[test]
fn compact_property_storage_covers_empty_and_populated_states() {
    let mut props = FrameworkProps::default();
    props.set_width(None);
    props.set_height(None);
    props.set_min_width(None);
    props.set_max_width(None);
    props.set_min_height(None);
    props.set_max_height(None);
    props.set_margin(None);
    props.set_horizontal_alignment(None);
    props.set_vertical_alignment(None);
    props.set_visibility(None);
    props.set_opacity(None);
    props.set_enabled(None);
    props.set_font_size(None);
    props.set_character_spacing(None);
    props.set_font_weight(None);
    props.set_font_style(None);
    props.set_font_stretch(None);
    props.set_text_wrapping(None);
    props.set_text_trimming(None);
    props.set_text_selection_enabled(None);
    props.set_keyboard_accelerators(Vec::new());
    props.compact_input();
    assert!(props.data.is_none());

    props.set_automation_name(Some("name".into()));
    props.set_automation_name(None);
    props.set_automation_id(Some("id".into()));
    props.set_automation_id(None);
    props.set_heading_level(Some(AutomationHeadingLevel::Level1));
    props.set_heading_level(None);
    props.set_help_text(Some("help".into()));
    props.set_help_text(None);
    props.set_font_family(Some("Arial".into()));
    props.set_font_family(None);
    props.set_foreground(Some(Color::rgb(1, 2, 3).into()));
    props.set_foreground(None);

    props.pointer_mut();
    props.compact_input();
    assert!(props.data.is_none());

    props.push_keyboard_accelerator(KeyboardAccelerator::new(
        VirtualKey::A,
        VirtualKeyModifiers::NONE,
        ignore,
    ));
    props.push_keyboard_accelerator(KeyboardAccelerator::new(
        VirtualKey::B,
        VirtualKeyModifiers::NONE,
        ignore,
    ));
    assert_eq!(props.keyboard_accelerators().len(), 2);
    props.set_keyboard_accelerators(Vec::new());
    assert!(props.data.is_none());
}

#[test]
fn compact_scalar_storage_round_trips_every_public_value() {
    let mut scalar = ScalarProps::default();
    for value in [Visibility::Visible, Visibility::Collapsed] {
        scalar.set_visibility(Some(value));
        assert_eq!(scalar.visual().visibility(), Some(value));
    }
    scalar.set_visibility(None);
    assert_eq!(scalar.visual().visibility(), None);

    for value in [false, true] {
        scalar.set_enabled(Some(value));
        assert_eq!(scalar.control().enabled(), Some(value));
    }
    scalar.set_enabled(None);
    assert_eq!(scalar.control().enabled(), None);

    for value in [FontStyle::Normal, FontStyle::Oblique, FontStyle::Italic] {
        scalar.set_font_style(Some(value));
        assert_eq!(scalar.text_style().font_style(), Some(value));
    }
    scalar.set_font_style(None);
    assert_eq!(scalar.text_style().font_style(), None);

    for value in [
        FontStretch::Undefined,
        FontStretch::UltraCondensed,
        FontStretch::ExtraCondensed,
        FontStretch::Condensed,
        FontStretch::SemiCondensed,
        FontStretch::Normal,
        FontStretch::SemiExpanded,
        FontStretch::Expanded,
        FontStretch::ExtraExpanded,
        FontStretch::UltraExpanded,
    ] {
        scalar.set_font_stretch(Some(value));
        assert_eq!(scalar.text_style().font_stretch(), Some(value));
    }
    scalar.set_font_stretch(None);
    assert_eq!(scalar.text_style().font_stretch(), None);

    for value in [
        TextWrapping::NoWrap,
        TextWrapping::Wrap,
        TextWrapping::WrapWholeWords,
    ] {
        scalar.set_text_wrapping(Some(value));
        assert_eq!(scalar.text_block_style().text_wrapping(), Some(value));
    }
    scalar.set_text_wrapping(None);
    assert_eq!(scalar.text_block_style().text_wrapping(), None);

    for value in [
        TextTrimming::None,
        TextTrimming::CharacterEllipsis,
        TextTrimming::WordEllipsis,
        TextTrimming::Clip,
    ] {
        scalar.set_text_trimming(Some(value));
        assert_eq!(scalar.text_block_style().text_trimming(), Some(value));
    }
    scalar.set_text_trimming(None);
    assert_eq!(scalar.text_block_style().text_trimming(), None);

    for value in [false, true] {
        scalar.set_text_selection_enabled(Some(value));
        assert_eq!(
            scalar.text_block_style().text_selection_enabled(),
            Some(value)
        );
    }
    scalar.set_text_selection_enabled(None);
    assert_eq!(scalar.text_block_style().text_selection_enabled(), None);
}

#[test]
fn attached_placement_defaults_and_values_round_trip() {
    let mut relative = RelativePanelPlacement::default();
    for (shift, value) in [
        (RelativePanelPlacement::LEFT, false),
        (RelativePanelPlacement::RIGHT, true),
        (RelativePanelPlacement::TOP, false),
        (RelativePanelPlacement::BOTTOM, true),
        (RelativePanelPlacement::HORIZONTAL_CENTER, false),
        (RelativePanelPlacement::VERTICAL_CENTER, true),
    ] {
        relative.set(shift, Some(value));
    }
    assert_eq!(relative.align_left(), Some(false));
    assert_eq!(relative.align_right(), Some(true));
    assert_eq!(relative.align_top(), Some(false));
    assert_eq!(relative.align_bottom(), Some(true));
    assert_eq!(relative.align_horizontal_center(), Some(false));
    assert_eq!(relative.align_vertical_center(), Some(true));

    let canvas = CanvasPlacement {
        left: 1.0,
        top: 2.0,
        z_index: 3,
        flags: CanvasPlacement::LEFT | CanvasPlacement::TOP | CanvasPlacement::Z_INDEX,
    };
    assert_eq!(canvas.left(), Some(1.0));
    assert_eq!(canvas.top(), Some(2.0));
    assert_eq!(canvas.z_index(), Some(3));

    let grid = GridPlacement {
        row: 1,
        column: 2,
        row_span: 3,
        column_span: 4,
    };
    assert_eq!(grid.row(), Some(1));
    assert_eq!(grid.column(), Some(2));
    assert_eq!(grid.row_span(), Some(3));
    assert_eq!(grid.column_span(), Some(4));

    assert_eq!(
        AttachedPlacement::Grid(grid).default_for(),
        AttachedPlacement::Grid(GridPlacement::default())
    );
    assert_eq!(
        AttachedPlacement::Canvas(canvas).default_for(),
        AttachedPlacement::Canvas(CanvasPlacement::default())
    );
    assert_eq!(
        AttachedPlacement::RelativePanel(relative).default_for(),
        AttachedPlacement::RelativePanel(RelativePanelPlacement::default())
    );
}

#[test]
fn every_framework_capability_builder_constructs_an_element() {
    macro_rules! framework_starters {
        ($make:expr) => {{
            let _ = ($make).width(1.0).build();
            let _ = ($make).height(1.0).build();
            let _ = ($make).min_width(1.0).build();
            let _ = ($make).max_width(1.0).build();
            let _ = ($make).min_height(1.0).build();
            let _ = ($make).max_height(1.0).build();
            let _ = ($make).margin(Thickness::uniform(1.0)).build();
            let _ = ($make)
                .horizontal_alignment(HorizontalAlignment::Center)
                .build();
            let _ = ($make)
                .vertical_alignment(VerticalAlignment::Center)
                .build();
            let _ = ($make).visibility(Visibility::Collapsed).build();
            let _ = ($make).opacity(0.5).build();
            let _ = ($make).automation_name("name").build();
            let _ = ($make).automation_id("id").build();
            let _ = ($make)
                .heading_level(AutomationHeadingLevel::Level2)
                .build();
            let _ = ($make).help_text("help").build();
            let _ = ($make)
                .keyboard_accelerator(KeyboardAccelerator::new(
                    VirtualKey::A,
                    VirtualKeyModifiers::CONTROL,
                    ignore,
                ))
                .build();
            let _ = ($make)
                .keyboard_accelerators([KeyboardAccelerator::new(
                    VirtualKey::B,
                    VirtualKeyModifiers::SHIFT,
                    ignore,
                )])
                .build();
            let _ = ($make).on_pointer_pressed(ignore_pointer).build();
            let _ = ($make).on_pointer_moved(ignore_pointer).build();
            let _ = ($make).on_pointer_released(ignore_pointer).build();
            let _ = ($make).on_pointer_capture_lost(ignore_pointer).build();
            let _ = ($make).on_pointer_canceled(ignore_pointer).build();
            let _ = ($make).on_pointer_entered(ignore_pointer).build();
            let _ = ($make).on_pointer_exited(ignore_pointer).build();
            let _ = ($make).on_tapped(ignore).build();
            let _ = ($make).on_right_tapped(ignore).build();
            let _ = ($make).capture_pointer_on_press().build();
            let _ = ($make)
                .on_drop(
                    DropTarget::new(DropOperation::Copy, DropFormats::TEXT),
                    ignore_drop,
                )
                .build();
        }};
    }

    framework_starters!(Button::new("button").on_click(ignore));
    framework_starters!(HyperlinkButton::new("hyperlink button").on_click(ignore));
    framework_starters!(RepeatButton::new("repeat button").on_click(ignore));
    framework_starters!(ToggleButton::new("toggle button", false, ignore_bool));
    framework_starters!(ToggleSwitch::new(false, ignore_bool));
    framework_starters!(ProgressBar::new(25.0));
    framework_starters!(ProgressRing::new(25.0));
    framework_starters!(Slider::new(25.0, ignore_float));
    framework_starters!(NumberBox::new(25.0, ignore_optional_float));
    framework_starters!(RatingControl::new(3.0, ignore_optional_float));
    framework_starters!(ColorPicker::new(Color::rgb(10, 20, 30), ignore_color));
    framework_starters!(DatePicker::new(None, ignore_optional_date));
    framework_starters!(CalendarDatePicker::new(None, ignore_optional_date));
    framework_starters!(CalendarView::new([], ignore_dates));
    framework_starters!(CheckBox::new("check box", false, ignore_bool));
    framework_starters!(RadioButton::new("radio button", false, ignore_bool));
    framework_starters!(PasswordBox::new("password", ignore_string));
    framework_starters!(TextBlock::new("text"));
    framework_starters!(TextBox::new("text", ignore_string));
    framework_starters!(Border::new(text_block("child")));
    framework_starters!(StackPanel::new([text_block("child")]));
    framework_starters!(Grid::new([text_block("child")]));
    framework_starters!(Canvas::new([text_block("child")]));
    framework_starters!(RelativePanel::new([text_block("child")]));
    framework_starters!(Viewbox::new(text_block("child")));
    framework_starters!(ScrollViewer::new(text_block("child")));
    framework_starters!(ScrollView::new(text_block("child")));
    framework_starters!(SplitView::display(
        text_block("content"),
        text_block("pane")
    ));
    framework_starters!(Expander::display(
        text_block("header"),
        text_block("content")
    ));

    let _ = TextBlock::new("text")
        .width(1.0)
        .width(1.5)
        .height(2.0)
        .min_width(3.0)
        .max_width(4.0)
        .min_height(5.0)
        .max_height(6.0)
        .margin(Thickness::uniform(7.0))
        .horizontal_alignment(HorizontalAlignment::Right)
        .vertical_alignment(VerticalAlignment::Bottom)
        .visibility(Visibility::Visible)
        .opacity(0.75)
        .automation_name("name")
        .automation_id("id")
        .heading_level(AutomationHeadingLevel::Level3)
        .help_text("help")
        .keyboard_accelerator(KeyboardAccelerator::new(
            VirtualKey::C,
            VirtualKeyModifiers::MENU,
            ignore,
        ))
        .keyboard_accelerators([KeyboardAccelerator::new(
            VirtualKey::D,
            VirtualKeyModifiers::WINDOWS,
            ignore,
        )])
        .on_pointer_pressed(ignore_pointer)
        .on_pointer_moved(ignore_pointer)
        .on_pointer_released(ignore_pointer)
        .on_pointer_capture_lost(ignore_pointer)
        .on_pointer_canceled(ignore_pointer)
        .on_pointer_entered(ignore_pointer)
        .on_pointer_exited(ignore_pointer)
        .on_tapped(ignore)
        .on_right_tapped(ignore)
        .capture_pointer_on_press()
        .on_drop(
            DropTarget::new(DropOperation::Move, DropFormats::STORAGE_ITEMS),
            ignore_drop,
        )
        .build();
}

#[test]
fn control_and_text_capability_builders_construct_elements() {
    macro_rules! enabled_starter {
        ($make:expr) => {
            let _ = ($make).enabled(false).enabled(true).build();
        };
    }
    enabled_starter!(Button::new("button").on_click(ignore));
    enabled_starter!(HyperlinkButton::new("hyperlink button").on_click(ignore));
    enabled_starter!(RepeatButton::new("repeat button").on_click(ignore));
    enabled_starter!(ToggleButton::new("toggle button", false, ignore_bool));
    enabled_starter!(ToggleSwitch::new(false, ignore_bool));
    enabled_starter!(ProgressBar::new(25.0));
    enabled_starter!(ProgressRing::new(25.0));
    enabled_starter!(Slider::new(25.0, ignore_float));
    enabled_starter!(NumberBox::new(25.0, ignore_optional_float));
    enabled_starter!(RatingControl::new(3.0, ignore_optional_float));
    enabled_starter!(ColorPicker::new(Color::rgb(10, 20, 30), ignore_color));
    enabled_starter!(DatePicker::new(None, ignore_optional_date));
    enabled_starter!(CalendarDatePicker::new(None, ignore_optional_date));
    enabled_starter!(CalendarView::new([], ignore_dates));
    enabled_starter!(CheckBox::new("check box", false, ignore_bool));
    enabled_starter!(RadioButton::new("radio button", false, ignore_bool));
    enabled_starter!(PasswordBox::new("password", ignore_string));
    enabled_starter!(TextBox::new("text", ignore_string));
    enabled_starter!(ScrollViewer::new(text_block("child")));
    enabled_starter!(ScrollView::new(text_block("child")));
    enabled_starter!(SplitView::display(
        text_block("content"),
        text_block("pane")
    ));
    enabled_starter!(Expander::display(
        text_block("header"),
        text_block("content")
    ));

    macro_rules! text_starters {
        ($make:expr) => {{
            let _ = ($make).font_size(12.0).build();
            let _ = ($make).character_spacing(10).build();
            let _ = ($make).font_weight(FontWeight::BOLD).build();
            let _ = ($make).font_style(FontStyle::Italic).build();
            let _ = ($make).font_stretch(FontStretch::Expanded).build();
            let _ = ($make).font_family(Some("Arial".to_string())).build();
            let _ = ($make).foreground(Color::rgb(1, 2, 3)).build();
        }};
    }
    text_starters!(Button::new("button").on_click(ignore));
    text_starters!(HyperlinkButton::new("hyperlink button").on_click(ignore));
    text_starters!(RepeatButton::new("repeat button").on_click(ignore));
    text_starters!(ToggleButton::new("toggle button", false, ignore_bool));
    text_starters!(CheckBox::new("check box", false, ignore_bool));
    text_starters!(RadioButton::new("radio button", false, ignore_bool));
    text_starters!(PasswordBox::new("password", ignore_string));
    text_starters!(TextBlock::new("text"));
    text_starters!(TextBox::new("text", ignore_string));

    let _ = ToggleSwitch::new(false, ignore_bool)
        .header("Header")
        .on_content("Yes")
        .off_content("No")
        .build();
    let _ = RadioButtons::display([(1, "One")]).header("Header").build();
    let _ = Slider::new(25.0, ignore_float)
        .step(2.0)
        .header("Volume")
        .build();
    let _ = NumberBox::new(2.0, ignore_optional_float)
        .header("Quantity")
        .build();
    let _ = TextBox::new("text", ignore_string)
        .header("Header")
        .placeholder_text("Placeholder")
        .multiline()
        .build();
    let _ = PasswordBox::new("password", ignore_string)
        .header("Header")
        .placeholder_text("Placeholder")
        .password_reveal_mode(PasswordRevealMode::Hidden)
        .build();

    let _ = TextBlock::new("text")
        .font_size(12.0)
        .character_spacing(10)
        .font_weight(FontWeight::BOLD)
        .font_style(FontStyle::Italic)
        .font_stretch(FontStretch::Expanded)
        .font_family(None)
        .foreground(Color::rgb(1, 2, 3))
        .text_wrapping(TextWrapping::Wrap)
        .text_trimming(TextTrimming::Clip)
        .text_selection_enabled(true)
        .build();

    let _ = TextBlock::new("text").text_wrapping(TextWrapping::Wrap);
    let _ = TextBlock::new("text").text_trimming(TextTrimming::Clip);
    let _ = TextBlock::new("text").text_selection_enabled(true);

    let _ = vstack(
        8.0,
        [
            text_block("top"),
            hstack(4.0, [text_block("left"), text_block("right")]),
        ],
    );
    let _ = grid([grid_child(text_block("child"))]);
    let _ = canvas([canvas_child(text_block("child"))]);
    let _ = relative_panel([relative_panel_child(text_block("child"))]);
    let _ = viewbox(text_block("child"));
    let _ = scroll_viewer(text_block("child"));
    let _ = scroll_view(text_block("child"));
    let _ = split_view(text_block("content"), text_block("pane"));
    let _ = expander(text_block("header"), text_block("content"));
    let _ = hyperlink_button("hyperlink", ignore);
    let _ = repeat_button("repeat", ignore);
    let _ = toggle_button("toggle", false, ignore_bool);
    let _ = toggle_switch(false, ignore_bool);
    let _ = password_box("password", ignore_string);
}

#[test]
fn framework_builders_are_order_independent() {
    let _ = Button::new("button")
        .width(120.0)
        .on_click(ignore)
        .enabled(true)
        .font_size(14.0)
        .build();
    let _ = Button::new("button")
        .on_click(ignore)
        .font_size(14.0)
        .enabled(true)
        .width(120.0)
        .build();

    let _ = Slider::new(25.0, ignore_float)
        .width(200.0)
        .range(0.0, 50.0)
        .enabled(true)
        .build();
    let _ = Slider::new(25.0, ignore_float)
        .range(0.0, 50.0)
        .enabled(true)
        .width(200.0)
        .build();

    let _ = TextBox::new("text", ignore_string)
        .width(200.0)
        .header("Header")
        .font_size(14.0)
        .enabled(true)
        .build();
    let _ = TextBox::new("text", ignore_string)
        .header("Header")
        .enabled(true)
        .font_size(14.0)
        .width(200.0)
        .build();

    let _ = Viewbox::new(text_block("content"))
        .width(120.0)
        .stretch(Stretch::UniformToFill)
        .build();
    let _ = Viewbox::new(text_block("content"))
        .stretch(Stretch::UniformToFill)
        .width(120.0)
        .build();

    let _ = Grid::new(std::iter::empty::<Element>())
        .width(240.0)
        .columns([GridLength::STAR])
        .build();
    let _ = Grid::new(std::iter::empty::<Element>())
        .columns([GridLength::STAR])
        .width(240.0)
        .build();

    let _ = ProgressBar::indeterminate()
        .width(200.0)
        .range(0.0, 1.0)
        .enabled(true)
        .build();
    let _ = ProgressBar::indeterminate()
        .range(0.0, 1.0)
        .enabled(true)
        .width(200.0)
        .build();

    let _ = TextBlock::new("text")
        .width(200.0)
        .padding(Thickness::uniform(4.0))
        .font_size(14.0)
        .text_wrapping(TextWrapping::Wrap)
        .build();
    let _ = TextBlock::new("text")
        .padding(Thickness::uniform(4.0))
        .text_wrapping(TextWrapping::Wrap)
        .font_size(14.0)
        .width(200.0)
        .build();
}

#[test]
fn display_only_control_constructors_enforce_noninteractive_state() {
    let controls = [
        ToggleButton::display("toggle", true).build(),
        ToggleSwitch::display(true).build(),
        Slider::display(25.0).build(),
        NumberBox::display(25.0).build(),
        ColorPicker::display(Color::rgb(10, 20, 30)).build(),
        DatePicker::display(None).build(),
        CalendarDatePicker::display(None).build(),
        TimePicker::display(None).build(),
        CalendarView::display([]).build(),
        CheckBox::display("check", true).build(),
        RadioButton::display("radio", true).build(),
        PasswordBox::display("password").build(),
        TextBox::display("text").build(),
        AutoSuggestBox::display("query").build(),
        ListBox::display([(1, "one")]).build(),
        ComboBox::display([(1, "one")]).build(),
        RadioButtons::display([(1, "one")]).build(),
        FlipView::display([FlipViewItem::new(1, text_block("page"))]).build(),
        Pivot::display([PivotItem::new(1, "page", text_block("page"))]).build(),
        TabView::display([TabViewItem::new(1, "page", text_block("page"))]).build(),
        SelectorBar::display([SelectorBarItem::new(1, "page")]).build(),
        NavigationView::display([NavigationItem::new(1, "page")], text_block("content")).build(),
    ];

    for control in controls {
        assert_eq!(
            control.kind.framework_props().unwrap().control().enabled(),
            Some(false)
        );
    }

    let rating = RatingControl::display(3.0).build();
    let ElementKind::RatingControl(props) = rating.kind else {
        unreachable!()
    };
    assert!(props.read_only);
    assert!(props.on_change.is_none());

    let rich_edit = RichEditBox::display("text").build();
    let ElementKind::RichEditBox(props) = rich_edit.kind else {
        unreachable!()
    };
    assert!(props.read_only);
    assert!(props.on_change.is_none());
}

#[test]
fn composite_and_virtual_display_contracts_preserve_content_without_feedback() {
    let split = SplitView::display(text_block("content"), text_block("pane")).build();
    let ElementKind::SplitView(split) = split.kind else {
        unreachable!()
    };
    assert!(split.props.on_pane_closed.is_none());

    let expander = Expander::display(text_block("header"), text_block("content")).build();
    let ElementKind::Expander(expander) = expander.kind else {
        unreachable!()
    };
    assert!(expander.props.on_expanded_changed.is_none());

    let tree = TreeView::display([TreeNode::new(1, "root").expanded(true)]).build();
    let ElementKind::TreeView(tree) = tree.kind else {
        unreachable!()
    };
    assert!(tree.on_expanded_changed.is_none());

    let collection = VirtualList::new(1, 100.0, |_| text_block("row"))
        .display_selection(CollectionSelection::new([0]))
        .build();
    let ElementKind::VirtualCollection(collection) = collection.kind else {
        unreachable!()
    };
    assert!(collection.selection_display_only);
    assert!(collection.on_selection_changed.is_none());
    assert!(!collection.can_reorder_items);
}

#[test]
#[should_panic(expected = "display-only controls cannot be enabled")]
fn display_only_control_rejects_enabled_override() {
    _ = CheckBox::display("check", true).enabled(true).build();
}

#[test]
fn navigation_uri_uses_owned_validated_storage() {
    let uri = NavigationUri::from("https://example.com");
    assert_eq!(uri.as_str(), "https://example.com");
    assert_eq!(NavigationUri::from("https://example.com".to_string()), uri);
    assert_eq!(
        NavigationUri::from(Box::<str>::from("https://example.com")),
        uri
    );
    _ = HyperlinkButton::new("Docs")
        .on_click(ignore)
        .navigate_uri("https://example.com")
        .build();
}

#[test]
#[should_panic(expected = "navigation URI must not be empty")]
fn navigation_uri_rejects_empty_values() {
    _ = NavigationUri::new("");
}

#[test]
#[should_panic(expected = "RepeatButton delay must be nonnegative")]
fn repeat_button_rejects_negative_delay() {
    _ = RepeatButton::new("repeat").on_click(ignore).delay(-1);
}

#[test]
#[should_panic(expected = "RepeatButton interval must be nonnegative")]
fn repeat_button_rejects_negative_interval() {
    _ = RepeatButton::new("repeat").on_click(ignore).interval(-1);
}

#[test]
fn progress_controls_accept_ranges_set_after_values() {
    _ = ProgressBar::new(225.0).range(200.0, 300.0).build();
    _ = ProgressRing::new(-25.0)
        .range(-50.0, 0.0)
        .active(false)
        .build();
    _ = ProgressBar::indeterminate().build();
    _ = ProgressRing::indeterminate().build();
}

#[test]
#[should_panic(expected = "range value and bounds must be finite")]
fn progress_controls_reject_nonfinite_values() {
    _ = ProgressBar::new(f64::NAN).build();
}

#[test]
#[should_panic(expected = "range minimum must not exceed maximum")]
fn progress_controls_reject_reversed_ranges() {
    _ = ProgressRing::new(1.0).range(2.0, 1.0).build();
}

#[test]
#[should_panic(expected = "value must be within the configured range")]
fn progress_controls_reject_values_outside_the_range() {
    _ = ProgressBar::new(101.0).build();
}

#[test]
fn number_box_accepts_values_empty_state_and_broad_default_range() {
    _ = NumberBox::new(-25.0, ignore_optional_float)
        .range(-50.0, 0.0)
        .build();
    _ = NumberBox::new(None, ignore_optional_float)
        .range(0.0, 100.0)
        .build();
    _ = number_box(Some(f64::MAX), ignore_optional_float);
}

#[test]
#[should_panic(expected = "NumberBox value must be finite")]
fn number_box_rejects_nonfinite_values() {
    _ = NumberBox::new(f64::INFINITY, ignore_optional_float).build();
}

#[test]
#[should_panic(expected = "range value and bounds must be finite")]
fn number_box_rejects_nonfinite_bounds() {
    _ = NumberBox::new(None, ignore_optional_float)
        .range(0.0, f64::INFINITY)
        .build();
}

#[test]
#[should_panic(expected = "value must be within the configured range")]
fn number_box_rejects_values_outside_the_range() {
    _ = NumberBox::new(101.0, ignore_optional_float)
        .range(0.0, 100.0)
        .build();
}

#[test]
fn rating_control_accepts_optional_values_and_configuration() {
    _ = RatingControl::new(None, ignore_optional_float)
        .max_rating(10)
        .placeholder(7.5)
        .caption("Average")
        .read_only(true)
        .build();
    _ = rating_control(Some(3.5), ignore_optional_float);
}

#[test]
#[should_panic(expected = "maximum rating must be positive")]
fn rating_control_rejects_nonpositive_maximum() {
    _ = RatingControl::new(None, ignore_optional_float)
        .max_rating(0)
        .build();
}

#[test]
#[should_panic(expected = "rating values must be finite")]
fn rating_control_rejects_nonfinite_values() {
    _ = RatingControl::new(f64::NAN, ignore_optional_float).build();
}

#[test]
#[should_panic(expected = "rating values must be within the configured maximum")]
fn rating_control_rejects_values_above_the_maximum() {
    _ = RatingControl::new(6.0, ignore_optional_float).build();
}

#[test]
#[should_panic(expected = "rating values must be within the configured maximum")]
fn rating_control_rejects_placeholders_above_the_maximum() {
    _ = RatingControl::new(None, ignore_optional_float)
        .placeholder(6.0)
        .build();
}

#[test]
fn color_picker_accepts_controlled_color_and_visibility_options() {
    _ = ColorPicker::new(Color::argb(128, 10, 20, 30), ignore_color)
        .alpha_enabled(false)
        .hex_input_visible(false)
        .color_slider_visible(false)
        .color_channel_text_input_visible(false)
        .build();
    _ = color_picker(Color::rgb(30, 20, 10), ignore_color);
}

#[test]
fn date_picker_accepts_optional_dates_and_visibility_options() {
    _ = DatePicker::new(DateTime::UNIX_EPOCH, ignore_optional_date)
        .header("Date")
        .day_visible(false)
        .month_visible(false)
        .year_visible(false)
        .build();
    _ = date_picker(None, ignore_optional_date);
    _ = CalendarDatePicker::new(None, ignore_optional_date)
        .header("Date")
        .placeholder_text("Choose")
        .today_highlighted(false)
        .build();
    _ = calendar_date_picker(None, ignore_optional_date);
    _ = CalendarView::new([DateTime::UNIX_EPOCH, DateTime::UNIX_EPOCH], ignore_dates)
        .selection_mode(CalendarSelectionMode::Multiple)
        .today_highlighted(false)
        .group_label_visible(true)
        .build();
    _ = ComboBox::display([(1, "one")])
        .header("Header")
        .placeholder_text("Placeholder")
        .editable(true)
        .build();
}

#[test]
fn heap_properties_share_storage_and_release_it_when_empty() {
    assert_eq!(size_of::<FrameworkData>(), 120);
    assert_eq!(size_of::<HeapProps>(), 136);

    let mut props = FrameworkProps::default();
    props.set_automation_name(Some("name".to_string()));
    props.set_automation_id(Some("id".to_string()));
    props.set_heading_level(Some(AutomationHeadingLevel::Level1));
    props.set_font_family(Some("Arial".to_string()));
    assert_eq!(props.automation_name(), Some("name"));
    assert_eq!(props.automation_id(), Some("id"));
    assert_eq!(props.heading_level(), Some(AutomationHeadingLevel::Level1));
    assert_eq!(props.font_family(), Some("Arial"));

    props.set_automation_name(None);
    props.set_automation_id(None);
    props.set_heading_level(None);
    assert_eq!(props.automation_name(), None);
    assert_eq!(props.font_family(), Some("Arial"));

    props.set_help_text(Some("help".to_string()));
    props.set_font_family(None);
    props.set_automation_name(Some(String::new()));
    props.set_foreground(Some(Color::argb(128, 10, 20, 30).into()));
    props.set_keyboard_accelerators(vec![KeyboardAccelerator::new(
        VirtualKey::S,
        VirtualKeyModifiers::CONTROL,
        || {},
    )]);
    assert_eq!(props.automation_name(), Some(""));
    assert_eq!(props.help_text(), Some("help"));
    assert_eq!(props.font_family(), None);
    assert_eq!(
        props.foreground(),
        Some(&Brush::Solid(Color::argb(128, 10, 20, 30)))
    );
    assert_eq!(props.keyboard_accelerators().len(), 1);

    props.set_help_text(None);
    props.set_automation_name(None);
    props.set_foreground(None);
    assert!(props.data.is_some());
    props.set_keyboard_accelerators(Vec::new());
    assert!(props.data.is_none());
}

#[test]
#[should_panic(expected = "duplicate keyboard accelerator")]
fn keyboard_accelerators_reject_duplicate_signatures() {
    let mut props = FrameworkProps::default();
    props.set_keyboard_accelerators(vec![
        KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::CONTROL, || {}),
        KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::CONTROL, || {}),
    ]);
}

#[test]
fn keyboard_accelerators_preserve_multiple_signatures() {
    let mut props = FrameworkProps::default();
    props.set_keyboard_accelerators(vec![
        KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::CONTROL, || {}),
        KeyboardAccelerator::new(VirtualKey::F5, VirtualKeyModifiers::NONE, || {}),
    ]);
    let values = props.keyboard_accelerators();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].key(), VirtualKey::S);
    assert_eq!(values[1].key(), VirtualKey::F5);
}

#[test]
fn grid_placement_uses_validated_compact_sentinels() {
    assert_eq!(size_of::<GridPlacement>(), 16);
    let default = GridPlacement::default();
    assert_eq!(default.row(), None);
    assert_eq!(default.column(), None);
    assert_eq!(default.row_span(), None);
    assert_eq!(default.column_span(), None);

    let child = GridChild::new(text_block("cell"))
        .row(i32::MAX)
        .column(0)
        .row_span(1)
        .column_span(i32::MAX);
    assert_eq!(child.placement.row(), Some(i32::MAX));
    assert_eq!(child.placement.column(), Some(0));
    assert_eq!(child.placement.row_span(), Some(1));
    assert_eq!(child.placement.column_span(), Some(i32::MAX));
}

#[test]
fn grid_definitions_accept_typed_lengths() {
    let grid = Grid::new([text_block("cell")])
        .columns([
            GridLength::Auto,
            GridLength::Pixel(0.0),
            GridLength::Star(2.0),
        ])
        .rows([GridLength::STAR])
        .column_spacing(6.0)
        .row_spacing(8.0);
    assert_eq!(
        grid.control.columns,
        [
            GridLength::Auto,
            GridLength::Pixel(0.0),
            GridLength::Star(2.0),
        ]
    );
    assert_eq!(grid.control.rows, [GridLength::STAR]);
    assert_eq!(grid.control.column_spacing, 6.0);
    assert_eq!(grid.control.row_spacing, 8.0);
}

#[test]
#[should_panic(expected = "Grid length must be finite and nonnegative")]
fn grid_definitions_reject_negative_lengths() {
    _ = Grid::new([text_block("cell")]).columns([GridLength::Pixel(-1.0)]);
}

#[test]
#[should_panic(expected = "Grid length must be finite and nonnegative")]
fn grid_definitions_reject_nonfinite_lengths() {
    _ = Grid::new([text_block("cell")]).rows([GridLength::Star(f64::NAN)]);
}

#[test]
#[should_panic(expected = "Grid column spacing must be finite and nonnegative")]
fn grid_rejects_negative_column_spacing() {
    _ = Grid::new([text_block("cell")]).column_spacing(-1.0);
}

#[test]
#[should_panic(expected = "Grid row spacing must be finite and nonnegative")]
fn grid_rejects_nonfinite_row_spacing() {
    _ = Grid::new([text_block("cell")]).row_spacing(f64::INFINITY);
}

#[test]
#[should_panic(expected = "Slider step must be finite and positive")]
fn slider_rejects_zero_step() {
    _ = Slider::new(0.0, ignore_float).step(0.0);
}

#[test]
#[should_panic(expected = "Slider step must be finite and positive")]
fn slider_rejects_negative_step() {
    _ = Slider::new(0.0, ignore_float).step(-1.0);
}

#[test]
#[should_panic(expected = "Slider step must be finite and positive")]
fn slider_rejects_nan_step() {
    _ = Slider::new(0.0, ignore_float).step(f64::NAN);
}

#[test]
#[should_panic(expected = "Slider step must be finite and positive")]
fn slider_rejects_infinite_step() {
    _ = Slider::new(0.0, ignore_float).step(f64::INFINITY);
}

#[test]
fn border_and_button_style_builders_accept_typed_values() {
    let _ = Border::new(text_block("child"))
        .background(Color::rgb(1, 2, 3))
        .border_brush(Color::rgb(4, 5, 6))
        .border_thickness(Thickness::uniform(2.0))
        .corner_radius(CornerRadius::uniform(8.0))
        .padding(Thickness::uniform(4.0))
        .build();
    let _ = Button::new("primary")
        .on_click(ignore)
        .emphasis(ButtonEmphasis::Accent)
        .build();
}

#[test]
#[should_panic(expected = "border thickness must be finite and nonnegative")]
fn border_rejects_negative_thickness() {
    _ = Border::new(text_block("child")).border_thickness(Thickness::uniform(-1.0));
}

#[test]
#[should_panic(expected = "corner radius must be finite and nonnegative")]
fn border_rejects_nonfinite_corner_radius() {
    _ = Border::new(text_block("child")).corner_radius(CornerRadius::uniform(f64::NAN));
}

#[test]
fn stack_panel_layout_and_text_padding_accept_valid_values() {
    let stack = StackPanel::new([text_block("cell")])
        .orientation(Orientation::Horizontal)
        .spacing(0.0)
        .padding(Thickness::xy(4.0, 2.0));
    assert_eq!(stack.control.orientation, Orientation::Horizontal);
    assert_eq!(stack.control.spacing, 0.0);
    assert_eq!(stack.control.padding, Some(Thickness::xy(4.0, 2.0)));

    let text = TextBlock::new("cell").padding(Thickness::uniform(3.0));
    assert_eq!(text.control.padding, Some(Thickness::uniform(3.0)));
}

#[test]
#[should_panic(expected = "StackPanel spacing must be finite and nonnegative")]
fn stack_panel_rejects_negative_spacing() {
    _ = StackPanel::new([text_block("cell")]).spacing(-1.0);
}

#[test]
#[should_panic(expected = "StackPanel spacing must be finite and nonnegative")]
fn stack_panel_rejects_nonfinite_spacing() {
    _ = StackPanel::new([text_block("cell")]).spacing(f64::INFINITY);
}

#[test]
#[should_panic(expected = "padding must be finite and nonnegative")]
fn stack_panel_rejects_invalid_padding() {
    _ = StackPanel::new([text_block("cell")]).padding(Thickness::uniform(-1.0));
}

#[test]
#[should_panic(expected = "padding must be finite and nonnegative")]
fn text_block_rejects_invalid_padding() {
    _ = TextBlock::new("cell").padding(Thickness::xy(f64::NAN, 0.0));
}

#[test]
fn canvas_placement_preserves_all_values() {
    assert_eq!(size_of::<CanvasPlacement>(), 24);
    let child = CanvasChild::new(text_block("cell"))
        .left(f64::NAN)
        .top(f64::INFINITY)
        .z_index(-99);
    assert!(child.placement.left().unwrap().is_nan());
    assert_eq!(child.placement.top(), Some(f64::INFINITY));
    assert_eq!(child.placement.z_index(), Some(-99));
}

#[test]
fn relative_panel_placement_packs_optional_booleans() {
    assert_eq!(size_of::<RelativePanelPlacement>(), 2);
    let child = RelativePanelChild::new(text_block("cell"))
        .align_left(true)
        .align_right(false)
        .align_top(true)
        .align_bottom(false)
        .align_horizontal_center(true)
        .align_vertical_center(false);
    assert_eq!(child.placement.align_left(), Some(true));
    assert_eq!(child.placement.align_right(), Some(false));
    assert_eq!(child.placement.align_top(), Some(true));
    assert_eq!(child.placement.align_bottom(), Some(false));
    assert_eq!(child.placement.align_horizontal_center(), Some(true));
    assert_eq!(child.placement.align_vertical_center(), Some(false));
}

#[test]
fn text_flow_uses_the_remaining_scalar_flag_bits() {
    assert_eq!(size_of::<FrameworkData>(), 120);

    let mut props = FrameworkProps::default();
    for value in [
        TextWrapping::NoWrap,
        TextWrapping::Wrap,
        TextWrapping::WrapWholeWords,
    ] {
        props.set_text_wrapping(Some(value));
        assert_eq!(props.text_block_style().text_wrapping(), Some(value));
    }
    for value in [
        TextTrimming::None,
        TextTrimming::CharacterEllipsis,
        TextTrimming::WordEllipsis,
        TextTrimming::Clip,
    ] {
        props.set_text_trimming(Some(value));
        assert_eq!(props.text_block_style().text_trimming(), Some(value));
    }
    props.set_font_weight(FontWeight::from_weight(999));
    props.set_text_selection_enabled(Some(false));
    assert_eq!(props.text_style().font_weight().unwrap().weight(), 999);
    assert_eq!(
        props.text_block_style().text_selection_enabled(),
        Some(false)
    );
    props.set_text_selection_enabled(Some(true));
    assert_eq!(
        props.text_block_style().text_selection_enabled(),
        Some(true)
    );

    props.set_text_wrapping(None);
    props.set_text_trimming(None);
    props.set_text_selection_enabled(None);
    props.set_font_weight(None);
    assert!(props.data.is_none());
}

#[test]
fn element_reference_cell_covers_mount_clear_and_stale_work() {
    let reference = ElementRef::<TextBox>::default();
    assert_eq!(format!("{reference:?}"), "ElementRef { mounted: false }");
    let binding = reference.binding();
    let first = NodeId::new(1, 0);
    binding.prepare_mount(first, None);
    binding.commit(NodeId::new(1, 1));
    assert!(!reference.is_mounted());
    assert!(binding.clear().is_none());
    binding.commit(first);
    assert!(!reference.is_mounted());

    let mounts = Rc::new(Cell::new(0));
    let unmounts = Rc::new(Cell::new(0));
    let mounts_for_callback = Rc::clone(&mounts);
    let unmounts_for_callback = Rc::clone(&unmounts);
    reference.set_lifecycle(
        Some(Rc::new(move || {
            mounts_for_callback.set(mounts_for_callback.get() + 1);
        })),
        Some(Rc::new(move || {
            unmounts_for_callback.set(unmounts_for_callback.get() + 1);
        })),
    );
    binding.prepare_mount(first, None);
    binding.commit(first);
    assert!(reference.is_mounted());
    assert_eq!(format!("{reference:?}"), "ElementRef { mounted: true }");
    assert_eq!(mounts.get(), 1);
    let cleanup = binding.clear().unwrap();
    assert!(!reference.is_mounted());
    assert_eq!(unmounts.get(), 0);
    cleanup();
    assert_eq!(unmounts.get(), 1);

    reference.set_lifecycle(None, None);
    let second = NodeId::new(1, 1);
    binding.prepare_mount(second, None);
    binding.commit(second);
    assert!(binding.clear().is_none());
}

#[test]
fn plain_element_reference_clears_lifecycle_callbacks() {
    let reference = ElementRef::<TextBox>::new();
    reference.set_lifecycle(Some(Rc::new(|| {})), Some(Rc::new(|| {})));
    reference.clear_lifecycle();
    let binding = reference.binding();
    let id = NodeId::new(1, 0);
    binding.prepare_mount(id, None);
    binding.commit(id);
    assert!(binding.clear().is_none());
}
