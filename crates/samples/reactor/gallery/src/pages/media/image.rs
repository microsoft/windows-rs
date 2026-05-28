use crate::controls::*;
use windows_reactor::*;

pub fn image_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (stretch_idx, set_stretch) = cx.use_state(0_i32);

    let stretch = match stretch_idx {
        1 => ImageStretch::UniformToFill,
        2 => ImageStretch::Fill,
        3 => ImageStretch::None,
        _ => ImageStretch::Uniform,
    };
    let stretch_name = match stretch_idx {
        1 => "UniformToFill",
        2 => "Fill",
        3 => "None",
        _ => "Uniform",
    };

    page_content(
        "Image",
        "Displays an image from a file or URI.",
        vec![sample_card(
            "Stretch Modes",
            vstack((
                border(
                    Image::new("https://picsum.photos/id/237/400/200")
                        .stretch(stretch)
                        .width(300.0)
                        .height(150.0),
                )
                .border_brush(Color::rgb(200, 200, 200))
                .border_thickness(Thickness::uniform(1.0)),
                hstack((
                    button("Uniform").on_click({
                        let set = set_stretch.clone();
                        move || set.call(0)
                    }),
                    button("UniformToFill").on_click({
                        let set = set_stretch.clone();
                        move || set.call(1)
                    }),
                    button("Fill").on_click({
                        let set = set_stretch.clone();
                        move || set.call(2)
                    }),
                    button("None").on_click({
                        let set = set_stretch;
                        move || set.call(3)
                    }),
                ))
                .spacing(4.0),
                text_block(format!("Current: {stretch_name}")).opacity(0.6),
            ))
            .spacing(8.0),
            r#"Image::new(uri).stretch(ImageStretch::Uniform).width(300.0).height(150.0)"#,
        )],
    )
}
