//! Hosts a `Microsoft.UI.Composition` visual tree inside a WinUI 3 reactor app.
//!
//! This is the composition-interop counterpart of the `swap_chain_panel`
//! sample: [`composition_host`] delivers a [`CompositionHostHandle`], we obtain
//! its [`Compositor`], build a `SpriteVisual` tree with reactor's own lifted
//! composition API, and attach it with `CompositionHostHandle::set_child_visual`.
//! The visuals are laid out on the UI thread and re-laid-out on resize — no
//! per-frame render loop is required.

#![windows_subsystem = "windows"]

use windows_reactor::*;

/// The composition visual tree hosted inside the reactor element.
struct Scene {
    root: ContainerVisual,
    background: SpriteVisual,
    boxes: Vec<SpriteVisual>,
}

impl Scene {
    fn build(compositor: &Compositor) -> Result<Self> {
        let root = compositor.create_container_visual()?;

        // A dark background sprite that always fills the host.
        let background = compositor.create_sprite_visual()?;
        background.set_brush(&compositor.create_color_brush(Color::rgb(30, 30, 40))?)?;
        root.children()?.insert_at_bottom(&background)?;

        // A row of colored squares drawn on top of the background.
        let palette = [
            Color::rgb(0xff, 0x5c, 0x57),
            Color::rgb(0x57, 0xc7, 0xff),
            Color::rgb(0x8a, 0xff, 0x80),
        ];
        let mut boxes = Vec::new();
        for color in palette {
            let sprite = compositor.create_sprite_visual()?;
            sprite.set_brush(&compositor.create_color_brush(color)?)?;
            sprite.set_size(BOX_SIZE, BOX_SIZE)?;
            root.children()?.insert_at_top(&sprite)?;
            boxes.push(sprite);
        }

        let scene = Self {
            root,
            background,
            boxes,
        };
        scene.layout(400.0, 300.0)?;
        Ok(scene)
    }

    /// Centers the row of squares and stretches the background to `width` x
    /// `height` (in DIPs).
    fn layout(&self, width: f32, height: f32) -> Result<()> {
        self.root.set_size(width, height)?;
        self.background.set_size(width, height)?;

        let count = self.boxes.len() as f32;
        let total = count * BOX_SIZE + (count - 1.0) * BOX_GAP;
        let mut x = (width - total) / 2.0;
        let y = (height - BOX_SIZE) / 2.0;
        for sprite in &self.boxes {
            sprite.set_offset(x, y, 0.0)?;
            x += BOX_SIZE + BOX_GAP;
        }
        Ok(())
    }
}

const BOX_SIZE: f32 = 120.0;
const BOX_GAP: f32 = 24.0;

fn app(cx: &mut RenderCx) -> Element {
    let scene = cx.use_ref::<Option<Scene>>(None);

    composition_host()
        .on_mounted({
            let scene = scene.clone();
            move |host| {
                let init = || -> Result<Scene> {
                    let compositor = host.compositor()?;
                    let built = Scene::build(&compositor)?;
                    host.set_child_visual(&built.root)?;
                    Ok(built)
                };
                match init() {
                    Ok(built) => scene.set(Some(built)),
                    Err(e) => eprintln!("composition init failed: {e}"),
                }
            }
        })
        .on_resize(move |w, h| {
            if let Some(scene) = scene.borrow().as_ref() {
                _ = scene.layout(w as f32, h as f32);
            }
        })
        .into()
}

fn main() -> Result<()> {
    App::new().title("Composition Host").render(app)
}
