use crate::controls::*;
use windows_reactor::*;

pub fn viewbox_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (content_size, set_content_size) = cx.use_state(120.0_f64);

    page_content(
        "Viewbox",
        "Scales its child to fill available space.",
        vec![sample_card(
            "Resizable Content",
            vstack((
                viewbox(
                    border(
                        text_block(format!("{} px", content_size.round() as i32))
                            .font_size((content_size / 4.0).max(12.0))
                            .bold(),
                    )
                    .width(content_size)
                    .height(content_size / 2.0)
                    .padding(Thickness::uniform(12.0))
                    .corner_radius(8.0),
                )
                .width(200.0)
                .height(100.0),
                Slider::new(content_size)
                    .range(60.0, 220.0)
                    .header("Content size")
                    .on_changed(move |value: f64| set_content_size.call(value)),
                text_block("The Viewbox stays fixed while its child grows or shrinks.")
                    .opacity(0.6),
            ))
            .spacing(8.0),
            r#"viewbox(content).width(200.0).height(100.0) // content size comes from state"#,
        )],
    )
}
