use crate::controls::*;
use windows_reactor::*;

pub fn calendar_view_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (today_highlighted, set_today_highlighted) = cx.use_state(true);
    let (selection_changes, set_selection_changes) = cx.use_state(0_u32);

    page_content(
        "CalendarView",
        "A calendar display for selecting a date.",
        vec![
            sample_card(
                "Basic CalendarView",
                vstack((
                    ToggleSwitch::new(today_highlighted)
                        .header("Highlight today")
                        .on_changed({
                            let set_today_highlighted = set_today_highlighted;
                            move |value| set_today_highlighted.call(value)
                        }),
                    calendar_view()
                        .today_highlighted(today_highlighted)
                        .on_changed({
                            let set_selection_changes = set_selection_changes;
                            move || set_selection_changes.call(selection_changes + 1)
                        }),
                    text_block(format!("Selection changed {selection_changes} time(s)"))
                        .opacity(0.6),
                ))
                .spacing(8.0),
                r#"ToggleSwitch::new(today_highlighted).on_changed(...)
calendar_view()
    .today_highlighted(today_highlighted)
    .on_changed(|| set_selection_changes.call(selection_changes + 1))"#,
            ),
            sample_card(
                "CalendarView without Labels",
                calendar_view()
                    .today_highlighted(true)
                    .group_label_visible(false),
                "calendar_view().group_label_visible(false)",
            ),
        ],
    )
}
