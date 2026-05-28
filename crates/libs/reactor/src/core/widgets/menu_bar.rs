use super::*;

/// Definition of a single item within a menu (used by both [`MenuBarWidget`]
/// and the menu flyout modifier on buttons).
#[derive(Clone, Debug, PartialEq)]
pub enum MenuItemDef {
    /// A clickable menu item with a text label.
    Item { text: String },
    /// A visual separator line.
    Separator,
    /// A submenu containing nested items.
    SubItem {
        text: String,
        children: Vec<MenuItemDef>,
    },
}

/// Builder for a [`MenuItemDef::Item`].
pub fn menu_item(text: impl Into<String>) -> MenuItemDef {
    MenuItemDef::Item { text: text.into() }
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

/// Definition of a top-level menu in a [`MenuBarWidget`].
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
pub struct MenuBarWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<MenuBarItemDef>,
    pub on_item_click: Option<Callback<String>>,
}

impl MenuBarWidget {
    pub fn new(items: Vec<MenuBarItemDef>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    pub fn on_item_click<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_click = Some(Callback::new(f));
        self
    }
}

impl Widget for MenuBarWidget {
    widget_header!(ControlKind::MenuBar);
    fn bindings(&self) -> PropBindings {
        vec![
            Binding::Prop(
                Prop::MenuBarItems,
                PropValue::MenuBarItems(self.items.clone()),
            ),
            Binding::Event(
                Event::MenuBarItemClicked,
                self.on_item_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone())),
            ),
        ]
    }
}

pub fn menu_bar(items: Vec<MenuBarItemDef>) -> MenuBarWidget {
    MenuBarWidget::new(items)
}
