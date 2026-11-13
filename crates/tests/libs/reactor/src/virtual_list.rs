use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static ROWS_CREATED: AtomicUsize = AtomicUsize::new(0);
static ROWS_RENDERED: AtomicUsize = AtomicUsize::new(0);
static ROWS_DROPPED: AtomicUsize = AtomicUsize::new(0);
static CLICKED_ROW_ZERO_DROPS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Rows {
    Default,
    Items,
    Virtual,
    Short,
    Mixed,
    Multiple,
    Extended,
    Bulk,
}

impl Rows {
    fn count(self) -> usize {
        match self {
            Self::Default => 256,
            Self::Items | Self::Virtual | Self::Multiple | Self::Extended => 200,
            Self::Short | Self::Mixed | Self::Bulk => 10_000,
        }
    }

    fn height(self, index: usize) -> f64 {
        match self {
            Self::Short => 4.0,
            Self::Mixed => [4.0, 20.0, 96.0][index % 3],
            _ => 32.0,
        }
    }
}

pub(super) struct VirtualListFixture {
    values: Rc<Vec<u32>>,
    keys: Rc<Vec<usize>>,
    key_revision: u64,
    revision: u32,
    sample: usize,
    counts: [usize; 3],
    editing: bool,
    selected: Option<Option<usize>>,
    selection_events: Vec<Option<usize>>,
}

#[derive(Clone)]
pub(super) enum Message {
    Rerender,
    Change,
    SwapFirst,
    Reverse,
    SelectFirst,
    ControlReset,
    ClearReset,
    ReleaseReset,
    Append,
    Selection(Option<usize>),
    Shorten,
    Reorder(Vec<Key>),
    ToggleFirst,
    ToggleEmpty,
    Count,
    ToggleEditor,
}

impl Component for VirtualListFixture {
    type Input = Rows;
    type Message = Message;

    fn create(rows: &Rows, _: &ComponentContext<Self>) -> Self {
        Self {
            values: Rc::new(vec![0; rows.count()]),
            keys: Rc::new((0..rows.count()).collect()),
            key_revision: 0,
            revision: 0,
            sample: 0,
            counts: [0; 3],
            editing: false,
            selected: None,
            selection_events: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Selection(index) => {
                self.selection_events.push(index);
                return;
            }
            Message::ControlReset => {
                self.selected = Some(Some(0));
                Rc::make_mut(&mut self.keys).reverse();
                self.key_revision += 1;
            }
            Message::ClearReset => {
                self.selected = Some(None);
                Rc::make_mut(&mut self.keys).reverse();
                self.key_revision += 1;
            }
            Message::ReleaseReset => {
                self.selected = None;
                Rc::make_mut(&mut self.keys).reverse();
                self.key_revision += 1;
            }
            Message::Append => {
                let key = self.values.len();
                Rc::make_mut(&mut self.values).push(0);
                Rc::make_mut(&mut self.keys).push(key);
                self.key_revision += 1;
            }
            Message::Rerender => {}
            Message::ToggleEditor => self.editing = !self.editing,
            Message::Change => Rc::make_mut(&mut self.values)[0] += 1,
            Message::SwapFirst => {
                if self.keys.len() >= 2 {
                    Rc::make_mut(&mut self.keys).swap(0, 1);
                }
                self.key_revision += 1;
            }
            Message::Reverse => {
                Rc::make_mut(&mut self.keys).reverse();
                self.key_revision += 1;
            }
            Message::SelectFirst => self.selected = Some(Some(0)),
            Message::Shorten => {
                Rc::make_mut(&mut self.keys).truncate(3);
                self.key_revision += 1;
            }
            Message::Reorder(order) => {
                let keys: std::collections::HashMap<_, _> = self
                    .keys
                    .iter()
                    .map(|key| (Key::from(*key), *key))
                    .collect();
                self.keys = Rc::new(order.iter().map(|key| keys[key]).collect());
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
                    (0..self.values.len()).collect()
                } else {
                    Vec::new()
                });
                self.key_revision += 1;
            }
            Message::Count => {
                self.sample += 1;
                self.counts = [
                    ROWS_CREATED.load(Ordering::Relaxed),
                    ROWS_RENDERED.load(Ordering::Relaxed),
                    ROWS_DROPPED.load(Ordering::Relaxed),
                ];
                return;
            }
        }
        self.revision += 1;
    }

    fn view(&self, rows: &Rows, context: &mut ViewContext<Self>) -> View {
        context.window_title(match rows {
            Rows::Default => "Reactor UI fixture: virtual_list",
            Rows::Items => "Reactor UI fixture: virtual_list_items",
            Rows::Virtual => "Reactor UI fixture: virtual_list_virtual",
            Rows::Short => "Reactor UI fixture: virtual_list_short",
            Rows::Mixed => "Reactor UI fixture: virtual_list_mixed",
            Rows::Multiple => "Reactor UI fixture: virtual_list_multiple",
            Rows::Extended => "Reactor UI fixture: virtual_list_extended",
            Rows::Bulk => "Reactor UI fixture: virtual_list_bulk",
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let values = Rc::clone(&self.values);
        let keys = Rc::clone(&self.keys);
        let row_keys = Rc::clone(&self.keys);
        let swap = context.message(Message::SwapFirst);
        let rows = *rows;
        let editing = self.editing;
        let row = move |index: usize| {
            let index = row_keys[index];
            if editing && index == 0 {
                return TextBox::new()
                    .automation_id("row-editor")
                    .automation_name("Edit row 0")
                    .height(rows.height(index))
                    .into();
            }
            View::component::<Row>(RowInput {
                index,
                value: values[index],
                on_click: (rows == Rows::Default && index == 1).then(|| swap.clone()),
                height: rows.height(index),
            })
        };
        let mut list = ListView::new()
            .automation_id("rows")
            .grid_row(1)
            .selection_mode(match rows {
                Rows::Multiple => ListViewSelectionMode::Multiple,
                Rows::Extended | Rows::Bulk => ListViewSelectionMode::Extended,
                _ => ListViewSelectionMode::Single,
            })
            .on_selection_changed(context.callback(Message::Selection));
        if let Some(selected) = self.selected {
            list = list.selected_index(
                selected.and_then(|selected| self.keys.iter().position(|key| *key == selected)),
            );
        }
        if rows == Rows::Virtual {
            list = list
                .can_drag_items(true)
                .can_reorder_items(true)
                .allow_drop(true)
                .on_reordered(context.callback(Message::Reorder));
        }
        let list = if rows == Rows::Items {
            list.items((0..self.keys.len()).map(|index| keyed(keys[index], row(index))))
        } else {
            list.virtual_source(VirtualSource::new(
                self.key_revision,
                self.keys.len(),
                move |index| keys[index],
                row,
            ))
        };
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                StackPanel::new().children((
                    TextBlock::new()
                        .automation_id("revision")
                        .text(format!("Revision {}", self.revision)),
                    TextBlock::new()
                        .automation_id("selection-events")
                        .text(format!("{:?}", self.selection_events)),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .children((
                            Button::new()
                                .automation_id("reverse")
                                .content("Reverse")
                                .on_click(context.message(Message::Reverse)),
                            Button::new()
                                .automation_id("select-first")
                                .content("Select")
                                .on_click(context.message(Message::SelectFirst)),
                            Button::new()
                                .automation_id("shorten")
                                .content("Keep 3")
                                .on_click(context.message(Message::Shorten)),
                            Button::new()
                                .automation_id("append")
                                .content("Append")
                                .on_click(context.message(Message::Append)),
                            Button::new()
                                .automation_id("control-reset")
                                .content("Set")
                                .on_click(context.message(Message::ControlReset)),
                            Button::new()
                                .automation_id("clear-reset")
                                .content("Clear")
                                .on_click(context.message(Message::ClearReset)),
                            Button::new()
                                .automation_id("release-reset")
                                .content("Release")
                                .on_click(context.message(Message::ReleaseReset)),
                        )),
                    TextBlock::new().automation_id("drops").text(format!(
                        "Clicked row 0 drops {}",
                        CLICKED_ROW_ZERO_DROPS.load(Ordering::Relaxed)
                    )),
                    Button::new()
                        .automation_id("count")
                        .content(format!(
                            "{}:{}:{}:{}",
                            self.sample, self.counts[0], self.counts[1], self.counts[2]
                        ))
                        .on_click(context.message(Message::Count)),
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
                            Button::new()
                                .automation_id("toggle-editor")
                                .content("Edit row 0")
                                .on_click(context.message(Message::ToggleEditor)),
                        )),
                )),
                list,
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
    FlipViewShort,
    FlipViewMixed,
    /// WinUI's `SelectedIndex` setter lays out the whole window synchronously.
    Pivot,
}

