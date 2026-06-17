use crate::controls::*;
use windows_reactor::*;

pub fn teaching_tip_page(_: &(), cx: &mut RenderCx) -> Element {
    let (show_basic, set_basic) = cx.use_state(false);
    let (show_action, set_action) = cx.use_state(false);
    let (action_count, set_count) = cx.use_state(0_i32);

    let close_basic = set_basic.clone();
    let close_action = set_action.clone();

    page_content(
        "TeachingTip",
        "A notification flyout for guiding users through features.",
        vec![
            sample_card(
                "Basic TeachingTip",
                vstack((
                    button("Show Tip").on_click(move || set_basic.call(true)),
                    teaching_tip("Did you know?")
                        .subtitle("You can customize this teaching tip with a subtitle.")
                        .is_open(show_basic)
                        .on_closed(move || close_basic.call(false)),
                ))
                .spacing(8.0),
                r#"teaching_tip("Title").subtitle("...").is_open(show).on_closed(handler)"#,
            ),
            sample_card(
                "TeachingTip with Action",
                vstack((
                    button("Show Action Tip").on_click(move || set_action.call(true)),
                    teaching_tip("Take Action")
                        .subtitle("Click the action button to increment the counter.")
                        .is_open(show_action)
                        .action_button("Got it!")
                        .close_button("Dismiss")
                        .on_action_button_click({
                            let set_count = set_count;
                            move || set_count.call(action_count + 1)
                        })
                        .on_closed(move || close_action.call(false)),
                    text_block(format!("Action clicked: {action_count} times")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"teaching_tip("Title").action_button("Got it!").on_action_button_click(handler)"#,
            ),
        ],
    )
}
