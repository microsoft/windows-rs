use super::*;
use std::collections::HashMap;
use std::ops::Range;

pub(super) const ROW: f32 = 26.0;
pub(super) const HEADER: f32 = 32.0;
pub(super) const BAR: f32 = 14.0;

pub(super) enum Action {
    Select(Key),
    Sort(usize),
}

enum Drag {
    Column {
        index: usize,
        start: f32,
        width: f32,
    },
    Scroll {
        axis: usize,
        start: f32,
        offset: f64,
    },
}

pub(super) struct State<R: Rows> {
    pub input: Input<R>,
    pub generation: u64,
    pub widths: Vec<f32>,
    pub size: [f32; 2],
    pub scroll: [f64; 2],
    pub hover: Option<usize>,
    pub focused: bool,
    indices: HashMap<Key, usize>,
    drag: Option<Drag>,
}

impl<R: Rows> State<R> {
    pub fn new(input: Input<R>) -> Self {
        let widths = Self::widths(&input);
        let mut state = Self {
            input,
            widths,
            generation: 0,
            size: [0.0; 2],
            scroll: [0.0; 2],
            hover: None,
            focused: false,
            indices: HashMap::new(),
            drag: None,
        };
        state.index();
        state
    }

    fn widths(input: &Input<R>) -> Vec<f32> {
        input
            .columns
            .iter()
            .map(|column| {
                assert!(column.width.is_finite() && column.width >= 48.0);
                column.width
            })
            .collect()
    }

    fn index(&mut self) {
        self.indices.clear();
        for index in 0..self.input.rows.len() {
            assert!(
                self.indices
                    .insert(self.input.rows.key(index), index)
                    .is_none(),
                "duplicate row key"
            );
        }
    }

    pub fn index_of(&self, key: &Key) -> Option<usize> {
        self.indices.get(key).copied()
    }

    pub fn set_input(&mut self, input: Input<R>) {
        let order_changed = input.order_revision != self.input.order_revision
            || input.rows.len() != self.input.rows.len();
        let columns_changed = input.columns != self.input.columns;
        if input.rows != self.input.rows || columns_changed {
            self.generation = self.generation.checked_add(1).unwrap();
        }
        if columns_changed {
            self.widths = Self::widths(&input);
            self.drag = None;
        }
        self.input = input;
        if order_changed {
            self.index();
            self.hover = None;
        }
        self.clamp();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.size = [width.max(0.0), height.max(0.0)];
        self.clamp();
    }

    pub fn viewport(&self) -> [f32; 2] {
        [
            (self.size[0] - BAR).max(0.0),
            (self.size[1] - HEADER - BAR).max(0.0),
        ]
    }

    pub fn maximum(&self, axis: usize) -> f64 {
        let extent = if axis == 0 {
            f64::from(self.widths.iter().sum::<f32>())
        } else {
            self.input.rows.len() as f64 * f64::from(ROW)
        };
        (extent - f64::from(self.viewport()[axis])).max(0.0)
    }

    fn clamp(&mut self) {
        for axis in 0..2 {
            self.scroll[axis] = self.scroll[axis].clamp(0.0, self.maximum(axis));
        }
    }

    pub fn visible(&self) -> Range<usize> {
        if self.viewport()[1] <= 0.0 {
            return 0..0;
        }
        let start = (self.scroll[1] / f64::from(ROW)).floor() as usize;
        let end =
            ((self.scroll[1] + f64::from(self.viewport()[1])) / f64::from(ROW)).ceil() as usize;
        start.min(self.input.rows.len())..end.min(self.input.rows.len())
    }

    pub fn thumb(&self, axis: usize) -> (f32, f32) {
        let track = self.viewport()[axis];
        let maximum = self.maximum(axis);
        let length = if maximum == 0.0 {
            track
        } else {
            (f64::from(track).powi(2) / (maximum + f64::from(track))) as f32
        }
        .max(24.0)
        .min(track);
        let start = if maximum == 0.0 {
            0.0
        } else {
            (self.scroll[axis] / maximum) as f32 * (track - length)
        };
        (start, length)
    }

    fn row_at(&self, x: f32, y: f32) -> Option<usize> {
        if x < 0.0 || x >= self.viewport()[0] || y < HEADER || y >= HEADER + self.viewport()[1] {
            return None;
        }
        let index = ((f64::from(y - HEADER) + self.scroll[1]) / f64::from(ROW)).floor() as usize;
        (index < self.input.rows.len()).then_some(index)
    }

    fn column_at(&self, x: f32) -> Option<(usize, f32)> {
        let mut end = -self.scroll[0] as f32;
        for (index, width) in self.widths.iter().enumerate() {
            end += width;
            if x < end {
                return Some((index, end));
            }
        }
        None
    }