/// Loads the rows and changes a selector's index in one update. Applying the selection lays out
/// the window while Reactor is applying the update, so the list requests rows from within the
/// update. `Count` shows how many rows have been created.
pub(super) struct VirtualListLayoutFixture {
    loaded: bool,
    created: usize,
}

#[derive(Clone)]
pub(super) enum LayoutMessage {
    Load,
    Count,
}

impl Component for VirtualListLayoutFixture {
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
            Selector::RadioButtons => "Reactor UI fixture: virtual_list_layout",
            Selector::FlipView => "Reactor UI fixture: virtual_list_layout_flip_view",
            Selector::FlipViewBeforeReset => "Reactor UI fixture: virtual_list_layout_keyed_reset",
            Selector::FlipViewShort => "Reactor UI fixture: virtual_list_layout_short",
            Selector::FlipViewMixed => "Reactor UI fixture: virtual_list_layout_mixed",
            Selector::Pivot => "Reactor UI fixture: virtual_list_layout_pivot",
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 360.0));
        let before_reset = *selector == Selector::FlipViewBeforeReset;
        let rows = match selector {
            Selector::FlipViewShort => Rows::Short,
            Selector::FlipViewMixed => Rows::Mixed,
            _ => Rows::Default,
        };
        let count = if self.loaded || before_reset {
            rows.count()
        } else {
            0
        };
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
            Selector::FlipView
            | Selector::FlipViewBeforeReset
            | Selector::FlipViewShort
            | Selector::FlipViewMixed => FlipView::new()
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
        let rows = ListView::new()
            .automation_id("rows")
            .grid_row(1)
            .virtual_source(VirtualSource::new(
                u64::from(swapped),
                count,
                move |index| if swapped { index ^ 1 } else { index },
                move |index| {
                    let index = if swapped { index ^ 1 } else { index };
                    View::component::<Row>(RowInput {
                        index,
                        value: 0,
                        on_click: None,
                        height: rows.height(index),
                    })
                },
            ));
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
    height: f64,
}

struct Row {
    index: usize,
    clicks: u32,
}

impl Drop for Row {
    fn drop(&mut self) {
        ROWS_DROPPED.fetch_add(1, Ordering::Relaxed);
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
        ROWS_RENDERED.fetch_add(1, Ordering::Relaxed);
        Button::new()
            .automation_id(format!("row-{}", input.index))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .min_height(0.0)
            .height(input.height)
            .content(format!(
                "Row {}; value {}; clicks {}",
                input.index, input.value, self.clicks
            ))
            .on_click(context.message(input.on_click.clone()))
            .into()
    }
}
