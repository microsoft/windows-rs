use super::menu_bar::MenuItemDef;
use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DropDownButtonWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Option<String>,
    pub is_enabled: bool,
    pub on_click: Option<Callback<()>>,
    pub flyout_items: Option<Vec<MenuItemDef>>,
    pub on_flyout_item_click: Option<Callback<String>>,
}

impl DropDownButtonWidget {
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

    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Callback::new(move |()| f()));
        self
    }

    /// Attach a menu flyout with the given items.
    pub fn menu_flyout(mut self, items: Vec<MenuItemDef>) -> Self {
        self.flyout_items = Some(items);
        self
    }

    /// Callback invoked when any flyout item is clicked (receives item text).
    pub fn on_flyout_item_click<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_flyout_item_click = Some(Callback::new(f));
        self
    }
}

impl Widget for DropDownButtonWidget {
    widget_header!(ControlKind::DropDownButton);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        if let Some(s) = &self.content {
            out.push(Binding::Prop(
                Prop::ButtonContent,
                PropValue::Str(s.clone()),
            ));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        if let Some(items) = &self.flyout_items {
            // Event must be registered before Prop so handler is in the map
            // when set_prop builds the flyout and wires click listeners.
            out.push(Binding::Event(
                Event::MenuFlyoutItemClicked,
                self.on_flyout_item_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone())),
            ));
            out.push(Binding::Prop(
                Prop::MenuFlyoutItems,
                PropValue::MenuFlyoutItems(items.clone()),
            ));
        } else {
            out.push(Binding::Event(
                Event::MenuFlyoutItemClicked,
                self.on_flyout_item_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone())),
            ));
        }
        out.push(Binding::Event(
            Event::Click,
            self.on_click
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
}

pub fn drop_down_button(content: impl Into<String>) -> DropDownButtonWidget {
    DropDownButtonWidget::new(content)
}
