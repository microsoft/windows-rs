use crate::controls::*;
use windows_reactor::*;

pub fn progress_ring_page(_: &(), cx: &mut RenderCx) -> Element {
    let (value, set_value) = cx.use_state(75.0_f64);

    page_content(
        "ProgressRing",
        "A circular indicator of ongoing progress.",
        vec![
            sample_card(
                "Indeterminate ProgressRing",
                ProgressRing::indeterminate(),
                "ProgressRing::indeterminate()",
            ),
            sample_card(
                "Determinate ProgressRing",
                vstack((
                    ProgressRing::new(value),
                    Slider::new(value)
                        .range(0.0, 100.0)
                        .on_value_changed(move |v: f64| set_value.call(v)),
                    text_block(format!("{value:.0}%")).opacity(0.6),
                ))
                .spacing(8.0),
                "ProgressRing::new(value) // controlled by slider",
            ),
        ],
    )
}
