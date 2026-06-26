use crate::controls::*;
use windows_reactor::*;

pub fn number_box_page(_: &(), cx: &mut RenderCx) -> Element {
    let (value, set_value) = cx.use_state(42.0_f64);
    let (clamped, set_clamped) = cx.use_state(5.0_f64);

    page_content(
        "NumberBox",
        "A text control for entering numeric values with validation.",
        vec![
            sample_card(
                "Basic NumberBox",
                vstack((
                    NumberBox::new(value).header("Quantity").on_value_changed({
                        let set_value = set_value;
                        move |v| set_value.call(v)
                    }),
                    text_block(format!("Value: {value}")),
                ))
                .spacing(8.0),
                r#"NumberBox::new(value).header("Quantity").on_value_changed(handler)"#,
            ),
            sample_card(
                "NumberBox with Range",
                vstack((
                    NumberBox::new(clamped)
                        .header("Rating (1-10)")
                        .range(1.0, 10.0)
                        .on_value_changed(set_clamped),
                    text_block(format!("Clamped value: {clamped}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"NumberBox::new(val).header("Rating").range(1.0, 10.0)"#,
            ),
            sample_card(
                "Disabled NumberBox",
                NumberBox::new(99.0).header("Fixed").enabled(false),
                r#"NumberBox::new(99.0).enabled(false)"#,
            ),
        ],
    )
}
