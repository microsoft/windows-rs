#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::f32::consts::TAU;
use std::rc::Rc;
use std::time::Duration;
use windows_composition::{CompositionColor, Compositor, ContainerVisual, SpriteVisual};
use windows_core::Result;
use windows_numerics::Vector3;
use windows_reactor::*;

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
    fn new(
        compositor: windows_core::IUnknown,
        host: &ElementRef<Grid>,
        width: f32,
        height: f32,
    ) -> Result<Self> {
        let compositor = Compositor::from_host(compositor)?;
        let root = compositor.create_container_visual();
        let _ = host.request_set_child_visual(Some(root.as_raw().into()), |_| {});
        Ok(Self {
            compositor,
            root,
            circles: Vec::new(),
            width,
            height,
        })
    }

    fn set_count(&mut self, count: usize) -> Result<()> {
        let children = self.root.children();
        children.remove_all();
        self.circles.clear();
        for i in 0..count {
            let circle = self.compositor.create_sprite_visual();
            circle.set_size(CIRCLE, CIRCLE);
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

fn ring_color(i: usize, count: usize) -> CompositionColor {
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
    CompositionColor::rgb(r, g, b)
}

struct Sample {
    count: u32,
    host: ElementRef<Grid>,
    scene: Rc<RefCell<Option<Scene>>>,
}

#[derive(Clone)]
enum Message {
    Add,
    Remove,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            count: 6,
            host: ElementRef::new(),
            scene: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count = (self.count + 1).min(MAX_CIRCLES),
            Message::Remove => self.count = self.count.saturating_sub(1).max(MIN_CIRCLES),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition Circles");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let scene = Rc::clone(&self.scene);
        let count = self.count;
        context.use_effect("circle-count", count, move || {
            if let Some(scene) = scene.borrow_mut().as_mut() {
                scene.set_count(count as usize).unwrap();
            }
            None
        });

        let host = self.host.clone();
        let scene = Rc::clone(&self.scene);
        context.use_effect("composition-host", (), move || {
            let event_host = host.clone();
            let observation = host.observe_composition_host(move |event| match event {
                CompositionHostEvent::Ready {
                    compositor,
                    width,
                    height,
                    ..
                } => match Scene::new(compositor, &event_host, width as f32, height as f32) {
                    Ok(mut built) => {
                        built.set_count(count as usize).unwrap();
                        *scene.borrow_mut() = Some(built);
                    }
                    Err(error) => eprintln!("composition init failed: {error}"),
                },
                CompositionHostEvent::Metrics { width, height, .. } => {
                    if let Some(scene) = scene.borrow_mut().as_mut() {
                        scene.resize(width as f32, height as f32).unwrap();
                    }
                }
            });
            Some(Box::new(move || drop(observation)))
        });

        Grid::new()
            .rows([GridLength::STAR, GridLength::Auto])
            .children((
                Grid::new().element_ref(&self.host).grid_row(0),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .margin(Thickness::uniform(16.0))
                    .grid_row(1)
                    .children((
                        Button::new()
                            .on_click(context.message(Message::Add))
                            .content("Add circle"),
                        Button::new()
                            .on_click(context.message(Message::Remove))
                            .content("Remove circle"),
                        TextBlock::new()
                            .text(format!("{} circles", self.count))
                            .font_size(16.0)
                            .opacity(0.75),
                    )),
            ))
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
