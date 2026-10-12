#![windows_subsystem = "windows"]

use std::sync::Arc;
use windows_reactor::*;

struct Sample {
    icons: Vec<Arc<[u8]>>,
    page: u32,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            icons: (0..48).map(icon).collect(),
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
                self.icons
                    .iter()
                    .enumerate()
                    .map(|(index, icon)| {
                        Image::new()
                            .width(48.0)
                            .height(48.0)
                            .grid_row(index as i32 / 8)
                            .grid_column(index as i32 % 8)
                            .source_data(EncodedImage::new(Arc::clone(icon)))
                            .into()
                    })
                    .collect::<Vec<View>>(),
            );
        context.window_frame(
            "Image reuse",
            StackPanel::new().spacing(12.0).children((
                Button::new()
                    .content("Replace page")
                    .on_click(context.message(())),
                StackPanel::new().keyed_children([keyed(self.page, icons)]),
            )),
        )
    }
}

// A 48x48 32-bit BMP filled with one color.
fn icon(index: usize) -> Arc<[u8]> {
    let pixels = 48 * 48 * 4;
    let mut bytes = Vec::with_capacity(54 + pixels);
    bytes.extend_from_slice(b"BM");
    bytes.extend_from_slice(&(54 + pixels as u32).to_le_bytes());
    bytes.extend_from_slice(&[0, 0, 0, 0, 54, 0, 0, 0, 40, 0, 0, 0]);
    bytes.extend_from_slice(&48i32.to_le_bytes());
    bytes.extend_from_slice(&48i32.to_le_bytes());
    bytes.extend_from_slice(&[1, 0, 32, 0]);
    bytes.extend_from_slice(&[0; 24]);
    let shade = (index * 5) as u8;
    bytes.extend([shade, 255 - shade, 160, 255].repeat(48 * 48));
    bytes.into()
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
