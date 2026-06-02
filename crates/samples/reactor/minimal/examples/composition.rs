//! Demonstrates `Element::Group` (fragments) and `CustomElement`.
//!
//! * `Group` contributes multiple siblings without an extra wrapper.
//! * `CustomElement` is the extension hatch for third-party widgets.

#![windows_subsystem = "windows"]

use std::any::Any;

use windows_reactor::core::backend::{Backend, ControlId, ControlKind, Prop, PropValue};
use windows_reactor::*;

fn labeled_row(label: &str, value: Element) -> Element {
    group(vec![
        text_block(label)
            .foreground(Color::rgb(120, 120, 120))
            .into(),
        value,
    ])
}

#[derive(Clone)]
struct BadgeButton {
    key: Option<String>,
    label: String,
    count: u32,
}

impl BadgeButton {
    fn new(label: impl Into<String>, count: u32) -> Self {
        Self {
            key: None,
            label: label.into(),
            count,
        }
    }
    fn rendered(&self) -> String {
        format!("{} ({})", self.label, self.count)
    }
}

impl CustomElement for BadgeButton {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn kind_name(&self) -> &'static str {
        "BadgeButton"
    }
    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool {
        other
            .as_any()
            .downcast_ref::<BadgeButton>()
            .is_some_and(|o| self.key == o.key && self.label == o.label && self.count == o.count)
    }
    fn clone_dyn(&self) -> Box<dyn CustomElement> {
        Box::new(self.clone())
    }
    fn mount(&self, backend: &mut dyn Backend) -> ControlId {
        let id = backend.create(ControlKind::Button);
        backend.set_prop(id, Prop::ButtonContent, PropValue::Str(self.rendered()));
        id
    }
    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend) {
        let prev = prev.as_any().downcast_ref::<BadgeButton>().unwrap();
        if prev.rendered() != self.rendered() {
            backend.set_prop(id, Prop::ButtonContent, PropValue::Str(self.rendered()));
        }
    }
}

impl From<BadgeButton> for Element {
    fn from(b: BadgeButton) -> Self {
        Element::Custom(CustomElementHandle::new(b))
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (inbox_count, set_inbox) = cx.use_state(3_u32);
    let (drafts_count, set_drafts) = cx.use_state(1_u32);
    let inbox_for_inc = inbox_count;
    let drafts_for_inc = drafts_count;

    vstack((
        TitleBar::new("windows_reactor — composition sample").subtitle("Group + CustomElement"),
        text_block("Settings (LabeledRow uses Element::Group)")
            .bold()
            .font_size(20.0),
        // Each labeled_row contributes two children directly into this vstack.
        vstack((
            labeled_row("Username", text_block("alice").into()),
            labeled_row("Theme", text_block("Dark").into()),
            labeled_row("Notifications", text_block("Enabled").into()),
        ))
        .spacing(6.0),
        text_block("Custom widgets (BadgeButton is a CustomElement)")
            .bold()
            .font_size(20.0),
        hstack((
            BadgeButton::new("Inbox", inbox_count),
            BadgeButton::new("Drafts", drafts_count),
            button("+ Inbox").on_click(move || set_inbox.call(inbox_for_inc + 1)),
            button("+ Drafts").on_click(move || set_drafts.call(drafts_for_inc + 1)),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new()
        .title("windows_reactor — composition")
        .render(app)
}
