//! Pure conversion helpers between reactor's backend-facing types and
//! the underlying `Microsoft.UI.Xaml` types. These are free functions
//! that touch neither `WinUIBackend` state nor the `Handle` enum — see
//! `winui/backend/mod.rs` for the dispatch tables and `Handle`-aware
//! helpers.

use super::*;
use bindings::Color as WinColor;

pub(super) fn to_xaml_thickness(t: Thickness) -> bindings::Thickness {
    bindings::Thickness {
        Left: t.left,
        Top: t.top,
        Right: t.right,
        Bottom: t.bottom,
    }
}

pub(super) fn reactor_key_to_virtual_key(k: KeyboardKey) -> bindings::VirtualKey {
    use KeyboardKey as K;
    match k {
        K::A => bindings::VirtualKey::A,
        K::B => bindings::VirtualKey::B,
        K::C => bindings::VirtualKey::C,
        K::D => bindings::VirtualKey::D,
        K::E => bindings::VirtualKey::E,
        K::F => bindings::VirtualKey::F,
        K::G => bindings::VirtualKey::G,
        K::H => bindings::VirtualKey::H,
        K::I => bindings::VirtualKey::I,
        K::J => bindings::VirtualKey::J,
        K::K => bindings::VirtualKey::K,
        K::L => bindings::VirtualKey::L,
        K::M => bindings::VirtualKey::M,
        K::N => bindings::VirtualKey::N,
        K::O => bindings::VirtualKey::O,
        K::P => bindings::VirtualKey::P,
        K::Q => bindings::VirtualKey::Q,
        K::R => bindings::VirtualKey::R,
        K::S => bindings::VirtualKey::S,
        K::T => bindings::VirtualKey::T,
        K::U => bindings::VirtualKey::U,
        K::V => bindings::VirtualKey::V,
        K::W => bindings::VirtualKey::W,
        K::X => bindings::VirtualKey::X,
        K::Y => bindings::VirtualKey::Y,
        K::Z => bindings::VirtualKey::Z,
        K::Num0 => bindings::VirtualKey::Number0,
        K::Num1 => bindings::VirtualKey::Number1,
        K::Num2 => bindings::VirtualKey::Number2,
        K::Num3 => bindings::VirtualKey::Number3,
        K::Num4 => bindings::VirtualKey::Number4,
        K::Num5 => bindings::VirtualKey::Number5,
        K::Num6 => bindings::VirtualKey::Number6,
        K::Num7 => bindings::VirtualKey::Number7,
        K::Num8 => bindings::VirtualKey::Number8,
        K::Num9 => bindings::VirtualKey::Number9,
        K::NumPad0 => bindings::VirtualKey::NumberPad0,
        K::NumPad1 => bindings::VirtualKey::NumberPad1,
        K::NumPad2 => bindings::VirtualKey::NumberPad2,
        K::NumPad3 => bindings::VirtualKey::NumberPad3,
        K::NumPad4 => bindings::VirtualKey::NumberPad4,
        K::NumPad5 => bindings::VirtualKey::NumberPad5,
        K::NumPad6 => bindings::VirtualKey::NumberPad6,
        K::NumPad7 => bindings::VirtualKey::NumberPad7,
        K::NumPad8 => bindings::VirtualKey::NumberPad8,
        K::NumPad9 => bindings::VirtualKey::NumberPad9,
        K::NumPadAdd => bindings::VirtualKey::Add,
        K::NumPadSubtract => bindings::VirtualKey::Subtract,
        K::NumPadMultiply => bindings::VirtualKey::Multiply,
        K::NumPadDivide => bindings::VirtualKey::Divide,
        K::NumPadDecimal => bindings::VirtualKey::Decimal,
        K::F1 => bindings::VirtualKey::F1,
        K::F2 => bindings::VirtualKey::F2,
        K::F3 => bindings::VirtualKey::F3,
        K::F4 => bindings::VirtualKey::F4,
        K::F5 => bindings::VirtualKey::F5,
        K::F6 => bindings::VirtualKey::F6,
        K::F7 => bindings::VirtualKey::F7,
        K::F8 => bindings::VirtualKey::F8,
        K::F9 => bindings::VirtualKey::F9,
        K::F10 => bindings::VirtualKey::F10,
        K::F11 => bindings::VirtualKey::F11,
        K::F12 => bindings::VirtualKey::F12,
        K::Enter => bindings::VirtualKey::Enter,
        K::Escape => bindings::VirtualKey::Escape,
        K::Tab => bindings::VirtualKey::Tab,
        K::Space => bindings::VirtualKey::Space,
        K::Backspace => bindings::VirtualKey::Back,
        K::Delete => bindings::VirtualKey::Delete,
        K::Insert => bindings::VirtualKey::Insert,
        K::Home => bindings::VirtualKey::Home,
        K::End => bindings::VirtualKey::End,
        K::PageUp => bindings::VirtualKey::PageUp,
        K::PageDown => bindings::VirtualKey::PageDown,
        K::Left => bindings::VirtualKey::Left,
        K::Right => bindings::VirtualKey::Right,
        K::Up => bindings::VirtualKey::Up,
        K::Down => bindings::VirtualKey::Down,
    }
}

pub(super) fn to_xaml_gridlength(v: GridLength) -> Result<bindings::GridLength> {
    use bindings::GridUnitType;
    match v {
        GridLength::Auto => Ok(bindings::GridLength {
            Value: 0.0,
            GridUnitType: GridUnitType::Auto,
        }),
        GridLength::Pixel(v) => Ok(bindings::GridLength {
            Value: v,
            GridUnitType: GridUnitType::Pixel,
        }),
        GridLength::Star(v) => Ok(bindings::GridLength {
            Value: v,
            GridUnitType: GridUnitType::Star,
        }),
    }
}

pub(super) fn solid_brush(c: Color) -> Result<bindings::SolidColorBrush> {
    let brush = bindings::SolidColorBrush::new()?;
    brush.put_Color(WinColor {
        A: c.a,
        R: c.r,
        G: c.g,
        B: c.b,
    })?;
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
