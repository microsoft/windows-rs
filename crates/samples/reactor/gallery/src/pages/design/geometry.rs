use crate::controls::*;
use windows_reactor::*;

pub fn geometry_page(_: &(), _cx: &mut RenderCx) -> Element {
    page_content(
        "Geometry",
        "WinUI defines shared corner radius values so controls and surfaces have a consistent shape language.",
        vec![
            sample_card(
                "Corner Radius Resources",
                vstack((
                    text_block("WinUI provides two standard corner radius values. ControlCornerRadius (4px) for interactive controls and OverlayCornerRadius (8px) for overlay surfaces.")
                        .opacity(0.6)
                        .font_size(13.0),
                    radius_row("Control", 4.0, "Buttons, text fields, combo boxes, sliders, toggles"),
                    radius_row("Overlay", 8.0, "Dialogs, flyouts, teaching tips, menus"),
                ))
                .spacing(12.0)
                ,
                r#"// ControlCornerRadius = 4px
border(content).corner_radius(4.0)

// OverlayCornerRadius = 8px
border(dialog).corner_radius(8.0)"#,
            ),
            sample_card(
                "ControlCornerRadius (4px)",
                vstack((
                    text_block("Interactive controls use 4px rounding:").opacity(0.6).font_size(13.0),
                    hstack((
                        border(text_block("Button").padding(Thickness::xy(12.0, 6.0)))
                            .corner_radius(4.0)
                            .border_brush(Color::rgb(128, 128, 128))
                            .border_thickness(Thickness::uniform(1.0)),
                        border(text_block("Input").padding(Thickness::xy(12.0, 6.0)))
                            .corner_radius(4.0)
                            .border_brush(Color::rgb(128, 128, 128))
                            .border_thickness(Thickness::uniform(1.0)),
                        border(text_block("Tag").padding(Thickness::xy(8.0, 4.0)))
                            .corner_radius(4.0)
                            .border_brush(Color::rgb(128, 128, 128))
                            .border_thickness(Thickness::uniform(1.0)),
                    ))
                    .spacing(8.0),
                ))
                .spacing(8.0)
                ,
                r#"border(content)
    .corner_radius(4.0)
    .border_brush(Color::rgb(128, 128, 128))
    .border_thickness(Thickness::uniform(1.0))"#,
            ),
            sample_card(
                "OverlayCornerRadius (8px)",
                vstack((
                    text_block("Overlay surfaces use 8px rounding:").opacity(0.6).font_size(13.0),
                    border(
                        vstack((
                            text_block("Dialog Title").font_size(16.0).bold(),
                            text_block("This surface uses OverlayCornerRadius for its border rounding.").opacity(0.6).font_size(13.0),
                        ))
                        .spacing(8.0)
                        .padding(Thickness::uniform(16.0)),
                    )
                    .corner_radius(8.0)
                    .border_brush(Color::rgb(128, 128, 128))
                    .border_thickness(Thickness::uniform(1.0)),
                ))
                .spacing(8.0)
                ,
                r#"border(dialog_content)
    .corner_radius(8.0)  // OverlayCornerRadius"#,
            ),
            sample_card(
                "Comparing Radii",
                hstack((
                    vstack((
                        Shape::rectangle()
                            .fill(Color::rgb(0, 120, 212))
                            .corner_radius(4.0)
                            .width(48.0)
                            .height(48.0),
                        text_block("4px
Control").font_size(11.0).opacity(0.6),
                    ))
                    .spacing(4.0),
                    vstack((
                        Shape::rectangle()
                            .fill(Color::rgb(0, 120, 212))
                            .corner_radius(8.0)
                            .width(48.0)
                            .height(48.0),
                        text_block("8px
Overlay").font_size(11.0).opacity(0.6),
                    ))
                    .spacing(4.0),
                ))
                .spacing(24.0)
                ,
                r#"Shape::rectangle()
    .fill(Color::rgb(0, 120, 212))
    .corner_radius(4.0)  // vs .corner_radius(8.0)"#,
            ),
        ],
    )
}

fn radius_row(name: &str, value: f64, recommendation: &str) -> Element {
    hstack((
        Shape::rectangle()
            .fill(Color::rgb(0, 120, 212))
            .corner_radius(value)
            .width(48.0)
            .height(48.0),
        vstack((
            text_block(format!("{name} — {value}px"))
                .font_size(14.0)
                .bold(),
            text_block(recommendation).font_size(12.0).opacity(0.6),
        ))
        .spacing(2.0),
    ))
    .spacing(12.0)
    .into()
}
