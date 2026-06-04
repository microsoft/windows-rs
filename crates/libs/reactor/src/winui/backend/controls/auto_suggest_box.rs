//! Typed handler for the `AutoSuggestBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::AutoSuggestBoxWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(
    widget: &AutoSuggestBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::AutoSuggestBox(asb) = handle else {
        return Ok(());
    };

    if !widget.text.is_empty() {
        asb.put_Text(&widget.text)?;
    }

    set_items(asb, &widget.items)?;

    if let Some(ph) = &widget.placeholder {
        asb.put_PlaceholderText(ph)?;
    }
    if let Some(hd) = &widget.header {
        let insp = windows_reference::IReference::from(hd.as_str());
        asb.put_Header(&insp)?;
    }

    ctx.mount_event(
        &widget.on_text_changed,
        Event::AutoSuggestTextChanged,
        EventHandler::TextChanged,
    );
    ctx.mount_event(
        &widget.on_query_submitted,
        Event::AutoSuggestQuerySubmitted,
        EventHandler::TextChanged,
    );
    ctx.mount_event(
        &widget.on_suggestion_chosen,
        Event::AutoSuggestSuggestionChosen,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &AutoSuggestBoxWidget,
    new: &AutoSuggestBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::AutoSuggestBox(asb) = handle else {
        return Ok(());
    };

    if old.text != new.text {
        // Skip SetText when the control already has this value —
        // calling SetText during a user-initiated TextChanged
        // cycle steals focus from the input field.
        if asb.get_Text().ok().as_deref() != Some(new.text.as_str()) {
            asb.put_Text(&new.text)?;
        }
    }

    if old.items != new.items {
        set_items(asb, &new.items)?;
    }

    super::diff_opt!(
        old,
        new,
        placeholder,
        |ph| asb.put_PlaceholderText(ph),
        asb.put_PlaceholderText("")
    );

    super::diff_opt!(
        old,
        new,
        header,
        |hd| {
            let insp = windows_reference::IReference::from(hd.as_str());
            asb.put_Header(&insp)
        },
        asb.put_Header(None)
    );

    ctx.diff_event(
        &old.on_text_changed,
        &new.on_text_changed,
        Event::AutoSuggestTextChanged,
        EventHandler::TextChanged,
    );
    ctx.diff_event(
        &old.on_query_submitted,
        &new.on_query_submitted,
        Event::AutoSuggestQuerySubmitted,
        EventHandler::TextChanged,
    );
    ctx.diff_event(
        &old.on_suggestion_chosen,
        &new.on_suggestion_chosen,
        Event::AutoSuggestSuggestionChosen,
        EventHandler::TextChanged,
    );
    Ok(())
}

fn set_items(asb: &Xaml::AutoSuggestBox, items: &[String]) -> windows_core::Result<()> {
    let vec: Vec<Option<windows_core::IInspectable>> = items
        .iter()
        .map(|s| {
            let r = windows_reference::IReference::from(s.as_str());
            Some(r.into())
        })
        .collect();
    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
    asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)
}
