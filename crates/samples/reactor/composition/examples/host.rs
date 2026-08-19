#![windows_subsystem = "windows"]

use windows_composition::{Color, ContainerVisual, SpriteVisual};
use windows_reactor::{CompositionContent, CompositionHost, Element, RenderCx};

struct Scene {
    root: ContainerVisual,
    background: SpriteVisual,
    square: SpriteVisual,
}

impl Scene {
    fn new(compositor: &windows_composition::Compositor) -> Self {
        let root = compositor.create_container_visual();
        let background = compositor.create_sprite_visual();
        background.set_brush(&compositor.create_color_brush(Color::rgb(24, 24, 32)));
        root.children().insert_at_bottom(&background);

        let square = compositor.create_sprite_visual();
        square.set_size(160.0, 160.0);
        square.set_brush(&compositor.create_color_brush(Color::rgb(0, 120, 215)));
        root.children().insert_at_top(&square);

        Self {
            root,
            background,
            square,
        }
    }

    fn layout(&self, width: f32, height: f32) {
        self.root.set_size(width, height);
        self.background.set_size(width, height);
        let size = self.square.size();
        self.square
            .set_offset((width - size.x) / 2.0, (height - size.y) / 2.0, 0.0);
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let host = cx.use_composition_host_ref::<Scene>();
    CompositionHost::new(
        &host,
        |compositor| {
            let scene = Scene::new(compositor);
            let root = scene.root.clone();
            Ok(CompositionContent::new(scene, root))
        },
        |scene, layout| {
            scene.layout(layout.width, layout.height);
            Ok(())
        },
    )
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_composition::run("Composition Host", app)
}
