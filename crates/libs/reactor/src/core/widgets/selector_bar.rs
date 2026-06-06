use super::*;

/// Definition of a single item in a [`SelectorBar`].
#[derive(Clone, Debug, PartialEq)]
pub struct SelectorBarItemDef {
    /// Display text.
    pub text: String,
    /// Optional symbol icon.
    pub icon: Option<SymbolGlyph>,
}

impl SelectorBarItemDef {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: SymbolGlyph) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Builder for a [`SelectorBarItemDef`].
pub fn selector_bar_item(text: impl Into<String>) -> SelectorBarItemDef {
    SelectorBarItemDef::new(text)
}

/// `Microsoft.UI.Xaml.Controls.SelectorBar`. A horizontal tab-like selector.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SelectorBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<SelectorBarItemDef>,
    pub on_selection_changed: Option<Callback<String>>,
}

impl SelectorBar {
    pub fn new(items: Vec<SelectorBarItemDef>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    pub fn on_selection_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
}

impl Widget for SelectorBar {
    widget_header!(ControlKind::SelectorBar);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::selector_bar_bindings(self)
    }
}

pub fn selector_bar(items: Vec<SelectorBarItemDef>) -> SelectorBar {
    SelectorBar::new(items)
}
