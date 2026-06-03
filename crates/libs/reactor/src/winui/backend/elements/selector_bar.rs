//! SelectorBar - event dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};

pub(in crate::winui::backend) fn attach_event(
    sb: &Xaml::SelectorBar,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::SelectorBarSelectionChanged => {
            let sb2 = sb.clone();
            revokers.push(
                sb.add_SelectionChanged(move |_sender, _args| {
                    if let Ok(selected) = sb2.get_SelectedItem()
                        && let Ok(text) = selected.get_Text()
                    {
                        handler.invoke_string(text);
                    }
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
