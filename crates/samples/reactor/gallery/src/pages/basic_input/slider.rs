use crate::controls::*;
use windows_reactor::*;

pub fn slider_page(_: &(), cx: &mut RenderCx) -> Element {
    let (volume, set_volume) = cx.use_state(35.0_f64);
    let (brightness, set_brightness) = cx.use_state(60.0_f64);
    let (temperature, set_temperature) = cx.use_state(21.0_f64);

    page_content(
        "Slider",
        "Select a value from a range with precise, touch-friendly input.",
        vec![
            sample_card(
                "Basic Slider",
                vstack((
                    Slider::new(volume)
                        .range(0.0, 100.0)
                        .step(1.0)
                        .header("Volume")
                        .on_value_changed({
                            let set_volume = set_volume;
                            move |value| set_volume.call(value)
                        }),
                    text_block(format!("Volume: {volume:.0}%")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"Slider::new(volume)
    .range(0.0, 100.0)
    .step(1.0)
    .on_value_changed(...)"#,
            ),
            sample_card(
                "Vertical Slider",
                hstack((
                    Slider::new(brightness)
                        .range(0.0, 100.0)
                        .step(5.0)
                        .header("Brightness")
                        .vertical()
                        .height(140.0)
                        .on_value_changed({
                            let set_brightness = set_brightness;
                            move |value| set_brightness.call(value)
                        }),
                    vstack((
                        text_block("Screen brightness").bold(),
                        text_block(format!("{brightness:.0}%")).font_size(18.0),
                        text_block("Drag the thumb to preview a stronger or softer look.")
                            .opacity(0.6),
                    ))
                    .spacing(4.0),
                ))
                .spacing(16.0),
                r#"Slider::new(brightness)
    .vertical()
    .height(140.0)
    .on_value_changed(...)"#,
            ),
            sample_card(
                "Range with Value Label",
                vstack((
                    Slider::new(temperature)
                        .range(16.0, 30.0)
                        .step(0.5)
                        .header("Room temperature")
                        .on_value_changed({
                            let set_temperature = set_temperature;
                            move |value| set_temperature.call(value)
                        }),
                    text_block(format!("Target: {temperature:.1} °C")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"Slider::new(temperature)
    .range(16.0, 30.0)
    .step(0.5)
    .on_value_changed(...)"#,
            ),
        ],
    )
}
