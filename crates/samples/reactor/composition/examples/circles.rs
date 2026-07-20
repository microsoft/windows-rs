//! Off-thread composition animation inside a reactor UI — the composition
//! counterpart of [`canvas/circles`](../../canvas/circles).
//!
//! The canvas sample redraws every frame from a reactor render callback. This
//! one builds a ring of `SpriteVisual`s **once** and starts a looping
//! `Vector3KeyFrameAnimation` on each; the pulse then runs entirely on the
//! *compositor thread* with zero reactor involvement. Reactor only rebuilds the
//! ring when the **Add**/**Remove** buttons change the circle count.
//!
//! ```text
//! cargo run -p reactor_composition --example circles
//! ```

#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use std::time::Duration;
use windows_composition::{Color, CompositionHostExt, Compositor, ContainerVisual, SpriteVisual};
use windows_numerics::Vector3;
use windows_reactor::*;

/// Diameter of one circle, in DIPs (before the pulse scales it up).
const CIRCLE: f32 = 72.0;
const MIN_CIRCLES: u32 = 1;
const MAX_CIRCLES: u32 = 24;

/// The hosted composition visual tree: a container of pulsing circles.
struct Scene {
    compositor: Compositor,
    root: ContainerVisual,
    circles: Vec<SpriteVisual>,
    width: f32,
    height: f32,
}

impl Scene {
    /// Attaches an empty root container to the host element.
    fn new(host: &CompositionHostHandle) -> Result<Self> {
        let compositor = host.compositor()?;
        let root = compositor.create_container_visual();
        host.set_child_visual(&root)?;
        Ok(Self {
            compositor,
            root,
            circles: Vec::new(),
            width: 400.0,
            height: 300.0,
        })
    }

    /// Rebuilds the ring so it holds exactly `count` pulsing circles.
    fn set_count(&mut self, count: usize) -> Result<()> {
        let children = self.root.children();
        children.remove_all();
        self.circles.clear();
        for i in 0..count {
            let circle = self.compositor.create_sprite_visual();
            circle.set_size(CIRCLE, CIRCLE);
            // Pulse (scale) about the circle's own center, not its top-left.
            circle.set_center_point(Vector3 {
                x: CIRCLE / 2.0,
                y: CIRCLE / 2.0,
                z: 0.0,
            });
            circle.set_brush(&self.compositor.create_color_brush(ring_color(i, count)));
            children.insert_at_top(&circle);
            self.start_pulse(&circle, i, count)?;
            self.circles.push(circle);
        }
        self.layout()
    }

    /// Starts an endless scale pulse, staggered so it travels around the ring.
    fn start_pulse(&self, circle: &SpriteVisual, index: usize, count: usize) -> Result<()> {
        let pulse = self.compositor.create_vector3_key_frame_animation();
        pulse.insert_key_frame(
            0.0,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        pulse.insert_key_frame(
            0.5,
            Vector3 {
                x: 1.6,
                y: 1.6,
                z: 1.0,
            },
        );
        pulse.insert_key_frame(
            1.0,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        pulse.set_duration(Duration::from_millis(2000));
        let phase = index as f64 / count.max(1) as f64;
        pulse.set_delay(Duration::from_millis((phase * 1800.0) as u64));
        pulse.set_iterate_forever();
        circle.start_animation("Scale", &pulse);
        Ok(())
    }

    fn resize(&mut self, width: f32, height: f32) -> Result<()> {
        self.width = width;
        self.height = height;
        self.layout()
    }

    /// Positions the circles evenly around a centered ring.
    fn layout(&self) -> Result<()> {
        self.root.set_size(self.width, self.height);
        let n = self.circles.len();
        if n == 0 {
            return Ok(());
        }
        let cx = self.width / 2.0;
        let cy = self.height / 2.0;
        let radius = (cx.min(cy) - CIRCLE) * 0.9;
        for (i, circle) in self.circles.iter().enumerate() {
            let angle = i as f32 / n as f32 * TAU;
            circle.set_offset(
                cx + angle.cos() * radius - CIRCLE / 2.0,
                cy + angle.sin() * radius - CIRCLE / 2.0,
                0.0,
            );
        }
        Ok(())
    }
}

/// A saturated rainbow color for circle `i` of `count`.
fn ring_color(i: usize, count: usize) -> Color {
    let hue = i as f32 / count.max(1) as f32 * 6.0;
    let x = (255.0 * (1.0 - (hue % 2.0 - 1.0).abs())) as u8;
    let (r, g, b) = match hue as u32 {
        0 => (255, x, 0),
        1 => (x, 255, 0),
        2 => (0, 255, x),
        3 => (0, x, 255),
        4 => (x, 0, 255),
        _ => (255, 0, x),
    };
    Color::rgb(r, g, b)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(6_u32);
    let scene = cx.use_ref::<Option<Scene>>(None);
    let first_effect = cx.use_ref(true);

    // The initial ring is built in `on_mounted`; rebuild it only when the count
    // actually changes (skip the effect's initial run).
    {
        let scene = scene.clone();
        cx.use_effect((count,), move || {
            if std::mem::replace(&mut *first_effect.borrow_mut(), false) {
                return;
            }
            if let Some(scene) = scene.borrow_mut().as_mut() {
                scene.set_count(count as usize).unwrap();
            }
        });
    }

    let add = {
        let set_count = set_count.clone();
        move || set_count.call((count + 1).min(MAX_CIRCLES))
    };
    let remove = move || {
        if count > MIN_CIRCLES {
            set_count.call(count - 1);
        }
    };

    let margin = 16.0;
    grid((
        Element::from(
            composition_host()
                .on_mounted({
                    let scene = scene.clone();
                    move |host| match Scene::new(&host) {
                        Ok(mut built) => {
                            built.set_count(count as usize).unwrap();
                            scene.set(Some(built));
                        }
                        Err(e) => eprintln!("composition init failed: {e}"),
                    }
                })
                .on_resize(move |w, h| {
                    if let Some(scene) = scene.borrow_mut().as_mut() {
                        scene.resize(w as f32, h as f32).unwrap();
                    }
                }),
        )
        .grid_row(0),
        Element::from(
            hstack((
                button("Add circle").on_click(add),
                button("Remove circle").on_click(remove),
                text_block(format!("{count} circles"))
                    .font_size(16.0)
                    .opacity(0.75),
            ))
            .spacing(8.0)
            .margin(Thickness::uniform(margin)),
        )
        .grid_row(1),
    ))
    .rows([GridLength::STAR, GridLength::Auto])
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new()
        .title("Composition Circles")
        .backdrop(Backdrop::Mica)
        .render(app)
}
