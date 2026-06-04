//! Typed handler for the `TreeView` widget.

use crate::bindings as Xaml;
use crate::core::widgets::{TreeSelectionMode, TreeViewWidget};
use crate::winui::backend::Handle;
use crate::winui::backend::convert::build_tree_view_node;

pub fn mount(widget: &TreeViewWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::TreeView(tv) = handle else {
        return Ok(false);
    };

    set_nodes(tv, widget)?;
    set_selection_mode(tv, widget.selection_mode)?;

    Ok(true)
}

pub fn diff(
    old: &TreeViewWidget,
    new: &TreeViewWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::TreeView(tv) = handle else {
        return Ok(false);
    };

    if old.nodes != new.nodes {
        set_nodes(tv, new)?;
    }
    if old.selection_mode != new.selection_mode {
        set_selection_mode(tv, new.selection_mode)?;
    }

    Ok(true)
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
