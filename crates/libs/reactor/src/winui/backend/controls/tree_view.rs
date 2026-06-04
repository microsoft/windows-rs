//! Typed handler for the `TreeView` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::{TreeSelectionMode, TreeViewWidget};
use crate::winui::backend::Handle;
use crate::winui::backend::convert::build_tree_view_node;

pub fn mount(
    widget: &TreeViewWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::TreeView(tv) = handle else {
        return Ok(());
    };

    set_nodes(tv, widget)?;
    set_selection_mode(tv, widget.selection_mode)?;

    ctx.mount_event(
        &widget.on_item_invoked,
        Event::TreeViewItemInvoked,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &TreeViewWidget,
    new: &TreeViewWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::TreeView(tv) = handle else {
        return Ok(());
    };

    if old.nodes != new.nodes {
        set_nodes(tv, new)?;
    }
    super::diff_val!(
        old,
        new,
        selection_mode,
        set_selection_mode(tv, new.selection_mode)
    );

    ctx.diff_event(
        &old.on_item_invoked,
        &new.on_item_invoked,
        Event::TreeViewItemInvoked,
        EventHandler::TextChanged,
    );
    Ok(())
}

fn set_nodes(tv: &Xaml::TreeView, widget: &TreeViewWidget) -> windows_core::Result<()> {
    let root = tv.get_RootNodes()?;
    root.Clear()?;
    for node_def in &widget.nodes {
        let node = build_tree_view_node(node_def)?;
        root.Append(&node)?;
    }
    Ok(())
}

fn set_selection_mode(tv: &Xaml::TreeView, mode: TreeSelectionMode) -> windows_core::Result<()> {
    use TreeSelectionMode as E;
    use Xaml::TreeViewSelectionMode as W;
    let mapped = match mode {
        E::None => W::None,
        E::Single => W::Single,
        E::Multiple => W::Multiple,
    };
    tv.put_SelectionMode(mapped)
}
