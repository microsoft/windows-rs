use crate::controls::*;
use windows_reactor::*;

pub fn auto_suggest_box_page(_: &(), cx: &mut RenderCx) -> Element {
    let (query, set_query) = cx.use_state(String::new());

    let suggestions: Vec<String> = ["Apple", "Banana", "Cherry", "Date", "Elderberry"]
        .iter()
        .filter(|s| query.is_empty() || s.to_lowercase().contains(&query.to_lowercase()))
        .map(|s| s.to_string())
        .collect();

    page_content(
        "AutoSuggestBox",
        "Text input that shows suggestions as you type.",
        vec![sample_card(
            "Basic AutoSuggestBox",
            vstack((
                auto_suggest_box(query.clone())
                    .items(suggestions)
                    .placeholder("Search fruits...")
                    .on_text_changed(move |s: String| set_query.call(s)),
                text_block(format!("Query: \"{query}\"")).opacity(0.6),
            ))
            .spacing(8.0),
            r#"auto_suggest_box(query).items(suggestions).on_text_changed(handler)"#,
        )],
    )
    .into()
}
