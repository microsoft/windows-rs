use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static ROWS_CREATED: AtomicUsize = AtomicUsize::new(0);

pub(super) struct RepeaterFixture {
    values: Rc<Vec<u32>>,
    revision: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    Rerender,
    Change,
}

impl Component for RepeaterFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            values: Rc::new(vec![0; 256]),
            revision: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        if matches!(message, Message::Change) {
            Rc::make_mut(&mut self.values)[0] += 1;
        }
        self.revision += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: repeater");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let values = Rc::clone(&self.values);
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                StackPanel::new().children((
                    TextBlock::new()
                        .automation_id("revision")
                        .text(format!("Revision {}", self.revision)),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .children((
                            Button::new()
                                .automation_id("rerender")
                                .content("Rerender")
                                .on_click(context.message(Message::Rerender)),
                            Button::new()
                                .automation_id("change")
                                .content("Change row 0")
                                .on_click(context.message(Message::Change)),
                        )),
                )),
                ScrollViewer::new()
                    .automation_id("rows")
                    .grid_row(1)
                    .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                        0,
                        values.len(),
                        Key::from,
                        move |index| {
                            View::component::<Row>(RowInput {
                                index,
                                value: values[index],
                            })
                        },
                    ))),
            ))
            .into()
    }
}

/// Loads the rows and selects a radio button in one update. Applying the `RadioButtons` selection
/// realizes its items while Reactor is applying the update, which must not lay out the rows while
/// the host is busy. `Count` shows how many rows have been created.
pub(super) struct RepeaterLayoutFixture {
    loaded: bool,
    created: usize,
}

#[derive(Clone)]
pub(super) enum LayoutMessage {
    Load,
    Count,
}

impl Component for RepeaterLayoutFixture {
    type Input = ();
    type Message = LayoutMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            loaded: false,
            created: 0,
        }
    }

    fn update(&mut self, message: LayoutMessage, _: &ComponentContext<Self>) {
        match message {
            LayoutMessage::Load => self.loaded = true,
            LayoutMessage::Count => self.created = ROWS_CREATED.load(Ordering::Relaxed),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: repeater_layout");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let count = if self.loaded { 256 } else { 0 };
        // The repeater precedes the radio buttons so its new rows are pending when layout runs.
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                ScrollViewer::new()
                    .automation_id("rows")
                    .grid_row(1)
                    .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                        0,
                        count,
                        Key::from,
                        |index| View::component::<Row>(RowInput { index, value: 0 }),
                    ))),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        Button::new()
                            .automation_id("load")
                            .content("Load")
                            .on_click(context.message(LayoutMessage::Load)),
                        Button::new()
                            .automation_id("count")
                            .content(format!("Created {}", self.created))
                            .on_click(context.message(LayoutMessage::Count)),
                        RadioButtons::new()
                            .automation_id("choice")
                            .items_source(["One", "Two"])
                            .selected_index(Some(usize::from(self.loaded)))
                            .on_selection_changed(context.callback(|_| LayoutMessage::Count)),
                    )),
            ))
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct RowInput {
    index: usize,
    value: u32,
}

struct Row {
    clicks: u32,
}

impl Component for Row {
    type Input = RowInput;
    type Message = ();

    fn create(_: &RowInput, _: &ComponentContext<Self>) -> Self {
        ROWS_CREATED.fetch_add(1, Ordering::Relaxed);
        Self { clicks: 0 }
    }

    fn update(&mut self, (): (), _: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, input: &RowInput, context: &mut ViewContext<Self>) -> View {
        Button::new()
            .automation_id(format!("row-{}", input.index))
            .height(32.0)
            .content(format!(
                "Row {}; value {}; clicks {}",
                input.index, input.value, self.clicks
            ))
            .on_click(context.message(()))
            .into()
    }
}
