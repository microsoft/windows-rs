//! TreeView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    tv: &Xaml::TreeView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::TreeViewNodes, PropValue::TreeViewNodes(nodes)) => Some((|| {
            let root = tv.get_RootNodes()?;
            root.Clear()?;
            for node_def in nodes {
                let node = super::super::build_tree_view_node(node_def)?;
                root.Append(&node)?;
            }
            Ok(())
        })()),
        (Prop::TreeViewSelectionMode, PropValue::TreeViewSelectionMode(mode)) => {
            use crate::core::widgets::TreeSelectionMode as E;
            use Xaml::TreeViewSelectionMode as W;
            let mapped = match mode {
                E::None => W::None,
                E::Single => W::Single,
                E::Multiple => W::Multiple,
            };
            Some(tv.put_SelectionMode(mapped))
        }
        _ => None,
    }
}
