use crate::controls::*;
use windows_reactor::*;

pub fn color_picker_page(_: &(), cx: &mut RenderCx) -> Element {
    let (color, set_color) = cx.use_state((50_u8, 120_u8, 200_u8, 255_u8));

    page_content(
        "ColorPicker",
        "A control that lets a user pick a color.",
        vec![
            sample_card(
                "Full ColorPicker",
                vstack((
                    color_picker(ColorArgb::new(color.0, color.1, color.2))
                        .on_color_changed(move |(a, r, g, b)| set_color.call((r, g, b, a))),
                    text_block(format!(
                        "RGBA({}, {}, {}, {})",
                        color.0, color.1, color.2, color.3
                    ))
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"color_picker(color).on_color_changed(handler)"#,
            ),
            sample_card(
                "Minimal ColorPicker",
                color_picker(ColorArgb::new(200, 50, 50))
                    .hex_input_visible(false)
                    .color_channel_text_input_visible(false),
                r#"color_picker(c).hex_input_visible(false).color_channel_text_input_visible(false)"#,
            ),
        ],
    )
}
