use super::*;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RichTextLineBreak;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RichTextRun {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_strikethrough: bool,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
}

impl RichTextRun {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RichTextHyperlink {
    pub text: String,
    pub uri: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RichTextInline {
    Run(RichTextRun),
    LineBreak,
    Hyperlink(RichTextHyperlink),
}

impl Default for RichTextInline {
    fn default() -> Self {
        Self::Run(RichTextRun::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RichTextParagraph {
    pub inlines: Vec<RichTextInline>,
}

impl RichTextParagraph {
    pub fn new(inlines: Vec<RichTextInline>) -> Self {
        Self { inlines }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RichTextBlock {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub paragraphs: Vec<RichTextParagraph>,
    pub font_size: Option<f64>,
    pub is_text_selection_enabled: bool,
    pub text_wrapping_wrap: bool,
}

/// Backward-compat alias.
pub type RichText = RichTextBlock;

impl RichTextBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single_paragraph(inlines: Vec<RichTextInline>) -> Self {
        Self {
            paragraphs: vec![RichTextParagraph::new(inlines)],
            ..Self::default()
        }
    }

    pub fn font_size(mut self, size: f64) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn selectable(mut self) -> Self {
        self.is_text_selection_enabled = true;
        self
    }

    pub fn wrap(mut self) -> Self {
        self.text_wrapping_wrap = true;
        self
    }
}

impl Widget for RichTextBlock {
    widget_header!(ControlKind::RichTextBlock);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(3);
        if let Some(fs) = self.font_size {
            out.push(Binding::Prop(Prop::FontSize, PropValue::F64(fs)));
        }
        if self.is_text_selection_enabled {
            out.push(Binding::Prop(
                Prop::IsTextSelectionEnabled,
                PropValue::Bool(true),
            ));
        }
        if self.text_wrapping_wrap {
            out.push(Binding::Prop(Prop::TextWrappingWrap, PropValue::Bool(true)));
        }
        out
    }
}
