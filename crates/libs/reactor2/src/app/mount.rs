use super::auto_suggest_box::*;
use super::breadcrumb_bar::*;
use super::collection::*;
use super::command::*;
use super::container::*;
use super::content::*;
use super::logical::*;
use super::media::*;
use super::menu::*;
use super::native_host::*;
use super::navigation::*;
use super::overlay::*;
use super::properties::*;
use super::selector::*;
use super::selector_bar::*;
use super::shape::*;
use super::status::*;
use super::text::*;
use super::value::*;
use super::window::{mount_application, mount_title_bar, mount_window};
use super::work::RenderServices;
use crate::element::Element;
use crate::element::tree::*;
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::runtime::*;

pub(crate) fn mount_element<R: NativeRuntime>(
    engine: &mut Engine<R>,
    element: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    #[cfg(test)]
    engine.record_element_created();
    let Element { key, kind } = element;
    let framework = element_framework(&kind);
    let control = element_control(&kind);
    let heap = diff_heap(HeapValues::default(), element_heap(&kind));
    let id = match kind {
        ElementKind::Application { windows, props } => {
            mount_application(engine, key, windows, props, services)?
        }
        ElementKind::Window(window) => mount_window(engine, key, *window, services)?,
        ElementKind::Component {
            identity,
            render,
            memo,
        } => mount_component(engine, key, identity, render, memo, services)?,
        ElementKind::Fragment { children } => mount_fragment(engine, key, children, services)?,
        ElementKind::StructuralSlot { slot, child } => {
            mount_structural_slot(engine, key, slot, *child, services)?
        }
        ElementKind::Context { props, child } => {
            mount_context(engine, key, props, *child, services)?
        }
        ElementKind::Reference { reference, child } => {
            mount_reference(engine, key, reference, *child, services)?
        }
        ElementKind::FadeTransition { child, enter, exit } => {
            mount_fade_transition(engine, key, *child, enter, exit, services)?
        }
        ElementKind::StackPanel(props) => mount_stack_panel(engine, key, props, services)?,
        ElementKind::Grid(props) => mount_grid(engine, key, props, services)?,
        ElementKind::TitleBar(title_bar) => mount_title_bar(engine, key, *title_bar, services)?,
        ElementKind::Canvas(props) => {
            mount_panel(engine, key, NativeKind::Canvas, props, services)?
        }
        ElementKind::RelativePanel(props) => {
            mount_panel(engine, key, NativeKind::RelativePanel, props, services)?
        }
        ElementKind::Viewbox { child, props } => {
            mount_viewbox(engine, key, *child, props, services)?
        }
        ElementKind::ScrollViewer { child, props } => {
            mount_scroll_viewer(engine, key, *child, props, services)?
        }
        ElementKind::ScrollView { child, props } => {
            mount_scroll_view(engine, key, *child, props, services)?
        }
        ElementKind::SplitView(split) => mount_split_view(engine, key, *split, services)?,
        ElementKind::Expander(expander) => mount_expander(engine, key, *expander, services)?,
        ElementKind::CommandBar(props) => mount_command_bar(engine, key, *props, services)?,
        ElementKind::CompositionHost(props) => mount_composition_host(engine, key, *props)?,
        #[cfg(feature = "webview")]
        ElementKind::WebViewHost(props) => mount_webview_host(engine, key, props)?,
        #[cfg(feature = "canvas")]
        ElementKind::CanvasImage(props) => mount_canvas_image(engine, key, props)?,
        #[cfg(feature = "canvas")]
        ElementKind::SwapChainCanvas(props) => mount_swap_chain_canvas(engine, key, props)?,
        #[cfg(feature = "canvas")]
        ElementKind::SwapChainHost(props) => mount_swap_chain_host(engine, key, props)?,
        ElementKind::Image(props) => mount_image(engine, key, props)?,
        ElementKind::Icon(icon) => mount_icon(engine, key, icon)?,
        ElementKind::Shape(props) => mount_shape(engine, key, *props)?,
        ElementKind::AppBarButton(props) => mount_app_bar_button(engine, key, props)?,
        ElementKind::AppBarToggleButton(props) => mount_app_bar_toggle_button(engine, key, props)?,
        ElementKind::AppBarSeparator => mount_app_bar_separator(engine, key)?,
        ElementKind::ContentDialog(dialog) => mount_content_dialog(engine, key, *dialog, services)?,
        ElementKind::TeachingTip(tip) => mount_teaching_tip(engine, key, *tip, services)?,
        ElementKind::ToolTip(tooltip) => mount_tooltip(engine, key, *tooltip, services)?,
        ElementKind::AttachedChild { placement, child } => {
            mount_attached_child(engine, key, placement, *child, services)?
        }
        ElementKind::Border(border) => mount_border(engine, key, *border, services)?,
        ElementKind::Button { child, props } => mount_button(engine, key, *child, props, services)?,
        ElementKind::ButtonFlyout { button, content } => {
            mount_button_flyout(engine, key, button, *content, services)?
        }
        ElementKind::ButtonMenuFlyout {
            button,
            label,
            flyout,
        } => mount_button_menu_flyout(engine, key, button, *label, flyout, services)?,
        ElementKind::ButtonCommandBarFlyout {
            button,
            label,
            flyout,
        } => mount_button_command_bar_flyout(engine, key, button, *label, *flyout, services)?,
        ElementKind::DropDownButton(drop_down) => {
            let DropDownButtonElement {
                label,
                flyout,
                props,
            } = *drop_down;
            match flyout {
                DropDownFlyoutElement::Content(flyout) => mount_drop_down_button(
                    engine,
                    key,
                    DropDownButtonElement {
                        label,
                        flyout: DropDownFlyoutElement::Content(flyout),
                        props,
                    },
                    services,
                )?,
                DropDownFlyoutElement::Menu(flyout) => {
                    mount_drop_down_menu_flyout(engine, key, props, *label, flyout, services)?
                }
            }
        }
        ElementKind::SplitButton { child, props } => {
            mount_split_button(engine, key, *child, props, services)?
        }
        ElementKind::SplitButtonFlyout { button, content } => {
            mount_split_button_flyout(engine, key, button, *content, services)?
        }
        ElementKind::HyperlinkButton { child, props } => {
            mount_hyperlink_button(engine, key, *child, props, services)?
        }
        ElementKind::RepeatButton { child, props } => {
            mount_repeat_button(engine, key, *child, props, services)?
        }
        ElementKind::ToggleButton { child, props } => {
            mount_toggle_button(engine, key, *child, props, services)?
        }
        ElementKind::ToggleSwitch(props) => mount_toggle_switch(engine, key, props)?,
        ElementKind::InfoBadge(props) => mount_info_badge(engine, key, props)?,
        ElementKind::InfoBar(props) => mount_info_bar(engine, key, *props)?,
        ElementKind::PersonPicture(props) => mount_person_picture(engine, key, *props)?,
        ElementKind::ProgressBar(props) => mount_progress_bar(engine, key, props)?,
        ElementKind::ProgressRing(props) => mount_progress_ring(engine, key, props)?,
        ElementKind::Slider(props) => mount_slider(engine, key, props)?,
        ElementKind::NumberBox(props) => mount_number_box(engine, key, props)?,
        ElementKind::RatingControl(props) => mount_rating_control(engine, key, props)?,
        ElementKind::ColorPicker(props) => mount_color_picker(engine, key, props)?,
        ElementKind::DatePicker(props) => mount_date_picker(engine, key, props)?,
        ElementKind::CalendarDatePicker(props) => mount_calendar_date_picker(engine, key, props)?,
        ElementKind::TimePicker(props) => mount_time_picker(engine, key, props)?,
        ElementKind::CalendarView(props) => mount_calendar_view(engine, key, props)?,
        ElementKind::RichEditBox(props) => mount_rich_edit_box(engine, key, *props)?,
        ElementKind::RichTextBlock(props) => mount_rich_text_block(engine, key, *props)?,
        ElementKind::TreeView(props) => mount_tree_view(engine, key, props)?,
        ElementKind::NavigationView(value) => mount_navigation_view(engine, key, *value, services)?,
        ElementKind::NavigationViewItem(props) => mount_navigation_view_item(engine, key, props)?,
        ElementKind::CheckBox { child, props } => {
            mount_check_box(engine, key, *child, props, services)?
        }
        ElementKind::RadioButton { child, props } => {
            mount_radio_button(engine, key, *child, props, services)?
        }
        ElementKind::TextBlock(props) => mount_text_block(engine, key, props)?,
        ElementKind::TextBox(props) => mount_text_box(engine, key, *props)?,
        ElementKind::PasswordBox(props) => mount_password_box(engine, key, props)?,
        ElementKind::ListBox(props) => mount_list_box(engine, key, props)?,
        ElementKind::ComboBox(props) => mount_combo_box(engine, key, props)?,
        ElementKind::RadioButtons(props) => mount_radio_buttons(engine, key, props)?,
        ElementKind::MenuBar(props) => mount_menu_bar(engine, key, props)?,
        ElementKind::FlipView(props) => mount_flip_view(engine, key, *props, services)?,
        ElementKind::TabView(props) => mount_tab_view(engine, key, *props, services)?,
        ElementKind::TabViewItem { child, props } => {
            mount_tab_view_item(engine, key, *child, props, services)?
        }
        ElementKind::SelectorBar(props) => mount_selector_bar(engine, key, *props, services)?,
        ElementKind::SelectorBarItem(props) => mount_selector_bar_item(engine, key, props)?,
        ElementKind::BreadcrumbBar(props) => mount_breadcrumb_bar(engine, key, props)?,
        ElementKind::AutoSuggestBox(props) => mount_auto_suggest_box(engine, key, props)?,
        ElementKind::Pivot(props) => mount_pivot(engine, key, *props, services)?,
        ElementKind::PivotItem { child, props } => {
            mount_pivot_item(engine, key, *child, props, services)?
        }
        ElementKind::VirtualCollection(props) => {
            mount_virtual_collection(engine, key, *props, services)?
        }
    };
    if framework != FrameworkValues::default() {
        let target = engine.single_projected_native_root(id).unwrap();
        apply_framework_props(engine, target, &framework)?;
    }
    if let Some(enabled) = control.props.enabled() {
        let target = engine.single_projected_native_root(id).unwrap();
        engine.queue_framework_update(target, FrameworkUpdate::Enabled(enabled))?;
    }
    if !heap.is_empty() {
        let target = engine.single_projected_native_root(id).unwrap();
        apply_heap_changes(engine, target, heap)?;
    }
    Ok(id)
}
