#![windows_subsystem = "windows"]

use windows_reactor::{AutoSuggestBox, Element, RenderCx, TextBlock, vstack};

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

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let query = cx.use_state(String::new);
    let current_query = query.value();
    let update_query = query.clone();
    let choose_query = query;
    let chosen = cx.use_state(|| None::<u64>);
    let current_chosen = chosen.value();
    let update_chosen = chosen;
    let suggestions = FRUITS
        .iter()
        .enumerate()
        .filter(|(_, fruit)| {
            !current_query.is_empty()
                && fruit.to_lowercase().contains(&current_query.to_lowercase())
        })
        .map(|(index, fruit)| ((index + 1) as u64, *fruit));

    vstack(
        12.0,
        [
            AutoSuggestBox::new(&current_query, move |text| {
                update_query.set(text);
            })
            .items(suggestions)
            .placeholder_text("Search fruits...")
            .on_suggestion_chosen(move |key| {
                update_chosen.set(Some(key));
                choose_query.set(FRUITS[key as usize - 1].to_string());
            })
            .build(),
            TextBlock::new(match current_chosen {
                Some(key) => format!("Chosen: {}", FRUITS[key as usize - 1]),
                None => "No selection".to_string(),
            })
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("AutoSuggestBox", app)
}
