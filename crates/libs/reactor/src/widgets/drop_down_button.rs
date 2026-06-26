use super::menu_bar::MenuItemDef;
use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DropDownButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Option<String>,
    pub is_enabled: bool,
    pub on_click: Option<Callback<()>>,
    pub menu_flyout_items: Option<Vec<MenuItemDef>>,
    pub on_item_clicked: Option<Callback<String>>,
}

impl DropDownButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_click = Some(f.into_unit_callback());
        self
    }

    /// Attach a menu flyout with the given items.
    pub fn menu_flyout(mut self, items: Vec<MenuItemDef>) -> Self {
        self.menu_flyout_items = Some(items);
        self
    }

    /// Callback invoked when any flyout item is clicked (receives item text).
    pub fn on_item_clicked(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_item_clicked = Some(f.into_callback());
        self
    }
}

impl Widget for DropDownButton {
    widget_header!(ControlKind::DropDownButton);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::drop_down_button_bindings(self);
        if let Some(v) = &self.menu_flyout_items {
            out.push(Binding::Prop(
                Prop::MenuFlyoutItems,
                PropValue::MenuFlyoutItems(v.clone()),
            ));
        }
        out
    }
}

pub fn drop_down_button(content: impl Into<String>) -> DropDownButton {
    DropDownButton::new(content)
}
