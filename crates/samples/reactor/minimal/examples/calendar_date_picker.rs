//! Sample for the `CalendarDatePicker` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (label, set_label) = cx.use_state(String::from("Pick a date to see days from today"));

    let on_date = move |date: Option<DateTime>| {
        let text = match date {
            Some(selected) => {
                let now = DateTime::now();
                let delta = selected.checked_duration_since(now);
                match delta {
                    Some(span) => {
                        let days = span.whole_days();
                        match days.cmp(&0) {
                            std::cmp::Ordering::Greater => format!("{days} day(s) from now"),
                            std::cmp::Ordering::Less => format!("{} day(s) ago", days.abs()),
                            std::cmp::Ordering::Equal => String::from("That's today!"),
                        }
                    }
                    None => String::from("Date too far away to compute"),
                }
            }
            None => String::from("Selection cleared"),
        };
        set_label.call(text);
    };

    vstack((
        calendar_date_picker()
            .header("Select a date")
            .placeholder_text("Choose...")
            .on_changed(on_date),
        text_block(&*label),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("CalendarDatePicker", app)
}
