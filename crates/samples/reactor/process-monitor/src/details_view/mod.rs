use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::{Canvas, Invalidator};
use windows_reactor::*;

mod draw;
mod input;
mod state;
use draw::Renderer;
use state::{Action, State};

#[cfg(test)]
mod tests;

/// An immutable snapshot. Equal snapshots have equal cell contents.
pub trait Rows: Clone + PartialEq + 'static {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn key(&self, index: usize) -> Key;
    fn cell(&self, index: usize, column: usize) -> String;
}

#[derive(Clone, PartialEq)]
pub struct Column {
    pub title: &'static str,
    pub width: f32,
    pub right_aligned: bool,
}

#[derive(Clone, PartialEq)]
pub struct Input<R: Rows> {
    pub automation_id: Rc<str>,
    pub name: Rc<str>,
    pub rows: R,
    /// Changes whenever row keys or their order change.
    pub order_revision: u64,
    pub columns: Rc<[Column]>,
    pub selected: Option<Key>,
    pub sort: Option<(usize, bool)>,
    pub scheme: ColorScheme,
    pub on_select: Callback<Key>,
    pub on_sort: Callback<usize>,
}

/// An experimental, read-only canvas table. Host outside an ancestor ScrollViewer.
///
/// Row/cell automation, high-contrast colors, touch panning, and multiple selection are not
/// implemented.
pub struct DetailsView<R: Rows> {
    state: Rc<RefCell<State<R>>>,
    invalidator: Invalidator,
    surface: View,
}

#[derive(Clone)]
pub enum Message {
    Press(PointerEventInfo),
    Move(PointerEventInfo),
    Wheel(PointerEventInfo),
    Key(VirtualKey),
    Focus(bool),
    Cancel(u32),
    Exit,
}

impl<R: Rows> Component for DetailsView<R> {
    type Input = Input<R>;
    type Message = Message;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        let state = Rc::new(RefCell::new(State::new(input.clone())));
        let draw_state = Rc::clone(&state);
        let renderer = RefCell::new(Renderer::new());
        let invalidator = Invalidator::new();
        let surface = Canvas::invalidated(&invalidator, move |ctx| {
            let mut state = draw_state.borrow_mut();
            state.resize(ctx.width, ctx.height);
            renderer
                .borrow_mut()
                .draw(ctx, &state, ctx.scale(), ctx.device_changed())
        })
        .into();
        Self {
            state,
            invalidator,
            surface,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _: &ComponentContext<Self>) {
        self.state.borrow_mut().set_input(input.clone());
        self.invalidator.invalidate();
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        let action = {
            let mut state = self.state.borrow_mut();
            match message {
                Message::Press(info) => state.press(info),
                Message::Move(info) => {
                    state.move_pointer(info);
                    None
                }
                Message::Wheel(info) => {
                    let horizontal =
                        info.is_horizontal_wheel || info.modifiers.contains(InputModifiers::SHIFT);
                    let amount =
                        input::wheel_amount(horizontal, state.viewport()[usize::from(!horizontal)]);
                    state.wheel(info, amount);
                    None
                }
                Message::Key(key) => state.key(key),
                Message::Focus(value) => {
                    state.focused = value;
                    None
                }
                Message::Cancel(pointer) => {
                    state.cancel(pointer);
                    None
                }
                Message::Exit => {
                    state.hover = None;
                    state.divider = None;
                    None
                }
            }
        };
        self.invalidator.invalidate();
        let input = self.state.borrow().input.clone();
        match action {
            Some(Action::Select(key)) => input.on_select.call(key),
            Some(Action::Sort(column)) => input.on_sort.call(column),
            None => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let on_key = context.callback(Message::Key);
        Border::new()
            .automation_id(input.automation_id.clone())
            .automation_name(input.name.clone())
            .automation_help_text(
                "Canvas prototype; accessible rows and cells are not implemented.",
            )
            .is_tab_stop(true)
            .capture_pointer_on_press(true)
            .pointer_cursor(self.state.borrow().cursor())
            .focus_on_pointer_release(true)
            .on_pointer_pressed(context.callback(Message::Press))
            .on_pointer_moved(context.callback(Message::Move))
            .on_pointer_released(
                context.callback(|info: PointerEventInfo| Message::Cancel(info.pointer_id)),
            )
            .on_pointer_wheel_changed(context.callback(Message::Wheel))
            .on_pointer_capture_lost(
                context.callback(|info: PointerEventInfo| Message::Cancel(info.pointer_id)),
            )
            .on_pointer_canceled(
                context.callback(|info: PointerEventInfo| Message::Cancel(info.pointer_id)),
            )
            .on_pointer_exited(context.callback(|_| Message::Exit))
            .on_got_focus(context.callback(|info: FocusEventInfo| Message::Focus(info.is_direct)))
            .on_lost_focus(context.callback(|_| Message::Focus(false)))
            .on_preview_key_down(RoutedCallback::new(move |info: KeyEventInfo| {
                if matches!(
                    info.key,
                    VirtualKey::UP
                        | VirtualKey::DOWN
                        | VirtualKey::HOME
                        | VirtualKey::END
                        | VirtualKey::PAGE_UP
                        | VirtualKey::PAGE_DOWN
                ) && info.modifiers == InputModifiers::NONE
                {
                    on_key.call(info.key);
                    true
                } else {
                    false
                }
            }))
            .content(self.surface.clone())
            .into()
    }
}
