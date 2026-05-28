use super::*;

/// Definition of a single command in a [`CommandBarWidget`].
#[derive(Clone, Debug, PartialEq)]
pub enum CommandBarCommandDef {
    /// A clickable button with optional icon and label.
    Button {
        label: String,
        icon: Option<SymbolGlyph>,
    },
    /// A toggle button with optional icon and label.
    Toggle {
        label: String,
        icon: Option<SymbolGlyph>,
    },
    /// A visual separator.
    Separator,
}

/// Builder for a command bar button.
pub fn app_bar_button(label: impl Into<String>) -> CommandBarCommandDef {
    CommandBarCommandDef::Button {
        label: label.into(),
        icon: None,
    }
}

/// Builder for a command bar button with icon.
pub fn app_bar_button_icon(label: impl Into<String>, icon: SymbolGlyph) -> CommandBarCommandDef {
    CommandBarCommandDef::Button {
        label: label.into(),
        icon: Some(icon),
    }
}

/// Builder for a command bar toggle button.
pub fn app_bar_toggle(label: impl Into<String>) -> CommandBarCommandDef {
    CommandBarCommandDef::Toggle {
        label: label.into(),
        icon: None,
    }
}

/// Builder for a command bar toggle button with icon.
pub fn app_bar_toggle_icon(label: impl Into<String>, icon: SymbolGlyph) -> CommandBarCommandDef {
    CommandBarCommandDef::Toggle {
        label: label.into(),
        icon: Some(icon),
    }
}

/// Builder for a command bar separator.
pub fn app_bar_separator() -> CommandBarCommandDef {
    CommandBarCommandDef::Separator
}

/// Default label position for a [`CommandBarWidget`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CommandBarLabelPos {
    /// Labels appear below the icon.
    #[default]
    Bottom,
    /// Labels appear to the right of the icon.
    Right,
    /// Labels are hidden.
    Collapsed,
}

/// `Microsoft.UI.Xaml.Controls.CommandBar`. A toolbar with primary and secondary commands.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct CommandBarWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub primary_commands: Vec<CommandBarCommandDef>,
    pub secondary_commands: Vec<CommandBarCommandDef>,
    pub default_label_position: CommandBarLabelPos,
    pub on_click: Option<Callback<String>>,
}

impl CommandBarWidget {
    pub fn new(primary: Vec<CommandBarCommandDef>) -> Self {
        Self {
            primary_commands: primary,
            ..Default::default()
        }
    }

    pub fn secondary_commands(mut self, cmds: Vec<CommandBarCommandDef>) -> Self {
        self.secondary_commands = cmds;
        self
    }

    pub fn default_label_position(mut self, pos: CommandBarLabelPos) -> Self {
        self.default_label_position = pos;
        self
    }

    pub fn on_click(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_click = Some(f.into_callback());
        self
    }
}

impl Widget for CommandBarWidget {
    widget_header!(ControlKind::CommandBar);
    fn bindings(&self) -> PropBindings {
        vec![
            Binding::Prop(
                Prop::CommandBarPrimaryCommands,
                PropValue::CommandBarCommands(self.primary_commands.clone()),
            ),
            Binding::Prop(
                Prop::CommandBarSecondaryCommands,
                PropValue::CommandBarCommands(self.secondary_commands.clone()),
            ),
            Binding::Prop(
                Prop::CommandBarDefaultLabelPosition,
                PropValue::CommandBarLabelPosition(self.default_label_position),
            ),
            Binding::Event(
                Event::CommandBarClick,
                self.on_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone())),
            ),
        ]
    }
}

pub fn command_bar(primary: Vec<CommandBarCommandDef>) -> CommandBarWidget {
    CommandBarWidget::new(primary)
}
