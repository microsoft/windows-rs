use crate::controls::page_content;
use windows_reactor::*;

pub fn iconography_page(_: &(), _cx: &mut RenderCx) -> Element {
    let icons: Vec<(&str, SymbolGlyph)> = vec![
        ("Home", SymbolGlyph::Home),
        ("Setting", SymbolGlyph::Setting),
        ("Find", SymbolGlyph::Find),
        ("Mail", SymbolGlyph::Mail),
        ("Camera", SymbolGlyph::Camera),
        ("Edit", SymbolGlyph::Edit),
        ("Favorite", SymbolGlyph::Favorite),
        ("Flag", SymbolGlyph::Flag),
        ("World", SymbolGlyph::World),
        ("Help", SymbolGlyph::Help),
        ("More", SymbolGlyph::More),
        ("People", SymbolGlyph::People),
        ("Add", SymbolGlyph::Add),
        ("Delete", SymbolGlyph::Delete),
        ("Save", SymbolGlyph::Save),
        ("Back", SymbolGlyph::Back),
    ];

    let mut cards: Vec<Element> = Vec::new();
    for (index, (name, glyph)) in icons.iter().enumerate() {
        let card = border(
            vstack((
                Button::new(*name).icon(*glyph),
                text_block(*name).font_size(11.0).opacity(0.7),
            ))
            .spacing(8.0),
        )
        .background(ThemeRef::CardBackground)
        .border_brush(Color::rgb(160, 160, 160))
        .border_thickness(Thickness::uniform(1.0))
        .padding(Thickness::uniform(16.0))
        .corner_radius(8.0)
        .grid_row((index / 4) as i32)
        .grid_column((index % 4) as i32);
        cards.push(card.into());
    }

    let rows = vec![GridLength::Auto; icons.len().div_ceil(4)];
    let mut icons_grid = grid(())
        .rows(rows)
        .columns([
            GridLength::Star(1.0),
            GridLength::Star(1.0),
            GridLength::Star(1.0),
            GridLength::Star(1.0),
        ])
        .row_spacing(8.0)
        .column_spacing(8.0);
    icons_grid.children = cards;

    page_content(
        "Iconography",
        "Segoe MDL2 Assets icons available through the SymbolGlyph enum.",
        vec![icons_grid.into()],
    )
    .into()
}
