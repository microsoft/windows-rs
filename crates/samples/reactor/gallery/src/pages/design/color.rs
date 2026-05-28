use crate::controls::page_content;
use windows_reactor::*;

pub fn color_page(_: &(), _cx: &mut RenderCx) -> impl Into<Element> {
    let swatches = [
        ("Accent", Color::rgb(0, 120, 212)),
        ("Accent Light 1", Color::rgb(51, 143, 221)),
        ("Accent Light 2", Color::rgb(102, 167, 230)),
        ("Accent Dark 1", Color::rgb(0, 96, 170)),
        ("Accent Dark 2", Color::rgb(0, 72, 128)),
        ("Success", Color::rgb(15, 123, 15)),
        ("Caution", Color::rgb(157, 93, 0)),
        ("Critical", Color::rgb(196, 43, 28)),
    ];

    let mut cards: Vec<Element> = Vec::new();
    for (index, (name, color)) in swatches.iter().enumerate() {
        let card = border(
            vstack((
                text_block(*name).font_size(12.0).bold(),
                text_block(format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b))
                    .font_size(11.0)
                    .opacity(0.7),
            ))
            .spacing(4.0),
        )
        .background(*color)
        .padding(Thickness::uniform(16.0))
        .corner_radius(8.0)
        .grid_row((index / 4) as i32)
        .grid_column((index % 4) as i32);
        cards.push(card.into());
    }

    let rows = vec![GridLength::Auto; swatches.len().div_ceil(4)];
    let mut color_grid = grid(())
        .rows(rows)
        .columns([
            GridLength::Star(1.0),
            GridLength::Star(1.0),
            GridLength::Star(1.0),
            GridLength::Star(1.0),
        ])
        .row_spacing(8.0)
        .column_spacing(8.0);
    color_grid.children = cards;

    page_content(
        "Color",
        "System accent and semantic colors used across WinUI 3 apps.",
        vec![color_grid.into()],
    )
}
