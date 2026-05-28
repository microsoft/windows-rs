use crate::controls::*;
use windows_reactor::*;

pub fn info_badge_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (count, set_count) = cx.use_state(3_i32);

    page_content(
        "InfoBadge",
        "A small indicator conveying status on another element.",
        vec![
            sample_card(
                "Dynamic Counter",
                vstack((
                    hstack((
                        button("Add notification").on_click({
                            let set_count = set_count.clone();
                            move || set_count.call(count + 1)
                        }),
                        button("Clear").on_click({
                            let set_count = set_count;
                            move || set_count.call(0)
                        }),
                    ))
                    .spacing(8.0),
                    hstack((
                        InfoBadge::numeric(count),
                        text_block(format!("{count} unread")).opacity(0.6),
                    ))
                    .spacing(8.0),
                ))
                .spacing(12.0),
                r#"InfoBadge::numeric(count) // updates reactively"#,
            ),
            sample_card(
                "Numeric InfoBadge",
                hstack((
                    InfoBadge::numeric(1),
                    InfoBadge::numeric(12),
                    InfoBadge::numeric(99),
                ))
                .spacing(16.0),
                r#"InfoBadge::numeric(1), InfoBadge::numeric(99)"#,
            ),
            sample_card(
                "Dot InfoBadge",
                hstack((
                    button("Notifications").icon(SymbolGlyph::Mail),
                    InfoBadge::dot(),
                ))
                .spacing(8.0),
                r#"InfoBadge::dot() // compact presence indicator"#,
            ),
        ],
    )
}
