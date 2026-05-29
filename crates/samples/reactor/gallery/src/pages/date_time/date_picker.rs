use crate::controls::*;
use windows_reactor::*;

pub fn date_picker_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected_date, set_selected_date) = cx.use_state(String::from("No date selected"));

    page_content(
        "DatePicker",
        "Pick a date using spinners.",
        vec![
            sample_card(
                "Full DatePicker",
                vstack((
                    date_picker().header("Select date").on_changed({
                        let set_selected_date = set_selected_date;
                        move |date| set_selected_date.call(format!("Selected: {date}"))
                    }),
                    text_block(selected_date).opacity(0.6),
                ))
                .spacing(8.0),
                r#"date_picker()
    .header(\"Select date\")
    .on_changed(|date| set_selected_date.call(format!(\"Selected: {date}\")))"#,
            ),
            sample_card(
                "Month and Year Only",
                date_picker().header("Month/Year").day_visible(false),
                r#"date_picker().header("Month/Year").day_visible(false)"#,
            ),
        ],
    )
}
