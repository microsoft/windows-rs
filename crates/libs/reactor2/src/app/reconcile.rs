use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::rc::Rc;
#[cfg(test)]
use std::time::Instant;

use super::auto_suggest_box::*;
use super::breadcrumb_bar::*;
use super::collection::*;
use super::command::*;
use super::container::*;
use super::content::*;
use super::logical::*;
use super::media::*;
use super::menu::*;
use super::mount::mount_element;
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
use super::window::{reconcile_application, reconcile_title_bar, reconcile_window};
use super::work::{RenderServices, same_contexts};
use crate::app::{contexts_for_node, window_owner_for_node};
use crate::arena::{NodeKind, RealizedRow};
use crate::element::tree::*;
use crate::element::{Element, RenderCx};
use crate::engine::{Engine, EngineError};
use crate::hooks::{ComponentMemo, HookCell, RenderFn};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::resources::ContextEntry;
use crate::runtime::*;

pub(crate) fn rerender_component<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    services: &RenderServices,
) -> Result<(), EngineError> {
    rerender_component_with(engine, id, None, services)
}

pub(super) fn rerender_component_with<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    next: Option<(RenderFn, Option<ComponentMemo>)>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let component_services = services
        .with_contexts(contexts_for_node(engine, id))
        .with_window_owner(window_owner_for_node(engine, id));
    let (key, identity, committed_render, committed_memo, committed_contexts, mut hooks) = {
        let node = engine
            .arena
            .get_mut(id)
            .ok_or(EngineError::InvalidNode(id))?;
        let Some(Mounted {
            key,
            kind:
                MountedKind::Component {
                    identity,
                    render,
                    memo,
                    contexts,
                    hooks,
                },
            ..
        }) = node.mounted.take()
        else {
            return Ok(());
        };
        (key, identity, render, memo, contexts, hooks)
    };
    let (render, memo) =
        next.unwrap_or_else(|| (Rc::clone(&committed_render), committed_memo.clone()));
    #[cfg(test)]
    let render_started = Instant::now();
    let render_result = catch_unwind(AssertUnwindSafe(|| {
        render_component(id, &render, &mut hooks, false, &component_services)
    }));
    #[cfg(test)]
    engine.record_tree_build(render_started.elapsed());
    let element = match render_result {
        Ok(element) => element,
        Err(payload) => {
            engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
                key,
                kind: MountedKind::Component {
                    identity,
                    render: committed_render,
                    memo: committed_memo,
                    contexts: committed_contexts,
                    hooks,
                },
            });
            resume_unwind(payload);
        }
    };
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Component {
            identity,
            render,
            memo,
            contexts: component_services.contexts.clone(),
            hooks,
        },
    });
    let child = engine.arena.get(id).unwrap().children[0];
    let reconcile_result = catch_unwind(AssertUnwindSafe(|| {
        reconcile(engine, child, element, &component_services)
    }));
    let replacement = match reconcile_result {
        Ok(Ok(replacement)) => replacement,
        Ok(Err(error)) => {
            restore_component_comparison(
                engine,
                id,
                committed_render,
                committed_memo,
                committed_contexts,
            );
            return Err(error);
        }
        Err(payload) => {
            restore_component_comparison(
                engine,
                id,
                committed_render,
                committed_memo,
                committed_contexts,
            );
            resume_unwind(payload);
        }
    };
    if replacement != child {
        debug_assert_eq!(engine.arena.get(id).unwrap().children, [replacement]);
    }
    Ok(())
}

fn restore_component_comparison<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    render: RenderFn,
    memo: Option<ComponentMemo>,
    contexts: Vec<ContextEntry>,
) {
    let Some(Mounted {
        kind:
            MountedKind::Component {
                render: mounted_render,
                memo: mounted_memo,
                contexts: mounted_contexts,
                ..
            },
        ..
    }) = &mut engine.arena.get_mut(id).unwrap().mounted
    else {
        unreachable!()
    };
    *mounted_render = render;
    *mounted_memo = memo;
    *mounted_contexts = contexts;
}

pub(crate) fn render_component(
    id: NodeId,
    render: &Rc<dyn for<'a> Fn(&mut RenderCx<'a>) -> Element>,
    hooks: &mut Vec<Rc<HookCell>>,
    mounting: bool,
    services: &RenderServices,
) -> Element {
    let mut cx = RenderCx {
        node: id,
        hooks,
        cursor: 0,
        mounting,
        scheduler: services.scheduler(),
        contexts: &services.contexts,
        context_defaults: services.context_defaults(),
    };
    let element = render(&mut cx);
    assert_eq!(
        cx.cursor,
        cx.hooks.len(),
        "component rendered fewer hooks than its initial render"
    );
    element
}

