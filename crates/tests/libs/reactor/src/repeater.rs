use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static ROWS_CREATED: AtomicUsize = AtomicUsize::new(0);
static CLICKED_ROW_ZERO_DROPS: AtomicUsize = AtomicUsize::new(0);

pub(super) struct RepeaterFixture {
    values: Rc<Vec<u32>>,
    keys: Rc<Vec<usize>>,
    key_revision: u64,
    revision: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    Rerender,
    Change,
    SwapFirst,
    ToggleFirst,
    ToggleEmpty,
}

impl Component for RepeaterFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            values: Rc::new(vec![0; 256]),
            keys: Rc::new((0..256).collect()),
            key_revision: 0,
            revision: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Rerender => {}
            Message::Change => Rc::make_mut(&mut self.values)[0] += 1,
            Message::SwapFirst => {
                if self.keys.len() >= 2 {
                    Rc::make_mut(&mut self.keys).swap(0, 1);
                }
                self.key_revision += 1;
            }
            Message::ToggleFirst => {
                let keys = Rc::make_mut(&mut self.keys);
                if keys.contains(&0) {
                    keys.retain(|key| *key != 0);
                } else {
                    keys.insert(0, 0);
                }
                self.key_revision += 1;
            }
            Message::ToggleEmpty => {
                self.keys = Rc::new(if self.keys.is_empty() {
                    (0..256).collect()
                } else {
                    Vec::new()
                });
                self.key_revision += 1;
            }
        }
        self.revision += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: repeater");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let values = Rc::clone(&self.values);
        let keys = Rc::clone(&self.keys);
        let row_keys = Rc::clone(&self.keys);
        let swap = context.message(Message::SwapFirst);
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                StackPanel::new().children((
                    TextBlock::new()
                        .automation_id("revision")
                        .text(format!("Revision {}", self.revision)),
                    TextBlock::new().automation_id("drops").text(format!(
                        "Clicked row 0 drops {}",
                        CLICKED_ROW_ZERO_DROPS.load(Ordering::Relaxed)
                    )),
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
                            Button::new()
                                .automation_id("toggle-first")
                                .content("Toggle row 0")
                                .on_click(context.message(Message::ToggleFirst)),
                            Button::new()
                                .automation_id("toggle-empty")
                                .content("Toggle empty")
                                .on_click(context.message(Message::ToggleEmpty)),
                        )),
                )),
                ScrollViewer::new()
                    .automation_id("rows")
                    .grid_row(1)
                    .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                        self.key_revision,
                        self.keys.len(),
                        move |index| keys[index],
                        move |index| {
                            let index = row_keys[index];
                            View::component::<Row>(RowInput {
                                index,
                                value: values[index],
                                on_click: (index == 1).then(|| swap.clone()),
                            })
                        },
                    ))),
            ))
            .into()
    }
}

/// The selector whose index changes in the same update that loads the rows.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Selector {
    /// Reactor measures the control to realize its items.
    RadioButtons,
    /// WinUI's `SelectedIndex` setter lays out the whole window synchronously.
    FlipView,
    /// Synchronous layout precedes a keyed source reset.
    FlipViewBeforeReset,
    /// WinUI's `SelectedIndex` setter lays out the whole window synchronously.
    Pivot,
}

/// Loads the rows and changes a selector's index in one update. Applying the selection lays out
/// the window while Reactor is applying the update, so the repeater requests rows from within the
/// update. `Count` shows how many rows have been created.
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
    type Input = Selector;
    type Message = LayoutMessage;

    fn create(_: &Selector, _: &ComponentContext<Self>) -> Self {
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

    fn view(&self, selector: &Selector, context: &mut ViewContext<Self>) -> View {
        context.window_title(match selector {
            Selector::RadioButtons => "Reactor UI fixture: repeater_layout",
            Selector::FlipView => "Reactor UI fixture: repeater_layout_flip_view",
            Selector::FlipViewBeforeReset => "Reactor UI fixture: repeater_layout_keyed_reset",
            Selector::Pivot => "Reactor UI fixture: repeater_layout_pivot",
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let before_reset = *selector == Selector::FlipViewBeforeReset;
        let count = if self.loaded || before_reset { 256 } else { 0 };
        let swapped = before_reset && self.loaded;
        let selected = Some(usize::from(self.loaded));
        let items = || ["One", "Two"].map(|name| keyed(name, TextBlock::new().text(name)));
        let selector: View = match selector {
            Selector::RadioButtons => RadioButtons::new()
                .automation_id("choice")
                .items_source(["One", "Two"])
                .selected_index(selected)
                .on_selection_changed(context.callback(|_| LayoutMessage::Count))
                .into(),
            Selector::FlipView | Selector::FlipViewBeforeReset => FlipView::new()
                .automation_id("choice")
                .width(120.0)
                .height(40.0)
                .selected_index(selected)
                .keyed_items(items())
                .into(),
            Selector::Pivot => Pivot::new()
                .automation_id("choice")
                .width(120.0)
                .height(80.0)
                .selected_index(selected)
                .items(["One", "Two"].map(|name| PivotItem::new().header(name)))
                .into(),
        };
        let header = StackPanel::new()
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
                selector,
            ));
        let rows = ScrollViewer::new()
            .automation_id("rows")
            .grid_row(1)
            .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                u64::from(swapped),
                count,
                move |index| if swapped { index ^ 1 } else { index },
                move |index| {
                    let index = if swapped { index ^ 1 } else { index };
                    View::component::<Row>(RowInput {
                        index,
                        value: 0,
                        on_click: None,
                    })
                },
            )));
        // Shrinking the header lets synchronous layout request more rows before the keyed reset.
        let children: [View; 2] = if before_reset {
            [
                header.height(if self.loaded { 40.0 } else { 300.0 }).into(),
                rows.into(),
            ]
        } else {
            [rows.into(), header.into()]
        };
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children(children)
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct RowInput {
    index: usize,
    value: u32,
    on_click: Option<Callback<()>>,
}

struct Row {
    index: usize,
    clicks: u32,
}

impl Drop for Row {
    fn drop(&mut self) {
        if self.index == 0 && self.clicks > 0 {
            CLICKED_ROW_ZERO_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl Component for Row {
    type Input = RowInput;
    type Message = Option<Callback<()>>;

    fn create(input: &RowInput, _: &ComponentContext<Self>) -> Self {
        ROWS_CREATED.fetch_add(1, Ordering::Relaxed);
        Self {
            index: input.index,
            clicks: 0,
        }
    }

    fn update(&mut self, on_click: Self::Message, _: &ComponentContext<Self>) {
        self.clicks += 1;
        if let Some(on_click) = on_click {
            on_click.call(());
        }
    }

    fn view(&self, input: &RowInput, context: &mut ViewContext<Self>) -> View {
        Button::new()
            .automation_id(format!("row-{}", input.index))
            .height(32.0)
            .content(format!(
                "Row {}; value {}; clicks {}",
                input.index, input.value, self.clicks
            ))
            .on_click(context.message(input.on_click.clone()))
            .into()
    }
}
