use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (keyed, set_keyed) = cx.use_state(true);
    let (last_close_key, set_last_close_key) = cx.use_state(String::new());

    let mut item = TabItem::new("Document", text_block("Close the tab to inspect its key."));
    if keyed {
        item = item.with_key("document");
    }

    vstack((
        button(if keyed {
            "Remove item key"
        } else {
            "Restore item key"
        })
        .on_click(move || set_keyed.call(!keyed)),
        TabView::new([item]).on_close_requested(move |key: String| {
            set_last_close_key.call(if key.is_empty() {
                "<none>".to_string()
            } else {
                key
            });
        }),
        text_block(format!(
            "configured key: {}; last close request: {}",
            if keyed { "document" } else { "<none>" },
            if last_close_key.is_empty() {
                "<not requested>"
            } else {
                &last_close_key
            }
        )),
    ))
    .spacing(8.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("TabView Item Key", app)
}
