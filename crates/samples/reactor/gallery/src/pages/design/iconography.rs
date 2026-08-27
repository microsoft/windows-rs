use crate::controls::*;
use windows_reactor::*;

pub struct IconographyPage;

impl Component for IconographyPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        let icons = [
            ("Home", Symbol::Home),
            ("Setting", Symbol::Setting),
            ("Find", Symbol::Find),
            ("Mail", Symbol::Mail),
            ("Camera", Symbol::Camera),
            ("Edit", Symbol::Edit),
            ("Favorite", Symbol::Favorite),
            ("Flag", Symbol::Flag),
            ("World", Symbol::World),
            ("Help", Symbol::Help),
            ("More", Symbol::More),
            ("People", Symbol::People),
        ];
        let cards = icons
            .into_iter()
            .enumerate()
            .map(|(index, (name, symbol))| {
                KeyedView::new(
                    name,
                    Border::new()
                        .background(ThemeBrush::CardBackground)
                        .border_brush(ThemeBrush::CardStroke)
                        .border_thickness(1.0)
                        .padding(16.0)
                        .corner_radius(8.0)
                        .grid_row((index / 4) as i32)
                        .grid_column((index % 4) as i32)
                        .content(StackPanel::new().spacing(8.0).children((
                            SymbolIcon::new().symbol(symbol),
                            TextBlock::new().text(name).font_size(11.0).opacity(0.7),
                        ))),
                )
            });
        page_content(
            "Iconography",
            "Segoe Fluent Icons symbols available through the Symbol enum.",
            [KeyedView::new(
                "symbols",
                Grid::new()
                    .rows([GridLength::Auto; 3])
                    .columns([GridLength::STAR; 4])
                    .row_spacing(8.0)
                    .column_spacing(8.0)
                    .keyed_children(cards),
            )],
        )
    }
}
