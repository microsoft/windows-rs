use crate::controls::*;
use windows_reactor::*;

pub fn time_picker_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected_time, set_selected_time) = cx.use_state(String::from("No time selected"));

    page_content(
        "TimePicker",
        "Pick a time using spinners.",
        vec![
            sample_card(
                "Basic TimePicker",
                vstack((
                    time_picker()
                        .header("Select time")
                        .on_selected_time_changed({
                            let set_selected_time = set_selected_time;
                            move |time: windows_reactor::TimeSpan| {
                                let hours = time.whole_hours();
                                let minutes = time.whole_minutes() % 60;
                                set_selected_time
                                    .call(format!("Selected: {hours:02}:{minutes:02}"));
                            }
                        }),
                    text_block(selected_time).opacity(0.6),
                ))
                .spacing(8.0),
                r#"time_picker()
    .header(\"Select time\")
    .on_selected_time_changed(|time| set_selected_time.call(...))"#,
            ),
            sample_card(
                "15-Minute Increments",
                time_picker().header("Meeting time").minute_increment(15),
                r#"time_picker().header("Meeting time").minute_increment(15)"#,
            ),
        ],
    )
}
