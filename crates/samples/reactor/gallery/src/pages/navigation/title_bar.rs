use crate::controls::*;
use windows_reactor::*;

pub fn title_bar_page(_: &(), _cx: &mut RenderCx) -> impl Into<Element> {
    page_content(
            "TitleBar",
            "A customizable application title bar.",
            vec![sample_card(
                "Basic TitleBar",
                text_block("TitleBar is automatically configured via App::new().title(...).\nSee the gallery shell for a live example.")
                    ,
                r#"App::new().title("My App").run(|| Shell)"#,
            )],
        )
}
