#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use std::time::Duration;

use windows_composition::{Color, Compositor, ContainerVisual, SpriteVisual};
use windows_numerics::Vector3;
use windows_reactor::{
    Border, Button, CompositionContent, CompositionHost, Element, Grid, GridChild, GridLength,
    RenderCx, TextBlock, Thickness, hstack,
};

const CIRCLE: f32 = 72.0;
const MIN_CIRCLES: u32 = 1;
const MAX_CIRCLES: u32 = 24;

struct Scene {
    compositor: Compositor,
    root: ContainerVisual,
    circles: Vec<SpriteVisual>,
    width: f32,
    height: f32,
}

impl Scene {
    fn new(compositor: Compositor, count: usize) -> windows_core::Result<Self> {
        let mut scene = Self {
            root: compositor.create_container_visual(),
            compositor,
            circles: Vec::new(),
            width: 400.0,
            height: 300.0,
        };
        scene.set_count(count)?;
        Ok(scene)
    }

    fn set_count(&mut self, count: usize) -> windows_core::Result<()> {
        let children = self.root.children();
        children.remove_all();
        self.circles.clear();
        for index in 0..count {
            let circle = self.compositor.create_sprite_visual();
            circle.set_size(CIRCLE, CIRCLE);
            circle.set_center_point(Vector3::new(CIRCLE / 2.0, CIRCLE / 2.0, 0.0));
            circle.set_brush(&self.compositor.create_color_brush(ring_color(index, count)));
            children.insert_at_top(&circle);

            let pulse = self.compositor.create_vector3_key_frame_animation();
            pulse.insert_key_frame(0.0, Vector3::new(1.0, 1.0, 1.0));
            pulse.insert_key_frame(0.5, Vector3::new(1.6, 1.6, 1.0));
            pulse.insert_key_frame(1.0, Vector3::new(1.0, 1.0, 1.0));
            pulse.set_duration(Duration::from_millis(2000));
            pulse.set_delay(Duration::from_millis(
                (index as f64 / count.max(1) as f64 * 1800.0) as u64,
            ));
            pulse.set_iterate_forever();
            circle.start_animation("Scale", &pulse);
            self.circles.push(circle);
        }
        self.layout();
        Ok(())
    }

    fn layout(&self) {
        self.root.set_size(self.width, self.height);
        let count = self.circles.len();
        if count == 0 {
            return;
        }
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;
        let radius = (center_x.min(center_y) - CIRCLE) * 0.9;
        for (index, circle) in self.circles.iter().enumerate() {
            let angle = index as f32 / count as f32 * TAU;
            circle.set_offset(
                center_x + angle.cos() * radius - CIRCLE / 2.0,
                center_y + angle.sin() * radius - CIRCLE / 2.0,
                0.0,
            );
        }
    }
}

fn ring_color(index: usize, count: usize) -> Color {
    let hue = index as f32 / count.max(1) as f32 * 6.0;
    let value = (255.0 * (1.0 - (hue % 2.0 - 1.0).abs())) as u8;
    match hue as u32 {
        0 => Color::rgb(255, value, 0),
        1 => Color::rgb(value, 255, 0),
        2 => Color::rgb(0, 255, value),
        3 => Color::rgb(0, value, 255),
        4 => Color::rgb(value, 0, 255),
        _ => Color::rgb(255, 0, value),
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 6_u32);
    let current = count.value();
    let host = cx.use_composition_host_ref::<Scene>();
    let add_host = host.clone();
    let remove_host = host.clone();
    let initial_count = current as usize;

    Grid::new([
        GridChild::new(
            CompositionHost::new(
                &host,
                move |compositor| {
                    let scene = Scene::new(compositor.clone(), initial_count)?;
                    let root = scene.root.clone();
                    Ok(CompositionContent::new(scene, root))
                },
                |scene, layout| {
                    scene.width = layout.width;
                    scene.height = layout.height;
                    scene.layout();
                    Ok(())
                },
            )
            .build(),
        )
        .row(0),
        GridChild::new(
            Border::new(hstack(
                8.0,
                [
                    Button::new("Add circle")
                        .on_click({
                            let count = count.clone();
                            move || {
                                let next = (current + 1).min(MAX_CIRCLES);
                                count.set(next);
                                add_host.update(move |scene| scene.set_count(next as usize));
                            }
                        })
                        .build(),
                    Button::new("Remove circle")
                        .on_click(move || {
                            let next = current.saturating_sub(1).max(MIN_CIRCLES);
                            count.set(next);
                            remove_host.update(move |scene| scene.set_count(next as usize));
                        })
                        .build(),
                    TextBlock::new(format!("{current} circles")).build(),
                ],
            ))
            .padding(Thickness::uniform(16.0))
            .build(),
        )
        .row(1),
    ])
    .rows([GridLength::STAR, GridLength::Auto])
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_composition::run("Composition Circles", app)
}
