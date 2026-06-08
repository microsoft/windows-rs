use crate::controls::*;
use windows_reactor::*;

pub fn canvas_page(_: &(), cx: &mut RenderCx) -> Element {
    let (x, set_x) = cx.use_state(100.0_f64);
    let (y, set_y) = cx.use_state(80.0_f64);

    let children: Vec<Element> = vec![
        text_block("Fixed")
            .canvas_left(10.0)
            .canvas_top(10.0)
            .into(),
        border(text_block("Move me!"))
            .background(ThemeRef::CardBackground)
            .padding(Thickness::uniform(8.0))
            .corner_radius(4.0)
            .canvas_left(x)
            .canvas_top(y)
            .into(),
    ];

    page_content(
        "Canvas",
        "Absolute positioning of child elements.",
        vec![sample_card(
            "Draggable Position",
            vstack((
                Canvas::new(children).width(320.0).height(200.0),
                Slider::new(x)
                    .range(0.0, 250.0)
                    .header("X position")
                    .on_value_changed(move |v: f64| set_x.call(v)),
                Slider::new(y)
                    .range(0.0, 160.0)
                    .header("Y position")
                    .on_value_changed(move |v: f64| set_y.call(v)),
            ))
            .spacing(8.0),
            r#"Canvas::new([el.canvas_left(x).canvas_top(y)]) // x, y from sliders"#,
        )],
    )
}
