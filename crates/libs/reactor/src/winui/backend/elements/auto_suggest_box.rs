//! AutoSuggestBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    asb: &Xaml::AutoSuggestBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::AutoSuggestText, PropValue::Str(s)) => Some((|| {
            if asb.get_Text().ok().as_deref() == Some(s.as_str()) {
                return Ok(());
            }
            asb.put_Text(s)
        })()),
        (Prop::AutoSuggestItems, PropValue::StrList(items)) => Some((|| {
            let vec: Vec<Option<windows_core::IInspectable>> = items
                .iter()
                .map(|s| {
                    let r = windows_reference::IReference::from(s.as_str());
                    Some(r.into())
                })
                .collect();
            let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
            asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)
        })()),
        (Prop::Placeholder, PropValue::Str(s)) => Some(asb.put_PlaceholderText(s)),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let insp = windows_reference::IReference::from(s.as_str());
            asb.put_Header(&insp)
        })()),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| asb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    asb: &Xaml::AutoSuggestBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::AutoSuggestTextChanged => {
            revokers.push(
                asb.add_TextChanged(move |sender, args| {
                    let is_user_input = args
                        .as_ref()
                        .and_then(|a| a.get_Reason().ok())
                        .is_some_and(|r| r == Xaml::AutoSuggestionBoxTextChangeReason::UserInput);
                    if is_user_input {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.get_Text().ok())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    }
                })
                .unwrap(),
            );
        }
        Event::AutoSuggestQuerySubmitted => {
            revokers.push(
                asb.add_QuerySubmitted(move |_sender, args| {
                    let text = args
                        .as_ref()
                        .and_then(|a| a.get_QueryText().ok())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                })
                .unwrap(),
            );
        }
        Event::AutoSuggestSuggestionChosen => {
            revokers.push(
                asb.add_SuggestionChosen(move |_sender, args| {
                    let item = args
                        .as_ref()
                        .and_then(|a| a.get_SelectedItem().ok())
                        .and_then(|insp| {
                            insp.cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                .ok()
                                .and_then(|pv| pv.Value().ok())
                        })
                        .map(|h| h.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(item);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
