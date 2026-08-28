use crate::controls::*;
use windows_reactor::*;

pub struct AutoSuggestBoxPage {
    query: String,
    chosen: String,
}

#[derive(Clone)]
pub enum Message {
    Query(String),
    Chosen(String),
}

impl Component for AutoSuggestBoxPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            query: String::new(),
            chosen: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Query(value) => self.query = value,
            Message::Chosen(value) => self.chosen = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let query = self.query.to_lowercase();
        let suggestions = ["Apple", "Banana", "Cherry", "Date", "Elderberry"]
            .into_iter()
            .filter(|item| query.is_empty() || item.to_lowercase().contains(&query));
        page_content(
            "AutoSuggestBox",
            "Text input that shows suggestions as you type.",
            [KeyedView::new(
                "basic",
                sample_card(
                    "Basic AutoSuggestBox",
                    StackPanel::new().spacing(8.0).children((
                        AutoSuggestBox::new()
                            .text(&self.query)
                            .items_source(suggestions)
                            .placeholder_text("Search fruits...")
                            .on_text_changed(context.callback(Message::Query))
                            .on_suggestion_chosen(context.callback(Message::Chosen)),
                        TextBlock::new()
                            .text(format!("Query: \"{}\"", self.query))
                            .opacity(0.6),
                        TextBlock::new()
                            .text(format!("Chosen: {}", self.chosen))
                            .opacity(0.6),
                    )),
                    "AutoSuggestBox::new().items_source(suggestions).on_text_changed(handler)",
                ),
            )],
        )
    }
}
