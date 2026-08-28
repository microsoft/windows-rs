#![allow(clippy::useless_conversion)]
use windows_reactor::*;
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum SurfaceKind {
    Control,
    Property,
    Event,
    CapabilityProperty,
    Structural,
    Extension,
}
pub(crate) struct SurfaceCase {
    pub(crate) name: &'static str,
    pub(crate) kind: SurfaceKind,
    pub(crate) stages: usize,
    pub(crate) subscription_delta: Option<usize>,
    pub(crate) build: fn(usize) -> View,
}
pub struct PropertySurface {
    pub control: &'static str,
    pub property: &'static str,
    pub value: &'static str,
    pub adapter: &'static str,
    pub validation: Option<&'static str>,
    pub clearable: bool,
    pub theme_style: bool,
}
pub struct EventSurface {
    pub control: &'static str,
    pub event: &'static str,
    pub payload: &'static str,
    pub conversion: &'static str,
    pub subscription: &'static str,
    pub delivery: &'static str,
    pub active_property: Option<&'static str>,
}
pub struct CapabilityPropertySurface {
    pub capability: &'static str,
    pub property: &'static str,
}
pub struct StructuralSurface {
    pub control: &'static str,
    pub member: &'static str,
}
pub struct ExtensionSurface {
    pub name: &'static str,
}
pub(crate) const PROJECTED_CONTROL_COUNT: usize = 79usize;
pub(crate) const PROJECTED_PROPERTY_COUNT: usize = 230usize;
pub(crate) const PROJECTED_EVENT_COUNT: usize = 63usize;
pub(crate) const CAPABILITY_PROPERTY_COUNT: usize = 27usize;
pub(crate) const STRUCTURAL_COUNT: usize = 64usize;
pub(crate) const EXTENSION_COUNT: usize = 5;
fn construct_text_block(_stage: usize) -> View {
    Grid::new().children((TextBlock::new(),))
}
fn construct_button(_stage: usize) -> View {
    Grid::new().children((Button::new(),))
}
fn construct_hyperlink_button(_stage: usize) -> View {
    Grid::new().children((HyperlinkButton::new(),))
}
fn construct_repeat_button(_stage: usize) -> View {
    Grid::new().children((RepeatButton::new(),))
}
fn construct_border(_stage: usize) -> View {
    Grid::new().children((Border::new(),))
}
fn construct_breadcrumb_bar(_stage: usize) -> View {
    Grid::new().children((BreadcrumbBar::new(),))
}
fn construct_stack_panel(_stage: usize) -> View {
    Grid::new().children((StackPanel::new(),))
}
fn construct_variable_sized_wrap_grid(_stage: usize) -> View {
    Grid::new().children((VariableSizedWrapGrid::new(),))
}
fn construct_grid(_stage: usize) -> View {
    Grid::new().children((Grid::new(),))
}
fn construct_text_box(_stage: usize) -> View {
    Grid::new().children((TextBox::new(),))
}
fn construct_auto_suggest_box(_stage: usize) -> View {
    Grid::new().children((AutoSuggestBox::new(),))
}
fn construct_password_box(_stage: usize) -> View {
    Grid::new().children((PasswordBox::new(),))
}
fn construct_number_box(_stage: usize) -> View {
    Grid::new().children((NumberBox::new(),))
}
fn construct_slider(_stage: usize) -> View {
    Grid::new().children((Slider::new(),))
}
fn construct_title_bar(_stage: usize) -> View {
    Grid::new().children((TitleBar::new().slots(std::iter::empty::<SlotView<TitleBarSlot>>()),))
}
fn construct_navigation_view(_stage: usize) -> View {
    Grid::new().children((NavigationView::new(),))
}
fn construct_navigation_view_item(_stage: usize) -> View {
    Grid::new().children((NavigationViewItem::new(),))
}
fn construct_split_view(_stage: usize) -> View {
    Grid::new().children((SplitView::new(),))
}
fn construct_progress_bar(_stage: usize) -> View {
    Grid::new().children((ProgressBar::new(),))
}
fn construct_toggle_switch(_stage: usize) -> View {
    Grid::new().children((ToggleSwitch::new(),))
}
fn construct_check_box(_stage: usize) -> View {
    Grid::new().children((CheckBox::new(),))
}
fn construct_toggle_button(_stage: usize) -> View {
    Grid::new().children((ToggleButton::new(),))
}
fn construct_radio_button(_stage: usize) -> View {
    Grid::new().children((RadioButton::new(),))
}
fn construct_radio_buttons(_stage: usize) -> View {
    Grid::new().children((RadioButtons::new(),))
}
fn construct_items_repeater(_stage: usize) -> View {
    Grid::new().children((ItemsRepeater::new(),))
}
fn construct_info_badge(_stage: usize) -> View {
    Grid::new().children((InfoBadge::new(),))
}
fn construct_info_bar(_stage: usize) -> View {
    Grid::new().children((InfoBar::new(),))
}
fn construct_person_picture(_stage: usize) -> View {
    Grid::new().children((PersonPicture::new(),))
}
fn construct_scroll_viewer(_stage: usize) -> View {
    Grid::new().children((ScrollViewer::new(),))
}
fn construct_scroll_view(_stage: usize) -> View {
    Grid::new().children((ScrollView::new(),))
}
fn construct_image(_stage: usize) -> View {
    Grid::new().children((Image::new(),))
}
fn construct_progress_ring(_stage: usize) -> View {
    Grid::new().children((ProgressRing::new(),))
}
fn construct_list_box(_stage: usize) -> View {
    Grid::new().children((ListBox::new(),))
}
fn construct_rectangle(_stage: usize) -> View {
    Grid::new().children((Rectangle::new(),))
}
fn construct_ellipse(_stage: usize) -> View {
    Grid::new().children((Ellipse::new(),))
}
fn construct_line(_stage: usize) -> View {
    Grid::new().children((Line::new(),))
}
fn construct_symbol_icon(_stage: usize) -> View {
    Grid::new().children((SymbolIcon::new(),))
}
fn construct_image_icon(_stage: usize) -> View {
    Grid::new().children((ImageIcon::new(),))
}
fn construct_font_icon(_stage: usize) -> View {
    Grid::new().children((FontIcon::new(),))
}
fn construct_bitmap_icon(_stage: usize) -> View {
    Grid::new().children((BitmapIcon::new(),))
}
fn construct_path_icon(_stage: usize) -> View {
    Grid::new().children((PathIcon::new(),))
}
fn construct_list_box_item(_stage: usize) -> View {
    Grid::new().children((ListBoxItem::new(),))
}
fn construct_rating_control(_stage: usize) -> View {
    Grid::new().children((RatingControl::new(),))
}
fn construct_expander(_stage: usize) -> View {
    Grid::new().children((Expander::new(),))
}
fn construct_combo_box(_stage: usize) -> View {
    Grid::new().children((ComboBox::new(),))
}
fn construct_pivot(_stage: usize) -> View {
    Grid::new().children((Pivot::new(),))
}
fn construct_pivot_item(_stage: usize) -> View {
    Grid::new().children((PivotItem::new(),))
}
fn construct_flip_view(_stage: usize) -> View {
    Grid::new().children((FlipView::new(),))
}
fn construct_selector_bar(_stage: usize) -> View {
    Grid::new().children((SelectorBar::new(),))
}
fn construct_selector_bar_item(_stage: usize) -> View {
    Grid::new().children((SelectorBarItem::new(),))
}
fn construct_tab_view(_stage: usize) -> View {
    Grid::new().children((TabView::new(),))
}
fn construct_tab_view_item(_stage: usize) -> View {
    Grid::new().children((TabViewItem::new(),))
}
fn construct_teaching_tip(_stage: usize) -> View {
    Grid::new().children((TeachingTip::new(),))
}
fn construct_drop_down_button(_stage: usize) -> View {
    Grid::new().children((DropDownButton::new(),))
}
fn construct_command_bar(_stage: usize) -> View {
    Grid::new().children((CommandBar::new(),))
}
fn construct_app_bar_button(_stage: usize) -> View {
    Grid::new().children((AppBarButton::new(),))
}
fn construct_app_bar_separator(_stage: usize) -> View {
    Grid::new().children((AppBarSeparator::new(),))
}
fn construct_menu_bar(_stage: usize) -> View {
    Grid::new().children((MenuBar::new(),))
}
fn construct_menu_bar_item(_stage: usize) -> View {
    Grid::new().children((MenuBarItem::new(),))
}
fn construct_split_button(_stage: usize) -> View {
    Grid::new().children((SplitButton::new(),))
}
fn construct_color_picker(_stage: usize) -> View {
    Grid::new().children((ColorPicker::new(),))
}
fn construct_date_picker(_stage: usize) -> View {
    Grid::new().children((DatePicker::new(),))
}
fn construct_time_picker(_stage: usize) -> View {
    Grid::new().children((TimePicker::new(),))
}
fn construct_calendar_date_picker(_stage: usize) -> View {
    Grid::new().children((CalendarDatePicker::new(),))
}
fn construct_tool_tip(_stage: usize) -> View {
    TextBlock::new()
        .text("tooltip target")
        .tooltip_with(Tooltip::rich(TextBlock::new().text("tooltip content")))
}
fn construct_content_dialog(_stage: usize) -> View {
    Grid::new().children((ContentDialog::new(),))
}
fn construct_calendar_view(_stage: usize) -> View {
    Grid::new().children((CalendarView::new(),))
}
fn construct_list_view(_stage: usize) -> View {
    Grid::new().children((ListView::new(),))
}
fn construct_list_view_item(_stage: usize) -> View {
    Grid::new().children((ListViewItem::new(),))
}
fn construct_tree_view(_stage: usize) -> View {
    Grid::new().children((TreeView::new(),))
}
fn construct_grid_view(_stage: usize) -> View {
    Grid::new().children((GridView::new(),))
}
fn construct_grid_view_item(_stage: usize) -> View {
    Grid::new().children((GridViewItem::new(),))
}
fn construct_relative_panel(_stage: usize) -> View {
    Grid::new().children((RelativePanel::new(),))
}
fn construct_canvas(_stage: usize) -> View {
    Grid::new().children((Canvas::new(),))
}
fn construct_rich_edit_box(_stage: usize) -> View {
    Grid::new().children((RichEditBox::new(),))
}
fn construct_rich_text_block(_stage: usize) -> View {
    Grid::new().children((RichTextBlock::new(),))
}
fn construct_viewbox(_stage: usize) -> View {
    Grid::new().children((Viewbox::new(),))
}
fn construct_web_view2(_stage: usize) -> View {
    Grid::new().children((WebView2::new(),))
}
fn construct_swap_chain_panel(_stage: usize) -> View {
    Grid::new().children((SwapChainPanel::new(),))
}
fn property_text_block_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().text("surface a"),)),
        2 => Grid::new().children((TextBlock::new().text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_text_block_text_wrapping(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().text_wrapping(TextWrapping::NoWrap),)),
        2 => Grid::new().children((TextBlock::new().text_wrapping(TextWrapping::Wrap),)),
        _ => unreachable!(),
    }
}
fn property_text_block_font_size(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().font_size(8.0),)),
        2 => Grid::new().children((TextBlock::new().font_size(16.0),)),
        _ => unreachable!(),
    }
}
fn property_text_block_font_weight(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().font_weight(FontWeight::NORMAL),)),
        2 => Grid::new().children((TextBlock::new().font_weight(FontWeight::BOLD),)),
        _ => unreachable!(),
    }
}
fn property_text_block_is_text_selection_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().is_text_selection_enabled(true),)),
        2 => Grid::new().children((TextBlock::new().is_text_selection_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_text_block_max_lines(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().max_lines(1),)),
        2 => Grid::new().children((TextBlock::new().max_lines(2),)),
        _ => unreachable!(),
    }
}
fn property_text_block_text_trimming(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().text_trimming(TextTrimming::None),)),
        2 => {
            Grid::new().children((TextBlock::new().text_trimming(TextTrimming::CharacterEllipsis),))
        }
        _ => unreachable!(),
    }
}
fn property_text_block_foreground(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().foreground(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((TextBlock::new().foreground(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new().children((Button::new().is_enabled(true),)),
        2 => Grid::new().children((Button::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_button_horizontal_content_alignment(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new()
            .children((Button::new().horizontal_content_alignment(HorizontalAlignment::Left),)),
        2 => Grid::new()
            .children((Button::new().horizontal_content_alignment(HorizontalAlignment::Right),)),
        _ => unreachable!(),
    }
}
fn property_button_vertical_content_alignment(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new()
            .children((Button::new().vertical_content_alignment(VerticalAlignment::Top),)),
        2 => Grid::new()
            .children((Button::new().vertical_content_alignment(VerticalAlignment::Bottom),)),
        _ => unreachable!(),
    }
}
fn property_button_resource_overrides(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new().children((Button::new().resource_overrides(
            ResourceOverrides::new().set("ButtonBackground", Color::rgb(32, 64, 96)),
        ),)),
        2 => Grid::new().children((Button::new().resource_overrides(
            ResourceOverrides::new().set("ButtonForeground", Color::rgb(96, 64, 32)),
        ),)),
        _ => unreachable!(),
    }
}
fn property_button_style(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new().children((Button::new().style(ButtonStyle::Default),)),
        2 => Grid::new().children((Button::new().style(ButtonStyle::Accent),)),
        _ => unreachable!(),
    }
}
fn property_button_key_accelerators(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new().children((Button::new().key_accelerators(KeyAccelerators::new([
            KeyAccelerator::new(AcceleratorKey::R, AcceleratorModifiers::Control, || {}),
        ])),)),
        2 => Grid::new().children((Button::new().key_accelerators(KeyAccelerators::new([
            KeyAccelerator::new(AcceleratorKey::Enter, AcceleratorModifiers::None, || {}),
        ])),)),
        _ => unreachable!(),
    }
}
fn property_hyperlink_button_navigate_uri(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((HyperlinkButton::new(),)),
        1 => Grid::new().children((HyperlinkButton::new()
            .navigate_uri("https://example.com/a")
            .unwrap(),)),
        2 => Grid::new().children((HyperlinkButton::new()
            .navigate_uri("https://example.com/b")
            .unwrap(),)),
        _ => unreachable!(),
    }
}
fn property_hyperlink_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((HyperlinkButton::new(),)),
        1 => Grid::new().children((HyperlinkButton::new().is_enabled(true),)),
        2 => Grid::new().children((HyperlinkButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_repeat_button_delay(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RepeatButton::new(),)),
        1 => Grid::new().children((RepeatButton::new().delay(1),)),
        2 => Grid::new().children((RepeatButton::new().delay(2),)),
        _ => unreachable!(),
    }
}
fn property_repeat_button_interval(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RepeatButton::new(),)),
        1 => Grid::new().children((RepeatButton::new().interval(1),)),
        2 => Grid::new().children((RepeatButton::new().interval(2),)),
        _ => unreachable!(),
    }
}
fn property_repeat_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RepeatButton::new(),)),
        1 => Grid::new().children((RepeatButton::new().is_enabled(true),)),
        2 => Grid::new().children((RepeatButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_border_padding(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().padding(Thickness::uniform(2.0)),)),
        2 => Grid::new().children((Border::new().padding(Thickness::uniform(4.0)),)),
        _ => unreachable!(),
    }
}
fn property_border_border_thickness(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().border_thickness(Thickness::uniform(2.0)),)),
        2 => Grid::new().children((Border::new().border_thickness(Thickness::uniform(4.0)),)),
        _ => unreachable!(),
    }
}
fn property_border_corner_radius(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().corner_radius(CornerRadius::uniform(2.0)),)),
        2 => Grid::new().children((Border::new().corner_radius(CornerRadius::uniform(4.0)),)),
        _ => unreachable!(),
    }
}
fn property_border_background(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().background(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Border::new().background(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_border_border_brush(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().border_brush(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Border::new().border_brush(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_border_opacity_transition(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new()
            .children((Border::new().opacity_transition(std::time::Duration::from_millis(50)),)),
        2 => Grid::new()
            .children((Border::new().opacity_transition(std::time::Duration::from_millis(100)),)),
        _ => unreachable!(),
    }
}
fn property_border_scale(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().scale(0.0),)),
        2 => Grid::new().children((Border::new().scale(8.0),)),
        _ => unreachable!(),
    }
}
fn property_border_scale_transition(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new()
            .children((Border::new().scale_transition(std::time::Duration::from_millis(50)),)),
        2 => Grid::new()
            .children((Border::new().scale_transition(std::time::Duration::from_millis(100)),)),
        _ => unreachable!(),
    }
}
fn property_border_capture_pointer_on_press(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().capture_pointer_on_press(true),)),
        2 => Grid::new().children((Border::new().capture_pointer_on_press(false),)),
        _ => unreachable!(),
    }
}
fn property_border_drop_policy(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().drop_policy(
            DragDropPolicy::new().text(DragDropAction::new(DragDropOperation::Copy)),
        ),)),
        2 => Grid::new().children((Border::new().drop_policy(
            DragDropPolicy::new().storage_items(DragDropAction::new(DragDropOperation::Move)),
        ),)),
        _ => unreachable!(),
    }
}
fn property_breadcrumb_bar_items_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((BreadcrumbBar::new(),)),
        1 => Grid::new().children((BreadcrumbBar::new().items_source(["surface a", "surface b"]),)),
        2 => Grid::new().children((BreadcrumbBar::new().items_source(["surface c", "surface d"]),)),
        _ => unreachable!(),
    }
}
fn property_stack_panel_orientation(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((StackPanel::new(),)),
        1 => Grid::new().children((StackPanel::new().orientation(Orientation::Vertical),)),
        2 => Grid::new().children((StackPanel::new().orientation(Orientation::Horizontal),)),
        _ => unreachable!(),
    }
}
fn property_stack_panel_spacing(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((StackPanel::new(),)),
        1 => Grid::new().children((StackPanel::new().spacing(1.0),)),
        2 => Grid::new().children((StackPanel::new().spacing(2.0),)),
        _ => unreachable!(),
    }
}
fn property_variable_sized_wrap_grid_item_width(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((VariableSizedWrapGrid::new(),)),
        1 => Grid::new().children((VariableSizedWrapGrid::new().item_width(8.0),)),
        2 => Grid::new().children((VariableSizedWrapGrid::new().item_width(16.0),)),
        _ => unreachable!(),
    }
}
fn property_variable_sized_wrap_grid_item_height(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((VariableSizedWrapGrid::new(),)),
        1 => Grid::new().children((VariableSizedWrapGrid::new().item_height(8.0),)),
        2 => Grid::new().children((VariableSizedWrapGrid::new().item_height(16.0),)),
        _ => unreachable!(),
    }
}
fn property_variable_sized_wrap_grid_orientation(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((VariableSizedWrapGrid::new(),)),
        1 => {
            Grid::new().children((VariableSizedWrapGrid::new().orientation(Orientation::Vertical),))
        }
        2 => Grid::new()
            .children((VariableSizedWrapGrid::new().orientation(Orientation::Horizontal),)),
        _ => unreachable!(),
    }
}
fn property_grid_row_spacing(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Grid::new(),)),
        1 => Grid::new().children((Grid::new().row_spacing(1.0),)),
        2 => Grid::new().children((Grid::new().row_spacing(2.0),)),
        _ => unreachable!(),
    }
}
fn property_grid_column_spacing(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Grid::new(),)),
        1 => Grid::new().children((Grid::new().column_spacing(1.0),)),
        2 => Grid::new().children((Grid::new().column_spacing(2.0),)),
        _ => unreachable!(),
    }
}
fn property_grid_key_accelerators(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Grid::new(),)),
        1 => Grid::new().children((Grid::new().key_accelerators(KeyAccelerators::new([
            KeyAccelerator::new(AcceleratorKey::R, AcceleratorModifiers::Control, || {}),
        ])),)),
        2 => Grid::new().children((Grid::new().key_accelerators(KeyAccelerators::new([
            KeyAccelerator::new(AcceleratorKey::Enter, AcceleratorModifiers::None, || {}),
        ])),)),
        _ => unreachable!(),
    }
}
fn property_grid_background(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Grid::new(),)),
        1 => Grid::new().children((Grid::new().background(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Grid::new().background(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_text_box_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().text("surface a"),)),
        2 => Grid::new().children((TextBox::new().text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_text_box_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((TextBox::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_text_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().is_enabled(true),)),
        2 => Grid::new().children((TextBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_text_box_accepts_return(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().accepts_return(true),)),
        2 => Grid::new().children((TextBox::new().accepts_return(false),)),
        _ => unreachable!(),
    }
}
fn property_text_box_text_wrapping(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().text_wrapping(TextWrapping::NoWrap),)),
        2 => Grid::new().children((TextBox::new().text_wrapping(TextWrapping::Wrap),)),
        _ => unreachable!(),
    }
}
fn property_text_box_background(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().background(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((TextBox::new().background(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_text_box_border_brush(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().border_brush(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((TextBox::new().border_brush(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_text_box_border_thickness(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().border_thickness(Thickness::uniform(2.0)),)),
        2 => Grid::new().children((TextBox::new().border_thickness(Thickness::uniform(4.0)),)),
        _ => unreachable!(),
    }
}
fn property_auto_suggest_box_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => Grid::new().children((AutoSuggestBox::new().text("surface a"),)),
        2 => Grid::new().children((AutoSuggestBox::new().text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_auto_suggest_box_items_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => {
            Grid::new().children((AutoSuggestBox::new().items_source(["surface a", "surface b"]),))
        }
        2 => {
            Grid::new().children((AutoSuggestBox::new().items_source(["surface c", "surface d"]),))
        }
        _ => unreachable!(),
    }
}
fn property_auto_suggest_box_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => Grid::new().children((AutoSuggestBox::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((AutoSuggestBox::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_auto_suggest_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => Grid::new().children((AutoSuggestBox::new().is_enabled(true),)),
        2 => Grid::new().children((AutoSuggestBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_password_box_password(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PasswordBox::new(),)),
        1 => Grid::new().children((PasswordBox::new().password("surface a"),)),
        2 => Grid::new().children((PasswordBox::new().password("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_password_box_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PasswordBox::new(),)),
        1 => Grid::new().children((PasswordBox::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((PasswordBox::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_password_box_password_reveal_mode(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PasswordBox::new(),)),
        1 => Grid::new()
            .children((PasswordBox::new().password_reveal_mode(PasswordRevealMode::Peek),)),
        2 => Grid::new()
            .children((PasswordBox::new().password_reveal_mode(PasswordRevealMode::Hidden),)),
        _ => unreachable!(),
    }
}
fn property_password_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PasswordBox::new(),)),
        1 => Grid::new().children((PasswordBox::new().is_enabled(true),)),
        2 => Grid::new().children((PasswordBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_number_box_minimum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NumberBox::new(),)),
        1 => Grid::new().children((NumberBox::new().minimum(1.0),)),
        2 => Grid::new().children((NumberBox::new().minimum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_number_box_maximum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NumberBox::new(),)),
        1 => Grid::new().children((NumberBox::new().maximum(1.0),)),
        2 => Grid::new().children((NumberBox::new().maximum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_number_box_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NumberBox::new(),)),
        1 => Grid::new().children((NumberBox::new().value(Some(1.0)),)),
        2 => Grid::new().children((NumberBox::new().value(Some(2.0)),)),
        _ => unreachable!(),
    }
}
fn property_number_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NumberBox::new(),)),
        1 => Grid::new().children((NumberBox::new().is_enabled(true),)),
        2 => Grid::new().children((NumberBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_slider_minimum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().minimum(1.0),)),
        2 => Grid::new().children((Slider::new().minimum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_slider_maximum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().maximum(1.0),)),
        2 => Grid::new().children((Slider::new().maximum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_slider_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().value(1.0),)),
        2 => Grid::new().children((Slider::new().value(2.0),)),
        _ => unreachable!(),
    }
}
fn property_slider_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().is_enabled(true),)),
        2 => Grid::new().children((Slider::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_slider_step_frequency(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().step_frequency(8.0),)),
        2 => Grid::new().children((Slider::new().step_frequency(16.0),)),
        _ => unreachable!(),
    }
}
fn property_slider_orientation(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().orientation(Orientation::Vertical),)),
        2 => Grid::new().children((Slider::new().orientation(Orientation::Horizontal),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().is_enabled(true),)),
        2 => Grid::new().children((NavigationView::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_pane_display_mode(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((
            NavigationView::new().pane_display_mode(NavigationViewPaneDisplayMode::Auto),
        )),
        2 => Grid::new().children((
            NavigationView::new().pane_display_mode(NavigationViewPaneDisplayMode::Left),
        )),
        _ => unreachable!(),
    }
}
fn property_navigation_view_is_pane_toggle_button_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().is_pane_toggle_button_visible(true),)),
        2 => Grid::new().children((NavigationView::new().is_pane_toggle_button_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_is_back_button_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new()
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed),)),
        2 => Grid::new()
            .children((NavigationView::new()
                .is_back_button_visible(NavigationViewBackButtonVisible::Visible),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_is_settings_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().is_settings_visible(true),)),
        2 => Grid::new().children((NavigationView::new().is_settings_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_always_show_header(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().always_show_header(true),)),
        2 => Grid::new().children((NavigationView::new().always_show_header(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_pane_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().pane_title("surface a"),)),
        2 => Grid::new().children((NavigationView::new().pane_title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_open_pane_length(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().open_pane_length(0.0),)),
        2 => Grid::new().children((NavigationView::new().open_pane_length(8.0),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_is_pane_open(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().is_pane_open(true),)),
        2 => Grid::new().children((NavigationView::new().is_pane_open(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_item_tag(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationViewItem::new(),)),
        1 => Grid::new().children((NavigationViewItem::new().tag("surface a"),)),
        2 => Grid::new().children((NavigationViewItem::new().tag("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_item_is_selected(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationViewItem::new(),)),
        1 => Grid::new().children((NavigationViewItem::new().is_selected(true),)),
        2 => Grid::new().children((NavigationViewItem::new().is_selected(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_item_selects_on_invoked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationViewItem::new(),)),
        1 => Grid::new().children((NavigationViewItem::new().selects_on_invoked(true),)),
        2 => Grid::new().children((NavigationViewItem::new().selects_on_invoked(false),)),
        _ => unreachable!(),
    }
}
fn property_navigation_view_item_is_expanded(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationViewItem::new(),)),
        1 => Grid::new().children((NavigationViewItem::new().is_expanded(true),)),
        2 => Grid::new().children((NavigationViewItem::new().is_expanded(false),)),
        _ => unreachable!(),
    }
}
fn property_split_view_open_pane_length(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitView::new(),)),
        1 => Grid::new().children((SplitView::new().open_pane_length(1.0),)),
        2 => Grid::new().children((SplitView::new().open_pane_length(2.0),)),
        _ => unreachable!(),
    }
}
fn property_split_view_compact_pane_length(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitView::new(),)),
        1 => Grid::new().children((SplitView::new().compact_pane_length(1.0),)),
        2 => Grid::new().children((SplitView::new().compact_pane_length(2.0),)),
        _ => unreachable!(),
    }
}
fn property_split_view_display_mode(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitView::new(),)),
        1 => Grid::new().children((SplitView::new().display_mode(SplitViewDisplayMode::Overlay),)),
        2 => Grid::new().children((SplitView::new().display_mode(SplitViewDisplayMode::Inline),)),
        _ => unreachable!(),
    }
}
fn property_split_view_is_pane_open(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitView::new(),)),
        1 => Grid::new().children((SplitView::new().is_pane_open(true),)),
        2 => Grid::new().children((SplitView::new().is_pane_open(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_minimum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().minimum(1.0),)),
        2 => Grid::new().children((ProgressBar::new().minimum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_maximum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().maximum(1.0),)),
        2 => Grid::new().children((ProgressBar::new().maximum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().value(1.0),)),
        2 => Grid::new().children((ProgressBar::new().value(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_is_indeterminate(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().is_indeterminate(true),)),
        2 => Grid::new().children((ProgressBar::new().is_indeterminate(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_show_error(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().show_error(true),)),
        2 => Grid::new().children((ProgressBar::new().show_error(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_show_paused(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().show_paused(true),)),
        2 => Grid::new().children((ProgressBar::new().show_paused(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_bar_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressBar::new(),)),
        1 => Grid::new().children((ProgressBar::new().is_enabled(true),)),
        2 => Grid::new().children((ProgressBar::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_toggle_switch_is_on(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleSwitch::new(),)),
        1 => Grid::new().children((ToggleSwitch::new().is_on(true),)),
        2 => Grid::new().children((ToggleSwitch::new().is_on(false),)),
        _ => unreachable!(),
    }
}
fn property_toggle_switch_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleSwitch::new(),)),
        1 => Grid::new().children((ToggleSwitch::new().is_enabled(true),)),
        2 => Grid::new().children((ToggleSwitch::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_check_box_is_checked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CheckBox::new(),)),
        1 => Grid::new().children((CheckBox::new().is_checked(true),)),
        2 => Grid::new().children((CheckBox::new().is_checked(false),)),
        _ => unreachable!(),
    }
}
fn property_check_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CheckBox::new(),)),
        1 => Grid::new().children((CheckBox::new().is_enabled(true),)),
        2 => Grid::new().children((CheckBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_toggle_button_is_checked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleButton::new(),)),
        1 => Grid::new().children((ToggleButton::new().is_checked(true),)),
        2 => Grid::new().children((ToggleButton::new().is_checked(false),)),
        _ => unreachable!(),
    }
}
fn property_toggle_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleButton::new(),)),
        1 => Grid::new().children((ToggleButton::new().is_enabled(true),)),
        2 => Grid::new().children((ToggleButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_radio_button_group_name(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButton::new(),)),
        1 => Grid::new().children((RadioButton::new().group_name("surface a"),)),
        2 => Grid::new().children((RadioButton::new().group_name("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_radio_button_is_checked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButton::new(),)),
        1 => Grid::new().children((RadioButton::new().is_checked(true),)),
        2 => Grid::new().children((RadioButton::new().is_checked(false),)),
        _ => unreachable!(),
    }
}
fn property_radio_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButton::new(),)),
        1 => Grid::new().children((RadioButton::new().is_enabled(true),)),
        2 => Grid::new().children((RadioButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_radio_buttons_items_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButtons::new(),)),
        1 => Grid::new().children((RadioButtons::new().items_source(["surface a", "surface b"]),)),
        2 => Grid::new().children((RadioButtons::new().items_source(["surface c", "surface d"]),)),
        _ => unreachable!(),
    }
}
fn property_radio_buttons_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButtons::new(),)),
        1 => Grid::new().children((RadioButtons::new().selected_index(Some(0)),)),
        2 => Grid::new().children((RadioButtons::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_radio_buttons_max_columns(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButtons::new(),)),
        1 => Grid::new().children((RadioButtons::new().max_columns(1),)),
        2 => Grid::new().children((RadioButtons::new().max_columns(2),)),
        _ => unreachable!(),
    }
}
fn property_info_badge_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBadge::new(),)),
        1 => Grid::new().children((InfoBadge::new().value(1),)),
        2 => Grid::new().children((InfoBadge::new().value(2),)),
        _ => unreachable!(),
    }
}
fn property_info_bar_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().title("surface a"),)),
        2 => Grid::new().children((InfoBar::new().title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_info_bar_message(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().message("surface a"),)),
        2 => Grid::new().children((InfoBar::new().message("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_info_bar_severity(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().severity(InfoBarSeverity::Informational),)),
        2 => Grid::new().children((InfoBar::new().severity(InfoBarSeverity::Success),)),
        _ => unreachable!(),
    }
}
fn property_info_bar_is_open(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().is_open(true),)),
        2 => Grid::new().children((InfoBar::new().is_open(false),)),
        _ => unreachable!(),
    }
}
fn property_info_bar_is_closable(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().is_closable(true),)),
        2 => Grid::new().children((InfoBar::new().is_closable(false),)),
        _ => unreachable!(),
    }
}
fn property_person_picture_display_name(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PersonPicture::new(),)),
        1 => Grid::new().children((PersonPicture::new().display_name("surface a"),)),
        2 => Grid::new().children((PersonPicture::new().display_name("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_person_picture_initials(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PersonPicture::new(),)),
        1 => Grid::new().children((PersonPicture::new().initials("surface a"),)),
        2 => Grid::new().children((PersonPicture::new().initials("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_scroll_viewer_horizontal_scroll_bar_visibility(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ScrollViewer::new(),)),
        1 => {
            Grid::new()
                .children((ScrollViewer::new()
                    .horizontal_scroll_bar_visibility(ScrollBarVisibility::Disabled),))
        }
        2 => Grid::new().children((
            ScrollViewer::new().horizontal_scroll_bar_visibility(ScrollBarVisibility::Auto),
        )),
        _ => unreachable!(),
    }
}
fn property_scroll_viewer_vertical_scroll_bar_visibility(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ScrollViewer::new(),)),
        1 => {
            Grid::new()
                .children((ScrollViewer::new()
                    .vertical_scroll_bar_visibility(ScrollBarVisibility::Disabled),))
        }
        2 => Grid::new().children((
            ScrollViewer::new().vertical_scroll_bar_visibility(ScrollBarVisibility::Auto),
        )),
        _ => unreachable!(),
    }
}
fn property_scroll_view_horizontal_scroll_bar_visibility(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ScrollView::new(),)),
        1 => Grid::new()
            .children((ScrollView::new()
                .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Auto),)),
        2 => Grid::new().children((ScrollView::new()
            .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible),)),
        _ => unreachable!(),
    }
}
fn property_scroll_view_vertical_scroll_bar_visibility(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ScrollView::new(),)),
        1 => Grid::new()
            .children((ScrollView::new()
                .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Auto),)),
        2 => Grid::new()
            .children((ScrollView::new()
                .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible),)),
        _ => unreachable!(),
    }
}
fn property_image_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Image::new(),)),
        1 => Grid::new().children((Image::new()
            .source("ms-appx:///Files/surface-a.png")
            .unwrap(),)),
        2 => Grid::new().children((Image::new()
            .source("ms-appx:///Files/surface-b.png")
            .unwrap(),)),
        _ => unreachable!(),
    }
}
fn property_image_stretch(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Image::new(),)),
        1 => Grid::new().children((Image::new().stretch(Stretch::None),)),
        2 => Grid::new().children((Image::new().stretch(Stretch::Fill),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_minimum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().minimum(1.0),)),
        2 => Grid::new().children((ProgressRing::new().minimum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_maximum(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().maximum(1.0),)),
        2 => Grid::new().children((ProgressRing::new().maximum(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().value(1.0),)),
        2 => Grid::new().children((ProgressRing::new().value(2.0),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_is_indeterminate(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().is_indeterminate(true),)),
        2 => Grid::new().children((ProgressRing::new().is_indeterminate(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_is_active(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().is_active(true),)),
        2 => Grid::new().children((ProgressRing::new().is_active(false),)),
        _ => unreachable!(),
    }
}
fn property_progress_ring_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ProgressRing::new(),)),
        1 => Grid::new().children((ProgressRing::new().is_enabled(true),)),
        2 => Grid::new().children((ProgressRing::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_list_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListBox::new(),)),
        1 => Grid::new().children((ListBox::new().is_enabled(true),)),
        2 => Grid::new().children((ListBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_rectangle_fill(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Rectangle::new(),)),
        1 => Grid::new().children((Rectangle::new().fill(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Rectangle::new().fill(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_rectangle_stroke(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Rectangle::new(),)),
        1 => Grid::new().children((Rectangle::new().stroke(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Rectangle::new().stroke(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_rectangle_stroke_thickness(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Rectangle::new(),)),
        1 => Grid::new().children((Rectangle::new().stroke_thickness(0.0),)),
        2 => Grid::new().children((Rectangle::new().stroke_thickness(8.0),)),
        _ => unreachable!(),
    }
}
fn property_rectangle_radius_x(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Rectangle::new(),)),
        1 => Grid::new().children((Rectangle::new().radius_x(0.0),)),
        2 => Grid::new().children((Rectangle::new().radius_x(8.0),)),
        _ => unreachable!(),
    }
}
fn property_rectangle_radius_y(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Rectangle::new(),)),
        1 => Grid::new().children((Rectangle::new().radius_y(0.0),)),
        2 => Grid::new().children((Rectangle::new().radius_y(8.0),)),
        _ => unreachable!(),
    }
}
fn property_ellipse_fill(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Ellipse::new(),)),
        1 => Grid::new().children((Ellipse::new().fill(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Ellipse::new().fill(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_ellipse_stroke(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Ellipse::new(),)),
        1 => Grid::new().children((Ellipse::new().stroke(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Ellipse::new().stroke(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_ellipse_stroke_thickness(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Ellipse::new(),)),
        1 => Grid::new().children((Ellipse::new().stroke_thickness(0.0),)),
        2 => Grid::new().children((Ellipse::new().stroke_thickness(8.0),)),
        _ => unreachable!(),
    }
}
fn property_line_stroke(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().stroke(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((Line::new().stroke(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_line_stroke_thickness(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().stroke_thickness(0.0),)),
        2 => Grid::new().children((Line::new().stroke_thickness(8.0),)),
        _ => unreachable!(),
    }
}
fn property_line_x1(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().x1(1.0),)),
        2 => Grid::new().children((Line::new().x1(2.0),)),
        _ => unreachable!(),
    }
}
fn property_line_y1(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().y1(1.0),)),
        2 => Grid::new().children((Line::new().y1(2.0),)),
        _ => unreachable!(),
    }
}
fn property_line_x2(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().x2(1.0),)),
        2 => Grid::new().children((Line::new().x2(2.0),)),
        _ => unreachable!(),
    }
}
fn property_line_y2(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Line::new(),)),
        1 => Grid::new().children((Line::new().y2(1.0),)),
        2 => Grid::new().children((Line::new().y2(2.0),)),
        _ => unreachable!(),
    }
}
fn property_symbol_icon_symbol(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SymbolIcon::new(),)),
        1 => Grid::new().children((SymbolIcon::new().symbol(Symbol::Previous),)),
        2 => Grid::new().children((SymbolIcon::new().symbol(Symbol::Next),)),
        _ => unreachable!(),
    }
}
fn property_image_icon_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ImageIcon::new(),)),
        1 => Grid::new().children((ImageIcon::new()
            .source("ms-appx:///Files/surface-a.png")
            .unwrap(),)),
        2 => Grid::new().children((ImageIcon::new()
            .source("ms-appx:///Files/surface-b.png")
            .unwrap(),)),
        _ => unreachable!(),
    }
}
fn property_font_icon_glyph(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((FontIcon::new(),)),
        1 => Grid::new().children((FontIcon::new().glyph("surface a"),)),
        2 => Grid::new().children((FontIcon::new().glyph("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_bitmap_icon_uri_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((BitmapIcon::new(),)),
        1 => Grid::new().children((BitmapIcon::new()
            .uri_source("https://example.com/a")
            .unwrap(),)),
        2 => Grid::new().children((BitmapIcon::new()
            .uri_source("https://example.com/b")
            .unwrap(),)),
        _ => unreachable!(),
    }
}
fn property_bitmap_icon_show_as_monochrome(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((BitmapIcon::new(),)),
        1 => Grid::new().children((BitmapIcon::new().show_as_monochrome(true),)),
        2 => Grid::new().children((BitmapIcon::new().show_as_monochrome(false),)),
        _ => unreachable!(),
    }
}
fn property_path_icon_data(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PathIcon::new(),)),
        1 => Grid::new().children((PathIcon::new().data("M 0 0 L 10 10 Z"),)),
        2 => Grid::new().children((PathIcon::new().data("M 0 0 L 20 0 L 10 10 Z"),)),
        _ => unreachable!(),
    }
}
fn property_list_box_item_tag(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListBoxItem::new(),)),
        1 => Grid::new().children((ListBoxItem::new().tag("surface a"),)),
        2 => Grid::new().children((ListBoxItem::new().tag("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_list_box_item_is_selected(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListBoxItem::new(),)),
        1 => Grid::new().children((ListBoxItem::new().is_selected(true),)),
        2 => Grid::new().children((ListBoxItem::new().is_selected(false),)),
        _ => unreachable!(),
    }
}
fn property_rating_control_max_rating(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RatingControl::new(),)),
        1 => Grid::new().children((RatingControl::new().max_rating(1),)),
        2 => Grid::new().children((RatingControl::new().max_rating(2),)),
        _ => unreachable!(),
    }
}
fn property_rating_control_value(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RatingControl::new(),)),
        1 => Grid::new().children((RatingControl::new().value(Some(3.0)),)),
        2 => Grid::new().children((RatingControl::new().value(Some(4.0)),)),
        _ => unreachable!(),
    }
}
fn property_rating_control_caption(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RatingControl::new(),)),
        1 => Grid::new().children((RatingControl::new().caption("surface a"),)),
        2 => Grid::new().children((RatingControl::new().caption("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_rating_control_is_read_only(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RatingControl::new(),)),
        1 => Grid::new().children((RatingControl::new().is_read_only(true),)),
        2 => Grid::new().children((RatingControl::new().is_read_only(false),)),
        _ => unreachable!(),
    }
}
fn property_expander_is_expanded(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Expander::new(),)),
        1 => Grid::new().children((Expander::new().is_expanded(true),)),
        2 => Grid::new().children((Expander::new().is_expanded(false),)),
        _ => unreachable!(),
    }
}
fn property_combo_box_items_source(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().items_source(["surface a", "surface b"]),)),
        2 => Grid::new().children((ComboBox::new().items_source(["surface c", "surface d"]),)),
        _ => unreachable!(),
    }
}
fn property_combo_box_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().selected_index(Some(0)),)),
        2 => Grid::new().children((ComboBox::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_combo_box_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((ComboBox::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_combo_box_is_editable(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().is_editable(true),)),
        2 => Grid::new().children((ComboBox::new().is_editable(false),)),
        _ => unreachable!(),
    }
}
fn property_combo_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().is_enabled(true),)),
        2 => Grid::new().children((ComboBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_pivot_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Pivot::new(),)),
        1 => Grid::new().children((Pivot::new().selected_index(Some(0)),)),
        2 => Grid::new().children((Pivot::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_pivot_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Pivot::new(),)),
        1 => Grid::new().children((Pivot::new().title("surface a"),)),
        2 => Grid::new().children((Pivot::new().title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_pivot_item_header(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PivotItem::new(),)),
        1 => Grid::new().children((PivotItem::new().header("surface a"),)),
        2 => Grid::new().children((PivotItem::new().header("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_flip_view_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((FlipView::new(),)),
        1 => Grid::new().children((FlipView::new().selected_index(Some(0)),)),
        2 => Grid::new().children((FlipView::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_selector_bar_item_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SelectorBarItem::new(),)),
        1 => Grid::new().children((SelectorBarItem::new().text("surface a"),)),
        2 => Grid::new().children((SelectorBarItem::new().text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_selector_bar_item_is_selected(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SelectorBarItem::new(),)),
        1 => Grid::new().children((SelectorBarItem::new().is_selected(true),)),
        2 => Grid::new().children((SelectorBarItem::new().is_selected(false),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().selected_index(Some(0)),)),
        2 => Grid::new().children((TabView::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_can_reorder_tabs(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().can_reorder_tabs(true),)),
        2 => Grid::new().children((TabView::new().can_reorder_tabs(false),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_is_add_tab_button_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().is_add_tab_button_visible(true),)),
        2 => Grid::new().children((TabView::new().is_add_tab_button_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_item_header(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabViewItem::new(),)),
        1 => Grid::new().children((TabViewItem::new().header("surface a"),)),
        2 => Grid::new().children((TabViewItem::new().header("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_item_is_closable(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabViewItem::new(),)),
        1 => Grid::new().children((TabViewItem::new().is_closable(true),)),
        2 => Grid::new().children((TabViewItem::new().is_closable(false),)),
        _ => unreachable!(),
    }
}
fn property_tab_view_item_tag(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabViewItem::new(),)),
        1 => Grid::new().children((TabViewItem::new().tag("surface a"),)),
        2 => Grid::new().children((TabViewItem::new().tag("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().title("surface a"),)),
        2 => Grid::new().children((TeachingTip::new().title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_subtitle(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().subtitle("surface a"),)),
        2 => Grid::new().children((TeachingTip::new().subtitle("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_is_open(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().is_open(true),)),
        2 => Grid::new().children((TeachingTip::new().is_open(false),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_is_light_dismiss_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().is_light_dismiss_enabled(true),)),
        2 => Grid::new().children((TeachingTip::new().is_light_dismiss_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_preferred_placement(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new()
            .children((TeachingTip::new().preferred_placement(TeachingTipPlacementMode::Auto),)),
        2 => Grid::new()
            .children((TeachingTip::new().preferred_placement(TeachingTipPlacementMode::Top),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_action_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().action_button_content("surface a"),)),
        2 => Grid::new().children((TeachingTip::new().action_button_content("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_teaching_tip_close_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().close_button_content("surface a"),)),
        2 => Grid::new().children((TeachingTip::new().close_button_content("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_drop_down_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DropDownButton::new(),)),
        1 => Grid::new().children((DropDownButton::new().is_enabled(true),)),
        2 => Grid::new().children((DropDownButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_app_bar_button_label(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AppBarButton::new(),)),
        1 => Grid::new().children((AppBarButton::new().label("surface a"),)),
        2 => Grid::new().children((AppBarButton::new().label("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_app_bar_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AppBarButton::new(),)),
        1 => Grid::new().children((AppBarButton::new().is_enabled(true),)),
        2 => Grid::new().children((AppBarButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_menu_bar_item_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((MenuBarItem::new(),)),
        1 => Grid::new().children((MenuBarItem::new().title("surface a"),)),
        2 => Grid::new().children((MenuBarItem::new().title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_split_button_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitButton::new(),)),
        1 => Grid::new().children((SplitButton::new().is_enabled(true),)),
        2 => Grid::new().children((SplitButton::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_color(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().color(Color::rgb(32, 64, 96)),)),
        2 => Grid::new().children((ColorPicker::new().color(Color::rgb(96, 64, 32)),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_is_alpha_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().is_alpha_enabled(true),)),
        2 => Grid::new().children((ColorPicker::new().is_alpha_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_is_hex_input_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().is_hex_input_visible(true),)),
        2 => Grid::new().children((ColorPicker::new().is_hex_input_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_is_color_slider_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().is_color_slider_visible(true),)),
        2 => Grid::new().children((ColorPicker::new().is_color_slider_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_is_color_channel_text_input_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().is_color_channel_text_input_visible(true),)),
        2 => Grid::new().children((ColorPicker::new().is_color_channel_text_input_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_color_picker_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().is_enabled(true),)),
        2 => Grid::new().children((ColorPicker::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_date_picker_day_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DatePicker::new(),)),
        1 => Grid::new().children((DatePicker::new().day_visible(true),)),
        2 => Grid::new().children((DatePicker::new().day_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_date_picker_month_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DatePicker::new(),)),
        1 => Grid::new().children((DatePicker::new().month_visible(true),)),
        2 => Grid::new().children((DatePicker::new().month_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_date_picker_year_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DatePicker::new(),)),
        1 => Grid::new().children((DatePicker::new().year_visible(true),)),
        2 => Grid::new().children((DatePicker::new().year_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_date_picker_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DatePicker::new(),)),
        1 => Grid::new().children((DatePicker::new().is_enabled(true),)),
        2 => Grid::new().children((DatePicker::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_time_picker_clock_identifier(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TimePicker::new(),)),
        1 => Grid::new().children((TimePicker::new().clock_identifier("12HourClock"),)),
        2 => Grid::new().children((TimePicker::new().clock_identifier("24HourClock"),)),
        _ => unreachable!(),
    }
}
fn property_time_picker_minute_increment(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TimePicker::new(),)),
        1 => Grid::new().children((TimePicker::new().minute_increment(5),)),
        2 => Grid::new().children((TimePicker::new().minute_increment(15),)),
        _ => unreachable!(),
    }
}
fn property_time_picker_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TimePicker::new(),)),
        1 => Grid::new().children((TimePicker::new().is_enabled(true),)),
        2 => Grid::new().children((TimePicker::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_date_picker_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarDatePicker::new(),)),
        1 => Grid::new().children((CalendarDatePicker::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((CalendarDatePicker::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_calendar_date_picker_is_today_highlighted(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarDatePicker::new(),)),
        1 => Grid::new().children((CalendarDatePicker::new().is_today_highlighted(true),)),
        2 => Grid::new().children((CalendarDatePicker::new().is_today_highlighted(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_date_picker_is_calendar_open(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarDatePicker::new(),)),
        1 => Grid::new().children((CalendarDatePicker::new().is_calendar_open(true),)),
        2 => Grid::new().children((CalendarDatePicker::new().is_calendar_open(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_date_picker_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarDatePicker::new(),)),
        1 => Grid::new().children((CalendarDatePicker::new().is_enabled(true),)),
        2 => Grid::new().children((CalendarDatePicker::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().title("surface a"),)),
        2 => Grid::new().children((ContentDialog::new().title("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_primary_button_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().primary_button_text("surface a"),)),
        2 => Grid::new().children((ContentDialog::new().primary_button_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_secondary_button_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().secondary_button_text("surface a"),)),
        2 => Grid::new().children((ContentDialog::new().secondary_button_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_close_button_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().close_button_text("surface a"),)),
        2 => Grid::new().children((ContentDialog::new().close_button_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_is_primary_button_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().is_primary_button_enabled(true),)),
        2 => Grid::new().children((ContentDialog::new().is_primary_button_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_content_dialog_is_secondary_button_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().is_secondary_button_enabled(true),)),
        2 => Grid::new().children((ContentDialog::new().is_secondary_button_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_view_is_today_highlighted(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarView::new(),)),
        1 => Grid::new().children((CalendarView::new().is_today_highlighted(true),)),
        2 => Grid::new().children((CalendarView::new().is_today_highlighted(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_view_is_group_label_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarView::new(),)),
        1 => Grid::new().children((CalendarView::new().is_group_label_visible(true),)),
        2 => Grid::new().children((CalendarView::new().is_group_label_visible(false),)),
        _ => unreachable!(),
    }
}
fn property_calendar_view_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarView::new(),)),
        1 => Grid::new().children((CalendarView::new().is_enabled(true),)),
        2 => Grid::new().children((CalendarView::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_list_view_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().selected_index(Some(0)),)),
        2 => Grid::new().children((ListView::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_list_view_selection_mode(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().selection_mode(ListViewSelectionMode::None),)),
        2 => Grid::new().children((ListView::new().selection_mode(ListViewSelectionMode::Single),)),
        _ => unreachable!(),
    }
}
fn property_list_view_can_drag_items(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().can_drag_items(true),)),
        2 => Grid::new().children((ListView::new().can_drag_items(false),)),
        _ => unreachable!(),
    }
}
fn property_list_view_can_reorder_items(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().can_reorder_items(true),)),
        2 => Grid::new().children((ListView::new().can_reorder_items(false),)),
        _ => unreachable!(),
    }
}
fn property_list_view_allow_drop(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().allow_drop(true),)),
        2 => Grid::new().children((ListView::new().allow_drop(false),)),
        _ => unreachable!(),
    }
}
fn property_list_view_item_tag(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListViewItem::new(),)),
        1 => Grid::new().children((ListViewItem::new().tag("surface a"),)),
        2 => Grid::new().children((ListViewItem::new().tag("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_tree_view_selection_mode(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TreeView::new(),)),
        1 => Grid::new().children((TreeView::new().selection_mode(TreeViewSelectionMode::None),)),
        2 => Grid::new().children((TreeView::new().selection_mode(TreeViewSelectionMode::Single),)),
        _ => unreachable!(),
    }
}
fn property_grid_view_selected_index(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().selected_index(Some(0)),)),
        2 => Grid::new().children((GridView::new().selected_index(None),)),
        _ => unreachable!(),
    }
}
fn property_grid_view_can_drag_items(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().can_drag_items(true),)),
        2 => Grid::new().children((GridView::new().can_drag_items(false),)),
        _ => unreachable!(),
    }
}
fn property_grid_view_can_reorder_items(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().can_reorder_items(true),)),
        2 => Grid::new().children((GridView::new().can_reorder_items(false),)),
        _ => unreachable!(),
    }
}
fn property_grid_view_allow_drop(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().allow_drop(true),)),
        2 => Grid::new().children((GridView::new().allow_drop(false),)),
        _ => unreachable!(),
    }
}
fn property_grid_view_item_tag(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridViewItem::new(),)),
        1 => Grid::new().children((GridViewItem::new().tag("surface a"),)),
        2 => Grid::new().children((GridViewItem::new().tag("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_rich_edit_box_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichEditBox::new(),)),
        1 => Grid::new().children((RichEditBox::new().text("surface a"),)),
        2 => Grid::new().children((RichEditBox::new().text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_rich_edit_box_placeholder_text(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichEditBox::new(),)),
        1 => Grid::new().children((RichEditBox::new().placeholder_text("surface a"),)),
        2 => Grid::new().children((RichEditBox::new().placeholder_text("surface b"),)),
        _ => unreachable!(),
    }
}
fn property_rich_edit_box_is_read_only(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichEditBox::new(),)),
        1 => Grid::new().children((RichEditBox::new().is_read_only(true),)),
        2 => Grid::new().children((RichEditBox::new().is_read_only(false),)),
        _ => unreachable!(),
    }
}
fn property_rich_edit_box_is_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichEditBox::new(),)),
        1 => Grid::new().children((RichEditBox::new().is_enabled(true),)),
        2 => Grid::new().children((RichEditBox::new().is_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_rich_text_block_paragraphs(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichTextBlock::new(),)),
        1 => {
            Grid::new().children(
                (RichTextBlock::new().paragraphs(RichText::single_paragraph([
                    RichTextInline::Run(RichTextRun::plain("surface a")),
                ])),),
            )
        }
        2 => {
            Grid::new().children(
                (RichTextBlock::new().paragraphs(RichText::single_paragraph([
                    RichTextInline::Run(RichTextRun::plain("surface b")),
                ])),),
            )
        }
        _ => unreachable!(),
    }
}
fn property_rich_text_block_font_size(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichTextBlock::new(),)),
        1 => Grid::new().children((RichTextBlock::new().font_size(8.0),)),
        2 => Grid::new().children((RichTextBlock::new().font_size(16.0),)),
        _ => unreachable!(),
    }
}
fn property_rich_text_block_is_text_selection_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichTextBlock::new(),)),
        1 => Grid::new().children((RichTextBlock::new().is_text_selection_enabled(true),)),
        2 => Grid::new().children((RichTextBlock::new().is_text_selection_enabled(false),)),
        _ => unreachable!(),
    }
}
fn property_rich_text_block_text_wrapping(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichTextBlock::new(),)),
        1 => Grid::new().children((RichTextBlock::new().text_wrapping(TextWrapping::NoWrap),)),
        2 => Grid::new().children((RichTextBlock::new().text_wrapping(TextWrapping::Wrap),)),
        _ => unreachable!(),
    }
}
fn property_viewbox_stretch(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Viewbox::new(),)),
        1 => Grid::new().children((Viewbox::new().stretch(Stretch::None),)),
        2 => Grid::new().children((Viewbox::new().stretch(Stretch::Fill),)),
        _ => unreachable!(),
    }
}
fn property_title_bar_title(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().title("surface a"))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().title("surface b"))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn property_title_bar_subtitle(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().subtitle("surface a"))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().subtitle("surface b"))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn property_title_bar_is_back_button_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().is_back_button_visible(true))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().is_back_button_visible(false))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn property_title_bar_is_back_button_enabled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().is_back_button_enabled(true))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().is_back_button_enabled(false))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn property_title_bar_is_pane_toggle_button_visible(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().is_pane_toggle_button_visible(true))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().is_pane_toggle_button_visible(false))
            .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn event_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Button::new(),)),
        1 => Grid::new().children((Button::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Button::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_hyperlink_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((HyperlinkButton::new(),)),
        1 => Grid::new().children((HyperlinkButton::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((HyperlinkButton::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_repeat_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RepeatButton::new(),)),
        1 => Grid::new().children((RepeatButton::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((RepeatButton::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_drag_enter(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_drag_enter(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_drag_enter(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_drag_over(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_drag_over(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_drag_over(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_drag_leave(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_drag_leave(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_drag_leave(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_drop(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_drop(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_drop(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_pressed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_pressed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_pressed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_moved(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_moved(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_moved(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_entered(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_entered(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_entered(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_exited(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_exited(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_exited(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_released(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_released(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_released(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_capture_lost(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_capture_lost(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_capture_lost(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_border_on_pointer_canceled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Border::new(),)),
        1 => Grid::new().children((Border::new().on_pointer_canceled(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Border::new().on_pointer_canceled(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_breadcrumb_bar_on_item_clicked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((BreadcrumbBar::new(),)),
        1 => Grid::new().children((BreadcrumbBar::new().on_item_clicked(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((BreadcrumbBar::new().on_item_clicked(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_text_box_on_text_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBox::new(),)),
        1 => Grid::new().children((TextBox::new().on_text_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TextBox::new().on_text_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_auto_suggest_box_on_text_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => Grid::new().children((AutoSuggestBox::new().on_text_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((AutoSuggestBox::new().on_text_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_auto_suggest_box_on_suggestion_chosen(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AutoSuggestBox::new(),)),
        1 => Grid::new().children((AutoSuggestBox::new().on_suggestion_chosen(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((AutoSuggestBox::new().on_suggestion_chosen(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_password_box_on_password_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((PasswordBox::new(),)),
        1 => Grid::new().children((PasswordBox::new().on_password_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((PasswordBox::new().on_password_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_number_box_on_value_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NumberBox::new(),)),
        1 => Grid::new().children((NumberBox::new().on_value_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((NumberBox::new().on_value_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_slider_on_value_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Slider::new(),)),
        1 => Grid::new().children((Slider::new().on_value_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Slider::new().on_value_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_navigation_view_on_is_pane_open_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().on_is_pane_open_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((NavigationView::new().on_is_pane_open_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_navigation_view_on_display_mode_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().on_display_mode_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((NavigationView::new().on_display_mode_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_navigation_view_on_selected_tag_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((NavigationView::new(),)),
        1 => Grid::new().children((NavigationView::new().on_selected_tag_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((NavigationView::new().on_selected_tag_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_split_view_on_pane_closed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitView::new(),)),
        1 => Grid::new().children((SplitView::new().on_pane_closed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((SplitView::new().on_pane_closed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_toggle_switch_on_toggled(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleSwitch::new(),)),
        1 => Grid::new().children((ToggleSwitch::new().on_toggled(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ToggleSwitch::new().on_toggled(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_check_box_on_is_checked_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CheckBox::new(),)),
        1 => Grid::new().children((CheckBox::new().on_is_checked_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((CheckBox::new().on_is_checked_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_toggle_button_on_is_checked_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ToggleButton::new(),)),
        1 => Grid::new().children((ToggleButton::new().on_is_checked_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ToggleButton::new().on_is_checked_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_radio_button_on_checked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButton::new(),)),
        1 => Grid::new().children((RadioButton::new().on_checked(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((RadioButton::new().on_checked(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_radio_buttons_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RadioButtons::new(),)),
        1 => Grid::new().children((RadioButtons::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((RadioButtons::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_info_bar_on_closed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((InfoBar::new(),)),
        1 => Grid::new().children((InfoBar::new().on_closed(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((InfoBar::new().on_closed(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_image_on_opened(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Image::new(),)),
        1 => Grid::new().children((Image::new().on_opened(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Image::new().on_opened(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_image_on_failed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Image::new(),)),
        1 => Grid::new().children((Image::new().on_failed(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Image::new().on_failed(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_list_box_on_selected_tag_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListBox::new(),)),
        1 => Grid::new().children((ListBox::new().on_selected_tag_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ListBox::new().on_selected_tag_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_rating_control_on_value_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RatingControl::new(),)),
        1 => Grid::new().children((RatingControl::new().on_value_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((RatingControl::new().on_value_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_expander_on_is_expanded_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Expander::new(),)),
        1 => Grid::new().children((Expander::new().on_is_expanded_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Expander::new().on_is_expanded_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_combo_box_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ComboBox::new(),)),
        1 => Grid::new().children((ComboBox::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ComboBox::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_pivot_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((Pivot::new(),)),
        1 => Grid::new().children((Pivot::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((Pivot::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_flip_view_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((FlipView::new(),)),
        1 => Grid::new().children((FlipView::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((FlipView::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_selector_bar_on_selected_text_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SelectorBar::new(),)),
        1 => Grid::new().children((SelectorBar::new().on_selected_text_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((SelectorBar::new().on_selected_text_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_tab_view_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TabView::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_tab_view_on_close_requested(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().on_close_requested(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TabView::new().on_close_requested(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_tab_view_on_add_tab_button_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().on_add_tab_button_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TabView::new().on_add_tab_button_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_tab_view_on_reordered(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TabView::new(),)),
        1 => Grid::new().children((TabView::new().on_reordered(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TabView::new().on_reordered(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_teaching_tip_on_closed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().on_closed(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TeachingTip::new().on_closed(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_teaching_tip_on_action_button_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TeachingTip::new(),)),
        1 => Grid::new().children((TeachingTip::new().on_action_button_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TeachingTip::new().on_action_button_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_drop_down_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DropDownButton::new(),)),
        1 => Grid::new().children((DropDownButton::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((DropDownButton::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_app_bar_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((AppBarButton::new(),)),
        1 => Grid::new().children((AppBarButton::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((AppBarButton::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_split_button_on_click(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((SplitButton::new(),)),
        1 => Grid::new().children((SplitButton::new().on_click(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((SplitButton::new().on_click(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_color_picker_on_color_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ColorPicker::new(),)),
        1 => Grid::new().children((ColorPicker::new().on_color_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ColorPicker::new().on_color_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_date_picker_on_selected_date_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((DatePicker::new(),)),
        1 => Grid::new().children((DatePicker::new().on_selected_date_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((DatePicker::new().on_selected_date_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_time_picker_on_selected_time_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TimePicker::new(),)),
        1 => Grid::new().children((TimePicker::new().on_selected_time_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TimePicker::new().on_selected_time_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_calendar_date_picker_on_date_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarDatePicker::new(),)),
        1 => Grid::new().children((CalendarDatePicker::new().on_date_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((CalendarDatePicker::new().on_date_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_content_dialog_on_closed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new().children((ContentDialog::new().on_closed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ContentDialog::new().on_closed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_calendar_view_on_selected_dates_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((CalendarView::new(),)),
        1 => Grid::new().children((CalendarView::new().on_selected_dates_changed(move || {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((CalendarView::new().on_selected_dates_changed(move || {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_list_view_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ListView::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_list_view_on_reordered(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ListView::new(),)),
        1 => Grid::new().children((ListView::new().on_reordered(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((ListView::new().on_reordered(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_tree_view_on_item_invoked(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TreeView::new(),)),
        1 => Grid::new().children((TreeView::new().on_item_invoked(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((TreeView::new().on_item_invoked(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_grid_view_on_reordered(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().on_reordered(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((GridView::new().on_reordered(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_grid_view_on_selection_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((GridView::new(),)),
        1 => Grid::new().children((GridView::new().on_selection_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((GridView::new().on_selection_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_rich_edit_box_on_text_changed(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((RichEditBox::new(),)),
        1 => Grid::new().children((RichEditBox::new().on_text_changed(move |_| {
            let _ = 0u8;
        }),)),
        2 => Grid::new().children((RichEditBox::new().on_text_changed(move |_| {
            let _ = 1u8;
        }),)),
        _ => unreachable!(),
    }
}
fn event_title_bar_on_back_requested(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().on_back_requested(move || {
            let _ = 0u8;
        }))
        .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().on_back_requested(move || {
            let _ = 1u8;
        }))
        .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn event_title_bar_on_pane_toggle_requested(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children(((TitleBar::new()).slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children(((TitleBar::new().on_pane_toggle_requested(move || {
            let _ = 0u8;
        }))
        .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        2 => Grid::new().children(((TitleBar::new().on_pane_toggle_requested(move || {
            let _ = 1u8;
        }))
        .slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        _ => unreachable!(),
    }
}
fn capability_layout_width(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().width(40.0),)),
        2 => Grid::new().children((TextBlock::new().width(80.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_height(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().height(30.0),)),
        2 => Grid::new().children((TextBlock::new().height(60.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_min_width(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().min_width(10.0),)),
        2 => Grid::new().children((TextBlock::new().min_width(20.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_max_width(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().max_width(100.0),)),
        2 => Grid::new().children((TextBlock::new().max_width(200.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_min_height(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().min_height(10.0),)),
        2 => Grid::new().children((TextBlock::new().min_height(20.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_max_height(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().max_height(100.0),)),
        2 => Grid::new().children((TextBlock::new().max_height(200.0),)),
        _ => unreachable!(),
    }
}
fn capability_layout_opacity(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().opacity(0.25),)),
        2 => Grid::new().children((TextBlock::new().opacity(0.75),)),
        _ => unreachable!(),
    }
}
fn capability_layout_horizontal_alignment(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new()
            .children((TextBlock::new().horizontal_alignment(HorizontalAlignment::Left),)),
        2 => Grid::new()
            .children((TextBlock::new().horizontal_alignment(HorizontalAlignment::Right),)),
        _ => unreachable!(),
    }
}
fn capability_layout_vertical_alignment(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().vertical_alignment(VerticalAlignment::Top),)),
        2 => {
            Grid::new().children((TextBlock::new().vertical_alignment(VerticalAlignment::Bottom),))
        }
        _ => unreachable!(),
    }
}
fn capability_layout_margin(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().margin(Thickness::uniform(4.0)),)),
        2 => Grid::new().children((TextBlock::new().margin(Thickness::uniform(8.0)),)),
        _ => unreachable!(),
    }
}
fn capability_grid_child_row(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().grid_row(1i32),)),
        2 => Grid::new().children((TextBlock::new().grid_row(2i32),)),
        _ => unreachable!(),
    }
}
fn capability_grid_child_column(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().grid_column(1i32),)),
        2 => Grid::new().children((TextBlock::new().grid_column(2i32),)),
        _ => unreachable!(),
    }
}
fn capability_grid_child_row_span(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().grid_row_span(2i32),)),
        2 => Grid::new().children((TextBlock::new().grid_row_span(3i32),)),
        _ => unreachable!(),
    }
}
fn capability_grid_child_column_span(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().grid_column_span(2i32),)),
        2 => Grid::new().children((TextBlock::new().grid_column_span(3i32),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_left(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_left(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_left(),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_top(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_top(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_top(),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_right(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_right(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_right(),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_bottom(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_bottom(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_bottom(),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_horizontal_center(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_horizontal_center(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_horizontal_center(),)),
        _ => unreachable!(),
    }
}
fn capability_relative_panel_child_align_vertical_center(stage: usize) -> View {
    match stage {
        0 | 3 => RelativePanel::new().children((TextBlock::new(),)),
        1 => RelativePanel::new().children((TextBlock::new().relative_align_vertical_center(),)),
        2 => RelativePanel::new().children((TextBlock::new().relative_align_vertical_center(),)),
        _ => unreachable!(),
    }
}
fn capability_canvas_child_left(stage: usize) -> View {
    match stage {
        0 | 3 => Canvas::new().children((TextBlock::new(),)),
        1 => Canvas::new().children((TextBlock::new().canvas_left(10.0),)),
        2 => Canvas::new().children((TextBlock::new().canvas_left(20.0),)),
        _ => unreachable!(),
    }
}
fn capability_canvas_child_top(stage: usize) -> View {
    match stage {
        0 | 3 => Canvas::new().children((TextBlock::new(),)),
        1 => Canvas::new().children((TextBlock::new().canvas_top(10.0),)),
        2 => Canvas::new().children((TextBlock::new().canvas_top(20.0),)),
        _ => unreachable!(),
    }
}
fn capability_automation_name(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().automation_name("surface a"),)),
        2 => Grid::new().children((TextBlock::new().automation_name("surface b"),)),
        _ => unreachable!(),
    }
}
fn capability_automation_id(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new().children((TextBlock::new().automation_id("surface-a"),)),
        2 => Grid::new().children((TextBlock::new().automation_id("surface-b"),)),
        _ => unreachable!(),
    }
}
fn capability_automation_heading_level(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new()
            .children((TextBlock::new().automation_heading_level(AutomationHeadingLevel::Level1),)),
        2 => Grid::new()
            .children((TextBlock::new().automation_heading_level(AutomationHeadingLevel::Level2),)),
        _ => unreachable!(),
    }
}
fn capability_grid_definitions_rows(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new()
            .rows([GridLength::Pixel(20.0)])
            .children((TextBlock::new(),)),
        2 => Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((TextBlock::new(),)),
        _ => unreachable!(),
    }
}
fn capability_grid_definitions_columns(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((TextBlock::new(),)),
        1 => Grid::new()
            .columns([GridLength::Pixel(20.0)])
            .children((TextBlock::new(),)),
        2 => Grid::new()
            .columns([GridLength::Auto, GridLength::STAR])
            .children((TextBlock::new(),)),
        _ => unreachable!(),
    }
}
fn structural_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (Button::new()).into(),
        1 => (Button::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (Button::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_hyperlink_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (HyperlinkButton::new()).into(),
        1 => (HyperlinkButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (HyperlinkButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_repeat_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (RepeatButton::new()).into(),
        1 => (RepeatButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (RepeatButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_border_content(stage: usize) -> View {
    match stage {
        0 | 3 => (Border::new()).into(),
        1 => (Border::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (Border::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_stack_panel_children(stage: usize) -> View {
    match stage {
        0 | 3 => (StackPanel::new()).into(),
        1 => (StackPanel::new().children((TextBlock::new().text("surface a"),))).into(),
        2 => (StackPanel::new().children((
            TextBlock::new().text("surface a"),
            TextBlock::new().text("surface b"),
        )))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_variable_sized_wrap_grid_children(stage: usize) -> View {
    match stage {
        0 | 3 => (VariableSizedWrapGrid::new()).into(),
        1 => (VariableSizedWrapGrid::new().children((TextBlock::new().text("surface a"),))).into(),
        2 => (VariableSizedWrapGrid::new().children((
            TextBlock::new().text("surface a"),
            TextBlock::new().text("surface b"),
        )))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_grid_children(stage: usize) -> View {
    match stage {
        0 | 3 => (Grid::new()).into(),
        1 => (Grid::new().children((TextBlock::new().text("surface a"),))).into(),
        2 => (Grid::new().children((
            TextBlock::new().text("surface a"),
            TextBlock::new().text("surface b"),
        )))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_text_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (TextBox::new().slots(std::iter::empty::<SlotView<TextBoxSlot>>())).into(),
        1 => (TextBox::new().slot(TextBoxSlot::Header, TextBlock::new().text("surface a"))).into(),
        2 => (TextBox::new().slot(TextBoxSlot::Header, TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_auto_suggest_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (AutoSuggestBox::new().slots(std::iter::empty::<SlotView<AutoSuggestBoxSlot>>())).into()
        }
        1 => (AutoSuggestBox::new().slot(
            AutoSuggestBoxSlot::Header,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (AutoSuggestBox::new().slot(
            AutoSuggestBoxSlot::Header,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_password_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (PasswordBox::new().slots(std::iter::empty::<SlotView<PasswordBoxSlot>>())).into(),
        1 => (PasswordBox::new().slot(PasswordBoxSlot::Header, TextBlock::new().text("surface a")))
            .into(),
        2 => (PasswordBox::new().slot(PasswordBoxSlot::Header, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_number_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (NumberBox::new().slots(std::iter::empty::<SlotView<NumberBoxSlot>>())).into(),
        1 => (NumberBox::new().slot(NumberBoxSlot::Header, TextBlock::new().text("surface a")))
            .into(),
        2 => (NumberBox::new().slot(NumberBoxSlot::Header, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_slider_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (Slider::new().slots(std::iter::empty::<SlotView<SliderSlot>>())).into(),
        1 => (Slider::new().slot(SliderSlot::Header, TextBlock::new().text("surface a"))).into(),
        2 => (Slider::new().slot(SliderSlot::Header, TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_slot_content(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (NavigationView::new().slots(std::iter::empty::<SlotView<NavigationViewSlot>>())).into()
        }
        1 => (NavigationView::new().slot(
            NavigationViewSlot::Content,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (NavigationView::new().slot(
            NavigationViewSlot::Content,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (NavigationView::new().slots(std::iter::empty::<SlotView<NavigationViewSlot>>())).into()
        }
        1 => (NavigationView::new().slot(
            NavigationViewSlot::Header,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (NavigationView::new().slot(
            NavigationViewSlot::Header,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_slot_pane_custom_content(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (NavigationView::new().slots(std::iter::empty::<SlotView<NavigationViewSlot>>())).into()
        }
        1 => (NavigationView::new().slot(
            NavigationViewSlot::PaneCustomContent,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (NavigationView::new().slot(
            NavigationViewSlot::PaneCustomContent,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_slot_pane_footer(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (NavigationView::new().slots(std::iter::empty::<SlotView<NavigationViewSlot>>())).into()
        }
        1 => (NavigationView::new().slot(
            NavigationViewSlot::PaneFooter,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (NavigationView::new().slot(
            NavigationViewSlot::PaneFooter,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_slot_menu_items(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (NavigationView::new().slots(std::iter::empty::<SlotView<NavigationViewSlot>>())).into()
        }
        1 => (NavigationView::new().collection_slot(
            NavigationViewSlot::MenuItems,
            [KeyedView::new(
                "surface",
                NavigationViewItem::new().width(40.0),
            )],
        ))
        .into(),
        2 => (NavigationView::new().collection_slot(
            NavigationViewSlot::MenuItems,
            [KeyedView::new(
                "surface",
                NavigationViewItem::new().width(80.0),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_item_slot_content(stage: usize) -> View {
    match stage {
        0 | 3 => (NavigationViewItem::new()
            .slots(std::iter::empty::<SlotView<NavigationViewItemSlot>>()))
        .into(),
        1 => (NavigationViewItem::new().slot(
            NavigationViewItemSlot::Content,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (NavigationViewItem::new().slot(
            NavigationViewItemSlot::Content,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_item_slot_icon(stage: usize) -> View {
    match stage {
        0 | 3 => (NavigationViewItem::new()
            .slots(std::iter::empty::<SlotView<NavigationViewItemSlot>>()))
        .into(),
        1 => (NavigationViewItem::new().slot(
            NavigationViewItemSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Add),
        ))
        .into(),
        2 => (NavigationViewItem::new().slot(
            NavigationViewItemSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Accept),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_navigation_view_item_slot_menu_items(stage: usize) -> View {
    match stage {
        0 | 3 => (NavigationViewItem::new()
            .slots(std::iter::empty::<SlotView<NavigationViewItemSlot>>()))
        .into(),
        1 => (NavigationViewItem::new().collection_slot(
            NavigationViewItemSlot::MenuItems,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface a"),
            )],
        ))
        .into(),
        2 => (NavigationViewItem::new().collection_slot(
            NavigationViewItemSlot::MenuItems,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface b"),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_split_view_slot_pane(stage: usize) -> View {
    match stage {
        0 | 3 => (SplitView::new().slots(std::iter::empty::<SlotView<SplitViewSlot>>())).into(),
        1 => {
            (SplitView::new().slot(SplitViewSlot::Pane, TextBlock::new().text("surface a"))).into()
        }
        2 => {
            (SplitView::new().slot(SplitViewSlot::Pane, TextBlock::new().text("surface b"))).into()
        }
        _ => unreachable!(),
    }
}
fn structural_split_view_slot_content(stage: usize) -> View {
    match stage {
        0 | 3 => (SplitView::new().slots(std::iter::empty::<SlotView<SplitViewSlot>>())).into(),
        1 => (SplitView::new().slot(SplitViewSlot::Content, TextBlock::new().text("surface a")))
            .into(),
        2 => (SplitView::new().slot(SplitViewSlot::Content, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_toggle_switch_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (ToggleSwitch::new().slots(std::iter::empty::<SlotView<ToggleSwitchSlot>>())).into()
        }
        1 => (ToggleSwitch::new()
            .slot(ToggleSwitchSlot::Header, TextBlock::new().text("surface a")))
        .into(),
        2 => (ToggleSwitch::new()
            .slot(ToggleSwitchSlot::Header, TextBlock::new().text("surface b")))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_toggle_switch_slot_on_content(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (ToggleSwitch::new().slots(std::iter::empty::<SlotView<ToggleSwitchSlot>>())).into()
        }
        1 => (ToggleSwitch::new().slot(
            ToggleSwitchSlot::OnContent,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (ToggleSwitch::new().slot(
            ToggleSwitchSlot::OnContent,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_toggle_switch_slot_off_content(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (ToggleSwitch::new().slots(std::iter::empty::<SlotView<ToggleSwitchSlot>>())).into()
        }
        1 => (ToggleSwitch::new().slot(
            ToggleSwitchSlot::OffContent,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (ToggleSwitch::new().slot(
            ToggleSwitchSlot::OffContent,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_check_box_content(stage: usize) -> View {
    match stage {
        0 | 3 => (CheckBox::new()).into(),
        1 => (CheckBox::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (CheckBox::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_toggle_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (ToggleButton::new()).into(),
        1 => (ToggleButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (ToggleButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_radio_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (RadioButton::new()).into(),
        1 => (RadioButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (RadioButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_radio_buttons_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (RadioButtons::new().slots(std::iter::empty::<SlotView<RadioButtonsSlot>>())).into()
        }
        1 => (RadioButtons::new()
            .slot(RadioButtonsSlot::Header, TextBlock::new().text("surface a")))
        .into(),
        2 => (RadioButtons::new()
            .slot(RadioButtonsSlot::Header, TextBlock::new().text("surface b")))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_items_repeater_items(stage: usize) -> View {
    match stage {
        0 | 3 => (ItemsRepeater::new()).into(),
        1 => (ItemsRepeater::new().item("a", TextBlock::new().text("surface a"))).into(),
        2 => (ItemsRepeater::new()
            .item("a", TextBlock::new().text("surface a"))
            .item("b", TextBlock::new().text("surface b")))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_scroll_viewer_content(stage: usize) -> View {
    match stage {
        0 | 3 => (ScrollViewer::new()).into(),
        1 => (ScrollViewer::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (ScrollViewer::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_scroll_view_content(stage: usize) -> View {
    match stage {
        0 | 3 => (ScrollView::new()).into(),
        1 => (ScrollView::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (ScrollView::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_list_box_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (ListBox::new().slots(std::iter::empty::<SlotView<ListBoxSlot>>())).into(),
        1 => (ListBox::new().collection_slot(
            ListBoxSlot::Items,
            [KeyedView::new("surface", ListBoxItem::new().width(40.0))],
        ))
        .into(),
        2 => (ListBox::new().collection_slot(
            ListBoxSlot::Items,
            [KeyedView::new("surface", ListBoxItem::new().width(80.0))],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_list_box_item_content(stage: usize) -> View {
    match stage {
        0 | 3 => (ListBoxItem::new()).into(),
        1 => (ListBoxItem::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (ListBoxItem::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_expander_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (Expander::new().slots(std::iter::empty::<SlotView<ExpanderSlot>>())).into(),
        1 => {
            (Expander::new().slot(ExpanderSlot::Header, TextBlock::new().text("surface a"))).into()
        }
        2 => {
            (Expander::new().slot(ExpanderSlot::Header, TextBlock::new().text("surface b"))).into()
        }
        _ => unreachable!(),
    }
}
fn structural_expander_slot_content(stage: usize) -> View {
    match stage {
        0 | 3 => (Expander::new().slots(std::iter::empty::<SlotView<ExpanderSlot>>())).into(),
        1 => {
            (Expander::new().slot(ExpanderSlot::Content, TextBlock::new().text("surface a"))).into()
        }
        2 => {
            (Expander::new().slot(ExpanderSlot::Content, TextBlock::new().text("surface b"))).into()
        }
        _ => unreachable!(),
    }
}
fn structural_combo_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (ComboBox::new().slots(std::iter::empty::<SlotView<ComboBoxSlot>>())).into(),
        1 => {
            (ComboBox::new().slot(ComboBoxSlot::Header, TextBlock::new().text("surface a"))).into()
        }
        2 => {
            (ComboBox::new().slot(ComboBoxSlot::Header, TextBlock::new().text("surface b"))).into()
        }
        _ => unreachable!(),
    }
}
fn structural_pivot_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (Pivot::new().slots(std::iter::empty::<SlotView<PivotSlot>>())).into(),
        1 => (Pivot::new().collection_slot(
            PivotSlot::Items,
            [KeyedView::new("surface", PivotItem::new().width(40.0))],
        ))
        .into(),
        2 => (Pivot::new().collection_slot(
            PivotSlot::Items,
            [KeyedView::new("surface", PivotItem::new().width(80.0))],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_pivot_item_content(stage: usize) -> View {
    match stage {
        0 | 3 => (PivotItem::new()).into(),
        1 => (PivotItem::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (PivotItem::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_flip_view_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (FlipView::new().slots(std::iter::empty::<SlotView<FlipViewSlot>>())).into(),
        1 => (FlipView::new().collection_slot(
            FlipViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface a"),
            )],
        ))
        .into(),
        2 => (FlipView::new().collection_slot(
            FlipViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface b"),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_selector_bar_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (SelectorBar::new().slots(std::iter::empty::<SlotView<SelectorBarSlot>>())).into(),
        1 => (SelectorBar::new().collection_slot(
            SelectorBarSlot::Items,
            [KeyedView::new(
                "surface",
                SelectorBarItem::new().width(40.0),
            )],
        ))
        .into(),
        2 => (SelectorBar::new().collection_slot(
            SelectorBarSlot::Items,
            [KeyedView::new(
                "surface",
                SelectorBarItem::new().width(80.0),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_selector_bar_item_slot_icon(stage: usize) -> View {
    match stage {
        0 | 3 => (SelectorBarItem::new()
            .slots(std::iter::empty::<SlotView<SelectorBarItemSlot>>()))
        .into(),
        1 => (SelectorBarItem::new().slot(
            SelectorBarItemSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Add),
        ))
        .into(),
        2 => (SelectorBarItem::new().slot(
            SelectorBarItemSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Accept),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_tab_view_slot_tab_items(stage: usize) -> View {
    match stage {
        0 | 3 => (TabView::new().slots(std::iter::empty::<SlotView<TabViewSlot>>())).into(),
        1 => (TabView::new().collection_slot(
            TabViewSlot::TabItems,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface a"),
            )],
        ))
        .into(),
        2 => (TabView::new().collection_slot(
            TabViewSlot::TabItems,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface b"),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_tab_view_item_content(stage: usize) -> View {
    match stage {
        0 | 3 => (TabViewItem::new()).into(),
        1 => (TabViewItem::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (TabViewItem::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_drop_down_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (DropDownButton::new()).into(),
        1 => (DropDownButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (DropDownButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_command_bar_slot_primary_commands(stage: usize) -> View {
    match stage {
        0 | 3 => (CommandBar::new().slots(std::iter::empty::<SlotView<CommandBarSlot>>())).into(),
        1 => (CommandBar::new().collection_slot(
            CommandBarSlot::PrimaryCommands,
            [KeyedView::new("surface", AppBarButton::new().width(40.0))],
        ))
        .into(),
        2 => (CommandBar::new().collection_slot(
            CommandBarSlot::PrimaryCommands,
            [KeyedView::new("surface", AppBarButton::new().width(80.0))],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_command_bar_slot_secondary_commands(stage: usize) -> View {
    match stage {
        0 | 3 => (CommandBar::new().slots(std::iter::empty::<SlotView<CommandBarSlot>>())).into(),
        1 => (CommandBar::new().collection_slot(
            CommandBarSlot::SecondaryCommands,
            [KeyedView::new("surface", AppBarButton::new().width(40.0))],
        ))
        .into(),
        2 => (CommandBar::new().collection_slot(
            CommandBarSlot::SecondaryCommands,
            [KeyedView::new("surface", AppBarButton::new().width(80.0))],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_app_bar_button_slot_icon(stage: usize) -> View {
    match stage {
        0 | 3 => {
            (AppBarButton::new().slots(std::iter::empty::<SlotView<AppBarButtonSlot>>())).into()
        }
        1 => (AppBarButton::new().slot(
            AppBarButtonSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Add),
        ))
        .into(),
        2 => (AppBarButton::new().slot(
            AppBarButtonSlot::Icon,
            SymbolIcon::new().symbol(Symbol::Accept),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_menu_bar_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (MenuBar::new().slots(std::iter::empty::<SlotView<MenuBarSlot>>())).into(),
        1 => (MenuBar::new().collection_slot(
            MenuBarSlot::Items,
            [KeyedView::new("surface", MenuBarItem::new().width(40.0))],
        ))
        .into(),
        2 => (MenuBar::new().collection_slot(
            MenuBarSlot::Items,
            [KeyedView::new("surface", MenuBarItem::new().width(80.0))],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_split_button_content(stage: usize) -> View {
    match stage {
        0 | 3 => (SplitButton::new()).into(),
        1 => (SplitButton::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (SplitButton::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_date_picker_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (DatePicker::new().slots(std::iter::empty::<SlotView<DatePickerSlot>>())).into(),
        1 => (DatePicker::new().slot(DatePickerSlot::Header, TextBlock::new().text("surface a")))
            .into(),
        2 => (DatePicker::new().slot(DatePickerSlot::Header, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_time_picker_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (TimePicker::new().slots(std::iter::empty::<SlotView<TimePickerSlot>>())).into(),
        1 => (TimePicker::new().slot(TimePickerSlot::Header, TextBlock::new().text("surface a")))
            .into(),
        2 => (TimePicker::new().slot(TimePickerSlot::Header, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_calendar_date_picker_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (CalendarDatePicker::new()
            .slots(std::iter::empty::<SlotView<CalendarDatePickerSlot>>()))
        .into(),
        1 => (CalendarDatePicker::new().slot(
            CalendarDatePickerSlot::Header,
            TextBlock::new().text("surface a"),
        ))
        .into(),
        2 => (CalendarDatePicker::new().slot(
            CalendarDatePickerSlot::Header,
            TextBlock::new().text("surface b"),
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_content_dialog_content(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new().children((ContentDialog::new(),)),
        1 => Grid::new()
            .children((ContentDialog::new().content(TextBlock::new().text("surface a")),)),
        2 => Grid::new()
            .children((ContentDialog::new().content(TextBlock::new().text("surface b")),)),
        _ => unreachable!(),
    }
}
fn structural_list_view_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (ListView::new().slots(std::iter::empty::<SlotView<ListViewSlot>>())).into(),
        1 => (ListView::new().collection_slot(
            ListViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface a"),
            )],
        ))
        .into(),
        2 => (ListView::new().collection_slot(
            ListViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface b"),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_list_view_item_content(stage: usize) -> View {
    match stage {
        0 | 3 => (ListViewItem::new()).into(),
        1 => (ListViewItem::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (ListViewItem::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_grid_view_slot_items(stage: usize) -> View {
    match stage {
        0 | 3 => (GridView::new().slots(std::iter::empty::<SlotView<GridViewSlot>>())).into(),
        1 => (GridView::new().collection_slot(
            GridViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface a"),
            )],
        ))
        .into(),
        2 => (GridView::new().collection_slot(
            GridViewSlot::Items,
            [KeyedView::new(
                "surface",
                TextBlock::new().text("surface b"),
            )],
        ))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_grid_view_item_content(stage: usize) -> View {
    match stage {
        0 | 3 => (GridViewItem::new()).into(),
        1 => (GridViewItem::new().content(TextBlock::new().text("surface a"))).into(),
        2 => (GridViewItem::new().content(TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_relative_panel_children(stage: usize) -> View {
    match stage {
        0 | 3 => (RelativePanel::new()).into(),
        1 => (RelativePanel::new().children((TextBlock::new().text("surface a"),))).into(),
        2 => (RelativePanel::new().children((
            TextBlock::new().text("surface a"),
            TextBlock::new().text("surface b"),
        )))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_canvas_children(stage: usize) -> View {
    match stage {
        0 | 3 => (Canvas::new()).into(),
        1 => (Canvas::new().children((TextBlock::new().text("surface a"),))).into(),
        2 => (Canvas::new().children((
            TextBlock::new().text("surface a"),
            TextBlock::new().text("surface b"),
        )))
        .into(),
        _ => unreachable!(),
    }
}
fn structural_rich_edit_box_slot_header(stage: usize) -> View {
    match stage {
        0 | 3 => (RichEditBox::new().slots(std::iter::empty::<SlotView<RichEditBoxSlot>>())).into(),
        1 => (RichEditBox::new().slot(RichEditBoxSlot::Header, TextBlock::new().text("surface a")))
            .into(),
        2 => (RichEditBox::new().slot(RichEditBoxSlot::Header, TextBlock::new().text("surface b")))
            .into(),
        _ => unreachable!(),
    }
}
fn structural_viewbox_slot_child(stage: usize) -> View {
    match stage {
        0 | 3 => (Viewbox::new().slots(std::iter::empty::<SlotView<ViewboxSlot>>())).into(),
        1 => (Viewbox::new().slot(ViewboxSlot::Child, TextBlock::new().text("surface a"))).into(),
        2 => (Viewbox::new().slot(ViewboxSlot::Child, TextBlock::new().text("surface b"))).into(),
        _ => unreachable!(),
    }
}
fn structural_title_bar_slot_content(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children((TitleBar::new().slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children((
            TitleBar::new().slot(TitleBarSlot::Content, TextBlock::new().text("surface a")),
        )),
        2 => Grid::new().children((
            TitleBar::new().slot(TitleBarSlot::Content, TextBlock::new().text("surface b")),
        )),
        _ => unreachable!(),
    }
}
fn structural_title_bar_slot_right_header(stage: usize) -> View {
    match stage {
        0 | 3 => Grid::new()
            .children((TitleBar::new().slots(std::iter::empty::<SlotView<TitleBarSlot>>()),)),
        1 => Grid::new().children((TitleBar::new().slot(
            TitleBarSlot::RightHeader,
            TextBlock::new().text("surface a"),
        ),)),
        2 => Grid::new().children((TitleBar::new().slot(
            TitleBarSlot::RightHeader,
            TextBlock::new().text("surface b"),
        ),)),
        _ => unreachable!(),
    }
}
fn extension_tooltip(stage: usize) -> View {
    match stage {
        0 | 3 => TextBlock::new().text("owner").into(),
        1 => TextBlock::new().text("owner").tooltip("surface a"),
        2 => {
            TextBlock::new()
                .text("owner")
                .tooltip_with(Tooltip::rich(StackPanel::new().children((
                    TextBlock::new().text("surface b"),
                    TextBlock::new().text("detail"),
                ))))
        }
        _ => unreachable!(),
    }
}
fn extension_flyout(stage: usize) -> View {
    match stage {
        0 | 3 => Button::new().content(TextBlock::new().text("owner")),
        1 => Button::new()
            .content(TextBlock::new().text("owner"))
            .flyout("surface a"),
        2 => Button::new()
            .content(TextBlock::new().text("owner"))
            .flyout_with(Flyout::rich(StackPanel::new().children((
                TextBlock::new().text("surface b"),
                TextBlock::new().text("detail"),
            )))),
        _ => unreachable!(),
    }
}
fn extension_menu(stage: usize) -> View {
    match stage {
        0 | 3 => Button::new().content(TextBlock::new().text("owner")),
        1 => Button::new()
            .content(TextBlock::new().text("owner"))
            .menu(Menu::new(
                [
                    MenuItem::item("open", "Open"),
                    MenuItem::separator("separator"),
                ],
                |_| {},
            )),
        2 => Button::new()
            .content(TextBlock::new().text("owner"))
            .menu(Menu::new(
                [MenuItem::submenu(
                    "share",
                    "Share",
                    [MenuItem::item("email", "Email")],
                )],
                |_| {},
            )),
        _ => unreachable!(),
    }
}
fn extension_command_bar_flyout(stage: usize) -> View {
    match stage {
        0 | 3 => Button::new().content(TextBlock::new().text("owner")),
        1 => Button::new()
            .content(TextBlock::new().text("owner"))
            .command_bar_flyout(CommandBarFlyout::new(
                [CommandBarCommand::button("bold", "Bold")],
                [CommandBarCommand::button("copy", "Copy")],
                |_| {},
            )),
        2 => Button::new()
            .content(TextBlock::new().text("owner"))
            .command_bar_flyout(CommandBarFlyout::new(
                [CommandBarCommand::separator("separator")],
                [CommandBarCommand::button("paste", "Paste")],
                |_| {},
            )),
        _ => unreachable!(),
    }
}
fn extension_tree_nodes(stage: usize) -> View {
    match stage {
        0 | 3 => TreeView::new().nodes(std::iter::empty::<TreeNode>()),
        1 => TreeView::new().nodes([TreeNode::new("root", "Root").children([
            TreeNode::new("first", "First"),
            TreeNode::new("second", "Second"),
        ])]),
        2 => TreeView::new().nodes([TreeNode::new("root", "Changed").expanded(true).children([
            TreeNode::new("second", "Second"),
            TreeNode::new("third", "Third"),
        ])]),
        _ => unreachable!(),
    }
}
pub(crate) static SURFACE_CASES: &[SurfaceCase] = &[
    SurfaceCase {
        name: "capability.Layout.Width",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_width,
    },
    SurfaceCase {
        name: "capability.Layout.Height",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_height,
    },
    SurfaceCase {
        name: "capability.Layout.MinWidth",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_min_width,
    },
    SurfaceCase {
        name: "capability.Layout.MaxWidth",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_max_width,
    },
    SurfaceCase {
        name: "capability.Layout.MinHeight",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_min_height,
    },
    SurfaceCase {
        name: "capability.Layout.MaxHeight",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_max_height,
    },
    SurfaceCase {
        name: "capability.Layout.Opacity",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_opacity,
    },
    SurfaceCase {
        name: "capability.Layout.HorizontalAlignment",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_horizontal_alignment,
    },
    SurfaceCase {
        name: "capability.Layout.VerticalAlignment",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_vertical_alignment,
    },
    SurfaceCase {
        name: "capability.Layout.Margin",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_layout_margin,
    },
    SurfaceCase {
        name: "capability.GridChild.Row",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_child_row,
    },
    SurfaceCase {
        name: "capability.GridChild.Column",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_child_column,
    },
    SurfaceCase {
        name: "capability.GridChild.RowSpan",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_child_row_span,
    },
    SurfaceCase {
        name: "capability.GridChild.ColumnSpan",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_child_column_span,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignLeft",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_left,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignTop",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_top,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignRight",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_right,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignBottom",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_bottom,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignHorizontalCenter",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_horizontal_center,
    },
    SurfaceCase {
        name: "capability.RelativePanelChild.AlignVerticalCenter",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_relative_panel_child_align_vertical_center,
    },
    SurfaceCase {
        name: "capability.CanvasChild.Left",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_canvas_child_left,
    },
    SurfaceCase {
        name: "capability.CanvasChild.Top",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_canvas_child_top,
    },
    SurfaceCase {
        name: "capability.Automation.Name",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_automation_name,
    },
    SurfaceCase {
        name: "capability.Automation.Id",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_automation_id,
    },
    SurfaceCase {
        name: "capability.Automation.HeadingLevel",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_automation_heading_level,
    },
    SurfaceCase {
        name: "capability.GridDefinitions.Rows",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_definitions_rows,
    },
    SurfaceCase {
        name: "capability.GridDefinitions.Columns",
        kind: SurfaceKind::CapabilityProperty,
        stages: 4,
        subscription_delta: None,
        build: capability_grid_definitions_columns,
    },
    SurfaceCase {
        name: "extension.Tooltip",
        kind: SurfaceKind::Extension,
        stages: 4,
        subscription_delta: None,
        build: extension_tooltip,
    },
    SurfaceCase {
        name: "extension.Flyout",
        kind: SurfaceKind::Extension,
        stages: 4,
        subscription_delta: None,
        build: extension_flyout,
    },
    SurfaceCase {
        name: "extension.Menu",
        kind: SurfaceKind::Extension,
        stages: 4,
        subscription_delta: None,
        build: extension_menu,
    },
    SurfaceCase {
        name: "extension.CommandBarFlyout",
        kind: SurfaceKind::Extension,
        stages: 4,
        subscription_delta: None,
        build: extension_command_bar_flyout,
    },
    SurfaceCase {
        name: "extension.TreeView.Nodes",
        kind: SurfaceKind::Extension,
        stages: 4,
        subscription_delta: None,
        build: extension_tree_nodes,
    },
    SurfaceCase {
        name: "control.TextBlock.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_text_block,
    },
    SurfaceCase {
        name: "property.TextBlock.Text",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_text,
    },
    SurfaceCase {
        name: "property.TextBlock.TextWrapping",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_text_wrapping,
    },
    SurfaceCase {
        name: "property.TextBlock.FontSize",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_font_size,
    },
    SurfaceCase {
        name: "property.TextBlock.FontWeight",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_font_weight,
    },
    SurfaceCase {
        name: "property.TextBlock.IsTextSelectionEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_is_text_selection_enabled,
    },
    SurfaceCase {
        name: "property.TextBlock.MaxLines",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_max_lines,
    },
    SurfaceCase {
        name: "property.TextBlock.TextTrimming",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_text_trimming,
    },
    SurfaceCase {
        name: "property.TextBlock.Foreground",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_block_foreground,
    },
    SurfaceCase {
        name: "control.Button.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_button,
    },
    SurfaceCase {
        name: "property.Button.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_is_enabled,
    },
    SurfaceCase {
        name: "property.Button.HorizontalContentAlignment",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_horizontal_content_alignment,
    },
    SurfaceCase {
        name: "property.Button.VerticalContentAlignment",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_vertical_content_alignment,
    },
    SurfaceCase {
        name: "property.Button.Resources",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_resource_overrides,
    },
    SurfaceCase {
        name: "property.Button.Style",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_style,
    },
    SurfaceCase {
        name: "property.Button.KeyboardAccelerators",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_button_key_accelerators,
    },
    SurfaceCase {
        name: "structural.Button.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_button_content,
    },
    SurfaceCase {
        name: "event.Button.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_button_on_click,
    },
    SurfaceCase {
        name: "control.HyperlinkButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_hyperlink_button,
    },
    SurfaceCase {
        name: "property.HyperlinkButton.NavigateUri",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_hyperlink_button_navigate_uri,
    },
    SurfaceCase {
        name: "property.HyperlinkButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_hyperlink_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.HyperlinkButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_hyperlink_button_content,
    },
    SurfaceCase {
        name: "event.HyperlinkButton.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_hyperlink_button_on_click,
    },
    SurfaceCase {
        name: "control.RepeatButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_repeat_button,
    },
    SurfaceCase {
        name: "property.RepeatButton.Delay",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_repeat_button_delay,
    },
    SurfaceCase {
        name: "property.RepeatButton.Interval",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_repeat_button_interval,
    },
    SurfaceCase {
        name: "property.RepeatButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_repeat_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.RepeatButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_repeat_button_content,
    },
    SurfaceCase {
        name: "event.RepeatButton.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_repeat_button_on_click,
    },
    SurfaceCase {
        name: "control.Border.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_border,
    },
    SurfaceCase {
        name: "property.Border.Padding",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_padding,
    },
    SurfaceCase {
        name: "property.Border.BorderThickness",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_border_thickness,
    },
    SurfaceCase {
        name: "property.Border.CornerRadius",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_corner_radius,
    },
    SurfaceCase {
        name: "property.Border.Background",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_background,
    },
    SurfaceCase {
        name: "property.Border.BorderBrush",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_border_brush,
    },
    SurfaceCase {
        name: "property.Border.OpacityTransition",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_opacity_transition,
    },
    SurfaceCase {
        name: "property.Border.Scale",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_scale,
    },
    SurfaceCase {
        name: "property.Border.ScaleTransition",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_scale_transition,
    },
    SurfaceCase {
        name: "property.Border.CapturePointerOnPress",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_capture_pointer_on_press,
    },
    SurfaceCase {
        name: "property.Border.AllowDrop",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_border_drop_policy,
    },
    SurfaceCase {
        name: "structural.Border.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_border_content,
    },
    SurfaceCase {
        name: "event.Border.DragEnter",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_drag_enter,
    },
    SurfaceCase {
        name: "event.Border.DragOver",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_drag_over,
    },
    SurfaceCase {
        name: "event.Border.DragLeave",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_drag_leave,
    },
    SurfaceCase {
        name: "event.Border.Drop",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_drop,
    },
    SurfaceCase {
        name: "event.Border.PointerPressed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_pressed,
    },
    SurfaceCase {
        name: "event.Border.PointerMoved",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_moved,
    },
    SurfaceCase {
        name: "event.Border.PointerEntered",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_entered,
    },
    SurfaceCase {
        name: "event.Border.PointerExited",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_exited,
    },
    SurfaceCase {
        name: "event.Border.PointerReleased",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_released,
    },
    SurfaceCase {
        name: "event.Border.PointerCaptureLost",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_capture_lost,
    },
    SurfaceCase {
        name: "event.Border.PointerCanceled",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_border_on_pointer_canceled,
    },
    SurfaceCase {
        name: "control.BreadcrumbBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_breadcrumb_bar,
    },
    SurfaceCase {
        name: "property.BreadcrumbBar.ItemsSource",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_breadcrumb_bar_items_source,
    },
    SurfaceCase {
        name: "event.BreadcrumbBar.ItemClicked",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_breadcrumb_bar_on_item_clicked,
    },
    SurfaceCase {
        name: "control.StackPanel.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_stack_panel,
    },
    SurfaceCase {
        name: "property.StackPanel.Orientation",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_stack_panel_orientation,
    },
    SurfaceCase {
        name: "property.StackPanel.Spacing",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_stack_panel_spacing,
    },
    SurfaceCase {
        name: "structural.StackPanel.Children",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_stack_panel_children,
    },
    SurfaceCase {
        name: "control.VariableSizedWrapGrid.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_variable_sized_wrap_grid,
    },
    SurfaceCase {
        name: "property.VariableSizedWrapGrid.ItemWidth",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_variable_sized_wrap_grid_item_width,
    },
    SurfaceCase {
        name: "property.VariableSizedWrapGrid.ItemHeight",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_variable_sized_wrap_grid_item_height,
    },
    SurfaceCase {
        name: "property.VariableSizedWrapGrid.Orientation",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_variable_sized_wrap_grid_orientation,
    },
    SurfaceCase {
        name: "structural.VariableSizedWrapGrid.Children",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_variable_sized_wrap_grid_children,
    },
    SurfaceCase {
        name: "control.Grid.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_grid,
    },
    SurfaceCase {
        name: "property.Grid.RowSpacing",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_row_spacing,
    },
    SurfaceCase {
        name: "property.Grid.ColumnSpacing",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_column_spacing,
    },
    SurfaceCase {
        name: "property.Grid.KeyboardAccelerators",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_key_accelerators,
    },
    SurfaceCase {
        name: "property.Grid.Background",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_background,
    },
    SurfaceCase {
        name: "structural.Grid.Children",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_grid_children,
    },
    SurfaceCase {
        name: "control.TextBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_text_box,
    },
    SurfaceCase {
        name: "property.TextBox.Text",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_text,
    },
    SurfaceCase {
        name: "property.TextBox.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_placeholder_text,
    },
    SurfaceCase {
        name: "property.TextBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_is_enabled,
    },
    SurfaceCase {
        name: "property.TextBox.AcceptsReturn",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_accepts_return,
    },
    SurfaceCase {
        name: "property.TextBox.TextWrapping",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_text_wrapping,
    },
    SurfaceCase {
        name: "property.TextBox.Background",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_background,
    },
    SurfaceCase {
        name: "property.TextBox.BorderBrush",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_border_brush,
    },
    SurfaceCase {
        name: "property.TextBox.BorderThickness",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_text_box_border_thickness,
    },
    SurfaceCase {
        name: "structural.TextBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_text_box_slot_header,
    },
    SurfaceCase {
        name: "event.TextBox.TextChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_text_box_on_text_changed,
    },
    SurfaceCase {
        name: "control.AutoSuggestBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_auto_suggest_box,
    },
    SurfaceCase {
        name: "property.AutoSuggestBox.Text",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_auto_suggest_box_text,
    },
    SurfaceCase {
        name: "property.AutoSuggestBox.ItemsSource",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_auto_suggest_box_items_source,
    },
    SurfaceCase {
        name: "property.AutoSuggestBox.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_auto_suggest_box_placeholder_text,
    },
    SurfaceCase {
        name: "property.AutoSuggestBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_auto_suggest_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.AutoSuggestBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_auto_suggest_box_slot_header,
    },
    SurfaceCase {
        name: "event.AutoSuggestBox.TextChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_auto_suggest_box_on_text_changed,
    },
    SurfaceCase {
        name: "event.AutoSuggestBox.SuggestionChosen",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_auto_suggest_box_on_suggestion_chosen,
    },
    SurfaceCase {
        name: "control.PasswordBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_password_box,
    },
    SurfaceCase {
        name: "property.PasswordBox.Password",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_password_box_password,
    },
    SurfaceCase {
        name: "property.PasswordBox.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_password_box_placeholder_text,
    },
    SurfaceCase {
        name: "property.PasswordBox.PasswordRevealMode",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_password_box_password_reveal_mode,
    },
    SurfaceCase {
        name: "property.PasswordBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_password_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.PasswordBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_password_box_slot_header,
    },
    SurfaceCase {
        name: "event.PasswordBox.PasswordChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_password_box_on_password_changed,
    },
    SurfaceCase {
        name: "control.NumberBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_number_box,
    },
    SurfaceCase {
        name: "property.NumberBox.Minimum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_number_box_minimum,
    },
    SurfaceCase {
        name: "property.NumberBox.Maximum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_number_box_maximum,
    },
    SurfaceCase {
        name: "property.NumberBox.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_number_box_value,
    },
    SurfaceCase {
        name: "property.NumberBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_number_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.NumberBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_number_box_slot_header,
    },
    SurfaceCase {
        name: "event.NumberBox.ValueChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_number_box_on_value_changed,
    },
    SurfaceCase {
        name: "control.Slider.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_slider,
    },
    SurfaceCase {
        name: "property.Slider.Minimum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_minimum,
    },
    SurfaceCase {
        name: "property.Slider.Maximum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_maximum,
    },
    SurfaceCase {
        name: "property.Slider.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_value,
    },
    SurfaceCase {
        name: "property.Slider.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_is_enabled,
    },
    SurfaceCase {
        name: "property.Slider.StepFrequency",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_step_frequency,
    },
    SurfaceCase {
        name: "property.Slider.Orientation",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_slider_orientation,
    },
    SurfaceCase {
        name: "structural.Slider.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_slider_slot_header,
    },
    SurfaceCase {
        name: "event.Slider.ValueChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_slider_on_value_changed,
    },
    SurfaceCase {
        name: "control.NavigationView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_navigation_view,
    },
    SurfaceCase {
        name: "property.NavigationView.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_is_enabled,
    },
    SurfaceCase {
        name: "property.NavigationView.PaneDisplayMode",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_pane_display_mode,
    },
    SurfaceCase {
        name: "property.NavigationView.IsPaneToggleButtonVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_is_pane_toggle_button_visible,
    },
    SurfaceCase {
        name: "property.NavigationView.IsBackButtonVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_is_back_button_visible,
    },
    SurfaceCase {
        name: "property.NavigationView.IsSettingsVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_is_settings_visible,
    },
    SurfaceCase {
        name: "property.NavigationView.AlwaysShowHeader",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_always_show_header,
    },
    SurfaceCase {
        name: "property.NavigationView.PaneTitle",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_pane_title,
    },
    SurfaceCase {
        name: "property.NavigationView.OpenPaneLength",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_open_pane_length,
    },
    SurfaceCase {
        name: "property.NavigationView.IsPaneOpen",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_is_pane_open,
    },
    SurfaceCase {
        name: "structural.NavigationView.Slot.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_slot_content,
    },
    SurfaceCase {
        name: "structural.NavigationView.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_slot_header,
    },
    SurfaceCase {
        name: "structural.NavigationView.Slot.PaneCustomContent",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_slot_pane_custom_content,
    },
    SurfaceCase {
        name: "structural.NavigationView.Slot.PaneFooter",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_slot_pane_footer,
    },
    SurfaceCase {
        name: "structural.NavigationView.Slot.MenuItems",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_slot_menu_items,
    },
    SurfaceCase {
        name: "event.NavigationView.IsPaneOpenChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_navigation_view_on_is_pane_open_changed,
    },
    SurfaceCase {
        name: "event.NavigationView.DisplayModeChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_navigation_view_on_display_mode_changed,
    },
    SurfaceCase {
        name: "event.NavigationView.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_navigation_view_on_selected_tag_changed,
    },
    SurfaceCase {
        name: "control.NavigationViewItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_navigation_view_item,
    },
    SurfaceCase {
        name: "property.NavigationViewItem.Tag",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_item_tag,
    },
    SurfaceCase {
        name: "property.NavigationViewItem.IsSelected",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_item_is_selected,
    },
    SurfaceCase {
        name: "property.NavigationViewItem.SelectsOnInvoked",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_item_selects_on_invoked,
    },
    SurfaceCase {
        name: "property.NavigationViewItem.IsExpanded",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_navigation_view_item_is_expanded,
    },
    SurfaceCase {
        name: "structural.NavigationViewItem.Slot.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_item_slot_content,
    },
    SurfaceCase {
        name: "structural.NavigationViewItem.Slot.Icon",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_item_slot_icon,
    },
    SurfaceCase {
        name: "structural.NavigationViewItem.Slot.MenuItems",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_navigation_view_item_slot_menu_items,
    },
    SurfaceCase {
        name: "control.SplitView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_split_view,
    },
    SurfaceCase {
        name: "property.SplitView.OpenPaneLength",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_split_view_open_pane_length,
    },
    SurfaceCase {
        name: "property.SplitView.CompactPaneLength",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_split_view_compact_pane_length,
    },
    SurfaceCase {
        name: "property.SplitView.DisplayMode",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_split_view_display_mode,
    },
    SurfaceCase {
        name: "property.SplitView.IsPaneOpen",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_split_view_is_pane_open,
    },
    SurfaceCase {
        name: "structural.SplitView.Slot.Pane",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_split_view_slot_pane,
    },
    SurfaceCase {
        name: "structural.SplitView.Slot.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_split_view_slot_content,
    },
    SurfaceCase {
        name: "event.SplitView.PaneClosed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_split_view_on_pane_closed,
    },
    SurfaceCase {
        name: "control.ProgressBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_progress_bar,
    },
    SurfaceCase {
        name: "property.ProgressBar.Minimum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_minimum,
    },
    SurfaceCase {
        name: "property.ProgressBar.Maximum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_maximum,
    },
    SurfaceCase {
        name: "property.ProgressBar.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_value,
    },
    SurfaceCase {
        name: "property.ProgressBar.IsIndeterminate",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_is_indeterminate,
    },
    SurfaceCase {
        name: "property.ProgressBar.ShowError",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_show_error,
    },
    SurfaceCase {
        name: "property.ProgressBar.ShowPaused",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_show_paused,
    },
    SurfaceCase {
        name: "property.ProgressBar.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_bar_is_enabled,
    },
    SurfaceCase {
        name: "control.ToggleSwitch.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_toggle_switch,
    },
    SurfaceCase {
        name: "property.ToggleSwitch.IsOn",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_toggle_switch_is_on,
    },
    SurfaceCase {
        name: "property.ToggleSwitch.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_toggle_switch_is_enabled,
    },
    SurfaceCase {
        name: "structural.ToggleSwitch.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_toggle_switch_slot_header,
    },
    SurfaceCase {
        name: "structural.ToggleSwitch.Slot.OnContent",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_toggle_switch_slot_on_content,
    },
    SurfaceCase {
        name: "structural.ToggleSwitch.Slot.OffContent",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_toggle_switch_slot_off_content,
    },
    SurfaceCase {
        name: "event.ToggleSwitch.Toggled",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_toggle_switch_on_toggled,
    },
    SurfaceCase {
        name: "control.CheckBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_check_box,
    },
    SurfaceCase {
        name: "property.CheckBox.IsChecked",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_check_box_is_checked,
    },
    SurfaceCase {
        name: "property.CheckBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_check_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.CheckBox.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_check_box_content,
    },
    SurfaceCase {
        name: "event.CheckBox.IsCheckedChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_check_box_on_is_checked_changed,
    },
    SurfaceCase {
        name: "control.ToggleButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_toggle_button,
    },
    SurfaceCase {
        name: "property.ToggleButton.IsChecked",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_toggle_button_is_checked,
    },
    SurfaceCase {
        name: "property.ToggleButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_toggle_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.ToggleButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_toggle_button_content,
    },
    SurfaceCase {
        name: "event.ToggleButton.IsCheckedChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_toggle_button_on_is_checked_changed,
    },
    SurfaceCase {
        name: "control.RadioButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_radio_button,
    },
    SurfaceCase {
        name: "property.RadioButton.GroupName",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_button_group_name,
    },
    SurfaceCase {
        name: "property.RadioButton.IsChecked",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_button_is_checked,
    },
    SurfaceCase {
        name: "property.RadioButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.RadioButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_radio_button_content,
    },
    SurfaceCase {
        name: "event.RadioButton.Checked",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_radio_button_on_checked,
    },
    SurfaceCase {
        name: "control.RadioButtons.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_radio_buttons,
    },
    SurfaceCase {
        name: "property.RadioButtons.ItemsSource",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_buttons_items_source,
    },
    SurfaceCase {
        name: "property.RadioButtons.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_buttons_selected_index,
    },
    SurfaceCase {
        name: "property.RadioButtons.MaxColumns",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_radio_buttons_max_columns,
    },
    SurfaceCase {
        name: "structural.RadioButtons.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_radio_buttons_slot_header,
    },
    SurfaceCase {
        name: "event.RadioButtons.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_radio_buttons_on_selection_changed,
    },
    SurfaceCase {
        name: "control.ItemsRepeater.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_items_repeater,
    },
    SurfaceCase {
        name: "structural.ItemsRepeater.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_items_repeater_items,
    },
    SurfaceCase {
        name: "control.InfoBadge.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_info_badge,
    },
    SurfaceCase {
        name: "property.InfoBadge.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_badge_value,
    },
    SurfaceCase {
        name: "control.InfoBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_info_bar,
    },
    SurfaceCase {
        name: "property.InfoBar.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_bar_title,
    },
    SurfaceCase {
        name: "property.InfoBar.Message",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_bar_message,
    },
    SurfaceCase {
        name: "property.InfoBar.Severity",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_bar_severity,
    },
    SurfaceCase {
        name: "property.InfoBar.IsOpen",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_bar_is_open,
    },
    SurfaceCase {
        name: "property.InfoBar.IsClosable",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_info_bar_is_closable,
    },
    SurfaceCase {
        name: "event.InfoBar.Closed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_info_bar_on_closed,
    },
    SurfaceCase {
        name: "control.PersonPicture.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_person_picture,
    },
    SurfaceCase {
        name: "property.PersonPicture.DisplayName",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_person_picture_display_name,
    },
    SurfaceCase {
        name: "property.PersonPicture.Initials",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_person_picture_initials,
    },
    SurfaceCase {
        name: "control.ScrollViewer.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_scroll_viewer,
    },
    SurfaceCase {
        name: "property.ScrollViewer.HorizontalScrollBarVisibility",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_scroll_viewer_horizontal_scroll_bar_visibility,
    },
    SurfaceCase {
        name: "property.ScrollViewer.VerticalScrollBarVisibility",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_scroll_viewer_vertical_scroll_bar_visibility,
    },
    SurfaceCase {
        name: "structural.ScrollViewer.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_scroll_viewer_content,
    },
    SurfaceCase {
        name: "control.ScrollView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_scroll_view,
    },
    SurfaceCase {
        name: "property.ScrollView.HorizontalScrollBarVisibility",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_scroll_view_horizontal_scroll_bar_visibility,
    },
    SurfaceCase {
        name: "property.ScrollView.VerticalScrollBarVisibility",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_scroll_view_vertical_scroll_bar_visibility,
    },
    SurfaceCase {
        name: "structural.ScrollView.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_scroll_view_content,
    },
    SurfaceCase {
        name: "control.Image.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_image,
    },
    SurfaceCase {
        name: "property.Image.Source",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_image_source,
    },
    SurfaceCase {
        name: "property.Image.Stretch",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_image_stretch,
    },
    SurfaceCase {
        name: "event.Image.ImageOpened",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_image_on_opened,
    },
    SurfaceCase {
        name: "event.Image.ImageFailed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_image_on_failed,
    },
    SurfaceCase {
        name: "control.ProgressRing.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_progress_ring,
    },
    SurfaceCase {
        name: "property.ProgressRing.Minimum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_minimum,
    },
    SurfaceCase {
        name: "property.ProgressRing.Maximum",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_maximum,
    },
    SurfaceCase {
        name: "property.ProgressRing.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_value,
    },
    SurfaceCase {
        name: "property.ProgressRing.IsIndeterminate",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_is_indeterminate,
    },
    SurfaceCase {
        name: "property.ProgressRing.IsActive",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_is_active,
    },
    SurfaceCase {
        name: "property.ProgressRing.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_progress_ring_is_enabled,
    },
    SurfaceCase {
        name: "control.ListBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_list_box,
    },
    SurfaceCase {
        name: "property.ListBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.ListBox.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_list_box_slot_items,
    },
    SurfaceCase {
        name: "event.ListBox.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_list_box_on_selected_tag_changed,
    },
    SurfaceCase {
        name: "control.Rectangle.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_rectangle,
    },
    SurfaceCase {
        name: "property.Rectangle.Fill",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rectangle_fill,
    },
    SurfaceCase {
        name: "property.Rectangle.Stroke",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rectangle_stroke,
    },
    SurfaceCase {
        name: "property.Rectangle.StrokeThickness",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rectangle_stroke_thickness,
    },
    SurfaceCase {
        name: "property.Rectangle.RadiusX",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rectangle_radius_x,
    },
    SurfaceCase {
        name: "property.Rectangle.RadiusY",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rectangle_radius_y,
    },
    SurfaceCase {
        name: "control.Ellipse.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_ellipse,
    },
    SurfaceCase {
        name: "property.Ellipse.Fill",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_ellipse_fill,
    },
    SurfaceCase {
        name: "property.Ellipse.Stroke",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_ellipse_stroke,
    },
    SurfaceCase {
        name: "property.Ellipse.StrokeThickness",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_ellipse_stroke_thickness,
    },
    SurfaceCase {
        name: "control.Line.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_line,
    },
    SurfaceCase {
        name: "property.Line.Stroke",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_stroke,
    },
    SurfaceCase {
        name: "property.Line.StrokeThickness",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_stroke_thickness,
    },
    SurfaceCase {
        name: "property.Line.X1",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_x1,
    },
    SurfaceCase {
        name: "property.Line.Y1",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_y1,
    },
    SurfaceCase {
        name: "property.Line.X2",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_x2,
    },
    SurfaceCase {
        name: "property.Line.Y2",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_line_y2,
    },
    SurfaceCase {
        name: "control.SymbolIcon.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_symbol_icon,
    },
    SurfaceCase {
        name: "property.SymbolIcon.Symbol",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_symbol_icon_symbol,
    },
    SurfaceCase {
        name: "control.ImageIcon.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_image_icon,
    },
    SurfaceCase {
        name: "property.ImageIcon.Source",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_image_icon_source,
    },
    SurfaceCase {
        name: "control.FontIcon.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_font_icon,
    },
    SurfaceCase {
        name: "property.FontIcon.Glyph",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_font_icon_glyph,
    },
    SurfaceCase {
        name: "control.BitmapIcon.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_bitmap_icon,
    },
    SurfaceCase {
        name: "property.BitmapIcon.UriSource",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_bitmap_icon_uri_source,
    },
    SurfaceCase {
        name: "property.BitmapIcon.ShowAsMonochrome",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_bitmap_icon_show_as_monochrome,
    },
    SurfaceCase {
        name: "control.PathIcon.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_path_icon,
    },
    SurfaceCase {
        name: "property.PathIcon.Data",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_path_icon_data,
    },
    SurfaceCase {
        name: "control.ListBoxItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_list_box_item,
    },
    SurfaceCase {
        name: "property.ListBoxItem.Tag",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_box_item_tag,
    },
    SurfaceCase {
        name: "property.ListBoxItem.IsSelected",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_box_item_is_selected,
    },
    SurfaceCase {
        name: "structural.ListBoxItem.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_list_box_item_content,
    },
    SurfaceCase {
        name: "control.RatingControl.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_rating_control,
    },
    SurfaceCase {
        name: "property.RatingControl.MaxRating",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rating_control_max_rating,
    },
    SurfaceCase {
        name: "property.RatingControl.Value",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rating_control_value,
    },
    SurfaceCase {
        name: "property.RatingControl.Caption",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rating_control_caption,
    },
    SurfaceCase {
        name: "property.RatingControl.IsReadOnly",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rating_control_is_read_only,
    },
    SurfaceCase {
        name: "event.RatingControl.ValueChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_rating_control_on_value_changed,
    },
    SurfaceCase {
        name: "control.Expander.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_expander,
    },
    SurfaceCase {
        name: "property.Expander.IsExpanded",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_expander_is_expanded,
    },
    SurfaceCase {
        name: "structural.Expander.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_expander_slot_header,
    },
    SurfaceCase {
        name: "structural.Expander.Slot.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_expander_slot_content,
    },
    SurfaceCase {
        name: "event.Expander.IsExpandedChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_expander_on_is_expanded_changed,
    },
    SurfaceCase {
        name: "control.ComboBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_combo_box,
    },
    SurfaceCase {
        name: "property.ComboBox.ItemsSource",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_combo_box_items_source,
    },
    SurfaceCase {
        name: "property.ComboBox.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_combo_box_selected_index,
    },
    SurfaceCase {
        name: "property.ComboBox.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_combo_box_placeholder_text,
    },
    SurfaceCase {
        name: "property.ComboBox.IsEditable",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_combo_box_is_editable,
    },
    SurfaceCase {
        name: "property.ComboBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_combo_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.ComboBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_combo_box_slot_header,
    },
    SurfaceCase {
        name: "event.ComboBox.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_combo_box_on_selection_changed,
    },
    SurfaceCase {
        name: "control.Pivot.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_pivot,
    },
    SurfaceCase {
        name: "property.Pivot.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_pivot_selected_index,
    },
    SurfaceCase {
        name: "property.Pivot.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_pivot_title,
    },
    SurfaceCase {
        name: "structural.Pivot.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_pivot_slot_items,
    },
    SurfaceCase {
        name: "event.Pivot.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_pivot_on_selection_changed,
    },
    SurfaceCase {
        name: "control.PivotItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_pivot_item,
    },
    SurfaceCase {
        name: "property.PivotItem.Header",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_pivot_item_header,
    },
    SurfaceCase {
        name: "structural.PivotItem.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_pivot_item_content,
    },
    SurfaceCase {
        name: "control.FlipView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_flip_view,
    },
    SurfaceCase {
        name: "property.FlipView.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_flip_view_selected_index,
    },
    SurfaceCase {
        name: "structural.FlipView.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_flip_view_slot_items,
    },
    SurfaceCase {
        name: "event.FlipView.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_flip_view_on_selection_changed,
    },
    SurfaceCase {
        name: "control.SelectorBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_selector_bar,
    },
    SurfaceCase {
        name: "structural.SelectorBar.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_selector_bar_slot_items,
    },
    SurfaceCase {
        name: "event.SelectorBar.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_selector_bar_on_selected_text_changed,
    },
    SurfaceCase {
        name: "control.SelectorBarItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_selector_bar_item,
    },
    SurfaceCase {
        name: "property.SelectorBarItem.Text",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_selector_bar_item_text,
    },
    SurfaceCase {
        name: "property.SelectorBarItem.IsSelected",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_selector_bar_item_is_selected,
    },
    SurfaceCase {
        name: "structural.SelectorBarItem.Slot.Icon",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_selector_bar_item_slot_icon,
    },
    SurfaceCase {
        name: "control.TabView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_tab_view,
    },
    SurfaceCase {
        name: "property.TabView.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_selected_index,
    },
    SurfaceCase {
        name: "property.TabView.CanReorderTabs",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_can_reorder_tabs,
    },
    SurfaceCase {
        name: "property.TabView.IsAddTabButtonVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_is_add_tab_button_visible,
    },
    SurfaceCase {
        name: "structural.TabView.Slot.TabItems",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_tab_view_slot_tab_items,
    },
    SurfaceCase {
        name: "event.TabView.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_tab_view_on_selection_changed,
    },
    SurfaceCase {
        name: "event.TabView.TabCloseRequested",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_tab_view_on_close_requested,
    },
    SurfaceCase {
        name: "event.TabView.AddTabButtonClick",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_tab_view_on_add_tab_button_click,
    },
    SurfaceCase {
        name: "event.TabView.TabItemsChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_tab_view_on_reordered,
    },
    SurfaceCase {
        name: "control.TabViewItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_tab_view_item,
    },
    SurfaceCase {
        name: "property.TabViewItem.Header",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_item_header,
    },
    SurfaceCase {
        name: "property.TabViewItem.IsClosable",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_item_is_closable,
    },
    SurfaceCase {
        name: "property.TabViewItem.Tag",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tab_view_item_tag,
    },
    SurfaceCase {
        name: "structural.TabViewItem.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_tab_view_item_content,
    },
    SurfaceCase {
        name: "control.TeachingTip.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_teaching_tip,
    },
    SurfaceCase {
        name: "property.TeachingTip.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_title,
    },
    SurfaceCase {
        name: "property.TeachingTip.Subtitle",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_subtitle,
    },
    SurfaceCase {
        name: "property.TeachingTip.IsOpen",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_is_open,
    },
    SurfaceCase {
        name: "property.TeachingTip.IsLightDismissEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_is_light_dismiss_enabled,
    },
    SurfaceCase {
        name: "property.TeachingTip.PreferredPlacement",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_preferred_placement,
    },
    SurfaceCase {
        name: "property.TeachingTip.ActionButtonContent",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_action_button_content,
    },
    SurfaceCase {
        name: "property.TeachingTip.CloseButtonContent",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_teaching_tip_close_button_content,
    },
    SurfaceCase {
        name: "event.TeachingTip.Closed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_teaching_tip_on_closed,
    },
    SurfaceCase {
        name: "event.TeachingTip.ActionButtonClick",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_teaching_tip_on_action_button_click,
    },
    SurfaceCase {
        name: "control.DropDownButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_drop_down_button,
    },
    SurfaceCase {
        name: "property.DropDownButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_drop_down_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.DropDownButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_drop_down_button_content,
    },
    SurfaceCase {
        name: "event.DropDownButton.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_drop_down_button_on_click,
    },
    SurfaceCase {
        name: "control.CommandBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_command_bar,
    },
    SurfaceCase {
        name: "structural.CommandBar.Slot.PrimaryCommands",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_command_bar_slot_primary_commands,
    },
    SurfaceCase {
        name: "structural.CommandBar.Slot.SecondaryCommands",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_command_bar_slot_secondary_commands,
    },
    SurfaceCase {
        name: "control.AppBarButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_app_bar_button,
    },
    SurfaceCase {
        name: "property.AppBarButton.Label",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_app_bar_button_label,
    },
    SurfaceCase {
        name: "property.AppBarButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_app_bar_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.AppBarButton.Slot.Icon",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_app_bar_button_slot_icon,
    },
    SurfaceCase {
        name: "event.AppBarButton.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_app_bar_button_on_click,
    },
    SurfaceCase {
        name: "control.AppBarSeparator.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_app_bar_separator,
    },
    SurfaceCase {
        name: "control.MenuBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_menu_bar,
    },
    SurfaceCase {
        name: "structural.MenuBar.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_menu_bar_slot_items,
    },
    SurfaceCase {
        name: "control.MenuBarItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_menu_bar_item,
    },
    SurfaceCase {
        name: "property.MenuBarItem.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_menu_bar_item_title,
    },
    SurfaceCase {
        name: "control.SplitButton.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_split_button,
    },
    SurfaceCase {
        name: "property.SplitButton.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_split_button_is_enabled,
    },
    SurfaceCase {
        name: "structural.SplitButton.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_split_button_content,
    },
    SurfaceCase {
        name: "event.SplitButton.Click",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_split_button_on_click,
    },
    SurfaceCase {
        name: "control.ColorPicker.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_color_picker,
    },
    SurfaceCase {
        name: "property.ColorPicker.Color",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_color,
    },
    SurfaceCase {
        name: "property.ColorPicker.IsAlphaEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_is_alpha_enabled,
    },
    SurfaceCase {
        name: "property.ColorPicker.IsHexInputVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_is_hex_input_visible,
    },
    SurfaceCase {
        name: "property.ColorPicker.IsColorSliderVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_is_color_slider_visible,
    },
    SurfaceCase {
        name: "property.ColorPicker.IsColorChannelTextInputVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_is_color_channel_text_input_visible,
    },
    SurfaceCase {
        name: "property.ColorPicker.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_color_picker_is_enabled,
    },
    SurfaceCase {
        name: "event.ColorPicker.ColorChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_color_picker_on_color_changed,
    },
    SurfaceCase {
        name: "control.DatePicker.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_date_picker,
    },
    SurfaceCase {
        name: "property.DatePicker.DayVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_date_picker_day_visible,
    },
    SurfaceCase {
        name: "property.DatePicker.MonthVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_date_picker_month_visible,
    },
    SurfaceCase {
        name: "property.DatePicker.YearVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_date_picker_year_visible,
    },
    SurfaceCase {
        name: "property.DatePicker.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_date_picker_is_enabled,
    },
    SurfaceCase {
        name: "structural.DatePicker.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_date_picker_slot_header,
    },
    SurfaceCase {
        name: "event.DatePicker.SelectedDateChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_date_picker_on_selected_date_changed,
    },
    SurfaceCase {
        name: "control.TimePicker.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_time_picker,
    },
    SurfaceCase {
        name: "property.TimePicker.ClockIdentifier",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_time_picker_clock_identifier,
    },
    SurfaceCase {
        name: "property.TimePicker.MinuteIncrement",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_time_picker_minute_increment,
    },
    SurfaceCase {
        name: "property.TimePicker.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_time_picker_is_enabled,
    },
    SurfaceCase {
        name: "structural.TimePicker.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_time_picker_slot_header,
    },
    SurfaceCase {
        name: "event.TimePicker.SelectedTimeChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_time_picker_on_selected_time_changed,
    },
    SurfaceCase {
        name: "control.CalendarDatePicker.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_calendar_date_picker,
    },
    SurfaceCase {
        name: "property.CalendarDatePicker.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_date_picker_placeholder_text,
    },
    SurfaceCase {
        name: "property.CalendarDatePicker.IsTodayHighlighted",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_date_picker_is_today_highlighted,
    },
    SurfaceCase {
        name: "property.CalendarDatePicker.IsCalendarOpen",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_date_picker_is_calendar_open,
    },
    SurfaceCase {
        name: "property.CalendarDatePicker.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_date_picker_is_enabled,
    },
    SurfaceCase {
        name: "structural.CalendarDatePicker.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_calendar_date_picker_slot_header,
    },
    SurfaceCase {
        name: "event.CalendarDatePicker.DateChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_calendar_date_picker_on_date_changed,
    },
    SurfaceCase {
        name: "control.ToolTip.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_tool_tip,
    },
    SurfaceCase {
        name: "control.ContentDialog.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_content_dialog,
    },
    SurfaceCase {
        name: "property.ContentDialog.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_title,
    },
    SurfaceCase {
        name: "property.ContentDialog.PrimaryButtonText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_primary_button_text,
    },
    SurfaceCase {
        name: "property.ContentDialog.SecondaryButtonText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_secondary_button_text,
    },
    SurfaceCase {
        name: "property.ContentDialog.CloseButtonText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_close_button_text,
    },
    SurfaceCase {
        name: "property.ContentDialog.IsPrimaryButtonEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_is_primary_button_enabled,
    },
    SurfaceCase {
        name: "property.ContentDialog.IsSecondaryButtonEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_content_dialog_is_secondary_button_enabled,
    },
    SurfaceCase {
        name: "structural.ContentDialog.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_content_dialog_content,
    },
    SurfaceCase {
        name: "event.ContentDialog.Closed",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_content_dialog_on_closed,
    },
    SurfaceCase {
        name: "control.CalendarView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_calendar_view,
    },
    SurfaceCase {
        name: "property.CalendarView.IsTodayHighlighted",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_view_is_today_highlighted,
    },
    SurfaceCase {
        name: "property.CalendarView.IsGroupLabelVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_view_is_group_label_visible,
    },
    SurfaceCase {
        name: "property.CalendarView.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_calendar_view_is_enabled,
    },
    SurfaceCase {
        name: "event.CalendarView.SelectedDatesChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_calendar_view_on_selected_dates_changed,
    },
    SurfaceCase {
        name: "control.ListView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_list_view,
    },
    SurfaceCase {
        name: "property.ListView.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_selected_index,
    },
    SurfaceCase {
        name: "property.ListView.SelectionMode",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_selection_mode,
    },
    SurfaceCase {
        name: "property.ListView.CanDragItems",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_can_drag_items,
    },
    SurfaceCase {
        name: "property.ListView.CanReorderItems",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_can_reorder_items,
    },
    SurfaceCase {
        name: "property.ListView.AllowDrop",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_allow_drop,
    },
    SurfaceCase {
        name: "structural.ListView.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_list_view_slot_items,
    },
    SurfaceCase {
        name: "event.ListView.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_list_view_on_selection_changed,
    },
    SurfaceCase {
        name: "event.ListView.DragItemsCompleted",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_list_view_on_reordered,
    },
    SurfaceCase {
        name: "control.ListViewItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_list_view_item,
    },
    SurfaceCase {
        name: "property.ListViewItem.Tag",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_list_view_item_tag,
    },
    SurfaceCase {
        name: "structural.ListViewItem.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_list_view_item_content,
    },
    SurfaceCase {
        name: "control.TreeView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_tree_view,
    },
    SurfaceCase {
        name: "property.TreeView.SelectionMode",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_tree_view_selection_mode,
    },
    SurfaceCase {
        name: "event.TreeView.ItemInvoked",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_tree_view_on_item_invoked,
    },
    SurfaceCase {
        name: "control.GridView.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_grid_view,
    },
    SurfaceCase {
        name: "property.GridView.SelectedIndex",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_view_selected_index,
    },
    SurfaceCase {
        name: "property.GridView.CanDragItems",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_view_can_drag_items,
    },
    SurfaceCase {
        name: "property.GridView.CanReorderItems",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_view_can_reorder_items,
    },
    SurfaceCase {
        name: "property.GridView.AllowDrop",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_view_allow_drop,
    },
    SurfaceCase {
        name: "structural.GridView.Slot.Items",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_grid_view_slot_items,
    },
    SurfaceCase {
        name: "event.GridView.DragItemsCompleted",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_grid_view_on_reordered,
    },
    SurfaceCase {
        name: "event.GridView.SelectionChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_grid_view_on_selection_changed,
    },
    SurfaceCase {
        name: "control.GridViewItem.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_grid_view_item,
    },
    SurfaceCase {
        name: "property.GridViewItem.Tag",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_grid_view_item_tag,
    },
    SurfaceCase {
        name: "structural.GridViewItem.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_grid_view_item_content,
    },
    SurfaceCase {
        name: "control.RelativePanel.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_relative_panel,
    },
    SurfaceCase {
        name: "structural.RelativePanel.Children",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_relative_panel_children,
    },
    SurfaceCase {
        name: "control.Canvas.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_canvas,
    },
    SurfaceCase {
        name: "structural.Canvas.Children",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_canvas_children,
    },
    SurfaceCase {
        name: "control.RichEditBox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_rich_edit_box,
    },
    SurfaceCase {
        name: "property.RichEditBox.Document",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_edit_box_text,
    },
    SurfaceCase {
        name: "property.RichEditBox.PlaceholderText",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_edit_box_placeholder_text,
    },
    SurfaceCase {
        name: "property.RichEditBox.IsReadOnly",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_edit_box_is_read_only,
    },
    SurfaceCase {
        name: "property.RichEditBox.IsEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_edit_box_is_enabled,
    },
    SurfaceCase {
        name: "structural.RichEditBox.Slot.Header",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_rich_edit_box_slot_header,
    },
    SurfaceCase {
        name: "event.RichEditBox.TextChanged",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(0usize),
        build: event_rich_edit_box_on_text_changed,
    },
    SurfaceCase {
        name: "control.RichTextBlock.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_rich_text_block,
    },
    SurfaceCase {
        name: "property.RichTextBlock.Blocks",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_text_block_paragraphs,
    },
    SurfaceCase {
        name: "property.RichTextBlock.FontSize",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_text_block_font_size,
    },
    SurfaceCase {
        name: "property.RichTextBlock.IsTextSelectionEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_text_block_is_text_selection_enabled,
    },
    SurfaceCase {
        name: "property.RichTextBlock.TextWrapping",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_rich_text_block_text_wrapping,
    },
    SurfaceCase {
        name: "control.Viewbox.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_viewbox,
    },
    SurfaceCase {
        name: "property.Viewbox.Stretch",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_viewbox_stretch,
    },
    SurfaceCase {
        name: "structural.Viewbox.Slot.Child",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_viewbox_slot_child,
    },
    SurfaceCase {
        name: "control.WebView2.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_web_view2,
    },
    SurfaceCase {
        name: "control.SwapChainPanel.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_swap_chain_panel,
    },
    SurfaceCase {
        name: "control.TitleBar.construct",
        kind: SurfaceKind::Control,
        stages: 1,
        subscription_delta: None,
        build: construct_title_bar,
    },
    SurfaceCase {
        name: "property.TitleBar.Title",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_title_bar_title,
    },
    SurfaceCase {
        name: "property.TitleBar.Subtitle",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_title_bar_subtitle,
    },
    SurfaceCase {
        name: "property.TitleBar.IsBackButtonVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_title_bar_is_back_button_visible,
    },
    SurfaceCase {
        name: "property.TitleBar.IsBackButtonEnabled",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_title_bar_is_back_button_enabled,
    },
    SurfaceCase {
        name: "property.TitleBar.IsPaneToggleButtonVisible",
        kind: SurfaceKind::Property,
        stages: 4,
        subscription_delta: None,
        build: property_title_bar_is_pane_toggle_button_visible,
    },
    SurfaceCase {
        name: "structural.TitleBar.Slot.Content",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_title_bar_slot_content,
    },
    SurfaceCase {
        name: "structural.TitleBar.Slot.RightHeader",
        kind: SurfaceKind::Structural,
        stages: 4,
        subscription_delta: None,
        build: structural_title_bar_slot_right_header,
    },
    SurfaceCase {
        name: "event.TitleBar.BackRequested",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_title_bar_on_back_requested,
    },
    SurfaceCase {
        name: "event.TitleBar.PaneToggleRequested",
        kind: SurfaceKind::Event,
        stages: 4,
        subscription_delta: Some(1usize),
        build: event_title_bar_on_pane_toggle_requested,
    },
];
pub static PROJECTED_PROPERTIES: &[PropertySurface] = &[
    PropertySurface {
        control: "TextBlock",
        property: "Text",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "TextWrapping",
        value: "TextWrapping",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "FontSize",
        value: "F64",
        adapter: "direct",
        validation: Some("FinitePositive"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "FontWeight",
        value: "FontWeight",
        adapter: "FontWeight",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "IsTextSelectionEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "MaxLines",
        value: "I32",
        adapter: "direct",
        validation: Some("NonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "TextTrimming",
        value: "TextTrimming",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBlock",
        property: "Foreground",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Button",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Button",
        property: "HorizontalContentAlignment",
        value: "HorizontalAlignment",
        adapter: "HorizontalContentAlignment",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Button",
        property: "VerticalContentAlignment",
        value: "VerticalAlignment",
        adapter: "VerticalContentAlignment",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Button",
        property: "Resources",
        value: "ResourceOverrides",
        adapter: "ResourceOverrides",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Button",
        property: "Style",
        value: "ButtonStyle",
        adapter: "ResourceStyle",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Button",
        property: "KeyboardAccelerators",
        value: "KeyAccelerators",
        adapter: "KeyAccelerators",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "HyperlinkButton",
        property: "NavigateUri",
        value: "Str",
        adapter: "Uri",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "HyperlinkButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RepeatButton",
        property: "Delay",
        value: "I32",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RepeatButton",
        property: "Interval",
        value: "I32",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RepeatButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "Padding",
        value: "Thickness",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "BorderThickness",
        value: "Thickness",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "CornerRadius",
        value: "CornerRadius",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "Background",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Border",
        property: "BorderBrush",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Border",
        property: "OpacityTransition",
        value: "Duration",
        adapter: "ImplicitOpacityTransition",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "Scale",
        value: "F64",
        adapter: "ImplicitScale",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "ScaleTransition",
        value: "Duration",
        adapter: "ImplicitScaleTransition",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "CapturePointerOnPress",
        value: "Bool",
        adapter: "PointerCapture",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Border",
        property: "AllowDrop",
        value: "DragDropPolicy",
        adapter: "DropPolicy",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "BreadcrumbBar",
        property: "ItemsSource",
        value: "StrList",
        adapter: "InspectableStringList",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "StackPanel",
        property: "Orientation",
        value: "Orientation",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "StackPanel",
        property: "Spacing",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "VariableSizedWrapGrid",
        property: "ItemWidth",
        value: "F64",
        adapter: "direct",
        validation: Some("FinitePositive"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "VariableSizedWrapGrid",
        property: "ItemHeight",
        value: "F64",
        adapter: "direct",
        validation: Some("FinitePositive"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "VariableSizedWrapGrid",
        property: "Orientation",
        value: "Orientation",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Grid",
        property: "RowSpacing",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Grid",
        property: "ColumnSpacing",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Grid",
        property: "KeyboardAccelerators",
        value: "KeyAccelerators",
        adapter: "KeyAccelerators",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Grid",
        property: "Background",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "TextBox",
        property: "Text",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBox",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBox",
        property: "AcceptsReturn",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBox",
        property: "TextWrapping",
        value: "TextWrapping",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TextBox",
        property: "Background",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "TextBox",
        property: "BorderBrush",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "TextBox",
        property: "BorderThickness",
        value: "Thickness",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AutoSuggestBox",
        property: "Text",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AutoSuggestBox",
        property: "ItemsSource",
        value: "StrList",
        adapter: "InspectableStringList",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AutoSuggestBox",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AutoSuggestBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PasswordBox",
        property: "Password",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PasswordBox",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PasswordBox",
        property: "PasswordRevealMode",
        value: "PasswordRevealMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PasswordBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NumberBox",
        property: "Minimum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NumberBox",
        property: "Maximum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NumberBox",
        property: "Value",
        value: "OptionalF64",
        adapter: "NumberBoxValue",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NumberBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "Minimum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "Maximum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "Value",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "StepFrequency",
        value: "F64",
        adapter: "direct",
        validation: Some("FinitePositive"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Slider",
        property: "Orientation",
        value: "Orientation",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TitleBar",
        property: "Title",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TitleBar",
        property: "Subtitle",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TitleBar",
        property: "IsBackButtonVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TitleBar",
        property: "IsBackButtonEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TitleBar",
        property: "IsPaneToggleButtonVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "PaneDisplayMode",
        value: "NavigationViewPaneDisplayMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "IsPaneToggleButtonVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "IsBackButtonVisible",
        value: "NavigationViewBackButtonVisible",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "IsSettingsVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "AlwaysShowHeader",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "PaneTitle",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "OpenPaneLength",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationView",
        property: "IsPaneOpen",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationViewItem",
        property: "Tag",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationViewItem",
        property: "IsSelected",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationViewItem",
        property: "SelectsOnInvoked",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "NavigationViewItem",
        property: "IsExpanded",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SplitView",
        property: "OpenPaneLength",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SplitView",
        property: "CompactPaneLength",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SplitView",
        property: "DisplayMode",
        value: "SplitViewDisplayMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SplitView",
        property: "IsPaneOpen",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "Minimum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "Maximum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "Value",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "IsIndeterminate",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "ShowError",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "ShowPaused",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressBar",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ToggleSwitch",
        property: "IsOn",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ToggleSwitch",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CheckBox",
        property: "IsChecked",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CheckBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ToggleButton",
        property: "IsChecked",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ToggleButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButton",
        property: "GroupName",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButton",
        property: "IsChecked",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButtons",
        property: "ItemsSource",
        value: "StrList",
        adapter: "InspectableStringList",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButtons",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RadioButtons",
        property: "MaxColumns",
        value: "I32",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBadge",
        property: "Value",
        value: "I32",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBar",
        property: "Title",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBar",
        property: "Message",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBar",
        property: "Severity",
        value: "InfoBarSeverity",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBar",
        property: "IsOpen",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "InfoBar",
        property: "IsClosable",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PersonPicture",
        property: "DisplayName",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PersonPicture",
        property: "Initials",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ScrollViewer",
        property: "HorizontalScrollBarVisibility",
        value: "ScrollBarVisibility",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ScrollViewer",
        property: "VerticalScrollBarVisibility",
        value: "ScrollBarVisibility",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ScrollView",
        property: "HorizontalScrollBarVisibility",
        value: "ScrollingScrollBarVisibility",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ScrollView",
        property: "VerticalScrollBarVisibility",
        value: "ScrollingScrollBarVisibility",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Image",
        property: "Source",
        value: "ImageValue",
        adapter: "ImageUri",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Image",
        property: "Stretch",
        value: "Stretch",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "Minimum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "Maximum",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "Value",
        value: "F64",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "IsIndeterminate",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "IsActive",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ProgressRing",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Rectangle",
        property: "Fill",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Rectangle",
        property: "Stroke",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Rectangle",
        property: "StrokeThickness",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Rectangle",
        property: "RadiusX",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Rectangle",
        property: "RadiusY",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Ellipse",
        property: "Fill",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Ellipse",
        property: "Stroke",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Ellipse",
        property: "StrokeThickness",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Line",
        property: "Stroke",
        value: "Brush",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: true,
    },
    PropertySurface {
        control: "Line",
        property: "StrokeThickness",
        value: "F64",
        adapter: "direct",
        validation: Some("FiniteNonNegative"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Line",
        property: "X1",
        value: "F64",
        adapter: "direct",
        validation: Some("Finite"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Line",
        property: "Y1",
        value: "F64",
        adapter: "direct",
        validation: Some("Finite"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Line",
        property: "X2",
        value: "F64",
        adapter: "direct",
        validation: Some("Finite"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Line",
        property: "Y2",
        value: "F64",
        adapter: "direct",
        validation: Some("Finite"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SymbolIcon",
        property: "Symbol",
        value: "Symbol",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ImageIcon",
        property: "Source",
        value: "ImageValue",
        adapter: "ImageUri",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "FontIcon",
        property: "Glyph",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "BitmapIcon",
        property: "UriSource",
        value: "Str",
        adapter: "Uri",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "BitmapIcon",
        property: "ShowAsMonochrome",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PathIcon",
        property: "Data",
        value: "Str",
        adapter: "PathData",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListBoxItem",
        property: "Tag",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListBoxItem",
        property: "IsSelected",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RatingControl",
        property: "MaxRating",
        value: "I32",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RatingControl",
        property: "Value",
        value: "OptionalF64",
        adapter: "RatingValue",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RatingControl",
        property: "Caption",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RatingControl",
        property: "IsReadOnly",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Expander",
        property: "IsExpanded",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ComboBox",
        property: "ItemsSource",
        value: "StrList",
        adapter: "InspectableStringList",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ComboBox",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ComboBox",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ComboBox",
        property: "IsEditable",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ComboBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Pivot",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Pivot",
        property: "Title",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "PivotItem",
        property: "Header",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "FlipView",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SelectorBarItem",
        property: "Text",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SelectorBarItem",
        property: "IsSelected",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabView",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabView",
        property: "CanReorderTabs",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabView",
        property: "IsAddTabButtonVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabViewItem",
        property: "Header",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabViewItem",
        property: "IsClosable",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TabViewItem",
        property: "Tag",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "Title",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "Subtitle",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "IsOpen",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "IsLightDismissEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "PreferredPlacement",
        value: "TeachingTipPlacementMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "ActionButtonContent",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TeachingTip",
        property: "CloseButtonContent",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "DropDownButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AppBarButton",
        property: "Label",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "AppBarButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "MenuBarItem",
        property: "Title",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "SplitButton",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "Color",
        value: "Color",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "IsAlphaEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "IsHexInputVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "IsColorSliderVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "IsColorChannelTextInputVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ColorPicker",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "DatePicker",
        property: "DayVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "DatePicker",
        property: "MonthVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "DatePicker",
        property: "YearVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "DatePicker",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TimePicker",
        property: "ClockIdentifier",
        value: "Str",
        adapter: "ClockIdentifier",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TimePicker",
        property: "MinuteIncrement",
        value: "I32",
        adapter: "direct",
        validation: Some("ZeroToFiftyNine"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TimePicker",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarDatePicker",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarDatePicker",
        property: "IsTodayHighlighted",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarDatePicker",
        property: "IsCalendarOpen",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarDatePicker",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "Title",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "PrimaryButtonText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "SecondaryButtonText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "CloseButtonText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "IsPrimaryButtonEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ContentDialog",
        property: "IsSecondaryButtonEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarView",
        property: "IsTodayHighlighted",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarView",
        property: "IsGroupLabelVisible",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "CalendarView",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListView",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListView",
        property: "SelectionMode",
        value: "ListViewSelectionMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListView",
        property: "CanDragItems",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListView",
        property: "CanReorderItems",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListView",
        property: "AllowDrop",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "ListViewItem",
        property: "Tag",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "TreeView",
        property: "SelectionMode",
        value: "TreeViewSelectionMode",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "GridView",
        property: "SelectedIndex",
        value: "SelectionIndex",
        adapter: "SelectionIndex",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "GridView",
        property: "CanDragItems",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "GridView",
        property: "CanReorderItems",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "GridView",
        property: "AllowDrop",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "GridViewItem",
        property: "Tag",
        value: "Str",
        adapter: "InspectableString",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichEditBox",
        property: "Document",
        value: "Str",
        adapter: "RichEditText",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichEditBox",
        property: "PlaceholderText",
        value: "Str",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichEditBox",
        property: "IsReadOnly",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichEditBox",
        property: "IsEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichTextBlock",
        property: "Blocks",
        value: "RichText",
        adapter: "RichTextBlocks",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichTextBlock",
        property: "FontSize",
        value: "F64",
        adapter: "direct",
        validation: Some("FinitePositive"),
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichTextBlock",
        property: "IsTextSelectionEnabled",
        value: "Bool",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "RichTextBlock",
        property: "TextWrapping",
        value: "TextWrapping",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
    PropertySurface {
        control: "Viewbox",
        property: "Stretch",
        value: "Stretch",
        adapter: "direct",
        validation: None,
        clearable: true,
        theme_style: false,
    },
];
pub static PROJECTED_EVENTS: &[EventSurface] = &[
    EventSurface {
        control: "Button",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "HyperlinkButton",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "RepeatButton",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "DragEnter",
        payload: "DragKind",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: Some("drop_policy"),
    },
    EventSurface {
        control: "Border",
        event: "DragOver",
        payload: "DragKind",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: Some("drop_policy"),
    },
    EventSurface {
        control: "Border",
        event: "DragLeave",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "Drop",
        payload: "DroppedData",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerPressed",
        payload: "PointerEventInfo",
        conversion: "Identity",
        subscription: "callback",
        delivery: "live:Pointer_RealInputGesture",
        active_property: Some("capture_pointer_on_press"),
    },
    EventSurface {
        control: "Border",
        event: "PointerMoved",
        payload: "PointerEventInfo",
        conversion: "Identity",
        subscription: "callback",
        delivery: "live:Pointer_RealInputGesture",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerEntered",
        payload: "PointerEventInfo",
        conversion: "Identity",
        subscription: "callback",
        delivery: "live:Pointer_RealInputGesture",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerExited",
        payload: "PointerEventInfo",
        conversion: "Identity",
        subscription: "callback",
        delivery: "live:Pointer_RealInputGesture",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerReleased",
        payload: "PointerEventInfo",
        conversion: "Identity",
        subscription: "callback",
        delivery: "live:Pointer_RealInputGesture",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerCaptureLost",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Border",
        event: "PointerCanceled",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "BreadcrumbBar",
        event: "ItemClicked",
        payload: "Str",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TextBox",
        event: "TextChanged",
        payload: "Str",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "AutoSuggestBox",
        event: "TextChanged",
        payload: "Str",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "AutoSuggestBox",
        event: "SuggestionChosen",
        payload: "Str",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "PasswordBox",
        event: "PasswordChanged",
        payload: "Str",
        conversion: "Identity",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "NumberBox",
        event: "ValueChanged",
        payload: "OptionalF64",
        conversion: "NumberBoxValue",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "Slider",
        event: "ValueChanged",
        payload: "F64",
        conversion: "Identity",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "TitleBar",
        event: "BackRequested",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TitleBar",
        event: "PaneToggleRequested",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "NavigationView",
        event: "IsPaneOpenChanged",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "NavigationView",
        event: "DisplayModeChanged",
        payload: "NavigationViewDisplayMode",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "NavigationView",
        event: "SelectionChanged",
        payload: "SelectionChange",
        conversion: "Selection",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "SplitView",
        event: "PaneClosed",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "ToggleSwitch",
        event: "Toggled",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "CheckBox",
        event: "IsCheckedChanged",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "live:Events_ReplacementAndRevocation",
        active_property: None,
    },
    EventSurface {
        control: "ToggleButton",
        event: "IsCheckedChanged",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "RadioButton",
        event: "Checked",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "RadioButtons",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "InfoBar",
        event: "Closed",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Image",
        event: "ImageOpened",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Image",
        event: "ImageFailed",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "ListBox",
        event: "SelectionChanged",
        payload: "SelectionChange",
        conversion: "Selection",
        subscription: "always",
        delivery: "live:Controlled_NativeFeedback",
        active_property: None,
    },
    EventSurface {
        control: "RatingControl",
        event: "ValueChanged",
        payload: "OptionalF64",
        conversion: "RatingValue",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Expander",
        event: "IsExpandedChanged",
        payload: "Bool",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "ComboBox",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "Pivot",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "FlipView",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "SelectorBar",
        event: "SelectionChanged",
        payload: "SelectionChange",
        conversion: "Selection",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TabView",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TabView",
        event: "TabCloseRequested",
        payload: "Str",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TabView",
        event: "AddTabButtonClick",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TabView",
        event: "TabItemsChanged",
        payload: "StrList",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TeachingTip",
        event: "Closed",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TeachingTip",
        event: "ActionButtonClick",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "DropDownButton",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "AppBarButton",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "SplitButton",
        event: "Click",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "ColorPicker",
        event: "ColorChanged",
        payload: "Color",
        conversion: "Identity",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "DatePicker",
        event: "SelectedDateChanged",
        payload: "OptionalDateTime",
        conversion: "Nullable",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TimePicker",
        event: "SelectedTimeChanged",
        payload: "OptionalTimeSpan",
        conversion: "Nullable",
        subscription: "callback",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "CalendarDatePicker",
        event: "DateChanged",
        payload: "OptionalDateTime",
        conversion: "Nullable",
        subscription: "callback",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "ContentDialog",
        event: "Closed",
        payload: "ContentDialogResult",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "CalendarView",
        event: "SelectedDatesChanged",
        payload: "Unit",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "ListView",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "live:Events_NativePayloadDelivery",
        active_property: None,
    },
    EventSurface {
        control: "ListView",
        event: "DragItemsCompleted",
        payload: "StrList",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "TreeView",
        event: "ItemInvoked",
        payload: "Str",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "GridView",
        event: "DragItemsCompleted",
        payload: "StrList",
        conversion: "Identity",
        subscription: "callback",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "GridView",
        event: "SelectionChanged",
        payload: "SelectionIndex",
        conversion: "SelectionIndex",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
    EventSurface {
        control: "RichEditBox",
        event: "TextChanged",
        payload: "Str",
        conversion: "Identity",
        subscription: "always",
        delivery: "registration+deterministic",
        active_property: None,
    },
];
pub static CAPABILITY_PROPERTIES: &[CapabilityPropertySurface] = &[
    CapabilityPropertySurface {
        capability: "Layout",
        property: "Width",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "Height",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "MinWidth",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "MaxWidth",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "MinHeight",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "MaxHeight",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "Opacity",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "HorizontalAlignment",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "VerticalAlignment",
    },
    CapabilityPropertySurface {
        capability: "Layout",
        property: "Margin",
    },
    CapabilityPropertySurface {
        capability: "GridChild",
        property: "Row",
    },
    CapabilityPropertySurface {
        capability: "GridChild",
        property: "Column",
    },
    CapabilityPropertySurface {
        capability: "GridChild",
        property: "RowSpan",
    },
    CapabilityPropertySurface {
        capability: "GridChild",
        property: "ColumnSpan",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignLeft",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignTop",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignRight",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignBottom",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignHorizontalCenter",
    },
    CapabilityPropertySurface {
        capability: "RelativePanelChild",
        property: "AlignVerticalCenter",
    },
    CapabilityPropertySurface {
        capability: "CanvasChild",
        property: "Left",
    },
    CapabilityPropertySurface {
        capability: "CanvasChild",
        property: "Top",
    },
    CapabilityPropertySurface {
        capability: "Automation",
        property: "Name",
    },
    CapabilityPropertySurface {
        capability: "Automation",
        property: "Id",
    },
    CapabilityPropertySurface {
        capability: "Automation",
        property: "HeadingLevel",
    },
    CapabilityPropertySurface {
        capability: "GridDefinitions",
        property: "Rows",
    },
    CapabilityPropertySurface {
        capability: "GridDefinitions",
        property: "Columns",
    },
];
pub static STRUCTURAL_SURFACES: &[StructuralSurface] = &[
    StructuralSurface {
        control: "Button",
        member: "Content",
    },
    StructuralSurface {
        control: "HyperlinkButton",
        member: "Content",
    },
    StructuralSurface {
        control: "RepeatButton",
        member: "Content",
    },
    StructuralSurface {
        control: "Border",
        member: "Content",
    },
    StructuralSurface {
        control: "StackPanel",
        member: "Children",
    },
    StructuralSurface {
        control: "VariableSizedWrapGrid",
        member: "Children",
    },
    StructuralSurface {
        control: "Grid",
        member: "Children",
    },
    StructuralSurface {
        control: "TextBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "AutoSuggestBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "PasswordBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "NumberBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "Slider",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "NavigationView",
        member: "Slot.Content",
    },
    StructuralSurface {
        control: "NavigationView",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "NavigationView",
        member: "Slot.PaneCustomContent",
    },
    StructuralSurface {
        control: "NavigationView",
        member: "Slot.PaneFooter",
    },
    StructuralSurface {
        control: "NavigationView",
        member: "Slot.MenuItems",
    },
    StructuralSurface {
        control: "NavigationViewItem",
        member: "Slot.Content",
    },
    StructuralSurface {
        control: "NavigationViewItem",
        member: "Slot.Icon",
    },
    StructuralSurface {
        control: "NavigationViewItem",
        member: "Slot.MenuItems",
    },
    StructuralSurface {
        control: "SplitView",
        member: "Slot.Pane",
    },
    StructuralSurface {
        control: "SplitView",
        member: "Slot.Content",
    },
    StructuralSurface {
        control: "ToggleSwitch",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "ToggleSwitch",
        member: "Slot.OnContent",
    },
    StructuralSurface {
        control: "ToggleSwitch",
        member: "Slot.OffContent",
    },
    StructuralSurface {
        control: "CheckBox",
        member: "Content",
    },
    StructuralSurface {
        control: "ToggleButton",
        member: "Content",
    },
    StructuralSurface {
        control: "RadioButton",
        member: "Content",
    },
    StructuralSurface {
        control: "RadioButtons",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "ItemsRepeater",
        member: "Items",
    },
    StructuralSurface {
        control: "ScrollViewer",
        member: "Content",
    },
    StructuralSurface {
        control: "ScrollView",
        member: "Content",
    },
    StructuralSurface {
        control: "ListBox",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "ListBoxItem",
        member: "Content",
    },
    StructuralSurface {
        control: "Expander",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "Expander",
        member: "Slot.Content",
    },
    StructuralSurface {
        control: "ComboBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "Pivot",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "PivotItem",
        member: "Content",
    },
    StructuralSurface {
        control: "FlipView",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "SelectorBar",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "SelectorBarItem",
        member: "Slot.Icon",
    },
    StructuralSurface {
        control: "TabView",
        member: "Slot.TabItems",
    },
    StructuralSurface {
        control: "TabViewItem",
        member: "Content",
    },
    StructuralSurface {
        control: "DropDownButton",
        member: "Content",
    },
    StructuralSurface {
        control: "CommandBar",
        member: "Slot.PrimaryCommands",
    },
    StructuralSurface {
        control: "CommandBar",
        member: "Slot.SecondaryCommands",
    },
    StructuralSurface {
        control: "AppBarButton",
        member: "Slot.Icon",
    },
    StructuralSurface {
        control: "MenuBar",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "SplitButton",
        member: "Content",
    },
    StructuralSurface {
        control: "DatePicker",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "TimePicker",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "CalendarDatePicker",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "ContentDialog",
        member: "Content",
    },
    StructuralSurface {
        control: "ListView",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "ListViewItem",
        member: "Content",
    },
    StructuralSurface {
        control: "GridView",
        member: "Slot.Items",
    },
    StructuralSurface {
        control: "GridViewItem",
        member: "Content",
    },
    StructuralSurface {
        control: "RelativePanel",
        member: "Children",
    },
    StructuralSurface {
        control: "Canvas",
        member: "Children",
    },
    StructuralSurface {
        control: "RichEditBox",
        member: "Slot.Header",
    },
    StructuralSurface {
        control: "Viewbox",
        member: "Slot.Child",
    },
    StructuralSurface {
        control: "TitleBar",
        member: "Slot.Content",
    },
    StructuralSurface {
        control: "TitleBar",
        member: "Slot.RightHeader",
    },
];
pub static EXTENSION_SURFACES: &[ExtensionSurface] = &[
    ExtensionSurface { name: "Tooltip" },
    ExtensionSurface { name: "Flyout" },
    ExtensionSurface { name: "Menu" },
    ExtensionSurface {
        name: "CommandBarFlyout",
    },
    ExtensionSurface {
        name: "TreeView.Nodes",
    },
];
const _: [(); PROJECTED_CONTROL_COUNT
    + PROJECTED_PROPERTY_COUNT
    + PROJECTED_EVENT_COUNT
    + CAPABILITY_PROPERTY_COUNT
    + STRUCTURAL_COUNT
    + EXTENSION_COUNT] = [(); SURFACE_CASES.len()];
const _: [(); PROJECTED_PROPERTY_COUNT] = [(); PROJECTED_PROPERTIES.len()];
const _: [(); PROJECTED_EVENT_COUNT] = [(); PROJECTED_EVENTS.len()];
const _: [(); CAPABILITY_PROPERTY_COUNT] = [(); CAPABILITY_PROPERTIES.len()];
const _: [(); STRUCTURAL_COUNT] = [(); STRUCTURAL_SURFACES.len()];
const _: [(); EXTENSION_COUNT] = [(); EXTENSION_SURFACES.len()];
