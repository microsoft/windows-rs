use crate::controls::*;
use windows_reactor::*;

pub fn progress_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (value, set_value) = cx.use_state(60.0_f64);
    let (loading, set_loading) = cx.use_state(true);

    page_content(
        "ProgressBar",
        "A horizontal bar that shows progress of an operation.",
        vec![
            sample_card(
                "Determinate ProgressBar",
                vstack((
                    ProgressBar::new(value).width(300.0),
                    text_block(format!("Progress: {value:.0}%")).opacity(0.6),
                    Slider::new(value)
                        .range(0.0, 100.0)
                        .on_changed(move |v: f64| set_value.call(v))
                        .width(300.0),
                ))
                .spacing(8.0),
                r#"ProgressBar::new(value).width(300.0)
Slider::new(value).range(0.0, 100.0).on_changed(handler)"#,
            ),
            sample_card(
                "Indeterminate ProgressBar",
                vstack((
                    if loading {
                        ProgressBar::indeterminate().width(300.0)
                    } else {
                        ProgressBar::new(100.0).width(300.0)
                    },
                    ToggleSwitch::new(loading)
                        .on_content("Loading")
                        .off_content("Complete")
                        .on_changed(move |v: bool| set_loading.call(v)),
                ))
                .spacing(8.0),
                r#"if loading {
    ProgressBar::indeterminate()
} else {
    ProgressBar::new(100.0)
}"#,
            ),
        ],
    )
    .into()
}
