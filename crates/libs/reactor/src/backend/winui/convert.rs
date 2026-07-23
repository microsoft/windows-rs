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
    brush.SetColor(c)?;
    Ok(brush)
}

pub(super) fn string_as_textblock(s: &str) -> Result<bindings::TextBlock> {
    let tb = bindings::TextBlock::new()?;
    tb.SetText(s)?;
    Ok(tb)
}

/// Builds the WinUI `IconElement` for an [`Icon`], dispatching to the matching
/// concrete type: `SymbolIcon`, `BitmapIcon`, or `FontIcon`.
pub(super) fn build_icon_element(icon: &Icon) -> Result<bindings::IconElement> {
    match icon {
        Icon::Symbol(sym) => bindings::SymbolIcon::CreateInstanceWithSymbol(*sym)?.cast(),
        Icon::Bitmap { uri } => {
            let bitmap_icon = bindings::BitmapIcon::new()?;
            bitmap_icon.SetUriSource(&bindings::Uri::CreateUri(uri)?)?;
            // Render the source image in full color rather than tinting it to
            // the foreground brush (the WinUI default for a BitmapIcon).
            bitmap_icon.SetShowAsMonochrome(false)?;
            bitmap_icon.cast()
        }
        Icon::Font { glyph, family } => {
            let font_icon = bindings::FontIcon::new()?;
            font_icon.SetGlyph(glyph)?;
            if let Some(family) = family {
                font_icon.SetFontFamily(&bindings::FontFamily::CreateInstanceWithName(family)?)?;
            }
            font_icon.cast()
        }
    }
}

pub(super) fn build_nav_view_item(item: &NavViewItem) -> Result<windows_core::IInspectable> {
    if item.is_header {
        let h = bindings::NavigationViewItemHeader::new()?;
        let tb = string_as_textblock(&item.content)?;
        h.cast::<bindings::IContentControl>()?.SetContent(&tb)?;
        return h.cast();
    }
    let nv_item = bindings::NavigationViewItem::new()?;
    let tb = string_as_textblock(&item.content)?;
    nv_item
        .cast::<bindings::IContentControl>()?
        .SetContent(&tb)?;
    let tag = item.tag.clone().unwrap_or_else(|| item.content.clone());
    let tag_inspectable = windows_reference::IReference::from(tag.as_str());
    nv_item
        .cast::<bindings::IFrameworkElement>()?
        .SetTag(&tag_inspectable)?;
    if let Some(icon) = &item.icon {
        let icon_elem = build_icon_element(icon)?;
        nv_item.SetIcon(&icon_elem)?;
    }
    if !item.children.is_empty() {
        let menu = nv_item
            .cast::<bindings::INavigationViewItem2>()?
            .MenuItems()?;
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
        .Tag()
        .ok()?
        .cast::<windows_reference::IReference<windows_core::HSTRING>>()
        .ok()?
        .Value()
        .ok()
        .map(|s| s.to_string_lossy())
}

pub(super) fn select_nav_item_by_tag(nv: &bindings::NavigationView, tag: &str) -> Result<()> {
    let menu = nv.MenuItems()?;

    for obj in &menu {
        let Ok(item) = obj.cast::<bindings::NavigationViewItem>() else {
            continue;
        };
        if nav_item_tag(&item).as_deref() == Some(tag) {
            let inspectable: windows_core::IInspectable = item.cast()?;
            return nv.SetSelectedItem(&inspectable);
        }
        if let Ok(children) = item.cast::<bindings::INavigationViewItem2>()?.MenuItems() {
            for child_obj in &children {
                let Ok(child) = child_obj.cast::<bindings::NavigationViewItem>() else {
                    continue;
                };
                if nav_item_tag(&child).as_deref() == Some(tag) {
                    let inspectable: windows_core::IInspectable = child.cast()?;
                    return nv.SetSelectedItem(&inspectable);
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
        MenuItemDef::Item { text, is_enabled } => {
            let item = bindings::MenuFlyoutItem::new()?;
            item.SetText(text)?;
            item.cast::<bindings::IControl>()?
                .SetIsEnabled(*is_enabled)?;
            item.cast()
        }
        MenuItemDef::Separator => {
            let sep = bindings::MenuFlyoutSeparator::new()?;
            sep.cast()
        }
        MenuItemDef::SubItem { text, children } => {
            let sub = bindings::MenuFlyoutSubItem::new()?;
            sub.SetText(text)?;
            let sub_items = sub.Items()?;
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
    node.SetContent(&content)?;
    node.SetIsExpanded(def.is_expanded)?;
    if !def.children.is_empty() {
        let children = node.Children()?;
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
            btn.SetLabel(label)?;
            if let Some(icon) = icon {
                let icon_elem = build_icon_element(icon)?;
                btn.SetIcon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Toggle { label, icon } => {
            let btn = bindings::AppBarToggleButton::new()?;
            btn.SetLabel(label)?;
            if let Some(icon) = icon {
                let icon_elem = build_icon_element(icon)?;
                btn.SetIcon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Separator => {
            let sep = bindings::AppBarSeparator::new()?;
            sep.cast()
        }
    }
}
