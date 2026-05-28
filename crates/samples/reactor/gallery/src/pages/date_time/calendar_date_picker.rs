use crate::controls::*;
use windows_reactor::*;

pub fn calendar_date_picker_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (selected_date, set_selected_date) = cx.use_state(String::from("No date selected"));

    page_content(
        "CalendarDatePicker",
        "Pick a date from a calendar dropdown.",
        vec![
            sample_card(
                "Basic CalendarDatePicker",
                vstack((
                    calendar_date_picker()
                        .header("Appointment Date")
                        .placeholder_text("Select a date")
                        .today_highlighted(true)
                        .on_changed({
                            let set_selected_date = set_selected_date;
                            move |value| {
                                set_selected_date.call(match value {
                                    Some(date) => format!("Selected: {date}"),
                                    None => String::from("Selection cleared"),
                                });
                            }
                        }),
                    text_block(selected_date).opacity(0.6),
                ))
                .spacing(8.0),
                r#"calendar_date_picker()
    .header(\"Appointment Date\")
    .on_changed(|value| set_selected_date.call(...))"#,
            ),
            sample_card(
                "Disabled CalendarDatePicker",
                calendar_date_picker()
                    .header("Locked Date")
                    .placeholder_text("Cannot change")
                    .enabled(false),
                r#"calendar_date_picker().enabled(false)"#,
            ),
        ],
    )
}
