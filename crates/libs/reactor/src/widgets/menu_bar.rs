use super::*;

/// Definition of a single item within a menu (used by both [`MenuBar`]
/// and the menu flyout modifier on buttons).
#[derive(Clone, Debug, PartialEq)]
pub enum MenuItemDef {
    /// A clickable menu item with a text label.
    Item {
        text: String,
        /// Whether the item is interactive. Disabled items are greyed out
        /// and do not raise click events.
        is_enabled: bool,
    },
    /// A visual separator line.
    Separator,
    /// A submenu containing nested items.
    SubItem {
        text: String,
        children: Vec<Self>,
    },
}

/// Builder for an enabled [`MenuItemDef::Item`].
pub fn menu_item(text: impl Into<String>) -> MenuItemDef {
    MenuItemDef::Item {
        text: text.into(),
        is_enabled: true,
    }
}

/// Builder for a disabled (greyed-out) [`MenuItemDef::Item`].
pub fn menu_item_disabled(text: impl Into<String>) -> MenuItemDef {
    MenuItemDef::Item {
        text: text.into(),
        is_enabled: false,
    }
}

/// Builder for a [`MenuItemDef::Separator`].
pub fn menu_separator() -> MenuItemDef {
    MenuItemDef::Separator
}

/// Builder for a [`MenuItemDef::SubItem`].
pub fn menu_sub_item(text: impl Into<String>, children: Vec<MenuItemDef>) -> MenuItemDef {
    MenuItemDef::SubItem {
        text: text.into(),
        children,
    }
}

/// Definition of a top-level menu in a [`MenuBar`].
#[derive(Clone, Debug, PartialEq)]
pub struct MenuBarItemDef {
    pub title: String,
    pub items: Vec<MenuItemDef>,
}

impl MenuBarItemDef {
    pub fn new(title: impl Into<String>, items: Vec<MenuItemDef>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

/// Builder for a [`MenuBarItemDef`].
pub fn menu_bar_item(title: impl Into<String>, items: Vec<MenuItemDef>) -> MenuBarItemDef {
    MenuBarItemDef::new(title, items)
}

/// `Microsoft.UI.Xaml.Controls.MenuBar`. A horizontal bar of dropdown menus.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct MenuBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<MenuBarItemDef>,
    pub on_item_clicked: Option<Callback<String>>,
}

impl MenuBar {
    pub fn new(items: Vec<MenuBarItemDef>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    pub fn on_item_clicked(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_item_clicked = Some(f.into_callback());
        self
    }
}

impl Widget for MenuBar {
    widget_header!(ControlKind::MenuBar);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::menu_bar_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::MenuBarItems(self.items.clone()),
        ));
        out
    }
}

pub fn menu_bar(items: Vec<MenuBarItemDef>) -> MenuBar {
    MenuBar::new(items)
}
