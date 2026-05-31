//! Minimal sample for the `AutoSuggestBox` element.

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

fn app(cx: &mut RenderCx) -> Element {
    let (query, set_query) = cx.use_state(String::new());
    let (chosen, set_chosen) = cx.use_state(String::new());

    // Filter suggestions based on current text.
    let suggestions: Vec<String> = if query.is_empty() {
        Vec::new()
    } else {
        FRUITS
            .iter()
            .filter(|f| f.to_lowercase().contains(&query.to_lowercase()))
            .map(|s| s.to_string())
            .collect()
    };

    let on_text = move |text: String| set_query.call(text);
    let on_chosen = move |item: String| set_chosen.call(item);

    vstack((
        auto_suggest_box(query)
            .items(suggestions)
            .placeholder("Search fruits…")
            .on_text_changed(on_text)
            .on_suggestion_chosen(on_chosen),
        text_block(if chosen.is_empty() {
            "No selection".to_string()
        } else {
            format!("Chosen: {chosen}")
        }),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("AutoSuggestBox Sample").render(app)
}
