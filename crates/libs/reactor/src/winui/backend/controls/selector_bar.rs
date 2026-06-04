//! Typed handler for the `SelectorBar` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::SelectorBarItemDef;
use crate::core::widgets::SelectorBarWidget;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &SelectorBarWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::SelectorBar(sb) = handle else {
        return Ok(());
    };

    set_items(sb, &widget.items)?;
    ctx.mount_event(
        &widget.on_selection_changed,
        Event::SelectorBarSelectionChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &SelectorBarWidget,
    new: &SelectorBarWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::SelectorBar(sb) = handle else {
        return Ok(());
    };

    if old.items != new.items {
        set_items(sb, &new.items)?;
    }

    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::SelectorBarSelectionChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}

fn set_items(sb: &Xaml::SelectorBar, items: &[SelectorBarItemDef]) -> windows_core::Result<()> {
    let vec = sb.get_Items()?;
    vec.Clear()?;
    for def in items {
        let item = Xaml::SelectorBarItem::new()?;
        item.put_Text(&def.text)?;
        if let Some(sym) = &def.icon {
            let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(sym.to_raw()))?;
            item.put_Icon(&icon_elem)?;
        }
        vec.Append(&item)?;
    }
    Ok(())
}
