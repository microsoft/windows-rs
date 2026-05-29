use crate::controls::*;
use windows_reactor::*;

pub fn breadcrumb_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (clicked, set_clicked) = cx.use_state(-1_i32);

    page_content(
        "BreadcrumbBar",
        "A trail showing the navigation path.",
        vec![
            sample_card(
                "Basic BreadcrumbBar",
                BreadcrumbBar::new(["Home", "Documents", "Report"]),
                r#"BreadcrumbBar::new(["Home", "Documents", "Report"])"#,
            ),
            sample_card(
                "Interactive BreadcrumbBar",
                vstack((
                    BreadcrumbBar::new(["Root", "Users", "Settings", "Profile"])
                        .on_item_clicked(move |i| set_clicked.call(i)),
                    text_block(format!("Clicked index: {clicked}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"BreadcrumbBar::new(items).on_item_clicked(|idx| handler(idx))"#,
            ),
        ],
    )
}
