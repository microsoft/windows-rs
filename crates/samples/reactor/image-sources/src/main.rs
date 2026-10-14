#![windows_subsystem = "windows"]

use std::path::Path;
use windows_reactor::*;

const PNG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/image.png"));
const SVG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/image.svg"));

struct Sample {
    png: EncodedImage,
    svg: EncodedImage,
    page: u32,
    visible: bool,
    large: bool,
    dark: bool,
    selected: usize,
}

#[derive(Clone)]
enum Message {
    Replace,
    Visibility,
    Size,
    Theme,
    Title(usize),
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            png: EncodedImage::new(PNG.to_vec()),
            svg: EncodedImage::new(SVG.to_vec()),
            page: 0,
            visible: true,
            large: false,
            dark: false,
            selected: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Replace => self.page += 1,
            Message::Visibility => self.visible = !self.visible,
            Message::Size => self.large = !self.large,
            Message::Theme => self.dark = !self.dark,
            Message::Title(index) => self.selected = index,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().client_size(1000.0, 820.0).theme(
            if self.dark {
                WindowTheme::Dark
            } else {
                WindowTheme::Light
            },
        ));
        let directory = Path::new(env!("CARGO_MANIFEST_DIR"));
        let png_path = directory.join("image.png");
        let svg_path = directory.join("image.svg");
        let uri = |name| {
            format!(
                "file:///{}/{name}",
                env!("CARGO_MANIFEST_DIR").replace('\\', "/")
            )
        };
        let png_uri = uri("image.png");
        let svg_uri = uri("image.svg");
        let mut index = 0;
        let mut title_icon = None;
        let mut title_label = "";
        let mut row = |label: &'static str, image, image_icon, icon: Icon| {
            let current = index;
            index += 1;
            if current == self.selected {
                title_label = label;
                if self.visible {
                    title_icon = Some(icon.clone());
                }
            }
            self.row(
                label,
                image,
                image_icon,
                icon,
                context.message(Message::Title(current)),
            )
        };
        let images = StackPanel::new().spacing(4.0).children((
            row(
                "PNG: URI",
                Image::new().source(png_uri.as_str()).unwrap(),
                ImageIcon::new().source(png_uri.as_str()).unwrap(),
                Icon::image_uri(png_uri.as_str()).unwrap(),
            ),
            row(
                "PNG: file",
                Image::new().source_file(&png_path).unwrap(),
                ImageIcon::new().source_file(&png_path).unwrap(),
                Icon::image_file(&png_path).unwrap(),
            ),
            row(
                "PNG: static bytes",
                Image::new().source_data(EncodedImage::from_static(PNG)),
                ImageIcon::new().source_data(EncodedImage::from_static(PNG)),
                Icon::image_data(EncodedImage::from_static(PNG)),
            ),
            row(
                "PNG: shared bytes",
                Image::new().source_data(self.png.clone()),
                ImageIcon::new().source_data(self.png.clone()),
                Icon::image(ImageSource::encoded(self.png.clone())),
            ),
            row(
                "SVG: URI",
                Image::new().source(svg_uri.as_str()).unwrap(),
                ImageIcon::new().source(svg_uri.as_str()).unwrap(),
                Icon::image_uri(svg_uri.as_str()).unwrap(),
            ),
            row(
                "SVG: file",
                Image::new().source_file(&svg_path).unwrap(),
                ImageIcon::new().source_file(&svg_path).unwrap(),
                Icon::image_file(&svg_path).unwrap(),
            ),
            row(
                "SVG: static bytes",
                Image::new().source_svg(EncodedImage::from_static(SVG)),
                ImageIcon::new().source_svg(EncodedImage::from_static(SVG)),
                Icon::image_svg(EncodedImage::from_static(SVG)),
            ),
            row(
                "SVG: shared bytes",
                Image::new().source_svg(self.svg.clone()),
                ImageIcon::new().source_svg(self.svg.clone()),
                Icon::image(ImageSource::svg(self.svg.clone())),
            ),
        ));
        let content = Grid::new()
            .margin(16.0)
            .rows([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
            ])
            .children((
                TextBlock::new()
                    .text("Matching artwork: red/blue above green/gold, with transparent gaps.")
                    .margin(Thickness::new(0.0, 0.0, 0.0, 12.0)),
                StackPanel::new()
                    .grid_row(1)
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .content("Replace page")
                            .on_click(context.message(Message::Replace)),
                        Button::new()
                            .content(if self.visible {
                                "Clear sources"
                            } else {
                                "Restore sources"
                            })
                            .on_click(context.message(Message::Visibility)),
                        Button::new()
                            .content(if self.large {
                                "Small images"
                            } else {
                                "Large images"
                            })
                            .on_click(context.message(Message::Size)),
                        Button::new()
                            .content(if self.dark {
                                "Light theme"
                            } else {
                                "Dark theme"
                            })
                            .on_click(context.message(Message::Theme)),
                    )),
                columns()
                    .grid_row(2)
                    .margin(Thickness::xy(0.0, 12.0))
                    .children(
                        [
                            "Source",
                            "Image",
                            "ImageIcon",
                            "IconElement",
                            "IconSource",
                            "Flyout",
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(column, label)| {
                            TextBlock::new()
                                .text(label)
                                .grid_column(column as i32)
                                .into()
                        })
                        .collect::<Vec<View>>(),
                    ),
                ScrollViewer::new()
                    .grid_row(3)
                    .content(StackPanel::new().keyed_children([keyed(self.page, images)])),
            ));
        context.window_title("Image sources");
        let title = TitleBar::new().title(format!("Image sources - {title_label}"));
        let title = if let Some(icon) = title_icon {
            title.icon(icon)
        } else {
            title
        };
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((title, content.grid_row(1)))
            .into()
    }
}

impl Sample {
    fn row(
        &self,
        label: &str,
        image: Image,
        image_icon: ImageIcon,
        icon: Icon,
        select: Callback<()>,
    ) -> View {
        let size: f64 = if self.large { 96.0 } else { 32.0 };
        let (image, image_icon, button) = if self.visible {
            (image, image_icon, AppBarButton::new().icon(icon.clone()))
        } else {
            (Image::new(), ImageIcon::new(), AppBarButton::new())
        };
        let command = |key, label| {
            if self.visible {
                CommandBarCommand::button_with_icon(key, label, icon.clone())
            } else {
                CommandBarCommand::button(key, label)
            }
        };
        columns()
            .min_height(size.max(64.0))
            .children((
                TextBlock::new()
                    .text(label)
                    .vertical_alignment(VerticalAlignment::Center),
                image.grid_column(1).width(size).height(size),
                image_icon.grid_column(2).width(size).height(size),
                button.grid_column(3).label("Button"),
                Button::new()
                    .grid_column(4)
                    .content("Use in title bar")
                    .on_click(select),
                Button::new()
                    .grid_column(5)
                    .content("Open")
                    .command_bar_flyout(CommandBarFlyout::new(
                        [command("primary", "Primary")],
                        [command("secondary", "Secondary")],
                        |_: Key| {},
                    )),
            ))
            .into()
    }
}

fn columns() -> Grid {
    Grid::new().columns([
        GridLength::Pixel(170.0),
        GridLength::Pixel(120.0),
        GridLength::Pixel(120.0),
        GridLength::Pixel(120.0),
        GridLength::STAR,
        GridLength::Pixel(80.0),
    ])
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
