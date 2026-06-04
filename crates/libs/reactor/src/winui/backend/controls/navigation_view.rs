//! Typed handler for the `NavigationView` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::navigation_view::{NavViewPaneDisplayMode, NavigationView};
use crate::winui::backend::Handle;
use crate::winui::backend::convert::{build_nav_view_item, select_nav_item_by_tag};
use windows_collections;
use windows_core::Interface;
use windows_reference;

pub fn mount(w: &NavigationView, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let nv: Xaml::NavigationView = handle.cast_inner()?;

    // Menu items
    let menu = nv.get_MenuItems()?;
    for item in &w.menu_items {
        let nv_item = build_nav_view_item(item)?;
        menu.Append(&nv_item)?;
    }
    // Pane
    if let Some(pane_open) = w.is_pane_open {
        nv.put_IsPaneOpen(pane_open)?;
    }
    if w.pane_display_mode != NavViewPaneDisplayMode::default() {
        let nv2 = nv.cast::<Xaml::INavigationView2>()?;
        nv2.put_PaneDisplayMode(map_pane_mode(w.pane_display_mode))?;
    }
    if w.is_back_enabled {
        nv.cast::<Xaml::INavigationView2>()?
            .put_IsBackEnabled(true)?;
    }
    if !w.is_settings_visible {
        nv.put_IsSettingsVisible(false)?;
    }
    if !w.is_pane_toggle_button_visible {
        nv.put_IsPaneToggleButtonVisible(false)?;
    }
    if !w.is_back_button_visible {
        nv.cast::<Xaml::INavigationView2>()?
            .put_IsBackButtonVisible(Xaml::NavigationViewBackButtonVisible::Collapsed)?;
    }
    if let Some(ref t) = w.pane_title {
        nv.cast::<Xaml::INavigationView2>()?
            .put_PaneTitle(t.as_str())?;
    }
    if let Some(ref h) = w.header {
        let tb = Xaml::TextBlock::new()?;
        tb.put_Text(h.as_str())?;
        nv.put_Header(&tb)?;
    }
    // Search box
    let has_search = w.auto_suggest_placeholder.is_some()
        || w.on_search_query_submitted.is_some()
        || w.on_search_text_changed.is_some()
        || w.on_search_suggestion_chosen.is_some();
    if has_search {
        let asb = Xaml::AutoSuggestBox::new()?;
        if let Some(ref ph) = w.auto_suggest_placeholder {
            asb.put_PlaceholderText(ph.as_str())?;
        }
        if !w.auto_suggest_items.is_empty() {
            set_suggest_items(&asb, &w.auto_suggest_items)?;
        }
        nv.put_AutoSuggestBox(&asb)?;
    }
    // Selected tag (after items are built)
    if let Some(ref tag) = w.selected_tag {
        select_nav_item_by_tag(&nv, tag)?;
    }
    // Events
    ctx.mount_event(
        &w.on_selection_changed,
        Event::NavSelectionChanged,
        EventHandler::TabKey,
    );
    ctx.mount_event(
        &w.on_back_requested,
        Event::NavBackRequested,
        EventHandler::Click,
    );
    ctx.mount_event(
        &w.on_search_query_submitted,
        Event::NavSearchQuerySubmitted,
        EventHandler::TextChanged,
    );
    ctx.mount_event(
        &w.on_search_text_changed,
        Event::NavSearchTextChanged,
        EventHandler::TextChanged,
    );
    ctx.mount_event(
        &w.on_search_suggestion_chosen,
        Event::NavSearchSuggestionChosen,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &NavigationView,
    new: &NavigationView,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let nv: Xaml::NavigationView = handle.cast_inner()?;

    // Menu items
    if new.menu_items != old.menu_items {
        let menu = nv.get_MenuItems()?;
        menu.Clear()?;
        for item in &new.menu_items {
            let nv_item = build_nav_view_item(item)?;
            menu.Append(&nv_item)?;
        }
    }
    // Pane open
    if new.is_pane_open != old.is_pane_open {
        match new.is_pane_open {
            Some(v) => nv.put_IsPaneOpen(v)?,
            None => nv.put_IsPaneOpen(true)?,
        }
    }
    // Pane display mode
    if new.pane_display_mode != old.pane_display_mode {
        nv.cast::<Xaml::INavigationView2>()?
            .put_PaneDisplayMode(map_pane_mode(new.pane_display_mode))?;
    }
    // Back enabled
    if new.is_back_enabled != old.is_back_enabled {
        nv.cast::<Xaml::INavigationView2>()?
            .put_IsBackEnabled(new.is_back_enabled)?;
    }
    // Settings visible
    if new.is_settings_visible != old.is_settings_visible {
        nv.put_IsSettingsVisible(new.is_settings_visible)?;
    }
    // Pane toggle button visible
    if new.is_pane_toggle_button_visible != old.is_pane_toggle_button_visible {
        nv.put_IsPaneToggleButtonVisible(new.is_pane_toggle_button_visible)?;
    }
    // Back button visible
    if new.is_back_button_visible != old.is_back_button_visible {
        let val = if new.is_back_button_visible {
            Xaml::NavigationViewBackButtonVisible::Auto
        } else {
            Xaml::NavigationViewBackButtonVisible::Collapsed
        };
        nv.cast::<Xaml::INavigationView2>()?
            .put_IsBackButtonVisible(val)?;
    }
    // Pane title
    if new.pane_title != old.pane_title {
        let nv2 = nv.cast::<Xaml::INavigationView2>()?;
        match &new.pane_title {
            Some(t) => nv2.put_PaneTitle(t.as_str())?,
            None => nv2.put_PaneTitle("")?,
        }
    }
    // Header
    if new.header != old.header {
        match &new.header {
            Some(h) => {
                let tb = Xaml::TextBlock::new()?;
                tb.put_Text(h.as_str())?;
                nv.put_Header(&tb)?;
            }
            None => nv.put_Header(None)?,
        }
    }
    // Search box
    let old_has_search = old.auto_suggest_placeholder.is_some()
        || old.on_search_query_submitted.is_some()
        || old.on_search_text_changed.is_some()
        || old.on_search_suggestion_chosen.is_some();
    let new_has_search = new.auto_suggest_placeholder.is_some()
        || new.on_search_query_submitted.is_some()
        || new.on_search_text_changed.is_some()
        || new.on_search_suggestion_chosen.is_some();

    if new_has_search != old_has_search {
        if new_has_search {
            let asb = Xaml::AutoSuggestBox::new()?;
            if let Some(ref ph) = new.auto_suggest_placeholder {
                asb.put_PlaceholderText(ph.as_str())?;
            }
            if !new.auto_suggest_items.is_empty() {
                set_suggest_items(&asb, &new.auto_suggest_items)?;
            }
            nv.put_AutoSuggestBox(&asb)?;
        } else {
            nv.put_AutoSuggestBox(None)?;
        }
    } else if new_has_search {
        if new.auto_suggest_placeholder != old.auto_suggest_placeholder
            && let Ok(asb) = nv.get_AutoSuggestBox()
        {
            asb.put_PlaceholderText(new.auto_suggest_placeholder.as_deref().unwrap_or(""))?;
        }
        if new.auto_suggest_items != old.auto_suggest_items
            && let Ok(asb) = nv.get_AutoSuggestBox()
        {
            set_suggest_items(&asb, &new.auto_suggest_items)?;
        }
    }
    // Selected tag
    if new.selected_tag != old.selected_tag {
        match &new.selected_tag {
            Some(tag) => select_nav_item_by_tag(&nv, tag)?,
            None => nv.put_SelectedItem(None)?,
        }
    }
    // Events
    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::NavSelectionChanged,
        EventHandler::TabKey,
    );
    ctx.diff_event(
        &old.on_back_requested,
        &new.on_back_requested,
        Event::NavBackRequested,
        EventHandler::Click,
    );
    ctx.diff_event(
        &old.on_search_query_submitted,
        &new.on_search_query_submitted,
        Event::NavSearchQuerySubmitted,
        EventHandler::TextChanged,
    );
    ctx.diff_event(
        &old.on_search_text_changed,
        &new.on_search_text_changed,
        Event::NavSearchTextChanged,
        EventHandler::TextChanged,
    );
    ctx.diff_event(
        &old.on_search_suggestion_chosen,
        &new.on_search_suggestion_chosen,
        Event::NavSearchSuggestionChosen,
        EventHandler::TextChanged,
    );
    Ok(())
}

fn map_pane_mode(mode: NavViewPaneDisplayMode) -> Xaml::NavigationViewPaneDisplayMode {
    match mode {
        NavViewPaneDisplayMode::Auto => Xaml::NavigationViewPaneDisplayMode::Auto,
        NavViewPaneDisplayMode::Left => Xaml::NavigationViewPaneDisplayMode::Left,
        NavViewPaneDisplayMode::Top => Xaml::NavigationViewPaneDisplayMode::Top,
        NavViewPaneDisplayMode::LeftCompact => Xaml::NavigationViewPaneDisplayMode::LeftCompact,
        NavViewPaneDisplayMode::LeftMinimal => Xaml::NavigationViewPaneDisplayMode::LeftMinimal,
    }
}

fn set_suggest_items(asb: &Xaml::AutoSuggestBox, items: &[String]) -> windows_core::Result<()> {
    let vec: Vec<Option<windows_core::IInspectable>> = items
        .iter()
        .map(|s| {
            let r = windows_reference::IReference::from(s.as_str());
            Some(r.into())
        })
        .collect();
    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
    asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)?;
    Ok(())
}
