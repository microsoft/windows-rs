use super::mount::mount_element;
use super::reconcile::{reconcile, reconcile_children, validate_sibling_keys};
use super::work::RenderServices;
use crate::element::WindowPresenter;
use crate::element::props::{ApplicationProps, TitleBarProps, WindowProps};
use crate::element::tree::{TitleBarElement, WindowElement};
use crate::element::{Element, GridLength};
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind, MountedWindow};
use crate::runtime::{
    ApplicationUpdate, AttachedUpdate, ControlUpdate, NativeKind, NativeRuntime, TitleBarUpdate,
    WindowCreate, WindowUpdate,
};

struct TitleBarChanges {
    title: Option<Option<String>>,
    subtitle: Option<Option<String>>,
    back_button_visible: Option<bool>,
    back_button_enabled: Option<bool>,
    pane_toggle_button_visible: Option<bool>,
    height: Option<crate::element::TitleBarHeight>,
}

pub(super) fn mount_application<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    windows: Vec<Element>,
    props: ApplicationProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    validate_sibling_keys(&windows)?;
    let id = engine.create_application()?;
    if !props.resources.is_empty() {
        engine.update_application(
            id,
            ApplicationUpdate::Resources(Box::new(props.resources.clone())),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Application(props)));
    for window in windows {
        let window = mount_element(engine, window, services)?;
        engine.attach(id, window)?;
    }
    Ok(id)
}

pub(super) fn mount_window<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    window: WindowElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let WindowElement {
        title_bar,
        content,
        owned_windows,
        props,
        custom_title_bar,
    } = window;
    validate_sibling_keys(&owned_windows)?;
    let id = engine.create_window(WindowCreate {
        title: props.title.clone(),
    })?;
    mount_window_options(engine, id, &props)?;
    if let Some(owner) = services.window_owner {
        engine.set_window_owner(owner, id)?;
    }
    if let Some(reference) = &props.reference {
        reference.prepare_mount(id, services.scheduler());
        engine.add_reference();
    }
    let layout = engine.create_native(NativeKind::Grid)?;
    engine.set_grid_rows(layout, vec![GridLength::Auto, GridLength::STAR])?;
    engine.attach(id, layout)?;
    let child_services = services.with_window_owner(None);
    let title_bar_node = mount_element(engine, *title_bar, &child_services)?;
    engine.attach(layout, title_bar_node)?;
    let title_bar_root = window_content_root(engine, id, title_bar_node)?;
    engine.queue_attached_update(title_bar_root, AttachedUpdate::Row(Some(0)))?;
    let body = mount_element(engine, *content, &child_services)?;
    engine.attach(layout, body)?;
    let body_root = window_content_root(engine, id, body)?;
    engine.queue_attached_update(body_root, AttachedUpdate::Row(Some(1)))?;
    let bound_title_bar = custom_title_bar.then_some(title_bar_root);
    if let Some(title_bar) = bound_title_bar {
        engine.update_window(id, WindowUpdate::BindTitleBar(title_bar))?;
    }
    engine.set_window_content(id, layout)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Window(MountedWindow {
            props,
            content: layout,
            title_bar: bound_title_bar,
        }),
    ));
    let owned_root = engine.create_logical()?;
    engine.arena.get_mut(owned_root).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::Fragment));
    engine.attach(id, owned_root)?;
    let owned_services = services.with_window_owner(Some(id));
    for window in owned_windows {
        let window = mount_element(engine, window, &owned_services)?;
        engine.attach(owned_root, window)?;
    }
    Ok(id)
}

pub(super) fn mount_title_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    title_bar: TitleBarElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let TitleBarElement {
        content,
        right_header,
        props,
    } = title_bar;
    let id = engine.create_native(NativeKind::TitleBar)?;
    apply_title_bar_props(engine, id, &props)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::TitleBar(Box::new(props))));
    let content = mount_element(engine, *content, services)?;
    engine.attach(id, content)?;
    let right_header = mount_element(engine, *right_header, services)?;
    engine.attach(id, right_header)?;
    Ok(id)
}

pub(super) fn reconcile_application<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    windows: Vec<Element>,
    props: ApplicationProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let old_resources = {
        let node = engine.arena.get(id).unwrap();
        let Some(Mounted {
            kind: MountedKind::Application(old),
            ..
        }) = &node.mounted
        else {
            unreachable!()
        };
        old.resources.clone()
    };
    if old_resources != props.resources {
        engine.update_application(
            id,
            ApplicationUpdate::Resources(Box::new(props.resources.clone())),
        )?;
    }
    reconcile_children(engine, id, windows, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Application(props)));
    Ok(())
}

