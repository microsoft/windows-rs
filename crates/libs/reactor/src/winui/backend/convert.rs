//! Pure conversion helpers between reactor's backend-facing types and
//! the underlying `Microsoft.UI.Xaml` types. These are free functions
//! that touch neither `WinUIBackend` state nor the `Handle` enum — see
//! `winui/backend/mod.rs` for the dispatch tables and `Handle`-aware
//! helpers.

use windows_core::Interface;

use crate::bindings as Xaml;
use crate::core::*;
use Xaml::Color as WinColor;

pub(super) fn to_xaml_thickness(t: Thickness) -> Xaml::Thickness {
    Xaml::Thickness {
        Left: t.left,
        Top: t.top,
        Right: t.right,
        Bottom: t.bottom,
    }
}

pub(super) fn reactor_key_to_virtual_key(k: KeyboardKey) -> Xaml::VirtualKey {
    use KeyboardKey as K;
    match k {
        K::A => Xaml::VirtualKey::A,
        K::B => Xaml::VirtualKey::B,
        K::C => Xaml::VirtualKey::C,
        K::D => Xaml::VirtualKey::D,
        K::E => Xaml::VirtualKey::E,
        K::F => Xaml::VirtualKey::F,
        K::G => Xaml::VirtualKey::G,
        K::H => Xaml::VirtualKey::H,
        K::I => Xaml::VirtualKey::I,
        K::J => Xaml::VirtualKey::J,
        K::K => Xaml::VirtualKey::K,
        K::L => Xaml::VirtualKey::L,
        K::M => Xaml::VirtualKey::M,
        K::N => Xaml::VirtualKey::N,
        K::O => Xaml::VirtualKey::O,
        K::P => Xaml::VirtualKey::P,
        K::Q => Xaml::VirtualKey::Q,
        K::R => Xaml::VirtualKey::R,
        K::S => Xaml::VirtualKey::S,
        K::T => Xaml::VirtualKey::T,
        K::U => Xaml::VirtualKey::U,
        K::V => Xaml::VirtualKey::V,
        K::W => Xaml::VirtualKey::W,
        K::X => Xaml::VirtualKey::X,
        K::Y => Xaml::VirtualKey::Y,
        K::Z => Xaml::VirtualKey::Z,
        K::Num0 => Xaml::VirtualKey::Number0,
        K::Num1 => Xaml::VirtualKey::Number1,
        K::Num2 => Xaml::VirtualKey::Number2,
        K::Num3 => Xaml::VirtualKey::Number3,
        K::Num4 => Xaml::VirtualKey::Number4,
        K::Num5 => Xaml::VirtualKey::Number5,
        K::Num6 => Xaml::VirtualKey::Number6,
        K::Num7 => Xaml::VirtualKey::Number7,
        K::Num8 => Xaml::VirtualKey::Number8,
        K::Num9 => Xaml::VirtualKey::Number9,
        K::NumPad0 => Xaml::VirtualKey::NumberPad0,
        K::NumPad1 => Xaml::VirtualKey::NumberPad1,
        K::NumPad2 => Xaml::VirtualKey::NumberPad2,
        K::NumPad3 => Xaml::VirtualKey::NumberPad3,
        K::NumPad4 => Xaml::VirtualKey::NumberPad4,
        K::NumPad5 => Xaml::VirtualKey::NumberPad5,
        K::NumPad6 => Xaml::VirtualKey::NumberPad6,
        K::NumPad7 => Xaml::VirtualKey::NumberPad7,
        K::NumPad8 => Xaml::VirtualKey::NumberPad8,
        K::NumPad9 => Xaml::VirtualKey::NumberPad9,
        K::NumPadAdd => Xaml::VirtualKey::Add,
        K::NumPadSubtract => Xaml::VirtualKey::Subtract,
        K::NumPadMultiply => Xaml::VirtualKey::Multiply,
        K::NumPadDivide => Xaml::VirtualKey::Divide,
        K::NumPadDecimal => Xaml::VirtualKey::Decimal,
        K::F1 => Xaml::VirtualKey::F1,
        K::F2 => Xaml::VirtualKey::F2,
        K::F3 => Xaml::VirtualKey::F3,
        K::F4 => Xaml::VirtualKey::F4,
        K::F5 => Xaml::VirtualKey::F5,
        K::F6 => Xaml::VirtualKey::F6,
        K::F7 => Xaml::VirtualKey::F7,
        K::F8 => Xaml::VirtualKey::F8,
        K::F9 => Xaml::VirtualKey::F9,
        K::F10 => Xaml::VirtualKey::F10,
        K::F11 => Xaml::VirtualKey::F11,
        K::F12 => Xaml::VirtualKey::F12,
        K::Enter => Xaml::VirtualKey::Enter,
        K::Escape => Xaml::VirtualKey::Escape,
        K::Tab => Xaml::VirtualKey::Tab,
        K::Space => Xaml::VirtualKey::Space,
        K::Backspace => Xaml::VirtualKey::Back,
        K::Delete => Xaml::VirtualKey::Delete,
        K::Insert => Xaml::VirtualKey::Insert,
        K::Home => Xaml::VirtualKey::Home,
        K::End => Xaml::VirtualKey::End,
        K::PageUp => Xaml::VirtualKey::PageUp,
        K::PageDown => Xaml::VirtualKey::PageDown,
        K::Left => Xaml::VirtualKey::Left,
        K::Right => Xaml::VirtualKey::Right,
        K::Up => Xaml::VirtualKey::Up,
        K::Down => Xaml::VirtualKey::Down,
    }
}