    fn select(&mut self, index: usize) -> Option<Action> {
        if index >= self.input.rows.len() {
            return None;
        }
        let key = self.input.rows.key(index);
        if self.input.selected.as_ref() == Some(&key) {
            return None;
        }
        self.input.selected = Some(key.clone());
        Some(Action::Select(key))
    }

    pub fn press(&mut self, info: PointerEventInfo) -> Option<Action> {
        if !info.is_left_button_pressed {
            return None;
        }
        self.drag = None;
        let x = info.x as f32;
        let y = info.y as f32;
        let viewport = self.viewport();
        if x >= viewport[0] && y >= HEADER && y < HEADER + viewport[1] {
            self.press_scroll(1, y - HEADER, info.is_captured);
        } else if y >= HEADER + viewport[1] && x >= 0.0 && x < viewport[0] {
            self.press_scroll(0, x, info.is_captured);
        } else if (0.0..HEADER).contains(&y) && (0.0..viewport[0]).contains(&x) {
            if let Some((index, end)) = self.column_at(x) {
                if end - x <= 5.0 {
                    if info.is_captured {
                        self.drag = Some(Drag::Column {
                            index,
                            start: x,
                            width: self.widths[index],
                        });
                    }
                } else {
                    return Some(Action::Sort(index));
                }
            }
        } else if let Some(index) = self.row_at(x, y) {
            return self.select(index);
        }
        None
    }

    fn press_scroll(&mut self, axis: usize, position: f32, captured: bool) {
        let (start, length) = self.thumb(axis);
        if position >= start && position <= start + length {
            if captured {
                self.drag = Some(Drag::Scroll {
                    axis,
                    start: position,
                    offset: self.scroll[axis],
                });
            }
        } else {
            self.scroll[axis] +=
                f64::from(self.viewport()[axis]) * if position < start { -1.0 } else { 1.0 };
            self.clamp();
        }
    }

    pub fn move_pointer(&mut self, info: PointerEventInfo) {
        let x = info.x as f32;
        let y = info.y as f32;
        self.hover = self.row_at(x, y);
        match self.drag {
            Some(Drag::Column {
                index,
                start,
                width,
            }) if info.is_captured => {
                self.widths[index] = (width + x - start).clamp(48.0, 1200.0);
            }
            Some(Drag::Scroll {
                axis,
                start,
                offset,
            }) if info.is_captured => {
                let position = if axis == 0 { x } else { y - HEADER };
                let travel = self.viewport()[axis] - self.thumb(axis).1;
                if travel > 0.0 {
                    self.scroll[axis] =
                        offset + f64::from((position - start) / travel) * self.maximum(axis);
                }
            }
            _ => {}
        }
        self.clamp();
    }

    pub fn cancel(&mut self) {
        self.drag = None;
    }

    pub fn wheel(&mut self, info: PointerEventInfo, amount: f64) {
        let horizontal = info.is_horizontal_wheel || info.modifiers.contains(InputModifiers::SHIFT);
        let axis = usize::from(!horizontal);
        let direction = if info.is_horizontal_wheel { 1.0 } else { -1.0 };
        self.scroll[axis] += f64::from(info.wheel_delta) / 120.0 * amount * direction;
        self.clamp();
    }

    pub fn key(&mut self, key: VirtualKey) -> Option<Action> {
        if self.input.rows.len() == 0 {
            return None;
        }
        let current = self
            .input
            .selected
            .as_ref()
            .and_then(|key| self.indices.get(key))
            .copied();
        let last = self.input.rows.len() - 1;
        let page = ((self.viewport()[1] / ROW).floor() as usize).max(1);
        let index = match key {
            VirtualKey::UP => current.map_or(last, |index| index.saturating_sub(1)),
            VirtualKey::DOWN => current.map_or(0, |index| index.saturating_add(1).min(last)),
            VirtualKey::HOME => 0,
            VirtualKey::END => last,
            VirtualKey::PAGE_UP => current.unwrap_or(0).saturating_sub(page),
            VirtualKey::PAGE_DOWN => current.unwrap_or(0).saturating_add(page).min(last),
            _ => return None,
        };
        let top = index as f64 * f64::from(ROW);
        let bottom = top + f64::from(ROW);
        if top < self.scroll[1] {
            self.scroll[1] = top;
        } else if bottom > self.scroll[1] + f64::from(self.viewport()[1]) {
            self.scroll[1] = bottom - f64::from(self.viewport()[1]);
        }
        self.clamp();
        self.select(index)
    }
}
