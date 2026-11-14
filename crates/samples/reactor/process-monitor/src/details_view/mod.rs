use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::{Canvas, Invalidator};
use windows_reactor::*;

mod draw;
mod input;
mod state;
use draw::Renderer;
use state::{Action, Frames, State};

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

/// A read-only canvas table with keyed selection and viewport-bounded rendering.
///
/// Row/cell automation, high-contrast colors, touch panning, and multiple selection are not
/// implemented.
pub struct DetailsView<R: Rows> {
    state: Rc<RefCell<State<R>>>,
    frames: Rc<RefCell<Frames>>,
    invalidator: Invalidator,
    surface: View,
}

#[derive(Clone)]
pub struct Message(MessageKind);

#[derive(Clone)]
enum MessageKind {
    Action(Action),
    Key(VirtualKey),
    Focus(bool),
    Refresh,
}

#[derive(Clone, Copy)]
enum PointerInput {
    Press,
    Move,
    Wheel,
    End,
    Exit,
}

impl<R: Rows> DetailsView<R> {
    fn pointer(
        &self,
        context: &ViewContext<Self>,
        event: PointerInput,
    ) -> RoutedCallback<PointerEventInfo> {
        let state = Rc::clone(&self.state);
        let frames = Rc::clone(&self.frames);
        let invalidator = self.invalidator.clone();
        let send = context.forward();
        RoutedCallback::new(move |info: PointerEventInfo| {
            let (action, cursor_changed) = {
                let mut state = state.borrow_mut();
                let frames = frames.borrow();
                let cursor = state.cursor();
                let action = match event {
                    PointerInput::Press => state.press(info, &frames.current),
                    PointerInput::Move => {
                        state.move_pointer(info, &frames.current);
                        None
                    }
                    PointerInput::Wheel => {
                        let horizontal = info.is_horizontal_wheel
                            || info.modifiers.contains(InputModifiers::SHIFT);
                        let amount = input::wheel_amount(
                            horizontal,
                            state.viewport()[usize::from(!horizontal)],
                        );
                        state.wheel(info, amount);
                        None
                    }
                    PointerInput::End => {
                        state.cancel(info.pointer_id);
                        None
                    }
                    PointerInput::Exit => {
                        state.hover = None;
                        state.divider = None;
                        None
                    }
                };
                (action, state.cursor() != cursor)
            };
            invalidator.invalidate();
            if let Some(action) = action {
                send.call(Message(MessageKind::Action(action)));
            } else if cursor_changed {
                send.call(Message(MessageKind::Refresh));
            }
            match event {
                PointerInput::Press => info.is_left_button_pressed,
                PointerInput::Wheel => info.wheel_delta != 0,
                _ => false,
            }
        })
    }
}

impl<R: Rows> Component for DetailsView<R> {
    type Input = Input<R>;
    type Message = Message;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        let state = Rc::new(RefCell::new(State::new(input.clone())));
        let frames = Rc::new(RefCell::new(Frames::default()));
        let draw_frames = Rc::clone(&frames);
        let presented_frames = Rc::clone(&frames);
        let draw_state = Rc::clone(&state);
        let renderer = RefCell::new(Renderer::new());
        let invalidator = Invalidator::new();
        let surface = Canvas::invalidated(&invalidator, move |ctx| {
            let mut state = draw_state.borrow_mut();
            state.resize(ctx.width, ctx.height);
            renderer
                .borrow_mut()
                .draw(ctx, &state, ctx.scale(), ctx.device_changed())?;
            draw_frames.borrow_mut().pending.record(&state);
            Ok(())
        })
        .on_presented(move || presented_frames.borrow_mut().present())
        .into();
        Self {
            state,
            frames,
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
            match message.0 {
                MessageKind::Action(action) => state.apply_action(action),
                MessageKind::Key(key) => state.key(key),
                MessageKind::Focus(value) => {
                    state.focused = value;
                    None
                }
                MessageKind::Refresh => None,
            }
        };
        self.invalidator.invalidate();
        let input = self.state.borrow().input.clone();
        match action {
            Some(Action::Select(key)) => input.on_select.call(key),
            Some(Action::Sort { column, .. }) => input.on_sort.call(column),
            None => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let on_key = context.callback(|key| Message(MessageKind::Key(key)));
        Border::new()
            .automation_id(input.automation_id.clone())
            .automation_name(input.name.clone())
            .automation_help_text(
                "Use Up/Down, Home/End, or Page Up/Page Down to select a row. \
                 Accessible rows and cells are not implemented.",
            )
            .is_tab_stop(true)
            .capture_pointer_on_press(true)
            .pointer_cursor(self.state.borrow().cursor())
            .focus_on_pointer_release(true)
            .on_pointer_pressed(self.pointer(context, PointerInput::Press))
            .on_pointer_moved(self.pointer(context, PointerInput::Move))
            .on_pointer_released(self.pointer(context, PointerInput::End))
            .on_pointer_wheel_changed(self.pointer(context, PointerInput::Wheel))
            .on_pointer_capture_lost(self.pointer(context, PointerInput::End))
            .on_pointer_canceled(self.pointer(context, PointerInput::End))
            .on_pointer_exited(self.pointer(context, PointerInput::Exit))
            .on_got_focus(
                context
                    .callback(|info: FocusEventInfo| Message(MessageKind::Focus(info.is_direct))),
            )
            .on_lost_focus(context.callback(|_| Message(MessageKind::Focus(false))))
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
