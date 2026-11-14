use super::*;
use std::collections::HashMap;
use std::ops::Range;

pub(super) const ROW: f32 = 26.0;
pub(super) const HEADER: f32 = 32.0;
pub(super) const BAR: f32 = 14.0;

#[derive(Clone)]
pub(super) enum Action {
    Select(Key),
    Sort {
        column: usize,
        columns: Rc<[Column]>,
    },
}

#[derive(Default)]
pub(super) struct Frame {
    viewport: [f32; 2],
    scroll: [f64; 2],
    widths: Vec<f32>,
    columns: Rc<[Column]>,
    first: usize,
    rows: Vec<Key>,
    thumbs: [(f32, f32); 2],
    maximum: [f64; 2],
}

impl Frame {
    pub fn record<R: Rows>(&mut self, state: &State<R>) {
        self.viewport = state.viewport();
        self.scroll = state.scroll;
        self.widths.clone_from(&state.widths);
        self.columns = Rc::clone(&state.input.columns);
        let visible = state.visible();
        self.first = visible.start;
        self.rows.clear();
        self.rows
            .extend(visible.map(|row| state.input.rows.key(row)));
        self.thumbs = [state.thumb(0), state.thumb(1)];
        self.maximum = [state.maximum(0), state.maximum(1)];
    }

    fn row_at(&self, x: f32, y: f32) -> Option<&Key> {
        if x < 0.0 || x >= self.viewport[0] || y < HEADER || y >= HEADER + self.viewport[1] {
            return None;
        }
        let index = ((f64::from(y - HEADER) + self.scroll[1]) / f64::from(ROW)).floor() as usize;
        self.rows.get(index.checked_sub(self.first)?)
    }

    fn divider_at(&self, x: f32, y: f32) -> Option<usize> {
        if !(0.0..HEADER).contains(&y) || !(0.0..self.viewport[0]).contains(&x) {
            return None;
        }
        let mut end = -self.scroll[0] as f32;
        self.widths.iter().position(|width| {
            end += width;
            end > 0.0 && end < self.viewport[0] && (end - x).abs() <= 5.0
        })
    }

    fn column_at(&self, x: f32) -> Option<usize> {
        let mut end = -self.scroll[0] as f32;
        self.widths.iter().position(|width| {
            end += width;
            x < end
        })
    }
}

#[derive(Default)]
pub(super) struct Frames {
    pub current: Frame,
    pub pending: Frame,
}

impl Frames {
    pub fn present(&mut self) {
        std::mem::swap(&mut self.current, &mut self.pending);
    }
}

enum Drag {
    Column {
        pointer: u32,
        index: usize,
        start: f32,
        width: f32,
    },
    Scroll {
        pointer: u32,
        axis: usize,
        start: f32,
        offset: f64,
        units_per_dip: f64,
    },
}

impl Drag {
    fn pointer(&self) -> u32 {
        match self {
            Self::Column { pointer, .. } | Self::Scroll { pointer, .. } => *pointer,
        }
    }
}

pub(super) struct State<R: Rows> {
    pub input: Input<R>,
    pub generation: u64,
    pub widths: Vec<f32>,
    pub size: [f32; 2],
    pub scroll: [f64; 2],
    pub hover: Option<usize>,
    pub divider: Option<usize>,
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
            divider: None,
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
            self.divider = None;
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

    pub fn cursor(&self) -> InputSystemCursorShape {
        if matches!(self.drag, Some(Drag::Column { .. })) || self.divider.is_some() {
            InputSystemCursorShape::SizeWestEast
        } else {
            InputSystemCursorShape::Arrow
        }
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

    pub fn press(&mut self, info: PointerEventInfo, frame: &Frame) -> Option<Action> {
        if !info.is_left_button_pressed || self.drag.is_some() {
            return None;
        }
        let x = info.x as f32;
        let y = info.y as f32;
        let viewport = frame.viewport;
        if x >= viewport[0] && y >= HEADER && y < HEADER + viewport[1] {
            self.press_scroll(1, y - HEADER, info, frame);
        } else if y >= HEADER + viewport[1] && x >= 0.0 && x < viewport[0] {
            self.press_scroll(0, x, info, frame);
        } else if (0.0..HEADER).contains(&y) && (0.0..viewport[0]).contains(&x) {
            if frame.columns != self.input.columns {
                return None;
            }
            if let Some(index) = frame.divider_at(x, y) {
                if info.is_captured {
                    self.drag = Some(Drag::Column {
                        pointer: info.pointer_id,
                        index,
                        start: x,
                        width: frame.widths[index],
                    });
                }
            } else if let Some(column) = frame.column_at(x) {
                return Some(Action::Sort {
                    column,
                    columns: Rc::clone(&frame.columns),
                });
            }
        } else if let Some(index) = frame.row_at(x, y).and_then(|key| self.index_of(key)) {
            return self.select(index);
        }
        None
    }

    fn press_scroll(&mut self, axis: usize, position: f32, info: PointerEventInfo, frame: &Frame) {
        let (start, length) = frame.thumbs[axis];
        let travel = frame.viewport[axis] - length;
        if travel <= 0.0 {
            return;
        }
        if position >= start && position <= start + length {
            if info.is_captured {
                self.drag = Some(Drag::Scroll {
                    pointer: info.pointer_id,
                    axis,
                    start: position,
                    offset: frame.scroll[axis],
                    units_per_dip: frame.maximum[axis] / f64::from(travel),
                });
            }
        } else {
            self.scroll[axis] = frame.scroll[axis]
                + f64::from(frame.viewport[axis]) * if position < start { -1.0 } else { 1.0 };
            self.clamp();
        }
    }

    pub fn move_pointer(&mut self, info: PointerEventInfo, frame: &Frame) {
        if let Some(drag) = &self.drag {
            if drag.pointer() != info.pointer_id {
                return;
            }
            if !info.is_captured || !info.is_left_button_pressed {
                self.drag = None;
            }
        }
        let x = info.x as f32;
        let y = info.y as f32;
        self.hover = frame.row_at(x, y).and_then(|key| self.index_of(key));
        self.divider = (frame.columns == self.input.columns)
            .then(|| frame.divider_at(x, y))
            .flatten();
        match self.drag {
            Some(Drag::Column {
                index,
                start,
                width,
                ..
            }) => {
                self.widths[index] = (width + x - start).clamp(48.0, 1200.0);
                self.divider = Some(index);
            }
            Some(Drag::Scroll {
                axis,
                start,
                offset,
                units_per_dip,
                ..
            }) => {
                let position = if axis == 0 { x } else { y - HEADER };
                self.scroll[axis] = offset + f64::from(position - start) * units_per_dip;
            }
            _ => {}
        }
        self.clamp();
    }

    pub fn cancel(&mut self, pointer: u32) {
        if self
            .drag
            .as_ref()
            .is_some_and(|drag| drag.pointer() == pointer)
        {
            self.drag = None;
            self.divider = None;
        }
    }

    pub fn apply_action(&mut self, action: Action) -> Option<Action> {
        // Native input and component delivery can straddle removal of the target.
        match &action {
            Action::Select(key) => {
                self.index_of(key)?;
                self.input.selected = Some(key.clone());
            }
            Action::Sort { columns, .. } if *columns != self.input.columns => return None,
            Action::Sort { .. } => {}
        }
        Some(action)
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
