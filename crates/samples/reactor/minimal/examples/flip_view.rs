//! Minimal sample for the `flip_view` templated list.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (page, set_page) = cx.use_state(0_i32);

    let items: Vec<String> = ["Red", "Green", "Blue"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let last = (items.len() as i32) - 1;

    let prev = {
        let set_page = set_page.clone();
        move || set_page.call((page - 1).max(0))
    };
    let next = {
        let set_page = set_page.clone();
        move || set_page.call((page + 1).min(last))
    };
    let on_sel = {
        let set_page = set_page;
        move |i: i32| set_page.call(i)
    };

    vstack((
        flip_view(items, |s, _idx| {
            border(text_block(s.clone()).font_size(20.0).bold())
                .background(Color::rgb(245, 230, 220))
                .padding(Thickness::uniform(24.0))
        })
        .with_key_selector(|s| s.clone())
        .selected_index(page)
        .on_selection_changed(on_sel)
        .height(180.0),
        // External controls drive the same bound selection.
        hstack((
            button("Prev").on_click(prev),
            button("Next").on_click(next),
            text_block(format!("page = {page}")).opacity(0.7),
        ))
        .spacing(8.0),
    ))
    .spacing(8.0)
    .max_width(360.0)
}

fn main() -> Result<()> {
    App::new()
        .title("Sample")
        .eager_templated_realization(true)
        .render(app)
}
