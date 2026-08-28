use crate::controls::*;
use windows_reactor::*;

pub struct GridPage {
    wide: bool,
}

impl Component for GridPage {
    type Message = bool;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { wide: false }
    }

    fn update(&mut self, wide: bool, _: &ComponentContext<Self>) {
        self.wide = wide;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let cell = |text, row, column| {
            Border::new()
                .background(ThemeBrush::CardBackground)
                .padding(12.0)
                .grid_row(row)
                .grid_column(column)
                .content(text)
        };
        let dynamic = if self.wide {
            Grid::new()
                .rows([GridLength::Auto])
                .columns([GridLength::STAR; 3])
                .column_spacing(8.0)
                .children((
                    cell("Column 1", 0, 0),
                    cell("Column 2", 0, 1),
                    cell("Column 3", 0, 2),
                ))
        } else {
            Grid::new()
                .rows([GridLength::Auto; 2])
                .columns([GridLength::STAR; 2])
                .row_spacing(8.0)
                .column_spacing(8.0)
                .children((
                    cell("Row 1, column 1", 0, 0),
                    cell("Row 1, column 2", 0, 1),
                    cell("Row 2, column 1", 1, 0),
                    cell("Row 2, column 2", 1, 1),
                ))
        };
        page_content(
            "Grid",
            "Arranges children in rows and columns with star, pixel, and auto sizing.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Grid",
                        Grid::new()
                            .rows([GridLength::Auto; 2])
                            .columns([GridLength::STAR; 3])
                            .row_spacing(4.0)
                            .column_spacing(4.0)
                            .children((
                                cell("1,1", 0, 0),
                                cell("1,2", 0, 1),
                                cell("1,3", 0, 2),
                                cell("2,1", 1, 0),
                                cell("2,2", 1, 1),
                                cell("2,3", 1, 2),
                            )),
                        "Grid::new().rows(rows).columns(columns).children(items)",
                    ),
                ),
                KeyedView::new(
                    "mixed",
                    sample_card(
                        "Mixed Sizing",
                        Grid::new()
                            .rows([GridLength::Auto])
                            .columns([
                                GridLength::Pixel(100.0),
                                GridLength::Star(1.0),
                                GridLength::Star(2.0),
                            ])
                            .column_spacing(4.0)
                            .children((cell("100px", 0, 0), cell("1*", 0, 1), cell("2*", 0, 2))),
                        "Grid::new().columns([Pixel(100.0), Star(1.0), Star(2.0)])",
                    ),
                ),
                KeyedView::new(
                    "switch",
                    sample_card(
                        "Switchable Layout",
                        StackPanel::new().spacing(12.0).children((
                            ToggleSwitch::new()
                                .is_on(self.wide)
                                .on_toggled(context.forward())
                                .slot(ToggleSwitchSlot::Header, "Wide layout"),
                            dynamic,
                        )),
                        "if wide { three columns } else { two by two }",
                    ),
                ),
            ],
        )
    }
}
