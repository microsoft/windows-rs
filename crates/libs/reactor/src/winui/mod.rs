mod auto_suggest_box;
mod bootstrap;
mod breadcrumb_bar;
#[cfg(feature = "canvas")]
mod canvas;
mod collection;
mod command;
mod container;
mod controlled;
mod conversion;
mod deferred;
mod host;
mod input;
mod lifecycle;
mod media;
mod menu;
mod native_host;
mod navigation;
mod overlay;
mod rich;
mod selector;
mod selector_bar;
mod shape;
mod state;
mod status;
mod tree;
mod updates;
#[cfg(feature = "webview")]
mod webview;
mod window;

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
use std::time::Duration;

use windows_core::{Interface, Result as WindowsResult};
use windows_time::{DateTime, TimeSpan};

use crate::bindings;
use crate::element::*;
use crate::id::NodeId;
use crate::runtime::*;

pub use self::bootstrap::bootstrap;
use self::conversion::*;
pub use self::host::{run_reactor_winui, run_reactor_winui_app};
#[cfg(test)]
pub(crate) use self::host::{run_reactor_winui_performance, terminate_host};
pub use self::state::WinUiRuntime;
use self::state::{Handle, NativeNode, NativeTimer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LatestEventSlot {
    TextChanged,
    SelectionChanged,
    SelectedKeyChanged,
    IndexChanged,
    TabsReordered,
    ItemsReordered,
    #[cfg(feature = "canvas")]
    CanvasLayout,
    #[cfg(feature = "canvas")]
    CanvasImageLayout,
    #[cfg(feature = "canvas")]
    SwapChainHostLayout,
    CompositionLayout,
}

fn latest_event_slot(event: &NativeEvent) -> Option<(NodeId, LatestEventSlot)> {
    match event {
        NativeEvent::TextChanged { target, .. } => Some((*target, LatestEventSlot::TextChanged)),
        NativeEvent::SelectionChanged { target, .. } => {
            Some((*target, LatestEventSlot::SelectionChanged))
        }
        NativeEvent::SelectedKeyChanged { target, .. } => {
            Some((*target, LatestEventSlot::SelectedKeyChanged))
        }
        NativeEvent::IndexChanged { target, .. } => Some((*target, LatestEventSlot::IndexChanged)),
        NativeEvent::TabsReordered { target, .. } => {
            Some((*target, LatestEventSlot::TabsReordered))
        }
        NativeEvent::ItemsReordered { target, .. } => {
            Some((*target, LatestEventSlot::ItemsReordered))
        }
        #[cfg(feature = "canvas")]
        NativeEvent::CanvasLayout { target, .. } => Some((*target, LatestEventSlot::CanvasLayout)),
        #[cfg(feature = "canvas")]
        NativeEvent::CanvasImageLayout { target, .. } => {
            Some((*target, LatestEventSlot::CanvasImageLayout))
        }
        #[cfg(feature = "canvas")]
        NativeEvent::SwapChainHostLayout { target, .. } => {
            Some((*target, LatestEventSlot::SwapChainHostLayout))
        }
        NativeEvent::CompositionLayout { target, .. } => {
            Some((*target, LatestEventSlot::CompositionLayout))
        }
        _ => None,
    }
}

fn remove_queued_event(
    events: &RefCell<VecDeque<NativeEvent>>,
    target: NodeId,
    slot: LatestEventSlot,
) {
    events
        .borrow_mut()
        .retain(|event| latest_event_slot(event) != Some((target, slot)));
}

fn queue_latest_event(events: &RefCell<VecDeque<NativeEvent>>, event: NativeEvent) {
    let (target, slot) = latest_event_slot(&event).unwrap();
    remove_queued_event(events, target, slot);
    events.borrow_mut().push_back(event);
}

#[cfg(test)]
#[path = "../../testing/private/winui/mod.rs"]
mod tests;

fn subscribe_click(
    button: &bindings::IButtonBase,
    target: NodeId,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<windows_core::EventRevoker> {
    button.Click(move |_sender, _args| {
        events.borrow_mut().push_back(NativeEvent::Click { target });
        if let Some(wake) = waker.borrow().as_ref() {
            wake();
        }
    })
}

fn subscribe_toggle(
    toggle: &bindings::IToggleButton,
    target: NodeId,
    expected: &Rc<Cell<bool>>,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<[windows_core::EventRevoker; 2]> {
    let checked_expected = Rc::clone(expected);
    let checked_events = Rc::clone(&events);
    let checked_waker = Rc::clone(&waker);
    let checked = toggle.Checked(move |_sender, _args| {
        if checked_expected.replace(true) {
            return;
        }
        checked_events.borrow_mut().push_back(NativeEvent::Toggled {
            target,
            value: true,
        });
        if let Some(wake) = checked_waker.borrow().as_ref() {
            wake();
        }
    })?;
    let unchecked_expected = Rc::clone(expected);
    let unchecked = toggle.Unchecked(move |_sender, _args| {
        if !unchecked_expected.replace(false) {
            return;
        }
        events.borrow_mut().push_back(NativeEvent::Toggled {
            target,
            value: false,
        });
        if let Some(wake) = waker.borrow().as_ref() {
            wake();
        }
    })?;
    Ok([checked, unchecked])
}

fn subscribe_toggle_switch(
    toggle: &bindings::IToggleSwitch,
    target: NodeId,
    expected: &Rc<Cell<bool>>,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<windows_core::EventRevoker> {
    let control = toggle.clone();
    let expected = Rc::clone(expected);
    toggle.Toggled(move |_sender, _args| {
        let value = control.IsOn().unwrap();
        if expected.replace(value) == value {
            return;
        }
        events
            .borrow_mut()
            .push_back(NativeEvent::Toggled { target, value });
        if let Some(wake) = waker.borrow().as_ref() {
            wake();
        }
    })
}

impl WinUiRuntime {
    fn composition_visual(&self, id: NodeId) -> WindowsResult<windows_composition::Visual> {
        let element = self.node(id)?.handle.ui_element()?;
        let visual = bindings::ElementCompositionPreview::GetElementVisual(&element)?;
        windows_composition::Visual::from_host(visual.into())
    }

    fn apply_visual_update(&self, id: NodeId, update: VisualUpdate) -> WindowsResult<()> {
        let visual = self.composition_visual(id)?;
        match update {
            VisualUpdate::ImplicitTransitions(transitions) => {
                if transitions.is_empty() {
                    visual.set_implicit_animations(None);
                    return Ok(());
                }
                let compositor = visual.compositor();
                let collection = compositor.create_implicit_animation_collection();
                let easing = compositor.create_cubic_bezier_easing_function(
                    windows_numerics::Vector2 { x: 0.0, y: 0.0 },
                    windows_numerics::Vector2 { x: 0.58, y: 1.0 },
                );
                if let Some(duration) = transitions.opacity {
                    let animation = compositor.create_scalar_key_frame_animation();
                    animation.set_duration(duration);
                    animation.insert_expression_key_frame_with_easing(
                        1.0,
                        "this.FinalValue",
                        &easing,
                    );
                    animation.set_target("Opacity");
                    collection.insert("Opacity", &animation);
                }
                if let Some(duration) = transitions.scale {
                    let animation = compositor.create_vector3_key_frame_animation();
                    animation.set_duration(duration);
                    animation.insert_expression_key_frame_with_easing(
                        1.0,
                        "this.FinalValue",
                        &easing,
                    );
                    animation.set_target("Scale");
                    collection.insert("Scale", &animation);
                }
                visual.set_implicit_animations(Some(&collection));
                Ok(())
            }
            VisualUpdate::Scale(value) => {
                let scale = value.unwrap_or(1.0);
                assert!(
                    scale.is_finite() && scale >= 0.0,
                    "scale must be finite and non-negative"
                );
                let element = self.node(id)?.handle.framework_element()?;
                let width = element.ActualWidth()? as f32;
                let height = element.ActualHeight()? as f32;
                if width > 0.0 && height > 0.0 {
                    visual.set_center_point(windows_numerics::Vector3 {
                        x: width / 2.0,
                        y: height / 2.0,
                        z: 0.0,
                    });
                }
                visual.set_scale(windows_numerics::Vector3 {
                    x: scale,
                    y: scale,
                    z: 1.0,
                });
                Ok(())
            }
            VisualUpdate::FadeTo { opacity, duration } => {
                let compositor = visual.compositor();
                let animation = compositor.create_scalar_key_frame_animation();
                animation.set_duration(duration);
                let easing = compositor.create_cubic_bezier_easing_function(
                    windows_numerics::Vector2 { x: 0.0, y: 0.0 },
                    windows_numerics::Vector2 { x: 0.58, y: 1.0 },
                );
                animation.insert_key_frame_with_easing(1.0, opacity, &easing);
                visual.start_animation("Opacity", &animation);
                Ok(())
            }
        }
    }

    fn apply_element_resources(
        &mut self,
        id: NodeId,
        resources: &ElementResources,
    ) -> WindowsResult<()> {
        let map = self
            .node(id)?
            .handle
            .framework_element()?
            .Resources()?
            .cast::<windows_collections::IMap<
            windows_core::IInspectable,
            windows_core::IInspectable,
        >>()?;
        if let Some(previous) = self.element_resources.get(&id) {
            for (key, _) in previous.entries() {
                if resources.get(key).is_none() {
                    let key = windows_reference::IReference::from(key);
                    if map.HasKey(&key)? {
                        map.Remove(&key)?;
                    }
                }
            }
        }
        for (key, value) in resources.entries() {
            if self
                .element_resources
                .get(&id)
                .and_then(|previous| previous.get(key))
                != Some(value)
            {
                let key = windows_reference::IReference::from(key);
                map.Insert(&key, &application_resource_value(value)?)?;
            }
        }
        if resources.is_empty() {
            self.element_resources.remove(&id);
        } else {
            self.element_resources.insert(id, resources.clone());
        }
        Ok(())
    }

    fn create_title_bar(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TitleBar::new()?;
        let back_events = Rc::clone(&self.events);
        let back_waker = Rc::clone(&self.waker);
        let back = value.BackRequested(move |_sender, _args| {
            back_events
                .borrow_mut()
                .push_back(NativeEvent::TitleBarBackRequested { target: id });
            if let Some(wake) = back_waker.borrow().as_ref() {
                wake();
            }
        })?;
        let pane_events = Rc::clone(&self.events);
        let pane_waker = Rc::clone(&self.waker);
        let pane = value.PaneToggleRequested(move |_sender, _args| {
            pane_events
                .borrow_mut()
                .push_back(NativeEvent::TitleBarPaneRequested { target: id });
            if let Some(wake) = pane_waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::TitleBar {
            _revokers: [back, pane],
            value,
        })
    }

    fn create(&mut self, id: NodeId, kind: NativeKind) -> WindowsResult<()> {
        assert!(!self.nodes.contains_key(&id), "native node already exists");
        let handle = match kind {
            NativeKind::Border => Handle::Border(bindings::Border::new()?),
            NativeKind::Button => {
                let button = bindings::Button::new()?;
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let button_base: bindings::IButtonBase = button.cast()?;
                let revoker = subscribe_click(&button_base, id, events, waker)?;
                Handle::Button {
                    _revoker: revoker,
                    value: button,
                }
            }
            NativeKind::CommandBar => self.create_command_bar()?,
            NativeKind::CommandBarFlyout => self.create_command_bar_flyout(id)?,
            NativeKind::AppBarButton => self.create_app_bar_button(id)?,
            NativeKind::AppBarToggleButton => self.create_app_bar_toggle_button(id)?,
            NativeKind::AppBarSeparator => self.create_app_bar_separator()?,
            NativeKind::Image => self.create_image()?,
            NativeKind::SymbolIcon => {
                Handle::SymbolIcon(bindings::SymbolIcon::CreateInstanceWithSymbol(
                    bindings::Symbol(IconSymbol::ADD.value()),
                )?)
            }
            NativeKind::FontIcon => Handle::FontIcon(bindings::FontIcon::new()?),
            NativeKind::BitmapIcon => Handle::BitmapIcon(bindings::BitmapIcon::new()?),
            NativeKind::ImageIcon => Handle::ImageIcon(bindings::ImageIcon::new()?),
            NativeKind::PathIcon => Handle::PathIcon(bindings::PathIcon::new()?),
            NativeKind::Rectangle => Handle::Rectangle(bindings::Rectangle::new()?),
            NativeKind::Ellipse => Handle::Ellipse(bindings::Ellipse::new()?),
            NativeKind::Line => Handle::Line(bindings::Line::new()?),
            NativeKind::DropDownButton => Handle::DropDownButton(bindings::DropDownButton::new()?),
            NativeKind::SplitButton => {
                let button = bindings::SplitButton::new()?;
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let revoker = button.Click(move |_sender, _args| {
                    events
                        .borrow_mut()
                        .push_back(NativeEvent::Click { target: id });
                    if let Some(wake) = waker.borrow().as_ref() {
                        wake();
                    }
                })?;
                Handle::SplitButton {
                    _revoker: revoker,
                    value: button,
                }
            }
            NativeKind::Flyout => self.create_flyout(id)?,
            NativeKind::MenuFlyout => self.create_menu_flyout(id)?,
            NativeKind::MenuBar => self.create_menu_bar(id)?,
            NativeKind::ContentDialog => self.create_content_dialog(id)?,
            NativeKind::HyperlinkButton => {
                let button = bindings::HyperlinkButton::new()?;
                let button_base: bindings::IButtonBase = button.cast()?;
                let revoker = subscribe_click(
                    &button_base,
                    id,
                    Rc::clone(&self.events),
                    Rc::clone(&self.waker),
                )?;
                Handle::HyperlinkButton {
                    _revoker: revoker,
                    value: button,
                }
            }
            NativeKind::RepeatButton => {
                let button = bindings::RepeatButton::new()?;
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let button_base: bindings::IButtonBase = button.cast()?;
                let revoker = subscribe_click(&button_base, id, events, waker)?;
                Handle::RepeatButton {
                    _revoker: revoker,
                    value: button,
                }
            }
            NativeKind::Canvas => Handle::Canvas(bindings::Canvas::new()?),
            NativeKind::CompositionHost => self.create_composition_host(id)?,
            #[cfg(feature = "canvas")]
            NativeKind::CanvasImage => self.create_canvas_image(id)?,
            #[cfg(feature = "canvas")]
            NativeKind::SwapChainCanvas => self.create_swap_chain_canvas(id)?,
            #[cfg(feature = "canvas")]
            NativeKind::SwapChainHost => self.create_swap_chain_host(id)?,
            NativeKind::CheckBox => self.create_check_box(id)?,
            NativeKind::RadioButton => self.create_radio_button(id)?,
            NativeKind::ToggleButton => self.create_toggle_button(id)?,
            NativeKind::ToggleSwitch => self.create_toggle_switch(id)?,
            NativeKind::InfoBadge => Handle::InfoBadge(bindings::InfoBadge::new()?),
            NativeKind::InfoBar => self.create_info_bar(id)?,
            NativeKind::PersonPicture => Handle::PersonPicture(bindings::PersonPicture::new()?),
            NativeKind::ProgressBar => Handle::ProgressBar(bindings::ProgressBar::new()?),
            NativeKind::ProgressRing => Handle::ProgressRing(bindings::ProgressRing::new()?),
            NativeKind::Slider => self.create_slider(id)?,
            NativeKind::NumberBox => self.create_number_box(id)?,
            NativeKind::RatingControl => self.create_rating_control(id)?,
            NativeKind::ColorPicker => self.create_color_picker(id)?,
            NativeKind::DatePicker => self.create_date_picker(id)?,
            NativeKind::CalendarDatePicker => self.create_calendar_date_picker(id)?,
            NativeKind::TimePicker => self.create_time_picker(id)?,
            NativeKind::CalendarView => self.create_calendar_view(id)?,
            NativeKind::NavigationView => self.create_navigation_view(id)?,
            NativeKind::NavigationViewItem => self.create_navigation_view_item()?,
            NativeKind::ListBox => self.create_list_box(id)?,
            NativeKind::ComboBox => self.create_combo_box(id)?,
            NativeKind::RadioButtons => self.create_radio_buttons(id)?,
            NativeKind::FlipView => self.create_flip_view(id)?,
            NativeKind::TabView => self.create_tab_view(id)?,
            NativeKind::TabViewItem => self.create_tab_view_item()?,
            NativeKind::SelectorBar => self.create_selector_bar(id)?,
            NativeKind::SelectorBarItem => self.create_selector_bar_item()?,
            NativeKind::BreadcrumbBar => self.create_breadcrumb_bar(id)?,
            NativeKind::AutoSuggestBox => self.create_auto_suggest_box(id)?,
            NativeKind::Pivot => self.create_pivot(id)?,
            NativeKind::PivotItem => Handle::PivotItem(bindings::PivotItem::new()?),
            NativeKind::ListView | NativeKind::GridView => {
                self.create_virtual_collection(id, kind)?
            }
            NativeKind::StackPanel => Handle::StackPanel(bindings::StackPanel::new()?),
            NativeKind::ScrollViewer => self.create_scroll_viewer()?,
            NativeKind::ScrollView => self.create_scroll_view()?,
            NativeKind::SplitView => self.create_split_view(id)?,
            NativeKind::Expander => self.create_expander(id)?,
            NativeKind::TeachingTip => self.create_teaching_tip(id)?,
            NativeKind::TitleBar => self.create_title_bar(id)?,
            NativeKind::Grid => Handle::Grid(bindings::Grid::new()?),
            NativeKind::RelativePanel => Handle::RelativePanel(bindings::RelativePanel::new()?),
            NativeKind::TextBlock => Handle::TextBlock(bindings::TextBlock::new()?),
            NativeKind::RichEditBox => self.create_rich_edit_box(id)?,
            NativeKind::RichTextBlock => Handle::RichTextBlock(bindings::RichTextBlock::new()?),
            NativeKind::TreeView => self.create_tree_view(id)?,
            NativeKind::TextBox => self.create_text_box(id)?,
            NativeKind::PasswordBox => self.create_password_box(id)?,
            NativeKind::ToolTip => Handle::ToolTip(bindings::ToolTip::new()?),
            NativeKind::Viewbox => Handle::Viewbox(bindings::Viewbox::new()?),
            #[cfg(feature = "webview")]
            NativeKind::WebViewHost => self.create_webview_host(id)?,
        };
        self.nodes.insert(
            id,
            NativeNode {
                handle,
                parent: None,
                attachment: None,
                children: Vec::new(),
                input: None,
            },
        );
        Ok(())
    }

    fn apply_attached_update(&mut self, id: NodeId, update: AttachedUpdate) -> WindowsResult<()> {
        let element = self.node(id)?.handle.framework_element()?;
        let dependency = self.node(id)?.handle.dependency_object()?;

        (|| -> WindowsResult<()> {
            match update {
                AttachedUpdate::Row(Some(value)) => bindings::Grid::SetRow(&element, value),
                AttachedUpdate::Column(Some(value)) => bindings::Grid::SetColumn(&element, value),
                AttachedUpdate::RowSpan(Some(value)) => bindings::Grid::SetRowSpan(&element, value),
                AttachedUpdate::ColumnSpan(Some(value)) => {
                    bindings::Grid::SetColumnSpan(&element, value)
                }
                AttachedUpdate::Row(None) => dependency.ClearValue(&bindings::Grid::RowProperty()?),
                AttachedUpdate::Column(None) => {
                    dependency.ClearValue(&bindings::Grid::ColumnProperty()?)
                }
                AttachedUpdate::RowSpan(None) => {
                    dependency.ClearValue(&bindings::Grid::RowSpanProperty()?)
                }
                AttachedUpdate::ColumnSpan(None) => {
                    dependency.ClearValue(&bindings::Grid::ColumnSpanProperty()?)
                }
                AttachedUpdate::CanvasLeft(Some(value)) => {
                    bindings::Canvas::SetLeft(&element, value)
                }
                AttachedUpdate::CanvasTop(Some(value)) => bindings::Canvas::SetTop(&element, value),
                AttachedUpdate::CanvasZIndex(Some(value)) => {
                    bindings::Canvas::SetZIndex(&element, value)
                }
                AttachedUpdate::CanvasLeft(None) => {
                    dependency.ClearValue(&bindings::Canvas::LeftProperty()?)
                }
                AttachedUpdate::CanvasTop(None) => {
                    dependency.ClearValue(&bindings::Canvas::TopProperty()?)
                }
                AttachedUpdate::CanvasZIndex(None) => {
                    dependency.ClearValue(&bindings::Canvas::ZIndexProperty()?)
                }
                AttachedUpdate::RelativeAlignLeft(Some(value)) => {
                    bindings::RelativePanel::SetAlignLeftWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignRight(Some(value)) => {
                    bindings::RelativePanel::SetAlignRightWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignTop(Some(value)) => {
                    bindings::RelativePanel::SetAlignTopWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignBottom(Some(value)) => {
                    bindings::RelativePanel::SetAlignBottomWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignHorizontalCenter(Some(value)) => {
                    bindings::RelativePanel::SetAlignHorizontalCenterWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignVerticalCenter(Some(value)) => {
                    bindings::RelativePanel::SetAlignVerticalCenterWithPanel(&element, value)
                }
                AttachedUpdate::RelativeAlignLeft(None) => {
                    dependency.ClearValue(&bindings::RelativePanel::AlignLeftWithPanelProperty()?)
                }
                AttachedUpdate::RelativeAlignRight(None) => {
                    dependency.ClearValue(&bindings::RelativePanel::AlignRightWithPanelProperty()?)
                }
                AttachedUpdate::RelativeAlignTop(None) => {
                    dependency.ClearValue(&bindings::RelativePanel::AlignTopWithPanelProperty()?)
                }
                AttachedUpdate::RelativeAlignBottom(None) => {
                    dependency.ClearValue(&bindings::RelativePanel::AlignBottomWithPanelProperty()?)
                }
                AttachedUpdate::RelativeAlignHorizontalCenter(None) => dependency.ClearValue(
                    &bindings::RelativePanel::AlignHorizontalCenterWithPanelProperty()?,
                ),
                AttachedUpdate::RelativeAlignVerticalCenter(None) => dependency
                    .ClearValue(&bindings::RelativePanel::AlignVerticalCenterWithPanelProperty()?),
                AttachedUpdate::TooltipPlacement(Some(value)) => {
                    bindings::ToolTipService::SetPlacement(
                        &element,
                        native_tooltip_placement(value),
                    )
                }
                AttachedUpdate::TooltipPlacement(None) => {
                    dependency.ClearValue(&bindings::ToolTipService::PlacementProperty()?)
                }
            }
        })()
    }

    fn apply_text_style_update(
        &mut self,
        id: NodeId,
        update: &TextStyleUpdate,
    ) -> WindowsResult<()> {
        let node = self.node(id)?;

        (|| -> WindowsResult<()> {
            match update {
                TextStyleUpdate::FontFamily(None) => node.handle.clear_text_property(
                    bindings::Control::FontFamilyProperty,
                    bindings::TextBlock::FontFamilyProperty,
                ),
                TextStyleUpdate::FontFamily(Some(value)) => {
                    let family = bindings::FontFamily::CreateInstanceWithName(value)?;
                    match &node.handle {
                        Handle::TextBlock(text) => text.SetFontFamily(&family),
                        handle => handle.control()?.SetFontFamily(&family),
                    }
                }
                TextStyleUpdate::Foreground(None) => node.handle.clear_text_property(
                    bindings::Control::ForegroundProperty,
                    bindings::TextBlock::ForegroundProperty,
                ),
                TextStyleUpdate::Foreground(Some(value)) => {
                    let brush = native_brush(value)?;
                    match &node.handle {
                        Handle::TextBlock(text) => text.SetForeground(&brush),
                        handle => handle.control()?.SetForeground(&brush),
                    }
                }
            }
        })()
    }

    fn apply_accessibility_update(
        &mut self,
        id: NodeId,
        update: &AccessibilityUpdate,
    ) -> WindowsResult<()> {
        let element = self.node(id)?.handle.dependency_object()?;
        match update {
            AccessibilityUpdate::AutomationName(value) => {
                bindings::AutomationProperties::SetName(&element, value)
            }
            AccessibilityUpdate::AutomationId(value) => {
                bindings::AutomationProperties::SetAutomationId(&element, value)
            }
            AccessibilityUpdate::HeadingLevel(value) => {
                bindings::AutomationProperties::SetHeadingLevel(
                    &element,
                    match value {
                        Some(AutomationHeadingLevel::Level1) => {
                            bindings::AutomationHeadingLevel::Level1
                        }
                        Some(AutomationHeadingLevel::Level2) => {
                            bindings::AutomationHeadingLevel::Level2
                        }
                        Some(AutomationHeadingLevel::Level3) => {
                            bindings::AutomationHeadingLevel::Level3
                        }
                        Some(AutomationHeadingLevel::Level4) => {
                            bindings::AutomationHeadingLevel::Level4
                        }
                        Some(AutomationHeadingLevel::Level5) => {
                            bindings::AutomationHeadingLevel::Level5
                        }
                        Some(AutomationHeadingLevel::Level6) => {
                            bindings::AutomationHeadingLevel::Level6
                        }
                        Some(AutomationHeadingLevel::Level7) => {
                            bindings::AutomationHeadingLevel::Level7
                        }
                        Some(AutomationHeadingLevel::Level8) => {
                            bindings::AutomationHeadingLevel::Level8
                        }
                        Some(AutomationHeadingLevel::Level9) => {
                            bindings::AutomationHeadingLevel::Level9
                        }
                        None => bindings::AutomationHeadingLevel::None,
                    },
                )
            }
            AccessibilityUpdate::HelpText(value) => {
                bindings::AutomationProperties::SetHelpText(&element, value)
            }
        }
    }

    fn apply_framework_update(&mut self, id: NodeId, update: FrameworkUpdate) -> WindowsResult<()> {
        if let FrameworkUpdate::Enabled(value) = update {
            return self
                .node(id)?
                .handle
                .control()
                .and_then(|control| control.SetIsEnabled(value));
        }
        if let FrameworkUpdate::Visibility(value) = update {
            return self.node(id)?.handle.ui_element().and_then(|element| {
                element
                    .SetVisibility(value.map_or(bindings::Visibility::Visible, native_visibility))
            });
        }
        if let FrameworkUpdate::Opacity(value) = update {
            return self
                .node(id)?
                .handle
                .ui_element()
                .and_then(|element| element.SetOpacity(f64::from(value.unwrap_or(1.0))));
        }
        if let FrameworkUpdate::FontSize(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match (&node.handle, value) {
                    (Handle::TextBlock(text), Some(value)) => text.SetFontSize(f64::from(value)),
                    (handle, Some(value)) => handle.control()?.SetFontSize(f64::from(value)),
                    (handle, None) => handle.clear_text_property(
                        bindings::Control::FontSizeProperty,
                        bindings::TextBlock::FontSizeProperty,
                    ),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::CharacterSpacing(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match (&node.handle, value) {
                    (Handle::TextBlock(text), Some(value)) => text.SetCharacterSpacing(value),
                    (handle, Some(value)) => handle.control()?.SetCharacterSpacing(value),
                    (handle, None) => handle.clear_text_property(
                        bindings::Control::CharacterSpacingProperty,
                        bindings::TextBlock::CharacterSpacingProperty,
                    ),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::FontWeight(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match (&node.handle, value) {
                    (Handle::TextBlock(text), Some(value)) => {
                        text.SetFontWeight(bindings::FontWeight {
                            weight: value.weight(),
                        })
                    }
                    (handle, Some(value)) => {
                        handle.control()?.SetFontWeight(bindings::FontWeight {
                            weight: value.weight(),
                        })
                    }
                    (handle, None) => handle.clear_text_property(
                        bindings::Control::FontWeightProperty,
                        bindings::TextBlock::FontWeightProperty,
                    ),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::FontStyle(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match (&node.handle, value) {
                    (Handle::TextBlock(text), Some(value)) => {
                        text.SetFontStyle(bindings::FontStyle(value as i32))
                    }
                    (handle, Some(value)) => handle
                        .control()?
                        .SetFontStyle(bindings::FontStyle(value as i32)),
                    (handle, None) => handle.clear_text_property(
                        bindings::Control::FontStyleProperty,
                        bindings::TextBlock::FontStyleProperty,
                    ),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::FontStretch(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match (&node.handle, value) {
                    (Handle::TextBlock(text), Some(value)) => {
                        text.SetFontStretch(bindings::FontStretch(value as i32))
                    }
                    (handle, Some(value)) => handle
                        .control()?
                        .SetFontStretch(bindings::FontStretch(value as i32)),
                    (handle, None) => handle.clear_text_property(
                        bindings::Control::FontStretchProperty,
                        bindings::TextBlock::FontStretchProperty,
                    ),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::TextWrapping(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match value {
                    Some(value) => node
                        .handle
                        .text_block()?
                        .SetTextWrapping(bindings::TextWrapping(value as i32)),
                    None => node
                        .handle
                        .dependency_object()?
                        .ClearValue(&bindings::TextBlock::TextWrappingProperty()?),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::TextTrimming(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match value {
                    Some(value) => node
                        .handle
                        .text_block()?
                        .SetTextTrimming(bindings::TextTrimming(value as i32)),
                    None => node
                        .handle
                        .dependency_object()?
                        .ClearValue(&bindings::TextBlock::TextTrimmingProperty()?),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::TextSelectionEnabled(value) = update {
            let node = self.node(id)?;
            let result = (|| -> WindowsResult<()> {
                match value {
                    Some(value) => node.handle.text_block()?.SetIsTextSelectionEnabled(value),
                    None => node
                        .handle
                        .dependency_object()?
                        .ClearValue(&bindings::TextBlock::IsTextSelectionEnabledProperty()?),
                }
            })();
            return result;
        }
        if let FrameworkUpdate::Padding(value) = update {
            let node = self.node(id)?;
            let value = if value.left.is_nan() {
                bindings::Thickness::default()
            } else {
                native_thickness(value)
            };
            let result = match &node.handle {
                Handle::StackPanel(control) => control.SetPadding(value),
                Handle::TextBlock(control) => control.SetPadding(value),
                _ => unreachable!(),
            };
            return result;
        }
        let element = self.node(id)?.handle.framework_element()?;
        match update {
            FrameworkUpdate::Width(value) => element.SetWidth(dimension(value, f64::NAN)),
            FrameworkUpdate::Height(value) => element.SetHeight(dimension(value, f64::NAN)),
            FrameworkUpdate::MinWidth(value) => element.SetMinWidth(dimension(value, 0.0)),
            FrameworkUpdate::MaxWidth(value) => {
                element.SetMaxWidth(dimension(value, f64::INFINITY))
            }
            FrameworkUpdate::MinHeight(value) => element.SetMinHeight(dimension(value, 0.0)),
            FrameworkUpdate::MaxHeight(value) => {
                element.SetMaxHeight(dimension(value, f64::INFINITY))
            }
            FrameworkUpdate::Margin(value) => {
                element.SetMargin(value.map_or_else(bindings::Thickness::default, native_thickness))
            }
            FrameworkUpdate::HorizontalAlignment(value) => {
                element.SetHorizontalAlignment(value.map_or(
                    bindings::HorizontalAlignment::Stretch,
                    native_horizontal_alignment,
                ))
            }
            FrameworkUpdate::VerticalAlignment(value) => {
                element.SetVerticalAlignment(value.map_or(
                    bindings::VerticalAlignment::Stretch,
                    native_vertical_alignment,
                ))
            }
            FrameworkUpdate::Opacity(_)
            | FrameworkUpdate::Visibility(_)
            | FrameworkUpdate::Enabled(_)
            | FrameworkUpdate::Padding(_)
            | FrameworkUpdate::FontSize(_)
            | FrameworkUpdate::CharacterSpacing(_)
            | FrameworkUpdate::FontStretch(_)
            | FrameworkUpdate::FontStyle(_)
            | FrameworkUpdate::FontWeight(_)
            | FrameworkUpdate::TextSelectionEnabled(_)
            | FrameworkUpdate::TextTrimming(_)
            | FrameworkUpdate::TextWrapping(_) => unreachable!(),
        }
    }

    fn apply_title_bar_update(&self, id: NodeId, update: &TitleBarUpdate) -> WindowsResult<()> {
        let Handle::TitleBar { value, .. } = &self.node(id)?.handle else {
            panic!("TitleBar update target is not a TitleBar");
        };
        match update {
            TitleBarUpdate::Title(title) => value.SetTitle(title.as_deref().unwrap_or_default()),
            TitleBarUpdate::Subtitle(subtitle) => {
                value.SetSubtitle(subtitle.as_deref().unwrap_or_default())
            }
            TitleBarUpdate::BackButtonVisible(visible) => value.SetIsBackButtonVisible(*visible),
            TitleBarUpdate::BackButtonEnabled(enabled) => value.SetIsBackButtonEnabled(*enabled),
            TitleBarUpdate::PaneToggleButtonVisible(visible) => {
                value.SetIsPaneToggleButtonVisible(*visible)
            }
        }
    }

    fn apply_grid_update(&self, id: NodeId, update: &GridUpdate) -> WindowsResult<()> {
        let Handle::Grid(control) = &self.node(id)?.handle else {
            panic!("Grid update target is not a Grid");
        };
        match update {
            GridUpdate::Columns(values) => {
                let definitions = control.ColumnDefinitions()?;
                definitions.Clear()?;
                for value in values {
                    let definition = bindings::ColumnDefinition::new()?;
                    definition.SetWidth(native_grid_length(*value))?;
                    definitions.Append(&definition)?;
                }
            }
            GridUpdate::Rows(values) => {
                let definitions = control.RowDefinitions()?;
                definitions.Clear()?;
                for value in values {
                    let definition = bindings::RowDefinition::new()?;
                    definition.SetHeight(native_grid_length(*value))?;
                    definitions.Append(&definition)?;
                }
            }
            GridUpdate::ColumnSpacing(value) => control.SetColumnSpacing(*value)?,
            GridUpdate::RowSpacing(value) => control.SetRowSpacing(*value)?,
        }
        Ok(())
    }

    fn apply_border_update(&self, id: NodeId, update: BorderUpdate) -> WindowsResult<()> {
        let Handle::Border(control) = &self.node(id)?.handle else {
            panic!("Border update target is not a Border");
        };
        match update {
            BorderUpdate::Background(value) => {
                if let Some(value) = value {
                    let brush = native_brush(&value)?;
                    control.SetBackground(&brush)
                } else {
                    control.SetBackground(None::<&bindings::Brush>)
                }
            }
            BorderUpdate::BorderBrush(value) => {
                if let Some(value) = value {
                    let brush = native_brush(&value)?;
                    control.SetBorderBrush(&brush)
                } else {
                    control.SetBorderBrush(None::<&bindings::Brush>)
                }
            }
            BorderUpdate::BorderThickness(value) => control.SetBorderThickness(
                value.map_or_else(bindings::Thickness::default, native_thickness),
            ),
            BorderUpdate::CornerRadius(value) => control.SetCornerRadius(
                value.map_or_else(bindings::CornerRadius::default, native_corner_radius),
            ),
            BorderUpdate::Padding(value) => control
                .SetPadding(value.map_or_else(bindings::Thickness::default, native_thickness)),
        }
    }

    fn apply_button_emphasis(&self, id: NodeId, value: ButtonEmphasis) -> WindowsResult<()> {
        let Handle::Button { value: button, .. } = &self.node(id)?.handle else {
            panic!("button emphasis target is not a Button");
        };
        let element: bindings::IFrameworkElement = button.cast()?;
        match value {
            ButtonEmphasis::Standard => element.SetStyle(None::<&bindings::Style>),
            ButtonEmphasis::Accent => {
                let resources = bindings::Application::Current()?.Resources()?;
                let map = resources.cast::<windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >>()?;
                let key = windows_reference::IReference::from(windows_core::HSTRING::from(
                    "AccentButtonStyle",
                ));
                let style = map.Lookup(&key)?.cast::<bindings::Style>()?;
                element.SetStyle(&style)
            }
        }
    }

    fn apply_flyout_placement(&self, id: NodeId, value: FlyoutPlacement) -> WindowsResult<()> {
        let flyout: bindings::IFlyoutBase = match &self.node(id)?.handle {
            Handle::Flyout { value, .. } => value.cast()?,
            Handle::MenuFlyout { value, .. } => value.cast()?,
            Handle::CommandBarFlyout { value, .. } => value.cast()?,
            _ => panic!("flyout placement target is not a Flyout"),
        };
        flyout.SetPlacement(native_flyout_placement(value))
    }

    fn apply_stack_panel_update(&self, id: NodeId, update: StackPanelUpdate) -> WindowsResult<()> {
        let Handle::StackPanel(control) = &self.node(id)?.handle else {
            panic!("StackPanel update target is not a StackPanel");
        };
        match update {
            StackPanelUpdate::Orientation(value) => {
                control.SetOrientation(native_orientation(value))
            }
            StackPanelUpdate::Spacing(value) => control.SetSpacing(value),
        }
    }

    fn apply_hyperlink_button_navigate_uri(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::HyperlinkButton { value: button, .. } = &self.node(id)?.handle else {
            panic!("HyperlinkButton update target is not a HyperlinkButton");
        };
        let uri = value.as_deref().map(bindings::Uri::CreateUri).transpose()?;
        button.SetNavigateUri(uri.as_ref())
    }

    fn apply_repeat_button_update(
        &self,
        id: NodeId,
        update: RepeatButtonUpdate,
    ) -> WindowsResult<()> {
        let Handle::RepeatButton { value: button, .. } = &self.node(id)?.handle else {
            panic!("RepeatButton update target is not a RepeatButton");
        };
        match update {
            RepeatButtonUpdate::Delay(value) => button.SetDelay(value),
            RepeatButtonUpdate::Interval(value) => button.SetInterval(value),
        }
    }

    fn apply_viewbox_stretch(&self, id: NodeId, stretch: Stretch) -> WindowsResult<()> {
        let Handle::Viewbox(value) = &self.node(id)?.handle else {
            panic!("stretch target is not a Viewbox");
        };
        value.SetStretch(native_stretch(stretch))
    }

    fn apply_text_block_text(&self, id: NodeId, text: &str) -> WindowsResult<()> {
        let Handle::TextBlock(value) = &self.node(id)?.handle else {
            panic!("text target is not a TextBlock");
        };
        value.SetText(text)
    }

    fn apply_progress_bar_update(
        &self,
        id: NodeId,
        update: ProgressBarUpdate,
    ) -> WindowsResult<()> {
        match update {
            ProgressBarUpdate::Range(value) => self.apply_progress_bar_range(id, value),
            ProgressBarUpdate::Indeterminate(value) => {
                self.apply_progress_bar_indeterminate(id, value)
            }
        }
    }

    fn apply_progress_bar_range(&self, id: NodeId, range: RangeState) -> WindowsResult<()> {
        let Handle::ProgressBar(value) = &self.node(id)?.handle else {
            panic!("progress update target is not a ProgressBar");
        };
        let native: bindings::IRangeBase = value.cast()?;
        let current_maximum = native.Maximum()?;
        if range.minimum > current_maximum {
            native.SetMaximum(range.maximum)?;
            native.SetMinimum(range.minimum)?;
        } else {
            native.SetMinimum(range.minimum)?;
            native.SetMaximum(range.maximum)?;
        }
        native.SetValue(range.value)
    }

    fn apply_progress_bar_indeterminate(
        &self,
        id: NodeId,
        indeterminate: bool,
    ) -> WindowsResult<()> {
        let Handle::ProgressBar(value) = &self.node(id)?.handle else {
            panic!("progress update target is not a ProgressBar");
        };
        value
            .cast::<bindings::IProgressBar>()
            .and_then(|value| value.SetIsIndeterminate(indeterminate))
    }

    fn apply_progress_ring_update(
        &self,
        id: NodeId,
        update: ProgressRingUpdate,
    ) -> WindowsResult<()> {
        match update {
            ProgressRingUpdate::Range(value) => self.apply_progress_ring_range(id, value),
            ProgressRingUpdate::Active(value) => self.apply_progress_ring_active(id, value),
            ProgressRingUpdate::Indeterminate(value) => {
                self.apply_progress_ring_indeterminate(id, value)
            }
        }
    }

    fn apply_progress_ring_range(&self, id: NodeId, range: RangeState) -> WindowsResult<()> {
        let Handle::ProgressRing(value) = &self.node(id)?.handle else {
            panic!("progress update target is not a ProgressRing");
        };
        let ring: bindings::IProgressRing = value.cast()?;
        let current_maximum = ring.Maximum()?;
        if range.minimum > current_maximum {
            ring.SetMaximum(range.maximum)?;
            ring.SetMinimum(range.minimum)?;
        } else {
            ring.SetMinimum(range.minimum)?;
            ring.SetMaximum(range.maximum)?;
        }
        ring.SetValue(range.value)
    }

    fn apply_progress_ring_active(&self, id: NodeId, active: bool) -> WindowsResult<()> {
        let Handle::ProgressRing(value) = &self.node(id)?.handle else {
            panic!("progress update target is not a ProgressRing");
        };
        value
            .cast::<bindings::IProgressRing>()
            .and_then(|value| value.SetIsActive(active))
    }

    fn apply_progress_ring_indeterminate(
        &self,
        id: NodeId,
        indeterminate: bool,
    ) -> WindowsResult<()> {
        let Handle::ProgressRing(value) = &self.node(id)?.handle else {
            panic!("progress update target is not a ProgressRing");
        };
        value
            .cast::<bindings::IProgressRing>()
            .and_then(|value| value.SetIsIndeterminate(indeterminate))
    }
}

impl NativeRuntime for WinUiRuntime {
    fn apply(&mut self, commands: &[Command]) {
        (|| -> WindowsResult<()> {
            for command in commands {
                match command {
                    Command::StartTimer(spec) => self.start_timer(*spec)?,
                    Command::StopTimer {
                        owner,
                        slot,
                        revision,
                    } => self.stop_timer(*owner, *slot, *revision),
                    Command::UpdateApplication { id: _, update } => {
                        self.update_application(update)?;
                    }
                    Command::CreateWindow { id, create } => self.create_window(*id, create)?,
                    Command::SetWindowContent { window, content } => {
                        self.set_window_content(*window, *content)?;
                    }
                    Command::SetWindowOwner { owner, child } => {
                        self.set_window_owner(*owner, *child)?;
                    }
                    Command::UpdateWindow { id, update } => self.update_window(*id, update)?,
                    Command::ActivateWindow { id } => self.activate_window(*id)?,
                    Command::FocusElement { id } => {
                        self.node(*id)?
                            .handle
                            .ui_element()?
                            .Focus(bindings::FocusState::Programmatic)?;
                    }
                    Command::CloseWindow { id } => self.close_window(*id)?,
                    Command::Create { id, kind } => self.create(*id, *kind)?,
                    Command::Attach {
                        parent,
                        child,
                        attachment,
                    } => self.attach(*parent, *child, *attachment)?,
                    Command::Detach { parent, child } => self.detach(*parent, *child)?,
                    Command::BindOwner {
                        owner,
                        accessory,
                        relation,
                    } => self.bind_owner(*owner, *accessory, *relation)?,
                    Command::UnbindOwner {
                        owner,
                        accessory,
                        relation,
                    } => self.unbind_owner(*owner, *accessory, *relation)?,
                    Command::Move {
                        parent,
                        child,
                        index,
                    } => self.move_child(*parent, *child, *index)?,
                    Command::RunDeferred {
                        target,
                        window,
                        revision,
                        action,
                    } => self.run_deferred(*target, *window, *revision, *action)?,
                    #[cfg(feature = "canvas")]
                    Command::ApplyCanvasImageLayout {
                        target,
                        width,
                        height,
                        scale,
                    } => self.apply_canvas_image_layout(*target, *width, *height, *scale)?,
                    #[cfg(feature = "canvas")]
                    Command::RunCanvasImageFrame { target } => {
                        self.run_canvas_image_frame(*target)?;
                    }
                    #[cfg(feature = "canvas")]
                    Command::ApplyCanvasLayout {
                        target,
                        width,
                        height,
                        scale_x,
                        scale_y,
                    } => self.apply_canvas_layout(
                        *target,
                        canvas::NativeCanvasLayout {
                            width: *width,
                            height: *height,
                            scale_x: *scale_x,
                            scale_y: *scale_y,
                        },
                    )?,
                    #[cfg(feature = "canvas")]
                    Command::RunCanvasFrame { target } => self.run_canvas_frame(*target)?,
                    #[cfg(feature = "canvas")]
                    Command::ApplySwapChainHostLayout { target, layout } => {
                        self.apply_swap_chain_host_layout(*target, **layout)?;
                    }
                    #[cfg(feature = "canvas")]
                    Command::RunSwapChainHostFrame { target } => {
                        self.run_swap_chain_host_frame(*target)?;
                    }
                    Command::ApplyCompositionLayout {
                        target,
                        width,
                        height,
                        rasterization_scale,
                    } => self.apply_composition_layout(
                        *target,
                        *width,
                        *height,
                        *rasterization_scale,
                    )?,
                    #[cfg(feature = "webview")]
                    Command::FinishWebViewInitialization { target, revision } => {
                        self.finish_webview_initialization(*target, *revision)?;
                    }
                    Command::Destroy { id } => {
                        {
                            let node = self.node(*id)?;
                            assert!(
                                !(node.parent.is_some() || !node.children.is_empty()),
                                "destroyed native node is still attached"
                            );
                            if node
                                .input
                                .as_deref()
                                .is_some_and(input::NativeInputState::captures_pointer)
                            {
                                node.handle.ui_element()?.ReleasePointerCaptures()?;
                            }
                            if let Handle::ContentDialog { value, state } = &node.handle {
                                state.destroy(value, *id, &self.active_content_dialogs)?;
                            }
                        }
                        if let Handle::CompositionHost(state) = &mut self.node_mut(*id)?.handle {
                            state.detach()?;
                        }
                        #[cfg(feature = "canvas")]
                        if let Handle::SwapChainCanvas(state) = &self.node(*id)?.handle {
                            state.detach()?;
                        }
                        #[cfg(feature = "canvas")]
                        if let Handle::SwapChainHost(state) = &mut self.node_mut(*id)?.handle {
                            state.detach()?;
                        }
                        #[cfg(feature = "webview")]
                        if let Handle::WebViewHost(state) = &mut self.node_mut(*id)?.handle {
                            state.detach();
                        }
                        self.nodes.remove(id);
                        self.element_resources.remove(id);
                    }
                    Command::Update { id, update } => self.apply_update(*id, update)?,
                }
            }
            Ok(())
        })()
        .expect("native runtime failed");
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        let events = self.events.borrow_mut().drain(..).collect::<Vec<_>>();
        for event in &events {
            if let NativeEvent::TabsReordered { target, keys } = event {
                let children = self.node(*target).unwrap().children.clone();
                assert_eq!(
                    children.len(),
                    keys.len(),
                    "native TabView reorder changed the child count"
                );
                let reordered = keys
                    .iter()
                    .map(|key| {
                        children
                            .iter()
                            .copied()
                            .find(|child| {
                                matches!(
                                    &self.node(*child).unwrap().handle,
                                    Handle::TabViewItem(item) if item.key.get() == Some(*key)
                                )
                            })
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                self.node_mut(*target).unwrap().children = reordered;
                for (index, child) in self
                    .node(*target)
                    .unwrap()
                    .children
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    self.node_mut(child).unwrap().attachment = Some(Attachment::Item { index });
                }
            }
        }
        events
    }

    fn set_event_waker(&mut self, waker: Option<Rc<dyn Fn()>>) {
        *self.waker.borrow_mut() = waker;
    }
}

fn queue_scroll_event(
    events: &Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    target: NodeId,
    event: ScrollEvent,
) {
    events
        .borrow_mut()
        .push_back(NativeEvent::Scroll { target, event });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_expanded_event(
    events: &Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    target: NodeId,
    expanded: bool,
) {
    events
        .borrow_mut()
        .push_back(NativeEvent::ExpandedChanged { target, expanded });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

#[cfg(test)]
mod size_test {
    use super::NativeNode;
    use std::mem::size_of;

    #[test]
    fn pointer_state_keeps_the_native_node_size_bounded() {
        assert_eq!(size_of::<NativeNode>(), 144);
    }
}

#[cfg(test)]
mod latest_event_tests {
    use super::*;

    fn text(target: NodeId, value: &str) -> NativeEvent {
        NativeEvent::TextChanged {
            target,
            value: value.to_string(),
        }
    }

    fn selected(target: NodeId, key: u64) -> NativeEvent {
        NativeEvent::SelectedKeyChanged {
            target,
            key: Some(key),
        }
    }

    fn toggled(target: NodeId, value: bool) -> NativeEvent {
        NativeEvent::Toggled { target, value }
    }

    #[test]
    fn latest_event_replaces_every_same_slot_entry_at_the_queue_tail() {
        let target = NodeId::new(1, 1);
        let other_target = NodeId::new(2, 1);
        let events = RefCell::new(VecDeque::from([
            text(target, "first"),
            NativeEvent::Click { target },
            text(other_target, "other"),
            text(target, "second"),
            selected(target, 7),
        ]));

        queue_latest_event(&events, text(target, "latest"));

        assert_eq!(
            events.into_inner(),
            VecDeque::from([
                NativeEvent::Click { target },
                text(other_target, "other"),
                selected(target, 7),
                text(target, "latest"),
            ])
        );
    }

    #[test]
    fn queued_event_removal_is_limited_to_one_target_and_slot() {
        let target = NodeId::new(1, 1);
        let other_target = NodeId::new(2, 1);
        let events = RefCell::new(VecDeque::from([
            text(target, "remove"),
            text(other_target, "keep target"),
            NativeEvent::PasswordChanged {
                target,
                value: "keep variant".to_string(),
            },
            selected(target, 3),
        ]));

        remove_queued_event(&events, target, LatestEventSlot::TextChanged);

        assert_eq!(
            events.into_inner(),
            VecDeque::from([
                text(other_target, "keep target"),
                NativeEvent::PasswordChanged {
                    target,
                    value: "keep variant".to_string(),
                },
                selected(target, 3),
            ])
        );
    }

    #[test]
    fn controlled_events_without_latest_slots_remain_distinct() {
        let target = NodeId::new(1, 1);
        let events = RefCell::new(VecDeque::from([
            toggled(target, true),
            toggled(target, false),
        ]));

        queue_latest_event(&events, text(target, "first"));
        queue_latest_event(&events, text(target, "latest"));

        assert_eq!(
            events.into_inner(),
            VecDeque::from([
                toggled(target, true),
                toggled(target, false),
                text(target, "latest"),
            ])
        );
    }

    #[cfg(feature = "canvas")]
    fn canvas_layout(target: NodeId, size: f32) -> NativeEvent {
        NativeEvent::CanvasLayout {
            target,
            width: size,
            height: size,
            scale_x: size,
            scale_y: size,
        }
    }

    #[cfg(feature = "canvas")]
    fn canvas_image_layout(target: NodeId, size: f32) -> NativeEvent {
        NativeEvent::CanvasImageLayout {
            target,
            width: size,
            height: size,
            scale: size,
        }
    }

    #[cfg(feature = "canvas")]
    #[test]
    fn canvas_layout_slots_replace_only_layout_events() {
        let target = NodeId::new(1, 1);
        let events = RefCell::new(VecDeque::from([
            canvas_layout(target, 1.0),
            NativeEvent::CanvasFrame { target },
            canvas_image_layout(target, 2.0),
            NativeEvent::CanvasImageFrame { target },
        ]));

        queue_latest_event(&events, canvas_layout(target, 3.0));
        queue_latest_event(&events, canvas_image_layout(target, 4.0));

        assert_eq!(
            events.into_inner(),
            VecDeque::from([
                NativeEvent::CanvasFrame { target },
                NativeEvent::CanvasImageFrame { target },
                canvas_layout(target, 3.0),
                canvas_image_layout(target, 4.0),
            ])
        );
    }
}
