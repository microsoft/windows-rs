#![windows_subsystem = "windows"]

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), r"\bindings.rs"));
}

use bindings::*;
use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_text::{
    HitTest, LineMetrics, TextFormat, TextLayout, VerticalGlyphOrientation, WritingMode,
};
use windows_window::{Window, run_with};

const MARGIN: f32 = 24.0;
const HEADER: f32 = 72.0;
const FOOTER: f32 = 48.0;
const TEXT: &str = include_str!(r"..\fixtures\balloon.txt");
const CHOICES: [&str; 2] = ["もう一度", "おしまい"];

struct Panel {
    title: &'static str,
    mode: WritingMode,
    layout: TextLayout,
    lines: Vec<LineMetrics>,
    origin: Vector2,
    size: Vector2,
}

impl Panel {
    fn new(title: &'static str, mode: WritingMode, orientation: VerticalGlyphOrientation) -> Self {
        let format = TextFormat::new("Yu Gothic", 18.0)
            .with_locale("ja-jp")
            .with_writing_mode(mode)
            .with_vertical_glyph_orientation(orientation);
        let mut layout = TextLayout::new(TEXT, &format, 0.0, 0.0);
        for choice in CHOICES {
            let start = TEXT.find(choice).unwrap();
            layout.set_underline(start..start + choice.len(), true);
        }
        Self {
            title,
            mode,
            layout,
            lines: Vec::new(),
            origin: Vector2::zero(),
            size: Vector2::zero(),
        }
    }
}

struct Graphics {
    _device: GpuDevice,
    chain: SwapChain,
}

struct State {
    graphics: Option<Graphics>,
    panels: Vec<Panel>,
    pointer: Option<Vector2>,
    selected: Option<usize>,
    show_bands: bool,
    dirty: bool,
}

impl State {
    fn arrange(&mut self, width: f32, height: f32) {
        let size = Vector2::new(
            ((width - MARGIN * 3.0) / 2.0).max(0.0),
            ((height - HEADER - FOOTER - MARGIN * 3.0) / 2.0 - 28.0).max(0.0),
        );
        for (index, panel) in self.panels.iter_mut().enumerate() {
            panel.origin = Vector2::new(
                MARGIN + (index % 2) as f32 * (size.x + MARGIN),
                HEADER + 28.0 + (index / 2) as f32 * (size.y + 28.0 + MARGIN),
            );
            if panel.size != size {
                panel.layout.set_max_size(size.x, size.y);
                panel.lines = panel.layout.line_metrics();
                panel.size = size;
            }
        }
    }

    fn pointer_hit(&self) -> Option<(usize, HitTest)> {
        let point = self.pointer?;
        self.panels.iter().enumerate().find_map(|(index, panel)| {
            let local = point - panel.origin;
            (local.x >= 0.0 && local.y >= 0.0 && local.x < panel.size.x && local.y < panel.size.y)
                .then(|| (index, panel.layout.hit_test_point(local)))
        })
    }

    fn render(&mut self, window: &Window) -> Result<bool> {
        if !self.dirty {
            return Ok(false);
        }
        let (width, height) = window.client_size();
        if width <= 0 || height <= 0 {
            return Ok(false);
        }
        let scale = unsafe { GetDpiForWindow(window.hwnd().cast()) } as f32 / 96.0;
        self.arrange(width as f32 / scale, height as f32 / scale);
        let result = self.frame(window, width as u32, height as u32, scale);
        match result {
            Ok(true) => self.dirty = false,
            Ok(false) => self.graphics = None,
            Err(error) if is_device_lost(error.code()) => self.graphics = None,
            Err(error) => return Err(error),
        }
        Ok(self.dirty)
    }

