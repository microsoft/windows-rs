use super::*;

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
