use super::command_bar::CommandBarCommandDef;
use super::menu_bar::MenuItemDef;
use super::*;

/// Visual style for a [`Button`]. Not a WinRT enum — maps to resource key strings.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ButtonStyle(pub i32);
#[expect(non_upper_case_globals)]
impl ButtonStyle {
    /// Standard button (framework default).
    pub const Default: Self = Self(0);
    /// Accent-colored button (primary action).
    pub const Accent: Self = Self(1);
    /// Chromeless subtle button (secondary action).
    pub const Subtle: Self = Self(2);
    /// Borderless text-link style (inline hyperlink pattern).
    pub const TextLink: Self = Self(3);
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Button {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub is_enabled: bool,
    pub style: ButtonStyle,
    pub icon: Option<Icon>,
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
        let mut out = generated::button_bindings(self);
        out.push(Binding::Prop(
            Prop::Content,
            PropValue::Str(self.content.clone()),
        ));
        if let Some(v) = &self.icon {
            out.push(Binding::Prop(Prop::Icon, PropValue::Icon(v.clone())));
        }
        if let Some(v) = &self.menu_flyout_items {
            out.push(Binding::Prop(
                Prop::MenuFlyoutItems,
                PropValue::MenuFlyoutItems(v.clone()),
            ));
        }
        if self.style != ButtonStyle::Default {
            out.push(Binding::Prop(Prop::StyleVariant, PropValue::I32(self.style.0)));
        }
        // Flyout and CommandBarFlyout are compound types not in TOML.
        if let Some(ref fly) = self.flyout {
            out.push(Binding::Prop(
                Prop::FlyoutContent,
                PropValue::Str(fly.text.clone()),
            ));
            if fly.placement != FlyoutPlacementMode::default() {
                out.push(Binding::Prop(
                    Prop::FlyoutPlacement,
                    PropValue::I32(fly.placement.0),
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
    pub fn on_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_click = Some(f.into_unit_callback());
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

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn flyout(mut self, text: impl Into<String>) -> Self {
        self.flyout = Some(FlyoutDef {
            text: text.into(),
            placement: FlyoutPlacementMode::default(),
        });
        self
    }

    pub fn flyout_with_placement(
        mut self,
        text: impl Into<String>,
        placement: FlyoutPlacementMode,
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

    pub fn on_item_clicked(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_item_clicked = Some(f.into_callback());
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

    pub fn on_command_bar_flyout_click(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_command_bar_flyout_click = Some(f.into_callback());
        self
    }
}

pub fn button(content: impl Into<String>) -> Button {
    Button::new(content)
}
