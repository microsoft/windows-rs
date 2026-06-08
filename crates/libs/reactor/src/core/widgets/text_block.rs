use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TextWrapping {
    #[default]
    NoWrap,
    Wrap,
    WrapWholeWords,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TextBlock {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub text: String,
    pub font_size: Option<f64>,
    pub font_weight: Option<u16>,
    pub text_wrapping: TextWrapping,
    pub is_text_selection_enabled: bool,
}
impl TextBlock {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            text: content.into(),
            ..Self::default()
        }
    }
}

impl Widget for TextBlock {
    widget_header!(ControlKind::TextBlock);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::text_block_bindings(self)
    }
}

impl TextBlock {
    pub fn bold(mut self) -> Self {
        self.font_weight = Some(700);
        self
    }

    pub fn semibold(mut self) -> Self {
        self.font_weight = Some(600);
        self
    }

    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn font_size(mut self, v: f64) -> Self {
        self.font_size = Some(v);
        self
    }

    pub fn wrap(mut self) -> Self {
        self.text_wrapping = TextWrapping::Wrap;
        self
    }

    pub fn selectable(mut self) -> Self {
        self.is_text_selection_enabled = true;
        self
    }
}

pub fn text_block(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content)
}

// ── Type-ramp factories (WinUI 3 spec) ──────────────────────────────────────

/// 28 px Semibold — page/section titles.
pub fn title(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(28.0).semibold()
}

/// 20 px Semibold — secondary headings.
pub fn subtitle(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(20.0).semibold()
}

/// 18 px Normal — larger body text.
pub fn body_large(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(18.0)
}

/// 14 px Semibold — emphasized body text.
pub fn body_strong(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(14.0).semibold()
}

/// 14 px Normal — standard body text.
pub fn body(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(14.0)
}

/// 12 px Normal — captions and secondary labels.
pub fn caption(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content).font_size(12.0)
}
