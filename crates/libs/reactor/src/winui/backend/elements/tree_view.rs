//! TreeView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

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

pub(in crate::winui::backend) fn attach_event(
    tv: &Xaml::TreeView,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::TreeViewItemInvoked => {
            revokers.push(
                tv.add_ItemInvoked(move |_sender, args| {
                    let text = args
                        .as_ref()
                        .and_then(|a| a.get_InvokedItem().ok())
                        .and_then(|insp| {
                            insp.cast::<crate::bindings::ITreeViewNode>()
                                .ok()
                                .and_then(|node| node.get_Content().ok())
                        })
                        .and_then(|content| {
                            content
                                .cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                .ok()
                                .and_then(|r| r.Value().ok())
                        })
                        .map(|h| h.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