pub(super) fn reconcile_window<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    window: WindowElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let WindowElement {
        title_bar,
        content,
        owned_windows,
        props,
        custom_title_bar,
    } = window;
    let (old, old_title_bar, layout, current_title_bar, current_body, owned_root) = {
        let node = engine.arena.get(id).unwrap();
        let Some(Mounted {
            kind: MountedKind::Window(old),
            ..
        }) = &node.mounted
        else {
            unreachable!()
        };
        let layout = node.children[0];
        let layout_node = engine.arena.get(layout).unwrap();
        (
            old.props.clone(),
            old.title_bar,
            layout,
            layout_node.children[0],
            layout_node.children[1],
            node.children[1],
        )
    };
    if old_title_bar.is_some() && !custom_title_bar {
        engine.update_window(id, WindowUpdate::UnbindTitleBar)?;
    }
    if old_title_bar.is_none() && custom_title_bar {
        reconcile_window_options(engine, id, &old, &props)?;
    }
    let child_services = services.with_window_owner(None);
    let old_title_bar_root = engine
        .single_projected_native_root(current_title_bar)
        .unwrap();
    let title_bar_replacement = reconcile(engine, current_title_bar, *title_bar, &child_services)?;
    let new_title_bar_root = engine
        .single_projected_native_root(title_bar_replacement)
        .unwrap();
    if old_title_bar_root != new_title_bar_root {
        engine.queue_attached_update(new_title_bar_root, AttachedUpdate::Row(Some(0)))?;
    }
    let title_bar = custom_title_bar.then_some(new_title_bar_root);
    if old_title_bar.is_none()
        && let Some(title_bar) = title_bar
    {
        engine.update_window(id, WindowUpdate::BindTitleBar(title_bar))?;
    }
    let old_body_root = engine.single_projected_native_root(current_body).unwrap();
    let body_replacement = reconcile(engine, current_body, *content, &child_services)?;
    let new_body_root = window_content_root(engine, id, body_replacement)?;
    if old_body_root != new_body_root {
        engine.queue_attached_update(new_body_root, AttachedUpdate::Row(Some(1)))?;
    }
    if old_title_bar.is_some() || !custom_title_bar {
        reconcile_window_options(engine, id, &old, &props)?;
    }
    reconcile_children(
        engine,
        owned_root,
        owned_windows,
        &services.with_window_owner(Some(id)),
    )?;
    if old.reference != props.reference {
        if let Some(reference) = &old.reference {
            reference.clear();
            engine.remove_reference();
        }
        if let Some(reference) = &props.reference {
            reference.prepare_mount(id, services.scheduler());
            engine.add_reference();
        }
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Window(MountedWindow {
            props,
            content: layout,
            title_bar,
        }),
    ));
    Ok(())
}

pub(super) fn reconcile_title_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    title_bar: TitleBarElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let TitleBarElement {
        content,
        right_header,
        props,
    } = title_bar;
    reconcile_title_bar_props(engine, id, &props)?;
    let [current_content, current_right_header] =
        *engine.arena.get(id).unwrap().children.as_slice()
    else {
        unreachable!()
    };
    reconcile(engine, current_content, *content, services)?;
    reconcile(engine, current_right_header, *right_header, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::TitleBar(Box::new(props))));
    Ok(())
}

fn window_content_root<R: NativeRuntime>(
    engine: &Engine<R>,
    window: NodeId,
    content: NodeId,
) -> Result<NodeId, EngineError> {
    engine
        .single_projected_native_root(content)
        .ok_or(EngineError::WindowContentNativeRootCount {
            window,
            count: engine.projected_native_roots(content).len(),
        })
}

pub(super) fn apply_title_bar_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &TitleBarProps,
) -> Result<(), EngineError> {
    engine.queue_control_update(
        id,
        ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Title(props.title.clone()))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Subtitle(props.subtitle.clone()))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TitleBar(Box::new(TitleBarUpdate::BackButtonVisible(
            props.back_button_visible,
        ))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TitleBar(Box::new(TitleBarUpdate::BackButtonEnabled(
            props.back_button_enabled,
        ))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TitleBar(Box::new(TitleBarUpdate::PaneToggleButtonVisible(
            props.pane_toggle_button_visible,
        ))),
    )?;
    engine.set_height(
        id,
        Some(match props.height {
            crate::element::TitleBarHeight::Standard => 32.0,
            crate::element::TitleBarHeight::Tall => 48.0,
        }),
    )
}

