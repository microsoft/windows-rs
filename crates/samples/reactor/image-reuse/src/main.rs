#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Sample {
    icon: ImageSource,
    page: u32,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            icon: ImageSource::file(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.svg")).unwrap(),
            page: 0,
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.page += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let icons = Grid::new()
            .columns([GridLength::Pixel(56.0); 8])
            .rows([GridLength::Pixel(56.0); 6])
            .children(
                (0..48)
                    .map(|index| {
                        Image::new()
                            .width(48.0)
                            .height(48.0)
                            .grid_row(index / 8)
                            .grid_column(index % 8)
                            .source(self.icon.clone())
                            .unwrap()
                            .into()
                    })
                    .collect::<Vec<View>>(),
            );
        context.window_frame(
            "Reusable images",
            StackPanel::new().spacing(12.0).children((
                Button::new()
                    .content("Replace page")
                    .on_click(context.message(())),
                StackPanel::new().keyed_children([keyed(self.page, icons)]),
            )),
        )
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