    fn frame(&mut self, window: &Window, width: u32, height: u32, scale: f32) -> Result<bool> {
        let hit = self.pointer_hit();
        if self.graphics.is_none() {
            let device = GpuDevice::new()?;
            let chain = device.create_swap_chain_for_window(window, width, height)?;
            self.graphics = Some(Graphics {
                _device: device,
                chain,
            });
        }
        let graphics = self.graphics.as_mut().unwrap();
        if (graphics.chain.width(), graphics.chain.height()) != (width, height) {
            graphics.chain.resize(width, height)?;
        }
        graphics.chain.set_dpi(96.0 * scale, 96.0 * scale);
        let session = graphics.chain.begin_draw()?;
        session.clear(ColorF::from_rgb8(16, 20, 28));
        let white = session.create_solid_brush(ColorF::WHITE)?;
        let accent = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
        let selection = session.create_solid_brush(ColorF::new(0.3, 0.6, 1.0, 0.3))?;
        let frame = session.create_solid_brush(ColorF::from_rgb8(80, 90, 110))?;
        let band = session.create_solid_brush(ColorF::new(0.7, 0.5, 1.0, 0.4))?;
        let label_format = TextFormat::new("Segoe UI", 14.0);
        let label = |text: &str, origin, width| {
            let layout = TextLayout::new(text, &label_format, f32::max(width, 0.0), 48.0);
            session.draw_text_layout(origin, &layout, &white);
        };
        label(
            "windows-text + Canvas - resize to reflow; click a choice; B: line bands.\n\
             Yu Gothic / ja-jp. Stacked Latin is not tate-chu-yoko.",
            Vector2::new(MARGIN, 12.0),
            width as f32 / scale - 2.0 * MARGIN,
        );
        for (index, panel) in self.panels.iter().enumerate() {
            let origin = panel.origin;
            let metrics = panel.layout.metrics();
            label(
                &format!("{} / {} lines", panel.title, metrics.line_count),
                origin - Vector2::new(0.0, 26.0),
                panel.size.x,
            );
            session.draw_rect(
                &Rect::from_xywh(origin.x, origin.y, panel.size.x, panel.size.y),
                &frame,
                1.0,
            );
            session.with_clip(
                &Rect::from_xywh(origin.x, origin.y, panel.size.x, panel.size.y),
                || {
                    if self.show_bands {
                        for line in &panel.lines {
                            session.draw_rect(&line.bounds.offset(origin.x, origin.y), &band, 1.0);
                            let bounds = line.bounds;
                            let (a, b) = match panel.mode {
                                WritingMode::HorizontalLeftToRight
                                | WritingMode::HorizontalRightToLeft => (
                                    Vector2::new(bounds.left, bounds.top + line.baseline),
                                    Vector2::new(bounds.right, bounds.top + line.baseline),
                                ),
                                WritingMode::VerticalRightToLeft => (
                                    Vector2::new(bounds.right - line.baseline, bounds.top),
                                    Vector2::new(bounds.right - line.baseline, bounds.bottom),
                                ),
                                WritingMode::VerticalLeftToRight => (
                                    Vector2::new(bounds.left + line.baseline, bounds.top),
                                    Vector2::new(bounds.left + line.baseline, bounds.bottom),
                                ),
                                _ => unreachable!(),
                            };
                            session.draw_line(a + origin, b + origin, &band, 1.0);
                        }
                    }
                    for (choice, text) in CHOICES.iter().enumerate() {
                        let start = TEXT.find(text).unwrap();
                        for bounds in panel.layout.range_bounds(start..start + text.len()) {
                            let rect = bounds.offset(origin.x, origin.y);
                            if self.selected == Some(choice) {
                                session.fill_rect(&rect, &selection);
                            }
                            session.draw_rect(&rect, &accent, 1.0);
                        }
                    }
                    session.draw_text_layout(origin, &panel.layout, &white);
                    if let Some((hovered, hit)) = &hit
                        && *hovered == index
                        && hit.is_inside
                    {
                        let caret = panel
                            .layout
                            .caret_bounds(hit.text_range.start, hit.is_trailing_hit);
                        session.draw_rect(&hit.bounds.offset(origin.x, origin.y), &white, 1.0);
                        session.draw_line(
                            Vector2::new(caret.left, caret.top) + origin,
                            Vector2::new(caret.right, caret.bottom) + origin,
                            &white,
                            2.0,
                        );
                    }
                },
            );
        }
        let summary = hit.map_or_else(
            || {
                "Hover to inspect cluster bounds and caret positions.\n\
                Click an outlined choice; selection uses UTF-8 positions."
                    .to_owned()
            },
            |(_, hit)| {
                format!(
                    "UTF-8 {}..{}. Selected: {}\n\
                     White: cluster/caret. Purple: line bands/baselines.",
                    hit.text_range.start,
                    hit.text_range.end,
                    self.selected.map_or("none", |index| CHOICES[index]),
                )
            },
        );
        label(
            &summary,
            Vector2::new(MARGIN, height as f32 / scale - FOOTER),
            width as f32 / scale - 2.0 * MARGIN,
        );
        drop(session);
        graphics.chain.present()
    }
}

