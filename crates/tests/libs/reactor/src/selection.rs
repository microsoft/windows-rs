use super::*;

/// Controls that declare `selected_index(Some(1))` and record any selection callback, beside an
/// uncontrolled list that reports its selection. Buttons remove, insert, and rotate the items of
/// all of them.
pub(super) struct IndexSelectionFixture {
    names: Vec<&'static str>,
    echoes: Vec<(&'static str, Option<usize>)>,
    free: Option<usize>,
}

#[derive(Clone)]
pub(super) enum IndexMessage {
    Echo(&'static str, Option<usize>),
    Free(Option<usize>),
    RemoveFirst,
    InsertFirst,
    Rotate,
}

impl Component for IndexSelectionFixture {
    type Input = ();
    type Message = IndexMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            names: vec!["Zero", "One", "Two"],
            echoes: Vec::new(),
            free: None,
        }
    }

    fn update(&mut self, message: IndexMessage, _: &ComponentContext<Self>) {
        match message {
            IndexMessage::Echo(control, index) => self.echoes.push((control, index)),
            IndexMessage::Free(index) => self.free = index,
            IndexMessage::RemoveFirst => _ = self.names.remove(0),
            IndexMessage::InsertFirst => self.names.insert(0, "Front"),
            IndexMessage::Rotate => self.names.rotate_right(1),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let names = &self.names;
        let data_items = || names.iter().map(|name| DataItem::new(*name, *name));
        let keyed_text = || {
            names
                .iter()
                .map(|name| keyed(*name, TextBlock::new().text(*name)))
        };
        let echo = |control: &'static str| {
            context.callback(move |index: Option<usize>| IndexMessage::Echo(control, index))
        };
        let declared: [View; 8] = [
            ListView::new()
                .automation_id("list-view")
                .selection_mode(ListViewSelectionMode::Single)
                .selected_index(Some(1))
                .items(data_items())
                .on_selection_changed(echo("list-view"))
                .into(),
            GridView::new()
                .automation_id("grid-view")
                .selected_index(Some(1))
                .keyed_items(keyed_text())
                .on_selection_changed(echo("grid-view"))
                .into(),
            ComboBox::new()
                .automation_id("combo-box")
                .items_source(names.iter().copied())
                .selected_index(Some(1))
                .on_selection_changed(echo("combo-box"))
                .into(),
            RadioButtons::new()
                .automation_id("radio-buttons")
                .items_source(names.iter().copied())
                .selected_index(Some(1))
                .on_selection_changed(echo("radio-buttons"))
                .into(),
            Pivot::new()
                .automation_id("pivot")
                .selected_index(Some(1))
                .items(names.iter().map(|name| PivotItem::new().header(*name)))
                .on_selection_changed(echo("pivot"))
                .into(),
            FlipView::new()
                .automation_id("flip-view")
                .height(40.0)
                .selected_index(Some(1))
                .keyed_items(keyed_text())
                .on_selection_changed(echo("flip-view"))
                .into(),
            TabView::new()
                .automation_id("tab-view")
                .height(80.0)
                .selected_index(Some(1))
                .keyed_tab_items(
                    names
                        .iter()
                        .map(|name| keyed(*name, TabViewItem::new().header(*name))),
                )
                .on_selection_changed(echo("tab-view"))
                .into(),
            TextBlock::new()
                .automation_id("echoes")
                .text(format!("Echoes {:?}", self.echoes))
                .into(),
        ];
        let free: [View; 5] = [
            ListView::new()
                .automation_id("free-list")
                .selection_mode(ListViewSelectionMode::Single)
                .items(data_items())
                .on_selection_changed(context.callback(IndexMessage::Free))
                .into(),
            TextBlock::new()
                .automation_id("free")
                .text(format!("Free {:?}", self.free))
                .into(),
            Button::new()
                .automation_id("remove-first")
                .content("Remove first")
                .on_click(context.message(IndexMessage::RemoveFirst))
                .into(),
            Button::new()
                .automation_id("insert-first")
                .content("Insert first")
                .on_click(context.message(IndexMessage::InsertFirst))
                .into(),
            Button::new()
                .automation_id("rotate")
                .content("Rotate")
                .on_click(context.message(IndexMessage::Rotate))
                .into(),
        ];
        context.window_title("Reactor UI fixture: selection-index");
        context.window_visuals(WindowVisuals::new().client_size(900.0, 900.0));
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .margin(20.0)
            .spacing(20.0)
            .children((
                StackPanel::new()
                    .width(500.0)
                    .spacing(8.0)
                    .children(declared),
                StackPanel::new().spacing(8.0).children(free),
            ))
            .into()
    }
}
pub(super) struct SelectionFixture {
    selected: Option<usize>,
    generation: u32,
    attached: bool,
    calls: u32,
    received: Option<(u32, Option<usize>)>,
}

#[derive(Clone)]
pub(super) enum Message {
    SelectLast,
    Clear,
    Replace,
    Remove,
    Selected(u32, Option<usize>),
}

impl Component for SelectionFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            selected: None,
            generation: 0,
            attached: true,
            calls: 0,
            received: None,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::SelectLast => self.selected = Some(2),
            Message::Clear => self.selected = None,
            Message::Replace => self.generation += 1,
            Message::Remove => self.attached = false,
            Message::Selected(generation, selected) => {
                self.selected = selected;
                self.calls += 1;
                self.received = Some((generation, selected));
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let mut list = ListView::new()
            .automation_id("list")
            .selection_mode(ListViewSelectionMode::Single)
            .selected_index(self.selected)
            .items([
                DataItem::new("alpha", "Alpha"),
                DataItem::new("beta", "Beta"),
                DataItem::new("gamma", "Gamma"),
            ]);
        if self.attached {
            let generation = self.generation;
            list = list.on_selection_changed(
                context.callback(move |selected| Message::Selected(generation, selected)),
            );
        }
        context.window_title("Reactor UI fixture: selection");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 500.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                list,
                Button::new()
                    .automation_id("select-last")
                    .content("Select last item")
                    .on_click(context.message(Message::SelectLast)),
                Button::new()
                    .automation_id("clear")
                    .content("Clear selection")
                    .on_click(context.message(Message::Clear)),
                Button::new()
                    .automation_id("replace")
                    .content("Replace handler")
                    .on_click(context.message(Message::Replace)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove handler")
                    .on_click(context.message(Message::Remove)),
                TextBlock::new()
                    .automation_id("selection")
                    .text(format!("Selected {:?}", self.selected)),
                TextBlock::new().automation_id("handler").text(format!(
                    "Handler {}; attached {}",
                    self.generation, self.attached
                )),
                TextBlock::new().automation_id("events").text(format!(
                    "Calls {}; received {:?}",
                    self.calls, self.received
                )),
            ))
            .into()
    }
}
