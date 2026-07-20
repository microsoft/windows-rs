//! Minimal composition interop: obtain the host element's compositor, build a
//! tiny visual tree, and attach it. The "hello world" of hosting a lifted
//! `Microsoft.UI.Composition` tree in a reactor element.
//!
//! ```text
//! cargo run -p reactor_composition --example host
//! ```

#![windows_subsystem = "windows"]

use windows_composition::{Color, CompositionHostExt, ContainerVisual, SpriteVisual};
use windows_reactor::*;

/// A background that fills the host and a centered blue square on top.
struct Scene {
    root: ContainerVisual,
    background: SpriteVisual,
    square: SpriteVisual,
}

impl Scene {
    fn build(host: &CompositionHostHandle) -> Result<Self> {
        let compositor = host.compositor()?;

        let root = compositor.create_container_visual();
        let background = compositor.create_sprite_visual();
        background.set_brush(&compositor.create_color_brush(Color::rgb(24, 24, 32)));
        root.children().insert_at_bottom(&background);

        let square = compositor.create_sprite_visual();
        square.set_size(160.0, 160.0);
        square.set_brush(&compositor.create_color_brush(Color::rgb(0, 120, 215)));
        root.children().insert_at_top(&square);

        host.set_child_visual(&root)?;
        let scene = Self {
            root,
            background,
            square,
        };
        scene.layout(400.0, 300.0)?;
        Ok(scene)
    }

    fn layout(&self, width: f32, height: f32) -> Result<()> {
        self.root.set_size(width, height);
        self.background.set_size(width, height);
        let size = self.square.size();
        self.square
            .set_offset((width - size.x) / 2.0, (height - size.y) / 2.0, 0.0);
        Ok(())
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let scene = cx.use_ref::<Option<Scene>>(None);
    composition_host()
        .on_mounted({
            let scene = scene.clone();
            move |host| match Scene::build(&host) {
                Ok(built) => scene.set(Some(built)),
                Err(e) => eprintln!("composition init failed: {e}"),
            }
        })
        .on_resize(move |w, h| {
            if let Some(scene) = scene.borrow().as_ref() {
                scene.layout(w as f32, h as f32).unwrap();
            }
        })
        .into()
}

fn main() -> Result<()> {
    reactor_composition::run("Composition Host", app)
}
