//! Hosts a spinning composition `SpriteVisual` under a plain reactor element via
//! the composition-visual interop seam — with no dependency on the `windows`
//! crate. See [`CompositionHost`] / [`CompositionHostHandle`].

#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let host = composition_host()
        .width(220.0)
        .height(220.0)
        .on_mounted(|host| {
            // The element's DPI scale is available through the seam; a solid-color
            // island lives in the element's coordinate space (DIPs), so it is only
            // logged here to show the accessor.
            let scale = host.rasterization_scale().unwrap_or(1.0);
            eprintln!("composition host rasterization scale: {scale}");

            let island = ColorIsland::new(Color::rgb(80, 150, 240), 160.0, 160.0)
                .spin(Duration::from_secs(4));
            if let Err(e) = host.set_color_island(&island) {
                eprintln!("failed to host composition island: {e}");
            }
        });

    vstack((
        text_block("A composition SpriteVisual hosted under a reactor element,"),
        text_block("spinning forever on the composition thread."),
        border(host)
            .background(Color::rgb(24, 24, 24))
            .padding(Thickness::uniform(8.0)),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("CompositionIsland", app)
}
