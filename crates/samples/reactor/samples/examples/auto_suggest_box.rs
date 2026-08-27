#![windows_subsystem = "windows"]

use windows_reactor::*;

const FRUITS: &[&str] = &[
    "Apple",
    "Apricot",
    "Banana",
    "Blueberry",
    "Cherry",
    "Grape",
    "Lemon",
    "Mango",
    "Orange",
    "Peach",
    "Pear",
    "Pineapple",
    "Strawberry",
    "Watermelon",
];

enum Message {
    TextChanged(String),
    SuggestionChosen(String),
}

struct AutoSuggestBoxSample {
    query: String,
    chosen: String,
}

impl Component for AutoSuggestBoxSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            query: String::new(),
            chosen: String::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::TextChanged(query) => self.query = query,
            Message::SuggestionChosen(chosen) => self.chosen = chosen,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let query = self.query.to_lowercase();
        let suggestions = if query.is_empty() {
            Vec::new()
        } else {
            FRUITS
                .iter()
                .filter(|fruit| fruit.to_lowercase().contains(&query))
                .copied()
                .collect()
        };

        context.window_title("AutoSuggestBox");
        StackPanel::new().spacing(12.0).children((
            AutoSuggestBox::new()
                .text(self.query.clone())
                .items_source(suggestions)
                .placeholder_text("Search fruits...")
                .on_text_changed(context.callback(Message::TextChanged))
                .on_suggestion_chosen(context.callback(Message::SuggestionChosen)),
            if self.chosen.is_empty() {
                "No selection".to_string()
            } else {
                format!("Chosen: {}", self.chosen)
            },
        ))
    }
}

fn main() {
    App::run_component::<AutoSuggestBoxSample>(()).unwrap();
}