pub(super) fn to_xaml_gridlength(v: GridLength) -> windows_core::Result<Xaml::GridLength> {
    use Xaml::GridUnitType;
    match v {
        GridLength::Auto => Ok(Xaml::GridLength {
            Value: 0.0,
            GridUnitType: GridUnitType::Auto,
        }),
        GridLength::Pixel(v) => Ok(Xaml::GridLength {
            Value: v,
            GridUnitType: GridUnitType::Pixel,
        }),
        GridLength::Star(v) => Ok(Xaml::GridLength {
            Value: v,
            GridUnitType: GridUnitType::Star,
        }),
    }
}

pub(super) fn solid_brush(c: Color) -> windows_core::Result<Xaml::SolidColorBrush> {
    let brush = Xaml::SolidColorBrush::new()?;
    brush.put_Color(WinColor {
        A: c.a,
        R: c.r,
        G: c.g,
        B: c.b,
    })?;
    Ok(brush)
}

pub(super) fn brush_of(v: &Brush) -> windows_core::Result<Xaml::SolidColorBrush> {
    match v {
        Brush::Solid(c) => solid_brush(*c),
    }
}

pub(super) fn string_as_textblock(s: &str) -> windows_core::Result<Xaml::TextBlock> {
    let tb = Xaml::TextBlock::new()?;
    tb.put_Text(s)?;
    Ok(tb)
}

pub(super) fn build_nav_view_item(
    item: &NavViewItem,
) -> windows_core::Result<windows_core::IInspectable> {
    if item.is_header {
        let h = Xaml::NavigationViewItemHeader::new()?;
        let tb = string_as_textblock(&item.content)?;
        h.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;
        return h.cast();
    }
    let nv_item = Xaml::NavigationViewItem::new()?;
    let tb = string_as_textblock(&item.content)?;
    nv_item.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;
    let tag = item.tag.clone().unwrap_or_else(|| item.content.clone());
    let tag_inspectable = windows_reference::IReference::from(tag.as_str());
    nv_item
        .cast::<Xaml::IFrameworkElement>()?
        .put_Tag(&tag_inspectable)?;
    if let Some(sym) = &item.icon {
        let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
        nv_item.put_Icon(&icon_elem)?;
    }
    if !item.children.is_empty() {
        let menu = nv_item
            .cast::<Xaml::INavigationViewItem2>()?
            .get_MenuItems()?;
        for child in &item.children {
            let child_obj = build_nav_view_item(child)?;
            menu.Append(&child_obj)?;
        }
    }
    nv_item.cast()
}

fn nav_item_tag(item: &Xaml::NavigationViewItem) -> Option<String> {
    item.cast::<Xaml::IFrameworkElement>()
        .ok()?
        .get_Tag()
        .ok()?
        .cast::<windows_reference::IReference<windows_core::HSTRING>>()
        .ok()?
        .Value()
        .ok()
        .map(|s| s.to_string_lossy())
}

pub(super) fn select_nav_item_by_tag(
    nv: &Xaml::NavigationView,
    tag: &str,
) -> windows_core::Result<()> {
    let menu = nv.get_MenuItems()?;
    let len = menu.Size()?;

    for i in 0..len {
        let obj = menu.GetAt(i)?;
        let Ok(item) = obj.cast::<Xaml::NavigationViewItem>() else {
            continue;
        };
        if nav_item_tag(&item).as_deref() == Some(tag) {
            let inspectable: windows_core::IInspectable = item.cast()?;
            return nv.put_SelectedItem(&inspectable);
        }
        if let Ok(children) = item.cast::<Xaml::INavigationViewItem2>()?.get_MenuItems() {
            let child_count = children.Size().unwrap_or(0);
            for j in 0..child_count {
                let Ok(child_obj) = children.GetAt(j) else {
                    continue;
                };
                let Ok(child) = child_obj.cast::<Xaml::NavigationViewItem>() else {
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
) -> windows_core::Result<Xaml::MenuFlyoutItemBase> {
    match def {
        MenuItemDef::Item { text } => {
            let item = Xaml::MenuFlyoutItem::new()?;
            item.put_Text(text)?;
            item.cast()
        }
        MenuItemDef::Separator => {
            let sep = Xaml::MenuFlyoutSeparator::new()?;
            sep.cast()
        }
        MenuItemDef::SubItem { text, children } => {
            let sub = Xaml::MenuFlyoutSubItem::new()?;
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
pub(super) fn build_tree_view_node(def: &TreeNodeDef) -> windows_core::Result<Xaml::TreeViewNode> {
    let node = Xaml::TreeViewNode::new()?;
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
) -> windows_core::Result<Xaml::ICommandBarElement> {
    match def {
        CommandBarCommandDef::Button { label, icon } => {
            let btn = Xaml::AppBarButton::new()?;
            btn.put_Label(label)?;
            if let Some(sym) = icon {
                let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                btn.put_Icon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Toggle { label, icon } => {
            let btn = Xaml::AppBarToggleButton::new()?;
            btn.put_Label(label)?;
            if let Some(sym) = icon {
                let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                btn.put_Icon(&icon_elem)?;
            }
            btn.cast()
        }
        CommandBarCommandDef::Separator => {
            let sep = Xaml::AppBarSeparator::new()?;
            sep.cast()
        }
    }
}