fn choice(hit: &HitTest) -> Option<usize> {
    if !hit.is_inside {
        return None;
    }
    CHOICES.iter().position(|text| {
        let start = TEXT.find(text).unwrap();
        (start..start + text.len()).contains(&hit.text_range.start)
    })
}

fn main() -> Result<()> {
    let panels = [
        (
            "Horizontal",
            WritingMode::HorizontalLeftToRight,
            VerticalGlyphOrientation::Default,
        ),
        (
            "Vertical / columns left",
            WritingMode::VerticalRightToLeft,
            VerticalGlyphOrientation::Default,
        ),
        (
            "Vertical / columns right",
            WritingMode::VerticalLeftToRight,
            VerticalGlyphOrientation::Default,
        ),
        (
            "Vertical / stacked Latin",
            WritingMode::VerticalRightToLeft,
            VerticalGlyphOrientation::Stacked,
        ),
    ]
    .into_iter()
    .map(|(title, mode, orientation)| Panel::new(title, mode, orientation))
    .collect();
    let state = Rc::new(RefCell::new(State {
        graphics: None,
        panels,
        pointer: None,
        selected: None,
        show_bands: false,
        dirty: true,
    }));
    let messages = Rc::clone(&state);
    let resize = Rc::clone(&state);
    let window = Window::new("Vertical text")
        .client_size(1100, 900)
        .on_message(move |hwnd, message, wparam, lparam| {
            match message as i32 {
                WM_CHAR if matches!(char::from_u32(wparam as u32), Some('b' | 'B')) => {
                    let mut state = messages.borrow_mut();
                    state.show_bands = !state.show_bands;
                    state.dirty = true;
                }
                WM_MOUSEMOVE | WM_LBUTTONDOWN => {
                    let scale = unsafe { GetDpiForWindow(hwnd.cast()) } as f32 / 96.0;
                    let mut state = messages.borrow_mut();
                    state.pointer = Some(Vector2::new(
                        (lparam as i16) as f32 / scale,
                        ((lparam >> 16) as i16) as f32 / scale,
                    ));
                    if message as i32 == WM_LBUTTONDOWN {
                        state.selected = state.pointer_hit().and_then(|(_, hit)| choice(&hit));
                    }
                    state.dirty = true;
                }
                WM_DPICHANGED => {
                    let rect = unsafe { &*(lparam as *const RECT) };
                    unsafe {
                        SetWindowPos(
                            hwnd.cast(),
                            std::ptr::null_mut(),
                            rect.left,
                            rect.top,
                            rect.right - rect.left,
                            rect.bottom - rect.top,
                            (SWP_NOZORDER | SWP_NOACTIVATE) as u32,
                        )
                        .ok()
                        .unwrap();
                    }
                    messages.borrow_mut().dirty = true;
                }
                _ => return None,
            }
            Some(0)
        })
        .on_resize(move |_, _| resize.borrow_mut().dirty = true)
        .create()?;
    run_with(|| state.borrow_mut().render(&window))
}

#[test]
fn choices_use_public_utf8_hits_after_reflow() {
    for mode in [
        WritingMode::HorizontalLeftToRight,
        WritingMode::VerticalRightToLeft,
        WritingMode::VerticalLeftToRight,
    ] {
        let mut panel = Panel::new("", mode, VerticalGlyphOrientation::Default);
        for height in [700.0, 200.0] {
            panel.layout.set_max_size(700.0, height);
            for (index, text) in CHOICES.iter().enumerate() {
                let start = TEXT.find(text).unwrap();
                for bounds in panel.layout.range_bounds(start..start + text.len()) {
                    let hit = panel.layout.hit_test_point(Vector2::new(
                        (bounds.left + bounds.right) / 2.0,
                        (bounds.top + bounds.bottom) / 2.0,
                    ));
                    assert_eq!(choice(&hit), Some(index));
                    assert!(TEXT.is_char_boundary(hit.text_range.start));
                    assert!(TEXT.is_char_boundary(hit.text_range.end));
                }
            }
            assert_eq!(
                choice(&panel.layout.hit_test_point(Vector2::new(-100.0, -100.0))),
                None
            );
        }
    }
}
