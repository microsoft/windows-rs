#![windows_subsystem = "windows"]

use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{
    LineSpacing, TextAlignment, TextFormat, TextLayout, Trimming, WordWrapping, WritingMode,
};

const MARGIN: f32 = 36.0;
const LABEL: f32 = 28.0;
const LINE: f32 = 24.0;
const COLUMN: f32 = 340.0;

const ENGLISH: &str = "Paragraphs wrap at word boundaries and reflow as the window resizes. \
    Each panel lays out this text with one format setting changed, and the frame shows the \
    layout box the text must fit.";

// Arabic: "Hello world. This paragraph reads from right to left, and wraps at word boundaries
// with numbers such as 2026 and English words in the middle."
const ARABIC: &str = concat!(
    "\u{645}\u{631}\u{62d}\u{628}\u{627} \u{628}\u{627}\u{644}\u{639}\u{627}\u{644}\u{645}. ",
    "\u{647}\u{630}\u{647} \u{627}\u{644}\u{641}\u{642}\u{631}\u{629} \u{62a}\u{64f}\u{642}",
    "\u{631}\u{623} \u{645}\u{646} \u{627}\u{644}\u{64a}\u{645}\u{64a}\u{646} \u{625}\u{644}",
    "\u{649} \u{627}\u{644}\u{64a}\u{633}\u{627}\u{631}\u{60c} \u{648}\u{62a}\u{644}\u{62a}",
    "\u{641} \u{639}\u{646}\u{62f} \u{62d}\u{62f}\u{648}\u{62f} \u{627}\u{644}\u{643}\u{644}",
    "\u{645}\u{627}\u{62a} \u{645}\u{639} \u{623}\u{631}\u{642}\u{627}\u{645} \u{645}\u{62b}",
    "\u{644} 2026 \u{648}\u{643}\u{644}\u{645}\u{627}\u{62a} English \u{641}\u{64a} \u{645}",
    "\u{646}\u{62a}\u{635}\u{641}\u{647}\u{627}."
);

const TABLE: &str = "Font\tWeight\tStyle\nSegoe UI\t400\tNormal\nCalibri\t700\tItalic\n\
    Consolas\t400\tNormal\nCambria\t300\tOblique";

// A titled panel and the format and text it lays out, in a box at most `lines` lines tall.
struct Panel {
    title: &'static str,
    text: &'static str,
    format: TextFormat,
    lines: Option<f32>,
}

fn panels() -> Vec<Panel> {
    let body = TextFormat::new("Segoe UI", 16.0);

    vec![
        Panel {
            title: "Leading alignment",
            text: ENGLISH,
            format: body.clone(),
            lines: None,
        },
        Panel {
            title: "Justified",
            text: ENGLISH,
            format: body.clone().with_alignment(TextAlignment::Justified),
            lines: None,
        },
        Panel {
            title: "Line spacing 1.6",
            text: ENGLISH,
            format: body
                .clone()
                .with_line_spacing(LineSpacing::Proportional(1.6)),
            lines: None,
        },
        Panel {
            title: "Trimmed to three lines",
            text: ENGLISH,
            format: body
                .clone()
                .with_line_spacing(LineSpacing::Uniform {
                    height: LINE,
                    baseline: 18.0,
                })
                .with_trimming(Trimming::Word),
            lines: Some(3.0),
        },
        Panel {
            title: "Right to left",
            text: ARABIC,
            format: body
                .clone()
                .with_locale("ar-eg")
                .with_writing_mode(WritingMode::HorizontalRightToLeft),
            lines: None,
        },
        Panel {
            title: "Tab stops",
            text: TABLE,
            format: body
                .with_tab_width(100.0)
                .with_word_wrapping(WordWrapping::NoWrap)
                .with_trimming(Trimming::Character),
            lines: None,
        },
    ]
}

fn draw(ctx: &DrawContext, panels: &[Panel]) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let dim = ctx.create_solid_brush(ColorF::from_rgb8(140, 150, 170))?;
    let frame = ctx.create_solid_brush(ColorF::from_rgb8(60, 70, 90))?;
    let title = TextFormat::new("Segoe UI", 14.0).with_trimming(Trimming::Character);

    // Three columns of at most `COLUMN` width, or fewer in a narrow window.
    let columns = ((ctx.width - MARGIN) / 260.0).clamp(1.0, 3.0) as usize;
    let rows = panels.len().div_ceil(columns);
    let width = ((ctx.width - MARGIN) / columns as f32 - MARGIN).clamp(0.0, COLUMN);
    let height = ((ctx.height - MARGIN) / rows as f32 - MARGIN - LABEL).clamp(0.0, COLUMN / 1.5);

    for (index, panel) in panels.iter().enumerate() {
        let x = MARGIN + (index % columns) as f32 * (width + MARGIN);
        let y = MARGIN + (index / columns) as f32 * (height + LABEL + MARGIN);

        let label = TextLayout::new(panel.title, &title, width, LABEL);
        ctx.draw_text_layout(Vector2::new(x, y), &label, &dim);

        let height = panel.lines.map_or(height, |lines| height.min(lines * LINE));

        let layout = TextLayout::new(panel.text, &panel.format, width, height);
        let origin = Vector2::new(x, y + LABEL);
        // The frame sits just outside the layout box.
        ctx.draw_rect(
            &Rect::from_xywh(origin.x - 6.0, origin.y - 6.0, width + 12.0, height + 12.0),
            &frame,
            1.0,
        );
        ctx.draw_text_layout(origin, &layout, &white);
    }

    Ok(())
}

struct Sample {
    panels: Rc<[Panel]>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            panels: panels().into(),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Text paragraphs");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let panels = self.panels.clone();
        Border::new()
            .content(canvas(move |ctx| draw(ctx, &panels)))
            .into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
