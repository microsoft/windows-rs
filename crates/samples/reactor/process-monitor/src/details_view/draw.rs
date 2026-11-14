use super::*;
use state::{BAR, HEADER, ROW};
use std::collections::HashMap;
use windows_canvas::{Brush, ColorF, DrawingSession, Rect, Vector2};
use windows_text::{
    ParagraphAlignment, TextAlignment, TextFormat, TextLayout, Trimming, WordWrapping,
};

struct Cell {
    text: String,
    width: f32,
    generation: u64,
    layout: TextLayout,
}

struct Brushes {
    text: Brush,
    header: Brush,
    hover: Brush,
    selection: Brush,
    line: Brush,
    thumb: Brush,
}

pub(super) struct Renderer {
    formats: [TextFormat; 2],
    cells: HashMap<(Option<Key>, usize), Cell>,
    brushes: Option<(ColorScheme, Brushes)>,
    columns: Rc<[Column]>,
    visible_columns: Vec<usize>,
}

impl Renderer {
    #[cfg(test)]
    pub(super) fn cached_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn new() -> Self {
        let format = TextFormat::new("Segoe UI", 13.0)
            .with_word_wrapping(WordWrapping::NoWrap)
            .with_paragraph_alignment(ParagraphAlignment::Center)
            .with_trimming(Trimming::Character);
        Self {
            formats: [
                format.clone(),
                format.with_alignment(TextAlignment::Trailing),
            ],
            cells: HashMap::new(),
            brushes: None,
            columns: Rc::from([]),
            visible_columns: Vec::new(),
        }
    }

    fn cell<R: Rows>(
        &mut self,
        state: &State<R>,
        row: Option<usize>,
        column: usize,
        width: f32,
    ) -> &TextLayout {
        let key = (row.map(|row| state.input.rows.key(row)), column);
        let format =
            &self.formats[usize::from(row.is_some() && state.input.columns[column].right_aligned)];
        let text = || {
            row.map_or_else(
                || state.input.columns[column].title.to_owned(),
                |row| state.input.rows.cell(row, column),
            )
        };
        let height = if row.is_some() { ROW } else { HEADER };
        let entry = self.cells.entry(key).or_insert_with(|| {
            let text = text();
            Cell {
                layout: TextLayout::new(&text, format, width, height),
                text,
                width,
                generation: state.generation,
            }
        });
        if entry.generation != state.generation || entry.width != width {
            let text = text();
            if entry.text != text || entry.width != width {
                entry.layout = TextLayout::new(&text, format, width, height);
                entry.text = text;
                entry.width = width;
            }
            entry.generation = state.generation;
        }
        &entry.layout
    }

