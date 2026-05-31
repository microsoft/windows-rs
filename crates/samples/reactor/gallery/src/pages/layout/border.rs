use crate::controls::*;
use windows_reactor::*;

pub fn border_page(_: &(), cx: &mut RenderCx) -> Element {
    let (radius, set_radius) = cx.use_state(8.0_f64);
    let (thick, set_thick) = cx.use_state(false);

    let thickness = if thick { 4.0 } else { 1.0 };

    page_content(
        "Border",
        "A container that draws a border around its child element.",
        vec![
            sample_card(
                "Dynamic Border",
                vstack((
                    border(text_block("Adjustable border"))
                        .border_brush(Color::rgb(60, 100, 180))
                        .border_thickness(Thickness::uniform(thickness))
                        .padding(Thickness::uniform(16.0))
                        .corner_radius(radius),
                    Slider::new(radius)
                        .range(0.0, 32.0)
                        .header("Corner radius")
                        .on_changed({
                            let set_radius = set_radius;
                            move |value: f64| set_radius.call(value)
                        }),
                    ToggleSwitch::new(thick)
                        .header("Thick border")
                        .on_changed(move |value: bool| set_thick.call(value)),
                ))
                .spacing(12.0),
                r#"border(content)
    .corner_radius(radius)
    .border_thickness(Thickness::uniform(thickness))"#,
            ),
            sample_card(
                "Basic Border",
                border(text_block("Content inside a border"))
                    .padding(Thickness::uniform(16.0))
                    .corner_radius(8.0),
                r#"border(content).padding(Thickness::uniform(16.0)).corner_radius(8.0)"#,
            ),
            sample_card(
                "Colored Border",
                border(text_block("Styled border"))
                    .background(Color::rgb(60, 100, 180))
                    .padding(Thickness::uniform(16.0))
                    .corner_radius(4.0),
                r#"border(content).background(Color::rgb(...)).corner_radius(4.0)"#,
            ),
        ],
    )
}