pub(super) fn reconcile<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    element: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    #[cfg(test)]
    let performance_before = engine.begin_element_diff();
    if !same_kind(engine, id, &element) {
        return replace(engine, id, element, services);
    }
    if let ElementKind::Component { render, memo, .. } = &element.kind {
        let node = engine.arena.get_mut(id).unwrap();
        let Some(Mounted {
            kind:
                MountedKind::Component {
                    render: mounted_render,
                    memo: mounted_memo,
                    contexts,
                    ..
                },
            ..
        }) = &mut node.mounted
        else {
            unreachable!()
        };
        if !services.dirty.contains(&id)
            && same_contexts(contexts, &services.contexts)
            && mounted_memo
                .as_ref()
                .zip(memo.as_ref())
                .is_some_and(|(left, right)| left.equivalent(right))
        {
            *mounted_render = Rc::clone(render);
            mounted_memo.clone_from(memo);
            #[cfg(test)]
            engine.finish_element_diff(performance_before);
            return Ok(id);
        }
    }
    let Element { key, kind } = element;
    let framework = element_framework(&kind);
    let old_framework =
        mounted_framework(&engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind);
    let control = element_control(&kind);
    let old_control =
        mounted_control(&engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind);
    let heap = diff_heap(
        mounted_heap(&engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind),
        element_heap(&kind),
    );
    match kind {
        ElementKind::Application { windows, props } => {
            reconcile_application(engine, id, key, windows, props, services)?;
        }
        ElementKind::Window(window) => {
            reconcile_window(engine, id, key, *window, services)?;
        }
        ElementKind::Component { render, memo, .. } => {
            reconcile_component(engine, id, render, memo, services)?;
        }
        ElementKind::Fragment { children } => reconcile_fragment(engine, id, children, services)?,
        ElementKind::StructuralSlot { slot, child } => {
            reconcile_structural_slot(engine, id, key, slot, *child, services)?;
        }
        ElementKind::Context { props, child } => {
            reconcile_context(engine, id, key, props, *child, services)?;
        }
        ElementKind::Reference { reference, child } => {
            reconcile_reference(engine, id, key, reference, *child, services)?;
        }
        ElementKind::FadeTransition { child, enter, exit } => {
            reconcile_fade_transition(engine, id, key, *child, enter, exit, services)?;
        }
        ElementKind::StackPanel(props) => {
            reconcile_stack_panel(engine, id, key, props, services)?;
        }
        ElementKind::Grid(props) => reconcile_grid(engine, id, key, props, services)?,
        ElementKind::TitleBar(title_bar) => {
            reconcile_title_bar(engine, id, key, *title_bar, services)?;
        }
        ElementKind::Canvas(props) => {
            reconcile_panel(engine, id, key, NativeKind::Canvas, props, services)?;
        }
        ElementKind::RelativePanel(props) => {
            reconcile_panel(engine, id, key, NativeKind::RelativePanel, props, services)?;
        }
        ElementKind::Viewbox { child, props } => {
            reconcile_viewbox(engine, id, key, *child, props, services)?;
        }
        ElementKind::ScrollViewer { child, props } => {
            reconcile_scroll_viewer(engine, id, key, *child, props, services)?;
        }
        ElementKind::ScrollView { child, props } => {
            reconcile_scroll_view(engine, id, key, *child, props, services)?;
        }
        ElementKind::SplitView(split) => {
            reconcile_split_view(engine, id, key, *split, services)?;
        }
        ElementKind::Expander(expander) => {
            reconcile_expander(engine, id, key, *expander, services)?;
        }
        ElementKind::CommandBar(props) => {
            reconcile_command_bar(engine, id, key, *props, services)?;
        }
        ElementKind::CompositionHost(props) => {
            reconcile_composition_host(engine, id, key, *props)?;
        }
        #[cfg(feature = "webview")]
        ElementKind::WebViewHost(props) => {
            reconcile_webview_host(engine, id, key, props)?;
        }
        #[cfg(feature = "canvas")]
        ElementKind::CanvasImage(props) => reconcile_canvas_image(engine, id, key, props)?,
        #[cfg(feature = "canvas")]
        ElementKind::SwapChainCanvas(props) => {
            reconcile_swap_chain_canvas(engine, id, key, props)?;
        }
        #[cfg(feature = "canvas")]
        ElementKind::SwapChainHost(props) => {
            reconcile_swap_chain_host(engine, id, key, props)?;
        }
        ElementKind::Image(props) => reconcile_image(engine, id, key, props)?,
        ElementKind::Icon(icon) => reconcile_icon(engine, id, key, icon)?,
        ElementKind::Shape(props) => reconcile_shape(engine, id, key, *props)?,
        ElementKind::AppBarButton(props) => reconcile_app_bar_button(engine, id, key, props)?,
        ElementKind::AppBarToggleButton(props) => {
            reconcile_app_bar_toggle_button(engine, id, key, props)?;
        }
        ElementKind::AppBarSeparator => reconcile_app_bar_separator(engine, id, key),
        ElementKind::ContentDialog(dialog) => {
            reconcile_content_dialog(engine, id, key, *dialog, services)?;
        }
        ElementKind::ToolTip(tooltip) => {
            reconcile_tooltip(engine, id, key, *tooltip, services)?;
        }
        ElementKind::TeachingTip(tip) => {
            reconcile_teaching_tip(engine, id, key, *tip, services)?;
        }
        ElementKind::AttachedChild { placement, child } => {
            reconcile_attached_child(engine, id, key, placement, *child, services)?;
        }
        ElementKind::Border(border) => {
            reconcile_border(engine, id, key, *border, services)?;
        }
        ElementKind::Button { child, props } => {
            reconcile_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::ButtonFlyout { button, content } => {
            reconcile_button_flyout(engine, id, key, button, *content, services)?;
        }
        ElementKind::ButtonMenuFlyout {
            button,
            label,
            flyout,
        } => {
            reconcile_button_menu_flyout(engine, id, key, button, *label, flyout, services)?;
        }
        ElementKind::ButtonCommandBarFlyout {
            button,
            label,
            flyout,
        } => {
            reconcile_button_command_bar_flyout(
                engine, id, key, button, *label, *flyout, services,
            )?;
        }
        ElementKind::DropDownButton(drop_down) => {
            let DropDownButtonElement {
                label,
                flyout,
                props,
            } = *drop_down;
            match flyout {
                DropDownFlyoutElement::Content(flyout) => reconcile_drop_down_button(
                    engine,
                    id,
                    key,
                    DropDownButtonElement {
                        label,
                        flyout: DropDownFlyoutElement::Content(flyout),
                        props,
                    },
                    services,
                )?,
                DropDownFlyoutElement::Menu(flyout) => reconcile_drop_down_menu_flyout(
                    engine, id, key, props, *label, flyout, services,
                )?,
            }
        }
        ElementKind::SplitButton { child, props } => {
            reconcile_split_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::SplitButtonFlyout { button, content } => {
            reconcile_split_button_flyout(engine, id, key, button, *content, services)?;
        }
        ElementKind::HyperlinkButton { child, props } => {
            reconcile_hyperlink_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::RepeatButton { child, props } => {
            reconcile_repeat_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::ToggleButton { child, props } => {
            reconcile_toggle_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::ToggleSwitch(props) => reconcile_toggle_switch(engine, id, key, props)?,
        ElementKind::InfoBadge(props) => reconcile_info_badge(engine, id, key, props)?,
        ElementKind::InfoBar(props) => reconcile_info_bar(engine, id, key, *props)?,
        ElementKind::PersonPicture(props) => reconcile_person_picture(engine, id, key, *props)?,
        ElementKind::ProgressBar(props) => reconcile_progress_bar(engine, id, key, props)?,
        ElementKind::ProgressRing(props) => reconcile_progress_ring(engine, id, key, props)?,
        ElementKind::Slider(props) => reconcile_slider(engine, id, key, props)?,
        ElementKind::NumberBox(props) => reconcile_number_box(engine, id, key, props)?,
        ElementKind::RatingControl(props) => {
            reconcile_rating_control(engine, id, key, props)?;
        }
        ElementKind::ColorPicker(props) => reconcile_color_picker(engine, id, key, props)?,
        ElementKind::DatePicker(props) => reconcile_date_picker(engine, id, key, props)?,
        ElementKind::CalendarDatePicker(props) => {
            reconcile_calendar_date_picker(engine, id, key, props)?;
        }
        ElementKind::TimePicker(props) => reconcile_time_picker(engine, id, key, props)?,
        ElementKind::CalendarView(props) => reconcile_calendar_view(engine, id, key, props)?,
        ElementKind::RichEditBox(props) => reconcile_rich_edit_box(engine, id, key, *props)?,
        ElementKind::RichTextBlock(props) => {
            reconcile_rich_text_block(engine, id, key, *props)?;
        }
        ElementKind::TreeView(props) => reconcile_tree_view(engine, id, key, props)?,
        ElementKind::NavigationView(value) => {
            reconcile_navigation_view(engine, id, key, *value, services)?;
        }
        ElementKind::NavigationViewItem(props) => {
            reconcile_navigation_view_item(engine, id, key, props)?;
        }
        ElementKind::CheckBox { child, props } => {
            reconcile_check_box(engine, id, key, *child, props, services)?;
        }
        ElementKind::RadioButton { child, props } => {
            reconcile_radio_button(engine, id, key, *child, props, services)?;
        }
        ElementKind::TextBlock(props) => reconcile_text_block(engine, id, key, props)?,
        ElementKind::TextBox(props) => reconcile_text_box(engine, id, key, *props)?,
        ElementKind::PasswordBox(props) => reconcile_password_box(engine, id, key, props)?,
        ElementKind::ListBox(props) => reconcile_list_box(engine, id, key, props)?,
        ElementKind::ComboBox(props) => reconcile_combo_box(engine, id, key, props)?,
        ElementKind::RadioButtons(props) => reconcile_radio_buttons(engine, id, key, props)?,
        ElementKind::MenuBar(props) => reconcile_menu_bar(engine, id, key, props)?,
        ElementKind::FlipView(props) => {
            reconcile_flip_view(engine, id, key, *props, services)?;
        }
        ElementKind::TabView(props) => {
            reconcile_tab_view(engine, id, key, *props, services)?;
        }
        ElementKind::TabViewItem { child, props } => {
            reconcile_tab_view_item(engine, id, key, *child, props, services)?;
        }
        ElementKind::SelectorBar(props) => {
            reconcile_selector_bar(engine, id, key, *props, services)?;
        }
        ElementKind::SelectorBarItem(props) => {
            reconcile_selector_bar_item(engine, id, key, props)?;
        }
        ElementKind::BreadcrumbBar(props) => {
            reconcile_breadcrumb_bar(engine, id, key, props)?;
        }
        ElementKind::AutoSuggestBox(props) => {
            reconcile_auto_suggest_box(engine, id, key, props)?;
        }
        ElementKind::Pivot(props) => reconcile_pivot(engine, id, key, *props, services)?,
        ElementKind::PivotItem { child, props } => {
            reconcile_pivot_item(engine, id, key, *child, props, services)?;
        }
        ElementKind::VirtualCollection(props) => {
            reconcile_virtual_collection(engine, id, key, *props, services)?;
        }
    }
    if old_framework != framework {
        let target = engine.single_projected_native_root(id).unwrap();
        update_framework_props(engine, target, old_framework, framework)?;
    }
    if old_control != control {
        let target = engine.single_projected_native_root(id).unwrap();
        update_control_props(engine, target, old_control, control)?;
    }
    if !heap.is_empty() {
        let target = engine.single_projected_native_root(id).unwrap();
        apply_heap_changes(engine, target, heap)?;
    }
    #[cfg(test)]
    engine.finish_element_diff(performance_before);
    Ok(id)
}

pub(super) fn reconcile_children<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    elements: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    validate_sibling_keys(&elements)?;
    reconcile_children_validated(engine, parent, elements, services)
}

pub(super) fn reconcile_children_validated<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    elements: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(parent).unwrap().children.clone();
    let keyed = !elements.is_empty()
        && elements.iter().all(|element| element.key.is_some())
        && current.iter().all(|child| {
            engine
                .arena
                .get(*child)
                .and_then(|node| node.mounted.as_ref())
                .is_some_and(|mounted| mounted.key.is_some())
        });
    if keyed {
        return reconcile_keyed_children(engine, parent, current, elements, services);
    }
    let common = current.len().min(elements.len());
    let mut elements = elements.into_iter();
    for child in current.iter().take(common).copied() {
        reconcile(engine, child, elements.next().unwrap(), services)?;
    }
    for child in current.into_iter().skip(common).rev() {
        engine.remove_subtree(child)?;
    }
    for element in elements {
        let child = mount_element(engine, element, services)?;
        engine.attach(parent, child)?;
    }
    Ok(())
}

fn reconcile_keyed_children<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    current: Vec<NodeId>,
    elements: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    if current.iter().zip(&elements).all(|(id, element)| {
        engine.arena.get(*id).unwrap().mounted.as_ref().unwrap().key == element.key
    }) {
        let common = current.len().min(elements.len());
        let mut elements = elements.into_iter();
        for child in current.iter().take(common).copied() {
            reconcile(engine, child, elements.next().unwrap(), services)?;
        }
        for child in current.into_iter().skip(common).rev() {
            engine.remove_subtree(child)?;
        }
        let mut appended = Vec::with_capacity(elements.len());
        for element in elements {
            appended.push(mount_element(engine, element, services)?);
        }
        engine.attach_appended(parent, &appended)?;
        return Ok(());
    }

    let mut existing = BTreeMap::new();
    for id in current.iter().copied() {
        let key = engine
            .arena
            .get(id)
            .unwrap()
            .mounted
            .as_ref()
            .unwrap()
            .key
            .unwrap();
        if existing.insert(key, id).is_some() {
            return Err(EngineError::DuplicateSiblingKey { key });
        }
    }
    let mut desired = Vec::with_capacity(elements.len());
    for element in elements {
        let key = element.key.unwrap();
        let id = if let Some(id) = existing.remove(&key) {
            reconcile(engine, id, element, services)?
        } else {
            let id = mount_element(engine, element, services)?;
            engine.attach(parent, id)?;
            id
        };
        desired.push(id);
    }
    for id in existing.into_values() {
        engine.remove_subtree(id)?;
    }
    engine.reorder_children(parent, &desired)
}

pub(crate) fn validate_sibling_keys(elements: &[Element]) -> Result<(), EngineError> {
    let count = elements
        .iter()
        .filter(|element| element.key.is_some())
        .count();
    if count < 2 {
        return Ok(());
    }
    let mut keys = Vec::with_capacity(count);
    keys.extend(elements.iter().filter_map(|element| element.key));
    keys.sort_unstable();
    if let Some(key) = keys
        .windows(2)
        .find_map(|keys| (keys[0] == keys[1]).then_some(keys[0]))
    {
        return Err(EngineError::DuplicateSiblingKey { key });
    }
    Ok(())
}

fn same_kind<R: NativeRuntime>(engine: &Engine<R>, id: NodeId, element: &Element) -> bool {
    let Some(node) = engine.arena.get(id) else {
        return false;
    };
    let Some(mounted) = &node.mounted else {
        return false;
    };
    if mounted.key != element.key {
        return false;
    }
    if mounted.kind.reconcile_kind() != element.kind.reconcile_kind() {
        return false;
    }
    match (&mounted.kind, &element.kind) {
        (
            MountedKind::Component { identity: left, .. },
            ElementKind::Component {
                identity: right, ..
            },
        ) => left == right,
        (MountedKind::Context(left), ElementKind::Context { props: right, .. }) => {
            left.entry.id == right.entry.id
        }
        (MountedKind::VirtualCollection(left), ElementKind::VirtualCollection(right)) => {
            left.kind == right.kind
        }
        _ => true,
    }
}

fn replace<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    element: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let (parent, index, virtual_slot) = {
        let node = engine.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        let index = node.parent.and_then(|parent| {
            engine
                .arena
                .get(parent)
                .unwrap()
                .children
                .iter()
                .position(|child| *child == id)
        });
        let virtual_slot = node.parent.and_then(|parent| {
            let node = engine.arena.get(parent)?;
            let NodeKind::VirtualHost { realized } = &node.kind else {
                return None;
            };
            realized
                .iter()
                .find(|(_, row)| row.root == id)
                .map(|(item_index, row)| (*item_index, row.lease, row.key))
        });
        (node.parent, index, virtual_slot)
    };
    engine.remove_subtree(id)?;
    let replacement = mount_element(engine, element, services)?;
    if let (Some(parent), Some((item_index, lease, item_key)), Some(index)) =
        (parent, virtual_slot, index)
    {
        let NodeKind::VirtualHost { realized } = &mut engine.arena.get_mut(parent).unwrap().kind
        else {
            unreachable!()
        };
        realized.insert(
            item_index,
            RealizedRow {
                lease,
                key: item_key,
                root: replacement,
            },
        );
        engine.attach_at(parent, replacement, index)?;
    } else if let (Some(parent), Some(index)) = (parent, index) {
        engine.attach_at(parent, replacement, index)?;
    }
    Ok(replacement)
}
