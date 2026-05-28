use crate::controls::*;
use windows_reactor::*;

pub fn repeat_button_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (count, set_count) = cx.use_state(0_i32);
    let (fast_count, set_fast) = cx.use_state(0_i32);

    page_content(
        "RepeatButton",
        "A button that raises click events repeatedly while pressed.",
        vec![
            sample_card(
                "Basic RepeatButton",
                vstack((
                    RepeatButton::new("Hold me").on_click({
                        let set_count = set_count;
                        move || set_count.call(count + 1)
                    }),
                    text_block(format!("Count: {count}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RepeatButton::new("Hold me").on_click(handler)"#,
            ),
            sample_card(
                "Fast Repeat (50ms interval)",
                vstack((
                    RepeatButton::new("Fast +1")
                        .delay(200)
                        .interval(50)
                        .on_click(move || set_fast.call(fast_count + 1)),
                    text_block(format!("Fast count: {fast_count}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RepeatButton::new("Fast").delay(200).interval(50).on_click(handler)"#,
            ),
        ],
    )
}
