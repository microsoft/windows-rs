//! Pure conversion helpers between reactor's backend-facing types and
//! the underlying `Microsoft.UI.Xaml` types. These are free functions
//! that touch neither `WinUIBackend` state nor the `Handle` enum — see
//! `winui/backend/mod.rs` for the dispatch tables and `Handle`-aware
//! helpers.

use super::*;

pub(super) fn to_xaml_gridlength(v: GridLength) -> Result<bindings::GridLength> {
    use bindings::GridUnitType;
    match v {
        GridLength::Auto => Ok(bindings::GridLength {
            value: 0.0,
            grid_unit_type: GridUnitType::Auto,
        }),
        GridLength::Pixel(v) => Ok(bindings::GridLength {
            value: v,
            grid_unit_type: GridUnitType::Pixel,
        }),
        GridLength::Star(v) => Ok(bindings::GridLength {
            value: v,
            grid_unit_type: GridUnitType::Star,
        }),
    }
}

pub(super) fn solid_brush(c: Color) -> Result<bindings::SolidColorBrush> {
    let brush = bindings::SolidColorBrush::new()?;
    brush.put_Color(c)?;
    Ok(brush)
}

pub(super) fn brush_of(v: &Brush) -> Result<bindings::SolidColorBrush> {
    match v {
        Brush::Solid(c) => solid_brush(*c),
    }
}

pub(super) fn string_as_textblock(s: &str) -> Result<bindings::TextBlock> {
    let tb = bindings::TextBlock::new()?;
    tb.put_Text(s)?;
    Ok(tb)
}

pub(super) fn build_nav_view_item(item: &NavViewItem) -> Result<windows_core::IInspectable> {
    if item.is_header {
        let h = bindings::NavigationViewItemHeader::new()?;
        let tb = string_as_textblock(&item.content)?;
        h.cast::<bindings::IContentControl>()?.put_Content(&tb)?;
        return h.cast();
    }
    let nv_item = bindings::NavigationViewItem::new()?;
    let tb = string_as_textblock(&item.content)?;
    nv_item
        .cast::<bindings::IContentControl>()?
        .put_Content(&tb)?;
    let tag = item.tag.clone().unwrap_or_else(|| item.content.clone());
    let tag_inspectable = windows_reference::IReference::from(tag.as_str());
    nv_item
        .cast::<bindings::IFrameworkElement>()?
        .put_Tag(&tag_inspectable)?;
    if let Some(sym) = &item.icon {
        let icon_elem = bindings::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
        nv_item.put_Icon(&icon_elem)?;
    }
    if !item.children.is_empty() {
        let menu = nv_item
            .cast::<bindings::INavigationViewItem2>()?
            .get_MenuItems()?;
        for child in &item.children {
            let child_obj = build_nav_view_item(child)?;
            menu.Append(&child_obj)?;
        }
    }
    nv_item.cast()
}

fn nav_item_tag(item: &bindings::NavigationViewItem) -> Option<String> {
    item.cast::<bindings::IFrameworkElement>()
        .ok()?
        .get_Tag()
        .ok()?
        .cast::<windows_reference::IReference<windows_core::HSTRING>>()
        .ok()?
        .Value()
        .ok()
        .map(|s| s.to_string_lossy())
}

pub(super) fn select_nav_item_by_tag(nv: &bindings::NavigationView, tag: &str) -> Result<()> {
    let menu = nv.get_MenuItems()?;
    let len = menu.Size()?;

    for i in 0..len {
        let obj = menu.GetAt(i)?;
        let Ok(item) = obj.cast::<bindings::NavigationViewItem>() else {
            continue;
        };
        if nav_item_tag(&item).as_deref() == Some(tag) {
            let inspectable: windows_core::IInspectable = item.cast()?;
            return nv.put_SelectedItem(&inspectable);
        }
        if let Ok(children) = item
            .cast::<bindings::INavigationViewItem2>()?
            .get_MenuItems()
        {
            let child_count = children.Size().unwrap_or(0);
            for j in 0..child_count {
                let Ok(child_obj) = children.GetAt(j) else {
                    continue;
                };
                let Ok(child) = child_obj.cast::<bindings::NavigationViewItem>() else {
                    continue;
                };
                if nav_item_tag(&child).as_deref() == Some(tag) {
                    let inspectable: windows_core::IInspectable = child.cast()?;
                    return nv.put_SelectedItem(&inspectable);
                }
            }
        }
    }
    Ok(())
}

/// Build a `MenuFlyoutItemBase` from a [`MenuItemDef`].
pub(super) fn build_menu_flyout_item_base(
    def: &MenuItemDef,
) -> Result<bindings::MenuFlyoutItemBase> {
    match def {
        MenuItemDef::Item { text } => {
            let item = bindings::MenuFlyoutItem::new()?;
            item.put_Text(text)?;
            item.cast()
        }
        MenuItemDef::Separator => {
            let sep = bindings::MenuFlyoutSeparator::new()?;
            sep.cast()
        }
        MenuItemDef::SubItem { text, children } => {
            let sub = bindings::MenuFlyoutSubItem::new()?;
            sub.put_Text(text)?;
            let sub_items = sub.get_Items()?;
            for child in children {
                let child_item = build_menu_flyout_item_base(child)?;
                sub_items.Append(&child_item)?;
            }
            sub.cast()
        }
    }
}

/// Recursively build a `TreeViewNode` from a [`TreeNodeDef`].
pub(super) fn build_tree_view_node(def: &TreeNodeDef) -> Result<bindings::TreeViewNode> {
    let node = bindings::TreeViewNode::new()?;
    let content: windows_core::IInspectable =
        windows_reference::IReference::<windows_core::HSTRING>::from(windows_core::HSTRING::from(
            &def.text,
        ))
        .cast()?;
    node.put_Content(&content)?;
    node.put_IsExpanded(def.is_expanded)?;
    if !def.children.is_empty() {
        let children = node.get_Children()?;
        for child_def in &def.children {
            let child_node = build_tree_view_node(child_def)?;
            children.Append(&child_node)?;
        }
    }
    Ok(node)
}

/// Builds a WinUI `ICommandBarElement` from a [`CommandBarCommandDef`].
pub(super) fn build_command_bar_element(
    def: &CommandBarCommandDef,
) -> Result<bindings::ICommandBarElement> {
    match def {
        CommandBarCommandDef::Button { label, icon } => {
            let btn = bindings::AppBarButton::new()?;
            btn.put_Label(label)?;
            if let Some(sym) = icon {
                let icon_elem = bindings::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                btn.put_Icon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Toggle { label, icon } => {
            let btn = bindings::AppBarToggleButton::new()?;
            btn.put_Label(label)?;
            if let Some(sym) = icon {
                let icon_elem = bindings::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                btn.put_Icon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Separator => {
            let sep = bindings::AppBarSeparator::new()?;
            sep.cast()
        }
    }
}