    pub fn draw<R: Rows>(
        &mut self,
        session: &DrawingSession<'_>,
        state: &State<R>,
        scale: f32,
        device_changed: bool,
    ) -> windows_canvas::Result<()> {
        let dark = state.input.scheme == ColorScheme::Dark;
        let background = if dark {
            ColorF::from_rgb8(28, 28, 28)
        } else {
            ColorF::from_rgb8(255, 255, 255)
        };
        session.clear(background);
        if self.columns != state.input.columns {
            self.columns = Rc::clone(&state.input.columns);
            self.cells.clear();
        }
        if device_changed
            || self
                .brushes
                .as_ref()
                .is_none_or(|(scheme, _)| *scheme != state.input.scheme)
        {
            self.brushes = Some((
                state.input.scheme,
                Brushes {
                    text: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(240, 240, 240)
                    } else {
                        ColorF::from_rgb8(24, 24, 24)
                    })?,
                    header: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(38, 38, 38)
                    } else {
                        ColorF::from_rgb8(245, 245, 245)
                    })?,
                    hover: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(46, 46, 46)
                    } else {
                        ColorF::from_rgb8(235, 240, 246)
                    })?,
                    selection: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(35, 67, 94)
                    } else {
                        ColorF::from_rgb8(205, 229, 250)
                    })?,
                    line: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(90, 90, 90)
                    } else {
                        ColorF::from_rgb8(155, 155, 155)
                    })?,
                    thumb: session.create_solid_brush(if dark {
                        ColorF::from_rgb8(150, 150, 150)
                    } else {
                        ColorF::from_rgb8(105, 105, 105)
                    })?,
                },
            ));
        }
        // Keep device resources outside the mutable text-cache borrow.
        let (scheme, brushes) = self.brushes.take().unwrap();
        let viewport = state.viewport();
        let visible = state.visible();
        let pixel = 1.0 / scale;
        for row in visible.clone() {
            let y = HEADER + (row as f64 * f64::from(ROW) - state.scroll[1]) as f32;
            let selected = state.input.selected.as_ref() == Some(&state.input.rows.key(row));
            if selected || state.hover == Some(row) {
                session.fill_rect(
                    &Rect::from_xywh(0.0, y, viewport[0], ROW),
                    if selected {
                        &brushes.selection
                    } else {
                        &brushes.hover
                    },
                );
            }
            let mut x = -state.scroll[0] as f32;
            for (column, width) in state.widths.iter().copied().enumerate() {
                if x < viewport[0] && x + width > 0.0 {
                    let layout = self.cell(state, Some(row), column, (width - 20.0).max(1.0));
                    session.draw_text_layout(Vector2::new(x + 10.0, y), layout, &brushes.text);
                }
                x += width;
            }
            if selected && state.focused {
                session.draw_rect(
                    &Rect::from_xywh(
                        pixel / 2.0,
                        y + pixel / 2.0,
                        (viewport[0] - pixel).max(0.0),
                        ROW - pixel,
                    ),
                    &brushes.line,
                    pixel,
                );
            }
        }
        // Opaque chrome covers the partially visible first and last rows.
        session.fill_rect(
            &Rect::from_xywh(0.0, 0.0, state.size[0], HEADER),
            &brushes.header,
        );
        let mut x = -state.scroll[0] as f32;
        self.visible_columns.clear();
        for (column, width) in state.widths.iter().copied().enumerate() {
            if x < viewport[0] && x + width > 0.0 {
                self.visible_columns.push(column);
                let layout = self.cell(state, None, column, (width - 32.0).max(1.0));
                session.draw_text_layout(Vector2::new(x + 10.0, 0.0), layout, &brushes.text);
                let edge = (x + width) * scale;
                let edge = (edge.floor() - 0.5) / scale;
                session.draw_line(
                    Vector2::new(edge, 7.0),
                    Vector2::new(edge, HEADER - 7.0),
                    &brushes.line,
                    pixel,
                );
                if let Some((sorted, descending)) = state.input.sort
                    && sorted == column
                {
                    let cx = x + width - 12.0;
                    let cy = HEADER / 2.0;
                    let dy = if descending { -2.0 } else { 2.0 };
                    session.draw_line(
                        Vector2::new(cx - 3.0, cy + dy),
                        Vector2::new(cx, cy - dy),
                        &brushes.text,
                        pixel,
                    );
                    session.draw_line(
                        Vector2::new(cx, cy - dy),
                        Vector2::new(cx + 3.0, cy + dy),
                        &brushes.text,
                        pixel,
                    );
                }
            }
            x += width;
        }
        session.fill_rect(
            &Rect::from_xywh(viewport[0], 0.0, BAR, state.size[1]),
            &brushes.header,
        );
        session.fill_rect(
            &Rect::from_xywh(0.0, HEADER + viewport[1], state.size[0], BAR),
            &brushes.header,
        );
        if state.maximum(1) > 0.0 {
            let (start, length) = state.thumb(1);
            session.fill_rect(
                &Rect::from_xywh(viewport[0] + 4.0, HEADER + start, BAR - 8.0, length),
                &brushes.thumb,
            );
        }
        if state.maximum(0) > 0.0 {
            let (start, length) = state.thumb(0);
            session.fill_rect(
                &Rect::from_xywh(start, HEADER + viewport[1] + 4.0, length, BAR - 8.0),
                &brushes.thumb,
            );
        }
        self.cells.retain(|(key, column), _| {
            self.visible_columns.contains(column)
                && key.as_ref().is_none_or(|key| {
                    state
                        .index_of(key)
                        .is_some_and(|row| visible.contains(&row))
                })
        });
        self.brushes = Some((scheme, brushes));
        Ok(())
    }
}