pub(super) fn reconcile_title_bar_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &TitleBarProps,
) -> Result<(), EngineError> {
    let changes = {
        let node = engine.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        let Some(Mounted {
            kind: MountedKind::TitleBar(old),
            ..
        }) = &node.mounted
        else {
            return Err(EngineError::InvalidNode(id));
        };
        TitleBarChanges {
            title: (old.title != props.title).then(|| props.title.clone()),
            subtitle: (old.subtitle != props.subtitle).then(|| props.subtitle.clone()),
            back_button_visible: (old.back_button_visible != props.back_button_visible)
                .then_some(props.back_button_visible),
            back_button_enabled: (old.back_button_enabled != props.back_button_enabled)
                .then_some(props.back_button_enabled),
            pane_toggle_button_visible: (old.pane_toggle_button_visible
                != props.pane_toggle_button_visible)
                .then_some(props.pane_toggle_button_visible),
            height: (old.height != props.height).then_some(props.height),
        }
    };
    if let Some(value) = changes.title {
        engine.queue_control_update(
            id,
            ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Title(value))),
        )?;
    }
    if let Some(value) = changes.subtitle {
        engine.queue_control_update(
            id,
            ControlUpdate::TitleBar(Box::new(TitleBarUpdate::Subtitle(value))),
        )?;
    }
    if let Some(value) = changes.back_button_visible {
        engine.queue_control_update(
            id,
            ControlUpdate::TitleBar(Box::new(TitleBarUpdate::BackButtonVisible(value))),
        )?;
    }
    if let Some(value) = changes.back_button_enabled {
        engine.queue_control_update(
            id,
            ControlUpdate::TitleBar(Box::new(TitleBarUpdate::BackButtonEnabled(value))),
        )?;
    }
    if let Some(value) = changes.pane_toggle_button_visible {
        engine.queue_control_update(
            id,
            ControlUpdate::TitleBar(Box::new(TitleBarUpdate::PaneToggleButtonVisible(value))),
        )?;
    }
    if let Some(value) = changes.height {
        engine.set_height(
            id,
            Some(match value {
                crate::element::TitleBarHeight::Standard => 32.0,
                crate::element::TitleBarHeight::Tall => 48.0,
            }),
        )?;
    }
    Ok(())
}

pub(crate) fn mount_window_options<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &WindowProps,
) -> Result<(), EngineError> {
    if props.presenter != WindowPresenter::Default {
        engine.update_window(id, WindowUpdate::Presenter(props.presenter))?;
    }
    if props.backdrop.is_some() {
        engine.update_window(id, WindowUpdate::Backdrop(props.backdrop))?;
    }
    if let Some(icon) = &props.icon {
        engine.update_window(id, WindowUpdate::Icon(icon.clone()))?;
    }
    if props.theme != crate::element::WindowTheme::System {
        engine.update_window(id, WindowUpdate::Theme(props.theme))?;
    }
    if !props.title_bar.is_default() {
        engine.update_window(id, WindowUpdate::TitleBar(Box::new(props.title_bar)))?;
    }
    if !props.overlapped.is_default() {
        engine.update_window(id, WindowUpdate::Overlapped(props.overlapped))?;
    }
    if !props.constraints.is_empty() {
        engine.update_window(id, WindowUpdate::Constraints(props.constraints.into()))?;
    }
    if let Some(size) = props.client_size {
        engine.update_window(id, WindowUpdate::ClientSize(size))?;
    }
    Ok(())
}

pub(crate) fn reconcile_window_options<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    old: &WindowProps,
    props: &WindowProps,
) -> Result<(), EngineError> {
    if old.title != props.title {
        engine.update_window(id, WindowUpdate::Title(props.title.clone()))?;
    }
    let clear_constraints_before_presenter = old.presenter == WindowPresenter::Default
        && props.presenter != WindowPresenter::Default
        && !old.constraints.is_empty()
        && props.constraints.is_empty();
    let clear_title_bar_before_presenter = old.presenter == WindowPresenter::Default
        && props.presenter != WindowPresenter::Default
        && !old.title_bar.is_default();
    let clear_overlapped_before_presenter = old.presenter == WindowPresenter::Default
        && props.presenter != WindowPresenter::Default
        && !old.overlapped.is_default();
    if clear_constraints_before_presenter {
        engine.update_window(id, WindowUpdate::Constraints(props.constraints.into()))?;
    }
    if clear_title_bar_before_presenter {
        engine.update_window(id, WindowUpdate::TitleBar(Box::default()))?;
    }
    if clear_overlapped_before_presenter {
        engine.update_window(id, WindowUpdate::Overlapped(Default::default()))?;
    }
    if old.presenter != props.presenter {
        engine.update_window(id, WindowUpdate::Presenter(props.presenter))?;
    }
    if old.backdrop != props.backdrop {
        engine.update_window(id, WindowUpdate::Backdrop(props.backdrop))?;
    }
    if old.icon != props.icon
        && let Some(icon) = &props.icon
    {
        engine.update_window(id, WindowUpdate::Icon(icon.clone()))?;
    }
    if old.theme != props.theme {
        engine.update_window(id, WindowUpdate::Theme(props.theme))?;
    }
    if old.title_bar != props.title_bar && !clear_title_bar_before_presenter {
        engine.update_window(id, WindowUpdate::TitleBar(Box::new(props.title_bar)))?;
    }
    if old.overlapped != props.overlapped && !clear_overlapped_before_presenter {
        engine.update_window(id, WindowUpdate::Overlapped(props.overlapped))?;
    }
    if old.constraints != props.constraints && !clear_constraints_before_presenter {
        engine.update_window(id, WindowUpdate::Constraints(props.constraints.into()))?;
    }
    if old.client_size != props.client_size
        && let Some(size) = props.client_size
    {
        engine.update_window(id, WindowUpdate::ClientSize(size))?;
    }
    Ok(())
}
