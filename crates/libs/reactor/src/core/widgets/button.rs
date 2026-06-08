use super::command_bar::CommandBarCommandDef;
use super::menu_bar::MenuItemDef;
use super::*;

/// Visual style for a [`Button`].
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub enum ButtonStyle {
    /// Standard button (framework default).
    #[default]
    Default,
    /// Accent-colored button (primary action).
    Accent,
    /// Chromeless subtle button (secondary action).
    Subtle,
    /// Borderless text-link style (inline hyperlink pattern).
    TextLink,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Button {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub is_enabled: bool,
    pub style: ButtonStyle,
    pub icon: Option<SymbolGlyph>,
    pub on_click: Option<Callback<()>>,
    pub flyout: Option<FlyoutDef>,
    pub menu_flyout_items: Option<Vec<MenuItemDef>>,
    pub on_item_clicked: Option<Callback<String>>,
    pub command_bar_flyout_primary: Option<Vec<CommandBarCommandDef>>,
    pub command_bar_flyout_secondary: Option<Vec<CommandBarCommandDef>>,
    pub on_command_bar_flyout_click: Option<Callback<String>>,
}
impl Button {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_enabled: true,
            ..Default::default()
        }
    }
}

impl Widget for Button {
    widget_header!(ControlKind::Button);
    fn bindings(&self) -> PropBindings {
        let mut out = crate::core::generated_bindings::button_bindings(self);
        // Flyout and CommandBarFlyout are compound types not in TOML.
        if let Some(ref fly) = self.flyout {
            out.push(Binding::Prop(
                Prop::FlyoutContent,
                PropValue::Str(fly.text.clone()),
            ));
            if fly.placement != FlyoutPlacement::default() {
                out.push(Binding::Prop(
                    Prop::FlyoutPlacement,
                    PropValue::FlyoutPlacement(fly.placement),
                ));
            }
        }
        if let Some(ref primary) = self.command_bar_flyout_primary {
            out.push(Binding::Prop(
                Prop::CommandBarFlyoutCommands,
                PropValue::CommandBarFlyoutDef {
                    primary: primary.clone(),
                    secondary: self
                        .command_bar_flyout_secondary
                        .clone()
                        .unwrap_or_default(),
                },
            ));
        }
        out
    }
}

impl Button {
    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(crate::core::callback::Callback::new(move |()| f()));
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn accent(mut self) -> Self {
        self.style = ButtonStyle::Accent;
        self
    }

    pub fn subtle(mut self) -> Self {
        self.style = ButtonStyle::Subtle;
        self
    }

    pub fn text_link(mut self) -> Self {
        self.style = ButtonStyle::TextLink;
        self
    }

    pub fn icon(mut self, sym: SymbolGlyph) -> Self {
        self.icon = Some(sym);
        self
    }

    pub fn flyout(mut self, text: impl Into<String>) -> Self {
        self.flyout = Some(FlyoutDef {
            text: text.into(),
            placement: FlyoutPlacement::default(),
        });
        self
    }

    pub fn flyout_with_placement(
        mut self,
        text: impl Into<String>,
        placement: FlyoutPlacement,
    ) -> Self {
        self.flyout = Some(FlyoutDef {
            text: text.into(),
            placement,
        });
        self
    }

    pub fn menu_flyout(mut self, items: Vec<MenuItemDef>) -> Self {
        self.menu_flyout_items = Some(items);
        self
    }

    pub fn on_item_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_clicked = Some(Callback::new(f));
        self
    }

    pub fn command_bar_flyout(mut self, primary: Vec<CommandBarCommandDef>) -> Self {
        self.command_bar_flyout_primary = Some(primary);
        self
    }

    pub fn command_bar_flyout_secondary(mut self, secondary: Vec<CommandBarCommandDef>) -> Self {
        self.command_bar_flyout_secondary = Some(secondary);
        self
    }

    pub fn on_command_bar_flyout_click<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_command_bar_flyout_click = Some(Callback::new(f));
        self
    }
}

pub fn button(content: impl Into<String>) -> Button {
    Button::new(content)
}
