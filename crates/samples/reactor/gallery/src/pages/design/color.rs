use crate::controls::*;
use windows_reactor::*;

pub struct ColorPage;

impl Component for ColorPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        let swatches = [
            ("Accent", Color::rgb(0, 120, 212)),
            ("Accent Light", Color::rgb(51, 143, 221)),
            ("Accent Dark", Color::rgb(0, 72, 128)),
            ("Success", Color::rgb(15, 123, 15)),
            ("Caution", Color::rgb(157, 93, 0)),
            ("Critical", Color::rgb(196, 43, 28)),
        ];
        let cards = swatches
            .into_iter()
            .enumerate()
            .map(|(index, (name, color))| {
                KeyedView::new(
                    name,
                    Border::new()
                        .background(color)
                        .padding(16.0)
                        .corner_radius(8.0)
                        .grid_row((index / 3) as i32)
                        .grid_column((index % 3) as i32)
                        .content(
                            StackPanel::new().spacing(4.0).children((
                                TextBlock::new().text(name).font_weight(FontWeight::BOLD),
                                TextBlock::new()
                                    .text(format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b))
                                    .font_size(12.0),
                            )),
                        ),
                )
            });
        page_content(
            "Color",
            "System accent and semantic colors used across WinUI 3 apps.",
            [KeyedView::new(
                "palette",
                Grid::new()
                    .rows([GridLength::Auto, GridLength::Auto])
                    .columns([GridLength::STAR; 3])
                    .row_spacing(8.0)
                    .column_spacing(8.0)
                    .keyed_children(cards),
            )],
        )
    }
}
